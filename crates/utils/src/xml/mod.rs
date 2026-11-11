//! Port of utils/xml
// shortcut: CommonXMLStructure, GenericHandler, NamespaceIDs and SAXWeightsHandler are not ported yet
pub mod generic_sax_handler;
pub mod stand_ins;
pub mod string_bijection;
pub mod sumo_sax_attributes;
pub mod sumo_sax_handler;
pub mod sumo_sax_reader;
pub mod sumo_xml_definitions;
pub mod xml_sub_sys;

#[cfg(test)]
mod tests;
