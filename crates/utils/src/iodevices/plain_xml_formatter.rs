//! Port of utils/iodevices/PlainXMLFormatter.{h,cpp}
//! Write errors are ignored here as in C++ (the stream records them, see OutputDevice::ok).
use super::options_hook::{options, write_xml_header};
use crate::common::time::{time2string, SUMOTime};
use crate::xml::stand_ins::iso_time_string;
use crate::xml::sumo_xml_definitions::{Attrs, SumoXMLAttr};
use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug, Default)]
pub struct PlainXMLFormatter {
    my_xml_stack: Vec<String>,
    my_default_indentation: usize,
    my_have_pending_opener: bool,
}

impl PlainXMLFormatter {
    pub fn new(default_indentation: i32) -> Self {
        PlainXMLFormatter { my_xml_stack: Vec::new(), my_default_indentation: default_indentation as usize, my_have_pending_opener: false }
    }

    pub fn write_xml_header(&mut self, into: &mut dyn Write, root_element: &str, attrs: &BTreeMap<SumoXMLAttr, String>, write_metadata: bool, include_config: bool) -> bool {
        if !self.my_xml_stack.is_empty() {
            return false;
        }
        let oc = options();
        let _ = write_xml_header(into, include_config);
        self.open_tag(into, root_element);
        for (attr, val) in attrs {
            Self::write_attr(into, Attrs.get_string(*attr), val);
        }
        let _ = into.write_all(b">\n");
        if write_metadata {
            let _ = writeln!(into, "    <metadata created_at=\"{}\" created_by=\"{}\">", iso_time_string(), oc.get_full_name());
            oc.write_configuration(into, "        ");
            let _ = into.write_all(b"    </metadata>\n");
        }
        self.my_have_pending_opener = false;
        true
    }

    pub fn open_tag(&mut self, into: &mut dyn Write, xml_element: &str) {
        if self.my_have_pending_opener {
            let _ = into.write_all(b">\n");
        }
        self.my_have_pending_opener = true;
        let _ = write!(into, "{:indent$}<{xml_element}", "", indent = 4 * (self.my_xml_stack.len() + self.my_default_indentation));
        self.my_xml_stack.push(xml_element.to_string());
    }

    pub fn close_tag(&mut self, into: &mut dyn Write, comment: &str) -> bool {
        let Some(top) = self.my_xml_stack.last() else {
            return false;
        };
        if self.my_have_pending_opener {
            let _ = writeln!(into, "/>{comment}");
            self.my_have_pending_opener = false;
        } else {
            let indent = 4 * (self.my_xml_stack.len() + self.my_default_indentation - 1);
            let _ = writeln!(into, "{:indent$}</{top}>{comment}", "");
        }
        self.my_xml_stack.pop();
        true
    }

    pub fn write_preformatted_tag(&mut self, into: &mut dyn Write, val: &str) {
        if self.my_have_pending_opener {
            let _ = into.write_all(b">\n");
            self.my_have_pending_opener = false;
        }
        let _ = into.write_all(val.as_bytes());
    }

    pub fn write_padding(&mut self, into: &mut dyn Write, val: &str) {
        let _ = into.write_all(val.as_bytes());
    }

    /// writeAttr with the value already converted (toString / escapeXML done by the caller)
    pub fn write_attr(into: &mut dyn Write, attr: &str, val: &str) {
        let _ = write!(into, " {attr}=\"{val}\"");
    }

    pub fn write_time(&self, into: &mut dyn Write, attr: SumoXMLAttr, val: SUMOTime) {
        let _ = write!(into, " {}=\"{}\"", Attrs.get_string(attr), time2string(val));
    }

    pub fn wrote_header(&self) -> bool {
        !self.my_xml_stack.is_empty()
    }
}
