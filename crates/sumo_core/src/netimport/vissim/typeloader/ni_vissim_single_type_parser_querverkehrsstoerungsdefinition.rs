//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_error;
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
    let tok = String::from_utf8_lossy(&data[start..i]).into_owned();
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

/// C++ class `NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::~NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::~NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; the base destructor runs next.
        self.base.drop_();
    }

    /// C++: `bool NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parse(std::istream& from) {
    //     std::string tag;
    //     tag = myRead(from);
    //     if (tag == "nureigenestrecke") {
    //         return parseOnlyMe(from);
    //     } else if (tag == "ort") {
    //         return parsePositionDescribed(from);
    //     } else if (tag == "nummer") {
    //         return parseNumbered(from);
    //     }
    //     WRITE_ERROR(
    //         "NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition: format problem");
    //     throw 1;
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let tag = self.my_read(from);
        if tag == "nureigenestrecke" {
            return self.parse_only_me(from);
        } else if tag == "ort" {
            return self.parse_position_described(from);
        } else if tag == "nummer" {
            return self.parse_numbered(from);
        }
        write_error("NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition: format problem");
        // C++ `throw 1;` is uncaught and ends the run; the panic is its Rust counterpart
        panic!("NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition: format problem")
    }

    /// C++: `bool NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseOnlyMe(std::istream &)` [private]
    /// ?? (unknown vissim-description)
    // bool
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseOnlyMe(std::istream& from) {
    //     std::string tag;
    //     from >> tag;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_only_me(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let tag = read_token(from);
        true
    }

    /// C++: `bool NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePositionDescribed(std::istream &)` [private]
    /// parses a disturbance described by its position
    // bool
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePositionDescribed(std::istream& from) {
    //     std::string tag = myRead(from);
    //     NIVissimExtendedEdgePoint edge = parsePos(from);
    // //    from >> tag; // "Durch"
    //     bool ok = true;
    //     do {
    //         from >> tag; // "Strecke"
    //         NIVissimExtendedEdgePoint by = parsePos(from);
    //         //
    //         double timegap;
    //         from >> timegap;
    //
    //         from >> tag;
    //         double waygap;
    //         from >> waygap;
    //
    //         double vmax = -1;
    //         tag = readEndSecure(from);
    //         if (tag == "vmax") {
    //             from >> vmax;
    //         }
    //         ok = NIVissimDisturbance::dictionary("", edge, by);
    //         if (tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //     } while (tag != "DATAEND" && ok);
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_position_described(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        let mut tag = self.my_read(from);
        let edge = self.parse_pos(from);
        //    from >> tag; // "Durch"
        let mut ok = true;
        loop {
            tag = read_token(from); // "Strecke"
            let by = self.parse_pos(from);
            //
            let timegap = read_f64(from);
            //
            tag = read_token(from);
            let waygap = read_f64(from);
            //
            tag = self.read_end_secure(from, "");
            if tag == "vmax" {
                let vmax = read_f64(from);
            }
            ok = NIVissimDisturbance::dictionary("", &edge, &by);
            if tag != "DATAEND" {
                tag = self.read_end_secure(from, "");
            }
            if tag == "DATAEND" || !ok {
                break;
            }
        }
        ok
    }

    /// C++: `bool NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseNumbered(std::istream &)` [private]
    /// parses a full description of a disturbance
    // bool
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseNumbered(std::istream& from) {
    //     //
    //     int id;
    //     from >> id;
    //     //
    //     std::string tag;
    //     from >> tag;
    //     std::string name = readName(from);
    //     // skip optional "Beschriftung"
    //     while (tag != "ort") {
    //         tag = myRead(from);
    //     }
    //     //
    //     from >> tag; // "Strecke"
    //     NIVissimExtendedEdgePoint edge = parsePos(from);
    //     bool ok = true;
    //     do {
    //         from >> tag; // "Ort"
    //         from >> tag; // "Strecke"
    //         NIVissimExtendedEdgePoint by = parsePos(from);
    //         //
    //         double timegap;
    //         from >> timegap;
    //
    //         double waygap;
    //         from >> tag;
    //         from >> waygap;
    //
    //         double vmax = -1;
    //         tag = readEndSecure(from);
    //         if (tag == "vmax") {
    //             from >> vmax;
    //         }
    //
    //         ok = NIVissimDisturbance::dictionary(name, edge, by);
    //         if (tag != "DATAEND") {
    //             tag = readEndSecure(from);
    //         }
    //     } while (tag != "DATAEND" && ok);
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn parse_numbered(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        //
        let id = read_i32(from);
        //
        let mut tag = read_token(from);
        let name = self.read_name(from);
        // skip optional "Beschriftung"
        while tag != "ort" {
            tag = self.my_read(from);
        }
        //
        tag = read_token(from); // "Strecke"
        let edge = self.parse_pos(from);
        let mut ok = true;
        loop {
            tag = read_token(from); // "Ort"
            tag = read_token(from); // "Strecke"
            let by = self.parse_pos(from);
            //
            let timegap = read_f64(from);
            //
            tag = read_token(from);
            let waygap = read_f64(from);
            //
            tag = self.read_end_secure(from, "");
            if tag == "vmax" {
                let vmax = read_f64(from);
            }
            //
            ok = NIVissimDisturbance::dictionary(&name, &edge, &by);
            if tag != "DATAEND" {
                tag = self.read_end_secure(from, "");
            }
            if tag == "DATAEND" || !ok {
                break;
            }
        }
        ok
    }

    /// C++: `NIVissimExtendedEdgePoint NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePos(std::istream &)` [private]
    // NIVissimExtendedEdgePoint
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePos(std::istream& from) {
    //     int edgeid;
    //     from >> edgeid; // type-checking is missing!
    //     //
    //     std::string tag;
    //     from >> tag;
    //     from >> tag;
    //     std::vector<int> lanes;
    //     if (tag == "ALLE") {
    //         //lanes.push_back(1); // !!!
    //     } else {
    //         lanes.push_back(StringUtils::toInt(tag));
    //     }
    //     //
    //     double position;
    //     from >> tag;
    //     from >> position;
    //     // assigned vehicle types
    //     std::vector<int> types;
    //     from >> tag;
    //     while (tag != "zeitluecke" && tag != "durch" && tag != "DATAEND" && tag != "alle") {
    //         tag = readEndSecure(from);
    //         if (tag != "DATAEND") {
    //             if (tag == "alle") {
    //                 types.push_back(-1);
    //                 from >> tag;
    //                 tag = "alle";
    //             } else if (tag != "zeitluecke" && tag != "durch" && tag != "DATAEND") {
    //                 int tmp = StringUtils::toInt(tag);
    //                 types.push_back(tmp);
    //             }
    //         }
    //     }
    //     return NIVissimExtendedEdgePoint(edgeid, lanes, position, types);
    // }
    #[allow(unused_variables)]
    pub fn parse_pos(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> NIVissimExtendedEdgePoint {
        let edgeid = read_i32(from); // type-checking is missing!
        //
        let mut tag = read_token(from);
        tag = read_token(from);
        let mut lanes: Vec<i32> = Vec::new();
        if tag == "ALLE" {
            //lanes.push_back(1); // !!!
        } else {
            lanes.push(to_int(&tag).unwrap_or_else(|e| panic!("{e}")));
        }
        //
        tag = read_token(from);
        let position = read_f64(from);
        // assigned vehicle types
        let mut types: Vec<i32> = Vec::new();
        tag = read_token(from);
        while tag != "zeitluecke" && tag != "durch" && tag != "DATAEND" && tag != "alle" {
            tag = self.read_end_secure(from, "");
            if tag != "DATAEND" {
                if tag == "alle" {
                    types.push(-1);
                    tag = read_token(from);
                    tag = "alle".to_string();
                } else if tag != "zeitluecke" && tag != "durch" && tag != "DATAEND" {
                    let tmp = to_int(&tag).unwrap_or_else(|e| panic!("{e}"));
                    types.push(tmp);
                }
            }
        }
        NIVissimExtendedEdgePoint::new(edgeid, &lanes, position, &types)
    }

}

