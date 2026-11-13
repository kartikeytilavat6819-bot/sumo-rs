//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Richtungspfeildefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Richtungspfeildefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Richtungspfeildefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Richtungspfeildefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Richtungspfeildefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Richtungspfeildefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Richtungspfeildefinition {
    /// C++: `void NIVissimSingleTypeParser_Richtungspfeildefinition::NIVissimSingleTypeParser_Richtungspfeildefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Richtungspfeildefinition::NIVissimSingleTypeParser_Richtungspfeildefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Richtungspfeildefinition::~NIVissimSingleTypeParser_Richtungspfeildefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Richtungspfeildefinition::~NIVissimSingleTypeParser_Richtungspfeildefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
    }

    /// C++: `bool NIVissimSingleTypeParser_Richtungspfeildefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Richtungspfeildefinition::parse(std::istream& from) {
    //     std::string t;
    //     getline(from, t);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // getline(from, t): consume through the next newline, the text is unused
        let data = from.get_ref();
        let mut i = from.position() as usize;
        while i < data.len() && data[i] != b'\n' {
            i += 1;
        }
        if i < data.len() {
            i += 1;
        }
        from.set_position(i as u64);
        true
    }

}

