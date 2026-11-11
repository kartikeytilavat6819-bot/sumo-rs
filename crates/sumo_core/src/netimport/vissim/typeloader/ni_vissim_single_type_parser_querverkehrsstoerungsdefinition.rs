//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition` (netimport/vissim/typeloader/NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition.h)
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
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::~NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::~NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::~NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition")
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
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parse")
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
    pub fn parse_only_me(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseOnlyMe")
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
    pub fn parse_position_described(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePositionDescribed")
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
    pub fn parse_numbered(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parseNumbered")
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
    pub fn parse_pos(&mut self, from: &mut Unmapped /* std::istream */) -> NIVissimExtendedEdgePoint {
        todo!("port NIVissimSingleTypeParser_Querverkehrsstoerungsdefinition::parsePos")
    }

}

