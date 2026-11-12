//! Port of netimport/vissim/typeloader/NIVissimSingleTypeParser_Streckendefinition.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port NIVissimSingleTypeParser_Streckendefinition::NIVissimSingleTypeParser_Streckendefinition")
    }

    /// C++: `void NIVissimSingleTypeParser_Streckendefinition::~NIVissimSingleTypeParser_Streckendefinition() noexcept` [virtual]
    /// Destructor
    // NIVissimSingleTypeParser_Streckendefinition::~NIVissimSingleTypeParser_Streckendefinition() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NIVissimSingleTypeParser_Streckendefinition::~NIVissimSingleTypeParser_Streckendefinition")
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
    pub fn parse(&mut self, from: &mut Unmapped /* std::istream */) -> bool {
        todo!("port NIVissimSingleTypeParser_Streckendefinition::parse")
    }

}

