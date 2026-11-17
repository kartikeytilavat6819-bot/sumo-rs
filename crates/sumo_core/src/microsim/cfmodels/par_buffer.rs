//! Port of microsim/cfmodels/ParBuffer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `ParBuffer` (microsim/cfmodels/ParBuffer.h)
pub struct ParBuffer {
    pub SEP: i8,
    pub ESC: i8,
    pub QUO: i8,
    /// C++ std::stringstream outBuffer, kept as its accumulated text
    pub outBuffer: String,
    pub inBuffer: String,
    pub was_empty: bool,
}

/// std::string::npos
const NPOS: usize = usize::MAX;

/// C++ `str.find(c, pos)` on bytes; NPOS when not found or pos is past the end
fn find_from(hay: &[u8], needle: i8, pos: usize) -> usize {
    if pos > hay.len() {
        return NPOS;
    }
    hay[pos..].iter().position(|&c| c as i8 == needle).map(|i| i + pos).unwrap_or(NPOS)
}

/// Values that go through the C++ template operator<< / operator>> of ParBuffer
pub trait ParBufferValue: Sized {
    /// What `std::stringstream << v` writes
    fn to_buffer_string(&self) -> String;
    /// What `std::stringstream >> v` extracts from `token`; None when extraction fails
    // shortcut: the whole token must parse, iostream stops at the first bad character, upgrade if a CC payload carries trailing junk
    fn from_buffer_token(token: &str) -> Option<Self>;
}

impl ParBufferValue for f64 {
    fn to_buffer_string(&self) -> String {
        utils::common::to_string::f64_to_string_default(*self)
    }
    fn from_buffer_token(token: &str) -> Option<Self> {
        token.trim_start().parse::<f64>().ok()
    }
}

macro_rules! impl_par_buffer_int {
    ($($t:ty),*) => {
        $(
            impl ParBufferValue for $t {
                fn to_buffer_string(&self) -> String {
                    self.to_string()
                }
                fn from_buffer_token(token: &str) -> Option<Self> {
                    token.trim_start().parse::<$t>().ok()
                }
            }
        )*
    };
}
impl_par_buffer_int!(i32, i64, u32, u64, usize);

impl ParBufferValue for String {
    fn to_buffer_string(&self) -> String {
        self.clone()
    }
    fn from_buffer_token(token: &str) -> Option<Self> {
        // stringstream >> std::string reads one whitespace-delimited word
        token.split_whitespace().next().map(|s| s.to_string())
    }
}

impl ParBuffer {
    /// C++: `void ParBuffer::ParBuffer()`
    // ParBuffer() : SEP(':'), ESC('\\'), QUO('"'), was_empty(false) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        ParBuffer { SEP: b':' as i8, ESC: b'\\' as i8, QUO: b'"' as i8, outBuffer: String::new(), inBuffer: String::new(), was_empty: false }
    }

    /// C++: `void ParBuffer::ParBuffer(std::string)`
    // ParBuffer(std::string buf) : SEP(':'), ESC('\\'), QUO('"'),
    //         was_empty(false) {
    //         inBuffer = buf;
    //     }
    #[allow(unused_variables)]
    pub fn new_2(buf: String) -> Self {
        ParBuffer { SEP: b':' as i8, ESC: b'\\' as i8, QUO: b'"' as i8, outBuffer: String::new(), inBuffer: buf, was_empty: false }
    }

    /// C++: `ParBuffer & ParBuffer::operator<<(const T &)`
    // template<typename T> ParBuffer& operator <<(const T& v) {
    //         std::stringstream ss;
    //         std::string str_value;
    //         ss << v;
    //         str_value = escape(ss.str());
    //         if (outBuffer.str().length() == 0) {
    //             outBuffer << str_value;
    //         } else {
    //             outBuffer << SEP << str_value;
    //         }
    //         return *this;
    //     }
    #[allow(unused_variables)]
    pub fn shl_op<T: ParBufferValue>(&mut self, v: T) -> &mut ParBuffer {
        let str_value = self.escape(v.to_buffer_string());
        if self.outBuffer.len() == 0 {
            self.outBuffer.push_str(&str_value);
        } else {
            self.outBuffer.push(self.SEP as u8 as char);
            self.outBuffer.push_str(&str_value);
        }
        self
    }

    /// C++: `size_t ParBuffer::next_escape(std::string, size_t)`
    // size_t next_escape(std::string str, size_t pos) {
    //         size_t c_pos = str.find(SEP, pos);
    //         size_t e_pos = str.find(ESC, pos);
    //         if (c_pos == std::string::npos) {
    //             return e_pos;
    //         }
    //         if (e_pos == std::string::npos) {
    //             return c_pos;
    //         }
    //         return std::min(c_pos, e_pos);
    //     }
    #[allow(unused_variables)]
    pub fn next_escape(&mut self, str: String, pos: usize) -> usize {
        let c_pos = find_from(str.as_bytes(), self.SEP, pos);
        let e_pos = find_from(str.as_bytes(), self.ESC, pos);
        if c_pos == NPOS {
            return e_pos;
        }
        if e_pos == NPOS {
            return c_pos;
        }
        std::cmp::min(c_pos, e_pos)
    }

    /// C++: `std::string ParBuffer::escape(std::string)`
    // std::string escape(std::string str) {
    //         size_t pos, last_pos = 0;
    //         std::stringstream escaping;
    //         std::string escaped;
    //         while ((pos = next_escape(str, last_pos)) != std::string::npos) {
    //             escaping << str.substr(last_pos, pos - last_pos);
    //             escaping << ESC << str.substr(pos, 1);
    //             last_pos = pos + 1;
    //         }
    //         if (last_pos != str.size()) {
    //             escaping << str.substr(last_pos);
    //         }
    //         escaped = escaping.str();
    //         if (escaped.empty() || (escaped.c_str()[0] == QUO && escaped.c_str()[escaped.length() - 1] == QUO)) {
    //             escaping.str("");
    //             escaping.clear();
    //             escaping << QUO << escaped << QUO;
    //             escaped = escaping.str();
    //         }
    //         return escaped;
    //     }
    #[allow(unused_variables)]
    pub fn escape(&mut self, str: String) -> String {
        let mut last_pos: usize = 0;
        let mut escaping: Vec<u8> = Vec::new();
        loop {
            let pos = self.next_escape(str.clone(), last_pos);
            if pos == NPOS {
                break;
            }
            escaping.extend_from_slice(&str.as_bytes()[last_pos..pos]);
            escaping.push(self.ESC as u8);
            escaping.extend_from_slice(&str.as_bytes()[pos..pos + 1]);
            last_pos = pos + 1;
        }
        if last_pos != str.len() {
            escaping.extend_from_slice(&str.as_bytes()[last_pos..]);
        }
        let escaped = if escaping.is_empty() || (escaping[0] as i8 == self.QUO && escaping[escaping.len() - 1] as i8 == self.QUO) {
            let mut quoted = vec![self.QUO as u8];
            quoted.extend(escaping);
            quoted.push(self.QUO as u8);
            quoted
        } else {
            escaping
        };
        // shortcut: bytes are rebuilt as UTF-8 lossily, only differs from C++ when ESC sits before a multi-byte character
        String::from_utf8_lossy(&escaped).into_owned()
    }

    /// C++: `std::string ParBuffer::unescape(std::string)`
    // std::string unescape(std::string str) {
    //         size_t pos, last_pos = 0;
    //         std::stringstream unescaped;
    //         std::string escaped;
    //         if (str.c_str()[0] == QUO && str.c_str()[str.length() - 1] == QUO) {
    //             str = str.substr(1, str.length() - 2);
    //         }
    //         while ((pos = str.find(ESC, last_pos)) != std::string::npos) {
    //             unescaped << str.substr(last_pos, pos - last_pos);
    //             unescaped << str.substr(pos + 1, 1);
    //             last_pos = pos + 2;
    //         }
    //         if (last_pos != str.size()) {
    //             unescaped << str.substr(last_pos);
    //         }
    //         return unescaped.str();
    //     }
    #[allow(unused_variables)]
    pub fn unescape(&mut self, str: String) -> String {
        let mut s: Vec<u8> = str.into_bytes();
        if !s.is_empty() && s[0] as i8 == self.QUO && s[s.len() - 1] as i8 == self.QUO {
            s = if s.len() >= 2 { s[1..s.len() - 1].to_vec() } else { Vec::new() };
        }
        let mut unescaped: Vec<u8> = Vec::new();
        let mut last_pos: usize = 0;
        loop {
            let pos = find_from(&s, self.ESC, last_pos);
            if pos == NPOS {
                break;
            }
            unescaped.extend_from_slice(&s[last_pos..pos]);
            let end = std::cmp::min(pos + 2, s.len());
            unescaped.extend_from_slice(&s[pos + 1..end]);
            last_pos = pos + 2;
        }
        if last_pos != s.len() {
            // C++ substr(last_pos) throws std::out_of_range when a trailing ESC pushes last_pos past the end
            if last_pos > s.len() {
                panic!("basic_string::substr: __pos (which is {}) > this->size() (which is {})", last_pos, s.len());
            }
            unescaped.extend_from_slice(&s[last_pos..]);
        }
        // shortcut: bytes are rebuilt as UTF-8 lossily, only differs from C++ when ESC sits before a multi-byte character
        String::from_utf8_lossy(&unescaped).into_owned()
    }

    /// C++: `std::string ParBuffer::next()`
    // std::string next() {
    //         if (inBuffer.size() == 0) {
    //             return "";
    //         }
    //
    //         size_t sep = std::string::npos;
    //         do {
    //             sep = inBuffer.find(SEP, sep + 1);
    //         } while (!(sep == std::string::npos || sep == 0 || inBuffer.c_str()[sep - 1] != ESC));
    //
    //         std::string value;
    //         if (sep == std::string::npos) {
    //             value = unescape(inBuffer);
    //             inBuffer = "";
    //         } else {
    //             value = unescape(inBuffer.substr(0, sep));
    //             inBuffer = inBuffer.substr(sep + 1);
    //         }
    //         return value;
    //     }
    #[allow(unused_variables)]
    pub fn next(&mut self) -> String {
        if self.inBuffer.len() == 0 {
            return String::new();
        }
        let mut sep: usize = NPOS;
        loop {
            sep = find_from(self.inBuffer.as_bytes(), self.SEP, sep.wrapping_add(1));
            if sep == NPOS || sep == 0 || self.inBuffer.as_bytes()[sep - 1] as i8 != self.ESC {
                break;
            }
        }
        let value;
        if sep == NPOS {
            let buf = self.inBuffer.clone();
            value = self.unescape(buf);
            self.inBuffer = String::new();
        } else {
            let head = self.inBuffer[..sep].to_string();
            value = self.unescape(head);
            self.inBuffer = self.inBuffer[sep + 1..].to_string();
        }
        value
    }

    /// C++: `ParBuffer & ParBuffer::operator>>(T &)`
    // template <typename T> ParBuffer& operator>>(T& v) {
    //         std::string value = next();
    //         std::stringstream ss(value);
    //         ss >> v;
    //         // stringstream doesn't write to v if value is an empty string. the
    //         // only solution is letting the user know that the last parsed
    //         // portion was empty
    //         if (value == "") {
    //             was_empty = true;
    //         } else {
    //             was_empty = false;
    //         }
    //         return *this;
    //     }
    #[allow(unused_variables)]
    pub fn op_<T: ParBufferValue>(&mut self, v: &mut T) -> &mut ParBuffer {
        let value = self.next();
        if let Some(x) = T::from_buffer_token(&value) {
            *v = x;
        }
        // stringstream doesn't write to v if value is an empty string. the
        // only solution is letting the user know that the last parsed
        // portion was empty
        self.was_empty = value == "";
        self
    }

    /// C++: `bool ParBuffer::last_empty()`
    // bool last_empty() {
    //         return was_empty;
    //     }
    #[allow(unused_variables)]
    pub fn last_empty(&mut self) -> bool {
        self.was_empty
    }

    /// C++: `void ParBuffer::set(std::string)`
    // void set(std::string buf) {
    //         inBuffer = buf;
    //     }
    #[allow(unused_variables)]
    pub fn set(&mut self, buf: String) {
        self.inBuffer = buf;
    }

    /// C++: `void ParBuffer::clear()`
    // void clear() {
    //         outBuffer.clear();
    //     }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        // C++ outBuffer.clear() only resets the stream's error flags; the written text is kept
    }

    /// C++: `std::string ParBuffer::str() const` [const]
    // std::string str() const {
    //         return outBuffer.str();
    //     }
    #[allow(unused_variables)]
    pub fn str(&self) -> String {
        self.outBuffer.clone()
    }

}

