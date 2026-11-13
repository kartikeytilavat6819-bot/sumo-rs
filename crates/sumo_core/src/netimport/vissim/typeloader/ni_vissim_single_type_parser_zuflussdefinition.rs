//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Zuflussdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Zuflussdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Zuflussdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Zuflussdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Zuflussdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Zuflussdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

/// C++ `from >> tmp` for a std::string: skips whitespace, reads one whitespace-delimited token without lowercasing ("" at end of stream)
fn read_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let is_space = |c: u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && is_space(data[i]) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !is_space(data[i]) {
        i += 1;
    }
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
    from.set_position(i as u64);
    tok
}

impl NIVissimSingleTypeParser_Zuflussdefinition {
    /// C++: `void NIVissimSingleTypeParser_Zuflussdefinition::NIVissimSingleTypeParser_Zuflussdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Zuflussdefinition::NIVissimSingleTypeParser_Zuflussdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Zuflussdefinition::~NIVissimSingleTypeParser_Zuflussdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Zuflussdefinition::~NIVissimSingleTypeParser_Zuflussdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Zuflussdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Zuflussdefinition::parse(std::istream& from) {
    //     std::string id, edgeid;
    //     from >> id; // type-checking is missing!
    //     std::string tag, name;
    //     // override some optional values till q
    //     while (tag != "q") {
    //         tag = overrideOptionalLabel(from);
    //         if (tag == "name") {
    //             name = readName(from);
    //         } else if (tag == "strecke") {
    //             from >> edgeid; // type-checking is missing!
    //         }
    //     }
    //     // read q
    //     // bool exact = false;
    //     tag = myRead(from);
    //     if (tag == "exakt") {
    //         // exact = true;
    //         tag = myRead(from);
    //     }
    //     // double q = StringUtils::toDouble(tag);
    //     // read the vehicle types
    //     from >> tag;
    //     int vehicle_combination;
    //     from >> vehicle_combination;
    //     // check whether optional time information is available
    //     tag = readEndSecure(from);
    //     double beg, end;
    //     beg = -1;
    //     end = -1;
    //     if (tag == "zeit") {
    //         from >> tag;
    //         from >> beg;
    //         from >> tag;
    //         from >> end;
    //     }
    //     return NIVissimSource::dictionary(id, name, edgeid);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let id: String = read_raw_token(from); // type-checking is missing!
        let mut tag: String = String::new();
        let mut name: String = String::new();
        let mut edgeid: String = String::new();
        // override some optional values till q
        while tag != "q" {
            tag = self.override_optional_label(from, "");
            if tag == "name" {
                name = self.read_name(from);
            } else if tag == "strecke" {
                edgeid = read_raw_token(from); // type-checking is missing!
            }
        }
        // read q
        // bool exact = false;
        tag = self.my_read(from);
        if tag == "exakt" {
            // exact = true;
            tag = self.my_read(from);
        }
        // double q = StringUtils::toDouble(tag);
        // read the vehicle types
        tag = read_raw_token(from);
        // from >> vehicle_combination (an int that is never used)
        let _vehicle_combination: String = read_raw_token(from);
        // check whether optional time information is available
        tag = self.read_end_secure(from, "");
        // beg and end are read but never used in the C++ code
        let mut beg: f64 = -1.0;
        let mut end: f64 = -1.0;
        if tag == "zeit" {
            tag = read_raw_token(from);
            beg = read_raw_token(from).parse::<f64>().unwrap_or(0.0);
            tag = read_raw_token(from);
            end = read_raw_token(from).parse::<f64>().unwrap_or(0.0);
        }
        NIVissimSource::dictionary(&id, &name, &edgeid)
    }

}

