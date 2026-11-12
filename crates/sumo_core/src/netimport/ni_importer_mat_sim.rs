//! Port of netimport/NIImporter_MATSim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{progress_begin_message, progress_done_message, write_error, write_warning};
use utils::common::string_tokenizer::StringTokenizer;

// C++ enum values as plain constants (used by the element/attribute switches)
const MATSIM_TAG_NOTHING: i32 = NIImporter_MATSim_MatsimXMLTag::MATSIM_TAG_NOTHING as i32;
const MATSIM_TAG_NETWORK: i32 = NIImporter_MATSim_MatsimXMLTag::MATSIM_TAG_NETWORK as i32;
const MATSIM_TAG_NODE: i32 = NIImporter_MATSim_MatsimXMLTag::MATSIM_TAG_NODE as i32;
const MATSIM_TAG_LINK: i32 = NIImporter_MATSim_MatsimXMLTag::MATSIM_TAG_LINK as i32;
const MATSIM_TAG_LINKS: i32 = NIImporter_MATSim_MatsimXMLTag::MATSIM_TAG_LINKS as i32;
const MATSIM_ATTR_NOTHING: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_NOTHING as i32;
const MATSIM_ATTR_ID: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_ID as i32;
const MATSIM_ATTR_X: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_X as i32;
const MATSIM_ATTR_Y: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_Y as i32;
const MATSIM_ATTR_FROM: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_FROM as i32;
const MATSIM_ATTR_TO: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_TO as i32;
const MATSIM_ATTR_LENGTH: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_LENGTH as i32;
const MATSIM_ATTR_FREESPEED: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_FREESPEED as i32;
const MATSIM_ATTR_CAPACITY: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_CAPACITY as i32;
const MATSIM_ATTR_PERMLANES: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_PERMLANES as i32;
const MATSIM_ATTR_ONEWAY: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_ONEWAY as i32;
const MATSIM_ATTR_MODES: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_MODES as i32;
const MATSIM_ATTR_ORIGID: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_ORIGID as i32;
const MATSIM_ATTR_CAPPERIOD: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_CAPPERIOD as i32;
const MATSIM_ATTR_CAPDIVIDER: i32 = NIImporter_MATSim_MatsimXMLAttr::MATSIM_ATTR_CAPDIVIDER as i32;
// NBEdge static members (not ported as Rust consts yet); values from NBEdge.cpp
const NBEDGE_UNSPECIFIED_WIDTH: f64 = -1.0;
const NBEDGE_UNSPECIFIED_OFFSET: f64 = 0.0;
const NBEDGE_UNSPECIFIED_FRICTION: f64 = 1.0;

// C++ static members NIImporter_MATSim::matsimTags / matsimAttrs (module-level statics, see AGENTS.md)
static MATSIM_TAGS: &[(&str, i32)] = &[
    ("network", MATSIM_TAG_NETWORK),
    ("node", MATSIM_TAG_NODE),
    ("link", MATSIM_TAG_LINK),
    ("links", MATSIM_TAG_LINKS),
];
static MATSIM_ATTRS: &[(&str, i32)] = &[
    ("id", MATSIM_ATTR_ID),
    ("x", MATSIM_ATTR_X),
    ("y", MATSIM_ATTR_Y),
    ("from", MATSIM_ATTR_FROM),
    ("to", MATSIM_ATTR_TO),
    ("length", MATSIM_ATTR_LENGTH),
    ("freespeed", MATSIM_ATTR_FREESPEED),
    ("capacity", MATSIM_ATTR_CAPACITY),
    ("permlanes", MATSIM_ATTR_PERMLANES),
    ("oneway", MATSIM_ATTR_ONEWAY),
    ("modes", MATSIM_ATTR_MODES),
    ("origid", MATSIM_ATTR_ORIGID),
    ("capperiod", MATSIM_ATTR_CAPPERIOD),
    ("capDivider", MATSIM_ATTR_CAPDIVIDER),
];

/// C++ enum `NIImporter_MATSim::MatsimXMLAttr`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_MATSim_MatsimXMLAttr {
    MATSIM_ATTR_NOTHING = 0,
    MATSIM_ATTR_ID = 1,
    MATSIM_ATTR_X = 2,
    MATSIM_ATTR_Y = 3,
    MATSIM_ATTR_FROM = 4,
    MATSIM_ATTR_TO = 5,
    MATSIM_ATTR_LENGTH = 6,
    MATSIM_ATTR_FREESPEED = 7,
    MATSIM_ATTR_CAPACITY = 8,
    MATSIM_ATTR_PERMLANES = 9,
    MATSIM_ATTR_ONEWAY = 10,
    MATSIM_ATTR_MODES = 11,
    MATSIM_ATTR_ORIGID = 12,
    MATSIM_ATTR_CAPPERIOD = 13,
    MATSIM_ATTR_CAPDIVIDER = 14,
}

/// C++ enum `NIImporter_MATSim::MatsimXMLTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NIImporter_MATSim_MatsimXMLTag {
    MATSIM_TAG_NOTHING = 0,
    MATSIM_TAG_NETWORK = 1,
    MATSIM_TAG_NODE = 2,
    MATSIM_TAG_LINK = 3,
    MATSIM_TAG_LINKS = 4,
}

/// A class which extracts MATSIM-edges from a parsed MATSIM-file
/// C++ class `NIImporter_MATSim::EdgesHandler` (netimport/NIImporter_MATSim.h)
#[repr(C)]
pub struct NIImporter_MATSim_EdgesHandler {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    /// The previously parsed nodes
    pub myNodeCont: Ptr<NBNodeCont>,
    /// The edge container to fill
    pub myEdgeCont: Ptr<NBEdgeCont>,
    /// The capacity norming
    pub myCapacityNorm: f64,
    /// Whether the loaded lengths shal be used
    pub myKeepEdgeLengths: bool,
    /// Whether the lane number shall be computed from the capacity
    pub myLanesFromCapacity: bool,
    /// The converter from flow to lanes
    pub myCapacity2Lanes: NBCapacity2Lanes,
}

impl std::ops::Deref for NIImporter_MATSim_EdgesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_MATSim_EdgesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl SaxHandler for NIImporter_MATSim_EdgesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_MATSim_EdgesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}

impl NIImporter_MATSim_EdgesHandler {
    /// C++: `void EdgesHandler::EdgesHandler(NBNodeCont &, NBEdgeCont &, bool, bool, NBCapacity2Lanes)`
    /// Constructor
    // NIImporter_MATSim::EdgesHandler::EdgesHandler(NBNodeCont& nc, NBEdgeCont& toFill,
    //         bool keepEdgeLengths, bool lanesFromCapacity,
    //         NBCapacity2Lanes capacity2Lanes)
    //     : GenericSAXHandler(matsimTags, MATSIM_TAG_NOTHING,
    //                         matsimAttrs, MATSIM_ATTR_NOTHING, "matsim - file"),
    //       myNodeCont(nc), myEdgeCont(toFill), myCapacityNorm(3600),
    //       myKeepEdgeLengths(keepEdgeLengths), myLanesFromCapacity(lanesFromCapacity),
    //       myCapacity2Lanes(capacity2Lanes) {
    // }
    #[allow(unused_variables)]
    pub fn new(nc: &mut NBNodeCont, toFill: &mut NBEdgeCont, keepEdgeLengths: bool, lanesFromCapacity: bool, capacity2Lanes: NBCapacity2Lanes) -> Self {
        NIImporter_MATSim_EdgesHandler {
            base: GenericSAXHandler::new(MATSIM_TAGS, MATSIM_TAG_NOTHING, MATSIM_ATTRS, MATSIM_ATTR_NOTHING, "matsim - file", ""),
            myNodeCont: Ptr::from_mut(nc),
            myEdgeCont: Ptr::from_mut(toFill),
            myCapacityNorm: 3600.0,
            myKeepEdgeLengths: keepEdgeLengths,
            myLanesFromCapacity: lanesFromCapacity,
            myCapacity2Lanes: capacity2Lanes,
        }
    }

    /// C++: `void EdgesHandler::~EdgesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_MATSim::EdgesHandler::~EdgesHandler() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has an empty body
    }

    /// C++: `void EdgesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_MATSim::EdgesHandler::myStartElement(int element,
    //         const SUMOSAXAttributes& attrs) {
    //     if (element == MATSIM_TAG_NETWORK) {
    //         if (attrs.hasAttribute(MATSIM_ATTR_CAPDIVIDER)) {
    //             bool ok = true;
    //             int capDivider = attrs.get<int>(MATSIM_ATTR_CAPDIVIDER, "network", ok);
    //             if (ok) {
    //                 myCapacityNorm = (double)(capDivider * 3600);
    //             }
    //         }
    //     }
    //     if (element == MATSIM_TAG_LINKS) {
    //         bool ok = true;
    //         std::string capperiod = attrs.get<std::string>(MATSIM_ATTR_CAPPERIOD, "links", ok);
    //         StringTokenizer st(capperiod, ":");
    //         if (st.size() != 3) {
    //             WRITE_ERROR(TL("Bogus capacity period format; requires 'hh:mm:ss'."));
    //             return;
    //         }
    //         try {
    //             int hours = StringUtils::toInt(st.next());
    //             int minutes = StringUtils::toInt(st.next());
    //             int seconds = StringUtils::toInt(st.next());
    //             myCapacityNorm = (double)(hours * 3600 + minutes * 60 + seconds);
    //         } catch (NumberFormatException&) {
    //         } catch (EmptyData&) {
    //         }
    //         return;
    //     }
    //
    //     // parse "link" elements
    //     if (element != MATSIM_TAG_LINK) {
    //         return;
    //     }
    //     bool ok = true;
    //     const std::string id = SUMOXMLDefinitions::makeValidID(attrs.get<std::string>(MATSIM_ATTR_ID, nullptr, ok));
    //     const std::string fromNodeID = SUMOXMLDefinitions::makeValidID(attrs.get<std::string>(MATSIM_ATTR_FROM, id.c_str(), ok));
    //     const std::string toNodeID = SUMOXMLDefinitions::makeValidID(attrs.get<std::string>(MATSIM_ATTR_TO, id.c_str(), ok));
    //     const double length = attrs.get<double>(MATSIM_ATTR_LENGTH, id.c_str(), ok); // override computed?
    //     const double freeSpeed = attrs.get<double>(MATSIM_ATTR_FREESPEED, id.c_str(), ok); //
    //     const double capacity = attrs.get<double>(MATSIM_ATTR_CAPACITY, id.c_str(), ok); // override permLanes?
    //     double permLanes = attrs.get<double>(MATSIM_ATTR_PERMLANES, id.c_str(), ok);
    //     //bool oneWay = attrs.getOpt<bool>(MATSIM_ATTR_ONEWAY, id.c_str(), ok, true); // mandatory?
    //     const std::string modes = attrs.getOpt<std::string>(MATSIM_ATTR_MODES, id.c_str(), ok, "");
    //     const std::string origid = attrs.getOpt<std::string>(MATSIM_ATTR_ORIGID, id.c_str(), ok, "");
    //     NBNode* fromNode = myNodeCont.retrieve(fromNodeID);
    //     NBNode* toNode = myNodeCont.retrieve(toNodeID);
    //     if (fromNode == nullptr) {
    //         WRITE_ERRORF(TL("Could not find from-node for edge '%'."), id);
    //     }
    //     if (toNode == nullptr) {
    //         WRITE_ERRORF(TL("Could not find to-node for edge '%'."), id);
    //     }
    //     if (fromNode == nullptr || toNode == nullptr) {
    //         return;
    //     }
    //     if (myLanesFromCapacity) {
    //         permLanes = myCapacity2Lanes.get(capacity);
    //     }
    //     if (permLanes < 0.5) {
    //         WRITE_WARNINGF(TL("Ignoring edge % which has no lanes."), id);
    //         return;
    //     }
    //     if (fromNode == toNode) {
    //         // adding node and edge with a different naming scheme to keep the original edge id for easier route repair
    //         NBNode* intermediate = new NBNode(id + ".0", toNode->getPosition() + Position(POSITION_EPS, POSITION_EPS));
    //         if (myNodeCont.insert(intermediate)) {
    //             insertEdge(id + ".0", intermediate, toNode, freeSpeed, (int)(permLanes + 0.5), capacity, length, computePermission(modes));
    //             toNode = intermediate;
    //         } else {
    //             delete intermediate;
    //             WRITE_ERRORF(TL("Could not add intermediate node to split loop edge '%'."), id);
    //         }
    //     }
    //     insertEdge(id, fromNode, toNode, freeSpeed, (int)(permLanes + 0.5), capacity, length, computePermission(modes));
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == MATSIM_TAG_NETWORK {
            if attrs.has_attribute(MATSIM_ATTR_CAPDIVIDER) {
                let mut ok = true;
                let capDivider: i32 = attrs.get(MATSIM_ATTR_CAPDIVIDER, "network", &mut ok, true);
                if ok {
                    self.myCapacityNorm = (capDivider * 3600) as f64;
                }
            }
        }
        if element == MATSIM_TAG_LINKS {
            let mut ok = true;
            let capperiod: String = attrs.get(MATSIM_ATTR_CAPPERIOD, "links", &mut ok, true);
            let mut st = StringTokenizer::with_token(&capperiod, ":", false);
            if st.size() != 3 {
                write_error("Bogus capacity period format; requires 'hh:mm:ss'.");
                return;
            }
            let parsed = (|| -> Result<(i32, i32, i32), ProcessError> {
                let hours = utils::common::string_utils::to_int(&st.next()?)?;
                let minutes = utils::common::string_utils::to_int(&st.next()?)?;
                let seconds = utils::common::string_utils::to_int(&st.next()?)?;
                Ok((hours, minutes, seconds))
            })();
            match parsed {
                Ok((hours, minutes, seconds)) => {
                    self.myCapacityNorm = (hours * 3600 + minutes * 60 + seconds) as f64;
                }
                // C++ swallows NumberFormatException and EmptyData here
                Err(_) => {}
            }
            return;
        }

        // parse "link" elements
        if element != MATSIM_TAG_LINK {
            return;
        }
        let mut ok = true;
        let id: String = make_valid_id(&attrs.get::<String>(MATSIM_ATTR_ID, "", &mut ok, true));
        let fromNodeID: String = make_valid_id(&attrs.get::<String>(MATSIM_ATTR_FROM, &id, &mut ok, true));
        let toNodeID: String = make_valid_id(&attrs.get::<String>(MATSIM_ATTR_TO, &id, &mut ok, true));
        let length: f64 = attrs.get(MATSIM_ATTR_LENGTH, &id, &mut ok, true); // override computed?
        let freeSpeed: f64 = attrs.get(MATSIM_ATTR_FREESPEED, &id, &mut ok, true); //
        let capacity: f64 = attrs.get(MATSIM_ATTR_CAPACITY, &id, &mut ok, true); // override permLanes?
        let mut permLanes: f64 = attrs.get(MATSIM_ATTR_PERMLANES, &id, &mut ok, true);
        //bool oneWay = attrs.getOpt<bool>(MATSIM_ATTR_ONEWAY, id.c_str(), ok, true); // mandatory?
        let modes: String = attrs.get_opt(MATSIM_ATTR_MODES, &id, &mut ok, String::new(), true);
        let origid: String = attrs.get_opt(MATSIM_ATTR_ORIGID, &id, &mut ok, String::new(), true);
        let fromNode = self.myNodeCont.m().retrieve(&fromNodeID);
        let mut toNode = self.myNodeCont.m().retrieve(&toNodeID);
        if fromNode.is_null() {
            write_error(&format!("Could not find from-node for edge '{}'.", id));
        }
        if toNode.is_null() {
            write_error(&format!("Could not find to-node for edge '{}'.", id));
        }
        if fromNode.is_null() || toNode.is_null() {
            return;
        }
        if self.myLanesFromCapacity {
            permLanes = self.myCapacity2Lanes.get(capacity) as f64;
        }
        if permLanes < 0.5 {
            write_warning(&format!("Ignoring edge {} which has no lanes.", id));
            return;
        }
        if fromNode == toNode {
            // adding node and edge with a different naming scheme to keep the original edge id for easier route repair
            let intermediateID = format!("{}.0", id);
            let intermediatePos = *toNode.m().get_position() + Position::new(utils::geom::POSITION_EPS, utils::geom::POSITION_EPS);
            let intermediate = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&intermediateID, &intermediatePos, Ptr::null()))));
            if self.myNodeCont.m().insert_2(intermediate) {
                let perm = self.compute_permission(modes.clone());
                self.insert_edge(&intermediateID, intermediate, toNode, freeSpeed, (permLanes + 0.5) as i32, capacity, length, perm);
                toNode = intermediate;
            } else {
                unsafe { drop(Box::from_raw(intermediate.as_raw())) };
                write_error(&format!("Could not add intermediate node to split loop edge '{}'.", id));
            }
        }
        let perm = self.compute_permission(modes);
        self.insert_edge(&id, fromNode, toNode, freeSpeed, (permLanes + 0.5) as i32, capacity, length, perm);
    }

    /// C++: `void EdgesHandler::insertEdge(const std::string &, NBNode *, NBNode *, double, int, double, double, SVCPermissions)` [private]
    // void
    // NIImporter_MATSim::EdgesHandler::insertEdge(const std::string& id, NBNode* fromNode, NBNode* toNode, double freeSpeed, int numLanes, double capacity, double length, SVCPermissions perm) {
    //     NBEdge* edge = new NBEdge(id, fromNode, toNode, "", freeSpeed, NBEdge::UNSPECIFIED_FRICTION, numLanes, -1, NBEdge::UNSPECIFIED_WIDTH, NBEdge::UNSPECIFIED_OFFSET, LaneSpreadFunction::RIGHT);
    //     edge->setParameter("capacity", toString(capacity));
    //     edge->setPermissions(perm, -1);
    //     if (myKeepEdgeLengths) {
    //         edge->setLoadedLength(length);
    //     }
    //     if (!myEdgeCont.insert(edge)) {
    //         delete edge;
    //         WRITE_ERRORF(TL("Could not add edge '%'. Probably declared twice."), id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_edge(&mut self, id: &str, fromNode: Ptr<NBNode>, toNode: Ptr<NBNode>, freeSpeed: f64, numLanes: i32, capacity: f64, length: f64, perm: SVCPermissions) {
        let edge = Ptr::from_raw(Box::into_raw(Box::new(NBEdge::new(
            id,
            fromNode,
            toNode,
            String::new(),
            freeSpeed,
            NBEDGE_UNSPECIFIED_FRICTION,
            numLanes,
            -1,
            NBEDGE_UNSPECIFIED_WIDTH,
            NBEDGE_UNSPECIFIED_OFFSET,
            LaneSpreadFunction::RIGHT,
            "",
        ))));
        edge.m().base2.set_parameter("capacity", &utils::common::to_string::to_string(&capacity));
        edge.m().set_permissions(perm, -1);
        if self.myKeepEdgeLengths {
            edge.m().set_loaded_length(length);
        }
        if !self.myEdgeCont.m().insert(edge, false) {
            unsafe { drop(Box::from_raw(edge.as_raw())) };
            write_error(&format!("Could not add edge '{}'. Probably declared twice.", id));
        }
    }

    /// C++: `SVCPermissions EdgesHandler::computePermission(std::string)` [private]
    // SVCPermissions
    // NIImporter_MATSim::EdgesHandler::computePermission(std::string modes) {
    //     // using the MATSim modes from org.matsim.api.core.v01.TransportMode
    //     if (modes.size() == 0) {
    //         return SVCAll;
    //     }
    //     SVCPermissions result(SVC_IGNORING);
    //     for (StringTokenizer st(modes); st.hasNext();) {
    //         std::string mode = st.next();
    //         if (mode == "car") {
    //             result |= SVC_PASSENGER;
    //         } else if (mode == "bike") {
    //             result |= SVC_BICYCLE;
    //         } else if (mode == "motorcycle") {
    //             result |= SVC_MOTORCYCLE | SVC_MOPED;
    //         } else if (mode == "truck") {
    //             result |= SVC_TRUCK | SVC_TRAILER;
    //         } else if (mode == "pt") {
    //             result |= SVC_BUS | SVC_TRAM;
    //         } else if (mode == "drt" || mode == "taxi") {
    //             result |= SVC_TAXI;
    //         } else if (mode == "walk" || mode == "transit_walk") {
    //             result |= SVC_PEDESTRIAN;
    //         } else if (mode == "train") {
    //             result |= SVC_RAIL_CLASSES;
    //         } else if (mode == "ship") {
    //             result |= SVC_SHIP;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn compute_permission(&mut self, modes: String) -> SVCPermissions {
        // using the MATSim modes from org.matsim.api.core.v01.TransportMode
        if modes.is_empty() {
            return SVC_ALL;
        }
        let mut result: SVCPermissions = SVC_IGNORING;
        for mode in StringTokenizer::new(&modes).get_vector() {
            if mode == "car" {
                result |= SVC_PASSENGER;
            } else if mode == "bike" {
                result |= SVC_BICYCLE;
            } else if mode == "motorcycle" {
                result |= SVC_MOTORCYCLE | SVC_MOPED;
            } else if mode == "truck" {
                result |= SVC_TRUCK | SVC_TRAILER;
            } else if mode == "pt" {
                result |= SVC_BUS | SVC_TRAM;
            } else if mode == "drt" || mode == "taxi" {
                result |= SVC_TAXI;
            } else if mode == "walk" || mode == "transit_walk" {
                result |= SVC_PEDESTRIAN;
            } else if mode == "train" {
                result |= SVC_RAIL_CLASSES;
            } else if mode == "ship" {
                result |= SVC_SHIP;
            }
        }
        result
    }

    /// C++: `void EdgesHandler::EdgesHandler(const EdgesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_MATSim_EdgesHandler) -> Self {
        // C++ declares the copy constructor private and never defines it
        unreachable!("EdgesHandler copy constructor is private and undefined in C++")
    }

    /// C++: `EdgesHandler & EdgesHandler::operator=(const EdgesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_MATSim_EdgesHandler) -> &mut NIImporter_MATSim_EdgesHandler {
        // C++ declares the assignment operator private and never defines it
        unreachable!("EdgesHandler::operator= is private and undefined in C++")
    }

}

/// Importer for networks stored in MATSim format
/// C++ class `NIImporter_MATSim` (netimport/NIImporter_MATSim.h)
pub struct NIImporter_MATSim {
}

// C++ static members of NIImporter_MATSim (port as module-level statics, see AGENTS.md):
//   static SequentialStringBijection::Entry[] NIImporter_MATSim::matsimTags;
//   static SequentialStringBijection::Entry[] NIImporter_MATSim::matsimAttrs;

impl NIImporter_MATSim {
    /// C++: `void NIImporter_MATSim::loadNetwork(const OptionsCont &, NBNetBuilder &)` [static]
    /// Loads content of the optionally given MATSIM network files
    // void
    // NIImporter_MATSim::loadNetwork(const OptionsCont& oc, NBNetBuilder& nb) {
    //     // check whether the option is set properly and all files exist
    //     if (!oc.isUsableFileList("matsim-files")) {
    //         return;
    //     }
    //     /* Parse file(s)
    //      * Each file is parsed twice: first for nodes, second for edges. */
    //     const std::vector<std::string> files = oc.getStringVector("matsim-files");
    //     // load nodes, first
    //     NodesHandler nodesHandler(nb.getNodeCont());
    //     for (const std::string& file : files) {
    //         nodesHandler.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing nodes from matsim-file '" + file + "'");
    //         if (!XMLSubSys::runParser(nodesHandler, file, false, false, true)) {
    //             return;
    //         }
    //         PROGRESS_DONE_MESSAGE();
    //     }
    //     // load edges, then
    //     EdgesHandler edgesHandler(nb.getNodeCont(), nb.getEdgeCont(), oc.getBool("matsim.keep-length"),
    //                               oc.getBool("matsim.lanes-from-capacity"), NBCapacity2Lanes(oc.getFloat("lanes-from-capacity.norm")));
    //     for (const std::string& file : files) {
    //         edgesHandler.setFileName(file);
    //         PROGRESS_BEGIN_MESSAGE("Parsing edges from matsim-file '" + file + "'");
    //         XMLSubSys::runParser(edgesHandler, file, false, false, true);
    //         PROGRESS_DONE_MESSAGE();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_network(oc: &OptionsCont, nb: &mut NBNetBuilder) {
        // check whether the option is set properly and all files exist
        if !oc.is_usable_file_list("matsim-files").unwrap() {
            return;
        }
        // Parse file(s)
        // Each file is parsed twice: first for nodes, second for edges.
        let files: StringVector = oc.get_string_vector("matsim-files").unwrap().clone();
        // load nodes, first
        let mut nodesHandler = NIImporter_MATSim_NodesHandler::new(nb.get_node_cont());
        for file in files.iter() {
            nodesHandler.set_file_name(file);
            progress_begin_message(&format!("Parsing nodes from matsim-file '{}'", file));
            if !utils::xml::xml_sub_sys::run_parser(&mut nodesHandler, file, false, false, true, true).unwrap_or(false) {
                return;
            }
            progress_done_message();
        }
        // load edges, then
        let nodeCont = Ptr::from_mut(nb.get_node_cont());
        let edgeCont = Ptr::from_mut(nb.get_edge_cont());
        let mut edgesHandler = NIImporter_MATSim_EdgesHandler::new(
            nodeCont.m(),
            edgeCont.m(),
            oc.get_bool("matsim.keep-length").unwrap(),
            oc.get_bool("matsim.lanes-from-capacity").unwrap(),
            NBCapacity2Lanes::new(oc.get_float("lanes-from-capacity.norm").unwrap()),
        );
        for file in files.iter() {
            edgesHandler.set_file_name(file);
            progress_begin_message(&format!("Parsing edges from matsim-file '{}'", file));
            let _ = utils::xml::xml_sub_sys::run_parser(&mut edgesHandler, file, false, false, true, true);
            progress_done_message();
        }
    }

}

/// A class which extracts MATSIM-nodes from a parsed MATSIM-file
/// C++ class `NIImporter_MATSim::NodesHandler` (netimport/NIImporter_MATSim.h)
#[repr(C)]
pub struct NIImporter_MATSim_NodesHandler {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    /// The nodes container to fill
    pub myNodeCont: Ptr<NBNodeCont>,
}

impl std::ops::Deref for NIImporter_MATSim_NodesHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NIImporter_MATSim_NodesHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl SaxHandler for NIImporter_MATSim_NodesHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        NIImporter_MATSim_NodesHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}

impl NIImporter_MATSim_NodesHandler {
    /// C++: `void NodesHandler::NodesHandler(NBNodeCont &)`
    /// Contructor
    // NIImporter_MATSim::NodesHandler::NodesHandler(NBNodeCont& toFill)
    //     : GenericSAXHandler(matsimTags, MATSIM_TAG_NOTHING,
    //                         matsimAttrs, MATSIM_ATTR_NOTHING,
    //                         "matsim - file"), myNodeCont(toFill) {
    // }
    #[allow(unused_variables)]
    pub fn new(toFill: &mut NBNodeCont) -> Self {
        NIImporter_MATSim_NodesHandler {
            base: GenericSAXHandler::new(MATSIM_TAGS, MATSIM_TAG_NOTHING, MATSIM_ATTRS, MATSIM_ATTR_NOTHING, "matsim - file", ""),
            myNodeCont: Ptr::from_mut(toFill),
        }
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_MATSim::NodesHandler::~NodesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor has an empty body
    }

    /// C++: `void NodesHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NIImporter_MATSim::NodesHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     if (element != MATSIM_TAG_NODE) {
    //         return;
    //     }
    //     // get the id, report a warning if not given or empty...
    //     bool ok = true;
    //     const std::string id = SUMOXMLDefinitions::makeValidID(attrs.get<std::string>(MATSIM_ATTR_ID, nullptr, ok));
    //     const double x = attrs.get<double>(MATSIM_ATTR_X, id.c_str(), ok);
    //     const double y = attrs.get<double>(MATSIM_ATTR_Y, id.c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     Position pos(x, y);
    //     if (!NBNetBuilder::transformCoordinate(pos)) {
    //         WRITE_ERRORF(TL("Unable to project coordinates for node '%'."), id);
    //     }
    //     NBNode* node = new NBNode(id, pos);
    //     if (!myNodeCont.insert(node)) {
    //         delete node;
    //         WRITE_ERRORF(TL("Could not add node '%'. Probably declared twice."), id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element != MATSIM_TAG_NODE {
            return;
        }
        // get the id, report a warning if not given or empty...
        let mut ok = true;
        let id: String = make_valid_id(&attrs.get::<String>(MATSIM_ATTR_ID, "", &mut ok, true));
        let x: f64 = attrs.get(MATSIM_ATTR_X, &id, &mut ok, true);
        let y: f64 = attrs.get(MATSIM_ATTR_Y, &id, &mut ok, true);
        if !ok {
            return;
        }
        let mut pos = Position::new(x, y);
        if !NBNetBuilder::transform_coordinate(&mut pos, true, Ptr::null()) {
            write_error(&format!("Unable to project coordinates for node '{}'.", id));
        }
        let node = Ptr::from_raw(Box::into_raw(Box::new(NBNode::new_2(&id, &pos, Ptr::null()))));
        if !self.myNodeCont.m().insert_2(node) {
            unsafe { drop(Box::from_raw(node.as_raw())) };
            write_error(&format!("Could not add node '{}'. Probably declared twice.", id));
        }
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_MATSim_NodesHandler) -> Self {
        // C++ declares the copy constructor private and never defines it
        unreachable!("NodesHandler copy constructor is private and undefined in C++")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_MATSim_NodesHandler) -> &mut NIImporter_MATSim_NodesHandler {
        // C++ declares the assignment operator private and never defines it
        unreachable!("NodesHandler::operator= is private and undefined in C++")
    }

}

