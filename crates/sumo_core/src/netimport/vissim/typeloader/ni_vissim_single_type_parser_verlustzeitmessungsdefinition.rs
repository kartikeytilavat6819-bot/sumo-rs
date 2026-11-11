//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Verlustzeitmessungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Verlustzeitmessungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Verlustzeitmessungsdefinition.h)
pub struct NIVissimSingleTypeParser_Verlustzeitmessungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Verlustzeitmessungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Verlustzeitmessungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Verlustzeitmessungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::NIVissimSingleTypeParser_Verlustzeitmessungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::NIVissimSingleTypeParser_Verlustzeitmessungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::NIVissimSingleTypeParser_Verlustzeitmessungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::~NIVissimSingleTypeParser_Verlustzeitmessungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::~NIVissimSingleTypeParser_Verlustzeitmessungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::~NIVissimSingleTypeParser_Verlustzeitmessungsdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::parse(std::istream& from) {
    //     std::string tag = myRead(from);
    //     if (tag == "von") {
    //         while (tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //         return true;
    //     }
    //
    //     while (tag != "fahrzeugklassen") {
    //         tag = myRead(from);
    //     }
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Verlustzeitmessungsdefinition::parse")
    }

}

