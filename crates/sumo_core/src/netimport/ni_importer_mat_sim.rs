//! Port of netimport/NIImporter_MATSim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port EdgesHandler::EdgesHandler")
    }

    /// C++: `void EdgesHandler::~EdgesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_MATSim::EdgesHandler::~EdgesHandler() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EdgesHandler::~EdgesHandler")
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
        todo!("port EdgesHandler::myStartElement")
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
        todo!("port EdgesHandler::insertEdge")
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
        todo!("port EdgesHandler::computePermission")
    }

    /// C++: `void EdgesHandler::EdgesHandler(const EdgesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_DlrNavteq_EdgesHandler) -> Self {
        todo!("port EdgesHandler::EdgesHandler")
    }

    /// C++: `EdgesHandler & EdgesHandler::operator=(const EdgesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_DlrNavteq_EdgesHandler) -> &mut NIImporter_DlrNavteq_EdgesHandler {
        todo!("port EdgesHandler::operator=")
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
        todo!("port NIImporter_MATSim::loadNetwork")
    }

}

/// A class which extracts MATSIM-nodes from a parsed MATSIM-file
/// C++ class `NIImporter_MATSim::NodesHandler` (netimport/NIImporter_MATSim.h)
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
        todo!("port NodesHandler::NodesHandler")
    }

    /// C++: `void NodesHandler::~NodesHandler() noexcept` [virtual]
    /// Destructor
    // NIImporter_MATSim::NodesHandler::~NodesHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NodesHandler::~NodesHandler")
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
        todo!("port NodesHandler::myStartElement")
    }

    /// C++: `void NodesHandler::NodesHandler(const NodesHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NIImporter_DlrNavteq_NodesHandler) -> Self {
        todo!("port NodesHandler::NodesHandler")
    }

    /// C++: `NodesHandler & NodesHandler::operator=(const NodesHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NIImporter_DlrNavteq_NodesHandler) -> &mut NIImporter_DlrNavteq_NodesHandler {
        todo!("port NodesHandler::operator=")
    }

}

