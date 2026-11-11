//! Port of utils/importio/LineReader.{h,cpp}
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::line_handler::LineHandler;

/// Size of the chunks read from the file
const CHUNK: usize = 1024;

/// Retrieves a file linewise and reports the lines to a handler.
///
/// Positions are kept as in C++ (int counters). The file is read in chunks of CHUNK bytes;
/// `my_read` is advanced by CHUNK per chunk, as upstream does.
//
// shortcut: lines are handed to the handler as &str, so invalid UTF-8 bytes are replaced with U+FFFD
#[derive(Debug, Default)]
pub struct LineReader {
    /// the name of the file to read the contents from
    my_file_name: String,
    /// the open file, None when no file is open (C++ stream not open)
    my_strm: Option<File>,
    /// set after a short read or a failed read (C++ fail/eof bits); good() is false then
    my_strm_failed: bool,
    /// bytes taken from the file but not yet reported
    my_str_buffer: Vec<u8>,
    /// bytes supplied to my_str_buffer (in chunks)
    my_read: i32,
    /// bytes available within the used file (after the BOM); -1 if the file could not be opened
    my_available: i32,
    /// bytes consumed from my_str_buffer, i.e. the reported position
    my_rread: i32,
    /// lines read, for meaningful error messages
    my_lines_read: i32,
    /// number of skipped characters at the file begin (UTF-8 BOM)
    my_skip_bom: i32,
}

impl LineReader {
    /// Constructor without a file; a later set_file is necessary
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor: initialises reading from the file with the given name
    pub fn with_file(file: &str) -> Self {
        let mut r = LineReader { my_file_name: file.to_string(), ..Default::default() };
        r.reinit();
        r
    }

    /// Returns whether another line may be read (the file was not read completely)
    pub fn has_more(&self) -> bool {
        self.my_rread < self.my_available
    }

    /// Reads the whole file linewise, reporting every line to the given handler.
    /// Reading stops when the handler returns false.
    pub fn read_all(&mut self, lh: &mut dyn LineHandler) {
        while self.my_rread < self.my_available {
            if !self.read_line_to(lh) {
                return;
            }
        }
    }

    /// Reads the next line from the file and reports it to the given handler.
    /// Returns whether a further line exists.
    pub fn read_line_to(&mut self, lh: &mut dyn LineHandler) -> bool {
        let mut to_report: Vec<u8> = Vec::new();
        let mut more_available = true;
        while to_report.is_empty() {
            match self.my_str_buffer.iter().position(|&b| b == b'\n') {
                Some(0) => {
                    self.my_str_buffer.drain(..1);
                    self.my_rread += 1;
                    return lh.report("");
                }
                Some(idx) => {
                    to_report = self.my_str_buffer[..idx].to_vec();
                    self.my_str_buffer.drain(..idx + 1);
                    self.my_rread += idx as i32 + 1;
                }
                None => {
                    if self.my_read < self.my_available {
                        self.fill_buffer();
                    } else {
                        to_report = self.my_str_buffer.clone();
                        more_available = false;
                        if to_report.is_empty() {
                            return lh.report("");
                        }
                    }
                }
            }
        }
        trim_trailing_blanks(&mut to_report);
        if !lh.report(&String::from_utf8_lossy(&to_report)) {
            return false;
        }
        more_available
    }

    /// Reads the next line from the file and returns it (without trailing blanks)
    pub fn read_line(&mut self) -> String {
        let mut to_report: Vec<u8> = Vec::new();
        while to_report.is_empty() && self.good() {
            match self.my_str_buffer.iter().position(|&b| b == b'\n') {
                Some(0) => {
                    self.my_str_buffer.drain(..1);
                    self.my_rread += 1;
                    self.my_lines_read += 1;
                    return String::new();
                }
                Some(idx) => {
                    to_report = self.my_str_buffer[..idx].to_vec();
                    self.my_str_buffer.drain(..idx + 1);
                    self.my_rread += idx as i32 + 1;
                }
                None => {
                    if self.my_read < self.my_available {
                        self.fill_buffer();
                    } else {
                        // upstream does not clear my_str_buffer here, so the rest is returned again
                        to_report = self.my_str_buffer.clone();
                        self.my_rread += self.my_str_buffer.len() as i32;
                        if to_report.is_empty() {
                            self.my_lines_read += 1;
                            return String::new();
                        }
                    }
                }
            }
        }
        if !self.good() {
            return String::new();
        }
        trim_trailing_blanks(&mut to_report);
        self.my_lines_read += 1;
        String::from_utf8_lossy(&to_report).into_owned()
    }

    /// Closes the reading
    pub fn close(&mut self) {
        // not defined upstream (declared only); closing drops the stream so good() becomes false
        self.my_strm = None;
    }

    /// The name of the used file
    pub fn get_file_name(&self) -> &str {
        &self.my_file_name
    }

    /// Reinitialises the reader for reading from the given file; false if it is not readable
    pub fn set_file(&mut self, file: &str) -> bool {
        self.my_file_name = file.to_string();
        self.reinit();
        self.good()
    }

    /// The current position within the file
    pub fn get_position(&self) -> u64 {
        self.my_rread as u64
    }

    /// Reinitialises the reading (of the previous file)
    pub fn reinit(&mut self) {
        let opened = File::open(&self.my_file_name).ok().and_then(|f| {
            let len = f.metadata().ok()?.len();
            Some((f, len as i32))
        });
        match opened {
            Some((f, len)) => {
                self.my_strm = Some(f);
                self.my_available = len;
            }
            None => {
                self.my_strm = None;
                self.my_available = -1;
            }
        }
        self.my_strm_failed = false;
        self.my_skip_bom = 0;
        if self.my_available >= 3 {
            // check for BOM
            let head = self.read_chunk(3);
            if head == [0xef, 0xbb, 0xbf] {
                self.my_skip_bom = 3;
                self.my_available -= self.my_skip_bom;
            } else {
                self.seek_stream(0);
            }
        }
        self.my_read = 0;
        self.my_rread = 0;
        self.my_str_buffer.clear();
        self.my_lines_read = 0;
    }

    /// Sets the current position within the file to the given value
    pub fn set_pos(&mut self, pos: u64) {
        self.seek_stream(pos + self.my_skip_bom as u64);
        self.my_read = pos as i32;
        self.my_rread = pos as i32;
        self.my_str_buffer.clear();
    }

    /// Whether the file is usable (C++ good())
    pub fn good(&self) -> bool {
        self.my_strm.is_some() && !self.my_strm_failed
    }

    /// The number of lines read so far
    pub fn get_line_number(&self) -> i32 {
        self.my_lines_read
    }

    /// Appends the next chunk (at most CHUNK bytes) to the string buffer.
    /// Like upstream, my_read advances by CHUNK even for a shorter last chunk.
    fn fill_buffer(&mut self) {
        let n = (self.my_available - self.my_read).min(CHUNK as i32) as usize;
        let chunk = self.read_chunk(n);
        self.my_str_buffer.extend_from_slice(&chunk);
        self.my_read += CHUNK as i32;
    }

    /// Reads up to `n` bytes from the stream; a short read marks the stream as failed
    fn read_chunk(&mut self, n: usize) -> Vec<u8> {
        let mut buf = vec![0u8; n];
        let mut got = 0;
        match self.my_strm.as_mut() {
            Some(f) => {
                while got < n {
                    match f.read(&mut buf[got..]) {
                        Ok(0) | Err(_) => break,
                        Ok(k) => got += k,
                    }
                }
            }
            None => {}
        }
        if got < n {
            self.my_strm_failed = true;
        }
        buf.truncate(got);
        buf
    }

    fn seek_stream(&mut self, pos: u64) {
        if let Some(f) = self.my_strm.as_mut() {
            if f.seek(SeekFrom::Start(pos)).is_err() {
                self.my_strm_failed = true;
            }
        }
    }
}

/// Removes trailing characters below 32. Like the signed `char` comparison upstream,
/// bytes >= 0x80 also count as blanks here.
fn trim_trailing_blanks(v: &mut Vec<u8>) {
    while let Some(&b) = v.last() {
        if (b as i8) < 32 {
            v.pop();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Collect(Vec<String>);

    impl LineHandler for Collect {
        fn report(&mut self, result: &str) -> bool {
            self.0.push(result.to_string());
            true
        }
    }

    /// Writes `content` to a fresh file in the temp dir and returns its path
    fn temp_file(name: &str, content: &[u8]) -> String {
        let path = std::env::temp_dir().join(format!("line_reader_{}_{}.txt", std::process::id(), name));
        std::fs::write(&path, content).unwrap();
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn read_all_reports_every_line() {
        let path = temp_file("all", b"a\n\nb  \r\nc");
        let mut r = LineReader::with_file(&path);
        assert!(r.good());
        assert!(r.has_more());
        let mut c = Collect(Vec::new());
        r.read_all(&mut c);
        // trailing blanks are removed, but a space (0x20) is kept
        assert_eq!(c.0, ["a", "", "b  ", "c"]);
        assert_eq!(r.get_file_name(), path);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn bom_is_skipped() {
        let path = temp_file("bom", b"\xef\xbb\xbfx\ny");
        let mut r = LineReader::with_file(&path);
        let mut c = Collect(Vec::new());
        r.read_all(&mut c);
        assert_eq!(c.0, ["x", "y"]);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn read_line_returns_lines_and_counts() {
        let path = temp_file("line", b"a\nb");
        let mut r = LineReader::with_file(&path);
        assert_eq!(r.read_line(), "a");
        assert_eq!(r.read_line(), "b");
        assert_eq!(r.get_line_number(), 2);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn missing_file_is_not_good() {
        let mut r = LineReader::with_file("this/file/does/not/exist.txt");
        assert!(!r.good());
        assert!(!r.has_more());
        assert_eq!(r.read_line(), "");
        assert!(!r.set_file("this/file/does/not/exist.txt"));
    }
}
