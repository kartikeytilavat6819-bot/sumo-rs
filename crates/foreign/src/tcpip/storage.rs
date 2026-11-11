//! Port of foreign/tcpip/storage.{h,cpp}
//!
//! Byte buffer for TraCI messages with a read position. Multi-byte values are big-endian on the
//! wire on every host (the C++ code reverses bytes on little-endian hosts), so the Rust code uses
//! `to_be_bytes`/`from_be_bytes` directly.
//!
//! Like the C++ code, every write resets the read position to the start (C++ reassigns `iter_`
//! to `store.begin()` after each write).
//!
//! C++ virtual methods are plain methods here, since the port has no subclasses.
//! C++ throws std::invalid_argument; that becomes `ProcessError::InvalidArgument` with the same text.

use utils::common::util_exceptions::ProcessError;

type Result<T> = std::result::Result<T, ProcessError>;

fn invalid(msg: impl Into<String>) -> ProcessError {
    ProcessError::InvalidArgument(msg.into())
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Storage {
    store: Vec<u8>,
    /// C++ iter_: index of the next byte to read
    pos: usize,
}

impl Storage {
    /// C++ Storage()
    pub fn new() -> Self {
        Self::default()
    }

    /// C++ Storage(const unsigned char[], int length): copies the given bytes.
    /// (The C++ default length of -1 is not supported: its assert fails, so it is not ported.)
    pub fn from_bytes(packet: &[u8]) -> Self {
        Storage {
            store: packet.to_vec(),
            pos: 0,
        }
    }

    /// C++ valid_pos
    pub fn valid_pos(&self) -> bool {
        self.pos < self.store.len()
    }

    /// C++ position
    pub fn position(&self) -> usize {
        self.pos
    }

    /// C++ reset: clears the content and the read position.
    pub fn reset(&mut self) {
        self.store.clear();
        self.pos = 0;
    }

    /// C++ resetPos
    pub fn reset_pos(&mut self) {
        self.pos = 0;
    }

    /// C++ size
    pub fn size(&self) -> usize {
        self.store.len()
    }

    /// C++ begin()/end(): the whole content, regardless of the read position.
    pub fn as_bytes(&self) -> &[u8] {
        &self.store
    }

    /// C++ hexDump: bytes as two-digit lowercase hex, separated by spaces.
    pub fn hex_dump(&self) -> String {
        self.store.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
    }

    /// C++ checkReadSafe
    fn check_read_safe(&self, num: usize) -> Result<()> {
        let remaining = self.store.len() - self.pos;
        if remaining < num {
            return Err(invalid(format!(
                "tcpip::Storage::readIsSafe: want to read {num} bytes from Storage, but only {remaining} remaining"
            )));
        }
        Ok(())
    }

    /// Reads exactly K bytes (C++ readByEndianess, with the wire order already applied by the caller).
    fn read_array<const K: usize>(&mut self) -> Result<[u8; K]> {
        self.check_read_safe(K)?;
        let mut out = [0u8; K];
        out.copy_from_slice(&self.store[self.pos..self.pos + K]);
        self.pos += K;
        Ok(out)
    }

    /// Appends bytes and resets the read position, as every C++ write does.
    fn write_raw(&mut self, bytes: &[u8]) {
        self.store.extend_from_slice(bytes);
        self.pos = 0;
    }

    /// C++ readChar
    pub fn read_char(&mut self) -> Result<u8> {
        if !self.valid_pos() {
            return Err(invalid("Storage::readChar(): invalid position"));
        }
        let value = self.store[self.pos];
        self.pos += 1;
        Ok(value)
    }

    /// C++ writeChar
    pub fn write_char(&mut self, value: u8) {
        self.write_raw(&[value]);
    }

    /// C++ readByte: signed byte in [-128, 127]
    pub fn read_byte(&mut self) -> Result<i32> {
        let i = i32::from(self.read_char()?);
        Ok(if i < 128 { i } else { i - 256 })
    }

    /// C++ writeByte
    pub fn write_byte(&mut self, value: i32) -> Result<()> {
        if !(-128..=127).contains(&value) {
            return Err(invalid("Storage::writeByte(): Invalid value, not in [-128, 127]"));
        }
        self.write_char(((value + 256) % 256) as u8);
        Ok(())
    }

    /// C++ readUnsignedByte
    pub fn read_unsigned_byte(&mut self) -> Result<i32> {
        Ok(i32::from(self.read_char()?))
    }

    /// C++ writeUnsignedByte
    pub fn write_unsigned_byte(&mut self, value: i32) -> Result<()> {
        if !(0..=255).contains(&value) {
            return Err(invalid("Storage::writeUnsignedByte(): Invalid value, not in [0, 255]"));
        }
        self.write_char(value as u8);
        Ok(())
    }

    /// C++ readString: int length, then the bytes.
    /// shortcut: C++ returns raw bytes in a std::string; here the bytes must be valid UTF-8 (TraCI strings are).
    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_int()?;
        if len < 0 {
            return Err(invalid(format!("tcpip::Storage::readString(): negative length {len}")));
        }
        let len = len as usize;
        self.check_read_safe(len)?;
        let bytes = self.store[self.pos..self.pos + len].to_vec();
        self.pos += len;
        String::from_utf8(bytes).map_err(|e| invalid(format!("tcpip::Storage::readString(): invalid UTF-8: {e}")))
    }

    /// C++ writeString: int length, then the bytes.
    pub fn write_string(&mut self, s: &str) {
        self.write_int(s.len() as i32);
        self.write_raw(s.as_bytes());
    }

    /// C++ readStringList: int count, then that many strings.
    pub fn read_string_list(&mut self) -> Result<Vec<String>> {
        let len = self.read_int()?;
        if len < 0 {
            return Err(invalid(format!("tcpip::Storage::readStringList(): negative count {len}")));
        }
        let mut tmp = Vec::new();
        for _ in 0..len {
            tmp.push(self.read_string()?);
        }
        Ok(tmp)
    }

    /// C++ writeStringList
    pub fn write_string_list(&mut self, s: &[String]) {
        self.write_int(s.len() as i32);
        for item in s {
            self.write_string(item);
        }
    }

    /// C++ readDoubleList: int count, then that many doubles.
    pub fn read_double_list(&mut self) -> Result<Vec<f64>> {
        let len = self.read_int()?;
        if len < 0 {
            return Err(invalid(format!("tcpip::Storage::readDoubleList(): negative count {len}")));
        }
        let mut tmp = Vec::new();
        for _ in 0..len {
            tmp.push(self.read_double()?);
        }
        Ok(tmp)
    }

    /// C++ writeDoubleList
    pub fn write_double_list(&mut self, s: &[f64]) {
        self.write_int(s.len() as i32);
        for value in s {
            self.write_double(*value);
        }
    }

    /// C++ readShort: 2 bytes, big-endian, signed
    pub fn read_short(&mut self) -> Result<i32> {
        Ok(i32::from(i16::from_be_bytes(self.read_array::<2>()?)))
    }

    /// C++ writeShort
    pub fn write_short(&mut self, value: i32) -> Result<()> {
        if !(-32768..=32767).contains(&value) {
            return Err(invalid("Storage::writeShort(): Invalid value, not in [-32768, 32767]"));
        }
        self.write_raw(&(value as i16).to_be_bytes());
        Ok(())
    }

    /// C++ readInt: 4 bytes, big-endian, signed
    pub fn read_int(&mut self) -> Result<i32> {
        Ok(i32::from_be_bytes(self.read_array::<4>()?))
    }

    /// C++ writeInt
    pub fn write_int(&mut self, value: i32) {
        self.write_raw(&value.to_be_bytes());
    }

    /// C++ readFloat: 4 bytes, big-endian IEEE 754
    pub fn read_float(&mut self) -> Result<f32> {
        Ok(f32::from_be_bytes(self.read_array::<4>()?))
    }

    /// C++ writeFloat
    pub fn write_float(&mut self, value: f32) {
        self.write_raw(&value.to_be_bytes());
    }

    /// C++ readDouble: 8 bytes, big-endian IEEE 754
    pub fn read_double(&mut self) -> Result<f64> {
        Ok(f64::from_be_bytes(self.read_array::<8>()?))
    }

    /// C++ writeDouble
    pub fn write_double(&mut self, value: f64) {
        self.write_raw(&value.to_be_bytes());
    }

    /// C++ writePacket: appends raw bytes.
    pub fn write_packet(&mut self, packet: &[u8]) {
        self.write_raw(packet);
    }

    /// C++ writeStorage: appends the unread part of `other` (from its read position to its end).
    pub fn write_storage(&mut self, other: &Storage) {
        self.write_raw(&other.store[other.pos..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_are_big_endian_on_the_wire() {
        let mut s = Storage::new();
        s.write_int(0x0102_0304);
        s.write_short(-2).unwrap();
        assert_eq!(s.as_bytes(), &[1, 2, 3, 4, 0xff, 0xfe]);
        assert_eq!(s.hex_dump(), "01 02 03 04 ff fe");
    }

    #[test]
    fn write_resets_read_position_and_reads_back() {
        let mut s = Storage::new();
        s.write_int(7);
        s.write_double(1.5);
        assert_eq!(s.position(), 0);
        assert_eq!(s.read_int().unwrap(), 7);
        assert_eq!(s.read_double().unwrap(), 1.5);
        assert!(!s.valid_pos());
    }

    #[test]
    fn string_is_length_prefixed() {
        let mut s = Storage::new();
        s.write_string("abc");
        assert_eq!(s.as_bytes(), &[0, 0, 0, 3, b'a', b'b', b'c']);
        assert_eq!(s.read_string().unwrap(), "abc");
    }

    #[test]
    fn byte_ranges_are_checked() {
        let mut s = Storage::new();
        assert!(s.write_byte(-129).is_err());
        assert!(s.write_byte(128).is_err());
        s.write_byte(-1).unwrap();
        assert_eq!(s.as_bytes(), &[0xff]);
        assert_eq!(s.read_byte().unwrap(), -1);
        assert!(s.write_unsigned_byte(256).is_err());
        assert!(s.write_short(40000).is_err());
    }

    #[test]
    fn short_read_reports_remaining_bytes() {
        let mut s = Storage::from_bytes(&[0, 0, 1]);
        let err = s.read_int().unwrap_err();
        assert_eq!(
            err.to_string(),
            "tcpip::Storage::readIsSafe: want to read 4 bytes from Storage, but only 3 remaining"
        );
    }

    #[test]
    fn write_storage_appends_unread_part_of_other() {
        let mut other = Storage::from_bytes(&[9, 8, 7, 6]);
        other.read_char().unwrap();
        let mut s = Storage::new();
        s.write_storage(&other);
        assert_eq!(s.as_bytes(), &[8, 7, 6]);
    }

    #[test]
    fn lists_round_trip() {
        let mut s = Storage::new();
        s.write_string_list(&["ab".to_string(), String::new()]);
        s.write_double_list(&[0.25, -3.0]);
        assert_eq!(s.read_string_list().unwrap(), vec!["ab".to_string(), String::new()]);
        assert_eq!(s.read_double_list().unwrap(), vec![0.25, -3.0]);
    }
}
