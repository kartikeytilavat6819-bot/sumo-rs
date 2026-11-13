//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Liniendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Liniendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Liniendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Liniendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Liniendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Liniendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

/// C++ `from >> tag` for a std::string (same as NIImporter_Vissim's private vissim_read_token)
fn vissim_read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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

/// C++ `std::getline(strm, line)` (same as NIImporter_Vissim's private vissim_getline)
fn vissim_getline(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let start = from.position() as usize;
    if start >= data.len() {
        return String::new();
    }
    let mut i = start;
    while i < data.len() && data[i] != b'\n' {
        i += 1;
    }
    let line = String::from_utf8_lossy(&data[start..i]).into_owned();
    let next = if i < data.len() { i + 1 } else { i };
    from.set_position(next as u64);
    line
}

impl NIVissimSingleTypeParser_Liniendefinition {
    /// C++: `void NIVissimSingleTypeParser_Liniendefinition::NIVissimSingleTypeParser_Liniendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Liniendefinition::NIVissimSingleTypeParser_Liniendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Liniendefinition::~NIVissimSingleTypeParser_Liniendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Liniendefinition::~NIVissimSingleTypeParser_Liniendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // destructor: nothing to release explicitly, the base parser drops with the struct
    }

    /// C++: `bool NIVissimSingleTypeParser_Liniendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Liniendefinition::parse(std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     //cout << tag << endl;
    //     do {
    //         getline(from, tag);
    //     } while (tag.length() > 0 && tag[0] == ' ');
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let _ = vissim_read_token(from);
        loop {
            let tag = vissim_getline(from);
            if !(tag.len() > 0 && tag.starts_with(' ')) {
                break;
            }
        }
        true
    }

}

