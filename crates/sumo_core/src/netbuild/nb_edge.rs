//! Port of netbuild/NBEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MainDirections::Direction`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MainDirections_Direction {
    RIGHTMOST = 0,
    LEFTMOST = 1,
    FORWARD = 2,
}

/// C++ enum `NBEdge::EdgeBuildingStep`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NBEdge_EdgeBuildingStep {
    INIT_REJECT_CONNECTIONS = 0,
    INIT = 1,
    EDGE2EDGES = 2,
    LANES2EDGES = 3,
    LANES2LANES_RECHECK = 4,
    LANES2LANES_DONE = 5,
    LANES2LANES_USER = 6,
}

/// C++ enum `NBEdge::JunctionPriority`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NBEdge_JunctionPriority {
    MINOR_ROAD = 0,
    PRIORITY_ROAD = 1,
    ROUNDABOUT = 1000,
}

/// C++ enum `NBEdge::Lane2LaneInfoType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NBEdge_Lane2LaneInfoType {
    COMPUTED = 0,
    USER = 1,
    VALIDATED = 2,
}

// C++ static constants of NBEdge (NBEdge.cpp / NBEdge.h)
pub const UNSPECIFIED_WIDTH: f64 = -1.0;
pub const UNSPECIFIED_OFFSET: f64 = 0.0;
pub const UNSPECIFIED_SPEED: f64 = -1.0;
pub const UNSPECIFIED_FRICTION: f64 = 1.0;
pub const UNSPECIFIED_CONTPOS: f64 = -1.0;
pub const UNSPECIFIED_VISIBILITY_DISTANCE: f64 = -1.0;
pub const UNSPECIFIED_SIGNAL_OFFSET: f64 = -1.0;
pub const UNSPECIFIED_LOADED_LENGTH: f64 = -1.0;
pub const ANGLE_LOOKAHEAD: f64 = 10.0;
pub const UNSPECIFIED_INTERNAL_LANE_INDEX: i32 = -1;
pub const UNSPECIFIED_CONNECTION_UNCONTROLLED: bool = false;
// C++ `static double NBEdge::myDefaultConnectionLength`, stored as bits so it is a plain global like the C++ static
static MY_DEFAULT_CONNECTION_LENGTH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(UNSPECIFIED_LOADED_LENGTH.to_bits());
fn my_default_connection_length() -> f64 {
    f64::from_bits(MY_DEFAULT_CONNECTION_LENGTH.load(std::sync::atomic::Ordering::Relaxed))
}
// C++ `static ConstRouterEdgePairVector Connection::myViaSuccessors` (always empty)
struct ViaSuccessorsStatic(ConstRouterEdgePairVector);
// determinism: the static is never mutated, so sharing it across threads is safe
unsafe impl Sync for ViaSuccessorsStatic {}
static MY_VIA_SUCCESSORS: ViaSuccessorsStatic = ViaSuccessorsStatic(Vec::new());

/// A structure which describes a connection between edges or lanes
/// C++ class `NBEdge::Connection` (netbuild/NBEdge.h)
pub struct NBEdge_Connection {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// C++ base class `NBRouterEdge`
    pub base2: NBRouterEdge,
    /// The lane the connections starts at
    pub fromLane: i32,
    /// The edge the connections yields in
    pub toEdge: Ptr<NBEdge>,
    /// The lane the connections yields in
    pub toLane: i32,
    /// The id of the traffic light that controls this connection
    pub tlID: String,
    /// The index of this connection within the controlling traffic light
    pub tlLinkIndex: i32,
    /// The index of the internal junction within the controlling traffic light (optional)
    pub tlLinkIndex2: i32,
    /// Information about being definitely free to drive (on-ramps)
    pub mayDefinitelyPass: bool,
    /// whether the junction must be kept clear when using this connection
    pub keepClear: KeepClear,
    /// custom position for internal junction on this connection
    pub contPos: f64,
    /// custom foe visiblity for connection
    pub visibility: f64,
    /// custom speed for connection
    pub speed: f64,
    pub friction: f64,
    /// custom length for connection
    pub customLength: f64,
    /// custom shape for connection
    pub customShape: PositionVector,
    /// List of vehicle types that are allowed on this connection
    pub permissions: SVCPermissions,
    /// List of vehicle types that are allowed to change Left from this connections internal lane(s)
    pub changeLeft: SVCPermissions,
    /// List of vehicle types that are allowed to change right from this connections internal lane(s)
    pub changeRight: SVCPermissions,
    /// Whether this connection is an indirect left turn
    pub indirectLeft: bool,
    /// optional type of Connection
    pub edgeType: String,
    /// id of Connection
    pub id: String,
    /// shape of Connection
    pub shape: PositionVector,
    /// maximum velocity
    pub vmax: f64,
    /// check if Connection have a Via
    pub haveVia: bool,
    /// if Connection have a via, ID of it
    pub viaID: String,
    /// shape of via
    pub viaShape: PositionVector,
    /// the length of the via shape (maybe customized)
    pub viaLength: f64,
    /// FOE Internal links
    pub foeInternalLinks: Vec<i32>,
    /// FOE Incomings lanes
    pub foeIncomingLanes: Vec<String>,
    /// The lane index of this internal lane within the internal edge
    pub internalLaneIndex: i32,
    pub internalViaLaneIndex: i32,
    /// check if Connection is uncontrolled
    pub uncontrolled: bool,
    /// computed length (average of all internal lane shape lengths that share an internal edge)
    pub length: f64,
}

impl std::ops::Deref for NBEdge_Connection {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NBEdge_Connection {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of Connection (port as module-level statics, see AGENTS.md):
//   static ConstRouterEdgePairVector Connection::myViaSuccessors;

impl NBEdge_Connection {
    /// C++: `void Connection::Connection(int, NBEdge *, int, const bool)`
    /// Constructor
    // NBEdge::Connection::Connection(int fromLane_, NBEdge* toEdge_, int toLane_, const bool mayDefinitelyPass_) :
    //     fromLane(fromLane_),
    //     toEdge(toEdge_),
    //     toLane(toLane_),
    //     mayDefinitelyPass(mayDefinitelyPass_),
    //     customLength(myDefaultConnectionLength),
    //     id(toEdge_ == nullptr ? "" : toEdge->getFromNode()->getID()) {
    // }
    #[allow(unused_variables)]
    pub fn new(fromLane_: i32, toEdge_: Ptr<NBEdge>, toLane_: i32, mayDefinitelyPass_: bool) -> Self {
        let id = if toEdge_.is_null() { String::new() } else { toEdge_.get_from_node().get_id().to_string() };
        NBEdge_Connection {
            base: Parameterised::new(),
            base2: NBRouterEdge {},
            fromLane: fromLane_,
            toEdge: toEdge_,
            toLane: toLane_,
            tlID: String::new(),
            tlLinkIndex: -1,
            tlLinkIndex2: -1,
            mayDefinitelyPass: mayDefinitelyPass_,
            keepClear: KeepClear::KEEPCLEAR_UNSPECIFIED,
            contPos: UNSPECIFIED_CONTPOS,
            visibility: UNSPECIFIED_VISIBILITY_DISTANCE,
            speed: UNSPECIFIED_SPEED,
            friction: UNSPECIFIED_FRICTION,
            customLength: my_default_connection_length(),
            customShape: PositionVector::default(),
            permissions: SVC_UNSPECIFIED,
            changeLeft: SVC_UNSPECIFIED,
            changeRight: SVC_UNSPECIFIED,
            indirectLeft: false,
            edgeType: String::new(),
            id,
            shape: PositionVector::default(),
            vmax: UNSPECIFIED_SPEED,
            haveVia: false,
            viaID: String::new(),
            viaShape: PositionVector::default(),
            viaLength: 0.0,
            foeInternalLinks: Vec::new(),
            foeIncomingLanes: Vec::new(),
            internalLaneIndex: UNSPECIFIED_INTERNAL_LANE_INDEX,
            internalViaLaneIndex: 0,
            uncontrolled: false,
            length: UNSPECIFIED_LOADED_LENGTH,
        }
    }

    /// C++: `std::string Connection::getInternalLaneID() const` [const]
    /// get ID of internal lane
    // std::string
    // NBEdge::Connection::getInternalLaneID() const {
    //     return id + "_" + toString(internalLaneIndex);
    // }
    #[allow(unused_variables)]
    pub fn get_internal_lane_id(&self) -> String {
        format!("{}_{}", self.id, self.internalLaneIndex)
    }

    /// C++: `std::string Connection::getInternalViaLaneID() const` [const]
    /// get ID of internal lane (second part)
    // std::string
    // NBEdge::Connection::getInternalViaLaneID() const {
    //     return viaID + "_" + toString(internalViaLaneIndex);
    // }
    #[allow(unused_variables)]
    pub fn get_internal_via_lane_id(&self) -> String {
        format!("{}_{}", self.viaID, self.internalViaLaneIndex)
    }

    /// C++: `std::string Connection::getDescription(const NBEdge *) const` [const]
    /// get string describing this connection
    // std::string
    // NBEdge::Connection::getDescription(const NBEdge* parent) const {
    //     return (Named::getIDSecure(parent) + "_" + toString(fromLane) + "->" + Named::getIDSecure(toEdge) + "_" + toString(toLane)
    //             + (permissions == SVC_UNSPECIFIED ? "" : " (" + getVehicleClassNames(permissions) + ")"));
    // }
    #[allow(unused_variables)]
    pub fn get_description(&self, parent: Ptr<NBEdge>) -> String {
        let mut s = format!(
            "{}_{}->{}_{}",
            Named::get_id_secure(parent.get().map(|e| &e.base), "NULL"),
            self.fromLane,
            Named::get_id_secure(self.toEdge.get().map(|e| &e.base), "NULL"),
            self.toLane
        );
        if self.permissions != SVC_UNSPECIFIED {
            s += &format!(" ({})", get_vehicle_class_names(self.permissions, false));
        }
        s
    }

    /// C++: `const std::string & Connection::getID() const` [const, virtual]
    // const std::string& getID() const {
    //             return id;
    //         }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// C++: `double Connection::getSpeed() const` [const, virtual]
    // double getSpeed() const {
    //             return vmax;
    //         }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        self.vmax
    }

    /// C++: `double Connection::getLength() const` [const, virtual]
    // double getLength() const {
    //             return shape.length() + viaShape.length();
    //         }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.shape.length() + self.viaShape.length()
    }

    /// C++: `int Connection::getNumericalID() const` [const, virtual]
    // int getNumericalID() const {
    //             throw ProcessError("NBEdge::Connection does not implement getNumericalID()");
    //         }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        panic!("{}", ProcessError::new("NBEdge::Connection does not implement getNumericalID()"))
    }

    /// C++: `const Connection * Connection::getBidiEdge() const` [const, virtual]
    // const Connection* getBidiEdge() const {
    //             return nullptr;
    //         }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<NBEdge_Connection> {
        Ptr::null()
    }

    /// C++: `bool Connection::isInternal() const` [const, virtual]
    // bool isInternal() const {
    //             return true;
    //         }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        true
    }

    /// C++: `const ConstRouterEdgePairVector & Connection::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual]
    // const ConstRouterEdgePairVector& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //             UNUSED_PARAMETER(vClass);
    //             UNUSED_PARAMETER(ignoreTransientPermissions);
    //             return myViaSuccessors;
    //         }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ConstRouterEdgePairVector {
        &MY_VIA_SUCCESSORS.0
    }

}

/// An (internal) definition of a single lane of an edge
/// C++ class `NBEdge::Lane` (netbuild/NBEdge.h)
pub struct NBEdge_Lane {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// The lane's shape
    pub shape: PositionVector,
    /// The speed allowed on this lane
    pub speed: f64,
    /// The friction on this lane
    pub friction: f64,
    /// List of vehicle types that are allowed on this lane
    pub permissions: SVCPermissions,
    /// List of vehicle types that are preferred on this lane
    pub preferred: SVCPermissions,
    /// List of vehicle types that are allowed to change Left from this lane
    pub changeLeft: SVCPermissions,
    /// List of vehicle types that are allowed to change right from this lane
    pub changeRight: SVCPermissions,
    /// This lane's offset to the intersection begin
    pub endOffset: f64,
    /// stopOffsets.second - The stop offset for vehicles stopping at the lane's end. Applies if vClass is in in stopOffset.first bitset
    pub laneStopOffset: StopOffset,
    /// This lane's width
    pub width: f64,
    /// An opposite lane ID, if given
    pub oppositeID: String,
    /// Whether this lane is an acceleration lane
    pub accelRamp: bool,
    pub connectionsDone: bool,
    /// A custom shape for this lane set by the user
    pub customShape: PositionVector,
    /// the type of this lane
    pub type_: String,
    /// turning signs printed on the road, bitset of LinkDirection (imported from OSM)
    pub turnSigns: i32,
}

impl std::ops::Deref for NBEdge_Lane {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for NBEdge_Lane {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl NBEdge_Lane {
    /// C++: `void Lane::Lane(NBEdge *, const std::string &)`
    /// constructor
    // NBEdge::Lane::Lane(NBEdge* e, const std::string& origID_) :
    //     speed(e->getSpeed()),
    //     friction(e->getFriction()),
    //     permissions(SVCAll),
    //     preferred(0),
    //     changeLeft(SVCAll),
    //     changeRight(SVCAll),
    //     endOffset(e->getEndOffset()),
    //     laneStopOffset(e->getEdgeStopOffset()),
    //     width(e->getLaneWidth()),
    //     accelRamp(false),
    //     connectionsDone(false) {
    //     if (origID_ != "") {
    //         setParameter(SUMO_PARAM_ORIGID, origID_);
    //     }
    // }
    pub fn new(e: Ptr<NBEdge>, origID: &str) -> Self {
        let mut lane = NBEdge_Lane {
            base: Parameterised::new(),
            shape: PositionVector::default(),
            speed: e.get_speed(),
            friction: e.get_friction(),
            permissions: SVC_ALL,
            preferred: 0,
            changeLeft: SVC_ALL,
            changeRight: SVC_ALL,
            endOffset: e.get_end_offset(),
            laneStopOffset: *e.get_edge_stop_offset(),
            width: e.get_lane_width(),
            oppositeID: String::new(),
            accelRamp: false,
            connectionsDone: false,
            customShape: PositionVector::default(),
            type_: String::new(),
            turnSigns: 0,
        };
        if !origID.is_empty() {
            lane.set_parameter(SUMO_PARAM_ORIGID, origID);
        }
        lane
    }

}

/// Holds (- relative to the edge it is build from -!!!) the list of main directions a vehicle that drives on this street may take on the junction the edge ends in The back direction is not regarded
/// C++ class `NBEdge::MainDirections` (netbuild/NBEdge.h)
pub struct NBEdge_MainDirections {
    /// the index of the straightmost among the given outgoing edges
    pub myStraightest: i32,
    /// list of the main direction within the following junction relative to the edge
    pub myDirs: Vec<MainDirections_Direction>,
}

impl NBEdge_MainDirections {
    /// C++: `void MainDirections::MainDirections(const EdgeVector &, NBEdge *, NBNode *, const std::vector<int> &)`
    /// constructor
    // NBEdge::MainDirections::MainDirections(const EdgeVector& outgoing, NBEdge* parent, NBNode* to, const std::vector<int>& availableLanes) : myStraightest(-1) {
    //     NBContHelper::edge_similar_direction_sorter sorter(parent);
    //     const NBEdge* straight = nullptr;
    //     for (const NBEdge* const out : outgoing) {
    //         const SVCPermissions outPerms = out->getPermissions();
    //         for (const int l : availableLanes) {
    //             if ((parent->myLanes[l].permissions & outPerms) != 0) {
    //                 if (straight == nullptr || sorter(out, straight)) {
    //                     straight = out;
    //                 }
    //                 break;
    //             }
    //         }
    //     }
    //     if (straight == nullptr) {
    //         return;
    //     }
    //     myStraightest = (int)std::distance(outgoing.begin(), std::find(outgoing.begin(), outgoing.end(), straight));
    //
    //     // check whether the right turn has a higher priority
    //     assert(outgoing.size() > 0);
    //     const LinkDirection straightestDir = to->getDirection(parent, straight);
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND2(parent)) {
    //         std::cout << " MainDirections edge=" << parent->getID() << " straightest=" << straight->getID() << " dir=" << toString(straightestDir) << "\n";
    //     }
    // #endif
    //     if (NBNode::isTrafficLight(to->getType()) &&
    //             (straightestDir == LinkDirection::STRAIGHT || straightestDir == LinkDirection::PARTLEFT || straightestDir == LinkDirection::PARTRIGHT)) {
    //         myDirs.push_back(MainDirections::Direction::FORWARD);
    //         return;
    //     }
    //     if (outgoing[0]->getJunctionPriority(to) == 1) {
    //         myDirs.push_back(MainDirections::Direction::RIGHTMOST);
    //     }
    //     // check whether the left turn has a higher priority
    //     if (outgoing.back()->getJunctionPriority(to) == 1) {
    //         // ok, the left turn belongs to the higher priorised edges on the junction
    //         //  let's check, whether it has also a higher priority (lane number/speed)
    //         //  than the current
    //         if (outgoing.back()->getPriority() > straight->getPriority() ||
    //                 outgoing.back()->getNumLanes() > straight->getNumLanes()) {
    //             myDirs.push_back(MainDirections::Direction::LEFTMOST);
    //         }
    //     }
    //     // check whether the forward direction has a higher priority
    //     // check whether it has a higher priority and is going straight
    //     if (straight->getJunctionPriority(to) == 1 && to->getDirection(parent, straight) == LinkDirection::STRAIGHT) {
    //         myDirs.push_back(MainDirections::Direction::FORWARD);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(outgoing: &EdgeVector, parent: Ptr<NBEdge>, to: Ptr<NBNode>, availableLanes: &Vec<i32>) -> Self {
        let mut me = NBEdge_MainDirections { myStraightest: -1, myDirs: Vec::new() };
        let sorter = NBContHelper_edge_similar_direction_sorter::new(parent, true);
        let mut straight: Ptr<NBEdge> = Ptr::null();
        for out in outgoing {
            let outPerms = out.get_permissions(-1);
            for l in availableLanes {
                if (parent.myLanes[*l as usize].permissions & outPerms) != 0 {
                    if straight.is_null() || sorter.call_op(*out, straight) != 0 {
                        straight = *out;
                    }
                    break;
                }
            }
        }
        if straight.is_null() {
            return me;
        }
        me.myStraightest = outgoing.iter().position(|e| *e == straight).map_or(outgoing.len() as i32, |p| p as i32);

        // check whether the right turn has a higher priority
        assert!(outgoing.len() > 0);
        let straightestDir = to.get_direction(parent, straight, false);
        if NBNode::is_traffic_light(to.get_type())
            && (straightestDir == LinkDirection::STRAIGHT || straightestDir == LinkDirection::PARTLEFT || straightestDir == LinkDirection::PARTRIGHT)
        {
            me.myDirs.push(MainDirections_Direction::FORWARD);
            return me;
        }
        if outgoing[0].get_junction_priority(to) == 1 {
            me.myDirs.push(MainDirections_Direction::RIGHTMOST);
        }
        // check whether the left turn has a higher priority
        let back = outgoing[outgoing.len() - 1];
        if back.get_junction_priority(to) == 1 {
            // ok, the left turn belongs to the higher priorised edges on the junction
            //  let's check, whether it has also a higher priority (lane number/speed)
            //  than the current
            if back.get_priority() > straight.get_priority() || back.get_num_lanes() > straight.get_num_lanes() {
                me.myDirs.push(MainDirections_Direction::LEFTMOST);
            }
        }
        // check whether the forward direction has a higher priority
        // check whether it has a higher priority and is going straight
        if straight.get_junction_priority(to) == 1 && to.get_direction(parent, straight, false) == LinkDirection::STRAIGHT {
            me.myDirs.push(MainDirections_Direction::FORWARD);
        }
        me
    }

    /// C++: `void MainDirections::~MainDirections()`
    /// destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: the members are plain values, nothing to release
    }

    /// C++: `int MainDirections::getStraightest() const` [const]
    /// returns the index of the straightmost among the given outgoing edges
    // int getStraightest() const {
    //             return myStraightest;
    //         }
    #[allow(unused_variables)]
    pub fn get_straightest(&self) -> i32 {
        self.myStraightest
    }

    /// C++: `bool MainDirections::empty() const` [const]
    /// returns the information whether no following street has a higher priority
    // bool
    // NBEdge::MainDirections::empty() const {
    //     return myDirs.empty();
    // }
    #[allow(unused_variables)]
    pub fn empty(&self) -> bool {
        self.myDirs.is_empty()
    }

    /// C++: `bool MainDirections::includes(Direction) const` [const]
    /// returns the information whether the street in the given direction has a higher priority
    // bool
    // NBEdge::MainDirections::includes(Direction d) const {
    //     return std::find(myDirs.begin(), myDirs.end(), d) != myDirs.end();
    // }
    #[allow(unused_variables)]
    pub fn includes(&self, d: MainDirections_Direction) -> bool {
        self.myDirs.contains(&d)
    }

    /// C++: `void MainDirections::MainDirections(const MainDirections &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NBEdge_MainDirections) -> Self {
        panic!("MainDirections copy constructor is invalidated in C++ and must not be used")
    }

    /// C++: `MainDirections & MainDirections::operator=(const MainDirections &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBEdge_MainDirections) -> &mut NBEdge_MainDirections {
        panic!("MainDirections assignment operator is invalidated in C++ and must not be used")
    }

}

/// The representation of a single edge during network building
/// C++ class `NBEdge` (netbuild/NBEdge.h)
pub struct NBEdge {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// C++ base class `NBRouterEdge`
    pub base3: NBRouterEdge,
    /// The building step
    pub myStep: NBEdge_EdgeBuildingStep,
    /// The type of the edge
    pub myType: String,
    /// The source and the destination node
    pub myFrom: Ptr<NBNode>,
    /// The source and the destination node
    pub myTo: Ptr<NBNode>,
    /// node for which turnSign information applies
    pub myTurnSignTarget: String,
    /// The length of the edge
    pub myLength: f64,
    /// The angles of the edge @{
    pub myStartAngle: f64,
    pub myEndAngle: f64,
    pub myTotalAngle: f64,
    /// The priority of the edge
    pub myPriority: i32,
    /// The maximal speed
    pub mySpeed: f64,
    /// The current friction
    pub myFriction: f64,
    /// The mileage/kilometrage at the start of this edge in a linear coordination system
    pub myDistance: f64,
    /// List of connections to following edges
    pub myConnections: Vec<NBEdge_Connection>,
    /// List of connections marked for delayed removal
    pub myConnectionsToDelete: Vec<NBEdge_Connection>,
    /// The turn destination edge (if a connection exists)
    pub myTurnDestination: Ptr<NBEdge>,
    /// The edge that would be the turn destination if there was one
    pub myPossibleTurnDestination: Ptr<NBEdge>,
    /// The priority normalised for the node the edge is outgoing of
    pub myFromJunctionPriority: i32,
    /// The priority normalised for the node the edge is incoming in
    pub myToJunctionPriority: i32,
    /// The geometry for the edge
    pub myGeom: PositionVector,
    /// The information about how to spread the lanes
    pub myLaneSpreadFunction: LaneSpreadFunction,
    /// This edges's offset to the intersection begin (will be applied to all lanes)
    pub myEndOffset: f64,
    /// A vClass specific stop offset - assumed of length 0 (unspecified) or 1. For the latter case the int is a bit set specifying the vClasses, the offset applies to (see SUMOVehicleClass.h), and the double is the stopping offset in meters from the lane end
    pub myEdgeStopOffset: StopOffset,
    /// This width of this edge's lanes
    pub myLaneWidth: f64,
    /// Lane information
    pub myLanes: Vec<NBEdge_Lane>,
    /// An optional length to use (-1 if not valid)
    pub myLoadedLength: f64,
    /// Information whether this is lies within a joined tls
    pub myAmInTLS: bool,
    /// Information whether this edge is a (macroscopic) connector
    pub myAmMacroscopicConnector: bool,
    /// The street name (or whatever arbitrary string you wish to attach)
    pub myStreetName: String,
    /// the street signs along this edge
    pub mySigns: Vec<NBSign>,
    /// the position of a traffic light signal on this edge
    pub mySignalPosition: Position,
    pub mySignalNode: Ptr<NBNode>,
    /// intersection borders (because the node shape might be invalid) @{
    pub myFromBorder: PositionVector,
    pub myToBorder: PositionVector,
    /// whether this edge is an Off-Ramp or leads to one
    pub myIsOffRamp: bool,
    /// whether this edge is part of a non-rail bidi edge pair
    pub myIsBidi: bool,
    /// The routing type of the edge
    pub myRoutingType: String,
    /// the index of the edge in the list of all edges. Set by NBEdgeCont and requires re-set whenever the list of edges changes
    pub myIndex: i32,
    pub mySuccessors: EdgeVector,
    pub myViaSuccessors: ConstRouterEdgePairVector,
}

impl std::ops::Deref for NBEdge {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for NBEdge {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of NBEdge (port as module-level statics, see AGENTS.md):
//   static NBEdge NBEdge::DummyEdge;
//   static const double NBEdge::UNSPECIFIED_WIDTH;
//   static const double NBEdge::UNSPECIFIED_OFFSET;
//   static const double NBEdge::UNSPECIFIED_SPEED;
//   static const double NBEdge::UNSPECIFIED_FRICTION;
//   static const double NBEdge::UNSPECIFIED_CONTPOS;
//   static const double NBEdge::UNSPECIFIED_VISIBILITY_DISTANCE;
//   static const double NBEdge::UNSPECIFIED_LOADED_LENGTH;
//   static const double NBEdge::UNSPECIFIED_SIGNAL_OFFSET;
//   static const double NBEdge::ANGLE_LOOKAHEAD;
//   static const int NBEdge::UNSPECIFIED_INTERNAL_LANE_INDEX;
//   static const bool NBEdge::UNSPECIFIED_CONNECTION_UNCONTROLLED;
//   static const int NBEdge::TURN_SIGN_SHIFT_BUS;
//   static const int NBEdge::TURN_SIGN_SHIFT_TAXI;
//   static const int NBEdge::TURN_SIGN_SHIFT_BICYCLE;
//   static double NBEdge::myDefaultConnectionLength;

impl NBEdge {
    /// C++: `void NBEdge::setDefaultConnectionLength(double)` [static]
    // static void setDefaultConnectionLength(double length) {
    //         myDefaultConnectionLength = length;
    //     }
    #[allow(unused_variables)]
    pub fn set_default_connection_length(length: f64) {
        MY_DEFAULT_CONNECTION_LENGTH.store(length.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }

    /// C++: `void NBEdge::NBEdge(const std::string &, NBNode *, NBNode *, std::string, double, double, int, int, double, double, LaneSpreadFunction, const std::string &)`
    /// Constructor
    // NBEdge::NBEdge(const std::string& id, NBNode* from, NBNode* to,
    //                std::string type, double speed, double friction, int nolanes,
    //                int priority, double laneWidth, double endOffset,
    //                LaneSpreadFunction spread, const std::string& streetName) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myStep(EdgeBuildingStep::INIT),
    //     myType(StringUtils::convertUmlaute(type)),
    //     myFrom(from), myTo(to),
    //     myStartAngle(0), myEndAngle(0), myTotalAngle(0),
    //     myPriority(priority), mySpeed(speed), myFriction(friction),
    //     myDistance(0),
    //     myTurnDestination(nullptr),
    //     myPossibleTurnDestination(nullptr),
    //     myFromJunctionPriority(-1), myToJunctionPriority(-1),
    //     myLaneSpreadFunction(spread), myEndOffset(endOffset),
    //     myLaneWidth(laneWidth),
    //     myLoadedLength(UNSPECIFIED_LOADED_LENGTH),
    //     myAmInTLS(false), myAmMacroscopicConnector(false),
    //     myStreetName(streetName),
    //     mySignalPosition(Position::INVALID),
    //     mySignalNode(nullptr),
    //     myIsOffRamp(false),
    //     myIsBidi(false),
    //     myIndex(-1) {
    //     init(nolanes, false, "");
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<NBNode>, to: Ptr<NBNode>, type_: String, speed: f64, friction: f64, nolanes: i32, priority: i32, width: f64, endOffset: f64, spread: LaneSpreadFunction, streetName: &str) -> Self {
        todo!("port NBEdge::NBEdge")
    }

    /// C++: `void NBEdge::NBEdge(const std::string &, NBNode *, NBNode *, std::string, double, double, int, int, double, double, PositionVector, LaneSpreadFunction, const std::string &, const std::string &, bool)`
    /// Constructor
    // NBEdge::NBEdge(const std::string& id, NBNode* from, NBNode* to,
    //                std::string type, double speed, double friction, int nolanes,
    //                int priority, double laneWidth, double endOffset,
    //                PositionVector geom,
    //                LaneSpreadFunction spread,
    //                const std::string& streetName,
    //                const std::string& origID,
    //                bool tryIgnoreNodePositions) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myStep(EdgeBuildingStep::INIT),
    //     myType(StringUtils::convertUmlaute(type)),
    //     myFrom(from), myTo(to),
    //     myStartAngle(0), myEndAngle(0), myTotalAngle(0),
    //     myPriority(priority), mySpeed(speed), myFriction(friction),
    //     myDistance(0),
    //     myTurnDestination(nullptr),
    //     myPossibleTurnDestination(nullptr),
    //     myFromJunctionPriority(-1), myToJunctionPriority(-1),
    //     myGeom(geom), myLaneSpreadFunction(spread), myEndOffset(endOffset),
    //     myLaneWidth(laneWidth),
    //     myLoadedLength(UNSPECIFIED_LOADED_LENGTH),
    //     myAmInTLS(false), myAmMacroscopicConnector(false),
    //     myStreetName(streetName),
    //     mySignalPosition(Position::INVALID),
    //     mySignalNode(nullptr),
    //     myIsOffRamp(false),
    //     myIsBidi(false),
    //     myIndex(-1) {
    //     init(nolanes, tryIgnoreNodePositions, origID);
    // }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, from: Ptr<NBNode>, to: Ptr<NBNode>, type_: String, speed: f64, friction: f64, nolanes: i32, priority: i32, width: f64, endOffset: f64, geom: PositionVector, spread: LaneSpreadFunction, streetName: &str, origID: &str, tryIgnoreNodePositions: bool) -> Self {
        todo!("port NBEdge::NBEdge")
    }

    /// C++: `void NBEdge::NBEdge(const std::string &, NBNode *, NBNode *, const NBEdge *, const PositionVector &, int)`
    /// Constructor
    // NBEdge::NBEdge(const std::string& id, NBNode* from, NBNode* to, const NBEdge* tpl, const PositionVector& geom, int numLanes) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myStep(EdgeBuildingStep::INIT),
    //     myType(tpl->getTypeID()),
    //     myFrom(from), myTo(to),
    //     myStartAngle(0), myEndAngle(0), myTotalAngle(0),
    //     myPriority(tpl->getPriority()), mySpeed(tpl->getSpeed()),
    //     myFriction(tpl->getFriction()),
    //     myDistance(0),
    //     myTurnDestination(nullptr),
    //     myPossibleTurnDestination(nullptr),
    //     myFromJunctionPriority(-1), myToJunctionPriority(-1),
    //     myGeom(geom),
    //     myLaneSpreadFunction(tpl->getLaneSpreadFunction()),
    //     myEndOffset(tpl->getEndOffset()),
    //     myEdgeStopOffset(tpl->getEdgeStopOffset()),
    //     myLaneWidth(tpl->getLaneWidth()),
    //     myLoadedLength(UNSPECIFIED_LOADED_LENGTH),
    //     myAmInTLS(false),
    //     myAmMacroscopicConnector(false),
    //     myStreetName(tpl->getStreetName()),
    //     mySignalPosition(to == tpl->myTo ? tpl->mySignalPosition : Position::INVALID),
    //     mySignalNode(to == tpl->myTo ? tpl->mySignalNode : nullptr),
    //     myIsOffRamp(false),
    //     myIsBidi(tpl->myIsBidi),
    //     myIndex(-1) {
    //     init(numLanes > 0 ? numLanes : tpl->getNumLanes(), myGeom.size() > 0, "");
    //     for (int i = 0; i < getNumLanes(); i++) {
    //         const int tplIndex = MIN2(i, tpl->getNumLanes() - 1);
    //         setSpeed(i, tpl->getLaneSpeed(tplIndex));
    //         setFriction(i, tpl->getLaneFriction(tplIndex));
    //         setPermissions(tpl->getPermissions(tplIndex), i);
    //         setLaneWidth(i, tpl->myLanes[tplIndex].width);
    //         setLaneType(i, tpl->myLanes[tplIndex].type);
    //         myLanes[i].updateParameters(tpl->myLanes[tplIndex].getParametersMap());
    //         if (to == tpl->myTo) {
    //             setEndOffset(i, tpl->myLanes[tplIndex].endOffset);
    //             setEdgeStopOffset(i, tpl->myLanes[tplIndex].laneStopOffset);
    //         }
    //     }
    //     if (tpl->myLoadedLength > 0 && to == tpl->getFromNode() && from == tpl->getToNode() && geom == tpl->getGeometry().reverse()) {
    //         myLoadedLength = tpl->myLoadedLength;
    //     }
    //     updateParameters(tpl->getParametersMap());
    // }
    #[allow(unused_variables)]
    pub fn new_3(id: &str, from: Ptr<NBNode>, to: Ptr<NBNode>, tpl: Ptr<NBEdge>, geom: &PositionVector, numLanes: i32) -> Self {
        todo!("port NBEdge::NBEdge")
    }

    /// C++: `void NBEdge::~NBEdge() noexcept` [virtual]
    /// Destructor
    // NBEdge::~NBEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBEdge::~NBEdge")
    }

    /// C++: `void NBEdge::reinit(NBNode *, NBNode *, const std::string &, double, double, int, int, PositionVector, double, double, const std::string &, LaneSpreadFunction, bool)`
    /// Resets initial values
    // void
    // NBEdge::reinit(NBNode* from, NBNode* to, const std::string& type,
    //                double speed, double friction, int nolanes, int priority,
    //                PositionVector geom, double laneWidth, double endOffset,
    //                const std::string& streetName,
    //                LaneSpreadFunction spread,
    //                bool tryIgnoreNodePositions) {
    //     if (myFrom != from) {
    //         myFrom->removeEdge(this, false);
    //     }
    //     if (myTo != to) {
    //         myTo->removeEdge(this, false);
    //     }
    //     myType = StringUtils::convertUmlaute(type);
    //     myFrom = from;
    //     myTo = to;
    //     myPriority = priority;
    //     //?myTurnDestination(0),
    //     //?myFromJunctionPriority(-1), myToJunctionPriority(-1),
    //     myGeom = geom;
    //     myLaneSpreadFunction = spread;
    //     myLoadedLength = UNSPECIFIED_LOADED_LENGTH;
    //     myStreetName = streetName;
    //     //?, myAmTurningWithAngle(0), myAmTurningOf(0),
    //     //?myAmInTLS(false), myAmMacroscopicConnector(false)
    //
    //     // preserve lane-specific settings (geometry must be recomputed)
    //     // if new lanes are added they copy the values from the leftmost lane (if specified)
    //     const std::vector<Lane> oldLanes = myLanes;
    //     init(nolanes, tryIgnoreNodePositions, oldLanes.empty() ? "" : oldLanes[0].getParameter(SUMO_PARAM_ORIGID));
    //     for (int i = 0; i < (int)nolanes; ++i) {
    //         PositionVector newShape = myLanes[i].shape;
    //         myLanes[i] = oldLanes[MIN2(i, (int)oldLanes.size() - 1)];
    //         myLanes[i].shape = newShape;
    //     }
    //     // however, if the new edge defaults are explicityly given, they override the old settings
    //     if (endOffset != UNSPECIFIED_OFFSET) {
    //         setEndOffset(-1, endOffset);
    //     }
    //     if (laneWidth != UNSPECIFIED_WIDTH) {
    //         setLaneWidth(-1, laneWidth);
    //     }
    //     if (speed != UNSPECIFIED_SPEED) {
    //         setSpeed(-1, speed);
    //     }
    //     if (friction != UNSPECIFIED_FRICTION) {
    //         setFriction(-1, friction);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reinit(&mut self, from: Ptr<NBNode>, to: Ptr<NBNode>, type_: &str, speed: f64, friction: f64, nolanes: i32, priority: i32, geom: PositionVector, width: f64, endOffset: f64, streetName: &str, spread: LaneSpreadFunction, tryIgnoreNodePositions: bool) {
        todo!("port NBEdge::reinit")
    }

    /// C++: `void NBEdge::reinitNodes(NBNode *, NBNode *)`
    /// Resets nodes but keeps all other values the same (used when joining)
    // void
    // NBEdge::reinitNodes(NBNode* from, NBNode* to) {
    //     // connections may still be valid
    //     if (from == nullptr || to == nullptr) {
    //         throw ProcessError(TLF("At least one of edge's '%' nodes is not known.", myID));
    //     }
    //     if (myFrom != from) {
    //         myFrom->removeEdge(this, false);
    //     }
    //     if (myTo != to) {
    //         myTo->removeEdge(this, false);
    //     }
    //     // remove first from both nodes and then add to the new nodes
    //     // (otherwise reversing does not work)
    //     if (myFrom != from) {
    //         myFrom = from;
    //         myFrom->addOutgoingEdge(this);
    //     }
    //     if (myTo != to) {
    //         myTo = to;
    //         myTo->addIncomingEdge(this);
    //     }
    //     computeAngle();
    // }
    #[allow(unused_variables)]
    pub fn reinit_nodes(&mut self, from: Ptr<NBNode>, to: Ptr<NBNode>) {
        todo!("port NBEdge::reinitNodes")
    }

    /// C++: `void NBEdge::reshiftPosition(double, double)`
    /// Applies an offset to the edge
    // void
    // NBEdge::reshiftPosition(double xoff, double yoff) {
    //     myGeom.add(xoff, yoff, 0);
    //     for (Lane& lane : myLanes) {
    //         lane.customShape.add(xoff, yoff, 0);
    //     }
    //     computeLaneShapes(); // old shapes are dubious if computed with large coordinates
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         (*i).customShape.add(xoff, yoff, 0);
    //     }
    //     if (mySignalPosition != Position::INVALID) {
    //         mySignalPosition.add(xoff, yoff);
    //     }
    //     myFromBorder.add(xoff, yoff, 0);
    //     myToBorder.add(xoff, yoff, 0);
    //     computeEdgeShape();
    //     computeAngle(); // update angles because they are numerically sensitive (especially where based on centroids)
    // }
    #[allow(unused_variables)]
    pub fn reshift_position(&mut self, xoff: f64, yoff: f64) {
        todo!("port NBEdge::reshiftPosition")
    }

    /// C++: `void NBEdge::roundGeometry()`
    /// ensure consistency between input and output geometries
    // void
    // NBEdge::roundGeometry() {
    //     myGeom.round(gPrecision);
    //     for (Lane& lane : myLanes) {
    //         lane.customShape.round(gPrecision);
    //     }
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         (*i).customShape.round(gPrecision);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn round_geometry(&mut self) {
        todo!("port NBEdge::roundGeometry")
    }

    /// C++: `void NBEdge::roundSpeed()`
    /// ensure consistency between input and output speed
    // void
    // NBEdge::roundSpeed() {
    //     mySpeed = roundDecimalToEven(mySpeed, gPrecision);
    //     // lane speeds are not used for computation but are compared to mySpeed in hasLaneSpecificSpeed
    //     for (Lane& l : myLanes) {
    //         l.speed = roundDecimalToEven(l.speed, gPrecision);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn round_speed(&mut self) {
        todo!("port NBEdge::roundSpeed")
    }

    /// C++: `void NBEdge::mirrorX()`
    /// mirror coordinates along the x-axis
    // void
    // NBEdge::mirrorX() {
    //     myGeom.mirrorX();
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         myLanes[i].shape.mirrorX();
    //         myLanes[i].customShape.mirrorX();
    //     }
    //     for (Connection& c : myConnections) {
    //         c.shape.mirrorX();
    //         c.viaShape.mirrorX();
    //         c.customShape.mirrorX();
    //     }
    //     if (mySignalPosition != Position::INVALID) {
    //         mySignalPosition.sety(-mySignalPosition.y());
    //     }
    //     computeAngle(); // update angles because they are numerically sensitive (especially where based on centroids)
    // }
    #[allow(unused_variables)]
    pub fn mirror_x(&mut self) {
        todo!("port NBEdge::mirrorX")
    }

    /// C++: `int NBEdge::getNumLanes() const` [const]
    /// Returns the number of lanes
    // int getNumLanes() const {
    //         return (int)myLanes.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_lanes(&self) -> i32 {
        self.myLanes.len() as i32
    }

    /// C++: `int NBEdge::getPriority() const` [const]
    /// Returns the priority of the edge
    // int getPriority() const {
    //         return myPriority;
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        self.myPriority
    }

    /// C++: `void NBEdge::setPriority(int)`
    /// Sets the priority of the edge
    // void setPriority(int priority) {
    //         myPriority = priority;
    //     }
    #[allow(unused_variables)]
    pub fn set_priority(&mut self, priority: i32) {
        self.myPriority = priority;
    }

    /// C++: `NBNode * NBEdge::getFromNode() const` [const]
    /// Returns the origin node of the edge
    // inline NBNode* getFromNode() const {
    //         return myFrom;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_node(&self) -> Ptr<NBNode> {
        self.myFrom
    }

    /// C++: `NBNode * NBEdge::getToNode() const` [const]
    /// Returns the destination node of the edge
    // inline NBNode* getToNode() const {
    //         return myTo;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_node(&self) -> Ptr<NBNode> {
        self.myTo
    }

    /// C++: `double NBEdge::getStartAngle() const` [const]
    /// Returns the angle at the start of the edge (relative to the node shape center) The angle is computed in computeAngle()
    // inline double getStartAngle() const {
    //         return myStartAngle;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_angle(&self) -> f64 {
        self.myStartAngle
    }

    /// C++: `double NBEdge::getEndAngle() const` [const]
    /// Returns the angle at the end of the edge (relative to the node shape center) The angle is computed in computeAngle()
    // inline double getEndAngle() const {
    //         return myEndAngle;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_angle(&self) -> f64 {
        self.myEndAngle
    }

    /// C++: `double NBEdge::getShapeStartAngle() const` [const]
    /// Returns the angle at the start of the edge
    // double
    // NBEdge::getShapeStartAngle() const {
    //     const double angleLookahead = MIN2(myGeom.length2D() / 2, ANGLE_LOOKAHEAD);
    //     const Position referencePosStart = myGeom.positionAtOffset2D(angleLookahead);
    //     return GeomHelper::legacyDegree(myGeom.front().angleTo2D(referencePosStart), true);
    // }
    #[allow(unused_variables)]
    pub fn get_shape_start_angle(&self) -> f64 {
        todo!("port NBEdge::getShapeStartAngle")
    }

    /// C++: `double NBEdge::getShapeEndAngle() const` [const]
    /// Returns the angle at the end of the edge
    // double
    // NBEdge::getShapeEndAngle() const {
    //     const double angleLookahead = MIN2(myGeom.length2D() / 2, ANGLE_LOOKAHEAD);
    //     const Position referencePosEnd = myGeom.positionAtOffset2D(myGeom.length2D() - angleLookahead);
    //     return GeomHelper::legacyDegree(referencePosEnd.angleTo2D(myGeom.back()), true);
    // }
    #[allow(unused_variables)]
    pub fn get_shape_end_angle(&self) -> f64 {
        todo!("port NBEdge::getShapeEndAngle")
    }

    /// C++: `double NBEdge::getTotalAngle() const` [const]
    /// Returns the angle at the start of the edge
    // inline double getTotalAngle() const {
    //         return myTotalAngle;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_angle(&self) -> f64 {
        self.myTotalAngle
    }

    /// C++: `double NBEdge::getLength() const` [const, virtual]
    /// Returns the computed length of the edge
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `double NBEdge::getLoadedLength() const` [const]
    /// Returns the length was set explicitly or the computed length if it wasn't set
    // double getLoadedLength() const {
    //         return myLoadedLength > 0 ? myLoadedLength : myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_length(&self) -> f64 {
        if self.myLoadedLength > 0.0 { self.myLoadedLength } else { self.myLength }
    }

    /// C++: `double NBEdge::getFinalLength() const` [const]
    /// get length that will be assigned to the lanes in the final network
    // double
    // NBEdge::getFinalLength() const {
    //     double result = getLoadedLength();
    //     if (OptionsCont::getOptions().getBool("no-internal-links") && !hasLoadedLength()) {
    //         // use length to junction center even if a modified geometry was given
    //         PositionVector geom = cutAtIntersection(myGeom);
    //         geom.push_back_noDoublePos(getToNode()->getCenter());
    //         geom.push_front_noDoublePos(getFromNode()->getCenter());
    //         result = geom.length();
    //     }
    //     double avgEndOffset = 0;
    //     for (const Lane& lane : myLanes) {
    //         avgEndOffset += lane.endOffset;
    //     }
    //     if (isBidiRail()) {
    //         avgEndOffset += myPossibleTurnDestination->getEndOffset();
    //     }
    //     avgEndOffset /= (double)myLanes.size();
    //     return MAX2(result - avgEndOffset, POSITION_EPS);
    // }
    #[allow(unused_variables)]
    pub fn get_final_length(&self) -> f64 {
        todo!("port NBEdge::getFinalLength")
    }

    /// C++: `bool NBEdge::hasLoadedLength() const` [const]
    /// Returns whether a length was set explicitly
    // bool hasLoadedLength() const {
    //         return myLoadedLength > 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_loaded_length(&self) -> bool {
        self.myLoadedLength > 0.0
    }

    /// C++: `double NBEdge::getSpeed() const` [const, virtual]
    /// Returns the speed allowed on this edge
    // double getSpeed() const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        self.mySpeed
    }

    /// C++: `double NBEdge::getFriction() const` [const]
    /// Returns the friction on this edge
    // double getFriction() const {
    //         return myFriction;
    //     }
    #[allow(unused_variables)]
    pub fn get_friction(&self) -> f64 {
        self.myFriction
    }

    /// C++: `EdgeBuildingStep NBEdge::getStep() const` [const]
    /// The building step of this edge
    // EdgeBuildingStep getStep() const {
    //         return myStep;
    //     }
    #[allow(unused_variables)]
    pub fn get_step(&self) -> NBEdge_EdgeBuildingStep {
        self.myStep
    }

    /// C++: `double NBEdge::getLaneWidth() const` [const]
    /// Returns the default width of lanes of this edge
    // double getLaneWidth() const {
    //         return myLaneWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_width(&self) -> f64 {
        todo!("port NBEdge::getLaneWidth")
    }

    /// C++: `double NBEdge::getLaneWidth(int) const` [const]
    /// Returns the width of the lane of this edge
    // double
    // NBEdge::getLaneWidth(int lane) const {
    //     return myLanes[lane].width != UNSPECIFIED_WIDTH
    //            ? myLanes[lane].width
    //            : getLaneWidth() != UNSPECIFIED_WIDTH ? getLaneWidth() : SUMO_const_laneWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_width_2(&self, lane: i32) -> f64 {
        todo!("port NBEdge::getLaneWidth")
    }

    /// C++: `double NBEdge::getInternalLaneWidth(const NBNode &, const NBEdge::Connection &, const NBEdge::Lane &, bool) const` [const]
    /// Returns the width of the internal lane associated with the connection
    // double
    // NBEdge::getInternalLaneWidth(
    //     const NBNode& node,
    //     const NBEdge::Connection& connection,
    //     const NBEdge::Lane& successor,
    //     bool isVia) const {
    //
    //     if (!isVia && node.isConstantWidthTransition() && getNumLanes() > connection.toEdge->getNumLanes()) {
    //         return getLaneWidth(connection.fromLane);
    //     }
    //
    //     return (isBikepath(getPermissions(connection.fromLane)) && (
    //                 getLaneWidth(connection.fromLane) < successor.width || successor.width == UNSPECIFIED_WIDTH)) ?
    //            myLanes[connection.fromLane].width : successor.width; // getLaneWidth(connection.fromLane) never returns -1 (UNSPECIFIED_WIDTH)
    // }
    #[allow(unused_variables)]
    pub fn get_internal_lane_width(&self, node: &NBNode, connection: &NBEdge_Connection, successor: &NBEdge_Lane, isVia: bool) -> f64 {
        todo!("port NBEdge::getInternalLaneWidth")
    }

    /// C++: `double NBEdge::getTotalWidth() const` [const]
    /// Returns the combined width of all lanes of this edge
    // double
    // NBEdge::getTotalWidth() const {
    //     double result = 0;
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         result += getLaneWidth(i);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_total_width(&self) -> f64 {
        todo!("port NBEdge::getTotalWidth")
    }

    /// C++: `const std::string & NBEdge::getStreetName() const` [const]
    /// Returns the street name of this edge
    // const std::string& getStreetName() const {
    //         return myStreetName;
    //     }
    #[allow(unused_variables)]
    pub fn get_street_name(&self) -> &str {
        todo!("port NBEdge::getStreetName")
    }

    /// C++: `void NBEdge::setStreetName(const std::string &)`
    /// sets the street name of this edge
    // void setStreetName(const std::string& name) {
    //         myStreetName = name;
    //     }
    #[allow(unused_variables)]
    pub fn set_street_name(&mut self, name: &str) {
        todo!("port NBEdge::setStreetName")
    }

    /// C++: `double NBEdge::getDistance() const` [const]
    /// get distance
    // double getDistance() const {
    //         return myDistance;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port NBEdge::getDistance")
    }

    /// C++: `double NBEdge::getDistancAt(double) const` [const]
    /// get distance at the given offset
    // double
    // NBEdge::getDistancAt(double pos) const {
    //     // negative values of myDistances indicate descending kilometrage
    //     return fabs(myDistance + pos);
    // }
    #[allow(unused_variables)]
    pub fn get_distanc_at(&self, pos: f64) -> f64 {
        todo!("port NBEdge::getDistancAt")
    }

    /// C++: `double NBEdge::getEndOffset() const` [const]
    /// Returns the offset to the destination node
    // double getEndOffset() const {
    //         return myEndOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_offset(&self) -> f64 {
        todo!("port NBEdge::getEndOffset")
    }

    /// C++: `double NBEdge::getEndOffset(int) const` [const]
    /// Returns the offset to the destination node a the specified lane
    // double
    // NBEdge::getEndOffset(int lane) const {
    //     return myLanes[lane].endOffset != UNSPECIFIED_OFFSET ? myLanes[lane].endOffset : getEndOffset();
    // }
    #[allow(unused_variables)]
    pub fn get_end_offset_2(&self, lane: i32) -> f64 {
        todo!("port NBEdge::getEndOffset")
    }

    /// C++: `const StopOffset & NBEdge::getEdgeStopOffset() const` [const]
    /// Returns the stopOffset to the end of the edge
    // const StopOffset&
    // NBEdge::getEdgeStopOffset() const {
    //     return myEdgeStopOffset;
    // }
    #[allow(unused_variables)]
    pub fn get_edge_stop_offset(&self) -> &StopOffset {
        todo!("port NBEdge::getEdgeStopOffset")
    }

    /// C++: `const StopOffset & NBEdge::getLaneStopOffset(int) const` [const]
    /// Returns the stop offset to the specified lane's end
    // const StopOffset&
    // NBEdge::getLaneStopOffset(int lane) const {
    //     if (lane == -1) {
    //         return myEdgeStopOffset;
    //     } else {
    //         return myLanes[lane].laneStopOffset;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_stop_offset(&self, lane: i32) -> &StopOffset {
        todo!("port NBEdge::getLaneStopOffset")
    }

    /// C++: `double NBEdge::getSignalOffset() const` [const]
    /// Returns the offset of a traffic signal from the end of this edge
    // double
    // NBEdge::getSignalOffset() const {
    //     if (mySignalPosition == Position::INVALID) {
    //         return UNSPECIFIED_SIGNAL_OFFSET;
    //     } else {
    //         Position laneEnd = myLaneSpreadFunction == LaneSpreadFunction::RIGHT ?
    //                            myLanes.back().shape.back() : myLanes[getNumLanes() / 2].shape.back();
    //         //std::cout << getID() << " signalPos=" << mySignalPosition << " laneEnd=" << laneEnd << " toShape=" << myTo->getShape() << " toBorder=" << myToBorder << "\n";
    //         return mySignalPosition.distanceTo2D(laneEnd);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_signal_offset(&self) -> f64 {
        todo!("port NBEdge::getSignalOffset")
    }

    /// C++: `const Position & NBEdge::getSignalPosition() const` [const]
    /// Returns the position of a traffic signal on this edge
    // const Position& getSignalPosition() const {
    //         return mySignalPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_signal_position(&self) -> &Position {
        todo!("port NBEdge::getSignalPosition")
    }

    /// C++: `const NBNode * NBEdge::getSignalNode() const` [const]
    /// Returns the node that (possibly) represents a traffic signal controlling at the end of this edge
    // const NBNode* getSignalNode() const {
    //         return mySignalNode;
    //     }
    #[allow(unused_variables)]
    pub fn get_signal_node(&self) -> Ptr<NBNode> {
        todo!("port NBEdge::getSignalNode")
    }

    /// C++: `void NBEdge::setSignalPosition(const Position &, const NBNode *)`
    /// sets the offset of a traffic signal from the end of this edge
    // void setSignalPosition(const Position& pos, const NBNode* signalNode) {
    //         mySignalPosition = pos;
    //         mySignalNode = signalNode;
    //     }
    #[allow(unused_variables)]
    pub fn set_signal_position(&mut self, pos: &Position, signalNode: Ptr<NBNode>) {
        todo!("port NBEdge::setSignalPosition")
    }

    /// C++: `const std::vector<NBEdge::Lane> & NBEdge::getLanes() const` [const]
    /// Returns the lane definitions
    // const std::vector<NBEdge::Lane>& getLanes() const {
    //         return myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&self) -> &Vec<NBEdge_Lane> {
        todo!("port NBEdge::getLanes")
    }

    /// C++: `int NBEdge::getFirstNonPedestrianLaneIndex(int, bool) const` [const]
    /// return the first lane with permissions other than SVC_PEDESTRIAN and 0
    // int
    // NBEdge::getFirstNonPedestrianLaneIndex(int direction, bool exclusive) const {
    //     assert(direction == NBNode::FORWARD || direction == NBNode::BACKWARD);
    //     const int start = (direction == NBNode::FORWARD ? 0 : (int)myLanes.size() - 1);
    //     const int end = (direction == NBNode::FORWARD ? (int)myLanes.size() : - 1);
    //     for (int i = start; i != end; i += direction) {
    //         // SVCAll, does not count as a sidewalk, green verges (permissions = 0) do not count as road
    //         // in the exclusive case, lanes that allow pedestrians along with any other class also count as road
    //         if ((exclusive && myLanes[i].permissions != SVC_PEDESTRIAN && myLanes[i].permissions != 0)
    //                 || ((myLanes[i].permissions & SVC_PEDESTRIAN) == 0 && myLanes[i].permissions != 0)) {
    //             return i;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_first_non_pedestrian_lane_index(&self, direction: i32, exclusive: bool) -> i32 {
        todo!("port NBEdge::getFirstNonPedestrianLaneIndex")
    }

    /// C++: `int NBEdge::getFirstNonPedestrianNonBicycleLaneIndex(int, bool) const` [const]
    /// return the first lane with permissions other than SVC_PEDESTRIAN, SVC_BICYCLE and 0
    // int
    // NBEdge::getFirstNonPedestrianNonBicycleLaneIndex(int direction, bool exclusive) const {
    //     assert(direction == NBNode::FORWARD || direction == NBNode::BACKWARD);
    //     const int start = (direction == NBNode::FORWARD ? 0 : (int)myLanes.size() - 1);
    //     const int end = (direction == NBNode::FORWARD ? (int)myLanes.size() : - 1);
    //     for (int i = start; i != end; i += direction) {
    //         // SVCAll, does not count as a sidewalk, green verges (permissions = 0) do not count as road
    //         // in the exclusive case, lanes that allow pedestrians along with any other class also count as road
    //         SVCPermissions p = myLanes[i].permissions;
    //         if ((exclusive && p != SVC_PEDESTRIAN && p != SVC_BICYCLE && p != (SVC_PEDESTRIAN | SVC_BICYCLE) && p != 0)
    //                 || (p == SVCAll || ((p & (SVC_PEDESTRIAN | SVC_BICYCLE)) == 0 && p != 0))) {
    //             return i;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_first_non_pedestrian_non_bicycle_lane_index(&self, direction: i32, exclusive: bool) -> i32 {
        todo!("port NBEdge::getFirstNonPedestrianNonBicycleLaneIndex")
    }

    /// C++: `int NBEdge::getSpecialLane(SVCPermissions) const` [const]
    /// return index of the first lane that allows the given permissions
    // int
    // NBEdge::getSpecialLane(SVCPermissions permissions) const {
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         if (myLanes[i].permissions == permissions) {
    //             return i;
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_special_lane(&self, permissions: SVCPermissions) -> i32 {
        todo!("port NBEdge::getSpecialLane")
    }

    /// C++: `int NBEdge::getFirstAllowedLaneIndex(int) const` [const]
    /// return the first lane that permits at least 1 vClass or the last lane if search direction of there is no such lane
    // int
    // NBEdge::getFirstAllowedLaneIndex(int direction) const {
    //     assert(direction == NBNode::FORWARD || direction == NBNode::BACKWARD);
    //     const int start = (direction == NBNode::FORWARD ? 0 : (int)myLanes.size() - 1);
    //     const int end = (direction == NBNode::FORWARD ? (int)myLanes.size() : - 1);
    //     for (int i = start; i != end; i += direction) {
    //         if (myLanes[i].permissions != 0) {
    //             return i;
    //         }
    //     }
    //     return end - direction;
    // }
    #[allow(unused_variables)]
    pub fn get_first_allowed_lane_index(&self, direction: i32) -> i32 {
        todo!("port NBEdge::getFirstAllowedLaneIndex")
    }

    /// C++: `NBEdge::Lane NBEdge::getFirstNonPedestrianLane(int) const` [const]
    /// get first non-pedestrian lane
    // NBEdge::Lane
    // NBEdge::getFirstNonPedestrianLane(int direction) const {
    //     int index = getFirstNonPedestrianLaneIndex(direction);
    //     if (index < 0) {
    //         throw ProcessError(TLF("Edge % allows pedestrians on all lanes", getID()));
    //     }
    //     return myLanes[index];
    // }
    #[allow(unused_variables)]
    pub fn get_first_non_pedestrian_lane(&self, direction: i32) -> NBEdge_Lane {
        todo!("port NBEdge::getFirstNonPedestrianLane")
    }

    /// C++: `std::set<SVCPermissions> NBEdge::getPermissionVariants(int, int) const` [const]
    /// return all permission variants within the specified lane range [iStart, iEnd[
    // std::set<SVCPermissions>
    // NBEdge::getPermissionVariants(int iStart, int iEnd) const {
    //     std::set<SVCPermissions> result;
    //     if (iStart < 0 || iStart >= getNumLanes() || iEnd > getNumLanes()) {
    //         throw ProcessError("invalid indices iStart " + toString(iStart) + " iEnd " + toString(iEnd) + " for edge with " + toString(getNumLanes()) + " lanes.");
    //     }
    //     for (int i = iStart; i < iEnd; ++i) {
    //         result.insert(getPermissions(i));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_permission_variants(&self, iStart: i32, iEnd: i32) -> BTreeSet<SVCPermissions> {
        todo!("port NBEdge::getPermissionVariants")
    }

    /// C++: `int NBEdge::getNumLanesThatAllow(SVCPermissions, bool) const` [const]
    // int
    // NBEdge::getNumLanesThatAllow(SVCPermissions permissions, bool allPermissions) const {
    //     int result = 0;
    //     for (const Lane& lane : myLanes) {
    //         if ((allPermissions && (lane.permissions & permissions) == permissions)
    //                 || (!allPermissions && (lane.permissions & permissions) != 0)) {
    //             result++;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_num_lanes_that_allow(&self, permissions: SVCPermissions, allPermissions: bool) -> i32 {
        todo!("port NBEdge::getNumLanesThatAllow")
    }

    /// C++: `bool NBEdge::allowsChangingLeft(int, SUMOVehicleClass) const` [const]
    /// Returns whether the given vehicle class may change left from this lane
    // bool
    // NBEdge::allowsChangingLeft(int lane, SUMOVehicleClass vclass) const {
    //     assert(lane >= 0 && lane < getNumLanes());
    //     return myLanes[lane].changeLeft == SVC_UNSPECIFIED ? true : (myLanes[lane].changeLeft & vclass) == vclass;
    // }
    #[allow(unused_variables)]
    pub fn allows_changing_left(&self, lane: i32, vclass: SUMOVehicleClass) -> bool {
        todo!("port NBEdge::allowsChangingLeft")
    }

    /// C++: `bool NBEdge::allowsChangingRight(int, SUMOVehicleClass) const` [const]
    /// Returns whether the given vehicle class may change left from this lane
    // bool
    // NBEdge::allowsChangingRight(int lane, SUMOVehicleClass vclass) const {
    //     assert(lane >= 0 && lane < getNumLanes());
    //     return myLanes[lane].changeRight == SVC_UNSPECIFIED ? true : (myLanes[lane].changeRight & vclass) == vclass;
    // }
    #[allow(unused_variables)]
    pub fn allows_changing_right(&self, lane: i32, vclass: SUMOVehicleClass) -> bool {
        todo!("port NBEdge::allowsChangingRight")
    }

    /// C++: `double NBEdge::getCrossingAngle(NBNode *)`
    /// return the angle for computing pedestrian crossings at the given node
    // double
    // NBEdge::getCrossingAngle(NBNode* node) {
    //     double angle = getAngleAtNode(node) + (getFromNode() == node ? 180.0 : 0.0);
    //     if (angle < 0) {
    //         angle += 360.0;
    //     }
    //     if (angle >= 360) {
    //         angle -= 360.0;
    //     }
    //     if (gDebugFlag1) {
    //         std::cout << getID() << " angle=" << getAngleAtNode(node) << " convAngle=" << angle << "\n";
    //     }
    //     return angle;
    // }
    #[allow(unused_variables)]
    pub fn get_crossing_angle(&mut self, node: Ptr<NBNode>) -> f64 {
        todo!("port NBEdge::getCrossingAngle")
    }

    /// C++: `std::string NBEdge::getSidewalkID()`
    /// get the lane id for the canonical sidewalk lane
    // std::string
    // NBEdge::getSidewalkID() {
    //     // see IntermodalEdge::getSidewalk()
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         if (myLanes[i].permissions == SVC_PEDESTRIAN) {
    //             return getLaneID(i);
    //         }
    //     }
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         if ((myLanes[i].permissions & SVC_PEDESTRIAN) != 0) {
    //             return getLaneID(i);
    //         }
    //     }
    //     return getLaneID(0);
    // }
    #[allow(unused_variables)]
    pub fn get_sidewalk_id(&mut self) -> String {
        todo!("port NBEdge::getSidewalkID")
    }

    /// C++: `const PositionVector & NBEdge::getGeometry() const` [const]
    /// Returns the geometry of the edge
    // const PositionVector& getGeometry() const {
    //         return myGeom;
    //     }
    #[allow(unused_variables)]
    pub fn get_geometry(&self) -> &PositionVector {
        todo!("port NBEdge::getGeometry")
    }

    /// C++: `const PositionVector NBEdge::getInnerGeometry() const` [const]
    /// Returns the geometry of the edge without the endpoints
    // const PositionVector
    // NBEdge::getInnerGeometry() const {
    //     return myGeom.getSubpartByIndex(1, (int)myGeom.size() - 2);
    // }
    #[allow(unused_variables)]
    pub fn get_inner_geometry(&self) -> PositionVector {
        todo!("port NBEdge::getInnerGeometry")
    }

    /// C++: `bool NBEdge::hasDefaultGeometry() const` [const]
    /// Returns whether the geometry consists only of the node positions
    // bool
    // NBEdge::hasDefaultGeometry() const {
    //     return myGeom.size() == 2 && hasDefaultGeometryEndpoints();
    // }
    #[allow(unused_variables)]
    pub fn has_default_geometry(&self) -> bool {
        todo!("port NBEdge::hasDefaultGeometry")
    }

    /// C++: `bool NBEdge::hasDefaultGeometryEndpoints() const` [const]
    /// Returns whether the geometry is terminated by the node positions This default may be violated by initializing with tryIgnoreNodePositions=true' or with setGeometry() non-default endpoints are useful to control the generated node shape
    // bool
    // NBEdge::hasDefaultGeometryEndpoints() const {
    //     return myGeom.front().almostSame(myFrom->getPosition(), 0.01)  &&
    //            myGeom.back().almostSame(myTo->getPosition(), 0.01);
    // }
    #[allow(unused_variables)]
    pub fn has_default_geometry_endpoints(&self) -> bool {
        todo!("port NBEdge::hasDefaultGeometryEndpoints")
    }

    /// C++: `bool NBEdge::hasDefaultGeometryEndpointAtNode(const NBNode *) const` [const]
    /// Returns whether the geometry is terminated by the node positions This default may be violated by initializing with tryIgnoreNodePositions=true' or with setGeometry() non-default endpoints are useful to control the generated node shape
    // bool
    // NBEdge::hasDefaultGeometryEndpointAtNode(const NBNode* node) const {
    //     // do not extend past the node position
    //     if (node == myFrom) {
    //         return myGeom.front() == node->getPosition();
    //     } else {
    //         assert(node == myTo);
    //         return myGeom.back() == node->getPosition();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn has_default_geometry_endpoint_at_node(&self, node: Ptr<NBNode>) -> bool {
        todo!("port NBEdge::hasDefaultGeometryEndpointAtNode")
    }

    /// C++: `Position NBEdge::getEndpointAtNode(const NBNode *) const` [const]
    // Position
    // NBEdge::getEndpointAtNode(const NBNode* node) const {
    //     return node == myFrom ? myGeom.front() : myGeom.back();
    // }
    #[allow(unused_variables)]
    pub fn get_endpoint_at_node(&self, node: Ptr<NBNode>) -> Position {
        todo!("port NBEdge::getEndpointAtNode")
    }

    /// C++: `void NBEdge::resetEndpointAtNode(const NBNode *)`
    // void
    // NBEdge::resetEndpointAtNode(const NBNode* node) {
    //     assert(myGeom.size() >= 2);
    //     if (node == myFrom) {
    //         myGeom[0] = myFrom->getPosition();
    //     } else if (node == myTo) {
    //         myGeom[-1] = myTo->getPosition();
    //     } else {
    //         assert(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_endpoint_at_node(&mut self, node: Ptr<NBNode>) {
        todo!("port NBEdge::resetEndpointAtNode")
    }

    /// C++: `void NBEdge::setGeometry(const PositionVector &, bool)`
    /// (Re)sets the edge's geometry
    // void
    // NBEdge::setGeometry(const PositionVector& s, bool inner) {
    //     Position begin = myGeom.front(); // may differ from node position
    //     Position end = myGeom.back(); // may differ from node position
    //     myGeom = s;
    //     if (inner) {
    //         myGeom.insert(myGeom.begin(), begin);
    //         myGeom.push_back(end);
    //     }
    //     // ensure non-zero length (see ::init)
    //     if (myGeom.size() == 2 && myGeom[0] == myGeom[1]) {
    //         WRITE_WARNINGF(TL("Edge's '%' from- and to-node are at the same position."), myID);
    //         int patchIndex = myFrom->getID() < myTo->getID() ? 1 : 0;
    //         myGeom[patchIndex].add(Position(POSITION_EPS, POSITION_EPS));
    //     }
    //     computeLaneShapes();
    //     computeAngle();
    //     myLength = myGeom.length();
    // }
    #[allow(unused_variables)]
    pub fn set_geometry(&mut self, g: &PositionVector, inner: bool) {
        todo!("port NBEdge::setGeometry")
    }

    /// C++: `void NBEdge::addGeometryPoint(int, const Position &)`
    /// Adds a further geometry point
    // void
    // NBEdge::addGeometryPoint(int index, const Position& p) {
    //     if (index >= 0) {
    //         myGeom.insert(myGeom.begin() + index, p);
    //     } else {
    //         myGeom.insert(myGeom.end() + index, p);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_geometry_point(&mut self, index: i32, p: &Position) {
        todo!("port NBEdge::addGeometryPoint")
    }

    /// C++: `void NBEdge::extendGeometryAtNode(const NBNode *, double)`
    /// linearly extend the geometry at the given node
    // void
    // NBEdge::extendGeometryAtNode(const NBNode* node, double maxExtent) {
    //     //std::cout << "extendGeometryAtNode edge=" << getID() << " node=" << node->getID() << " nodePos=" << node->getPosition() << " extent=" << maxExtent << " geom=" << myGeom;
    //     if (node == myFrom) {
    //         myGeom.extrapolate(maxExtent, true);
    //         double offset = myGeom.nearest_offset_to_point2D(node->getPosition());
    //         //std::cout << " geom2=" << myGeom << " offset=" << offset;
    //         if (offset != GeomHelper::INVALID_OFFSET) {
    //             myGeom = myGeom.getSubpart2D(MIN2(offset, myGeom.length2D() - 2 * POSITION_EPS), myGeom.length2D());
    //         }
    //     } else {
    //         assert(node == myTo);
    //         myGeom.extrapolate(maxExtent, false, true);
    //         double offset = myGeom.nearest_offset_to_point2D(node->getPosition());
    //         //std::cout << " geom2=" << myGeom << " offset=" << offset;
    //         if (offset != GeomHelper::INVALID_OFFSET) {
    //             myGeom = myGeom.getSubpart2D(0, MAX2(offset, 2 * POSITION_EPS));
    //         }
    //     }
    //     //std::cout << " geom3=" << myGeom << "\n";
    // }
    #[allow(unused_variables)]
    pub fn extend_geometry_at_node(&mut self, node: Ptr<NBNode>, maxExtent: f64) {
        todo!("port NBEdge::extendGeometryAtNode")
    }

    /// C++: `void NBEdge::shortenGeometryAtNode(const NBNode *, double)`
    /// linearly extend the geometry at the given node
    // void
    // NBEdge::shortenGeometryAtNode(const NBNode* node, double reduction) {
    //     //std::cout << "shortenGeometryAtNode edge=" << getID() << " node=" << node->getID() << " nodePos=" << node->getPosition() << " reduction=" << reduction << " geom=" << myGeom;
    //     reduction = MIN2(reduction, myGeom.length2D() - 2 * POSITION_EPS);
    //     if (node == myFrom) {
    //         myGeom = myGeom.getSubpart2D(reduction, myGeom.length2D());
    //     } else {
    //         myGeom = myGeom.getSubpart2D(0, myGeom.length2D() - reduction);
    //     }
    //     computeLaneShapes();
    //     //std::cout << " geom2=" << myGeom << "\n";
    // }
    #[allow(unused_variables)]
    pub fn shorten_geometry_at_node(&mut self, node: Ptr<NBNode>, reduction: f64) {
        todo!("port NBEdge::shortenGeometryAtNode")
    }

    /// C++: `bool NBEdge::shiftPositionAtNode(NBNode *, NBEdge *)`
    /// shift geometry at the given node to avoid overlap and return whether geometry was changed
    // bool
    // NBEdge::shiftPositionAtNode(NBNode* node, NBEdge* other) {
    //     if (myLaneSpreadFunction == LaneSpreadFunction::CENTER
    //             && !isRailway(getPermissions())
    //             && !isRailway(other->getPermissions())
    //             && getBidiEdge() == nullptr) {
    //         const int i = (node == myTo ? -1 : 0);
    //         const int i2 = (node == myTo ? 0 : -1);
    //         const double dist = myGeom[i].distanceTo2D(node->getPosition());
    //         const double neededOffset = getTotalWidth() / 2;
    //         const double dist2 = MIN2(myGeom.distance2D(other->getGeometry()[i2]),
    //                                   other->getGeometry().distance2D(myGeom[i]));
    //         const double neededOffset2 = neededOffset + (other->getLaneSpreadFunction() == LaneSpreadFunction::CENTER
    //                                      ? (other->getTotalWidth()) / 2 : 0);
    //         const double missing = neededOffset - dist;
    //         const double missing2 = neededOffset2 - dist2;
    //         double shift = 0;
    //         if (missing > 0 && missing2 > 0) {
    //             shift = MIN2(missing, missing2);
    //         } else if (missing2) {
    //             shift = missing2;
    //         }
    //         if (shift > 0) {
    //             PositionVector tmp = myGeom;
    //             try {
    //                 tmp.move2side(shift);
    //                 tmp[i].round(gPrecision);
    //                 myGeom[i] = tmp[i];
    //                 computeAngle();
    //                 return true;
    //                 //std::cout << getID() << " shiftPositionAtNode needed=" << neededOffset << " dist=" << dist << " needed2=" << neededOffset2 << " dist2=" << dist2 << "  by=" << (neededOffset - dist) << " other=" << other->getID() << "\n";
    //             } catch (InvalidArgument&) {
    //                 WRITE_WARNINGF(TL("Could not avoid overlapping shape at node '%' for edge '%'."), node->getID(), getID());
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn shift_position_at_node(&mut self, node: Ptr<NBNode>, opposite: Ptr<NBEdge>) -> bool {
        todo!("port NBEdge::shiftPositionAtNode")
    }

    /// C++: `Position NBEdge::geometryPositionAtOffset(double) const` [const]
    /// return position taking into account loaded length
    // Position
    // NBEdge::geometryPositionAtOffset(double offset) const {
    //     if (myLoadedLength > 0) {
    //         return myGeom.positionAtOffset(offset * myLength / myLoadedLength);
    //     } else {
    //         return myGeom.positionAtOffset(offset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn geometry_position_at_offset(&self, offset: f64) -> Position {
        todo!("port NBEdge::geometryPositionAtOffset")
    }

    /// C++: `void NBEdge::computeEdgeShape(double)`
    /// Recomputeds the lane shapes to terminate at the node shape For every lane the intersection with the fromNode and toNode is calculated and the lane shorted accordingly. The edge length is then set to the average of all lane lengths (which may differ). This average length is used as the lane length when writing the network.
    // void
    // NBEdge::computeEdgeShape(double smoothElevationThreshold) {
    //     if (smoothElevationThreshold > 0 && myGeom.hasElevation()) {
    //         PositionVector cut = cutAtIntersection(myGeom);
    //         // cutting and patching z-coordinate may cause steep grades which should be smoothed
    //         if (!myFrom->geometryLike()) {
    //             cut[0].setz(myFrom->getPosition().z());
    //             const double d = cut[0].distanceTo2D(cut[1]);
    //             const double dZ = fabs(cut[0].z() - cut[1].z());
    //             if (dZ / smoothElevationThreshold > d) {
    //                 cut = cut.smoothedZFront(MIN2(cut.length2D() / 2, dZ / smoothElevationThreshold));
    //             }
    //         }
    //         if (!myTo->geometryLike()) {
    //             cut[-1].setz(myTo->getPosition().z());
    //             const double d = cut[-1].distanceTo2D(cut[-2]);
    //             const double dZ = fabs(cut[-1].z() - cut[-2].z());
    //             if (dZ / smoothElevationThreshold > d) {
    //                 cut = cut.reverse().smoothedZFront(MIN2(cut.length2D() / 2, dZ / smoothElevationThreshold)).reverse();
    //             }
    //         }
    //         cut[0] = myGeom[0];
    //         cut[-1] = myGeom[-1];
    //         if (cut != myGeom) {
    //             myGeom = cut;
    //             computeLaneShapes();
    //         }
    //     }
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         myLanes[i].shape = cutAtIntersection(myLanes[i].shape);
    //     }
    //     // recompute edge's length as the average of lane lengths
    //     double avgLength = 0;
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         avgLength += myLanes[i].shape.length();
    //     }
    //     myLength = avgLength / (double) myLanes.size();
    //     computeAngle(); // update angles using the finalized node and lane shapes
    // }
    #[allow(unused_variables)]
    pub fn compute_edge_shape(&mut self, smoothElevationThreshold: f64) {
        todo!("port NBEdge::computeEdgeShape")
    }

    /// C++: `const PositionVector & NBEdge::getLaneShape(int) const` [const]
    /// Returns the shape of the nth lane
    // const PositionVector&
    // NBEdge::getLaneShape(int i) const {
    //     return myLanes[i].shape;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_shape(&self, i: i32) -> &PositionVector {
        todo!("port NBEdge::getLaneShape")
    }

    /// C++: `void NBEdge::setLaneSpreadFunction(LaneSpreadFunction)`
    /// (Re)sets how the lanes lateral offset shall be computed
    // void
    // NBEdge::setLaneSpreadFunction(LaneSpreadFunction spread) {
    //     myLaneSpreadFunction = spread;
    // }
    #[allow(unused_variables)]
    pub fn set_lane_spread_function(&mut self, spread: LaneSpreadFunction) {
        todo!("port NBEdge::setLaneSpreadFunction")
    }

    /// C++: `LaneSpreadFunction NBEdge::getLaneSpreadFunction() const` [const]
    /// Returns how this edge's lanes' lateral offset is computed
    // LaneSpreadFunction
    // NBEdge::getLaneSpreadFunction() const {
    //     return myLaneSpreadFunction;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_spread_function(&self) -> LaneSpreadFunction {
        todo!("port NBEdge::getLaneSpreadFunction")
    }

    /// C++: `void NBEdge::reduceGeometry(const double)`
    /// Removes points with a distance lesser than the given
    // void
    // NBEdge::reduceGeometry(const double minDist) {
    //     // attempt symmetrical removal for forward and backward direction
    //     // (very important for bidiRail)
    //     if (myFrom->getID() < myTo->getID()) {
    //         PositionVector reverse = myGeom.reverse();
    //         reverse.removeDoublePoints(minDist, true, 0, 0, true);
    //         myGeom = reverse.reverse();
    //         for (Lane& lane : myLanes) {
    //             reverse = lane.customShape.reverse();
    //             reverse.removeDoublePoints(minDist, true, 0, 0, true);
    //             lane.customShape = reverse.reverse();
    //         }
    //     } else {
    //         myGeom.removeDoublePoints(minDist, true, 0, 0, true);
    //         for (Lane& lane : myLanes) {
    //             lane.customShape.removeDoublePoints(minDist, true, 0, 0, true);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reduce_geometry(&mut self, minDist: f64) {
        todo!("port NBEdge::reduceGeometry")
    }

    /// C++: `void NBEdge::checkGeometry(const double, bool, const double, bool, bool)`
    /// Check the angles of successive geometry segments
    // void
    // NBEdge::checkGeometry(const double maxAngle, bool fixAngle, const double minRadius, bool fix, bool silent) {
    //     if (myGeom.size() < 3) {
    //         return;
    //     }
    //     //std::cout << "checking geometry of " << getID() << " geometry = " << toString(myGeom) << "\n";
    //     std::vector<double> angles; // absolute segment angles
    //     //std::cout << "  absolute angles:";
    //     for (int i = 0; i < (int)myGeom.size() - 1; ++i) {
    //         angles.push_back(myGeom.angleAt2D(i));
    //         //std::cout << " " << angles.back();
    //     }
    //     //std::cout << "\n  relative angles: ";
    //     NBEdge* bidi = const_cast<NBEdge*>(getBidiEdge());
    //     for (int i = 0; i < (int)angles.size() - 1; ++i) {
    //         const double relAngle = fabs(GeomHelper::angleDiff(angles[i], angles[i + 1]));
    //         //std::cout << relAngle << " ";
    //         if (maxAngle > 0 && relAngle > maxAngle) {
    //             if (fixAngle) {
    //                 WRITE_MESSAGEF(TL("Removing sharp angle of % degrees at edge '%', segment %."),
    //                                toString(relAngle), getID(), i);
    //                 myGeom.erase(myGeom.begin() + i + 1);
    //                 if (bidi != nullptr) {
    //                     bidi->myGeom = myGeom.reverse();
    //                 }
    //                 checkGeometry(maxAngle, fixAngle, minRadius, fix, silent);
    //                 return;
    //             } else if (!silent) {
    //                 WRITE_WARNINGF(TL("Found angle of % degrees at edge '%', segment %."), RAD2DEG(relAngle), getID(), i);
    //             }
    //         }
    //         if (relAngle < DEG2RAD(1)) {
    //             continue;
    //         }
    //         if (i == 0 || i == (int)angles.size() - 2) {
    //             const bool start = i == 0;
    //             const double dist = (start ? myGeom[0].distanceTo2D(myGeom[1]) : myGeom[-2].distanceTo2D(myGeom[-1]));
    //             const double r = tan(0.5 * (M_PI - relAngle)) * dist;
    //             //std::cout << (start ? "  start" : "  end") << " length=" << dist << " radius=" << r << "  ";
    //             if (minRadius > 0 && r < minRadius) {
    //                 if (fix) {
    //                     WRITE_MESSAGEF(TL("Removing sharp turn with radius % at the % of edge '%'."),
    //                                    toString(r), start ? TL("start") : TL("end"), getID());
    //                     myGeom.erase(myGeom.begin() + (start ? 1 : i + 1));
    //                     if (bidi != nullptr) {
    //                         bidi->myGeom = myGeom.reverse();
    //                     }
    //                     checkGeometry(maxAngle, fixAngle, minRadius, fix, silent);
    //                     return;
    //                 } else if (!silent) {
    //                     WRITE_WARNINGF(TL("Found sharp turn with radius % at the % of edge '%'."),
    //                                    toString(r), start ? TL("start") : TL("end"), getID());
    //                 }
    //             }
    //         }
    //     }
    //     //std::cout << "\n";
    // }
    #[allow(unused_variables)]
    pub fn check_geometry(&mut self, maxAngle: f64, fixAngle: bool, minRadius: f64, fix: bool, silent: bool) {
        todo!("port NBEdge::checkGeometry")
    }

    /// C++: `bool NBEdge::addEdge2EdgeConnection(NBEdge *, bool, SVCPermissions)`
    /// Adds a connection to another edge
    // bool
    // NBEdge::addEdge2EdgeConnection(NBEdge* dest, bool overrideRemoval, SVCPermissions permissions) {
    //     if (myStep == EdgeBuildingStep::INIT_REJECT_CONNECTIONS) {
    //         return true;
    //     }
    //     // check whether the node was merged and now a connection between
    //     //  not matching edges is tried to be added
    //     //  This happens f.e. within the ptv VISSIM-example "Beijing"
    //     if (dest != nullptr && myTo != dest->myFrom) {
    //         return false;
    //     }
    //     if (dest == nullptr) {
    //         invalidateConnections();
    //         myConnections.push_back(Connection(-1, dest, -1));
    //         myStep = EdgeBuildingStep::LANES2LANES_USER;
    //     } else if (find_if(myConnections.begin(), myConnections.end(), connections_toedge_finder(dest)) == myConnections.end()) {
    //         myConnections.push_back(Connection(-1, dest, -1));
    //         myConnections.back().permissions = permissions;
    //     }
    //     if (overrideRemoval) {
    //         // override earlier delete decision
    //         for (std::vector<Connection>::iterator it = myConnectionsToDelete.begin(); it != myConnectionsToDelete.end();) {
    //             if (it->toEdge == dest) {
    //                 it = myConnectionsToDelete.erase(it);
    //             } else {
    //                 it++;
    //             }
    //         }
    //     }
    //     if (myStep < EdgeBuildingStep::EDGE2EDGES) {
    //         myStep = EdgeBuildingStep::EDGE2EDGES;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_edge2_edge_connection(&mut self, dest: Ptr<NBEdge>, overrideRemoval: bool, permission: SVCPermissions) -> bool {
        todo!("port NBEdge::addEdge2EdgeConnection")
    }

    /// C++: `bool NBEdge::addLane2LaneConnection(int, NBEdge *, int, Lane2LaneInfoType, bool, bool, KeepClear, double, double, double, double, double, const PositionVector &, const bool, SVCPermissions, const bool, const std::string &, SVCPermissions, SVCPermissions, bool)`
    /// Adds a connection between the specified this edge's lane and an approached one
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_lane2_lane_connection(&mut self, fromLane: i32, dest: Ptr<NBEdge>, toLane: i32, type_: NBEdge_Lane2LaneInfoType, mayUseSameDestination: bool, mayDefinitelyPass: bool, keepClear: KeepClear, contPos: f64, visibility: f64, speed: f64, friction: f64, length: f64, customShape: &PositionVector, uncontrolled: bool, permissions: SVCPermissions, indirectLeft: bool, edgeType: &str, changeLeft: SVCPermissions, changeRight: SVCPermissions, postProcess: bool) -> bool {
        todo!("port NBEdge::addLane2LaneConnection")
    }

    /// C++: `bool NBEdge::addLane2LaneConnections(int, NBEdge *, int, int, Lane2LaneInfoType, bool, bool)`
    /// Builds no connections starting at the given lanes
    // bool
    // NBEdge::addLane2LaneConnections(int fromLane,
    //                                 NBEdge* dest, int toLane,
    //                                 int no, Lane2LaneInfoType type,
    //                                 bool invalidatePrevious,
    //                                 bool mayDefinitelyPass) {
    //     if (invalidatePrevious) {
    //         invalidateConnections(true);
    //     }
    //     bool ok = true;
    //     for (int i = 0; i < no && ok; i++) {
    //         ok &= addLane2LaneConnection(fromLane + i, dest, toLane + i, type, false, mayDefinitelyPass);
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn add_lane2_lane_connections(&mut self, fromLane: i32, dest: Ptr<NBEdge>, toLane: i32, no: i32, type_: NBEdge_Lane2LaneInfoType, invalidatePrevious: bool, mayDefinitelyPass: bool) -> bool {
        todo!("port NBEdge::addLane2LaneConnections")
    }

    /// C++: `bool NBEdge::setConnection(int, NBEdge *, int, Lane2LaneInfoType, bool, bool, KeepClear, double, double, double, double, double, const PositionVector &, const bool, SVCPermissions, bool, const std::string &, SVCPermissions, SVCPermissions, bool)`
    /// Adds a connection to a certain lane of a certain edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_connection(&mut self, lane: i32, destEdge: Ptr<NBEdge>, destLane: i32, type_: NBEdge_Lane2LaneInfoType, mayUseSameDestination: bool, mayDefinitelyPass: bool, keepClear: KeepClear, contPos: f64, visibility: f64, speed: f64, friction: f64, length: f64, customShape: &PositionVector, uncontrolled: bool, permissions: SVCPermissions, indirectLeft: bool, edgeType: &str, changeLeft: SVCPermissions, changeRight: SVCPermissions, postProcess: bool) -> bool {
        todo!("port NBEdge::setConnection")
    }

    /// C++: `std::vector<Connection> NBEdge::getConnectionsFromLane(int, const NBEdge *, int) const` [const]
    /// Returns connections from a given lane
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_connections_from_lane(&self, lane: i32, to: Ptr<NBEdge>, toLane: i32) -> Vec<NBEdge_Connection> {
        todo!("port NBEdge::getConnectionsFromLane")
    }

    /// C++: `const Connection & NBEdge::getConnection(int, const NBEdge *, int) const` [const]
    /// Returns the specified connection (unmodifiable) This method goes through "myConnections" and returns the specified one
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_connection(&self, fromLane: i32, to: Ptr<NBEdge>, toLane: i32) -> &NBEdge_Connection {
        todo!("port NBEdge::getConnection")
    }

    /// C++: `Connection & NBEdge::getConnectionRef(int, const NBEdge *, int)`
    /// Returns reference to the specified connection This method goes through "myConnections" and returns the specified one
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_connection_ref(&mut self, fromLane: i32, to: Ptr<NBEdge>, toLane: i32) -> &mut NBEdge_Connection {
        todo!("port NBEdge::getConnectionRef")
    }

    /// C++: `bool NBEdge::hasConnectionTo(const NBEdge *, int, int) const` [const]
    /// Retrieves info about a connection to a certain lane of a certain edge
    // bool
    // NBEdge::hasConnectionTo(const NBEdge* destEdge, int destLane, int fromLane) const {
    //     return destEdge != nullptr && find_if(myConnections.begin(), myConnections.end(), connections_toedgelane_finder(destEdge, destLane, fromLane)) != myConnections.end();
    // }
    #[allow(unused_variables)]
    pub fn has_connection_to(&self, destEdge: Ptr<NBEdge>, destLane: i32, fromLane: i32) -> bool {
        todo!("port NBEdge::hasConnectionTo")
    }

    /// C++: `bool NBEdge::isConnectedTo(const NBEdge *, const bool) const` [const]
    /// Returns the information whethe a connection to the given edge has been added (or computed)
    // bool
    // NBEdge::isConnectedTo(const NBEdge* e, const bool ignoreTurnaround) const {
    //     if (!ignoreTurnaround && (e == myTurnDestination)) {
    //         return true;
    //     }
    //     return
    //         find_if(myConnections.begin(), myConnections.end(), connections_toedge_finder(e))
    //         !=
    //         myConnections.end();
    //
    // }
    #[allow(unused_variables)]
    pub fn is_connected_to(&self, e: Ptr<NBEdge>, ignoreTurnaround: bool) -> bool {
        todo!("port NBEdge::isConnectedTo")
    }

    /// C++: `const std::vector<Connection> & NBEdge::getConnections() const` [const]
    /// Returns the connections
    // const std::vector<Connection>& getConnections() const {
    //         return myConnections;
    //     }
    #[allow(unused_variables)]
    pub fn get_connections(&self) -> &Vec<NBEdge_Connection> {
        todo!("port NBEdge::getConnections")
    }

    /// C++: `std::vector<Connection> & NBEdge::getConnections()`
    /// Returns the connections
    // std::vector<Connection>& getConnections() {
    //         return myConnections;
    //     }
    #[allow(unused_variables)]
    pub fn get_connections_2(&mut self) -> &mut Vec<NBEdge_Connection> {
        todo!("port NBEdge::getConnections")
    }

    /// C++: `const EdgeVector * NBEdge::getConnectedSorted()`
    /// Returns the list of outgoing edges without the turnaround sorted in clockwise direction
    // const EdgeVector*
    // NBEdge::getConnectedSorted() {
    //     // check whether connections exist and if not, use edges from the node
    //     EdgeVector outgoing;
    //     if (myConnections.size() == 0) {
    //         outgoing = myTo->getOutgoingEdges();
    //     } else {
    //         for (std::vector<Connection>::const_iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //             if (find(outgoing.begin(), outgoing.end(), (*i).toEdge) == outgoing.end()) {
    //                 outgoing.push_back((*i).toEdge);
    //             }
    //         }
    //     }
    //     for (std::vector<Connection>::iterator it = myConnectionsToDelete.begin(); it != myConnectionsToDelete.end(); ++it) {
    //         if (it->fromLane < 0 && it->toLane < 0) {
    //             // found an edge that shall not be connected
    //             EdgeVector::iterator forbidden = std::find(outgoing.begin(), outgoing.end(), it->toEdge);
    //             if (forbidden != outgoing.end()) {
    //                 outgoing.erase(forbidden);
    //             }
    //         }
    //     }
    //     // allocate the sorted container
    //     int size = (int) outgoing.size();
    //     EdgeVector* edges = new EdgeVector();
    //     edges->reserve(size);
    //     for (EdgeVector::const_iterator i = outgoing.begin(); i != outgoing.end(); i++) {
    //         NBEdge* outedge = *i;
    //         if (outedge != nullptr && outedge != myTurnDestination) {
    //             edges->push_back(outedge);
    //         }
    //     }
    //     std::sort(edges->begin(), edges->end(), NBContHelper::relative_outgoing_edge_sorter(this));
    //     return edges;
    // }
    #[allow(unused_variables)]
    pub fn get_connected_sorted(&mut self) -> Ptr<EdgeVector> {
        todo!("port NBEdge::getConnectedSorted")
    }

    /// C++: `EdgeVector NBEdge::getConnectedEdges() const` [const]
    /// Returns the list of outgoing edges unsorted
    // EdgeVector
    // NBEdge::getConnectedEdges() const {
    //     EdgeVector ret;
    //     for (std::vector<Connection>::const_iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if (find(ret.begin(), ret.end(), (*i).toEdge) == ret.end()) {
    //             ret.push_back((*i).toEdge);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_connected_edges(&self) -> EdgeVector {
        todo!("port NBEdge::getConnectedEdges")
    }

    /// C++: `EdgeVector NBEdge::getIncomingEdges() const` [const]
    /// Returns the list of incoming edges unsorted
    // EdgeVector
    // NBEdge::getIncomingEdges() const {
    //     EdgeVector ret;
    //     const EdgeVector& candidates = myFrom->getIncomingEdges();
    //     for (EdgeVector::const_iterator i = candidates.begin(); i != candidates.end(); i++) {
    //         if ((*i)->isConnectedTo(this)) {
    //             ret.push_back(*i);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_edges(&self) -> EdgeVector {
        todo!("port NBEdge::getIncomingEdges")
    }

    /// C++: `std::vector<int> NBEdge::getConnectionLanes(NBEdge *, bool, bool) const` [const]
    /// Returns the list of lanes that may be used to reach the given edge
    // std::vector<int>
    // NBEdge::getConnectionLanes(NBEdge* currentOutgoing, bool withBikes, bool withBusLanes) const {
    //     std::vector<int> ret;
    //     if (currentOutgoing != myTurnDestination) {
    //         for (const Connection& c : myConnections) {
    //             if (c.toEdge == currentOutgoing
    //                     && (withBikes || getPermissions(c.fromLane) != SVC_BICYCLE)
    //                     && (withBusLanes || getPermissions(c.fromLane) != SVC_BUS)) {
    //                 ret.push_back(c.fromLane);
    //             }
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_connection_lanes(&self, currentOutgoing: Ptr<NBEdge>, withBikes: bool, withBusLanes: bool) -> Vec<i32> {
        todo!("port NBEdge::getConnectionLanes")
    }

    /// C++: `void NBEdge::sortOutgoingConnectionsByAngle()`
    /// sorts the outgoing connections by their angle relative to their junction
    // void
    // NBEdge::sortOutgoingConnectionsByAngle() {
    //     sort(myConnections.begin(), myConnections.end(), connections_relative_edgelane_sorter(this));
    // }
    #[allow(unused_variables)]
    pub fn sort_outgoing_connections_by_angle(&mut self) {
        todo!("port NBEdge::sortOutgoingConnectionsByAngle")
    }

    /// C++: `void NBEdge::sortOutgoingConnectionsByIndex()`
    /// sorts the outgoing connections by their from-lane-index and their to-lane-index
    // void
    // NBEdge::sortOutgoingConnectionsByIndex() {
    //     sort(myConnections.begin(), myConnections.end(), connections_sorter);
    // }
    #[allow(unused_variables)]
    pub fn sort_outgoing_connections_by_index(&mut self) {
        todo!("port NBEdge::sortOutgoingConnectionsByIndex")
    }

    /// C++: `void NBEdge::remapConnections(const EdgeVector &)`
    /// Remaps the connection in a way that allows the removal of it
    // void
    // NBEdge::remapConnections(const EdgeVector& incoming) {
    //     EdgeVector connected = getConnectedEdges();
    //     for (EdgeVector::const_iterator i = incoming.begin(); i != incoming.end(); i++) {
    //         NBEdge* inc = *i;
    //         // We have to do this
    //         inc->myStep = EdgeBuildingStep::EDGE2EDGES;
    //         // add all connections
    //         for (EdgeVector::iterator j = connected.begin(); j != connected.end(); j++) {
    //             inc->addEdge2EdgeConnection(*j);
    //         }
    //         inc->removeFromConnections(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remap_connections(&mut self, incoming: &EdgeVector) {
        todo!("port NBEdge::remapConnections")
    }

    /// C++: `void NBEdge::removeFromConnections(NBEdge *, int, int, bool, const bool, const bool)`
    /// Removes the specified connection(s)
    // void
    // NBEdge::removeFromConnections(NBEdge* toEdge, int fromLane, int toLane, bool tryLater, const bool adaptToLaneRemoval,
    //                               const bool keepPossibleTurns) {
    //     // remove from "myConnections"
    //     const int fromLaneRemoved = adaptToLaneRemoval && fromLane >= 0 ? fromLane : -1;
    //     const int toLaneRemoved = adaptToLaneRemoval && toLane >= 0 ? toLane : -1;
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         Connection& c = *i;
    //         if ((toEdge == nullptr || c.toEdge == toEdge)
    //                 && (fromLane < 0 || c.fromLane == fromLane)
    //                 && (toLane < 0 || c.toLane == toLane)) {
    //             if (myTo->isTLControlled()) {
    //                 std::set<NBTrafficLightDefinition*> tldefs = myTo->getControllingTLS();
    //                 for (std::set<NBTrafficLightDefinition*>::iterator it = tldefs.begin(); it != tldefs.end(); it++) {
    //                     (*it)->removeConnection(NBConnection(this, c.fromLane, c.toEdge, c.toLane));
    //                 }
    //             }
    //             i = myConnections.erase(i);
    //             tryLater = false;
    //         } else {
    //             if (fromLaneRemoved >= 0 && c.fromLane > fromLaneRemoved) {
    //                 if (myTo->isTLControlled()) {
    //                     std::set<NBTrafficLightDefinition*> tldefs = myTo->getControllingTLS();
    //                     for (std::set<NBTrafficLightDefinition*>::iterator it = tldefs.begin(); it != tldefs.end(); it++) {
    //                         for (NBConnectionVector::iterator tlcon = (*it)->getControlledLinks().begin(); tlcon != (*it)->getControlledLinks().end(); ++tlcon) {
    //                             NBConnection& tc = *tlcon;
    //                             if (tc.getTo() == c.toEdge && tc.getFromLane() == c.fromLane && tc.getToLane() == c.toLane) {
    //                                 tc.shiftLaneIndex(this, -1);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 //std::cout << getID() << " removeFromConnections fromLane=" << fromLane << " to=" << Named::getIDSecure(toEdge) << " toLane=" << toLane << " reduceFromLane=" << c.fromLane << " (to=" << c.toLane << ")\n";
    //                 c.fromLane--;
    //             }
    //             if (toLaneRemoved >= 0 && c.toLane > toLaneRemoved && (toEdge == nullptr || c.toEdge == toEdge)) {
    //                 //std::cout << getID() << " removeFromConnections fromLane=" << fromLane << " to=" << Named::getIDSecure(toEdge) << " toLane=" << toLane << " reduceToLane=" << c.toLane << " (from=" << c.fromLane << ")\n";
    //                 c.toLane--;
    //             }
    //             ++i;
    //         }
    //     }
    //     // check whether it was the turn destination
    //     if (myTurnDestination == toEdge && fromLane < 0) {
    //         myTurnDestination = nullptr;
    //     }
    //     if (myPossibleTurnDestination == toEdge && fromLane < 0 && !keepPossibleTurns) {
    //         myPossibleTurnDestination = nullptr;
    //     }
    //     if (tryLater) {
    //         myConnectionsToDelete.push_back(Connection(fromLane, toEdge, toLane));
    // #ifdef DEBUG_CONNECTION_GUESSING
    //         if (DEBUGCOND) {
    //             std::cout << "removeFromConnections " << getID() << "_" << fromLane << "->" << toEdge->getID() << "_" << toLane << "\n";
    //             for (Connection& c : myConnections) {
    //                 std::cout << "  conn " << c.getDescription(this) << "\n";
    //             }
    //             for (Connection& c : myConnectionsToDelete) {
    //                 std::cout << "  connToDelete " << c.getDescription(this) << "\n";
    //             }
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_from_connections(&mut self, toEdge: Ptr<NBEdge>, fromLane: i32, toLane: i32, tryLater: bool, adaptToLaneRemoval: bool, keepPossibleTurns: bool) {
        todo!("port NBEdge::removeFromConnections")
    }

    /// C++: `bool NBEdge::removeFromConnections(const NBEdge::Connection &)`
    /// remove an existent connection of edge
    // bool
    // NBEdge::removeFromConnections(const NBEdge::Connection& connectionToRemove) {
    //     // iterate over connections
    //     for (auto i = myConnections.begin(); i !=  myConnections.end(); i++) {
    //         if ((i->toEdge == connectionToRemove.toEdge) && (i->fromLane == connectionToRemove.fromLane) && (i->toLane == connectionToRemove.toLane)) {
    //             // remove connection
    //             myConnections.erase(i);
    //             return true;
    //         }
    //     }
    //     // assert(false);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn remove_from_connections_2(&mut self, connectionToRemove: &NBEdge_Connection) -> bool {
        todo!("port NBEdge::removeFromConnections")
    }

    /// C++: `void NBEdge::invalidateConnections(bool)`
    /// invalidate current connections of edge
    // void
    // NBEdge::invalidateConnections(bool reallowSetting) {
    //     myTurnDestination = nullptr;
    //     myConnections.clear();
    //     if (reallowSetting) {
    //         myStep = EdgeBuildingStep::INIT;
    //     } else {
    //         myStep = EdgeBuildingStep::INIT_REJECT_CONNECTIONS;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn invalidate_connections(&mut self, reallowSetting: bool) {
        todo!("port NBEdge::invalidateConnections")
    }

    /// C++: `void NBEdge::replaceInConnections(NBEdge *, NBEdge *, int)`
    /// replace in current connections of edge
    // void
    // NBEdge::replaceInConnections(NBEdge* which, NBEdge* by, int laneOff) {
    //     // replace in "_connectedEdges"
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if ((*i).toEdge == which) {
    //             (*i).toEdge = by;
    //             (*i).toLane += laneOff;
    //         }
    //     }
    //     // check whether it was the turn destination
    //     if (myTurnDestination == which) {
    //         myTurnDestination = by;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_in_connections(&mut self, which: Ptr<NBEdge>, by: Ptr<NBEdge>, laneOff: i32) {
        todo!("port NBEdge::replaceInConnections")
    }

    /// C++: `void NBEdge::replaceInConnections(NBEdge *, const std::vector<NBEdge::Connection> &)`
    /// replace in current connections of edge
    // void
    // NBEdge::replaceInConnections(NBEdge* which, const std::vector<NBEdge::Connection>& origConns) {
    //     std::map<int, int> laneMap;
    //     int minLane = -1;
    //     int maxLane = -1;
    //     // get lanes used to approach the edge to remap
    //     bool wasConnected = false;
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if ((*i).toEdge != which) {
    //             continue;
    //         }
    //         wasConnected = true;
    //         if ((*i).fromLane != -1) {
    //             int fromLane = (*i).fromLane;
    //             laneMap[(*i).toLane] = fromLane;
    //             if (minLane == -1 || minLane > fromLane) {
    //                 minLane = fromLane;
    //             }
    //             if (maxLane == -1 || maxLane < fromLane) {
    //                 maxLane = fromLane;
    //             }
    //         }
    //     }
    //     if (!wasConnected) {
    //         return;
    //     }
    //     // add new connections
    //     std::vector<NBEdge::Connection> conns = origConns;
    //     EdgeVector origTargets = getSuccessors();
    //     for (std::vector<NBEdge::Connection>::iterator i = conns.begin(); i != conns.end(); ++i) {
    //         if ((*i).toEdge == which || (*i).toEdge == this
    //                 // if we already have connections to the target edge, do not add new ones as they are probably from a circular replacement
    //                 || std::find(origTargets.begin(), origTargets.end(), (*i).toEdge) != origTargets.end()) {
    // #ifdef DEBUG_REPLACECONNECTION
    //             if (DEBUGCOND) {
    //                 std::cout << " replaceInConnections edge=" << getID() << " which=" << which->getID()
    //                           << " origTargets=" << toString(origTargets) << " newTarget=" << i->toEdge->getID() << " skipped\n";
    //             }
    // #endif
    //             continue;
    //         }
    //         if (which->getStep() == EdgeBuildingStep::EDGE2EDGES) {
    //             // do not set lane-level connections
    //             replaceInConnections(which, (*i).toEdge, 0);
    //             continue;
    //         }
    //         int fromLane = (*i).fromLane;
    //         int toUse = -1;
    //         if (laneMap.find(fromLane) == laneMap.end()) {
    //             if (fromLane >= 0 && fromLane <= minLane) {
    //                 toUse = minLane;
    //                 // patch laneMap to avoid crossed-over connections
    //                 for (auto& item : laneMap) {
    //                     if (item.first < fromLane) {
    //                         item.second = MIN2(item.second, minLane);
    //                     }
    //                 }
    //             }
    //             if (fromLane >= 0 && fromLane >= maxLane) {
    //                 toUse = maxLane;
    //                 // patch laneMap to avoid crossed-over connections
    //                 for (auto& item : laneMap) {
    //                     if (item.first > fromLane) {
    //                         item.second = MAX2(item.second, maxLane);
    //                     }
    //                 }
    //             }
    //         } else {
    //             toUse = laneMap[fromLane];
    //         }
    //         if (toUse == -1) {
    //             toUse = 0;
    //         }
    // #ifdef DEBUG_REPLACECONNECTION
    //         if (DEBUGCOND) {
    //             std::cout  << " replaceInConnections edge=" << getID() << " which=" << which->getID() << " origTargets=" << toString(origTargets)
    //                        << " origFrom=" << fromLane << " laneMap=" << joinToString(laneMap, ":", ",") << " minLane=" << minLane << " maxLane=" << maxLane
    //                        << " newTarget=" << i->toEdge->getID() << " fromLane=" << toUse << " toLane=" << i->toLane << "\n";
    //         }
    // #endif
    //         setConnection(toUse, i->toEdge, i->toLane, Lane2LaneInfoType::COMPUTED, false, i->mayDefinitelyPass, i->keepClear,
    //                       i->contPos, i->visibility, i->speed, i->friction, i->customLength, i->customShape, i->uncontrolled);
    //     }
    //     // remove the remapped edge from connections
    //     removeFromConnections(which);
    // }
    #[allow(unused_variables)]
    pub fn replace_in_connections_2(&mut self, which: Ptr<NBEdge>, origConns: &Vec<NBEdge_Connection>) {
        todo!("port NBEdge::replaceInConnections")
    }

    /// C++: `void NBEdge::copyConnectionsFrom(NBEdge *)`
    /// copy connections from antoher edge
    // void
    // NBEdge::copyConnectionsFrom(NBEdge* src) {
    //     myStep = src->myStep;
    //     myConnections = src->myConnections;
    // }
    #[allow(unused_variables)]
    pub fn copy_connections_from(&mut self, src: Ptr<NBEdge>) {
        todo!("port NBEdge::copyConnectionsFrom")
    }

    /// C++: `void NBEdge::shiftToLanesToEdge(NBEdge *, int)`
    /// modifify the toLane for all connections to the given edge
    // void
    // NBEdge::shiftToLanesToEdge(NBEdge* to, int laneOff) {
    //     /// XXX could we repurpose the function replaceInConnections ?
    //     for (std::vector<Connection>::iterator it = myConnections.begin(); it != myConnections.end(); ++it) {
    //         if ((*it).toEdge == to && (*it).toLane >= 0) {
    //             (*it).toLane += laneOff;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn shift_to_lanes_to_edge(&mut self, to: Ptr<NBEdge>, laneOff: i32) {
        todo!("port NBEdge::shiftToLanesToEdge")
    }

    /// C++: `bool NBEdge::isTurningDirectionAt(const NBEdge *const) const` [const]
    /// Returns whether the given edge is the opposite direction to this edge
    // bool
    // NBEdge::isTurningDirectionAt(const NBEdge* const edge) const {
    //     // maybe it was already set as the turning direction
    //     if (edge == myTurnDestination) {
    //         return true;
    //     } else if (myTurnDestination != nullptr) {
    //         // otherwise - it's not if a turning direction exists
    //         return false;
    //     }
    //     return edge == myPossibleTurnDestination;
    // }
    #[allow(unused_variables)]
    pub fn is_turning_direction_at(&self, edge: Ptr<NBEdge>) -> bool {
        todo!("port NBEdge::isTurningDirectionAt")
    }

    /// C++: `void NBEdge::setTurningDestination(NBEdge *, bool)`
    /// Sets the turing destination at the given edge
    // void
    // NBEdge::setTurningDestination(NBEdge* e, bool onlyPossible) {
    //     if (!onlyPossible) {
    //         myTurnDestination = e;
    //     }
    //     myPossibleTurnDestination = e;
    // }
    #[allow(unused_variables)]
    pub fn set_turning_destination(&mut self, e: Ptr<NBEdge>, onlyPossible: bool) {
        todo!("port NBEdge::setTurningDestination")
    }

    /// C++: `void NBEdge::setAsMacroscopicConnector()`
    /// Marks this edge as a macroscopic connector
    // void setAsMacroscopicConnector() {
    //         myAmMacroscopicConnector = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_as_macroscopic_connector(&mut self) {
        todo!("port NBEdge::setAsMacroscopicConnector")
    }

    /// C++: `bool NBEdge::isMacroscopicConnector() const` [const]
    /// Returns whether this edge was marked as a macroscopic connector
    // bool isMacroscopicConnector() const {
    //         return myAmMacroscopicConnector;
    //     }
    #[allow(unused_variables)]
    pub fn is_macroscopic_connector(&self) -> bool {
        todo!("port NBEdge::isMacroscopicConnector")
    }

    /// C++: `void NBEdge::setInsideTLS(bool)`
    /// Marks this edge being within an intersection
    // void setInsideTLS(bool inside) {
    //         myAmInTLS = inside;
    //     }
    #[allow(unused_variables)]
    pub fn set_inside_tls(&mut self, inside: bool) {
        todo!("port NBEdge::setInsideTLS")
    }

    /// C++: `bool NBEdge::isInsideTLS() const` [const]
    /// Returns whether this edge was marked as being within an intersection
    // bool isInsideTLS() const {
    //         return myAmInTLS;
    //     }
    #[allow(unused_variables)]
    pub fn is_inside_tls(&self) -> bool {
        todo!("port NBEdge::isInsideTLS")
    }

    /// C++: `void NBEdge::setJunctionPriority(const NBNode *const, int)`
    /// Sets the junction priority of the edge
    // void
    // NBEdge::setJunctionPriority(const NBNode* const node, int prio) {
    //     if (node == myFrom) {
    //         myFromJunctionPriority = prio;
    // #ifdef DEBUG_JUNCTIONPRIO
    //         setParameter("fromPrio", toString(prio));
    // #endif
    //     } else {
    //         myToJunctionPriority = prio;
    // #ifdef DEBUG_JUNCTIONPRIO
    //         setParameter("toPrio", toString(prio));
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_priority(&mut self, node: Ptr<NBNode>, prio: i32) {
        todo!("port NBEdge::setJunctionPriority")
    }

    /// C++: `int NBEdge::getJunctionPriority(const NBNode *const) const` [const]
    /// Returns the junction priority (normalised for the node currently build)
    // int
    // NBEdge::getJunctionPriority(const NBNode* const node) const {
    //     if (node == myFrom) {
    //         return myFromJunctionPriority;
    //     } else {
    //         return myToJunctionPriority;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_junction_priority(&self, node: Ptr<NBNode>) -> i32 {
        todo!("port NBEdge::getJunctionPriority")
    }

    /// C++: `void NBEdge::setLoadedLength(double)`
    /// set loaded length
    // void
    // NBEdge::setLoadedLength(double val) {
    //     myLoadedLength = val;
    // }
    #[allow(unused_variables)]
    pub fn set_loaded_length(&mut self, val: f64) {
        todo!("port NBEdge::setLoadedLength")
    }

    /// C++: `void NBEdge::setAverageLengthWithOpposite(double)`
    /// patch average lane length in regard to the opposite edge
    // void
    // NBEdge::setAverageLengthWithOpposite(double val) {
    //     myLength = val;
    // }
    #[allow(unused_variables)]
    pub fn set_average_length_with_opposite(&mut self, val: f64) {
        todo!("port NBEdge::setAverageLengthWithOpposite")
    }

    /// C++: `void NBEdge::dismissVehicleClassInformation()`
    /// dimiss vehicle class information
    // void
    // NBEdge::dismissVehicleClassInformation() {
    //     for (std::vector<Lane>::iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         (*i).permissions = SVCAll;
    //         (*i).preferred = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dismiss_vehicle_class_information(&mut self) {
        todo!("port NBEdge::dismissVehicleClassInformation")
    }

    /// C++: `const std::string & NBEdge::getTypeID() const` [const]
    /// get ID of type
    // const std::string& getTypeID() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type_id(&self) -> &str {
        todo!("port NBEdge::getTypeID")
    }

    /// C++: `bool NBEdge::needsLaneSpecificOutput() const` [const]
    /// whether at least one lane has values differing from the edges values
    // bool
    // NBEdge::needsLaneSpecificOutput() const {
    //     return (hasLaneSpecificPermissions()
    //             || hasLaneSpecificSpeed()
    //             || hasLaneSpecificWidth()
    //             || hasLaneSpecificType()
    //             || hasLaneSpecificEndOffset()
    //             || hasLaneSpecificStopOffsets()
    //             || hasAccelLane()
    //             || hasCustomLaneShape()
    //             || hasLaneParams()
    //             || prohibitsChanging()
    //             || (!myLanes.empty() && myLanes.back().oppositeID != ""));
    // }
    #[allow(unused_variables)]
    pub fn needs_lane_specific_output(&self) -> bool {
        todo!("port NBEdge::needsLaneSpecificOutput")
    }

    /// C++: `bool NBEdge::hasPermissions() const` [const]
    /// whether at least one lane has restrictions
    // bool
    // NBEdge::hasPermissions() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if ((*i).permissions != SVCAll) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_permissions(&self) -> bool {
        todo!("port NBEdge::hasPermissions")
    }

    /// C++: `bool NBEdge::hasLaneSpecificPermissions() const` [const]
    /// whether lanes differ in allowed vehicle classes
    // bool
    // NBEdge::hasLaneSpecificPermissions() const {
    //     std::vector<Lane>::const_iterator i = myLanes.begin();
    //     SVCPermissions firstLanePermissions = i->permissions;
    //     i++;
    //     for (; i != myLanes.end(); ++i) {
    //         if (i->permissions != firstLanePermissions) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_permissions(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificPermissions")
    }

    /// C++: `bool NBEdge::hasLaneSpecificSpeed() const` [const]
    /// whether lanes differ in speed
    // bool
    // NBEdge::hasLaneSpecificSpeed() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->speed != getSpeed()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_speed(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificSpeed")
    }

    /// C++: `bool NBEdge::hasLaneSpecificFriction() const` [const]
    /// whether lanes differ in friction
    // bool
    // NBEdge::hasLaneSpecificFriction() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->friction != myLanes.begin()->friction) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_friction(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificFriction")
    }

    /// C++: `bool NBEdge::hasLaneSpecificWidth() const` [const]
    /// whether lanes differ in width
    // bool
    // NBEdge::hasLaneSpecificWidth() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->width != myLanes.begin()->width) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_width(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificWidth")
    }

    /// C++: `bool NBEdge::hasLaneSpecificType() const` [const]
    /// whether lanes differ in type
    // bool
    // NBEdge::hasLaneSpecificType() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->type != myLanes.begin()->type) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_type(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificType")
    }

    /// C++: `bool NBEdge::hasLaneSpecificEndOffset() const` [const]
    /// whether lanes differ in offset
    // bool
    // NBEdge::hasLaneSpecificEndOffset() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->endOffset != myLanes.begin()->endOffset) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_end_offset(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificEndOffset")
    }

    /// C++: `bool NBEdge::hasLaneSpecificStopOffsets() const` [const]
    /// whether lanes differ in stopOffsets
    // bool
    // NBEdge::hasLaneSpecificStopOffsets() const {
    //     for (const auto& lane : myLanes) {
    //         if (lane.laneStopOffset.isDefined()) {
    //             if (myEdgeStopOffset.isDefined() || (myEdgeStopOffset != lane.laneStopOffset)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_specific_stop_offsets(&self) -> bool {
        todo!("port NBEdge::hasLaneSpecificStopOffsets")
    }

    /// C++: `bool NBEdge::hasAccelLane() const` [const]
    /// whether one of the lanes is an acceleration lane
    // bool
    // NBEdge::hasAccelLane() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->accelRamp) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_accel_lane(&self) -> bool {
        todo!("port NBEdge::hasAccelLane")
    }

    /// C++: `bool NBEdge::hasCustomLaneShape() const` [const]
    /// whether one of the lanes has a custom shape
    // bool
    // NBEdge::hasCustomLaneShape() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->customShape.size() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_custom_lane_shape(&self) -> bool {
        todo!("port NBEdge::hasCustomLaneShape")
    }

    /// C++: `bool NBEdge::hasLaneParams() const` [const]
    /// whether one of the lanes has parameters set
    // bool
    // NBEdge::hasLaneParams() const {
    //     for (std::vector<Lane>::const_iterator i = myLanes.begin(); i != myLanes.end(); ++i) {
    //         if (i->getParametersMap().size() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_lane_params(&self) -> bool {
        todo!("port NBEdge::hasLaneParams")
    }

    /// C++: `bool NBEdge::prohibitsChanging() const` [const]
    /// whether one of the lanes prohibits lane changing
    // bool
    // NBEdge::prohibitsChanging() const {
    //     for (const Lane& lane : myLanes) {
    //         if (lane.changeLeft != SVCAll || lane.changeRight != SVCAll) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn prohibits_changing(&self) -> bool {
        todo!("port NBEdge::prohibitsChanging")
    }

    /// C++: `bool NBEdge::computeEdge2Edges(bool)`
    /// computes the edge (step1: computation of approached edges)
    // bool
    // NBEdge::computeEdge2Edges(bool noLeftMovers) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "computeEdge2Edges  edge=" << getID() << " step=" << (int)myStep << " noLeftMovers=" << noLeftMovers << "\n";
    //         for (Connection& c : myConnections) {
    //             std::cout << "  conn " << c.getDescription(this) << "\n";
    //         }
    //         for (Connection& c : myConnectionsToDelete) {
    //             std::cout << "  connToDelete " << c.getDescription(this) << "\n";
    //         }
    //     }
    // #endif
    //     // return if this relationship has been build in previous steps or
    //     //  during the import
    //     if (myStep >= EdgeBuildingStep::EDGE2EDGES) {
    //         return true;
    //     }
    //     const bool fromRail = isRailway(getPermissions());
    //     for (NBEdge* out : myTo->getOutgoingEdges()) {
    //         if (noLeftMovers && myTo->isLeftMover(this, out)) {
    //             continue;
    //         }
    //         // avoid sharp railway turns
    //         if (fromRail && isRailway(out->getPermissions())) {
    //             const double angle = fabs(NBHelpers::normRelAngle(getAngleAtNode(myTo), out->getAngleAtNode(myTo)));
    //             if (angle > 150) {
    //                 continue;
    //             } else if (angle > 90) {
    //                 // possibly the junction is large enough to achieve a plausible radius:
    //                 const PositionVector& fromShape = myLanes.front().shape;
    //                 const PositionVector& toShape = out->getLanes().front().shape;
    //                 PositionVector shape = myTo->computeSmoothShape(fromShape, toShape, 5, getTurnDestination() == out, 5, 5);
    //                 const double radius = shape.length2D() / DEG2RAD(angle);
    //                 const double minRadius = (getPermissions() & SVC_TRAM) != 0 ? 20 : 80;
    //                 //std::cout << getID() << " to=" << out->getID() << " radius=" << radius << " minRadius=" << minRadius << "\n";
    //                 if (radius < minRadius) {
    //                     continue;
    //                 }
    //             }
    //         }
    //         if (out == myTurnDestination) {
    //             // will be added by appendTurnaround
    //             continue;
    //         }
    //         if ((getPermissions() & out->getPermissions() & ~SVC_PEDESTRIAN) == 0) {
    //             // no common permissions
    //             continue;
    //         }
    //         myConnections.push_back(Connection(-1, out, -1));
    //     }
    //     myStep = EdgeBuildingStep::EDGE2EDGES;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn compute_edge2_edges(&mut self, noLeftMovers: bool) -> bool {
        todo!("port NBEdge::computeEdge2Edges")
    }

    /// C++: `bool NBEdge::computeLanes2Edges()`
    /// computes the edge, step2: computation of which lanes approach the edges)
    // bool
    // NBEdge::computeLanes2Edges() {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "computeLanes2Edges edge=" << getID() << " step=" << (int)myStep << "\n";
    //         for (Connection& c : myConnections) {
    //             std::cout << "  conn " << c.getDescription(this) << "\n";
    //         }
    //         for (Connection& c : myConnectionsToDelete) {
    //             std::cout << "  connToDelete " << c.getDescription(this) << "\n";
    //         }
    //     }
    // #endif
    //     // return if this relationship has been build in previous steps or
    //     //  during the import
    //     if (myStep >= EdgeBuildingStep::LANES2EDGES) {
    //         if (myStep == EdgeBuildingStep::LANES2LANES_USER && myConnections.size() > 1) {
    //             for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end();) {
    //                 if ((*i).toEdge == nullptr) {
    //                     WRITE_WARNINGF("Inconsistent connection definitions at edge '%'.", getID());
    //                     i = myConnections.erase(i);
    //                 } else {
    //                     i++;
    //                 }
    //             }
    //         }
    //         return true;
    //     }
    //     assert(myStep == EdgeBuildingStep::EDGE2EDGES);
    //     // get list of possible outgoing edges sorted by direction clockwise
    //     //  the edge in the backward direction (turnaround) is not in the list
    //     const EdgeVector* edges = getConnectedSorted();
    //     if (myConnections.size() != 0 && edges->size() == 0) {
    //         // dead end per definition!?
    //         myConnections.clear();
    //     } else {
    //         // divide the lanes on reachable edges
    //         divideOnEdges(edges);
    //     }
    //     delete edges;
    //     myStep = EdgeBuildingStep::LANES2EDGES;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn compute_lanes2_edges(&mut self) -> bool {
        todo!("port NBEdge::computeLanes2Edges")
    }

    /// C++: `bool NBEdge::recheckLanes()`
    /// recheck whether all lanes within the edge are all right and optimises the connections once again
    // bool
    // NBEdge::recheckLanes() {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "recheckLanes (initial) edge=" << getID() << "\n";
    //         for (Connection& c : myConnections) {
    //             std::cout << "  conn " << c.getDescription(this) << "\n";
    //         }
    //         for (Connection& c : myConnectionsToDelete) {
    //             std::cout << "  connToDelete " << c.getDescription(this) << "\n";
    //         }
    //     }
    // #endif
    //     // check delayed removals
    //     for (std::vector<Connection>::iterator it = myConnectionsToDelete.begin(); it != myConnectionsToDelete.end(); ++it) {
    //         removeFromConnections(it->toEdge, it->fromLane, it->toLane, false, false, true);
    //     }
    //     std::vector<int> connNumbersPerLane(myLanes.size(), 0);
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         if ((*i).toEdge == nullptr || (*i).fromLane < 0 || (*i).toLane < 0) {
    //             i = myConnections.erase(i);
    //         } else {
    //             if ((*i).fromLane >= 0) {
    //                 ++connNumbersPerLane[(*i).fromLane];
    //             }
    //             ++i;
    //         }
    //     }
    //     if (myStep != EdgeBuildingStep::LANES2LANES_DONE && myStep != EdgeBuildingStep::LANES2LANES_USER) {
    // #ifdef DEBUG_TURNSIGNS
    //         if (myLanes.back().turnSigns != 0) {
    //             std::cout << getID() << " hasTurnSigns\n";
    //             if (myTurnSignTarget != myTo->getID()) {
    //                 std::cout << "   tst=" << myTurnSignTarget << " to=" << myTo->getID() << "\n";
    //             }
    //         }
    // #endif
    //         if (myLanes.back().turnSigns == 0 || myTurnSignTarget != myTo->getID() || !applyTurnSigns()) {
    //             // check #1:
    //             // If there is a lane with no connections and any neighbour lane has
    //             //  more than one connections, try to move one of them.
    //             // This check is only done for edges which connections were assigned
    //             //  using the standard algorithm.
    //             for (int i = 0; i < (int)myLanes.size(); i++) {
    //                 if (connNumbersPerLane[i] == 0 && !isForbidden(getPermissions(i) & ~SVC_PEDESTRIAN)) {
    //                     // dead-end lane found
    //                     bool hasDeadEnd = true;
    //                     // find lane with two connections or more to the right of the current lane
    //                     for (int i2 = i - 1; hasDeadEnd && i2 >= 0; i2--) {
    //                         if (getPermissions(i) != getPermissions(i2)) {
    //                             break;
    //                         }
    //                         if (connNumbersPerLane[i2] > 1) {
    //                             connNumbersPerLane[i2]--;
    //                             for (int i3 = i2; i3 != i; i3++) {
    //                                 moveConnectionToLeft(i3);
    //                                 sortOutgoingConnectionsByAngle();
    //                                 sortOutgoingConnectionsByIndex();
    //                             }
    //                             hasDeadEnd = false;
    //                         }
    //                     }
    //                     if (hasDeadEnd) {
    //                         // find lane with two connections or more to the left of the current lane
    //                         for (int i2 = i + 1; hasDeadEnd && i2 < getNumLanes(); i2++) {
    //                             if (getPermissions(i) != getPermissions(i2)) {
    //                                 break;
    //                             }
    //                             if (connNumbersPerLane[i2] > 1) {
    //                                 connNumbersPerLane[i2]--;
    //                                 for (int i3 = i2; i3 != i; i3--) {
    //                                     moveConnectionToRight(i3);
    //                                     sortOutgoingConnectionsByAngle();
    //                                     sortOutgoingConnectionsByIndex();
    //                                 }
    //                                 hasDeadEnd = false;
    //                             }
    //                         }
    //                     }
    //                     if (hasDeadEnd && myTo->getOutgoingEdges().size() > 1) {
    //                         int passengerLanes = 0;
    //                         int passengerTargetLanes = 0;
    //                         for (const Lane& lane : myLanes) {
    //                             if ((lane.permissions & SVC_PASSENGER) != 0) {
    //                                 passengerLanes++;
    //                             }
    //                         }
    //                         for (const NBEdge* out : myTo->getOutgoingEdges()) {
    //                             if (!isTurningDirectionAt(out)) {
    //                                 for (const Lane& lane : out->getLanes()) {
    //                                     if ((lane.permissions & SVC_PASSENGER) != 0) {
    //                                         passengerTargetLanes++;
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                         if (passengerLanes > 0 && passengerLanes <= passengerTargetLanes) {
    //                             // no need for dead-ends
    //                             if (i > 0) {
    //                                 // check if a connection to the right has a usable target to the left of its target
    //                                 std::vector<Connection> rightCons = getConnectionsFromLane(i - 1);
    //                                 if (rightCons.size() > 0) {
    //                                     const Connection& rc = rightCons.back();
    //                                     NBEdge* to = rc.toEdge;
    //                                     int toLane = rc.toLane + 1;
    //                                     if (toLane < to->getNumLanes()
    //                                             && (getPermissions(i) & ~SVC_PEDESTRIAN & to->getPermissions(toLane)) != 0
    //                                             && !hasConnectionTo(to, toLane)) {
    // #ifdef DEBUG_CONNECTION_CHECKING
    //                                         std::cout << " recheck1 setConnection " << getID() << "_" << i << "->" << to->getID() << "_" << toLane << "\n";
    // #endif
    //                                         setConnection(i, to, toLane, Lane2LaneInfoType::COMPUTED);
    //                                         hasDeadEnd = false;
    //                                         sortOutgoingConnectionsByAngle();
    //                                         sortOutgoingConnectionsByIndex();
    //                                     }
    //                                     if (hasDeadEnd) {
    //                                         // check if a connection to the right has a usable target to the right of its target
    //                                         toLane = rc.toLane - 1;
    //                                         if (toLane >= 0
    //                                                 && (getPermissions(i) & ~SVC_PEDESTRIAN & to->getPermissions(rc.toLane)) != 0
    //                                                 && (getPermissions(rc.fromLane) & ~SVC_PEDESTRIAN & to->getPermissions(toLane)) != 0
    //                                                 && !hasConnectionTo(to, toLane)) {
    //                                             // shift the right lane connection target right and connect the dead lane to the old target
    //                                             getConnectionRef(rc.fromLane, to, rc.toLane).toLane = toLane;
    // #ifdef DEBUG_CONNECTION_CHECKING
    //                                             std::cout << " recheck2 setConnection " << getID() << "_" << i << "->" << to->getID() << "_" << (toLane + 1) << "\n";
    // #endif
    //                                             setConnection(i, to, toLane + 1, Lane2LaneInfoType::COMPUTED);
    //                                             hasDeadEnd = false;
    //                                             sortOutgoingConnectionsByAngle();
    //                                             sortOutgoingConnectionsByIndex();
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                             if (hasDeadEnd && i < getNumLanes() - 1) {
    //                                 // check if a connection to the left has a usable target to the right of its target
    //                                 std::vector<Connection> leftCons = getConnectionsFromLane(i + 1);
    //                                 if (leftCons.size() > 0) {
    //                                     NBEdge* to = leftCons.front().toEdge;
    //                                     int toLane = leftCons.front().toLane - 1;
    //                                     if (toLane >= 0
    //                                             && (getPermissions(i) & ~SVC_PEDESTRIAN & to->getPermissions(toLane)) != 0
    //                                             && !hasConnectionTo(to, toLane)) {
    // #ifdef DEBUG_CONNECTION_CHECKING
    //                                         std::cout << " recheck3 setConnection " << getID() << "_" << i << "->" << to->getID() << "_" << toLane << "\n";
    // #endif
    //                                         setConnection(i, to, toLane, Lane2LaneInfoType::COMPUTED);
    //                                         hasDeadEnd = false;
    //                                         sortOutgoingConnectionsByAngle();
    //                                         sortOutgoingConnectionsByIndex();
    //                                     }
    //                                 }
    //                             }
    // #ifdef ADDITIONAL_WARNINGS
    //                             if (hasDeadEnd) {
    //                                 WRITE_WARNING("Found dead-end lane " + getLaneID(i));
    //                             }
    // #endif
    //                         }
    //                     }
    //                 }
    //             }
    //             removeInvalidConnections();
    //         }
    //     }
    //     // check involuntary dead end at "real" junctions
    //     if (getPermissions() != SVC_PEDESTRIAN) {
    //         if (myConnections.empty() && myTo->getOutgoingEdges().size() > 1 && (getPermissions() & ~SVC_PEDESTRIAN) != 0) {
    //             WRITE_WARNINGF(TL("Edge '%' is not connected to outgoing edges at junction '%'."), getID(), myTo->getID());
    //         }
    //         const EdgeVector& incoming = myFrom->getIncomingEdges();
    //         if (incoming.size() > 1) {
    //             for (int i = 0; i < (int)myLanes.size(); i++) {
    //                 if (getPermissions(i) != 0 && getPermissions(i) != SVC_PEDESTRIAN) {
    //                     bool connected = false;
    //                     for (std::vector<NBEdge*>::const_iterator in = incoming.begin(); in != incoming.end(); ++in) {
    //                         if ((*in)->hasConnectionTo(this, i)) {
    //                             connected = true;
    //                             break;
    //                         }
    //                     }
    //                     if (!connected) {
    //                         WRITE_WARNINGF(TL("Lane '%' is not connected from any incoming edge at junction '%'."), getLaneID(i), myFrom->getID());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // avoid deadend due to change prohibitions
    //     if (getNumLanes() > 1 && myConnections.size() > 0) {
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             Lane& lane = myLanes[i];
    //             if ((connNumbersPerLane[i] == 0 || ((lane.accelRamp || (i > 0 && myLanes[i - 1].accelRamp && connNumbersPerLane[i - 1] > 0))
    //                                                 && getSuccessors(SVC_PASSENGER).size() > 1))
    //                     && getPermissions(i) != SVC_PEDESTRIAN && !isForbidden(getPermissions(i))) {
    //                 const bool forbiddenLeft = lane.changeLeft != SVCAll && lane.changeLeft != SVC_IGNORING && lane.changeLeft != SVC_UNSPECIFIED;
    //                 const bool forbiddenRight = lane.changeRight != SVCAll && lane.changeRight != SVC_IGNORING && lane.changeRight != SVC_UNSPECIFIED;
    //                 if (forbiddenLeft && (i == 0 || forbiddenRight)) {
    //                     lane.changeLeft = SVC_UNSPECIFIED;
    //                     WRITE_WARNINGF(TL("Ignoring changeLeft prohibition for '%' to avoid dead-end"), getLaneID(i));
    //                 } else if (forbiddenRight && (i == getNumLanes() - 1 || (i > 0 && myLanes[i - 1].accelRamp))) {
    //                     lane.changeRight = SVC_UNSPECIFIED;
    //                     WRITE_WARNINGF(TL("Ignoring changeRight prohibition for '%' to avoid dead-end"), getLaneID(i));
    //                 }
    //             }
    //         }
    //     }
    // #ifdef ADDITIONAL_WARNINGS
    //     // check for connections with bad access permissions
    //     for (const Connection& c : myConnections) {
    //         SVCPermissions fromP = getPermissions(c.fromLane);
    //         SVCPermissions toP = c.toEdge->getPermissions(c.toLane);
    //         if ((fromP & SVC_PASSENGER) != 0
    //                 && toP == SVC_BICYCLE) {
    //             bool hasAlternative = false;
    //             for (const Connection& c2 : myConnections) {
    //                 if (c.fromLane == c2.fromLane && c.toEdge == c2.toEdge
    //                         && (c.toEdge->getPermissions(c2.toLane) & SVC_PASSENGER) != 0) {
    //                     hasAlternative = true;
    //                 }
    //             }
    //             if (!hasAlternative) {
    //                 WRITE_WARNING("Road lane ends on bikeLane for connection " + c.getDescription(this));
    //             }
    //         }
    //     }
    //
    // #endif
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "recheckLanes (final) edge=" << getID() << "\n";
    //         for (Connection& c : myConnections) {
    //             std::cout << "  conn " << c.getDescription(this) << "\n";
    //         }
    //     }
    // #endif
    //     if (myStep != EdgeBuildingStep::LANES2LANES_USER) {
    //         myStep = EdgeBuildingStep::LANES2LANES_DONE;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn recheck_lanes(&mut self) -> bool {
        todo!("port NBEdge::recheckLanes")
    }

    /// C++: `void NBEdge::recheckOpposite(const NBEdgeCont &, bool)`
    /// recheck whether all opposite and bidi settings are consistent
    // void NBEdge::recheckOpposite(const NBEdgeCont& ec, bool fixOppositeLengths) {
    //     if (getNumLanes() == 0) {
    //         return;
    //     }
    //     const int leftmostLane = getNumLanes() - 1;
    //     // check oppositeID stored in other lanes
    //     for (int i = 0; i < leftmostLane; i++) {
    //         const std::string& oppositeID = getLanes()[i].oppositeID;
    //         NBEdge* oppEdge = ec.retrieve(oppositeID.substr(0, oppositeID.rfind("_")));
    //         if (oppositeID != "" && oppositeID != "-") {
    //             if (getLanes().back().oppositeID == "" && oppEdge != nullptr) {
    //                 getLaneStruct(leftmostLane).oppositeID = oppositeID;
    //                 WRITE_WARNINGF(TL("Moving opposite lane '%' from invalid lane '%' to lane index %."), oppositeID, getLaneID(i), leftmostLane);
    //             } else {
    //                 WRITE_WARNINGF(TL("Removing opposite lane '%' for invalid lane '%'."), oppositeID, getLaneID(i));
    //             }
    //             getLaneStruct(i).oppositeID = "";
    //         }
    //     }
    //     const std::string& oppositeID = getLanes().back().oppositeID;
    //     if (oppositeID != "" && oppositeID != "-") {
    //         NBEdge* oppEdge = ec.retrieve(oppositeID.substr(0, oppositeID.rfind("_")));
    //         if (oppEdge == nullptr) {
    //             WRITE_WARNINGF(TL("Removing unknown opposite lane '%' for edge '%'."), oppositeID, getID());
    //             getLaneStruct(leftmostLane).oppositeID = "";
    //         } else {
    //             if (oppEdge->getFromNode() != getToNode() || oppEdge->getToNode() != getFromNode()) {
    //                 WRITE_WARNINGF(TL("Opposite lane '%' does not reverse-connect the same nodes as edge '%'!"), oppositeID, getID());
    //                 getLaneStruct(getNumLanes() - 1).oppositeID = "";
    //             } else {
    //                 if (oppEdge->getLaneID(oppEdge->getNumLanes() - 1) != oppositeID) {
    //                     const std::string oppEdgeLeftmost = oppEdge->getLaneID(oppEdge->getNumLanes() - 1);
    //                     WRITE_WARNINGF(TL("Adapting invalid opposite lane '%' for edge '%' to '%'."), oppositeID, getID(), oppEdgeLeftmost);
    //                     getLaneStruct(leftmostLane).oppositeID = oppEdgeLeftmost;
    //                 }
    //                 NBEdge::Lane& oppLane = oppEdge->getLaneStruct(oppEdge->getNumLanes() - 1);
    //                 const std::string leftmostID = getLaneID(leftmostLane);
    //                 if (oppLane.oppositeID == "") {
    //                     WRITE_WARNINGF(TL("Adapting missing opposite lane '%' for edge '%'."), leftmostID, oppEdge->getID());
    //                     oppLane.oppositeID = leftmostID;
    //                 } else if (oppLane.oppositeID != leftmostID && oppLane.oppositeID != "-") {
    //                     const std::string oppOpp = oppLane.oppositeID.substr(0, oppLane.oppositeID.rfind("_"));
    //                     NBEdge* oppOppEdge = ec.retrieve(oppOpp);
    //                     if (oppOppEdge == nullptr) {
    //                         WRITE_WARNINGF(TL("Adapting invalid opposite lane '%' for edge '%' to '%'."), oppLane.oppositeID, oppEdge->getID(), leftmostID);
    //                         oppLane.oppositeID = leftmostID;
    //                     } else {
    //                         if (oppEdge->getFromNode() != oppOppEdge->getToNode() || oppEdge->getToNode() != oppOppEdge->getFromNode()) {
    //                             WRITE_ERRORF(TL("Opposite edge '%' does not reverse-connect the same nodes as edge '%'!"), oppEdge->getID(), oppOppEdge->getID());
    //                         } else {
    //                             WRITE_WARNINGF(TL("Adapting inconsistent opposite lanes for edges '%', '%' and '%'."), getID(), oppEdge->getID(), oppOpp);
    //                         }
    //                         oppLane.oppositeID = leftmostID;
    //                         NBEdge::Lane& oppOppLane = oppOppEdge->getLaneStruct(oppOppEdge->getNumLanes() - 1);
    //                         if (oppOppLane.oppositeID == oppEdge->getLaneID(oppEdge->getNumLanes() - 1)) {
    //                             oppOppLane.oppositeID = "";
    //                         }
    //                     }
    //                 }
    //                 if (fabs(oppEdge->getLoadedLength() - getLoadedLength()) > NUMERICAL_EPS) {
    //                     if (fixOppositeLengths) {
    //                         const double avgLength = 0.5 * (getFinalLength() + oppEdge->getFinalLength());
    //                         WRITE_WARNINGF(TL("Averaging edge lengths for lane '%' (length %) and edge '%' (length %)."),
    //                                        oppositeID, oppEdge->getLoadedLength(), getID(), getLoadedLength());
    //                         setLoadedLength(avgLength);
    //                         oppEdge->setLoadedLength(avgLength);
    //                     } else {
    //                         WRITE_ERROR("Opposite lane '" + oppositeID + "' (length " + toString(oppEdge->getLoadedLength()) +
    //                                     ") differs in length from edge '" + getID() + "' (length " +
    //                                     toString(getLoadedLength()) + "). Set --opposites.guess.fix-lengths to fix this.");
    //                         getLaneStruct(getNumLanes() - 1).oppositeID = "";
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // check for matching bidi lane shapes (at least for the simple case of 1-lane edges)
    //     const NBEdge* bidi = getBidiEdge();
    //     if (bidi != nullptr && getNumLanes() == 1 && bidi->getNumLanes() == 1 && getID() < bidi->getID()) {
    //         getLaneStruct(0).shape = bidi->getLaneStruct(0).shape.reverse();
    //     }
    //     // check for valid offset and speed
    //     const double startOffset = isBidiRail() ? getTurnDestination(true)->getEndOffset() : 0;
    //     int i = 0;
    //     for (const NBEdge::Lane& l : getLanes()) {
    //         if (startOffset + l.endOffset > getLength()) {
    //             WRITE_WARNINGF(TL("Invalid endOffset % at lane '%' with length % (startOffset %)."),
    //                            toString(l.endOffset), getLaneID(i), toString(l.shape.length()), toString(startOffset));
    //         } else if (l.speed < 0.) {
    //             WRITE_WARNINGF(TL("Negative allowed speed (%) on lane '%', use --speed.minimum to prevent this."), toString(l.speed), getLaneID(i));
    //         } else if (l.speed == 0.) {
    //             WRITE_WARNINGF(TL("Lane '%' has a maximum allowed speed of 0."), getLaneID(i));
    //         }
    //         i++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recheck_opposite(&mut self, ec: &NBEdgeCont, fixOppositeLengths: bool) {
        todo!("port NBEdge::recheckOpposite")
    }

    /// C++: `void NBEdge::appendTurnaround(bool, bool, bool, bool, bool, bool)`
    /// Add a connection to the previously computed turnaround, if wished and a turning direction exists (myTurnDestination!=0)
    // void
    // NBEdge::appendTurnaround(bool noTLSControlled, bool noFringe, bool onlyDeadends, bool onlyTurnlane, bool noGeometryLike, bool checkPermissions) {
    //     // do nothing if no turnaround is known
    //     if (myTurnDestination == nullptr || myTo->getType() == SumoXMLNodeType::RAIL_CROSSING) {
    //         return;
    //     }
    //     // do nothing if the destination node is controlled by a tls and no turnarounds
    //     //  shall be appended for such junctions
    //     if (noTLSControlled && myTo->isTLControlled()) {
    //         return;
    //     }
    //     if (noFringe && myTo->getFringeType() == FringeType::OUTER) {
    //         return;
    //     }
    //     bool isDeadEnd = true;
    //     for (const Connection& c : myConnections) {
    //         if ((c.toEdge->getPermissions(c.toLane)
    //                 & getPermissions(c.fromLane)
    //                 & SVC_PASSENGER) != 0
    //                 || (c.toEdge->getPermissions() & getPermissions()) == getPermissions()) {
    //             isDeadEnd = false;
    //             break;
    //         }
    //     }
    //     if (onlyDeadends && !isDeadEnd) {
    //         return;
    //     }
    //     const int fromLane = getFirstAllowedLaneIndex(NBNode::BACKWARD);
    //     if (onlyTurnlane) {
    //         for (const Connection& c : getConnectionsFromLane(fromLane)) {
    //             LinkDirection dir = myTo->getDirection(this, c.toEdge);
    //             if (dir != LinkDirection::LEFT && dir != LinkDirection::PARTLEFT) {
    //                 return;
    //             }
    //         }
    //     }
    //     const int toLane = myTurnDestination->getFirstAllowedLaneIndex(NBNode::BACKWARD);
    //     if (checkPermissions) {
    //         if ((getPermissions(fromLane) & myTurnDestination->getPermissions(toLane)) == 0) {
    //             // exclude connection if fromLane and toEdge have no common permissions
    //             return;
    //         }
    //         if ((getPermissions(fromLane) & myTurnDestination->getPermissions(toLane)) == SVC_PEDESTRIAN) {
    //             // exclude connection if the only commonly permitted class are pedestrians
    //             // these connections are later built in NBNode::buildWalkingAreas
    //             return;
    //         }
    //     }
    //     // avoid railway turn-arounds
    //     if (isRailway(getPermissions() & myTurnDestination->getPermissions())
    //             && fabs(NBHelpers::normRelAngle(getAngleAtNode(myTo), myTurnDestination->getAngleAtNode(myTo))) > 90) {
    //         // except at dead-ends on bidi-edges where they model a reversal in train direction
    //         // @todo #4382: once the network fringe is tagged, it also should not receive turn-arounds)
    //         if (isBidiRail() && isRailDeadEnd()) {
    //             // add a slow connection because direction-reversal implies stopping
    //             setConnection(fromLane, myTurnDestination, toLane, Lane2LaneInfoType::VALIDATED, false, false, KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS, UNSPECIFIED_VISIBILITY_DISTANCE, SUMO_const_haltingSpeed);
    //             return;
    //         } else {
    //             return;
    //         }
    //     };
    //     if (noGeometryLike && !isDeadEnd) {
    //         // ignore paths and service entrances if this edge is for passenger traffic
    //         if (myTo->geometryLike() || ((getPermissions() & SVC_PASSENGER) != 0
    //                                      && !onlyTurnlane
    //                                      && myTo->geometryLike(
    //                                          NBEdge::filterByPermissions(myTo->getIncomingEdges(), ~(SVC_BICYCLE | SVC_PEDESTRIAN | SVC_DELIVERY)),
    //                                          NBEdge::filterByPermissions(myTo->getOutgoingEdges(), ~(SVC_BICYCLE | SVC_PEDESTRIAN | SVC_DELIVERY))))) {
    //             // make sure the turnDestination has other incoming edges
    //             EdgeVector turnIncoming = myTurnDestination->getIncomingEdges();
    //             if (turnIncoming.size() > 1) {
    //                 // this edge is always part of incoming
    //                 return;
    //             }
    //         }
    //     }
    //     setConnection(fromLane, myTurnDestination, toLane, Lane2LaneInfoType::VALIDATED);
    // }
    #[allow(unused_variables)]
    pub fn append_turnaround(&mut self, noTLSControlled: bool, noFringe: bool, onlyDeadends: bool, onlyTurnlane: bool, noGeometryLike: bool, checkPermissions: bool) {
        todo!("port NBEdge::appendTurnaround")
    }

    /// C++: `NBNode * NBEdge::tryGetNodeAtPosition(double, double) const` [const]
    /// Returns the node at the given edges length (using an epsilon)
    // NBNode*
    // NBEdge::tryGetNodeAtPosition(double pos, double tolerance) const {
    //     // return the from-node when the position is at the begin of the edge
    //     if (pos < tolerance) {
    //         return myFrom;
    //     }
    //     // return the to-node when the position is at the end of the edge
    //     if (pos > myLength - tolerance) {
    //         return myTo;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn try_get_node_at_position(&self, pos: f64, tolerance: f64) -> Ptr<NBNode> {
        todo!("port NBEdge::tryGetNodeAtPosition")
    }

    /// C++: `double NBEdge::getMaxLaneOffset()`
    /// get max lane offset
    // double
    // NBEdge::getMaxLaneOffset() {
    //     return SUMO_const_laneWidth * (double)myLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_max_lane_offset(&mut self) -> f64 {
        todo!("port NBEdge::getMaxLaneOffset")
    }

    /// C++: `bool NBEdge::lanesWereAssigned() const` [const]
    /// Check if lanes were assigned
    // bool
    // NBEdge::lanesWereAssigned() const {
    //     return myStep == EdgeBuildingStep::LANES2LANES_DONE || myStep == EdgeBuildingStep::LANES2LANES_USER;
    // }
    #[allow(unused_variables)]
    pub fn lanes_were_assigned(&self) -> bool {
        todo!("port NBEdge::lanesWereAssigned")
    }

    /// C++: `bool NBEdge::mayBeTLSControlled(int, NBEdge *, int) const` [const]
    /// return true if certain connection must be controlled by TLS
    // bool
    // NBEdge::mayBeTLSControlled(int fromLane, NBEdge* toEdge, int toLane) const {
    //     for (const Connection& c : myConnections) {
    //         if (c.fromLane == fromLane && c.toEdge == toEdge && c.toLane == toLane && c.uncontrolled) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn may_be_tls_controlled(&self, fromLane: i32, toEdge: Ptr<NBEdge>, toLane: i32) -> bool {
        todo!("port NBEdge::mayBeTLSControlled")
    }

    /// C++: `bool NBEdge::setControllingTLInformation(const NBConnection &, const std::string &)`
    /// Returns if the link could be set as to be controlled
    // bool
    // NBEdge::setControllingTLInformation(const NBConnection& c, const std::string& tlID) {
    //     const int fromLane = c.getFromLane();
    //     NBEdge* toEdge = c.getTo();
    //     const int toLane = c.getToLane();
    //     const int tlIndex = c.getTLIndex();
    //     const int tlIndex2 = c.getTLIndex2();
    //     // check whether the connection was not set as not to be controled previously
    //     if (!mayBeTLSControlled(fromLane, toEdge, toLane)) {
    //         return false;
    //     }
    //
    //     assert(fromLane < 0 || fromLane < (int) myLanes.size());
    //     // try to use information about the connections if given
    //     if (fromLane >= 0 && toLane >= 0) {
    //         // find the specified connection
    //         std::vector<Connection>::iterator i =
    //             find_if(myConnections.begin(), myConnections.end(), connections_finder(fromLane, toEdge, toLane));
    //         // ok, we have to test this as on the removal of self-loop edges some connections
    //         //  will be reassigned
    //         if (i != myConnections.end()) {
    //             // get the connection
    //             Connection& connection = *i;
    //             // set the information about the tl
    //             connection.tlID = tlID;
    //             connection.tlLinkIndex = tlIndex;
    //             connection.tlLinkIndex2 = tlIndex2;
    //             return true;
    //         }
    //     }
    //     // if the original connection was not found, set the information for all
    //     //  connections
    //     int no = 0;
    //     bool hadError = false;
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if ((*i).toEdge != toEdge) {
    //             continue;
    //         }
    //         if (fromLane >= 0 && fromLane != (*i).fromLane) {
    //             continue;
    //         }
    //         if (toLane >= 0 && toLane != (*i).toLane) {
    //             continue;
    //         }
    //         if ((*i).tlID == "") {
    //             (*i).tlID = tlID;
    //             (*i).tlLinkIndex = tlIndex;
    //             (*i).tlLinkIndex2 = tlIndex2;
    //             no++;
    //         } else {
    //             if ((*i).tlID != tlID && (*i).tlLinkIndex == tlIndex) {
    //                 WRITE_WARNINGF(TL("The lane '%' on edge '%' already had a traffic light signal."), i->fromLane, getID());
    //                 hadError = true;
    //             }
    //         }
    //     }
    //     if (hadError && no == 0) {
    //         WRITE_WARNINGF(TL("Could not set any signal of the tlLogic '%' (unknown group)."), tlID);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn set_controlling_tl_information(&mut self, c: &NBConnection, tlID: &str) -> bool {
        todo!("port NBEdge::setControllingTLInformation")
    }

    /// C++: `void NBEdge::clearControllingTLInformation()`
    /// clears tlID for all connections
    // void
    // NBEdge::clearControllingTLInformation() {
    //     for (std::vector<Connection>::iterator it = myConnections.begin(); it != myConnections.end(); it++) {
    //         it->tlID = "";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_controlling_tl_information(&mut self) {
        todo!("port NBEdge::clearControllingTLInformation")
    }

    /// C++: `PositionVector NBEdge::getCWBoundaryLine(const NBNode &) const` [const]
    /// get the outer boundary of this edge when going clock-wise around the given node
    // PositionVector
    // NBEdge::getCWBoundaryLine(const NBNode& n) const {
    //     PositionVector ret;
    //     int lane;
    //     if (myFrom == (&n)) {
    //         // outgoing
    //         lane = getFirstAllowedLaneIndex(NBNode::FORWARD);
    //         ret = myLanes[lane].shape;
    //     } else {
    //         // incoming
    //         lane = getFirstAllowedLaneIndex(NBNode::BACKWARD);
    //         ret = myLanes[lane].shape.reverse();
    //     }
    //     ret.move2side(getLaneWidth(lane) / 2.);
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_cw_boundary_line(&self, n: &NBNode) -> PositionVector {
        todo!("port NBEdge::getCWBoundaryLine")
    }

    /// C++: `PositionVector NBEdge::getCCWBoundaryLine(const NBNode &) const` [const]
    /// get the outer boundary of this edge when going counter-clock-wise around the given node
    // PositionVector
    // NBEdge::getCCWBoundaryLine(const NBNode& n) const {
    //     PositionVector ret;
    //     int lane;
    //     if (myFrom == (&n)) {
    //         // outgoing
    //         lane = getFirstAllowedLaneIndex(NBNode::BACKWARD);
    //         ret = myLanes[lane].shape;
    //     } else {
    //         // incoming
    //         lane = getFirstAllowedLaneIndex(NBNode::FORWARD);
    //         ret = myLanes[lane].shape.reverse();
    //     }
    //     ret.move2side(-getLaneWidth(lane) / 2.);
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_ccw_boundary_line(&self, n: &NBNode) -> PositionVector {
        todo!("port NBEdge::getCCWBoundaryLine")
    }

    /// C++: `bool NBEdge::expandableBy(NBEdge *, std::string &) const` [const]
    /// Check if Node is expandable
    // bool
    // NBEdge::expandableBy(NBEdge* possContinuation, std::string& reason) const {
    //     // ok, the number of lanes must match
    //     if (myLanes.size() != possContinuation->myLanes.size()) {
    //         reason = "laneNumber";
    //         return false;
    //     }
    //     // do not create self loops
    //     if (myFrom == possContinuation->myTo) {
    //         reason = "loop";
    //         return false;
    //     }
    //     // conserve bidi-rails
    //     if (isBidiRail() != possContinuation->isBidiRail()) {
    //         reason = "bidi-rail";
    //         return false;
    //     }
    //     // also, check whether the connections - if any exit do allow to join
    //     //  both edges
    //     // This edge must have a one-to-one connection to the following lanes
    //     switch (myStep) {
    //         case EdgeBuildingStep::INIT_REJECT_CONNECTIONS:
    //             break;
    //         case EdgeBuildingStep::INIT:
    //             break;
    //         case EdgeBuildingStep::EDGE2EDGES: {
    //             // the following edge must be connected
    //             const EdgeVector& conn = getConnectedEdges();
    //             if (find(conn.begin(), conn.end(), possContinuation) == conn.end()) {
    //                 reason = "disconnected";
    //                 return false;
    //             }
    //         }
    //         break;
    //         case EdgeBuildingStep::LANES2EDGES:
    //         case EdgeBuildingStep::LANES2LANES_RECHECK:
    //         case EdgeBuildingStep::LANES2LANES_DONE:
    //         case EdgeBuildingStep::LANES2LANES_USER: {
    //             // the possible continuation must be connected
    //             if (find_if(myConnections.begin(), myConnections.end(), connections_toedge_finder(possContinuation)) == myConnections.end()) {
    //                 reason = "disconnected";
    //                 return false;
    //             }
    //             // all lanes must go to the possible continuation
    //             std::vector<int> conns = getConnectionLanes(possContinuation);
    //             const int offset = MAX2(0, getFirstNonPedestrianLaneIndex(NBNode::FORWARD, true));
    //             if (conns.size() < myLanes.size() - offset) {
    //                 reason = "some lanes disconnected";
    //                 return false;
    //             }
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    //     const double minLength = OptionsCont::getOptions().getFloat("geometry.remove.min-length");
    //     if (minLength > 0 && (possContinuation->getLoadedLength() < minLength || getLoadedLength() < minLength)) {
    //         return true;
    //     }
    //     const double maxJunctionSize = OptionsCont::getOptions().getFloat("geometry.remove.max-junction-size");
    //     if (maxJunctionSize >= 0) {
    //         const double junctionSize = myGeom.back().distanceTo2D(possContinuation->myGeom.front());
    //         if (junctionSize > maxJunctionSize + POSITION_EPS) {
    //             reason = "junction size (" + toString(junctionSize) + ") > max-junction-size (" + toString(maxJunctionSize) + ")";
    //             return false;
    //         }
    //     }
    //     // the priority, too (?)
    //     if (getPriority() != possContinuation->getPriority()) {
    //         reason = "priority";
    //         return false;
    //     }
    //     // the speed allowed
    //     if (mySpeed != possContinuation->mySpeed) {
    //         reason = "speed";
    //         return false;
    //     }
    //     // the routingType
    //     if (myRoutingType != possContinuation->myRoutingType) {
    //         reason = "routingType";
    //         return false;
    //     }
    //     // spreadtype should match or it will look ugly
    //     if (myLaneSpreadFunction != possContinuation->myLaneSpreadFunction) {
    //         reason = "spreadType";
    //         return false;
    //     }
    //     // matching lanes must have identical properties
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         if (myLanes[i].speed != possContinuation->myLanes[i].speed) {
    //             reason = "lane " + toString(i) + " speed";
    //             return false;
    //         } else if (myLanes[i].permissions != possContinuation->myLanes[i].permissions) {
    //             reason = "lane " + toString(i) + " permissions";
    //             return false;
    //         } else if (myLanes[i].changeLeft != possContinuation->myLanes[i].changeLeft || myLanes[i].changeRight != possContinuation->myLanes[i].changeRight) {
    //             reason = "lane " + toString(i) + " change restrictions";
    //             return false;
    //         } else if (myLanes[i].width != possContinuation->myLanes[i].width &&
    //                    fabs(myLanes[i].width - possContinuation->myLanes[i].width) > OptionsCont::getOptions().getFloat("geometry.remove.width-tolerance")) {
    //             reason = "lane " + toString(i) + " width";
    //             return false;
    //         }
    //     }
    //     // if given identically osm names
    //     if (!OptionsCont::getOptions().isDefault("output.street-names") && myStreetName != possContinuation->getStreetName()
    //             && ((myStreetName != "" && possContinuation->getStreetName() != "")
    //                 // only permit merging a short unnamed road with a longer named road
    //                 || (myStreetName != "" && myLength <= possContinuation->getLength())
    //                 || (myStreetName == "" && myLength >= possContinuation->getLength()))) {
    //         return false;
    //     }
    //
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn expandable_by(&self, possContinuation: Ptr<NBEdge>, reason: &mut String) -> bool {
        todo!("port NBEdge::expandableBy")
    }

    /// C++: `void NBEdge::append(NBEdge *)`
    /// append another edge
    // void
    // NBEdge::append(NBEdge* e) {
    //     // append geometry
    //     myGeom.append(e->myGeom);
    //     for (int i = 0; i < (int)myLanes.size(); i++) {
    //         myLanes[i].customShape.append(e->myLanes[i].customShape);
    //         if (myLanes[i].hasParameter(SUMO_PARAM_ORIGID) || e->myLanes[i].hasParameter(SUMO_PARAM_ORIGID)
    //                 || OptionsCont::getOptions().getBool("output.original-names")) {
    //             const std::string origID = myLanes[i].getParameter(SUMO_PARAM_ORIGID, getID());
    //             const std::string origID2 = e->myLanes[i].getParameter(SUMO_PARAM_ORIGID, e->getID());
    //             if (origID != origID2) {
    //                 myLanes[i].setParameter(SUMO_PARAM_ORIGID, origID + " " + origID2);
    //             }
    //         }
    //         myLanes[i].connectionsDone = e->myLanes[i].connectionsDone;
    //         myLanes[i].turnSigns = e->myLanes[i].turnSigns;
    //     }
    //     if (e->getLength() > myLength) {
    //         // possibly some lane attributes differ (when using option geometry.remove.min-length)
    //         // make sure to use the attributes from the longer edge
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             myLanes[i].width = e->myLanes[i].width;
    //         }
    //         // defined name prevails over undefined name of shorter road
    //         if (myStreetName == "") {
    //             myStreetName = e->myStreetName;
    //         }
    //     }
    //     // recompute length
    //     myLength += e->myLength;
    //     if (myLoadedLength > 0 || e->myLoadedLength > 0) {
    //         myLoadedLength = getFinalLength() + e->getFinalLength();
    //     }
    //     // copy the connections and the building step if given
    //     myStep = e->myStep;
    //     myConnections = e->myConnections;
    //     myTurnDestination = e->myTurnDestination;
    //     myPossibleTurnDestination = e->myPossibleTurnDestination;
    //     myConnectionsToDelete = e->myConnectionsToDelete;
    //     updateRemovedNodes(e->getParameter(SUMO_PARAM_REMOVED_NODES));
    //     // set the node
    //     myTo = e->myTo;
    //     myTurnSignTarget = e->myTurnSignTarget;
    //     myToBorder = e->myToBorder;
    //     mergeParameters(e->getParametersMap());
    //     if (e->mySignalPosition != Position::INVALID) {
    //         mySignalPosition = e->mySignalPosition;
    //     }
    //     computeAngle(); // myEndAngle may be different now
    // }
    #[allow(unused_variables)]
    pub fn append(&mut self, continuation: Ptr<NBEdge>) {
        todo!("port NBEdge::append")
    }

    /// C++: `void NBEdge::updateRemovedNodes(const std::string &)`
    /// update parameter with removed nodes
    // void
    // NBEdge::updateRemovedNodes(const std::string& removed) {
    //     std::string result = getParameter(SUMO_PARAM_REMOVED_NODES);
    //     if (!result.empty() && !removed.empty()) {
    //         result += " ";
    //     }
    //     result += removed;
    //     if (!result.empty()) {
    //         setParameter(SUMO_PARAM_REMOVED_NODES, result);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_removed_nodes(&mut self, removed: &str) {
        todo!("port NBEdge::updateRemovedNodes")
    }

    /// C++: `bool NBEdge::hasSignalisedConnectionTo(const NBEdge *const) const` [const]
    /// Check if edge has signalised connections
    // bool
    // NBEdge::hasSignalisedConnectionTo(const NBEdge* const e) const {
    //     for (std::vector<Connection>::const_iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if ((*i).toEdge == e && (*i).tlID != "") {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_signalised_connection_to(&self, e: Ptr<NBEdge>) -> bool {
        todo!("port NBEdge::hasSignalisedConnectionTo")
    }

    /// C++: `void NBEdge::moveOutgoingConnectionsFrom(NBEdge *, int)`
    /// move outgoing connection
    // void
    // NBEdge::moveOutgoingConnectionsFrom(NBEdge* e, int laneOff) {
    //     int lanes = e->getNumLanes();
    //     for (int i = 0; i < lanes; i++) {
    //         for (const NBEdge::Connection& el : e->getConnectionsFromLane(i)) {
    //             assert(el.tlID == "");
    //             addLane2LaneConnection(i + laneOff, el.toEdge, el.toLane, Lane2LaneInfoType::COMPUTED);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_outgoing_connections_from(&mut self, e: Ptr<NBEdge>, laneOff: i32) {
        todo!("port NBEdge::moveOutgoingConnectionsFrom")
    }

    /// C++: `NBEdge * NBEdge::getTurnDestination(bool) const` [const]
    // NBEdge*
    // NBEdge::getTurnDestination(bool possibleDestination) const {
    //     if (myTurnDestination == nullptr && possibleDestination) {
    //         return myPossibleTurnDestination;
    //     }
    //     return myTurnDestination;
    // }
    #[allow(unused_variables)]
    pub fn get_turn_destination(&self, possibleDestination: bool) -> Ptr<NBEdge> {
        todo!("port NBEdge::getTurnDestination")
    }

    /// C++: `std::string NBEdge::getLaneID(int) const` [const]
    /// get lane ID
    // std::string
    // NBEdge::getLaneID(int lane) const {
    //     return myID + "_" + toString(lane);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(&self, lane: i32) -> String {
        todo!("port NBEdge::getLaneID")
    }

    /// C++: `double NBEdge::getLaneSpeed(int) const` [const]
    /// get lane speed
    // double
    // NBEdge::getLaneSpeed(int lane) const {
    //     return myLanes[lane].speed;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_speed(&self, lane: i32) -> f64 {
        todo!("port NBEdge::getLaneSpeed")
    }

    /// C++: `double NBEdge::getLaneFriction(int) const` [const]
    /// get lane friction of specified lane
    // double
    // NBEdge::getLaneFriction(int lane) const {
    //     return myLanes[lane].friction;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_friction(&self, lane: i32) -> f64 {
        todo!("port NBEdge::getLaneFriction")
    }

    /// C++: `bool NBEdge::isNearEnough2BeJoined2(NBEdge *, double) const` [const]
    /// Check if edge is near enough to be joined to another edge
    // bool
    // NBEdge::isNearEnough2BeJoined2(NBEdge* e, double threshold) const {
    //     std::vector<double> distances = myGeom.distances(e->getGeometry());
    //     assert(distances.size() > 0);
    //     return VectorHelper<double>::maxValue(distances) < threshold;
    // }
    #[allow(unused_variables)]
    pub fn is_near_enough2_be_joined2(&self, e: Ptr<NBEdge>, threshold: f64) -> bool {
        todo!("port NBEdge::isNearEnough2BeJoined2")
    }

    /// C++: `double NBEdge::getAngleAtNode(const NBNode *const) const` [const]
    /// Returns the angle of the edge's geometry at the given node
    // double
    // NBEdge::getAngleAtNode(const NBNode* const atNode) const {
    //     if (atNode == myFrom) {
    //         return GeomHelper::legacyDegree(myGeom.angleAt2D(0));
    //     }
    //     assert(atNode == myTo);
    //     return GeomHelper::legacyDegree(myGeom.angleAt2D(-2));
    // }
    #[allow(unused_variables)]
    pub fn get_angle_at_node(&self, node: Ptr<NBNode>) -> f64 {
        todo!("port NBEdge::getAngleAtNode")
    }

    /// C++: `double NBEdge::getAngleAtNodeNormalized(const NBNode *const) const` [const]
    /// Returns the angle of the edge's geometry at the given node and disregards edge direction
    // double
    // NBEdge::getAngleAtNodeNormalized(const NBNode* const atNode) const {
    //     double res;
    //     if (atNode == myFrom) {
    //         res = GeomHelper::legacyDegree(myGeom.angleAt2D(0)) - 180;
    //     } else {
    //         assert(atNode == myTo);
    //         res = GeomHelper::legacyDegree(myGeom.angleAt2D(-2));
    //     }
    //     if (res < 0) {
    //         res += 360;
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn get_angle_at_node_normalized(&self, node: Ptr<NBNode>) -> f64 {
        todo!("port NBEdge::getAngleAtNodeNormalized")
    }

    /// C++: `double NBEdge::getAngleAtNodeToCenter(const NBNode *const) const` [const]
    /// Returns the angle of from the node shape center to where the edge meets the node shape
    // double
    // NBEdge::getAngleAtNodeToCenter(const NBNode* const atNode) const {
    //     if (atNode == myFrom) {
    //         double res = myStartAngle - 180;
    //         if (res < 0) {
    //             res += 360;
    //         }
    //         return res;
    //     } else {
    //         assert(atNode == myTo);
    //         return myEndAngle;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_angle_at_node_to_center(&self, node: Ptr<NBNode>) -> f64 {
        todo!("port NBEdge::getAngleAtNodeToCenter")
    }

    /// C++: `void NBEdge::incLaneNo(int)`
    /// increment lane
    // void
    // NBEdge::incLaneNo(int by) {
    //     int newLaneNo = (int)myLanes.size() + by;
    //     while ((int)myLanes.size() < newLaneNo) {
    //         // recompute shapes on last addition
    //         const bool recompute = ((int)myLanes.size() == newLaneNo - 1) && myStep < EdgeBuildingStep::LANES2LANES_USER;
    //         addLane((int)myLanes.size(), recompute, recompute, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inc_lane_no(&mut self, by: i32) {
        todo!("port NBEdge::incLaneNo")
    }

    /// C++: `void NBEdge::decLaneNo(int)`
    /// decrement lane
    // void
    // NBEdge::decLaneNo(int by) {
    //     int newLaneNo = (int) myLanes.size() - by;
    //     assert(newLaneNo > 0);
    //     while ((int)myLanes.size() > newLaneNo) {
    //         // recompute shapes on last removal
    //         const bool recompute = (int)myLanes.size() == newLaneNo + 1 && myStep < EdgeBuildingStep::LANES2LANES_USER;
    //         deleteLane((int)myLanes.size() - 1, recompute, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dec_lane_no(&mut self, by: i32) {
        todo!("port NBEdge::decLaneNo")
    }

    /// C++: `void NBEdge::deleteLane(int, bool, bool)`
    /// delete lane
    // void
    // NBEdge::deleteLane(int index, bool recompute, bool shiftIndices) {
    //     assert(index < (int)myLanes.size());
    //     myLanes.erase(myLanes.begin() + index);
    //     if (recompute) {
    //         computeLaneShapes();
    //         const EdgeVector& incs = myFrom->getIncomingEdges();
    //         for (EdgeVector::const_iterator i = incs.begin(); i != incs.end(); ++i) {
    //             (*i)->invalidateConnections(true);
    //         }
    //         invalidateConnections(true);
    //     } else if (shiftIndices) {
    //         removeFromConnections(nullptr, index, -1, false, true);
    //         for (NBEdge* inc : myFrom->getIncomingEdges()) {
    //             inc->removeFromConnections(this, -1, index, false, true);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn delete_lane(&mut self, index: i32, recompute: bool, shiftIndices: bool) {
        todo!("port NBEdge::deleteLane")
    }

    /// C++: `void NBEdge::addLane(int, bool, bool, bool)`
    /// add lane
    // void
    // NBEdge::addLane(int index, bool recomputeShape, bool recomputeConnections, bool shiftIndices) {
    //     assert(index <= (int)myLanes.size());
    //     myLanes.insert(myLanes.begin() + index, Lane(this, ""));
    //     // copy attributes
    //     if (myLanes.size() > 1) {
    //         int templateIndex = index > 0 ? index - 1 : index + 1;
    //         myLanes[index].speed = myLanes[templateIndex].speed;
    //         myLanes[index].friction = myLanes[templateIndex].friction;
    //         myLanes[index].permissions = myLanes[templateIndex].permissions;
    //         myLanes[index].preferred = myLanes[templateIndex].preferred;
    //         myLanes[index].endOffset = myLanes[templateIndex].endOffset;
    //         myLanes[index].width = myLanes[templateIndex].width;
    //         myLanes[index].updateParameters(myLanes[templateIndex].getParametersMap());
    //     }
    //     const EdgeVector& incs = myFrom->getIncomingEdges();
    //     if (recomputeShape) {
    //         computeLaneShapes();
    //     }
    //     if (recomputeConnections) {
    //         for (EdgeVector::const_iterator i = incs.begin(); i != incs.end(); ++i) {
    //             (*i)->invalidateConnections(true);
    //         }
    //         invalidateConnections(true);
    //     } else if (shiftIndices) {
    //         // shift outgoing connections above the added lane to the left
    //         for (Connection& c : myConnections) {
    //             if (c.fromLane >= index) {
    //                 c.fromLane += 1;
    //             }
    //         }
    //         // shift incoming connections above the added lane to the left
    //         for (NBEdge* inc : myFrom->getIncomingEdges()) {
    //             for (Connection& c : inc->myConnections) {
    //                 if (c.toEdge == this && c.toLane >= index) {
    //                     c.toLane += 1;
    //                 }
    //             }
    //         }
    //         myFrom->shiftTLConnectionLaneIndex(this, +1, index - 1);
    //         myTo->shiftTLConnectionLaneIndex(this, +1, index - 1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, index: i32, recomputeShape: bool, recomputeConnections: bool, shiftIndices: bool) {
        todo!("port NBEdge::addLane")
    }

    /// C++: `void NBEdge::markAsInLane2LaneState()`
    /// mark edge as in lane to state lane
    // void
    // NBEdge::markAsInLane2LaneState() {
    //     assert(myTo->getOutgoingEdges().size() == 0);
    //     myStep = EdgeBuildingStep::LANES2LANES_DONE;
    // }
    #[allow(unused_variables)]
    pub fn mark_as_in_lane2_lane_state(&mut self) {
        todo!("port NBEdge::markAsInLane2LaneState")
    }

    /// C++: `void NBEdge::addSidewalk(double)`
    /// add a pedestrian sidewalk of the given width and shift existing connections
    // void
    // NBEdge::addSidewalk(double width) {
    //     addRestrictedLane(width, SVC_PEDESTRIAN);
    // }
    #[allow(unused_variables)]
    pub fn add_sidewalk(&mut self, width: f64) {
        todo!("port NBEdge::addSidewalk")
    }

    /// C++: `void NBEdge::restoreSidewalk(std::vector<NBEdge::Lane>, PositionVector, std::vector<NBEdge::Connection>)`
    /// restore an previously added sidewalk
    // void
    // NBEdge::restoreSidewalk(std::vector<NBEdge::Lane> oldLanes, PositionVector oldGeometry, std::vector<NBEdge::Connection> oldConnections) {
    //     restoreRestrictedLane(SVC_PEDESTRIAN, oldLanes, oldGeometry, oldConnections);
    // }
    #[allow(unused_variables)]
    pub fn restore_sidewalk(&mut self, oldLanes: Vec<NBEdge_Lane>, oldGeometry: PositionVector, oldConnections: Vec<NBEdge_Connection>) {
        todo!("port NBEdge::restoreSidewalk")
    }

    /// C++: `void NBEdge::addBikeLane(double)`
    /// add a bicycle lane of the given width and shift existing connections
    // void
    // NBEdge::addBikeLane(double width) {
    //     addRestrictedLane(width, SVC_BICYCLE);
    // }
    #[allow(unused_variables)]
    pub fn add_bike_lane(&mut self, width: f64) {
        todo!("port NBEdge::addBikeLane")
    }

    /// C++: `void NBEdge::restoreBikelane(std::vector<NBEdge::Lane>, PositionVector, std::vector<NBEdge::Connection>)`
    /// restore an previously added BikeLane
    // void
    // NBEdge::restoreBikelane(std::vector<NBEdge::Lane> oldLanes, PositionVector oldGeometry, std::vector<NBEdge::Connection> oldConnections) {
    //     restoreRestrictedLane(SVC_BICYCLE, oldLanes, oldGeometry, oldConnections);
    // }
    #[allow(unused_variables)]
    pub fn restore_bikelane(&mut self, oldLanes: Vec<NBEdge_Lane>, oldGeometry: PositionVector, oldConnections: Vec<NBEdge_Connection>) {
        todo!("port NBEdge::restoreBikelane")
    }

    /// C++: `void NBEdge::addRestrictedLane(double, SUMOVehicleClass)`
    /// add a lane of the given width, restricted to the given class and shift existing connections
    // void
    // NBEdge::addRestrictedLane(double width, SUMOVehicleClass vclass) {
    //     if (hasRestrictedLane(vclass)) {
    //         WRITE_WARNINGF(TL("Edge '%' already has a dedicated lane for %s. Not adding another one."), getID(), toString(vclass));
    //         return;
    //     }
    //     if (myLaneSpreadFunction == LaneSpreadFunction::CENTER) {
    //         myGeom.move2side(width / 2);
    //     }
    //     // disallow the designated vclass on all "old" lanes
    //     disallowVehicleClass(-1, vclass);
    //     // don't create a restricted vehicle lane to the right of a sidewalk
    //     const int newIndex = (vclass != SVC_PEDESTRIAN && myLanes[0].permissions == SVC_PEDESTRIAN) ? 1 : 0;
    //     if (newIndex == 0) {
    //         // disallow pedestrians on all "higher" lanes to ensure that sidewalk remains the rightmost lane
    //         disallowVehicleClass(-1, SVC_PEDESTRIAN);
    //     }
    //     // add new lane
    //     myLanes.insert(myLanes.begin() + newIndex, Lane(this, myLanes[0].getParameter(SUMO_PARAM_ORIGID)));
    //     myLanes[newIndex].permissions = vclass;
    //     myLanes[newIndex].width = fabs(width);
    //     // shift outgoing connections to the left
    //     for (std::vector<Connection>::iterator it = myConnections.begin(); it != myConnections.end(); ++it) {
    //         Connection& c = *it;
    //         if (c.fromLane >= newIndex) {
    //             c.fromLane += 1;
    //         }
    //     }
    //     // shift incoming connections to the left
    //     const EdgeVector& incoming = myFrom->getIncomingEdges();
    //     for (EdgeVector::const_iterator it = incoming.begin(); it != incoming.end(); ++it) {
    //         (*it)->shiftToLanesToEdge(this, 1);
    //     }
    //     myFrom->shiftTLConnectionLaneIndex(this, 1);
    //     myTo->shiftTLConnectionLaneIndex(this, 1);
    //     computeLaneShapes();
    // }
    #[allow(unused_variables)]
    pub fn add_restricted_lane(&mut self, width: f64, vclass: SUMOVehicleClass) {
        todo!("port NBEdge::addRestrictedLane")
    }

    /// C++: `void NBEdge::setPermissions(SVCPermissions, int)`
    /// set allowed/disallowed classes for the given lane or for all lanes if -1 is given
    // void
    // NBEdge::setPermissions(SVCPermissions permissions, int lane) {
    //     if (lane < 0) {
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setPermissions(permissions, i);
    //         }
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         myLanes[lane].permissions = permissions;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_permissions(&mut self, permissions: SVCPermissions, lane: i32) {
        todo!("port NBEdge::setPermissions")
    }

    /// C++: `void NBEdge::setPreferredVehicleClass(SVCPermissions, int)`
    /// set preferred Vehicle Class
    // void
    // NBEdge::setPreferredVehicleClass(SVCPermissions permissions, int lane) {
    //     if (lane < 0) {
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setPreferredVehicleClass(permissions, i);
    //         }
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         myLanes[lane].preferred = permissions;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_preferred_vehicle_class(&mut self, permissions: SVCPermissions, lane: i32) {
        todo!("port NBEdge::setPreferredVehicleClass")
    }

    /// C++: `void NBEdge::setPermittedChanging(int, SVCPermissions, SVCPermissions)`
    /// set allowed classes for changing to the left and right from the given lane
    // void
    // NBEdge::setPermittedChanging(int lane, SVCPermissions changeLeft, SVCPermissions changeRight) {
    //     assert(lane >= 0);
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].changeLeft = changeLeft;
    //     myLanes[lane].changeRight = changeRight;
    // }
    #[allow(unused_variables)]
    pub fn set_permitted_changing(&mut self, lane: i32, changeLeft: SVCPermissions, changeRight: SVCPermissions) {
        todo!("port NBEdge::setPermittedChanging")
    }

    /// C++: `void NBEdge::allowVehicleClass(int, SUMOVehicleClass)`
    /// set allowed class for the given lane or for all lanes if -1 is given
    // void
    // NBEdge::allowVehicleClass(int lane, SUMOVehicleClass vclass) {
    //     if (lane < 0) { // all lanes are meant...
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             allowVehicleClass(i, vclass);
    //         }
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         myLanes[lane].permissions |= vclass;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn allow_vehicle_class(&mut self, lane: i32, vclass: SUMOVehicleClass) {
        todo!("port NBEdge::allowVehicleClass")
    }

    /// C++: `void NBEdge::disallowVehicleClass(int, SUMOVehicleClass)`
    /// set disallowed class for the given lane or for all lanes if -1 is given
    // void
    // NBEdge::disallowVehicleClass(int lane, SUMOVehicleClass vclass) {
    //     if (lane < 0) { // all lanes are meant...
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             disallowVehicleClass((int) i, vclass);
    //         }
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         myLanes[lane].permissions &= ~vclass;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn disallow_vehicle_class(&mut self, lane: i32, vclass: SUMOVehicleClass) {
        todo!("port NBEdge::disallowVehicleClass")
    }

    /// C++: `void NBEdge::preferVehicleClass(int, SVCPermissions)`
    /// prefer certain vehicle classes for the given lane or for all lanes if -1 is given (ensures also permissions)
    // void
    // NBEdge::preferVehicleClass(int lane, SVCPermissions vclasses) {
    //     if (lane < 0) { // all lanes are meant...
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             preferVehicleClass(i, vclasses);
    //         }
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         myLanes[lane].permissions |= vclasses;
    //         myLanes[lane].preferred |= vclasses;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prefer_vehicle_class(&mut self, lane: i32, vclasses: SVCPermissions) {
        todo!("port NBEdge::preferVehicleClass")
    }

    /// C++: `void NBEdge::setLaneWidth(int, double)`
    /// set lane specific width (negative lane implies set for all lanes)
    // void
    // NBEdge::setLaneWidth(int lane, double width) {
    //     if (lane < 0) {
    //         // all lanes are meant...
    //         myLaneWidth = width;
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setLaneWidth(i, width);
    //         }
    //         return;
    //     }
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].width = width;
    // }
    #[allow(unused_variables)]
    pub fn set_lane_width(&mut self, lane: i32, width: f64) {
        todo!("port NBEdge::setLaneWidth")
    }

    /// C++: `void NBEdge::setLaneType(int, const std::string &)`
    /// set lane specific type (negative lane implies set for all lanes)
    // void
    // NBEdge::setLaneType(int lane, const std::string& type) {
    //     if (lane < 0) {
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setLaneType(i, type);
    //         }
    //         return;
    //     }
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].type = type;
    // }
    #[allow(unused_variables)]
    pub fn set_lane_type(&mut self, lane: i32, type_: &str) {
        todo!("port NBEdge::setLaneType")
    }

    /// C++: `void NBEdge::setEndOffset(int, double)`
    /// set lane specific end-offset (negative lane implies set for all lanes)
    // void
    // NBEdge::setEndOffset(int lane, double offset) {
    //     if (lane < 0) {
    //         // all lanes are meant...
    //         myEndOffset = offset;
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setEndOffset(i, offset);
    //         }
    //         return;
    //     }
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].endOffset = offset;
    // }
    #[allow(unused_variables)]
    pub fn set_end_offset(&mut self, lane: i32, offset: f64) {
        todo!("port NBEdge::setEndOffset")
    }

    /// C++: `void NBEdge::setSpeed(int, double)`
    /// set lane specific speed (negative lane implies set for all lanes)
    // void
    // NBEdge::setSpeed(int lane, double speed) {
    //     if (lane < 0) {
    //         // all lanes are meant...
    //         mySpeed = speed;
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setSpeed(i, speed);
    //         }
    //         return;
    //     }
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].speed = speed;
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, lane: i32, speed: f64) {
        todo!("port NBEdge::setSpeed")
    }

    /// C++: `void NBEdge::setFriction(int, double)`
    /// set lane specific friction (negative lane implies set for all lanes)
    // void
    // NBEdge::setFriction(int lane, double friction) {
    //     if (lane < 0) {
    //         // all lanes are meant...
    //         myFriction = friction;
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             // ... do it for each lane
    //             setFriction(i, friction);
    //         }
    //         return;
    //     }
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].friction = friction;
    // }
    #[allow(unused_variables)]
    pub fn set_friction(&mut self, lane: i32, friction: f64) {
        todo!("port NBEdge::setFriction")
    }

    /// C++: `bool NBEdge::setEdgeStopOffset(int, const StopOffset &, bool)`
    /// set lane and vehicle class specific stopOffset (negative lane implies set for all lanes)
    // bool
    // NBEdge::setEdgeStopOffset(int lane, const StopOffset& offset, bool overwrite) {
    //     if (lane < 0) {
    //         if (!overwrite && myEdgeStopOffset.isDefined()) {
    //             return false;
    //         }
    //         // all lanes are meant...
    //         if (offset.getOffset() < 0) {
    //             // Edge length unknown at parsing time, thus check here.
    //             WRITE_WARNINGF(TL("Ignoring invalid stopOffset for edge '%' (negative offset)."), getID());
    //             return false;
    //         } else {
    //             myEdgeStopOffset = offset;
    //         }
    //     } else if (lane < (int)myLanes.size()) {
    //         if (!myLanes[lane].laneStopOffset.isDefined() || overwrite) {
    //             if (offset.getOffset() < 0) {
    //                 // Edge length unknown at parsing time, thus check here.
    //                 WRITE_WARNINGF(TL("Ignoring invalid stopOffset for lane '%' (negative offset)."), getLaneID(lane));
    //             } else {
    //                 myLanes[lane].laneStopOffset = offset;
    //             }
    //         }
    //     } else {
    //         WRITE_WARNINGF(TL("Ignoring invalid stopOffset for lane '%' (invalid lane index)."), toString(lane));
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn set_edge_stop_offset(&mut self, lane: i32, offset: &StopOffset, overwrite: bool) -> bool {
        todo!("port NBEdge::setEdgeStopOffset")
    }

    /// C++: `void NBEdge::setAcceleration(int, bool)`
    /// marks one lane as acceleration lane
    // void
    // NBEdge::setAcceleration(int lane, bool accelRamp) {
    //     assert(lane >= 0);
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].accelRamp = accelRamp;
    // }
    #[allow(unused_variables)]
    pub fn set_acceleration(&mut self, lane: i32, accelRamp: bool) {
        todo!("port NBEdge::setAcceleration")
    }

    /// C++: `void NBEdge::markOffRamp(bool)`
    /// marks this edge has being an offRamp or leading to one (used for connection computation)
    // void markOffRamp(bool isOffRamp) {
    //         myIsOffRamp = isOffRamp;
    //     }
    #[allow(unused_variables)]
    pub fn mark_off_ramp(&mut self, isOffRamp: bool) {
        todo!("port NBEdge::markOffRamp")
    }

    /// C++: `bool NBEdge::isOffRamp() const` [const]
    // bool isOffRamp() const {
    //         return myIsOffRamp;
    //     }
    #[allow(unused_variables)]
    pub fn is_off_ramp(&self) -> bool {
        todo!("port NBEdge::isOffRamp")
    }

    /// C++: `void NBEdge::setLaneShape(int, const PositionVector &)`
    /// sets a custom lane shape
    // void
    // NBEdge::setLaneShape(int lane, const PositionVector& shape) {
    //     assert(lane >= 0);
    //     assert(lane < (int)myLanes.size());
    //     myLanes[lane].customShape = shape;
    // }
    #[allow(unused_variables)]
    pub fn set_lane_shape(&mut self, lane: i32, shape: &PositionVector) {
        todo!("port NBEdge::setLaneShape")
    }

    /// C++: `SVCPermissions NBEdge::getPermissions(int) const` [const]
    /// get the union of allowed classes over all lanes or for a specific lane
    // SVCPermissions
    // NBEdge::getPermissions(int lane) const {
    //     if (lane < 0) {
    //         SVCPermissions result = 0;
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             result |= getPermissions(i);
    //         }
    //         return result;
    //     } else {
    //         assert(lane < (int)myLanes.size());
    //         return myLanes[lane].permissions;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_permissions(&self, lane: i32) -> SVCPermissions {
        todo!("port NBEdge::getPermissions")
    }

    /// C++: `void NBEdge::setOrigID(const std::string, const bool, const int)`
    /// set origID for all lanes or for a specific lane
    // void
    // NBEdge::setOrigID(const std::string origID, const bool append, const int laneIdx) {
    //     if (laneIdx == -1) {
    //         for (int i = 0; i < (int)myLanes.size(); i++) {
    //             setOrigID(origID, append, i);
    //         }
    //     } else {
    //         if (origID != "") {
    //             if (append) {
    //                 std::vector<std::string> oldIDs = StringTokenizer(myLanes[laneIdx].getParameter(SUMO_PARAM_ORIGID)).getVector();
    //                 if (std::find(oldIDs.begin(), oldIDs.end(), origID) == oldIDs.end()) {
    //                     oldIDs.push_back(origID);
    //                 }
    //                 myLanes[laneIdx].setParameter(SUMO_PARAM_ORIGID, toString(oldIDs));
    //             } else {
    //                 myLanes[laneIdx].setParameter(SUMO_PARAM_ORIGID, origID);
    //             }
    //         } else {
    //             // do not record empty origID parameter
    //             myLanes[laneIdx].unsetParameter(SUMO_PARAM_ORIGID);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_orig_id(&mut self, origID: String, append: bool, laneIdx: i32) {
        todo!("port NBEdge::setOrigID")
    }

    /// C++: `void NBEdge::setDistance(double)`
    /// set kilometrage at start of edge (negative value implies couting down along the edge)
    // void setDistance(double distance) {
    //         myDistance = distance;
    //     }
    #[allow(unused_variables)]
    pub fn set_distance(&mut self, distance: f64) {
        todo!("port NBEdge::setDistance")
    }

    /// C++: `void NBEdge::setBidi(bool)`
    /// mark this edge as a bidi edge
    // void setBidi(bool isBidi) {
    //         myIsBidi = isBidi;
    //     }
    #[allow(unused_variables)]
    pub fn set_bidi(&mut self, isBidi: bool) {
        todo!("port NBEdge::setBidi")
    }

    /// C++: `bool NBEdge::isBidi() const` [const]
    /// return whether this edge should be a bidi edge
    // bool isBidi() const {
    //         return myIsBidi;
    //     }
    #[allow(unused_variables)]
    pub fn is_bidi(&self) -> bool {
        todo!("port NBEdge::isBidi")
    }

    /// C++: `void NBEdge::setRoutingType(const std::string &)`
    /// set the routingType for this edge
    // void setRoutingType(const std::string& routingType) {
    //         myRoutingType = routingType;
    //     }
    #[allow(unused_variables)]
    pub fn set_routing_type(&mut self, routingType: &str) {
        todo!("port NBEdge::setRoutingType")
    }

    /// C++: `const std::string & NBEdge::getRoutingType() const` [const]
    /// return whether this edge should be a bidi edge
    // const std::string& getRoutingType() const {
    //         return myRoutingType;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_type(&self) -> &str {
        todo!("port NBEdge::getRoutingType")
    }

    /// C++: `Lane & NBEdge::getLaneStruct(int)`
    // Lane& getLaneStruct(int lane) {
    //         assert(lane >= 0);
    //         assert(lane < (int)myLanes.size());
    //         return myLanes[lane];
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_struct(&mut self, lane: i32) -> &mut NBEdge_Lane {
        todo!("port NBEdge::getLaneStruct")
    }

    /// C++: `const Lane & NBEdge::getLaneStruct(int) const` [const]
    // const Lane& getLaneStruct(int lane) const {
    //         assert(lane >= 0);
    //         assert(lane < (int)myLanes.size());
    //         return myLanes[lane];
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_struct_2(&self, lane: i32) -> &NBEdge_Lane {
        todo!("port NBEdge::getLaneStruct")
    }

    /// C++: `void NBEdge::declareConnectionsAsLoaded(EdgeBuildingStep)`
    /// declares connections as fully loaded. This is needed to avoid recomputing connections if an edge has no connections intentionally.
    // void declareConnectionsAsLoaded(EdgeBuildingStep step = EdgeBuildingStep::LANES2LANES_USER) {
    //         myStep = step;
    //     }
    #[allow(unused_variables)]
    pub fn declare_connections_as_loaded(&mut self, step: NBEdge_EdgeBuildingStep) {
        todo!("port NBEdge::declareConnectionsAsLoaded")
    }

    /// C++: `double NBEdge::buildInnerEdges(const NBNode &, int, int &, int &)`
    // double
    // NBEdge::buildInnerEdges(const NBNode& n, int noInternalNoSplits, int& linkIndex, int& splitIndex) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const int numPoints = oc.getInt("junctions.internal-link-detail");
    //     const bool joinTurns = oc.getBool("junctions.join-turns");
    //     const double limitTurnSpeed = oc.getFloat("junctions.limit-turn-speed");
    //     const double limitTurnSpeedMinAngle = DEG2RAD(oc.getFloat("junctions.limit-turn-speed.min-angle"));
    //     const double limitTurnSpeedMinAngleRail = DEG2RAD(oc.getFloat("junctions.limit-turn-speed.min-angle.railway"));
    //     const double limitTurnSpeedWarnStraight = oc.getFloat("junctions.limit-turn-speed.warn.straight");
    //     const double limitTurnSpeedWarnTurn = oc.getFloat("junctions.limit-turn-speed.warn.turn");
    //     const bool higherSpeed = oc.getBool("junctions.higher-speed");
    //     const double interalJunctionVehicleWidth = oc.getFloat("internal-junctions.vehicle-width");
    //     const double defaultContPos = oc.getFloat("default.connection.cont-pos");
    //     const bool fromRail = isRailway(getPermissions());
    //     std::string innerID = ":" + n.getID();
    //     NBEdge* toEdge = nullptr;
    //     int edgeIndex = linkIndex;
    //     int internalLaneIndex = 0;
    //     int numLanes = 0; // number of lanes that share the same edge
    //     double lengthSum = 0; // total shape length of all lanes that share the same edge
    //     int avoidedIntersectingLeftOriginLane = std::numeric_limits<int>::max();
    //     bool averageLength = true;
    //     double maxCross = 0.;
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         Connection& con = *i;
    //         con.haveVia = false; // reset first since this may be called multiple times
    //         if (con.toEdge == nullptr) {
    //             continue;
    //         }
    //         LinkDirection dir = n.getDirection(this, con.toEdge);
    //         const bool isRightTurn = (dir == LinkDirection::RIGHT || dir == LinkDirection::PARTRIGHT);
    //         const bool isTurn = (isRightTurn || dir == LinkDirection::LEFT || dir == LinkDirection::PARTLEFT);
    //         // put turning internal lanes on separate edges
    //         if (con.toEdge != toEdge) {
    //             // skip indices to keep some correspondence between edge ids and link indices:
    //             // internalEdgeIndex + internalLaneIndex = linkIndex
    //             edgeIndex = linkIndex;
    //             toEdge = con.toEdge;
    //             internalLaneIndex = 0;
    //             maxCross = MAX2(maxCross, assignInternalLaneLength(i, numLanes, lengthSum, averageLength));
    //             numLanes = 0;
    //             lengthSum = 0;
    //         }
    //         averageLength = !isTurn || joinTurns; // legacy behavior
    //         SVCPermissions conPermissions = getPermissions(con.fromLane) & con.toEdge->getPermissions(con.toLane);
    //         const int conShapeFlag = (conPermissions & ~SVC_PEDESTRIAN) != 0 ? 0 : NBNode::SCURVE_IGNORE;
    //         PositionVector shape = n.computeInternalLaneShape(this, con, numPoints, myTo, conShapeFlag);
    //         std::vector<int> foeInternalLinks;
    //
    //         if (dir != LinkDirection::STRAIGHT && shape.length() < POSITION_EPS && !(isBidiRail() && getTurnDestination(true) == con.toEdge)) {
    //             WRITE_WARNINGF(TL("Connection '%_%->%_%' is only %m short."), getID(), con.fromLane, con.toEdge->getID(), con.toLane, shape.length());
    //         }
    //
    //         // crossingPosition, list of foe link indices
    //         std::pair<double, std::vector<int> > crossingPositions(-1, std::vector<int>());
    //         std::set<std::string> tmpFoeIncomingLanes;
    //         if (dir != LinkDirection::STRAIGHT || con.contPos != UNSPECIFIED_CONTPOS) {
    //             int index = 0;
    //             std::vector<PositionVector> otherShapes;
    //             const double width1 = MIN2(interalJunctionVehicleWidth / 2, getLaneWidth(con.fromLane) / 2);
    //             const double width1OppositeLeft = 0; // using width1 changes a lot of curves even though they are rarely responsible for collisions
    //             for (const NBEdge* i2 : n.getIncomingEdges()) {
    //                 for (const Connection& k2 : i2->getConnections()) {
    //                     if (k2.toEdge == nullptr) {
    //                         continue;
    //                     }
    //                     // vehicles are typically less wide than the lane
    //                     // they drive on but but bicycle lanes should be kept clear for their whole width
    //                     double width2 = k2.toEdge->getLaneWidth(k2.toLane);
    //                     if (k2.toEdge->getPermissions(k2.toLane) != SVC_BICYCLE) {
    //                         width2 *= 0.5;
    //                     }
    //                     const bool foes = n.foes(this, con.toEdge, i2, k2.toEdge);
    //                     LinkDirection dir2 = n.getDirection(i2, k2.toEdge);
    //                     bool needsCont = !isRailway(conPermissions) && (n.needsCont(this, i2, con, k2) || (con.contPos != UNSPECIFIED_CONTPOS && !con.indirectLeft));
    //                     const bool avoidIntersectCandidate = !foes && bothLeftTurns(dir, i2, dir2);
    //                     bool oppositeLeftIntersect = avoidIntersectCandidate && haveIntersection(n, shape, i2, k2, numPoints, width1OppositeLeft, width2);
    //                     int shapeFlag = 0;
    //                     SVCPermissions warn = SVCAll & ~(SVC_PEDESTRIAN | SVC_BICYCLE | SVC_DELIVERY | SVC_RAIL_CLASSES);
    //                     // do not warn if only bicycles, pedestrians or delivery vehicles are involved as this is a typical occurrence
    //                     if (con.customShape.size() == 0
    //                             && k2.customShape.size() == 0
    //                             && (oppositeLeftIntersect || (avoidedIntersectingLeftOriginLane < con.fromLane  && avoidIntersectCandidate))
    //                             && ((i2->getPermissions(k2.fromLane) & warn) != 0
    //                                 && (k2.toEdge->getPermissions(k2.toLane) & warn) != 0)) {
    //                         // recompute with different curve parameters (unless
    //                         // the other connection is "unimportant"
    //                         shapeFlag = NBNode::AVOID_INTERSECTING_LEFT_TURNS;
    //                         PositionVector origShape = shape;
    //                         shape = n.computeInternalLaneShape(this, con, numPoints, myTo, shapeFlag);
    //                         oppositeLeftIntersect = haveIntersection(n, shape, i2, k2, numPoints, width1OppositeLeft, width2, shapeFlag);
    //                         if (oppositeLeftIntersect
    //                                 && (conPermissions & (SVCAll & ~(SVC_BICYCLE | SVC_PEDESTRIAN))) == 0) {
    //                             shape = origShape;
    //                         } else {
    //                             // recompute previously computed crossing positions
    //                             if (avoidedIntersectingLeftOriginLane == std::numeric_limits<int>::max()
    //                                     || avoidedIntersectingLeftOriginLane < con.fromLane) {
    //                                 for (const PositionVector& otherShape : otherShapes) {
    //                                     const bool secondIntersection = con.indirectLeft && this == i2 && con.fromLane == k2.fromLane;
    //                                     const double minDV = firstIntersection(shape, otherShape, width1OppositeLeft, width2,
    //                                                                            "Could not compute intersection of conflicting internal lanes at node '" + myTo->getID() + "'", secondIntersection);
    //                                     if (minDV < shape.length() - POSITION_EPS && minDV > POSITION_EPS) { // !!!?
    //                                         assert(minDV >= 0);
    //                                         if (crossingPositions.first < 0 || crossingPositions.first > minDV) {
    //                                             crossingPositions.first = minDV;
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                             // make sure connections further to the left do not get a wider angle
    //                             avoidedIntersectingLeftOriginLane = con.fromLane;
    //                         }
    //                     }
    //                     const bool bothPrio = getJunctionPriority(&n) > 0 && i2->getJunctionPriority(&n) > 0;
    //                     //std::cout << "n=" << n.getID() << " e1=" << getID() << " prio=" << getJunctionPriority(&n) << " e2=" << i2->getID() << " prio2=" << i2->getJunctionPriority(&n) << " both=" << bothPrio << " bothLeftIntersect=" << bothLeftIntersect(n, shape, dir, i2, k2, numPoints, width2) << " needsCont=" << needsCont << "\n";
    //                     // the following special case might get obsolete once we have solved #9745
    //                     const bool isBicycleLeftTurn = k2.indirectLeft || (dir2 == LinkDirection::LEFT && (i2->getPermissions(k2.fromLane) & k2.toEdge->getPermissions(k2.toLane)) == SVC_BICYCLE);
    //                     // compute the crossing point
    //                     if ((needsCont || (bothPrio && oppositeLeftIntersect && !isRailway(conPermissions))) && (!con.indirectLeft || dir2 == LinkDirection::STRAIGHT) && !isBicycleLeftTurn) {
    //                         crossingPositions.second.push_back(index);
    //                         const PositionVector otherShape = n.computeInternalLaneShape(i2, k2, numPoints, 0, shapeFlag);
    //                         otherShapes.push_back(otherShape);
    //                         const bool secondIntersection = con.indirectLeft && this == i2 && con.fromLane == k2.fromLane;
    //                         const double minDV = firstIntersection(shape, otherShape, width1, width2,
    //                                                                "Could not compute intersection of conflicting internal lanes at node '" + myTo->getID() + "'", secondIntersection);
    //                         if (minDV < shape.length() - POSITION_EPS && minDV > POSITION_EPS) { // !!!?
    //                             assert(minDV >= 0);
    //                             if (crossingPositions.first < 0 || crossingPositions.first > minDV) {
    //                                 crossingPositions.first = minDV;
    //                             }
    //                         }
    //                     }
    //                     const bool rightTurnConflict = NBNode::rightTurnConflict(
    //                                                        this, con.toEdge, con.fromLane, i2, k2.toEdge, k2.fromLane);
    //                     const bool indirectTurnConflit = con.indirectLeft && this == i2 && (dir2 == LinkDirection::STRAIGHT ||
    //                                                      (con.fromLane < k2.fromLane && (dir2 == LinkDirection::RIGHT || dir2 == LinkDirection::PARTRIGHT)));
    //                     const bool mergeConflict = myTo->mergeConflict(this, con, i2, k2, true);
    //                     const bool mergeResponse = myTo->mergeConflict(this, con, i2, k2, false);
    //                     const bool bidiConflict = myTo->bidiConflict(this, con, i2, k2, true);
    //                     // compute foe internal lanes
    //                     if (foes || rightTurnConflict || oppositeLeftIntersect || mergeConflict || indirectTurnConflit || bidiConflict) {
    //                         foeInternalLinks.push_back(index);
    //                     }
    //                     // only warn once per pair of intersecting turns
    //                     if (oppositeLeftIntersect && getID() > i2->getID()
    //                             && (getPermissions(con.fromLane) & warn) != 0
    //                             && (con.toEdge->getPermissions(con.toLane) & warn) != 0
    //                             && (i2->getPermissions(k2.fromLane) & warn) != 0
    //                             && (k2.toEdge->getPermissions(k2.toLane) & warn) != 0
    //                             // do not warn for unregulated nodes
    //                             && n.getType() != SumoXMLNodeType::NOJUNCTION
    //                        ) {
    //                         WRITE_WARNINGF(TL("Intersecting left turns at junction '%' from lane '%' and lane '%' (increase junction radius to avoid this)."),
    //                                        n.getID(), getLaneID(con.fromLane), i2->getLaneID(k2.fromLane));
    //                     }
    //                     // compute foe incoming lanes
    //                     const bool signalised = hasSignalisedConnectionTo(con.toEdge);
    //                     if ((n.forbids(i2, k2.toEdge, this, con.toEdge, signalised) || rightTurnConflict || indirectTurnConflit || mergeResponse)
    //                             && (needsCont || dir == LinkDirection::TURN || (!signalised && this != i2 && !con.indirectLeft))) {
    //                         tmpFoeIncomingLanes.insert(i2->getID() + "_" + toString(k2.fromLane));
    //                     }
    //                     if (bothPrio && oppositeLeftIntersect && getID() < i2->getID()) {
    //                         //std::cout << " c1=" << con.getDescription(this) << " c2=" << k2.getDescription(i2) << " bothPrio=" << bothPrio << " oppositeLeftIntersect=" << oppositeLeftIntersect << "\n";
    //                         // break symmetry using edge id
    //                         // only store link index and resolve actual lane id later (might be multi-lane internal edge)
    //                         tmpFoeIncomingLanes.insert(":" + toString(index));
    //                     }
    //                     index++;
    //                 }
    //             }
    //             if (dir == LinkDirection::TURN && crossingPositions.first < 0 && crossingPositions.second.size() != 0 && shape.length() > 2. * POSITION_EPS) {
    //                 // let turnarounds wait in the middle if no other crossing point was found and it has a sensible length
    //                 // (if endOffset is used, the crossing point is in the middle of the part within the junction shape)
    //                 crossingPositions.first = (double)(shape.length() + getEndOffset(con.fromLane)) / 2.;
    //             }
    //             // foe pedestrian crossings
    //             std::vector<NBNode::Crossing*> crossings = n.getCrossings();
    //             for (auto c : crossings) {
    //                 const NBNode::Crossing& crossing = *c;
    //                 for (EdgeVector::const_iterator it_e = crossing.edges.begin(); it_e != crossing.edges.end(); ++it_e) {
    //                     const NBEdge* edge = *it_e;
    //                     // compute foe internal lanes
    //                     if ((this == edge || con.toEdge == edge) && !isRailway(conPermissions)) {
    //                         foeInternalLinks.push_back(index);
    //                         if (con.toEdge == edge &&
    //                                 ((isRightTurn && getJunctionPriority(&n) > 0) || (isTurn && con.tlID != ""))) {
    //                             // build internal junctions (not for left turns at uncontrolled intersections)
    //                             PositionVector crossingShape = crossing.shape;
    //                             crossingShape.extrapolate(5.0); // sometimes shapes miss each other by a small margin
    //                             const double minDV = firstIntersection(shape, crossingShape, 0, crossing.width / 2);
    //                             if (minDV < shape.length() - POSITION_EPS && minDV > POSITION_EPS) {
    //                                 assert(minDV >= 0);
    //                                 if (crossingPositions.first < 0 || crossingPositions.first > minDV) {
    //                                     crossingPositions.first = minDV;
    //                                 }
    //                             }
    //                         } else if (this == edge && crossing.priority && !myTo->isTLControlled()) {
    //                             crossingPositions.first = 0;
    //                         }
    //                     }
    //                 }
    //                 index++;
    //             }
    //
    //         }
    //         if (con.contPos == UNSPECIFIED_CONTPOS) {
    //             con.contPos = defaultContPos;
    //         }
    //         if (con.contPos != UNSPECIFIED_CONTPOS) {
    //             // apply custom internal junction position
    //             if (con.contPos <= 0 || con.contPos >= shape.length()) {
    //                 // disable internal junction
    //                 crossingPositions.first = -1;
    //             } else {
    //                 // set custom position
    //                 crossingPositions.first = con.contPos;
    //             }
    //         }
    //
    //         // @todo compute the maximum speed allowed based on angular velocity
    //         //  see !!! for an explanation (with a_lat_mean ~0.3)
    //         /*
    //         double vmax = (double) 0.3 * (double) 9.80778 *
    //                         getLaneShape(con.fromLane).back().distanceTo(
    //                             con.toEdge->getLaneShape(con.toLane).front())
    //                         / (double) 2.0 / (double) M_PI;
    //         vmax = MIN2(vmax, ((getSpeed() + con.toEdge->getSpeed()) / (double) 2.0));
    //         */
    //         if (con.speed == UNSPECIFIED_SPEED) {
    //             if (higherSpeed) {
    //                 con.vmax = MAX2(myLanes[con.fromLane].speed, con.toEdge->getLanes()[con.toLane].speed);
    //             } else {
    //                 con.vmax = (myLanes[con.fromLane].speed + con.toEdge->getLanes()[con.toLane].speed) / (double) 2.0;
    //             }
    //             if (limitTurnSpeed > 0) {
    //                 // see [Odhams and Cole, Models of Driver Speed Choice in Curves, 2004]
    //                 const double angleRaw = fabs(GeomHelper::angleDiff(
    //                                                  getLaneShape(con.fromLane).angleAt2D(-2),
    //                                                  con.toEdge->getLaneShape(con.toLane).angleAt2D(0)));
    //                 const double angle = MAX2(0.0, angleRaw - (fromRail ? limitTurnSpeedMinAngleRail : limitTurnSpeedMinAngle));
    //                 const double length = shape.length2D();
    //                 // do not trust the radius of tiny junctions
    //                 // formula adapted from [Odhams, Andre and Cole, David, Models of Driver Speed Choice in Curves, 2004]
    //                 if (angle > 0 && length > 1) {
    //                     // permit higher turning speed on wide lanes
    //                     const double radius = length / angle + getLaneWidth(con.fromLane) / 4;
    //                     const double limit = sqrt(limitTurnSpeed * radius);
    //                     const double reduction = con.vmax - limit;
    //                     // always treat connctions at roundabout as turns when warning
    //                     const bool atRoundabout = getJunctionPriority(myTo) == JunctionPriority::ROUNDABOUT || con.toEdge->getJunctionPriority(myFrom) == JunctionPriority::ROUNDABOUT;
    //                     const LinkDirection dir2 = atRoundabout ? LinkDirection::LEFT : dir;
    //                     if ((dir2 == LinkDirection::STRAIGHT && reduction > limitTurnSpeedWarnStraight)
    //                             || (dir2 != LinkDirection::TURN && reduction > limitTurnSpeedWarnTurn)) {
    //                         std::string dirType = std::string(dir == LinkDirection::STRAIGHT ? "straight" : "turning");
    //                         if (atRoundabout) {
    //                             dirType = "roundabout";
    //                         }
    //                         WRITE_WARNINGF(TL("Speed of % connection '%' reduced by % due to turning radius of % (length=%, angle=%)."),
    //                                        dirType, con.getDescription(this), reduction, radius, length, RAD2DEG(angleRaw));
    //                     }
    //                     con.vmax = MIN2(con.vmax, limit);
    //                     // value is saved in <net> attribute. Must be set again when importing from .con.xml
    //                     // con.speed = con.vmax;
    //                 }
    //                 assert(con.vmax > 0);
    //                 //if (getID() == "-1017000.0.00") {
    //                 //    std::cout << con.getDescription(this) << " angleRaw=" << angleRaw << " angle=" << RAD2DEG(angle) << " length=" << length << " radius=" << length / angle
    //                 //        << " vmaxTurn=" << sqrt(limitTurnSpeed * length / angle) << " vmax=" << con.vmax << "\n";
    //                 //}
    //             } else if (fromRail && dir == LinkDirection::TURN) {
    //                 con.vmax = 0.01;
    //             }
    //         } else {
    //             con.vmax = con.speed;
    //         }
    //         if (con.friction == UNSPECIFIED_FRICTION) {
    //             con.friction = (myLanes[con.fromLane].friction + con.toEdge->getLanes()[con.toLane].friction) / 2.;
    //         }
    //         //
    //         assert(shape.size() >= 2);
    //         // get internal splits if any
    //         con.id = innerID + "_" + toString(edgeIndex);
    //         const double shapeLength = shape.length();
    //         double firstLength = shapeLength;
    //         con.internalViaLaneIndex = 0; // reset to default for netedit
    //         if (crossingPositions.first > 0 && crossingPositions.first < shapeLength) {
    //             std::pair<PositionVector, PositionVector> split = shape.splitAt(crossingPositions.first);
    //             con.shape = split.first;
    //             con.foeIncomingLanes = std::vector<std::string>(tmpFoeIncomingLanes.begin(), tmpFoeIncomingLanes.end());
    //             con.foeInternalLinks = foeInternalLinks; // resolve link indices to lane ids later
    //             if (i != myConnections.begin() && (i - 1)->toEdge == con.toEdge && (i - 1)->haveVia)  {
    //                 --splitIndex;
    //                 con.internalViaLaneIndex = (i - 1)->internalViaLaneIndex + 1;
    //             }
    //             con.viaID = innerID + "_" + toString(splitIndex + noInternalNoSplits);
    //             ++splitIndex;
    //             con.viaShape = split.second;
    //             con.haveVia = true;
    //             firstLength = con.shape.length();
    //         } else {
    //             con.shape = shape;
    //         }
    //         con.internalLaneIndex = internalLaneIndex;
    //         ++internalLaneIndex;
    //         ++linkIndex;
    //         ++numLanes;
    //         if (con.customLength != UNSPECIFIED_LOADED_LENGTH) {
    //             // split length proportionally
    //             lengthSum += (shapeLength != 0 ? firstLength / shapeLength : 1) * con.customLength;
    //         } else {
    //             lengthSum += firstLength;
    //         }
    //     }
    //     return MAX2(maxCross, assignInternalLaneLength(myConnections.end(), numLanes, lengthSum, averageLength));
    // }
    #[allow(unused_variables)]
    pub fn build_inner_edges(&mut self, n: &NBNode, noInternalNoSplits: i32, linkIndex: &mut i32, splitIndex: &mut i32) -> f64 {
        todo!("port NBEdge::buildInnerEdges")
    }

    /// C++: `const std::vector<NBSign> & NBEdge::getSigns() const` [const]
    /// get Signs
    // inline const std::vector<NBSign>& getSigns() const {
    //         return mySigns;
    //     }
    #[allow(unused_variables)]
    pub fn get_signs(&self) -> &Vec<NBSign> {
        todo!("port NBEdge::getSigns")
    }

    /// C++: `void NBEdge::addSign(NBSign)`
    /// add Sign
    // inline void addSign(NBSign sign) {
    //         mySigns.push_back(sign);
    //     }
    #[allow(unused_variables)]
    pub fn add_sign(&mut self, sign: NBSign) {
        todo!("port NBEdge::addSign")
    }

    /// C++: `PositionVector NBEdge::cutAtIntersection(const PositionVector &) const` [const]
    /// cut shape at the intersection shapes
    // PositionVector
    // NBEdge::cutAtIntersection(const PositionVector& old) const {
    //     PositionVector shape = old;
    //     shape = startShapeAt(shape, myFrom, myFromBorder);
    // #ifdef DEBUG_CUT_LANES
    //     if (DEBUGCOND) {
    //         std::cout << getID() << " cutFrom=" << shape << "\n";
    //     }
    // #endif
    //     if (shape.size() < 2) {
    //         // only keep the last snippet
    //         const double oldLength = old.length();
    //         shape = old.getSubpart(oldLength - 2 * POSITION_EPS, oldLength);
    // #ifdef DEBUG_CUT_LANES
    //         if (DEBUGCOND) {
    //             std::cout << getID() << " cutFromFallback=" << shape << "\n";
    //         }
    // #endif
    //     }
    //     shape = startShapeAt(shape.reverse(), myTo, myToBorder).reverse();
    // #ifdef DEBUG_CUT_LANES
    //     if (DEBUGCOND) {
    //         std::cout << getID() << " cutTo=" << shape << "\n";
    //     }
    // #endif
    //     // sanity checks
    //     if (shape.length() < POSITION_EPS) {
    //         if (old.length() < 2 * POSITION_EPS) {
    //             shape = old;
    //         } else {
    //             const double midpoint = old.length() / 2;
    //             // EPS*2 because otherwhise shape has only a single point
    //             shape = old.getSubpart(midpoint - POSITION_EPS, midpoint + POSITION_EPS);
    //             assert(shape.size() >= 2);
    //             assert(shape.length() > 0);
    // #ifdef DEBUG_CUT_LANES
    //             if (DEBUGCOND) {
    //                 std::cout << getID() << " fallBackShort=" << shape << "\n";
    //             }
    // #endif
    //         }
    //     } else {
    //         // @note If the node shapes are overlapping we may get a shape which goes in the wrong direction
    //         // in this case the result shape should shortened
    //         if (DEG2RAD(135) < fabs(GeomHelper::angleDiff(shape.beginEndAngle(), old.beginEndAngle()))) {
    //             // eliminate intermediate points
    //             PositionVector tmp;
    //             tmp.push_back(shape[0]);
    //             tmp.push_back(shape[-1]);
    //             shape = tmp;
    //             if (tmp.length() < POSITION_EPS) {
    //                 // fall back to original shape
    //                 if (old.length() < 2 * POSITION_EPS) {
    //                     shape = old;
    //                 } else {
    //                     const double midpoint = old.length() / 2;
    //                     // EPS*2 because otherwhise shape has only a single point
    //                     shape = old.getSubpart(midpoint - POSITION_EPS, midpoint + POSITION_EPS);
    //                     assert(shape.size() >= 2);
    //                     assert(shape.length() > 0);
    //                 }
    // #ifdef DEBUG_CUT_LANES
    //                 if (DEBUGCOND) {
    //                     std::cout << getID() << " fallBackReversed=" << shape << "\n";
    //                 }
    // #endif
    //             } else {
    //                 const double midpoint = shape.length() / 2;
    //                 // cut to size and reverse
    //                 shape = shape.getSubpart(midpoint - POSITION_EPS, midpoint + POSITION_EPS);
    //                 if (shape.length() < POSITION_EPS) {
    //                     assert(false);
    //                     // the shape has a sharp turn near the midpoint
    //                 }
    //                 shape = shape.reverse();
    // #ifdef DEBUG_CUT_LANES
    //                 if (DEBUGCOND) {
    //                     std::cout << getID() << " fallBackReversed2=" << shape << " mid=" << midpoint << "\n";
    //                 }
    // #endif
    //             }
    //             // make short edge flat (length <= 2 * POSITION_EPS)
    //             const double z = (shape[0].z() + shape[1].z()) / 2;
    //             shape[0].setz(z);
    //             shape[1].setz(z);
    //         }
    //     }
    //     return shape;
    // }
    #[allow(unused_variables)]
    pub fn cut_at_intersection(&self, old: &PositionVector) -> PositionVector {
        todo!("port NBEdge::cutAtIntersection")
    }

    /// C++: `void NBEdge::setNodeBorder(const NBNode *, const Position &, const Position &, bool)`
    /// Set Node border
    // void
    // NBEdge::setNodeBorder(const NBNode* node, const Position& p, const Position& p2, bool rectangularCut) {
    //     PositionVector border;
    //     if (rectangularCut) {
    //         const double extend = 100;
    //         border = myGeom.getOrthogonal(p, extend, node == myTo);
    //     } else {
    //         border.push_back(p);
    //         border.push_back(p2);
    //     }
    //     if (border.size() == 2) {
    //         border.extrapolate2D(getTotalWidth());
    //         if (node == myFrom) {
    //             myFromBorder = border;
    //         } else {
    //             assert(node == myTo);
    //             myToBorder = border;
    //         }
    //     }
    // #ifdef DEBUG_NODE_BORDER
    //     gDebugFlag1 = DEBUGCOND;
    //     if (DEBUGCOND) std::cout << "setNodeBorder edge=" << getID() << " node=" << node->getID()
    //                                  << " rect=" << rectangularCut
    //                                  << " p=" << p << " p2=" << p2
    //                                  << " border=" << border
    //                                  << " myGeom=" << myGeom
    //                                  << "\n";
    //
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn set_node_border(&mut self, node: Ptr<NBNode>, p: &Position, p2: &Position, rectangularCut: bool) {
        todo!("port NBEdge::setNodeBorder")
    }

    /// C++: `const PositionVector & NBEdge::getNodeBorder(const NBNode *) const` [const]
    // const PositionVector&
    // NBEdge::getNodeBorder(const NBNode* node) const {
    //     if (node == myFrom) {
    //         return myFromBorder;
    //     } else {
    //         assert(node == myTo);
    //         return myToBorder;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_node_border(&self, node: Ptr<NBNode>) -> &PositionVector {
        todo!("port NBEdge::getNodeBorder")
    }

    /// C++: `void NBEdge::resetNodeBorder(const NBNode *)`
    // void
    // NBEdge::resetNodeBorder(const NBNode* node) {
    //     if (node == myFrom) {
    //         myFromBorder.clear();
    //     } else {
    //         assert(node == myTo);
    //         myToBorder.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_node_border(&mut self, node: Ptr<NBNode>) {
        todo!("port NBEdge::resetNodeBorder")
    }

    /// C++: `bool NBEdge::isBidiRail(bool) const` [const]
    /// whether this edge is part of a bidirectional railway
    // bool
    // NBEdge::isBidiRail(bool ignoreSpread) const {
    //     return (isRailway(getPermissions())
    //             && (ignoreSpread || myLaneSpreadFunction == LaneSpreadFunction::CENTER)
    //             && myPossibleTurnDestination != nullptr
    //             && myPossibleTurnDestination->myPossibleTurnDestination == this
    //             && (ignoreSpread || myPossibleTurnDestination->getLaneSpreadFunction() == LaneSpreadFunction::CENTER)
    //             && isRailway(myPossibleTurnDestination->getPermissions())
    //             && myPossibleTurnDestination->getGeometry().reverse() == getGeometry());
    // }
    #[allow(unused_variables)]
    pub fn is_bidi_rail(&self, ignoreSpread: bool) -> bool {
        todo!("port NBEdge::isBidiRail")
    }

    /// C++: `bool NBEdge::isBidiEdge(bool) const` [const]
    /// whether this edge is part of a bidirectional edge pair
    // bool
    // NBEdge::isBidiEdge(bool checkPotential) const {
    //     return myPossibleTurnDestination != nullptr
    //            && myPossibleTurnDestination->myPossibleTurnDestination == this
    //            && (myIsBidi || myPossibleTurnDestination->myIsBidi || checkPotential)
    //            && myPossibleTurnDestination->getToNode() == getFromNode()
    //            && myPossibleTurnDestination->getLaneSpreadFunction() == myLaneSpreadFunction
    //            // geometry check a) full overlap geometry
    //            && ((myLaneSpreadFunction == LaneSpreadFunction::CENTER
    //                 && (myPossibleTurnDestination->getGeometry().reverse() == getGeometry()
    //                     || (checkPotential && getGeometry().size() == 2 && myPossibleTurnDestination->getGeometry().size() == 2)))
    //                // b) TWLT (Two-Way-Left-Turn-lane)
    //                || (myLanes.back().shape.reverse().almostSame(myPossibleTurnDestination->myLanes.back().shape, POSITION_EPS))
    //               );
    //
    // }
    #[allow(unused_variables)]
    pub fn is_bidi_edge(&self, checkPotential: bool) -> bool {
        todo!("port NBEdge::isBidiEdge")
    }

    /// C++: `bool NBEdge::isRailDeadEnd() const` [const]
    /// whether this edge is a railway edge that does not continue
    // bool
    // NBEdge::isRailDeadEnd() const {
    //     if (!isRailway(getPermissions())) {
    //         return false;
    //     }
    //     for (NBEdge* out : myTo->getOutgoingEdges()) {
    //         if (isRailway(out->getPermissions()) &&
    //                 out != getTurnDestination(true)) {
    //             return true;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_rail_dead_end(&self) -> bool {
        todo!("port NBEdge::isRailDeadEnd")
    }

    /// C++: `void NBEdge::debugPrintConnections(bool, bool) const` [const]
    /// debugging helper to print all connections
    // void
    // NBEdge::debugPrintConnections(bool outgoing, bool incoming) const {
    //     if (outgoing) {
    //         for (const Connection& c : myConnections) {
    //             std::cout << " " << getID() << "_" << c.fromLane << "->" << c.toEdge->getID() << "_" << c.toLane << "\n";
    //         }
    //     }
    //     if (incoming) {
    //         for (NBEdge* inc : myFrom->getIncomingEdges()) {
    //             for (Connection& c : inc->myConnections) {
    //                 if (c.toEdge == this) {
    //                     std::cout << " " << inc->getID() << "_" << c.fromLane << "->" << c.toEdge->getID() << "_" << c.toLane << "\n";
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn debug_print_connections(&self, outgoing: bool, incoming: bool) {
        todo!("port NBEdge::debugPrintConnections")
    }

    /// C++: `double NBEdge::firstIntersection(const PositionVector &, const PositionVector &, double, double, const std::string &, bool)` [static]
    /// compute the first intersection point between the given lane geometries considering their rspective widths
    // double
    // NBEdge::firstIntersection(const PositionVector& v1, const PositionVector& v2, double width1, double width2, const std::string& error, bool secondIntersection) {
    //     double intersect = std::numeric_limits<double>::max();
    //     if (v2.length() < POSITION_EPS) {
    //         return intersect;
    //     }
    //     try {
    //         PositionVector v1Right = v1;
    //         v1Right.move2side(width1);
    //
    //         PositionVector v1Left = v1;
    //         v1Left.move2side(-width1);
    //
    //         PositionVector v2Right = v2;
    //         v2Right.move2side(width2);
    //
    //         PositionVector v2Left = v2;
    //         v2Left.move2side(-width2);
    //
    //         // intersect all border combinations
    //         bool skip = secondIntersection;
    //         for (double cand : v1Left.intersectsAtLengths2D(v2Right)) {
    //             if (skip) {
    //                 skip = false;
    //                 continue;
    //             }
    //             intersect = MIN2(intersect, cand);
    //         }
    //         skip = secondIntersection;
    //         for (double cand : v1Left.intersectsAtLengths2D(v2Left)) {
    //             if (skip) {
    //                 skip = false;
    //                 continue;
    //             }
    //             intersect = MIN2(intersect, cand);
    //         }
    //         skip = secondIntersection;
    //         for (double cand : v1Right.intersectsAtLengths2D(v2Right)) {
    //             if (skip) {
    //                 skip = false;
    //                 continue;
    //             }
    //             intersect = MIN2(intersect, cand);
    //         }
    //         skip = secondIntersection;
    //         for (double cand : v1Right.intersectsAtLengths2D(v2Left)) {
    //             if (skip) {
    //                 skip = false;
    //                 continue;
    //             }
    //             intersect = MIN2(intersect, cand);
    //         }
    //     } catch (InvalidArgument&) {
    //         if (error != "") {
    //             WRITE_WARNING(error);
    //         }
    //     }
    //     //std::cout << " v1=" << v1 << " v2Right=" << v2Right << " v2Left=" << v2Left << "\n";
    //     //std::cout << "  intersectsRight=" << toString(v1.intersectsAtLengths2D(v2Right)) << "\n";
    //     //std::cout << "  intersectsLeft=" << toString(v1.intersectsAtLengths2D(v2Left)) << "\n";
    //     return intersect;
    // }
    #[allow(unused_variables)]
    pub fn first_intersection(v1: &PositionVector, v2: &PositionVector, width1: f64, width2: f64, error: &str, secondIntersection: bool) -> f64 {
        todo!("port NBEdge::firstIntersection")
    }

    /// C++: `PositionVector NBEdge::startShapeAt(const PositionVector &, const NBNode *, PositionVector)` [static]
    /// returns a modified version of laneShape which starts at the outside of startNode. laneShape may be shorted or extended
    // PositionVector
    // NBEdge::startShapeAt(const PositionVector& laneShape, const NBNode* startNode, PositionVector nodeShape) {
    //     if (nodeShape.size() == 0) {
    //         nodeShape = startNode->getShape();
    //         nodeShape.closePolygon();
    //     }
    //     PositionVector lb = laneShape;
    //     lb.extrapolate2D(100.0);
    //     if (nodeShape.intersects(laneShape)) {
    //         // shape intersects directly
    //         std::vector<double> pbv = laneShape.intersectsAtLengths2D(nodeShape);
    //         assert(pbv.size() > 0);
    //         // ensure that the subpart has at least two points
    //         double pb = MIN2(laneShape.length2D() - POSITION_EPS - NUMERICAL_EPS, VectorHelper<double>::maxValue(pbv));
    //         if (pb < 0) {
    //             return laneShape;
    //         }
    //         PositionVector ns = laneShape.getSubpart2D(pb, laneShape.length2D());
    //         //PositionVector ns = pb < (laneShape.length() - POSITION_EPS) ? laneShape.getSubpart2D(pb, laneShape.length()) : laneShape;
    //         const double delta = ns[0].z() - laneShape[0].z();
    //         //std::cout << "a) startNode=" << startNode->getID() << " z=" << startNode->getPosition().z() << " oldZ=" << laneShape[0].z() << " cutZ=" << ns[0].z() << " delta=" << delta << "\n";
    //         if (fabs(delta) > 2 * POSITION_EPS && (!startNode->geometryLike() || pb < 1)) {
    //             // make "real" intersections and small intersections flat
    //             //std::cout << "a) startNode=" << startNode->getID() << " z=" << startNode->getPosition().z() << " oldZ=" << laneShape[0].z() << " cutZ=" << ns[0].z() << " delta=" << delta << "\n";
    //             ns[0].setz(startNode->getPosition().z());
    //         }
    //         assert(ns.size() >= 2);
    //         return ns;
    //     } else if (nodeShape.intersects(lb)) {
    //         // extension of first segment intersects
    //         std::vector<double> pbv = lb.intersectsAtLengths2D(nodeShape);
    //         assert(pbv.size() > 0);
    //         double pb = VectorHelper<double>::maxValue(pbv);
    //         assert(pb >= 0);
    //         PositionVector result = laneShape.getSubpartByIndex(1, (int)laneShape.size() - 1);
    //         Position np = lb.positionAtOffset2D(pb);
    //         const double delta = np.z() - laneShape[0].z();
    //         //std::cout << "b) startNode=" << startNode->getID() << " z=" << startNode->getPosition().z() << " oldZ=" << laneShape[0].z() << " cutZ=" << np.z() << " delta=" << delta << "\n";
    //         if (fabs(delta) > 2 * POSITION_EPS && !startNode->geometryLike()) {
    //             // avoid z-overshoot when extrapolating
    //             //std::cout << "b) startNode=" << startNode->getID() << " z=" << startNode->getPosition().z() << " oldZ=" << laneShape[0].z() << " cutZ=" << np.z() << " delta=" << delta << "\n";
    //             np.setz(startNode->getPosition().z());
    //         }
    //         result.push_front_noDoublePos(np);
    //         return result;
    //         //if (result.size() >= 2) {
    //         //    return result;
    //         //} else {
    //         //    WRITE_WARNING(error + " (resulting shape is too short)");
    //         //    return laneShape;
    //         //}
    //     } else {
    //         // could not find proper intersection. Probably the edge is very short
    //         // and lies within nodeShape
    //         // @todo enable warning WRITE_WARNING(error + " (laneShape lies within nodeShape)");
    //         return laneShape;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn start_shape_at(laneShape: &PositionVector, startNode: Ptr<NBNode>, nodeShape: PositionVector) -> PositionVector {
        todo!("port NBEdge::startShapeAt")
    }

    /// C++: `double NBEdge::getTravelTimeStatic(const NBEdge *const, const NBVehicle *const, double)` [static]
    // static inline double getTravelTimeStatic(const NBEdge* const edge, const NBVehicle* const /*veh*/, double /*time*/) {
    //         return edge->getLength() / edge->getSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<NBEdge>, arg1: Ptr<NBVehicle>, arg2: f64) -> f64 {
        todo!("port NBEdge::getTravelTimeStatic")
    }

    /// C++: `int NBEdge::getLaneIndexFromLaneID(const std::string)` [static]
    // int
    // NBEdge::getLaneIndexFromLaneID(const std::string laneID) {
    //     return StringUtils::toInt(laneID.substr(laneID.rfind("_") + 1));
    // }
    #[allow(unused_variables)]
    pub fn get_lane_index_from_lane_id(laneID: String) -> i32 {
        todo!("port NBEdge::getLaneIndexFromLaneID")
    }

    /// C++: `void NBEdge::setNumericalID(int)`
    /// sets the index of the edge in the list of all network edges
    // void setNumericalID(int index) {
    //         myIndex = index;
    //     }
    #[allow(unused_variables)]
    pub fn set_numerical_id(&mut self, index: i32) {
        todo!("port NBEdge::setNumericalID")
    }

    /// C++: `int NBEdge::getNumericalID() const` [const, virtual]
    /// Returns the index (numeric id) of the edge
    // int getNumericalID() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port NBEdge::getNumericalID")
    }

    /// C++: `const NBEdge * NBEdge::getBidiEdge() const` [const, virtual]
    // const NBEdge* getBidiEdge() const {
    //         return isBidiRail() || isBidiEdge() ? myPossibleTurnDestination : nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<NBEdge> {
        todo!("port NBEdge::getBidiEdge")
    }

    /// C++: `const EdgeVector & NBEdge::getSuccessors(SUMOVehicleClass) const` [const]
    /// Returns the following edges for the given vClass
    // const EdgeVector&
    // NBEdge::getSuccessors(SUMOVehicleClass vClass) const {
    //     // @todo cache successors instead of recomputing them every time
    //     mySuccessors.clear();
    //     //std::cout << "getSuccessors edge=" << getID() << " svc=" << toString(vClass) << " cons=" << myConnections.size() << "\n";
    //     for (const Connection& con : myConnections) {
    //         if (con.fromLane >= 0 && con.toLane >= 0 && con.toEdge != nullptr &&
    //                 (vClass == SVC_IGNORING || (getPermissions(con.fromLane)
    //                                             & con.toEdge->getPermissions(con.toLane) & vClass) != 0)
    //                 && std::find(mySuccessors.begin(), mySuccessors.end(), con.toEdge) == mySuccessors.end()) {
    //             mySuccessors.push_back(con.toEdge);
    //             //std::cout << "   succ=" << con.toEdge->getID() << "\n";
    //         }
    //     }
    //     return mySuccessors;
    // }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &EdgeVector {
        todo!("port NBEdge::getSuccessors")
    }

    /// C++: `const ConstRouterEdgePairVector & NBEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual]
    /// Returns the following edges for the given vClass
    // const ConstRouterEdgePairVector&
    // NBEdge::getViaSuccessors(SUMOVehicleClass vClass, bool /*ignoreTransientPermissions*/) const {
    //     // @todo cache successors instead of recomputing them every time
    //     myViaSuccessors.clear();
    //     for (const Connection& con : myConnections) {
    //         std::pair<const NBEdge*, const Connection*> pair(con.toEdge, nullptr);
    //         // special case for Persons in Netedit
    //         if (vClass == SVC_PEDESTRIAN) {
    //             myViaSuccessors.push_back(pair);    // Pedestrians have complete freedom of movement in all successors
    //         } else if ((con.fromLane >= 0) && (con.toLane >= 0) &&
    //                    (con.toEdge != nullptr) &&
    //                    ((getPermissions(con.fromLane) & con.toEdge->getPermissions(con.toLane) & vClass) == vClass)) {
    //             // ignore duplicates
    //             if (con.getLength() > 0) {
    //                 pair.second = &con;
    //             }
    //             myViaSuccessors.push_back(pair);
    //         }
    //     }
    //     return myViaSuccessors;
    // }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ConstRouterEdgePairVector {
        todo!("port NBEdge::getViaSuccessors")
    }

    /// C++: `const std::string & NBEdge::getID() const` [const, virtual]
    // const std::string& getID() const {
    //         return Named::getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        todo!("port NBEdge::getID")
    }

    /// C++: `bool NBEdge::joinLanes(SVCPermissions)`
    /// join adjacent lanes with the given permissions
    // bool
    // NBEdge::joinLanes(SVCPermissions perms) {
    //     bool haveJoined = false;
    //     int i = 0;
    //     while (i < getNumLanes() - 1) {
    //         if ((getPermissions(i) == perms) && (getPermissions(i + 1) == perms)) {
    //             const double newWidth = getLaneWidth(i) + getLaneWidth(i + 1);
    //             const std::string newType = myLanes[i].type + "|" + myLanes[i + 1].type;
    //             deleteLane(i, false, true);
    //             setLaneWidth(i, newWidth);
    //             setLaneType(i, newType);
    //             haveJoined = true;
    //         } else {
    //             i++;
    //         }
    //     }
    //     return haveJoined;
    // }
    #[allow(unused_variables)]
    pub fn join_lanes(&mut self, perms: SVCPermissions) -> bool {
        todo!("port NBEdge::joinLanes")
    }

    /// C++: `void NBEdge::resetLaneShapes()`
    /// reset lane shapes to what they would be before cutting with the junction shapes
    // void
    // NBEdge::resetLaneShapes() {
    //     computeLaneShapes();
    // }
    #[allow(unused_variables)]
    pub fn reset_lane_shapes(&mut self) {
        todo!("port NBEdge::resetLaneShapes")
    }

    /// C++: `void NBEdge::updateChangeRestrictions(SVCPermissions)`
    /// modify all existing restrictions on lane changing
    // void
    // NBEdge::updateChangeRestrictions(SVCPermissions ignoring) {
    //     for (Lane& lane : myLanes) {
    //         if (lane.changeLeft != SVCAll) {
    //             lane.changeLeft = ignoring;
    //         }
    //         if (lane.changeRight != SVCAll) {
    //             lane.changeRight = ignoring;
    //         }
    //     }
    //     for (Connection& con : myConnections) {
    //         if (con.changeLeft != SVC_UNSPECIFIED && con.changeLeft != SVCAll) {
    //             con.changeLeft = ignoring;
    //         }
    //         if (con.changeRight != SVC_UNSPECIFIED && con.changeRight != SVCAll) {
    //             con.changeRight = ignoring;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_change_restrictions(&mut self, ignoring: SVCPermissions) {
        todo!("port NBEdge::updateChangeRestrictions")
    }

    /// C++: `NBEdge * NBEdge::getStraightContinuation(SVCPermissions) const` [const]
    /// return the straightest follower edge for the given permissions or nullptr (never returns turn-arounds)
    // NBEdge*
    // NBEdge::getStraightContinuation(SVCPermissions permissions) const {
    //     EdgeVector cands = filterByPermissions(myTo->getOutgoingEdges(), permissions);
    //     if (cands.size() == 0) {
    //         return nullptr;
    //     }
    //     sort(cands.begin(), cands.end(), NBContHelper::edge_similar_direction_sorter(this));
    //     NBEdge* best = cands.front();
    //     if (isTurningDirectionAt(best)) {
    //         return nullptr;
    //     } else {
    //         return best;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_straight_continuation(&self, permissions: SVCPermissions) -> Ptr<NBEdge> {
        todo!("port NBEdge::getStraightContinuation")
    }

    /// C++: `NBEdge * NBEdge::getStraightPredecessor(SVCPermissions) const` [const]
    /// return the straightest predecessor edge for the given permissions or nullptr (never returns turn-arounds)
    // NBEdge*
    // NBEdge::getStraightPredecessor(SVCPermissions permissions) const {
    //     EdgeVector cands = filterByPermissions(myFrom->getIncomingEdges(), permissions);
    //     if (cands.size() == 0) {
    //         return nullptr;
    //     }
    //     sort(cands.begin(), cands.end(), NBContHelper::edge_similar_direction_sorter(this, false));
    //     NBEdge* best = cands.front();
    //     if (best->isTurningDirectionAt(this)) {
    //         return nullptr;
    //     } else {
    //         return best;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_straight_predecessor(&self, permissions: SVCPermissions) -> Ptr<NBEdge> {
        todo!("port NBEdge::getStraightPredecessor")
    }

    /// C++: `NBEdge * NBEdge::guessOpposite(bool)`
    /// set oppositeID and return opposite edge if found
    // NBEdge*
    // NBEdge::guessOpposite(bool reguess) {
    //     NBEdge* opposite = nullptr;
    //     if (getNumLanes() > 0) {
    //         NBEdge::Lane& lastLane = myLanes.back();
    //         const double lastWidth = getLaneWidth(getNumLanes() - 1);
    //         if (lastLane.oppositeID == "" || reguess) {
    //             for (NBEdge* cand : getToNode()->getOutgoingEdges()) {
    //                 if (cand->getToNode() == getFromNode() && !cand->getLanes().empty()) {
    //                     const NBEdge::Lane& candLastLane = cand->getLanes().back();
    //                     if (candLastLane.oppositeID == "" || candLastLane.oppositeID == getLaneID(getNumLanes() - 1)) {
    //                         const double lastWidthCand = cand->getLaneWidth(cand->getNumLanes() - 1);
    //                         // in sharp corners, the difference may be higher
    //                         // factor (sqrt(2) for 90 degree corners
    //                         const double threshold = 1.42 * 0.5 * (lastWidth + lastWidthCand) + 0.5;
    //                         const double distance = VectorHelper<double>::maxValue(lastLane.shape.distances(cand->getLanes().back().shape));
    //                         //std::cout << " distance=" << distance << " threshold=" << threshold << " distances=" << toString(lastLane.shape.distances(cand->getLanes().back().shape)) << "\n";
    //                         if (distance < threshold) {
    //                             opposite = cand;
    //                         }
    //                     }
    //                 }
    //             }
    //             if (opposite != nullptr) {
    //                 lastLane.oppositeID = opposite->getLaneID(opposite->getNumLanes() - 1);
    //             }
    //         }
    //     }
    //     return opposite;
    // }
    #[allow(unused_variables)]
    pub fn guess_opposite(&mut self, reguess: bool) -> Ptr<NBEdge> {
        todo!("port NBEdge::guessOpposite")
    }

    /// C++: `const std::string & NBEdge::getTurnSignTarget() const` [const]
    // const std::string& getTurnSignTarget() const {
    //         return myTurnSignTarget;
    //     }
    #[allow(unused_variables)]
    pub fn get_turn_sign_target(&self) -> &str {
        todo!("port NBEdge::getTurnSignTarget")
    }

    /// C++: `void NBEdge::setTurnSignTarget(const std::string &)`
    // void setTurnSignTarget(const std::string& target) {
    //         myTurnSignTarget = target;
    //     }
    #[allow(unused_variables)]
    pub fn set_turn_sign_target(&mut self, target: &str) {
        todo!("port NBEdge::setTurnSignTarget")
    }

    /// C++: `EdgeVector NBEdge::filterByPermissions(const EdgeVector &, SVCPermissions)` [static]
    /// return only those edges that permit at least one of the give permissions
    // EdgeVector
    // NBEdge::filterByPermissions(const EdgeVector& edges, SVCPermissions permissions) {
    //     EdgeVector result;
    //     for (NBEdge* edge : edges) {
    //         if ((edge->getPermissions() & permissions) != 0) {
    //             result.push_back(edge);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn filter_by_permissions(edges: &EdgeVector, permissions: SVCPermissions) -> EdgeVector {
        todo!("port NBEdge::filterByPermissions")
    }

    /// C++: `PositionVector NBEdge::computeLaneShape(int, double) const` [const, private]
    /// Computes the shape for the given lane
    // PositionVector
    // NBEdge::computeLaneShape(int lane, double offset) const {
    //     PositionVector shape = myGeom;
    //     try {
    //         shape.move2side(offset);
    //     } catch (InvalidArgument& e) {
    //         WRITE_WARNINGF(TL("In lane '%': Could not build shape (%)."), getLaneID(lane), e.what());
    //     }
    //     return shape;
    // }
    #[allow(unused_variables)]
    pub fn compute_lane_shape(&self, lane: i32, offset: f64) -> PositionVector {
        todo!("port NBEdge::computeLaneShape")
    }

    /// C++: `void NBEdge::computeLaneShapes()` [private]
    /// compute lane shapes
    // void
    // NBEdge::computeLaneShapes() {
    //     // vissim needs this
    //     if (myFrom == myTo) {
    //         return;
    //     }
    //     // compute lane offset, first
    //     std::vector<double> offsets(myLanes.size(), 0.);
    //     double offset = 0;
    //     for (int i = (int)myLanes.size() - 2; i >= 0; --i) {
    //         offset += (getLaneWidth(i) + getLaneWidth(i + 1)) / 2.;
    //         offsets[i] = offset;
    //     }
    //     if (myLaneSpreadFunction == LaneSpreadFunction::CENTER) {
    //         double width = 0;
    //         for (int i = 0; i < (int)myLanes.size(); ++i) {
    //             width += getLaneWidth(i);
    //         }
    //         offset = -width / 2. + getLaneWidth((int)myLanes.size() - 1) / 2.;
    //     } else {
    //         double laneWidth = myLanes.back().width != UNSPECIFIED_WIDTH ? myLanes.back().width : SUMO_const_laneWidth;
    //         offset = laneWidth / 2.;
    //     }
    //     if (myLaneSpreadFunction == LaneSpreadFunction::ROADCENTER) {
    //         for (NBEdge* e : myTo->getOutgoingEdges()) {
    //             if (e->getToNode() == myFrom && getInnerGeometry().reverse() == e->getInnerGeometry()) {
    //                 offset += (e->getTotalWidth() - getTotalWidth()) / 2;
    //                 break;
    //             }
    //         }
    //     }
    //
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         offsets[i] += offset;
    //     }
    //
    //     // build the shape of each lane
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         if (myLanes[i].customShape.size() != 0) {
    //             myLanes[i].shape = myLanes[i].customShape;
    //             continue;
    //         }
    //         try {
    //             myLanes[i].shape = computeLaneShape(i, offsets[i]);
    //         } catch (InvalidArgument& e) {
    //             WRITE_WARNINGF(TL("In lane '%': lane shape could not be determined (%)."), getLaneID(i), e.what());
    //             myLanes[i].shape = myGeom;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_lane_shapes(&mut self) {
        todo!("port NBEdge::computeLaneShapes")
    }

    /// C++: `void NBEdge::init(int, bool, const std::string &)` [private]
    /// Initialization routines common to all constructors
    // void
    // NBEdge::init(int noLanes, bool tryIgnoreNodePositions, const std::string& origID) {
    //     if (noLanes == 0) {
    //         throw ProcessError(TLF("Edge '%' needs at least one lane.", myID));
    //     }
    //     if (myFrom == nullptr || myTo == nullptr) {
    //         throw ProcessError(TLF("At least one of edge's '%' nodes is not known.", myID));
    //     }
    //     if (!SUMOXMLDefinitions::isValidNetID(myID)) {
    //         throw ProcessError(TLF("Invalid edge id '%'.", myID));
    //     }
    //     // revisit geometry
    //     //  should have at least two points at the end...
    //     //  and in dome cases, the node positions must be added
    //     // attempt symmetrical removal for forward and backward direction
    //     // (very important for bidiRail)
    //     if (myFrom->getID() < myTo->getID()) {
    //         PositionVector reverse = myGeom.reverse();
    //         reverse.removeDoublePoints(POSITION_EPS, true);
    //         myGeom = reverse.reverse();
    //     } else {
    //         myGeom.removeDoublePoints(POSITION_EPS, true);
    //     }
    //
    //     if (!tryIgnoreNodePositions || myGeom.size() < 2) {
    //         if (myGeom.size() == 0) {
    //             myGeom.push_back(myFrom->getPosition());
    //             myGeom.push_back(myTo->getPosition());
    //         } else {
    //             myGeom.push_back_noDoublePos(myTo->getPosition());
    //             myGeom.push_front_noDoublePos(myFrom->getPosition());
    //         }
    //     }
    //     if (myGeom.size() < 2) {
    //         myGeom.clear();
    //         myGeom.push_back(myFrom->getPosition());
    //         myGeom.push_back(myTo->getPosition());
    //     }
    //     if (myGeom.size() == 2 && myGeom[0] == myGeom[1]) {
    //         WRITE_WARNINGF(TL("Edge's '%' from- and to-node are at the same position."), myID);
    //         int patchIndex = myFrom->getID() < myTo->getID() ? 1 : 0;
    //         myGeom[patchIndex].add(Position(POSITION_EPS, POSITION_EPS));
    //     }
    //     // avoid degeneration of near-0-length geometrie when shifting later
    //     myGeom.ensureMinLength(gPrecision);
    //     //
    //     myFrom->addOutgoingEdge(this);
    //     myTo->addIncomingEdge(this);
    //     // prepare container
    //     assert(myGeom.size() >= 2);
    //     myLength = myGeom.length();
    //     if ((int)myLanes.size() > noLanes) {
    //         // remove connections starting at the removed lanes
    //         for (int lane = noLanes; lane < (int)myLanes.size(); ++lane) {
    //             removeFromConnections(nullptr, lane, -1);
    //         }
    //         // remove connections targeting the removed lanes
    //         const EdgeVector& incoming = myFrom->getIncomingEdges();
    //         for (EdgeVector::const_iterator i = incoming.begin(); i != incoming.end(); i++) {
    //             for (int lane = noLanes; lane < (int)myLanes.size(); ++lane) {
    //                 (*i)->removeFromConnections(this, -1, lane);
    //             }
    //         }
    //     }
    //     myLanes.clear();
    //     for (int i = 0; i < noLanes; i++) {
    //         myLanes.push_back(Lane(this, origID));
    //     }
    //     computeLaneShapes();
    //     computeAngle();
    //
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "init edge=" << getID() << "\n";
    //         for (Connection& c : myConnections) {
    //             std::cout << "  conn " << c.getDescription(this) << "\n";
    //         }
    //         for (Connection& c : myConnectionsToDelete) {
    //             std::cout << "  connToDelete " << c.getDescription(this) << "\n";
    //         }
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, noLanes: i32, tryIgnoreNodePositions: bool, origID: &str) {
        todo!("port NBEdge::init")
    }

    /// C++: `void NBEdge::divideOnEdges(const EdgeVector *)` [private]
    /// divides the lanes on the outgoing edges
    // void
    // NBEdge::divideOnEdges(const EdgeVector* outgoing) {
    //     if (outgoing->size() == 0) {
    //         // we have to do this, because the turnaround may have been added before
    //         myConnections.clear();
    //         return;
    //     }
    //
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << " divideOnEdges " << getID() << " outgoing=" << toString(*outgoing) << "\n";
    //     }
    // #endif
    //
    //     // build connections for miv lanes
    //     std::vector<int> availableLanes;
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         if ((getPermissions(i) & SVC_PASSENGER) != 0) {
    //             availableLanes.push_back(i);
    //         }
    //     }
    //     if (availableLanes.size() > 0) {
    //         divideSelectedLanesOnEdges(outgoing, availableLanes);
    //     }
    //     // build connections for miscellaneous further modes (more than bike,peds,bus and without passenger)
    //     availableLanes.clear();
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         const SVCPermissions perms = getPermissions(i);
    //         if ((perms & ~(SVC_PEDESTRIAN | SVC_BICYCLE | SVC_BUS)) == 0 || (perms & SVC_PASSENGER) != 0 || isForbidden(perms)) {
    //             continue;
    //         }
    //         availableLanes.push_back(i);
    //     }
    //     if (availableLanes.size() > 0) {
    //         divideSelectedLanesOnEdges(outgoing, availableLanes);
    //     }
    //     // build connections for busses from lanes that were excluded in the previous step
    //     availableLanes.clear();
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         const SVCPermissions perms = getPermissions(i);
    //         if ((perms & SVC_BUS) == 0 || (perms & ~(SVC_PEDESTRIAN | SVC_BICYCLE | SVC_BUS)) != 0 || (perms & SVC_PASSENGER) != 0) {
    //             continue;
    //         }
    //         availableLanes.push_back(i);
    //     }
    //     if (availableLanes.size() > 0) {
    //         divideSelectedLanesOnEdges(outgoing, availableLanes);
    //     }
    //     // build connections for bicycles (possibly combined with pedestrians)
    //     availableLanes.clear();
    //     for (int i = 0; i < (int)myLanes.size(); ++i) {
    //         const SVCPermissions perms = getPermissions(i);
    //         if (perms != SVC_BICYCLE && perms != (SVC_BICYCLE | SVC_PEDESTRIAN)) {
    //             continue;
    //         }
    //         availableLanes.push_back(i);
    //     }
    //     if (availableLanes.size() > 0) {
    //         divideSelectedLanesOnEdges(outgoing, availableLanes);
    //     }
    //     // clean up unassigned fromLanes
    //     bool explicitTurnaround = false;
    //     SVCPermissions turnaroundPermissions = SVC_UNSPECIFIED;
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         if ((*i).fromLane == -1) {
    //             if ((*i).toEdge == myTurnDestination && myTurnDestination != nullptr) {
    //                 explicitTurnaround = true;
    //                 turnaroundPermissions = (*i).permissions;
    //             }
    //             if ((*i).permissions != SVC_UNSPECIFIED) {
    //                 for (Connection& c : myConnections) {
    //                     if (c.toLane == -1 && c.toEdge == (*i).toEdge) {
    //                         // carry over loaded edge2edge permissions
    //                         c.permissions = (*i).permissions;
    //                     }
    //                 }
    //             }
    //             i = myConnections.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    //     if (explicitTurnaround) {
    //         myConnections.push_back(Connection((int)myLanes.size() - 1, myTurnDestination, myTurnDestination->getNumLanes() - 1));
    //         myConnections.back().permissions = turnaroundPermissions;
    //     }
    //     sortOutgoingConnectionsByIndex();
    // }
    #[allow(unused_variables)]
    pub fn divide_on_edges(&mut self, outgoing: Ptr<EdgeVector>) {
        todo!("port NBEdge::divideOnEdges")
    }

    /// C++: `void NBEdge::divideSelectedLanesOnEdges(const EdgeVector *, const std::vector<int> &)` [private]
    /// divide selected lanes on edges
    // void
    // NBEdge::divideSelectedLanesOnEdges(const EdgeVector* outgoing, const std::vector<int>& availableLanes) {
    //     const std::vector<int>& priorities = prepareEdgePriorities(outgoing, availableLanes);
    //     if (priorities.empty()) {
    //         return;
    //     }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "divideSelectedLanesOnEdges " << getID() << " out=" << toString(*outgoing) << " prios=" << toString(priorities) << " avail=" << toString(availableLanes) << "\n";
    //     }
    // #endif
    //     // compute the resulting number of lanes that should be used to reach the following edge
    //     const int numOutgoing = (int)outgoing->size();
    //     std::vector<int> resultingLanesFactor;
    //     resultingLanesFactor.reserve(numOutgoing);
    //     int minResulting = std::numeric_limits<int>::max();
    //     for (int i = 0; i < numOutgoing; i++) {
    //         // res / minResulting will be the number of lanes which are meant to reach the current outgoing edge
    //         const int res = priorities[i] * (int)availableLanes.size();
    //         resultingLanesFactor.push_back(res);
    //         if (minResulting > res && res > 0) {
    //             // prevent minResulting from becoming 0
    //             minResulting = res;
    //         }
    //     }
    //     // compute the number of virtual edges
    //     //  a virtual edge is used as a replacement for a real edge from now on
    //     //  it shall allow to divide the existing lanes on this structure without
    //     //  regarding the structure of outgoing edges
    //     int numVirtual = 0;
    //     // compute the transition from virtual to real edges
    //     EdgeVector transition;
    //     transition.reserve(numOutgoing);
    //     for (int i = 0; i < numOutgoing; i++) {
    //         // tmpNum will be the number of connections from this edge to the next edge
    //         assert(i < (int)resultingLanesFactor.size());
    //         const int tmpNum = (resultingLanesFactor[i] + minResulting - 1) / minResulting; // integer division rounding up
    //         numVirtual += tmpNum;
    //         for (int j = 0; j < tmpNum; j++) {
    //             transition.push_back((*outgoing)[i]);
    //         }
    //     }
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "   minResulting=" << minResulting << " numVirtual=" << numVirtual << " availLanes=" << toString(availableLanes) << " resLanes=" << toString(resultingLanesFactor) << " transition=" << toString(transition) << "\n";
    //     }
    // #endif
    //
    //     // assign lanes to edges
    //     //  (conversion from virtual to real edges is done)
    //     ToEdgeConnectionsAdder adder(transition);
    //     Bresenham::compute(&adder, static_cast<int>(availableLanes.size()), numVirtual);
    //     const std::map<NBEdge*, std::vector<int> >& l2eConns = adder.getBuiltConnections();
    //     for (NBEdge* const target : *outgoing) {
    //         assert(l2eConns.find(target) != l2eConns.end());
    //         for (const int j : l2eConns.find(target)->second) {
    //             const int fromIndex = availableLanes[j];
    //             if ((getPermissions(fromIndex) & target->getPermissions()) == 0) {
    //                 // exclude connection if fromLane and toEdge have no common permissions
    //                 continue;
    //             }
    //             if ((getPermissions(fromIndex) & target->getPermissions()) == SVC_PEDESTRIAN) {
    //                 // exclude connection if the only commonly permitted class are pedestrians
    //                 // these connections are later built in NBNode::buildWalkingAreas
    //                 continue;
    //             }
    //             // avoid building more connections than the edge has viable lanes (earlier
    //             // ones have precedence). This is necessary when running divideSelectedLanesOnEdges more than once.
    //             //    @todo To decide which target lanes are still available we need to do a
    //             // preliminary lane-to-lane assignment in regard to permissions (rather than to ordering)
    //             const int numConsToTarget = (int)count_if(myConnections.begin(), myConnections.end(), connections_toedge_finder(target, true));
    //             int targetLanes = target->getNumLanes();
    //             if (target->getPermissions(0) == SVC_PEDESTRIAN) {
    //                 --targetLanes;
    //             }
    //             if (numConsToTarget >= targetLanes) {
    //                 continue;
    //             }
    //             if (myLanes[fromIndex].connectionsDone) {
    //                 // we already have complete information about connections from
    //                 // this lane. do not add anything else
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) {
    //                     std::cout << "     connectionsDone from " << getID() << "_" << fromIndex << ": ";
    //                     for (const Connection& c : getConnectionsFromLane(fromIndex)) {
    //                         std::cout << c.getDescription(this) << ", ";
    //                     }
    //                     std::cout << "\n";
    //                 }
    // #endif
    //                 continue;
    //             }
    //             myConnections.push_back(Connection(fromIndex, target, -1));
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "     request connection from " << getID() << "_" << fromIndex << " to " << target->getID() << "\n";
    //             }
    // #endif
    //         }
    //     }
    //
    //     addStraightConnections(outgoing, availableLanes, priorities);
    // }
    #[allow(unused_variables)]
    pub fn divide_selected_lanes_on_edges(&mut self, outgoing: Ptr<EdgeVector>, availableLanes: &Vec<i32>) {
        todo!("port NBEdge::divideSelectedLanesOnEdges")
    }

    /// C++: `void NBEdge::addStraightConnections(const EdgeVector *, const std::vector<int> &, const std::vector<int> &)` [private]
    /// add some straight connections
    // void
    // NBEdge::addStraightConnections(const EdgeVector* outgoing, const std::vector<int>& availableLanes, const std::vector<int>& priorities) {
    //     // ensure sufficient straight connections for the (highest-priority) straight target
    //     const int numOutgoing = (int) outgoing->size();
    //     NBEdge* target = nullptr;
    //     NBEdge* rightOfTarget = nullptr;
    //     NBEdge* leftOfTarget = nullptr;
    //     int maxPrio = 0;
    //     for (int i = 0; i < numOutgoing; i++) {
    //         if (maxPrio < priorities[i]) {
    //             const LinkDirection dir = myTo->getDirection(this, (*outgoing)[i]);
    //             if (dir == LinkDirection::STRAIGHT) {
    //                 maxPrio = priorities[i];
    //                 target = (*outgoing)[i];
    //                 rightOfTarget = i == 0 ? outgoing->back() : (*outgoing)[i - 1];
    //                 leftOfTarget = i + 1 == numOutgoing ? outgoing->front() : (*outgoing)[i + 1];
    //             }
    //         }
    //     }
    //     if (target == nullptr) {
    //         return;
    //     }
    //     int numConsToTarget = (int)count_if(myConnections.begin(), myConnections.end(), connections_toedge_finder(target, true));
    //     int targetLanes = (int)target->getNumLanes();
    //     if (target->getPermissions(0) == SVC_PEDESTRIAN) {
    //         --targetLanes;
    //     }
    //     const int numDesiredConsToTarget = MIN2(targetLanes, (int)availableLanes.size());
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) {
    //         std::cout << "  checking extra lanes for target=" << target->getID() << " cons=" << numConsToTarget << " desired=" << numDesiredConsToTarget << "\n";
    //     }
    // #endif
    //     std::vector<int>::const_iterator it_avail = availableLanes.begin();
    //     while (numConsToTarget < numDesiredConsToTarget && it_avail != availableLanes.end()) {
    //         const int fromIndex = *it_avail;
    //         if (
    //             // not yet connected
    //             (count_if(myConnections.begin(), myConnections.end(), connections_finder(fromIndex, target, -1)) == 0)
    //             // matching permissions
    //             && ((getPermissions(fromIndex) & target->getPermissions()) != 0)
    //             // more than pedestrians
    //             && ((getPermissions(fromIndex) & target->getPermissions()) != SVC_PEDESTRIAN)
    //             // lane not yet fully defined
    //             && !myLanes[fromIndex].connectionsDone
    //         ) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "    candidate from " << getID() << "_" << fromIndex << " to " << target->getID() << "\n";
    //             }
    // #endif
    //             // prevent same-edge conflicts
    //             if (
    //                 // no outgoing connections to the right from further left
    //                 ((it_avail + 1) == availableLanes.end() || count_if(myConnections.begin(), myConnections.end(), connections_conflict_finder(fromIndex, rightOfTarget, false)) == 0)
    //                 // no outgoing connections to the left from further right
    //                 && (it_avail == availableLanes.begin() || count_if(myConnections.begin(), myConnections.end(), connections_conflict_finder(fromIndex, leftOfTarget, true)) == 0)) {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) {
    //                     std::cout << "     request additional connection from " << getID() << "_" << fromIndex << " to " << target->getID() << "\n";
    //                 }
    // #endif
    //                 myConnections.push_back(Connection(fromIndex, target, -1));
    //                 numConsToTarget++;
    //             } else {
    // #ifdef DEBUG_CONNECTION_GUESSING
    //                 if (DEBUGCOND) std::cout
    //                             << "     fail check1="
    //                             << ((it_avail + 1) == availableLanes.end() || count_if(myConnections.begin(), myConnections.end(), connections_conflict_finder(fromIndex, rightOfTarget, false)) == 0)
    //                             << " check2=" << (it_avail == availableLanes.begin() || count_if(myConnections.begin(), myConnections.end(), connections_conflict_finder(fromIndex, leftOfTarget, true)) == 0)
    //                             << " rightOfTarget=" << rightOfTarget->getID()
    //                             << " leftOfTarget=" << leftOfTarget->getID()
    //                             << "\n";
    // #endif
    //
    //             }
    //         }
    //         ++it_avail;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_straight_connections(&mut self, outgoing: Ptr<EdgeVector>, availableLanes: &Vec<i32>, priorities: &Vec<i32>) {
        todo!("port NBEdge::addStraightConnections")
    }

    /// C++: `const std::vector<int> NBEdge::prepareEdgePriorities(const EdgeVector *, const std::vector<int> &)` [private]
    /// recomputes the edge priorities and manipulates them for a distribution of lanes on edges which is more like in real-life
    // const std::vector<int>
    // NBEdge::prepareEdgePriorities(const EdgeVector* outgoing, const std::vector<int>& availableLanes) {
    //     std::vector<int> priorities;
    //     MainDirections mainDirections(*outgoing, this, myTo, availableLanes);
    //     const int dist = mainDirections.getStraightest();
    //     if (dist == -1) {
    //         return priorities;
    //     }
    //     // copy the priorities first
    //     priorities.reserve(outgoing->size());
    //     for (const NBEdge* const out : *outgoing) {
    //         int prio = NBNode::isTrafficLight(myTo->getType()) ? 0 : out->getJunctionPriority(myTo);
    //         assert((prio + 1) * 2 > 0);
    //         prio = (prio + 1) * 2;
    //         priorities.push_back(prio);
    //     }
    //     // when the right turning direction has not a higher priority, divide
    //     //  the importance by 2 due to the possibility to leave the junction
    //     //  faster from this lane
    // #ifdef DEBUG_CONNECTION_GUESSING
    //     if (DEBUGCOND) std::cout << "  prepareEdgePriorities " << getID()
    //                                  << " outgoing=" << toString(*outgoing)
    //                                  << " priorities1=" << toString(priorities)
    //                                  << " dist=" << dist
    //                                  << "\n";
    // #endif
    //     if (dist != 0 && !mainDirections.includes(MainDirections::Direction::RIGHTMOST)) {
    //         assert(priorities.size() > 0);
    //         priorities[0] /= 2;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //         if (DEBUGCOND) {
    //             std::cout << "   priorities2=" << toString(priorities) << "\n";
    //         }
    // #endif
    //     }
    //     // HEURISTIC:
    //     // when no higher priority exists, let the forward direction be
    //     //  the main direction
    //     if (mainDirections.empty()) {
    //         assert(dist < (int)priorities.size());
    //         priorities[dist] *= 2;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //         if (DEBUGCOND) {
    //             std::cout << "   priorities3=" << toString(priorities) << "\n";
    //         }
    // #endif
    //     }
    //     if (NBNode::isTrafficLight(myTo->getType())) {
    //         priorities[dist] += 1;
    //     } else {
    //         // try to ensure separation of left turns
    //         if (mainDirections.includes(MainDirections::Direction::RIGHTMOST) && mainDirections.includes(MainDirections::Direction::LEFTMOST)) {
    //             priorities[0] /= 4;
    //             priorities[(int)priorities.size() - 1] /= 2;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "   priorities6=" << toString(priorities) << "\n";
    //             }
    // #endif
    //         } else if (mainDirections.includes(MainDirections::Direction::RIGHTMOST)
    //                    && outgoing->size() > 2
    //                    && availableLanes.size() == 2
    //                    && (*outgoing)[dist]->getPriority() == (*outgoing)[0]->getPriority()) {
    //             priorities[0] /= 4;
    //             priorities.back() /= 2;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "   priorities7=" << toString(priorities) << "\n";
    //             }
    // #endif
    //         }
    //     }
    //     if (mainDirections.includes(MainDirections::Direction::FORWARD)) {
    //         if (myLanes.size() > 2) {
    //             priorities[dist] *= 2;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "   priorities4=" << toString(priorities) << "\n";
    //             }
    // #endif
    //         } else {
    //             priorities[dist] *= 3;
    // #ifdef DEBUG_CONNECTION_GUESSING
    //             if (DEBUGCOND) {
    //                 std::cout << "   priorities5=" << toString(priorities) << "\n";
    //             }
    // #endif
    //         }
    //     }
    //     return priorities;
    // }
    #[allow(unused_variables)]
    pub fn prepare_edge_priorities(&mut self, outgoing: Ptr<EdgeVector>, availableLanes: &Vec<i32>) -> Vec<i32> {
        todo!("port NBEdge::prepareEdgePriorities")
    }

    /// C++: `void NBEdge::moveConnectionToLeft(int)` [private]
    /// moves a connection one place to the left;
    // void
    // NBEdge::moveConnectionToLeft(int lane) {
    // #ifdef DEBUG_CONNECTION_CHECKING
    //     std::cout << " moveConnectionToLeft " << getID() << " lane=" << lane << "\n";
    // #endif
    //     int index = 0;
    //     for (int i = 0; i < (int)myConnections.size(); ++i) {
    //         if (myConnections[i].fromLane == (int)(lane) && canMoveConnection(myConnections[i], lane + 1)) {
    //             index = i;
    //         }
    //     }
    //     std::vector<Connection>::iterator i = myConnections.begin() + index;
    //     Connection c = *i;
    //     myConnections.erase(i);
    //     setConnection(lane + 1, c.toEdge, c.toLane, Lane2LaneInfoType::VALIDATED, false);
    // }
    #[allow(unused_variables)]
    pub fn move_connection_to_left(&mut self, lane: i32) {
        todo!("port NBEdge::moveConnectionToLeft")
    }

    /// C++: `void NBEdge::moveConnectionToRight(int)` [private]
    /// moves a connection one place to the right; ! no checking for field validity
    // void
    // NBEdge::moveConnectionToRight(int lane) {
    // #ifdef DEBUG_CONNECTION_CHECKING
    //     std::cout << " moveConnectionToRight " << getID() << " lane=" << lane << "\n";
    // #endif
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end(); ++i) {
    //         if ((*i).fromLane == (int)lane && canMoveConnection(*i, lane - 1)) {
    //             Connection c = *i;
    //             i = myConnections.erase(i);
    //             setConnection(lane - 1, c.toEdge, c.toLane, Lane2LaneInfoType::VALIDATED, false);
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_connection_to_right(&mut self, lane: i32) {
        todo!("port NBEdge::moveConnectionToRight")
    }

    /// C++: `bool NBEdge::canMoveConnection(const Connection &, int) const` [const, private]
    /// whether the connection can originate on newFromLane
    // bool
    // NBEdge::canMoveConnection(const Connection& con, int newFromLane) const {
    //     // only allow using newFromLane if at least 1 vClass is permitted to use
    //     // this connection. If the connection shall be moved to a sidewalk, only create the connection if there is no walking area
    //     const SVCPermissions common = (getPermissions(newFromLane) & con.toEdge->getPermissions(con.toLane));
    //     return (common > 0 && common != SVC_PEDESTRIAN);
    // }
    #[allow(unused_variables)]
    pub fn can_move_connection(&self, con: &NBEdge_Connection, newFromLane: i32) -> bool {
        todo!("port NBEdge::canMoveConnection")
    }

    /// C++: `void NBEdge::computeAngle()` [private]
    /// computes the angle of this edge and stores it in myAngle
    // void
    // NBEdge::computeAngle() {
    //     // taking the angle at the first might be unstable, thus we take the angle
    //     // at a certain distance. (To compare two edges, additional geometry
    //     // segments are considered to resolve ambiguities)
    //     const bool hasFromShape = myFrom->getShape().size() > 0;
    //     const bool hasToShape = myTo->getShape().size() > 0;
    //     Position fromCenter = (hasFromShape ? myFrom->getShape().getCentroid() : myFrom->getPosition());
    //     Position toCenter = (hasToShape ? myTo->getShape().getCentroid() : myTo->getPosition());
    //     PositionVector shape = myGeom;
    //     if ((hasFromShape || hasToShape) && getNumLanes() > 0) {
    //         if (myLaneSpreadFunction == LaneSpreadFunction::RIGHT) {
    //             shape = myLanes[getNumLanes() - 1].shape ;
    //         } else {
    //             shape = myLanes[getNumLanes() / 2].shape;
    //             if (getNumLanes() % 2 == 0) {
    //                 // there is no center lane. shift to get the center
    //                 shape.move2side(getLaneWidth(getNumLanes() / 2) * 0.5);
    //             }
    //         }
    //     }
    //
    //     // if the junction shape is suspicious we cannot trust the angle to the centroid
    //     const bool suspiciousFromShape = hasFromShape && (myFrom->getShape().distance2D(shape[0]) > 2 * POSITION_EPS
    //                                      || myFrom->getShape().around(shape[-1])
    //                                      || !(myFrom->getShape().around(fromCenter)));
    //     const bool suspiciousToShape = hasToShape && (myTo->getShape().distance2D(shape[-1]) > 2 * POSITION_EPS
    //                                    || myTo->getShape().around(shape[0])
    //                                    || !(myTo->getShape().around(toCenter)));
    //
    //     const double angleLookahead = MIN2(shape.length2D() / 2, ANGLE_LOOKAHEAD);
    //     const Position referencePosStart = shape.positionAtOffset2D(angleLookahead);
    //     const Position referencePosEnd = shape.positionAtOffset2D(shape.length2D() - angleLookahead);
    //
    //     myStartAngle = GeomHelper::legacyDegree(fromCenter.angleTo2D(referencePosStart), true);
    //     const double myStartAngle2 = GeomHelper::legacyDegree(myFrom->getPosition().angleTo2D(referencePosStart), true);
    //     const double myStartAngle3 = getAngleAtNode(myFrom);
    //     myEndAngle = GeomHelper::legacyDegree(referencePosEnd.angleTo2D(toCenter), true);
    //     const double myEndAngle2 = GeomHelper::legacyDegree(referencePosEnd.angleTo2D(myTo->getPosition()), true);
    //     const double myEndAngle3 = getAngleAtNode(myTo);
    //
    // #ifdef DEBUG_ANGLES
    //     if (DEBUGCOND) {
    //         if (suspiciousFromShape) {
    //             std::cout << "suspiciousFromShape len=" << shape.length() << " startA=" << myStartAngle << " startA2=" << myStartAngle2 << " startA3=" << myStartAngle3
    //                       << " rel=" << NBHelpers::normRelAngle(myStartAngle, myStartAngle2)
    //                       << " fromCenter=" << fromCenter
    //                       << " fromPos=" << myFrom->getPosition()
    //                       << " refStart=" << referencePosStart
    //                       << "\n";
    //         }
    //         if (suspiciousToShape) {
    //             std::cout << "suspiciousToShape len=" << shape.length() << "  endA=" << myEndAngle << " endA2=" << myEndAngle2 << " endA3=" << myEndAngle3
    //                       << " rel=" << NBHelpers::normRelAngle(myEndAngle, myEndAngle2)
    //                       << " toCenter=" << toCenter
    //                       << " toPos=" << myTo->getPosition()
    //                       << " refEnd=" << referencePosEnd
    //                       << "\n";
    //         }
    //     }
    // #endif
    //
    //     if (suspiciousFromShape && shape.length() > 1) {
    //         myStartAngle = myStartAngle2;
    //     } else if (suspiciousToShape && fabs(NBHelpers::normRelAngle(myStartAngle, myStartAngle3)) > 90
    //                // don't trust footpath angles
    //                && (getPermissions() & ~SVC_PEDESTRIAN) != 0) {
    //         myStartAngle = myStartAngle3;
    //         if (myStartAngle < 0) {
    //             myStartAngle += 360;
    //         }
    //     }
    //
    //     if (suspiciousToShape && shape.length() > 1) {
    //         myEndAngle = myEndAngle2;
    //     } else if (suspiciousToShape && fabs(NBHelpers::normRelAngle(myEndAngle, myEndAngle3)) > 90
    //                // don't trust footpath angles
    //                && (getPermissions() & ~SVC_PEDESTRIAN) != 0) {
    //         myEndAngle = myEndAngle3;
    //         if (myEndAngle < 0) {
    //             myEndAngle += 360;
    //         }
    //     }
    //
    //     myTotalAngle = GeomHelper::legacyDegree(myFrom->getPosition().angleTo2D(myTo->getPosition()), true);
    // #ifdef DEBUG_ANGLES
    //     if (DEBUGCOND) std::cout << "computeAngle edge=" << getID()
    //                                  << " fromCenter=" << fromCenter << " toCenter=" << toCenter
    //                                  << " refStart=" << referencePosStart << " refEnd=" << referencePosEnd << " shape=" << shape
    //                                  << " hasFromShape=" << hasFromShape
    //                                  << " hasToShape=" << hasToShape
    //                                  << " numLanes=" << getNumLanes()
    //                                  << " shapeLane=" << getNumLanes() / 2
    //                                  << " startA=" << myStartAngle << " endA=" << myEndAngle << " totA=" << myTotalAngle << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_angle(&mut self) {
        todo!("port NBEdge::computeAngle")
    }

    /// C++: `bool NBEdge::bothLeftTurns(LinkDirection, const NBEdge *, LinkDirection) const` [const, private]
    /// determine conflict between opposite left turns
    // bool
    // NBEdge::bothLeftTurns(LinkDirection dir, const NBEdge* otherFrom, LinkDirection dir2) const {
    //     if (otherFrom == this) {
    //         // not an opposite pair
    //         return false;
    //     }
    //     return (dir == LinkDirection::LEFT || dir == LinkDirection::PARTLEFT) && (dir2 == LinkDirection::LEFT || dir2 == LinkDirection::PARTLEFT);
    // }
    #[allow(unused_variables)]
    pub fn both_left_turns(&self, dir: LinkDirection, otherFrom: Ptr<NBEdge>, dir2: LinkDirection) -> bool {
        todo!("port NBEdge::bothLeftTurns")
    }

    /// C++: `bool NBEdge::haveIntersection(const NBNode &, const PositionVector &, const NBEdge *, const NBEdge::Connection &, int, double, double, int) const` [const, private]
    // bool
    // NBEdge::haveIntersection(const NBNode& n, const PositionVector& shape, const NBEdge* otherFrom, const NBEdge::Connection& otherCon, int numPoints,
    //                          double width1, double width2, int shapeFlag) const {
    //     const PositionVector otherShape = n.computeInternalLaneShape(otherFrom, otherCon, numPoints, 0, shapeFlag);
    //     const double minDV = firstIntersection(shape, otherShape, width1, width2);
    //     return minDV < shape.length() - POSITION_EPS && minDV > POSITION_EPS;
    // }
    #[allow(unused_variables)]
    pub fn have_intersection(&self, n: &NBNode, shape: &PositionVector, otherFrom: Ptr<NBEdge>, otherCon: &NBEdge_Connection, numPoints: i32, width1: f64, width2: f64, shapeFlag: i32) -> bool {
        todo!("port NBEdge::haveIntersection")
    }

    /// C++: `bool NBEdge::hasRestrictedLane(SUMOVehicleClass) const` [const, private]
    /// returns whether any lane already allows the given vclass exclusively
    // bool
    // NBEdge::hasRestrictedLane(SUMOVehicleClass vclass) const {
    //     for (const Lane& lane : myLanes) {
    //         if (lane.permissions == vclass) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_restricted_lane(&self, vclass: SUMOVehicleClass) -> bool {
        todo!("port NBEdge::hasRestrictedLane")
    }

    /// C++: `void NBEdge::restoreRestrictedLane(SUMOVehicleClass, std::vector<NBEdge::Lane>, PositionVector, std::vector<NBEdge::Connection>)` [private]
    /// restore a restricted lane
    // void
    // NBEdge::restoreRestrictedLane(SUMOVehicleClass vclass, std::vector<NBEdge::Lane> oldLanes, PositionVector oldGeometry, std::vector<NBEdge::Connection> oldConnections) {
    //     // check that previously lane was transformed
    //     if (myLanes[0].permissions != vclass) {
    //         WRITE_WARNINGF(TL("Edge '%' doesn't have a dedicated lane for %s. Cannot be restored."), getID(), toString(vclass));
    //         return;
    //     }
    //     // restore old values
    //     myGeom = oldGeometry;
    //     myLanes = oldLanes;
    //     myConnections = oldConnections;
    //     // shift incoming connections to the right
    //     const EdgeVector& incoming = myFrom->getIncomingEdges();
    //     for (EdgeVector::const_iterator it = incoming.begin(); it != incoming.end(); ++it) {
    //         (*it)->shiftToLanesToEdge(this, 0);
    //     }
    //     // Shift TL conections
    //     myFrom->shiftTLConnectionLaneIndex(this, 0);
    //     myTo->shiftTLConnectionLaneIndex(this, 0);
    //     computeLaneShapes();
    // }
    #[allow(unused_variables)]
    pub fn restore_restricted_lane(&mut self, vclass: SUMOVehicleClass, oldLanes: Vec<NBEdge_Lane>, oldGeometry: PositionVector, oldConnections: Vec<NBEdge_Connection>) {
        todo!("port NBEdge::restoreRestrictedLane")
    }

    /// C++: `double NBEdge::assignInternalLaneLength(std::vector<Connection>::iterator, int, double, bool)` [private]
    /// assign length to all lanes of an internal edge
    // double
    // NBEdge::assignInternalLaneLength(std::vector<Connection>::iterator i, int numLanes, double lengthSum, bool averageLength) {
    //     // assign average length to all lanes of the same internal edge if averageLength is set
    //     // the lengthSum only covers the part up to the first internal junction
    //     // TODO This code assumes that either all connections in question have a via or none
    //     double maxCross = 0.;
    //     assert(i - myConnections.begin() >= numLanes);
    //     for (int prevIndex = 1; prevIndex <= numLanes; prevIndex++) {
    //         //std::cout << " con=" << (*(i - prevIndex)).getDescription(this) << " numLanes=" << numLanes << " avgLength=" << lengthSum / numLanes << "\n";
    //         Connection& c = (*(i - prevIndex));
    //         const double minLength = c.customLength != UNSPECIFIED_LOADED_LENGTH ? pow(10, -gPrecision) : POSITION_EPS;
    //         c.length = MAX2(minLength, averageLength ? lengthSum / numLanes : c.shape.length());
    //         if (c.haveVia) {
    //             c.viaLength = MAX2(minLength, c.viaShape.length());
    //         }
    //         if (c.customLength != UNSPECIFIED_LOADED_LENGTH) {
    //             if (c.haveVia) {
    //                 // split length proportionally
    //                 const double a = c.viaLength / (c.shape.length() + c.viaLength);
    //                 c.viaLength = MAX2(minLength, a * c.customLength);
    //             }
    //             if (!averageLength) {
    //                 c.length = MAX2(minLength, c.customLength - c.viaLength);
    //             }
    //         }
    //         if (c.haveVia) {
    //             // we need to be able to leave from the internal junction by accelerating from 0
    //             maxCross = MAX2(maxCross, sqrt(2. * c.viaLength)); // t = sqrt(2*s/a) and we assume 'a' is at least 1 (default value for tram in SUMOVTypeParameter)
    //         }
    //         // we need to be able to cross the junction in one go but not if we have an indirect left turn
    //         if (c.indirectLeft) {
    //             maxCross = MAX2(maxCross, MAX2(c.length, c.viaLength) / MAX2(c.vmax, NBOwnTLDef::MIN_SPEED_CROSSING_TIME));
    //         } else {
    //             maxCross = MAX2(maxCross, (c.length + c.viaLength) / MAX2(c.vmax, NBOwnTLDef::MIN_SPEED_CROSSING_TIME));
    //         }
    //     }
    //     return maxCross;
    // }
    #[allow(unused_variables)]
    pub fn assign_internal_lane_length(&mut self, i: Unmapped /* std::vector<Connection>::iterator */, numLanes: i32, lengthSum: f64, averageLength: bool) -> f64 {
        todo!("port NBEdge::assignInternalLaneLength")
    }

    /// C++: `std::vector<LinkDirection> NBEdge::decodeTurnSigns(int, int)` [static, private]
    /// decode bitset
    // std::vector<LinkDirection>
    // NBEdge::decodeTurnSigns(int turnSigns, int shift) {
    //     std::vector<LinkDirection> result;
    //     for (int i = 0; i < 8; i++) {
    //         // see LinkDirection in SUMOXMLDefinitions.h
    //         if ((turnSigns & (1 << (i + shift))) != 0) {
    //             result.push_back((LinkDirection)(1 << i));
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn decode_turn_signs(turnSigns: i32, shift: i32) -> Vec<LinkDirection> {
        todo!("port NBEdge::decodeTurnSigns")
    }

    /// C++: `void NBEdge::updateTurnPermissions(SVCPermissions &, LinkDirection, SVCPermissions, std::vector<LinkDirection>)` [static, private]
    // void
    // NBEdge::updateTurnPermissions(SVCPermissions& perm, LinkDirection dir, SVCPermissions spec, std::vector<LinkDirection> dirs) {
    //     if (dirs.size() > 0) {
    //         if (std::find(dirs.begin(), dirs.end(), dir) == dirs.end()) {
    //             perm &= ~spec;
    //         } else {
    //             perm |= spec;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_turn_permissions(perm: &mut SVCPermissions, dir: LinkDirection, spec: SVCPermissions, dirs: Vec<LinkDirection>) {
        todo!("port NBEdge::updateTurnPermissions")
    }

    /// C++: `bool NBEdge::applyTurnSigns()` [private]
    /// apply loaded turn sign information
    // bool
    // NBEdge::applyTurnSigns() {
    // #ifdef DEBUG_TURNSIGNS
    //     std::cout << "applyTurnSigns edge=" << getID() << "\n";
    // #endif
    //     // build a map of target edges and lanes
    //     std::vector<const NBEdge*> targets;
    //     std::map<const NBEdge*, std::vector<int> > toLaneMap;
    //     for (const Connection& c : myConnections) {
    //         if (myLanes[c.fromLane].turnSigns != 0) {
    //             if (std::find(targets.begin(), targets.end(), c.toEdge) == targets.end()) {
    //                 targets.push_back(c.toEdge);
    //             }
    //             toLaneMap[c.toEdge].push_back(c.toLane);
    //         }
    //     }
    //     // might be unsorted due to bike lane connections
    //     for (auto& item : toLaneMap) {
    //         std::sort(item.second.begin(), item.second.end());
    //     }
    //
    //     // check number of distinct signed directions and count the number of signs for each direction
    //     std::map<LinkDirection, int> signCons;
    //     int allDirs = 0;
    //     for (const Lane& lane : myLanes) {
    //         allDirs |= lane.turnSigns;
    //         for (LinkDirection dir : decodeTurnSigns(lane.turnSigns)) {
    //             signCons[dir]++;
    //         }
    //     }
    //     allDirs |= allDirs >> TURN_SIGN_SHIFT_BUS;
    //     allDirs |= allDirs >> TURN_SIGN_SHIFT_TAXI;
    //     allDirs |= allDirs >> TURN_SIGN_SHIFT_BICYCLE;
    //
    //     if ((allDirs & (int)LinkDirection::NODIR) != 0) {
    //         targets.push_back(nullptr); // dead end
    //     }
    //
    //     SVCPermissions defaultPermissions = SVC_PASSENGER | SVC_DELIVERY;
    //     // build a mapping from sign directions to targets
    //     std::vector<LinkDirection> signedDirs = decodeTurnSigns(allDirs);
    //     std::map<LinkDirection, const NBEdge*> dirMap;
    // #ifdef DEBUG_TURNSIGNS
    //     std::cout << "  numDirs=" << signedDirs.size() << " numTargets=" << targets.size() << "\n";
    // #endif
    //     if (signedDirs.size() > targets.size()) {
    //         WRITE_WARNINGF(TL("Cannot apply turn sign information for edge '%' because there are % signed directions but only % targets"), getID(), signedDirs.size(), targets.size());
    //         return false;
    //     } else if (signedDirs.size() < targets.size()) {
    //         // we need to drop some targets (i.e. turn-around)
    //         // use sumo-directions as a guide
    //         std::vector<LinkDirection> sumoDirs;
    //         for (const NBEdge* to : targets) {
    //             sumoDirs.push_back(myTo->getDirection(this, to));
    //         }
    //         // remove targets to the left
    //         bool checkMore = true;
    //         while (signedDirs.size() < targets.size() && checkMore) {
    //             checkMore = false;
    //             //std::cout << getID() << " sumoDirs=" << joinToString(sumoDirs, ",") << " signedDirs=" << joinToString(signedDirs, ",") << "\n";
    //             if (sumoDirs.back() != signedDirs.back()) {
    //                 targets.pop_back();
    //                 sumoDirs.pop_back();
    //                 checkMore = true;
    //             }
    //         }
    //         // remove targets to the right
    //         checkMore = true;
    //         while (signedDirs.size() < targets.size() && checkMore) {
    //             checkMore = false;
    //             if (sumoDirs.front() != signedDirs.front()) {
    //                 targets.erase(targets.begin());
    //                 sumoDirs.erase(sumoDirs.begin());
    //                 checkMore = true;
    //             }
    //         }
    //         // remove targets by permissions
    //         int i = 0;
    //         while (signedDirs.size() < targets.size() && i < (int)targets.size()) {
    //             if (targets[i] != nullptr && (targets[i]->getPermissions() & defaultPermissions) == 0) {
    //                 targets.erase(targets.begin() + i);
    //                 sumoDirs.erase(sumoDirs.begin() + i);
    //             } else {
    //                 i++;
    //             }
    //         }
    //         if (signedDirs.size() != targets.size()) {
    //             WRITE_WARNINGF(TL("Cannot apply turn sign information for edge '%' because there are % signed directions and % targets (after target pruning)"), getID(), signedDirs.size(), targets.size());
    //             return false;
    //         }
    //     }
    //     // directions and connections are both sorted from right to left
    //     for (int i = 0; i < (int)signedDirs.size(); i++) {
    //         dirMap[signedDirs[i]] = targets[i];
    //     }
    //     // check whether we have enough target lanes for a each signed direction
    //     for (auto item : signCons) {
    //         const LinkDirection dir = item.first;
    //         if (dir == LinkDirection::NODIR) {
    //             continue;
    //         }
    //         const NBEdge* to = dirMap[dir];
    //         int candidates = to->getNumLanesThatAllow(defaultPermissions, false);
    //         if (candidates == 0) {
    //             WRITE_WARNINGF(TL("Cannot apply turn sign information for edge '%' because the target edge '%' has no suitable lanes"), getID(), to->getID());
    //             return false;
    //         }
    //         std::vector<int>& knownTargets = toLaneMap[to];
    //         if ((int)knownTargets.size() < item.second) {
    //             if (candidates < item.second) {
    //                 WRITE_WARNINGF(TL("Cannot apply turn sign information for edge '%' because there are % signed connections with directions '%' but target edge '%' has only % suitable lanes"),
    //                                getID(), item.second, toString(dir), to->getID(), candidates);
    //                 return false;
    //             }
    //             int i;
    //             int iInc;
    //             int iEnd;
    //             if (dir > LinkDirection::STRAIGHT) {
    //                 // set more targets on the left
    //                 i = to->getNumLanes() - 1;
    //                 iInc = -1;
    //                 iEnd = -1;
    //             } else {
    //                 // set more targets on the right
    //                 i = 0;
    //                 iInc = 1;
    //                 iEnd = to->getNumLanes();
    //             }
    //             while ((int)knownTargets.size() < item.second && i != iEnd) {
    //                 if ((to->getPermissions(i) & defaultPermissions) != 0) {
    //                     if (std::find(knownTargets.begin(), knownTargets.end(), i) == knownTargets.end()) {
    //                         knownTargets.push_back(i);
    //                     }
    //                 }
    //                 i += iInc;
    //             }
    //             if ((int)knownTargets.size() != item.second) {
    //                 WRITE_WARNINGF(TL("Cannot apply turn sign information for edge '%' because not enough target lanes could be determined for direction '%'"), getID(), toString(dir));
    //                 return false;
    //             }
    //             std::sort(knownTargets.begin(), knownTargets.end());
    //         }
    //     }
    //     std::map<const NBEdge*, int> toLaneIndex;
    //     for (int i = 0; i < getNumLanes(); i++) {
    //         const int turnSigns = myLanes[i].turnSigns;
    //         // no turnSigns are given for bicycle lanes and sidewalks
    //         if (turnSigns != 0) {
    //             // clear existing connections
    //             for (auto it = myConnections.begin(); it != myConnections.end();) {
    //                 if (it->fromLane == i) {
    //                     it = myConnections.erase(it);
    //                 } else {
    //                     it++;
    //                 }
    //             }
    //             // add new connections
    //             int allSigns = (turnSigns
    //                             | turnSigns >> TURN_SIGN_SHIFT_BUS
    //                             | turnSigns >> TURN_SIGN_SHIFT_TAXI
    //                             | turnSigns >> TURN_SIGN_SHIFT_BICYCLE);
    //             std::vector<LinkDirection> all = decodeTurnSigns(turnSigns);
    //             std::vector<LinkDirection> bus = decodeTurnSigns(turnSigns, TURN_SIGN_SHIFT_BUS);
    //             std::vector<LinkDirection> taxi = decodeTurnSigns(turnSigns, TURN_SIGN_SHIFT_TAXI);
    //             std::vector<LinkDirection> bike = decodeTurnSigns(turnSigns, TURN_SIGN_SHIFT_BICYCLE);
    //             //std::cout << "  allSigns=" << allSigns << " turnSigns=" << turnSigns << " bus=" << bus.size() << "\n";
    //             SVCPermissions fromP = getPermissions(i);
    //             if ((fromP & SVC_PASSENGER) != 0) {
    //                 // if the source permits passenger traffic, the target should too
    //                 fromP = SVC_PASSENGER;
    //             }
    //             for (LinkDirection dir : decodeTurnSigns(allSigns)) {
    //                 SVCPermissions perm = 0;
    //                 updateTurnPermissions(perm, dir, SVCAll, all);
    //                 updateTurnPermissions(perm, dir, SVC_BUS, bus);
    //                 updateTurnPermissions(perm, dir, SVC_TAXI, taxi);
    //                 updateTurnPermissions(perm, dir, SVC_BICYCLE, bike);
    //                 if (perm == SVCAll) {
    //                     perm = SVC_UNSPECIFIED;
    //                 }
    //                 //std::cout << "   lane=" << i << " dir=" << toString(dir) << " perm=" << getVehicleClassNames(perm) << "\n";
    //                 NBEdge* to = const_cast<NBEdge*>(dirMap[dir]);
    //                 if (to != nullptr) {
    //                     if (toLaneIndex.count(to) == 0) {
    //                         // initialize to rightmost feasible lane
    //                         int toLane = toLaneMap[to][0];
    //                         while ((to->getPermissions(toLane) & fromP) == 0 && (toLane + 1 < to->getNumLanes())) {
    //                             toLane++;
    //                             /*
    //                             if (toLane == to->getNumLanes()) {
    //                                 SOFT_ASSERT(false);
    //                             #ifdef DEBUG_TURNSIGNS
    //                                 std::cout << "  could not find passenger lane for target=" << to->getID() << "\n";
    //                             #endif
    //                                 return false;
    //                             }
    //                             */
    //                         }
    // #ifdef DEBUG_TURNSIGNS
    //                         std::cout << "  target=" << to->getID() << " initial toLane=" << toLane << "\n";
    // #endif
    //                         toLaneIndex[to] = toLane;
    //                     }
    // #ifdef DEBUG_TURNSIGNS
    //                     //std::cout << "  set fromLane=" << i << " to=" << to->getID() << " toLane=" << toLaneIndex[to] << "\n";
    // #endif
    //                     setConnection(i, to, toLaneIndex[to], Lane2LaneInfoType::VALIDATED, true,
    //                                   false, KEEPCLEAR_UNSPECIFIED, UNSPECIFIED_CONTPOS,
    //                                   UNSPECIFIED_VISIBILITY_DISTANCE, UNSPECIFIED_SPEED, UNSPECIFIED_FRICTION,
    //                                   myDefaultConnectionLength, PositionVector::EMPTY,
    //                                   UNSPECIFIED_CONNECTION_UNCONTROLLED,
    //                                   perm);
    //                     if (toLaneIndex[to] < to->getNumLanes() - 1
    //                             && (to->getPermissions(toLaneIndex[to] + 1) & fromP) != 0) {
    //                         toLaneIndex[to]++;
    //                     } else if (toLaneIndex[to] < to->getNumLanes() - 2
    //                                && (to->getPermissions(toLaneIndex[to] + 2) & fromP) != 0) {
    //                         // skip forbidden lane
    //                         toLaneIndex[to] += 2;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     sortOutgoingConnectionsByAngle();
    //     sortOutgoingConnectionsByIndex();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn apply_turn_signs(&mut self) -> bool {
        todo!("port NBEdge::applyTurnSigns")
    }

    /// C++: `void NBEdge::removeInvalidConnections()` [private]
    // void NBEdge::removeInvalidConnections() {
    //     // check restrictions
    //     for (std::vector<Connection>::iterator i = myConnections.begin(); i != myConnections.end();) {
    //         Connection& c = *i;
    //         const SVCPermissions common = getPermissions(c.fromLane) & c.toEdge->getPermissions(c.toLane);
    //         if (common == SVC_PEDESTRIAN || getPermissions(c.fromLane) == SVC_PEDESTRIAN) {
    //             // these are computed in NBNode::buildWalkingAreas
    // #ifdef DEBUG_CONNECTION_CHECKING
    //             std::cout << " remove pedCon " << c.getDescription(this) << "\n";
    // #endif
    //             i = myConnections.erase(i);
    //         } else if (common == 0) {
    //             // no common permissions.
    //             // try to find a suitable target lane to the right
    //             const int origToLane = c.toLane;
    //             c.toLane = -1; // ignore this connection when calling hasConnectionTo
    //             int toLane = origToLane;
    //             while (toLane > 0
    //                     && (getPermissions(c.fromLane) & c.toEdge->getPermissions(toLane)) == 0
    //                     && !hasConnectionTo(c.toEdge, toLane)
    //                   ) {
    //                 toLane--;
    //             }
    //             if ((getPermissions(c.fromLane) & c.toEdge->getPermissions(toLane)) != 0
    //                     && !hasConnectionTo(c.toEdge, toLane)) {
    //                 c.toLane = toLane;
    //                 ++i;
    //             } else {
    //                 // try to find a suitable target lane to the left
    //                 toLane = origToLane;
    //                 while (toLane < (int)c.toEdge->getNumLanes() - 1
    //                         && (getPermissions(c.fromLane) & c.toEdge->getPermissions(toLane)) == 0
    //                         && !hasConnectionTo(c.toEdge, toLane)
    //                       ) {
    //                     toLane++;
    //                 }
    //                 if ((getPermissions(c.fromLane) & c.toEdge->getPermissions(toLane)) != 0
    //                         && !hasConnectionTo(c.toEdge, toLane)) {
    //                     c.toLane = toLane;
    //                     ++i;
    //                 } else {
    //                     // no alternative target found
    // #ifdef DEBUG_CONNECTION_CHECKING
    //                     std::cout << " remove " << c.getDescription(this) << " with no alternative target\n";
    // #endif
    //                     i = myConnections.erase(i);
    //                 }
    //             }
    //         } else if (isRailway(getPermissions(c.fromLane)) && isRailway(c.toEdge->getPermissions(c.toLane))
    //                    && isTurningDirectionAt(c.toEdge))  {
    //             // do not allow sharp rail turns
    // #ifdef DEBUG_CONNECTION_CHECKING
    //             std::cout << " remove " << c.getDescription(this) << " (rail turnaround)\n";
    // #endif
    //             i = myConnections.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_invalid_connections(&mut self) {
        todo!("port NBEdge::removeInvalidConnections")
    }

    /// C++: `bool NBEdge::connections_sorter(const Connection &, const Connection &)` [static]
    /// connections_sorter sort by fromLane, toEdge and toLane
    // bool
    // NBEdge::connections_sorter(const Connection& c1, const Connection& c2) {
    //     if (c1.fromLane != c2.fromLane) {
    //         return c1.fromLane < c2.fromLane;
    //     }
    //     if (c1.toEdge != c2.toEdge) {
    //         return false; // do not change ordering among toEdges as this is determined by angle in an earlier step
    //     }
    //     return c1.toLane < c2.toLane;
    // }
    #[allow(unused_variables)]
    pub fn connections_sorter(c1: &NBEdge_Connection, c2: &NBEdge_Connection) -> bool {
        todo!("port NBEdge::connections_sorter")
    }

    /// C++: `void NBEdge::NBEdge(const NBEdge &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_4(s: &NBEdge) -> Self {
        todo!("port NBEdge::NBEdge")
    }

    /// C++: `NBEdge & NBEdge::operator=(const NBEdge &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBEdge) -> &mut NBEdge {
        todo!("port NBEdge::operator=")
    }

    /// C++: `void NBEdge::NBEdge()` [private]
    /// constructor for dummy edge
    // NBEdge::NBEdge() :
    //     Named("DUMMY"),
    //     myStep(EdgeBuildingStep::INIT),
    //     myFrom(nullptr), myTo(nullptr),
    //     myStartAngle(0), myEndAngle(0), myTotalAngle(0),
    //     myPriority(0), mySpeed(0), myFriction(UNSPECIFIED_FRICTION),
    //     myDistance(0),
    //     myTurnDestination(nullptr),
    //     myPossibleTurnDestination(nullptr),
    //     myFromJunctionPriority(-1), myToJunctionPriority(-1),
    //     myLaneSpreadFunction(LaneSpreadFunction::RIGHT),
    //     myEndOffset(0),
    //     myEdgeStopOffset(StopOffset()),
    //     myLaneWidth(0),
    //     myLoadedLength(UNSPECIFIED_LOADED_LENGTH),
    //     myAmInTLS(false),
    //     myAmMacroscopicConnector(false),
    //     mySignalPosition(Position::INVALID),
    //     mySignalNode(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new_5() -> Self {
        todo!("port NBEdge::NBEdge")
    }

}

/// Superclass for NBEdge and NBEdge::Connection to initialize Router
/// C++ class `NBRouterEdge` (netbuild/NBEdge.h)
pub struct NBRouterEdge {
}

impl NBRouterEdge {
    /// C++: `void NBRouterEdge::~NBRouterEdge() noexcept` [virtual]
    // virtual ~NBRouterEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NBRouterEdge::~NBRouterEdge")
    }

    /// C++: `const std::string & NBRouterEdge::getID() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        todo!("port NBRouterEdge::getID")
    }

    /// C++: `double NBRouterEdge::getSpeed() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port NBRouterEdge::getSpeed")
    }

    /// C++: `double NBRouterEdge::getLength() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port NBRouterEdge::getLength")
    }

    /// C++: `const NBRouterEdge * NBRouterEdge::getBidiEdge() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<NBRouterEdge> {
        todo!("port NBRouterEdge::getBidiEdge")
    }

    /// C++: `int NBRouterEdge::getNumericalID() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port NBRouterEdge::getNumericalID")
    }

    /// C++: `const ConstRouterEdgePairVector & NBRouterEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ConstRouterEdgePairVector {
        todo!("port NBRouterEdge::getViaSuccessors")
    }

    /// C++: `bool NBRouterEdge::isInternal() const` [const, virtual]
    // virtual bool isInternal() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        todo!("port NBRouterEdge::isInternal")
    }

    /// C++: `bool NBRouterEdge::prohibits(const NBVehicle *const) const` [const]
    // inline bool prohibits(const NBVehicle* const /*veh*/) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, arg0: Ptr<NBVehicle>) -> bool {
        todo!("port NBRouterEdge::prohibits")
    }

    /// C++: `bool NBRouterEdge::restricts(const NBVehicle *const) const` [const]
    // inline bool restricts(const NBVehicle* const /*veh*/) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, arg0: Ptr<NBVehicle>) -> bool {
        todo!("port NBRouterEdge::restricts")
    }

    /// C++: `double NBRouterEdge::getTravelTimeStatic(const NBRouterEdge *const, const NBVehicle *const, double)` [static]
    // static inline double getTravelTimeStatic(const NBRouterEdge* const edge, const NBVehicle* const /*veh*/, double /*time*/) {
    //         return edge->getLength() / edge->getSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<NBRouterEdge>, arg1: Ptr<NBVehicle>, arg2: f64) -> f64 {
        todo!("port NBRouterEdge::getTravelTimeStatic")
    }

}

/// A class that being a bresenham-callback assigns the incoming lanes to the edges
/// C++ class `NBEdge::ToEdgeConnectionsAdder` (netbuild/NBEdge.h)
pub struct NBEdge_ToEdgeConnectionsAdder {
    /// C++ base class `Bresenham::BresenhamCallBack`
    pub base: Unmapped /* Bresenham::BresenhamCallBack */,
    /// map of edges to this edge's lanes that reach them
    pub myConnections: BTreeMap<Ptr<NBEdge>, Vec<i32>>,
    /// the transition from the virtual lane to the edge it belongs to
    pub myTransitions: Ptr<EdgeVector>,
}

impl std::ops::Deref for NBEdge_ToEdgeConnectionsAdder {
    type Target = Unmapped /* Bresenham::BresenhamCallBack */;
    fn deref(&self) -> &Unmapped /* Bresenham::BresenhamCallBack */ {
        &self.base
    }
}
impl std::ops::DerefMut for NBEdge_ToEdgeConnectionsAdder {
    fn deref_mut(&mut self) -> &mut Unmapped /* Bresenham::BresenhamCallBack */ {
        &mut self.base
    }
}

impl NBEdge_ToEdgeConnectionsAdder {
    /// C++: `void ToEdgeConnectionsAdder::ToEdgeConnectionsAdder(const EdgeVector &)`
    /// constructor
    // ToEdgeConnectionsAdder(const EdgeVector& transitions)
    //             : myTransitions(transitions) { }
    #[allow(unused_variables)]
    pub fn new(transitions: &EdgeVector) -> Self {
        todo!("port ToEdgeConnectionsAdder::ToEdgeConnectionsAdder")
    }

    /// C++: `void ToEdgeConnectionsAdder::~ToEdgeConnectionsAdder() noexcept` [virtual]
    /// destructor
    // ~ToEdgeConnectionsAdder() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ToEdgeConnectionsAdder::~ToEdgeConnectionsAdder")
    }

    /// C++: `void ToEdgeConnectionsAdder::execute(const int, const int)` [virtual]
    /// executes a bresenham - step
    // void
    // NBEdge::ToEdgeConnectionsAdder::execute(const int lane, const int virtEdge) {
    //     // check
    //     assert((int)myTransitions.size() > virtEdge);
    //     // get the approached edge
    //     NBEdge* succEdge = myTransitions[virtEdge];
    //     std::vector<int> lanes;
    //
    //     // check whether the currently regarded, approached edge has already
    //     //  a connection starting at the edge which is currently being build
    //     std::map<NBEdge*, std::vector<int> >::iterator i = myConnections.find(succEdge);
    //     if (i != myConnections.end()) {
    //         // if there were already lanes assigned, get them
    //         lanes = (*i).second;
    //     }
    //
    //     // check whether the current lane was already used to connect the currently
    //     //  regarded approached edge
    //     std::vector<int>::iterator j = std::find(lanes.begin(), lanes.end(), lane);
    //     if (j == lanes.end()) {
    //         // if not, add it to the list
    //         lanes.push_back(lane);
    //     }
    //     // set information about connecting lanes
    //     myConnections[succEdge] = lanes;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, lane: i32, virtEdge: i32) {
        todo!("port ToEdgeConnectionsAdder::execute")
    }

    /// C++: `const std::map<NBEdge *, std::vector<int>> & ToEdgeConnectionsAdder::getBuiltConnections() const` [const]
    /// get built connections
    // const std::map<NBEdge*, std::vector<int> >& getBuiltConnections() const {
    //             return myConnections;
    //         }
    #[allow(unused_variables)]
    pub fn get_built_connections(&self) -> &BTreeMap<Ptr<NBEdge>, Vec<i32>> {
        todo!("port ToEdgeConnectionsAdder::getBuiltConnections")
    }

    /// C++: `void ToEdgeConnectionsAdder::ToEdgeConnectionsAdder(const ToEdgeConnectionsAdder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &NBEdge_ToEdgeConnectionsAdder) -> Self {
        todo!("port ToEdgeConnectionsAdder::ToEdgeConnectionsAdder")
    }

    /// C++: `ToEdgeConnectionsAdder & ToEdgeConnectionsAdder::operator=(const ToEdgeConnectionsAdder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &NBEdge_ToEdgeConnectionsAdder) -> &mut NBEdge_ToEdgeConnectionsAdder {
        todo!("port ToEdgeConnectionsAdder::operator=")
    }

}

/// C++ class `NBEdge::connections_conflict_finder` (netbuild/NBEdge.h)
pub struct NBEdge_connections_conflict_finder {
    /// index of from lane
    pub myFromLane: i32,
    /// edge to find
    pub myEdge2Find: Ptr<NBEdge>,
    /// check if is right
    pub myCheckRight: bool,
}

impl NBEdge_connections_conflict_finder {
    /// C++: `void connections_conflict_finder::connections_conflict_finder(int, NBEdge *const, bool)`
    /// constructor
    // connections_conflict_finder(int fromLane, NBEdge* const edge2find, bool checkRight) :
    //             myFromLane(fromLane), myEdge2Find(edge2find), myCheckRight(checkRight) { }
    #[allow(unused_variables)]
    pub fn new(fromLane: i32, edge2find: Ptr<NBEdge>, checkRight: bool) -> Self {
        todo!("port connections_conflict_finder::connections_conflict_finder")
    }

    /// C++: `bool connections_conflict_finder::operator()(const Connection &) const` [const]
    /// operator ()
    // bool operator()(const Connection& c) const {
    //             return (((myCheckRight && c.fromLane < myFromLane) || (!myCheckRight && c.fromLane > myFromLane))
    //                     && c.fromLane >= 0 // already assigned
    //                     && c.toEdge == myEdge2Find);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBEdge_Connection) -> bool {
        todo!("port connections_conflict_finder::operator()")
    }

}

/// C++ class `NBEdge::connections_finder` (netbuild/NBEdge.h)
pub struct NBEdge_connections_finder {
    /// index of from lane
    pub myFromLane: i32,
    /// edge to find
    pub myEdge2Find: Ptr<NBEdge>,
    /// lane to find
    pub myLane2Find: i32,
    /// invert edge to find
    pub myInvertEdge2find: bool,
}

impl NBEdge_connections_finder {
    /// C++: `void connections_finder::connections_finder(int, NBEdge *const, int, bool)`
    /// constructor
    // connections_finder(int fromLane, NBEdge* const edge2find, int lane2find, bool invertEdge2find = false) :
    //             myFromLane(fromLane), myEdge2Find(edge2find), myLane2Find(lane2find), myInvertEdge2find(invertEdge2find) { }
    #[allow(unused_variables)]
    pub fn new(fromLane: i32, edge2find: Ptr<NBEdge>, lane2find: i32, invertEdge2find: bool) -> Self {
        todo!("port connections_finder::connections_finder")
    }

    /// C++: `bool connections_finder::operator()(const Connection &) const` [const]
    /// operator ()
    // bool operator()(const Connection& c) const {
    //             return ((c.fromLane == myFromLane || myFromLane == -1)
    //                     && ((!myInvertEdge2find && c.toEdge == myEdge2Find) || (myInvertEdge2find && c.toEdge != myEdge2Find))
    //                     && (c.toLane == myLane2Find || myLane2Find == -1));
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBEdge_Connection) -> bool {
        todo!("port connections_finder::operator()")
    }

}

/// C++ class `NBEdge::connections_fromlane_finder` (netbuild/NBEdge.h)
pub struct NBEdge_connections_fromlane_finder {
    /// index of lane to find
    pub myLane2Find: i32,
}

impl NBEdge_connections_fromlane_finder {
    /// C++: `void connections_fromlane_finder::connections_fromlane_finder(int)`
    // connections_fromlane_finder(int lane2find) : myLane2Find(lane2find) { }
    #[allow(unused_variables)]
    pub fn new(lane2find: i32) -> Self {
        todo!("port connections_fromlane_finder::connections_fromlane_finder")
    }

    /// C++: `bool connections_fromlane_finder::operator()(const Connection &) const` [const]
    /// operator ()
    // bool operator()(const Connection& c) const {
    //             return c.fromLane == myLane2Find;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBEdge_Connection) -> bool {
        todo!("port connections_fromlane_finder::operator()")
    }

    /// C++: `connections_fromlane_finder & connections_fromlane_finder::operator=(const connections_fromlane_finder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NBEdge_connections_fromlane_finder) -> &mut NBEdge_connections_fromlane_finder {
        todo!("port connections_fromlane_finder::operator=")
    }

}

/// Class to sort edges by their angle
/// C++ class `NBEdge::connections_relative_edgelane_sorter` (netbuild/NBEdge.h)
pub struct NBEdge_connections_relative_edgelane_sorter {
    /// the edge to compute the relative angle of
    pub myEdge: Ptr<NBEdge>,
}

impl NBEdge_connections_relative_edgelane_sorter {
    /// C++: `void connections_relative_edgelane_sorter::connections_relative_edgelane_sorter(NBEdge *)`
    /// constructor
    // explicit connections_relative_edgelane_sorter(NBEdge* e) : myEdge(e) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<NBEdge>) -> Self {
        todo!("port connections_relative_edgelane_sorter::connections_relative_edgelane_sorter")
    }

    /// C++: `int connections_relative_edgelane_sorter::operator()(const Connection &, const Connection &) const` [const]
    /// comparing operation
    // int
    // NBEdge::connections_relative_edgelane_sorter::operator()(const Connection& c1, const Connection& c2) const {
    //     if (c1.toEdge != c2.toEdge) {
    //         return NBContHelper::relative_outgoing_edge_sorter(myEdge)(c1.toEdge, c2.toEdge);
    //     }
    //     return c1.toLane < c2.toLane;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, c1: &NBEdge_Connection, c2: &NBEdge_Connection) -> i32 {
        todo!("port connections_relative_edgelane_sorter::operator()")
    }

}

/// C++ class `NBEdge::connections_toedge_finder` (netbuild/NBEdge.h)
pub struct NBEdge_connections_toedge_finder {
    /// check if has from lane
    pub myHasFromLane: bool,
    /// edge to find
    pub myEdge2Find: Ptr<NBEdge>,
}

impl NBEdge_connections_toedge_finder {
    /// C++: `void connections_toedge_finder::connections_toedge_finder(const NBEdge *const, bool)`
    /// constructor
    // connections_toedge_finder(const NBEdge* const edge2find, bool hasFromLane = false) :
    //             myHasFromLane(hasFromLane),
    //             myEdge2Find(edge2find) { }
    #[allow(unused_variables)]
    pub fn new(edge2find: Ptr<NBEdge>, hasFromLane: bool) -> Self {
        todo!("port connections_toedge_finder::connections_toedge_finder")
    }

    /// C++: `bool connections_toedge_finder::operator()(const Connection &) const` [const]
    /// operator ()
    // bool operator()(const Connection& c) const {
    //             return c.toEdge == myEdge2Find && (!myHasFromLane || c.fromLane != -1);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBEdge_Connection) -> bool {
        todo!("port connections_toedge_finder::operator()")
    }

}

/// C++ class `NBEdge::connections_toedgelane_finder` (netbuild/NBEdge.h)
pub struct NBEdge_connections_toedgelane_finder {
    /// edge to find
    pub myEdge2Find: Ptr<NBEdge>,
    /// lane to find
    pub myLane2Find: i32,
    /// from lane to find
    pub myFromLane2Find: i32,
}

impl NBEdge_connections_toedgelane_finder {
    /// C++: `void connections_toedgelane_finder::connections_toedgelane_finder(const NBEdge *const, int, int)`
    /// constructor
    // connections_toedgelane_finder(const NBEdge* const edge2find, int lane2find, int fromLane2find) :
    //             myEdge2Find(edge2find),
    //             myLane2Find(lane2find),
    //             myFromLane2Find(fromLane2find) { }
    #[allow(unused_variables)]
    pub fn new(edge2find: Ptr<NBEdge>, lane2find: i32, fromLane2find: i32) -> Self {
        todo!("port connections_toedgelane_finder::connections_toedgelane_finder")
    }

    /// C++: `bool connections_toedgelane_finder::operator()(const Connection &) const` [const]
    /// operator ()
    // bool operator()(const Connection& c) const {
    //             return c.toEdge == myEdge2Find && c.toLane == myLane2Find && (myFromLane2Find < 0 || c.fromLane == myFromLane2Find);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, c: &NBEdge_Connection) -> bool {
        todo!("port connections_toedgelane_finder::operator()")
    }

}

