//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Haltestellendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `from >> tmp` for a std::string: skips whitespace and reads one token without lower-casing it
fn read_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

/// C++ class `NIVissimSingleTypeParser_Haltestellendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Haltestellendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Haltestellendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Haltestellendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Haltestellendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Haltestellendefinition {
    /// C++: `void NIVissimSingleTypeParser_Haltestellendefinition::NIVissimSingleTypeParser_Haltestellendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Haltestellendefinition::NIVissimSingleTypeParser_Haltestellendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Haltestellendefinition::~NIVissimSingleTypeParser_Haltestellendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Haltestellendefinition::~NIVissimSingleTypeParser_Haltestellendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Haltestellendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Haltestellendefinition::parse(std::istream& from) {
    //     std::string tag;
    //     while (tag != "strecke") {
    //         tag = myRead(from);
    //     }
    //     while (tag != "laenge") {
    //         tag = myRead(from);
    //     }
    //     from >> tag;
    //     tag = readEndSecure(from);
    //     while (tag == "einsteiger" && tag != "DATAEND") {
    //         while (tag != "linie") {
    //             tag = myRead(from);
    //         }
    //         while (tag != "einsteiger" && tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let mut tag = String::new();
        while tag != "strecke" {
            tag = self.my_read(from);
        }
        while tag != "laenge" {
            tag = self.my_read(from);
        }
        read_raw_token(from);
        tag = self.read_end_secure(from, "");
        while tag == "einsteiger" && tag != "DATAEND" {
            while tag != "linie" {
                tag = self.my_read(from);
            }
            while tag != "einsteiger" && tag != "DATAEND" {
                tag = self.read_end_secure(from, "");
            }
        }
        true
    }

}

