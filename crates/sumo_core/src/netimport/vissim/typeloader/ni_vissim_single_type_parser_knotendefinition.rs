//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Knotendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::to_int;

/// C++ `from >> tmp` for a std::string: skips whitespace, reads one whitespace-delimited token
fn read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
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
    let tok = vissim_bytes_to_string(&data[start..i]);
    from.set_position(i as u64);
    tok
}

/// C++ `from >> int`: a token that does not parse gives 0 (C++11 behaviour)
fn read_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    read_token(from).parse::<i32>().unwrap_or(0)
}

/// C++ `from >> double`: a token that does not parse gives 0 (C++11 behaviour)
fn read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    read_token(from).parse::<f64>().unwrap_or(0.0)
}

/// C++ class `NIVissimSingleTypeParser_Knotendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Knotendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Knotendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Knotendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Knotendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Knotendefinition {
    /// C++: `void NIVissimSingleTypeParser_Knotendefinition::NIVissimSingleTypeParser_Knotendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Knotendefinition::NIVissimSingleTypeParser_Knotendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Knotendefinition::~NIVissimSingleTypeParser_Knotendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Knotendefinition::~NIVissimSingleTypeParser_Knotendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; the base destructor runs next.
        self.base.drop_();
    }

    /// C++: `bool NIVissimSingleTypeParser_Knotendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Knotendefinition::parse(std::istream& from) {
    //     //
    //     int id;
    //     from >> id;
    //     //
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     //
    //     tag = overrideOptionalLabel(from);
    //     //
    //     while (tag != "netzausschnitt") {
    //         tag = myRead(from);
    //     }
    //     //
    //     tag = myRead(from);
    //     if (tag == "strecke") {
    //         NIVissimNodeParticipatingEdgeVector edges;
    //         while (tag == "strecke") {
    //             int edgeid;
    //             double from_pos, to_pos;
    //             from_pos = to_pos = -1.0;
    //             from >> edgeid;
    //             tag = readEndSecure(from, "strecke");
    //             if (tag == "von") {
    //                 from >> from_pos; // type-checking is missing!
    //                 from >> tag;
    //                 from >> to_pos; // type-checking is missing!
    //                 tag = readEndSecure(from, "strecke");
    //             }
    //             edges.push_back(new NIVissimNodeParticipatingEdge(edgeid, from_pos, to_pos));
    //         }
    //         NIVissimNodeDef_Edges::dictionary(id, name, edges);
    //     } else {
    //         int no = StringUtils::toInt(tag);
    //         PositionVector poly;
    //         for (int i = 0; i < no; i++) {
    //             poly.push_back(getPosition(from));
    //         }
    //         poly.closePolygon();
    //         NIVissimNodeDef_Poly::dictionary(id, name, poly);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        //
        let id = read_i32(from);
        //
        let mut tag = read_token(from);
        let name = self.read_name(from);
        //
        tag = self.override_optional_label(from, "");
        //
        while tag != "netzausschnitt" {
            tag = self.my_read(from);
        }
        //
        tag = self.my_read(from);
        if tag == "strecke" {
            let mut edges: NIVissimNodeParticipatingEdgeVector = Vec::new();
            while tag == "strecke" {
                let edgeid = read_i32(from);
                let (mut from_pos, mut to_pos) = (-1.0, -1.0);
                tag = self.read_end_secure(from, "strecke");
                if tag == "von" {
                    from_pos = read_f64(from); // type-checking is missing!
                    tag = read_token(from);
                    to_pos = read_f64(from); // type-checking is missing!
                    tag = self.read_end_secure(from, "strecke");
                }
                // shortcut: the edge defs are owned by the NIVissimNodeDef_Edges dictionary, which never frees them; the leak matches C++ ownership
                edges.push(Ptr::from_mut(Box::leak(Box::new(NIVissimNodeParticipatingEdge::new(edgeid, from_pos, to_pos)))));
            }
            NIVissimNodeDef_Edges::dictionary(id, &name, &edges);
        } else {
            let no = to_int(&tag).unwrap_or_else(|e| panic!("{e}"));
            let mut poly = PositionVector::default();
            for _ in 0..no {
                let p = self.get_position(from);
                poly.0.push(p);
            }
            poly.close_polygon();
            NIVissimNodeDef_Poly::dictionary(id, &name, &poly);
        }
        true
    }

}

