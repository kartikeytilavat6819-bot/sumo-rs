//! Port of utils/xml
// shortcut: GenericHandler and NamespaceIDs are not ported yet
pub mod common_xml_structure;
pub mod generic_sax_handler;
pub mod saxweights_handler;
pub mod stand_ins;
pub mod string_bijection;
pub mod sumo_sax_attributes;
pub mod sumo_sax_handler;
pub mod sumo_sax_reader;
pub mod sumo_xml_definitions;
pub mod xml_sub_sys;

#[cfg(test)]
mod tests;
