//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Kantensperrung.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Kantensperrung` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Kantensperrung.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Kantensperrung {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Kantensperrung {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Kantensperrung {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Kantensperrung {
    /// C++: `void NIVissimSingleTypeParser_Kantensperrung::NIVissimSingleTypeParser_Kantensperrung(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Kantensperrung::NIVissimSingleTypeParser_Kantensperrung(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Kantensperrung::~NIVissimSingleTypeParser_Kantensperrung() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Kantensperrung::~NIVissimSingleTypeParser_Kantensperrung() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Kantensperrung::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Kantensperrung::parse(std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     //
    //     std::string id;
    //     from >> id;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     int from_node;
    //     from >> from_node;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     int to_node;
    //     from >> to_node;
    //     //
    //     from >> tag;
    //     from >> tag;
    //     std::vector<int> edges;
    //     while (tag != "DATAEND") {
    //         tag = readEndSecure(from);
    //         if (tag != "DATAEND") {
    //             edges.push_back(StringUtils::toInt(tag));
    //         }
    //     }
    //     NIVissimClosures::dictionary(id, from_node, to_node, edges);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let mut tag: String = vissim_raw_token(from);
        //
        let id: String = vissim_raw_token(from);
        //
        tag = vissim_raw_token(from);
        tag = vissim_raw_token(from);
        // shortcut: C++ `>>` into int stops at the first character that does not fit, here the whole token is parsed
        let from_node: i32 = vissim_raw_token(from).parse::<i32>().unwrap_or(0);
        //
        tag = vissim_raw_token(from);
        tag = vissim_raw_token(from);
        let to_node: i32 = vissim_raw_token(from).parse::<i32>().unwrap_or(0);
        //
        tag = vissim_raw_token(from);
        tag = vissim_raw_token(from);
        let mut edges: Vec<i32> = Vec::new();
        while tag != "DATAEND" {
            tag = self.read_end_secure(from, "");
            if tag != "DATAEND" {
                edges.push(utils::common::string_utils::to_int(&tag).unwrap_or_else(|e| panic!("{e}")));
            }
        }
        crate::netimport::vissim::tempstructs::ni_vissim_closures::NIVissimClosures::dictionary(&id, from_node, to_node, &mut edges);
        true
    }

}

/// C++ `from >> tmp` for a std::string: skips whitespace, reads one whitespace-delimited token ("" at end of stream)
fn vissim_raw_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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

