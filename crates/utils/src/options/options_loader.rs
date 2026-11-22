//! Port of utils/options/OptionsLoader.{h,cpp}
//! The xerces SAX callbacks are driven by quick-xml in `parse_file` / `parse_str`.
use super::options_cont::OptionsCont;
use crate::common::msg_handler::{write_error, write_warning};
use crate::common::util_exceptions::ProcessError;
use crate::{write_errorf, write_warningf};
use quick_xml::events::Event;
use quick_xml::Reader;

/// Reproduce the xerces UTF-8 diagnostic for the first invalid byte (SUMO meta/umlaut).
fn xerces_utf8_error(bytes: &[u8], e: &std::str::Utf8Error) -> (String, u64) {
    let pos = e.valid_up_to();
    let lead = bytes.get(pos).copied().unwrap_or(0);
    let expected = if lead >= 0xF0 { 4 } else if lead >= 0xE0 { 3 } else if lead >= 0xC0 { 2 } else { 1 };
    let mut at = 1;
    while at < expected {
        let b = bytes.get(pos + at).copied().unwrap_or(0);
        if b & 0xC0 != 0x80 {
            break;
        }
        at += 1;
    }
    let bad = bytes.get(pos + at).copied().unwrap_or(0) as char;
    let msg = format!("invalid byte '{}' at position {} of a {}-byte sequence", bad, at + 1, expected);
    // xerces reports the offending line 0-based; error() prints line + 1.
    let line = bytes[..pos.min(bytes.len())].iter().filter(|&&b| b == b'\n').count() as u64;
    (msg, line.saturating_sub(1))
}

pub struct OptionsLoader {
    root_only: bool,
    error: bool,
    item: String,
    value: String,
    found_value: bool,
}

impl OptionsLoader {
    pub fn new(root_only: bool) -> Self {
        OptionsLoader { root_only, error: false, item: String::new(), value: String::new(), found_value: false }
    }

    pub fn start_element(&mut self, oc: &mut OptionsCont, name: &str, attributes: &[(String, String)]) {
        self.found_value = false;
        self.item = name.to_string();
        if !self.root_only {
            for (key, value) in attributes {
                if key == "value" || key == "v" {
                    let item = self.item.clone();
                    self.set_value(oc, &item, value);
                    self.found_value = true;
                } else if key != "xmlns:xsi"
                    && key != "xsi:noNamespaceSchemaLocation"
                    && key != "synonymes"
                    && key != "deprecated"
                    && key != "type"
                    && key != "help"
                    // parsing a network file as single argument
                    && (key != "version" && self.item != "net")
                {
                    write_warningf!("Ignoring attribute '%' for option '%'", key, self.item);
                }
            }
            self.value.clear();
        }
    }

    fn set_value(&mut self, oc: &mut OptionsCont, key: &str, value: &str) {
        if !value.is_empty() {
            // try to add value in option container
            match Self::set_secure(oc, key, value) {
                Ok(true) => {}
                Ok(false) => {
                    write_errorf!("Could not set option '%' (probably defined twice).", key);
                    self.error = true;
                }
                Err(e) => {
                    write_error(&e.to_string());
                    self.error = true;
                }
            }
        }
    }

    pub fn characters(&mut self, chars: &str) {
        self.value += chars;
    }

    fn set_secure(oc: &mut OptionsCont, name: &str, value: &str) -> Result<bool, ProcessError> {
        if oc.is_writeable(name)? {
            oc.set(name, value, false)?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn end_element(&mut self, oc: &mut OptionsCont, _name: &str) {
        if self.item.is_empty() || self.value.is_empty() {
            if !self.found_value {
                write_errorf!("Could not set option '%' because attribute 'value' is missing.", self.item);
            }
            return;
        }
        if !self.value.contains(|c| !"\n\t \x07".contains(c)) {
            return;
        }
        let (item, value) = (self.item.clone(), self.value.clone());
        self.set_value(oc, &item, &value);
        self.item.clear();
        self.value.clear();
    }

    pub fn warning(&mut self, msg: &str, line: u64, column: u64) {
        write_warning(msg);
        write_warning(&format!(" (At line/column {}/{}).", line + 1, column));
        self.error = true;
    }

    pub fn error(&mut self, msg: &str, line: u64, column: u64) {
        write_error(msg);
        write_error(&format!(" (At line/column {}/{}).", line + 1, column));
        self.error = true;
    }

    pub fn fatal_error(&mut self, msg: &str, line: u64, column: u64) {
        self.error(msg, line, column);
    }

    pub fn error_occurred(&self) -> bool {
        self.error
    }

    pub fn get_item(&self) -> &str {
        &self.item
    }

    /// SAXParser::parse(path); with `stop_at_root` it mimics the parseFirst/parseNext loop of
    /// OptionsIO::getRoot which stops once the root element was seen.
    /// Err only for files that cannot be read (xerces XMLException).
    pub fn parse_file(&mut self, oc: &mut OptionsCont, path: &str, stop_at_root: bool) -> Result<(), ProcessError> {
        let bytes = std::fs::read(path).map_err(|e| ProcessError::new(e.to_string()))?;
        match std::str::from_utf8(&bytes) {
            Ok(text) => self.parse_str(oc, text, stop_at_root),
            Err(e) => {
                // xerces rejects invalid UTF-8 while quick-xml only sees the lossy
                // replacement; reproduce the xerces message for the first bad byte.
                let (msg, line) = xerces_utf8_error(&bytes, &e);
                self.error(&msg, line, 1);
            }
        }
        Ok(())
    }

    pub fn parse_str(&mut self, oc: &mut OptionsCont, text: &str, stop_at_root: bool) {
        let mut reader = Reader::from_str(text);
        let line_col = |pos: u64| -> (u64, u64) {
            let before = &text.as_bytes()[..(pos as usize).min(text.len())];
            let line = before.iter().filter(|&&b| b == b'\n').count() as u64 + 1;
            let col = before.iter().rev().take_while(|&&b| b != b'\n').count() as u64 + 1;
            (line, col)
        };
        loop {
            let ev = reader.read_event();
            match ev {
                Ok(ref event @ (Event::Start(ref e) | Event::Empty(ref e))) => {
                    let is_empty = matches!(event, Event::Empty(_));
                    let name = e.name().as_ref().to_string();
                    let mut attrs = Vec::new();
                    for a in e.attributes() {
                        match a.map_err(quick_xml::Error::from).and_then(|a| {
                            let v = a.normalized_value(quick_xml::XmlVersion::Implicit1_0)?.into_owned();
                            Ok((a.key.as_ref().to_string(), v))
                        }) {
                            Ok(kv) => attrs.push(kv),
                            Err(err) => {
                                let (l, c) = line_col(reader.buffer_position());
                                self.fatal_error(&err.to_string(), l, c);
                                return;
                            }
                        }
                    }
                    self.start_element(oc, &name, &attrs);
                    if stop_at_root && !self.item.is_empty() {
                        return;
                    }
                    if is_empty {
                        self.end_element(oc, &name);
                    }
                }
                Ok(Event::End(e)) => {
                    let name = e.name().as_ref().to_string();
                    self.end_element(oc, &name);
                }
                Ok(Event::Text(t)) => self.characters(&t.xml10_content()),
                Ok(Event::CData(t)) => self.characters(&t.xml10_content()),
                Ok(Event::GeneralRef(r)) => {
                    let raw = format!("&{};", &*r);
                    match quick_xml::escape::unescape(&raw) {
                        Ok(s) => self.characters(&s),
                        Err(err) => {
                            let (l, c) = line_col(reader.buffer_position());
                            self.fatal_error(&err.to_string(), l, c);
                            return;
                        }
                    }
                }
                Ok(Event::Eof) => return,
                Ok(_) => {}
                Err(err) => {
                    let (l, c) = line_col(reader.error_position());
                    self.fatal_error(&err.to_string(), l, c);
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::options_cont::tests::sample;

    #[test]
    fn load_config() {
        let mut oc = sample();
        let mut l = OptionsLoader::new(false);
        l.parse_str(
            &mut oc,
            r#"<?xml version="1.0" encoding="UTF-8"?>
<configuration xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <!-- comment -->
    <input>
        <net-file value="a&amp;b.net.xml"/>
        <route-files v="r.xml"/>
        <begin>  12 </begin>
    </input>
    <report>
        <verbose value="true"/>
    </report>
</configuration>"#,
            false,
        );
        assert!(!l.error_occurred());
        assert_eq!(oc.get_string("net-file").unwrap(), "a&b.net.xml");
        assert_eq!(oc.get_string("route-files").unwrap(), "r.xml");
        assert_eq!(oc.get_string("begin").unwrap(), "  12 ");
        assert!(oc.get_bool("verbose").unwrap());
    }

    #[test]
    fn errors() {
        let mut oc = sample();
        let mut l = OptionsLoader::new(false);
        l.parse_str(&mut oc, r#"<c><unknown-opt value="1"/></c>"#, false);
        assert!(l.error_occurred());
        let mut l = OptionsLoader::new(false);
        l.parse_str(&mut oc, r#"<c><begin value="1"></c>"#, false);
        assert!(l.error_occurred());
    }

    #[test]
    fn xerces_utf8_diagnostic() {
        // meta/umlaut: 'n' 0xF6 't' -> xerces reports the bad continuation byte.
        let bytes = b"<?xml version=\"1.0\"?>\n<net-file value=\"n\xF6t.net.xml\"/>";
        let e = std::str::from_utf8(bytes).unwrap_err();
        let (msg, line) = super::xerces_utf8_error(bytes, &e);
        assert_eq!(msg, "invalid byte 't' at position 2 of a 4-byte sequence");
        assert_eq!(line, 0); // error() prints line + 1 == 1 (config line 1 here)
        // 2-byte and 3-byte leads with a bad continuation at the next position
        let b2 = b"x\xC3t";
        let e2 = std::str::from_utf8(b2).unwrap_err();
        assert_eq!(super::xerces_utf8_error(b2, &e2).0,
                   "invalid byte 't' at position 2 of a 2-byte sequence");
        let b3 = b"x\xE2\x82t";
        let e3 = std::str::from_utf8(b3).unwrap_err();
        assert_eq!(super::xerces_utf8_error(b3, &e3).0,
                   "invalid byte 't' at position 3 of a 3-byte sequence");
    }

    #[test]
    fn root() {
        let mut oc = sample();
        let mut l = OptionsLoader::new(false);
        l.parse_str(&mut oc, r#"<?xml version="1.0"?><net version="1.20"><edge id="a"/></net>"#, true);
        assert_eq!(l.get_item(), "net");
        assert!(!l.error_occurred());
    }
}
