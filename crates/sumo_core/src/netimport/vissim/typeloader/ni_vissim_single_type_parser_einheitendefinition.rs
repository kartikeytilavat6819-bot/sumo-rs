//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Einheitendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Einheitendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Einheitendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Einheitendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Einheitendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Einheitendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Einheitendefinition {
    /// C++: `void NIVissimSingleTypeParser_Einheitendefinition::NIVissimSingleTypeParser_Einheitendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Einheitendefinition::NIVissimSingleTypeParser_Einheitendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Einheitendefinition::~NIVissimSingleTypeParser_Einheitendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Einheitendefinition::~NIVissimSingleTypeParser_Einheitendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Einheitendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Einheitendefinition::parse(std::istream&) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        true
    }

}

