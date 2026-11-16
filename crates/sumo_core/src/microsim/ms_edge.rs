//! Port of microsim/MSEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};

/// C++ `#define BEST_LANE_LOOKAHEAD 3000.0` (MSEdge.cpp)
const BEST_LANE_LOOKAHEAD: f64 = 3000.0;

// shortcut: MSGlobals is still an empty skeleton (ms_globals.rs), so the g* values used here are the
// default-run values (lateral-resolution=-1, no-internal-links=false, no meso, zero penalties);
// upgrade these to MSGlobals::g* reads once that module is ported.
const G_LATERAL_RESOLUTION: f64 = -1.;
const G_USING_INTERNAL_LANES: bool = true;
const G_USE_MESO_SIM: bool = false;
const G_TLS_PENALTY: f64 = 0.;
const G_MINOR_PENALTY: f64 = 0.;
const G_TURNAROUND_PENALTY: f64 = 0.;
const G_LANE_CHANGE_DURATION: SUMOTime = 0;
const G_STOP_TOLERANCE: f64 = 10.; // default of option ride.stop-tolerance

/// C++ typedef `std::map<const MSEdge *, AllowedLanesCont> MSEdge::AllowedLanesByTarget`
pub type MSEdge_AllowedLanesByTarget = BTreeMap<Ptr<MSEdge>, MSEdge_AllowedLanesCont>;

/// C++ typedef `std::vector<std::pair<SVCPermissions, std::shared_ptr<const std::vector<MSLane *>>>> MSEdge::AllowedLanesCont`
pub type MSEdge_AllowedLanesCont = Vec<(SVCPermissions, Rc<Vec<Ptr<MSLane>>>)>;

/// C++ typedef `std::map<std::string, MSEdge *> MSEdge::DictType`
pub type MSEdge_DictType = BTreeMap<String, Ptr<MSEdge>>;

/// C++ typedef `std::vector<std::pair<const MSEdge *, const MSEdge *>> MSConstEdgePairVector`
pub type MSConstEdgePairVector = Vec<(Ptr<MSEdge>, Ptr<MSEdge>)>;

/// C++ class `MSEdge` (microsim/MSEdge.h)
#[repr(C)]
pub struct MSEdge {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// This edge's numerical id
    pub myNumericalID: i32,
    /// Container for the edge's lane; should be sorted: (right-hand-traffic) the more left the lane, the higher the container-index
    pub myLanes: Rc<Vec<Ptr<MSLane>>>,
    /// This member will do the lane-change
    pub myLaneChanger: Ptr<MSLaneChanger>,
    /// the purpose of the edge
    pub myFunction: SumoXMLEdgeFunc,
    /// Vaporizer counter
    pub myVaporizationRequests: i32,
    /// The time of last insertion failure
    pub myLastFailedInsertionTime: std::cell::Cell<SUMOTime>,
    /// A cache for the rejected insertion attempts. Used to assure that no further insertion attempts are made on a lane where an attempt has already failed in the current time step if MSInsertionControl::myEagerInsertionCheck is off.
    pub myFailedInsertionMemory: BTreeSet<i32>,
    /// The crossed edges id for a crossing edge. On not crossing edges it is empty
    pub myCrossingEdges: Vec<String>,
    /// The succeeding edges
    pub mySuccessors: MSEdgeVector,
    pub myViaSuccessors: MSConstEdgePairVector,
    /// The preceeding edges
    pub myPredecessors: MSEdgeVector,
    /// the junctions for this edge
    pub myFromJunction: Ptr<MSJunction>,
    pub myToJunction: Ptr<MSJunction>,
    /// Persons on the edge for drawing and pushbutton
    pub myPersons: BTreeSet<Ptr<MSTransportable>>, // C++: std::set<MSTransportable *, ComparatorNumericalIdLess>
    /// Containers on the edge
    pub myContainers: BTreeSet<Ptr<MSTransportable>>, // C++: std::set<MSTransportable *, ComparatorNumericalIdLess>
    /// Associative container from vehicle class to allowed-lanes.
    pub myAllowed: MSEdge_AllowedLanesCont,
    pub myOrigAllowed: MSEdge_AllowedLanesCont,
    /// From target edge to lanes allowed to be used to reach it
    pub myAllowedTargets: MSEdge_AllowedLanesByTarget,
    pub myOrigAllowedTargets: MSEdge_AllowedLanesByTarget,
    /// The intersection of lane permissions for this edge
    pub myMinimumPermissions: SVCPermissions,
    /// The union of lane permissions for this edge
    pub myCombinedPermissions: SVCPermissions,
    /// The original intersection of lane permissions for this edge (before temporary modifications)
    pub myOriginalMinimumPermissions: SVCPermissions,
    /// The original union of lane permissions for this edge (before temporary modifications)
    pub myOriginalCombinedPermissions: SVCPermissions,
    /// whether transient permission changes were applied to this edge or a predecessor
    pub myHaveTransientPermissions: bool,
    /// the other taz-connector if this edge isTazConnector, otherwise nullptr
    pub myOtherTazConnector: Ptr<MSEdge>,
    /// the real-world name of this edge (need not be unique)
    pub myStreetName: String,
    /// the type of the edge (optionally used during network creation)
    pub myEdgeType: String,
    /// the routing type of the edge (used to look up vType and vClass specific routing preferences)
    pub myRoutingType: String,
    /// the priority of the edge (used during network creation)
    pub myPriority: i32,
    /// the kilometrage/mileage at the start of the edge
    pub myDistance: f64,
    /// Edge width [m]
    pub myWidth: f64,
    /// the length of the edge (cached value for speedup)
    pub myLength: f64,
    /// the traveltime on the empty edge (cached value for speedup)
    pub myEmptyTraveltime: f64,
    /// flat penalty when computing traveltime
    pub myTimePenalty: f64,
    /// whether this edge had a vehicle with less than max speed on it
    pub myAmDelayed: std::cell::Cell<bool>,
    /// whether this edge belongs to a roundabout
    pub myAmRoundabout: bool,
    /// whether this edge is at the network fringe
    pub myAmFringe: bool,
    /// the right side for each sublane on this edge
    pub mySublaneSides: Vec<f64>,
    /// The successors available for a given vClass
    pub myClassesSuccessorMap: BTreeMap<SUMOVehicleClass, MSEdgeVector>,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, MSConstEdgePairVector>,
    pub myOrigClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, MSConstEdgePairVector>,
    /// The bounding rectangle of end nodes incoming or outgoing edges for taz connectors or of my own start and end node for normal edges
    pub myBoundary: Boundary,
    /// List of waiting vehicles
    pub myWaiting: Vec<Ptr<SUMOVehicle>>,
    /// Mesoscopic vehicle positions
    pub myCachedMesoPos: BTreeMap<Ptr<MEVehicle>, (f64, i32)>, // C++: std::map<const MEVehicle *, std::pair<double, int>>
    /// time stamp of mesoscopic vehicle positions
    pub myLastCacheUpdate: SUMOTime,
    /// the oppositing superposable edge
    pub myBidiEdge: Ptr<MSEdge>,
    /// a reversed version for backward routing
    pub myReversedRoutingEdge: Ptr<ReversedEdge<MSEdge, SUMOVehicle>>,
    pub myRailwayRoutingEdge: Ptr<RailEdge<MSEdge, SUMOVehicle>>,
}

impl RouterEdge<SUMOVehicle> for MSEdge {
    type Node = MSJunction;
    type Lane = MSLane;
    fn bidi_edge(&self) -> Ptr<MSEdge> {
        self.get_bidi_edge()
    }
    fn to_junction(&self) -> Ptr<MSJunction> {
        self.get_to_junction()
    }
    fn from_junction(&self) -> Ptr<MSJunction> {
        self.get_from_junction()
    }
    fn is_crossing(&self) -> bool {
        self.is_crossing()
    }
    fn is_walking_area(&self) -> bool {
        self.is_walking_area()
    }
    fn railway_routing_edge(&self) -> Ptr<RailEdge<MSEdge, SUMOVehicle>> {
        self.get_railway_routing_edge()
    }
    fn all_edges() -> Vec<Ptr<MSEdge>> {
        MSEdge::get_all_edges().clone()
    }
    fn id(&self) -> &str {
        self.get_id()
    }
    fn numerical_id(&self) -> i32 {
        self.get_numerical_id()
    }
    fn length(&self) -> f64 {
        self.get_length()
    }
    fn length_geometry_factor(&self) -> f64 {
        self.get_length_geometry_factor()
    }
    fn distance_to(&self, other: Ptr<MSEdge>, do_boundary_estimate: bool) -> f64 {
        self.get_distance_to(other, do_boundary_estimate)
    }
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<MSEdge>] {
        self.get_successors(vclass)
    }
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<MSEdge>, Ptr<MSEdge>)] {
        self.get_via_successors(vclass, ignore_transient)
    }
    fn predecessors(&self) -> &[Ptr<MSEdge>] {
        self.get_predecessors()
    }
    fn lanes(&self) -> &[Ptr<MSLane>] {
        self.get_lanes()
    }
    fn speed_limit(&self) -> f64 {
        self.get_speed_limit()
    }
    fn preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        self.get_preference(pars)
    }
    fn travel_time_static(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>, time: f64) -> f64 {
        MSEdge::get_travel_time_static(edge, veh, time)
    }
    fn travel_time_aggregated(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>, time: f64) -> f64 {
        MSEdge::get_travel_time_aggregated(edge, veh, time)
    }
    fn minimum_travel_time(&self, veh: Ptr<SUMOVehicle>) -> f64 {
        self.get_minimum_travel_time(veh)
    }
    fn is_internal(&self) -> bool {
        self.is_internal()
    }
    fn is_taz_connector(&self) -> bool {
        self.is_taz_connector()
    }
    // C++ MSEdge::prohibits(const SUMOVehicle*) takes no check_restrictions flag
    fn prohibits(&self, veh: Ptr<SUMOVehicle>, _check_restrictions: bool) -> bool {
        self.prohibits(veh)
    }
    fn restricts(&self, veh: Ptr<SUMOVehicle>) -> bool {
        self.restricts(veh)
    }
}

impl std::ops::Deref for MSEdge {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSEdge {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

/// Runs a C++ `operator()(a, b)` comparator (returns int, nonzero = a before b) as a std::sort ordering.
fn sort_by_sorter(less: impl Fn(Ptr<MSTransportable>, Ptr<MSTransportable>) -> i32, a: Ptr<MSTransportable>, b: Ptr<MSTransportable>) -> std::cmp::Ordering {
    if less(a, b) != 0 {
        std::cmp::Ordering::Less
    } else if less(b, a) != 0 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

// C++ static members of MSEdge (port as module-level statics, see AGENTS.md):
//   static DictType MSEdge::myDict;
//   static std::unordered_map<std::string_view, MSEdge *> MSEdge::myHashDict;
//   static MSEdgeVector MSEdge::myEdges;
//   static SVCPermissions MSEdge::myMesoIgnoredVClasses;
//   static DepartLaneDefinition MSEdge::myDefaultDepartLaneDefinition;
//   static int MSEdge::myDefaultDepartLane;

// C++ static members of MSEdge, ported as module-level statics (zero-initialised like C++ statics).
// Accessed only through the helper fns below, which keep the C++ `T&` return shape.
static mut MY_DICT: MSEdge_DictType = BTreeMap::new();
// shortcut: C++ myHashDict is an unordered_map; a BTreeMap is used since it is only queried by key
static mut MY_HASH_DICT: BTreeMap<String, Ptr<MSEdge>> = BTreeMap::new();
static mut MY_EDGES: MSEdgeVector = Vec::new();
static mut MY_MESO_IGNORED_V_CLASSES: SVCPermissions = 0;
static mut MY_DEFAULT_DEPART_LANE_DEFINITION: DepartLaneDefinition = DepartLaneDefinition::DEFAULT;
static mut MY_DEFAULT_DEPART_LANE: i32 = 0;

fn my_dict() -> &'static mut MSEdge_DictType {
    unsafe { &mut *std::ptr::addr_of_mut!(MY_DICT) }
}
fn my_hash_dict() -> &'static mut BTreeMap<String, Ptr<MSEdge>> {
    unsafe { &mut *std::ptr::addr_of_mut!(MY_HASH_DICT) }
}
fn my_edges() -> &'static mut MSEdgeVector {
    unsafe { &mut *std::ptr::addr_of_mut!(MY_EDGES) }
}
fn my_meso_ignored_v_classes() -> SVCPermissions {
    unsafe { *std::ptr::addr_of!(MY_MESO_IGNORED_V_CLASSES) }
}

impl MSEdge {
    /// C++: `void MSEdge::MSEdge(const std::string &, int, const SumoXMLEdgeFunc, const std::string &, const std::string &, const std::string &, int, double)`
    /// Constructor.
    // MSEdge::MSEdge(const std::string& id, int numericalID,
    //                const SumoXMLEdgeFunc function,
    //                const std::string& streetName,
    //                const std::string& edgeType,
    //                const std::string& routingType,
    //                int priority,
    //                double distance) :
    //     Named(id), myNumericalID(numericalID), myLanes(nullptr),
    //     myLaneChanger(nullptr), myFunction(function), myVaporizationRequests(0),
    //     myLastFailedInsertionTime(-1),
    //     myFromJunction(nullptr), myToJunction(nullptr),
    //     myHaveTransientPermissions(false),
    //     myOtherTazConnector(nullptr),
    //     myStreetName(streetName),
    //     myEdgeType(edgeType),
    //     myRoutingType(routingType),
    //     myPriority(priority),
    //     myDistance(distance),
    //     myWidth(0.),
    //     myLength(0.),
    //     myEmptyTraveltime(0.),
    //     myTimePenalty(0.),
    //     myAmDelayed(false),
    //     myAmRoundabout(false),
    //     myAmFringe(true),
    //     myBidiEdge(nullptr)
    // { }
    #[allow(unused_variables)]
    pub fn new(id: &str, numericalID: i32, function: SumoXMLEdgeFunc, streetName: &str, edgeType: &str, routingType: &str, priority: i32, distance: f64) -> Self {
        Self {
            base: Named::new(id),
            base2: Parameterised::new(),
            myNumericalID: numericalID,
            myLanes: Rc::new(Vec::new()), // C++ null shared_ptr until initialize()
            myLaneChanger: Ptr::null(),
            myFunction: function,
            myVaporizationRequests: 0,
            myLastFailedInsertionTime: std::cell::Cell::new(-1),
            myFailedInsertionMemory: BTreeSet::new(),
            myCrossingEdges: Vec::new(),
            mySuccessors: Vec::new(),
            myViaSuccessors: Vec::new(),
            myPredecessors: Vec::new(),
            myFromJunction: Ptr::null(),
            myToJunction: Ptr::null(),
            myPersons: BTreeSet::new(),
            myContainers: BTreeSet::new(),
            myAllowed: Vec::new(),
            myOrigAllowed: Vec::new(),
            myAllowedTargets: BTreeMap::new(),
            myOrigAllowedTargets: BTreeMap::new(),
            myMinimumPermissions: SVC_ALL,
            myCombinedPermissions: 0,
            myOriginalMinimumPermissions: SVC_ALL,
            myOriginalCombinedPermissions: SVC_ALL,
            myHaveTransientPermissions: false,
            myOtherTazConnector: Ptr::null(),
            myStreetName: streetName.to_string(),
            myEdgeType: edgeType.to_string(),
            myRoutingType: routingType.to_string(),
            myPriority: priority,
            myDistance: distance,
            myWidth: 0.,
            myLength: 0.,
            myEmptyTraveltime: 0.,
            myTimePenalty: 0.,
            myAmDelayed: std::cell::Cell::new(false),
            myAmRoundabout: false,
            myAmFringe: true,
            mySublaneSides: Vec::new(),
            myClassesSuccessorMap: BTreeMap::new(),
            myClassesViaSuccessorMap: BTreeMap::new(),
            myOrigClassesViaSuccessorMap: BTreeMap::new(),
            myBoundary: Boundary::default(),
            myWaiting: Vec::new(),
            myCachedMesoPos: BTreeMap::new(),
            myLastCacheUpdate: -1,
            myBidiEdge: Ptr::null(),
            myReversedRoutingEdge: Ptr::null(),
            myRailwayRoutingEdge: Ptr::null(),
        }
    }

    /// C++: `void MSEdge::~MSEdge() noexcept` [virtual]
    /// Destructor.
    // MSEdge::~MSEdge() {
    //     delete myLaneChanger;
    //     delete myReversedRoutingEdge;
    //     delete myRailwayRoutingEdge;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: myLaneChanger may be a MSLaneChangerSublane base pointer (only when lateral-resolution > 0, which MSGlobals shim never enables), freeing it here would be the wrong type
        if !self.myLaneChanger.is_null() {
            drop(unsafe { Box::from_raw(self.myLaneChanger.as_raw()) });
        }
        if !self.myReversedRoutingEdge.is_null() {
            drop(unsafe { Box::from_raw(self.myReversedRoutingEdge.as_raw()) });
        }
        if !self.myRailwayRoutingEdge.is_null() {
            drop(unsafe { Box::from_raw(self.myRailwayRoutingEdge.as_raw()) });
        }
    }

    /// C++: `void MSEdge::initialize(const std::vector<MSLane *> *)`
    /// Initialize the edge.
    // void
    // MSEdge::initialize(const std::vector<MSLane*>* lanes) {
    //     assert(lanes != 0);
    //     myLanes = std::shared_ptr<const std::vector<MSLane*> >(lanes);
    //     if (myFunction == SumoXMLEdgeFunc::CONNECTOR) {
    //         myCombinedPermissions = SVCAll;
    //     }
    //     for (MSLane* const lane : *lanes) {
    //         lane->setRightSideOnEdge(myWidth, (int)mySublaneSides.size());
    //         MSLeaderInfo ahead(lane->getWidth());
    //         for (int j = 0; j < ahead.numSublanes(); ++j) {
    //             mySublaneSides.push_back(myWidth + j * MSGlobals::gLateralResolution);
    //         }
    //         myWidth += lane->getWidth();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn initialize(&mut self, lanes: Ptr<Vec<Ptr<MSLane>>>) {
        assert!(!lanes.is_null());
        // C++ takes ownership of the pointer; the Rust side keeps its own copy of the lane list.
        self.myLanes = Rc::new(lanes.m().clone());
        if self.myFunction == SumoXMLEdgeFunc::CONNECTOR {
            self.myCombinedPermissions = SVC_ALL;
        }
        let laneList = self.myLanes.clone();
        for lane in laneList.iter() {
            lane.m().set_right_side_on_edge(self.myWidth, self.mySublaneSides.len() as i32);
            let ahead = MSLeaderInfo::new(lane.get_width(), Ptr::null(), 0.);
            for j in 0..ahead.num_sublanes() {
                self.mySublaneSides.push(self.myWidth + j as f64 * G_LATERAL_RESOLUTION);
            }
            self.myWidth += lane.get_width();
        }
    }

    /// C++: `void MSEdge::recalcCache()`
    /// Recalculates the cached values
    // void MSEdge::recalcCache() {
    //     if (myLanes->empty()) {
    //         return;
    //     }
    //     myLength = myLanes->front()->getLength();
    //     myEmptyTraveltime = myLength / MAX2(getSpeedLimit(), NUMERICAL_EPS);
    //     if (isNormal() && (MSGlobals::gUseMesoSim || MSGlobals::gTLSPenalty > 0)) {
    //         SUMOTime minorPenalty = 0;
    //         bool haveTLSPenalty = MSGlobals::gTLSPenalty > 0;
    //         if (MSGlobals::gUseMesoSim) {
    //             const MESegment::MesoEdgeType& edgeType = MSNet::getInstance()->getMesoType(getEdgeType());
    //             minorPenalty = edgeType.minorPenalty;
    //             haveTLSPenalty = edgeType.tlsPenalty > 0;
    //         }
    //         if (haveTLSPenalty || minorPenalty > 0) {
    //             // add tls penalties to the minimum travel time
    //             SUMOTime minPenalty = -1;
    //             for (const MSLane* const l : *myLanes) {
    //                 for (const MSLink* const link : l->getLinkCont()) {
    //                     if (link->getLane()->isWalkingArea() && link->getLaneBefore()->isNormal()) {
    //                         continue;
    //                     }
    //                     SUMOTime linkPenalty = link->isTLSControlled() ? link->getMesoTLSPenalty() : (link->havePriority() ? 0 : minorPenalty);
    //                     if (minPenalty == -1) {
    //                         minPenalty = linkPenalty;
    //                     } else {
    //                         minPenalty = MIN2(minPenalty, linkPenalty);
    //                     }
    //                 }
    //             }
    //             if (minPenalty > 0) {
    //                 myEmptyTraveltime += STEPS2TIME(minPenalty);
    //                 myTimePenalty = STEPS2TIME(minPenalty);
    //             }
    //         }
    //     } else if (isCrossing() && MSGlobals::gTLSPenalty > 0) {
    //         // penalties are recorded for the entering link
    //         for (const auto& ili : myLanes->front()->getIncomingLanes()) {
    //             double penalty = STEPS2TIME(ili.viaLink->getMesoTLSPenalty());
    //             if (!ili.viaLink->haveOffPriority()) {
    //                 penalty = MAX2(penalty, MSGlobals::gMinorPenalty);
    //             }
    //             if (penalty > 0) {
    //                 myEmptyTraveltime += penalty;
    //                 myTimePenalty = penalty;
    //             }
    //         }
    //     } else if (isInternal() && MSGlobals::gUsingInternalLanes) {
    //         const MSLink* link = myLanes->front()->getIncomingLanes()[0].viaLink;
    //         if (!link->isTLSControlled() && !link->havePriority()) {
    //             if (link->isTurnaround()) {
    //                 myEmptyTraveltime += MSGlobals::gTurnaroundPenalty;
    //                 myTimePenalty = MSGlobals::gTurnaroundPenalty;
    //             } else {
    //                 myEmptyTraveltime += MSGlobals::gMinorPenalty;
    //                 myTimePenalty = MSGlobals::gMinorPenalty;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recalc_cache(&mut self) {
        if self.myLanes.is_empty() {
            return;
        }
        self.myLength = self.myLanes[0].get_length();
        let speed = self.get_speed_limit();
        // MAX2(getSpeedLimit(), NUMERICAL_EPS)
        self.myEmptyTraveltime = self.myLength / (if speed > NUMERICAL_EPS { speed } else { NUMERICAL_EPS });
        if self.is_normal() && (G_USE_MESO_SIM || G_TLS_PENALTY > 0.) {
            let mut minorPenalty: SUMOTime = 0;
            let mut haveTLSPenalty = G_TLS_PENALTY > 0.;
            if G_USE_MESO_SIM {
                let net = MSNet::get_instance();
                let edgeType = net.m().get_meso_type(self.get_edge_type());
                minorPenalty = edgeType.minorPenalty;
                haveTLSPenalty = edgeType.tlsPenalty > 0.;
            }
            if haveTLSPenalty || minorPenalty > 0 {
                // add tls penalties to the minimum travel time
                let mut minPenalty: SUMOTime = -1;
                for l in self.myLanes.iter() {
                    for link in l.get_link_cont() {
                        if link.get_lane().is_walking_area() && link.get_lane_before().is_normal() {
                            continue;
                        }
                        let linkPenalty = if link.is_tls_controlled() {
                            link.get_meso_tls_penalty()
                        } else if link.have_priority() {
                            0
                        } else {
                            minorPenalty
                        };
                        if minPenalty == -1 {
                            minPenalty = linkPenalty;
                        } else {
                            minPenalty = std::cmp::min(minPenalty, linkPenalty);
                        }
                    }
                }
                if minPenalty > 0 {
                    self.myEmptyTraveltime += steps2time(minPenalty);
                    self.myTimePenalty = steps2time(minPenalty);
                }
            }
        } else if self.is_crossing() && G_TLS_PENALTY > 0. {
            // penalties are recorded for the entering link
            for ili in self.myLanes[0].get_incoming_lanes() {
                let mut penalty = steps2time(ili.viaLink.get_meso_tls_penalty());
                if !ili.viaLink.have_off_priority() {
                    penalty = if penalty > G_MINOR_PENALTY { penalty } else { G_MINOR_PENALTY };
                }
                if penalty > 0. {
                    self.myEmptyTraveltime += penalty;
                    self.myTimePenalty = penalty;
                }
            }
        } else if self.is_internal() && G_USING_INTERNAL_LANES {
            let link = self.myLanes[0].get_incoming_lanes()[0].viaLink;
            if !link.is_tls_controlled() && !link.have_priority() {
                if link.is_turnaround() {
                    self.myEmptyTraveltime += G_TURNAROUND_PENALTY;
                    self.myTimePenalty = G_TURNAROUND_PENALTY;
                } else {
                    self.myEmptyTraveltime += G_MINOR_PENALTY;
                    self.myTimePenalty = G_MINOR_PENALTY;
                }
            }
        }
    }

    /// C++: `void MSEdge::closeBuilding()` [virtual]
    // void
    // MSEdge::closeBuilding() {
    //     for (MSLane* const lane : *myLanes) {
    //         for (MSLink* const link : lane->getLinkCont()) {
    //             link->initParallelLinks();
    //             MSLane* const toL = link->getLane();
    //             MSLane* const viaL = link->getViaLane();
    //             if (toL != nullptr) {
    //                 MSEdge& to = toL->getEdge();
    //                 if (std::find(mySuccessors.begin(), mySuccessors.end(), &to) == mySuccessors.end()) {
    //                     mySuccessors.push_back(&to);
    //                     myViaSuccessors.push_back(std::make_pair(&to, (viaL == nullptr ? nullptr : &viaL->getEdge())));
    //                 }
    //                 if (std::find(to.myPredecessors.begin(), to.myPredecessors.end(), this) == to.myPredecessors.end()) {
    //                     to.myPredecessors.push_back(this);
    //                 }
    //                 if (link->getDirection() != LinkDirection::TURN) {
    //                     myAmFringe = false;
    //                 }
    //             }
    //             if (viaL != nullptr) {
    //                 MSEdge& to = viaL->getEdge();
    //                 if (std::find(to.myPredecessors.begin(), to.myPredecessors.end(), this) == to.myPredecessors.end()) {
    //                     to.myPredecessors.push_back(this);
    //                 }
    //             }
    //         }
    //         lane->checkBufferType();
    //     }
    //     std::sort(mySuccessors.begin(), mySuccessors.end(), by_id_sorter());
    //     rebuildAllowedLanes(true);
    //     recalcCache();
    //
    //     // extend lookup table for sublane model after all edges are read
    //     if (myLanes->back()->getOpposite() != nullptr) {
    //         MSLane* opposite = myLanes->back()->getOpposite();
    //         MSLeaderInfo ahead(opposite->getWidth());
    //         for (int j = 0; j < ahead.numSublanes(); ++j) {
    //             mySublaneSides.push_back(myWidth + j * MSGlobals::gLateralResolution);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_building(&mut self) {
        let me = Ptr::from_mut(self);
        let laneList = self.myLanes.clone();
        for lane in laneList.iter() {
            let links = lane.get_link_cont().clone();
            for link in links.iter() {
                link.m().init_parallel_links();
                let toL = link.get_lane();
                let viaL = link.get_via_lane();
                if !toL.is_null() {
                    let to = Ptr::from_mut(toL.get_edge());
                    if !self.mySuccessors.contains(&to) {
                        self.mySuccessors.push(to);
                        let viaEdge = if viaL.is_null() { Ptr::null() } else { Ptr::from_mut(viaL.get_edge()) };
                        self.myViaSuccessors.push((to, viaEdge));
                    }
                    if !to.m().myPredecessors.contains(&me) {
                        to.m().myPredecessors.push(me);
                    }
                    if link.get_direction() != LinkDirection::TURN {
                        self.myAmFringe = false;
                    }
                }
                if !viaL.is_null() {
                    let to = Ptr::from_mut(viaL.get_edge());
                    if !to.m().myPredecessors.contains(&me) {
                        to.m().myPredecessors.push(me);
                    }
                }
            }
            lane.m().check_buffer_type();
        }
        // determinism: numerical ids are unique, so the sort order is total
        self.mySuccessors.sort_by_key(|e| e.get_numerical_id());
        self.rebuild_allowed_lanes(true, false);
        self.recalc_cache();

        // extend lookup table for sublane model after all edges are read
        let last = self.myLanes.last().copied();
        if let Some(last) = last {
            let opposite = last.get_opposite();
            if !opposite.is_null() {
                let ahead = MSLeaderInfo::new(opposite.get_width(), Ptr::null(), 0.);
                for j in 0..ahead.num_sublanes() {
                    self.mySublaneSides.push(self.myWidth + j as f64 * G_LATERAL_RESOLUTION);
                }
            }
        }
    }

    /// C++: `void MSEdge::buildLaneChanger()`
    /// Has to be called after all successors and predecessors have been set (after closeBuilding())
    // void
    // MSEdge::buildLaneChanger() {
    //     if (!myLanes->empty()) {
    //         const bool allowChanging = allowsLaneChanging();
    //         if (MSGlobals::gLateralResolution > 0) {
    //             // may always initiate sublane-change
    //             if (!isInternal() || MSGlobals::gUsingInternalLanes) {
    //                 myLaneChanger = new MSLaneChangerSublane(myLanes.get(), allowChanging);
    //             }
    //         } else {
    //             if (MSGlobals::gLaneChangeDuration > 0) {
    //                 myLaneChanger = new MSLaneChanger(myLanes.get(), allowChanging);
    //             } else if (myLanes->size() > 1 || canChangeToOpposite()) {
    //                 myLaneChanger = new MSLaneChanger(myLanes.get(), allowChanging);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_lane_changer(&mut self) {
        if !self.myLanes.is_empty() {
            let allowChanging = self.allows_lane_changing();
            let lanes = Ptr::new(self.myLanes.as_ref());
            if G_LATERAL_RESOLUTION > 0. {
                // may always initiate sublane-change
                if !self.is_internal() || G_USING_INTERNAL_LANES {
                    // shortcut: sublane changer is stored through its MSLaneChanger base pointer, only reachable when lateral-resolution > 0
                    let sub = Box::into_raw(Box::new(MSLaneChangerSublane::new(lanes, allowChanging)));
                    self.myLaneChanger = Ptr::from_raw(unsafe { &mut (*sub).base } as *mut MSLaneChanger);
                }
            } else if G_LANE_CHANGE_DURATION > 0 || self.myLanes.len() > 1 || self.can_change_to_opposite() {
                self.myLaneChanger = Ptr::from_raw(Box::into_raw(Box::new(MSLaneChanger::new(lanes, allowChanging))));
            }
        }
    }

    /// C++: `bool MSEdge::allowsLaneChanging() const` [const]
    // bool
    // MSEdge::allowsLaneChanging() const {
    //     if (isInternal() && MSGlobals::gUsingInternalLanes) {
    //         // allow changing only if all links leading to this internal lane have priority
    //         // or they are controlled by a traffic light
    //         for (const MSLane* const lane : *myLanes) {
    //             const MSLink* const link = lane->getLogicalPredecessorLane()->getLinkTo(lane);
    //             assert(link != nullptr);
    //             const LinkState state = link->getState();
    //             if ((state == LINKSTATE_MINOR && lane->getBidiLane() == nullptr)
    //                     || state == LINKSTATE_EQUAL
    //                     || state == LINKSTATE_STOP
    //                     || state == LINKSTATE_ALLWAY_STOP
    //                     || state == LINKSTATE_DEADEND) {
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn allows_lane_changing(&self) -> bool {
        if self.is_internal() && G_USING_INTERNAL_LANES {
            // allow changing only if all links leading to this internal lane have priority
            // or they are controlled by a traffic light
            for lane in self.myLanes.iter() {
                let link = lane.get_logical_predecessor_lane().get_link_to(*lane);
                let state = link.get_state();
                if (state == LINKSTATE_MINOR && lane.get_bidi_lane().is_null())
                    || state == LINKSTATE_EQUAL
                    || state == LINKSTATE_STOP
                    || state == LINKSTATE_ALLWAY_STOP
                    || state == LINKSTATE_DEADEND
                {
                    return false;
                }
            }
        }
        true
    }

    /// C++: `MSLane * MSEdge::leftLane(const MSLane *const) const` [const]
    /// Returns the lane left to the one given, 0 if the given lane is leftmost
    // MSLane*
    // MSEdge::leftLane(const MSLane* const lane) const {
    //     return parallelLane(lane, 1);
    // }
    #[allow(unused_variables)]
    pub fn left_lane(&self, lane: Ptr<MSLane>) -> Ptr<MSLane> {
        self.parallel_lane(lane, 1, true)
    }

    /// C++: `MSLane * MSEdge::rightLane(const MSLane *const) const` [const]
    /// Returns the lane right to the one given, 0 if the given lane is rightmost
    // MSLane*
    // MSEdge::rightLane(const MSLane* const lane) const {
    //     return parallelLane(lane, -1);
    // }
    #[allow(unused_variables)]
    pub fn right_lane(&self, lane: Ptr<MSLane>) -> Ptr<MSLane> {
        self.parallel_lane(lane, -1, true)
    }

    /// C++: `MSLane * MSEdge::parallelLane(const MSLane *const, int, bool) const` [const]
    /// Returns the lane with the given offset parallel to the given lane one or 0 if it does not exist
    // MSLane*
    // MSEdge::parallelLane(const MSLane* const lane, int offset, bool includeOpposite) const {
    //     const int resultIndex = lane->getIndex() + offset;
    //     if (resultIndex >= getNumLanes() && includeOpposite) {
    //         const MSEdge* opposite = getOppositeEdge();
    //         if (opposite != nullptr && resultIndex < getNumLanes() + opposite->getNumLanes()) {
    //             return opposite->getLanes()[opposite->getNumLanes() + getNumLanes() - resultIndex - 1];
    //         }
    //         return nullptr;
    //     } else if (resultIndex >= (int)myLanes->size() || resultIndex < 0) {
    //         return nullptr;
    //     } else {
    //         return (*myLanes)[resultIndex];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parallel_lane(&self, lane: Ptr<MSLane>, offset: i32, includeOpposite: bool) -> Ptr<MSLane> {
        let resultIndex = lane.get_index() + offset;
        if resultIndex >= self.get_num_lanes() && includeOpposite {
            let opposite = self.get_opposite_edge();
            if !opposite.is_null() && resultIndex < self.get_num_lanes() + opposite.get_num_lanes() {
                return opposite.get_lanes()[(opposite.get_num_lanes() + self.get_num_lanes() - resultIndex - 1) as usize];
            }
            return Ptr::null();
        } else if resultIndex >= self.myLanes.len() as i32 || resultIndex < 0 {
            return Ptr::null();
        } else {
            return self.myLanes[resultIndex as usize];
        }
    }

    /// C++: `const std::vector<MSLane *> & MSEdge::getLanes() const` [const]
    /// Returns this edge's lanes
    // inline const std::vector<MSLane*>& getLanes() const {
    //         return *myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&self) -> &Vec<Ptr<MSLane>> {
        &self.myLanes
    }

    /// C++: `int MSEdge::getNumLanes() const` [const]
    // inline int getNumLanes() const {
    //         return (int)myLanes->size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_lanes(&self) -> i32 {
        self.myLanes.len() as i32
    }

    /// C++: `int MSEdge::getNumDrivingLanes() const` [const]
    /// return the number of lanes that permit non-weak modes if the edge allows non weak modes and the number of lanes otherwise
    // int
    // MSEdge::getNumDrivingLanes() const {
    //     int result = 0;
    //     SVCPermissions filter = SVCAll;
    //     if ((myCombinedPermissions & ~(SVC_PEDESTRIAN | SVC_WHEELCHAIR)) != 0) {
    //         filter = ~(SVC_PEDESTRIAN | SVC_WHEELCHAIR);
    //     } else if ((myCombinedPermissions & (SVC_PEDESTRIAN | SVC_WHEELCHAIR)) != 0) {
    //         // filter out green verge
    //         filter = (SVC_PEDESTRIAN | SVC_WHEELCHAIR);
    //     }
    //     for (const MSLane* const l : *myLanes) {
    //         if ((l->getPermissions() & filter) != 0) {
    //             result++;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_num_driving_lanes(&self) -> i32 {
        let mut result = 0;
        let mut filter: SVCPermissions = SVC_ALL;
        if (self.myCombinedPermissions & !(SVC_PEDESTRIAN | SVC_WHEELCHAIR)) != 0 {
            filter = !(SVC_PEDESTRIAN | SVC_WHEELCHAIR);
        } else if (self.myCombinedPermissions & (SVC_PEDESTRIAN | SVC_WHEELCHAIR)) != 0 {
            // filter out green verge
            filter = SVC_PEDESTRIAN | SVC_WHEELCHAIR;
        }
        for l in self.myLanes.iter() {
            if (l.get_permissions() & filter) != 0 {
                result += 1;
            }
        }
        result
    }

    /// C++: `int MSEdge::getVehicleNumber() const` [const]
    /// return total number of vehicles on this edges lanes or segments
    // int
    // MSEdge::getVehicleNumber() const {
    //     return (int)getVehicles().size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_number(&self) -> i32 {
        self.get_vehicles().len() as i32
    }

    /// C++: `bool MSEdge::isEmpty() const` [const]
    /// whether this edge has no vehicles
    // bool
    // MSEdge::isEmpty() const {
    //     /// more efficient than retrieving vehicle number
    //     if (MSGlobals::gUseMesoSim) {
    //         for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //             if (segment->getCarNumber() > 0) {
    //                 return false;
    //             }
    //         }
    //     } else {
    //         for (MSLane* lane : getLanes()) {
    //             if (lane->getVehicleNumber() > 0) {
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        // shortcut: meso branch (MSGlobals::gMesoNet segment walk) not ported, unreachable while G_USE_MESO_SIM is false
        for lane in self.get_lanes() {
            if lane.get_vehicle_number() > 0 {
                return false;
            }
        }
        true
    }

    /// C++: `std::vector<const SUMOVehicle *> MSEdge::getVehicles() const` [const]
    /// return vehicles on this edges lanes or segments
    // std::vector<const SUMOVehicle*>
    // MSEdge::getVehicles() const {
    //     std::vector<const SUMOVehicle*> result;
    //     if (MSGlobals::gUseMesoSim) {
    //         for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //             std::vector<const MEVehicle*> segmentVehs = segment->getVehicles();
    //             result.insert(result.end(), segmentVehs.begin(), segmentVehs.end());
    //         }
    //     } else {
    //         for (MSLane* lane : getLanes()) {
    //             for (auto veh : lane->getVehiclesSecure()) {
    //                 result.push_back(veh);
    //             }
    //             lane->releaseVehicles();
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicles(&self) -> Vec<Ptr<SUMOVehicle>> {
        // shortcut: meso branch (MEVehicle segment lists) not ported, unreachable while G_USE_MESO_SIM is false
        let mut result = Vec::new();
        for lane in self.get_lanes() {
            for veh in lane.get_vehicles_secure().iter() {
                result.push(Ptr::from_mut(&mut veh.m().base.base));
            }
            lane.release_vehicles();
        }
        result
    }

    /// C++: `double MSEdge::getBruttoOccupancy() const` [const]
    // double
    // MSEdge::getBruttoOccupancy() const {
    //     if (myLanes->size() == 0) {
    //         return 0;
    //     }
    //     double occ = 0;
    //     for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //         occ += segment->getBruttoOccupancy();
    //     }
    //     return occ / (*myLanes)[0]->getLength() / (double)(myLanes->size());
    // }
    #[allow(unused_variables)]
    pub fn get_brutto_occupancy(&self) -> f64 {
        if self.myLanes.len() == 0 {
            return 0.;
        }
        // shortcut: only the meso segment walk is in C++ (gMesoNet is null in non-meso runs); meso segments not ported, upgrade with MESegment
        0.
    }

    /// C++: `double MSEdge::getFlow() const` [const]
    /// return flow based on meanSpeed
    // double
    // MSEdge::getFlow() const {
    //     if (myLanes->size() == 0) {
    //         return 0;
    //     }
    //     double flow = 0;
    //     for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //         flow += (double) segment->getCarNumber() * segment->getMeanSpeed();
    //     }
    //     return 3600 * flow / (*myLanes)[0]->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_flow(&self) -> f64 {
        if self.myLanes.len() == 0 {
            return 0.;
        }
        // shortcut: only the meso segment walk is in C++ (gMesoNet is null in non-meso runs); meso segments not ported, upgrade with MESegment
        0.
    }

    /// C++: `double MSEdge::getWaitingSeconds() const` [const]
    /// return accumulated waiting time for all vehicles on this edges lanes or segments
    // double
    // MSEdge::getWaitingSeconds() const {
    //     double wtime = 0;
    //     if (MSGlobals::gUseMesoSim) {
    //         for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //             wtime += segment->getWaitingSeconds();
    //         }
    //     } else {
    //         for (MSLane* lane : getLanes()) {
    //             wtime += lane->getWaitingSeconds();
    //         }
    //     }
    //     return wtime;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        // shortcut: meso branch (MESegment::getWaitingSeconds) not ported, unreachable while G_USE_MESO_SIM is false
        let mut wtime = 0.;
        for lane in self.get_lanes() {
            wtime += lane.get_waiting_seconds();
        }
        wtime
    }

    /// C++: `double MSEdge::getOccupancy() const` [const]
    /// return mean occupancy on this edges lanes or segments
    // double
    // MSEdge::getOccupancy() const {
    //     if (myLanes->size() == 0) {
    //         return 0;
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         /// @note MESegment only tracks brutto occupancy so we compute this from sratch
    //         double sum = 0;
    //         for (const SUMOVehicle* veh : getVehicles()) {
    //             sum += dynamic_cast<const MEVehicle*>(veh)->getVehicleType().getLength();
    //         }
    //         return sum / (myLength * (double)myLanes->size());
    //     } else {
    //         double sum = 0;
    //         for (auto lane : getLanes()) {
    //             sum += lane->getNettoOccupancy();
    //         }
    //         return sum / (double)myLanes->size();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy(&self) -> f64 {
        if self.myLanes.len() == 0 {
            return 0.;
        }
        // shortcut: meso branch (vehicle-type length sum) not ported, unreachable while G_USE_MESO_SIM is false
        let mut sum = 0.;
        for lane in self.get_lanes() {
            sum += lane.get_netto_occupancy();
        }
        sum / (self.myLanes.len() as f64)
    }

    /// C++: `const std::set<MSTransportable *, ComparatorNumericalIdLess> & MSEdge::getPersons() const` [const]
    /// Avoids the creation of new vector as in getSortedPersons
    // inline const std::set<MSTransportable*, ComparatorNumericalIdLess>& getPersons() const {
    //         return myPersons;
    //     }
    #[allow(unused_variables)]
    pub fn get_persons(&self) -> &BTreeSet<Ptr<MSTransportable>> {
        &self.myPersons
    }

    /// C++: `std::vector<MSTransportable *> MSEdge::getSortedPersons(SUMOTime, bool) const` [const]
    /// Returns this edge's persons sorted by pos
    // std::vector<MSTransportable*>
    // MSEdge::getSortedPersons(SUMOTime timestep, bool includeRiding) const {
    //     std::vector<MSTransportable*> result(myPersons.begin(), myPersons.end());
    //     if (includeRiding) {
    //         for (std::vector<MSLane*>::const_iterator i = myLanes->begin(); i != myLanes->end(); ++i) {
    //             const MSLane::VehCont& vehs = (*i)->getVehiclesSecure();
    //             for (MSLane::VehCont::const_iterator j = vehs.begin(); j != vehs.end(); ++j) {
    //                 const std::vector<MSTransportable*>& persons = (*j)->getPersons();
    //                 result.insert(result.end(), persons.begin(), persons.end());
    //             }
    //             (*i)->releaseVehicles();
    //         }
    //     }
    //     sort(result.begin(), result.end(), transportable_by_position_sorter(timestep));
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_sorted_persons(&self, timestep: SUMOTime, includeRiding: bool) -> Vec<Ptr<MSTransportable>> {
        let mut result: Vec<Ptr<MSTransportable>> = self.myPersons.iter().copied().collect();
        if includeRiding {
            for lane in self.myLanes.iter() {
                for veh in lane.get_vehicles_secure().iter() {
                    result.extend(veh.get_persons().iter().copied());
                }
                lane.release_vehicles();
            }
        }
        // determinism: transportable ids are unique, so the position/id order is total
        let sorter = MSEdge_transportable_by_position_sorter::new(timestep);
        result.sort_by(|a, b| sort_by_sorter(|x, y| sorter.call_op(x, y), *a, *b));
        result
    }

    /// C++: `std::vector<MSTransportable *> MSEdge::getSortedContainers(SUMOTime, bool) const` [const]
    /// Returns this edge's containers sorted by pos
    // std::vector<MSTransportable*>
    // MSEdge::getSortedContainers(SUMOTime timestep, bool /* includeRiding */) const {
    //     std::vector<MSTransportable*> result(myContainers.begin(), myContainers.end());
    //     sort(result.begin(), result.end(), transportable_by_position_sorter(timestep));
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_sorted_containers(&self, timestep: SUMOTime, includeRiding: bool) -> Vec<Ptr<MSTransportable>> {
        let mut result: Vec<Ptr<MSTransportable>> = self.myContainers.iter().copied().collect();
        // determinism: transportable ids are unique, so the position/id order is total
        let sorter = MSEdge_transportable_by_position_sorter::new(timestep);
        result.sort_by(|a, b| sort_by_sorter(|x, y| sorter.call_op(x, y), *a, *b));
        result
    }

    /// C++: `const std::vector<MSLane *> * MSEdge::allowedLanes(const MSEdge &, SUMOVehicleClass, bool) const` [const]
    /// Get the allowed lanes to reach the destination-edge.
    // const std::vector<MSLane*>*
    // MSEdge::allowedLanes(const MSEdge& destination, SUMOVehicleClass vclass, bool ignoreTransientPermissions) const {
    //     const auto& targets = ignoreTransientPermissions && myHaveTransientPermissions ? myOrigAllowedTargets : myAllowedTargets;
    //     AllowedLanesByTarget::const_iterator i = targets.find(&destination);
    //     if (i != targets.end()) {
    //         for (const auto& allowed : i->second) {
    //             if ((allowed.first & vclass) == vclass) {
    //                 return allowed.second.get();
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn allowed_lanes(&self, destination: &MSEdge, vclass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> Ptr<Vec<Ptr<MSLane>>> {
        let targets = if ignoreTransientPermissions && self.myHaveTransientPermissions { &self.myOrigAllowedTargets } else { &self.myAllowedTargets };
        if let Some(cont) = targets.get(&Ptr::new(destination)) {
            for (allowed, lanes) in cont.iter() {
                if (allowed & vclass) == vclass {
                    return Ptr::new(&**lanes);
                }
            }
        }
        Ptr::null()
    }

    /// C++: `const std::vector<MSLane *> * MSEdge::allowedLanes(SUMOVehicleClass) const` [const]
    /// Get the allowed lanes for the given vehicle class.
    // const std::vector<MSLane*>*
    // MSEdge::allowedLanes(SUMOVehicleClass vclass) const {
    //     if ((myMinimumPermissions & vclass) == vclass) {
    //         return myLanes.get();
    //     } else {
    //         if ((myCombinedPermissions & vclass) == vclass) {
    //             for (const auto& allowed : myAllowed) {
    //                 if ((allowed.first & vclass) == vclass) {
    //                     return allowed.second.get();
    //                 }
    //             }
    //         }
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn allowed_lanes_2(&self, vclass: SUMOVehicleClass) -> Ptr<Vec<Ptr<MSLane>>> {
        if (self.myMinimumPermissions & vclass) == vclass {
            return Ptr::new(&*self.myLanes);
        } else {
            if (self.myCombinedPermissions & vclass) == vclass {
                for (allowed, lanes) in self.myAllowed.iter() {
                    if (allowed & vclass) == vclass {
                        return Ptr::new(&**lanes);
                    }
                }
            }
            return Ptr::null();
        }
    }

    /// C++: `const std::vector<MSLane *> * MSEdge::allowedLanes(SUMOVehicleClass, bool) const` [const]
    // const std::vector<MSLane*>*
    // MSEdge::allowedLanes(SUMOVehicleClass vclass, bool ignoreTransientPermissions) const {
    //     const SVCPermissions& minP = ignoreTransientPermissions ? myOriginalMinimumPermissions : myMinimumPermissions;
    //     if ((minP & vclass) == vclass) {
    //         return myLanes.get();
    //     } else {
    //         const SVCPermissions comP = ignoreTransientPermissions ? myOriginalCombinedPermissions : myCombinedPermissions;
    //         if ((comP & vclass) == vclass) {
    //             const AllowedLanesCont& allowedCont = ignoreTransientPermissions && myHaveTransientPermissions ? myOrigAllowed : myAllowed;
    //             for (const auto& allowed : allowedCont) {
    //                 if ((allowed.first & vclass) == vclass) {
    //                     return allowed.second.get();
    //                 }
    //             }
    //         }
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn allowed_lanes_3(&self, vclass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> Ptr<Vec<Ptr<MSLane>>> {
        let minP = if ignoreTransientPermissions { self.myOriginalMinimumPermissions } else { self.myMinimumPermissions };
        if (minP & vclass) == vclass {
            return Ptr::new(&*self.myLanes);
        } else {
            let comP = if ignoreTransientPermissions { self.myOriginalCombinedPermissions } else { self.myCombinedPermissions };
            if (comP & vclass) == vclass {
                let allowedCont = if ignoreTransientPermissions && self.myHaveTransientPermissions { &self.myOrigAllowed } else { &self.myAllowed };
                for (allowed, lanes) in allowedCont.iter() {
                    if (allowed & vclass) == vclass {
                        return Ptr::new(&**lanes);
                    }
                }
            }
            return Ptr::null();
        }
    }

    /// C++: `bool MSEdge::isConnectedTo(const MSEdge &, SUMOVehicleClass, bool) const` [const]
    // inline bool isConnectedTo(const MSEdge& destination, SUMOVehicleClass vclass, bool ignoreTransientPermissions = false) const {
    //         const std::vector<MSLane*>* const lanes = allowedLanes(destination, vclass, ignoreTransientPermissions);
    //         return lanes != nullptr && !lanes->empty();
    //     }
    #[allow(unused_variables)]
    pub fn is_connected_to(&self, destination: &MSEdge, vclass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> bool {
        let lanes = self.allowed_lanes(destination, vclass, ignoreTransientPermissions);
        match lanes.get() {
            Some(l) => !l.is_empty(),
            None => false,
        }
    }

    /// C++: `SumoXMLEdgeFunc MSEdge::getFunction() const` [const]
    /// Returns the edge type (SumoXMLEdgeFunc)
    // inline SumoXMLEdgeFunc getFunction() const {
    //         return myFunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_function(&self) -> SumoXMLEdgeFunc {
        self.myFunction
    }

    /// C++: `bool MSEdge::isNormal() const` [const]
    /// return whether this edge is an internal edge
    // inline bool isNormal() const {
    //         return myFunction == SumoXMLEdgeFunc::NORMAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_normal(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::NORMAL
    }

    /// C++: `bool MSEdge::isInternal() const` [const]
    /// return whether this edge is an internal edge
    // inline bool isInternal() const {
    //         return myFunction == SumoXMLEdgeFunc::INTERNAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::INTERNAL
    }

    /// C++: `bool MSEdge::isCrossing() const` [const]
    /// return whether this edge is a pedestrian crossing
    // inline bool isCrossing() const {
    //         return myFunction == SumoXMLEdgeFunc::CROSSING;
    //     }
    #[allow(unused_variables)]
    pub fn is_crossing(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::CROSSING
    }

    /// C++: `void MSEdge::checkAndRegisterBiDirEdge(const std::string &)`
    /// check and register the opposite superposable edge if any
    // void
    // MSEdge::checkAndRegisterBiDirEdge(const std::string& bidiID) {
    //     if (bidiID != "") {
    //         myBidiEdge = dictionary(bidiID);
    //         if (myBidiEdge == nullptr) {
    //             WRITE_ERRORF(TL("Bidi-edge '%' does not exist"), bidiID);
    //         }
    //         setBidiLanes();
    //         return;
    //     }
    //     if (getFunction() != SumoXMLEdgeFunc::NORMAL) {
    //         return;
    //     }
    //     // legacy networks (no bidi attribute)
    //     ConstMSEdgeVector candidates = myToJunction->getOutgoing();
    //     for (ConstMSEdgeVector::const_iterator it = candidates.begin(); it != candidates.end(); it++) {
    //         if ((*it)->getToJunction() == myFromJunction) { //reverse edge
    //             if (myBidiEdge != nullptr && isSuperposable(*it)) {
    //                 WRITE_WARNINGF(TL("Ambiguous superposable edges between junction '%' and '%'."), myToJunction->getID(), myFromJunction->getID());
    //                 break;
    //             }
    //             if (isSuperposable(*it)) {
    //                 myBidiEdge = *it;
    //                 setBidiLanes();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_and_register_bi_dir_edge(&mut self, bidiID: &str) {
        if bidiID != "" {
            self.myBidiEdge = MSEdge::dictionary_2(bidiID);
            if self.myBidiEdge.is_null() {
                utils::common::msg_handler::write_error(&format!("Bidi-edge '{}' does not exist", bidiID));
            }
            self.set_bidi_lanes();
            return;
        }
        if self.get_function() != SumoXMLEdgeFunc::NORMAL {
            return;
        }
        // legacy networks (no bidi attribute)
        let candidates = self.myToJunction.get_outgoing().clone();
        for it in candidates.iter() {
            if it.get_to_junction() == self.myFromJunction {
                // reverse edge
                if !self.myBidiEdge.is_null() && self.is_superposable(*it) {
                    utils::common::msg_handler::write_warning(&format!(
                        "Ambiguous superposable edges between junction '{}' and '{}'.",
                        self.myToJunction.get_id(),
                        self.myFromJunction.get_id()
                    ));
                    break;
                }
                if self.is_superposable(*it) {
                    self.myBidiEdge = *it;
                    self.set_bidi_lanes();
                }
            }
        }
    }

    /// C++: `const MSEdge * MSEdge::getBidiEdge() const` [const]
    /// return opposite superposable/congruent edge, if it exist and 0 else
    // inline const MSEdge* getBidiEdge() const {
    //         return myBidiEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<MSEdge> {
        self.myBidiEdge
    }

    /// C++: `bool MSEdge::isWalkingArea() const` [const]
    /// return whether this edge is walking area
    // inline bool isWalkingArea() const {
    //         return myFunction == SumoXMLEdgeFunc::WALKINGAREA;
    //     }
    #[allow(unused_variables)]
    pub fn is_walking_area(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::WALKINGAREA
    }

    /// C++: `bool MSEdge::isTazConnector() const` [const]
    // inline bool isTazConnector() const {
    //         return myFunction == SumoXMLEdgeFunc::CONNECTOR;
    //     }
    #[allow(unused_variables)]
    pub fn is_taz_connector(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::CONNECTOR
    }

    /// C++: `void MSEdge::setOtherTazConnector(const MSEdge *)`
    // void setOtherTazConnector(const MSEdge* edge) {
    //         myOtherTazConnector = edge;
    //     }
    #[allow(unused_variables)]
    pub fn set_other_taz_connector(&mut self, edge: Ptr<MSEdge>) {
        self.myOtherTazConnector = edge;
    }

    /// C++: `const MSEdge * MSEdge::getOtherTazConnector() const` [const]
    // const MSEdge* getOtherTazConnector() const {
    //         return myOtherTazConnector;
    //     }
    #[allow(unused_variables)]
    pub fn get_other_taz_connector(&self) -> Ptr<MSEdge> {
        self.myOtherTazConnector
    }

    /// C++: `int MSEdge::getNumericalID() const` [const]
    /// Returns the numerical id of the edge
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `const std::string & MSEdge::getStreetName() const` [const]
    /// Returns the street name of the edge
    // const std::string& getStreetName() const {
    //         return myStreetName;
    //     }
    #[allow(unused_variables)]
    pub fn get_street_name(&self) -> &str {
        &self.myStreetName
    }

    /// C++: `const std::string & MSEdge::getEdgeType() const` [const]
    /// Returns the type of the edge
    // const std::string& getEdgeType() const {
    //         return myEdgeType;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_type(&self) -> &str {
        &self.myEdgeType
    }

    /// C++: `const std::string & MSEdge::getRoutingType() const` [const]
    /// Returns the type of the edge
    // const std::string& getRoutingType() const {
    //         return myRoutingType.empty() ? myEdgeType : myRoutingType;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_type(&self) -> &str {
        if self.myRoutingType.is_empty() { &self.myEdgeType } else { &self.myRoutingType }
    }

    /// C++: `double MSEdge::getPreference(const SUMOVTypeParameter &) const` [const]
    // double
    // MSEdge::getPreference(const SUMOVTypeParameter& pars) const {
    //     return MSNet::getInstance()->getPreference(getRoutingType(), pars);
    // }
    #[allow(unused_variables)]
    pub fn get_preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        MSNet::get_instance().get_preference(self.get_routing_type(), pars)
    }

    /// C++: `void MSEdge::inferEdgeType()`
    // void
    // MSEdge::inferEdgeType() {
    //     // @note must be called after closeBuilding() to ensure successors and
    //     // predecessors are set
    //     if (isInternal() && myEdgeType == "") {
    //         const std::string typeBefore = getNormalBefore()->getEdgeType();
    //         if (typeBefore != "") {
    //             const std::string typeAfter = getNormalSuccessor()->getEdgeType();
    //             if (typeBefore == typeAfter) {
    //                 myEdgeType = typeBefore;
    //             } else if (typeAfter != "") {
    //                 MSNet* net = MSNet::getInstance();
    //                 auto resBefore = net->getRestrictions(typeBefore);
    //                 auto resAfter = net->getRestrictions(typeAfter);
    //                 if (resBefore != nullptr && resAfter != nullptr) {
    //                     // create new restrictions for this type-combination
    //                     myEdgeType = typeBefore + "|" + typeAfter;
    //                     if (net->getRestrictions(myEdgeType) == nullptr) {
    //                         for (const auto& item : *resBefore) {
    //                             const SUMOVehicleClass svc = item.first;
    //                             const double speed = item.second;
    //                             const auto it = (*resAfter).find(svc);
    //                             if (it != (*resAfter).end()) {
    //                                 const double speed2 = it->second;
    //                                 const double newSpeed = (MSNet::getInstance()->hasJunctionHigherSpeeds()
    //                                                          ? MAX2(speed, speed2) : (speed + speed2) / 2);
    //                                 net->addRestriction(myEdgeType, svc, newSpeed);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn infer_edge_type(&mut self) {
        // must be called after closeBuilding() to ensure successors and predecessors are set
        if self.is_internal() && self.myEdgeType.is_empty() {
            let typeBefore = self.get_normal_before().get_edge_type().to_string();
            if !typeBefore.is_empty() {
                let typeAfter = self.get_normal_successor().get_edge_type().to_string();
                if typeBefore == typeAfter {
                    self.myEdgeType = typeBefore;
                } else if !typeAfter.is_empty() {
                    let net = MSNet::get_instance();
                    let resBefore = net.get_restrictions(&typeBefore);
                    let resAfter = net.get_restrictions(&typeAfter);
                    if !resBefore.is_null() && !resAfter.is_null() {
                        // create new restrictions for this type-combination
                        self.myEdgeType = format!("{}|{}", typeBefore, typeAfter);
                        if net.get_restrictions(&self.myEdgeType).is_null() {
                            for (svc, speed) in resBefore.iter() {
                                if let Some(speed2) = (*resAfter).get(svc) {
                                    let newSpeed = if net.has_junction_higher_speeds() { if *speed > *speed2 { *speed } else { *speed2 } } else { (speed + speed2) / 2. };
                                    net.m().add_restriction(&self.myEdgeType, *svc, newSpeed);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++: `int MSEdge::getPriority() const` [const]
    /// Returns the priority of the edge
    // int getPriority() const {
    //         return myPriority;
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        self.myPriority
    }

    /// C++: `double MSEdge::getDistance() const` [const]
    /// Returns the kilometrage/mileage encoding at the start of the edge (negative values encode descending direction)
    // double getDistance() const {
    //         return myDistance;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        self.myDistance
    }

    /// C++: `double MSEdge::getDistanceAt(double) const` [const]
    /// Returns the kilometrage/mileage at the given offset along the edge
    // double
    // MSEdge::getDistanceAt(double pos) const {
    //     // negative values of myDistances indicate descending kilometrage
    //     return fabs(myDistance + pos);
    // }
    #[allow(unused_variables)]
    pub fn get_distance_at(&self, pos: f64) -> f64 {
        (self.myDistance + pos).abs()
    }

    /// C++: `bool MSEdge::hasDistance() const` [const]
    // bool hasDistance() const {
    //         return myDistance != 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_distance(&self) -> bool {
        self.myDistance != 0.
    }

    /// C++: `void MSEdge::setCrossingEdges(const std::vector<std::string> &)`
    /// Sets the crossed edge ids for a crossing edge
    // void setCrossingEdges(const std::vector<std::string>& crossingEdges)        {
    //         myCrossingEdges.clear();
    //         myCrossingEdges.insert(myCrossingEdges.begin(), crossingEdges.begin(), crossingEdges.end());
    //     }
    #[allow(unused_variables)]
    pub fn set_crossing_edges(&mut self, crossingEdges: &Vec<String>) {
        self.myCrossingEdges = crossingEdges.clone();
    }

    /// C++: `const std::vector<std::string> & MSEdge::getCrossingEdges() const` [const]
    /// Gets the crossed edge ids
    // const std::vector<std::string>& getCrossingEdges() const {
    //         return myCrossingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_crossing_edges(&self) -> &Vec<String> {
        &self.myCrossingEdges
    }

    /// C++: `void MSEdge::addSuccessor(MSEdge *, const MSEdge *)`
    /// Adds an edge to the list of edges which may be reached from this edge and to the incoming of the other edge
    // void
    // MSEdge::addSuccessor(MSEdge* edge, const MSEdge* via) {
    //     mySuccessors.push_back(edge);
    //     myViaSuccessors.push_back(std::make_pair(edge, via));
    //     if (isTazConnector() && edge->getFromJunction() != nullptr) {
    //         myBoundary.add(edge->getFromJunction()->getPosition());
    //     }
    //
    //     edge->myPredecessors.push_back(this);
    //     if (edge->isTazConnector() && getToJunction() != nullptr) {
    //         edge->myBoundary.add(getToJunction()->getPosition());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, edge: Ptr<MSEdge>, via: Ptr<MSEdge>) {
        let me = Ptr::from_mut(self);
        self.mySuccessors.push(edge);
        self.myViaSuccessors.push((edge, via));
        if self.is_taz_connector() && !edge.get_from_junction().is_null() {
            self.myBoundary.add_pos(edge.get_from_junction().get_position(false));
        }

        edge.m().myPredecessors.push(me);
        if edge.is_taz_connector() && !self.get_to_junction().is_null() {
            edge.m().myBoundary.add_pos(self.get_to_junction().get_position(false));
        }
    }

    /// C++: `void MSEdge::resetTAZ(MSJunction *)`
    // void
    // MSEdge::resetTAZ(MSJunction* junction) {
    //     mySuccessors.clear();
    //     myPredecessors.clear();
    //     for (const MSEdge* edge : junction->getIncoming()) {
    //         if (!edge->isInternal()) {
    //             MSEdgeVector& succ = const_cast<MSEdgeVector&>(edge->mySuccessors);
    //             MSConstEdgePairVector& succVia = const_cast<MSConstEdgePairVector&>(edge->myViaSuccessors);
    //             MSEdgeVector& pred = const_cast<MSEdgeVector&>(edge->myPredecessors);
    //             auto it = std::find(succ.begin(), succ.end(), this);
    //             auto it2 = std::find(succVia.begin(), succVia.end(), std::make_pair(const_cast<const MSEdge*>(this), (const MSEdge*)nullptr));
    //             auto it3 = std::find(pred.begin(), pred.end(), this);
    //             if (it != succ.end()) {
    //                 succ.erase(it);
    //                 succVia.erase(it2);
    //             }
    //             if (it3 != pred.end()) {
    //                 pred.erase(it3);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_taz(&mut self, junction: Ptr<MSJunction>) {
        self.mySuccessors.clear();
        self.myPredecessors.clear();
        let me = Ptr::from_mut(self);
        for edge in junction.get_incoming().iter() {
            if !edge.is_internal() {
                let e = edge.m();
                let it = e.mySuccessors.iter().position(|x| *x == me);
                let it2 = e.myViaSuccessors.iter().position(|x| *x == (me, Ptr::null()));
                let it3 = e.myPredecessors.iter().position(|x| *x == me);
                if let Some(i) = it {
                    e.mySuccessors.remove(i);
                    if let Some(i2) = it2 {
                        e.myViaSuccessors.remove(i2);
                    }
                }
                if let Some(i3) = it3 {
                    e.myPredecessors.remove(i3);
                }
            }
        }
    }

    /// C++: `int MSEdge::getNumSuccessors() const` [const]
    /// Returns the number of edges that may be reached from this edge
    // int getNumSuccessors() const {
    //         return (int) mySuccessors.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_successors(&self) -> i32 {
        self.mySuccessors.len() as i32
    }

    /// C++: `const MSEdgeVector & MSEdge::getSuccessors(SUMOVehicleClass) const` [const]
    /// Returns the following edges, restricted by vClass
    // const MSEdgeVector&
    // MSEdge::getSuccessors(SUMOVehicleClass vClass) const {
    //     if (vClass == SVC_IGNORING || !MSNet::getInstance()->hasPermissions() || myFunction == SumoXMLEdgeFunc::CONNECTOR) {
    //         return mySuccessors;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(mySuccessorMutex, MSGlobals::gNumThreads > 1);
    // #endif
    //     std::map<SUMOVehicleClass, MSEdgeVector>::iterator i = myClassesSuccessorMap.find(vClass);
    //     if (i == myClassesSuccessorMap.end()) {
    //         // instantiate vector
    //         myClassesSuccessorMap[vClass];
    //         i = myClassesSuccessorMap.find(vClass);
    //         // this vClass is requested for the first time. rebuild all successors
    //         for (MSEdgeVector::const_iterator it = mySuccessors.begin(); it != mySuccessors.end(); ++it) {
    //             if ((*it)->isTazConnector()) {
    //                 i->second.push_back(*it);
    //             } else {
    //                 const std::vector<MSLane*>* allowed = allowedLanes(**it, vClass);
    //                 if (allowed != nullptr && allowed->size() > 0) {
    //                     i->second.push_back(*it);
    //                 }
    //             }
    //         }
    //     }
    //     // can use cached value
    //     return i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &MSEdgeVector {
        if vClass == SVC_IGNORING || !MSNet::get_instance().has_permissions() || self.myFunction == SumoXMLEdgeFunc::CONNECTOR {
            return &self.mySuccessors;
        }
        // C++ caches into mutable maps; mutate through a raw handle as C++ does
        let me = Ptr::new(self).get_mut().unwrap();
        if !me.myClassesSuccessorMap.contains_key(&vClass) {
            // this vClass is requested for the first time. rebuild all successors
            let mut entry: MSEdgeVector = Vec::new();
            for it in self.mySuccessors.iter() {
                if it.is_taz_connector() {
                    entry.push(*it);
                } else {
                    let allowed = self.allowed_lanes(&**it, vClass, false);
                    if allowed.get().map_or(false, |a| !a.is_empty()) {
                        entry.push(*it);
                    }
                }
            }
            me.myClassesSuccessorMap.insert(vClass, entry);
        }
        // can use cached value
        me.myClassesSuccessorMap.get(&vClass).unwrap()
    }

    /// C++: `const MSConstEdgePairVector & MSEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    /// Returns the following edges with internal vias, restricted by vClass
    // const MSConstEdgePairVector&
    // MSEdge::getViaSuccessors(SUMOVehicleClass vClass, bool ignoreTransientPermissions) const {
    //     if (vClass == SVC_IGNORING || !MSNet::getInstance()->hasPermissions() || myFunction == SumoXMLEdgeFunc::CONNECTOR) {
    //         return myViaSuccessors;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(mySuccessorMutex, MSGlobals::gNumThreads > 1);
    // #endif
    //     auto& viaMap = ignoreTransientPermissions && myHaveTransientPermissions ? myOrigClassesViaSuccessorMap : myClassesViaSuccessorMap;
    //     auto i = viaMap.find(vClass);
    //     if (i == viaMap.end()) {
    //         // instantiate vector
    //         MSConstEdgePairVector& result = viaMap[vClass];
    //         // this vClass is requested for the first time. rebuild all successors
    //         for (const auto& viaPair : myViaSuccessors) {
    //             if (viaPair.first->isTazConnector()) {
    //                 result.push_back(viaPair);
    //             } else {
    //                 const std::vector<MSLane*>* allowed = allowedLanes(*viaPair.first, vClass, ignoreTransientPermissions);
    //                 if (allowed != nullptr && allowed->size() > 0) {
    //                     result.push_back(viaPair);
    //                 }
    //             }
    //         }
    //         i = viaMap.find(vClass);
    //     }
    //     return i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &MSConstEdgePairVector {
        if vClass == SVC_IGNORING || !MSNet::get_instance().has_permissions() || self.myFunction == SumoXMLEdgeFunc::CONNECTOR {
            return &self.myViaSuccessors;
        }
        // C++ caches into mutable maps; mutate through a raw handle as C++ does
        let me = Ptr::new(self).get_mut().unwrap();
        let viaMap = if ignoreTransientPermissions && self.myHaveTransientPermissions {
            &mut me.myOrigClassesViaSuccessorMap
        } else {
            &mut me.myClassesViaSuccessorMap
        };
        if !viaMap.contains_key(&vClass) {
            // this vClass is requested for the first time. rebuild all successors
            let mut result: MSConstEdgePairVector = Vec::new();
            for viaPair in self.myViaSuccessors.iter() {
                if viaPair.0.is_taz_connector() {
                    result.push(*viaPair);
                } else {
                    let allowed = self.allowed_lanes(&*viaPair.0, vClass, ignoreTransientPermissions);
                    if allowed.get().map_or(false, |a| !a.is_empty()) {
                        result.push(*viaPair);
                    }
                }
            }
            viaMap.insert(vClass, result);
        }
        viaMap.get(&vClass).unwrap()
    }

    /// C++: `int MSEdge::getNumPredecessors() const` [const]
    /// Returns the number of edges this edge is connected to
    // int getNumPredecessors() const {
    //         return (int) myPredecessors.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_predecessors(&self) -> i32 {
        self.myPredecessors.len() as i32
    }

    /// C++: `const MSEdgeVector & MSEdge::getPredecessors() const` [const]
    /// Returns
    // const MSEdgeVector& getPredecessors() const {
    //         return myPredecessors;
    //     }
    #[allow(unused_variables)]
    pub fn get_predecessors(&self) -> &MSEdgeVector {
        &self.myPredecessors
    }

    /// C++: `const MSJunction * MSEdge::getFromJunction() const` [const]
    // const MSJunction* getFromJunction() const {
    //         return myFromJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<MSJunction> {
        self.myFromJunction
    }

    /// C++: `const MSJunction * MSEdge::getToJunction() const` [const]
    // const MSJunction* getToJunction() const {
    //         return myToJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<MSJunction> {
        self.myToJunction
    }

    /// C++: `void MSEdge::setJunctions(MSJunction *, MSJunction *)`
    // void
    // MSEdge::setJunctions(MSJunction* from, MSJunction* to) {
    //     myFromJunction = from;
    //     myToJunction = to;
    //     if (!isTazConnector()) {
    //         myBoundary.add(from->getPosition());
    //         myBoundary.add(to->getPosition());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junctions(&mut self, from: Ptr<MSJunction>, to: Ptr<MSJunction>) {
        self.myFromJunction = from;
        self.myToJunction = to;
        if !self.is_taz_connector() {
            self.myBoundary.add_pos(from.get_position(false));
            self.myBoundary.add_pos(to.get_position(false));
        }
    }

    /// C++: `bool MSEdge::isVaporizing() const` [const]
    /// Returns whether vehicles on this edge shall be vaporized
    // bool isVaporizing() const {
    //         return myVaporizationRequests > 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_vaporizing(&self) -> bool {
        self.myVaporizationRequests > 0
    }

    /// C++: `SUMOTime MSEdge::incVaporization(SUMOTime)`
    /// Enables vaporization
    // SUMOTime
    // MSEdge::incVaporization(SUMOTime) {
    //     ++myVaporizationRequests;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn inc_vaporization(&mut self, t: SUMOTime) -> SUMOTime {
        self.myVaporizationRequests += 1;
        0
    }

    /// C++: `SUMOTime MSEdge::decVaporization(SUMOTime)`
    /// Disables vaporization
    // SUMOTime
    // MSEdge::decVaporization(SUMOTime) {
    //     --myVaporizationRequests;
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn dec_vaporization(&mut self, t: SUMOTime) -> SUMOTime {
        self.myVaporizationRequests -= 1;
        0
    }

    /// C++: `double MSEdge::getCurrentTravelTime(const double) const` [const]
    /// Computes and returns the current travel time for this edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_current_travel_time(&self, minSpeed: f64) -> f64 {
        assert!(minSpeed > 0.);
        if !self.myAmDelayed.get() {
            return self.myEmptyTraveltime;
        }
        // MAX2(minSpeed, getMeanSpeed())
        let meanSpeed = self.get_mean_speed();
        self.get_length() / (if minSpeed > meanSpeed { minSpeed } else { meanSpeed })
    }

    /// C++: `double MSEdge::getMinimumTravelTime(const SUMOVehicle *const) const` [const]
    /// returns the minimum travel time for the given vehicle
    // inline double getMinimumTravelTime(const SUMOVehicle* const veh) const {
    //         if (myFunction == SumoXMLEdgeFunc::CONNECTOR) {
    //             return 0;
    //         } else if (veh != 0) {
    //             return getLength() / getVehicleMaxSpeed(veh) + myTimePenalty;
    //         } else {
    //             return myEmptyTraveltime;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, veh: Ptr<SUMOVehicle>) -> f64 {
        if self.myFunction == SumoXMLEdgeFunc::CONNECTOR {
            0.
        } else if !veh.is_null() {
            self.get_length() / self.get_vehicle_max_speed(Ptr::from_mut(&mut veh.m().base)) + self.myTimePenalty
        } else {
            self.myEmptyTraveltime
        }
    }

    /// C++: `double MSEdge::getTimePenalty() const` [const]
    // double getTimePenalty() const {
    //         return myTimePenalty;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_penalty(&self) -> f64 {
        self.myTimePenalty
    }

    /// C++: `double MSEdge::getTravelTimeStatic(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    /// Returns the travel time for the given edge
    // static inline double getTravelTimeStatic(const MSEdge* const edge, const SUMOVehicle* const veh, double time) {
    //         return MSNet::getInstance()->getTravelTime(edge, veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>, time: f64) -> f64 {
        MSNet::get_travel_time(edge, veh, time)
    }

    /// C++: `double MSEdge::getTravelTimeAggregated(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    // double
    // MSEdge::getTravelTimeAggregated(const MSEdge* const edge, const SUMOVehicle* const veh, double /*time*/) {
    //     return edge->getLength() / MIN2(MSRoutingEngine::getAssumedSpeed(edge, veh), veh->getMaxSpeed());
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>, time: f64) -> f64 {
        // MIN2(getAssumedSpeed, veh->getMaxSpeed())
        let assumed = MSRoutingEngine::get_assumed_speed(edge, veh);
        let maxSpeed = veh.get_max_speed();
        edge.get_length() / (if assumed < maxSpeed { assumed } else { maxSpeed })
    }

    /// C++: `double MSEdge::getRoutingSpeed() const` [const]
    /// Returns the averaged speed used by the routing device
    // double
    // MSEdge::getRoutingSpeed() const {
    //     return MSRoutingEngine::getAssumedSpeed(this, nullptr);
    // }
    #[allow(unused_variables)]
    pub fn get_routing_speed(&self) -> f64 {
        MSRoutingEngine::get_assumed_speed(Ptr::new(self), Ptr::null())
    }

    /// C++: `bool MSEdge::insertVehicle(SUMOVehicle &, SUMOTime, const bool, const bool) const` [const]
    /// Tries to insert the given vehicle into the network
    // bool
    // MSEdge::insertVehicle(SUMOVehicle& v, SUMOTime time, const bool checkOnly, const bool forceCheck) const {
    //     // when vaporizing, no vehicles are inserted, but checking needs to be successful to trigger removal
    //     if (isVaporizing() || isTazConnector()
    //             || v.getRouteValidity(true, checkOnly) != MSBaseVehicle::ROUTE_VALID) {
    //         return checkOnly;
    //     }
    //     const SUMOVehicleParameter& pars = v.getParameter();
    //     if (!validateDepartSpeed(v)) {
    //         if (MSGlobals::gCheckRoutes) {
    //             throw ProcessError(TLF("Departure speed for vehicle '%' is too high for the departure edge '%', time=%.",
    //                                    pars.id, getID(), time2string(time)));
    //         } else {
    //             WRITE_WARNINGF(TL("Departure speed for vehicle '%' is too high for the departure edge '%', time=%."),
    //                            pars.id, getID(), time2string(time));
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         if (!forceCheck && myLastFailedInsertionTime == time) {
    //             return false;
    //         }
    //         double pos = 0.0;
    //         switch (pars.departPosProcedure) {
    //             case DepartPosDefinition::GIVEN:
    //                 if (pars.departPos >= 0.) {
    //                     pos = pars.departPos;
    //                 } else {
    //                     pos = pars.departPos + getLength();
    //                 }
    //                 if (pos < 0 || pos > getLength()) {
    //                     WRITE_WARNINGF(TL("Invalid departPos % given for vehicle '%', time=%. Inserting at lane end instead."),
    //                                    pos, v.getID(), time2string(time));
    //                     pos = getLength();
    //                 }
    //                 break;
    //             case DepartPosDefinition::RANDOM:
    //             case DepartPosDefinition::RANDOM_FREE:
    //                 pos = RandHelper::rand(getLength());
    //                 break;
    //             default:
    //                 break;
    //         }
    //         bool result = false;
    //         MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this, pos);
    //         MEVehicle* veh = static_cast<MEVehicle*>(&v);
    //         int qIdx;
    //         if (pars.departPosProcedure == DepartPosDefinition::FREE) {
    //             while (segment != nullptr && !result) {
    //                 if (checkOnly) {
    //                     result = segment->hasSpaceFor(veh, time, qIdx, true) == time;
    //                 } else {
    //                     result = segment->initialise(veh, time);
    //                 }
    //                 segment = segment->getNextSegment();
    //             }
    //         } else {
    //             if (checkOnly) {
    //                 result = segment->hasSpaceFor(veh, time, qIdx, true) == time;
    //             } else {
    //                 result = segment->initialise(veh, time);
    //             }
    //         }
    //         return result;
    //     }
    //     if (checkOnly) {
    //         DepartLaneDefinition dld = v.getParameter().departLaneProcedure;
    //         if (dld == DepartLaneDefinition::DEFAULT) {
    //             dld = myDefaultDepartLaneDefinition;
    //         }
    //         switch (dld) {
    //             case DepartLaneDefinition::GIVEN:
    //             case DepartLaneDefinition::DEFAULT:
    //             case DepartLaneDefinition::FIRST_ALLOWED: {
    //                 MSLane* insertionLane = getDepartLane(static_cast<MSVehicle&>(v));
    //                 if (insertionLane == nullptr) {
    //                     WRITE_WARNINGF(TL("Could not insert vehicle '%' on any lane of edge '%', time=%."),
    //                                    v.getID(), getID(), time2string(time));
    //                     return false;
    //                 }
    //                 const double occupancy = insertionLane->getBruttoOccupancy();
    //                 return (occupancy == 0 || occupancy * myLength + v.getVehicleType().getLengthWithGap() <= myLength ||
    //                         v.getParameter().departProcedure == DepartDefinition::SPLIT);
    //             }
    //             default:
    //                 for (std::vector<MSLane*>::const_iterator i = myLanes->begin(); i != myLanes->end(); ++i) {
    //                     const double occupancy = (*i)->getBruttoOccupancy();
    //                     if (occupancy == 0 || occupancy * myLength + v.getVehicleType().getLengthWithGap() <= myLength ||
    //                             v.getParameter().departProcedure == DepartDefinition::SPLIT) {
    //                         return true;
    //                     }
    //                 }
    //         }
    //         return false;
    //     }
    //     MSLane* insertionLane = getDepartLane(static_cast<MSVehicle&>(v));
    //     if (insertionLane == nullptr) {
    //         return false;
    //     }
    //
    //     if (!forceCheck) {
    //         if (myLastFailedInsertionTime == time) {
    //             if (myFailedInsertionMemory.count(insertionLane->getIndex())) {
    //                 // A vehicle was already rejected for the proposed insertionLane in this timestep
    //                 return false;
    //             }
    //         } else {
    //             // last rejection occurred in a previous timestep, clear cache
    //             myFailedInsertionMemory.clear();
    //         }
    //     }
    //
    //     bool success = insertionLane->insertVehicle(static_cast<MSVehicle&>(v));
    //
    //     if (!success) {
    //         // constraints may enforce explicit re-ordering so we need to try other vehicles after failure
    //         if (!insertionLane->hasParameter("insertionOrder" + v.getID())) {
    //             myFailedInsertionMemory.insert(insertionLane->getIndex());
    //         }
    //     }
    //     return success;
    // }
    #[allow(unused_variables)]
    pub fn insert_vehicle(&self, v: &mut SUMOVehicle, time: SUMOTime, checkOnly: bool, forceCheck: bool) -> bool {
        // when vaporizing, no vehicles are inserted, but checking needs to be successful to trigger removal
        let vp: Ptr<SUMOVehicle> = Ptr::from_mut(v);
        if self.is_vaporizing() || self.is_taz_connector()
            || vp.m().get_route_validity(true, checkOnly, Ptr::null()) != crate::microsim::ms_base_vehicle::MSBaseVehicle_RouteValidity::ROUTE_VALID as i32 {
            return checkOnly;
        }
        let base: Ptr<MSBaseVehicle> = vp.cast();
        let pars = base.m().get_parameter();
        if !self.validate_depart_speed(vp.m()) {
            if MSGlobals::check_routes() {
                panic!("{}", utils::common::util_exceptions::ProcessError::Process(utils::tlf!("Departure speed for vehicle '%' is too high for the departure edge '%', time=%.", pars.id.clone(), self.get_id().to_string(), utils::common::time::time2string(time))));
            } else {
                utils::write_warningf!("Departure speed for vehicle '%' is too high for the departure edge '%', time=%.", pars.id.as_str(), self.get_id(), utils::common::time::time2string(time).as_str());
            }
        }
        if MSGlobals::use_meso_sim() {
            if !forceCheck && self.myLastFailedInsertionTime.get() == time {
                return false;
            }
            let mut pos: f64 = 0.0;
            match pars.departPosProcedure {
                DepartPosDefinition::GIVEN => {
                    if pars.departPos >= 0. {
                        pos = pars.departPos;
                    } else {
                        pos = pars.departPos + self.get_length();
                    }
                    if pos < 0. || pos > self.get_length() {
                        utils::write_warningf!("Invalid departPos % given for vehicle '%', time=%. Inserting at lane end instead.", pos, pars.id.as_str(), utils::common::time::time2string(time).as_str());
                        pos = self.get_length();
                    }
                }
                DepartPosDefinition::RANDOM | DepartPosDefinition::RANDOM_FREE => {
                    pos = utils::common::rand_helper::with_default(|r| r.rand_max(self.get_length()));
                }
                _ => {}
            }
            let mut result = false;
            let mut segment = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(self, pos);
            let veh = vp.cast::<MEVehicle>();
            let mut qIdx: i32 = 0;
            if pars.departPosProcedure == DepartPosDefinition::FREE {
                while !segment.is_null() && !result {
                    if checkOnly {
                        result = segment.m().has_space_for(veh, time, &mut qIdx, true) == time;
                    } else {
                        result = segment.m().initialise(veh, time);
                    }
                    segment = segment.m().get_next_segment();
                }
            } else {
                if checkOnly {
                    result = segment.m().has_space_for(veh, time, &mut qIdx, true) == time;
                } else {
                    result = segment.m().initialise(veh, time);
                }
            }
            return result;
        }
        if checkOnly {
            let mut dld = pars.departLaneProcedure;
            if dld == DepartLaneDefinition::DEFAULT {
                dld = *Self::get_default_depart_lane_definition();
            }
            match dld {
                DepartLaneDefinition::GIVEN | DepartLaneDefinition::DEFAULT | DepartLaneDefinition::FIRST_ALLOWED => {
                    let insertionLane = self.get_depart_lane(vp.cast::<MSVehicle>().m());
                    if insertionLane.is_null() {
                        utils::write_warningf!("Could not insert vehicle '%' on any lane of edge '%', time=%.", pars.id.as_str(), self.get_id(), utils::common::time::time2string(time).as_str());
                        return false;
                    }
                    let occupancy = insertionLane.get_brutto_occupancy();
                    let vlen = base.m().get_vehicle_type().get_length_with_gap();
                    return occupancy == 0. || occupancy * self.myLength + vlen <= self.myLength || pars.departProcedure == DepartDefinition::SPLIT;
                }
                _ => {
                    let vlen = base.m().get_vehicle_type().get_length_with_gap();
                    for lane in self.myLanes.iter() {
                        let occupancy = lane.get_brutto_occupancy();
                        if occupancy == 0. || occupancy * self.myLength + vlen <= self.myLength || pars.departProcedure == DepartDefinition::SPLIT {
                            return true;
                        }
                    }
                }
            }
            return false;
        }
        let insertionLane = self.get_depart_lane(vp.cast::<MSVehicle>().m());
        if insertionLane.is_null() {
            return false;
        }
        // determinism: the failed-insertion memory is a set of lane indices, so iteration order does not matter
        let me: Ptr<MSEdge> = Ptr::new(self);
        if !forceCheck {
            if self.myLastFailedInsertionTime.get() == time {
                if self.myFailedInsertionMemory.contains(&insertionLane.get_index()) {
                    // A vehicle was already rejected for the proposed insertionLane in this timestep
                    return false;
                }
            } else {
                // last rejection occurred in a previous timestep, clear cache
                me.m().myFailedInsertionMemory.clear();
            }
        }
        let success = insertionLane.m().insert_vehicle(vp.cast::<MSVehicle>().m());
        if !success {
            // constraints may enforce explicit re-ordering so we need to try other vehicles after failure
            if !insertionLane.base2.has_parameter(&format!("insertionOrder{}", vp.m().get_id())) {
                me.m().myFailedInsertionMemory.insert(insertionLane.get_index());
            }
        }
        success
    }

    /// C++: `bool MSEdge::validateDepartSpeed(SUMOVehicle &) const` [const]
    /// check whether the given departSpeed is valid for this edge
    // bool
    // MSEdge::validateDepartSpeed(SUMOVehicle& v) const {
    //     const SUMOVehicleParameter& pars = v.getParameter();
    //     const MSVehicleType& type = v.getVehicleType();
    //     if (pars.departSpeedProcedure == DepartSpeedDefinition::GIVEN) {
    //         // departSpeed could have been rounded down in the output
    //         double vMax = getVehicleMaxSpeed(&v) + SPEED_EPS;
    //         if (pars.departSpeed > vMax) {
    //             // check departLane (getVehicleMaxSpeed checks lane 0)
    //             MSLane* departLane = MSGlobals::gMesoNet ? getDepartLaneMeso(v) : getDepartLane(dynamic_cast<MSVehicle&>(v));
    //             if (departLane != nullptr) {
    //                 vMax = departLane->getVehicleMaxSpeed(&v);
    //                 if (pars.wasSet(VEHPARS_SPEEDFACTOR_SET)) {
    //                     // speedFactor could have been rounded down in the output
    //                     vMax *= (1 + SPEED_EPS);
    //                 }
    //                 // additive term must come after multiplication!
    //                 vMax += SPEED_EPS;
    //                 if (pars.departSpeed > vMax) {
    //                     if (type.getSpeedFactor().getParameter(1) > 0.) {
    //                         v.setChosenSpeedFactor(type.computeChosenSpeedDeviation(pars.speedFactor, nullptr, pars.departSpeed / MIN2(getSpeedLimit(), type.getDesiredMaxSpeed() - SPEED_EPS)));
    //                         if (v.getChosenSpeedFactor() > type.getSpeedFactor().getParameter(0) + 2 * type.getSpeedFactor().getParameter(1)) {
    //                             // only warn for significant deviation
    //                             WRITE_WARNINGF(TL("Choosing new speed factor % for vehicle '%' to match departure speed % (max %)."),
    //                                            toString(v.getChosenSpeedFactor()), pars.id, pars.departSpeed, vMax);
    //                         }
    //                     } else {
    //                         return false;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn validate_depart_speed(&self, v: &mut SUMOVehicle) -> bool {
        let vp: Ptr<SUMOVehicle> = Ptr::from_mut(v);
        let base: Ptr<MSBaseVehicle> = vp.cast();
        let pars = base.m().get_parameter();
        if pars.departSpeedProcedure == DepartSpeedDefinition::GIVEN {
            // departSpeed could have been rounded down in the output
            let mut vMax = self.get_vehicle_max_speed(vp.cast::<SUMOTrafficObject>()) + crate::microsim::ms_base_vehicle::SPEED_EPS;
            if pars.departSpeed > vMax {
                // check departLane (getVehicleMaxSpeed checks lane 0)
                let departLane = if !crate::microsim::ms_globals::meso_net().is_null() {
                    self.get_depart_lane_meso(vp.m())
                } else {
                    self.get_depart_lane(vp.cast::<MSVehicle>().m())
                };
                if !departLane.is_null() {
                    vMax = departLane.get_vehicle_max_speed(vp.cast::<SUMOTrafficObject>());
                    if pars.was_set(crate::microsim::ms_base_vehicle::VEHPARS_SPEEDFACTOR_SET) {
                        // speedFactor could have been rounded down in the output
                        vMax *= 1. + crate::microsim::ms_base_vehicle::SPEED_EPS;
                    }
                    // additive term must come after multiplication!
                    vMax += crate::microsim::ms_base_vehicle::SPEED_EPS;
                    if pars.departSpeed > vMax {
                        // shortcut: MSVehicleType::getSpeedFactor() is Unmapped, so the speed-factor deviation (parameter 1 > 0) cannot be read; the no-deviation branch is taken and the departure is rejected. Upgrade when getSpeedFactor() returns Distribution_Parameterized.
                        return false;
                    }
                }
            }
        }
        true
    }

    /// C++: `MSLane * MSEdge::getFreeLane(const std::vector<MSLane *> *, const SUMOVehicleClass, double) const` [const]
    /// Finds the emptiest lane allowing the vehicle class
    // MSLane*
    // MSEdge::getFreeLane(const std::vector<MSLane*>* allowed, const SUMOVehicleClass vclass, double departPos) const {
    //     if (allowed == nullptr) {
    //         allowed = allowedLanes(vclass);
    //     }
    //     MSLane* res = nullptr;
    //     if (allowed != nullptr) {
    //         double largestGap = 0;
    //         MSLane* resByGap = nullptr;
    //         double leastOccupancy = std::numeric_limits<double>::max();
    //         for (std::vector<MSLane*>::const_iterator i = allowed->begin(); i != allowed->end(); ++i) {
    //             const double occupancy = (*i)->getBruttoOccupancy();
    //             if (occupancy < leastOccupancy) {
    //                 res = (*i);
    //                 leastOccupancy = occupancy;
    //             }
    //             const MSVehicle* last = (*i)->getLastFullVehicle();
    //             const double lastGap = (last != nullptr ? last->getPositionOnLane() : myLength) - departPos;
    //             if (lastGap > largestGap) {
    //                 largestGap = lastGap;
    //                 resByGap = (*i);
    //             }
    //         }
    //         if (resByGap != nullptr) {
    //             //if (res != resByGap) std::cout << SIMTIME << " edge=" << getID() << " departPos=" << departPos << " res=" << Named::getIDSecure(res) << " resByGap=" << Named::getIDSecure(resByGap) << " largestGap=" << largestGap << "\n";
    //             res = resByGap;
    //         }
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn get_free_lane(&self, allowed: Ptr<Vec<Ptr<MSLane>>>, vclass: SUMOVehicleClass, departPos: f64) -> Ptr<MSLane> {
        let allowed = if allowed.is_null() { self.allowed_lanes_2(vclass) } else { allowed };
        let mut res: Ptr<MSLane> = Ptr::null();
        if let Some(list) = allowed.get() {
            let mut largestGap = 0.;
            let mut resByGap: Ptr<MSLane> = Ptr::null();
            let mut leastOccupancy = f64::MAX;
            for lane in list.iter() {
                let occupancy = lane.get_brutto_occupancy();
                if occupancy < leastOccupancy {
                    res = *lane;
                    leastOccupancy = occupancy;
                }
                let last = lane.get_last_full_vehicle();
                let lastGap = (if !last.is_null() { last.get_position_on_lane() } else { self.myLength }) - departPos;
                if lastGap > largestGap {
                    largestGap = lastGap;
                    resByGap = *lane;
                }
            }
            if !resByGap.is_null() {
                res = resByGap;
            }
        }
        res
    }

    /// C++: `MSLane * MSEdge::getProbableLane(const std::vector<MSLane *> *, const SUMOVehicleClass, double, double) const` [const]
    /// Finds the most probable lane allowing the vehicle class
    // MSLane*
    // MSEdge::getProbableLane(const std::vector<MSLane*>* allowed, const SUMOVehicleClass vclass, double departPos, double maxSpeed) const {
    //     if (allowed == nullptr) {
    //         allowed = allowedLanes(vclass);
    //     }
    //     MSLane* res = nullptr;
    //     if (allowed != nullptr) {
    //         double largestGap = 0;
    //         double largestSpeed = 0;
    //         MSLane* resByGap = nullptr;
    //         double leastOccupancy = std::numeric_limits<double>::max();
    //         int aIndex = 0;
    //         for (std::vector<MSLane*>::const_iterator i = allowed->begin(); i != allowed->end(); ++i, aIndex++) {
    //             const double occupancy = (*i)->getBruttoOccupancy();
    //             if (occupancy < leastOccupancy) {
    //                 res = (*i);
    //                 leastOccupancy = occupancy;
    //             }
    //             const MSVehicle* last = (*i)->getLastFullVehicle();
    //             double lastGap = (last != nullptr ? last->getPositionOnLane() : myLength) - departPos;
    //             // never insert to the left of a vehicle with a larger speedFactor
    //             if (lastGap > largestGap && maxSpeed >= largestSpeed) {
    //                 largestGap = lastGap;
    //                 resByGap = (*i);
    //             }
    //             if (last != nullptr) {
    //                 largestSpeed = MAX2(largestSpeed, getVehicleMaxSpeed(last));
    //             }
    //         }
    //         if (resByGap != nullptr) {
    //             //if (res != resByGap) std::cout << SIMTIME << " edge=" << getID() << " departPos=" << departPos << " res=" << Named::getIDSecure(res) << " resByGap=" << Named::getIDSecure(resByGap) << " largestGap=" << largestGap << "\n";
    //             res = resByGap;
    //         }
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn get_probable_lane(&self, allowed: Ptr<Vec<Ptr<MSLane>>>, vclass: SUMOVehicleClass, departPos: f64, maxSpeed: f64) -> Ptr<MSLane> {
        let allowed = if allowed.is_null() { self.allowed_lanes_2(vclass) } else { allowed };
        let mut res: Ptr<MSLane> = Ptr::null();
        if let Some(list) = allowed.get() {
            let mut largestGap = 0.;
            let mut largestSpeed = 0.;
            let mut resByGap: Ptr<MSLane> = Ptr::null();
            let mut leastOccupancy = f64::MAX;
            for lane in list.iter() {
                let occupancy = lane.get_brutto_occupancy();
                if occupancy < leastOccupancy {
                    res = *lane;
                    leastOccupancy = occupancy;
                }
                let last = lane.get_last_full_vehicle();
                let lastGap = (if !last.is_null() { last.get_position_on_lane() } else { self.myLength }) - departPos;
                // never insert to the left of a vehicle with a larger speedFactor
                if lastGap > largestGap && maxSpeed >= largestSpeed {
                    largestGap = lastGap;
                    resByGap = *lane;
                }
                if !last.is_null() {
                    let v = self.get_vehicle_max_speed(Ptr::from_mut(&mut last.m().base.base.base));
                    largestSpeed = if largestSpeed > v { largestSpeed } else { v }; // MAX2
                }
            }
            if !resByGap.is_null() {
                res = resByGap;
            }
        }
        res
    }

    /// C++: `MSLane * MSEdge::getDepartLane(MSVehicle &) const` [const]
    /// Finds a depart lane for the given vehicle parameters
    // MSLane*
    // MSEdge::getDepartLane(MSVehicle& veh) const {
    //     DepartLaneDefinition dld = veh.getParameter().departLaneProcedure;
    //     int departLane = veh.getParameter().departLane;
    //     if (dld == DepartLaneDefinition::DEFAULT) {
    //         dld = myDefaultDepartLaneDefinition;
    //         departLane = myDefaultDepartLane;
    //     }
    //     switch (dld) {
    //         case DepartLaneDefinition::GIVEN:
    //             if ((int) myLanes->size() <= departLane || !(*myLanes)[departLane]->allowsVehicleClass(veh.getVClass())) {
    //                 return nullptr;
    //             }
    //             return (*myLanes)[departLane];
    //         case DepartLaneDefinition::RANDOM:
    //             return RandHelper::getRandomFrom(*allowedLanes(veh.getVehicleType().getVehicleClass()));
    //         case DepartLaneDefinition::FREE:
    //             return getFreeLane(nullptr, veh.getVehicleType().getVehicleClass(), getDepartPosBound(veh, false));
    //         case DepartLaneDefinition::ALLOWED_FREE:
    //             if (veh.getRoute().size() == 1) {
    //                 return getFreeLane(nullptr, veh.getVehicleType().getVehicleClass(), getDepartPosBound(veh, false));
    //             } else {
    //                 return getFreeLane(allowedLanes(**(veh.getRoute().begin() + 1), veh.getVehicleType().getVehicleClass()), veh.getVehicleType().getVehicleClass(), getDepartPosBound(veh, false));
    //             }
    //         case DepartLaneDefinition::BEST_FREE:
    //         case DepartLaneDefinition::BEST_PROB: {
    //             veh.updateBestLanes(false, myLanes->front());
    //             const std::vector<MSVehicle::LaneQ>& bl = veh.getBestLanes();
    //             double bestLength = -1;
    //             for (std::vector<MSVehicle::LaneQ>::const_iterator i = bl.begin(); i != bl.end(); ++i) {
    //                 if ((*i).length > bestLength) {
    //                     bestLength = (*i).length;
    //                 }
    //             }
    //             // beyond a certain length, all lanes are suitable
    //             // however, we still need to check departPos to avoid unsuitable insertion
    //             // (this is only possible in some cases)
    //             double departPos = 0;
    //             if (bestLength > BEST_LANE_LOOKAHEAD) {
    //                 departPos = getDepartPosBound(veh);
    //                 bestLength = MIN2(bestLength - departPos, BEST_LANE_LOOKAHEAD);
    //             }
    //             std::vector<MSLane*>* bestLanes = new std::vector<MSLane*>();
    //             for (std::vector<MSVehicle::LaneQ>::const_iterator i = bl.begin(); i != bl.end(); ++i) {
    //                 if (((*i).length - departPos) >= bestLength) {
    //                     if (isInternal()) {
    //                         for (MSLane* lane : *myLanes) {
    //                             if (lane->getNormalSuccessorLane() == (*i).lane && lane->allowsVehicleClass(veh.getVClass())) {
    //                                 bestLanes->push_back(lane);
    //                             }
    //                         }
    //                     } else if ((*i).lane->allowsVehicleClass(veh.getVClass())) {
    //                         bestLanes->push_back((*i).lane);
    //                     }
    //                 }
    //             }
    //             MSLane* ret = nullptr;
    //             if (veh.getParameter().departLaneProcedure == DepartLaneDefinition::BEST_FREE) {
    //                 ret = getFreeLane(bestLanes, veh.getVehicleType().getVehicleClass(), getDepartPosBound(veh, false));
    //             } else {
    //                 ret = getProbableLane(bestLanes, veh.getVehicleType().getVehicleClass(), getDepartPosBound(veh, false), getVehicleMaxSpeed(&veh));
    //             }
    //             delete bestLanes;
    //             return ret;
    //         }
    //         case DepartLaneDefinition::DEFAULT:
    //         case DepartLaneDefinition::FIRST_ALLOWED:
    //             return getFirstAllowed(veh.getVehicleType().getVehicleClass());
    //         default:
    //             break;
    //     }
    //     if (!(*myLanes)[0]->allowsVehicleClass(veh.getVehicleType().getVehicleClass())) {
    //         return nullptr;
    //     }
    //     return (*myLanes)[0];
    // }
    #[allow(unused_variables)]
    pub fn get_depart_lane(&self, veh: &mut MSVehicle) -> Ptr<MSLane> {
        let mut dld = veh.get_parameter().departLaneProcedure;
        let mut departLane = veh.get_parameter().departLane;
        if dld == DepartLaneDefinition::DEFAULT {
            dld = *Self::get_default_depart_lane_definition();
            departLane = *Self::get_default_depart_lane();
        }
        let vclass = veh.get_vehicle_type().get_vehicle_class();
        match dld {
            DepartLaneDefinition::GIVEN => {
                if (self.myLanes.len() as i32) <= departLane || !self.myLanes[departLane as usize].allows_vehicle_class(veh.get_v_class()) {
                    return Ptr::null();
                }
                return self.myLanes[departLane as usize];
            }
            DepartLaneDefinition::RANDOM => {
                let allowed = self.allowed_lanes_2(vclass);
                return utils::common::rand_helper::with_default(|r| *r.get_random_from(&allowed.m()[..]));
            }
            DepartLaneDefinition::FREE => {
                return self.get_free_lane(Ptr::null(), vclass, self.get_depart_pos_bound(veh, false));
            }
            DepartLaneDefinition::ALLOWED_FREE => {
                let route = veh.get_route();
                if route.size() == 1 {
                    return self.get_free_lane(Ptr::null(), vclass, self.get_depart_pos_bound(veh, false));
                } else {
                    let allowed = self.allowed_lanes(&*route.get_edges()[1], vclass, false);
                    return self.get_free_lane(allowed, vclass, self.get_depart_pos_bound(veh, false));
                }
            }
            DepartLaneDefinition::BEST_FREE | DepartLaneDefinition::BEST_PROB => {
                veh.update_best_lanes(false, self.myLanes[0]);
                let mut bestLength: f64 = -1.;
                for lq in veh.get_best_lanes().iter() {
                    if lq.length > bestLength {
                        bestLength = lq.length;
                    }
                }
                // beyond a certain length, all lanes are suitable
                // however, we still need to check departPos to avoid unsuitable insertion
                // (this is only possible in some cases)
                let mut departPos: f64 = 0.;
                if bestLength > BEST_LANE_LOOKAHEAD {
                    departPos = self.get_depart_pos_bound(veh, true);
                    let t = bestLength - departPos;
                    bestLength = if t < BEST_LANE_LOOKAHEAD { t } else { BEST_LANE_LOOKAHEAD };
                }
                let mut bestLanes: Vec<Ptr<MSLane>> = Vec::new();
                for lq in veh.get_best_lanes().iter() {
                    if (lq.length - departPos) >= bestLength {
                        if self.is_internal() {
                            for lane in self.myLanes.iter() {
                                if lane.get_normal_successor_lane().as_raw() == lq.lane.as_raw() && lane.allows_vehicle_class(veh.get_v_class()) {
                                    bestLanes.push(*lane);
                                }
                            }
                        } else if lq.lane.allows_vehicle_class(veh.get_v_class()) {
                            bestLanes.push(lq.lane);
                        }
                    }
                }
                let ret = if veh.get_parameter().departLaneProcedure == DepartLaneDefinition::BEST_FREE {
                    self.get_free_lane(Ptr::new(&bestLanes), vclass, self.get_depart_pos_bound(veh, false))
                } else {
                    let vmax = self.get_vehicle_max_speed(Ptr::from_mut(veh).cast::<SUMOTrafficObject>());
                    self.get_probable_lane(Ptr::new(&bestLanes), vclass, self.get_depart_pos_bound(veh, false), vmax)
                };
                return ret;
            }
            DepartLaneDefinition::DEFAULT | DepartLaneDefinition::FIRST_ALLOWED => {
                return self.get_first_allowed(vclass, false, 0);
            }
            _ => {}
        }
        if !self.myLanes[0].allows_vehicle_class(vclass) {
            return Ptr::null();
        }
        self.myLanes[0]
    }

    /// C++: `MSLane * MSEdge::getFirstAllowed(SUMOVehicleClass, bool, int) const` [const]
    // MSLane*
    // MSEdge::getFirstAllowed(SUMOVehicleClass vClass, bool defaultFirst, int routingMode) const {
    //     for (std::vector<MSLane*>::const_iterator i = myLanes->begin(); i != myLanes->end(); ++i) {
    //         if ((*i)->allowsVehicleClass(vClass, routingMode)) {
    //             return *i;
    //         }
    //     }
    //     return defaultFirst && !myLanes->empty() ? myLanes->front() : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_first_allowed(&self, vClass: SUMOVehicleClass, defaultFirst: bool, routingMode: i32) -> Ptr<MSLane> {
        for lane in self.myLanes.iter() {
            if lane.allows_vehicle_class_2(vClass, routingMode) {
                return *lane;
            }
        }
        if defaultFirst && !self.myLanes.is_empty() {
            self.myLanes[0]
        } else {
            Ptr::null()
        }
    }

    /// C++: `MSLane * MSEdge::getDepartLaneMeso(SUMOVehicle &) const` [const]
    /// consider given departLane parameter (only for validating speeds)
    // MSLane*
    // MSEdge::getDepartLaneMeso(SUMOVehicle& veh) const {
    //     if (veh.getParameter().departLaneProcedure == DepartLaneDefinition::GIVEN) {
    //         if ((int) myLanes->size() <= veh.getParameter().departLane || !(*myLanes)[veh.getParameter().departLane]->allowsVehicleClass(veh.getVehicleType().getVehicleClass())) {
    //             return nullptr;
    //         }
    //         return (*myLanes)[veh.getParameter().departLane];
    //     }
    //     return (*myLanes)[0];
    // }
    #[allow(unused_variables)]
    pub fn get_depart_lane_meso(&self, veh: &mut SUMOVehicle) -> Ptr<MSLane> {
        let base: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast();
        let pars = base.m().get_parameter();
        if pars.departLaneProcedure == DepartLaneDefinition::GIVEN {
            if (self.myLanes.len() as i32) <= pars.departLane || !self.myLanes[pars.departLane as usize].allows_vehicle_class(base.m().get_vehicle_type().get_vehicle_class()) {
                return Ptr::null();
            }
            return self.myLanes[pars.departLane as usize];
        }
        self.myLanes[0]
    }

    /// C++: `SUMOTime MSEdge::getLastFailedInsertionTime() const` [const]
    /// Returns the last time a vehicle could not be inserted
    // inline SUMOTime getLastFailedInsertionTime() const {
    //         return myLastFailedInsertionTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_failed_insertion_time(&self) -> SUMOTime {
        self.myLastFailedInsertionTime.get()
    }

    /// C++: `void MSEdge::setLastFailedInsertionTime(SUMOTime) const` [const]
    /// Sets the last time a vehicle could not be inserted
    // inline void setLastFailedInsertionTime(SUMOTime time) const {
    //         myLastFailedInsertionTime = time;
    //     }
    #[allow(unused_variables)]
    pub fn set_last_failed_insertion_time(&self, time: SUMOTime) {
        // C++ field is mutable; the setter is const
        self.myLastFailedInsertionTime.set(time);
    }

    /// C++: `void MSEdge::changeLanes(SUMOTime) const` [const]
    /// Performs lane changing on this edge
    // void
    // MSEdge::changeLanes(SUMOTime t) const {
    //     if (myLaneChanger != nullptr) {
    //         myLaneChanger->laneChange(t);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_lanes(&self, t: SUMOTime) {
        if !self.myLaneChanger.is_null() {
            self.myLaneChanger.m().lane_change(t);
        }
    }

    /// C++: `const MSEdge * MSEdge::getInternalFollowingEdge(const MSEdge *, SUMOVehicleClass) const` [const]
    // const MSEdge*
    // MSEdge::getInternalFollowingEdge(const MSEdge* followerAfterInternal, SUMOVehicleClass vClass) const {
    //     //@todo to be optimized
    //     for (const MSLane* const l : *myLanes) {
    //         for (const MSLink* const link : l->getLinkCont()) {
    //             if (&link->getLane()->getEdge() == followerAfterInternal) {
    //                 if (link->getViaLane() != nullptr) {
    //                     if (link->getViaLane()->allowsVehicleClass(vClass)) {
    //                         return &link->getViaLane()->getEdge();
    //                     } else {
    //                         continue;
    //                     }
    //                 } else {
    //                     return nullptr; // network without internal links
    //                 }
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_following_edge(&self, followerAfterInternal: Ptr<MSEdge>, vClass: SUMOVehicleClass) -> Ptr<MSEdge> {
        for l in self.myLanes.iter() {
            for link in l.get_link_cont() {
                if Ptr::new(link.get_lane().get_edge()) == followerAfterInternal {
                    if !link.get_via_lane().is_null() {
                        if link.get_via_lane().allows_vehicle_class(vClass) {
                            return Ptr::new(link.get_via_lane().get_edge());
                        } else {
                            continue;
                        }
                    } else {
                        return Ptr::null(); // network without internal links
                    }
                }
            }
        }
        Ptr::null()
    }

    /// C++: `double MSEdge::getInternalFollowingLengthTo(const MSEdge *, SUMOVehicleClass) const` [const]
    /// returns the length of all internal edges on the junction until reaching the non-internal edge followerAfterInternal.
    // double
    // MSEdge::getInternalFollowingLengthTo(const MSEdge* followerAfterInternal, SUMOVehicleClass vClass) const {
    //     assert(followerAfterInternal != 0);
    //     assert(!followerAfterInternal->isInternal());
    //     double dist = 0.;
    //     const MSEdge* edge = getInternalFollowingEdge(followerAfterInternal, vClass);
    //     // Take into account non-internal lengths until next non-internal edge
    //     while (edge != nullptr && edge->isInternal()) {
    //         dist += edge->getLength();
    //         edge = edge->getInternalFollowingEdge(followerAfterInternal, vClass);
    //     }
    //     return dist;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_following_length_to(&self, followerAfterInternal: Ptr<MSEdge>, vClass: SUMOVehicleClass) -> f64 {
        assert!(!followerAfterInternal.is_null());
        assert!(!followerAfterInternal.is_internal());
        let mut dist = 0.;
        let mut edge = self.get_internal_following_edge(followerAfterInternal, vClass);
        // Take into account non-internal lengths until next non-internal edge
        while !edge.is_null() && edge.is_internal() {
            dist += edge.get_length();
            edge = edge.get_internal_following_edge(followerAfterInternal, vClass);
        }
        dist
    }

    /// C++: `const MSEdge * MSEdge::getNormalBefore() const` [const]
    /// if this edge is an internal edge, return its first normal predecessor, otherwise the edge itself
    // const MSEdge*
    // MSEdge::getNormalBefore() const {
    //     const MSEdge* result = this;
    //     while (result->isInternal() && MSGlobals::gUsingInternalLanes) {
    //         assert(result->getPredecessors().size() == 1);
    //         result = result->getPredecessors().front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_before(&self) -> Ptr<MSEdge> {
        let mut result = Ptr::new(self);
        while result.is_internal() && G_USING_INTERNAL_LANES {
            assert!(result.get_predecessors().len() == 1);
            result = result.get_predecessors()[0];
        }
        result
    }

    /// C++: `const MSEdge * MSEdge::getNormalSuccessor() const` [const]
    /// if this edge is an internal edge, return its first normal successor, otherwise the edge itself
    // const MSEdge*
    // MSEdge::getNormalSuccessor() const {
    //     const MSEdge* result = this;
    //     while (result->isInternal()) {
    //         assert(result->getSuccessors().size() == 1);
    //         result = result->getSuccessors().front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_successor(&self) -> Ptr<MSEdge> {
        let mut result = Ptr::new(self);
        while result.is_internal() {
            assert!(result.get_successors(SVC_IGNORING).len() == 1);
            result = result.get_successors(SVC_IGNORING)[0];
        }
        result
    }

    /// C++: `bool MSEdge::prohibits(const SUMOVehicle *const) const` [const]
    /// Returns whether the vehicle (class) is not allowed on the edge
    // inline bool prohibits(const SUMOVehicle* const vehicle) const {
    //         if (vehicle == nullptr) {
    //             return false;
    //         }
    //         const SUMOVehicleClass svc = vehicle->getVClass();
    //         return (vehicle->ignoreTransientPermissions()
    //                 ? (myOriginalCombinedPermissions & svc) != svc
    //                 : (myCombinedPermissions & svc) != svc);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        if vehicle.is_null() {
            return false;
        }
        let svc = vehicle.get_v_class();
        if vehicle.ignore_transient_permissions() {
            (self.myOriginalCombinedPermissions & svc) != svc
        } else {
            (self.myCombinedPermissions & svc) != svc
        }
    }

    /// C++: `bool MSEdge::hasTransientPermissions() const` [const]
    // bool
    // MSEdge::hasTransientPermissions() const {
    //     return myHaveTransientPermissions;
    // }
    #[allow(unused_variables)]
    pub fn has_transient_permissions(&self) -> bool {
        self.myHaveTransientPermissions
    }

    /// C++: `bool MSEdge::restricts(const SUMOVehicle *const) const` [const]
    /// Returns whether this edge has restriction parameters forbidding the given vehicle to pass it The restriction mechanism is not implemented yet for the microsim, so it always returns false.
    // inline bool restricts(const SUMOVehicle* const /* vehicle */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, arg0: Ptr<SUMOVehicle>) -> bool {
        false
    }

    /// C++: `SVCPermissions MSEdge::getPermissions() const` [const]
    /// Returns the combined permissions of all lanes of this edge
    // inline SVCPermissions getPermissions() const {
    //         return myCombinedPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        self.myCombinedPermissions
    }

    /// C++: `double MSEdge::getWidth() const` [const]
    /// Returns the edges's width (sum over all lanes)
    // double getWidth() const {
    //         return myWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        self.myWidth
    }

    /// C++: `const std::vector<double> MSEdge::getSubLaneSides() const` [const]
    /// Returns the right side offsets of this edge's sublanes
    // const std::vector<double> getSubLaneSides() const {
    //         return mySublaneSides;
    //     }
    #[allow(unused_variables)]
    pub fn get_sub_lane_sides(&self) -> Vec<f64> {
        self.mySublaneSides.clone()
    }

    /// C++: `void MSEdge::rebuildAllowedLanes(const bool, bool)`
    // void
    // MSEdge::rebuildAllowedLanes(const bool onInit, bool updateVehicles) {
    //     // hadPermissionChanges() only tracks transient (rerouter/TraCI-temporary)
    //     // changes; a permanent change (e.g. traci.lane.setDisallowed /
    //     // traci.edge.setAllowed) goes straight through MSLane::setPermissions and
    //     // never registers there. Comparing the aggregated permissions before and
    //     // after this rebuild catches that case too.
    //     const SVCPermissions oldMinimumPermissions = myMinimumPermissions;
    //     const SVCPermissions oldCombinedPermissions = myCombinedPermissions;
    //     // rebuild myMinimumPermissions and myCombinedPermissions
    //     myMinimumPermissions = SVCAll;
    //     myCombinedPermissions = 0;
    //     bool lanesChangedPermission = false;
    //     for (MSLane* const lane : *myLanes) {
    //         // same dedicated lanes are ignored in meso to avoid capacity errors.
    //         // Here we have to make sure that vehicles which are set to depart on
    //         // such lanes trigger an error.
    //         SVCPermissions allow = getMesoPermissions(lane->getPermissions(), SVC_PEDESTRIAN);
    //         myMinimumPermissions &= allow;
    //         myCombinedPermissions |= allow;
    //         lanesChangedPermission |= lane->hadPermissionChanges();
    //     }
    //     if (!onInit && !myHaveTransientPermissions && lanesChangedPermission) {
    //         myHaveTransientPermissions = true;
    //         // backup original structures when first needed
    //         myOrigAllowed = myAllowed;
    //         myOrigAllowedTargets = myAllowedTargets;
    //         myOrigClassesViaSuccessorMap = myClassesViaSuccessorMap;
    //     }
    //     if (!onInit && (lanesChangedPermission
    //                     || myMinimumPermissions != oldMinimumPermissions
    //                     || myCombinedPermissions != oldCombinedPermissions)) {
    //         // a runtime permission change (closure / re-opening, transient or
    //         // permanent) must reach the CCH metric even though the speed table
    //         // does not move
    //         MSRoutingEngine::invalidateCCHEdge(this);
    //     }
    //     // rebuild myAllowed
    //     myAllowed.clear();
    //     if (myCombinedPermissions != myMinimumPermissions) {
    //         myAllowed.push_back(std::make_pair(SVC_IGNORING, myLanes));
    //         for (SVCPermissions vclass = SVC_PRIVATE; vclass <= SUMOVehicleClass_MAX; vclass *= 2) {
    //             if ((myCombinedPermissions & vclass) == vclass) {
    //                 std::shared_ptr<std::vector<MSLane*> > allowedLanes = std::make_shared<std::vector<MSLane*> >();
    //                 for (MSLane* const lane : *myLanes) {
    //                     if (lane->allowsVehicleClass((SUMOVehicleClass)vclass)) {
    //                         allowedLanes->push_back(lane);
    //                     }
    //                 }
    //                 addToAllowed(vclass, allowedLanes, myAllowed);
    //             }
    //         }
    //     }
    //     if (onInit) {
    //         myOriginalMinimumPermissions = myMinimumPermissions;
    //         myOriginalCombinedPermissions = myCombinedPermissions;
    //     } else {
    //         rebuildAllowedTargets(updateVehicles);
    //         for (MSEdge* pred : myPredecessors) {
    //             if (myHaveTransientPermissions && !pred->myHaveTransientPermissions) {
    //                 pred->myOrigAllowed = pred->myAllowed;
    //                 pred->myOrigAllowedTargets = pred->myAllowedTargets;
    //                 pred->myOrigClassesViaSuccessorMap = pred->myClassesViaSuccessorMap;
    //                 pred->myHaveTransientPermissions = true;
    //             }
    //             pred->rebuildAllowedTargets(updateVehicles);
    //         }
    //         if (MSGlobals::gUseMesoSim) {
    //             for (MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*this); s != nullptr; s = s->getNextSegment()) {
    //                 s->updatePermissions();
    //             }
    //         }
    //         for (MSLane* const lane : *myLanes) {
    //             for (MSLink* link : lane->getLinkCont()) {
    //                 link->updatePermissions();
    //             }
    //             for (auto ili : lane->getIncomingLanes()) {
    //                 ili.viaLink->updatePermissions();
    //             }
    //         }
    //         // permissions of links entering this edge also depend on its new permissions,
    //         // so they must be refreshed as well (important for connections via internal lanes)
    //         for (MSEdge* const pred : myPredecessors) {
    //             for (MSLane* const lane : *pred->myLanes) {
    //                 for (MSLink* link : lane->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == this) {
    //                         link->updatePermissions();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn rebuild_allowed_lanes(&mut self, onInit: bool, updateVehicles: bool) {
        let me = Ptr::from_mut(self);
        let oldMinimumPermissions = self.myMinimumPermissions;
        let oldCombinedPermissions = self.myCombinedPermissions;
        // rebuild myMinimumPermissions and myCombinedPermissions
        self.myMinimumPermissions = SVC_ALL;
        self.myCombinedPermissions = 0;
        let mut lanesChangedPermission = false;
        let laneList = self.myLanes.clone();
        for lane in laneList.iter() {
            // same dedicated lanes are ignored in meso to avoid capacity errors.
            let allow = MSEdge::get_meso_permissions(lane.get_permissions(), SVC_PEDESTRIAN);
            self.myMinimumPermissions &= allow;
            self.myCombinedPermissions |= allow;
            lanesChangedPermission |= lane.had_permission_changes();
        }
        if !onInit && !self.myHaveTransientPermissions && lanesChangedPermission {
            self.myHaveTransientPermissions = true;
            // backup original structures when first needed
            self.myOrigAllowed = self.myAllowed.clone();
            self.myOrigAllowedTargets = self.myAllowedTargets.clone();
            self.myOrigClassesViaSuccessorMap = self.myClassesViaSuccessorMap.clone();
        }
        if !onInit && (lanesChangedPermission || self.myMinimumPermissions != oldMinimumPermissions || self.myCombinedPermissions != oldCombinedPermissions) {
            // a runtime permission change must reach the CCH metric even though the speed table does not move
            MSRoutingEngine::invalidate_cch_edge(me);
        }
        // rebuild myAllowed
        let mut allowed: MSEdge_AllowedLanesCont = Vec::new();
        if self.myCombinedPermissions != self.myMinimumPermissions {
            allowed.push((SVC_IGNORING, Rc::clone(&self.myLanes)));
            let mut vclass = SVC_PRIVATE;
            while vclass <= SUMO_VEHICLE_CLASS_MAX {
                if (self.myCombinedPermissions & vclass) == vclass {
                    let mut allowedLanes: Vec<Ptr<MSLane>> = Vec::new();
                    for lane in laneList.iter() {
                        if lane.allows_vehicle_class(vclass) {
                            allowedLanes.push(*lane);
                        }
                    }
                    self.add_to_allowed(vclass, Rc::new(allowedLanes), &mut allowed);
                }
                vclass *= 2;
            }
        }
        self.myAllowed = allowed;
        if onInit {
            self.myOriginalMinimumPermissions = self.myMinimumPermissions;
            self.myOriginalCombinedPermissions = self.myCombinedPermissions;
        } else {
            self.rebuild_allowed_targets(updateVehicles);
            let preds = self.myPredecessors.clone();
            for pred in preds.iter() {
                let p = pred.m();
                if self.myHaveTransientPermissions && !p.myHaveTransientPermissions {
                    p.myOrigAllowed = p.myAllowed.clone();
                    p.myOrigAllowedTargets = p.myAllowedTargets.clone();
                    p.myOrigClassesViaSuccessorMap = p.myClassesViaSuccessorMap.clone();
                    p.myHaveTransientPermissions = true;
                }
                p.rebuild_allowed_targets(updateVehicles);
            }
            // shortcut: meso segment updatePermissions (MSGlobals::gMesoNet) not ported, unreachable while G_USE_MESO_SIM is false
            for lane in laneList.iter() {
                for link in lane.get_link_cont() {
                    link.m().update_permissions();
                }
                for ili in lane.get_incoming_lanes() {
                    ili.viaLink.m().update_permissions();
                }
            }
            // permissions of links entering this edge also depend on its new permissions
            for pred in preds.iter() {
                for lane in pred.myLanes.iter() {
                    for link in lane.get_link_cont() {
                        if Ptr::new(link.get_lane().get_edge()) == me {
                            link.m().update_permissions();
                        }
                    }
                }
            }
        }
    }

    /// C++: `void MSEdge::rebuildAllowedTargets(const bool)`
    // void
    // MSEdge::rebuildAllowedTargets(const bool updateVehicles) {
    //     myAllowedTargets.clear();
    //     for (const MSEdge* target : mySuccessors) {
    //         bool universalMap = true; // whether the mapping for SVC_IGNORING is also valid for all vehicle classes
    //         std::shared_ptr<std::vector<MSLane*> > allLanes = std::make_shared<std::vector<MSLane*> >();
    //         // compute the mapping for SVC_IGNORING
    //         for (MSLane* const lane : *myLanes) {
    //             SVCPermissions combinedTargetPermissions = 0;
    //             for (const MSLink* const link : lane->getLinkCont()) {
    //                 if (&link->getLane()->getEdge() == target) {
    //                     allLanes->push_back(lane);
    //                     combinedTargetPermissions |= link->getLane()->getPermissions();
    //                     if (link->getViaLane() != nullptr &&
    //                             ((lane->getPermissions() & link->getLane()->getPermissions()) != link->getViaLane()->getPermissions())) {
    //                         // custom connection permissions
    //                         universalMap = false;
    //                     }
    //                 }
    //             }
    //             if (combinedTargetPermissions == 0 || (lane->getPermissions() & combinedTargetPermissions) != lane->getPermissions()) {
    //                 universalMap = false;
    //             }
    //         }
    //         if (universalMap) {
    //             if (myAllowed.empty()) {
    //                 // we have no lane specific permissions
    //                 myAllowedTargets[target].push_back(std::make_pair(myMinimumPermissions, myLanes));
    //             } else {
    //                 for (const auto& i : myAllowed) {
    //                     addToAllowed(i.first, i.second, myAllowedTargets[target]);
    //                 }
    //             }
    //         } else {
    //             addToAllowed(SVC_IGNORING, allLanes, myAllowedTargets[target]);
    //             // compute the vclass specific mapping
    //             for (SVCPermissions vclass = SVC_PRIVATE; vclass <= SUMOVehicleClass_MAX; vclass *= 2) {
    //                 if ((myCombinedPermissions & vclass) == vclass) {
    //                     std::shared_ptr<std::vector<MSLane*> > allowedLanes = std::make_shared<std::vector<MSLane*> >();
    //                     for (MSLane* const lane : *myLanes) {
    //                         if (lane->allowsVehicleClass((SUMOVehicleClass)vclass)) {
    //                             for (const MSLink* const link : lane->getLinkCont()) {
    //                                 if (link->getLane()->allowsVehicleClass((SUMOVehicleClass)vclass) && &link->getLane()->getEdge() == target && (link->getViaLane() == nullptr || link->getViaLane()->allowsVehicleClass((SUMOVehicleClass)vclass))) {
    //                                     allowedLanes->push_back(lane);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                     addToAllowed(vclass, allowedLanes, myAllowedTargets[target]);
    //                 }
    //             }
    //         }
    //     }
    //     if (updateVehicles) {
    //         for (const MSLane* const lane : *myLanes) {
    //             const MSLane::VehCont& vehs = lane->getVehiclesSecure();
    //             for (MSVehicle* veh : vehs) {
    //                 veh->updateBestLanes(true);
    //             }
    //             lane->releaseVehicles();
    //         }
    //     }
    //     myClassesSuccessorMap.clear();
    //     myClassesViaSuccessorMap.clear();
    // }
    #[allow(unused_variables)]
    pub fn rebuild_allowed_targets(&mut self, updateVehicles: bool) {
        self.myAllowedTargets.clear();
        let succs = self.mySuccessors.clone();
        let laneList = self.myLanes.clone();
        for target in succs.iter() {
            let target = *target;
            let mut universalMap = true; // whether the mapping for SVC_IGNORING is also valid for all vehicle classes
            let mut allLanes: Vec<Ptr<MSLane>> = Vec::new();
            let mut cont: MSEdge_AllowedLanesCont = Vec::new();
            // compute the mapping for SVC_IGNORING
            for lane in laneList.iter() {
                let mut combinedTargetPermissions: SVCPermissions = 0;
                for link in lane.get_link_cont() {
                    if Ptr::new(link.get_lane().get_edge()) == target {
                        allLanes.push(*lane);
                        combinedTargetPermissions |= link.get_lane().get_permissions();
                        if !link.get_via_lane().is_null() && ((lane.get_permissions() & link.get_lane().get_permissions()) != link.get_via_lane().get_permissions()) {
                            // custom connection permissions
                            universalMap = false;
                        }
                    }
                }
                if combinedTargetPermissions == 0 || (lane.get_permissions() & combinedTargetPermissions) != lane.get_permissions() {
                    universalMap = false;
                }
            }
            if universalMap {
                if self.myAllowed.is_empty() {
                    // we have no lane specific permissions
                    cont.push((self.myMinimumPermissions, Rc::clone(&self.myLanes)));
                } else {
                    for (perm, lanes) in self.myAllowed.iter() {
                        self.add_to_allowed(*perm, Rc::clone(lanes), &mut cont);
                    }
                }
            } else {
                self.add_to_allowed(SVC_IGNORING, Rc::new(allLanes), &mut cont);
                // compute the vclass specific mapping
                let mut vclass = SVC_PRIVATE;
                while vclass <= SUMO_VEHICLE_CLASS_MAX {
                    if (self.myCombinedPermissions & vclass) == vclass {
                        let mut allowedLanes: Vec<Ptr<MSLane>> = Vec::new();
                        for lane in laneList.iter() {
                            if lane.allows_vehicle_class(vclass) {
                                for link in lane.get_link_cont() {
                                    if link.get_lane().allows_vehicle_class(vclass)
                                        && Ptr::new(link.get_lane().get_edge()) == target
                                        && (link.get_via_lane().is_null() || link.get_via_lane().allows_vehicle_class(vclass))
                                    {
                                        allowedLanes.push(*lane);
                                    }
                                }
                            }
                        }
                        self.add_to_allowed(vclass, Rc::new(allowedLanes), &mut cont);
                    }
                    vclass *= 2;
                }
            }
            self.myAllowedTargets.insert(target, cont);
        }
        if updateVehicles {
            for lane in laneList.iter() {
                for veh in lane.get_vehicles_secure().iter() {
                    veh.m().update_best_lanes(true, Ptr::null());
                }
                lane.release_vehicles();
            }
        }
        self.myClassesSuccessorMap.clear();
        self.myClassesViaSuccessorMap.clear();
    }

    /// C++: `double MSEdge::getDistanceTo(const MSEdge *, const bool) const` [const]
    /// optimistic air distance heuristic for use in routing
    // double
    // MSEdge::getDistanceTo(const MSEdge* other, const bool doBoundaryEstimate) const {
    //     assert(this != other);
    //     if (doBoundaryEstimate) {
    //         return myBoundary.distanceTo2D(other->myBoundary);
    //     }
    //     if (isTazConnector()) {
    //         if (other->isTazConnector()) {
    //             return myBoundary.distanceTo2D(other->myBoundary);
    //         }
    //         return myBoundary.distanceTo2D(other->getLanes()[0]->getShape()[0]);
    //     }
    //     if (other->isTazConnector()) {
    //         return other->myBoundary.distanceTo2D(getLanes()[0]->getShape()[-1]);
    //     }
    //     return getLanes()[0]->getShape()[-1].distanceTo2D(other->getLanes()[0]->getShape()[0]);
    // }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<MSEdge>, doBoundaryEstimate: bool) -> f64 {
        assert!(Ptr::new(self) != other);
        if doBoundaryEstimate {
            return self.myBoundary.distance_to2d_boundary(&other.myBoundary);
        }
        if self.is_taz_connector() {
            if other.is_taz_connector() {
                return self.myBoundary.distance_to2d_boundary(&other.myBoundary);
            }
            return self.myBoundary.distance_to2d(other.get_lanes()[0].get_shape_2().at(0));
        }
        if other.is_taz_connector() {
            return other.myBoundary.distance_to2d(self.get_lanes()[0].get_shape_2().at(-1));
        }
        self.get_lanes()[0].get_shape_2().at(-1).distance_to2d(other.get_lanes()[0].get_shape_2().at(0))
    }

    /// C++: `const Position MSEdge::getStopPosition(const SUMOVehicleParameter::Stop &)` [static]
    /// return the coordinates of the center of the given stop
    // const Position
    // MSEdge::getStopPosition(const SUMOVehicleParameter::Stop& stop) {
    //     return MSLane::dictionary(stop.lane)->geometryPositionAtOffset((stop.endPos + stop.startPos) / 2.);
    // }
    #[allow(unused_variables)]
    pub fn get_stop_position(stop: &SUMOVehicleParameter_Stop) -> Position {
        MSLane::dictionary_2(&stop.lane).geometry_position_at_offset((stop.endPos + stop.startPos) / 2., 0.)
    }

    /// C++: `double MSEdge::getLength() const` [const]
    /// return the length of the edge
    // inline double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `double MSEdge::getMinLength() const` [const]
    /// return the minimum length of all lanes (only differs from getLength for curved internal lanes)
    // double
    // MSEdge::getMinLength() const {
    //     double result = std::numeric_limits<double>::max();
    //     for (const MSLane* l : *myLanes) {
    //         result = MIN2(result, l->getLength());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_min_length(&self) -> f64 {
        let mut result = f64::MAX;
        for l in self.myLanes.iter() {
            // MIN2(result, l->getLength())
            let len = l.get_length();
            result = if result < len { result } else { len };
        }
        result
    }

    /// C++: `double MSEdge::getSpeedLimit() const` [const]
    /// Returns the speed limit of the edge The speed limit of the first lane is returned; should probably be the fastest lane
    // double
    // MSEdge::getSpeedLimit() const {
    //     // @note lanes might have different maximum speeds in theory
    //     return myLanes->empty() ? 1 : getLanes()[0]->getSpeedLimit();
    // }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        if self.myLanes.is_empty() { 1. } else { self.myLanes[0].get_speed_limit() }
    }

    /// C++: `double MSEdge::getSpeedLimit(SUMOVehicleClass) const` [const]
    /// Returns the speed limit of the edge The speed limit of the first lane is returned; should probably be the fastest lane
    // double
    // MSEdge::getSpeedLimit(SUMOVehicleClass svc) const {
    //     // @note lanes might have different maximum speeds in theory
    //     return myLanes->empty() ? 1 : getLanes()[0]->getSpeedLimit(svc);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_limit_2(&self, svc: SUMOVehicleClass) -> f64 {
        if self.myLanes.is_empty() { 1. } else { self.myLanes[0].get_speed_limit_2(svc) }
    }

    /// C++: `double MSEdge::getLengthGeometryFactor() const` [const]
    /// return shape.length() / myLength
    // double
    // MSEdge::getLengthGeometryFactor() const {
    //     return myLanes->empty() ? 1 : getLanes()[0]->getLengthGeometryFactor();
    // }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        if self.myLanes.is_empty() { 1. } else { self.myLanes[0].get_length_geometry_factor_2() }
    }

    /// C++: `void MSEdge::setMaxSpeed(const double, const bool, const double)`
    /// Sets a new maximum speed for all lanes (used by TraCI and MSCalibrator)
    // void
    // MSEdge::setMaxSpeed(const double val, const bool modified, const double jamThreshold) {
    //     assert(val >= 0);
    //     if (myLanes != nullptr) {
    //         for (MSLane* const lane : *myLanes) {
    //             lane->setMaxSpeed(val, modified, jamThreshold);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(&mut self, val: f64, modified: bool, jamThreshold: f64) {
        assert!(val >= 0.);
        let laneList = self.myLanes.clone();
        for lane in laneList.iter() {
            lane.m().set_max_speed(val, modified, jamThreshold);
        }
    }

    /// C++: `void MSEdge::setFrictionCoefficient(double) const` [const]
    /// Sets a new friction coefficient COF for all lanes [*later to be (used by TraCI and MSCalibrator)*]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_friction_coefficient(&self, val: f64) {
        for lane in self.myLanes.iter() {
            lane.m().set_friction_coefficient(val);
        }
    }

    /// C++: `double MSEdge::getVehicleMaxSpeed(const SUMOTrafficObject *const) const` [const]
    /// Returns the maximum speed the vehicle may use on this edge
    // double
    // MSEdge::getVehicleMaxSpeed(const SUMOTrafficObject* const veh) const {
    //     // @note lanes might have different maximum speeds in theory
    //     return myLanes->empty() ? 1 : getLanes()[0]->getVehicleMaxSpeed(veh);
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_max_speed(&self, veh: Ptr<SUMOTrafficObject>) -> f64 {
        if self.myLanes.is_empty() { 1. } else { self.myLanes[0].get_vehicle_max_speed(veh) }
    }

    /// C++: `void MSEdge::addTransportable(MSTransportable *) const` [const, virtual]
    // void
    // MSEdge::addTransportable(MSTransportable* t) const {
    //     if (t->isPerson()) {
    //         myPersons.insert(t);
    //     } else {
    //         myContainers.insert(t);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_transportable(&self, t: Ptr<MSTransportable>) {
        if t.is_person() {
            Ptr::new(self).m().myPersons.insert(t);
        } else {
            Ptr::new(self).m().myContainers.insert(t);
        }
    }

    /// C++: `void MSEdge::removeTransportable(MSTransportable *) const` [const, virtual]
    // void
    // MSEdge::removeTransportable(MSTransportable* t) const {
    //     std::set<MSTransportable*, ComparatorNumericalIdLess>& tc = t->isPerson() ? myPersons : myContainers;
    //     auto it = tc.find(t);
    //     if (it != tc.end()) {
    //         tc.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable(&self, t: Ptr<MSTransportable>) {
        let meP = Ptr::new(self); let me = meP.m();
        let tc = if t.is_person() { &mut me.myPersons } else { &mut me.myContainers };
        tc.remove(&t);
    }

    /// C++: `bool MSEdge::isRoundabout() const` [const]
    // inline bool isRoundabout() const {
    //         return myAmRoundabout;
    //     }
    #[allow(unused_variables)]
    pub fn is_roundabout(&self) -> bool {
        self.myAmRoundabout
    }

    /// C++: `void MSEdge::markAsRoundabout()`
    // void markAsRoundabout() {
    //         myAmRoundabout = true;
    //     }
    #[allow(unused_variables)]
    pub fn mark_as_roundabout(&mut self) {
        self.myAmRoundabout = true;
    }

    /// C++: `void MSEdge::markDelayed() const` [const]
    // void markDelayed() const {
    //         myAmDelayed = true;
    //     }
    #[allow(unused_variables)]
    pub fn mark_delayed(&self) {
        // C++ field is mutable; the marker is const
        self.myAmDelayed.set(true);
    }

    /// C++: `bool MSEdge::isDelayed() const` [const]
    // inline bool isDelayed() const {
    //         return myAmDelayed || (myBidiEdge != nullptr && myBidiEdge->myAmDelayed);
    //     }
    #[allow(unused_variables)]
    pub fn is_delayed(&self) -> bool {
        self.myAmDelayed.get() || (!self.myBidiEdge.is_null() && self.myBidiEdge.myAmDelayed.get())
    }

    /// C++: `bool MSEdge::hasLaneChanger() const` [const]
    // bool hasLaneChanger() const {
    //         return myLaneChanger != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_lane_changer(&self) -> bool {
        !self.myLaneChanger.is_null()
    }

    /// C++: `std::pair<double, SUMOTime> MSEdge::getLastBlocked(int) const` [const]
    /// retrieve properties of a blocked vehicle that wants to chane to the lane with the given index
    // std::pair<double, SUMOTime>
    // MSEdge::getLastBlocked(int index) const {
    //     if (myLaneChanger != nullptr) {
    //         return myLaneChanger->getLastBlocked(index);
    //     }
    //     return std::make_pair(-1, -1);
    // }
    #[allow(unused_variables)]
    pub fn get_last_blocked(&self, index: i32) -> (f64, SUMOTime) {
        if !self.myLaneChanger.is_null() {
            return self.myLaneChanger.get_last_blocked(index);
        }
        (-1., -1)
    }

    /// C++: `bool MSEdge::canChangeToOpposite() const` [const]
    /// whether this edge allows changing to the opposite direction edge
    // bool
    // MSEdge::canChangeToOpposite() const {
    //     return (!myLanes->empty() && myLanes->back()->getOpposite() != nullptr &&
    //             // do not change on curved internal lanes
    //             (!isInternal()
    //              || (MSGlobals::gUsingInternalLanes
    //                  && myLanes->back()->getIncomingLanes()[0].viaLink->getDirection() == LinkDirection::STRAIGHT)));
    // }
    #[allow(unused_variables)]
    pub fn can_change_to_opposite(&self) -> bool {
        !self.myLanes.is_empty()
            && !self.myLanes.last().unwrap().get_opposite().is_null()
            // do not change on curved internal lanes
            && (!self.is_internal()
                || (G_USING_INTERNAL_LANES
                    && self.myLanes.last().unwrap().get_incoming_lanes()[0].viaLink.get_direction() == LinkDirection::STRAIGHT))
    }

    /// C++: `const MSEdge * MSEdge::getOppositeEdge() const` [const]
    /// Returns the opposite direction edge if on exists else a nullptr
    // const MSEdge*
    // MSEdge::getOppositeEdge() const {
    //     if (!myLanes->empty() && myLanes->back()->getOpposite() != nullptr) {
    //         return &(myLanes->back()->getOpposite()->getEdge());
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_edge(&self) -> Ptr<MSEdge> {
        if !self.myLanes.is_empty() && !self.myLanes.last().unwrap().get_opposite().is_null() {
            Ptr::new(self.myLanes.last().unwrap().get_opposite().get_edge())
        } else {
            Ptr::null()
        }
    }

    /// C++: `double MSEdge::getMeanSpeed() const` [const]
    /// get the mean speed
    // double
    // MSEdge::getMeanSpeed() const {
    //     double v = 0;
    //     double totalNumVehs = 0;
    //     if (MSGlobals::gUseMesoSim) {
    //         for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this); segment != nullptr; segment = segment->getNextSegment()) {
    //             const int numVehs = segment->getCarNumber();
    //             if (numVehs > 0) {
    //                 v += numVehs * segment->getMeanSpeed();
    //                 totalNumVehs += numVehs;
    //             }
    //         }
    //         if (totalNumVehs == 0) {
    //             return getLength() / myEmptyTraveltime; // may include tls-penalty
    //         }
    //     } else {
    //         for (const MSLane* const lane : *myLanes) {
    //             int numVehs = lane->getVehicleNumber();
    //             if (numVehs == 0) {
    //                 // take speed limit but with lowest possible weight
    //                 numVehs = 1;
    //             }
    //             v += numVehs * lane->getMeanSpeed();
    //             totalNumVehs += numVehs;
    //         }
    //         if (myBidiEdge != nullptr) {
    //             for (const MSLane* const lane : myBidiEdge->getLanes()) {
    //                 if (lane->getVehicleNumber() > 0) {
    //                     // do not route across edges which are already occupied in reverse direction
    //                     return 0;
    //                 }
    //             }
    //         }
    //         if (totalNumVehs == 0) {
    //             return getSpeedLimit();
    //         }
    //     }
    //     return v / totalNumVehs;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed(&self) -> f64 {
        // shortcut: meso branch (MESegment::getMeanSpeed) not ported, unreachable while G_USE_MESO_SIM is false
        let mut v = 0.;
        let mut totalNumVehs = 0.;
        for lane in self.myLanes.iter() {
            let mut numVehs = lane.get_vehicle_number();
            if numVehs == 0 {
                // take speed limit but with lowest possible weight
                numVehs = 1;
            }
            v += numVehs as f64 * lane.get_mean_speed();
            totalNumVehs += numVehs as f64;
        }
        if !self.myBidiEdge.is_null() {
            for lane in self.myBidiEdge.get_lanes().iter() {
                if lane.get_vehicle_number() > 0 {
                    // do not route across edges which are already occupied in reverse direction
                    return 0.;
                }
            }
        }
        if totalNumVehs == 0. {
            return self.get_speed_limit();
        }
        v / totalNumVehs
    }

    /// C++: `double MSEdge::getMeanFriction() const` [const]
    /// get the mean friction over the lanes
    // double
    // MSEdge::getMeanFriction() const {
    //     double f = 0.;
    //     for (const MSLane* const lane : *myLanes) {
    //         f += lane->getFrictionCoefficient();
    //     }
    //     if (!myLanes->empty()) {
    //         return f / (double)myLanes->size();
    //     }
    //     return 1.;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_friction(&self) -> f64 {
        let mut f = 0.;
        for lane in self.myLanes.iter() {
            f += lane.get_friction_coefficient();
        }
        if !self.myLanes.is_empty() {
            return f / (self.myLanes.len() as f64);
        }
        1.
    }

    /// C++: `double MSEdge::getMeanSpeedBike() const` [const]
    /// get the mean speed of all bicycles on this edge
    // double
    // MSEdge::getMeanSpeedBike() const {
    //     if (MSGlobals::gUseMesoSim) {
    //         // no separate bicycle speeds in meso
    //         return getMeanSpeed();
    //     }
    //     double v = 0;
    //     double totalNumVehs = 0;
    //     for (const MSLane* const lane : *myLanes) {
    //         const int numVehs = lane->getVehicleNumber();
    //         v += numVehs * lane->getMeanSpeedBike();
    //         totalNumVehs += numVehs;
    //     }
    //     if (totalNumVehs == 0) {
    //         return getSpeedLimit();
    //     }
    //     return v / totalNumVehs;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_bike(&self) -> f64 {
        let mut v = 0.;
        let mut totalNumVehs = 0.;
        for lane in self.myLanes.iter() {
            let numVehs = lane.get_vehicle_number();
            v += numVehs as f64 * lane.get_mean_speed_bike();
            totalNumVehs += numVehs as f64;
        }
        if totalNumVehs == 0. {
            return self.get_speed_limit();
        }
        v / totalNumVehs
    }

    /// C++: `bool MSEdge::hasMinorLink() const` [const]
    /// whether any lane has a minor link
    // bool
    // MSEdge::hasMinorLink() const {
    //     for (const MSLane* const l : *myLanes) {
    //         for (const MSLink* const link : l->getLinkCont()) {
    //             if (!link->havePriority()) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_minor_link(&self) -> bool {
        for l in self.myLanes.iter() {
            for link in l.get_link_cont() {
                if !link.have_priority() {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool MSEdge::isFringe() const` [const]
    /// return whether this edge is at the fringe of the network
    // bool isFringe() const {
    //         return myAmFringe;
    //     }
    #[allow(unused_variables)]
    pub fn is_fringe(&self) -> bool {
        self.myAmFringe
    }

    /// C++: `bool MSEdge::hasChangeProhibitions(SUMOVehicleClass, int) const` [const]
    /// return whether this edge prohibits changing for the given vClass when starting on the given lane index
    // bool
    // MSEdge::hasChangeProhibitions(SUMOVehicleClass svc, int index) const {
    //     if (myLanes->size() == 1) {
    //         return false;
    //     }
    //     for (const MSLane* const l : *myLanes) {
    //         if (l->getIndex() <= index && !l->allowsChangingRight(svc) && l->getIndex() > 0) {
    //             return true;
    //         } else if (l->getIndex() >= index && !l->allowsChangingLeft(svc) && l->getIndex() < (int)(myLanes->size() - 1)) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_change_prohibitions(&self, svc: SUMOVehicleClass, index: i32) -> bool {
        if self.myLanes.len() == 1 {
            return false;
        }
        for l in self.myLanes.iter() {
            if l.get_index() <= index && !l.allows_changing_right(svc) && l.get_index() > 0 {
                return true;
            } else if l.get_index() >= index && !l.allows_changing_left(svc) && l.get_index() < (self.myLanes.len() as i32 - 1) {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSEdge::isSelected() const` [const, virtual]
    /// whether this lane is selected in the GUI
    // virtual bool isSelected() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        false
    }

    /// C++: `void MSEdge::lock() const` [const, virtual]
    /// grant exclusive access to the mesoscopic state
    // virtual void lock() const {}
    #[allow(unused_variables)]
    pub fn lock(&self) {
        // C++ virtual no-op (mesoscopic locking is not used in the microsim)
    }

    /// C++: `void MSEdge::unlock() const` [const, virtual]
    /// release exclusive access to the mesoscopic state
    // virtual void unlock() const {}
    #[allow(unused_variables)]
    pub fn unlock(&self) {
        // C++ virtual no-op (mesoscopic locking is not used in the microsim)
    }

    /// C++: `void MSEdge::addWaiting(SUMOVehicle *) const` [const]
    /// Adds a vehicle to the list of waiting vehicles
    // void
    // MSEdge::addWaiting(SUMOVehicle* vehicle) const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myWaitingMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     myWaiting.push_back(vehicle);
    // }
    #[allow(unused_variables)]
    pub fn add_waiting(&self, vehicle: Ptr<SUMOVehicle>) {
        Ptr::new(self).m().myWaiting.push(vehicle);
    }

    /// C++: `void MSEdge::removeWaiting(const SUMOVehicle *) const` [const]
    /// Removes a vehicle from the list of waiting vehicles
    // void
    // MSEdge::removeWaiting(const SUMOVehicle* vehicle) const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myWaitingMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     std::vector<SUMOVehicle*>::iterator it = std::find(myWaiting.begin(), myWaiting.end(), vehicle);
    //     if (it != myWaiting.end()) {
    //         myWaiting.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_waiting(&self, vehicle: Ptr<SUMOVehicle>) {
        let meP = Ptr::new(self); let me = meP.m();
        if let Some(i) = me.myWaiting.iter().position(|v| *v == vehicle) {
            me.myWaiting.remove(i);
        }
    }

    /// C++: `SUMOVehicle * MSEdge::getWaitingVehicle(MSTransportable *, const double) const` [const]
    // SUMOVehicle*
    // MSEdge::getWaitingVehicle(MSTransportable* transportable, const double position) const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myWaitingMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     for (SUMOVehicle* const vehicle : myWaiting) {
    //         if (transportable->isWaitingFor(vehicle)) {
    //             if (vehicle->isStoppedInRange(position, MSGlobals::gStopTolerance) ||
    //                     (!vehicle->hasDeparted() &&
    //                      (vehicle->getParameter().departProcedure == DepartDefinition::TRIGGERED ||
    //                       vehicle->getParameter().departProcedure == DepartDefinition::CONTAINER_TRIGGERED))) {
    //                 return vehicle;
    //             }
    //             if (!vehicle->isLineStop(position) && vehicle->allowsBoarding(transportable)) {
    //                 WRITE_WARNING((transportable->isPerson() ? "Person '" : "Container '")
    //                               + transportable->getID() + "' at edge '" + getID() + "' position " + toString(position) + " cannot use waiting vehicle '"
    //                               + vehicle->getID() + "' at position " + toString(vehicle->getPositionOnLane()) + " because it is too far away.");
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_vehicle(&self, transportable: Ptr<MSTransportable>, position: f64) -> Ptr<SUMOVehicle> {
        for vehicle in self.myWaiting.iter() {
            let vehicle = *vehicle;
            if transportable.is_waiting_for(vehicle) {
                if vehicle.is_stopped_in_range(position, G_STOP_TOLERANCE, true)
                    || (!vehicle.has_departed()
                        && (vehicle.get_parameter().departProcedure == DepartDefinition::TRIGGERED
                            || vehicle.get_parameter().departProcedure == DepartDefinition::CONTAINER_TRIGGERED))
                {
                    return vehicle;
                }
                if !vehicle.is_line_stop(position) && vehicle.allows_boarding(transportable) {
                    // shortcut: C++ toString(double) uses the global output precision; Rust default formatting is used here
                    utils::common::msg_handler::write_warning(&format!(
                        "{} '{}' at edge '{}' position {} cannot use waiting vehicle '{}' at position {} because it is too far away.",
                        if transportable.is_person() { "Person" } else { "Container" },
                        transportable.get_id(),
                        self.get_id(),
                        position,
                        vehicle.get_id(),
                        vehicle.get_position_on_lane()
                    ));
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void MSEdge::clearState()`
    /// Remove all transportables before quick-loading state
    // void
    // MSEdge::clearState() {
    //     myPersons.clear();
    //     myContainers.clear();
    //     myWaiting.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myPersons.clear();
        self.myContainers.clear();
        self.myWaiting.clear();
    }

    /// C++: `void MSEdge::postLoadInitLaneChanger()`
    // void
    // MSEdge::postLoadInitLaneChanger() {
    //     if (myLaneChanger != nullptr) {
    //         myLaneChanger->postloadInitLC();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn post_load_init_lane_changer(&mut self) {
        if !self.myLaneChanger.is_null() {
            self.myLaneChanger.m().postload_init_lc();
        }
    }

    /// C++: `DepartLaneDefinition & MSEdge::getDefaultDepartLaneDefinition()` [static]
    // static DepartLaneDefinition& getDefaultDepartLaneDefinition() {
    //         return myDefaultDepartLaneDefinition;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_depart_lane_definition() -> &'static mut DepartLaneDefinition {
        unsafe { &mut *std::ptr::addr_of_mut!(MY_DEFAULT_DEPART_LANE_DEFINITION) }
    }

    /// C++: `int & MSEdge::getDefaultDepartLane()` [static]
    // static int& getDefaultDepartLane() {
    //         return myDefaultDepartLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_depart_lane() -> &'static mut i32 {
        unsafe { &mut *std::ptr::addr_of_mut!(MY_DEFAULT_DEPART_LANE) }
    }

    /// C++: `bool MSEdge::dictionary(const std::string &, MSEdge *)` [static]
    /// Inserts edge into the static dictionary Returns true if the key id isn't already in the dictionary. Otherwise returns false.
    // bool
    // MSEdge::dictionary(const std::string& id, MSEdge* ptr) {
    //     const DictType::iterator it = myDict.lower_bound(id);
    //     if (it == myDict.end() || it->first != id) {
    //         // id not in myDict
    //         const auto it2 = myDict.emplace_hint(it, id, ptr);
    //         myHashDict[it2->first] = ptr;
    //         while (ptr->getNumericalID() >= (int)myEdges.size()) {
    //             myEdges.push_back(nullptr);
    //         }
    //         myEdges[ptr->getNumericalID()] = ptr;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: &str, edge: Ptr<MSEdge>) -> bool {
        let dict = my_dict();
        if !dict.contains_key(id) {
            // id not in myDict
            dict.insert(id.to_string(), edge);
            my_hash_dict().insert(id.to_string(), edge);
            let numericalID = edge.get_numerical_id();
            let edges = my_edges();
            while numericalID >= edges.len() as i32 {
                edges.push(Ptr::null());
            }
            edges[numericalID as usize] = edge;
            return true;
        }
        false
    }

    /// C++: `MSEdge * MSEdge::dictionary(const std::string_view &)` [static]
    /// Returns the MSEdge associated to the key id if it exists, otherwise returns nullptr.
    // MSEdge*
    // MSEdge::dictionary(const std::string_view& id) {
    //     auto it = myHashDict.find(id);
    //     if (it == myHashDict.end()) {
    //         return nullptr;
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: &str) -> Ptr<MSEdge> {
        my_hash_dict().get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `MSEdge * MSEdge::dictionaryHint(const std::string &, const int)` [static]
    /// Returns the MSEdge associated to the key id giving a hint with a numerical id.
    // MSEdge*
    // MSEdge::dictionaryHint(const std::string& id, const int startIdx) {
    //     // this method is mainly useful when parsing connections from the net.xml which are sorted by "from" id
    //     if (myEdges[startIdx] != nullptr && myEdges[startIdx]->getID() == id) {
    //         return myEdges[startIdx];
    //     }
    //     if (startIdx + 1 < (int)myEdges.size() && myEdges[startIdx + 1] != nullptr && myEdges[startIdx + 1]->getID() == id) {
    //         return myEdges[startIdx + 1];
    //     }
    //     return dictionary(id);
    // }
    #[allow(unused_variables)]
    pub fn dictionary_hint(id: &str, startIdx: i32) -> Ptr<MSEdge> {
        // this method is mainly useful when parsing connections from the net.xml which are sorted by "from" id
        let edges = my_edges();
        let idx = startIdx as usize;
        if startIdx >= 0 && idx < edges.len() && !edges[idx].is_null() && edges[idx].get_id() == id {
            return edges[idx];
        }
        if startIdx >= 0 && idx + 1 < edges.len() && !edges[idx + 1].is_null() && edges[idx + 1].get_id() == id {
            return edges[idx + 1];
        }
        MSEdge::dictionary_2(id)
    }

    /// C++: `const MSEdgeVector & MSEdge::getAllEdges()` [static]
    /// Returns all edges with a numerical id
    // const MSEdgeVector&
    // MSEdge::getAllEdges() {
    //     return myEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_all_edges() -> &'static MSEdgeVector {
        my_edges()
    }

    /// C++: `void MSEdge::clear()` [static]
    /// Clears the dictionary
    // void
    // MSEdge::clear() {
    //     // the keys point into myDict so we need to clear this one first
    //     myHashDict.clear();
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    //     myEdges.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear() {
        // shortcut: C++ deletes the edges here; MSEdge object ownership is not modelled, so the owner must drop them
        my_hash_dict().clear();
        my_dict().clear();
        my_edges().clear();
    }

    /// C++: `void MSEdge::insertIDs(std::vector<std::string> &)` [static]
    /// Inserts IDs of all known edges into the given vector
    // void
    // MSEdge::insertIDs(std::vector<std::string>& into) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_i_ds(into: &mut Vec<String>) {
        for key in my_dict().keys() {
            into.push(key.clone());
        }
    }

    /// C++: `SVCPermissions MSEdge::getMesoPermissions(SVCPermissions, SVCPermissions)` [static]
    // SVCPermissions
    // MSEdge::getMesoPermissions(SVCPermissions p, SVCPermissions ignoreIgnored) {
    //     SVCPermissions ignored = myMesoIgnoredVClasses & ~ignoreIgnored;
    //     return (p | ignored) == ignored ? 0 : p;
    // }
    #[allow(unused_variables)]
    pub fn get_meso_permissions(p: SVCPermissions, ignoreIgnored: SVCPermissions) -> SVCPermissions {
        let ignored = my_meso_ignored_v_classes() & !ignoreIgnored;
        if (p | ignored) == ignored { 0 } else { p }
    }

    /// C++: `void MSEdge::setMesoIgnoredVClasses(SVCPermissions)` [static]
    // static void setMesoIgnoredVClasses(SVCPermissions ignored) {
    //         myMesoIgnoredVClasses = ignored;
    //     }
    #[allow(unused_variables)]
    pub fn set_meso_ignored_v_classes(ignored: SVCPermissions) {
        unsafe { *std::ptr::addr_of_mut!(MY_MESO_IGNORED_V_CLASSES) = ignored };
    }

    /// C++: `void MSEdge::parseEdgesList(const std::string &, ConstMSEdgeVector &, const std::string &)` [static]
    /// Parses the given string assuming it contains a list of edge ids divided by spaces
    // void
    // MSEdge::parseEdgesList(const std::string& desc, ConstMSEdgeVector& into,
    //                        const std::string& rid) {
    //     try {
    //         StringTokenizer::tokenizeAndLookup(desc, into);
    //     } catch (const InvalidArgument& e) {
    //         throw ProcessError("The edge '" + std::string(e.what()) + "' within the route " + rid + " is not known."
    //                            + "\n The route can not be build.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edges_list(desc: &str, into: &mut ConstMSEdgeVector, rid: &str) {
        for id in desc.split_whitespace() {
            let edge = MSEdge::dictionary_2(id);
            if edge.is_null() {
                // shortcut: C++ throws ProcessError here; the Rust signature has no Result, so this panics with the same text
                panic!("{}", ProcessError::Process(format!("The edge '{}' within the route {} is not known.\n The route can not be build.", id, rid)));
            }
            into.push(edge);
        }
    }

    /// C++: `void MSEdge::parseEdgesList(const std::vector<std::string> &, ConstMSEdgeVector &, const std::string &)` [static]
    /// Parses the given string vector assuming it edge ids
    // void
    // MSEdge::parseEdgesList(const std::vector<std::string>& desc, ConstMSEdgeVector& into,
    //                        const std::string& rid) {
    //     for (std::vector<std::string>::const_iterator i = desc.begin(); i != desc.end(); ++i) {
    //         const MSEdge* edge = MSEdge::dictionary(*i);
    //         // check whether the edge exists
    //         if (edge == nullptr) {
    //             throw ProcessError("The edge '" + *i + "' within the route " + rid + " is not known."
    //                                + "\n The route can not be build.");
    //         }
    //         into.push_back(edge);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edges_list_2(desc: &Vec<String>, into: &mut ConstMSEdgeVector, rid: &str) {
        for id in desc.iter() {
            let edge = MSEdge::dictionary_2(id);
            // check whether the edge exists
            if edge.is_null() {
                // shortcut: C++ throws ProcessError here; the Rust signature has no Result, so this panics with the same text
                panic!("{}", ProcessError::Process(format!("The edge '{}' within the route {} is not known.\n The route can not be build.", id, rid)));
            }
            into.push(edge);
        }
    }

    /// C++: `ReversedEdge<MSEdge, SUMOVehicle> * MSEdge::getReversedRoutingEdge() const` [const]
    /// @}
    // ReversedEdge<MSEdge, SUMOVehicle>* getReversedRoutingEdge() const {
    //         if (myReversedRoutingEdge == nullptr) {
    //             myReversedRoutingEdge = new ReversedEdge<MSEdge, SUMOVehicle>(this);
    //         }
    //         return myReversedRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_reversed_routing_edge(&self) -> Ptr<ReversedEdge<MSEdge, SUMOVehicle>> {
        let meP = Ptr::new(self); let me = meP.m();
        if me.myReversedRoutingEdge.is_null() {
            me.myReversedRoutingEdge = Ptr::from_raw(Box::into_raw(Box::new(ReversedEdge::new(Ptr::new(self)))));
        }
        me.myReversedRoutingEdge
    }

    /// C++: `RailEdge<MSEdge, SUMOVehicle> * MSEdge::getRailwayRoutingEdge() const` [const]
    // RailEdge<MSEdge, SUMOVehicle>* getRailwayRoutingEdge() const {
    //         if (myRailwayRoutingEdge == nullptr) {
    //             myRailwayRoutingEdge = new RailEdge<MSEdge, SUMOVehicle>(this);
    //         }
    //         return myRailwayRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_railway_routing_edge(&self) -> Ptr<RailEdge<MSEdge, SUMOVehicle>> {
        let meP = Ptr::new(self); let me = meP.m();
        if me.myRailwayRoutingEdge.is_null() {
            me.myRailwayRoutingEdge = Ptr::from_raw(Box::into_raw(Box::new(RailEdge::new(Ptr::new(self)))));
        }
        me.myRailwayRoutingEdge
    }

    /// C++: `const std::map<const MEVehicle *, std::pair<double, int>> & MSEdge::getMesoPositions() const` [const]
    // const std::map<const MEVehicle*, std::pair<double, int> >&
    // MSEdge::getMesoPositions() const {
    //     assert(MSGlobals::gUseMesoSim);
    //     if (myLastCacheUpdate < SIMSTEP) {
    //         myLastCacheUpdate = SIMSTEP;
    //         auto old = std::move(myCachedMesoPos);
    //         myCachedMesoPos.clear();  // moved-from map is valid-but-unspecified; make it defined-empty
    //         int laneIndex = 0;
    //         const double now = SIMTIME;
    //         for (std::vector<MSLane*>::const_iterator msl = myLanes->begin(); msl != myLanes->end(); ++msl, ++laneIndex) {
    //             // go through the vehicles
    //             double segmentOffset = 0; // offset at start of current segment
    //             for (MESegment* segment = MSGlobals::gMesoNet->getSegmentForEdge(*this);
    //                     segment != nullptr; segment = segment->getNextSegment()) {
    //                 const double segLength = segment->getLength();
    //                 const double lanesCovered = segment->numQueues() == 1 ? std::round(segment->getCapacity() / segLength) : 1.;
    //                 if (laneIndex < segment->numQueues()) {
    //                     // make a copy so we don't have to worry about synchronization
    //                     std::vector<MEVehicle*> queue = segment->getQueue(laneIndex);
    //                     const int queueSize = (int)queue.size();
    //                     SUMOTime earliestExitTime = segment->getQueueBlockTime(laneIndex);
    //                     int overlap = 0;
    //                     double prevPos = std::numeric_limits<double>::max();
    //                     for (int i = 0; i < queueSize; ++i) {
    //                         const MEVehicle* const veh = queue[queueSize - i - 1];
    //                         earliestExitTime = MAX2(earliestExitTime, veh->getEventTime());
    //                         const double vehLength = veh->getVehicleType().getLengthWithGap();
    //                         double maxPos = segmentOffset + segLength;
    //                         auto it = old.find(veh);
    //                         const double oldPos = it != old.end() ? it->second.first : 0.;  // store the old position to prevent backwards moving vehicles
    //                         if (i > 0) {
    //                             earliestExitTime += segment->getMinTauWithVehLength(vehLength, veh->getVehicleType().getCarFollowModel().getHeadwayTime());
    //                             maxPos = MIN2(maxPos, prevPos - vehLength / lanesCovered);
    //                         }
    //                         const double entry = veh->getLastEntryTimeSeconds();
    //                         assert(STEPS2TIME(earliestExitTime) > entry);
    //                         const double pos = MAX2(MIN2(segmentOffset + segLength * (now - entry) / (STEPS2TIME(earliestExitTime) - entry), maxPos), oldPos);
    //                         // check if we overlap with the previous vehicle such that the gui has the chance to add some lateral offset
    //                         if (overlap == 0 && prevPos - pos < vehLength) {
    //                             overlap = lanesCovered > 1. ? -3 : -1;
    //                         } else {
    //                             overlap = 0;
    //                         }
    //                         myCachedMesoPos[veh] = std::make_pair(pos, overlap);
    //                         prevPos = pos;
    //                     }
    //                 }
    //                 segmentOffset += segLength;
    //             }
    //         }
    //     }
    //     return myCachedMesoPos;
    // }
    #[allow(unused_variables)]
    pub fn get_meso_positions(&self) -> &BTreeMap<Ptr<MEVehicle>, (f64, i32)> {
        // shortcut: meso segment vehicle positions (MESegment walk) not ported; returns the cache as it stands, which is what C++ returns when no meso segments exist
        &self.myCachedMesoPos
    }

    /// C++: `void MSEdge::invalidateMesoCache() const` [const]
    // inline void invalidateMesoCache() const {
    //         myLastCacheUpdate = -1;
    //     }
    #[allow(unused_variables)]
    pub fn invalidate_meso_cache(&self) {
        // C++ field is mutable; the invalidation is const
        Ptr::new(self).m().myLastCacheUpdate = -1;
    }

    /// C++: `double MSEdge::getDepartPosBound(const MSVehicle &, bool) const` [const, protected]
    /// return upper bound for the depart position on this edge
    // double
    // MSEdge::getDepartPosBound(const MSVehicle& veh, bool upper) const {
    //     const SUMOVehicleParameter& pars = veh.getParameter();
    //     double pos = getLength();
    //     // determine the position
    //     switch (pars.departPosProcedure) {
    //         case DepartPosDefinition::GIVEN:
    //             pos = pars.departPos;
    //             if (pos < 0.) {
    //                 pos += myLength;
    //             }
    //             break;
    //         case DepartPosDefinition::RANDOM:
    //             // could be any position on the edge
    //             break;
    //         case DepartPosDefinition::RANDOM_FREE:
    //             // could be any position on the edge due to multiple random attempts
    //             break;
    //         case DepartPosDefinition::FREE:
    //             // many candidate positions, upper bound could be computed exactly
    //             // with much effort
    //             break;
    //         case DepartPosDefinition::LAST:
    //             if (upper) {
    //                 for (std::vector<MSLane*>::const_iterator i = myLanes->begin(); i != myLanes->end(); ++i) {
    //                     MSVehicle* last = (*i)->getLastFullVehicle();
    //                     if (last != nullptr) {
    //                         pos = MIN2(pos, last->getPositionOnLane());
    //                     }
    //                 }
    //             } else {
    //                 pos = 0;
    //             }
    //             break;
    //         case DepartPosDefinition::BASE:
    //         case DepartPosDefinition::DEFAULT:
    //             if (!upper) {
    //                 pos = 0;
    //             }
    //             break;
    //         default:
    //             pos = MIN2(pos, veh.getVehicleType().getLength());
    //             break;
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_pos_bound(&self, veh: &MSVehicle, upper: bool) -> f64 {
        let pars = veh.get_parameter();
        let mut pos = self.get_length();
        // determine the position
        match pars.departPosProcedure {
            DepartPosDefinition::GIVEN => {
                pos = pars.departPos;
                if pos < 0. {
                    pos += self.myLength;
                }
            }
            DepartPosDefinition::RANDOM | DepartPosDefinition::RANDOM_FREE | DepartPosDefinition::FREE => {
                // could be any position on the edge (no tighter bound is computed)
            }
            DepartPosDefinition::LAST => {
                if upper {
                    for lane in self.myLanes.iter() {
                        let last = lane.get_last_full_vehicle();
                        if !last.is_null() {
                            let p = last.m().get_position_on_lane();
                            pos = if pos < p { pos } else { p };
                        }
                    }
                } else {
                    pos = 0.;
                }
            }
            DepartPosDefinition::BASE | DepartPosDefinition::DEFAULT => {
                if !upper {
                    pos = 0.;
                }
            }
            _ => {
                let len = veh.get_vehicle_type().get_length();
                pos = if pos < len { pos } else { len };
            }
        }
        pos
    }

    /// C++: `void MSEdge::MSEdge(const MSEdge &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSEdge) -> Self {
        // C++ copy constructor is private and invalidated (MSEdge is never copied)
        panic!("MSEdge copy constructor is invalidated in C++")
    }

    /// C++: `MSEdge & MSEdge::operator=(const MSEdge &)` [private]
    /// assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEdge) -> &mut MSEdge {
        // C++ assignment operator is private and invalidated (MSEdge is never assigned)
        panic!("MSEdge assignment operator is invalidated in C++")
    }

    /// C++: `void MSEdge::setBidiLanes()` [private]
    // void
    // MSEdge::setBidiLanes() {
    //     assert(myBidiEdge != nullptr);
    //     if (getNumLanes() == 1 && myBidiEdge->getNumLanes() == 1) {
    //         // the other way round is set when this method runs for the bidiEdge
    //         getLanes()[0]->setBidiLane(myBidiEdge->getLanes()[0]);
    //     } else {
    //         // find lanes with matching reversed shapes
    //         int numBidiLanes = 0;
    //         for (MSLane* l1 : *myLanes) {
    //             for (MSLane* l2 : *myBidiEdge->myLanes) {
    //                 if (l1->getShape().reverse().almostSame(l2->getShape(), POSITION_EPS * 2)) {
    //                     l1->setBidiLane(l2);
    //                     numBidiLanes++;
    //                 }
    //             }
    //         }
    //         // warn only once for each pair
    //         if (numBidiLanes == 0 && getNumericalID() < myBidiEdge->getNumericalID()) {
    //             WRITE_WARNINGF(TL("Edge '%' and bidi edge '%' have no matching bidi lanes"), getID(), myBidiEdge->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_bidi_lanes(&mut self) {
        let bidi = self.myBidiEdge;
        if self.get_num_lanes() == 1 && bidi.get_num_lanes() == 1 {
            // the other way round is set when this method runs for the bidiEdge
            self.myLanes[0].m().set_bidi_lane(bidi.get_lanes()[0]);
        } else {
            // find lanes with matching reversed shapes
            let mut numBidiLanes = 0;
            for l1 in self.myLanes.iter() {
                for l2 in bidi.get_lanes().iter() {
                    if l1.get_shape_2().reverse().almost_same(l2.get_shape_2(), POSITION_EPS * 2.) {
                        l1.m().set_bidi_lane(*l2);
                        numBidiLanes += 1;
                    }
                }
            }
            // warn only once for each pair
            if numBidiLanes == 0 && self.get_numerical_id() < bidi.get_numerical_id() {
                utils::common::msg_handler::write_warning(&format!("Edge '{}' and bidi edge '{}' have no matching bidi lanes", self.get_id(), bidi.get_id()));
            }
        }
    }

    /// C++: `bool MSEdge::isSuperposable(const MSEdge *)` [private]
    // bool
    // MSEdge::isSuperposable(const MSEdge* other) {
    //     if (other == nullptr || other->getLanes().size() != myLanes->size()) {
    //         return false;
    //     }
    //     std::vector<MSLane*>::const_iterator it1 = myLanes->begin();
    //     std::vector<MSLane*>::const_reverse_iterator it2 = other->getLanes().rbegin();
    //     do {
    //         if ((*it1)->getShape().reverse() != (*it2)->getShape()) {
    //             return false;
    //         }
    //         it1++;
    //         it2++;
    //     } while (it1 != myLanes->end());
    //
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_superposable(&mut self, other: Ptr<MSEdge>) -> bool {
        if other.is_null() || other.get_lanes().len() != self.myLanes.len() {
            return false;
        }
        let n = self.myLanes.len();
        let otherLanes = other.get_lanes();
        for i in 0..n {
            // it2 walks the other edge's lanes in reverse order
            if self.myLanes[i].get_shape_2().reverse() != *otherLanes[n - 1 - i].get_shape_2() {
                return false;
            }
        }
        true
    }

    /// C++: `void MSEdge::addToAllowed(const SVCPermissions, std::shared_ptr<const std::vector<MSLane *>>, AllowedLanesCont &) const` [const, private]
    // void
    // MSEdge::addToAllowed(const SVCPermissions permissions, std::shared_ptr<const std::vector<MSLane*> > allowedLanes, AllowedLanesCont& laneCont) const {
    //     if (!allowedLanes->empty()) {
    //         // recheck whether we had this list to save memory
    //         for (auto& allowed : laneCont) {
    //             if (*allowed.second == *allowedLanes) {
    //                 allowed.first |= permissions;
    //                 return;
    //             }
    //         }
    //         laneCont.push_back(std::make_pair(permissions, allowedLanes));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_to_allowed(&self, permissions: SVCPermissions, allowedLanes: Rc<Vec<Ptr<MSLane>>>, laneCont: &mut MSEdge_AllowedLanesCont) {
        if !allowedLanes.is_empty() {
            // recheck whether we had this list to save memory
            for allowed in laneCont.iter_mut() {
                if *allowed.1 == *allowedLanes {
                    allowed.0 |= permissions;
                    return;
                }
            }
            laneCont.push((permissions, allowedLanes));
        }
    }

}

/// Sorts edges by their ids
/// C++ class `MSEdge::by_id_sorter` (microsim/MSEdge.h)
pub struct MSEdge_by_id_sorter {
}

impl MSEdge_by_id_sorter {
    /// C++: `void by_id_sorter::by_id_sorter()`
    /// constructor
    // explicit by_id_sorter() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {}
    }

    /// C++: `int by_id_sorter::operator()(const MSEdge *const, const MSEdge *const) const` [const]
    /// comparing operator
    // int operator()(const MSEdge* const e1, const MSEdge* const e2) const {
    //             return e1->getNumericalID() < e2->getNumericalID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<MSEdge>, e2: Ptr<MSEdge>) -> i32 {
        (e1.get_numerical_id() < e2.get_numerical_id()) as i32
    }

}

/// Sorts transportables by their positions
/// C++ class `MSEdge::transportable_by_position_sorter` (microsim/MSEdge.h)
pub struct MSEdge_transportable_by_position_sorter {
    pub myTime: SUMOTime,
}

impl MSEdge_transportable_by_position_sorter {
    /// C++: `void transportable_by_position_sorter::transportable_by_position_sorter(SUMOTime)`
    /// constructor
    // explicit transportable_by_position_sorter(SUMOTime timestep): myTime(timestep) { }
    #[allow(unused_variables)]
    pub fn new(timestep: SUMOTime) -> Self {
        Self { myTime: timestep }
    }

    /// C++: `int transportable_by_position_sorter::operator()(const MSTransportable *const, const MSTransportable *const) const` [const]
    /// comparing operator
    // int
    // MSEdge::transportable_by_position_sorter::operator()(const MSTransportable* const c1, const MSTransportable* const c2) const {
    //     const double pos1 = c1->getCurrentStage()->getEdgePos(myTime);
    //     const double pos2 = c2->getCurrentStage()->getEdgePos(myTime);
    //     if (pos1 != pos2) {
    //         return pos1 < pos2;
    //     }
    //     return c1->getID() < c2->getID();
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, c1: Ptr<MSTransportable>, c2: Ptr<MSTransportable>) -> i32 {
        let pos1 = c1.get_current_stage().get_edge_pos(self.myTime);
        let pos2 = c2.get_current_stage().get_edge_pos(self.myTime);
        if pos1 != pos2 {
            return (pos1 < pos2) as i32;
        }
        (c1.get_id() < c2.get_id()) as i32
    }

}

