//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrtverlaufdateien.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Fahrtverlaufdateien` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Fahrtverlaufdateien.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Fahrtverlaufdateien {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Fahrtverlaufdateien {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Fahrtverlaufdateien {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Fahrtverlaufdateien {
    /// C++: `void NIVissimSingleTypeParser_Fahrtverlaufdateien::NIVissimSingleTypeParser_Fahrtverlaufdateien(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Fahrtverlaufdateien::NIVissimSingleTypeParser_Fahrtverlaufdateien(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Fahrtverlaufdateien::NIVissimSingleTypeParser_Fahrtverlaufdateien")
    }

    /// C++: `void NIVissimSingleTypeParser_Fahrtverlaufdateien::~NIVissimSingleTypeParser_Fahrtverlaufdateien() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Fahrtverlaufdateien::~NIVissimSingleTypeParser_Fahrtverlaufdateien() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Fahrtverlaufdateien::~NIVissimSingleTypeParser_Fahrtverlaufdateien")
    }

    /// C++: `bool NIVissimSingleTypeParser_Fahrtverlaufdateien::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Fahrtverlaufdateien::parse(std::istream&) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Fahrtverlaufdateien::parse")
    }

}

