//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Rautedefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Rautedefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Rautedefinition.h)
pub struct NIVissimSingleTypeParser_Rautedefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Rautedefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Rautedefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Rautedefinition {
    /// C++: `void NIVissimSingleTypeParser_Rautedefinition::NIVissimSingleTypeParser_Rautedefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Rautedefinition::NIVissimSingleTypeParser_Rautedefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Rautedefinition::NIVissimSingleTypeParser_Rautedefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Rautedefinition::~NIVissimSingleTypeParser_Rautedefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Rautedefinition::~NIVissimSingleTypeParser_Rautedefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Rautedefinition::~NIVissimSingleTypeParser_Rautedefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Rautedefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Rautedefinition::parse(std::istream& from) {
    //     std::string t;
    //     getline(from, t);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Rautedefinition::parse")
    }

}

