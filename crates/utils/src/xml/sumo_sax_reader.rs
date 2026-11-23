//! Port of utils/xml/SUMOSAXReader.{h,cpp} (and IStreamInputSource.h) over quick-xml.
//! The handler is passed to each parse call instead of being stored (C++ setHandler).
// shortcut: no schema validation or DTD/entity handling (tests run with --xml-validation never);
// parse errors are worded and positioned like Xerces only for the cases measured against C++
// (tools/kaggle/agent_run.sh probes); other error states still use quick-xml's text. input must be UTF-8
use super::generic_sax_handler::{self, SaxHandler};
use super::stand_ins::{is_readable, write_warning, ProcessError};
use super::sumo_sax_attributes::SUMOSAXAttributes;
use super::sumo_xml_definitions::Tags;
use quick_xml::errors::{IllFormedError, SyntaxError};
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use crate::iodevices::zstr::open_read as open_input;
use std::io::{BufRead, Read};

/// Xerces reports errors at end of input at this position
const EOF_POS: u64 = u64::MAX;

pub struct SUMOSAXReader {
    my_validation_scheme: String,
    /// the progressive parse (myToken + myIStream)
    my_stream: Option<XmlStream>,
    my_next_section: Option<(i32, SUMOSAXAttributes)>,
}

enum Source {
    File(String),
    Str(String),
}

struct XmlStream {
    reader: Reader<Box<dyn BufRead>>,
    source: Source,
    buf: Vec<u8>,
    pending: Option<Event<'static>>,
    /// byte offset where the last event read from the reader started
    event_start: u64,
    /// names of the elements that are still open
    open: Vec<String>,
    root_seen: bool,
}

/// first reference `&name;` at or after `from` that is neither a character reference nor predefined:
/// (name, offset just past its ';')
fn first_undeclared_reference(data: &[u8], from: usize) -> Option<(String, usize)> {
    let mut i = from;
    while let Some(k) = data.get(i..)?.iter().position(|&c| c == b'&') {
        let amp = i + k;
        let semi = amp + data[amp..].iter().position(|&c| c == b';')?;
        let name = String::from_utf8_lossy(&data[amp + 1..semi]).into_owned();
        if !name.starts_with('#') && quick_xml::escape::resolve_predefined_entity(&name).is_none() {
            return Some((name, semi + 1));
        }
        i = semi + 1;
    }
    None
}

impl XmlStream {
    fn new(input: Box<dyn BufRead>, source: Source) -> Self {
        XmlStream { reader: Reader::from_reader(input), source, buf: Vec::new(), pending: None, event_start: 0, open: Vec::new(), root_seen: false }
    }

    /// the whole input (re-read from the file: only used on the error path)
    fn data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match &self.source {
            Source::Str(s) => data.extend_from_slice(s.as_bytes()),
            Source::File(f) => {
                if let Ok(r) = open_input(f) {
                    let _ = r.take(u64::MAX).read_to_end(&mut data);
                }
            }
        }
        data
    }

    /// 1-based line and column of byte offset `pos`
    fn line_col(&self, pos: u64) -> (u64, u64) {
        let data = self.data();
        let before = &data[..(pos as usize).min(data.len())];
        let line = before.iter().filter(|&&c| c == b'\n').count() as u64 + 1;
        let col = before.iter().rev().take_while(|&&c| c != b'\n').count() as u64 + 1;
        (line, col)
    }

    fn fatal(&self, h: &mut dyn SaxHandler, msg: &str, pos: u64) -> ProcessError {
        let (line, col) = self.line_col(pos);
        ProcessError::Process(h.handler().build_error_message(msg, line, col))
    }

    /// Xerces' "entity 'x' not found" for the first undeclared reference at or after `pos`, else `fallback`
    fn reference_error(&self, h: &mut dyn SaxHandler, pos: u64, fallback: &str) -> ProcessError {
        match first_undeclared_reference(&self.data(), pos as usize) {
            Some((name, end)) => self.fatal(h, &format!("entity '{name}' not found"), end as u64),
            None => self.fatal(h, fallback, pos),
        }
    }

    /// Xerces reports "equal sign expected" at the first token after an
    /// attribute name that is not followed by '=' (e.g. `<tag broken state>`).
    fn missing_equals_error(&self, h: &mut dyn SaxHandler, tag_pos: u64) -> Option<ProcessError> {
        let data = self.data();
        let start = tag_pos as usize;
        if data.get(start) != Some(&b'<') {
            return None;
        }
        let end = data[start..].iter().position(|&c| c == b'>').map(|i| start + i).unwrap_or(data.len());
        let tag = &data[start..end];
        let mut i = 1usize;
        // skip the element name
        while i < tag.len() && !tag[i].is_ascii_whitespace() {
            i += 1;
        }
        loop {
            while i < tag.len() && tag[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= tag.len() {
                return None;
            }
            let name_start = i;
            while i < tag.len() && !tag[i].is_ascii_whitespace() && tag[i] != b'=' {
                i += 1;
            }
            if i == name_start {
                return None;
            }
            while i < tag.len() && tag[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= tag.len() || tag[i] != b'=' {
                return Some(self.fatal(h, "equal sign expected", start as u64 + i as u64));
            }
            // quoted or unquoted value
            i += 1;
            while i < tag.len() && tag[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < tag.len() && (tag[i] == b'"' || tag[i] == b'\'') {
                let quote = tag[i];
                i += 1;
                while i < tag.len() && tag[i] != quote {
                    i += 1;
                }
                if i < tag.len() {
                    i += 1;
                }
            } else {
                while i < tag.len() && !tag[i].is_ascii_whitespace() {
                    i += 1;
                }
            }
        }
    }

    /// a quick-xml error, worded and positioned as Xerces reports it
    fn read_error(&self, h: &mut dyn SaxHandler, e: &quick_xml::Error) -> ProcessError {
        match e {
            quick_xml::Error::Syntax(SyntaxError::UnclosedTag) => {
                // Xerces names the state it was in when the input ended
                let data = self.data();
                let tail = data.get(self.event_start as usize..).unwrap_or_default().trim_ascii_end();
                let msg = match tail.last().copied() {
                    Some(b'=') => "attribute value expected",
                    Some(b'"' | b'\'') => "whitespace expected",
                    _ => "unexpected end of input", // shortcut: other Xerces states inside a tag are not measured
                };
                self.fatal(h, msg, EOF_POS)
            }
            quick_xml::Error::Syntax(SyntaxError::InvalidBangMarkup) => self.fatal(h, &e.to_string(), self.reader.error_position()),
            quick_xml::Error::Syntax(_) => self.fatal(h, "unexpected end of input", EOF_POS),
            quick_xml::Error::IllFormed(IllFormedError::MismatchedEndTag { expected, .. }) => {
                self.fatal(h, &format!("expected end of tag '{expected}'"), self.event_start + 2)
            }
            _ => self.fatal(h, &e.to_string(), self.reader.error_position()),
        }
    }

    fn next_event(&mut self, h: &mut dyn SaxHandler) -> Result<Event<'static>, ProcessError> {
        if let Some(e) = self.pending.take() {
            return Ok(e);
        }
        self.buf.clear();
        self.event_start = self.reader.buffer_position();
        match self.reader.read_event_into(&mut self.buf) {
            Ok(e) => Ok(e.into_owned()),
            Err(e) => Err(self.read_error(h, &e)),
        }
    }

    /// parseFirst: consumes the prolog, stops before the root element
    fn parse_prolog(&mut self, h: &mut dyn SaxHandler) -> Result<(), ProcessError> {
        loop {
            let e = self.next_event(h)?;
            match e {
                Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {}
                Event::Text(ref t) if t.trim_ascii().is_empty() => {}
                _ => {
                    self.pending = Some(e);
                    return Ok(());
                }
            }
        }
    }

    /// Processes the next token (parseNext); false at the end of the document.
    fn step(&mut self, h: &mut dyn SaxHandler) -> Result<bool, ProcessError> {
        let e = self.next_event(h)?;
        let pos = self.event_start;
        match e {
            Event::Start(ref s) | Event::Empty(ref s) => {
                if self.root_seen && self.open.is_empty() {
                    return Err(self.fatal(h, "comment or processing instruction expected", pos));
                }
                self.root_seen = true;
                let name = s.name().as_ref().to_string();
                let mut attrs = Vec::new();
                for a in s.attributes() {
                    let a = a.map_err(|err| {
                        if let Some(e) = self.missing_equals_error(h, pos) {
                            e
                        } else {
                            self.fatal(h, &err.to_string(), pos)
                        }
                    })?;
                    let v = a.normalized_value(XmlVersion::Implicit1_0).map_err(|err| self.reference_error(h, pos, &err.to_string()))?;
                    attrs.push((a.key.as_ref().to_string(), v.into_owned()));
                }
                generic_sax_handler::start_element(h, &name, attrs)?;
                if matches!(e, Event::Empty(_)) {
                    generic_sax_handler::end_element(h, &name)?;
                } else {
                    self.open.push(name);
                }
            }
            Event::End(ref s) => {
                self.open.pop();
                generic_sax_handler::end_element(h, s.name().as_ref())?;
            }
            Event::Text(ref t) => {
                if !self.open.is_empty() {
                    generic_sax_handler::characters(h, &t.xml10_content());
                } else if !t.trim_ascii().is_empty() {
                    let at = pos + (t.len() - t.trim_ascii_start().len()) as u64;
                    let msg = if self.root_seen { "comment or processing instruction expected" } else { "invalid document structure" };
                    return Err(self.fatal(h, msg, at));
                }
            }
            Event::CData(ref c) => {
                if !self.open.is_empty() {
                    generic_sax_handler::characters(h, &c.xml10_content());
                }
            }
            Event::GeneralRef(ref r) => {
                let resolved = match r.resolve_char_ref() {
                    Ok(Some(c)) => c.to_string(),
                    Ok(None) => match quick_xml::escape::resolve_predefined_entity(r.as_ref()) {
                        Some(s) => s.to_string(),
                        None => return Err(self.reference_error(h, pos, &format!("entity '{}' not found", r.as_ref()))),
                    },
                    Err(err) => return Err(self.fatal(h, &err.to_string(), pos)),
                };
                if !self.open.is_empty() {
                    generic_sax_handler::characters(h, &resolved);
                }
            }
            Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {}
            Event::Eof => {
                if !self.root_seen {
                    return Err(self.fatal(h, "invalid document structure", EOF_POS));
                }
                if let Some(last) = self.open.last() {
                    let msg = format!("input ended before all started tags were ended; last tag started is '{last}'");
                    return Err(self.fatal(h, &msg, EOF_POS));
                }
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl SUMOSAXReader {
    pub fn new(validation_scheme: &str) -> Self {
        SUMOSAXReader { my_validation_scheme: validation_scheme.to_string(), my_stream: None, my_next_section: None }
    }

    pub fn set_validation(&mut self, validation_scheme: &str) {
        // shortcut: the scheme is only stored, no validation happens
        if !validation_scheme.is_empty() {
            self.my_validation_scheme = validation_scheme.to_string();
        }
    }

    fn open(system_id: &str) -> Result<XmlStream, ProcessError> {
        if !is_readable(system_id) {
            return Err(ProcessError::IOError(format!("Cannot read file '{system_id}'!")));
        }
        if std::path::Path::new(system_id).is_dir() {
            return Err(ProcessError::IOError(format!("File '{system_id}' is a directory!")));
        }
        let input = open_input(system_id).map_err(|_| ProcessError::IOError(format!("Cannot read file '{system_id}'!")))?;
        Ok(XmlStream::new(input, Source::File(system_id.to_string())))
    }

    pub fn parse(&mut self, h: &mut dyn SaxHandler, system_id: &str) -> Result<(), ProcessError> {
        let mut stream = Self::open(system_id)?;
        while stream.step(h)? {}
        Ok(())
    }

    pub fn parse_string(&mut self, h: &mut dyn SaxHandler, content: &str) -> Result<(), ProcessError> {
        let mut stream = XmlStream::new(Box::new(std::io::Cursor::new(content.as_bytes().to_vec())), Source::Str(content.to_string()));
        while stream.step(h)? {}
        Ok(())
    }

    pub fn parse_first(&mut self, h: &mut dyn SaxHandler, system_id: &str) -> Result<bool, ProcessError> {
        let mut stream = Self::open(system_id)?;
        stream.parse_prolog(h)?;
        self.my_stream = Some(stream);
        Ok(true)
    }

    pub fn parse_next(&mut self, h: &mut dyn SaxHandler) -> Result<bool, ProcessError> {
        match &mut self.my_stream {
            None => Err(ProcessError::Process("The XML-parser was not initialized.".into())),
            Some(s) => s.step(h),
        }
    }

    pub fn parse_section(&mut self, h: &mut dyn SaxHandler, element: i32) -> Result<bool, ProcessError> {
        if self.my_stream.is_none() {
            return Err(ProcessError::Process("The XML-parser was not initialized.".into()));
        }
        let mut started = false;
        if let Some((first, attrs)) = self.my_next_section.take() {
            started = first == element;
            if !started {
                // This enforces that the next parsed section starts right after the last one.
                // If we want to skip sections we need to change this.
                write_warning(&format!("Expected different XML section '{}', some content may be missing.", Tags.get_string(element)));
            }
            h.my_start_element(first, &attrs)?;
        }
        h.handler().set_section(element, started);
        while !h.handler().section_finished() {
            if !self.parse_next(h)? {
                return Ok(false);
            }
        }
        self.my_next_section = h.handler().retrieve_next_section_start();
        Ok(true)
    }
}
