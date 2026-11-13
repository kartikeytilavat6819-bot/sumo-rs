//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Verbindungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `from >> tag` for a std::string: skips whitespace and reads one token without lower-casing it
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

/// C++ class `NIVissimSingleTypeParser_Verbindungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Verbindungsdefinition.h)
#[repr(C)]
pub struct NIVissimSingleTypeParser_Verbindungsdefinition {
    /// C++ base class `NIImporter_Vissim::VissimSingleTypeParser`
    pub base: NIImporter_Vissim_VissimSingleTypeParser,
}

impl std::ops::Deref for NIVissimSingleTypeParser_Verbindungsdefinition {
    type Target = NIImporter_Vissim_VissimSingleTypeParser;
    fn deref(&self) -> &NIImporter_Vissim_VissimSingleTypeParser {
        &self.base
    }
}
impl std::ops::DerefMut for NIVissimSingleTypeParser_Verbindungsdefinition {
    fn deref_mut(&mut self) -> &mut NIImporter_Vissim_VissimSingleTypeParser {
        &mut self.base
    }
}

impl NIVissimSingleTypeParser_Verbindungsdefinition {
    /// C++: `void NIVissimSingleTypeParser_Verbindungsdefinition::NIVissimSingleTypeParser_Verbindungsdefinition(NIImporter_Vissim &)`
    /// Constructor
    // NIVissimSingleTypeParser_Verbindungsdefinition::NIVissimSingleTypeParser_Verbindungsdefinition(NIImporter_Vissim& parent)
    //     : NIImporter_Vissim::VissimSingleTypeParser(parent) {}
    #[allow(unused_variables)]
    pub fn new(parent: &mut NIImporter_Vissim) -> Self {
        Self { base: NIImporter_Vissim_VissimSingleTypeParser::new(parent) }
    }

    /// C++: `void NIVissimSingleTypeParser_Verbindungsdefinition::~NIVissimSingleTypeParser_Verbindungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Verbindungsdefinition::~NIVissimSingleTypeParser_Verbindungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool NIVissimSingleTypeParser_Verbindungsdefinition::parse(std::istream &)` [virtual]
    /// Parses the data type from the given stream
    // bool
    // NIVissimSingleTypeParser_Verbindungsdefinition::parse(std::istream& from) {
    //     int id;
    //     from >> id; // type-checking is missing!
    //     std::string tag;
    //     // Read optional value "Name", skip optional value "Beschriftung"
    //     std::string name;
    //     while (tag != "von") {
    //         tag = overrideOptionalLabel(from);
    //         if (tag == "name") {
    //             name = readName(from);
    //         }
    //     }
    //     // Read the geometry information
    //     NIVissimExtendedEdgePoint from_def = readExtEdgePointDef(from);
    //     PositionVector geom;
    //     tag = myRead(from); // "ueber"
    //     while (tag != "nach") {
    //         std::string x = myRead(from);
    //         std::string y = myRead(from);
    //         if (y != "nach") {
    //             geom.push_back_noDoublePos(
    //                 Position(
    //                     StringUtils::toDouble(x),
    //                     StringUtils::toDouble(y)
    //                 ));
    //             tag = myRead(from);
    //             try {
    //                 StringUtils::toDouble(tag);
    //                 tag = myRead(from);
    //             } catch (NumberFormatException&) {}
    //         } else {
    //             tag = y;
    //         }
    //     }
    //     NIVissimExtendedEdgePoint to_def = readExtEdgePointDef(from);
    //     // read some optional values until mandatory "Fahrzeugklassen" occurs
    //     double dxnothalt = 0;
    //     double dxeinordnen = 0;
    //     double zuschlag1, zuschlag2;
    //     zuschlag1 = zuschlag2 = 0;
    //     double seglength = 0;
    //     tag = myRead(from);
    // //    NIVissimConnection::Direction direction = NIVissimConnection::NIVC_DIR_ALL;
    //     while (tag != "fahrzeugklassen" && tag != "sperrung" && tag != "auswertung" && tag != "DATAEND") {
    //         if (tag == "rechts") {
    // //            direction = NIVissimConnection::NIVC_DIR_RIGHT;
    //         } else if (tag == "links") {
    // //            direction = NIVissimConnection::NIVC_DIR_LEFT;
    //         } else if (tag == "alle") {
    // //            direction = NIVissimConnection::NIVC_DIR_ALL;
    //         } else if (tag == "dxnothalt") {
    //             from >> dxnothalt; // type-checking is missing!
    //         } else if (tag == "dxeinordnen") {
    //             from >> dxeinordnen; // type-checking is missing!
    //         } else if (tag == "segment") {
    //             from >> tag;
    //             from >> seglength;
    //         }
    //         if (tag == "zuschlag") {
    //             from >> zuschlag1; // type-checking is missing!
    //             tag = readEndSecure(from);
    //             if (tag == "zuschlag") {
    //                 from >> zuschlag2; // type-checking is missing!
    //                 tag = readEndSecure(from, "auswertung");
    //             }
    //         } else {
    //             tag = readEndSecure(from, "auswertung");
    //         }
    //     }
    //     // read in allowed vehicle classes
    //     std::vector<int> assignedVehicles;
    //     if (tag == "fahrzeugklassen") {
    //         tag = readEndSecure(from);
    //         while (tag != "DATAEND" && tag != "sperrung" && tag != "auswertung") {
    //             int classes = StringUtils::toInt(tag);
    //             assignedVehicles.push_back(classes);
    //             tag = readEndSecure(from, "auswertung");
    //         }
    //     }
    //     // Read definitions of closed lanes
    //     NIVissimClosedLanesVector clv;
    //     if (tag != "DATAEND") {
    //         do {
    //             // check whether a next close lane definition can be found
    //             tag = readEndSecure(from);
    //             if (tag == "keinspurwechsel") {
    //                 while (tag != "DATAEND") {
    //                     tag = readEndSecure(from);
    //                 }
    //             } else if (tag == "spur") {
    //                 // get the lane number
    // //                from >> tag;
    //                 int laneNo;
    //                 from >> laneNo; // unused and type-checking is missing!
    //                 // get the list of assigned car classes
    //                 std::vector<int> assigned;
    //                 tag = myRead(from);
    //                 if (tag == "fahrzeugklassen") {
    //                     tag = myRead(from);
    //                 }
    //                 while (tag != "DATAEND" && tag != "spur" && tag != "keinspurwechsel") {
    //                     int classes = StringUtils::toInt(tag);
    //                     assigned.push_back(classes);
    //                     tag = readEndSecure(from);
    //                 }
    //                 // build and add the definition
    //                 NIVissimClosedLaneDef* cld = new NIVissimClosedLaneDef(assigned);
    //                 clv.push_back(cld);
    //             }
    //         } while (tag != "DATAEND");
    //     }
    //     NIVissimConnection* c = new NIVissimConnection(id, name, from_def, to_def, geom,
    //             assignedVehicles, clv);
    //
    //     if (!NIVissimConnection::dictionary(id, c)) {
    //         return false;
    //     }
    //     return true;
    //     //return NIVissimAbstractEdge::dictionary(id, c);
    // }
    #[allow(unused_variables)]
    pub fn parse(&mut self, from: &mut std::io::Cursor<Vec<u8>> /* std::istream */) -> bool {
        // C++ `from >> id` / `from >> double` read one token via myRead (lowercased, which number parsing ignores)
        let id: i32 = self.my_read(from).parse::<i32>().unwrap_or(0);
        let mut tag: String = String::new();
        // Read optional value "Name", skip optional value "Beschriftung"
        let mut name: String = String::new();
        while tag != "von" {
            tag = self.override_optional_label(from, "");
            if tag == "name" {
                name = self.read_name(from);
            }
        }
        // Read the geometry information
        let from_def = self.read_ext_edge_point_def(from);
        let mut geom = PositionVector::default();
        tag = self.my_read(from); // "ueber"
        while tag != "nach" {
            let x = self.my_read(from);
            let y = self.my_read(from);
            if y != "nach" {
                geom.push_back_no_double_pos(&Position::new(
                    utils::common::string_utils::to_double(&x).unwrap_or_else(|e| panic!("{e}")),
                    utils::common::string_utils::to_double(&y).unwrap_or_else(|e| panic!("{e}")),
                ));
                tag = self.my_read(from);
                if utils::common::string_utils::to_double(&tag).is_ok() {
                    tag = self.my_read(from);
                }
            } else {
                tag = y;
            }
        }
        let to_def = self.read_ext_edge_point_def(from);
        // read some optional values until mandatory "Fahrzeugklassen" occurs
        let mut dxnothalt: f64 = 0.0;
        let mut dxeinordnen: f64 = 0.0;
        let mut zuschlag1: f64 = 0.0;
        let mut zuschlag2: f64 = 0.0;
        let mut seglength: f64 = 0.0;
        tag = self.my_read(from);
        // rechts / links / alle only select a direction, which is not used here
        while tag != "fahrzeugklassen" && tag != "sperrung" && tag != "auswertung" && tag != "DATAEND" {
            if tag == "dxnothalt" {
                dxnothalt = self.my_read(from).parse::<f64>().unwrap_or(0.0);
            } else if tag == "dxeinordnen" {
                dxeinordnen = self.my_read(from).parse::<f64>().unwrap_or(0.0);
            } else if tag == "segment" {
                tag = read_raw_token(from);
                seglength = self.my_read(from).parse::<f64>().unwrap_or(0.0);
            }
            if tag == "zuschlag" {
                zuschlag1 = self.my_read(from).parse::<f64>().unwrap_or(0.0);
                tag = self.read_end_secure(from, "");
                if tag == "zuschlag" {
                    zuschlag2 = self.my_read(from).parse::<f64>().unwrap_or(0.0);
                    tag = self.read_end_secure(from, "auswertung");
                }
            } else {
                tag = self.read_end_secure(from, "auswertung");
            }
        }
        // read in allowed vehicle classes
        let mut assignedVehicles: Vec<i32> = Vec::new();
        if tag == "fahrzeugklassen" {
            tag = self.read_end_secure(from, "");
            while tag != "DATAEND" && tag != "sperrung" && tag != "auswertung" {
                let classes = utils::common::string_utils::to_int(&tag).unwrap_or_else(|e| panic!("{e}"));
                assignedVehicles.push(classes);
                tag = self.read_end_secure(from, "auswertung");
            }
        }
        // Read definitions of closed lanes
        let mut clv: NIVissimClosedLanesVector = Vec::new();
        if tag != "DATAEND" {
            loop {
                // check whether a next close lane definition can be found
                tag = self.read_end_secure(from, "");
                if tag == "keinspurwechsel" {
                    while tag != "DATAEND" {
                        tag = self.read_end_secure(from, "");
                    }
                } else if tag == "spur" {
                    // get the lane number: unused and type-checking is missing!
                    let _laneNo: i32 = self.my_read(from).parse::<i32>().unwrap_or(0);
                    // get the list of assigned car classes
                    let mut assigned: Vec<i32> = Vec::new();
                    tag = self.my_read(from);
                    if tag == "fahrzeugklassen" {
                        tag = self.my_read(from);
                    }
                    while tag != "DATAEND" && tag != "spur" && tag != "keinspurwechsel" {
                        let classes = utils::common::string_utils::to_int(&tag).unwrap_or_else(|e| panic!("{e}"));
                        assigned.push(classes);
                        tag = self.read_end_secure(from, "");
                    }
                    // build and add the definition
                    let cld = NIVissimClosedLaneDef::new(&assigned);
                    clv.push(Ptr::from_raw(Box::into_raw(Box::new(cld))));
                }
                if tag == "DATAEND" {
                    break;
                }
            }
        }
        let c = NIVissimConnection::new(id, &name, &from_def, &to_def, &geom, &assignedVehicles, &clv);

        if !NIVissimConnection::dictionary(id, Ptr::from_box(&c)) {
            return false;
        }
        // the dictionary keeps the pointer from here on, like the C++ `new` object it stores
        let _ = Box::into_raw(c);
        true
        //return NIVissimAbstractEdge::dictionary(id, c);
    }

}

