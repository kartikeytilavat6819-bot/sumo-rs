//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Liniendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Liniendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Liniendefinition.h)
pub struct NIVissimSingleTypeParser_Liniendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Liniendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Liniendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Liniendefinition {
    /// C++: `void NIVissimSingleTypeParser_Liniendefinition::NIVissimSingleTypeParser_Liniendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Liniendefinition::NIVissimSingleTypeParser_Liniendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Liniendefinition::NIVissimSingleTypeParser_Liniendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Liniendefinition::~NIVissimSingleTypeParser_Liniendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Liniendefinition::~NIVissimSingleTypeParser_Liniendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Liniendefinition::~NIVissimSingleTypeParser_Liniendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Liniendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Liniendefinition::parse(std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     //cout << tag << endl;
    //     do {
    //         getline(from, tag);
    //     } while (tag.length() > 0 && tag[0] == ' ');
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Liniendefinition::parse")
    }

}

