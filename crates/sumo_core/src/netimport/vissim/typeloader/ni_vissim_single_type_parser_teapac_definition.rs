//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_TEAPACDefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_TEAPACDefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_TEAPACDefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_TEAPACDefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_TEAPACDefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_TEAPACDefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_TEAPACDefinition {
    /// C++: `void NIVissimSingleTypeParser_TEAPACDefinition::NIVissimSingleTypeParser_TEAPACDefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_TEAPACDefinition::NIVissimSingleTypeParser_TEAPACDefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_TEAPACDefinition::~NIVissimSingleTypeParser_TEAPACDefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_TEAPACDefinition::~NIVissimSingleTypeParser_TEAPACDefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_TEAPACDefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_TEAPACDefinition::parse(std::istream&) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        true
    }

}

