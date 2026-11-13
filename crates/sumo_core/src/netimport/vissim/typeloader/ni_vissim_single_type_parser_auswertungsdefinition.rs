//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Auswertungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `from >> id` for a std::string: skips whitespace, reads one token, case preserved.
/// Mirrors the private `vissim_read_token` of ni_importer_vissim.rs (not visible here).
fn auswertung_read_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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

/// C++ class `NIVissimSingleTypeParser_Auswertungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Auswertungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Auswertungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Auswertungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Auswertungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Auswertungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Auswertungsdefinition::NIVissimSingleTypeParser_Auswertungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Auswertungsdefinition::NIVissimSingleTypeParser_Auswertungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Auswertungsdefinition::~NIVissimSingleTypeParser_Auswertungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Auswertungsdefinition::~NIVissimSingleTypeParser_Auswertungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Auswertungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Auswertungsdefinition::parse(std::istream& from) {
    //     std::string id;
    //     from >> id; // "typ"
    //
    //     if (id == "DATENBANK") { // !!! unverified
    //         return true;
    //     }
    //
    //     std::string type = myRead(from);
    //     if (type == "abfluss") {
    //         while (type != "signalgruppe") {
    //             type = myRead(from);
    //         }
    //         while (type != "DATAEND") {
    //             type = readEndSecure(from, "messung");
    //         }
    //     } else if (type == "vbv") {} else if (type == "dichte") {} else if (type == "emissionen") {} else if (type == "fzprot") {} else if (type == "spwprot") {} else if (type == "segment") {
    //         while (type != "konfdatei") {
    //             type = myRead(from);
    //         }
    //     } else if (type == "wegeausw") {} else if (type == "knoten") {} else if (type == "konvergenz") {
    //         while (type != "zeit") {
    //             type = myRead(from);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let id = auswertung_read_raw_token(from);
        if id == "DATENBANK" {
            return true;
        }

        let mut ty = self.my_read(from);
        if ty == "abfluss" {
            while ty != "signalgruppe" {
                ty = self.my_read(from);
            }
            while ty != "DATAEND" {
                ty = self.read_end_secure(from, "messung");
            }
        } else if ty == "vbv" {
        } else if ty == "dichte" {
        } else if ty == "emissionen" {
        } else if ty == "fzprot" {
        } else if ty == "spwprot" {
        } else if ty == "segment" {
            while ty != "konfdatei" {
                ty = self.my_read(from);
            }
        } else if ty == "wegeausw" {
        } else if ty == "knoten" {
        } else if ty == "konvergenz" {
            while ty != "zeit" {
                ty = self.my_read(from);
            }
        }
        true
    }

}

