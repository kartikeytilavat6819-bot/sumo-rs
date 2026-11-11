//! Port of utils/xml/SUMOSAXHandler.{h,cpp}: a GenericSAXHandler using the SUMO tag and attribute tables
use super::generic_sax_handler::GenericSAXHandler;
use super::sumo_xml_definitions::{ATTRS, SUMO_ATTR_NOTHING, SUMO_TAG_NOTHING, TAGS};

pub fn new(file: &str, expected_root: &str) -> GenericSAXHandler {
    GenericSAXHandler::new(&TAGS, SUMO_TAG_NOTHING, &ATTRS, SUMO_ATTR_NOTHING, file, expected_root)
}
