//! Port of utils/common/StringTokenizer.{h,cpp}
// shortcut: the templated tokenizeAndLookup is not ported, add it when a caller needs it
use super::util_exceptions::ProcessError;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct StringTokenizer {
    tosplit: String,
    pos: usize,
    starts: Vec<usize>,
    lengths: Vec<usize>,
}

impl StringTokenizer {
    /// split at newlines ("\r\n", all chars)
    pub const NEWLINE: i32 = -256;
    /// split at all chars <= SPACE
    pub const WHITECHARS: i32 = -257;
    pub const SPACE: i32 = 32;
    pub const TAB: i32 = 9;

    /// StringTokenizer(tosplit): splits at whitespace
    pub fn new(tosplit: &str) -> Self {
        let mut t = Self { tosplit: tosplit.to_string(), ..Default::default() };
        t.prepare_whitechar(tosplit);
        t
    }

    /// StringTokenizer(tosplit, token, splitAtAllChars)
    pub fn with_token(tosplit: &str, token: &str, split_at_all_chars: bool) -> Self {
        let mut t = Self { tosplit: tosplit.to_string(), ..Default::default() };
        t.prepare(tosplit, token, split_at_all_chars);
        t
    }

    /// StringTokenizer(tosplit, special)
    pub fn with_special(tosplit: &str, special: i32) -> Self {
        let mut t = Self { tosplit: tosplit.to_string(), ..Default::default() };
        match special {
            Self::NEWLINE => t.prepare(tosplit, "\r\n", true),
            Self::TAB => t.prepare(tosplit, "\t", true),
            Self::WHITECHARS => t.prepare_whitechar(tosplit),
            _ => {
                let c = char::from(special as u8);
                t.prepare(tosplit, &c.to_string(), false)
            }
        }
        t
    }

    pub fn reinit(&mut self) {
        self.pos = 0;
    }

    pub fn has_next(&self) -> bool {
        self.pos != self.starts.len()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<String, ProcessError> {
        if self.pos >= self.starts.len() {
            return Err(ProcessError::out_of_bounds());
        }
        if self.lengths[self.pos] == 0 {
            self.pos += 1;
            return Ok(String::new());
        }
        let start = self.starts[self.pos];
        let length = self.lengths[self.pos];
        self.pos += 1;
        Ok(self.tosplit[start..start + length].to_string())
    }

    pub fn front(&self) -> Result<String, ProcessError> {
        if self.starts.is_empty() {
            return Err(ProcessError::out_of_bounds());
        }
        self.get(0)
    }

    pub fn get(&self, pos: usize) -> Result<String, ProcessError> {
        if pos >= self.starts.len() {
            return Err(ProcessError::out_of_bounds());
        }
        if self.lengths[pos] == 0 {
            return Ok(String::new());
        }
        let start = self.starts[pos];
        Ok(self.tosplit[start..start + self.lengths[pos]].to_string())
    }

    pub fn size(&self) -> usize {
        self.starts.len()
    }

    fn prepare(&mut self, tosplit: &str, token: &str, split_at_all_chars: bool) {
        let mut beg = 0usize;
        let mut len = token.len();
        while beg < tosplit.len() {
            let end = if split_at_all_chars {
                // len is the byte length of the matched char (C++: 1)
                match tosplit[beg..].char_indices().find(|(_, c)| token.contains(*c)) {
                    Some((i, c)) => {
                        len = c.len_utf8();
                        beg + i
                    }
                    None => tosplit.len(),
                }
            } else {
                // C++ std::string::find("") returns beg, which loops forever; avoid that
                match tosplit[beg..].find(token).filter(|_| !token.is_empty()) {
                    Some(i) => beg + i,
                    None => tosplit.len(),
                }
            };
            self.starts.push(beg);
            self.lengths.push(end - beg);
            beg = end + len;
            if beg == tosplit.len() {
                self.starts.push(beg - 1);
                self.lengths.push(0);
            }
        }
    }

    fn prepare_whitechar(&mut self, tosplit: &str) {
        let b = tosplit.as_bytes();
        let len = b.len();
        let mut beg = 0usize;
        while beg < len && b[beg] <= Self::SPACE as u8 {
            beg += 1;
        }
        while beg < len {
            let mut end = beg;
            while end < len && b[end] > Self::SPACE as u8 {
                end += 1;
            }
            self.starts.push(beg);
            self.lengths.push(end - beg);
            beg = end;
            while beg < len && b[beg] <= Self::SPACE as u8 {
                beg += 1;
            }
        }
    }

    pub fn get_vector(&mut self) -> Vec<String> {
        let mut ret = Vec::with_capacity(self.size());
        while self.has_next() {
            ret.push(self.next().unwrap());
        }
        self.reinit();
        ret
    }

    pub fn get_set(&mut self) -> BTreeSet<String> {
        self.get_vector().into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace() {
        assert_eq!(StringTokenizer::new("  a b\t\nc  ").get_vector(), ["a", "b", "c"]);
        assert!(StringTokenizer::new("   ").get_vector().is_empty());
    }

    #[test]
    fn token() {
        assert_eq!(StringTokenizer::with_token("a,b,,c", ",", false).get_vector(), ["a", "b", "", "c"]);
        // trailing separator yields an empty token
        assert_eq!(StringTokenizer::with_token("a,b,", ",", false).get_vector(), ["a", "b", ""]);
        assert_eq!(StringTokenizer::with_token("a::b", "::", false).get_vector(), ["a", "b"]);
        assert_eq!(StringTokenizer::with_token("a;b,c", ",;", true).get_vector(), ["a", "b", "c"]);
        assert!(StringTokenizer::with_token("", ",", false).get_vector().is_empty());
        assert_eq!(StringTokenizer::with_token(",", ",", false).get_vector(), ["", ""]);
    }

    #[test]
    fn special() {
        assert_eq!(StringTokenizer::with_special("a\r\nb", StringTokenizer::NEWLINE).get_vector(), ["a", "", "b"]);
        assert_eq!(StringTokenizer::with_special("a\tb", StringTokenizer::TAB).get_vector(), ["a", "b"]);
        assert_eq!(StringTokenizer::with_special("a b", StringTokenizer::SPACE).get_vector(), ["a", "b"]);
    }

    #[test]
    fn access() {
        let mut st = StringTokenizer::with_token("x y", " ", false);
        assert_eq!(st.size(), 2);
        assert_eq!(st.front().unwrap(), "x");
        assert_eq!(st.get(1).unwrap(), "y");
        assert_eq!(st.next().unwrap(), "x");
        assert_eq!(st.next().unwrap(), "y");
        assert!(!st.has_next());
        assert_eq!(st.next().unwrap_err().to_string(), "Out Of Bounds");
    }
}
