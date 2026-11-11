//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Streckentypdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Streckentypdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Streckentypdefinition.h)
pub struct NIVissimSingleTypeParser_Streckentypdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Streckentypdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Streckentypdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Streckentypdefinition {
    /// C++: `void NIVissimSingleTypeParser_Streckentypdefinition::NIVissimSingleTypeParser_Streckentypdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Streckentypdefinition::NIVissimSingleTypeParser_Streckentypdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Streckentypdefinition::NIVissimSingleTypeParser_Streckentypdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Streckentypdefinition::~NIVissimSingleTypeParser_Streckentypdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Streckentypdefinition::~NIVissimSingleTypeParser_Streckentypdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Streckentypdefinition::~NIVissimSingleTypeParser_Streckentypdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Streckentypdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Streckentypdefinition::parse(std::istream& from) {
    //     readUntil(from, "default");
    //     std::string tag;
    //     from >> tag;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Streckentypdefinition::parse")
    }

}

