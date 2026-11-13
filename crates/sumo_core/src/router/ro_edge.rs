//! Port of router/ROEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// C++ typedef `std::vector<std::pair<const ROEdge *, const ROEdge *>> ROConstEdgePairVector`
pub type ROConstEdgePairVector = Vec<(Ptr<ROEdge>, Ptr<ROEdge>)>;

/// C++ typedef `std::vector<ROEdge *> ROEdgeVector`
pub type ROEdgeVector = Vec<Ptr<ROEdge>>;

/// A basic edge for routing applications
/// C++ class `ROEdge` (router/ROEdge.h)
#[repr(C)]
pub struct ROEdge {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// the junctions for this edge
    pub myFromJunction: Ptr<RONode>,
    pub myToJunction: Ptr<RONode>,
    /// The index (numeric id) of the edge
    pub myIndex: i32,
    /// The edge priority (road class)
    pub myPriority: i32,
    /// the type of this edge
    pub myType: String,
    /// the routing type of the edge (used to look up vType and vClass specific routing preferences)
    pub myRoutingType: String,
    /// The maximum speed allowed on this edge
    pub mySpeed: f64,
    /// The length of the edge
    pub myLength: f64,
    /// whether the edge is a source or a sink
    pub myAmSink: bool,
    /// whether the edge is a source or a sink
    pub myAmSource: bool,
    /// Container storing passing time varying over time for the edge
    pub myTravelTimes: ValueTimeLine,
    /// Information whether the time line shall be used instead of the length value
    pub myUsingTTTimeLine: bool,
    /// Container storing passing time varying over time for the edge
    pub myEfforts: ValueTimeLine,
    /// Information whether the time line shall be used instead of the length value
    pub myUsingETimeLine: bool,
    /// List of edges that may be approached from this edge
    pub myFollowingEdges: ROEdgeVector,
    pub myFollowingViaEdges: ROConstEdgePairVector,
    /// List of edges that approached this edge
    pub myApproachingEdges: ROEdgeVector,
    /// The function of the edge
    pub myFunction: SumoXMLEdgeFunc,
    /// The vClass speed restrictions for this edge
    pub mySpeedRestrictions: Ptr<BTreeMap<SUMOVehicleClass, f64>>,
    /// This edge's lanes
    pub myLanes: Vec<Ptr<ROLane>>,
    /// The list of allowed vehicle classes combined across lanes
    pub myCombinedPermissions: SVCPermissions,
    /// the other taz-connector if this edge isTazConnector, otherwise nullptr
    pub myOtherTazConnector: Ptr<ROEdge>,
    /// the bidirectional rail edge or nullpr
    pub myBidiEdge: Ptr<ROEdge>,
    /// The bounding rectangle of end nodes incoming or outgoing edges for taz connectors or of my own start and end node for normal edges
    pub myBoundary: Boundary,
    /// flat penalty when computing traveltime
    pub myTimePenalty: f64,
    /// cached value of parameters which may restrict access
    pub myParamRestrictions: Vec<f64>,
    /// The successors available for a given vClass
    pub myClassesSuccessorMap: BTreeMap<SUMOVehicleClass, ROEdgeVector>,
    /// The successors with vias available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, ROConstEdgePairVector>,
    /// a reversed version for backward routing
    pub myReversedRoutingEdge: Ptr<ReversedEdge<ROEdge, ROVehicle>>,
    /// An extended version of the reversed edge for backward routing (used for the arc flag router)
    pub myFlippedRoutingEdge: Ptr<FlippedEdge<ROEdge, RONode, ROVehicle>>,
    pub myRailwayRoutingEdge: Ptr<RailEdge<ROEdge, ROVehicle>>,
}

impl std::ops::Deref for ROEdge {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for ROEdge {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl RouterEdge<ROVehicle> for ROEdge {
    type Node = RONode;
    type Lane = ROLane;
    fn bidi_edge(&self) -> Ptr<ROEdge> {
        self.get_bidi_edge()
    }
    fn to_junction(&self) -> Ptr<RONode> {
        self.get_to_junction()
    }
    fn from_junction(&self) -> Ptr<RONode> {
        self.get_from_junction()
    }
    fn is_crossing(&self) -> bool {
        self.is_crossing()
    }
    fn is_walking_area(&self) -> bool {
        self.is_walking_area()
    }
    fn railway_routing_edge(&self) -> Ptr<RailEdge<ROEdge, ROVehicle>> {
        self.get_railway_routing_edge()
    }
    fn all_edges() -> Vec<Ptr<ROEdge>> {
        Self::get_all_edges().clone()
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
    fn distance_to(&self, other: Ptr<ROEdge>, do_boundary_estimate: bool) -> f64 {
        self.get_distance_to(other, do_boundary_estimate)
    }
    fn successors(&self, vclass: SUMOVehicleClass) -> &[Ptr<ROEdge>] {
        self.get_successors(vclass)
    }
    fn via_successors(&self, vclass: SUMOVehicleClass, ignore_transient: bool) -> &[(Ptr<ROEdge>, Ptr<ROEdge>)] {
        self.get_via_successors(vclass, ignore_transient)
    }
    fn predecessors(&self) -> &[Ptr<ROEdge>] {
        self.get_predecessors()
    }
    fn lanes(&self) -> &[Ptr<ROLane>] {
        self.get_lanes()
    }
    fn speed_limit(&self) -> f64 {
        self.get_speed_limit()
    }
    fn preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        self.get_preference(pars)
    }
    fn flipped_routing_edge(&self) -> Ptr<FlippedEdge<ROEdge, RONode, ROVehicle>> {
        self.get_flipped_routing_edge()
    }
    fn travel_time_static(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        ROEdge::get_travel_time_static(edge, veh, time)
    }
    fn travel_time_aggregated(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        ROEdge::get_travel_time_aggregated(edge, veh, time)
    }
    fn is_internal(&self) -> bool {
        self.is_internal()
    }
    fn is_taz_connector(&self) -> bool {
        self.is_taz_connector()
    }
    fn prohibits(&self, veh: Ptr<ROVehicle>, check_restrictions: bool) -> bool {
        self.prohibits(veh, check_restrictions)
    }
    fn restricts(&self, veh: Ptr<ROVehicle>) -> bool {
        self.restricts(veh)
    }
}

// C++ static members of ROEdge (port as module-level statics, see AGENTS.md):
//   static bool ROEdge::myInterpolate;
//   static bool ROEdge::myHaveEWarned;
//   static bool ROEdge::myHaveTTWarned;
//   static ROEdgeVector ROEdge::myEdges;
//   static double ROEdge::myPriorityFactor;
//   static double ROEdge::myMinEdgePriority;
//   static double ROEdge::myEdgePriorityRange;

/// C++ `ValueTimeLine<double>` (utils/common/ValueTimeLine.h): non-overlapping time ranges with values
// shortcut: the generic utils/common ValueTimeLine<T> is not ported; this f64 copy lives here (its only user is ROEdge) until that port lands, then switch to it
#[derive(Default, Clone)]
pub struct ValueTimeLine {
    myValues: BTreeMap<ValueTimeLineKey, (bool, f64)>,
}

/// Key ordering of std::map<double, ...> (times are finite)
#[derive(Clone, Copy, PartialEq)]
struct ValueTimeLineKey(f64);
impl Eq for ValueTimeLineKey {}
impl PartialOrd for ValueTimeLineKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ValueTimeLineKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl ValueTimeLine {
    /// the first entry with key > t (std::map::upper_bound)
    fn entry_gt(&self, t: f64) -> Option<(&ValueTimeLineKey, &(bool, f64))> {
        self.myValues.range((std::ops::Bound::Excluded(ValueTimeLineKey(t)), std::ops::Bound::Unbounded)).next()
    }
    /// the last entry with key <= t (std::prev(std::map::upper_bound))
    fn entry_le(&self, t: f64) -> Option<(&ValueTimeLineKey, &(bool, f64))> {
        self.myValues.range(..=ValueTimeLineKey(t)).next_back()
    }
    /// the last entry with key < t (std::prev(std::map::lower_bound))
    fn entry_lt(&self, t: f64) -> Option<(&ValueTimeLineKey, &(bool, f64))> {
        self.myValues.range(..ValueTimeLineKey(t)).next_back()
    }
    /// erases the entries strictly between begin and end
    fn erase_between(&mut self, begin: f64, end: f64) {
        self.myValues.retain(|k, _| !(k.0 > begin && k.0 < end));
    }

    /// C++: `void ValueTimeLine<T>::add(double, double, T)`
    pub fn add(&mut self, begin: f64, end: f64, value: f64) {
        // inserting strictly before the first or after the last interval (includes empty case)
        if self.entry_gt(begin).is_none() || self.entry_le(end).is_none() {
            self.myValues.insert(ValueTimeLineKey(begin), (true, value));
            self.myValues.insert(ValueTimeLineKey(end), (false, value));
            return;
        }
        // our end already has a value
        if self.myValues.contains_key(&ValueTimeLineKey(end)) {
            self.erase_between(begin, end);
            self.myValues.insert(ValueTimeLineKey(begin), (true, value));
            return;
        }
        // we have at least one entry strictly before our end
        let oldEndValue = *self.entry_lt(end).expect("ValueTimeLine::add: entry before end").1;
        self.erase_between(begin, end);
        self.myValues.insert(ValueTimeLineKey(begin), (true, value));
        self.myValues.insert(ValueTimeLineKey(end), oldEndValue);
    }

    /// C++: `T ValueTimeLine<T>::getValue(double) const`
    pub fn get_value(&self, time: f64) -> f64 {
        self.entry_le(time).expect("ValueTimeLine::getValue: no value set").1 .1
    }

    /// C++: `bool ValueTimeLine<T>::describesTime(double) const`
    pub fn describes_time(&self, time: f64) -> bool {
        self.entry_le(time).map_or(false, |e| e.1 .0)
    }

    /// C++: `double ValueTimeLine<T>::getSplitTime(double, double) const`
    pub fn get_split_time(&self, low: f64, high: f64) -> f64 {
        // afterLow == afterHigh (after the decrement) means both lie in the same entry
        match (self.entry_gt(low).map(|e| *e.0), self.entry_le(high).map(|e| *e.0)) {
            (Some(a), Some(b)) if a == b => a.0,
            _ => -1.,
        }
    }

    /// C++: `void ValueTimeLine<T>::fillGaps(T, bool)`
    pub fn fill_gaps(&mut self, value: f64, extendOverBoundaries: bool) {
        let mut value = value;
        for (_, v) in self.myValues.iter_mut() {
            if !v.0 {
                v.1 = value;
            }
        }
        if extendOverBoundaries && !self.myValues.is_empty() {
            let (lastKey, lastValid) = self.myValues.iter().next_back().map(|(k, v)| (*k, v.0)).expect("non-empty");
            if !lastValid {
                self.myValues.remove(&lastKey);
            }
            if let Some((_, first)) = self.myValues.iter().next() {
                value = first.1;
            }
        }
        self.myValues.insert(ValueTimeLineKey(-1.), (false, value));
    }
}

// C++ static members of ROEdge
static mut MY_INTERPOLATE: bool = false;
static mut MY_HAVE_E_WARNED: bool = false;
static mut MY_HAVE_TT_WARNED: bool = false;
static mut MY_EDGES: ROEdgeVector = Vec::new();
static mut MY_PRIORITY_FACTOR: f64 = 0.;
static mut MY_MIN_EDGE_PRIORITY: f64 = f64::MAX;
static mut MY_EDGE_PRIORITY_RANGE: f64 = 0.;

fn my_edges() -> &'static mut ROEdgeVector {
    unsafe { &mut *std::ptr::addr_of_mut!(MY_EDGES) }
}

/// C++ `std::max(a, b)` as used by MAX2
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ `std::min(a, b)` as used by MIN2
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ `gWeightsRandomFactor` (StdDefs.cpp) is not ported; it is set from the option of the same name
// shortcut: gWeightsRandomFactor is not a ported global, read the option it is copied from (SystemFrame.cpp) on each use, upgrade when std_defs gains it
fn g_weights_random_factor() -> f64 {
    utils::options::options_cont::get_float("weights.random-factor")
}

/// C++ implicit conversion `ROVehicle *` -> `RORoutable *` (the base subobject)
fn as_routable(veh: Ptr<ROVehicle>) -> Ptr<RORoutable> {
    if veh.is_null() { Ptr::null() } else { Ptr::new(&veh.base) }
}

/// C++ `const_cast` to call a non-const method through a const `this` (the routing caches are mutable in C++)
fn as_mut_edge(e: &ROEdge) -> &'static mut ROEdge {
    Ptr::new(e).get_mut().expect("non-null ROEdge")
}

impl ROEdge {
    /// C++ `myEdges[index] = this` from the constructor; Rust constructors return by value, so the owner calls this once the edge sits at its final address
    pub fn register_in_all_edges(this: Ptr<ROEdge>) {
        let index = this.myIndex as usize;
        let edges = my_edges();
        while edges.len() <= index {
            edges.push(Ptr::null());
        }
        edges[index] = this;
    }
}

impl ROEdge {
    /// C++: `void ROEdge::ROEdge(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    /// Constructor
    // ROEdge::ROEdge(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType) :
    //     Named(id),
    //     myFromJunction(from),
    //     myToJunction(to),
    //     myIndex(index),
    //     myPriority(priority),
    //     myType(type),
    //     myRoutingType(routingType),
    //     mySpeed(-1),
    //     myLength(0),
    //     myAmSink(false),
    //     myAmSource(false),
    //     myUsingTTTimeLine(false),
    //     myUsingETimeLine(false),
    //     mySpeedRestrictions(nullptr),
    //     myCombinedPermissions(0),
    //     myOtherTazConnector(nullptr),
    //     myBidiEdge(nullptr),
    //     myTimePenalty(0) {
    //     while ((int)myEdges.size() <= index) {
    //         myEdges.push_back(0);
    //     }
    //     myEdges[index] = this;
    //     if (from == nullptr && to == nullptr) {
    //         // TAZ edge, no lanes
    //         myCombinedPermissions = SVCAll;
    //     } else {
    //         // TODO we should not calculate the boundary here, the position for the nodes is not valid yet
    //         myBoundary.add(from->getPosition());
    //         myBoundary.add(to->getPosition());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        let mut this = ROEdge {
            base: Named::new(id),
            base2: Parameterised::new(),
            myFromJunction: from,
            myToJunction: to,
            myIndex: index,
            myPriority: priority,
            myType: type_.to_string(),
            myRoutingType: routingType.to_string(),
            mySpeed: -1.,
            myLength: 0.,
            myAmSink: false,
            myAmSource: false,
            myTravelTimes: ValueTimeLine::default(),
            myUsingTTTimeLine: false,
            myEfforts: ValueTimeLine::default(),
            myUsingETimeLine: false,
            myFollowingEdges: Vec::new(),
            myFollowingViaEdges: Vec::new(),
            myApproachingEdges: Vec::new(),
            myFunction: SumoXMLEdgeFunc::UNKNOWN, // C++ leaves this uninitialised; set later by setFunction
            mySpeedRestrictions: Ptr::null(),
            myLanes: Vec::new(),
            myCombinedPermissions: 0,
            myOtherTazConnector: Ptr::null(),
            myBidiEdge: Ptr::null(),
            myBoundary: Boundary::new(),
            myTimePenalty: 0.,
            myParamRestrictions: Vec::new(),
            myClassesSuccessorMap: BTreeMap::new(),
            myClassesViaSuccessorMap: BTreeMap::new(),
            myReversedRoutingEdge: Ptr::null(),
            myFlippedRoutingEdge: Ptr::null(),
            myRailwayRoutingEdge: Ptr::null(),
        };
        // shortcut: C++ also does myEdges[index] = this here; Rust returns the edge by value, so the owner must call ROEdge::register_in_all_edges once it is at its final address
        if from.is_null() && to.is_null() {
            // TAZ edge, no lanes
            this.myCombinedPermissions = SVC_ALL;
        } else {
            // TODO we should not calculate the boundary here, the position for the nodes is not valid yet
            this.myBoundary.add_pos(from.get_position());
            this.myBoundary.add_pos(to.get_position());
        }
        this
    }

    /// C++: `void ROEdge::ROEdge(const std::string &, const RONode *, const RONode *, SVCPermissions)`
    /// Constructor for dummy edge, only used when building the connectivity graph *
    // ROEdge::ROEdge(const std::string& id, const RONode* from, const RONode* to, SVCPermissions p) :
    //     Named(id),
    //     myFromJunction(const_cast<RONode*>(from)),
    //     myToJunction(const_cast<RONode*>(to)),
    //     myIndex(-1),
    //     myPriority(0),
    //     myType(""),
    //     mySpeed(std::numeric_limits<double>::max()),
    //     myLength(0),
    //     myAmSink(false),
    //     myAmSource(false),
    //     myUsingTTTimeLine(false),
    //     myUsingETimeLine(false),
    //     myCombinedPermissions(p),
    //     myOtherTazConnector(nullptr),
    //     myBidiEdge(nullptr),
    //     myTimePenalty(0)
    // { }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, p: SVCPermissions) -> Self {
        ROEdge {
            base: Named::new(id),
            base2: Parameterised::new(),
            myFromJunction: from,
            myToJunction: to,
            myIndex: -1,
            myPriority: 0,
            myType: String::new(),
            myRoutingType: String::new(),
            mySpeed: f64::MAX,
            myLength: 0.,
            myAmSink: false,
            myAmSource: false,
            myTravelTimes: ValueTimeLine::default(),
            myUsingTTTimeLine: false,
            myEfforts: ValueTimeLine::default(),
            myUsingETimeLine: false,
            myFollowingEdges: Vec::new(),
            myFollowingViaEdges: Vec::new(),
            myApproachingEdges: Vec::new(),
            myFunction: SumoXMLEdgeFunc::UNKNOWN,
            mySpeedRestrictions: Ptr::null(),
            myLanes: Vec::new(),
            myCombinedPermissions: p,
            myOtherTazConnector: Ptr::null(),
            myBidiEdge: Ptr::null(),
            myBoundary: Boundary::new(),
            myTimePenalty: 0.,
            myParamRestrictions: Vec::new(),
            myClassesSuccessorMap: BTreeMap::new(),
            myClassesViaSuccessorMap: BTreeMap::new(),
            myReversedRoutingEdge: Ptr::null(),
            myFlippedRoutingEdge: Ptr::null(),
            myRailwayRoutingEdge: Ptr::null(),
        }
    }

    /// C++: `void ROEdge::~ROEdge() noexcept` [virtual]
    /// Destructor
    // ROEdge::~ROEdge() {
    //     for (ROLane* const lane : myLanes) {
    //         delete lane;
    //     }
    //     delete myReversedRoutingEdge;
    //     delete myFlippedRoutingEdge;
    //     delete myRailwayRoutingEdge;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for lane in self.myLanes.iter() {
            drop(unsafe { Box::from_raw(lane.as_raw()) });
        }
        let reversed = self.myReversedRoutingEdge;
        if !reversed.is_null() {
            drop(unsafe { Box::from_raw(reversed.as_raw()) });
        }
        let flipped = self.myFlippedRoutingEdge;
        if !flipped.is_null() {
            drop(unsafe { Box::from_raw(flipped.as_raw()) });
        }
        let railway = self.myRailwayRoutingEdge;
        if !railway.is_null() {
            drop(unsafe { Box::from_raw(railway.as_raw()) });
        }
    }

    /// C++: `void ROEdge::addLane(ROLane *)` [virtual]
    /// Adds a lane to the edge while loading
    // void
    // ROEdge::addLane(ROLane* lane) {
    //     const double speed = lane->getSpeed();
    //     if (speed > mySpeed) {
    //         mySpeed = speed;
    //         myLength = lane->getLength();
    //     }
    //     mySpeed = speed > mySpeed ? speed : mySpeed;
    //     myLanes.push_back(lane);
    //
    //     // integrate new allowed classes
    //     myCombinedPermissions |= lane->getPermissions();
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, lane: Ptr<ROLane>) {
        let speed = lane.get_speed();
        if speed > self.mySpeed {
            self.mySpeed = speed;
            self.myLength = lane.get_length();
        }
        self.mySpeed = if speed > self.mySpeed { speed } else { self.mySpeed };
        self.myLanes.push(lane);

        // integrate new allowed classes
        self.myCombinedPermissions |= lane.get_permissions();
    }

    /// C++: `void ROEdge::addSuccessor(ROEdge *, ROEdge *, std::string)` [virtual]
    /// Adds information about a connected edge
    // void
    // ROEdge::addSuccessor(ROEdge* s, ROEdge* via, std::string) {
    //     if (isInternal()) {
    //         // for internal edges after an internal junction,
    //         // this is called twice and only the second call counts
    //         myFollowingEdges.clear();
    //         myFollowingViaEdges.clear();
    //     }
    //     if (find(myFollowingEdges.begin(), myFollowingEdges.end(), s) == myFollowingEdges.end()) {
    //         myFollowingEdges.push_back(s);
    //         myFollowingViaEdges.push_back(std::make_pair(s, via));
    //         if (isTazConnector() && s->getFromJunction() != nullptr) {
    //             myBoundary.add(s->getFromJunction()->getPosition());
    //         }
    //         if (!isInternal()) {
    //             s->myApproachingEdges.push_back(this);
    //             if (s->isTazConnector() && getToJunction() != nullptr) {
    //                 s->myBoundary.add(getToJunction()->getPosition());
    //             }
    //         }
    //         if (via != nullptr) {
    //             if (via->myApproachingEdges.size() == 0) {
    //                 via->myApproachingEdges.push_back(this);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<ROEdge>, via: Ptr<ROEdge>, dir: String) {
        let this = Ptr::from_mut(self);
        if self.is_internal() {
            // for internal edges after an internal junction,
            // this is called twice and only the second call counts
            self.myFollowingEdges.clear();
            self.myFollowingViaEdges.clear();
        }
        if !self.myFollowingEdges.contains(&s) {
            self.myFollowingEdges.push(s);
            self.myFollowingViaEdges.push((s, via));
            if self.is_taz_connector() && !s.get_from_junction().is_null() {
                self.myBoundary.add_pos(s.get_from_junction().get_position());
            }
            if !self.is_internal() {
                s.m().myApproachingEdges.push(this);
                if s.is_taz_connector() && !self.get_to_junction().is_null() {
                    s.m().myBoundary.add_pos(self.get_to_junction().get_position());
                }
            }
            if !via.is_null() {
                if via.m().myApproachingEdges.is_empty() {
                    via.m().myApproachingEdges.push(this);
                }
            }
        }
    }

    /// C++: `void ROEdge::setFunction(SumoXMLEdgeFunc)`
    /// Sets the function of the edge
    // inline void setFunction(SumoXMLEdgeFunc func) {
    //         myFunction = func;
    //     }
    #[allow(unused_variables)]
    pub fn set_function(&mut self, func: SumoXMLEdgeFunc) {
        self.myFunction = func;
    }

    /// C++: `void ROEdge::setSource(const bool)`
    /// Sets whether the edge is a source
    // inline void setSource(const bool isSource = true) {
    //         myAmSource = isSource;
    //     }
    #[allow(unused_variables)]
    pub fn set_source(&mut self, isSource: bool) {
        self.myAmSource = isSource;
    }

    /// C++: `void ROEdge::setSink(const bool)`
    /// Sets whether the edge is a sink
    // inline void setSink(const bool isSink = true) {
    //         myAmSink = isSink;
    //     }
    #[allow(unused_variables)]
    pub fn set_sink(&mut self, isSink: bool) {
        self.myAmSink = isSink;
    }

    /// C++: `void ROEdge::setSpeedRestrictions(const std::map<SUMOVehicleClass, double> *)`
    /// Sets the vehicle class specific speed limits of the edge
    // inline void setSpeedRestrictions(const std::map<SUMOVehicleClass, double>* restrictions) {
    //         mySpeedRestrictions = restrictions;
    //     }
    #[allow(unused_variables)]
    pub fn set_speed_restrictions(&mut self, restrictions: Ptr<BTreeMap<SUMOVehicleClass, f64>>) {
        self.mySpeedRestrictions = restrictions;
    }

    /// C++: `void ROEdge::setTimePenalty(double)`
    // inline void setTimePenalty(double value) {
    //         myTimePenalty = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_time_penalty(&mut self, value: f64) {
        self.myTimePenalty = value;
    }

    /// C++: `double ROEdge::getTimePenalty() const` [const]
    // inline double getTimePenalty() const {
    //         return myTimePenalty;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_penalty(&self) -> f64 {
        self.myTimePenalty
    }

    /// C++: `bool ROEdge::isNormal() const` [const]
    /// return whether this edge is a normal edge
    // inline bool isNormal() const {
    //         return myFunction == SumoXMLEdgeFunc::NORMAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_normal(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::NORMAL
    }

    /// C++: `bool ROEdge::isInternal() const` [const]
    /// return whether this edge is an internal edge
    // inline bool isInternal() const {
    //         return myFunction == SumoXMLEdgeFunc::INTERNAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::INTERNAL
    }

    /// C++: `bool ROEdge::isCrossing() const` [const]
    /// return whether this edge is a pedestrian crossing
    // inline bool isCrossing() const {
    //         return myFunction == SumoXMLEdgeFunc::CROSSING;
    //     }
    #[allow(unused_variables)]
    pub fn is_crossing(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::CROSSING
    }

    /// C++: `bool ROEdge::isWalkingArea() const` [const]
    /// return whether this edge is walking area
    // inline bool isWalkingArea() const {
    //         return myFunction == SumoXMLEdgeFunc::WALKINGAREA;
    //     }
    #[allow(unused_variables)]
    pub fn is_walking_area(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::WALKINGAREA
    }

    /// C++: `bool ROEdge::isTazConnector() const` [const]
    // inline bool isTazConnector() const {
    //         return myFunction == SumoXMLEdgeFunc::CONNECTOR;
    //     }
    #[allow(unused_variables)]
    pub fn is_taz_connector(&self) -> bool {
        self.myFunction == SumoXMLEdgeFunc::CONNECTOR
    }

    /// C++: `void ROEdge::setOtherTazConnector(const ROEdge *)`
    // void setOtherTazConnector(const ROEdge* edge) {
    //         myOtherTazConnector = edge;
    //     }
    #[allow(unused_variables)]
    pub fn set_other_taz_connector(&mut self, edge: Ptr<ROEdge>) {
        self.myOtherTazConnector = edge;
    }

    /// C++: `const ROEdge * ROEdge::getOtherTazConnector() const` [const]
    // const ROEdge* getOtherTazConnector() const {
    //         return myOtherTazConnector;
    //     }
    #[allow(unused_variables)]
    pub fn get_other_taz_connector(&self) -> Ptr<ROEdge> {
        self.myOtherTazConnector
    }

    /// C++: `void ROEdge::buildTimeLines(const std::string &, const bool)`
    /// Builds the internal representation of the travel time/effort
    // void
    // ROEdge::buildTimeLines(const std::string& measure, const bool boundariesOverride) {
    //     if (myUsingETimeLine) {
    //         double value = myLength / mySpeed;
    //         const SUMOEmissionClass c = PollutantsInterface::getClassByName("HBEFA4/default");
    //         if (measure == "CO") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::CO, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "CO2") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::CO2, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "HC") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::HC, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "PMx") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::PM_X, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "NOx") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::NO_X, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "fuel") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::FUEL, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "electricity") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::ELEC, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         myEfforts.fillGaps(value, boundariesOverride);
    //     }
    //     if (myUsingTTTimeLine) {
    //         myTravelTimes.fillGaps(myLength / mySpeed + myTimePenalty, boundariesOverride);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_time_lines(&mut self, measure: &str, boundariesOverride: bool) {
        if self.myUsingETimeLine {
            let mut value = self.myLength / self.mySpeed;
            let c = PollutantsInterface::get_class_by_name("HBEFA4/default", SVC_IGNORING);
            if measure == "CO" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::CO, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "CO2" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::CO2, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "HC" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::HC, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "PMx" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::PM_X, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "NOx" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::NO_X, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "fuel" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::FUEL, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            if measure == "electricity" {
                value = PollutantsInterface::compute(c, PollutantsInterface_EmissionType::ELEC, self.mySpeed, 0., 0., Ptr::null()) * value; // @todo: give correct slope
            }
            self.myEfforts.fill_gaps(value, boundariesOverride);
        }
        if self.myUsingTTTimeLine {
            let tt = self.myLength / self.mySpeed + self.myTimePenalty;
            self.myTravelTimes.fill_gaps(tt, boundariesOverride);
        }
    }

    /// C++: `void ROEdge::cacheParamRestrictions(const std::vector<std::string> &)`
    // void
    // ROEdge::cacheParamRestrictions(const std::vector<std::string>& restrictionKeys) {
    //     for (const std::string& key : restrictionKeys) {
    //         const std::string value = getParameter(key, "1e40");
    //         myParamRestrictions.push_back(StringUtils::toDouble(value));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cache_param_restrictions(&mut self, restrictionKeys: &Vec<String>) {
        for key in restrictionKeys {
            let value = self.base2.get_parameter(key, "1e40");
            // shortcut: C++ StringUtils::toDouble throws on malformed input; here it panics with the same message
            self.myParamRestrictions.push(utils::common::string_utils::to_double(&value).unwrap_or_else(|e| panic!("{e}")));
        }
    }

    /// C++: `SumoXMLEdgeFunc ROEdge::getFunction() const` [const]
    /// Returns the function of the edge
    // inline SumoXMLEdgeFunc getFunction() const {
    //         return myFunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_function(&self) -> SumoXMLEdgeFunc {
        self.myFunction
    }

    /// C++: `bool ROEdge::isSink() const` [const]
    /// Returns whether the edge acts as a sink
    // inline bool isSink() const {
    //         return myAmSink;
    //     }
    #[allow(unused_variables)]
    pub fn is_sink(&self) -> bool {
        self.myAmSink
    }

    /// C++: `double ROEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `int ROEdge::getNumericalID() const` [const]
    /// Returns the index (numeric id) of the edge
    // int getNumericalID() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myIndex
    }

    /// C++: `double ROEdge::getSpeedLimit() const` [const]
    /// Returns the speed allowed on this edge
    // double getSpeedLimit() const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        self.mySpeed
    }

    /// C++: `double ROEdge::getLengthGeometryFactor() const` [const]
    /// return a lower bound on shape.length() / myLength that is
    // double
    // ROEdge::getLengthGeometryFactor() const {
    //     return myLanes.empty() ? 1. : myLanes[0]->getShape().length() / myLanes[0]->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        if self.myLanes.is_empty() {
            1.
        } else {
            self.myLanes[0].get_shape().length() / self.myLanes[0].get_length()
        }
    }

    /// C++: `int ROEdge::getNumLanes() const` [const]
    /// Returns the number of lanes this edge has
    // int getNumLanes() const {
    //         return (int) myLanes.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_lanes(&self) -> i32 {
        self.myLanes.len() as i32
    }

    /// C++: `bool ROEdge::isConnectedTo(const ROEdge &, const SUMOVehicleClass, bool) const` [const]
    /// returns the information whether this edge is directly connected to the given
    // bool
    // ROEdge::isConnectedTo(const ROEdge& e, const SUMOVehicleClass vClass, bool ignoreTransientPermissions) const {
    //     // @todo needs to be used with #12501
    //     UNUSED_PARAMETER(ignoreTransientPermissions);
    //     const ROEdgeVector& followers = getSuccessors(vClass);
    //     return std::find(followers.begin(), followers.end(), &e) != followers.end();
    // }
    #[allow(unused_variables)]
    pub fn is_connected_to(&self, e: &ROEdge, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> bool {
        let followers = self.get_successors(vClass);
        followers.contains(&Ptr::new(e))
    }

    /// C++: `bool ROEdge::prohibits(const ROVehicle *const, bool) const` [const]
    /// Returns whether this edge prohibits the given vehicle to pass it
    // inline bool prohibits(const ROVehicle* const vehicle, bool checkRestrictions = false) const {
    //         const SUMOVehicleClass vclass = vehicle->getVClass();
    //         return (myCombinedPermissions & vclass) != vclass || (checkRestrictions && restricts(vehicle));
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<ROVehicle>, checkRestrictions: bool) -> bool {
        let vclass = vehicle.get_v_class();
        (self.myCombinedPermissions & vclass) != vclass || (checkRestrictions && self.restricts(vehicle))
    }

    /// C++: `SVCPermissions ROEdge::getPermissions() const` [const]
    // inline SVCPermissions getPermissions() const {
    //         return myCombinedPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        self.myCombinedPermissions
    }

    /// C++: `bool ROEdge::restricts(const ROVehicle *const) const` [const]
    /// Returns whether this edge has restriction parameters forbidding the given vehicle to pass it
    // inline bool restricts(const ROVehicle* const vehicle) const {
    //         const std::vector<double>& vTypeRestrictions = vehicle->getType()->paramRestrictions;
    //         assert(vTypeRestrictions.size() == myParamRestrictions.size());
    //         for (int i = 0; i < (int)vTypeRestrictions.size(); i++) {
    //             if (vTypeRestrictions[i] > myParamRestrictions[i]) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<ROVehicle>) -> bool {
        let vType = vehicle.get_type();
        let vTypeRestrictions = &vType.paramRestrictions;
        debug_assert_eq!(vTypeRestrictions.len(), self.myParamRestrictions.len());
        for i in 0..vTypeRestrictions.len() {
            if vTypeRestrictions[i] > self.myParamRestrictions[i] {
                return true;
            }
        }
        false
    }

    /// C++: `bool ROEdge::allFollowersProhibit(const ROVehicle *const) const` [const]
    /// Returns whether this edge succeeding edges prohibit the given vehicle to pass them
    // bool
    // ROEdge::allFollowersProhibit(const ROVehicle* const vehicle) const {
    //     for (ROEdgeVector::const_iterator i = myFollowingEdges.begin(); i != myFollowingEdges.end(); ++i) {
    //         if (!(*i)->prohibits(vehicle)) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn all_followers_prohibit(&self, vehicle: Ptr<ROVehicle>) -> bool {
        for f in self.myFollowingEdges.iter() {
            if !f.prohibits(vehicle, false) {
                return false;
            }
        }
        true
    }

    /// C++: `void ROEdge::addEffort(double, double, double)`
    /// Adds a weight value
    // void
    // ROEdge::addEffort(double value, double timeBegin, double timeEnd) {
    //     myEfforts.add(timeBegin, timeEnd, value);
    //     myUsingETimeLine = true;
    // }
    #[allow(unused_variables)]
    pub fn add_effort(&mut self, value: f64, timeBegin: f64, timeEnd: f64) {
        self.myEfforts.add(timeBegin, timeEnd, value);
        self.myUsingETimeLine = true;
    }

    /// C++: `void ROEdge::addTravelTime(double, double, double)`
    /// Adds a travel time value
    // void
    // ROEdge::addTravelTime(double value, double timeBegin, double timeEnd) {
    //     myTravelTimes.add(timeBegin, timeEnd, value);
    //     myUsingTTTimeLine = true;
    // }
    #[allow(unused_variables)]
    pub fn add_travel_time(&mut self, value: f64, timeBegin: f64, timeEnd: f64) {
        self.myTravelTimes.add(timeBegin, timeEnd, value);
        self.myUsingTTTimeLine = true;
    }

    /// C++: `int ROEdge::getNumSuccessors() const` [const]
    /// Returns the number of edges this edge is connected to
    // int
    // ROEdge::getNumSuccessors() const {
    //     if (myAmSink) {
    //         return 0;
    //     }
    //     return (int) myFollowingEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_num_successors(&self) -> i32 {
        if self.myAmSink {
            0
        } else {
            self.myFollowingEdges.len() as i32
        }
    }

    /// C++: `const ROEdgeVector & ROEdge::getSuccessors(SUMOVehicleClass) const` [const]
    /// Returns the following edges, restricted by vClass
    // const ROEdgeVector&
    // ROEdge::getSuccessors(SUMOVehicleClass vClass) const {
    //     if (vClass == SVC_IGNORING || !RONet::getInstance()->hasPermissions() || isTazConnector()) {
    //         return myFollowingEdges;
    //     }
    // #ifdef HAVE_FOX
    //     FXMutexLock locker(myLock);
    // #endif
    //     std::map<SUMOVehicleClass, ROEdgeVector>::const_iterator i = myClassesSuccessorMap.find(vClass);
    //     if (i != myClassesSuccessorMap.end()) {
    //         // can use cached value
    //         return i->second;
    //     }
    //     // this vClass is requested for the first time. rebuild all successors
    //     std::set<ROEdge*> followers;
    //     for (const ROLane* const lane : myLanes) {
    //         if ((lane->getPermissions() & vClass) != 0) {
    //             for (const auto& next : lane->getOutgoingViaLanes()) {
    //                 if ((next.first->getPermissions() & vClass) != 0 && (next.second == nullptr || (next.second->getPermissions() & vClass) != 0)) {
    //                     followers.insert(&next.first->getEdge());
    //                 }
    //             }
    //         }
    //     }
    //     // also add district edges (they are not connected at the lane level
    //     for (ROEdgeVector::const_iterator it = myFollowingEdges.begin(); it != myFollowingEdges.end(); ++it) {
    //         if ((*it)->isTazConnector()) {
    //             followers.insert(*it);
    //         }
    //     }
    //     myClassesSuccessorMap[vClass].insert(myClassesSuccessorMap[vClass].begin(),
    //                                          followers.begin(), followers.end());
    //     return myClassesSuccessorMap[vClass];
    // }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &ROEdgeVector {
        if vClass == SVC_IGNORING || !RONet::get_instance().has_permissions() || self.is_taz_connector() {
            return &self.myFollowingEdges;
        }
        // the C++ caches the result per vClass in a mutable map; same here, through the non-owning pointer
        if !self.myClassesSuccessorMap.contains_key(&vClass) {
            // determinism: C++ std::set<ROEdge*> orders by address; BTreeSet<Ptr<ROEdge>> keeps that order
            let mut followers: BTreeSet<Ptr<ROEdge>> = BTreeSet::new();
            for lane in self.myLanes.iter() {
                if (lane.get_permissions() & vClass) != 0 {
                    for next in lane.get_outgoing_via_lanes() {
                        if (next.0.get_permissions() & vClass) != 0 && (next.1.is_null() || (next.1.get_permissions() & vClass) != 0) {
                            followers.insert(Ptr::from_mut(next.0.get_edge()));
                        }
                    }
                }
            }
            // also add district edges (they are not connected at the lane level)
            for e in self.myFollowingEdges.iter() {
                if e.is_taz_connector() {
                    followers.insert(*e);
                }
            }
            as_mut_edge(self).myClassesSuccessorMap.insert(vClass, followers.into_iter().collect());
        }
        self.myClassesSuccessorMap.get(&vClass).expect("inserted above")
    }

    /// C++: `const ROConstEdgePairVector & ROEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    /// Returns the following edges including vias, restricted by vClass
    // const ROConstEdgePairVector&
    // ROEdge::getViaSuccessors(SUMOVehicleClass vClass, bool /*ignoreTransientPermissions*/) const {
    //     if (vClass == SVC_IGNORING || !RONet::getInstance()->hasPermissions() || isTazConnector()) {
    //         return myFollowingViaEdges;
    //     }
    // #ifdef HAVE_FOX
    //     FXMutexLock locker(myLock);
    // #endif
    //     std::map<SUMOVehicleClass, ROConstEdgePairVector>::const_iterator i = myClassesViaSuccessorMap.find(vClass);
    //     if (i != myClassesViaSuccessorMap.end()) {
    //         // can use cached value
    //         return i->second;
    //     }
    //     // this vClass is requested for the first time. rebuild all successors
    //     std::set<std::pair<const ROEdge*, const ROEdge*> > followers;
    //     for (const ROLane* const lane : myLanes) {
    //         if ((lane->getPermissions() & vClass) != 0) {
    //             for (const auto& next : lane->getOutgoingViaLanes()) {
    //                 if ((next.first->getPermissions() & vClass) != 0 && (next.second == nullptr || (next.second->getPermissions() & vClass) != 0)) {
    //                     followers.insert(std::make_pair(&next.first->getEdge(), next.second));
    //                 }
    //             }
    //         }
    //     }
    //     // also add district edges (they are not connected at the lane level
    //     for (const ROEdge* e : myFollowingEdges) {
    //         if (e->isTazConnector()) {
    //             followers.insert(std::make_pair(e, e));
    //         }
    //     }
    //     myClassesViaSuccessorMap[vClass].insert(myClassesViaSuccessorMap[vClass].begin(),
    //                                             followers.begin(), followers.end());
    //     return myClassesViaSuccessorMap[vClass];
    // }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ROConstEdgePairVector {
        if vClass == SVC_IGNORING || !RONet::get_instance().has_permissions() || self.is_taz_connector() {
            return &self.myFollowingViaEdges;
        }
        // the C++ caches the result per vClass in a mutable map; same here, through the non-owning pointer
        if !self.myClassesViaSuccessorMap.contains_key(&vClass) {
            // determinism: C++ std::set<pair<ROEdge*, ROEdge*>> orders by address; BTreeSet keeps that order
            let mut followers: BTreeSet<(Ptr<ROEdge>, Ptr<ROEdge>)> = BTreeSet::new();
            for lane in self.myLanes.iter() {
                if (lane.get_permissions() & vClass) != 0 {
                    for next in lane.get_outgoing_via_lanes() {
                        if (next.0.get_permissions() & vClass) != 0 && (next.1.is_null() || (next.1.get_permissions() & vClass) != 0) {
                            followers.insert((Ptr::from_mut(next.0.get_edge()), next.1));
                        }
                    }
                }
            }
            // also add district edges (they are not connected at the lane level)
            for e in self.myFollowingEdges.iter() {
                if e.is_taz_connector() {
                    followers.insert((*e, *e));
                }
            }
            as_mut_edge(self).myClassesViaSuccessorMap.insert(vClass, followers.into_iter().collect());
        }
        self.myClassesViaSuccessorMap.get(&vClass).expect("inserted above")
    }

    /// C++: `void ROEdge::resetSuccessors()`
    /// reset after lane permissions changes
    // void resetSuccessors() {
    //         myClassesSuccessorMap.clear();
    //         myClassesViaSuccessorMap.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset_successors(&mut self) {
        self.myClassesSuccessorMap.clear();
        self.myClassesViaSuccessorMap.clear();
    }

    /// C++: `int ROEdge::getNumPredecessors() const` [const]
    /// Returns the number of edges connected to this edge
    // int
    // ROEdge::getNumPredecessors() const {
    //     if (myAmSource) {
    //         return 0;
    //     }
    //     return (int) myApproachingEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_num_predecessors(&self) -> i32 {
        if self.myAmSource {
            0
        } else {
            self.myApproachingEdges.len() as i32
        }
    }

    /// C++: `const ROEdgeVector & ROEdge::getPredecessors() const` [const]
    /// Returns the edge at the given position from the list of incoming edges
    // const ROEdgeVector& getPredecessors() const {
    //         return myApproachingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_predecessors(&self) -> &ROEdgeVector {
        &self.myApproachingEdges
    }

    /// C++: `const ROEdge * ROEdge::getNormalBefore() const` [const]
    /// if this edge is an internal edge, return its first normal predecessor, otherwise the edge itself
    // const ROEdge*
    // ROEdge::getNormalBefore() const {
    //     const ROEdge* result = this;
    //     while (result->isInternal()) {
    //         assert(myApproachingEdges.size() == 1);
    //         result = result->myApproachingEdges.front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_before(&self) -> Ptr<ROEdge> {
        let mut result = Ptr::new(self);
        while result.is_internal() {
            // C++ asserts that exactly one approaching edge exists
            result = result.myApproachingEdges[0];
        }
        result
    }

    /// C++: `const ROEdge * ROEdge::getNormalAfter() const` [const]
    /// if this edge is an internal edge, return its first normal successor, otherwise the edge itself
    // const ROEdge*
    // ROEdge::getNormalAfter() const {
    //     const ROEdge* result = this;
    //     while (result->isInternal()) {
    //         assert(myFollowingEdges.size() == 1);
    //         result = result->myFollowingEdges.front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_after(&self) -> Ptr<ROEdge> {
        let mut result = Ptr::new(self);
        while result.is_internal() {
            // C++ asserts that exactly one following edge exists
            result = result.myFollowingEdges[0];
        }
        result
    }

    /// C++: `double ROEdge::getEffort(const ROVehicle *const, double) const` [const]
    /// Returns the effort for this edge
    // double
    // ROEdge::getEffort(const ROVehicle* const veh, double time) const {
    //     double ret = 0;
    //     if (!getStoredEffort(time, ret)) {
    //         return myLength / getMaxSpeed(veh);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_effort(&self, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut ret = 0.;
        if !self.get_stored_effort_2(time, &mut ret) {
            return self.myLength / self.get_max_speed(as_routable(veh));
        }
        ret
    }

    /// C++: `bool ROEdge::hasLoadedTravelTime(double) const` [const]
    /// Returns whether a travel time for this edge was loaded
    // bool
    // ROEdge::hasLoadedTravelTime(double time) const {
    //     return myUsingTTTimeLine && myTravelTimes.describesTime(time);
    // }
    #[allow(unused_variables)]
    pub fn has_loaded_travel_time(&self, time: f64) -> bool {
        self.myUsingTTTimeLine && self.myTravelTimes.describes_time(time)
    }

    /// C++: `double ROEdge::getTravelTime(const ROVehicle *const, double) const` [const]
    /// Returns the travel time for this edge
    // double
    // ROEdge::getTravelTime(const ROVehicle* const veh, double time) const {
    //     if (myUsingTTTimeLine) {
    //         if (myTravelTimes.describesTime(time)) {
    //             double lineTT = myTravelTimes.getValue(time);
    //             if (myInterpolate) {
    //                 const double inTT = lineTT;
    //                 const double split = (double)(myTravelTimes.getSplitTime(time, time + inTT) - time);
    //                 if (split >= 0) {
    //                     lineTT = myTravelTimes.getValue(time + inTT) * ((double)1. - split / inTT) + split;
    //                 }
    //             }
    //             return MAX2(getMinimumTravelTime(veh), lineTT);
    //         } else {
    //             if (!myHaveTTWarned) {
    //                 WRITE_WARNINGF(TL("No interval matches passed time=% in edge '%'.\n Using edge's length / max speed."), time, myID);
    //                 myHaveTTWarned = true;
    //             }
    //         }
    //     }
    //     const double speed = veh != nullptr ? getMaxSpeed(veh) : mySpeed;
    //     return myLength / speed + myTimePenalty;
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        if self.myUsingTTTimeLine {
            if self.myTravelTimes.describes_time(time) {
                let mut lineTT = self.myTravelTimes.get_value(time);
                if unsafe { MY_INTERPOLATE } {
                    let inTT = lineTT;
                    let split = self.myTravelTimes.get_split_time(time, time + inTT) - time;
                    if split >= 0. {
                        lineTT = self.myTravelTimes.get_value(time + inTT) * (1. - split / inTT) + split;
                    }
                }
                return max2(self.get_minimum_travel_time(veh), lineTT);
            } else if unsafe { !MY_HAVE_TT_WARNED } {
                write_warningf!("No interval matches passed time=% in edge '%'.\n Using edge's length / max speed.", time, self.get_id());
                unsafe { MY_HAVE_TT_WARNED = true; }
            }
        }
        let speed = if !veh.is_null() { self.get_max_speed(as_routable(veh)) } else { self.mySpeed };
        self.myLength / speed + self.myTimePenalty
    }

    /// C++: `double ROEdge::getEffortStatic(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the effort for the given edge
    // static inline double getEffortStatic(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getEffort(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_effort_static(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        edge.get_effort(veh, time)
    }

    /// C++: `double ROEdge::getTravelTimeStatic(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the travel time for the given edge
    // static inline double getTravelTimeStatic(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time) * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        edge.get_travel_time(veh, time) * ROEdge::get_routing_factor(edge, veh)
    }

    /// C++: `double ROEdge::getTravelTimeStaticRandomized(const ROEdge *const, const ROVehicle *const, double)` [static]
    // static inline double getTravelTimeStaticRandomized(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time)
    //                * (1 + RandHelper::randHash(veh->getRandomSeed() ^ edge->getNumericalID()) * (gWeightsRandomFactor - 1))
    //                * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        edge.get_travel_time(veh, time)
            * (1. + utils::common::rand_helper::rand_hash(veh.get_random_seed() ^ edge.get_numerical_id() as i64) * (g_weights_random_factor() - 1.))
            * ROEdge::get_routing_factor(edge, veh)
    }

    /// C++: `double ROEdge::getTravelTimeAggregated(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Alias for getTravelTimeStatic (there is no routing device to provide aggregated travel times)
    // static inline double getTravelTimeAggregated(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time) * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        edge.get_travel_time(veh, time) * ROEdge::get_routing_factor(edge, veh)
    }

    /// C++: `double ROEdge::getTravelTimeStaticPriorityFactor(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Return traveltime weighted by edge priority (scaled penalty for low-priority edges)
    // static inline double getTravelTimeStaticPriorityFactor(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double result = edge->getTravelTime(veh, time);
    //         // lower priority should result in higher effort (and the edge with
    //         // minimum priority receives a factor of myPriorityFactor
    //         const double relativeInversePrio = 1 - ((edge->getPriority() - myMinEdgePriority) / myEdgePriorityRange);
    //         result *= 1 + relativeInversePrio * myPriorityFactor;
    //         return result * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_priority_factor(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut result = edge.get_travel_time(veh, time);
        // lower priority should result in higher effort (and the edge with
        // minimum priority receives a factor of myPriorityFactor
        let relativeInversePrio = 1. - ((edge.get_priority() as f64 - unsafe { MY_MIN_EDGE_PRIORITY }) / unsafe { MY_EDGE_PRIORITY_RANGE });
        result *= 1. + relativeInversePrio * unsafe { MY_PRIORITY_FACTOR };
        result * ROEdge::get_routing_factor(edge, veh)
    }

    /// C++: `double ROEdge::getTravelTimeStaticPriorityFactorRandomized(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Return traveltime weighted by edge priority (scaled penalty for low-priority edges)
    // static inline double getTravelTimeStaticPriorityFactorRandomized(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double result = edge->getTravelTime(veh, time);
    //         // lower priority should result in higher effort (and the edge with
    //         // minimum priority receives a factor of myPriorityFactor
    //         const double relativeInversePrio = 1 - ((edge->getPriority() - myMinEdgePriority) / myEdgePriorityRange);
    //         result *= 1 + relativeInversePrio * myPriorityFactor;
    //         result *= (1 + RandHelper::randHash(veh->getRandomSeed() ^ edge->getNumericalID()) * (gWeightsRandomFactor - 1));
    //         return result * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_priority_factor_randomized(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut result = edge.get_travel_time(veh, time);
        // lower priority should result in higher effort (and the edge with
        // minimum priority receives a factor of myPriorityFactor
        let relativeInversePrio = 1. - ((edge.get_priority() as f64 - unsafe { MY_MIN_EDGE_PRIORITY }) / unsafe { MY_EDGE_PRIORITY_RANGE });
        result *= 1. + relativeInversePrio * unsafe { MY_PRIORITY_FACTOR };
        result *= 1. + utils::common::rand_helper::rand_hash(veh.get_random_seed() ^ edge.get_numerical_id() as i64) * (g_weights_random_factor() - 1.);
        result * ROEdge::get_routing_factor(edge, veh)
    }

    /// C++: `double ROEdge::getRoutingFactor(const ROEdge *const, const ROVehicle *const)` [static]
    // static inline double getRoutingFactor(const ROEdge* const edge, const ROVehicle* const veh) {
    //         return gRoutingPreferences ?  1 / edge->getPreference(veh->getVTypeParameter()) : 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_factor(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>) -> f64 {
        if utils::common::std_defs::g_routing_preferences() {
            1. / edge.get_preference(veh.get_v_type_parameter())
        } else {
            1.
        }
    }

    /// C++: `double ROEdge::getMinimumTravelTime(const ROVehicle *const) const` [const]
    /// Returns a lower bound for the travel time on this edge without using any stored timeLine
    // inline double getMinimumTravelTime(const ROVehicle* const veh) const {
    //         if (isTazConnector()) {
    //             return 0;
    //         } else if (veh != 0) {
    //             return myLength / getMaxSpeed(veh);
    //         } else {
    //             return myLength / mySpeed;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, veh: Ptr<ROVehicle>) -> f64 {
        if self.is_taz_connector() {
            0.
        } else if !veh.is_null() {
            self.myLength / self.get_max_speed(as_routable(veh))
        } else {
            self.myLength / self.mySpeed
        }
    }

    /// C++: `double ROEdge::getMaxSpeed(const RORoutable *const) const` [const]
    // inline double getMaxSpeed(const RORoutable* const veh) const {
    //         return MIN2(veh->getMaxSpeed(), veh->getChosenSpeedFactor() * getVClassMaxSpeed(veh->getVClass()));
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, veh: Ptr<RORoutable>) -> f64 {
        let a = veh.get_max_speed();
        let b = veh.get_chosen_speed_factor() * self.get_v_class_max_speed(veh.get_v_class());
        if a < b { a } else { b }
    }

    /// C++: `double ROEdge::getVClassMaxSpeed(SUMOVehicleClass) const` [const]
    /// Returns the lane's maximum speed, given a vehicle's speed limit adaptation
    // inline double getVClassMaxSpeed(SUMOVehicleClass vclass) const {
    //         if (mySpeedRestrictions != 0) {
    //             std::map<SUMOVehicleClass, double>::const_iterator r = mySpeedRestrictions->find(vclass);
    //             if (r != mySpeedRestrictions->end()) {
    //                 return r->second;
    //             }
    //         }
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class_max_speed(&self, vclass: SUMOVehicleClass) -> f64 {
        if !self.mySpeedRestrictions.is_null() {
            if let Some(r) = self.mySpeedRestrictions.get().and_then(|m| m.get(&vclass)) {
                return *r;
            }
        }
        self.mySpeed
    }

    /// C++: `double ROEdge::getEmissionEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // template<PollutantsInterface::EmissionType ET>
    //     static double getEmissionEffort(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double ret = 0;
    //         if (!edge->getStoredEffort(time, ret)) {
    //             const SUMOVTypeParameter* const type = veh->getType();
    //             const double vMax = edge->getMaxSpeed(veh);
    //             const double accel = type->getCFParam(SUMO_ATTR_ACCEL, SUMOVTypeParameter::getDefaultAccel(type->vehicleClass)) * type->getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(type->vehicleClass)) / 2.;
    //             ret = PollutantsInterface::computeDefault(type->emissionClass, ET, vMax, accel, 0, edge->getTravelTime(veh, time), nullptr); // @todo: give correct slope
    //         }
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_effort(et: PollutantsInterface_EmissionType, edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut ret = 0.;
        if !edge.get_stored_effort_2(time, &mut ret) {
            let typ = veh.get_type();
            let vMax = edge.get_max_speed(as_routable(veh));
            let accel = typ.get_cf_param(SUMO_ATTR_ACCEL, SUMOVTypeParameter::get_default_accel(typ.vehicleClass)) * typ.get_cf_param(SUMO_ATTR_SIGMA, SUMOVTypeParameter::get_default_imperfection(typ.vehicleClass)) / 2.;
            ret = PollutantsInterface::compute_default(typ.emissionClass, et, vMax, accel, 0., edge.get_travel_time(veh, time), Ptr::null()); // @todo: give correct slope
        }
        ret
    }

    /// C++: `double ROEdge::getNoiseEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // double
    // ROEdge::getNoiseEffort(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //     double ret = 0;
    //     if (!edge->getStoredEffort(time, ret)) {
    //         ret = HelpersHarmonoise::computeNoise(veh->getType()->emissionClass, edge->getMaxSpeed(veh), 0);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_noise_effort(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut ret = 0.;
        if !edge.get_stored_effort_2(time, &mut ret) {
            ret = HelpersHarmonoise::compute_noise(veh.get_type().emissionClass, edge.get_max_speed(as_routable(veh)), 0.);
        }
        ret
    }

    /// C++: `double ROEdge::getStoredEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // static double getStoredEffort(const ROEdge* const edge, const ROVehicle* const /*veh*/, double time) {
    //         double ret = 0;
    //         edge->getStoredEffort(time, ret);
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn get_stored_effort(edge: Ptr<ROEdge>, arg1: Ptr<ROVehicle>, time: f64) -> f64 {
        let mut ret = 0.;
        edge.get_stored_effort_2(time, &mut ret);
        ret
    }

    /// C++: `double ROEdge::getDistanceTo(const ROEdge *, const bool) const` [const]
    /// optimistic distance heuristic for use in routing
    // double
    // ROEdge::getDistanceTo(const ROEdge* other, const bool doBoundaryEstimate) const {
    //     assert(this != other);
    //     if (doBoundaryEstimate) {
    //         return myBoundary.distanceTo2D(other->myBoundary);
    //     }
    //     if (isTazConnector()) {
    //         if (other->isTazConnector()) {
    //             return myBoundary.distanceTo2D(other->myBoundary);
    //         }
    //         return myBoundary.distanceTo2D(other->getFromJunction()->getPosition());
    //     }
    //     if (other->isTazConnector()) {
    //         return other->myBoundary.distanceTo2D(getToJunction()->getPosition());
    //     }
    //     return getLanes()[0]->getShape()[-1].distanceTo2D(other->getLanes()[0]->getShape()[0]);
    //     //return getToJunction()->getPosition().distanceTo2D(other->getFromJunction()->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<ROEdge>, doBoundaryEstimate: bool) -> f64 {
        // optimistic distance heuristic for use in routing
        if doBoundaryEstimate {
            return self.myBoundary.distance_to2d_boundary(&other.myBoundary);
        }
        if self.is_taz_connector() {
            if other.is_taz_connector() {
                return self.myBoundary.distance_to2d_boundary(&other.myBoundary);
            }
            return self.myBoundary.distance_to2d(other.get_from_junction().get_position());
        }
        if other.is_taz_connector() {
            return other.myBoundary.distance_to2d(self.get_to_junction().get_position());
        }
        self.get_lanes()[0].get_shape().at(-1).distance_to2d(other.get_lanes()[0].get_shape().at(0))
    }

    /// C++: `const ROEdgeVector & ROEdge::getAllEdges()` [static]
    /// Returns all ROEdges
    // const ROEdgeVector&
    // ROEdge::getAllEdges() {
    //     return myEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_all_edges() -> &'static ROEdgeVector {
        my_edges()
    }

    /// C++: `void ROEdge::setGlobalOptions(const bool)` [static]
    // static void setGlobalOptions(const bool interpolate) {
    //         myInterpolate = interpolate;
    //     }
    #[allow(unused_variables)]
    pub fn set_global_options(interpolate: bool) {
        unsafe { MY_INTERPOLATE = interpolate; }
    }

    /// C++: `void ROEdge::disableTimelineWarning()` [static]
    // static void disableTimelineWarning() {
    //         myHaveTTWarned = true;
    //     }
    #[allow(unused_variables)]
    pub fn disable_timeline_warning() {
        unsafe { MY_HAVE_TT_WARNED = true; }
    }

    /// C++: `const Position ROEdge::getStopPosition(const SUMOVehicleParameter::Stop &)` [static]
    /// return the coordinates of the center of the given stop
    // const Position
    // ROEdge::getStopPosition(const SUMOVehicleParameter::Stop& stop) {
    //     const double middle = (stop.endPos + stop.startPos) / 2.;
    //     const ROEdge* const edge = RONet::getInstance()->getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(stop.lane));
    //     return (edge->getFromJunction()->getPosition() + edge->getToJunction()->getPosition()) * (middle / edge->getLength());
    // }
    #[allow(unused_variables)]
    pub fn get_stop_position(stop: &SUMOVehicleParameter_Stop) -> Position {
        let middle = (stop.endPos + stop.startPos) / 2.;
        let edge = RONet::get_instance().get_edge(&utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&stop.lane));
        (*edge.get_from_junction().get_position() + *edge.get_to_junction().get_position()) * (middle / edge.get_length())
    }

    /// C++: `double ROEdge::getPreference(const SUMOVTypeParameter &) const` [const]
    /// return loaded edge preference based on routingType
    // inline double getPreference(const SUMOVTypeParameter& pars) const {
    //         return RONet::getInstance()->getPreference(getRoutingType(), pars);
    //     }
    #[allow(unused_variables)]
    pub fn get_preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        RONet::get_instance().get_preference(self.get_routing_type(), pars)
    }

    /// C++: `int ROEdge::getPriority() const` [const]
    /// get edge priority (road class)
    // int getPriority() const {
    //         return myPriority;
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        self.myPriority
    }

    /// C++: `const std::string & ROEdge::getType() const` [const]
    /// get edge type
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        &self.myType
    }

    /// C++: `const std::string & ROEdge::getRoutingType() const` [const]
    // const std::string& getRoutingType() const {
    //         return myRoutingType.empty() ? myType : myRoutingType;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_type(&self) -> &str {
        if self.myRoutingType.is_empty() {
            &self.myType
        } else {
            &self.myRoutingType
        }
    }

    /// C++: `const RONode * ROEdge::getFromJunction() const` [const]
    // const RONode* getFromJunction() const {
    //         return myFromJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<RONode> {
        self.myFromJunction
    }

    /// C++: `const RONode * ROEdge::getToJunction() const` [const]
    // const RONode* getToJunction() const {
    //         return myToJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<RONode> {
        self.myToJunction
    }

    /// C++: `const std::vector<ROLane *> & ROEdge::getLanes() const` [const]
    /// Returns this edge's lanes
    // const std::vector<ROLane*>& getLanes() const {
    //         return myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&self) -> &Vec<Ptr<ROLane>> {
        &self.myLanes
    }

    /// C++: `const ROEdge * ROEdge::getBidiEdge() const` [const]
    /// return opposite superposable/congruent edge, if it exist and 0 else
    // inline const ROEdge* getBidiEdge() const {
    //         return myBidiEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<ROEdge> {
        self.myBidiEdge
    }

    /// C++: `void ROEdge::setBidiEdge(const ROEdge *)`
    /// set opposite superposable/congruent edge
    // inline void setBidiEdge(const ROEdge* bidiEdge) {
    //         myBidiEdge = bidiEdge;
    //     }
    #[allow(unused_variables)]
    pub fn set_bidi_edge(&mut self, bidiEdge: Ptr<ROEdge>) {
        self.myBidiEdge = bidiEdge;
    }

    /// C++: `ReversedEdge<ROEdge, ROVehicle> * ROEdge::getReversedRoutingEdge() const` [const]
    // ReversedEdge<ROEdge, ROVehicle>* getReversedRoutingEdge() const {
    //         if (myReversedRoutingEdge == nullptr) {
    //             myReversedRoutingEdge = new ReversedEdge<ROEdge, ROVehicle>(this);
    //         }
    //         return myReversedRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_reversed_routing_edge(&self) -> Ptr<ReversedEdge<ROEdge, ROVehicle>> {
        let me = as_mut_edge(self);
        if me.myReversedRoutingEdge.is_null() {
            me.myReversedRoutingEdge = Ptr::from_raw(Box::into_raw(Box::new(ReversedEdge::new(Ptr::new(self)))));
        }
        me.myReversedRoutingEdge
    }

    /// C++: `FlippedEdge<ROEdge, RONode, ROVehicle> * ROEdge::getFlippedRoutingEdge() const` [const]
    // FlippedEdge<ROEdge, RONode, ROVehicle>* getFlippedRoutingEdge() const {
    //         if (myFlippedRoutingEdge == nullptr) {
    //             myFlippedRoutingEdge = new FlippedEdge<ROEdge, RONode, ROVehicle>(this);
    //         }
    //         return myFlippedRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_flipped_routing_edge(&self) -> Ptr<FlippedEdge<ROEdge, RONode, ROVehicle>> {
        let me = as_mut_edge(self);
        if me.myFlippedRoutingEdge.is_null() {
            me.myFlippedRoutingEdge = Ptr::from_raw(Box::into_raw(Box::new(FlippedEdge::new(Ptr::new(self)))));
        }
        me.myFlippedRoutingEdge
    }

    /// C++: `RailEdge<ROEdge, ROVehicle> * ROEdge::getRailwayRoutingEdge() const` [const]
    // RailEdge<ROEdge, ROVehicle>* getRailwayRoutingEdge() const {
    //         if (myRailwayRoutingEdge == nullptr) {
    //             myRailwayRoutingEdge = new RailEdge<ROEdge, ROVehicle>(this);
    //         }
    //         return myRailwayRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_railway_routing_edge(&self) -> Ptr<RailEdge<ROEdge, ROVehicle>> {
        let me = as_mut_edge(self);
        if me.myRailwayRoutingEdge.is_null() {
            me.myRailwayRoutingEdge = Ptr::from_raw(Box::into_raw(Box::new(RailEdge::new(Ptr::new(self)))));
        }
        me.myRailwayRoutingEdge
    }

    /// C++: `bool ROEdge::hasStoredEffort() const` [const]
    /// whether effort data was loaded for this edge
    // bool hasStoredEffort() const {
    //         return myUsingETimeLine;
    //     }
    #[allow(unused_variables)]
    pub fn has_stored_effort(&self) -> bool {
        self.myUsingETimeLine
    }

    /// C++: `bool ROEdge::initPriorityFactor(double)` [static]
    /// initialize priority factor range
    // bool
    // ROEdge::initPriorityFactor(double priorityFactor) {
    //     myPriorityFactor = priorityFactor;
    //     double maxEdgePriority = -std::numeric_limits<double>::max();
    //     for (ROEdge* edge : myEdges) {
    //         maxEdgePriority = MAX2(maxEdgePriority, (double)edge->getPriority());
    //         myMinEdgePriority = MIN2(myMinEdgePriority, (double)edge->getPriority());
    //     }
    //     myEdgePriorityRange = maxEdgePriority - myMinEdgePriority;
    //     if (myEdgePriorityRange == 0) {
    //         WRITE_WARNING(TL("Option weights.priority-factor does not take effect because all edges have the same priority."));
    //         myPriorityFactor = 0;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn init_priority_factor(priorityFactor: f64) -> bool {
        unsafe { MY_PRIORITY_FACTOR = priorityFactor; }
        let mut maxEdgePriority = -f64::MAX;
        for edge in my_edges().iter() {
            maxEdgePriority = max2(maxEdgePriority, edge.get_priority() as f64);
            unsafe { MY_MIN_EDGE_PRIORITY = min2(MY_MIN_EDGE_PRIORITY, edge.get_priority() as f64); }
        }
        unsafe { MY_EDGE_PRIORITY_RANGE = maxEdgePriority - MY_MIN_EDGE_PRIORITY; }
        if unsafe { MY_EDGE_PRIORITY_RANGE } == 0. {
            utils::common::msg_handler::write_warning("Option weights.priority-factor does not take effect because all edges have the same priority.");
            unsafe { MY_PRIORITY_FACTOR = 0.; }
            return false;
        }
        true
    }

    /// C++: `bool ROEdge::getStoredEffort(double, double &) const` [const, protected]
    /// Retrieves the stored effort
    // bool
    // ROEdge::getStoredEffort(double time, double& ret) const {
    //     if (myUsingETimeLine) {
    //         if (!myEfforts.describesTime(time)) {
    //             if (!myHaveEWarned) {
    //                 WRITE_WARNINGF(TL("No interval matches passed time=% in edge '%'.\n Using edge's length / edge's speed."), time, myID);
    //                 myHaveEWarned = true;
    //             }
    //             return false;
    //         }
    //         if (myInterpolate) {
    //             const double inTT = myTravelTimes.getValue(time);
    //             const double ratio = (myEfforts.getSplitTime(time, time + inTT) - time) / inTT;
    //             if (ratio >= 0.) {
    //                 ret = ratio * myEfforts.getValue(time) + (1. - ratio) * myEfforts.getValue(time + inTT);
    //                 return true;
    //             }
    //         }
    //         ret = myEfforts.getValue(time);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn get_stored_effort_2(&self, time: f64, ret: &mut f64) -> bool {
        if self.myUsingETimeLine {
            if !self.myEfforts.describes_time(time) {
                if unsafe { !MY_HAVE_E_WARNED } {
                    write_warningf!("No interval matches passed time=% in edge '%'.\n Using edge's length / edge's speed.", time, self.get_id());
                    unsafe { MY_HAVE_E_WARNED = true; }
                }
                return false;
            }
            if unsafe { MY_INTERPOLATE } {
                let inTT = self.myTravelTimes.get_value(time);
                let ratio = (self.myEfforts.get_split_time(time, time + inTT) - time) / inTT;
                if ratio >= 0. {
                    *ret = ratio * self.myEfforts.get_value(time) + (1. - ratio) * self.myEfforts.get_value(time + inTT);
                    return true;
                }
            }
            *ret = self.myEfforts.get_value(time);
            return true;
        }
        false
    }

    /// C++: `void ROEdge::ROEdge(const ROEdge &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(src: &ROEdge) -> Self {
        // C++ declares this copy constructor private and never defines it, so it can not be called
        panic!("ROEdge copy constructor is private and not defined in C++");
    }

    /// C++: `ROEdge & ROEdge::operator=(const ROEdge &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROEdge) -> &mut ROEdge {
        // C++ declares this assignment operator private and never defines it, so it can not be called
        panic!("ROEdge assignment operator is private and not defined in C++");
    }

}

