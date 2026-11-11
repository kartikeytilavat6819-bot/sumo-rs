//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Verbindungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Verbindungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Verbindungsdefinition.h)
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
        todo!("port NIVissimSingleTypeParser_Verbindungsdefinition::NIVissimSingleTypeParser_Verbindungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Verbindungsdefinition::~NIVissimSingleTypeParser_Verbindungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Verbindungsdefinition::~NIVissimSingleTypeParser_Verbindungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Verbindungsdefinition::~NIVissimSingleTypeParser_Verbindungsdefinition")
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
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Verbindungsdefinition::parse")
    }

}

