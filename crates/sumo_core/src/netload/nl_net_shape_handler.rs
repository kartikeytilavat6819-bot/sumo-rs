//! Port of netload/NLNetShapeHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The XML-Handler for network loading
/// C++ class `NLNetShapeHandler` (netload/NLNetShapeHandler.h)
pub struct NLNetShapeHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The net to fill (preinitialised)
    pub myNet: Ptr<MSNet>,
    /// mapping between primary internal lane and corresponding secondary internal lane
    pub myShuffledJunctions: BTreeMap<Ptr<MSJunction>, BTreeMap<Ptr<MSLane>, Ptr<MSLane>>>, // C++: std::map<const MSJunction *, std::map<MSLane *, MSLane *>>
    /// lanes of the primary network that should receive a secondary shape
    pub myPrimaryEdges: BTreeSet<Ptr<MSEdge>>,
}

impl std::ops::Deref for NLNetShapeHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NLNetShapeHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl NLNetShapeHandler {
    /// C++: `void NLNetShapeHandler::NLNetShapeHandler(const std::string &, MSNet &)`
    /// Constructor
    // NLNetShapeHandler::NLNetShapeHandler(const std::string& file, MSNet& net) :
    //     SUMOSAXHandler(file, "net"),
    //     myNet(net),
    //     myPrimaryEdges(MSEdge::getAllEdges().begin(), MSEdge::getAllEdges().end()) {
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, net: &mut MSNet) -> Self {
        Self {
            base: utils::xml::sumo_sax_handler::new(file, "net"),
            myNet: Ptr::from_mut(net),
            myShuffledJunctions: BTreeMap::new(),
            // determinism: std::set<MSEdge*> is ordered by address in C++ as well, BTreeSet<Ptr> keeps that order
            myPrimaryEdges: MSEdge::get_all_edges().iter().copied().collect(),
        }
    }

    /// C++: `void NLNetShapeHandler::~NLNetShapeHandler() noexcept` [virtual]
    /// Destructor
    // NLNetShapeHandler::~NLNetShapeHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void NLNetShapeHandler::sortInternalShapes()`
    /// resolve mismatch between internal lane ids of both networks
    // void
    // NLNetShapeHandler::sortInternalShapes() {
    //     // even when the alternative network has the same topology as
    //     // the primary network, the ids of internal lanes may differ
    //     // since they are based on a clockwise sorting of the edges.
    //     // hence we must verify connections and shuffle the shapes as needed
    //     for (auto item : myShuffledJunctions) {
    //         const MSJunction* junction = item.first;
    //         std::map<MSLane*, PositionVector> shapes2;
    //         for (MSLane* lane : junction->getInternalLanes()) {
    //             shapes2[lane] = lane->getShape(true);
    //         }
    //
    //         for (auto laneMap : item.second) {
    //             laneMap.first->addSecondaryShape(shapes2[laneMap.second]);
    //         }
    //     }
    //     // final warning
    //     if (myPrimaryEdges.size() > 0) {
    //         WRITE_WARNING(TLF("% edges of the primary network did not occur in the alternative-net-file", myPrimaryEdges.size()));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sort_internal_shapes(&mut self) {
        // even when the alternative network has the same topology as
        // the primary network, the ids of internal lanes may differ
        // since they are based on a clockwise sorting of the edges.
        // hence we must verify connections and shuffle the shapes as needed
        for (junction, laneMap) in self.myShuffledJunctions.iter() {
            let mut shapes2: BTreeMap<Ptr<MSLane>, PositionVector> = BTreeMap::new();
            for lane in junction.get_internal_lanes() {
                shapes2.insert(lane, lane.get_shape(true).clone());
            }
            for (first, second) in laneMap.iter() {
                let shape = shapes2.entry(*second).or_default().clone();
                first.m().add_secondary_shape(&shape);
            }
        }
        // final warning
        if self.myPrimaryEdges.len() > 0 {
            utils::common::msg_handler::write_warning(&format!("{} edges of the primary network did not occur in the alternative-net-file", self.myPrimaryEdges.len()));
        }
    }

    /// C++: `void NLNetShapeHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NLNetShapeHandler::myStartElement(int element,
    //                                   const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_LANE:
    //             addLane(attrs);
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             addJunction(attrs);
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             if (myNet.hasInternalLinks()) {
    //                 // see sortInternalShapes
    //                 addConnection(attrs);
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        match element {
            SUMO_TAG_LANE => self.add_lane(attrs),
            SUMO_TAG_JUNCTION => self.add_junction(attrs),
            SUMO_TAG_CONNECTION => {
                if self.myNet.has_internal_links() {
                    // see sortInternalShapes
                    self.add_connection(attrs);
                }
            }
            _ => {}
        }
    }

    /// C++: `void NLNetShapeHandler::addLane(const SUMOSAXAttributes &)` [private]
    /// adds a secondary lane shape
    // void
    // NLNetShapeHandler::addLane(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!myNet.hasInternalLinks() && id[0] == ':') {
    //         return;
    //     }
    //     MSLane* lane = MSLane::dictionary(id);
    //     if (lane == nullptr) {
    //         WRITE_WARNINGF("The lane '%' does not exist in the primary network.", id);
    //         return;
    //     }
    //     const PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //     lane->addSecondaryShape(shape);
    //     myPrimaryEdges.erase(&lane->getEdge());
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !self.myNet.has_internal_links() && id.starts_with(':') {
            return;
        }
        let lane = MSLane::dictionary_2(&id);
        if lane.is_null() {
            utils::common::msg_handler::write_warning(&format!("The lane '{}' does not exist in the primary network.", id));
            return;
        }
        let shape = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut ok, true);
        lane.m().add_secondary_shape(&shape);
        self.myPrimaryEdges.remove(&Ptr::from_mut(lane.get_edge()));
    }

    /// C++: `void NLNetShapeHandler::addJunction(const SUMOSAXAttributes &)` [private]
    /// adds a junction position
    // void
    // NLNetShapeHandler::addJunction(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!myNet.hasInternalLinks() && id[0] == ':') {
    //         return;
    //     }
    //     MSJunction* junction = myNet.getJunctionControl().get(id);
    //     if (junction == nullptr) {
    //         WRITE_WARNINGF("The junction '%' does not exist in the primary network.", id);
    //         return;
    //     }
    //     double x = attrs.get<double>(SUMO_ATTR_X, id.c_str(), ok);
    //     double y = attrs.get<double>(SUMO_ATTR_Y, id.c_str(), ok);
    //     double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, 0);
    //     junction->addSecondaryPosition(Position(x, y, z));
    // }
    #[allow(unused_variables)]
    pub fn add_junction(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !self.myNet.has_internal_links() && id.starts_with(':') {
            return;
        }
        let junction = self.myNet.m().get_junction_control().get(&id);
        if junction.is_null() {
            utils::common::msg_handler::write_warning(&format!("The junction '{}' does not exist in the primary network.", id));
            return;
        }
        let x = attrs.get::<f64>(SUMO_ATTR_X, &id, &mut ok, true);
        let y = attrs.get::<f64>(SUMO_ATTR_Y, &id, &mut ok, true);
        let z = attrs.get_opt::<f64>(SUMO_ATTR_Z, &id, &mut ok, 0., true);
        junction.m().add_secondary_position(&Position::new3(x, y, z));
    }

    /// C++: `void NLNetShapeHandler::addConnection(const SUMOSAXAttributes &)` [private]
    /// records connection topology for later resorting
    // void
    // NLNetShapeHandler::addConnection(const SUMOSAXAttributes& attrs) {
    //     if (!attrs.hasAttribute(SUMO_ATTR_VIA)) {
    //         return;
    //     }
    //     bool ok = true;
    //     const std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     const std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     const int fromLaneIdx = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     const int toLaneIdx = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     std::string viaID = attrs.get<std::string>(SUMO_ATTR_VIA, nullptr, ok);
    //     MSLane* lane = MSLane::dictionary(viaID);
    //     if (lane == nullptr) {
    //         // warning already given in addLane
    //         return;
    //     }
    //     std::string fromLaneID = fromID + "_" + toString(fromLaneIdx);
    //     std::string toLaneID = toID + "_" + toString(toLaneIdx);
    //
    //     if (lane->getLinkCont()[0]->getLane()->getID() != toLaneID
    //             || lane->getIncomingLanes()[0].lane->getID() != fromLaneID) {
    //         // mismatch: find the correct lane that connects fromLaneID and toLaneID
    //         const MSJunction* junction = lane->getEdge().getToJunction();
    //         for (MSLane* lane2 : junction->getInternalLanes()) {
    //             if (lane2->getLinkCont()[0]->getLane()->getID() == toLaneID
    //                     && lane2->getIncomingLanes()[0].lane->getID() == fromLaneID) {
    //                 myShuffledJunctions[junction][lane2] = lane;
    //                 break;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_connection(&mut self, attrs: &SUMOSAXAttributes) {
        if !attrs.has_attribute(SUMO_ATTR_VIA) {
            return;
        }
        let mut ok = true;
        let fromID = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
        let toID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
        let fromLaneIdx = attrs.get::<i32>(SUMO_ATTR_FROM_LANE, "", &mut ok, true);
        let toLaneIdx = attrs.get::<i32>(SUMO_ATTR_TO_LANE, "", &mut ok, true);
        let viaID = attrs.get::<String>(SUMO_ATTR_VIA, "", &mut ok, true);
        let lane = MSLane::dictionary_2(&viaID);
        if lane.is_null() {
            // warning already given in addLane
            return;
        }
        let fromLaneID = format!("{}_{}", fromID, fromLaneIdx);
        let toLaneID = format!("{}_{}", toID, toLaneIdx);

        if lane.get_link_cont()[0].get_lane().get_id() != toLaneID.as_str()
                || lane.get_incoming_lanes()[0].lane.get_id() != fromLaneID.as_str() {
            // mismatch: find the correct lane that connects fromLaneID and toLaneID
            let junction = lane.get_edge().get_to_junction();
            for lane2 in junction.get_internal_lanes() {
                if lane2.get_link_cont()[0].get_lane().get_id() == toLaneID.as_str()
                        && lane2.get_incoming_lanes()[0].lane.get_id() == fromLaneID.as_str() {
                    self.myShuffledJunctions.entry(junction).or_default().insert(lane2, lane);
                    break;
                }
            }
        }
    }

    /// C++: `void NLNetShapeHandler::NLNetShapeHandler(const NLNetShapeHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLNetShapeHandler) -> Self {
        panic!("NLNetShapeHandler copy constructor is declared private and never defined in C++")
    }

    /// C++: `NLNetShapeHandler & NLNetShapeHandler::operator=(const NLNetShapeHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLNetShapeHandler) -> &mut NLNetShapeHandler {
        panic!("NLNetShapeHandler assignment is declared private and never defined in C++")
    }

}

