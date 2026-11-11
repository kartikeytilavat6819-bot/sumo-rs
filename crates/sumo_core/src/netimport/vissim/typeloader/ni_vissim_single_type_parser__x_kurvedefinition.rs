//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser__XKurvedefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser__XKurvedefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser__XKurvedefinition.h)
pub struct NIVissimSingleTypeParser__XKurvedefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser__XKurvedefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser__XKurvedefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser__XKurvedefinition {
    /// C++: `void NIVissimSingleTypeParser__XKurvedefinition::NIVissimSingleTypeParser__XKurvedefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser__XKurvedefinition::NIVissimSingleTypeParser__XKurvedefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser__XKurvedefinition::NIVissimSingleTypeParser__XKurvedefinition")
    }

    /// C++: `void NIVissimSingleTypeParser__XKurvedefinition::~NIVissimSingleTypeParser__XKurvedefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser__XKurvedefinition::~NIVissimSingleTypeParser__XKurvedefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser__XKurvedefinition::~NIVissimSingleTypeParser__XKurvedefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser__XKurvedefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser__XKurvedefinition::parse(std::istream&) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser__XKurvedefinition::parse")
    }

}

