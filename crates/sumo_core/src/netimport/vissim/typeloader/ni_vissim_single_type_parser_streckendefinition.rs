//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Streckendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::string_utils::{to_double, to_int};

/// C++ class `NIVissimSingleTypeParser_Streckendefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Streckendefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Streckendefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Streckendefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Streckendefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Streckendefinition {
    /// C++: `void NIVissimSingleTypeParser_Streckendefinition::NIVissimSingleTypeParser_Streckendefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Streckendefinition::NIVissimSingleTypeParser_Streckendefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Streckendefinition::~NIVissimSingleTypeParser_Streckendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Streckendefinition::~NIVissimSingleTypeParser_Streckendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Streckendefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Streckendefinition::parse(std::istream& from) {
    //     // read in the id
    //     int id;
    //     from >> id;
    //     //
    //     std::string tag;
    //     // the following elements may occur: "Name", "Beschriftung", "Typ",
    //     //  followed by the mandatory "Laenge"
    //     std::string name, label, type;
    //     double length = -1;
    //     while (length < 0) {
    //         tag = overrideOptionalLabel(from);
    //         if (tag == "name") {
    //             name = readName(from);
    //         } else if (tag == "typ") {
    //             type = myRead(from);
    //         } else if (tag == "laenge") {
    //             from >> length; // type-checking is missing!
    //         }
    //     }
    //     // read in the number of lanes
    //     int noLanes;
    //     tag = myRead(from);
    //     from >> noLanes;
    //     // skip some parameter, except optional "Zuschlag" until "Von" (mandatory)
    //     //  occurs
    //     double zuschlag1, zuschlag2;
    //     zuschlag1 = zuschlag2 = 0;
    //     while (tag != "von") {
    //         tag = myRead(from);
    //         if (tag == "zuschlag") {
    //             from >> zuschlag1; // type-checking is missing!
    //             tag = myRead(from);
    //             if (tag == "zuschlag") {
    //                 from >> zuschlag2; // type-checking is missing!
    //             }
    //         }
    //     }
    //     // Read the geometry information
    //     PositionVector geom;
    //     while (tag != "nach") {
    //         geom.push_back_noDoublePos(getPosition(from));
    //         tag = myRead(from);
    //         try {
    //             StringUtils::toDouble(tag);
    //             tag = myRead(from);
    //         } catch (NumberFormatException&) {}
    //     }
    //     geom.push_back_noDoublePos(getPosition(from));
    //     // Read definitions of closed lanes
    //     NIVissimClosedLanesVector clv;
    //     // check whether a next close lane definition can be found
    //     tag = readEndSecure(from);
    //     while (tag != "DATAEND") {
    //         if (tag == "keinspurwechsel") {
    //             while (tag != "DATAEND") {
    //                 tag = readEndSecure(from);
    //             }
    //         } else if (tag == "spur") {
    //             // get the lane number
    //             int laneNo;
    //             from >> laneNo; // unused and type-checking is missing!
    //             // get the list of assigned car classes
    //             std::vector<int> assignedVehicles;
    //             tag = myRead(from);
    //             tag = myRead(from);
    //             while (tag != "DATAEND" && tag != "spur" && tag != "keinspurwechsel") {
    //                 int classes = StringUtils::toInt(tag);
    //                 assignedVehicles.push_back(classes);
    //                 tag = readEndSecure(from);
    //             }
    //             // build and add the definition
    //             NIVissimClosedLaneDef* cld = new NIVissimClosedLaneDef(assignedVehicles);
    //             clv.push_back(cld);
    //         } else {
    //             tag = readEndSecure(from);
    //         }
    //     }
    //     NIVissimEdge* e = new NIVissimEdge(id, name, type, std::vector<double>(noLanes, NBEdge::UNSPECIFIED_WIDTH),
    //                                        zuschlag1, zuschlag2, length, geom, clv);
    //     if (!NIVissimEdge::dictionary(id, e)) {
    //         return false;
    //     }
    //     return true;
    //     //return NIVissimAbstractEdge::dictionary(id, e);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // read in the id
        let id = read_i32(from);
        //
        let mut tag: String;
        // the following elements may occur: "Name", "Beschriftung", "Typ",
        //  followed by the mandatory "Laenge"
        let mut name = String::new();
        let mut type_ = String::new();
        let mut length: f64 = -1.0;
        while length < 0.0 {
            tag = self.override_optional_label(from, "");
            if tag == "name" {
                name = self.read_name(from);
            } else if tag == "typ" {
                type_ = self.my_read(from);
            } else if tag == "laenge" {
                length = read_f64(from); // type-checking is missing!
            }
        }
        // read in the number of lanes
        tag = self.my_read(from);
        let noLanes = read_i32(from);
        // skip some parameter, except optional "Zuschlag" until "Von" (mandatory)
        //  occurs
        let mut zuschlag1 = 0.0;
        let mut zuschlag2 = 0.0;
        while tag != "von" {
            tag = self.my_read(from);
            if tag == "zuschlag" {
                zuschlag1 = read_f64(from); // type-checking is missing!
                tag = self.my_read(from);
                if tag == "zuschlag" {
                    zuschlag2 = read_f64(from); // type-checking is missing!
                }
            }
        }
        // Read the geometry information
        let mut geom = PositionVector::new();
        while tag != "nach" {
            geom.push_back_no_double_pos(&self.get_position(from));
            tag = self.my_read(from);
            if to_double(&tag).is_ok() {
                tag = self.my_read(from);
            }
        }
        geom.push_back_no_double_pos(&self.get_position(from));
        // Read definitions of closed lanes
        let mut clv: NIVissimClosedLanesVector = Vec::new();
        // check whether a next close lane definition can be found
        tag = self.read_end_secure(from, "");
        while tag != "DATAEND" {
            if tag == "keinspurwechsel" {
                while tag != "DATAEND" {
                    tag = self.read_end_secure(from, "");
                }
            } else if tag == "spur" {
                // get the lane number
                let _laneNo = read_i32(from); // unused and type-checking is missing!
                // get the list of assigned car classes
                let mut assignedVehicles: Vec<i32> = Vec::new();
                tag = self.my_read(from);
                tag = self.my_read(from);
                while tag != "DATAEND" && tag != "spur" && tag != "keinspurwechsel" {
                    let classes = to_int(&tag).unwrap_or_else(|e| panic!("{e}"));
                    assignedVehicles.push(classes);
                    tag = self.read_end_secure(from, "");
                }
                // build and add the definition
                // shortcut: the closed lane definition is owned by the edge for the rest of the run (C++ never frees it), so leak the Box
                let cld = NIVissimClosedLaneDef::new(&assignedVehicles);
                clv.push(Ptr::from_mut(Box::leak(Box::new(cld))));
            } else {
                tag = self.read_end_secure(from, "");
            }
        }
        // shortcut: the edge is owned by the dictionary for the rest of the run (C++ deletes it in clearDict, not yet ported), so leak the Box even when the dictionary rejects it, as C++ does
        let e = Box::leak(NIVissimEdge::new(id, &name, &type_, vec![crate::netbuild::nb_edge::UNSPECIFIED_WIDTH; noLanes as usize], zuschlag1, zuschlag2, length, &geom, &clv));
        if !NIVissimEdge::dictionary_2(id, Ptr::from_mut(e)) {
            return false;
        }
        true
    }

}

/// C++ `from >> tag` for a std::string: skips whitespace, reads one whitespace-delimited token ("" at end of stream)
fn read_token(from: &mut std::io::Cursor<Vec<u8>>) -> String {
    let data = from.get_ref();
    let mut i = from.position() as usize;
    while i < data.len() && matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let start = i;
    while i < data.len() && !matches!(data[i], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c) {
        i += 1;
    }
    let tok = vissim_bytes_to_string(&data[start..i]);
    from.set_position(i as u64);
    tok
}

/// C++ `from >> int`: a number that does not parse gives 0 (C++11 behaviour)
fn read_i32(from: &mut std::io::Cursor<Vec<u8>>) -> i32 {
    crate::netimport::vissim::ni_importer_vissim::vissim_read_number(from, false).parse::<i32>().unwrap_or(0)
}

/// C++ `from >> double` (see read_i32)
fn read_f64(from: &mut std::io::Cursor<Vec<u8>>) -> f64 {
    crate::netimport::vissim::ni_importer_vissim::vissim_read_number(from, true).parse::<f64>().unwrap_or(0.0)
}

