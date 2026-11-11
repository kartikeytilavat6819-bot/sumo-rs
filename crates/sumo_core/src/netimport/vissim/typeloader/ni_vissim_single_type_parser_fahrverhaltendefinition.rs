//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrverhaltendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Fahrverhaltendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrverhaltendefinition.h)
pub struct NIVissimSingleTypeParser_Fahrverhaltendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Fahrverhaltendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Fahrverhaltendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Fahrverhaltendefinition {
    /// C++: `void NIVissimSingleTypeParser_Fahrverhaltendefinition::NIVissimSingleTypeParser_Fahrverhaltendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Fahrverhaltendefinition::NIVissimSingleTypeParser_Fahrverhaltendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Fahrverhaltendefinition::NIVissimSingleTypeParser_Fahrverhaltendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Fahrverhaltendefinition::~NIVissimSingleTypeParser_Fahrverhaltendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Fahrverhaltendefinition::~NIVissimSingleTypeParser_Fahrverhaltendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Fahrverhaltendefinition::~NIVissimSingleTypeParser_Fahrverhaltendefinition")
    }

    /// C++: `bool NIVissimSingleTypeParser_Fahrverhaltendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Fahrverhaltendefinition::parse(std::istream& from) {
    //     std::string tmp;
    //     from >> tmp;
    //     // in the both next cases, we do not have to overread anything
    //     if (tmp == "BEHALT_ALTE_PARA" || tmp == "ANZ_VM") {
    //         return true;
    //     }
    //     //
    //     from >> tmp;
    //     if (tmp == "NAME") {
    //         readUntil(from, "gelbverhalten");
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Fahrverhaltendefinition::parse")
    }

}

