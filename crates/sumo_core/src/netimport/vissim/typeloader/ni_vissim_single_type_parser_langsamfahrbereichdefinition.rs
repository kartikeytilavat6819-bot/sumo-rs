//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Langsamfahrbereichdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
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

/// C++ class `NIVissimSingleTypeParser_Langsamfahrbereichdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Langsamfahrbereichdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Langsamfahrbereichdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Langsamfahrbereichdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Langsamfahrbereichdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Langsamfahrbereichdefinition {
    /// C++: `void NIVissimSingleTypeParser_Langsamfahrbereichdefinition::NIVissimSingleTypeParser_Langsamfahrbereichdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Langsamfahrbereichdefinition::NIVissimSingleTypeParser_Langsamfahrbereichdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Langsamfahrbereichdefinition::~NIVissimSingleTypeParser_Langsamfahrbereichdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Langsamfahrbereichdefinition::~NIVissimSingleTypeParser_Langsamfahrbereichdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Langsamfahrbereichdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Langsamfahrbereichdefinition::parse(std::istream& from) {
    //     std::string id;
    //     from >> id;
    //     readUntil(from, "fahrzeugklasse");
    //     std::string tag = "fahrzeugklasse";
    //     while (tag == "fahrzeugklasse") {
    //         readUntil(from, "maxverzoegerung");
    //         tag = myRead(from);
    //         tag = myRead(from);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        read_raw_token(from);
        self.read_until(from, "fahrzeugklasse");
        let mut tag = "fahrzeugklasse".to_string();
        while tag == "fahrzeugklasse" {
            self.read_until(from, "maxverzoegerung");
            tag = self.my_read(from);
            tag = self.my_read(from);
        }
        true
    }

}

