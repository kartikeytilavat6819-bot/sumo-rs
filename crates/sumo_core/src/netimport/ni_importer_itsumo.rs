//! Port of netimport/NIImporter_ITSUMO.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `NIImporter_ITSUMO::ItsumoXMLAttr`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_ITSUMO_ItsumoXMLAttr {
    ITSUMO_ATTR_NOTHING = 0,
}

/// C++ enum `NIImporter_ITSUMO::ItsumoXMLTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_ITSUMO_ItsumoXMLTag {
    ITSUMO_TAG_NOTHING = 0,
    ITSUMO_TAG_SIMULATION = 1,
    ITSUMO_TAG_NETWORK_ID = 2,
    ITSUMO_TAG_NETWORK_NAME = 3,
    ITSUMO_TAG_NODES = 4,
    ITSUMO_TAG_NODE = 5,
    ITSUMO_TAG_NODE_ID = 6,
    ITSUMO_TAG_NODE_NAME = 7,
    ITSUMO_TAG_X_COORD = 8,
    ITSUMO_TAG_Y_COORD = 9,
    ITSUMO_TAG_SOURCES = 10,
    ITSUMO_TAG_SINKS = 11,
    ITSUMO_TAG_TRAFFIC_LIGHTS = 12,
    ITSUMO_TAG_STREETS = 13,
    ITSUMO_TAG_STREET = 14,
    ITSUMO_TAG_STREET_ID = 15,
    ITSUMO_TAG_STREET_NAME = 16,
    ITSUMO_TAG_SECTIONS = 17,
    ITSUMO_TAG_SECTION = 18,
    ITSUMO_TAG_SECTION_ID = 19,
    ITSUMO_TAG_SECTION_NAME = 20,
    ITSUMO_TAG_IS_PREFERENCIAL = 21,
    ITSUMO_TAG_DELIMITING_NODE = 22,
    ITSUMO_TAG_LANESETS = 23,
    ITSUMO_TAG_LANESET = 24,
    ITSUMO_TAG_LANESET_ID = 25,
    ITSUMO_TAG_LANESET_POSITION = 26,
    ITSUMO_TAG_START_NODE = 27,
    ITSUMO_TAG_END_NODE = 28,
    ITSUMO_TAG_TURNING_PROBABILITIES = 29,
    ITSUMO_TAG_DIRECTION = 30,
    ITSUMO_TAG_DESTINATION_LANESET = 31,
    ITSUMO_TAG_PROBABILITY = 32,
    ITSUMO_TAG_LANES = 33,
    ITSUMO_TAG_LANE = 34,
    ITSUMO_TAG_LANE_ID = 35,
    ITSUMO_TAG_LANE_POSITION = 36,
    ITSUMO_TAG_MAXIMUM_SPEED = 37,
    ITSUMO_TAG_DECELERATION_PROB = 38,
}

/// A class which parses an ITSUMO file
/// C++ class `NIImporter_ITSUMO::Handler` (netimport/NIImporter_ITSUMO.h)
pub struct NIImporter_ITSUMO_Handler {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    /// The container to fill
    pub myNetBuilder: Ptr<NBNetBuilder>,
    /// A temporary parameter map
    pub myParameter: Unmapped /* Parameterised::Map */,
    pub myCurrentLanes: Vec<Lane>,
    pub myLaneSets: BTreeMap<String, Ptr<MSDriveWay_LaneSet>>,
    pub myCurrentLaneSets: Vec<Ptr<MSDriveWay_LaneSet>>,
    pub mySections: Vec<Ptr<Handler_Section>>,
}

impl std::ops::Deref for NIImporter_ITSUMO_Handler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_ITSUMO_Handler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NIImporter_ITSUMO_Handler {
    /// C++: `void Handler::Handler(NBNetBuilder &)`
    /// Contructor
    // NIImporter_ITSUMO::Handler::Handler(NBNetBuilder& toFill)
    //     : GenericSAXHandler(itsumoTags, ITSUMO_TAG_NOTHING, itsumoAttrs, ITSUMO_ATTR_NOTHING, "itsumo - file"), myNetBuilder(toFill) {
    // }
    #[allow(unused_variables)]
    pub fn new(toFill: &mut NBNetBuilder) -> Self {
        todo!("port Handler::Handler")
    }

    /// C++: `void Handler::~Handler() noexcept` [virtual]
    /// Destructor
    // NIImporter_ITSUMO::Handler::~Handler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Handler::~Handler")
    }

    /// C++: `void Handler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_ITSUMO::Handler::myStartElement(int element, const SUMOSAXAttributes& /* attrs */) {
    //     switch (element) {
    //         case ITSUMO_TAG_NODE:
    //             myParameter.clear();
    //             break;
    //         case ITSUMO_TAG_LANESET:
    //             myParameter.clear();
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port Handler::myStartElement")
    }

    /// C++: `void Handler::myCharacters(int, const std::string &)` [virtual, protected]
    /// Callback method for characters to implement by derived classes
    // void
    // NIImporter_ITSUMO::Handler::myCharacters(int element, const std::string& chars) {
    //     std::string mc = StringUtils::prune(chars);
    //     switch (element) {
    //         // node parsing
    //         case ITSUMO_TAG_NODE_ID:
    //             myParameter["id"] = mc;
    //             break;
    //         case ITSUMO_TAG_NODE_NAME:
    //             myParameter["name"] = mc;
    //             break;
    //         case ITSUMO_TAG_X_COORD:
    //             myParameter["x"] = mc;
    //             break;
    //         case ITSUMO_TAG_Y_COORD:
    //             myParameter["y"] = mc;
    //             break;
    //         // section parsing
    //         case ITSUMO_TAG_SECTION_ID:
    //             myParameter["sectionID"] = mc;
    //             break;
    //         // laneset parsing
    //         case ITSUMO_TAG_LANESET_ID:
    //             myParameter["lanesetID"] = mc;
    //             break;
    //         case ITSUMO_TAG_LANESET_POSITION:
    //             myParameter["pos"] = mc;
    //             break;
    //         case ITSUMO_TAG_START_NODE:
    //             myParameter["from"] = mc;
    //             break;
    //         case ITSUMO_TAG_END_NODE:
    //             myParameter["to"] = mc;
    //             break;
    //         // lane parsing
    //         case ITSUMO_TAG_LANE_ID:
    //             myParameter["laneID"] = mc;
    //             break;
    //         case ITSUMO_TAG_LANE_POSITION:
    //             myParameter["i"] = mc;
    //             break;
    //         case ITSUMO_TAG_MAXIMUM_SPEED:
    //             myParameter["v"] = mc;
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_characters(&mut self, element: i32, chars: &str) {
        todo!("port Handler::myCharacters")
    }

    /// C++: `void Handler::myEndElement(int)` [virtual, protected]
    /// Callback method for a closing tag to implement by derived classes
    // void
    // NIImporter_ITSUMO::Handler::myEndElement(int element) {
    //     switch (element) {
    //         case ITSUMO_TAG_SIMULATION: {
    //             for (std::vector<Section*>::iterator i = mySections.begin(); i != mySections.end(); ++i) {
    //                 for (std::vector<LaneSet*>::iterator j = (*i)->laneSets.begin(); j != (*i)->laneSets.end(); ++j) {
    //                     LaneSet* ls = (*j);
    //                     NBEdge* edge = new NBEdge(ls->id, ls->from, ls->to, "", ls->v, NBEdge::UNSPECIFIED_FRICTION, (int)ls->lanes.size(), -1,
    //                                               NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT);
    //                     if (!myNetBuilder.getEdgeCont().insert(edge)) {
    //                         delete edge;
    //                         WRITE_ERRORF(TL("Could not add edge '%'. Probably declared twice."), ls->id);
    //                     }
    //                     delete ls;
    //                 }
    //                 delete *i;
    //             }
    //         }
    //         break;
    //         case ITSUMO_TAG_NODE: {
    //             try {
    //                 std::string id = myParameter["id"];
    //                 double x = StringUtils::toDouble(myParameter["x"]);
    //                 double y = StringUtils::toDouble(myParameter["y"]);
    //                 Position pos(x, y);
    //                 if (!NBNetBuilder::transformCoordinate(pos)) {
    //                     WRITE_ERRORF(TL("Unable to project coordinates for node '%'."), id);
    //                 }
    //                 NBNode* node = new NBNode(id, pos);
    //                 if (!myNetBuilder.getNodeCont().insert(node)) {
    //                     delete node;
    //                     WRITE_ERRORF(TL("Could not add node '%'. Probably declared twice."), id);
    //                 }
    //             } catch (NumberFormatException&) {
    //                 WRITE_ERRORF(TL("Not numeric position information for node '%'."), myParameter["id"]);
    //             } catch (EmptyData&) {
    //                 WRITE_ERRORF(TL("Missing data in node '%'."), myParameter["id"]);
    //             }
    //         }
    //         break;
    //         case ITSUMO_TAG_SECTION: {
    //             mySections.push_back(new Section(myParameter["sectionID"], myCurrentLaneSets));
    //             myCurrentLaneSets.clear();
    //         }
    //         break;
    //         case ITSUMO_TAG_LANESET: {
    //             try {
    //                 std::string id = myParameter["lanesetID"];
    //                 int i = StringUtils::toInt(myParameter["i"]);
    //                 std::string fromID = myParameter["from"];
    //                 std::string toID = myParameter["to"];
    //                 NBNode* from = myNetBuilder.getNodeCont().retrieve(fromID);
    //                 NBNode* to = myNetBuilder.getNodeCont().retrieve(toID);
    //                 if (from == nullptr || to == nullptr) {
    //                     WRITE_ERRORF(TL("Missing node in laneset '%'."), myParameter["lanesetID"]);
    //                 } else {
    //                     if (myLaneSets.find(id) != myLaneSets.end()) {
    //                         WRITE_ERRORF(TL("Found laneset-id '%' twice."), id);
    //                     } else {
    //                         double vSum = 0;
    //                         for (std::vector<Lane>::iterator j = myCurrentLanes.begin(); j != myCurrentLanes.end(); ++j) {
    //                             vSum += (*j).v;
    //                         }
    //                         vSum /= (double) myCurrentLanes.size();
    //                         LaneSet* ls = new LaneSet(id, myCurrentLanes, vSum, i, from, to);
    //                         myLaneSets[id] = ls;
    //                         myCurrentLaneSets.push_back(ls);
    //                         myCurrentLanes.clear();
    //                     }
    //                 }
    //             } catch (NumberFormatException&) {
    //                 WRITE_ERRORF(TL("Not numeric value in laneset '%'."), myParameter["lanesetID"]);
    //             } catch (EmptyData&) {
    //                 WRITE_ERRORF(TL("Missing data in laneset '%'."), myParameter["lanesetID"]);
    //             }
    //         }
    //         break;
    //         case ITSUMO_TAG_LANE: {
    //             try {
    //                 std::string id = myParameter["laneID"];
    //                 int i = StringUtils::toInt(myParameter["i"]);
    //                 double v = StringUtils::toDouble(myParameter["v"]);
    //                 myCurrentLanes.push_back(Lane(id, (int) i, v));
    //             } catch (NumberFormatException&) {
    //                 WRITE_ERRORF(TL("Not numeric value in lane '%'."), myParameter["laneID"]);
    //             } catch (EmptyData&) {
    //                 WRITE_ERRORF(TL("Missing data in lane '%'."), myParameter["laneID"]);
    //             }
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        todo!("port Handler::myEndElement")
    }

    /// C++: `void Handler::Handler(const Handler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_ITSUMO_Handler) -> Self {
        todo!("port Handler::Handler")
    }

    /// C++: `Handler & Handler::operator=(const Handler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_ITSUMO_Handler) -> &mut NIImporter_ITSUMO_Handler {
        todo!("port Handler::operator=")
    }

}

/// C++ class `Handler::Lane` (netimport/NIImporter_ITSUMO.h)
pub struct Handler_Lane {
    pub id: String,
    pub index: i32,
    pub v: f64,
}

impl Handler_Lane {
    /// C++: `void Lane::Lane(const std::string &, int, double)`
    // Lane(const std::string& _id, int _idx, double _v)
    //                 : id(_id), index(_idx), v(_v) {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _idx: i32, _v: f64) -> Self {
        todo!("port Lane::Lane")
    }

}

/// C++ class `Handler::LaneSet` (netimport/NIImporter_ITSUMO.h)
pub struct Handler_LaneSet {
    pub id: String,
    pub lanes: Vec<Lane>,
    pub v: f64,
    pub position: i32,
    pub from: Ptr<NBNode>,
    pub to: Ptr<NBNode>,
}

impl Handler_LaneSet {
    /// C++: `void LaneSet::LaneSet(const std::string &, const std::vector<Lane> &, double, int, NBNode *, NBNode *)`
    // LaneSet(const std::string& _id, const std::vector<Lane>& _lanes, double _v, int _pos, NBNode* _from, NBNode* _to)
    //                 : id(_id), lanes(_lanes), v(_v), position(_pos), from(_from), to(_to) {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _lanes: &Vec<Lane>, _v: f64, _pos: i32, _from: Ptr<NBNode>, _to: Ptr<NBNode>) -> Self {
        todo!("port LaneSet::LaneSet")
    }

}

/// Importer for networks stored in ITSUMO format
/// C++ class `NIImporter_ITSUMO` (netimport/NIImporter_ITSUMO.h)
pub struct NIImporter_ITSUMO {
}

// C++ static members of NIImporter_ITSUMO (port as module-level statics, see AGENTS.md):
//   static SequentialStringBijection::Entry[] NIImporter_ITSUMO::itsumoTags;
//   static SequentialStringBijection::Entry[] NIImporter_ITSUMO::itsumoAttrs;

impl NIImporter_ITSUMO {
    /// C++: `void NIImporter_ITSUMO::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given ITSUMO network files
    // void
    // NIImporter_ITSUMO::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether the option is set (properly)
    //     if (!oc.isSet("itsumo-files")) {
    //         return;
    //     }
    //     /* Parse file(s)
    //      * Each file is parsed twice: first for nodes, second for edges. */
    //     std::vector<std::string> files = oc.getStringVector("itsumo-files");
    //     // load nodes, first
    //     Handler handler(nb);
    //     handler.needsCharacterData();
    //     for (std::vector<std::string>::const_iterator file = files.begin(); file != files.end(); ++file) {
    //         // nodes
    //         if (!FileHelpers::isReadable(*file)) {
    //             WRITE_ERRORF(TL("Could not open itsumo-file '%'."), *file);
    //             return;
    //         }
    //         handler.setFileName(*file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing nodes from itsumo-file '" + *file + "'");
    //         if (!XMLSubSys::runParser(handler, *file)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        todo!("port NIImporter_ITSUMO::loadNetwork")
    }

}

/// C++ class `Handler::Section` (netimport/NIImporter_ITSUMO.h)
pub struct Handler_Section {
    pub id: String,
    pub laneSets: Vec<Ptr<MSDriveWay_LaneSet>>,
}

impl Handler_Section {
    /// C++: `void Section::Section(const std::string &, const std::vector<LaneSet *> &)`
    // Section(const std::string& _id, const std::vector<LaneSet*>& _laneSets)
    //                 : id(_id), laneSets(_laneSets) {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _laneSets: &Vec<Ptr<MSDriveWay_LaneSet>>) -> Self {
        todo!("port Section::Section")
    }

}

