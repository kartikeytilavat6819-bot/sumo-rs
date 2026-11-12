//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Reisezeitmessungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Reisezeitmessungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Reisezeitmessungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Reisezeitmessungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Reisezeitmessungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Reisezeitmessungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Reisezeitmessungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Reisezeitmessungsdefinition::NIVissimSingleTypeParser_Reisezeitmessungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Reisezeitmessungsdefinition::NIVissimSingleTypeParser_Reisezeitmessungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Reisezeitmessungsdefinition::NIVissimSingleTypeParser_Reisezeitmessungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Reisezeitmessungsdefinition::~NIVissimSingleTypeParser_Reisezeitmessungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Reisezeitmessungsdefinition::~NIVissimSingleTypeParser_Reisezeitmessungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Reisezeitmessungsdefinition::~NIVissimSingleTypeParser_Reisezeitmessungsdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Reisezeitmessungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Reisezeitmessungsdefinition::parse(std::istream& from) {
    //     std::string tag = myRead(from);
    //     if (tag == "aggregationsintervall") {
    //         while (tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //         return true;
    //     }
    //
    //     while (tag != "strecke") {
    //         tag = myRead(from);
    //     }
    //     tag = myRead(from);
    //     while (tag != "strecke") {
    //         tag = myRead(from);
    //     }
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Reisezeitmessungsdefinition::parse")
    }

}

