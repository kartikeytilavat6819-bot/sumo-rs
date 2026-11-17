//! Port of utils/xml/SUMOSAXReader.{h,cpp} (and IStreamInputSource.h) over quick-xml.
//! The handler is passed to each parse call instead of being stored (C++ setHandler).
// shortcut: no schema validation or DTD/entity handling (tests run with --xml-validation never);
// parse error texts come from quick-xml, not Xerces; input must be UTF-8
use super::generic_sax_handler::{self, SaxHandler};
use super::stand_ins::{is_readable, write_warning, ProcessError};
use super::sumo_sax_attributes::SUMOSAXAttributes;
use super::sumo_xml_definitions::Tags;
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use crate::iodevices::zstr::open_read as open_input;
use std::io::{BufRead, Read};

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
    depth: usize,
    root_seen: bool,
}

impl XmlStream {
    fn new(input: Box<dyn BufRead>, source: Source) -> Self {
        XmlStream { reader: Reader::from_reader(input), source, buf: Vec::new(), pending: None, depth: 0, root_seen: false }
    }

    /// 1-based line and column of byte offset `pos`
    fn line_col(&self, pos: u64) -> (u64, u64) {
        let mut data = Vec::new();
        match &self.source {
            Source::Str(s) => data.extend_from_slice(s.as_bytes()),
            Source::File(f) => {
                if let Ok(r) = open_input(f) {
                    let _ = r.take(pos).read_to_end(&mut data);
                }
            }
        }
        let before = &data[..(pos as usize).min(data.len())];
        let line = before.iter().filter(|&&c| c == b'\n').count() as u64 + 1;
        let col = before.iter().rev().take_while(|&&c| c != b'\n').count() as u64 + 1;
        (line, col)
    }

    fn fatal(&self, h: &mut dyn SaxHandler, msg: &str, pos: u64) -> ProcessError {
        let (line, col) = self.line_col(pos);
        ProcessError::Process(h.handler().build_error_message(msg, line, col))
    }

    fn next_event(&mut self, h: &mut dyn SaxHandler) -> Result<Event<'static>, ProcessError> {
        if let Some(e) = self.pending.take() {
            return Ok(e);
        }
        self.buf.clear();
        match self.reader.read_event_into(&mut self.buf) {
            Ok(e) => Ok(e.into_owned()),
            Err(e) => Err(self.fatal(h, &e.to_string(), self.reader.error_position())),
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
        let pos = self.reader.buffer_position();
        let e = self.next_event(h)?;
        match e {
            Event::Start(ref s) | Event::Empty(ref s) => {
                if self.depth == 0 && self.root_seen {
                    return Err(self.fatal(h, "Extra content at the end of the document (second root element)", pos));
                }
                self.root_seen = true;
                let name = s.name().as_ref().to_string();
                let mut attrs = Vec::new();
                for a in s.attributes() {
                    let a = a.map_err(|err| self.fatal(h, &err.to_string(), pos))?;
                    let v = a.normalized_value(XmlVersion::Implicit1_0).map_err(|err| self.fatal(h, &err.to_string(), pos))?;
                    attrs.push((a.key.as_ref().to_string(), v.into_owned()));
                }
                generic_sax_handler::start_element(h, &name, attrs)?;
                if matches!(e, Event::Empty(_)) {
                    generic_sax_handler::end_element(h, &name)?;
                } else {
                    self.depth += 1;
                }
            }
            Event::End(ref s) => {
                self.depth -= 1;
                generic_sax_handler::end_element(h, s.name().as_ref())?;
            }
            Event::Text(ref t) => {
                if self.depth > 0 {
                    generic_sax_handler::characters(h, &t.xml10_content());
                } else if !t.trim_ascii().is_empty() {
                    return Err(self.fatal(h, "Text is not allowed outside of the root element", pos));
                }
            }
            Event::CData(ref c) => {
                if self.depth > 0 {
                    generic_sax_handler::characters(h, &c.xml10_content());
                }
            }
            Event::GeneralRef(ref r) => {
                let resolved = match r.resolve_char_ref() {
                    Ok(Some(c)) => c.to_string(),
                    Ok(None) => match quick_xml::escape::resolve_predefined_entity(r.as_ref()) {
                        Some(s) => s.to_string(),
                        None => return Err(self.fatal(h, &format!("The entity '{}' was referenced, but not declared", r.as_ref()), pos)),
                    },
                    Err(err) => return Err(self.fatal(h, &err.to_string(), pos)),
                };
                if self.depth > 0 {
                    generic_sax_handler::characters(h, &resolved);
                }
            }
            Event::Decl(_) | Event::PI(_) | Event::Comment(_) | Event::DocType(_) => {}
            Event::Eof => {
                if self.depth > 0 || !self.root_seen {
                    return Err(self.fatal(h, "Unexpected end of file", pos));
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
