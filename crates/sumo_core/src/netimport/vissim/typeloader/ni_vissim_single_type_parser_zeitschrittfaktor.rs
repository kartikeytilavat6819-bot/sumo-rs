//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Zeitschrittfaktor.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Zeitschrittfaktor` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Zeitschrittfaktor.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Zeitschrittfaktor {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Zeitschrittfaktor {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Zeitschrittfaktor {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Zeitschrittfaktor {
    /// C++: `void NIVissimSingleTypeParser_Zeitschrittfaktor::NIVissimSingleTypeParser_Zeitschrittfaktor(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Zeitschrittfaktor::NIVissimSingleTypeParser_Zeitschrittfaktor(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        todo!("port NIVissimSingleTypeParser_Zeitschrittfaktor::NIVissimSingleTypeParser_Zeitschrittfaktor")
    }

    /// C++: `void NIVissimSingleTypeParser_Zeitschrittfaktor::~NIVissimSingleTypeParser_Zeitschrittfaktor() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Zeitschrittfaktor::~NIVissimSingleTypeParser_Zeitschrittfaktor() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Zeitschrittfaktor::~NIVissimSingleTypeParser_Zeitschrittfaktor")
    }

    /// C++: `bool NIVissimSingleTypeParser_Zeitschrittfaktor::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Zeitschrittfaktor::parse(std::istream&) {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Zeitschrittfaktor::parse")
    }

}

