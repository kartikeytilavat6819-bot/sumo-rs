//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Routenentscheidungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Routenentscheidungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Routenentscheidungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Routenentscheidungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Routenentscheidungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Routenentscheidungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Routenentscheidungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Routenentscheidungsdefinition::NIVissimSingleTypeParser_Routenentscheidungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Routenentscheidungsdefinition::NIVissimSingleTypeParser_Routenentscheidungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Routenentscheidungsdefinition::NIVissimSingleTypeParser_Routenentscheidungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Routenentscheidungsdefinition::~NIVissimSingleTypeParser_Routenentscheidungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Routenentscheidungsdefinition::~NIVissimSingleTypeParser_Routenentscheidungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Routenentscheidungsdefinition::~NIVissimSingleTypeParser_Routenentscheidungsdefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Routenentscheidungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Routenentscheidungsdefinition::parse(std::istream& from) {
    //     std::string tag;
    //     while (tag != "fahrzeugklassen") {
    //         tag = myRead(from);
    //     }
    //     do {
    //         while (tag != "DATAEND" || tag == "route") {
    //             if (tag == "route") {
    //                 while (tag != "strecke") {
    //                     tag = myRead(from);
    //                 }
    //                 tag = readEndSecure(from);
    //             } else {
    //                 tag = readEndSecure(from);
    //             }
    //         }
    //         if (tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //     } while (tag != "DATAEND");
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Routenentscheidungsdefinition::parse")
    }

}

