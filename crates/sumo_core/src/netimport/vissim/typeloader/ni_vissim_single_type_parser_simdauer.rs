//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Simdauer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Simdauer` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Simdauer.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Simdauer {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Simdauer {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Simdauer {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Simdauer {
    /// C++: `void NIVissimSingleTypeParser_Simdauer::NIVissimSingleTypeParser_Simdauer(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Simdauer::NIVissimSingleTypeParser_Simdauer(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Simdauer::NIVissimSingleTypeParser_Simdauer")
    }

    /// C++: `void NIVissimSingleTypeParser_Simdauer::~NIVissimSingleTypeParser_Simdauer() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Simdauer::~NIVissimSingleTypeParser_Simdauer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Simdauer::~NIVissimSingleTypeParser_Simdauer")
    }

    /// C++: `bool NIVissimSingleTypeParser_Simdauer::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Simdauer::parse(std::istream& from) {
    //     std::string duration;
    //     from >> duration;
    //     // !!!
    //     try {
    //         StringUtils::toDouble(duration);
    //     } catch (...) {
    //         WRITE_ERROR(TL("Simulation duration could not be parsed"));
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Simdauer::parse")
    }

}

