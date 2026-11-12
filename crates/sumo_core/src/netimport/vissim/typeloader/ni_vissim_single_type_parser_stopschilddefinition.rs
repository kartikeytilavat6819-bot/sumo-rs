//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Stopschilddefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Stopschilddefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Stopschilddefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Stopschilddefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Stopschilddefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Stopschilddefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Stopschilddefinition {
    /// C++: `void NIVissimSingleTypeParser_Stopschilddefinition::NIVissimSingleTypeParser_Stopschilddefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Stopschilddefinition::NIVissimSingleTypeParser_Stopschilddefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Stopschilddefinition::NIVissimSingleTypeParser_Stopschilddefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Stopschilddefinition::~NIVissimSingleTypeParser_Stopschilddefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Stopschilddefinition::~NIVissimSingleTypeParser_Stopschilddefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Stopschilddefinition::~NIVissimSingleTypeParser_Stopschilddefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Stopschilddefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Stopschilddefinition::parse(std::istream& from) {
    //     readUntil(from, "strecke");
    //     std::string tag;
    //     from >> tag; // edge name
    //     from >> tag; // "spur"
    //     from >> tag; // lane no
    //     from >> tag; // "bei"
    //     from >> tag; // pos
    //     from >> tag;
    //     if (tag == "RTOR") {
    //         from >> tag; // "lsa"
    //         from >> tag; // lsa id
    //         from >> tag; // "gruppe"
    //         from >> tag; // gruppe id
    //     } else {
    //         while (tag == "fahrzeugklasse") {
    //             from >> tag; // class no
    //             from >> tag; // "zeiten"
    //             from >> tag; // times no
    //             tag = readEndSecure(from, "fahrzeugklasse");
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Stopschilddefinition::parse")
    }

}

