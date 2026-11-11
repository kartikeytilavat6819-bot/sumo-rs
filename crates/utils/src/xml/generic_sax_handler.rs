//! Port of utils/xml/GenericSAXHandler.{h,cpp}
//! The C++ base class is split into the state struct `GenericSAXHandler` and the trait
//! `SaxHandler` (the virtual myStartElement/myCharacters/myEndElement). A handler embeds a
//! GenericSAXHandler and returns it from `handler()`. The Xerces callbacks
//! (startElement/characters/endElement) are the free functions below, driven by SUMOSAXReader.
use super::stand_ins::{get_configuration_relative, is_absolute, write_warning, ProcessError};
use super::sumo_sax_attributes::SUMOSAXAttributes;
use super::sumo_xml_definitions::{SUMO_ATTR_HREF, SUMO_TAG_INCLUDE, SUMO_TAG_NOTHING};
use super::xml_sub_sys;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub trait SaxHandler {
    /// the embedded GenericSAXHandler state
    fn handler(&mut self) -> &mut GenericSAXHandler;
    fn my_start_element(&mut self, _element: i32, _attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        Ok(())
    }
    fn my_characters(&mut self, _element: i32, _chars: &str) -> Result<(), ProcessError> {
        Ok(())
    }
    fn my_end_element(&mut self, _element: i32) -> Result<(), ProcessError> {
        Ok(())
    }
}

pub struct GenericSAXHandler {
    my_tag_map: HashMap<String, i32>,
    my_predefined_tags_mml: Arc<Vec<String>>,
    my_characters_vector: Vec<String>,
    /// registerParent, inverted for Rust ownership: (parent indicator tag, child handler)
    my_child: Option<(i32, Rc<RefCell<dyn SaxHandler>>)>,
    /// callParentEnd requests, executed by the parent once the child returns
    my_parent_end: Vec<i32>,
    my_file_name: String,
    my_expected_root: String,
    my_collect_character_data: bool,
    my_root_seen: bool,
    my_section: i32,
    my_section_seen: bool,
    my_section_ended: bool,
    my_section_open: bool,
    my_next_section_start: Option<(i32, SUMOSAXAttributes)>,
}

impl SaxHandler for GenericSAXHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        self
    }
}

impl GenericSAXHandler {
    /// `tags`/`attrs` are read up to (excluding) the terminator key
    pub fn new(tags: &[(&str, i32)], terminator_tag: i32, attrs: &[(&str, i32)], terminator_attr: i32, file: &str, expected_root: &str) -> Self {
        let my_tag_map = tags.iter().take_while(|e| e.1 != terminator_tag).map(|&(s, k)| (s.to_string(), k)).collect();
        let mut mml: Vec<String> = Vec::new();
        for &(s, key) in attrs.iter().take_while(|e| e.1 != terminator_attr) {
            assert!(key >= 0);
            if key as usize >= mml.len() {
                mml.resize(key as usize + 1, String::new());
            }
            mml[key as usize] = s.to_string();
        }
        GenericSAXHandler {
            my_tag_map,
            my_predefined_tags_mml: Arc::new(mml),
            my_characters_vector: Vec::new(),
            my_child: None,
            my_parent_end: Vec::new(),
            my_file_name: file.to_string(),
            my_expected_root: expected_root.to_string(),
            my_collect_character_data: false,
            my_root_seen: false,
            my_section: -1,
            my_section_seen: false,
            my_section_ended: false,
            my_section_open: false,
            my_next_section_start: None,
        }
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.my_file_name = name.to_string();
    }
    pub fn get_file_name(&self) -> &str {
        &self.my_file_name
    }
    /// Hands all events up to and including the end of element `tag` to `child`
    /// (C++: child->registerParent(tag, parent)).
    pub fn register_child(&mut self, tag: i32, child: Rc<RefCell<dyn SaxHandler>>) {
        self.my_child = Some((tag, child));
    }
    /// Lets the parent handler process `element` after the current callback returns.
    pub fn call_parent_end(&mut self, element: i32) {
        self.my_parent_end.push(element);
    }
    pub fn set_section(&mut self, element: i32, seen: bool) {
        self.my_section = element;
        self.my_section_seen = seen;
        self.my_section_open = seen;
        self.my_section_ended = false;
    }
    pub fn section_finished(&self) -> bool {
        self.my_section_ended
    }
    pub fn retrieve_next_section_start(&mut self) -> Option<(i32, SUMOSAXAttributes)> {
        self.my_next_section_start.take()
    }
    pub fn needs_character_data(&mut self, value: bool) {
        self.my_collect_character_data = value;
    }
    pub fn convert_tag(&self, tag: &str) -> i32 {
        self.my_tag_map.get(tag).copied().unwrap_or(SUMO_TAG_NOTHING)
    }
    /// " In file '...'\n At line/column l/c.\n" part of the parse error message
    pub fn build_error_message(&self, msg: &str, line: u64, column: u64) -> String {
        format!("{msg}\n In file '{}'\n At line/column {}/{column}.\n", self.my_file_name, line + 1)
    }
}

pub fn start_element(h: &mut dyn SaxHandler, name: &str, attrs: Vec<(String, String)>) -> Result<(), ProcessError> {
    if let Some((_, child)) = &h.handler().my_child {
        let child = child.clone();
        return start_element(&mut *child.borrow_mut(), name, attrs);
    }
    let base = h.handler();
    if !base.my_root_seen && !base.my_expected_root.is_empty() && name != base.my_expected_root {
        write_warning(&format!("Found root element '{name}' in file '{}' (expected '{}').", base.my_file_name, base.my_expected_root));
    }
    base.my_root_seen = true;
    base.my_characters_vector.clear();
    let element = base.convert_tag(name);
    let na = SUMOSAXAttributes::new(attrs, base.my_predefined_tags_mml.clone(), name);
    if base.my_section_seen && !base.my_section_open && element != base.my_section {
        base.my_section_ended = true;
        base.my_next_section_start = Some((element, na));
        return Ok(());
    }
    if element == base.my_section {
        base.my_section_seen = true;
        base.my_section_open = true;
    }
    if element == SUMO_TAG_INCLUDE {
        let mut file = na.get_string(SUMO_ATTR_HREF);
        if !is_absolute(&file) {
            file = get_configuration_relative(base.get_file_name(), &file);
        }
        xml_sub_sys::run_parser(h, &file, false, false, false, true)?;
        Ok(())
    } else {
        h.my_start_element(element, &na)
    }
}

pub fn end_element(h: &mut dyn SaxHandler, name: &str) -> Result<(), ProcessError> {
    if let Some((indicator, child)) = h.handler().my_child.clone() {
        let (element, parent_end) = {
            let mut c = child.borrow_mut();
            end_element(&mut *c, name)?;
            let cb = c.handler();
            (cb.convert_tag(name), std::mem::take(&mut cb.my_parent_end))
        };
        for e in parent_end {
            h.my_end_element(e)?;
        }
        if element == indicator && element != SUMO_TAG_INCLUDE {
            h.handler().my_child = None;
        }
        return Ok(());
    }
    let base = h.handler();
    let element = base.convert_tag(name);
    // collect characters (the vector is only cleared by the next start element, as in C++)
    if !base.my_characters_vector.is_empty() {
        let buf = base.my_characters_vector.concat();
        h.my_characters(element, &buf)?;
    }
    let base = h.handler();
    if element == base.my_section {
        base.my_section_open = false;
    }
    if element != SUMO_TAG_INCLUDE {
        h.my_end_element(element)?;
    }
    Ok(())
}

pub fn characters(h: &mut dyn SaxHandler, chars: &str) {
    if let Some((_, child)) = &h.handler().my_child {
        let child = child.clone();
        return characters(&mut *child.borrow_mut(), chars);
    }
    let base = h.handler();
    if base.my_collect_character_data {
        base.my_characters_vector.push(chars.to_string());
    }
}
