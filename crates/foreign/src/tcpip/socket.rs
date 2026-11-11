//! Port of foreign/tcpip/socket.{h,cpp}
//!
//! Uses std::net::TcpStream / TcpListener in place of the BSD socket calls. A `Socket` is either
//! a client connection (`stream`, C++ socket_) or a listening server (`listener`, C++ server_socket_),
//! and accepting a connection on a listening socket can hand the connection to a new `Socket`.
//!
//! Wire framing (sendExact/receiveExact): 4-byte big-endian total length (including the 4 bytes),
//! then the payload.
//!
//! shortcut: the listen backlog is std's default (128) instead of the C++ value 10; std has no backlog setter.
//! shortcut: the SHAWN wrapper (init_tcpip, ENABLE_TCPIP config) is not ported; Winsock is initialised by std.
//! shortcut: getaddrinfo's AI_PASSIVE hint has no effect for connect and is not passed on.

use std::fmt;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::storage::Storage;

/// C++ Socket::lengthLen: size of the length prefix of a message
const LENGTH_LEN: usize = 4;

/// C++ SocketException (std::runtime_error with the message as what())
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SocketException(pub String);

impl fmt::Display for SocketException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SocketException {}

/// C++ BailOnSocketError: `context + ": " + strerror(errno)`. The OS error text is shown without
/// Rust's " (os error N)" suffix, to match strerror.
fn bail(context: &str, err: io::Error) -> SocketException {
    let text = err.to_string();
    let text = match text.rfind(" (os error") {
        Some(i) => text[..i].to_string(),
        None => text,
    };
    SocketException(format!("{context}: {text}"))
}

pub struct Socket {
    host: String,
    port: i32,
    /// C++ socket_: client connection, -1 when none
    stream: Option<TcpStream>,
    /// C++ server_socket_: listening socket, created on first accept()
    listener: Option<TcpListener>,
    blocking: bool,
    verbose: bool,
}

impl Socket {
    /// C++ Socket(int port): prepared for accepting connections on `port`.
    pub fn new(port: i32) -> Self {
        Self::with_host("", port)
    }

    /// C++ Socket(std::string host, int port): prepared to connect to host:port.
    pub fn with_host(host: impl Into<String>, port: i32) -> Self {
        Socket {
            host: host.into(),
            port,
            stream: None,
            listener: None,
            blocking: true,
            verbose: false,
        }
    }

    /// C++ getFreeSocketPort: binds port 0 and returns the port the OS assigned.
    pub fn get_free_socket_port() -> std::result::Result<i32, SocketException> {
        let listener = TcpListener::bind(("0.0.0.0", 0))
            .map_err(|e| bail("tcpip::Socket::getFreeSocketPort() Unable to bind socket", e))?;
        let port = listener
            .local_addr()
            .map_err(|e| bail("tcpip::Socket::getFreeSocketPort() Unable to get socket name", e))?
            .port();
        Ok(i32::from(port))
    }

    /// C++ port
    pub fn port(&self) -> i32 {
        self.port
    }

    /// C++ verbose
    pub fn verbose(&self) -> bool {
        self.verbose
    }

    /// C++ set_verbose: if verbose, each send and receive is written to stderr.
    pub fn set_verbose(&mut self, new_verbose: bool) {
        self.verbose = new_verbose;
    }

    /// C++ connect: connects to host:port, trying each resolved address in order.
    pub fn connect(&mut self) -> std::result::Result<(), SocketException> {
        let port = u16::try_from(self.port).map_err(|_| {
            SocketException("tcpip::Socket::connect() @ Invalid network address: invalid port".to_string())
        })?;
        let addrs = (self.host.as_str(), port)
            .to_socket_addrs()
            .map_err(|e| bail("tcpip::Socket::connect() @ Invalid network address", e))?;

        self.stream = None;
        let mut last_error = None;
        for addr in addrs {
            match TcpStream::connect(addr) {
                Ok(stream) => {
                    let _ = stream.set_nodelay(true); // C++ ignores the setsockopt result
                    self.stream = Some(stream);
                    return Ok(());
                }
                Err(e) => last_error = Some(e),
            }
        }
        let err = last_error.unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no address resolved"));
        Err(bail("tcpip::Socket::connect() @ socket", err))
    }

    /// C++ accept: waits for an incoming connection on `port`. With `create`, the connection is
    /// returned as a new Socket; otherwise it becomes this socket's client connection.
    /// Returns None when already connected, or when no connection was accepted (the C++ code
    /// ignores accept() errors in that case).
    pub fn accept(&mut self, create: bool) -> std::result::Result<Option<Socket>, SocketException> {
        if self.stream.is_some() {
            return Ok(None);
        }

        if self.listener.is_none() {
            // C++ sets SO_REUSEADDR on non-Windows; std's TcpListener::bind does the same on Unix.
            let port = u16::try_from(self.port).map_err(|_| {
                SocketException("tcpip::Socket::accept() Unable to create listening socket: invalid port".to_string())
            })?;
            let listener = TcpListener::bind(("0.0.0.0", port))
                .map_err(|e| bail("tcpip::Socket::accept() Unable to create listening socket", e))?;
            self.listener = Some(listener);
            // Make the newly created socket blocking or not
            self.set_blocking(self.blocking)?;
        }

        let accepted = match self.listener.as_ref().map(|l| l.accept()) {
            Some(Ok((stream, _addr))) => Some(stream),
            _ => None,
        };
        let Some(stream) = accepted else {
            return Ok(None);
        };
        let _ = stream.set_nodelay(true);
        let _ = stream.set_nonblocking(false); // client connections are always blocking in C++

        if create {
            let mut result = Socket::new(0);
            result.stream = Some(stream);
            Ok(Some(result))
        } else {
            self.stream = Some(stream);
            Ok(None)
        }
    }

    /// C++ set_blocking: applies to the listening socket (the client connection is always blocking).
    pub fn set_blocking(&mut self, blocking: bool) -> std::result::Result<(), SocketException> {
        self.blocking = blocking;
        if let Some(listener) = &self.listener {
            listener
                .set_nonblocking(!blocking)
                .map_err(|e| bail("tcpip::Socket::set_blocking() Unable to initialize non blocking I/O", e))?;
        }
        Ok(())
    }

    /// C++ is_blocking
    pub fn is_blocking(&self) -> bool {
        self.blocking
    }

    /// C++ has_client_connection
    pub fn has_client_connection(&self) -> bool {
        self.stream.is_some()
    }

    /// C++ close: closes the client connection (the listening socket stays open).
    pub fn close(&mut self) {
        self.stream = None;
    }

    /// C++ send: writes all bytes. Does nothing without a client connection.
    pub fn send(&mut self, buffer: &[u8]) -> std::result::Result<(), SocketException> {
        if self.stream.is_none() {
            return Ok(());
        }
        self.print_buffer_on_verbose(buffer, "Send");
        let stream = self.stream.as_mut().expect("checked above");
        stream.write_all(buffer).map_err(|e| bail("send failed", e))
    }

    /// C++ sendExact: sends the storage content with the 4-byte total length prefix.
    pub fn send_exact(&mut self, b: &Storage) -> std::result::Result<(), SocketException> {
        let length = b.size() as i32;
        let mut length_storage = Storage::new();
        length_storage.write_int(LENGTH_LEN as i32 + length);

        // Sending length_storage and b in one buffer avoids two TCP writes
        let mut msg = Vec::with_capacity(LENGTH_LEN + b.size());
        msg.extend_from_slice(length_storage.as_bytes());
        msg.extend_from_slice(b.as_bytes());
        self.send(&msg)
    }

    /// C++ datawaiting: true when data (or EOF) is readable now, without blocking.
    /// Implemented with a non-blocking peek in place of select(), then the socket is made blocking again.
    fn datawaiting(&mut self) -> std::result::Result<bool, SocketException> {
        let Some(stream) = self.stream.as_mut() else {
            return Ok(false);
        };
        stream
            .set_nonblocking(true)
            .map_err(|e| bail("tcpip::Socket::datawaiting @ select", e))?;
        let mut probe = [0u8; 1];
        let result = stream.peek(&mut probe);
        stream
            .set_nonblocking(false)
            .map_err(|e| bail("tcpip::Socket::datawaiting @ select", e))?;
        match result {
            // EOF also counts as readable (select semantics); the following recv reports the shutdown
            Ok(_) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(false),
            Err(e) => Err(bail("tcpip::Socket::datawaiting @ select", e)),
        }
    }

    /// C++ recvAndCheck: one recv() call. Errors on EOF ("peer shutdown").
    fn recv_and_check(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, SocketException> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(SocketException(
                "tcpip::Socket::recvAndCheck @ recv: Bad file descriptor".to_string(),
            ));
        };
        match stream.read(buffer) {
            Ok(0) => Err(SocketException(
                "tcpip::Socket::recvAndCheck @ recv: peer shutdown".to_string(),
            )),
            Ok(n) => Ok(n),
            Err(e) => Err(bail("tcpip::Socket::recvAndCheck @ recv", e)),
        }
    }

    /// C++ receiveComplete: loops recv until `buffer` is full.
    fn receive_complete(&mut self, buffer: &mut [u8]) -> std::result::Result<(), SocketException> {
        let mut offset = 0;
        while offset < buffer.len() {
            let received = self.recv_and_check(&mut buffer[offset..])?;
            offset += received;
        }
        Ok(())
    }

    /// C++ receive: up to `buf_size` bytes that are available now (empty when none).
    /// Connects first when there is no client connection.
    pub fn receive(&mut self, buf_size: usize) -> std::result::Result<Vec<u8>, SocketException> {
        if self.stream.is_none() {
            self.connect()?;
        }
        if !self.datawaiting()? {
            return Ok(Vec::new());
        }
        let mut buffer = vec![0u8; buf_size];
        let received = self.recv_and_check(&mut buffer)?;
        buffer.truncate(received);
        self.print_buffer_on_verbose(&buffer, "Rcvd");
        Ok(buffer)
    }

    /// C++ receiveExact: reads one framed message into `msg` (replacing its content).
    pub fn receive_exact(&mut self, msg: &mut Storage) -> std::result::Result<bool, SocketException> {
        let mut buffer = vec![0u8; LENGTH_LEN];

        // receive length of the message
        self.receive_complete(&mut buffer)?;
        let total_len = Storage::from_bytes(&buffer).read_int().expect("4 bytes present");
        // C++ asserts total_len > lengthLen (debug only); a length below the prefix is rejected here
        if total_len < LENGTH_LEN as i32 {
            return Err(SocketException(format!(
                "tcpip::Socket::receiveExact: invalid message length {total_len}"
            )));
        }
        let total_len = total_len as usize;

        // extend buffer and receive the rest of the message
        buffer.resize(total_len, 0);
        self.receive_complete(&mut buffer[LENGTH_LEN..])?;

        // copy message content into the passed Storage
        msg.reset();
        msg.write_packet(&buffer[LENGTH_LEN..]);

        self.print_buffer_on_verbose(&buffer, "Rcvd Storage with");
        Ok(true)
    }

    /// C++ printBufferOnVerbose: `label N bytes via tcpip::Socket: [ b0 b1 ... ]` on stderr.
    fn print_buffer_on_verbose(&self, buffer: &[u8], label: &str) {
        if !self.verbose {
            return;
        }
        let mut line = format!("{label} {} bytes via tcpip::Socket: [", buffer.len());
        for b in buffer {
            line.push_str(&format!(" {b} "));
        }
        line.push_str("]\n");
        eprint!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn free_port_is_nonzero() {
        assert!(Socket::get_free_socket_port().unwrap() > 0);
    }

    #[test]
    fn framed_storage_round_trip_over_loopback() {
        let port = Socket::get_free_socket_port().unwrap();

        let server = thread::spawn(move || {
            let mut listening = Socket::new(port);
            let mut conn = loop {
                if let Some(conn) = listening.accept(true).unwrap() {
                    break conn;
                }
                thread::sleep(Duration::from_millis(5));
            };
            let mut received = Storage::new();
            assert!(conn.receive_exact(&mut received).unwrap());
            assert_eq!(received.read_int().unwrap(), 42);
            assert_eq!(received.read_string().unwrap(), "hi");

            let mut reply = Storage::new();
            reply.write_double(2.5);
            conn.send_exact(&reply).unwrap();
        });

        let mut client = Socket::with_host("127.0.0.1", port);
        let mut connected = false;
        for _ in 0..200 {
            if client.connect().is_ok() {
                connected = true;
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert!(connected, "client could not connect to the test server");

        let mut out = Storage::new();
        out.write_int(42);
        out.write_string("hi");
        client.send_exact(&out).unwrap();

        let mut reply = Storage::new();
        assert!(client.receive_exact(&mut reply).unwrap());
        assert_eq!(reply.read_double().unwrap(), 2.5);

        server.join().unwrap();
    }
}
