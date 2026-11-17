//! Port of microsim/MSLane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::SUMO_CONST_HALTING_SPEED;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};
use crate::microsim::ms_net::STAGE_MOVEMENTS;

/// C++ enum `MSLane::CollisionAction`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSLane_CollisionAction {
    COLLISION_ACTION_NONE = 0,
    COLLISION_ACTION_WARN = 1,
    COLLISION_ACTION_TELEPORT = 2,
    COLLISION_ACTION_REMOVE = 3,
}

/// C++ enum `MSLane::MinorLinkMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSLane_MinorLinkMode {
    FOLLOW_NEVER = 0,
    FOLLOW_ALWAYS = 1,
    FOLLOW_ONCOMING = 2,
}

/// C++ typedef `std::map<std::string, MSLane *> MSLane::DictType`
pub type MSLane_DictType = BTreeMap<String, Ptr<MSLane>>;

/// C++ typedef `std::map<const MSLane *, std::pair<double, double>> LaneCoverageInfo`
pub type LaneCoverageInfo = BTreeMap<Ptr<MSLane>, (f64, f64)>;

/// C++ typedef `std::vector<MSVehicle *> MSLane::VehCont`
pub type MSLane_VehCont = Vec<Ptr<MSVehicle>>;

/// Member-wise copy of an MSLeaderInfo (C++ implicit copy constructor/assignment).
/// MSLeaderInfo in ms_leader_info.rs does not derive Clone.
fn leader_info_copy(src: &MSLeaderInfo) -> MSLeaderInfo {
    MSLeaderInfo {
        myWidth: src.myWidth,
        myOffset: src.myOffset,
        myVehicles: src.myVehicles.clone(),
        myFreeSublanes: src.myFreeSublanes,
        egoRightMost: src.egoRightMost,
        egoLeftMost: src.egoLeftMost,
        myHasVehicles: src.myHasVehicles,
    }
}

/// AnyVehicleIterator is a structure, which manages the iteration through all vehicles on the lane, that may be of importance for the car-following dynamics along that lane. The relevant types of vehicles are: 1) vehicles with their front on the lane (myVehicles), 2) vehicles intersecting the lane but with front on another lane (myPartialVehicles)
/// C++ class `MSLane::AnyVehicleIterator` (microsim/MSLane.h)
pub struct MSLane_AnyVehicleIterator {
    /// the lane that is being iterated
    pub myLane: Ptr<MSLane>,
    /// index for myVehicles
    pub myI1: i32,
    /// index for myPartialVehicles
    pub myI2: i32,
    /// index for myTmpVehicles
    pub myI3: i32,
    /// end index for myVehicles
    pub myI1End: i32,
    /// end index for myPartialVehicles
    pub myI2End: i32,
    /// end index for myTmpVehicles
    pub myI3End: i32,
    /// iteration direction
    pub myDownstream: bool,
    /// index delta
    pub myDirection: i32,
}

impl MSLane_AnyVehicleIterator {
    /// C++: `void AnyVehicleIterator::AnyVehicleIterator(const MSLane *, int, int, int, const int, const int, const int, bool)`
    // AnyVehicleIterator(
    //             const MSLane* lane,
    //             int i1,
    //             int i2,
    //             int i3,
    //             const int i1End,
    //             const int i2End,
    //             const int i3End,
    //             bool downstream = true) :
    //             myLane(lane),
    //             myI1(i1),
    //             myI2(i2),
    //             myI3(i3),
    //             myI1End(i1End),
    //             myI2End(i2End),
    //             myI3End(i3End),
    //             myDownstream(downstream),
    //             myDirection(downstream ? 1 : -1) {
    //         }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>, i1: i32, i2: i32, i3: i32, i1End: i32, i2End: i32, i3End: i32, downstream: bool) -> Self {
        MSLane_AnyVehicleIterator {
            myLane: lane,
            myI1: i1,
            myI2: i2,
            myI3: i3,
            myI1End: i1End,
            myI2End: i2End,
            myI3End: i3End,
            myDownstream: downstream,
            myDirection: if downstream { 1 } else { -1 },
        }
    }

    /// C++: `bool AnyVehicleIterator::operator==(const AnyVehicleIterator &) const` [const]
    // bool operator== (AnyVehicleIterator const& other) const {
    //             return (myI1 == other.myI1
    //                     && myI2 == other.myI2
    //                     && myI3 == other.myI3
    //                     && myI1End == other.myI1End
    //                     && myI2End == other.myI2End
    //                     && myI3End == other.myI3End);
    //         }
    #[allow(unused_variables)]
    pub fn eq_op(&self, other: &MSLane_AnyVehicleIterator) -> bool {
        self.myI1 == other.myI1
            && self.myI2 == other.myI2
            && self.myI3 == other.myI3
            && self.myI1End == other.myI1End
            && self.myI2End == other.myI2End
            && self.myI3End == other.myI3End
    }

    /// C++: `bool AnyVehicleIterator::operator!=(const AnyVehicleIterator &) const` [const]
    // bool operator!= (AnyVehicleIterator const& other) const {
    //             return !(*this == other);
    //         }
    #[allow(unused_variables)]
    pub fn ne_op(&self, other: &MSLane_AnyVehicleIterator) -> bool {
        !(self.eq_op(other))
    }

    /// C++: `const MSVehicle * AnyVehicleIterator::operator->()`
    // const MSVehicle* operator->() {
    //             return **this;
    //         }
    #[allow(unused_variables)]
    pub fn op_(&mut self) -> Ptr<MSVehicle> {
        self.mul_op()
    }

    /// C++: `const MSVehicle * AnyVehicleIterator::operator*()`
    // const MSVehicle*
    // MSLane::AnyVehicleIterator::operator*() {
    //     if (nextIsMyVehicles()) {
    //         if (myI1 != myI1End) {
    //             return myLane->myVehicles[myI1];
    //         } else if (myI3 != myI3End) {
    //             return myLane->myTmpVehicles[myI3];
    //         } else {
    //             assert(myI2 == myI2End);
    //             return nullptr;
    //         }
    //     } else {
    //         return myLane->myPartialVehicles[myI2];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn mul_op(&mut self) -> Ptr<MSVehicle> {
        if self.next_is_my_vehicles() {
            if self.myI1 != self.myI1End {
                self.myLane.myVehicles[self.myI1 as usize]
            } else if self.myI3 != self.myI3End {
                self.myLane.myTmpVehicles[self.myI3 as usize]
            } else {
                assert!(self.myI2 == self.myI2End);
                Ptr::null()
            }
        } else {
            self.myLane.myPartialVehicles[self.myI2 as usize]
        }
    }

    /// C++: `AnyVehicleIterator & AnyVehicleIterator::operator++()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn op__2(&mut self) -> &mut MSLane_AnyVehicleIterator {
        if self.next_is_my_vehicles() {
            if self.myI1 != self.myI1End {
                self.myI1 += self.myDirection;
            } else if self.myI3 != self.myI3End {
                self.myI3 += self.myDirection;
            }
            // else: already at end
        } else {
            self.myI2 += self.myDirection;
        }
        self
    }

    /// C++: `bool AnyVehicleIterator::nextIsMyVehicles() const` [const, private]
    // bool
    // MSLane::AnyVehicleIterator::nextIsMyVehicles() const {
    // #ifdef DEBUG_ITERATOR
    //     if (DEBUG_COND2(myLane)) std::cout << SIMTIME << "          AnyVehicleIterator::nextIsMyVehicles lane=" << myLane->getID()
    //                                            << " myDownstream=" << myDownstream
    //                                            << " myI1=" << myI1
    //                                            << " myI1End=" << myI1End
    //                                            << " myI2=" << myI2
    //                                            << " myI2End=" << myI2End
    //                                            << " myI3=" << myI3
    //                                            << " myI3End=" << myI3End
    //                                            << "\n";
    // #endif
    //     if (myI1 == myI1End && myI3 == myI3End) {
    //         if (myI2 != myI2End) {
    //             return false;
    //         } else {
    //             return true; // @note. must be caught
    //         }
    //     } else {
    //         if (myI2 == myI2End) {
    //             return true;
    //         } else {
    //             MSVehicle* cand = myI1 == myI1End ? myLane->myTmpVehicles[myI3] : myLane->myVehicles[myI1];
    // #ifdef DEBUG_ITERATOR
    //             if (DEBUG_COND2(myLane)) std::cout << "              "
    //                                                    << " veh1=" << cand->getID()
    //                                                    << " isTmp=" << (myI1 == myI1End)
    //                                                    << " veh2=" << myLane->myPartialVehicles[myI2]->getID()
    //                                                    << " pos1=" << cand->getPositionOnLane(myLane)
    //                                                    << " pos2=" << myLane->myPartialVehicles[myI2]->getPositionOnLane(myLane)
    //                                                    << "\n";
    // #endif
    //             if (cand->getPositionOnLane() < myLane->myPartialVehicles[myI2]->getPositionOnLane(myLane)) {
    //                 return myDownstream;
    //             } else {
    //                 return !myDownstream;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn next_is_my_vehicles(&self) -> bool {
        if self.myI1 == self.myI1End && self.myI3 == self.myI3End {
            if self.myI2 != self.myI2End {
                return false;
            } else {
                return true; // @note. must be caught
            }
        } else {
            if self.myI2 == self.myI2End {
                return true;
            } else {
                let cand = if self.myI1 == self.myI1End { self.myLane.myTmpVehicles[self.myI3 as usize] } else { self.myLane.myVehicles[self.myI1 as usize] };
                if cand.get_position_on_lane() < self.myLane.myPartialVehicles[self.myI2 as usize].get_position_on_lane_2(self.myLane) {
                    return self.myDownstream;
                } else {
                    return !self.myDownstream;
                }
            }
        }
    }

}

/// C++ class `MSLane::IncomingLaneInfo` (microsim/MSLane.h)
pub struct MSLane_IncomingLaneInfo {
    pub lane: Ptr<MSLane>,
    pub length: f64,
    pub viaLink: Ptr<MSLink>,
}

impl MSLane_IncomingLaneInfo {
}

/// Representation of a lane in the micro simulation
/// C++ class `MSLane` (microsim/MSLane.h)
#[repr(C)]
pub struct MSLane {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// Unique numerical ID (set on reading by netload)
    pub myNumericalID: i32,
    /// The shape of the lane
    pub myShape: PositionVector,
    /// the outline of the lane (optional)
    pub myOutlineShape: Ptr<PositionVector>,
    /// The lane index
    pub myIndex: i32,
    /// The lane's vehicles. This container holds all vehicles that have their front (longitudinally) and their center (laterally) on this lane. These are the vehicles that this lane is 'responsibly' for (i.e. when executing movements)
    pub myVehicles: MSLane_VehCont,
    /// The lane's partial vehicles. This container holds all vehicles that are partially on this lane but which are in myVehicles of another lane. Reasons for partial occupancies include the following - the back is still on this lane during regular movement - the vehicle is performing a continuous lane-change maneuver - sub-lane simulation where vehicles can freely move laterally among the lanes of an edge
    pub myPartialVehicles: MSLane_VehCont,
    /// Container for lane-changing vehicles. After completion of lane-change- process, the containers will be swapped with myVehicles.
    pub myTmpVehicles: MSLane_VehCont,
    /// Buffer for vehicles that moved from their previous lane onto this one. Integrated after all vehicles executed their moves
    pub myVehBuffer: Vec<Ptr<MSVehicle>> /* MFXSynchQue<MSVehicle *, std::vector<MSVehicle *>> */,
    /// The vehicles which registered maneuvering into the lane within their current action step. This is currently only relevant for sublane simulation, since continuous lanechanging uses the partial vehicle mechanism.
    pub myManeuverReservations: MSLane_VehCont,
    pub myParkingVehicles: BTreeSet<Ptr<MSBaseVehicle>>,
    /// Lane length [m]
    pub myLength: f64,
    /// Lane width [m]
    pub myWidth: f64,
    /// Lane's vClass specific stop offset [m]. The map is either of length 0, which means no special stopOffset was set, or of length 1, where the key is a bitset representing a subset of the SUMOVehicleClass Enum and the value is the offset in meters.
    pub myLaneStopOffset: StopOffset,
    /// The lane's edge, for routing only.
    pub myEdge: Ptr<MSEdge>,
    /// Lane-wide speed limit [m/s]
    pub myMaxSpeed: f64,
    /// Lane-wide friction coefficient [0..1]
    pub myFrictionCoefficient: f64,
    /// Whether the current speed limit is set by a variable speed sign (VSS), TraCI or a MSCalibrator
    pub mySpeedModified: bool,
    /// The vClass permissions for this lane
    pub myPermissions: SVCPermissions,
    /// The vClass permissions for changing from this lane
    pub myChangeLeft: SVCPermissions,
    pub myChangeRight: SVCPermissions,
    /// The original vClass permissions for this lane (before temporary modifications)
    pub myOriginalPermissions: SVCPermissions,
    /// The vClass speed restrictions for this lane
    pub myRestrictions: Ptr<BTreeMap<SUMOVehicleClass, f64>>,
    /// All direct predecessor lanes
    pub myIncomingLanes: Vec<MSLane_IncomingLaneInfo>,
    /// Lazily cached by get_logical_predecessor_lane(&self), hence Cell (interior mutability).
    pub myLogicalPredecessorLane: std::cell::Cell<Ptr<MSLane>>,
    /// Similar to LogicalPredecessorLane,
    pub myCanonicalPredecessorLane: std::cell::Cell<Ptr<MSLane>>,
    /// Main successor lane,
    pub myCanonicalSuccessorLane: std::cell::Cell<Ptr<MSLane>>,
    /// The current length of all vehicles on this lane, including their minGaps
    pub myBruttoVehicleLengthSum: f64,
    /// The current length of all vehicles on this lane, excluding their minGaps
    pub myNettoVehicleLengthSum: f64,
    /// The length of all vehicles that have left this lane in the current step (this lane, including their minGaps)
    pub myBruttoVehicleLengthSumToRemove: f64,
    /// The length of all vehicles that have left this lane in the current step (this lane, excluding their minGaps)
    pub myNettoVehicleLengthSumToRemove: f64,
    /// Flag to recalculate the occupancy (including minGaps) after a change in minGap
    pub myRecalculateBruttoSum: bool,
    /// The lane's Links to its succeeding lanes and the default right-of-way rule, i.e. blocked or not blocked.
    pub myLinks: Vec<Ptr<MSLink>>,
    /// All direct internal and direct (disregarding internal predecessors) non-internal predecessor lanes of this lane
    pub myApproachingLanes: BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSLane>>>,
    /// leaders on all sublanes as seen by approaching vehicles (cached)
    pub myLeaderInfo: std::cell::RefCell<MSLeaderInfo>,
    /// followers on all sublanes as seen by vehicles on consecutive lanes (cached)
    pub myFollowerInfo: std::cell::RefCell<MSLeaderInfo>,
    /// time step for which myLeaderInfo was last updated
    pub myLeaderInfoTime: std::cell::Cell<SUMOTime>,
    /// time step for which myFollowerInfo was last updated
    pub myFollowerInfoTime: std::cell::Cell<SUMOTime>,
    /// precomputed myShape.length / myLength
    pub myLengthGeometryFactor: f64,
    /// whether this lane is an acceleration lane
    pub myIsRampAccel: bool,
    /// the type of this lane
    pub myLaneType: String,
    /// the combined width of all lanes with lower index on myEdge
    pub myRightSideOnEdge: f64,
    /// the index of the rightmost sublane of this lane on myEdge
    pub myRightmostSublane: i32,
    /// whether a collision check is currently needed
    pub myNeedsCollisionCheck: bool,
    pub myOpposite: Ptr<MSLane>,
    pub myBidiLane: Ptr<MSLane>,
    pub myPermissionChanges: BTreeMap<i64, SVCPermissions>,
    pub myRNGIndex: i32,
    /// This lane's move reminder
    pub myMoveReminders: Vec<Ptr<MSMoveReminder>>,
    pub myStopWatch: Vec<Unmapped /* StopWatch<std::chrono::nanoseconds> */>,
}

impl std::ops::Deref for MSLane {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSLane {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSLane (port as module-level statics, see AGENTS.md):
//   static const long MSLane::CHANGE_PERMISSIONS_PERMANENT;
//   static const long MSLane::CHANGE_PERMISSIONS_GUI;
//   static DictType MSLane::myDict;
//   static std::vector<SumoRNG> MSLane::myRNGs;
//   static CollisionAction MSLane::myCollisionAction;
//   static CollisionAction MSLane::myIntermodalCollisionAction;
//   static bool MSLane::myCheckJunctionCollisions;
//   static double MSLane::myCheckJunctionCollisionMinGap;
//   static SUMOTime MSLane::myCollisionStopTime;
//   static SUMOTime MSLane::myIntermodalCollisionStopTime;
//   static double MSLane::myCollisionMinGapFactor;
//   static bool MSLane::myExtrapolateSubstepDepart;
//   static DepartSpeedDefinition MSLane::myDefaultDepartSpeedDefinition;
//   static double MSLane::myDefaultDepartSpeed;

/// C++: `MSGlobals::gLateralResolution`
fn g_lateral_resolution() -> f64 {
    crate::microsim::ms_globals::lateral_resolution()
}

/// C++: `MSPModel::SAFETY_GAP`
const MSP_MODEL_SAFETY_GAP: f64 = 1.0;

/// C++: `MSGlobals::gEmergencyInsert`
fn g_emergency_insert() -> bool {
    crate::microsim::ms_globals::emergency_insert()
}

/// C++: `MSGlobals::gCheckRoutes` (set from the inverse of `ignore-route-errors`)
fn g_check_routes() -> bool {
    !utils::options::options_cont::get_bool("ignore-route-errors")
}

/// C++: `RandHelper::rand(double max)` with the default RNG
fn rand_max_default(max: f64) -> f64 {
    utils::common::rand_helper::with_default(|r| r.rand_max(max))
}

/// C++: `MSLane::CHANGE_PERMISSIONS_PERMANENT`
pub const MSLane_CHANGE_PERMISSIONS_PERMANENT: i64 = 0;
/// C++: `MSLane::CHANGE_PERMISSIONS_GUI`
pub const MSLane_CHANGE_PERMISSIONS_GUI: i64 = 1;

thread_local! {
    /// C++: `MSLane::myDict`
    static MY_DICT: std::cell::RefCell<MSLane_DictType> = std::cell::RefCell::new(BTreeMap::new());
    /// C++: `MSLane::myRNGs`
    static MY_RNGS: std::cell::RefCell<Vec<SumoRNG>> = std::cell::RefCell::new(Vec::new());
    /// C++: `MSLane::myExtrapolateSubstepDepart`
    static MY_EXTRAPOLATE_SUBSTEP_DEPART: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static MY_COLLISION_ACTION: std::cell::Cell<MSLane_CollisionAction> = std::cell::Cell::new(MSLane_CollisionAction::COLLISION_ACTION_TELEPORT);
    static MY_INTERMODAL_COLLISION_ACTION: std::cell::Cell<MSLane_CollisionAction> = std::cell::Cell::new(MSLane_CollisionAction::COLLISION_ACTION_WARN);
    static MY_CHECK_JUNCTION_COLLISIONS: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static MY_CHECK_JUNCTION_COLLISION_MIN_GAP: std::cell::Cell<f64> = std::cell::Cell::new(0.);
    /// C++: `MSLane::myCollisionStopTime`
    static MY_COLLISION_STOP_TIME: std::cell::Cell<SUMOTime> = std::cell::Cell::new(0);
    /// C++: `MSLane::myIntermodalCollisionStopTime`
    static MY_INTERMODAL_COLLISION_STOP_TIME: std::cell::Cell<SUMOTime> = std::cell::Cell::new(0);
    /// C++: `MSLane::myCollisionMinGapFactor`
    static MY_COLLISION_MIN_GAP_FACTOR: std::cell::Cell<f64> = std::cell::Cell::new(1.0);
}

// C++: `MSLane::myDefaultDepartSpeedDefinition` / `MSLane::myDefaultDepartSpeed` (handed out as T&, so static mut like MSEdge)
static mut MY_DEFAULT_DEPART_SPEED_DEFINITION: DepartSpeedDefinition = DepartSpeedDefinition::DEFAULT;
static mut MY_DEFAULT_DEPART_SPEED: f64 = 0.;

// C++ MSNet::STAGE_LANECHANGE (private const in ms_net.rs)
const STAGE_LANECHANGE: &str = "laneChange";
// C++ SUMOVehicleParameter::Stop flags (utils/vehicle/SUMOVehicleParameter.h)
const STOP_START_SET: i32 = 1;
const STOP_END_SET: i32 = 2;
const STOP_DURATION_SET: i32 = 2 << 1;

/// C++ default-constructed `SUMOVehicleParameter::Stop` (the Rust struct has no Default impl yet)
fn default_stop() -> SUMOVehicleParameter_Stop {
    SUMOVehicleParameter_Stop {
        base: Parameterised::default(),
        edge: String::new(),
        lane: String::new(),
        busstop: String::new(),
        containerstop: String::new(),
        parkingarea: String::new(),
        chargingStation: String::new(),
        overheadWireSegment: String::new(),
        startPos: 0.,
        endPos: 0.,
        arrival: -1,
        duration: -1,
        durationDist: (),
        until: -1,
        extension: -1,
        waitUntil: -1,
        triggered: false,
        containerTriggered: false,
        joinTriggered: false,
        parking: ParkingType::ONROAD,
        awaitedPersons: BTreeSet::new(),
        permitted: BTreeSet::new(),
        awaitedContainers: BTreeSet::new(),
        friendlyPos: false,
        actType: String::new(),
        tripId: String::new(),
        line: String::new(),
        split: String::new(),
        join: String::new(),
        speed: 0.,
        posLat: utils::common::std_defs::INVALID_DOUBLE,
        onDemand: false,
        priority: -1.,
        jump: -1,
        jumpUntil: -1,
        started: -1,
        ended: -1,
        accessPos: Vec::new(),
        index: 0,
        routeIndex: 0,
        parametersSet: 0,
        collision: false,
        breakDown: false,
    }
}

/// C++ static `MSLane::myCheckJunctionCollisionMinGap` (set by initCollisionOptions)
fn check_junction_collision_min_gap() -> f64 {
    MY_CHECK_JUNCTION_COLLISION_MIN_GAP.with(|c| c.get())
}

impl MSLane {
    /// C++: `void MSLane::MSLane(const std::string &, double, double, double, MSEdge *const, int, const PositionVector &, double, SVCPermissions, SVCPermissions, SVCPermissions, int, bool, const std::string &, const PositionVector &)`
    /// Constructor
    // MSLane::MSLane(const std::string& id, double maxSpeed, double friction, double length, MSEdge* const edge,
    //                int numericalID, const PositionVector& shape, double width,
    //                SVCPermissions permissions,
    //                SVCPermissions changeLeft, SVCPermissions changeRight,
    //                int index, bool isRampAccel,
    //                const std::string& type,
    //                const PositionVector& outlineShape) :
    //     Named(id),
    //     myNumericalID(numericalID), myShape(shape), myIndex(index),
    //     myVehicles(), myLength(length), myWidth(width),
    //     myEdge(edge), myMaxSpeed(maxSpeed),
    //     myFrictionCoefficient(friction),
    //     mySpeedModified(false),
    //     myPermissions(permissions),
    //     myChangeLeft(changeLeft),
    //     myChangeRight(changeRight),
    //     myOriginalPermissions(permissions),
    //     myLogicalPredecessorLane(nullptr),
    //     myCanonicalPredecessorLane(nullptr),
    //     myCanonicalSuccessorLane(nullptr),
    //     myBruttoVehicleLengthSum(0), myNettoVehicleLengthSum(0),
    //     myBruttoVehicleLengthSumToRemove(0), myNettoVehicleLengthSumToRemove(0),
    //     myRecalculateBruttoSum(false),
    //     myLeaderInfo(width, nullptr, 0.),
    //     myFollowerInfo(width, nullptr, 0.),
    //     myLeaderInfoTime(SUMOTime_MIN),
    //     myFollowerInfoTime(SUMOTime_MIN),
    //     myLengthGeometryFactor(MAX2(POSITION_EPS, myShape.length()) / myLength), // factor should not be 0
    //     myIsRampAccel(isRampAccel),
    //     myLaneType(type),
    //     myRightSideOnEdge(0), // initialized in MSEdge::initialize
    //     myRightmostSublane(0),
    //     myNeedsCollisionCheck(false),
    //     myOpposite(nullptr),
    //     myBidiLane(nullptr),
    // #ifdef HAVE_FOX
    //     mySimulationTask(*this, 0),
    // #endif
    //     myStopWatch(3) {
    //     // initialized in MSEdge::initialize
    //     initRestrictions();// may be reloaded again from initialized in MSEdge::closeBuilding
    //     assert(myRNGs.size() > 0);
    //     myRNGIndex = numericalID % myRNGs.size();
    //     if (outlineShape.size() > 0) {
    //         myOutlineShape = new PositionVector(outlineShape);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, maxSpeed: f64, friction: f64, length: f64, edge: Ptr<MSEdge>, numericalID: i32, shape: &PositionVector, width: f64, permissions: SVCPermissions, changeLeft: SVCPermissions, changeRight: SVCPermissions, index: i32, isRampAccel: bool, type_: &str, outlineShape: &PositionVector) -> Self {
        // shortcut: myStopWatch holds placeholder entries until StopWatch is ported; mySimulationTask is HAVE_FOX-only and skipped
        let mut lane = MSLane {
            base: Named::new(id),
            base2: Parameterised::new(),
            myNumericalID: numericalID,
            myShape: shape.clone(),
            myOutlineShape: Ptr::null(),
            myIndex: index,
            myVehicles: Vec::new(),
            myPartialVehicles: Vec::new(),
            myTmpVehicles: Vec::new(),
            myVehBuffer: Vec::new(),
            myManeuverReservations: Vec::new(),
            myParkingVehicles: BTreeSet::new(),
            myLength: length,
            myWidth: width,
            myLaneStopOffset: StopOffset::default(),
            myEdge: edge,
            myMaxSpeed: maxSpeed,
            myFrictionCoefficient: friction,
            mySpeedModified: false,
            myPermissions: permissions,
            myChangeLeft: changeLeft,
            myChangeRight: changeRight,
            myOriginalPermissions: permissions,
            myRestrictions: Ptr::null(),
            myIncomingLanes: Vec::new(),
            myLogicalPredecessorLane: std::cell::Cell::new(Ptr::null()),
            myCanonicalPredecessorLane: std::cell::Cell::new(Ptr::null()),
            myCanonicalSuccessorLane: std::cell::Cell::new(Ptr::null()),
            myBruttoVehicleLengthSum: 0.,
            myNettoVehicleLengthSum: 0.,
            myBruttoVehicleLengthSumToRemove: 0.,
            myNettoVehicleLengthSumToRemove: 0.,
            myRecalculateBruttoSum: false,
            myLinks: Vec::new(),
            myApproachingLanes: BTreeMap::new(),
            myLeaderInfo: std::cell::RefCell::new(MSLeaderInfo::new(width, Ptr::null(), 0.)),
            myFollowerInfo: std::cell::RefCell::new(MSLeaderInfo::new(width, Ptr::null(), 0.)),
            myLeaderInfoTime: std::cell::Cell::new(SUMOTIME_MIN),
            myFollowerInfoTime: std::cell::Cell::new(SUMOTIME_MIN),
            myLengthGeometryFactor: utils::geom::POSITION_EPS.max(shape.length()) / length,
            myIsRampAccel: isRampAccel,
            myLaneType: type_.to_string(),
            myRightSideOnEdge: 0.,
            myRightmostSublane: 0,
            myNeedsCollisionCheck: false,
            myOpposite: Ptr::null(),
            myBidiLane: Ptr::null(),
            myPermissionChanges: BTreeMap::new(),
            myRNGIndex: 0,
            myMoveReminders: Vec::new(),
            myStopWatch: vec![(); 3],
        };
        // initialized in MSEdge::initialize
        lane.init_restrictions(); // may be reloaded again from initialized in MSEdge::closeBuilding
        let nRNGs = MY_RNGS.with(|r| r.borrow().len());
        assert!(nRNGs > 0);
        lane.myRNGIndex = (numericalID as usize % nRNGs) as i32;
        if outlineShape.len() > 0 {
            lane.myOutlineShape = Ptr::from_raw(Box::into_raw(Box::new(outlineShape.clone())));
        }
        lane
    }

    /// C++: `void MSLane::~MSLane() noexcept` [virtual]
    /// Destructor
    // MSLane::~MSLane() {
    //     for (MSLink* const l : myLinks) {
    //         delete l;
    //     }
    //     delete myOutlineShape;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: links are not owned by MSLane in the Rust port yet (no delete l), only the outline shape is freed
        self.myLinks.clear();
        if !self.myOutlineShape.is_null() {
            unsafe { drop(Box::from_raw(self.myOutlineShape.as_raw())) };
            self.myOutlineShape = Ptr::null();
        }
    }

    /// C++: `int MSLane::getThreadIndex() const` [const]
    /// returns the associated thread index
    // inline int getThreadIndex() const {
    //         return myRNGIndex % MSGlobals::gNumSimThreads;
    //     }
    #[allow(unused_variables)]
    pub fn get_thread_index(&self) -> i32 {
        self.myRNGIndex % crate::microsim::ms_globals::num_sim_threads()
    }

    /// C++: `int MSLane::getRNGIndex() const` [const]
    /// returns the associated RNG index
    // inline int getRNGIndex() const {
    //         return myRNGIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        self.myRNGIndex
    }

    /// C++: `SumoRNG * MSLane::getRNG() const` [const]
    /// return the associated RNG
    // SumoRNG* getRNG() const {
    //         return &myRNGs[myRNGIndex];
    //     }
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        let idx = self.myRNGIndex as usize;
        Ptr::from_raw(MY_RNGS.with(|r| &mut r.borrow_mut()[idx] as *mut SumoRNG))
    }

    /// C++: `int MSLane::getNumRNGs()` [static]
    /// return the number of RNGs
    // static int getNumRNGs() {
    //         return (int)myRNGs.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_rn_gs() -> i32 {
        MY_RNGS.with(|r| r.borrow().len() as i32)
    }

    /// C++: `void MSLane::saveRNGStates(OutputDevice &)` [static]
    /// save random number generator states to the given output device
    // void
    // MSLane::saveRNGStates(OutputDevice& out) {
    //     for (int i = 0; i < getNumRNGs(); i++) {
    //         out.openTag(SUMO_TAG_RNGLANE);
    //         out.writeAttr(SUMO_ATTR_INDEX, i);
    //         out.writeAttr(SUMO_ATTR_STATE, RandHelper::saveState(&myRNGs[i]));
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_rng_states(out: &mut OutputDevice) {
        for i in 0..MSLane::get_num_rn_gs() {
            out.open_tag_id(SUMO_TAG_RNGLANE);
            out.write_attr(SUMO_ATTR_INDEX, &i);
            let state = MY_RNGS.with(|r| r.borrow()[i as usize].save_state());
            out.write_attr(SUMO_ATTR_STATE, &state);
            out.close_tag("");
        }
    }

    /// C++: `void MSLane::loadRNGState(int, const std::string &)` [static]
    /// load random number generator state for the given rng index
    // void
    // MSLane::loadRNGState(int index, const std::string& state) {
    //     if (index >= getNumRNGs()) {
    //         throw ProcessError(TLF("State was saved with more than % threads. Change the number of threads or do not load RNG state", toString(getNumRNGs())));
    //     }
    //     RandHelper::loadState(state, &myRNGs[index]);
    // }
    #[allow(unused_variables)]
    pub fn load_rng_state(index: i32, state: &str) {
        if index >= MSLane::get_num_rn_gs() {
            panic!("{}", ProcessError::Process(utils::tlf!("State was saved with more than % threads. Change the number of threads or do not load RNG state", MSLane::get_num_rn_gs().to_string())));
        }
        MY_RNGS.with(|r| r.borrow_mut()[index as usize].load_state(state));
    }

    /// C++: `void MSLane::addLink(MSLink *)`
    /// Delayed initialization
    // void
    // MSLane::addLink(MSLink* link) {
    //     myLinks.push_back(link);
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>) {
        self.myLinks.push(link);
    }

    /// C++: `void MSLane::setOpposite(MSLane *)`
    /// Adds a neighbor to this lane
    // void
    // MSLane::setOpposite(MSLane* oppositeLane) {
    //     myOpposite = oppositeLane;
    //     if (myOpposite != nullptr && getLength() > myOpposite->getLength()) {
    //         WRITE_WARNINGF(TL("Unequal lengths of neigh lane '%' and lane '%' (% != %)."), getID(), myOpposite->getID(), getLength(), myOpposite->getLength());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_opposite(&mut self, oppositeLane: Ptr<MSLane>) {
        self.myOpposite = oppositeLane;
        if !self.myOpposite.is_null() && self.get_length() > self.myOpposite.get_length() {
            utils::write_warningf!("Unequal lengths of neigh lane '%' and lane '%' (% != %).", self.get_id(), self.myOpposite.get_id(), self.get_length(), self.myOpposite.get_length());
        }
    }

    /// C++: `void MSLane::setBidiLane(MSLane *)`
    /// Adds the (overlapping) reverse direction lane to this lane
    // void
    // MSLane::setBidiLane(MSLane* bidiLane) {
    //     myBidiLane = bidiLane;
    //     if (myBidiLane != nullptr && getLength() > myBidiLane->getLength()) {
    //         if (isNormal() || MSGlobals::gUsingInternalLanes) {
    //             WRITE_WARNINGF(TL("Unequal lengths of bidi lane '%' and lane '%' (% != %)."), getID(), myBidiLane->getID(), getLength(), myBidiLane->getLength());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_bidi_lane(&mut self, bidyLane: Ptr<MSLane>) {
        self.myBidiLane = bidyLane;
        if !self.myBidiLane.is_null() && self.get_length() > self.myBidiLane.get_length() {
            if self.is_normal() || crate::microsim::ms_globals::using_internal_lanes() {
                utils::write_warningf!("Unequal lengths of bidi lane '%' and lane '%' (% != %).", self.get_id(), self.myBidiLane.get_id(), self.get_length(), self.myBidiLane.get_length());
            }
        }
    }

    /// C++: `void MSLane::addSecondaryShape(const PositionVector &)` [virtual]
    /// @{
    // virtual void addSecondaryShape(const PositionVector& /*shape*/) {}
    #[allow(unused_variables)]
    pub fn add_secondary_shape(&mut self, arg0: &PositionVector) {
        // virtual no-op in the base class; subclasses override

    }

    /// C++: `double MSLane::getLengthGeometryFactor(bool) const` [const, virtual]
    // virtual double getLengthGeometryFactor(bool /*secondaryShape*/) const {
    //         return myLengthGeometryFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self, arg0: bool) -> f64 {
        self.myLengthGeometryFactor
    }

    /// C++: `const PositionVector & MSLane::getShape(bool) const` [const, virtual]
    // virtual const PositionVector& getShape(bool /*secondaryShape*/) const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape(&self, arg0: bool) -> &PositionVector {
        &self.myShape
    }

    /// C++: `void MSLane::updateMesoGUISegments()` [virtual]
    /// @}
    // virtual void updateMesoGUISegments() {}
    #[allow(unused_variables)]
    pub fn update_meso_gui_segments(&mut self) {
        // virtual no-op in the base class; subclasses override
    }

    /// C++: `void MSLane::addMoveReminder(MSMoveReminder *, bool)` [virtual]
    /// Add a move-reminder to move-reminder container
    // void
    // MSLane::addMoveReminder(MSMoveReminder* rem, bool addToVehicles) {
    //     myMoveReminders.push_back(rem);
    //     if (addToVehicles) {
    //         for (MSVehicle* const veh : myVehicles) {
    //             veh->addReminder(rem);
    //         }
    //     }
    //     // XXX: Here, the partial occupators are ignored!? Refs. #3255
    // }
    #[allow(unused_variables)]
    pub fn add_move_reminder(&mut self, rem: Ptr<MSMoveReminder>, addToVehicles: bool) {
        self.myMoveReminders.push(rem);
        if addToVehicles {
            for veh in self.myVehicles.clone() {
                veh.m().add_reminder(rem, 0.);
            }
        }
        // XXX: Here, the partial occupators are ignored!? Refs. #3255
    }

    /// C++: `void MSLane::removeMoveReminder(MSMoveReminder *)` [virtual]
    /// Remove a move-reminder from move-reminder container
    // void
    // MSLane::removeMoveReminder(MSMoveReminder* rem) {
    //     auto it = std::find(myMoveReminders.begin(), myMoveReminders.end(), rem);
    //     if (it != myMoveReminders.end()) {
    //         myMoveReminders.erase(it);
    //         for (MSVehicle* const veh : myVehicles) {
    //             veh->removeReminder(rem);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_move_reminder(&mut self, rem: Ptr<MSMoveReminder>) {
        if let Some(it) = self.myMoveReminders.iter().position(|r| *r == rem) {
            self.myMoveReminders.remove(it);
            for veh in self.myVehicles.clone() {
                veh.m().remove_reminder(rem);
            }
        }
    }

    /// C++: `const std::vector<MSMoveReminder *> & MSLane::getMoveReminders() const` [const]
    /// Return the list of this lane's move reminders
    // inline const std::vector< MSMoveReminder* >& getMoveReminders() const {
    //         return myMoveReminders;
    //     }
    #[allow(unused_variables)]
    pub fn get_move_reminders(&self) -> &Vec<Ptr<MSMoveReminder>> {
        &self.myMoveReminders
    }

    /// C++: `bool MSLane::insertVehicle(MSVehicle &)`
    /// Tries to insert the given vehicle
    // bool
    // MSLane::insertVehicle(MSVehicle& veh) {
    //     double pos = 0;
    //     bool patchSpeed = true; // whether the speed shall be adapted to infrastructure/traffic in front
    //     const SUMOVehicleParameter& pars = veh.getParameter();
    //     double speed = getDepartSpeed(veh, patchSpeed);
    //     double posLat = getDepartPosLat(veh);
    //
    //     // determine the position
    //     switch (pars.departPosProcedure) {
    //         case DepartPosDefinition::GIVEN:
    //             pos = pars.departPos;
    //             if (pos < 0.) {
    //                 pos += myLength;
    //             }
    //             break;
    //         case DepartPosDefinition::RANDOM:
    //             pos = roundDecimal(RandHelper::rand(getLength()), gPrecisionRandom);
    //             break;
    //         case DepartPosDefinition::RANDOM_FREE: {
    //             for (int i = 0; i < 10; i++) {
    //                 // we will try some random positions ...
    //                 pos = RandHelper::rand(getLength());
    //                 posLat = getDepartPosLat(veh); // could be random as well
    //                 if (isInsertionSuccess(&veh, speed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED)) {
    //                     MSNet::getInstance()->getInsertionControl().retractDescheduleDeparture(&veh);
    //                     return true;
    //                 }
    //             }
    //             // ... and if that doesn't work, we put the vehicle to the free position
    //             bool success = freeInsertion(veh, speed, posLat);
    //             if (success) {
    //                 MSNet::getInstance()->getInsertionControl().retractDescheduleDeparture(&veh);
    //             }
    //             return success;
    //         }
    //         case DepartPosDefinition::FREE:
    //             return freeInsertion(veh, speed, posLat);
    //         case DepartPosDefinition::LAST:
    //             return lastInsertion(veh, speed, posLat, patchSpeed);
    //         case DepartPosDefinition::STOP:
    //             if (veh.hasStops() && veh.getNextStop().edge == veh.getCurrentRouteEdge()) {
    //                 // getLastFreePos of stopping place could return negative position to avoid blocking the stop
    //                 pos = MAX2(0.0, veh.getNextStop().getEndPos(veh));
    //                 break;
    //             }
    //             FALLTHROUGH;
    //         case DepartPosDefinition::BASE:
    //         case DepartPosDefinition::DEFAULT:
    //         case DepartPosDefinition::SPLIT_FRONT:
    //         default:
    //             if (pars.departProcedure == DepartDefinition::SPLIT) {
    //                 pos = getLength();
    //                 // find the vehicle from which we are splitting off (should only be a single lane to check)
    //                 AnyVehicleIterator end = anyVehiclesEnd();
    //                 for (AnyVehicleIterator it = anyVehiclesBegin(); it != end; ++it) {
    //                     const MSVehicle* cand = *it;
    //                     if (cand->isStopped() && cand->getNextStopParameter()->split == veh.getID()) {
    //                         if (pars.departPosProcedure == DepartPosDefinition::SPLIT_FRONT) {
    //                             pos = cand->getPositionOnLane() + cand->getVehicleType().getMinGap() + veh.getLength();
    //                         } else {
    //                             pos = cand->getBackPositionOnLane() - veh.getVehicleType().getMinGap();
    //                         }
    //                         break;
    //                     }
    //                 }
    //             } else {
    //                 pos = veh.basePos(myEdge);
    //             }
    //             break;
    //     }
    //     // determine the lateral position for special cases
    //     if (MSGlobals::gLateralResolution > 0) {
    //         switch (pars.departPosLatProcedure) {
    //             case DepartPosLatDefinition::RANDOM_FREE: {
    //                 for (int i = 0; i < 10; i++) {
    //                     // we will try some random positions ...
    //                     posLat = RandHelper::rand(getWidth()) - getWidth() * 0.5;
    //                     if (isInsertionSuccess(&veh, speed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED)) {
    //                         return true;
    //                     }
    //                 }
    //                 FALLTHROUGH;
    //             }
    //             // no break! continue with DepartPosLatDefinition::FREE
    //             case DepartPosLatDefinition::FREE: {
    //                 // systematically test all positions until a free lateral position is found
    //                 double posLatMin = -getWidth() * 0.5 + veh.getVehicleType().getWidth() * 0.5;
    //                 double posLatMax = getWidth() * 0.5 - veh.getVehicleType().getWidth() * 0.5;
    //                 for (posLat = posLatMin; posLat < posLatMax; posLat += MSGlobals::gLateralResolution) {
    //                     if (isInsertionSuccess(&veh, speed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED)) {
    //                         return true;
    //                     }
    //                 }
    //                 return false;
    //             }
    //             default:
    //                 break;
    //         }
    //     }
    //     // try to insert
    //     const bool success = isInsertionSuccess(&veh, speed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED);
    // #ifdef DEBUG_EXTRAPOLATE_DEPARTPOS
    //     if (DEBUG_COND2(&veh)) {
    //         std::cout << SIMTIME << " veh=" << veh.getID() << " success=" << success << " extrapolate=" << myExtrapolateSubstepDepart << " delay=" << veh.getDepartDelay() << " speed=" << speed << "\n";
    //     }
    // #endif
    //     if (success && myExtrapolateSubstepDepart && veh.getDepartDelay() > 0) {
    //         SUMOTime relevantDelay = MIN2(DELTA_T, veh.getDepartDelay());
    //         // try to compensate sub-step depart delay by moving the vehicle forward
    //         speed = veh.getSpeed(); // may have been adapted in isInsertionSuccess
    //         double dist = speed * STEPS2TIME(relevantDelay);
    //         std::pair<MSVehicle* const, double> leaderInfo = getLeader(&veh, pos, veh.getBestLanesContinuation());
    //         if (leaderInfo.first != nullptr) {
    //             MSVehicle* leader = leaderInfo.first;
    //             const double frontGapNeeded = veh.getCarFollowModel().getSecureGap(&veh, leader, speed, leader->getSpeed(),
    //                                           leader->getCarFollowModel().getMaxDecel());
    //             dist = MIN2(dist, leaderInfo.second - frontGapNeeded);
    //         }
    //         if (dist > 0) {
    //             veh.executeFractionalMove(dist);
    //         }
    //     }
    //     return success;
    // }
    #[allow(unused_variables)]
    pub fn insert_vehicle(&mut self, v: &mut MSVehicle) -> bool {
        let mut pos: f64 = 0.;
        let mut patchSpeed: bool = true; // whether the speed shall be adapted to infrastructure/traffic in front
        // the C++ holds a reference into the vehicle; a raw pointer keeps the same aliasing
        let pars: &SUMOVehicleParameter = unsafe { &*(v.get_parameter() as *const SUMOVehicleParameter) };
        let mut speed = self.get_depart_speed(v, &mut patchSpeed);
        let mut posLat = self.get_depart_pos_lat(v);

        // determine the position
        match pars.departPosProcedure {
            DepartPosDefinition::GIVEN => {
                pos = pars.departPos;
                if pos < 0. {
                    pos += self.myLength;
                }
            }
            DepartPosDefinition::RANDOM => {
                pos = utils::common::std_defs::round_decimal(rand_max_default(self.get_length()), utils::common::std_defs::g_precision_random());
            }
            DepartPosDefinition::RANDOM_FREE => {
                for i in 0..10 {
                    // we will try some random positions ...
                    pos = rand_max_default(self.get_length());
                    posLat = self.get_depart_pos_lat(v); // could be random as well
                    if self.is_insertion_success(Ptr::from_mut(v), speed, pos, posLat, patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
                        MSNet::get_instance().m().get_insertion_control().retract_deschedule_departure(Ptr::from_mut(v));
                        return true;
                    }
                }
                // ... and if that doesn't work, we put the vehicle to the free position
                let success = self.free_insertion(v, speed, posLat, MSMoveReminder_Notification::NOTIFICATION_DEPARTED);
                if success {
                    MSNet::get_instance().m().get_insertion_control().retract_deschedule_departure(Ptr::from_mut(v));
                }
                return success;
            }
            DepartPosDefinition::FREE => {
                return self.free_insertion(v, speed, posLat, MSMoveReminder_Notification::NOTIFICATION_DEPARTED);
            }
            DepartPosDefinition::LAST => {
                return self.last_insertion(v, speed, posLat, patchSpeed);
            }
            DepartPosDefinition::STOP if v.has_stops() && v.get_next_stop().edge == *v.get_current_route_edge() => {
                // getLastFreePos of stopping place could return negative position to avoid blocking the stop
                pos = 0.0f64.max(v.get_next_stop().get_end_pos(v));
            }
            _ => {
                // BASE, DEFAULT, SPLIT_FRONT, and the STOP fallthrough
                if pars.departProcedure == DepartDefinition::SPLIT {
                    pos = self.get_length();
                    // find the vehicle from which we are splitting off (should only be a single lane to check)
                    let end = self.any_vehicles_end();
                    let mut it = self.any_vehicles_begin();
                    while it.ne_op(&end) {
                        let cand = it.mul_op();
                        if cand.is_stopped() && cand.get_next_stop_parameter().split == v.get_id() {
                            if pars.departPosProcedure == DepartPosDefinition::SPLIT_FRONT {
                                pos = cand.get_position_on_lane() + cand.get_vehicle_type().get_min_gap() + v.get_length();
                            } else {
                                pos = cand.get_back_position_on_lane(Ptr::from_mut(self)) - v.get_vehicle_type().get_min_gap();
                            }
                            break;
                        }
                        it.op__2();
                    }
                } else {
                    pos = v.base_pos(self.myEdge);
                }
            }
        }
        // determine the lateral position for special cases
        if g_lateral_resolution() > 0. {
            match pars.departPosLatProcedure {
                DepartPosLatDefinition::RANDOM_FREE => {
                    for i in 0..10 {
                        // we will try some random positions ...
                        posLat = rand_max_default(self.get_width()) - self.get_width() * 0.5;
                        if self.is_insertion_success(Ptr::from_mut(v), speed, pos, posLat, patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
                            return true;
                        }
                    }
                    // no break! continue with DepartPosLatDefinition::FREE
                    return self.insert_vehicle_lat_free(v, speed, pos, &mut posLat, &mut patchSpeed);
                }
                DepartPosLatDefinition::FREE => {
                    return self.insert_vehicle_lat_free(v, speed, pos, &mut posLat, &mut patchSpeed);
                }
                _ => {}
            }
        }
        // try to insert
        let success = self.is_insertion_success(Ptr::from_mut(v), speed, pos, posLat, patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED);
        if success && MY_EXTRAPOLATE_SUBSTEP_DEPART.with(|c| c.get()) && v.get_depart_delay() > 0 {
            let relevantDelay = utils::common::time::delta_t().min(v.get_depart_delay());
            // try to compensate sub-step depart delay by moving the vehicle forward
            speed = v.get_speed(); // may have been adapted in isInsertionSuccess
            let mut dist = speed * utils::common::time::steps2time(relevantDelay);
            let (leader, leaderGap) = self.get_leader(Ptr::from_mut(v), pos, v.get_best_lanes_continuation(), -1., false);
            if !leader.is_null() {
                let vp = Ptr::from_mut(v);
                let frontGapNeeded = v.get_car_follow_model().get_secure_gap(vp, leader, speed, leader.get_speed(), leader.get_car_follow_model().get_max_decel());
                dist = dist.min(leaderGap - frontGapNeeded);
            }
            if dist > 0. {
                v.execute_fractional_move(dist);
            }
        }
        success
    }

    /// Helper for insertVehicle: the FREE lateral search (C++ `case DepartPosLatDefinition::FREE`), shared by the fallthrough from RANDOM_FREE.
    fn insert_vehicle_lat_free(&mut self, v: &mut MSVehicle, speed: f64, pos: f64, posLat: &mut f64, patchSpeed: &mut bool) -> bool {
        // systematically test all positions until a free lateral position is found
        let posLatMin = -self.get_width() * 0.5 + v.get_vehicle_type().get_width() * 0.5;
        let posLatMax = self.get_width() * 0.5 - v.get_vehicle_type().get_width() * 0.5;
        *posLat = posLatMin;
        while *posLat < posLatMax {
            if self.is_insertion_success(Ptr::from_mut(v), speed, pos, *posLat, *patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED) {
                return true;
            }
            *posLat += g_lateral_resolution();
        }
        false
    }

    /// C++: `bool MSLane::isInsertionSuccess(MSVehicle *, double, double, double, bool, MSMoveReminder::Notification)`
    /// Tries to insert the given vehicle with the given state (speed and pos)
    // bool
    // MSLane::isInsertionSuccess(MSVehicle* aVehicle,
    //                            double speed, double pos, double posLat, bool patchSpeed,
    //                            MSMoveReminder::Notification notification) {
    //     int insertionChecks = aVehicle->getInsertionChecks();
    //     if (pos < 0 || pos > myLength) {
    //         // we may not start there
    //         WRITE_WARNINGF(TL("Invalid departPos % given for vehicle '%', time=%. Inserting at lane end instead."),
    //                        pos, aVehicle->getID(), time2string(SIMSTEP));
    //         pos = myLength;
    //     }
    //
    // #ifdef DEBUG_INSERTION
    //     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //         std::cout << "\nIS_INSERTION_SUCCESS\n"
    //                   << SIMTIME  << " lane=" << getID()
    //                   << " veh '" << aVehicle->getID()
    //                   << " bestLanes=" << toString(aVehicle->getBestLanesContinuation(this))
    //                   << " pos=" << pos
    //                   << " speed=" << speed
    //                   << " patchSpeed=" << patchSpeed
    //                   << "'\n";
    //     }
    // #endif
    //
    //     aVehicle->setTentativeLaneAndPosition(this, pos, posLat);
    //     aVehicle->updateBestLanes(false, this);
    //     const MSCFModel& cfModel = aVehicle->getCarFollowModel();
    //     const std::vector<MSLane*>& bestLaneConts = aVehicle->getBestLanesContinuation(this);
    //     std::vector<MSLane*>::const_iterator ri = bestLaneConts.begin();
    //     double seen = getLength() - pos; // == distance from insertion position until the end of the currentLane
    //     double dist = cfModel.brakeGap(speed) + aVehicle->getVehicleType().getMinGap();
    //     const bool isRail = aVehicle->isRail();
    //     if (isRail && insertionChecks != (int)InsertionCheck::NONE
    //             && aVehicle->getParameter().departProcedure != DepartDefinition::SPLIT
    //             && MSRailSignalControl::isSignalized(aVehicle->getVClass())
    //             && isRailwayOrShared(myPermissions)) {
    //         const MSDriveWay* dw = MSDriveWay::getDepartureDriveway(aVehicle);
    //         MSEdgeVector occupied;
    // #ifdef DEBUG_INSERTION
    //         gDebugFlag4 = DEBUG_COND2(aVehicle) || DEBUG_COND;
    // #endif
    //         if (dw->foeDriveWayOccupied(false, aVehicle, occupied)) {
    //             setParameter("insertionBlocked:" + aVehicle->getID(), dw->getID());
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << "   foe of driveway " + dw->getID() + " has occupied edges " + toString(occupied) << "\n";
    //             }
    //             gDebugFlag4 = false;
    // #endif
    //             return false;
    //         }
    // #ifdef DEBUG_INSERTION
    //         gDebugFlag4 = false;
    // #endif
    //     }
    //     if (getBidiLane() != nullptr && isRail) {
    //         // do not insert if the bidirectional edge is occupied
    //         if (getBidiLane()->getVehicleNumberWithPartials() > 0 && (insertionChecks & (int)InsertionCheck::BIDI) != 0) {
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << "   bidi-lane occupied\n";
    //             }
    // #endif
    //             return false;
    //         }
    //         // do not insert the back of the train would be put onto an occupied bidi-lane
    //         double backLength = aVehicle->getLength() - pos;
    //         if (backLength > 0 && (insertionChecks & (int)InsertionCheck::BIDI) != 0) {
    //             MSLane* pred = getLogicalPredecessorLane();
    //             MSLane* bidi = pred == nullptr ? nullptr : pred->getBidiLane();
    //             while (backLength > 0 && bidi != nullptr) {
    //                 if (bidi->getVehicleNumberWithPartials() > 0) {
    // #ifdef DEBUG_INSERTION
    //                     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                         std::cout << "   bidi-lane furtherLanes occupied\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //                 backLength -= bidi->getLength();
    //                 pred = pred->getLogicalPredecessorLane();
    //                 bidi = pred == nullptr ? nullptr : pred->getBidiLane();
    //             }
    //         }
    //     }
    //     MSLink* firstRailSignal = nullptr;
    //     double firstRailSignalDist = -1;
    //     // whether speed may be patched for unavoidable reasons (stops, speedLimits, ...)
    //     const bool patchSpeedSpecial = patchSpeed || aVehicle->getParameter().departSpeedProcedure != DepartSpeedDefinition::GIVEN;
    //
    //     // before looping through the continuation lanes, check if a stop is scheduled on this lane
    //     // (the code is duplicated in the loop)
    //     if (aVehicle->hasStops()) {
    //         const MSStop& nextStop = aVehicle->getNextStop();
    //         if (nextStop.lane == this) {
    //             std::stringstream msg;
    //             double distToStop, safeSpeed;
    //             if (nextStop.pars.speed > 0) {
    //                 msg << "scheduled waypoint on lane '" << myID << "' too close";
    //                 distToStop = MAX2(0.0, nextStop.pars.startPos - pos);
    //                 safeSpeed = cfModel.freeSpeed(aVehicle, speed, distToStop, nextStop.pars.speed, true, MSCFModel::CalcReason::FUTURE);
    //             } else {
    //                 msg << "scheduled stop on lane '" << myID << "' too close";
    //                 distToStop = nextStop.pars.endPos - pos;
    //                 safeSpeed = cfModel.stopSpeed(aVehicle, speed, distToStop, MSCFModel::CalcReason::FUTURE);
    //             }
    //             if (checkFailure(aVehicle, speed, dist, MAX2(0.0, safeSpeed), patchSpeedSpecial, msg.str(), InsertionCheck::STOP)) {
    //                 // we may not drive with the given velocity - we cannot stop at the stop
    //                 return false;
    //             }
    //         }
    //     }
    //     // check leader vehicle first because it could have influenced the departSpeed (for departSpeed=avg)
    //     // get the pointer to the vehicle next in front of the given position
    //     const MSLeaderInfo leaders = getLastVehicleInformation(aVehicle, 0, pos);
    //     //if (aVehicle->getID() == "disabled") std::cout << " leaders=" << leaders.toString() << "\n";
    //     const double nspeed = safeInsertionSpeed(aVehicle, -pos, leaders, speed);
    //     if (nspeed == INVALID_SPEED || checkFailure(aVehicle, speed, dist, nspeed, patchSpeed, "", InsertionCheck::LEADER_GAP)) {
    //         // we may not drive with the given velocity - we crash into the leader
    // #ifdef DEBUG_INSERTION
    //         if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //             std::cout << SIMTIME << " isInsertionSuccess lane=" << getID()
    //                       << " veh=" << aVehicle->getID()
    //                       << " pos=" << pos
    //                       << " posLat=" << posLat
    //                       << " patchSpeed=" << patchSpeed
    //                       << " speed=" << speed
    //                       << " nspeed=" << nspeed
    //                       << " leaders=" << leaders.toString()
    //                       << " failed (@700)!\n";
    //         }
    // #endif
    //         return false;
    //     }
    // #ifdef DEBUG_INSERTION
    //     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //         std::cout << SIMTIME << " speed = " << speed << " nspeed = " << nspeed << " leaders=" << leaders.toString() << "\n";
    //     }
    // #endif
    //
    //     const MSRoute& r = aVehicle->getRoute();
    //     MSRouteIterator ce = r.begin();
    //     int nRouteSuccs = 1;
    //     MSLane* currentLane = this;
    //     MSLane* nextLane = this;
    //     SUMOTime arrivalTime = MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(seen / MAX2(speed, SUMO_const_haltingSpeed));
    //     while ((seen < dist || (isRail && firstRailSignal == nullptr)) && ri != bestLaneConts.end()) {
    //         // get the next link used...
    //         std::vector<MSLink*>::const_iterator link = succLinkSec(*aVehicle, nRouteSuccs, *currentLane, bestLaneConts);
    //         // get the next used lane (including internal)
    //         if (currentLane->isLinkEnd(link)) {
    //             if (&currentLane->getEdge() == r.getLastEdge()) {
    //                 // reached the end of the route
    //                 if (aVehicle->getParameter().arrivalSpeedProcedure == ArrivalSpeedDefinition::GIVEN) {
    //                     const double remaining = seen + aVehicle->getArrivalPos() - currentLane->getLength();
    //                     const double fspeed = cfModel.freeSpeed(aVehicle, speed, remaining, aVehicle->getParameter().arrivalSpeed, true, MSCFModel::CalcReason::FUTURE);
    //                     if (checkFailure(aVehicle, speed, dist, fspeed,
    //                                      patchSpeedSpecial, "arrival speed too low", InsertionCheck::ARRIVAL_SPEED)) {
    //                         // we may not drive with the given velocity - we cannot match the specified arrival speed
    //                         return false;
    //                     }
    //                 }
    //                 if (mayContinue(aVehicle) && hasUnsafeLink()) {
    //                     // since the route is likely to continue we must be prepared for braking
    //                     if (checkFailure(aVehicle, speed, dist, cfModel.insertionStopSpeed(aVehicle, speed, seen),
    //                                      patchSpeedSpecial, "junction '" + currentLane->getEdge().getToJunction()->getID() + "' too close", InsertionCheck::JUNCTION)) {
    //                         // we may not drive with the given velocity - we cannot stop at the junction
    //                         return false;
    //                     }
    //                 }
    //             } else {
    //                 // lane does not continue
    //                 if (checkFailure(aVehicle, speed, dist, cfModel.insertionStopSpeed(aVehicle, speed, seen),
    //                                  patchSpeedSpecial, "junction '" + currentLane->getEdge().getToJunction()->getID() + "' too close", InsertionCheck::JUNCTION)) {
    //                     // we may not drive with the given velocity - we cannot stop at the junction
    //                     return false;
    //                 }
    //             }
    //             break;
    //         }
    //         if (isRail && firstRailSignal == nullptr) {
    //             std::string constraintInfo;
    //             bool isInsertionOrder;
    //             if (MSRailSignal::hasInsertionConstraint(*link, aVehicle, constraintInfo, isInsertionOrder)) {
    //                 setParameter((isInsertionOrder ? "insertionOrder" : "insertionConstraint:")
    //                              + aVehicle->getID(), constraintInfo);
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                     std::cout << " insertion constraint at link " << (*link)->getDescription() << " not cleared \n";
    //                 }
    // #endif
    //                 return false;
    //             }
    //         }
    //
    //         // might also by a regular traffic_light instead of a rail_signal
    //         if (firstRailSignal == nullptr && (*link)->getTLLogic() != nullptr) {
    //             firstRailSignal = *link;
    //             firstRailSignalDist = seen;
    //         }
    //         nextLane = (*link)->getViaLaneOrLane();
    //         if (!(*link)->opened(arrivalTime, speed, speed, aVehicle->getVehicleType().getLength(), aVehicle->getImpatience(),
    //                              cfModel.getMaxDecel(), 0, posLat, nullptr, false, aVehicle)
    //                 || (*link)->railSignalWasPassed()
    //                 || !(*link)->havePriority()
    //                 || (*link)->getState() == LINKSTATE_ZIPPER) {
    //             // have to stop at junction
    //             std::string errorMsg = "";
    //             const LinkState state = (*link)->getState();
    //             if (state == LINKSTATE_MINOR
    //                     || state == LINKSTATE_EQUAL
    //                     || state == LINKSTATE_STOP
    //                     || state == LINKSTATE_ALLWAY_STOP) {
    //                 // no sense in trying later
    //                 errorMsg = "unpriorised junction too close";
    //             } else if ((*link)->getTLLogic() != nullptr && !(*link)->getTLLogic()->getsMajorGreen((*link)->getTLIndex())) {
    //                 // traffic light never turns 'G'?
    //                 errorMsg = "tlLogic '" + (*link)->getTLLogic()->getID() + "' link " + toString((*link)->getTLIndex()) + " never switches to 'G'";
    //             }
    //             const double laneStopOffset = MAX2(getVehicleStopOffset(aVehicle),
    //                                                aVehicle->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_STOPLINE_CROSSING_GAP, MSPModel::SAFETY_GAP) - (*link)->getDistToFoePedCrossing());
    //             const double remaining = seen - laneStopOffset;
    //             if (checkFailure(aVehicle, speed, dist, cfModel.insertionStopSpeed(aVehicle, speed, remaining),
    //                              patchSpeedSpecial, errorMsg, InsertionCheck::JUNCTION)) {
    //                 // we may not drive with the given velocity - we cannot stop at the junction in time
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                     std::cout << SIMTIME << " isInsertionSuccess lane=" << getID()
    //                               << " veh=" << aVehicle->getID()
    //                               << " patchSpeed=" << patchSpeed
    //                               << " speed=" << speed
    //                               << " remaining=" << remaining
    //                               << " leader=" << currentLane->getLastVehicleInformation(aVehicle, 0, 0).toString()
    //                               << " last=" << Named::getIDSecure(getLastAnyVehicle())
    //                               << " meanSpeed=" << getMeanSpeed()
    //                               << " failed (@926)!\n";
    //                 }
    // #endif
    //                 return false;
    //             }
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << "trying insertion before minor link: "
    //                           << "insertion speed = " << speed << " dist=" << dist
    //                           << "\n";
    //             }
    // #endif
    //             if (seen >= aVehicle->getVehicleType().getMinGap()) {
    //                 break;
    //             }
    //         } else if (nextLane->isInternal()) {
    //             double tmp = 0;
    //             bool dummyReq = true;
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << "checking linkLeader for lane '" << nextLane->getID() << "'\n";
    //                 gDebugFlag1 = true;
    //             }
    // #endif
    //             double nSpeed = speed;
    //             aVehicle->checkLinkLeader(nextLane->getLinkCont()[0], nextLane, seen + nextLane->getLength(), nullptr, nSpeed, tmp, tmp, dummyReq);
    // #ifdef DEBUG_INSERTION
    //             gDebugFlag1 = false;
    // #endif
    //             if (checkFailure(aVehicle, speed, dist, nSpeed, patchSpeed, "", InsertionCheck::LEADER_GAP)) {
    //                 // we may not drive with the given velocity - there is a junction leader
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                     std::cout << " linkLeader nSpeed=" << nSpeed << " failed (@1058)!\n";
    //                 }
    // #endif
    //                 return false;
    //             }
    //         }
    //         // check how next lane affects the journey
    //         if (nextLane != nullptr) {
    //
    //             // do not insert if the bidirectional edge is occupied before a railSignal has been encountered
    //             if (firstRailSignal == nullptr && nextLane->getBidiLane() != nullptr && nextLane->getBidiLane()->getVehicleNumberWithPartials() > 0) {
    //                 if ((insertionChecks & (int)InsertionCheck::ONCOMING_TRAIN) != 0) {
    // #ifdef DEBUG_INSERTION
    //                     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                         std::cout << " nextLane=" << nextLane->getID() << " occupiedBidi\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //             }
    //
    //             // check if there are stops on the next lane that should be regarded
    //             // (this block is duplicated before the loop to deal with the insertion lane)
    //             if (aVehicle->hasStops()) {
    //                 const MSStop& nextStop = aVehicle->getNextStop();
    //                 if (nextStop.lane == nextLane) {
    //                     std::stringstream msg;
    //                     msg << "scheduled stop on lane '" << nextStop.lane->getID() << "' too close";
    //                     const double distToStop = seen + nextStop.pars.endPos;
    //                     if (checkFailure(aVehicle, speed, dist, cfModel.insertionStopSpeed(aVehicle, speed, distToStop),
    //                                      patchSpeedSpecial, msg.str(), InsertionCheck::STOP)) {
    //                         // we may not drive with the given velocity - we cannot stop at the stop
    //                         return false;
    //                     }
    //                 }
    //             }
    //
    //             // check leader on next lane
    //             const MSLeaderInfo nextLeaders = nextLane->getLastVehicleInformation(aVehicle, 0, 0);
    //             if (nextLeaders.hasVehicles()) {
    //                 const double nextLaneSpeed = nextLane->safeInsertionSpeed(aVehicle, seen, nextLeaders, speed);
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                     std::cout << SIMTIME << " leader on lane '" << nextLane->getID() << "': " << nextLeaders.toString() << " nspeed=" << nextLaneSpeed << "\n";
    //                 }
    // #endif
    //                 if (nextLaneSpeed == INVALID_SPEED || checkFailure(aVehicle, speed, dist, nextLaneSpeed, patchSpeed, "", InsertionCheck::LEADER_GAP)) {
    //                     // we may not drive with the given velocity - we crash into the leader
    // #ifdef DEBUG_INSERTION
    //                     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                         std::cout << " isInsertionSuccess lane=" << getID()
    //                                   << " veh=" << aVehicle->getID()
    //                                   << " pos=" << pos
    //                                   << " posLat=" << posLat
    //                                   << " patchSpeed=" << patchSpeed
    //                                   << " speed=" << speed
    //                                   << " nspeed=" << nextLaneSpeed
    //                                   << " nextLane=" << nextLane->getID()
    //                                   << " lead=" << nextLeaders.toString()
    //                                   << " failed (@641)!\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //             }
    //             if (!nextLane->checkForPedestrians(aVehicle, speed, dist, -seen, patchSpeed)) {
    //                 return false;
    //             }
    //             // check next lane's maximum velocity
    //             const double freeSpeed = cfModel.freeSpeed(aVehicle, speed, seen, nextLane->getVehicleMaxSpeed(aVehicle), true, MSCFModel::CalcReason::FUTURE);
    //             if (freeSpeed < speed) {
    //                 if (patchSpeedSpecial) {
    //                     speed = freeSpeed;
    //                     dist = cfModel.brakeGap(speed) + aVehicle->getVehicleType().getMinGap();
    //                 } else {
    //                     if ((insertionChecks & (int)InsertionCheck::SPEED_LIMIT) != 0) {
    //                         if (!MSGlobals::gCheckRoutes) {
    //                             WRITE_WARNINGF(TL("Vehicle '%' is inserted too fast and will violate the speed limit on a lane '%', time=%."),
    //                                            aVehicle->getID(), nextLane->getID(), time2string(SIMSTEP));
    //                         } else {
    //                             // we may not drive with the given velocity - we would be too fast on the next lane
    //                             WRITE_ERRORF(TL("Vehicle '%' will not be able to depart using the given velocity (slow lane ahead), time=%."), aVehicle->getID(), time2string(SIMSTEP));
    //                             MSNet::getInstance()->getInsertionControl().descheduleDeparture(aVehicle);
    //                             return false;
    //                         }
    //                     }
    //                 }
    //             }
    //             // check traffic on next junction
    //             // we cannot use (*link)->opened because a vehicle without priority
    //             // may already be comitted to blocking the link and unable to stop
    //             const SUMOTime leaveTime = (*link)->getLeaveTime(arrivalTime, speed, speed, aVehicle->getVehicleType().getLength());
    //             if ((*link)->hasApproachingFoe(arrivalTime, leaveTime, speed, cfModel.getMaxDecel())) {
    //                 if (checkFailure(aVehicle, speed, dist, cfModel.insertionStopSpeed(aVehicle, speed, seen), patchSpeed, "", InsertionCheck::JUNCTION)) {
    //                     // we may not drive with the given velocity - we crash at the junction
    //                     return false;
    //                 }
    //             }
    //             arrivalTime += TIME2STEPS(nextLane->getLength() / MAX2(speed, NUMERICAL_EPS));
    //             seen += nextLane->getLength();
    //             currentLane = nextLane;
    //             if ((*link)->getViaLane() == nullptr) {
    //                 nRouteSuccs++;
    //                 ++ce;
    //                 ++ri;
    //             }
    //         }
    //     }
    //
    //     const MSLeaderDistanceInfo& followers = getFollowersOnConsecutive(aVehicle, aVehicle->getBackPositionOnLane(), false);
    //     for (int i = 0; i < followers.numSublanes(); ++i) {
    //         const MSVehicle* follower = followers[i].first;
    //         if (follower != nullptr) {
    //             const double backGapNeeded = follower->getCarFollowModel().getSecureGap(follower, aVehicle, follower->getSpeed(), speed, cfModel.getMaxDecel());
    //             if (followers[i].second < backGapNeeded
    //                     && ((insertionChecks & (int)InsertionCheck::FOLLOWER_GAP) != 0
    //                         || (followers[i].second < 0 && (insertionChecks & (int)InsertionCheck::COLLISION) != 0))) {
    //                 // too close to the follower on this lane
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                     std::cout << SIMTIME << " isInsertionSuccess lane=" << getID()
    //                               << " veh=" << aVehicle->getID()
    //                               << " pos=" << pos
    //                               << " posLat=" << posLat
    //                               << " speed=" << speed
    //                               << " nspeed=" << nspeed
    //                               << " follower=" << follower->getID()
    //                               << " backGapNeeded=" << backGapNeeded
    //                               << " gap=" << followers[i].second
    //                               << " failure (@719)!\n";
    //                 }
    // #endif
    //                 return false;
    //             }
    //         }
    //     }
    //
    //     if (!checkForPedestrians(aVehicle, speed, dist, pos, patchSpeed)) {
    //         return false;
    //     }
    //
    //     MSLane* shadowLane = aVehicle->getLaneChangeModel().getShadowLane(this);
    // #ifdef DEBUG_INSERTION
    //     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //         std::cout << "    shadowLane=" << Named::getIDSecure(shadowLane) << "\n";
    //     }
    // #endif
    //     if (shadowLane != nullptr) {
    //         const MSLeaderDistanceInfo& shadowFollowers = shadowLane->getFollowersOnConsecutive(aVehicle, aVehicle->getBackPositionOnLane(), false);
    //         for (int i = 0; i < shadowFollowers.numSublanes(); ++i) {
    //             const MSVehicle* follower = shadowFollowers[i].first;
    //             if (follower != nullptr) {
    //                 const double backGapNeeded = follower->getCarFollowModel().getSecureGap(follower, aVehicle, follower->getSpeed(), speed, cfModel.getMaxDecel());
    //                 if (shadowFollowers[i].second < backGapNeeded
    //                         && ((insertionChecks & (int)InsertionCheck::FOLLOWER_GAP) != 0
    //                             || (shadowFollowers[i].second < 0 && (insertionChecks & (int)InsertionCheck::COLLISION) != 0))) {
    //                     // too close to the follower on this lane
    // #ifdef DEBUG_INSERTION
    //                     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                         std::cout << SIMTIME
    //                                   << " isInsertionSuccess shadowlane=" << shadowLane->getID()
    //                                   << " veh=" << aVehicle->getID()
    //                                   << " pos=" << pos
    //                                   << " posLat=" << posLat
    //                                   << " speed=" << speed
    //                                   << " nspeed=" << nspeed
    //                                   << " follower=" << follower->getID()
    //                                   << " backGapNeeded=" << backGapNeeded
    //                                   << " gap=" << shadowFollowers[i].second
    //                                   << " failure (@812)!\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //             }
    //         }
    //         const MSLeaderInfo& ahead = shadowLane->getLastVehicleInformation(nullptr, 0, aVehicle->getPositionOnLane(), false);
    //         for (int i = 0; i < ahead.numSublanes(); ++i) {
    //             const MSVehicle* veh = ahead[i];
    //             if (veh != nullptr) {
    //                 const double gap = veh->getBackPositionOnLane(shadowLane) - aVehicle->getPositionOnLane() - aVehicle->getVehicleType().getMinGap();
    //                 const double gapNeeded = aVehicle->getCarFollowModel().getSecureGap(aVehicle, veh, speed, veh->getSpeed(), veh->getCarFollowModel().getMaxDecel());
    //                 if (gap <  gapNeeded
    //                         && ((insertionChecks & (int)InsertionCheck::LEADER_GAP) != 0
    //                             || (gap < 0 && (insertionChecks & (int)InsertionCheck::COLLISION) != 0))) {
    //                     // too close to the shadow leader
    // #ifdef DEBUG_INSERTION
    //                     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                         std::cout << SIMTIME
    //                                   << " isInsertionSuccess shadowlane=" << shadowLane->getID()
    //                                   << " veh=" << aVehicle->getID()
    //                                   << " pos=" << pos
    //                                   << " posLat=" << posLat
    //                                   << " speed=" << speed
    //                                   << " nspeed=" << nspeed
    //                                   << " leader=" << veh->getID()
    //                                   << " gapNeeded=" << gapNeeded
    //                                   << " gap=" << gap
    //                                   << " failure (@842)!\n";
    //                     }
    // #endif
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     if (followers.numFreeSublanes() > 0) {
    //         // check approaching vehicles to prevent rear-end collisions
    //         const double backOffset = pos - aVehicle->getVehicleType().getLength();
    //         const double missingRearGap = getMissingRearGap(aVehicle, backOffset, speed);
    //         if (missingRearGap > 0
    //                 && (insertionChecks & (int)InsertionCheck::LEADER_GAP) != 0) {
    //             // too close to a follower
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << SIMTIME
    //                           << " isInsertionSuccess lane=" << getID()
    //                           << " veh=" << aVehicle->getID()
    //                           << " pos=" << pos
    //                           << " posLat=" << posLat
    //                           << " speed=" << speed
    //                           << " nspeed=" << nspeed
    //                           << " missingRearGap=" << missingRearGap
    //                           << " failure (@728)!\n";
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //     if (insertionChecks == (int)InsertionCheck::NONE) {
    //         speed = MAX2(0.0, speed);
    //     }
    //     // may got negative while adaptation
    //     if (speed < 0) {
    // #ifdef DEBUG_INSERTION
    //         if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //             std::cout << SIMTIME
    //                       << " isInsertionSuccess lane=" << getID()
    //                       << " veh=" << aVehicle->getID()
    //                       << " pos=" << pos
    //                       << " posLat=" << posLat
    //                       << " speed=" << speed
    //                       << " nspeed=" << nspeed
    //                       << " failed (@733)!\n";
    //         }
    // #endif
    //         return false;
    //     }
    //     const int bestLaneOffset = aVehicle->getBestLaneOffset();
    //     const double extraReservation = aVehicle->getLaneChangeModel().getExtraReservation(bestLaneOffset);
    //     if (extraReservation > 0) {
    //         std::stringstream msg;
    //         msg << "too many lane changes required on lane '" << myID << "'";
    //         // we need to take into acount one extra actionStep of delay due to #3665
    //         double distToStop = aVehicle->getBestLaneDist() - pos - extraReservation - speed * aVehicle->getActionStepLengthSecs();
    //         if (distToStop >= 0) {
    //             double stopSpeed = cfModel.stopSpeed(aVehicle, speed, distToStop, MSCFModel::CalcReason::FUTURE);
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //                 std::cout << "\nIS_INSERTION_SUCCESS\n"
    //                           << SIMTIME << " veh=" << aVehicle->getID() << " bestLaneOffset=" << bestLaneOffset << " bestLaneDist=" << aVehicle->getBestLaneDist() << " extraReservation=" << extraReservation
    //                           << " distToStop=" << distToStop << " v=" << speed << " v2=" << stopSpeed << "\n";
    //             }
    // #endif
    //             if (checkFailure(aVehicle, speed, distToStop, MAX2(0.0, stopSpeed),
    //                              patchSpeedSpecial, msg.str(), InsertionCheck::LANECHANGE)) {
    //                 // we may not drive with the given velocity - we cannot reserve enough space for lane changing
    //                 return false;
    //             }
    //         }
    //     }
    //     // enter
    //     incorporateVehicle(aVehicle, pos, speed, posLat, find_if(myVehicles.begin(), myVehicles.end(), [&](MSVehicle * const v) {
    //         return v->getPositionOnLane() >= pos;
    //     }), notification);
    // #ifdef DEBUG_INSERTION
    //     if (DEBUG_COND2(aVehicle) || DEBUG_COND) {
    //         std::cout << SIMTIME
    //                   << " isInsertionSuccess lane=" << getID()
    //                   << " veh=" << aVehicle->getID()
    //                   << " pos=" << pos
    //                   << " posLat=" << posLat
    //                   << " speed=" << speed
    //                   << " nspeed=" << nspeed
    //                   << "\n myVehicles=" << toString(myVehicles)
    //                   << " myPartial=" << toString(myPartialVehicles)
    //                   << " myManeuverReservations=" << toString(myManeuverReservations)
    //                   << "\n success!\n";
    //     }
    // #endif
    //     if (isRail) {
    //         unsetParameter("insertionConstraint:" + aVehicle->getID());
    //         unsetParameter("insertionOrder:" + aVehicle->getID());
    //         unsetParameter("insertionBlocked:" + aVehicle->getID());
    //         // rail_signal (not traffic_light) requires approach information for
    //         // switching correctly at the start of the next simulation step
    //         if (firstRailSignal != nullptr && firstRailSignal->getJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             aVehicle->registerInsertionApproach(firstRailSignal, firstRailSignalDist);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_insertion_success(&mut self, vehicle: Ptr<MSVehicle>, speed: f64, pos: f64, posLat: f64, patchSpeed: bool, notification: MSMoveReminder_Notification) -> bool {
        use utils::xml::sumo_xml_definitions::{LINKSTATE_ALLWAY_STOP, LINKSTATE_EQUAL, LINKSTATE_MINOR, LINKSTATE_STOP, LINKSTATE_ZIPPER, SUMO_ATTR_JM_STOPLINE_CROSSING_GAP, SumoXMLNodeType};
        use crate::microsim::cfmodels::mscf_model::INVALID_SPEED;
        let aVehicle = vehicle;
        let mut speed = speed;
        let mut pos = pos;
        let insertionChecks = aVehicle.get_insertion_checks();
        if pos < 0. || pos > self.myLength {
            // we may not start there
            utils::write_warningf!("Invalid departPos % given for vehicle '%', time=%. Inserting at lane end instead.",
                pos, aVehicle.get_id(), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
            pos = self.myLength;
        }

        aVehicle.m().set_tentative_lane_and_position(Ptr::from_mut(self), pos, posLat);
        aVehicle.m().update_best_lanes(false, Ptr::from_mut(self));
        let cfModel = aVehicle.get_car_follow_model();
        let bestLaneConts = aVehicle.get_best_lanes_continuation_2(Ptr::from_mut(self));
        let mut ri: usize = 0;
        let mut seen = self.get_length() - pos; // == distance from insertion position until the end of the currentLane
        let mut dist = cfModel.brake_gap(speed) + aVehicle.get_vehicle_type().get_min_gap();
        let isRail = aVehicle.is_rail();
        if isRail && insertionChecks != InsertionCheck::NONE as i32
            && aVehicle.get_parameter().departProcedure != DepartDefinition::SPLIT
            && MSRailSignalControl::is_signalized(aVehicle.get_v_class())
            && utils::common::sumo_vehicle_class::is_railway_or_shared(self.myPermissions) {
            let dw = MSDriveWay::get_departure_driveway(aVehicle.cast::<SUMOVehicle>(), false);
            let mut occupied: MSEdgeVector = Vec::new();
            if dw.foe_drive_way_occupied(false, aVehicle.cast::<SUMOVehicle>(), &mut occupied) {
                self.base2.set_parameter(&format!("insertionBlocked:{}", aVehicle.get_id()), dw.base2.get_id());
                return false;
            }
        }
        if !self.get_bidi_lane().is_null() && isRail {
            // do not insert if the bidirectional edge is occupied
            if self.get_bidi_lane().get_vehicle_number_with_partials() > 0 && (insertionChecks & (InsertionCheck::BIDI as i32)) != 0 {
                return false;
            }
            // do not insert the back of the train would be put onto an occupied bidi-lane
            let mut backLength = aVehicle.get_length() - pos;
            if backLength > 0. && (insertionChecks & (InsertionCheck::BIDI as i32)) != 0 {
                let mut pred = self.get_logical_predecessor_lane();
                let mut bidi = if pred.is_null() { Ptr::default() } else { pred.get_bidi_lane() };
                while backLength > 0. && !bidi.is_null() {
                    if bidi.get_vehicle_number_with_partials() > 0 {
                        return false;
                    }
                    backLength -= bidi.get_length();
                    pred = pred.get_logical_predecessor_lane();
                    bidi = if pred.is_null() { Ptr::default() } else { pred.get_bidi_lane() };
                }
            }
        }
        let mut firstRailSignal: Ptr<MSLink> = Ptr::default();
        let mut firstRailSignalDist: f64 = -1.;
        // whether speed may be patched for unavoidable reasons (stops, speedLimits, ...)
        let patchSpeedSpecial = patchSpeed || aVehicle.get_parameter().departSpeedProcedure != DepartSpeedDefinition::GIVEN;

        // before looping through the continuation lanes, check if a stop is scheduled on this lane
        // (the code is duplicated in the loop)
        if aVehicle.has_stops() {
            let nextStop = aVehicle.get_next_stop();
            if nextStop.lane == Ptr::from_mut(self) {
                let msg: String;
                let distToStop: f64;
                let safeSpeed: f64;
                if nextStop.pars.speed > 0. {
                    msg = format!("scheduled waypoint on lane '{}' too close", self.get_id());
                    distToStop = 0f64.max(nextStop.pars.startPos - pos);
                    safeSpeed = cfModel.free_speed(aVehicle, speed, distToStop, nextStop.pars.speed, true, MSCFModel_CalcReason::FUTURE);
                } else {
                    msg = format!("scheduled stop on lane '{}' too close", self.get_id());
                    distToStop = nextStop.pars.endPos - pos;
                    safeSpeed = cfModel.stop_speed(aVehicle, speed, distToStop, MSCFModel_CalcReason::FUTURE);
                }
                if self.check_failure(aVehicle, &mut speed, &mut dist, 0f64.max(safeSpeed), patchSpeedSpecial, msg, InsertionCheck::STOP) {
                    // we may not drive with the given velocity - we cannot stop at the stop
                    return false;
                }
            }
        }
        // check leader vehicle first because it could have influenced the departSpeed (for departSpeed=avg)
        // get the pointer to the vehicle next in front of the given position
        let leaders = self.get_last_vehicle_information(aVehicle, 0., pos, true, Ptr::default());
        let nspeed = self.safe_insertion_speed(aVehicle, -pos, &leaders, speed);
        if std::env::var_os("TX02_TRACE").is_some() && aVehicle.get_id() == "taxi0" {
            eprintln!("TX02 ins t={} lane={} pos={:.6} speed={:.6} leaders={} nspeed={:.6}", MSNet::get_instance().get_current_time_step(), self.get_id(), pos, speed, leaders.to_string(), nspeed);
        }
        if nspeed == INVALID_SPEED || self.check_failure(aVehicle, &mut speed, &mut dist, nspeed, patchSpeed, String::new(), InsertionCheck::LEADER_GAP) {
            // we may not drive with the given velocity - we crash into the leader
            return false;
        }

        let r = aVehicle.get_route();
        let mut nRouteSuccs: i32 = 1;
        let mut currentLane: Ptr<MSLane> = Ptr::from_mut(self);
        let mut nextLane: Ptr<MSLane> = Ptr::from_mut(self);
        let mut arrivalTime: SUMOTime = MSNet::get_instance().get_current_time_step() + utils::common::time::time2steps(seen / speed.max(SUMO_CONST_HALTING_SPEED));
        while (seen < dist || (isRail && firstRailSignal.is_null())) && ri < bestLaneConts.len() {
            // get the next link used...
            let mut link: usize = MSLane::succ_link_sec(&*aVehicle, nRouteSuccs, &*currentLane, bestLaneConts);
            // get the next used lane (including internal)
            if currentLane.is_link_end(&mut link) {
                if Ptr::from_mut(currentLane.get_edge()) == r.get_last_edge() {
                    // reached the end of the route
                    if aVehicle.get_parameter().arrivalSpeedProcedure == ArrivalSpeedDefinition::GIVEN {
                        let remaining = seen + aVehicle.get_arrival_pos() - currentLane.get_length();
                        let fspeed = cfModel.free_speed(aVehicle, speed, remaining, aVehicle.get_parameter().arrivalSpeed, true, MSCFModel_CalcReason::FUTURE);
                        if self.check_failure(aVehicle, &mut speed, &mut dist, fspeed, patchSpeedSpecial, "arrival speed too low".to_string(), InsertionCheck::ARRIVAL_SPEED) {
                            // we may not drive with the given velocity - we cannot match the specified arrival speed
                            return false;
                        }
                    }
                    if self.may_continue(aVehicle) && self.has_unsafe_link() {
                        // since the route is likely to continue we must be prepared for braking
                        let stopSpeed = cfModel.insertion_stop_speed(aVehicle, speed, seen);
                        let errorMsg = format!("junction '{}' too close", currentLane.get_edge().get_to_junction().get_id());
                        if self.check_failure(aVehicle, &mut speed, &mut dist, stopSpeed, patchSpeedSpecial, errorMsg, InsertionCheck::JUNCTION) {
                            // we may not drive with the given velocity - we cannot stop at the junction
                            return false;
                        }
                    }
                } else {
                    // lane does not continue
                    let stopSpeed = cfModel.insertion_stop_speed(aVehicle, speed, seen);
                    let errorMsg = format!("junction '{}' too close", currentLane.get_edge().get_to_junction().get_id());
                    if self.check_failure(aVehicle, &mut speed, &mut dist, stopSpeed, patchSpeedSpecial, errorMsg, InsertionCheck::JUNCTION) {
                        // we may not drive with the given velocity - we cannot stop at the junction
                        return false;
                    }
                }
                break;
            }
            let l: Ptr<MSLink> = currentLane.myLinks[link];
            if isRail && firstRailSignal.is_null() {
                let mut constraintInfo = String::new();
                let mut isInsertionOrder = false;
                if MSRailSignal::has_insertion_constraint(l, aVehicle, &mut constraintInfo, &mut isInsertionOrder) {
                    let key = if isInsertionOrder { "insertionOrder" } else { "insertionConstraint:" };
                    self.base2.set_parameter(&format!("{}{}", key, aVehicle.get_id()), &constraintInfo);
                    return false;
                }
            }

            // might also by a regular traffic_light instead of a rail_signal
            if firstRailSignal.is_null() && !l.get_tl_logic().is_null() {
                firstRailSignal = l;
                firstRailSignalDist = seen;
            }
            nextLane = l.get_via_lane_or_lane();
            if !l.opened(arrivalTime, speed, speed, aVehicle.get_vehicle_type().get_length(), aVehicle.get_impatience(),
                cfModel.get_max_decel(), 0, posLat, Ptr::default(), false, aVehicle.cast::<SUMOTrafficObject>(), -1.)
                || l.rail_signal_was_passed()
                || !l.have_priority()
                || l.get_state() == LINKSTATE_ZIPPER {
                // have to stop at junction
                let mut errorMsg = String::new();
                let state = l.get_state();
                if state == LINKSTATE_MINOR || state == LINKSTATE_EQUAL || state == LINKSTATE_STOP || state == LINKSTATE_ALLWAY_STOP {
                    // no sense in trying later
                    errorMsg = "unpriorised junction too close".to_string();
                } else if !l.get_tl_logic().is_null() && !l.get_tl_logic().gets_major_green(l.get_tl_index()) {
                    // traffic light never turns 'G'?
                    errorMsg = format!("tlLogic '{}' link {} never switches to 'G'", l.get_tl_logic().get_id(), l.get_tl_index());
                }
                let laneStopOffset = self.get_vehicle_stop_offset(aVehicle).max(
                    aVehicle.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_STOPLINE_CROSSING_GAP, MSP_MODEL_SAFETY_GAP) - l.get_dist_to_foe_ped_crossing());
                let remaining = seen - laneStopOffset;
                let stopSpeed = cfModel.insertion_stop_speed(aVehicle, speed, remaining);
                if self.check_failure(aVehicle, &mut speed, &mut dist, stopSpeed, patchSpeedSpecial, errorMsg, InsertionCheck::JUNCTION) {
                    // we may not drive with the given velocity - we cannot stop at the junction in time
                    return false;
                }
                if seen >= aVehicle.get_vehicle_type().get_min_gap() {
                    break;
                }
            } else if nextLane.is_internal() {
                let mut tmp = 0f64;
                let mut tmp2 = 0f64;
                let mut dummyReq = true;
                let mut nSpeed = speed;
                aVehicle.check_link_leader(nextLane.get_link_cont()[0], nextLane, seen + nextLane.get_length(), Ptr::default(), &mut nSpeed, &mut tmp, &mut tmp2, &mut dummyReq, false);
                if self.check_failure(aVehicle, &mut speed, &mut dist, nSpeed, patchSpeed, String::new(), InsertionCheck::LEADER_GAP) {
                    // we may not drive with the given velocity - there is a junction leader
                    return false;
                }
            }
            // check how next lane affects the journey
            if !nextLane.is_null() {
                // do not insert if the bidirectional edge is occupied before a railSignal has been encountered
                if firstRailSignal.is_null() && !nextLane.get_bidi_lane().is_null() && nextLane.get_bidi_lane().get_vehicle_number_with_partials() > 0 {
                    if (insertionChecks & (InsertionCheck::ONCOMING_TRAIN as i32)) != 0 {
                        return false;
                    }
                }

                // check if there are stops on the next lane that should be regarded
                // (this block is duplicated before the loop to deal with the insertion lane)
                if aVehicle.has_stops() {
                    let nextStop = aVehicle.get_next_stop();
                    if nextStop.lane == nextLane {
                        let msg = format!("scheduled stop on lane '{}' too close", nextStop.lane.get_id());
                        let distToStop = seen + nextStop.pars.endPos;
                        let stopSpeed = cfModel.insertion_stop_speed(aVehicle, speed, distToStop);
                        if self.check_failure(aVehicle, &mut speed, &mut dist, stopSpeed, patchSpeedSpecial, msg, InsertionCheck::STOP) {
                            // we may not drive with the given velocity - we cannot stop at the stop
                            return false;
                        }
                    }
                }

                // check leader on next lane
                let nextLeaders = nextLane.get_last_vehicle_information(aVehicle, 0., 0., true, Ptr::default());
                if nextLeaders.has_vehicles() {
                    let nextLaneSpeed = nextLane.m().safe_insertion_speed(aVehicle, seen, &nextLeaders, speed);
                    if nextLaneSpeed == INVALID_SPEED || self.check_failure(aVehicle, &mut speed, &mut dist, nextLaneSpeed, patchSpeed, String::new(), InsertionCheck::LEADER_GAP) {
                        // we may not drive with the given velocity - we crash into the leader
                        return false;
                    }
                }
                if !nextLane.check_for_pedestrians(aVehicle, &mut speed, &mut dist, -seen, patchSpeed) {
                    return false;
                }
                // check next lane's maximum velocity
                let freeSpeed = cfModel.free_speed(aVehicle, speed, seen, nextLane.get_vehicle_max_speed(aVehicle.cast::<SUMOTrafficObject>()), true, MSCFModel_CalcReason::FUTURE);
                if freeSpeed < speed {
                    if patchSpeedSpecial {
                        speed = freeSpeed;
                        dist = cfModel.brake_gap(speed) + aVehicle.get_vehicle_type().get_min_gap();
                    } else if (insertionChecks & (InsertionCheck::SPEED_LIMIT as i32)) != 0 {
                        if !g_check_routes() {
                            utils::write_warningf!("Vehicle '%' is inserted too fast and will violate the speed limit on a lane '%', time=%.",
                                aVehicle.get_id(), nextLane.get_id(), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                        } else {
                            // we may not drive with the given velocity - we would be too fast on the next lane
                            utils::write_errorf!("Vehicle '%' will not be able to depart using the given velocity (slow lane ahead), time=%.",
                                aVehicle.get_id(), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                            MSNet::get_instance().m().get_insertion_control().deschedule_departure(aVehicle.cast::<SUMOVehicle>());
                            return false;
                        }
                    }
                }
                // check traffic on next junction
                // we cannot use (*link)->opened because a vehicle without priority
                // may already be comitted to blocking the link and unable to stop
                let leaveTime = l.get_leave_time(arrivalTime, speed, speed, aVehicle.get_vehicle_type().get_length());
                if l.has_approaching_foe(arrivalTime, leaveTime, speed, cfModel.get_max_decel()) {
                    let stopSpeed = cfModel.insertion_stop_speed(aVehicle, speed, seen);
                    if self.check_failure(aVehicle, &mut speed, &mut dist, stopSpeed, patchSpeed, String::new(), InsertionCheck::JUNCTION) {
                        // we may not drive with the given velocity - we crash at the junction
                        return false;
                    }
                }
                arrivalTime += utils::common::time::time2steps(nextLane.get_length() / speed.max(NUMERICAL_EPS));
                seen += nextLane.get_length();
                currentLane = nextLane;
                if l.get_via_lane().is_null() {
                    nRouteSuccs += 1;
                    ri += 1;
                }
            }
        }

        let followers = self.get_followers_on_consecutive(aVehicle, aVehicle.get_back_position_on_lane_2(), false, -1., MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
        let tx02 = std::env::var_os("TX02_TRACE").is_some() && aVehicle.get_id() == "taxi0";
        if tx02 {
            eprintln!("TX02 reachfollow t={} lane={}", MSNet::get_instance().get_current_time_step(), self.get_id());
        }
        for i in 0..followers.num_sublanes() {
            let (follower, gap) = followers.index_op(i);
            if !follower.is_null() {
                let backGapNeeded = follower.get_car_follow_model().get_secure_gap(follower, aVehicle, follower.get_speed(), speed, cfModel.get_max_decel());
                if tx02 {
                    eprintln!("TX02 follower={} gap={:.6} backGapNeeded={:.6}", follower.get_id(), gap, backGapNeeded);
                }
                if gap < backGapNeeded
                    && ((insertionChecks & (InsertionCheck::FOLLOWER_GAP as i32)) != 0
                        || (gap < 0. && (insertionChecks & (InsertionCheck::COLLISION as i32)) != 0)) {
                    // too close to the follower on this lane
                    return false;
                }
            }
        }

        if !self.check_for_pedestrians(aVehicle, &mut speed, &mut dist, pos, patchSpeed) {
            return false;
        }

        let shadowLane = aVehicle.m().get_lane_change_model().get_shadow_lane_2(Ptr::from_mut(self));
        if !shadowLane.is_null() {
            let shadowFollowers = shadowLane.get_followers_on_consecutive(aVehicle, aVehicle.get_back_position_on_lane_2(), false, -1., MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            for i in 0..shadowFollowers.num_sublanes() {
                let (follower, gap) = shadowFollowers.index_op(i);
                if !follower.is_null() {
                    let backGapNeeded = follower.get_car_follow_model().get_secure_gap(follower, aVehicle, follower.get_speed(), speed, cfModel.get_max_decel());
                    if gap < backGapNeeded
                        && ((insertionChecks & (InsertionCheck::FOLLOWER_GAP as i32)) != 0
                            || (gap < 0. && (insertionChecks & (InsertionCheck::COLLISION as i32)) != 0)) {
                        // too close to the follower on this lane
                        return false;
                    }
                }
            }
            let ahead = shadowLane.get_last_vehicle_information(Ptr::default(), 0., aVehicle.get_position_on_lane(), false, Ptr::default());
            for i in 0..ahead.num_sublanes() {
                let veh = ahead.index_op(i);
                if !veh.is_null() {
                    let gap = veh.get_back_position_on_lane(shadowLane) - aVehicle.get_position_on_lane() - aVehicle.get_vehicle_type().get_min_gap();
                    let gapNeeded = aVehicle.get_car_follow_model().get_secure_gap(aVehicle, veh, speed, veh.get_speed(), veh.get_car_follow_model().get_max_decel());
                    if gap < gapNeeded
                        && ((insertionChecks & (InsertionCheck::LEADER_GAP as i32)) != 0
                            || (gap < 0. && (insertionChecks & (InsertionCheck::COLLISION as i32)) != 0)) {
                        // too close to the shadow leader
                        return false;
                    }
                }
            }
        }
        if followers.num_free_sublanes() > 0 {
            // check approaching vehicles to prevent rear-end collisions
            let backOffset = pos - aVehicle.get_vehicle_type().get_length();
            let missingRearGap = self.get_missing_rear_gap(aVehicle, backOffset, speed);
            if missingRearGap > 0. && (insertionChecks & (InsertionCheck::LEADER_GAP as i32)) != 0 {
                // too close to a follower
                return false;
            }
        }
        if insertionChecks == InsertionCheck::NONE as i32 {
            speed = 0f64.max(speed);
        }
        // may got negative while adaptation
        if speed < 0. {
            return false;
        }
        let bestLaneOffset = aVehicle.get_best_lane_offset();
        let extraReservation = aVehicle.m().get_lane_change_model().get_extra_reservation(bestLaneOffset, 0.);
        if extraReservation > 0. {
            let msg = format!("too many lane changes required on lane '{}'", self.get_id());
            // we need to take into acount one extra actionStep of delay due to #3665
            let mut distToStop = aVehicle.get_best_lane_dist() - pos - extraReservation - speed * aVehicle.get_action_step_length_secs();
            if distToStop >= 0. {
                let stopSpeed = cfModel.stop_speed(aVehicle, speed, distToStop, MSCFModel_CalcReason::FUTURE);
                if self.check_failure(aVehicle, &mut speed, &mut distToStop, 0f64.max(stopSpeed), patchSpeedSpecial, msg, InsertionCheck::LANECHANGE) {
                    // we may not drive with the given velocity - we cannot reserve enough space for lane changing
                    return false;
                }
            }
        }
        // enter
        let at = self.myVehicles.iter().position(|v| v.get_position_on_lane() >= pos).unwrap_or(self.myVehicles.len());
        self.incorporate_vehicle(aVehicle, pos, speed, posLat, at, notification);
        if isRail {
            self.base2.unset_parameter(&format!("insertionConstraint:{}", aVehicle.get_id()));
            self.base2.unset_parameter(&format!("insertionOrder:{}", aVehicle.get_id()));
            self.base2.unset_parameter(&format!("insertionBlocked:{}", aVehicle.get_id()));
            // rail_signal (not traffic_light) requires approach information for
            // switching correctly at the start of the next simulation step
            if !firstRailSignal.is_null() && firstRailSignal.get_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                aVehicle.m().register_insertion_approach(firstRailSignal, firstRailSignalDist);
            }
        }
        true
    }

    /// C++: `bool MSLane::checkFailure(const MSVehicle *, double &, double &, const double, const bool, const std::string, InsertionCheck) const` [const]
    // bool
    // MSLane::checkFailure(const MSVehicle* aVehicle, double& speed, double& dist, const double nspeed, const bool patchSpeed, const std::string errorMsg, InsertionCheck check) const {
    //     if (nspeed < speed) {
    //         if (patchSpeed) {
    //             speed = MIN2(nspeed, speed);
    //             dist = aVehicle->getCarFollowModel().brakeGap(speed) + aVehicle->getVehicleType().getMinGap();
    //         } else if (speed > 0) {
    //             if ((aVehicle->getInsertionChecks() & (int)check) == 0) {
    //                 return false;
    //             }
    //             if (MSGlobals::gEmergencyInsert) {
    //                 // Check whether vehicle can stop at the given distance when applying emergency braking
    //                 double emergencyBrakeGap = 0.5 * speed * speed / aVehicle->getCarFollowModel().getEmergencyDecel();
    //                 if (emergencyBrakeGap <= dist) {
    //                     // Vehicle may stop in time with emergency deceleration
    //                     // still, emit a warning
    //                     WRITE_WARNINGF(TL("Vehicle '%' is inserted in an emergency situation, time=%."), aVehicle->getID(), time2string(SIMSTEP));
    //                     return false;
    //                 }
    //             }
    //
    //             if (errorMsg != "") {
    //                 WRITE_ERRORF(TL("Vehicle '%' will not be able to depart on lane '%' with speed % (%), time=%."),
    //                              aVehicle->getID(), getID(), speed, errorMsg, time2string(SIMSTEP));
    //                 MSNet::getInstance()->getInsertionControl().descheduleDeparture(aVehicle);
    //             }
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_failure(&self, aVehicle: Ptr<MSVehicle>, speed: &mut f64, dist: &mut f64, nspeed: f64, patchSpeed: bool, errorMsg: String, check: InsertionCheck) -> bool {
        if nspeed < *speed {
            if patchSpeed {
                *speed = nspeed.min(*speed);
                *dist = aVehicle.get_car_follow_model().brake_gap(*speed) + aVehicle.get_vehicle_type().get_min_gap();
            } else if *speed > 0. {
                if (aVehicle.get_insertion_checks() & (check as i32)) == 0 {
                    return false;
                }
                if g_emergency_insert() {
                    // Check whether vehicle can stop at the given distance when applying emergency braking
                    let emergencyBrakeGap = 0.5 * *speed * *speed / aVehicle.get_car_follow_model().get_emergency_decel();
                    if emergencyBrakeGap <= *dist {
                        // Vehicle may stop in time with emergency deceleration
                        // still, emit a warning
                        utils::write_warningf!("Vehicle '%' is inserted in an emergency situation, time=%.", aVehicle.get_id(), time2string(MSNet::get_instance().get_current_time_step()));
                        return false;
                    }
                }

                if errorMsg != "" {
                    utils::write_errorf!("Vehicle '%' will not be able to depart on lane '%' with speed % (%), time=%.",
                        aVehicle.get_id(), self.get_id(), *speed, errorMsg, time2string(MSNet::get_instance().get_current_time_step()));
                    MSNet::get_instance().m().get_insertion_control().deschedule_departure(aVehicle.cast::<SUMOVehicle>());
                }
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLane::lastInsertion(MSVehicle &, double, double, bool)`
    /// inserts vehicle as close as possible to the last vehicle on this lane (or at the end of the lane if there is no leader)
    // bool
    // MSLane::lastInsertion(MSVehicle& veh, double mspeed, double posLat, bool patchSpeed) {
    //     double pos = getLength() - POSITION_EPS;
    //     MSVehicle* leader = getLastAnyVehicle();
    //     // back position of leader relative to this lane
    //     double leaderBack;
    //     if (leader == nullptr) {
    //         /// look for a leaders on consecutive lanes
    //         veh.setTentativeLaneAndPosition(this, pos, posLat);
    //         veh.updateBestLanes(false, this);
    //         std::pair<MSVehicle* const, double> leaderInfo = getLeader(&veh, pos, veh.getBestLanesContinuation(), veh.getCarFollowModel().brakeGap(mspeed));
    //         leader = leaderInfo.first;
    //         leaderBack = pos + leaderInfo.second + veh.getVehicleType().getMinGap();
    //     } else {
    //         leaderBack = leader->getBackPositionOnLane(this);
    //         //std::cout << " leaderPos=" << leader->getPositionOnLane(this) << " leaderBack=" << leader->getBackPositionOnLane(this) << " leaderLane=" << leader->getLane()->getID() << "\n";
    //     }
    //     if (leader == nullptr) {
    //         // insert at the end of this lane
    //         return isInsertionSuccess(&veh, mspeed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED);
    //     } else {
    //         // try to insert behind the leader
    //         const double frontGapNeeded = veh.getCarFollowModel().getSecureGap(&veh, leader, mspeed, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel()) + veh.getVehicleType().getMinGap() + POSITION_EPS;
    //         if (leaderBack >= frontGapNeeded) {
    //             pos = MIN2(pos, leaderBack - frontGapNeeded);
    //             bool result = isInsertionSuccess(&veh, mspeed, pos, posLat, patchSpeed, MSMoveReminder::NOTIFICATION_DEPARTED);
    //             //if (!result) std::cout << " insertLast failed for " << veh.getID() << " pos=" << pos << " leaderBack=" << leaderBack << " frontGapNeeded=" << frontGapNeeded << "\n";
    //             return result;
    //         }
    //         //std::cout << " insertLast failed for " << veh.getID() << " pos=" << pos << " leaderBack=" << leaderBack << " frontGapNeeded=" << frontGapNeeded << "\n";
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn last_insertion(&mut self, veh: &mut MSVehicle, mspeed: f64, posLat: f64, patchSpeed: bool) -> bool {
        let mut pos = self.myLength - POSITION_EPS;
        let mut leader = self.get_last_any_vehicle();
        let vp = Ptr::from_mut(veh);
        // back position of leader relative to this lane
        let leaderBack: f64;
        if leader.is_null() {
            // look for a leaders on consecutive lanes
            veh.set_tentative_lane_and_position(Ptr::from_mut(self), pos, posLat);
            veh.update_best_lanes(false, Ptr::from_mut(self));
            let brakeGap = veh.get_car_follow_model().brake_gap(mspeed);
            let (leaderInfoFirst, leaderInfoSecond) = self.get_leader(vp, pos, veh.get_best_lanes_continuation(), brakeGap, false);
            leader = leaderInfoFirst;
            leaderBack = pos + leaderInfoSecond + veh.get_vehicle_type().get_min_gap();
        } else {
            leaderBack = leader.get_back_position_on_lane(Ptr::from_mut(self));
        }
        if leader.is_null() {
            // insert at the end of this lane
            return self.is_insertion_success(vp, mspeed, pos, posLat, patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED);
        } else {
            // try to insert behind the leader
            let frontGapNeeded = veh.get_car_follow_model().get_secure_gap(vp, leader, mspeed, leader.get_speed(), leader.get_car_follow_model().get_max_decel()) + veh.get_vehicle_type().get_min_gap() + POSITION_EPS;
            if leaderBack >= frontGapNeeded {
                pos = pos.min(leaderBack - frontGapNeeded);
                let result = self.is_insertion_success(vp, mspeed, pos, posLat, patchSpeed, MSMoveReminder_Notification::NOTIFICATION_DEPARTED);
                return result;
            }
        }
        false
    }

    /// C++: `bool MSLane::freeInsertion(MSVehicle &, double, double, MSMoveReminder::Notification)`
    /// Tries to insert the given vehicle on any place
    // bool
    // MSLane::freeInsertion(MSVehicle& veh, double mspeed, double posLat,
    //                       MSMoveReminder::Notification notification) {
    //     // try to insert teleporting vehicles fully on this lane
    //     double maxPos = myLength;
    //     if (veh.hasStops() && veh.getNextStop().edge == veh.getCurrentRouteEdge()) {
    //         maxPos = MAX2(0.0, veh.getNextStop().getEndPos(veh));
    //     }
    //     const double minPos = (notification == MSMoveReminder::NOTIFICATION_TELEPORT ?
    //                            MIN2(maxPos, veh.getVehicleType().getLength()) : 0);
    //     veh.setTentativeLaneAndPosition(this, minPos, 0);
    //     if (myVehicles.size() == 0) {
    //         // ensure sufficient gap to followers on predecessor lanes
    //         const double backOffset = minPos - veh.getVehicleType().getLength();
    //         const double missingRearGap = getMissingRearGap(&veh, backOffset, mspeed);
    //         if (missingRearGap > 0) {
    //             if (minPos + missingRearGap <= maxPos) {
    //                 // @note. The rear gap is tailored to mspeed. If it changes due
    //                 // to a leader vehicle (on subsequent lanes) insertion will
    //                 // still fail. Under the right combination of acceleration and
    //                 // deceleration values there might be another insertion
    //                 // positions that would be successful be we do not look for it.
    //                 //std::cout << SIMTIME << " freeInsertion lane=" << getID() << " veh=" << veh.getID() << " unclear @(340)\n";
    //                 return isInsertionSuccess(&veh, mspeed, minPos + missingRearGap, posLat, true, notification);
    //             }
    //             return false;
    //         } else {
    //             return isInsertionSuccess(&veh, mspeed, minPos, posLat, true, notification);
    //         }
    //
    //     } else {
    //         // check whether the vehicle can be put behind the last one if there is such
    //         const MSVehicle* const leader = myVehicles.back(); // @todo reproduction of bogus old behavior. see #1961
    //         const double leaderPos = leader->getBackPositionOnLane(this);
    //         const double speed = leader->getSpeed();
    //         const double frontGapNeeded = veh.getCarFollowModel().getSecureGap(&veh, leader, speed, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel()) + veh.getVehicleType().getMinGap();
    //         if (leaderPos >= frontGapNeeded) {
    //             const double tspeed = MIN2(veh.getCarFollowModel().insertionFollowSpeed(&veh, mspeed, frontGapNeeded, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel(), leader), mspeed);
    //             // check whether we can insert our vehicle behind the last vehicle on the lane
    //             if (isInsertionSuccess(&veh, tspeed, minPos, posLat, true, notification)) {
    //                 //std::cout << SIMTIME << " freeInsertion lane=" << getID() << " veh=" << veh.getID() << " pos=" << minPos<< " speed=" << speed  << " tspeed=" << tspeed << " frontGapNeeded=" << frontGapNeeded << " lead=" << leader->getID() << " lPos=" << leaderPos << "\n   vehsOnLane=" << toString(myVehicles) << " @(358)\n";
    //                 return true;
    //             }
    //         }
    //     }
    //     // go through the lane, look for free positions (starting after the last vehicle)
    //     MSLane::VehCont::iterator predIt = myVehicles.begin();
    //     while (predIt != myVehicles.end()) {
    //         // get leader (may be zero) and follower
    //         // @todo compute secure position in regard to sublane-model
    //         const MSVehicle* leader = predIt != myVehicles.end() - 1 ? *(predIt + 1) : nullptr;
    //         if (leader == nullptr && myPartialVehicles.size() > 0) {
    //             leader = myPartialVehicles.front();
    //         }
    //         const MSVehicle* follower = *predIt;
    //
    //         // patch speed if allowed
    //         double speed = mspeed;
    //         if (leader != nullptr) {
    //             speed = MIN2(leader->getSpeed(), mspeed);
    //         }
    //
    //         // compute the space needed to not collide with leader
    //         double frontMax = maxPos;
    //         if (leader != nullptr) {
    //             double leaderRearPos = leader->getBackPositionOnLane(this);
    //             double frontGapNeeded = veh.getCarFollowModel().getSecureGap(&veh, leader, speed, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel()) + veh.getVehicleType().getMinGap();
    //             frontMax = MIN2(maxPos, leaderRearPos - frontGapNeeded);
    //         }
    //         // compute the space needed to not let the follower collide
    //         const double followPos = follower->getPositionOnLane() + follower->getVehicleType().getMinGap();
    //         const double backGapNeeded = follower->getCarFollowModel().getSecureGap(follower, &veh, follower->getSpeed(), veh.getSpeed(), veh.getCarFollowModel().getMaxDecel());
    //         const double backMin = followPos + backGapNeeded + veh.getVehicleType().getLength();
    //
    //         // check whether there is enough room (given some extra space for rounding errors)
    //         if (frontMax > minPos && backMin + POSITION_EPS < frontMax) {
    //             // try to insert vehicle (should be always ok)
    //             if (isInsertionSuccess(&veh, speed, backMin + POSITION_EPS, posLat, true, notification)) {
    //                 //std::cout << SIMTIME << " freeInsertion lane=" << getID() << " veh=" << veh.getID() << " @(393)\n";
    //                 return true;
    //             }
    //         }
    //         ++predIt;
    //     }
    //     // first check at lane's begin
    //     //std::cout << SIMTIME << " freeInsertion lane=" << getID() << " veh=" << veh.getID() << " fail final\n";
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn free_insertion(&mut self, veh: &mut MSVehicle, mspeed: f64, posLat: f64, notification: MSMoveReminder_Notification) -> bool {
        // try to insert teleporting vehicles fully on this lane
        let mut maxPos = self.myLength;
        if veh.has_stops() && veh.get_next_stop().edge == *veh.get_current_route_edge() {
            maxPos = 0f64.max(veh.get_next_stop().get_end_pos(veh));
        }
        let minPos = if notification == MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            maxPos.min(veh.get_vehicle_type().get_length())
        } else {
            0.
        };
        veh.set_tentative_lane_and_position(Ptr::from_mut(self), minPos, 0.);
        let vp = Ptr::from_mut(veh);
        if self.myVehicles.is_empty() {
            // ensure sufficient gap to followers on predecessor lanes
            let backOffset = minPos - veh.get_vehicle_type().get_length();
            let missingRearGap = self.get_missing_rear_gap(vp, backOffset, mspeed);
            if missingRearGap > 0. {
                if minPos + missingRearGap <= maxPos {
                    // @note. The rear gap is tailored to mspeed. If it changes due
                    // to a leader vehicle (on subsequent lanes) insertion will
                    // still fail.
                    return self.is_insertion_success(vp, mspeed, minPos + missingRearGap, posLat, true, notification);
                }
                return false;
            } else {
                return self.is_insertion_success(vp, mspeed, minPos, posLat, true, notification);
            }
        } else {
            // check whether the vehicle can be put behind the last one if there is such
            let leader = *self.myVehicles.last().unwrap(); // @todo reproduction of bogus old behavior. see #1961
            let leaderPos = leader.get_back_position_on_lane(Ptr::from_mut(self));
            let speed = leader.get_speed();
            let frontGapNeeded = veh.get_car_follow_model().get_secure_gap(vp, leader, speed, leader.get_speed(), leader.get_car_follow_model().get_max_decel()) + veh.get_vehicle_type().get_min_gap();
            if leaderPos >= frontGapNeeded {
                let tspeed = mspeed.min(veh.get_car_follow_model().insertion_follow_speed(vp, mspeed, frontGapNeeded, leader.get_speed(), leader.get_car_follow_model().get_max_decel(), leader));
                // check whether we can insert our vehicle behind the last vehicle on the lane
                if self.is_insertion_success(vp, tspeed, minPos, posLat, true, notification) {
                    return true;
                }
            }
        }
        // go through the lane, look for free positions (starting after the last vehicle)
        let mut predIt: usize = 0;
        while predIt < self.myVehicles.len() {
            // get leader (may be zero) and follower
            // @todo compute secure position in regard to sublane-model
            let mut leader = if predIt != self.myVehicles.len() - 1 { self.myVehicles[predIt + 1] } else { Ptr::default() };
            if leader.is_null() && !self.myPartialVehicles.is_empty() {
                leader = self.myPartialVehicles[0];
            }
            let follower = self.myVehicles[predIt];

            // patch speed if allowed
            let mut speed = mspeed;
            if !leader.is_null() {
                speed = leader.get_speed().min(mspeed);
            }

            // compute the space needed to not collide with leader
            let mut frontMax = maxPos;
            if !leader.is_null() {
                let leaderRearPos = leader.get_back_position_on_lane(Ptr::from_mut(self));
                let frontGapNeeded = veh.get_car_follow_model().get_secure_gap(vp, leader, speed, leader.get_speed(), leader.get_car_follow_model().get_max_decel()) + veh.get_vehicle_type().get_min_gap();
                frontMax = maxPos.min(leaderRearPos - frontGapNeeded);
            }
            // compute the space needed to not let the follower collide
            let followPos = follower.get_position_on_lane() + follower.get_vehicle_type().get_min_gap();
            let backGapNeeded = follower.get_car_follow_model().get_secure_gap(follower, vp, follower.get_speed(), veh.get_speed(), veh.get_car_follow_model().get_max_decel());
            let backMin = followPos + backGapNeeded + veh.get_vehicle_type().get_length();

            // check whether there is enough room (given some extra space for rounding errors)
            if frontMax > minPos && backMin + POSITION_EPS < frontMax {
                // try to insert vehicle (should be always ok)
                if self.is_insertion_success(vp, speed, backMin + POSITION_EPS, posLat, true, notification) {
                    return true;
                }
            }
            predIt += 1;
        }
        // first check at lane's begin
        false
    }

    /// C++: `void MSLane::forceVehicleInsertion(MSVehicle *, double, MSMoveReminder::Notification, double)`
    /// Inserts the given vehicle at the given position
    // void
    // MSLane::forceVehicleInsertion(MSVehicle* veh, double pos, MSMoveReminder::Notification notification, double posLat) {
    //     veh->updateBestLanes(true, this);
    //     bool dummy;
    //     const double speed = veh->hasDeparted() ? veh->getSpeed() : getDepartSpeed(*veh, dummy);
    //     incorporateVehicle(veh, pos, speed, posLat, find_if(myVehicles.begin(), myVehicles.end(), [&](MSVehicle * const v) {
    //         return v->getPositionOnLane() >= pos;
    //     }), notification);
    // }
    #[allow(unused_variables)]
    pub fn force_vehicle_insertion(&mut self, veh: Ptr<MSVehicle>, pos: f64, notification: MSMoveReminder_Notification, posLat: f64) {
        veh.m().update_best_lanes(true, Ptr::from_mut(self));
        let mut dummy = false;
        let speed = if veh.has_departed() { veh.get_speed() } else { self.get_depart_speed(veh.m(), &mut dummy) };
        let at = self.myVehicles.iter().position(|v| v.get_position_on_lane() >= pos).unwrap_or(self.myVehicles.len());
        self.incorporate_vehicle(veh, pos, speed, posLat, at, notification);
    }

    /// C++: `double MSLane::setPartialOccupation(MSVehicle *)` [virtual]
    /// Sets the information about a vehicle lapping into this lane
    // double
    // MSLane::setPartialOccupation(MSVehicle* v) {
    //     // multithreading: there are concurrent writes to myNeedsCollisionCheck but all of them should set it to true
    //     myNeedsCollisionCheck = true; // always check
    // #ifdef DEBUG_PARTIALS
    //     if (DEBUG_COND2(v)) {
    //         std::cout << SIMTIME << " setPartialOccupation. lane=" << getID() << " veh=" << v->getID() << "\n";
    //     }
    // #endif
    //     // XXX update occupancy here?
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myPartialOccupatorMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     //assert(std::find(myPartialVehicles.begin(), myPartialVehicles.end(), v) == myPartialVehicles.end());
    //     myPartialVehicles.push_back(v);
    //     return myLength;
    // }
    #[allow(unused_variables)]
    pub fn set_partial_occupation(&mut self, v: Ptr<MSVehicle>) -> f64 {
        // always check
        self.myNeedsCollisionCheck = true;
        self.myPartialVehicles.push(v);
        self.myLength
    }

    /// C++: `void MSLane::resetPartialOccupation(MSVehicle *)` [virtual]
    /// Removes the information about a vehicle lapping into this lane
    // void
    // MSLane::resetPartialOccupation(MSVehicle* v) {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myPartialOccupatorMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    // #ifdef DEBUG_PARTIALS
    //     if (DEBUG_COND2(v)) {
    //         std::cout << SIMTIME << " resetPartialOccupation. lane=" << getID() << " veh=" << v->getID() << "\n";
    //     }
    // #endif
    //     for (VehCont::iterator i = myPartialVehicles.begin(); i != myPartialVehicles.end(); ++i) {
    //         if (v == *i) {
    //             myPartialVehicles.erase(i);
    //             // XXX update occupancy here?
    //             //std::cout << "    removed from myPartialVehicles\n";
    //             return;
    //         }
    //     }
    //     // bluelight eqipped vehicle can teleport onto the intersection without using a connection
    //     assert(false || MSGlobals::gClearState || v->getLaneChangeModel().hasBlueLight());
    // }
    #[allow(unused_variables)]
    pub fn reset_partial_occupation(&mut self, v: Ptr<MSVehicle>) {
        if let Some(i) = self.myPartialVehicles.iter().position(|x| *x == v) {
            self.myPartialVehicles.remove(i);
            return;
        }
        // bluelight eqipped vehicle can teleport onto the intersection without using a connection
        debug_assert!(false || MSGlobals::clear_state() || v.get_lane_change_model_2().has_blue_light());
    }

    /// C++: `void MSLane::setManeuverReservation(MSVehicle *)` [virtual]
    /// Registers the lane change intentions (towards this lane) for the given vehicle
    // void
    // MSLane::setManeuverReservation(MSVehicle* v) {
    // #ifdef DEBUG_MANEUVER_RESERVATIONS
    //     if (DEBUG_COND2(v)) {
    //         std::cout << SIMTIME << " setManeuverReservation. lane=" << getID() << " veh=" << v->getID() << "\n";
    //     }
    // #endif
    //     myManeuverReservations.push_back(v);
    // }
    #[allow(unused_variables)]
    pub fn set_maneuver_reservation(&mut self, v: Ptr<MSVehicle>) {
        self.myManeuverReservations.push(v);
    }

    /// C++: `void MSLane::resetManeuverReservation(MSVehicle *)` [virtual]
    /// Unregisters a vehicle, which previously registered for maneuvering into this lane
    // void
    // MSLane::resetManeuverReservation(MSVehicle* v) {
    // #ifdef DEBUG_MANEUVER_RESERVATIONS
    //     if (DEBUG_COND2(v)) {
    //         std::cout << SIMTIME << " resetManeuverReservation(): lane=" << getID() << " veh=" << v->getID() << "\n";
    //     }
    // #endif
    //     for (VehCont::iterator i = myManeuverReservations.begin(); i != myManeuverReservations.end(); ++i) {
    //         if (v == *i) {
    //             myManeuverReservations.erase(i);
    //             return;
    //         }
    //     }
    //     assert(false);
    // }
    #[allow(unused_variables)]
    pub fn reset_maneuver_reservation(&mut self, v: Ptr<MSVehicle>) {
        if let Some(i) = self.myManeuverReservations.iter().position(|x| *x == v) {
            self.myManeuverReservations.remove(i);
            return;
        }
        debug_assert!(false);
    }

    /// C++: `const MSLeaderInfo MSLane::getLastVehicleInformation(const MSVehicle *, double, double, bool, const MSVehicle *) const` [const]
    /// Returns the last vehicles on the lane
    // const MSLeaderInfo
    // MSLane::getLastVehicleInformation(const MSVehicle* ego, double latOffset, double minPos, bool allowCached, const MSVehicle* ignore) const {
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND2(ego) || DEBUG_COND) {
    //         std::cout << "    getLastVehicleInformation lane=" << getID() << " ego=" << Named::getIDSecure(ego) << " latOffset=" << latOffset << " minPos=" << minPos << " allowCached=" << allowCached
    //                   << " hasCache=" << (myLeaderInfoTime >= MSNet::getInstance()->getCurrentTimeStep()) << "\n";
    //     }
    // #endif
    //     if (myLeaderInfoTime < MSNet::getInstance()->getCurrentTimeStep() || ego != nullptr || minPos > 0 || !allowCached) {
    //         MSLeaderInfo leaderTmp(myWidth, ego, latOffset);
    //         AnyVehicleIterator last = anyVehiclesBegin();
    //         int freeSublanes = 1; // number of sublanes for which no leader was found
    //         //if (ego->getID() == "disabled" && SIMTIME == 58) {
    //         //    std::cout << "DEBUG\n";
    //         //}
    //         const MSVehicle* veh = *last;
    //         while (freeSublanes > 0 && veh != nullptr) {
    // #ifdef DEBUG_PLAN_MOVE
    //             if (DEBUG_COND2(ego) || DEBUG_COND) {
    //                 gDebugFlag1 = true;
    //                 std::cout << "      getLastVehicleInformation lane=" << getID() << " minPos=" << minPos << " veh=" << veh->getID() << " pos=" << veh->getPositionOnLane(this)  << "\n";
    //             }
    // #endif
    //             if (veh != ego && veh != ignore && MAX2(0.0, veh->getPositionOnLane(this)) >= minPos) {
    //                 const double vehLatOffset = veh->getLatOffset(this);
    //                 freeSublanes = leaderTmp.addLeader(veh, true, vehLatOffset);
    // #ifdef DEBUG_PLAN_MOVE
    //                 if (DEBUG_COND2(ego) || DEBUG_COND) {
    //                     std::cout << "         latOffset=" << vehLatOffset << " newLeaders=" << leaderTmp.toString() << "\n";
    //                 }
    // #endif
    //             }
    //             veh = *(++last);
    //         }
    //         if (ego == nullptr && minPos == 0) {
    // #ifdef HAVE_FOX
    //             ScopedLocker<> lock(myLeaderInfoMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //             // update cached value
    //             myLeaderInfo = leaderTmp;
    //             myLeaderInfoTime = MSNet::getInstance()->getCurrentTimeStep();
    //         }
    // #ifdef DEBUG_PLAN_MOVE
    //         //if (DEBUG_COND2(ego)) std::cout << SIMTIME
    //         //    << " getLastVehicleInformation lane=" << getID()
    //         //        << " ego=" << Named::getIDSecure(ego)
    //         //        << "\n"
    //         //        << "    vehicles=" << toString(myVehicles)
    //         //        << "    partials=" << toString(myPartialVehicles)
    //         //        << "\n"
    //         //        << "    result=" << leaderTmp.toString()
    //         //        << "    cached=" << myLeaderInfo.toString()
    //         //        << "    myLeaderInfoTime=" << myLeaderInfoTime
    //         //        << "\n";
    //         gDebugFlag1 = false;
    // #endif
    //         return leaderTmp;
    //     }
    //     return myLeaderInfo;
    // }
    #[allow(unused_variables)]
    pub fn get_last_vehicle_information(&self, ego: Ptr<MSVehicle>, latOffset: f64, minPos: f64, allowCached: bool, ignore: Ptr<MSVehicle>) -> MSLeaderInfo {
        if self.myLeaderInfoTime.get() < MSNet::get_instance().get_current_time_step() || !ego.is_null() || minPos > 0.0 || !allowCached {
            let mut leaderTmp = MSLeaderInfo::new(self.myWidth, ego, latOffset);
            let mut last = self.any_vehicles_begin();
            let mut freeSublanes: i32 = 1; // number of sublanes for which no leader was found
            let mut veh = last.mul_op();
            while freeSublanes > 0 && !veh.is_null() {
                if veh != ego && veh != ignore && f64::max(0.0, veh.get_position_on_lane_2(Ptr::new(self))) >= minPos {
                    let vehLatOffset = veh.get_lat_offset(Ptr::new(self));
                    freeSublanes = leaderTmp.add_leader(veh, true, vehLatOffset);
                }
                veh = last.op__2().mul_op();
            }
            if ego.is_null() && minPos == 0.0 {
                // update cached value
                *self.myLeaderInfo.borrow_mut() = leader_info_copy(&leaderTmp);
                self.myLeaderInfoTime.set(MSNet::get_instance().get_current_time_step());
            }
            return leaderTmp;
        }
        leader_info_copy(&self.myLeaderInfo.borrow())
    }

    /// C++: `const MSLeaderInfo MSLane::getFirstVehicleInformation(const MSVehicle *, double, bool, double, bool) const` [const]
    /// analogue to getLastVehicleInformation but in the upstream direction
    // const MSLeaderInfo
    // MSLane::getFirstVehicleInformation(const MSVehicle* ego, double latOffset, bool onlyFrontOnLane, double maxPos, bool allowCached) const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myFollowerInfoMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     if (myFollowerInfoTime < MSNet::getInstance()->getCurrentTimeStep() || ego != nullptr || maxPos < myLength || !allowCached || onlyFrontOnLane) {
    //         // XXX separate cache for onlyFrontOnLane = true
    //         MSLeaderInfo followerTmp(myWidth, ego, latOffset);
    //         AnyVehicleIterator first = anyVehiclesUpstreamBegin();
    //         int freeSublanes = 1; // number of sublanes for which no leader was found
    //         const MSVehicle* veh = *first;
    //         while (freeSublanes > 0 && veh != nullptr) {
    // #ifdef DEBUG_PLAN_MOVE
    //             if (DEBUG_COND2(ego)) {
    //                 std::cout << "       veh=" << veh->getID() << " pos=" << veh->getPositionOnLane(this) << " maxPos=" << maxPos << "\n";
    //             }
    // #endif
    //             if (veh != ego && veh->getPositionOnLane(this) <= maxPos
    //                     && (!onlyFrontOnLane || veh->isFrontOnLane(this))) {
    //                 //const double vehLatOffset = veh->getLane()->getRightSideOnEdge() - getRightSideOnEdge();
    //                 const double vehLatOffset = veh->getLatOffset(this);
    // #ifdef DEBUG_PLAN_MOVE
    //                 if (DEBUG_COND2(ego)) {
    //                     std::cout << "          veh=" << veh->getID() << " latOffset=" << vehLatOffset << "\n";
    //                 }
    // #endif
    //                 freeSublanes = followerTmp.addLeader(veh, true, vehLatOffset);
    //             }
    //             veh = *(++first);
    //         }
    //         if (ego == nullptr && maxPos == std::numeric_limits<double>::max()) {
    //             // update cached value
    //             myFollowerInfo = followerTmp;
    //             myFollowerInfoTime = MSNet::getInstance()->getCurrentTimeStep();
    //         }
    // #ifdef DEBUG_PLAN_MOVE
    //         //if (DEBUG_COND2(ego)) std::cout << SIMTIME
    //         //    << " getFirstVehicleInformation lane=" << getID()
    //         //        << " ego=" << Named::getIDSecure(ego)
    //         //        << "\n"
    //         //        << "    vehicles=" << toString(myVehicles)
    //         //        << "    partials=" << toString(myPartialVehicles)
    //         //        << "\n"
    //         //        << "    result=" << followerTmp.toString()
    //         //        //<< "    cached=" << myFollowerInfo.toString()
    //         //        << "    myLeaderInfoTime=" << myLeaderInfoTime
    //         //        << "\n";
    // #endif
    //         return followerTmp;
    //     }
    //     return myFollowerInfo;
    // }
    #[allow(unused_variables)]
    pub fn get_first_vehicle_information(&self, ego: Ptr<MSVehicle>, latOffset: f64, onlyFrontOnLane: bool, maxPos: f64, allowCached: bool) -> MSLeaderInfo {
        if self.myFollowerInfoTime.get() < MSNet::get_instance().get_current_time_step() || !ego.is_null() || maxPos < self.myLength || !allowCached || onlyFrontOnLane {
            let mut followerTmp = MSLeaderInfo::new(self.myWidth, ego, latOffset);
            let mut first = self.any_vehicles_upstream_begin();
            let mut freeSublanes: i32 = 1; // number of sublanes for which no leader was found
            let mut veh = first.mul_op();
            while freeSublanes > 0 && !veh.is_null() {
                if veh != ego && veh.get_position_on_lane_2(Ptr::new(self)) <= maxPos && (!onlyFrontOnLane || veh.is_front_on_lane(Ptr::new(self))) {
                    let vehLatOffset = veh.get_lat_offset(Ptr::new(self));
                    freeSublanes = followerTmp.add_leader(veh, true, vehLatOffset);
                }
                veh = first.op__2().mul_op();
            }
            if ego.is_null() && maxPos == f64::MAX {
                // update cached value
                *self.myFollowerInfo.borrow_mut() = leader_info_copy(&followerTmp);
                self.myFollowerInfoTime.set(MSNet::get_instance().get_current_time_step());
            }
            return followerTmp;
        }
        leader_info_copy(&self.myFollowerInfo.borrow())
    }

    /// C++: `int MSLane::getVehicleNumber() const` [const]
    /// Returns the number of vehicles on this lane (for which this lane is responsible)
    // int getVehicleNumber() const {
    //         return (int)myVehicles.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_number(&self) -> i32 {
        self.myVehicles.len() as i32
    }

    /// C++: `int MSLane::getVehicleNumberWithPartials() const` [const]
    /// Returns the number of vehicles on this lane (including partial occupators)
    // int getVehicleNumberWithPartials() const {
    //         return (int)myVehicles.size() + (int)myPartialVehicles.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_number_with_partials(&self) -> i32 {
        self.myVehicles.len() as i32 + self.myPartialVehicles.len() as i32
    }

    /// C++: `int MSLane::getPartialVehicleNumber() const` [const]
    /// Returns the number of vehicles partially on this lane (for which this lane is not responsible)
    // int getPartialVehicleNumber() const {
    //         return (int)myPartialVehicles.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_vehicle_number(&self) -> i32 {
        self.myPartialVehicles.len() as i32
    }

    /// C++: `const VehCont & MSLane::getVehiclesSecure() const` [const, virtual]
    /// Returns the vehicles container; locks it for microsimulation
    // virtual const VehCont& getVehiclesSecure() const {
    //         return myVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicles_secure(&self) -> &MSLane_VehCont {
        &self.myVehicles
    }

    /// C++: `AnyVehicleIterator MSLane::anyVehiclesBegin() const` [const]
    /// begin iterator for iterating over all vehicles touching this lane in downstream direction
    // AnyVehicleIterator anyVehiclesBegin() const {
    //         return AnyVehicleIterator(this, 0, 0, 0,
    //                                   (int)myVehicles.size(), (int)myPartialVehicles.size(), (int)myTmpVehicles.size(), true);
    //     }
    #[allow(unused_variables)]
    pub fn any_vehicles_begin(&self) -> MSLane_AnyVehicleIterator {
        MSLane_AnyVehicleIterator::new(Ptr::new(self), 0, 0, 0, self.myVehicles.len() as i32, self.myPartialVehicles.len() as i32, self.myTmpVehicles.len() as i32, true)
    }

    /// C++: `AnyVehicleIterator MSLane::anyVehiclesEnd() const` [const]
    /// end iterator for iterating over all vehicles touching this lane in downstream direction
    // AnyVehicleIterator anyVehiclesEnd() const {
    //         return AnyVehicleIterator(this, (int)myVehicles.size(), (int)myPartialVehicles.size(), (int)myTmpVehicles.size(),
    //                                   (int)myVehicles.size(), (int)myPartialVehicles.size(), (int)myTmpVehicles.size(), true);
    //     }
    #[allow(unused_variables)]
    pub fn any_vehicles_end(&self) -> MSLane_AnyVehicleIterator {
        let (n1, n2, n3) = (self.myVehicles.len() as i32, self.myPartialVehicles.len() as i32, self.myTmpVehicles.len() as i32);
        MSLane_AnyVehicleIterator::new(Ptr::new(self), n1, n2, n3, n1, n2, n3, true)
    }

    /// C++: `AnyVehicleIterator MSLane::anyVehiclesUpstreamBegin() const` [const]
    /// begin iterator for iterating over all vehicles touching this lane in upstream direction
    // AnyVehicleIterator anyVehiclesUpstreamBegin() const {
    //         return AnyVehicleIterator(this, (int)myVehicles.size() - 1, (int)myPartialVehicles.size() - 1, (int)myTmpVehicles.size() - 1,
    //                                   -1, -1, -1, false);
    //     }
    #[allow(unused_variables)]
    pub fn any_vehicles_upstream_begin(&self) -> MSLane_AnyVehicleIterator {
        MSLane_AnyVehicleIterator::new(Ptr::new(self), self.myVehicles.len() as i32 - 1, self.myPartialVehicles.len() as i32 - 1, self.myTmpVehicles.len() as i32 - 1, -1, -1, -1, false)
    }

    /// C++: `AnyVehicleIterator MSLane::anyVehiclesUpstreamEnd() const` [const]
    /// end iterator for iterating over all vehicles touching this lane in upstream direction
    // AnyVehicleIterator anyVehiclesUpstreamEnd() const {
    //         return AnyVehicleIterator(this, -1, -1, -1, -1, -1, -1, false);
    //     }
    #[allow(unused_variables)]
    pub fn any_vehicles_upstream_end(&self) -> MSLane_AnyVehicleIterator {
        MSLane_AnyVehicleIterator::new(Ptr::new(self), -1, -1, -1, -1, -1, -1, false)
    }

    /// C++: `void MSLane::releaseVehicles() const` [const, virtual]
    /// Allows to use the container for microsimulation again
    // virtual void releaseVehicles() const { }
    #[allow(unused_variables)]
    pub fn release_vehicles(&self) {
        // C++ body is empty: the vehicle container is not locked in this port
    }

    /// C++: `int MSLane::getNumericalID() const` [const]
    /// Returns this lane's numerical id
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myNumericalID
    }

    /// C++: `const PositionVector & MSLane::getShape() const` [const]
    /// Returns this lane's shape
    // inline const PositionVector& getShape() const {
    //         return myShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape_2(&self) -> &PositionVector {
        &self.myShape
    }

    /// C++: `double MSLane::getLengthGeometryFactor() const` [const]
    /// return shape.length() / myLength
    // inline double getLengthGeometryFactor() const {
    //         return myLengthGeometryFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor_2(&self) -> f64 {
        self.myLengthGeometryFactor
    }

    /// C++: `bool MSLane::isAccelLane() const` [const]
    /// return whether this lane is an acceleration lane
    // inline bool isAccelLane() const {
    //         return myIsRampAccel;
    //     }
    #[allow(unused_variables)]
    pub fn is_accel_lane(&self) -> bool {
        self.myIsRampAccel
    }

    /// C++: `const std::string & MSLane::getLaneType() const` [const]
    /// return the type of this lane
    // const std::string& getLaneType() const {
    //         return myLaneType;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_type(&self) -> &str {
        &self.myLaneType
    }

    /// C++: `double MSLane::interpolateLanePosToGeometryPos(double) const` [const]
    // inline double interpolateLanePosToGeometryPos(double lanePos) const {
    //         return lanePos * myLengthGeometryFactor;
    //     }
    #[allow(unused_variables)]
    pub fn interpolate_lane_pos_to_geometry_pos(&self, lanePos: f64) -> f64 {
        lanePos * self.myLengthGeometryFactor
    }

    /// C++: `const Position MSLane::geometryPositionAtOffset(double, double) const` [const]
    // inline const Position geometryPositionAtOffset(double offset, double lateralOffset = 0) const {
    //         return myShape.positionAtOffset(interpolateLanePosToGeometryPos(offset), lateralOffset);
    //     }
    #[allow(unused_variables)]
    pub fn geometry_position_at_offset(&self, offset: f64, lateralOffset: f64) -> Position {
        self.myShape.position_at_offset(self.interpolate_lane_pos_to_geometry_pos(offset), lateralOffset)
    }

    /// C++: `double MSLane::interpolateGeometryPosToLanePos(double) const` [const]
    // inline double interpolateGeometryPosToLanePos(double geometryPos) const {
    //         return geometryPos / myLengthGeometryFactor;
    //     }
    #[allow(unused_variables)]
    pub fn interpolate_geometry_pos_to_lane_pos(&self, geometryPos: f64) -> f64 {
        geometryPos / self.myLengthGeometryFactor
    }

    /// C++: `double MSLane::getVehicleMaxSpeed(const SUMOTrafficObject *const) const` [const]
    /// Returns the lane's maximum speed, given a vehicle's speed limit adaptation
    // inline double getVehicleMaxSpeed(const SUMOTrafficObject* const veh) const {
    //         return getVehicleMaxSpeed(veh, veh->getMaxSpeed());
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_max_speed(&self, veh: Ptr<SUMOTrafficObject>) -> f64 {
        self.get_vehicle_max_speed_2(veh, veh.get_max_speed())
    }

    /// C++: `double MSLane::getVehicleMaxSpeed(const SUMOTrafficObject *const, double) const` [const]
    // inline double getVehicleMaxSpeed(const SUMOTrafficObject* const veh, double vehMaxSpeed) const {
    //         if (myRestrictions != nullptr) {
    //             std::map<SUMOVehicleClass, double>::const_iterator r = myRestrictions->find(veh->getVClass());
    //             if (r != myRestrictions->end()) {
    //                 if (mySpeedModified) {
    //                     return MIN2(myMaxSpeed, MIN2(vehMaxSpeed, r->second * veh->getChosenSpeedFactor()));
    //                 } else {
    //                     return MIN2(vehMaxSpeed, r->second * veh->getChosenSpeedFactor());
    //                 }
    //             }
    //         }
    //         return MIN2(vehMaxSpeed, myMaxSpeed * veh->getChosenSpeedFactor());
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_max_speed_2(&self, veh: Ptr<SUMOTrafficObject>, vehMaxSpeed: f64) -> f64 {
        if !self.myRestrictions.is_null() {
            if let Some(r) = self.myRestrictions.m().get(&veh.get_v_class()) {
                if self.mySpeedModified {
                    return self.myMaxSpeed.min(vehMaxSpeed.min(*r * veh.get_chosen_speed_factor()));
                } else {
                    return vehMaxSpeed.min(*r * veh.get_chosen_speed_factor());
                }
            }
        }
        vehMaxSpeed.min(self.myMaxSpeed * veh.get_chosen_speed_factor())
    }

    /// C++: `bool MSLane::isSpeedModified() const` [const]
    // inline bool isSpeedModified() const {
    //         return mySpeedModified;
    //     }
    #[allow(unused_variables)]
    pub fn is_speed_modified(&self) -> bool {
        self.mySpeedModified
    }

    /// C++: `double MSLane::getSpeedLimit() const` [const]
    /// Returns the lane's maximum allowed speed
    // inline double getSpeedLimit() const {
    //         return myMaxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        self.myMaxSpeed
    }

    /// C++: `double MSLane::getSpeedLimit(SUMOVehicleClass) const` [const]
    // inline double getSpeedLimit(SUMOVehicleClass svc) const {
    //         if (myRestrictions != nullptr) {
    //             std::map<SUMOVehicleClass, double>::const_iterator r = myRestrictions->find(svc);
    //             if (r != myRestrictions->end()) {
    //                 if (mySpeedModified) {
    //                     return MIN2(myMaxSpeed, r->second);
    //                 } else {
    //                     return r->second;
    //                 }
    //             }
    //         }
    //         return myMaxSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit_2(&self, svc: SUMOVehicleClass) -> f64 {
        if !self.myRestrictions.is_null() {
            if let Some(r) = self.myRestrictions.m().get(&svc) {
                if self.mySpeedModified {
                    return self.myMaxSpeed.min(*r);
                } else {
                    return *r;
                }
            }
        }
        self.myMaxSpeed
    }

    /// C++: `double MSLane::getFrictionCoefficient() const` [const]
    /// Returns the lane's friction coefficient
    // inline double getFrictionCoefficient() const {
    //         return myFrictionCoefficient;
    //     }
    #[allow(unused_variables)]
    pub fn get_friction_coefficient(&self) -> f64 {
        self.myFrictionCoefficient
    }

    /// C++: `double MSLane::getLength() const` [const]
    /// Returns the lane's length
    // inline double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `SVCPermissions MSLane::getPermissions() const` [const]
    /// Returns the vehicle class permissions for this lane
    // inline SVCPermissions getPermissions() const {
    //         return myPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        self.myPermissions
    }

    /// C++: `SVCPermissions MSLane::getChangeLeft() const` [const]
    /// Returns the vehicle class permissions for changing to the left neighbour lane
    // inline SVCPermissions getChangeLeft() const {
    //         return myChangeLeft;
    //     }
    #[allow(unused_variables)]
    pub fn get_change_left(&self) -> SVCPermissions {
        self.myChangeLeft
    }

    /// C++: `SVCPermissions MSLane::getChangeRight() const` [const]
    /// Returns the vehicle class permissions for changing to the right neighbour lane
    // inline SVCPermissions getChangeRight() const {
    //         return myChangeRight;
    //     }
    #[allow(unused_variables)]
    pub fn get_change_right(&self) -> SVCPermissions {
        self.myChangeRight
    }

    /// C++: `double MSLane::getWidth() const` [const]
    /// Returns the lane's width
    // double getWidth() const {
    //         return myWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        self.myWidth
    }

    /// C++: `int MSLane::getIndex() const` [const]
    /// Returns the lane's index
    // int getIndex() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_index(&self) -> i32 {
        self.myIndex
    }

    /// C++: `int MSLane::getCrossingIndex() const` [const]
    /// return the index of the link to the next crossing if this is walkingArea, else -1
    // int
    // MSLane::getCrossingIndex() const {
    //     for (std::vector<MSLink*>::const_iterator i = myLinks.begin(); i != myLinks.end(); ++i) {
    //         if ((*i)->getLane()->isCrossing()) {
    //             return (int)(i - myLinks.begin());
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_crossing_index(&self) -> i32 {
        for (i, link) in self.myLinks.iter().enumerate() {
            if link.get_lane().is_crossing() {
                return i as i32;
            }
        }
        -1
    }

    /// C++: `void MSLane::planMovements(const SUMOTime)` [virtual]
    /// Compute safe velocities for all vehicles based on positions and speeds from the last time step. Also registers ApproachingVehicleInformation for all links
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn plan_movements(&mut self, t: SUMOTime) {
        debug_assert!(self.myVehicles.len() != 0);
        let mut cumulatedVehLength: f64 = 0.;
        let mut leaders = MSLeaderInfo::new(self.myWidth, Ptr::null(), 0.);

        // iterate over myVehicles, myPartialVehicles, and myManeuverReservations merge-sort style
        // reverse iterators are represented by the number of elements already consumed from the back
        // shortcut: assert(MSGlobals::gLateralResolution || myManeuverReservations.size() == 0) skipped, gLateralResolution not ported
        let mut veh: usize = 0;
        let mut vehPart: usize = 0;
        let mut vehRes: usize = 0;
        while veh < self.myVehicles.len() {
            let v = self.myVehicles[self.myVehicles.len() - 1 - veh];
            self.update_leader_info(v, &mut vehPart, &mut vehRes, &mut leaders); // 36ns with 8 threads, 9ns with 1
            v.m().plan_move(t, &leaders, cumulatedVehLength); // 4800ns with 8 threads, 3100 with 1
            cumulatedVehLength += v.get_vehicle_type().get_length_with_gap();
            leaders.add_leader(v, false, 0.);
            veh += 1;
        }
    }

    /// C++: `void MSLane::setJunctionApproaches() const` [const, virtual]
    /// Register junction approaches for all vehicles after velocities have been planned.
    // void
    // MSLane::setJunctionApproaches() const {
    //     for (MSVehicle* const veh : myVehicles) {
    //         veh->setApproachingForAllLinks();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_approaches(&self) {
        for veh in self.myVehicles.iter() {
            veh.m().set_approaching_for_all_links();
        }
    }

    /// C++: `void MSLane::updateLeaderInfo(const MSVehicle *, VehCont::reverse_iterator &, VehCont::reverse_iterator &, MSLeaderInfo &) const` [const]
    /// This updates the MSLeaderInfo argument with respect to the given MSVehicle. All leader-vehicles on the same edge, which are relevant for the vehicle (i.e. with position > vehicle's position) and not already integrated into the LeaderInfo, are integrated. The given iterators vehPart and vehRes give access to these vehicles which are either partial occupators or have issued a maneuver reservation for the lane (the latter occurs only for the sublane model).
    // void
    // MSLane::updateLeaderInfo(const MSVehicle* veh, VehCont::reverse_iterator& vehPart, VehCont::reverse_iterator& vehRes, MSLeaderInfo& ahead) const {
    //     bool morePartialVehsAhead = vehPart != myPartialVehicles.rend();
    //     bool moreReservationsAhead = vehRes != myManeuverReservations.rend();
    //     bool nextToConsiderIsPartial;
    //
    //     // Determine relevant leaders for veh
    //     while (moreReservationsAhead || morePartialVehsAhead) {
    //         if ((!moreReservationsAhead || (*vehRes)->getPositionOnLane(this) <= veh->getPositionOnLane())
    //                 && (!morePartialVehsAhead || (*vehPart)->getPositionOnLane(this) <= veh->getPositionOnLane())) {
    //             // All relevant downstream vehicles have been collected.
    //             break;
    //         }
    //
    //         // Check whether next farthest relevant vehicle downstream is a partial vehicle or a maneuver reservation
    //         if (moreReservationsAhead && !morePartialVehsAhead) {
    //             nextToConsiderIsPartial = false;
    //         } else if (morePartialVehsAhead && !moreReservationsAhead) {
    //             nextToConsiderIsPartial = true;
    //         } else {
    //             assert(morePartialVehsAhead && moreReservationsAhead);
    //             // Add farthest downstream vehicle first
    //             nextToConsiderIsPartial = (*vehPart)->getPositionOnLane(this) > (*vehRes)->getPositionOnLane(this);
    //         }
    //         // Add appropriate leader information
    //         if (nextToConsiderIsPartial) {
    //             const double latOffset = (*vehPart)->getLatOffset(this);
    // #ifdef DEBUG_PLAN_MOVE
    //             if (DEBUG_COND) {
    //                 std::cout << "        partial ahead: " << (*vehPart)->getID() << " latOffset=" << latOffset << "\n";
    //             }
    // #endif
    //             if (!(MSGlobals::gLaneChangeDuration > 0 && (*vehPart)->getLaneChangeModel().isOpposite()
    //                     && !(*vehPart)->getLaneChangeModel().isChangingLanes())) {
    //                 ahead.addLeader(*vehPart, false, latOffset);
    //             }
    //             ++vehPart;
    //             morePartialVehsAhead = vehPart != myPartialVehicles.rend();
    //         } else {
    //             const double latOffset = (*vehRes)->getLatOffset(this);
    // #ifdef DEBUG_PLAN_MOVE
    //             if (DEBUG_COND) {
    //                 std::cout << "    reservation ahead: " << (*vehRes)->getID() << " latOffset=" << latOffset << "\n";
    //             }
    // #endif
    //             ahead.addLeader(*vehRes, false, latOffset);
    //             ++vehRes;
    //             moreReservationsAhead = vehRes != myManeuverReservations.rend();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_leader_info(&self, veh: Ptr<MSVehicle>, vehPart: &mut usize /* VehCont::reverse_iterator: count consumed from the back */, vehRes: &mut usize /* VehCont::reverse_iterator: count consumed from the back */, ahead: &mut MSLeaderInfo) {
        let nPart = self.myPartialVehicles.len();
        let nRes = self.myManeuverReservations.len();
        let mut morePartialVehsAhead = *vehPart != nPart;
        let mut moreReservationsAhead = *vehRes != nRes;
        let mut nextToConsiderIsPartial: bool;

        // Determine relevant leaders for veh
        while moreReservationsAhead || morePartialVehsAhead {
            // reverse iterator *it refers to index len-1-consumed
            let partVeh: Option<Ptr<MSVehicle>> = if morePartialVehsAhead { Some(self.myPartialVehicles[nPart - 1 - *vehPart]) } else { None };
            let resVeh: Option<Ptr<MSVehicle>> = if moreReservationsAhead { Some(self.myManeuverReservations[nRes - 1 - *vehRes]) } else { None };
            if (resVeh.map_or(true, |r| r.get_position_on_lane_2(Ptr::new(self)) <= veh.get_position_on_lane()))
                && (partVeh.map_or(true, |p| p.get_position_on_lane_2(Ptr::new(self)) <= veh.get_position_on_lane()))
            {
                // All relevant downstream vehicles have been collected.
                break;
            }

            // Check whether next farthest relevant vehicle downstream is a partial vehicle or a maneuver reservation
            if moreReservationsAhead && !morePartialVehsAhead {
                nextToConsiderIsPartial = false;
            } else if morePartialVehsAhead && !moreReservationsAhead {
                nextToConsiderIsPartial = true;
            } else {
                // Add farthest downstream vehicle first
                nextToConsiderIsPartial = partVeh.unwrap().get_position_on_lane_2(Ptr::new(self)) > resVeh.unwrap().get_position_on_lane_2(Ptr::new(self));
            }

            // Add appropriate leader information
            if nextToConsiderIsPartial {
                let part = partVeh.unwrap();
                let latOffset = part.get_lat_offset(Ptr::new(self));
                if !(MSGlobals::lane_change_duration() > 0 && part.get_lane_change_model_2().is_opposite() && !part.get_lane_change_model_2().is_changing_lanes()) {
                    ahead.add_leader(part, false, latOffset);
                }
                *vehPart += 1;
                morePartialVehsAhead = *vehPart != nPart;
            } else {
                let res = resVeh.unwrap();
                let latOffset = res.get_lat_offset(Ptr::new(self));
                ahead.add_leader(res, false, latOffset);
                *vehRes += 1;
                moreReservationsAhead = *vehRes != nRes;
            }
        }
    }

    /// C++: `void MSLane::executeMovements(const SUMOTime)` [virtual]
    /// Executes planned vehicle movements with regards to right-of-way
    // void
    // MSLane::executeMovements(const SUMOTime t) {
    //     // multithreading: there are concurrent writes to myNeedsCollisionCheck but all of them should set it to true
    //     myNeedsCollisionCheck = true;
    //     MSLane* bidi = getBidiLane();
    //     if (bidi != nullptr && bidi->getVehicleNumber() == 0) {
    //         MSNet::getInstance()->getEdgeControl().checkCollisionForInactive(bidi);
    //     }
    //     MSVehicle* firstNotStopped = nullptr;
    //     // iterate over vehicles in reverse so that move reminders will be called in the correct order
    //     for (VehCont::reverse_iterator i = myVehicles.rbegin(); i != myVehicles.rend();) {
    //         MSVehicle* veh = *i;
    //         // length is needed later when the vehicle may not exist anymore
    //         const double length = veh->getVehicleType().getLengthWithGap();
    //         const double nettoLength = veh->getVehicleType().getLength();
    //         const bool moved = veh->executeMove();
    //         MSLane* const target = veh->getMutableLane();
    //         if (veh->hasArrived()) {
    //             // vehicle has reached its arrival position
    // #ifdef DEBUG_EXEC_MOVE
    //             if DEBUG_COND2(veh) {
    //                 std::cout << SIMTIME << " veh " << veh->getID() << " has arrived." << std::endl;
    //             }
    // #endif
    //             veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_ARRIVED);
    //             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         } else if (target != nullptr && moved) {
    //             if (target->getEdge().isVaporizing()) {
    //                 // vehicle has reached a vaporizing edge
    //                 veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_VAPORIZED_VAPORIZER);
    //                 MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //             } else {
    //                 // vehicle has entered a new lane (leaveLane and workOnMoveReminders were already called in MSVehicle::executeMove)
    //                 target->myVehBuffer.push_back(veh);
    //                 MSNet::getInstance()->getEdgeControl().needsVehicleIntegration(target);
    //                 if (MSGlobals::gSublane && veh->getLaneChangeModel().getShadowLane() != nullptr) {
    //                     // trigger sorting of partial vehicles as their order may have changed (lane might not be active and only contain partial vehicles)
    //                     MSNet::getInstance()->getEdgeControl().needsVehicleIntegration(veh->getLaneChangeModel().getShadowLane());
    //                 }
    //             }
    //         } else if (veh->isParking()) {
    //             // vehicle started to park
    //             MSVehicleTransfer::getInstance()->add(t, veh);
    //             myParkingVehicles.insert(veh);
    //         } else if (veh->brokeDown()) {
    //             veh->resumeFromStopping();
    //             WRITE_WARNINGF(TL("Removing vehicle '%' after breaking down, lane='%', time=%."),
    //                            veh->getID(), veh->getLane()->getID(), time2string(t));
    //             veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_VAPORIZED_BREAKDOWN);
    //             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         } else if (veh->isJumping()) {
    //             // vehicle jumps to next route edge
    //             MSVehicleTransfer::getInstance()->add(t, veh);
    //         } else if (veh->getPositionOnLane() > getLength()) {
    //             // for any reasons the vehicle is beyond its lane...
    //             // this should never happen because it is handled in MSVehicle::executeMove
    //             assert(false);
    //             WRITE_WARNINGF(TL("Teleporting vehicle '%'; beyond end of lane, target lane='%', time=%."),
    //                            veh->getID(), getID(), time2string(t));
    //             MSNet::getInstance()->getVehicleControl().countCollision(true);
    //             MSVehicleTransfer::getInstance()->add(t, veh);
    //
    //         } else if (veh->collisionStopTime() == 0) {
    //             veh->resumeFromStopping();
    //             if (getCollisionAction() == COLLISION_ACTION_REMOVE) {
    //                 WRITE_WARNINGF(TL("Removing vehicle '%' after earlier collision, lane='%', time=%."),
    //                                veh->getID(), veh->getLane()->getID(), time2string(t));
    //                 veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_VAPORIZED_COLLISION);
    //                 MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //             } else if (getCollisionAction() == COLLISION_ACTION_TELEPORT) {
    //                 WRITE_WARNINGF(TL("Teleporting vehicle '%' after earlier collision, lane='%', time=%."),
    //                                veh->getID(), veh->getLane()->getID(), time2string(t));
    //                 MSVehicleTransfer::getInstance()->add(t, veh);
    //             } else {
    //                 if (firstNotStopped == nullptr && !(*i)->isStopped() && (*i)->getLane() == this) {
    //                     firstNotStopped = *i;
    //                 }
    //                 ++i;
    //                 continue;
    //             }
    //         } else {
    //             if (firstNotStopped == nullptr && !(*i)->isStopped() && (*i)->getLane() == this) {
    //                 firstNotStopped = *i;
    //             }
    //             ++i;
    //             continue;
    //         }
    //         myBruttoVehicleLengthSumToRemove += length;
    //         myNettoVehicleLengthSumToRemove += nettoLength;
    //         ++i;
    //         i = VehCont::reverse_iterator(myVehicles.erase(i.base()));
    //     }
    //     if (firstNotStopped != nullptr) {
    //         const SUMOTime ttt = firstNotStopped->getVehicleType().getParameter().getTimeToTeleport(MSGlobals::gTimeToGridlock);
    //         const SUMOTime tttb = firstNotStopped->getVehicleType().getParameter().getTimeToTeleportBidi(MSGlobals::gTimeToTeleportBidi);
    //         if (ttt > 0 || MSGlobals::gTimeToGridlockHighways > 0 || MSGlobals::gTimeToTeleportDisconnected >= 0 || tttb > 0 || MSGlobals::gTimeToTeleportRSDeadlock > 0) {
    //             const bool wrongLane = !appropriate(firstNotStopped);
    //             const bool disconnected = (MSGlobals::gTimeToTeleportDisconnected >= 0
    //                                        && firstNotStopped->succEdge(1) != nullptr
    //                                        && firstNotStopped->getEdge()->allowedLanes(*firstNotStopped->succEdge(1), firstNotStopped->getVClass()) == nullptr);
    //
    //             const bool r1 = ttt > 0 && firstNotStopped->getWaitingTime() > ttt && !disconnected
    //                             // never teleport a taxi on the last edge of it's route (where it would exit the simulation)
    //                             && (firstNotStopped->getDevice(typeid(MSDevice_Taxi)) == nullptr || firstNotStopped->getRoutePosition() < (firstNotStopped->getRoute().size() - 1));
    //             const bool r2 = !r1 && MSGlobals::gTimeToGridlockHighways > 0
    //                             && firstNotStopped->getWaitingTime() > MSGlobals::gTimeToGridlockHighways
    //                             && getSpeedLimit() > MSGlobals::gGridlockHighwaysSpeed && wrongLane
    //                             && !disconnected;
    //             const bool r3 = disconnected && firstNotStopped->getWaitingTime() > MSGlobals::gTimeToTeleportDisconnected;
    //             const bool r4 = !r1 && !r2 && !r3 && tttb > 0
    //                             && firstNotStopped->getWaitingTime() > tttb && getBidiLane() && !disconnected;
    //             const bool r5 = MSGlobals::gTimeToTeleportRSDeadlock > 0 && MSRailSignalControl::hasInstance() && !r1 && !r2 && !r3 && !r4
    //                             && firstNotStopped->getWaitingTime() > MSGlobals::gTimeToTeleportRSDeadlock && MSRailSignalControl::getInstance().haveDeadlock(firstNotStopped);
    //             if (r1 || r2 || r3 || r4 || r5) {
    //                 const std::vector<MSLink*>::const_iterator link = succLinkSec(*firstNotStopped, 1, *this, firstNotStopped->getBestLanesContinuation());
    //                 const bool minorLink = !wrongLane && (link != myLinks.end()) && !((*link)->havePriority());
    //                 std::string reason = (wrongLane ? " (wrong lane" : (minorLink ? " (yield" : " (jam"));
    //                 myBruttoVehicleLengthSumToRemove += firstNotStopped->getVehicleType().getLengthWithGap();
    //                 myNettoVehicleLengthSumToRemove += firstNotStopped->getVehicleType().getLength();
    //                 if (firstNotStopped == myVehicles.back()) {
    //                     myVehicles.pop_back();
    //                 } else {
    //                     myVehicles.erase(std::find(myVehicles.begin(), myVehicles.end(), firstNotStopped));
    //                     reason = " (blocked";
    //                 }
    //                 WRITE_WARNINGF("Teleporting vehicle '%'; waited too long" + reason
    //                                + (r2 ? ", highway" : "")
    //                                + (r3 ? ", disconnected" : "")
    //                                + (r4 ? ", bidi" : "")
    //                                + (r5 ? ", railSignal" : "")
    //                                + "), lane='%', time=%.", firstNotStopped->getID(), getID(), time2string(t));
    //                 if (wrongLane) {
    //                     MSNet::getInstance()->getVehicleControl().registerTeleportWrongLane();
    //                 } else if (minorLink) {
    //                     MSNet::getInstance()->getVehicleControl().registerTeleportYield();
    //                 } else {
    //                     MSNet::getInstance()->getVehicleControl().registerTeleportJam();
    //                 }
    //                 if (MSGlobals::gRemoveGridlocked) {
    //                     firstNotStopped->onRemovalFromNet(MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED);
    //                     MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(firstNotStopped);
    //                 } else {
    //                     MSVehicleTransfer::getInstance()->add(t, firstNotStopped);
    //                 }
    //             }
    //         }
    //     }
    //     if (MSGlobals::gSublane) {
    //         // trigger sorting of vehicles as their order may have changed
    //         MSNet::getInstance()->getEdgeControl().needsVehicleIntegration(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn execute_movements(&mut self, t: SUMOTime) {
        // multithreading: there are concurrent writes to myNeedsCollisionCheck but all of them should set it to true
        self.myNeedsCollisionCheck = true;
        let bidi = self.get_bidi_lane();
        if !bidi.is_null() && bidi.get_vehicle_number() == 0 {
            MSNet::get_instance().m().get_edge_control().check_collision_for_inactive(bidi);
        }
        // removes `veh` from the network-wide bookkeeping: the same call the C++ makes as
        // `MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh)` (checkDuplicate defaults to false)
        let sched = |v: Ptr<MSVehicle>| {
            MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(Ptr::from_mut(&mut v.m().base.base), false);
        };
        let mut firstNotStopped: Option<Ptr<MSVehicle>> = None;
        // iterate over vehicles in reverse so that move reminders will be called in the correct order.
        // `b` is the forward index one past the element `*i` refers to (C++ reverse_iterator base()).
        let mut b = self.myVehicles.len();
        while b > 0 {
            let veh = self.myVehicles[b - 1];
            // length is needed later when the vehicle may not exist anymore
            let length = veh.get_vehicle_type().get_length_with_gap();
            let nettoLength = veh.get_vehicle_type().get_length();
            let moved = veh.m().execute_move();
            let target = veh.get_mutable_lane();
            // true when C++ falls through to the length bookkeeping and erase; false for the `continue` branches
            let removed: bool = if veh.has_arrived() {
                // vehicle has reached its arrival position
                veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_ARRIVED);
                sched(veh);
                true
            } else if !target.is_null() && moved {
                if target.get_edge().is_vaporizing() {
                    // vehicle has reached a vaporizing edge
                    veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_VAPORIZER);
                    sched(veh);
                } else {
                    // vehicle has entered a new lane (leaveLane and workOnMoveReminders were already called in MSVehicle::executeMove)
                    target.m().myVehBuffer.push(veh);
                    MSNet::get_instance().m().get_edge_control().needs_vehicle_integration(target);
                    if MSGlobals::sublane() && !veh.get_lane_change_model_2().get_shadow_lane().is_null() {
                        // trigger sorting of partial vehicles as their order may have changed (lane might not be active and only contain partial vehicles)
                        MSNet::get_instance().m().get_edge_control().needs_vehicle_integration(veh.get_lane_change_model_2().get_shadow_lane());
                    }
                }
                true
            } else if veh.is_parking() {
                // vehicle started to park
                MSVehicleTransfer::get_instance().m().add(t, veh);
                self.myParkingVehicles.insert(Ptr::from_mut(&mut veh.m().base));
                true
            } else if veh.broke_down() {
                veh.m().resume_from_stopping();
                utils::common::msg_handler::write_warning(&format!("Removing vehicle '{}' after breaking down, lane='{}', time={}.", veh.get_id(), veh.get_lane().get_id(), utils::common::time::time2string(t)));
                veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_BREAKDOWN);
                sched(veh);
                true
            } else if veh.is_jumping() {
                // vehicle jumps to next route edge
                MSVehicleTransfer::get_instance().m().add(t, veh);
                true
            } else if veh.get_position_on_lane() > self.get_length() {
                // for any reasons the vehicle is beyond its lane...
                // this should never happen because it is handled in MSVehicle::executeMove
                debug_assert!(false);
                utils::common::msg_handler::write_warning(&format!("Teleporting vehicle '{}'; beyond end of lane, target lane='{}', time={}.", veh.get_id(), self.get_id(), utils::common::time::time2string(t)));
                MSNet::get_instance().m().get_vehicle_control().count_collision(true);
                MSVehicleTransfer::get_instance().m().add(t, veh);
                true
            } else if veh.collision_stop_time() == 0 {
                veh.m().resume_from_stopping();
                if MSLane::get_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_REMOVE {
                    utils::common::msg_handler::write_warning(&format!("Removing vehicle '{}' after earlier collision, lane='{}', time={}.", veh.get_id(), veh.get_lane().get_id(), utils::common::time::time2string(t)));
                    veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_COLLISION);
                    sched(veh);
                    true
                } else if MSLane::get_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_TELEPORT {
                    utils::common::msg_handler::write_warning(&format!("Teleporting vehicle '{}' after earlier collision, lane='{}', time={}.", veh.get_id(), veh.get_lane().get_id(), utils::common::time::time2string(t)));
                    MSVehicleTransfer::get_instance().m().add(t, veh);
                    true
                } else {
                    if firstNotStopped.is_none() && !veh.is_stopped() && veh.get_lane() == Ptr::new(self) {
                        firstNotStopped = Some(veh);
                    }
                    false
                }
            } else {
                if firstNotStopped.is_none() && !veh.is_stopped() && veh.get_lane() == Ptr::new(self) {
                    firstNotStopped = Some(veh);
                }
                false
            };
            if removed {
                self.myBruttoVehicleLengthSumToRemove += length;
                self.myNettoVehicleLengthSumToRemove += nettoLength;
                self.myVehicles.remove(b - 1);
            }
            b -= 1;
        }
        if let Some(fns) = firstNotStopped {
            let ttt = fns.get_vehicle_type().get_parameter().get_time_to_teleport(MSGlobals::time_to_gridlock());
            let tttb = fns.get_vehicle_type().get_parameter().get_time_to_teleport_bidi(MSGlobals::time_to_teleport_bidi());
            if ttt > 0 || MSGlobals::time_to_gridlock_highways() > 0 || MSGlobals::time_to_teleport_disconnected() >= 0 || tttb > 0 || MSGlobals::time_to_teleport_rs_deadlock() > 0 {
                let wrongLane = !self.appropriate(fns);
                let disconnected = MSGlobals::time_to_teleport_disconnected() >= 0
                    && !fns.succ_edge(1).is_null()
                    && fns.get_edge().allowed_lanes(&*fns.succ_edge(1), fns.get_v_class(), false).is_null();
                // shortcut: MSDevice_Taxi is not ported, so every vehicle is treated as having no taxi device (C++ `getDevice(typeid(MSDevice_Taxi)) == nullptr` is true);
                // taxis that are not on their last edge would be wrongly teleported. Upgrade when MSDevice_Taxi is ported.
                let r1 = ttt > 0 && fns.get_waiting_time(false) > ttt && !disconnected;
                let r2 = !r1 && MSGlobals::time_to_gridlock_highways() > 0
                    && fns.get_waiting_time(false) > MSGlobals::time_to_gridlock_highways()
                    && self.get_speed_limit() > MSGlobals::gridlock_highways_speed() && wrongLane
                    && !disconnected;
                let r3 = disconnected && fns.get_waiting_time(false) > MSGlobals::time_to_teleport_disconnected();
                let r4 = !r1 && !r2 && !r3 && tttb > 0
                    && fns.get_waiting_time(false) > tttb && !self.get_bidi_lane().is_null() && !disconnected;
                let r5 = MSGlobals::time_to_teleport_rs_deadlock() > 0 && MSRailSignalControl::has_instance() && !r1 && !r2 && !r3 && !r4
                    && fns.get_waiting_time(false) > MSGlobals::time_to_teleport_rs_deadlock()
                    && MSRailSignalControl::get_instance().have_deadlock(Ptr::from_mut(&mut fns.m().base.base));
                if r1 || r2 || r3 || r4 || r5 {
                    let link = MSLane::succ_link_sec(&fns.base.base, 1, self, fns.get_best_lanes_continuation());
                    let minorLink = !wrongLane && link != self.myLinks.len() && !self.myLinks[link].have_priority();
                    let mut reason: String = (if wrongLane { " (wrong lane" } else if minorLink { " (yield" } else { " (jam" }).to_string();
                    self.myBruttoVehicleLengthSumToRemove += fns.get_vehicle_type().get_length_with_gap();
                    self.myNettoVehicleLengthSumToRemove += fns.get_vehicle_type().get_length();
                    if self.myVehicles.last() == Some(&fns) {
                        self.myVehicles.pop();
                    } else {
                        if let Some(pos) = self.myVehicles.iter().position(|v| *v == fns) {
                            self.myVehicles.remove(pos);
                        }
                        reason = " (blocked".to_string();
                    }
                    utils::common::msg_handler::write_warning(&format!(
                        "Teleporting vehicle '{}'; waited too long{}{}{}{}{}), lane='{}', time={}.",
                        fns.get_id(),
                        reason,
                        if r2 { ", highway" } else { "" },
                        if r3 { ", disconnected" } else { "" },
                        if r4 { ", bidi" } else { "" },
                        if r5 { ", railSignal" } else { "" },
                        self.get_id(),
                        utils::common::time::time2string(t)
                    ));
                    if wrongLane {
                        MSNet::get_instance().m().get_vehicle_control().register_teleport_wrong_lane();
                    } else if minorLink {
                        MSNet::get_instance().m().get_vehicle_control().register_teleport_yield();
                    } else {
                        MSNet::get_instance().m().get_vehicle_control().register_teleport_jam();
                    }
                    if MSGlobals::remove_gridlocked() {
                        fns.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED);
                        sched(fns);
                    } else {
                        MSVehicleTransfer::get_instance().m().add(t, fns);
                    }
                }
            }
        }
        if MSGlobals::sublane() {
            // trigger sorting of vehicles as their order may have changed
            MSNet::get_instance().m().get_edge_control().needs_vehicle_integration(Ptr::new(self));
        }
    }

    /// C++: `void MSLane::integrateNewVehicles()` [virtual]
    /// Insert buffered vehicle into the real lane.
    // void
    // MSLane::integrateNewVehicles() {
    //     myNeedsCollisionCheck = true;
    //     std::vector<MSVehicle*>& buffered = myVehBuffer.getContainer();
    //     sort(buffered.begin(), buffered.end(), vehicle_position_sorter(this));
    //     for (MSVehicle* const veh : buffered) {
    //         assert(veh->getLane() == this);
    //         myVehicles.insert(myVehicles.begin(), veh);
    //         myBruttoVehicleLengthSum += veh->getVehicleType().getLengthWithGap();
    //         myNettoVehicleLengthSum += veh->getVehicleType().getLength();
    //         //if (true) std::cout << SIMTIME << " integrateNewVehicle lane=" << getID() << " veh=" << veh->getID() << " (on lane " << veh->getLane()->getID() << ") into lane=" << getID() << " myBrutto=" << myBruttoVehicleLengthSum << "\n";
    //         myEdge->markDelayed();
    //     }
    //     buffered.clear();
    //     myVehBuffer.unlock();
    //     //std::cout << SIMTIME << " integrateNewVehicle lane=" << getID() << " myVehicles1=" << toString(myVehicles);
    //     if (MSGlobals::gLateralResolution > 0 || myOpposite != nullptr) {
    //         sort(myVehicles.begin(), myVehicles.end(), vehicle_natural_position_sorter(this));
    //     }
    //     sortPartialVehicles();
    // #ifdef DEBUG_VEHICLE_CONTAINER
    //     if (DEBUG_COND) std::cout << SIMTIME << " integrateNewVehicle lane=" << getID()
    //                                   << " vehicles=" << toString(myVehicles) << " partials=" << toString(myPartialVehicles) << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn integrate_new_vehicles(&mut self) {
        self.myNeedsCollisionCheck = true;
        let sorter = MSLane_vehicle_position_sorter::new(Ptr::new(self));
        // determinism: C++ std::sort is unstable on ties; sort_by is stable here
        self.myVehBuffer.sort_by(|a, b| {
            if sorter.call_op(*a, *b) != 0 {
                std::cmp::Ordering::Less
            } else if sorter.call_op(*b, *a) != 0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        for veh in self.myVehBuffer.iter() {
            // assert(veh->getLane() == this)
            self.myVehicles.insert(0, *veh);
            self.myBruttoVehicleLengthSum += veh.get_vehicle_type().get_length_with_gap();
            self.myNettoVehicleLengthSum += veh.get_vehicle_type().get_length();
            self.myEdge.mark_delayed();
        }
        self.myVehBuffer.clear();
        if g_lateral_resolution() > 0. || !self.myOpposite.is_null() {
            // determinism: C++ std::sort is unstable on ties; sort_by is stable here
            let natSorter = MSLane_vehicle_natural_position_sorter::new(Ptr::new(self));
            self.myVehicles.sort_by(|a, b| {
                if natSorter.call_op(*a, *b) != 0 {
                    std::cmp::Ordering::Less
                } else if natSorter.call_op(*b, *a) != 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
        self.sort_partial_vehicles();
    }

    /// C++: `void MSLane::markRecalculateBruttoSum()`
    /// Set a flag to recalculate the brutto (including minGaps) occupancy of this lane (used if mingap is changed)
    // void
    // MSLane::markRecalculateBruttoSum() {
    //     myRecalculateBruttoSum = true;
    // }
    #[allow(unused_variables)]
    pub fn mark_recalculate_brutto_sum(&mut self) {
        self.myRecalculateBruttoSum = true;
    }

    /// C++: `void MSLane::updateLengthSum()`
    /// updated current vehicle length sum (delayed to avoid lane-order-dependency)
    // void
    // MSLane::updateLengthSum() {
    //     myBruttoVehicleLengthSum -= myBruttoVehicleLengthSumToRemove;
    //     myNettoVehicleLengthSum -= myNettoVehicleLengthSumToRemove;
    //     myBruttoVehicleLengthSumToRemove = 0;
    //     myNettoVehicleLengthSumToRemove = 0;
    //     if (myVehicles.empty()) {
    //         // avoid numerical instability
    //         myBruttoVehicleLengthSum = 0;
    //         myNettoVehicleLengthSum = 0;
    //     } else if (myRecalculateBruttoSum) {
    //         myBruttoVehicleLengthSum = 0;
    //         for (VehCont::const_iterator i = myVehicles.begin(); i != myVehicles.end(); ++i) {
    //             myBruttoVehicleLengthSum += (*i)->getVehicleType().getLengthWithGap();
    //         }
    //         myRecalculateBruttoSum = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_length_sum(&mut self) {
        self.myBruttoVehicleLengthSum -= self.myBruttoVehicleLengthSumToRemove;
        self.myNettoVehicleLengthSum -= self.myNettoVehicleLengthSumToRemove;
        self.myBruttoVehicleLengthSumToRemove = 0.;
        self.myNettoVehicleLengthSumToRemove = 0.;
        if self.myVehicles.is_empty() {
            // avoid numerical instability
            self.myBruttoVehicleLengthSum = 0.;
            self.myNettoVehicleLengthSum = 0.;
        } else if self.myRecalculateBruttoSum {
            self.myBruttoVehicleLengthSum = 0.;
            for veh in self.myVehicles.iter() {
                self.myBruttoVehicleLengthSum += veh.get_vehicle_type().get_length_with_gap();
            }
            self.myRecalculateBruttoSum = false;
        }
    }

    /// C++: `bool MSLane::needsCollisionCheck() const` [const]
    /// short-circuit collision check if nothing changed since the last check
    // inline bool needsCollisionCheck() const {
    //         return myNeedsCollisionCheck;
    //     }
    #[allow(unused_variables)]
    pub fn needs_collision_check(&self) -> bool {
        self.myNeedsCollisionCheck
    }

    /// C++: `void MSLane::requireCollisionCheck()`
    /// require another collision check due to relevant changes in the simulation
    // inline void requireCollisionCheck() {
    //         myNeedsCollisionCheck = true;
    //     }
    #[allow(unused_variables)]
    pub fn require_collision_check(&mut self) {
        self.myNeedsCollisionCheck = true;
    }

    /// C++: `void MSLane::detectCollisions(SUMOTime, const std::string &)` [virtual]
    /// Check if vehicles are too close.
    // void
    // MSLane::detectCollisions(SUMOTime timestep, const std::string& stage) {
    //     myNeedsCollisionCheck = false;
    // #ifdef DEBUG_COLLISIONS
    //     if (DEBUG_COND) {
    //         std::vector<const MSVehicle*> all;
    //         for (AnyVehicleIterator last = anyVehiclesBegin(); last != anyVehiclesEnd(); ++last) {
    //             all.push_back(*last);
    //         }
    //         std::cout << SIMTIME << " detectCollisions stage=" << stage << " lane=" << getID() << ":\n"
    //                   << "   vehs=" << toString(myVehicles) << "\n"
    //                   << "   part=" << toString(myPartialVehicles) << "\n"
    //                   << "   all=" << toString(all) << "\n"
    //                   << "\n";
    //     }
    // #endif
    //
    //     if (myCollisionAction == COLLISION_ACTION_NONE) {
    //         return;
    //     }
    //
    //     std::set<const MSVehicle*, ComparatorNumericalIdLess> toRemove;
    //     std::set<const MSVehicle*, ComparatorNumericalIdLess> toTeleport;
    //     if (mustCheckJunctionCollisions()) {
    //         myNeedsCollisionCheck = true; // always check
    // #ifdef DEBUG_JUNCTION_COLLISIONS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " detect junction Collisions stage=" << stage << " lane=" << getID() << ":\n"
    //                       << "   vehs=" << toString(myVehicles) << "\n"
    //                       << "   part=" << toString(myPartialVehicles) << "\n"
    //                       << "\n";
    //         }
    // #endif
    //         assert(myLinks.size() == 1);
    //         const std::vector<const MSLane*>& foeLanes = myLinks.front()->getFoeLanes();
    //         // save the iterator, it might get modified, see #8842
    //         MSLane::AnyVehicleIterator end = anyVehiclesEnd();
    //         for (AnyVehicleIterator veh = anyVehiclesBegin(); veh != end; ++veh) {
    //             const MSVehicle* const collider = *veh;
    //             //std::cout << "   collider " << collider->getID() << "\n";
    //             PositionVector colliderBoundary = collider->getBoundingBox(myCheckJunctionCollisionMinGap);
    //             for (const MSLane* const foeLane : foeLanes) {
    // #ifdef DEBUG_JUNCTION_COLLISIONS
    //                 if (DEBUG_COND) {
    //                     std::cout << "     foeLane " << foeLane->getID()
    //                               << " foeVehs=" << toString(foeLane->myVehicles)
    //                               << " foePart=" << toString(foeLane->myPartialVehicles) << "\n";
    //                 }
    // #endif
    //                 MSLane::AnyVehicleIterator foeEnd = foeLane->anyVehiclesEnd();
    //                 for (MSLane::AnyVehicleIterator it_veh = foeLane->anyVehiclesBegin(); it_veh != foeEnd; ++it_veh) {
    //                     const MSVehicle* const victim = *it_veh;
    //                     if (victim == collider) {
    //                         // may happen if the vehicles lane and shadow lane are siblings
    //                         continue;
    //                     }
    // #ifdef DEBUG_JUNCTION_COLLISIONS
    //                     if (DEBUG_COND && DEBUG_COND2(collider)) {
    //                         std::cout << SIMTIME << " foe=" << victim->getID()
    //                                   << " bound=" << colliderBoundary << " foeBound=" << victim->getBoundingBox()
    //                                   << " overlaps=" << colliderBoundary.overlapsWith(victim->getBoundingBox())
    //                                   << " poly=" << collider->getBoundingPoly()
    //                                   << " foePoly=" << victim->getBoundingPoly()
    //                                   << " overlaps2=" << collider->getBoundingPoly().overlapsWith(victim->getBoundingPoly())
    //                                   << "\n";
    //                     }
    // #endif
    //                     if (MSGlobals::gIgnoreJunctionBlocker < std::numeric_limits<SUMOTime>::max()) {
    //                         if (collider->getWaitingTime() >= MSGlobals::gIgnoreJunctionBlocker
    //                                 || victim->getWaitingTime() >= MSGlobals::gIgnoreJunctionBlocker) {
    //                             // ignored vehicles should not tigger collision
    //                             continue;
    //                         }
    //                     }
    //
    //                     if (colliderBoundary.overlapsWith(victim->getBoundingBox())) {
    //                         // make a detailed check
    //                         PositionVector boundingPoly = collider->getBoundingPoly();
    //                         if (collider->getBoundingPoly(myCheckJunctionCollisionMinGap).overlapsWith(victim->getBoundingPoly())) {
    //                             // junction leader is the victim (collider must still be on junction)
    //                             assert(isInternal());
    //                             if (victim->getLane()->isInternal() && victim->isLeader(myLinks.front(), collider, -1)) {
    //                                 foeLane->handleCollisionBetween(timestep, stage, victim, collider, -1, 0, toRemove, toTeleport);
    //                             } else {
    //                                 handleCollisionBetween(timestep, stage, collider, victim, -1, 0, toRemove, toTeleport);
    //                             }
    //                         }
    //                     }
    //                 }
    //                 detectPedestrianJunctionCollision(collider, colliderBoundary, foeLane, timestep, stage, toRemove, toTeleport);
    //             }
    //             if (myLinks.front()->getWalkingAreaFoe() != nullptr) {
    //                 detectPedestrianJunctionCollision(collider, colliderBoundary, myLinks.front()->getWalkingAreaFoe(), timestep, stage, toRemove, toTeleport);
    //             }
    //             if (myLinks.front()->getWalkingAreaFoeExit() != nullptr) {
    //                 detectPedestrianJunctionCollision(collider, colliderBoundary, myLinks.front()->getWalkingAreaFoeExit(), timestep, stage, toRemove, toTeleport);
    //             }
    //         }
    //     }
    //
    //
    //     if (myIntermodalCollisionAction != COLLISION_ACTION_NONE && myEdge->getPersons().size() > 0 && hasPedestrians()) {
    // #ifdef DEBUG_PEDESTRIAN_COLLISIONS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " detect pedestrian collisions stage=" << stage << " lane=" << getID() << "\n";
    //         }
    // #endif
    //         AnyVehicleIterator v_end = anyVehiclesEnd();
    //         for (AnyVehicleIterator it_v = anyVehiclesBegin(); it_v != v_end; ++it_v) {
    //             const MSVehicle* v = *it_v;
    //             double back = v->getBackPositionOnLane(this);
    //             const double length = v->getVehicleType().getLength();
    //             const double right = v->getRightSideOnEdge(this) - getRightSideOnEdge();
    //             if (v->getLane() == getBidiLane()) {
    //                 // use the front position for checking
    //                 back -= length;
    //             }
    //             PersonDist leader = nextBlocking(back, right, right + v->getVehicleType().getWidth());
    // #ifdef DEBUG_PEDESTRIAN_COLLISIONS
    //             if (DEBUG_COND && DEBUG_COND2(v)) {
    //                 std::cout << SIMTIME << " back=" << back << " right=" << right << " person=" << Named::getIDSecure(leader.first)
    //                           << " dist=" << leader.second << " jammed=" << (leader.first == nullptr ? false : leader.first->isJammed()) << "\n";
    //             }
    // #endif
    //             if (leader.first != 0 && leader.second < length && !leader.first->isJammed()) {
    //                 if (v->getVehicleType().getGuiShape() == SUMOVehicleShape::AIRCRAFT) {
    //                     // aircraft wings and body are above walking level
    //                     continue;
    //                 }
    //                 const double gap = leader.second - length;
    //                 handleIntermodalCollisionBetween(timestep, stage, v, leader.first, gap, "sharedLane", toRemove, toTeleport);
    //             }
    //         }
    //     }
    //
    //     if (myVehicles.size() == 0) {
    //         return;
    //     }
    //     if (!MSGlobals::gSublane) {
    //         // no sublanes
    //         VehCont::reverse_iterator lastVeh = myVehicles.rend() - 1;
    //         for (VehCont::reverse_iterator pred = myVehicles.rbegin(); pred != lastVeh; ++pred) {
    //             VehCont::reverse_iterator veh = pred + 1;
    //             detectCollisionBetween(timestep, stage, *veh, *pred, toRemove, toTeleport);
    //         }
    //         if (myPartialVehicles.size() > 0) {
    //             detectCollisionBetween(timestep, stage, *lastVeh, myPartialVehicles.front(), toRemove, toTeleport);
    //         }
    //         if (getBidiLane() != nullptr) {
    //             // bidirectional railway
    //             MSLane* bidiLane = getBidiLane();
    //             if (bidiLane->getVehicleNumberWithPartials() > 0) {
    //                 for (auto veh = myVehicles.begin(); veh != myVehicles.end(); ++veh) {
    //                     double high = (*veh)->getPositionOnLane(this);
    //                     double low = (*veh)->getBackPositionOnLane(this);
    //                     if (stage == MSNet::STAGE_MOVEMENTS) {
    //                         // use previous back position to catch trains that
    //                         // "jump" through each other
    //                         low -= SPEED2DIST((*veh)->getSpeed());
    //                     }
    //                     for (AnyVehicleIterator veh2 = bidiLane->anyVehiclesBegin(); veh2 != bidiLane->anyVehiclesEnd(); ++veh2) {
    //                         // self-collisions might legitemately occur when a long train loops back on itself
    //                         if (*veh == *veh2 && !(*veh)->isRail()) {
    //                             continue;
    //                         }
    //                         if ((*veh)->getLane() == (*veh2)->getLane() ||
    //                                 (*veh)->getLane() == (*veh2)->getBackLane() ||
    //                                 (*veh)->getBackLane() == (*veh2)->getLane()) {
    //                             // vehicles are not in a bidi relation
    //                             continue;
    //                         }
    //                         double low2 = myLength - (*veh2)->getPositionOnLane(bidiLane);
    //                         double high2 = myLength - (*veh2)->getBackPositionOnLane(bidiLane);
    //                         if (stage == MSNet::STAGE_MOVEMENTS) {
    //                             // use previous back position to catch trains that
    //                             // "jump" through each other
    //                             high2 += SPEED2DIST((*veh2)->getSpeed());
    //                         }
    //                         if (!(high < low2 || high2 < low)) {
    // #ifdef DEBUG_COLLISIONS
    //                             if (DEBUG_COND) {
    //                                 std::cout << SIMTIME << " bidi-collision veh=" << (*veh)->getID() << " bidiVeh=" << (*veh2)->getID()
    //                                           << " vehFurther=" << toString((*veh)->getFurtherLanes())
    //                                           << " high=" << high << " low=" << low << " high2=" << high2 << " low2=" << low2 << "\n";
    //                             }
    // #endif
    //                             // the faster vehicle is at fault
    //                             MSVehicle* collider = const_cast<MSVehicle*>(*veh);
    //                             MSVehicle* victim = const_cast<MSVehicle*>(*veh2);
    //                             if (collider->getSpeed() < victim->getSpeed()) {
    //                                 std::swap(victim, collider);
    //                             }
    //                             handleCollisionBetween(timestep, stage, collider, victim, -1, 0, toRemove, toTeleport);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         // in the sublane-case it is insufficient to check the vehicles ordered
    //         // by their front position as there might be more than 2 vehicles next to each
    //         // other on the same lane
    //         // instead, a moving-window approach is used where all vehicles that
    //         // overlap in the longitudinal direction receive pairwise checks
    //         // XXX for efficiency, all lanes of an edge should be checked together
    //         // (lanechanger-style)
    //
    //         // XXX quick hack: check each in myVehicles against all others
    //         for (AnyVehicleIterator veh = anyVehiclesBegin(); veh != anyVehiclesEnd(); ++veh) {
    //             MSVehicle* follow = (MSVehicle*)*veh;
    //             for (AnyVehicleIterator veh2 = anyVehiclesBegin(); veh2 != anyVehiclesEnd(); ++veh2) {
    //                 MSVehicle* lead = (MSVehicle*)*veh2;
    //                 if (lead == follow) {
    //                     continue;
    //                 }
    //                 if (lead->getPositionOnLane(this) < follow->getPositionOnLane(this)) {
    //                     continue;
    //                 }
    //                 if (detectCollisionBetween(timestep, stage, follow, lead, toRemove, toTeleport)) {
    //                     // XXX what about collisions with multiple leaders at once?
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     for (std::set<const MSVehicle*, ComparatorNumericalIdLess>::iterator it = toRemove.begin(); it != toRemove.end(); ++it) {
    //         MSVehicle* veh = const_cast<MSVehicle*>(*it);
    //         MSLane* vehLane = veh->getMutableLane();
    //         vehLane->removeVehicle(veh, MSMoveReminder::NOTIFICATION_TELEPORT, false);
    //         if (toTeleport.count(veh) > 0) {
    //             MSVehicleTransfer::getInstance()->add(timestep, veh);
    //         } else {
    //             veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_VAPORIZED_COLLISION);
    //             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detect_collisions(&mut self, timestep: SUMOTime, stage: &str) {
        self.myNeedsCollisionCheck = false;
        if MSLane::get_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_NONE {
            return;
        }

        // determinism: C++ std::set<const MSVehicle*, ComparatorNumericalIdLess> orders by numerical ID; BTreeSet here orders by pointer
        let mut toRemove: BTreeSet<Ptr<MSVehicle>> = BTreeSet::new();
        let mut toTeleport: BTreeSet<Ptr<MSVehicle>> = BTreeSet::new();
        if self.must_check_junction_collisions() {
            self.myNeedsCollisionCheck = true; // always check
            debug_assert!(self.myLinks.len() == 1);
            let foeLanes: Vec<Ptr<MSLane>> = self.myLinks[0].get_foe_lanes().clone();
            // save the iterator, it might get modified, see #8842
            let end = self.any_vehicles_end();
            let mut veh = self.any_vehicles_begin();
            while veh.ne_op(&end) {
                let collider = veh.op_();
                veh.op__2();
                let colliderBoundary: PositionVector = collider.get_bounding_box(check_junction_collision_min_gap());
                for foeLane in foeLanes.iter().copied() {
                    let foeEnd = foeLane.any_vehicles_end();
                    let mut it_veh = foeLane.any_vehicles_begin();
                    while it_veh.ne_op(&foeEnd) {
                        let victim = it_veh.op_();
                        it_veh.op__2();
                        if victim == collider {
                            // may happen if the vehicles lane and shadow lane are siblings
                            continue;
                        }
                        if MSGlobals::ignore_junction_blocker() < SUMOTime::MAX {
                            if collider.get_waiting_time(false) >= MSGlobals::ignore_junction_blocker()
                                || victim.get_waiting_time(false) >= MSGlobals::ignore_junction_blocker() {
                                // ignored vehicles should not tigger collision
                                continue;
                            }
                        }

                        if colliderBoundary.overlaps_with(&victim.get_bounding_box(0.), 0.) {
                            // make a detailed check
                            let _boundingPoly = collider.get_bounding_poly(0.);
                            if collider.get_bounding_poly(check_junction_collision_min_gap()).overlaps_with(&victim.get_bounding_poly(0.), 0.) {
                                // junction leader is the victim (collider must still be on junction)
                                debug_assert!(self.is_internal());
                                if victim.get_lane().is_internal() && victim.is_leader(self.myLinks[0], collider, -1.) {
                                    foeLane.handle_collision_between(timestep, stage, victim, collider, -1., 0., &mut toRemove, &mut toTeleport);
                                } else {
                                    self.handle_collision_between(timestep, stage, collider, victim, -1., 0., &mut toRemove, &mut toTeleport);
                                }
                            }
                        }
                    }
                    self.detect_pedestrian_junction_collision(collider, &colliderBoundary, foeLane, timestep, stage, &mut toRemove, &mut toTeleport);
                }
                let walkingAreaFoe = self.myLinks[0].m().get_walking_area_foe();
                if !walkingAreaFoe.is_null() {
                    self.detect_pedestrian_junction_collision(collider, &colliderBoundary, walkingAreaFoe, timestep, stage, &mut toRemove, &mut toTeleport);
                }
                let walkingAreaFoeExit = self.myLinks[0].m().get_walking_area_foe_exit();
                if !walkingAreaFoeExit.is_null() {
                    self.detect_pedestrian_junction_collision(collider, &colliderBoundary, walkingAreaFoeExit, timestep, stage, &mut toRemove, &mut toTeleport);
                }
            }
        }

        if MSLane::get_intermodal_collision_action() != MSLane_CollisionAction::COLLISION_ACTION_NONE && self.myEdge.get_persons().len() > 0 && self.has_pedestrians() {
            let v_end = self.any_vehicles_end();
            let mut it_v = self.any_vehicles_begin();
            while it_v.ne_op(&v_end) {
                let v = it_v.op_();
                it_v.op__2();
                let mut back = v.get_back_position_on_lane(Ptr::new(self));
                let length = v.get_vehicle_type().get_length();
                let right = v.get_right_side_on_edge(Ptr::new(self)) - self.get_right_side_on_edge();
                if v.get_lane() == self.get_bidi_lane() {
                    // use the front position for checking
                    back -= length;
                }
                let leader = self.next_blocking(back, right, right + v.get_vehicle_type().get_width(), 0., false);
                if !leader.0.is_null() && leader.1 < length && !leader.0.is_jammed() {
                    if v.get_vehicle_type().get_gui_shape() == SUMOVehicleShape::Aircraft {
                        // aircraft wings and body are above walking level
                        continue;
                    }
                    let gap = leader.1 - length;
                    self.handle_intermodal_collision_between(timestep, stage, v, leader.0.cast::<MSTransportable>(), gap, "sharedLane", &mut toRemove, &mut toTeleport);
                }
            }
        }

        if self.myVehicles.len() == 0 {
            return;
        }
        if !MSGlobals::sublane() {
            // no sublanes
            let n = self.myVehicles.len();
            // C++ reverse iteration: pred runs over indices n-1..1, veh is pred-1
            for i in (1..n).rev() {
                let pred = self.myVehicles[i];
                let veh = self.myVehicles[i - 1];
                self.detect_collision_between(timestep, stage, veh, pred, &mut toRemove, &mut toTeleport);
            }
            if self.myPartialVehicles.len() > 0 {
                let lastVeh = self.myVehicles[0];
                let partial = self.myPartialVehicles[0];
                self.detect_collision_between(timestep, stage, lastVeh, partial, &mut toRemove, &mut toTeleport);
            }
            let bidiLane = self.get_bidi_lane();
            if !bidiLane.is_null() {
                // bidirectional railway
                if bidiLane.get_vehicle_number_with_partials() > 0 {
                    for i in 0..self.myVehicles.len() {
                        let veh = self.myVehicles[i];
                        let high = veh.get_position_on_lane_2(Ptr::new(self));
                        let mut low = veh.get_back_position_on_lane(Ptr::new(self));
                        if stage == STAGE_MOVEMENTS {
                            // use previous back position to catch trains that
                            // "jump" through each other
                            low -= utils::common::time::speed2dist(veh.get_speed());
                        }
                        let end2 = bidiLane.any_vehicles_end();
                        let mut veh2_it = bidiLane.any_vehicles_begin();
                        while veh2_it.ne_op(&end2) {
                            let veh2 = veh2_it.op_();
                            veh2_it.op__2();
                            // self-collisions might legitemately occur when a long train loops back on itself
                            if veh == veh2 && !veh.is_rail() {
                                continue;
                            }
                            if veh.get_lane() == veh2.get_lane() || veh.get_lane() == veh2.get_back_lane() || veh.get_back_lane() == veh2.get_lane() {
                                // vehicles are not in a bidi relation
                                continue;
                            }
                            let low2 = self.myLength - veh2.get_position_on_lane_2(bidiLane);
                            let mut high2 = self.myLength - veh2.get_back_position_on_lane(bidiLane);
                            if stage == STAGE_MOVEMENTS {
                                // use previous back position to catch trains that
                                // "jump" through each other
                                high2 += utils::common::time::speed2dist(veh2.get_speed());
                            }
                            if !(high < low2 || high2 < low) {
                                // the faster vehicle is at fault
                                let mut collider = veh;
                                let mut victim = veh2;
                                if collider.get_speed() < victim.get_speed() {
                                    std::mem::swap(&mut victim, &mut collider);
                                }
                                self.handle_collision_between(timestep, stage, collider, victim, -1., 0., &mut toRemove, &mut toTeleport);
                            }
                        }
                    }
                }
            }
        } else {
            // in the sublane-case it is insufficient to check the vehicles ordered
            // by their front position as there might be more than 2 vehicles next to each
            // other on the same lane; a pairwise check over all vehicles is used
            // XXX quick hack: check each in myVehicles against all others
            let end = self.any_vehicles_end();
            let mut veh = self.any_vehicles_begin();
            while veh.ne_op(&end) {
                let follow = veh.op_();
                veh.op__2();
                let end2 = self.any_vehicles_end();
                let mut veh2 = self.any_vehicles_begin();
                while veh2.ne_op(&end2) {
                    let lead = veh2.op_();
                    veh2.op__2();
                    if lead == follow {
                        continue;
                    }
                    if lead.get_position_on_lane_2(Ptr::new(self)) < follow.get_position_on_lane_2(Ptr::new(self)) {
                        continue;
                    }
                    if self.detect_collision_between(timestep, stage, follow, lead, &mut toRemove, &mut toTeleport) {
                        // XXX what about collisions with multiple leaders at once?
                        break;
                    }
                }
            }
        }

        // determinism: iteration order follows Ptr ordering here, C++ uses numerical ID order
        for veh in toRemove.iter().copied().collect::<Vec<Ptr<MSVehicle>>>() {
            let vehLane = veh.get_mutable_lane();
            vehLane.m().remove_vehicle(veh, MSMoveReminder_Notification::NOTIFICATION_TELEPORT, false);
            if toTeleport.contains(&veh) {
                MSVehicleTransfer::get_instance().m().add(timestep, veh);
            } else {
                veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_COLLISION);
                MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(veh.cast::<SUMOVehicle>(), true);
            }
        }
    }

    /// C++: `bool MSLane::appropriate(const MSVehicle *) const` [const, virtual]
    /// Returns the information whether this lane may be used to continue the current route
    // bool
    // MSLane::appropriate(const MSVehicle* veh) const {
    //     if (veh->getLaneChangeModel().isOpposite()) {
    //         return false;
    //     }
    //     if (myEdge->isInternal()) {
    //         return true;
    //     }
    //     if (veh->succEdge(1) == nullptr) {
    //         assert((int)veh->getBestLanes().size() > veh->getLaneIndex());
    //         if (veh->getBestLanes()[veh->getLaneIndex()].bestLaneOffset == 0) {
    //             return true;
    //         } else {
    //             return false;
    //         }
    //     }
    //     std::vector<MSLink*>::const_iterator link = succLinkSec(*veh, 1, *this, veh->getBestLanesContinuation());
    //     return (link != myLinks.end());
    // }
    #[allow(unused_variables)]
    pub fn appropriate(&self, veh: Ptr<MSVehicle>) -> bool {
        if veh.get_lane_change_model_2().is_opposite() {
            return false;
        }
        if self.myEdge.is_internal() {
            return true;
        }
        if veh.succ_edge(1).is_null() {
            debug_assert!(veh.get_best_lanes().len() as i32 > veh.get_lane_index());
            if veh.get_best_lanes()[veh.get_lane_index() as usize].bestLaneOffset == 0 {
                return true;
            } else {
                return false;
            }
        }
        let link = MSLane::succ_link_sec(&*veh, 1, self, veh.get_best_lanes_continuation());
        link != self.myLinks.len()
    }

    /// C++: `const std::vector<MSLink *> & MSLane::getLinkCont() const` [const]
    /// returns the container with all links !!!
    // const std::vector<MSLink*>& getLinkCont() const {
    //         return myLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_link_cont(&self) -> &Vec<Ptr<MSLink>> {
        &self.myLinks
    }

    /// C++: `const MSLink * MSLane::getLinkTo(const MSLane *const) const` [const]
    /// returns the link to the given lane or nullptr, if it is not connected
    // const MSLink*
    // MSLane::getLinkTo(const MSLane* const target) const {
    //     const bool internal = target->isInternal();
    //     for (const MSLink* const l : myLinks) {
    //         if ((internal && l->getViaLane() == target) || (!internal && l->getLane() == target)) {
    //             return l;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_link_to(&self, arg0: Ptr<MSLane>) -> Ptr<MSLink> {
        let internal = arg0.is_internal();
        for l in self.myLinks.iter() {
            if (internal && l.get_via_lane() == arg0) || (!internal && l.get_lane() == arg0) {
                return *l;
            }
        }
        Ptr::null()
    }

    /// C++: `const MSLane * MSLane::getInternalFollowingLane(const MSLane *const) const` [const]
    /// returns the internal lane leading to the given lane or nullptr, if there is none
    // const MSLane*
    // MSLane::getInternalFollowingLane(const MSLane* const target) const {
    //     for (const MSLink* const l : myLinks) {
    //         if (l->getLane() == target) {
    //             return l->getViaLane();
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_following_lane(&self, arg0: Ptr<MSLane>) -> Ptr<MSLane> {
        for l in self.myLinks.iter() {
            if l.get_lane() == arg0 {
                return l.get_via_lane();
            }
        }
        Ptr::null()
    }

    /// C++: `const MSLink * MSLane::getEntryLink() const` [const]
    /// Returns the entry link if this is an internal lane, else nullptr
    // const MSLink*
    // MSLane::getEntryLink() const {
    //     if (!isInternal()) {
    //         return nullptr;
    //     }
    //     const MSLane* internal = this;
    //     const MSLane* lane = this->getCanonicalPredecessorLane();
    //     assert(lane != nullptr);
    //     while (lane->isInternal()) {
    //         internal = lane;
    //         lane = lane->getCanonicalPredecessorLane();
    //         assert(lane != nullptr);
    //     }
    //     return lane->getLinkTo(internal);
    // }
    #[allow(unused_variables)]
    pub fn get_entry_link(&self) -> Ptr<MSLink> {
        if !self.is_internal() {
            return Ptr::null();
        }
        let mut internal = Ptr::new(self);
        let mut lane = self.get_canonical_predecessor_lane();
        debug_assert!(!lane.is_null());
        while lane.is_internal() {
            internal = lane;
            lane = lane.get_canonical_predecessor_lane();
            debug_assert!(!lane.is_null());
        }
        lane.get_link_to(internal)
    }

    /// C++: `bool MSLane::empty() const` [const]
    /// Returns true if there is not a single vehicle on the lane.
    // bool empty() const {
    //         assert(myVehBuffer.size() == 0);
    //         return myVehicles.empty();
    //     }
    #[allow(unused_variables)]
    pub fn empty(&self) -> bool {
        debug_assert!(self.myVehBuffer.len() == 0);
        self.myVehicles.is_empty()
    }

    /// C++: `void MSLane::setMaxSpeed(const double, const bool, const double)`
    /// Sets a new maximum speed for the lane (used by TraCI, MSLaneSpeedTrigger (VSS) and MSCalibrator)
    // void
    // MSLane::setMaxSpeed(const double val, const bool modified, const double jamThreshold) {
    //     myMaxSpeed = val;
    //     mySpeedModified = modified;
    //     myEdge->recalcCache();
    //     if (MSGlobals::gUseMesoSim) {
    //         MESegment* first = MSGlobals::gMesoNet->getSegmentForEdge(*myEdge);
    //         while (first != nullptr) {
    //             first->setSpeed(val, SIMSTEP, jamThreshold, myIndex);
    //             first = first->getNextSegment();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(&mut self, val: f64, modified: bool, jamThreshold: f64) {
        self.myMaxSpeed = val;
        self.mySpeedModified = modified;
        self.myEdge.m().recalc_cache();
        if crate::microsim::ms_globals::use_meso_sim() {
            let mut first = crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(self.myEdge.m(), 0.);
            while !first.is_null() {
                first.m().set_speed(val, MSNet::get_instance().get_current_time_step(), jamThreshold, self.myIndex);
                first = first.m().get_next_segment();
            }
        }
    }

    /// C++: `void MSLane::setFrictionCoefficient(double)`
    /// Sets a new friction coefficient for the lane [*to be later (used by TraCI and MSCalibrator)*]
    // void
    // MSLane::setFrictionCoefficient(double val) {
    //     myFrictionCoefficient = val;
    //     myEdge->recalcCache();
    // }
    #[allow(unused_variables)]
    pub fn set_friction_coefficient(&mut self, val: f64) {
        self.myFrictionCoefficient = val;
        self.myEdge.m().recalc_cache();
    }

    /// C++: `void MSLane::setLength(double)`
    /// Sets a new length for the lane (used by TraCI only)
    // void
    // MSLane::setLength(double val) {
    //     myLength = val;
    //     myEdge->recalcCache();
    // }
    #[allow(unused_variables)]
    pub fn set_length(&mut self, val: f64) {
        self.myLength = val;
        self.myEdge.m().recalc_cache();
    }

    /// C++: `MSEdge & MSLane::getEdge() const` [const]
    /// Returns the lane's edge
    // MSEdge& getEdge() const {
    //         return *myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> &mut MSEdge {
        self.myEdge.m()
    }

    /// C++: `const MSJunction * MSLane::getFromJunction() const` [const]
    // const MSJunction*
    // MSLane::getFromJunction() const {
    //     return myEdge->getFromJunction();
    // }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<MSJunction> {
        self.myEdge.get_from_junction()
    }

    /// C++: `const MSJunction * MSLane::getToJunction() const` [const]
    // const MSJunction*
    // MSLane::getToJunction() const {
    //     return myEdge->getToJunction();
    // }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<MSJunction> {
        self.myEdge.get_to_junction()
    }

    /// C++: `const MSEdge * MSLane::getNextNormal() const` [const]
    /// Returns the lane's follower if it is an internal lane, the edge of the lane otherwise
    // const MSEdge*
    // MSLane::getNextNormal() const {
    //     return myEdge->getNormalSuccessor();
    // }
    #[allow(unused_variables)]
    pub fn get_next_normal(&self) -> Ptr<MSEdge> {
        self.myEdge.get_normal_successor()
    }

    /// C++: `const MSLane * MSLane::getFirstInternalInConnection(double &) const` [const]
    /// Returns 0 if the lane is not internal. Otherwise the first part of the connection (sequence of internal lanes along junction) corresponding to the lane is returned and the offset is set to the distance of the begin of this lane to the begin of the returned.
    // const MSLane*
    // MSLane::getFirstInternalInConnection(double& offset) const {
    //     if (!this->isInternal()) {
    //         return nullptr;
    //     }
    //     offset = 0.;
    //     const MSLane* firstInternal = this;
    //     MSLane* pred = getCanonicalPredecessorLane();
    //     while (pred != nullptr && pred->isInternal()) {
    //         firstInternal = pred;
    //         offset += pred->getLength();
    //         pred = firstInternal->getCanonicalPredecessorLane();
    //     }
    //     return firstInternal;
    // }
    #[allow(unused_variables)]
    pub fn get_first_internal_in_connection(&self, offset: &mut f64) -> Ptr<MSLane> {
        if !self.is_internal() {
            return Ptr::null();
        }
        *offset = 0.;
        let mut firstInternal = Ptr::new(self);
        let mut pred = self.get_canonical_predecessor_lane();
        while !pred.is_null() && pred.is_internal() {
            firstInternal = pred;
            *offset += pred.get_length();
            pred = firstInternal.get_canonical_predecessor_lane();
        }
        firstInternal
    }

    /// C++: `bool MSLane::dictionary(const std::string &, MSLane *)` [static]
    /// Inserts a MSLane into the static dictionary
    // bool
    // MSLane::dictionary(const std::string& id, MSLane* ptr) {
    //     const DictType::iterator it = myDict.lower_bound(id);
    //     if (it == myDict.end() || it->first != id) {
    //         // id not in myDict
    //         myDict.emplace_hint(it, id, ptr);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn dictionary(id: &str, lane: Ptr<MSLane>) -> bool {
        MY_DICT.with(|d| {
            let mut d = d.borrow_mut();
            if d.contains_key(id) {
                false
            } else {
                d.insert(id.to_string(), lane);
                true
            }
        })
    }

    /// C++: `MSLane * MSLane::dictionary(const std::string &)` [static]
    /// Returns the MSLane associated to the key id
    // MSLane*
    // MSLane::dictionary(const std::string& id) {
    //     const DictType::iterator it = myDict.find(id);
    //     if (it == myDict.end()) {
    //         // id not in myDict
    //         return nullptr;
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn dictionary_2(id: &str) -> Ptr<MSLane> {
        MY_DICT.with(|d| d.borrow().get(id).copied().unwrap_or(Ptr::null()))
    }

    /// C++: `void MSLane::clear()` [static]
    /// Clears the dictionary
    // void
    // MSLane::clear() {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     myDict.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear() {
        // shortcut: C++ deletes the stored lane objects; Ptr entries are not owned here, so only the map is cleared
        MY_DICT.with(|d| d.borrow_mut().clear());
    }

    /// C++: `int MSLane::dictSize()` [static]
    /// Returns the number of stored lanes
    // static int dictSize() {
    //         return (int)myDict.size();
    //     }
    #[allow(unused_variables)]
    pub fn dict_size() -> i32 {
        MY_DICT.with(|d| d.borrow().len() as i32)
    }

    /// C++: `void MSLane::insertIDs(std::vector<std::string> &)` [static]
    /// Adds the ids of all stored lanes into the given vector
    // void
    // MSLane::insertIDs(std::vector<std::string>& into) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_i_ds(into: &mut Vec<String>) {
        MY_DICT.with(|d| {
            for k in d.borrow().keys() {
                into.push(k.clone());
            }
        });
    }

    /// C++: `void MSLane::fill(RTREE &)` [static]
    /// Fills the given RTree with lane instances
    // template<class RTREE> void
    // MSLane::fill(RTREE& into) {
    //     for (DictType::iterator i = myDict.begin(); i != myDict.end(); ++i) {
    //         MSLane* l = (*i).second;
    //         Boundary b = l->getShape().getBoxBoundary();
    //         b.grow(3.);
    //         const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //         const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //         into.Insert(cmin, cmax, l);
    //     }
    // }
    // determinism: lanes arrive in std::map id order (MY_DICT is a BTreeMap), which fixes the tree shape
    // the RTREE argument is an insert closure (cmin, cmax, lane), i.e. `into.Insert(cmin, cmax, l)`
    #[allow(unused_variables)]
    pub fn fill<F: FnMut(&[f32; 2], &[f32; 2], Ptr<MSLane>)>(mut into: F) {
        let lanes: Vec<Ptr<MSLane>> = MY_DICT.with(|d| d.borrow().values().copied().collect());
        for l in lanes {
            let mut b = l.get_shape_2().get_box_boundary();
            b.grow(3.);
            let cmin = [b.xmin() as f32, b.ymin() as f32];
            let cmax = [b.xmax() as f32, b.ymax() as f32];
            into(&cmin, &cmax, l);
        }
    }

    /// C++: `void MSLane::initRNGs(const OptionsCont &)` [static]
    /// initialize rngs
    // void
    // MSLane::initRNGs(const OptionsCont& oc) {
    //     myRNGs.clear();
    //     const int numRNGs = oc.getInt("thread-rngs");
    //     const bool random = oc.getBool("random");
    //     int seed = oc.getInt("seed");
    //     myRNGs.reserve(numRNGs); // this is needed for stable pointers on debugging
    //     for (int i = 0; i < numRNGs; i++) {
    //         myRNGs.push_back(SumoRNG("lanes_" + toString(i)));
    //         RandHelper::initRand(&myRNGs.back(), random, seed++);
    //     }
    // }
    #[allow(unused_variables)]
    /// Test-only stand-in for init_rn_gs (which needs the global options): one RNG, so MSLane::new works in unit tests
    #[cfg(test)]
    pub(crate) fn init_one_rng_for_tests() {
        MY_RNGS.with(|r| if r.borrow().is_empty() { r.borrow_mut().push(SumoRNG::new("lanes_0")) });
    }

    pub fn init_rn_gs(oc: &OptionsCont) {
        MY_RNGS.with(|r| r.borrow_mut().clear());
        let numRNGs = oc.get_int("thread-rngs").unwrap();
        let random = oc.get_bool("random").unwrap();
        let mut seed = oc.get_int("seed").unwrap();
        MY_RNGS.with(|r| r.borrow_mut().reserve(numRNGs as usize)); // this is needed for stable pointers on debugging
        for i in 0..numRNGs {
            let rng = SumoRNG::new(&format!("lanes_{}", i));
            MY_RNGS.with(|r| r.borrow_mut().push(rng));
            MY_RNGS.with(|r| utils::common::rand_helper::init_rand(r.borrow_mut().last_mut(), random, seed));
            seed += 1;
        }
    }

    /// C++: `std::vector<MSLink *>::const_iterator MSLane::succLinkSec(const SUMOVehicle &, int, const MSLane &, const std::vector<MSLane *> &)` [static]
    /// Same as succLink, but does not throw any assertions when the succeeding link could not be found; Returns the myLinks.end() instead; Further, the number of edges to look forward may be given
    // std::vector<MSLink*>::const_iterator
    // MSLane::succLinkSec(const SUMOVehicle& veh, int nRouteSuccs,
    //                     const MSLane& succLinkSource, const std::vector<MSLane*>& conts) {
    //     const MSEdge* nRouteEdge = veh.succEdge(nRouteSuccs);
    //     // check whether the vehicle tried to look beyond its route
    //     if (nRouteEdge == nullptr) {
    //         // return end (no succeeding link) if so
    //         return succLinkSource.myLinks.end();
    //     }
    //     // if we are on an internal lane there should only be one link and it must be allowed
    //     if (succLinkSource.isInternal()) {
    //         assert(succLinkSource.myLinks.size() == 1);
    //         // could have been disallowed dynamically with a rerouter or via TraCI
    //         // assert(succLinkSource.myLinks[0]->getLane()->allowsVehicleClass(veh.getVehicleType().getVehicleClass()));
    //         return succLinkSource.myLinks.begin();
    //     }
    //     // a link may be used if
    //     //  1) there is a destination lane ((*link)->getLane()!=0)
    //     //  2) the destination lane belongs to the next edge in route ((*link)->getLane()->myEdge == nRouteEdge)
    //     //  3) the destination lane allows the vehicle's class ((*link)->getLane()->allowsVehicleClass(veh.getVehicleClass()))
    //
    //     // there should be a link which leads to the next desired lane our route in "conts" (built in "getBestLanes")
    //     // "conts" stores the best continuations of our current lane
    //     // we should never return an arbitrary link since this may cause collisions
    //
    //     if (nRouteSuccs < (int)conts.size()) {
    //         // we go through the links in our list and return the matching one
    //         for (std::vector<MSLink*>::const_iterator link = succLinkSource.myLinks.begin(); link != succLinkSource.myLinks.end(); ++link) {
    //             if ((*link)->getLane() != nullptr && (*link)->getLane()->myEdge == nRouteEdge
    //                     && (*link)->getLane()->allowsVehicleClass(veh.getVClass())
    //                     && ((*link)->getViaLane() == nullptr || (*link)->getViaLane()->allowsVehicleClass(veh.getVClass()))) {
    //                 // we should use the link if it connects us to the best lane
    //                 if ((*link)->getLane() == conts[nRouteSuccs]) {
    //                     return link;
    //                 }
    //             }
    //         }
    //     } else {
    //         // the source lane is a dead end (no continuations exist)
    //         return succLinkSource.myLinks.end();
    //     }
    //     // the only case where this should happen is for a disconnected route (deliberately ignored)
    // #ifdef DEBUG_NO_CONNECTION
    //     // the "'" around the ids are missing intentionally in the message below because it slows messaging down, resulting in test timeouts
    //     WRITE_WARNING("Could not find connection between lane " + succLinkSource.getID() + " and lane " + conts[nRouteSuccs]->getID() +
    //                   " for vehicle " + veh.getID() + ", time=" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    // #endif
    //     return succLinkSource.myLinks.end();
    // }
    /// Returns the index into `succLinkSource.myLinks` of the link to use (`myLinks.len()` stands for C++ `end()`).
    // shortcut: iterators are modelled as indices into myLinks (usize), `len()` is end()
    pub fn succ_link_sec(veh: &SUMOVehicle, nRouteSuccs: i32, succLinkSource: &MSLane, conts: &Vec<Ptr<MSLane>>) -> usize {
        let nRouteEdge = veh.succ_edge(nRouteSuccs);
        // check whether the vehicle tried to look beyond its route
        if nRouteEdge.is_null() {
            // return end (no succeeding link) if so
            return succLinkSource.myLinks.len();
        }
        // if we are on an internal lane there should only be one link and it must be allowed
        if succLinkSource.is_internal() {
            debug_assert!(succLinkSource.myLinks.len() == 1);
            return 0;
        }
        // a link may be used if
        //  1) there is a destination lane ((*link)->getLane()!=0)
        //  2) the destination lane belongs to the next edge in route ((*link)->getLane()->myEdge == nRouteEdge)
        //  3) the destination lane allows the vehicle's class ((*link)->getLane()->allowsVehicleClass(veh.getVehicleClass()))
        // there should be a link which leads to the next desired lane our route in "conts" (built in "getBestLanes")
        // "conts" stores the best continuations of our current lane
        // we should never return an arbitrary link since this may cause collisions
        if (nRouteSuccs as usize) < conts.len() {
            // we go through the links in our list and return the matching one
            for (idx, link) in succLinkSource.myLinks.iter().enumerate() {
                let lane = link.get_lane();
                if !lane.is_null() && lane.myEdge == nRouteEdge
                    && lane.allows_vehicle_class(veh.get_v_class())
                    && (link.get_via_lane().is_null() || link.get_via_lane().allows_vehicle_class(veh.get_v_class())) {
                    // we should use the link if it connects us to the best lane
                    if lane == conts[nRouteSuccs as usize] {
                        return idx;
                    }
                }
            }
        } else {
            // the source lane is a dead end (no continuations exist)
            return succLinkSource.myLinks.len();
        }
        // the only case where this should happen is for a disconnected route (deliberately ignored)
        succLinkSource.myLinks.len()
    }

    /// C++: `bool MSLane::isLinkEnd(std::vector<MSLink *>::const_iterator &) const` [const]
    /// Returns the information whether the given link shows at the end of the list of links (is not valid)
    // inline bool isLinkEnd(std::vector<MSLink*>::const_iterator& i) const {
    //         return i == myLinks.end();
    //     }
    pub fn is_link_end(&self, i: &mut usize) -> bool {
        *i == self.myLinks.len()
    }

    /// C++: `bool MSLane::isLinkEnd(std::vector<MSLink *>::iterator &)`
    /// Returns the information whether the given link shows at the end of the list of links (is not valid)
    // inline bool isLinkEnd(std::vector<MSLink*>::iterator& i) {
    //         return i == myLinks.end();
    //     }
    pub fn is_link_end_2(&mut self, i: &mut usize) -> bool {
        *i == self.myLinks.len()
    }

    /// C++: `bool MSLane::isEmpty() const` [const]
    /// Returns the information whether the lane is has no vehicle and no partial occupation
    // inline bool isEmpty() const {
    //         return myVehicles.empty() && myPartialVehicles.empty();
    //     }
    #[allow(unused_variables)]
    pub fn is_empty(&self) -> bool {
        self.myVehicles.is_empty() && self.myPartialVehicles.is_empty()
    }

    /// C++: `bool MSLane::isInternal() const` [const]
    /// Returns whether the lane pertains to an internal edge
    // bool
    // MSLane::isInternal() const {
    //     return myEdge->isInternal();
    // }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        self.myEdge.is_internal()
    }

    /// C++: `bool MSLane::isNormal() const` [const]
    /// Returns whether the lane pertains to a normal edge
    // bool
    // MSLane::isNormal() const {
    //     return myEdge->isNormal();
    // }
    #[allow(unused_variables)]
    pub fn is_normal(&self) -> bool {
        self.myEdge.is_normal()
    }

    /// C++: `bool MSLane::isCrossing() const` [const]
    /// Returns whether the lane pertains to a crossing edge
    // bool
    // MSLane::isCrossing() const {
    //     return myEdge->isCrossing();
    // }
    #[allow(unused_variables)]
    pub fn is_crossing(&self) -> bool {
        self.myEdge.is_crossing()
    }

    /// C++: `bool MSLane::isPriorityCrossing() const` [const]
    /// Returns whether the lane pertains to a crossing edge
    // bool
    // MSLane::isPriorityCrossing() const {
    //     return isCrossing() && getIncomingLanes()[0].viaLink->getOffState() == LINKSTATE_MAJOR;
    // }
    #[allow(unused_variables)]
    pub fn is_priority_crossing(&self) -> bool {
        self.is_crossing() && self.get_incoming_lanes()[0].viaLink.get_off_state() == LINKSTATE_MAJOR
    }

    /// C++: `bool MSLane::isWalkingArea() const` [const]
    /// Returns whether the lane pertains to a walkingarea
    // bool
    // MSLane::isWalkingArea() const {
    //     return myEdge->isWalkingArea();
    // }
    #[allow(unused_variables)]
    pub fn is_walking_area(&self) -> bool {
        self.myEdge.is_walking_area()
    }

    /// C++: `MSVehicle * MSLane::getLastFullVehicle() const` [const]
    /// returns the last vehicle for which this lane is responsible or 0
    // MSVehicle*
    // MSLane::getLastFullVehicle() const {
    //     if (myVehicles.size() == 0) {
    //         return nullptr;
    //     }
    //     return myVehicles.front();
    // }
    #[allow(unused_variables)]
    pub fn get_last_full_vehicle(&self) -> Ptr<MSVehicle> {
        self.myVehicles.first().copied().unwrap_or(Ptr::null())
    }

    /// C++: `MSVehicle * MSLane::getFirstFullVehicle() const` [const]
    /// returns the first vehicle for which this lane is responsible or 0
    // MSVehicle*
    // MSLane::getFirstFullVehicle() const {
    //     if (myVehicles.size() == 0) {
    //         return nullptr;
    //     }
    //     return myVehicles.back();
    // }
    #[allow(unused_variables)]
    pub fn get_first_full_vehicle(&self) -> Ptr<MSVehicle> {
        self.myVehicles.last().copied().unwrap_or(Ptr::null())
    }

    /// C++: `MSVehicle * MSLane::getLastAnyVehicle() const` [const]
    /// returns the last vehicle that is fully or partially on this lane
    // MSVehicle*
    // MSLane::getLastAnyVehicle() const {
    //     // all vehicles in myVehicles should have positions smaller or equal to
    //     // those in myPartialVehicles (unless we're on a bidi-lane)
    //     if (myVehicles.size() > 0) {
    //         if (myBidiLane != nullptr && myPartialVehicles.size() > 0) {
    //             if (myVehicles.front()->getPositionOnLane() > myPartialVehicles.front()->getPositionOnLane(this)) {
    //                 return myPartialVehicles.front();
    //             }
    //         }
    //         return myVehicles.front();
    //     }
    //     if (myPartialVehicles.size() > 0) {
    //         return myPartialVehicles.front();
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_last_any_vehicle(&self) -> Ptr<MSVehicle> {
        // all vehicles in myVehicles should have positions smaller or equal to
        // those in myPartialVehicles (unless we're on a bidi-lane)
        if !self.myVehicles.is_empty() {
            if !self.myBidiLane.is_null() && !self.myPartialVehicles.is_empty() {
                if self.myVehicles[0].get_position_on_lane() > self.myPartialVehicles[0].get_position_on_lane_2(Ptr::new(self)) {
                    return self.myPartialVehicles[0];
                }
            }
            return self.myVehicles[0];
        }
        if !self.myPartialVehicles.is_empty() {
            return self.myPartialVehicles[0];
        }
        Ptr::null()
    }

    /// C++: `MSVehicle * MSLane::getFirstAnyVehicle() const` [const]
    /// returns the first vehicle that is fully or partially on this lane
    // MSVehicle*
    // MSLane::getFirstAnyVehicle() const {
    //     MSVehicle* result = nullptr;
    //     if (myVehicles.size() > 0) {
    //         result = myVehicles.back();
    //     }
    //     if (myPartialVehicles.size() > 0
    //             && (result == nullptr || result->getPositionOnLane(this) < myPartialVehicles.back()->getPositionOnLane(this))) {
    //         result = myPartialVehicles.back();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_first_any_vehicle(&self) -> Ptr<MSVehicle> {
        let mut result = Ptr::null();
        if !self.myVehicles.is_empty() {
            result = *self.myVehicles.last().unwrap();
        }
        if !self.myPartialVehicles.is_empty()
            && (result.is_null() || result.get_position_on_lane_2(Ptr::new(self)) < self.myPartialVehicles.last().unwrap().get_position_on_lane_2(Ptr::new(self)))
        {
            result = *self.myPartialVehicles.last().unwrap();
        }
        result
    }

    /// C++: `MSVehicle * MSLane::removeVehicle(MSVehicle *, MSMoveReminder::Notification, bool)` [virtual]
    // MSVehicle*
    // MSLane::removeVehicle(MSVehicle* remVehicle, MSMoveReminder::Notification notification, bool notify) {
    //     assert(remVehicle->getLane() == this);
    //     for (MSLane::VehCont::iterator it = myVehicles.begin(); it < myVehicles.end(); it++) {
    //         if (remVehicle == *it) {
    //             if (notify) {
    //                 remVehicle->leaveLane(notification);
    //             }
    //             myVehicles.erase(it);
    //             myBruttoVehicleLengthSum -= remVehicle->getVehicleType().getLengthWithGap();
    //             myNettoVehicleLengthSum -= remVehicle->getVehicleType().getLength();
    //             break;
    //         }
    //     }
    //     return remVehicle;
    // }
    #[allow(unused_variables)]
    pub fn remove_vehicle(&mut self, remVehicle: Ptr<MSVehicle>, notification: MSMoveReminder_Notification, notify: bool) -> Ptr<MSVehicle> {
        if let Some(pos) = self.myVehicles.iter().position(|v| *v == remVehicle) {
            if notify {
                remVehicle.m().leave_lane(notification, Ptr::null());
            }
            self.myVehicles.remove(pos);
            self.myBruttoVehicleLengthSum -= remVehicle.get_vehicle_type().get_length_with_gap();
            self.myNettoVehicleLengthSum -= remVehicle.get_vehicle_type().get_length();
        }
        remVehicle
    }

    /// C++: `void MSLane::leftByLaneChange(MSVehicle *)`
    // void
    // MSLane::leftByLaneChange(MSVehicle* v) {
    //     myBruttoVehicleLengthSum -= v->getVehicleType().getLengthWithGap();
    //     myNettoVehicleLengthSum -= v->getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn left_by_lane_change(&mut self, v: Ptr<MSVehicle>) {
        self.myBruttoVehicleLengthSum -= v.get_vehicle_type().get_length_with_gap();
        self.myNettoVehicleLengthSum -= v.get_vehicle_type().get_length();
    }

    /// C++: `void MSLane::enteredByLaneChange(MSVehicle *)`
    // void
    // MSLane::enteredByLaneChange(MSVehicle* v) {
    //     myBruttoVehicleLengthSum += v->getVehicleType().getLengthWithGap();
    //     myNettoVehicleLengthSum += v->getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn entered_by_lane_change(&mut self, v: Ptr<MSVehicle>) {
        self.myBruttoVehicleLengthSum += v.get_vehicle_type().get_length_with_gap();
        self.myNettoVehicleLengthSum += v.get_vehicle_type().get_length();
    }

    /// C++: `MSLane * MSLane::getParallelLane(int, bool) const` [const]
    /// Returns the lane with the given offset parallel to this one or 0 if it does not exist
    // MSLane*
    // MSLane::getParallelLane(int offset, bool includeOpposite) const {
    //     return myEdge->parallelLane(this, offset, includeOpposite);
    // }
    #[allow(unused_variables)]
    pub fn get_parallel_lane(&self, offset: i32, includeOpposite: bool) -> Ptr<MSLane> {
        self.myEdge.parallel_lane(Ptr::new(self), offset, includeOpposite)
    }

    /// C++: `void MSLane::setPermissions(SVCPermissions, long long)`
    /// Sets the permissions to the given value. If a transientID is given, the permissions are recored as temporary
    // void
    // MSLane::setPermissions(SVCPermissions permissions, long long transientID) {
    //     if (transientID == CHANGE_PERMISSIONS_PERMANENT) {
    //         myPermissions = permissions;
    //         myOriginalPermissions = permissions;
    //     } else {
    //         myPermissionChanges[transientID] = permissions;
    //         resetPermissions(CHANGE_PERMISSIONS_PERMANENT);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_permissions(&mut self, permissions: SVCPermissions, transientID: i64) {
        if transientID == MSLane_CHANGE_PERMISSIONS_PERMANENT {
            self.myPermissions = permissions;
            self.myOriginalPermissions = permissions;
        } else {
            self.myPermissionChanges.insert(transientID, permissions);
            self.reset_permissions(MSLane_CHANGE_PERMISSIONS_PERMANENT);
        }
    }

    /// C++: `void MSLane::resetPermissions(long long)`
    // void
    // MSLane::resetPermissions(long long transientID) {
    //     myPermissionChanges.erase(transientID);
    //     if (myPermissionChanges.empty()) {
    //         myPermissions = myOriginalPermissions;
    //     } else {
    //         // combine all permission changes
    //         myPermissions = SVCAll;
    //         for (const auto& item : myPermissionChanges) {
    //             myPermissions &= item.second;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_permissions(&mut self, transientID: i64) {
        self.myPermissionChanges.remove(&transientID);
        if self.myPermissionChanges.is_empty() {
            self.myPermissions = self.myOriginalPermissions;
        } else {
            // combine all permission changes
            self.myPermissions = SVC_ALL;
            for item in self.myPermissionChanges.values() {
                self.myPermissions &= *item;
            }
        }
    }

    /// C++: `bool MSLane::hadPermissionChanges() const` [const]
    // bool
    // MSLane::hadPermissionChanges() const {
    //     return !myPermissionChanges.empty();
    // }
    #[allow(unused_variables)]
    pub fn had_permission_changes(&self) -> bool {
        !self.myPermissionChanges.is_empty()
    }

    /// C++: `void MSLane::setChangeLeft(SVCPermissions)`
    /// Sets the permissions for changing to the left neighbour lane
    // void
    // MSLane::setChangeLeft(SVCPermissions permissions) {
    //     myChangeLeft = permissions;
    // }
    #[allow(unused_variables)]
    pub fn set_change_left(&mut self, permissions: SVCPermissions) {
        self.myChangeLeft = permissions;
    }

    /// C++: `void MSLane::setChangeRight(SVCPermissions)`
    /// Sets the permissions for changing to the right neighbour lane
    // void
    // MSLane::setChangeRight(SVCPermissions permissions) {
    //     myChangeRight = permissions;
    // }
    #[allow(unused_variables)]
    pub fn set_change_right(&mut self, permissions: SVCPermissions) {
        self.myChangeRight = permissions;
    }

    /// C++: `bool MSLane::allowsVehicleClass(SUMOVehicleClass) const` [const]
    // inline bool allowsVehicleClass(SUMOVehicleClass vclass) const {
    //         return (myPermissions & vclass) == vclass;
    //     }
    #[allow(unused_variables)]
    pub fn allows_vehicle_class(&self, vclass: SUMOVehicleClass) -> bool {
        (self.myPermissions & vclass) == vclass
    }

    /// C++: `bool MSLane::allowsVehicleClass(SUMOVehicleClass, int) const` [const]
    // bool
    // MSLane::allowsVehicleClass(SUMOVehicleClass vclass, int routingMode) const {
    //     return (((routingMode & libsumo::ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS) ? myOriginalPermissions : myPermissions) & vclass) == vclass;
    // }
    #[allow(unused_variables)]
    pub fn allows_vehicle_class_2(&self, vclass: SUMOVehicleClass, routingMode: i32) -> bool {
        let perms = if (routingMode & ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS) != 0 { self.myOriginalPermissions } else { self.myPermissions };
        (perms & vclass) == vclass
    }

    /// C++: `bool MSLane::allowsChangingLeft(SUMOVehicleClass) const` [const]
    /// Returns whether the given vehicle class may change left from this lane
    // inline bool allowsChangingLeft(SUMOVehicleClass vclass) const {
    //         return (myChangeLeft & vclass) == vclass;
    //     }
    #[allow(unused_variables)]
    pub fn allows_changing_left(&self, vclass: SUMOVehicleClass) -> bool {
        (self.myChangeLeft & vclass) == vclass
    }

    /// C++: `bool MSLane::allowsChangingRight(SUMOVehicleClass) const` [const]
    /// Returns whether the given vehicle class may change left from this lane
    // inline bool allowsChangingRight(SUMOVehicleClass vclass) const {
    //         return (myChangeRight & vclass) == vclass;
    //     }
    #[allow(unused_variables)]
    pub fn allows_changing_right(&self, vclass: SUMOVehicleClass) -> bool {
        (self.myChangeRight & vclass) == vclass
    }

    /// C++: `void MSLane::addIncomingLane(MSLane *, MSLink *)`
    // void
    // MSLane::addIncomingLane(MSLane* lane, MSLink* viaLink) {
    //     IncomingLaneInfo ili;
    //     ili.lane = lane;
    //     ili.viaLink = viaLink;
    //     ili.length = lane->getLength();
    //     myIncomingLanes.push_back(ili);
    // }
    #[allow(unused_variables)]
    pub fn add_incoming_lane(&mut self, lane: Ptr<MSLane>, viaLink: Ptr<MSLink>) {
        let ili = MSLane_IncomingLaneInfo {
            lane,
            viaLink,
            length: lane.get_length(),
        };
        self.myIncomingLanes.push(ili);
    }

    /// C++: `const std::vector<IncomingLaneInfo> & MSLane::getIncomingLanes() const` [const]
    // const std::vector<IncomingLaneInfo>& getIncomingLanes() const {
    //         return myIncomingLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_incoming_lanes(&self) -> &Vec<MSLane_IncomingLaneInfo> {
        &self.myIncomingLanes
    }

    /// C++: `void MSLane::addApproachingLane(MSLane *, bool)`
    // void
    // MSLane::addApproachingLane(MSLane* lane, bool warnMultiCon) {
    //     MSEdge* approachingEdge = &lane->getEdge();
    //     if (myApproachingLanes.find(approachingEdge) == myApproachingLanes.end()) {
    //         myApproachingLanes[approachingEdge] = std::vector<MSLane*>();
    //     } else if (!approachingEdge->isInternal() && warnMultiCon) {
    //         // whenever a normal edge connects twice, there is a corresponding
    //         // internal edge which connects twice, one warning is sufficient
    //         WRITE_WARNINGF(TL("Lane '%' is approached multiple times from edge '%'. This may cause collisions."),
    //                        getID(), approachingEdge->getID());
    //     }
    //     myApproachingLanes[approachingEdge].push_back(lane);
    // }
    #[allow(unused_variables)]
    pub fn add_approaching_lane(&mut self, lane: Ptr<MSLane>, warnMultiCon: bool) {
        let approachingEdge: Ptr<MSEdge> = Ptr::from_mut(lane.get_edge());
        if !self.myApproachingLanes.contains_key(&approachingEdge) {
            self.myApproachingLanes.insert(approachingEdge, Vec::new());
        } else if !approachingEdge.is_internal() && warnMultiCon {
            // whenever a normal edge connects twice, there is a corresponding
            // internal edge which connects twice, one warning is sufficient
            utils::write_warningf!("Lane '%' is approached multiple times from edge '%'. This may cause collisions.", self.get_id(), approachingEdge.get_id());
        }
        self.myApproachingLanes.entry(approachingEdge).or_default().push(lane);
    }

    /// C++: `bool MSLane::isApproachedFrom(MSEdge *const)`
    // inline bool isApproachedFrom(MSEdge* const edge) {
    //         return myApproachingLanes.find(edge) != myApproachingLanes.end();
    //     }
    #[allow(unused_variables)]
    pub fn is_approached_from(&mut self, edge: Ptr<MSEdge>) -> bool {
        self.myApproachingLanes.contains_key(&edge)
    }

    /// C++: `bool MSLane::isApproachedFrom(MSLane *const, SUMOVehicleClass)`
    // bool
    // MSLane::isApproachedFrom(MSLane* const lane, SUMOVehicleClass svc) {
    //     for (MSLink* link : lane->getLinkCont()) {
    //         if (link->getLane() == this && (link->getPermissions() & svc) == svc) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_approached_from_2(&mut self, lane: Ptr<MSLane>, svc: SUMOVehicleClass) -> bool {
        for link in lane.get_link_cont() {
            if link.get_lane() == Ptr::new(self) && (link.get_permissions() & svc) == svc {
                return true;
            }
        }
        false
    }

    /// C++: `double MSLane::getVehicleStopOffset(const MSVehicle *) const` [const]
    /// Returns vehicle class specific stopOffset for the vehicle
    // double
    // MSLane::getVehicleStopOffset(const MSVehicle* veh) const {
    //     if (!myLaneStopOffset.isDefined()) {
    //         return 0;
    //     }
    //     if ((myLaneStopOffset.getPermissions() & veh->getVClass()) != 0) {
    //         return myLaneStopOffset.getOffset();
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_stop_offset(&self, veh: Ptr<MSVehicle>) -> f64 {
        if !self.myLaneStopOffset.is_defined() {
            return 0.;
        }
        if (self.myLaneStopOffset.get_permissions() & veh.get_vehicle_type().get_vehicle_class()) != 0 {
            return self.myLaneStopOffset.get_offset();
        } else {
            return 0.;
        }
    }

    /// C++: `const StopOffset & MSLane::getLaneStopOffsets() const` [const]
    /// Returns vehicle class specific stopOffsets
    // const StopOffset&
    // MSLane::getLaneStopOffsets() const {
    //     return myLaneStopOffset;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_stop_offsets(&self) -> &StopOffset {
        &self.myLaneStopOffset
    }

    /// C++: `void MSLane::setLaneStopOffset(const StopOffset &)`
    /// Set vehicle class specific stopOffsets
    // void
    // MSLane::setLaneStopOffset(const StopOffset& stopOffset) {
    //     myLaneStopOffset = stopOffset;
    // }
    #[allow(unused_variables)]
    pub fn set_lane_stop_offset(&mut self, stopOffset: &StopOffset) {
        self.myLaneStopOffset = stopOffset.clone();
    }

    /// C++: `MSLeaderDistanceInfo MSLane::getFollowersOnConsecutive(const MSVehicle *, double, bool, double, MinorLinkMode, bool) const` [const]
    /// return the sublane followers with the largest missing rear gap among all predecessor lanes (within dist)
    // MSLeaderDistanceInfo
    // MSLane::getFollowersOnConsecutive(const MSVehicle* ego, double backOffset,
    //                                   bool allSublanes, double searchDist, MinorLinkMode mLinkMode, bool maxSearchDist) const {
    //     assert(ego != 0);
    //     // get the follower vehicle on the lane to change to
    //     const double egoPos = backOffset + ego->getVehicleType().getLength();
    //     const double egoLatDist = ego->getLane()->getRightSideOnEdge() - getRightSideOnEdge();
    //     const bool getOppositeLeaders = ((ego->getLaneChangeModel().isOpposite() && ego->getLane() == this)
    //                                      || (!ego->getLaneChangeModel().isOpposite() && &ego->getLane()->getEdge() != &getEdge()));
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(ego)) {
    //         std::cout << SIMTIME << " getFollowers lane=" << getID() << " ego=" << ego->getID()
    //                   << " backOffset=" << backOffset << " pos=" << egoPos
    //                   << " allSub=" << allSublanes << " searchDist=" << searchDist << " ignoreMinor=" << mLinkMode
    //                   << " maxSearchDist=" << maxSearchDist
    //                   << " egoLatDist=" << egoLatDist
    //                   << " getOppositeLeaders=" << getOppositeLeaders
    //                   << "\n";
    //     }
    // #endif
    //     MSCriticalFollowerDistanceInfo result(myWidth, allSublanes ? nullptr : ego, allSublanes ? 0 : egoLatDist, getOppositeLeaders);
    //     if (MSGlobals::gLateralResolution > 0 && egoLatDist == 0) {
    //         // check whether ego is outside lane bounds far enough so that another vehicle might
    //         // be between itself and the first "actual" sublane
    //         // shift the offset so that we "see" this vehicle
    //         if (ego->getLeftSideOnLane() < -MSGlobals::gLateralResolution) {
    //             result.setSublaneOffset(int(-ego->getLeftSideOnLane() / MSGlobals::gLateralResolution));
    //         } else if (ego->getRightSideOnLane() > getWidth() + MSGlobals::gLateralResolution) {
    //             result.setSublaneOffset(-int((ego->getRightSideOnLane() - getWidth()) / MSGlobals::gLateralResolution));
    //         }
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(ego)) {
    //             std::cout << SIMTIME << " getFollowers lane=" << getID() << " ego=" << ego->getID()
    //                       << " egoPosLat=" << ego->getLateralPositionOnLane()
    //                       << " egoLatDist=" << ego->getLane()->getRightSideOnEdge() - getRightSideOnEdge()
    //                       << " extraOffset=" << result.getSublaneOffset()
    //                       << "\n";
    //         }
    // #endif
    //     }
    //     /// XXX iterate in reverse and abort when there are no more freeSublanes
    //     for (AnyVehicleIterator last = anyVehiclesBegin(); last != anyVehiclesEnd(); ++last) {
    //         const MSVehicle* veh = *last;
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(ego)) {
    //             std::cout << "  veh=" << veh->getID() << " lane=" << veh->getLane()->getID() << " pos=" << veh->getPositionOnLane(this) << "\n";
    //         }
    // #endif
    //         if (veh != ego && veh->getPositionOnLane(this) < egoPos) {
    //             //const double latOffset = veh->getLane()->getRightSideOnEdge() - getRightSideOnEdge();
    //             const double latOffset = veh->getLatOffset(this);
    //             double dist = backOffset - veh->getPositionOnLane(this) - veh->getVehicleType().getMinGap();
    //             if (veh->isBidiOn(this)) {
    //                 dist -= veh->getLength();
    //             }
    //             result.addFollower(veh, ego, dist, latOffset);
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(ego)) {
    //                 std::cout << "  (1) added veh=" << veh->getID() << " latOffset=" << latOffset << " result=" << result.toString() << "\n";
    //             }
    // #endif
    //         }
    //     }
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(ego)) {
    //         std::cout << "  result.numFreeSublanes=" << result.numFreeSublanes() << "\n";
    //     }
    // #endif
    //     if (result.numFreeSublanes() > 0) {
    //         // do a tree search among all follower lanes and check for the most
    //         // important vehicle (the one requiring the largest reargap)
    //         // to get a safe bound on the necessary search depth, we need to consider the maximum speed and minimum
    //         // deceleration of potential follower vehicles
    //         if (searchDist == -1) {
    //             searchDist = getMaximumBrakeDist() - backOffset;
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(ego)) {
    //                 std::cout << "   computed searchDist=" << searchDist << "\n";
    //             }
    // #endif
    //         }
    //         std::set<const MSEdge*> egoFurther;
    //         for (MSLane* further : ego->getFurtherLanes()) {
    //             egoFurther.insert(&further->getEdge());
    //         }
    //         if (ego->getPositionOnLane() < ego->getVehicleType().getLength() && egoFurther.size() == 0
    //                 && ego->getLane()->getLogicalPredecessorLane() != nullptr) {
    //             // on insertion
    //             egoFurther.insert(&ego->getLane()->getLogicalPredecessorLane()->getEdge());
    //         }
    //
    //         // avoid loops
    //         std::set<const MSLane*> visited(myEdge->getLanes().begin(), myEdge->getLanes().end());
    //         if (myEdge->getBidiEdge() != nullptr) {
    //             visited.insert(myEdge->getBidiEdge()->getLanes().begin(), myEdge->getBidiEdge()->getLanes().end());
    //         }
    //         std::vector<MSLane::IncomingLaneInfo> newFound;
    //         std::vector<MSLane::IncomingLaneInfo> toExamine = myIncomingLanes;
    //         while (toExamine.size() != 0) {
    //             for (std::vector<MSLane::IncomingLaneInfo>::iterator it = toExamine.begin(); it != toExamine.end(); ++it) {
    //                 MSLane* next = (*it).lane;
    //                 searchDist = maxSearchDist
    //                              ? MAX2(searchDist, next->getMaximumBrakeDist() - backOffset)
    //                              : MIN2(searchDist, next->getMaximumBrakeDist() - backOffset);
    //                 MSLeaderInfo first = next->getFirstVehicleInformation(nullptr, 0, false, std::numeric_limits<double>::max(), false);
    //                 MSLeaderInfo firstFront = next->getFirstVehicleInformation(nullptr, 0, true);
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(ego)) {
    //                     std::cout << "   next=" << next->getID() << " seen=" << (*it).length << " first=" << first.toString() << " firstFront=" << firstFront.toString() << " backOffset=" << backOffset << "\n";
    //                     gDebugFlag1 = true; // for calling getLeaderInfo
    //                 }
    // #endif
    //                 if (backOffset + (*it).length - next->getLength() < 0
    //                         && egoFurther.count(&next->getEdge()) != 0
    //                    )  {
    //                     // check for junction foes that would interfere with lane changing
    //                     // @note: we are passing the back of ego as its front position so
    //                     //        we need to add this back to the returned gap
    //                     const MSLink::LinkLeaders linkLeaders = (*it).viaLink->getLeaderInfo(ego, -backOffset);
    //                     for (const auto& ll : linkLeaders) {
    //                         if (ll.vehAndGap.first != nullptr) {
    //                             const bool bidiFoe = (*it).viaLink->getLane() == ll.vehAndGap.first->getLane()->getNormalPredecessorLane()->getBidiLane();
    //                             const bool egoIsLeader = !bidiFoe && ll.vehAndGap.first->isLeader((*it).viaLink, ego, ll.vehAndGap.second);
    //                             // if ego is leader the returned gap still assumes that ego follows the leader
    //                             // if the foe vehicle follows ego we need to deduce that gap
    //                             const double gap = (egoIsLeader
    //                                                 ? -ll.vehAndGap.second - ll.vehAndGap.first->getVehicleType().getLengthWithGap() - ego->getVehicleType().getMinGap()
    //                                                 : ll.vehAndGap.second + ego->getVehicleType().getLength());
    //                             result.addFollower(ll.vehAndGap.first, ego, gap);
    // #ifdef DEBUG_CONTEXT
    //                             if (DEBUG_COND2(ego)) {
    //                                 std::cout << SIMTIME << " ego=" << ego->getID() << "    link=" << (*it).viaLink->getViaLaneOrLane()->getID()
    //                                           << " (3) added veh=" << Named::getIDSecure(ll.vehAndGap.first)
    //                                           << " gap=" << ll.vehAndGap.second << " dtC=" << ll.distToCrossing
    //                                           << " bidiFoe=" << bidiFoe
    //                                           << " egoIsLeader=" << egoIsLeader << " gap2=" << gap
    //                                           << "\n";
    //                             }
    // #endif
    //                         }
    //                     }
    //                 }
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(ego)) {
    //                     gDebugFlag1 = false;
    //                 }
    // #endif
    //
    //                 for (int i = 0; i < first.numSublanes(); ++i) {
    //                     const MSVehicle* v = first[i] == ego ? firstFront[i] : first[i];
    //                     double agap = 0;
    //
    //                     if (v != nullptr && v != ego) {
    //                         if (!v->isFrontOnLane(next)) {
    //                             // the front of v is already on divergent trajectory from the ego vehicle
    //                             // for which this method is called (in the context of MSLaneChanger).
    //                             // Therefore, technically v is not a follower but only an obstruction and
    //                             // the gap is not between the front of v and the back of ego
    //                             // but rather between the flank of v and the back of ego.
    //                             agap = (*it).length - next->getLength() + backOffset;
    //                             if (MSGlobals::gUsingInternalLanes) {
    //                                 // ego should have left the intersection still occupied by v
    //                                 agap -= v->getVehicleType().getMinGap();
    //                             }
    // #ifdef DEBUG_CONTEXT
    //                             if (DEBUG_COND2(ego)) {
    //                                 std::cout << "    agap1=" << agap << "\n";
    //                             }
    // #endif
    //                             const bool differentEdge = &v->getLane()->getEdge() != &ego->getLane()->getEdge();
    //                             if (agap > 0 && differentEdge) {
    //                                 // Only if ego overlaps we treat v as if it were a real follower
    //                                 // Otherwise we ignore it and look for another follower
    //                                 if (!getOppositeLeaders) {
    //                                     // even if the vehicle is not a real
    //                                     // follower, it still forms a real
    //                                     // obstruction in opposite direction driving
    //                                     v = firstFront[i];
    //                                     if (v != nullptr && v != ego) {
    //                                         agap = (*it).length - v->getPositionOnLane() + backOffset - v->getVehicleType().getMinGap();
    //                                     } else {
    //                                         v = nullptr;
    //                                     }
    //                                 }
    //                             } else if (differentEdge && result.hasVehicle(v)) {
    //                                 // ignore this vehicle as it was already seen on another lane
    //                                 agap = 0;
    //                             }
    //                         } else {
    //                             if (next->getBidiLane() != nullptr && v->isBidiOn(next)) {
    //                                 agap = v->getPositionOnLane() + backOffset - v->getVehicleType().getLengthWithGap();
    //                             } else {
    //                                 agap = (*it).length - v->getPositionOnLane() + backOffset - v->getVehicleType().getMinGap();
    //                             }
    //                             if (!(*it).viaLink->havePriority() && egoFurther.count(&(*it).lane->getEdge()) == 0
    //                                     && ego->isOnRoad() // during insertion, this can lead to collisions because ego's further lanes are not set (see #3053)
    //                                     && !ego->getLaneChangeModel().isOpposite()
    //                                     && v->getSpeed() < SUMO_const_haltingSpeed
    //                                ) {
    //                                 // if v is stopped on a minor side road it should not block lane changing
    //                                 agap = MAX2(agap, 0.0);
    //                             }
    //                         }
    //                         result.addFollower(v, ego, agap, 0, i);
    // #ifdef DEBUG_CONTEXT
    //                         if (DEBUG_COND2(ego)) {
    //                             std::cout << " (2) added veh=" << Named::getIDSecure(v) << " agap=" << agap << " next=" << next->getID() << " result=" << result.toString() << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //                 if ((*it).length < searchDist) {
    //                     const std::vector<MSLane::IncomingLaneInfo>& followers = next->getIncomingLanes();
    //                     for (std::vector<MSLane::IncomingLaneInfo>::const_iterator j = followers.begin(); j != followers.end(); ++j) {
    //                         if (visited.find((*j).lane) == visited.end() && (((*j).viaLink->havePriority() && !(*j).viaLink->isTurnaround())
    //                                 || mLinkMode == MinorLinkMode::FOLLOW_ALWAYS
    //                                 || (mLinkMode == MinorLinkMode::FOLLOW_ONCOMING && (*j).viaLink->getDirection() == LinkDirection::STRAIGHT))) {
    //                             visited.insert((*j).lane);
    //                             MSLane::IncomingLaneInfo ili;
    //                             ili.lane = (*j).lane;
    //                             ili.length = (*j).length + (*it).length;
    //                             ili.viaLink = (*j).viaLink;
    //                             newFound.push_back(ili);
    //                         }
    //                     }
    //                 }
    //             }
    //             toExamine.clear();
    //             swap(newFound, toExamine);
    //         }
    //         //return result;
    //
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_followers_on_consecutive(&self, ego: Ptr<MSVehicle>, backOffset: f64, allSublanes: bool, searchDist: f64, mLinkMode: MSLane_MinorLinkMode, maxSearchDist: bool) -> MSLeaderDistanceInfo {
        debug_assert!(!ego.is_null());
        // get the follower vehicle on the lane to change to
        let egoPos = backOffset + ego.get_vehicle_type().get_length();
        let egoLatDist = ego.get_lane().get_right_side_on_edge() - self.get_right_side_on_edge();
        let getOppositeLeaders = (ego.get_lane_change_model_2().is_opposite() && ego.get_lane() == Ptr::new(self))
            || (!ego.get_lane_change_model_2().is_opposite() && ego.get_lane().myEdge != self.myEdge);
        let mut result = MSCriticalFollowerDistanceInfo::new(self.myWidth, if allSublanes { Ptr::null() } else { ego }, if allSublanes { 0. } else { egoLatDist }, getOppositeLeaders);
        if g_lateral_resolution() > 0. && egoLatDist == 0. {
            // check whether ego is outside lane bounds far enough so that another vehicle might
            // be between itself and the first "actual" sublane
            // shift the offset so that we "see" this vehicle
            if ego.get_left_side_on_lane() < -g_lateral_resolution() {
                result.set_sublane_offset((-ego.get_left_side_on_lane() / g_lateral_resolution()) as i32);
            } else if ego.get_right_side_on_lane() > self.get_width() + g_lateral_resolution() {
                result.set_sublane_offset(-(((ego.get_right_side_on_lane() - self.get_width()) / g_lateral_resolution()) as i32));
            }
        }
        // XXX iterate in reverse and abort when there are no more freeSublanes
        let mut last = self.any_vehicles_begin();
        let lastEnd = self.any_vehicles_end();
        while last.ne_op(&lastEnd) {
            let veh = last.mul_op();
            if veh != ego && veh.get_position_on_lane_2(Ptr::new(self)) < egoPos {
                let latOffset = veh.get_lat_offset(Ptr::new(self));
                let mut dist = backOffset - veh.get_position_on_lane_2(Ptr::new(self)) - veh.get_vehicle_type().get_min_gap();
                if veh.is_bidi_on(Ptr::new(self)) {
                    dist -= veh.get_length();
                }
                result.add_follower(veh, ego, dist, latOffset, -1);
            }
            last.op__2();
        }
        if result.num_free_sublanes() > 0 {
            // do a tree search among all follower lanes and check for the most
            // important vehicle (the one requiring the largest reargap)
            // to get a safe bound on the necessary search depth, we need to consider the maximum speed and minimum
            // deceleration of potential follower vehicles
            let mut searchDist = searchDist;
            if searchDist == -1. {
                searchDist = self.get_maximum_brake_dist() - backOffset;
            }
            let mut egoFurther: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
            for further in ego.get_further_lanes() {
                egoFurther.insert(further.myEdge);
            }
            if ego.get_position_on_lane() < ego.get_vehicle_type().get_length() && egoFurther.is_empty()
                && !ego.get_lane().get_logical_predecessor_lane().is_null() {
                // on insertion
                egoFurther.insert(ego.get_lane().get_logical_predecessor_lane().myEdge);
            }

            // avoid loops
            let mut visited: BTreeSet<Ptr<MSLane>> = self.myEdge.get_lanes().iter().copied().collect();
            if !self.myEdge.get_bidi_edge().is_null() {
                visited.extend(self.myEdge.get_bidi_edge().get_lanes().iter().copied());
            }
            let mut newFound: Vec<MSLane_IncomingLaneInfo> = Vec::new();
            let mut toExamine: Vec<MSLane_IncomingLaneInfo> = self.myIncomingLanes.iter()
                .map(|ili| MSLane_IncomingLaneInfo { lane: ili.lane, length: ili.length, viaLink: ili.viaLink })
                .collect();
            while toExamine.len() != 0 {
                for it in toExamine.iter() {
                    let next = it.lane;
                    searchDist = if maxSearchDist {
                        let b = next.get_maximum_brake_dist() - backOffset;
                        if searchDist > b { searchDist } else { b }
                    } else {
                        let b = next.get_maximum_brake_dist() - backOffset;
                        if searchDist < b { searchDist } else { b }
                    };
                    let first = next.get_first_vehicle_information(Ptr::null(), 0., false, f64::MAX, false);
                    let firstFront = next.get_first_vehicle_information(Ptr::null(), 0., true, f64::MAX, true);

                    if backOffset + it.length - next.get_length() < 0. && egoFurther.contains(&next.myEdge) {
                        // check for junction foes that would interfere with lane changing
                        // @note: we are passing the back of ego as its front position so
                        //        we need to add this back to the returned gap
                        let linkLeaders = it.viaLink.get_leader_info(ego, -backOffset, Ptr::null(), false);
                        for ll in linkLeaders.iter() {
                            if !ll.vehAndGap.0.is_null() {
                                let bidiFoe = it.viaLink.get_lane() == ll.vehAndGap.0.get_lane().get_normal_predecessor_lane().get_bidi_lane();
                                let egoIsLeader = !bidiFoe && ll.vehAndGap.0.is_leader(it.viaLink, ego, ll.vehAndGap.1);
                                // if ego is leader the returned gap still assumes that ego follows the leader
                                // if the foe vehicle follows ego we need to deduce that gap
                                let gap = if egoIsLeader {
                                    -ll.vehAndGap.1 - ll.vehAndGap.0.get_vehicle_type().get_length_with_gap() - ego.get_vehicle_type().get_min_gap()
                                } else {
                                    ll.vehAndGap.1 + ego.get_vehicle_type().get_length()
                                };
                                result.add_follower(ll.vehAndGap.0, ego, gap, 0., -1);
                            }
                        }
                    }

                    for i in 0..first.num_sublanes() {
                        let mut v = if first.index_op(i) == ego { firstFront.index_op(i) } else { first.index_op(i) };
                        let mut agap = 0.;

                        if !v.is_null() && v != ego {
                            if !v.is_front_on_lane(next) {
                                // the front of v is already on divergent trajectory from the ego vehicle
                                // for which this method is called (in the context of MSLaneChanger).
                                // Therefore, technically v is not a follower but only an obstruction and
                                // the gap is not between the front of v and the back of ego
                                // but rather between the flank of v and the back of ego.
                                agap = it.length - next.get_length() + backOffset;
                                if MSGlobals::using_internal_lanes() {
                                    // ego should have left the intersection still occupied by v
                                    agap -= v.get_vehicle_type().get_min_gap();
                                }
                                let differentEdge = v.get_lane().myEdge != ego.get_lane().myEdge;
                                if agap > 0. && differentEdge {
                                    // Only if ego overlaps we treat v as if it were a real follower
                                    // Otherwise we ignore it and look for another follower
                                    if !getOppositeLeaders {
                                        // even if the vehicle is not a real
                                        // follower, it still forms a real
                                        // obstruction in opposite direction driving
                                        v = firstFront.index_op(i);
                                        if !v.is_null() && v != ego {
                                            agap = it.length - v.get_position_on_lane() + backOffset - v.get_vehicle_type().get_min_gap();
                                        } else {
                                            v = Ptr::null();
                                        }
                                    }
                                } else if differentEdge && result.has_vehicle(v) {
                                    // ignore this vehicle as it was already seen on another lane
                                    agap = 0.;
                                }
                            } else {
                                if !next.get_bidi_lane().is_null() && v.is_bidi_on(next) {
                                    agap = v.get_position_on_lane() + backOffset - v.get_vehicle_type().get_length_with_gap();
                                } else {
                                    agap = it.length - v.get_position_on_lane() + backOffset - v.get_vehicle_type().get_min_gap();
                                }
                                if !it.viaLink.have_priority() && !egoFurther.contains(&it.lane.myEdge)
                                    && ego.is_on_road() // during insertion, this can lead to collisions because ego's further lanes are not set (see #3053)
                                    && !ego.get_lane_change_model_2().is_opposite()
                                    && v.get_speed() < utils::common::std_defs::SUMO_CONST_HALTING_SPEED {
                                    // if v is stopped on a minor side road it should not block lane changing
                                    agap = if agap > 0.0 { agap } else { 0.0 };
                                }
                            }
                            result.add_follower(v, ego, agap, 0., i);
                        }
                    }
                    if it.length < searchDist {
                        let followers = next.get_incoming_lanes();
                        for j in followers.iter() {
                            if !visited.contains(&j.lane) && ((j.viaLink.have_priority() && !j.viaLink.is_turnaround())
                                || mLinkMode == MSLane_MinorLinkMode::FOLLOW_ALWAYS
                                || (mLinkMode == MSLane_MinorLinkMode::FOLLOW_ONCOMING && j.viaLink.get_direction() == LinkDirection::STRAIGHT)) {
                                visited.insert(j.lane);
                                newFound.push(MSLane_IncomingLaneInfo { lane: j.lane, length: j.length + it.length, viaLink: j.viaLink });
                            }
                        }
                    }
                }
                toExamine.clear();
                std::mem::swap(&mut newFound, &mut toExamine);
            }
        }
        // the sliced MSLeaderDistanceInfo base is returned, as in C++
        result.base
    }

    /// C++: `double MSLane::getMissingRearGap(const MSVehicle *, double, double) const` [const]
    /// return by how much further the leader must be inserted to avoid rear end collisions
    // double MSLane::getMissingRearGap(const MSVehicle* leader, double backOffset, double leaderSpeed) const {
    //     // this follows the same logic as getFollowerOnConsecutive. we do a tree
    //     // search and check for the vehicle with the largest missing rear gap within
    //     // relevant range
    //     double result = 0;
    //     const double leaderDecel = leader->getCarFollowModel().getMaxDecel();
    //     CLeaderDist followerInfo = getFollowersOnConsecutive(leader, backOffset, false)[0];
    //     const MSVehicle* v = followerInfo.first;
    //     if (v != nullptr) {
    //         result = v->getCarFollowModel().getSecureGap(v, leader, v->getSpeed(), leaderSpeed, leaderDecel) - followerInfo.second;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_missing_rear_gap(&self, leader: Ptr<MSVehicle>, backOffset: f64, leaderSpeed: f64) -> f64 {
        // this follows the same logic as getFollowerOnConsecutive. we do a tree
        // search and check for the vehicle with the largest missing rear gap within
        // relevant range
        let mut result = 0.;
        let leaderDecel = leader.get_car_follow_model().get_max_decel();
        let followerInfo = self.get_followers_on_consecutive(leader, backOffset, false, -1., MSLane_MinorLinkMode::FOLLOW_ALWAYS, false).index_op(0);
        let v = followerInfo.0;
        if !v.is_null() {
            result = v.get_car_follow_model().get_secure_gap(v, leader, v.get_speed(), leaderSpeed, leaderDecel) - followerInfo.1;
        }
        result
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getLeader(const MSVehicle *, const double, const std::vector<MSLane *> &, double, bool) const` [const]
    /// Returns the immediate leader of veh and the distance to veh starting on this lane
    // std::pair<MSVehicle* const, double>
    // MSLane::getLeader(const MSVehicle* veh, const double vehPos, const std::vector<MSLane*>& bestLaneConts, double dist, bool checkTmpVehicles) const {
    //     // get the leading vehicle for (shadow) veh
    //     // XXX this only works as long as all lanes of an edge have equal length
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(veh)) {
    //         std::cout << "   getLeader lane=" << getID() << " ego=" << veh->getID() << " vehs=" << toString(myVehicles) << " tmpVehs=" << toString(myTmpVehicles) << "\n";
    //     }
    // #endif
    //     if (checkTmpVehicles) {
    //         for (VehCont::const_iterator last = myTmpVehicles.begin(); last != myTmpVehicles.end(); ++last) {
    //             // XXX refactor leaderInfo to use a const vehicle all the way through the call hierarchy
    //             MSVehicle* pred = (MSVehicle*)*last;
    //             if (pred == veh) {
    //                 continue;
    //             }
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(veh)) {
    //                 std::cout << std::setprecision(gPrecision) << "   getLeader lane=" << getID() << " ego=" << veh->getID() << " egoPos=" << vehPos << " pred=" << pred->getID() << " predPos=" << pred->getPositionOnLane() << "\n";
    //             }
    // #endif
    //             if (pred->getPositionOnLane() >= vehPos) {
    //                 return std::pair<MSVehicle* const, double>(pred, pred->getBackPositionOnLane(this) - veh->getVehicleType().getMinGap() - vehPos);
    //             }
    //         }
    //     } else {
    //         for (AnyVehicleIterator last = anyVehiclesBegin(); last != anyVehiclesEnd(); ++last) {
    //             // XXX refactor leaderInfo to use a const vehicle all the way through the call hierarchy
    //             MSVehicle* pred = (MSVehicle*)*last;
    //             if (pred == veh) {
    //                 continue;
    //             }
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(veh)) {
    //                 std::cout << "   getLeader lane=" << getID() << " ego=" << veh->getID() << " egoPos=" << vehPos
    //                           << " pred=" << pred->getID() << " predPos=" << pred->getPositionOnLane(this) << " predBack=" << pred->getBackPositionOnLane(this) << "\n";
    //             }
    // #endif
    //             if (pred->getPositionOnLane(this) >= vehPos) {
    //                 if (MSGlobals::gLaneChangeDuration > 0
    //                         && pred->getLaneChangeModel().isOpposite()
    //                         && !pred->getLaneChangeModel().isChangingLanes()
    //                         && pred->getLaneChangeModel().getShadowLane() == this) {
    //                     // skip non-overlapping shadow
    //                     continue;
    //                 }
    //                 return std::pair<MSVehicle* const, double>(pred, pred->getBackPositionOnLane(this) - veh->getVehicleType().getMinGap() - vehPos);
    //             }
    //         }
    //     }
    //     // XXX from here on the code mirrors MSLaneChanger::getRealLeader
    //     if (bestLaneConts.size() > 0) {
    //         double seen = getLength() - vehPos;
    //         double speed = veh->getSpeed();
    //         if (dist < 0) {
    //             dist = veh->getCarFollowModel().brakeGap(speed) + veh->getVehicleType().getMinGap();
    //         }
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(veh)) {
    //             std::cout << "   getLeader lane=" << getID() << " seen=" << seen << " dist=" << dist << "\n";
    //         }
    // #endif
    //         if (seen > dist) {
    //             return std::pair<MSVehicle* const, double>(static_cast<MSVehicle*>(nullptr), -1);
    //         }
    //         return getLeaderOnConsecutive(dist, seen, speed, *veh, bestLaneConts);
    //     } else {
    //         return std::make_pair(static_cast<MSVehicle*>(nullptr), -1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_leader(&self, veh: Ptr<MSVehicle>, vehPos: f64, bestLaneConts: &Vec<Ptr<MSLane>>, dist: f64, checkTmpVehicles: bool) -> (Ptr<MSVehicle>, f64) {
        // get the leading vehicle for (shadow) veh
        // XXX this only works as long as all lanes of an edge have equal length
        if checkTmpVehicles {
            for last in self.myTmpVehicles.iter() {
                // XXX refactor leaderInfo to use a const vehicle all the way through the call hierarchy
                let pred = *last;
                if pred == veh {
                    continue;
                }
                if pred.get_position_on_lane() >= vehPos {
                    return (pred, pred.get_back_position_on_lane(Ptr::new(self)) - veh.get_vehicle_type().get_min_gap() - vehPos);
                }
            }
        } else {
            let mut last = self.any_vehicles_begin();
            let lastEnd = self.any_vehicles_end();
            while last.ne_op(&lastEnd) {
                // XXX refactor leaderInfo to use a const vehicle all the way through the call hierarchy
                let pred = last.mul_op();
                last.op__2();
                if pred == veh {
                    continue;
                }
                if pred.get_position_on_lane_2(Ptr::new(self)) >= vehPos {
                    if MSGlobals::lane_change_duration() > 0
                        && pred.get_lane_change_model_2().is_opposite()
                        && !pred.get_lane_change_model_2().is_changing_lanes()
                        && pred.get_lane_change_model_2().get_shadow_lane() == Ptr::new(self) {
                        // skip non-overlapping shadow
                        continue;
                    }
                    return (pred, pred.get_back_position_on_lane(Ptr::new(self)) - veh.get_vehicle_type().get_min_gap() - vehPos);
                }
            }
        }
        // XXX from here on the code mirrors MSLaneChanger::getRealLeader
        if bestLaneConts.len() > 0 {
            let seen = self.get_length() - vehPos;
            let speed = veh.get_speed();
            let mut dist = dist;
            if dist < 0. {
                dist = veh.get_car_follow_model().brake_gap(speed) + veh.get_vehicle_type().get_min_gap();
            }
            if seen > dist {
                return (Ptr::null(), -1.);
            }
            self.get_leader_on_consecutive(dist, seen, speed, &veh, bestLaneConts, true)
        } else {
            (Ptr::null(), -1.)
        }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getLeaderOnConsecutive(double, double, double, const MSVehicle &, const std::vector<MSLane *> &, bool) const` [const]
    /// Returns the immediate leader and the distance to him
    // std::pair<MSVehicle* const, double>
    // MSLane::getLeaderOnConsecutive(double dist, double seen, double speed, const MSVehicle& veh,
    //                                const std::vector<MSLane*>& bestLaneConts, bool considerCrossingFoes) const {
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(&veh)) {
    //         std::cout << "   getLeaderOnConsecutive lane=" << getID() << " ego=" << veh.getID() << " seen=" << seen << " dist=" << dist << " conts=" << toString(bestLaneConts) << "\n";
    //     }
    // #endif
    //     if (seen > dist && !isInternal()) {
    //         return std::make_pair(static_cast<MSVehicle*>(nullptr), -1);
    //     }
    //     int view = 1;
    //     // loop over following lanes
    //     if (myPartialVehicles.size() > 0) {
    //         // XXX
    //         MSVehicle* pred = myPartialVehicles.front();
    //         const double gap = seen - (getLength() - pred->getBackPositionOnLane(this)) - veh.getVehicleType().getMinGap();
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(&veh)) {
    //             std::cout << "    predGap=" << gap << " partials=" << toString(myPartialVehicles) << "\n";
    //         }
    // #endif
    //         // make sure pred is really a leader and not doing continous lane-changing behind ego
    //         if (gap > 0) {
    //             return std::pair<MSVehicle* const, double>(pred, gap);
    //         }
    //     }
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(&veh)) {
    //         gDebugFlag1 = true;
    //     }
    // #endif
    //     const MSLane* nextLane = this;
    //     do {
    //         nextLane->getVehiclesSecure(); // lock against running sim when called from GUI for time gap coloring
    //         // get the next link used
    //         std::vector<MSLink*>::const_iterator link = succLinkSec(veh, view, *nextLane, bestLaneConts);
    //         if (nextLane->isLinkEnd(link) && view < veh.getRoute().size() - veh.getRoutePosition()) {
    //             const MSEdge* nextEdge = *(veh.getCurrentRouteEdge() + view);
    //             if (nextEdge->getNumLanes() == 1) {
    //                 // lanes are unambiguous on the next route edge, continue beyond bestLaneConts
    //                 for (link = nextLane->getLinkCont().begin(); link < nextLane->getLinkCont().end(); link++) {
    //                     if ((*link)->getLane() == nextEdge->getLanes().front()) {
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         if (nextLane->isLinkEnd(link)) {
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(&veh)) {
    //                 std::cout << "    cannot continue after nextLane=" << nextLane->getID() << "\n";
    //             }
    // #endif
    //             nextLane->releaseVehicles();
    //             break;
    //         }
    //         // check for link leaders
    //         const bool laneChanging = veh.getLane() != this;
    //         const MSLink::LinkLeaders linkLeaders = (*link)->getLeaderInfo(&veh, seen);
    //         nextLane->releaseVehicles();
    //         if (linkLeaders.size() > 0) {
    //             std::pair<MSVehicle*, double> result;
    //             double shortestGap = std::numeric_limits<double>::max();
    //             for (auto ll : linkLeaders) {
    //                 double gap = ll.vehAndGap.second;
    //                 MSVehicle* lVeh = ll.vehAndGap.first;
    //                 if (lVeh != nullptr) {
    //                     // leader is a vehicle, not a pedestrian
    //                     gap += lVeh->getCarFollowModel().brakeGap(lVeh->getSpeed(), lVeh->getCarFollowModel().getMaxDecel(), 0);
    //                 }
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(&veh)) {
    //                     std::cout << "      linkLeader candidate " << Named::getIDSecure(lVeh)
    //                               << " isLeader=" << veh.isLeader(*link, lVeh, ll.vehAndGap.second)
    //                               << " gap=" << ll.vehAndGap.second
    //                               << " gap+brakeing=" << gap
    //                               << "\n";
    //                 }
    // #endif
    //                 // skip vehicles which do not share the outgoing edge (to get only real leader vehicles in TraCI #13842)
    //                 if (!considerCrossingFoes && !ll.sameTarget()) {
    //                     continue;
    //                 }
    //                 // in the context of lane-changing, all candidates are leaders
    //                 if (lVeh != nullptr && !laneChanging && !veh.isLeader(*link, lVeh, ll.vehAndGap.second)) {
    //                     continue;
    //                 }
    //                 if (gap < shortestGap) {
    //                     shortestGap = gap;
    //                     if (ll.vehAndGap.second < 0 && !MSGlobals::gComputeLC) {
    //                         // can always continue up to the stop line or crossing point
    //                         // @todo: figure out whether this should also impact lane changing
    //                         ll.vehAndGap.second = MAX2(seen - nextLane->getLength(), ll.distToCrossing);
    //                     }
    //                     result = ll.vehAndGap;
    //                 }
    //             }
    //             if (shortestGap != std::numeric_limits<double>::max()) {
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(&veh)) {
    //                     std::cout << "    found linkLeader after nextLane=" << nextLane->getID() << "\n";
    //                     gDebugFlag1 = false;
    //                 }
    // #endif
    //                 return result;
    //             }
    //         }
    //         bool nextInternal = (*link)->getViaLane() != nullptr;
    //         nextLane = (*link)->getViaLaneOrLane();
    //         if (nextLane == nullptr) {
    //             break;
    //         }
    //         nextLane->getVehiclesSecure(); // lock against running sim when called from GUI for time gap coloring
    //         MSVehicle* leader = nextLane->getLastAnyVehicle();
    //         if (leader != nullptr) {
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(&veh)) {
    //                 std::cout << "    found leader " << leader->getID() << " on nextLane=" << nextLane->getID() << "\n";
    //             }
    // #endif
    //             const double leaderDist = seen + leader->getBackPositionOnLane(nextLane) - veh.getVehicleType().getMinGap();
    //             nextLane->releaseVehicles();
    //             return std::make_pair(leader, leaderDist);
    //         }
    //         nextLane->releaseVehicles();
    //         if (nextLane->getVehicleMaxSpeed(&veh) < speed) {
    //             dist = veh.getCarFollowModel().brakeGap(nextLane->getVehicleMaxSpeed(&veh));
    //         }
    //         seen += nextLane->getLength();
    //         if (!nextInternal) {
    //             view++;
    //         }
    //     } while (seen <= dist || nextLane->isInternal());
    // #ifdef DEBUG_CONTEXT
    //     gDebugFlag1 = false;
    // #endif
    //     return std::make_pair(static_cast<MSVehicle*>(nullptr), -1);
    // }
    #[allow(unused_variables)]
    pub fn get_leader_on_consecutive(&self, dist: f64, seen: f64, speed: f64, veh: &MSVehicle, bestLaneConts: &Vec<Ptr<MSLane>>, considerCrossingFoes: bool) -> (Ptr<MSVehicle>, f64) {
        let mut dist = dist;
        let mut seen = seen;
        if seen > dist && !self.is_internal() {
            return (Ptr::null(), -1.);
        }
        let mut view: i32 = 1;
        // loop over following lanes
        if self.myPartialVehicles.len() > 0 {
            // XXX
            let pred = self.myPartialVehicles[0];
            let gap = seen - (self.get_length() - pred.get_back_position_on_lane(Ptr::new(self))) - veh.get_vehicle_type().get_min_gap();
            // make sure pred is really a leader and not doing continous lane-changing behind ego
            if gap > 0. {
                return (pred, gap);
            }
        }
        let mut nextLane: Ptr<MSLane> = Ptr::new(self);
        loop {
            nextLane.get_vehicles_secure(); // lock against running sim when called from GUI for time gap coloring
            // get the next link used
            let mut link: usize = MSLane::succ_link_sec(veh, view, &nextLane, bestLaneConts);
            if nextLane.is_link_end(&mut link) && view < veh.get_route().size() - veh.get_route_position() {
                let nextEdge = veh.succ_edge(view);
                if nextEdge.get_num_lanes() == 1 {
                    // lanes are unambiguous on the next route edge, continue beyond bestLaneConts
                    link = 0;
                    while link < nextLane.get_link_cont().len() {
                        if nextLane.get_link_cont()[link].get_lane() == nextEdge.get_lanes()[0] {
                            break;
                        }
                        link += 1;
                    }
                }
            }
            if nextLane.is_link_end(&mut link) {
                nextLane.release_vehicles();
                break;
            }
            // check for link leaders
            let laneChanging = veh.get_lane() != Ptr::new(self);
            let linkLeaders = nextLane.get_link_cont()[link].get_leader_info(Ptr::new(veh), seen, Ptr::null(), false);
            nextLane.release_vehicles();
            if linkLeaders.len() > 0 {
                let mut result: (Ptr<MSVehicle>, f64) = (Ptr::null(), 0.);
                let mut shortestGap = f64::MAX;
                for ll in linkLeaders.iter() {
                    // C++ iterates over a copy: work on local copies of the mutable fields
                    let mut vehAndGap = ll.vehAndGap;
                    let mut gap = vehAndGap.1;
                    let lVeh = vehAndGap.0;
                    if !lVeh.is_null() {
                        // leader is a vehicle, not a pedestrian
                        gap += lVeh.get_car_follow_model().brake_gap_2(lVeh.get_speed(), lVeh.get_car_follow_model().get_max_decel(), 0.);
                    }
                    // skip vehicles which do not share the outgoing edge (to get only real leader vehicles in TraCI #13842)
                    if !considerCrossingFoes && !ll.same_target() {
                        continue;
                    }
                    // in the context of lane-changing, all candidates are leaders
                    if !lVeh.is_null() && !laneChanging && !veh.is_leader(nextLane.get_link_cont()[link], lVeh, vehAndGap.1) {
                        continue;
                    }
                    if gap < shortestGap {
                        shortestGap = gap;
                        if vehAndGap.1 < 0. && !MSGlobals::compute_lc() {
                            // can always continue up to the stop line or crossing point
                            // @todo: figure out whether this should also impact lane changing
                            let a = seen - nextLane.get_length();
                            let b = ll.distToCrossing;
                            vehAndGap.1 = if a > b { a } else { b };
                        }
                        result = vehAndGap;
                    }
                }
                if shortestGap != f64::MAX {
                    return result;
                }
            }
            let nextInternal = !nextLane.get_link_cont()[link].get_via_lane().is_null();
            nextLane = nextLane.get_link_cont()[link].get_via_lane_or_lane();
            if nextLane.is_null() {
                break;
            }
            nextLane.get_vehicles_secure();
            let leader = nextLane.get_last_any_vehicle();
            if !leader.is_null() {
                let leaderDist = seen + leader.get_back_position_on_lane(nextLane) - veh.get_vehicle_type().get_min_gap();
                nextLane.release_vehicles();
                return (leader, leaderDist);
            }
            nextLane.release_vehicles();
            if nextLane.get_vehicle_max_speed(Ptr::new(veh).cast::<SUMOTrafficObject>()) < speed {
                dist = veh.get_car_follow_model().brake_gap(nextLane.get_vehicle_max_speed(Ptr::new(veh).cast::<SUMOTrafficObject>()));
            }
            seen += nextLane.get_length();
            if !nextInternal {
                view += 1;
            }
            if !(seen <= dist || nextLane.is_internal()) {
                break;
            }
        }
        (Ptr::null(), -1.)
    }

    /// C++: `void MSLane::getLeadersOnConsecutive(double, double, double, const MSVehicle *, const std::vector<MSLane *> &, MSLeaderDistanceInfo &, bool) const` [const]
    /// Returns the immediate leaders and the distance to them (as getLeaderOnConsecutive but for the sublane case)
    // void
    // MSLane::getLeadersOnConsecutive(double dist, double seen, double speed, const MSVehicle* ego,
    //                                 const std::vector<MSLane*>& bestLaneConts, MSLeaderDistanceInfo& result,
    //                                 bool oppositeDirection) const {
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(ego)) {
    //         std::cout << "   getLeadersOnConsecutive " << getID() << " ego=" << Named::getIDSecure(ego) << " dist=" << dist << " seen=" << seen << "\n";
    //     }
    // #endif
    //     if (seen > dist && !(isInternal() && MSGlobals::gComputeLC)) {
    //         return;
    //     }
    //     // check partial vehicles (they might be on a different route and thus not
    //     // found when iterating along bestLaneConts)
    //     for (VehCont::const_iterator it = myPartialVehicles.begin(); it != myPartialVehicles.end(); ++it) {
    //         MSVehicle* veh = *it;
    //         if (!veh->isFrontOnLane(this)) {
    //             result.addLeader(veh, seen, veh->getLatOffset(this));
    //         } else {
    //             break;
    //         }
    //     }
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(ego)) {
    //         gDebugFlag1 = true;
    //     }
    // #endif
    //     const MSLane* nextLane = this;
    //     int view = 1;
    //     // loop over following lanes
    //     while ((seen < dist && result.numFreeSublanes() > 0) || nextLane->isInternal()) {
    //         if (nextLane != this) {
    //             seen += nextLane->getLength();
    //         }
    //         // get the next link used
    //         bool nextInternal = false;
    //         if (oppositeDirection) {
    //             if (view >= (int)bestLaneConts.size()) {
    //                 break;
    //             }
    //             nextLane = bestLaneConts[view];
    //         } else {
    //             std::vector<MSLink*>::const_iterator link = succLinkSec(*ego, view, *nextLane, bestLaneConts);
    //             if (nextLane->isLinkEnd(link)) {
    //                 break;
    //             }
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(ego)) {
    //                 std::cout << "    link=" << (*link)->getDescription() << " debugflag=" << gDebugFlag1 << "\n";
    //             }
    // #endif
    //             // check for link leaders
    //             const MSLink::LinkLeaders linkLeaders = (*link)->getLeaderInfo(ego, seen);
    //             for (const MSLink::LinkLeader& ll : linkLeaders) {
    //                 MSVehicle* veh = ll.vehAndGap.first;
    //                 // in the context of lane changing all junction leader candidates must be respected
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(ego)) {
    //                     std::cout << "   linkleader=" << veh->getID() << " gap=" << ll.vehAndGap.second << " leaderOffset=" << ll.latOffset << " flags=" << ll.llFlags << "\n";
    //                 }
    // #endif
    //                 if (veh != 0 && (ego->isLeader(*link, veh, ll.vehAndGap.second)
    //                                  || (MSGlobals::gComputeLC
    //                                      && veh->getPosition().distanceTo2D(ego->getPosition()) - veh->getVehicleType().getMinGap() - ego->getVehicleType().getLength()
    //                                      < veh->getCarFollowModel().brakeGap(veh->getSpeed())))) {
    //                     if (ll.sameTarget() || ll.sameSource()) {
    //                         result.addLeader(veh, ll.vehAndGap.second, ll.latOffset);
    // #ifdef DEBUG_CONTEXT
    //                         if (DEBUG_COND2(ego)) {
    //                             std::cout << "   added selective: result=" << result.toString() << "\n";
    //                         }
    // #endif
    //                     } else {
    //                         // add link leader to all sublanes and return
    //                         for (int i = 0; i < result.numSublanes(); ++i) {
    //                             result.addLeader(veh, ll.vehAndGap.second, 0, i);
    //                         }
    // #ifdef DEBUG_CONTEXT
    //                         if (DEBUG_COND2(ego)) {
    //                             std::cout << "   added allSublanes: result=" << result.toString() << "\n";
    //                         }
    // #endif
    //                     }
    //                 } // XXX else, deal with pedestrians
    //             }
    //             nextInternal = (*link)->getViaLane() != nullptr;
    //             nextLane = (*link)->getViaLaneOrLane();
    //             if (nextLane == nullptr) {
    //                 break;
    //             }
    //         }
    //
    //         MSLeaderInfo leaders = nextLane->getLastVehicleInformation(nullptr, 0, 0, false);
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(ego)) {
    //             std::cout << SIMTIME << " getLeadersOnConsecutive lane=" << getID() << " nextLane=" << nextLane->getID() << " leaders=" << leaders.toString() << "\n";
    //         }
    // #endif
    //         // @todo check alignment issues if the lane width changes
    //         const int iMax = MIN2(leaders.numSublanes(), result.numSublanes());
    //         for (int i = 0; i < iMax; ++i) {
    //             const MSVehicle* veh = leaders[i];
    //             if (veh != nullptr) {
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(ego)) std::cout << "   lead=" << veh->getID()
    //                                                     << " seen=" << seen
    //                                                     << " minGap=" << ego->getVehicleType().getMinGap()
    //                                                     << " backPos=" << veh->getBackPositionOnLane(nextLane)
    //                                                     << " gap=" << seen - ego->getVehicleType().getMinGap() + veh->getBackPositionOnLane(nextLane)
    //                                                     << "\n";
    // #endif
    //                 result.addLeader(veh, seen - ego->getVehicleType().getMinGap() + veh->getBackPositionOnLane(nextLane), 0, i);
    //             }
    //         }
    //
    //         if (nextLane->getVehicleMaxSpeed(ego) < speed) {
    //             dist = ego->getCarFollowModel().brakeGap(nextLane->getVehicleMaxSpeed(ego));
    //         }
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(ego)) {
    //             std::cout << "   newDist=" << dist << " newSeen=" << seen << "\n";
    //         }
    // #endif
    //         if (!nextInternal) {
    //             view++;
    //         }
    //     }
    // #ifdef DEBUG_CONTEXT
    //     gDebugFlag1 = false;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn get_leaders_on_consecutive(&self, dist: f64, seen: f64, speed: f64, ego: Ptr<MSVehicle>, bestLaneConts: &Vec<Ptr<MSLane>>, result: &mut MSLeaderDistanceInfo, oppositeDirection: bool) {
        let mut seen = seen;
        let mut dist = dist;
        if seen > dist && !(self.is_internal() && MSGlobals::compute_lc()) {
            return;
        }
        // check partial vehicles (they might be on a different route and thus not
        // found when iterating along bestLaneConts)
        for it in self.myPartialVehicles.iter() {
            let veh = *it;
            if !veh.is_front_on_lane(Ptr::new(self)) {
                result.add_leader(veh, seen, veh.get_lat_offset(Ptr::new(self)), -1);
            } else {
                break;
            }
        }
        let mut nextLane: Ptr<MSLane> = Ptr::new(self);
        let mut view: i32 = 1;
        // loop over following lanes
        while (seen < dist && result.num_free_sublanes() > 0) || nextLane.is_internal() {
            if nextLane != Ptr::new(self) {
                seen += nextLane.get_length();
            }
            // get the next link used
            let mut nextInternal = false;
            if oppositeDirection {
                if view >= bestLaneConts.len() as i32 {
                    break;
                }
                nextLane = bestLaneConts[view as usize];
            } else {
                let mut link: usize = MSLane::succ_link_sec(&ego, view, &nextLane, bestLaneConts);
                if nextLane.is_link_end(&mut link) {
                    break;
                }
                // check for link leaders
                let linkLeaders = nextLane.get_link_cont()[link].get_leader_info(ego, seen, Ptr::null(), false);
                for ll in linkLeaders.iter() {
                    let veh = ll.vehAndGap.0;
                    // in the context of lane changing all junction leader candidates must be respected
                    if !veh.is_null() && (ego.is_leader(nextLane.get_link_cont()[link], veh, ll.vehAndGap.1)
                        || (MSGlobals::compute_lc()
                            && veh.get_position(0.).distance_to2d(&ego.get_position(0.)) - veh.get_vehicle_type().get_min_gap() - ego.get_vehicle_type().get_length()
                                < veh.get_car_follow_model().brake_gap(veh.get_speed()))) {
                        if ll.same_target() || ll.same_source() {
                            result.add_leader(veh, ll.vehAndGap.1, ll.latOffset, -1);
                        } else {
                            // add link leader to all sublanes and return
                            for i in 0..result.num_sublanes() {
                                result.add_leader(veh, ll.vehAndGap.1, 0., i);
                            }
                        }
                    } // XXX else, deal with pedestrians
                }
                nextInternal = !nextLane.get_link_cont()[link].get_via_lane().is_null();
                nextLane = nextLane.get_link_cont()[link].get_via_lane_or_lane();
                if nextLane.is_null() {
                    break;
                }
            }
            let leaders = nextLane.get_last_vehicle_information(Ptr::null(), 0., 0., false, Ptr::null());
            // @todo check alignment issues if the lane width changes
            let iMax = if leaders.num_sublanes() < result.num_sublanes() { leaders.num_sublanes() } else { result.num_sublanes() };
            for i in 0..iMax {
                let veh = leaders.index_op(i);
                if !veh.is_null() {
                    result.add_leader(veh, seen - ego.get_vehicle_type().get_min_gap() + veh.get_back_position_on_lane(nextLane), 0., i);
                }
            }
            if nextLane.get_vehicle_max_speed(ego.cast::<SUMOTrafficObject>()) < speed {
                dist = ego.get_car_follow_model().brake_gap(nextLane.get_vehicle_max_speed(ego.cast::<SUMOTrafficObject>()));
            }
            if !nextInternal {
                view += 1;
            }
        }
    }

    /// C++: `void MSLane::addLeaders(const MSVehicle *, double, MSLeaderDistanceInfo &, bool)`
    /// get leaders for ego on the given lane
    // void
    // MSLane::addLeaders(const MSVehicle* vehicle, double vehPos, MSLeaderDistanceInfo& result, bool opposite) {
    //     // if there are vehicles on the target lane with the same position as ego,
    //     // they may not have been added to 'ahead' yet
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //         std::cout << " addLeaders lane=" << getID() << " veh=" << vehicle->getID() << " vehPos=" << vehPos << " opposite=" << opposite << "\n";
    //     }
    // #endif
    //     const MSLeaderInfo& aheadSamePos = getLastVehicleInformation(nullptr, 0, vehPos, false, vehicle);
    //     for (int i = 0; i < aheadSamePos.numSublanes(); ++i) {
    //         const MSVehicle* veh = aheadSamePos[i];
    //         if (veh != nullptr && veh != vehicle) {
    //             const double gap = veh->getBackPositionOnLane(this) - vehPos - vehicle->getVehicleType().getMinGap();
    // #ifdef DEBUG_SURROUNDING
    //             if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //                 std::cout << " further lead=" << veh->getID() << " leadBack=" << veh->getBackPositionOnLane(this) << " gap=" << gap << "\n";
    //             }
    // #endif
    //             result.addLeader(veh, gap, 0, i);
    //         }
    //     }
    //
    //     // we must consider linkLeaders (via getLeadersOnConsecutive) while on a junction
    //     if (result.numFreeSublanes() > 0 || isInternal()) {
    //         double seen = vehicle->getLane()->getLength() - vehPos;
    //         double speed = vehicle->getSpeed();
    //         // leader vehicle could be link leader on the next junction
    //         double dist = MAX2(vehicle->getCarFollowModel().brakeGap(speed), 10.0) + vehicle->getVehicleType().getMinGap();
    //         if (getBidiLane() != nullptr) {
    //             dist = MAX2(dist, myMaxSpeed * 20);
    //         }
    //         // check for link leaders when on internal
    //         if (seen > dist && !(isInternal() && MSGlobals::gComputeLC)) {
    // #ifdef DEBUG_SURROUNDING
    //             if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //                 std::cout << " aborting forward search. dist=" << dist << " seen=" << seen << "\n";
    //             }
    // #endif
    //             return;
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //             std::cout << " add consecutive before=" << result.toString() << " seen=" << seen << " dist=" << dist;
    //         }
    // #endif
    //         if (opposite) {
    //             const std::vector<MSLane*> bestLaneConts = vehicle->getUpstreamOppositeLanes();
    // #ifdef DEBUG_SURROUNDING
    //             if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //                 std::cout << " upstreamOpposite=" << toString(bestLaneConts);
    //             }
    // #endif
    //             getLeadersOnConsecutive(dist, seen, speed, vehicle, bestLaneConts, result, opposite);
    //         } else {
    //             const std::vector<MSLane*>& bestLaneConts = vehicle->getBestLanesContinuation(this);
    //             getLeadersOnConsecutive(dist, seen, speed, vehicle, bestLaneConts, result);
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         if (DEBUG_COND || DEBUG_COND2(vehicle)) {
    //             std::cout << " after=" << result.toString() << "\n";
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_leaders(&mut self, vehicle: Ptr<MSVehicle>, vehPos: f64, result: &mut MSLeaderDistanceInfo, oppositeDirection: bool) {
        // if there are vehicles on the target lane with the same position as ego,
        // they may not have been added to 'ahead' yet
        let aheadSamePos = self.get_last_vehicle_information(Ptr::null(), 0., vehPos, false, vehicle);
        for i in 0..aheadSamePos.num_sublanes() {
            let veh = aheadSamePos.index_op(i);
            if !veh.is_null() && veh != vehicle {
                let gap = veh.get_back_position_on_lane(Ptr::new(self)) - vehPos - vehicle.get_vehicle_type().get_min_gap();
                result.add_leader(veh, gap, 0., i);
            }
        }
        // we must consider linkLeaders (via getLeadersOnConsecutive) while on a junction
        if result.num_free_sublanes() > 0 || self.is_internal() {
            let seen = vehicle.get_lane().get_length() - vehPos;
            let speed = vehicle.get_speed();
            // leader vehicle could be link leader on the next junction
            let brake = vehicle.get_car_follow_model().brake_gap(speed);
            let mut dist = (if brake > 10.0 { brake } else { 10.0 }) + vehicle.get_vehicle_type().get_min_gap();
            if !self.get_bidi_lane().is_null() {
                dist = if dist > self.myMaxSpeed * 20. { dist } else { self.myMaxSpeed * 20. };
            }
            // check for link leaders when on internal
            if seen > dist && !(self.is_internal() && MSGlobals::compute_lc()) {
                return;
            }
            if oppositeDirection {
                let bestLaneConts = vehicle.get_upstream_opposite_lanes();
                self.get_leaders_on_consecutive(dist, seen, speed, vehicle, &bestLaneConts, result, oppositeDirection);
            } else {
                let bestLaneConts = vehicle.get_best_lanes_continuation_2(Ptr::new(self));
                self.get_leaders_on_consecutive(dist, seen, speed, vehicle, bestLaneConts, result, false);
            }
        }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getCriticalLeader(double, double, double, const MSVehicle &) const` [const]
    /// Returns the most dangerous leader and the distance to him
    // std::pair<MSVehicle* const, double>
    // MSLane::getCriticalLeader(double dist, double seen, double speed, const MSVehicle& veh) const {
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(&veh)) {
    //         std::cout << SIMTIME << " getCriticalLeader. lane=" << getID() << " veh=" << veh.getID() << "\n";
    //     }
    // #endif
    //     const std::vector<MSLane*>& bestLaneConts = veh.getBestLanesContinuation(this);
    //     std::pair<MSVehicle*, double> result = std::make_pair(static_cast<MSVehicle*>(nullptr), -1);
    //     double safeSpeed = std::numeric_limits<double>::max();
    //     int view = 1;
    //     // loop over following lanes
    //     // @note: we don't check the partial occupator for this lane since it was
    //     // already checked in MSLaneChanger::getRealLeader()
    //     const MSLane* nextLane = this;
    //     SUMOTime arrivalTime = MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(seen / MAX2(speed, NUMERICAL_EPS));
    //     do {
    //         // get the next link used
    //         std::vector<MSLink*>::const_iterator link = succLinkSec(veh, view, *nextLane, bestLaneConts);
    //         if (nextLane->isLinkEnd(link) || !(*link)->opened(arrivalTime, speed, speed, veh.getVehicleType().getLength(),
    //                 veh.getImpatience(), veh.getCarFollowModel().getMaxDecel(), 0, veh.getLateralPositionOnLane(), nullptr, false, &veh) || (*link)->haveRed()) {
    //             return result;
    //         }
    //         // check for link leaders
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(&veh)) {
    //             gDebugFlag1 = true;    // See MSLink::getLeaderInfo
    //         }
    // #endif
    //         const MSLink::LinkLeaders linkLeaders = (*link)->getLeaderInfo(&veh, seen);
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(&veh)) {
    //             gDebugFlag1 = false;    // See MSLink::getLeaderInfo
    //         }
    // #endif
    //         for (MSLink::LinkLeaders::const_iterator it = linkLeaders.begin(); it != linkLeaders.end(); ++it) {
    //             const MSVehicle* leader = (*it).vehAndGap.first;
    //             if (leader != nullptr && leader != result.first) {
    //                 // XXX ignoring pedestrians here!
    //                 // XXX ignoring the fact that the link leader may alread by following us
    //                 // XXX ignoring the fact that we may drive up to the crossing point
    //                 double tmpSpeed = safeSpeed;
    //                 veh.adaptToJunctionLeader((*it).vehAndGap, seen, nullptr, nextLane, tmpSpeed, tmpSpeed, (*it).distToCrossing);
    // #ifdef DEBUG_CONTEXT
    //                 if (DEBUG_COND2(&veh)) {
    //                     std::cout << "    linkLeader=" << leader->getID() << " gap=" << result.second << " tmpSpeed=" << tmpSpeed << " safeSpeed=" << safeSpeed << "\n";
    //                 }
    // #endif
    //                 if (tmpSpeed < safeSpeed) {
    //                     safeSpeed = tmpSpeed;
    //                     result = (*it).vehAndGap;
    //                 }
    //             }
    //         }
    //         bool nextInternal = (*link)->getViaLane() != nullptr;
    //         nextLane = (*link)->getViaLaneOrLane();
    //         if (nextLane == nullptr) {
    //             break;
    //         }
    //         MSVehicle* leader = nextLane->getLastAnyVehicle();
    //         if (leader != nullptr && leader != result.first) {
    //             const double gap = seen + leader->getBackPositionOnLane(nextLane) - veh.getVehicleType().getMinGap();
    //             const double tmpSpeed = veh.getCarFollowModel().insertionFollowSpeed(&veh, speed, gap, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel(), leader);
    //             if (tmpSpeed < safeSpeed) {
    //                 safeSpeed = tmpSpeed;
    //                 result = std::make_pair(leader, gap);
    //             }
    //         }
    //         if (nextLane->getVehicleMaxSpeed(&veh) < speed) {
    //             dist = veh.getCarFollowModel().brakeGap(nextLane->getVehicleMaxSpeed(&veh));
    //         }
    //         seen += nextLane->getLength();
    //         if (seen <= dist) {
    //             // delaying the update of arrivalTime and making it conditional to avoid possible integer overflows
    //             arrivalTime += TIME2STEPS(nextLane->getLength() / MAX2(speed, NUMERICAL_EPS));
    //         }
    //         if (!nextInternal) {
    //             view++;
    //         }
    //     } while (seen <= dist || nextLane->isInternal());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_critical_leader(&self, dist: f64, seen: f64, speed: f64, veh: &MSVehicle) -> (Ptr<MSVehicle>, f64) {
        let bestLaneConts = veh.get_best_lanes_continuation_2(Ptr::new(self));
        let mut result: (Ptr<MSVehicle>, f64) = (Ptr::null(), -1.);
        let mut safeSpeed = f64::MAX;
        let mut view: i32 = 1;
        let mut dist = dist;
        let mut seen = seen;
        // loop over following lanes
        // @note: we don't check the partial occupator for this lane since it was
        // already checked in MSLaneChanger::getRealLeader()
        let mut nextLane: Ptr<MSLane> = Ptr::new(self);
        let mut arrivalTime: SUMOTime = MSNet::get_instance().get_current_time_step()
            + utils::common::time::time2steps(seen / (if speed > NUMERICAL_EPS { speed } else { NUMERICAL_EPS }));
        loop {
            // get the next link used
            let mut link: usize = MSLane::succ_link_sec(veh, view, &nextLane, bestLaneConts);
            if nextLane.is_link_end(&mut link)
                || !nextLane.get_link_cont()[link].opened(arrivalTime, speed, speed, veh.get_vehicle_type().get_length(),
                    veh.get_impatience(), veh.get_car_follow_model().get_max_decel(), 0, veh.get_lateral_position_on_lane(), Ptr::null(), false, Ptr::new(veh).cast::<SUMOTrafficObject>(), -1.)
                || nextLane.get_link_cont()[link].have_red() {
                return result;
            }
            // check for link leaders
            let linkLeaders = nextLane.get_link_cont()[link].get_leader_info(Ptr::new(veh), seen, Ptr::null(), false);
            for it in linkLeaders.iter() {
                let leader = it.vehAndGap.0;
                if !leader.is_null() && leader != result.0 {
                    // XXX ignoring pedestrians here!
                    // XXX ignoring the fact that the link leader may alread by following us
                    // XXX ignoring the fact that we may drive up to the crossing point
                    let mut tmpSpeed = safeSpeed;
                    // shortcut: C++ passes tmpSpeed as both v and vLinkPass (aliased references), two separate locals here, merged back into tmpSpeed afterwards, upgrade when adaptToJunctionLeader takes one mutable state
                    let mut v = tmpSpeed;
                    let mut vLinkPass = tmpSpeed;
                    veh.adapt_to_junction_leader(it.vehAndGap, seen, Ptr::null(), nextLane, &mut v, &mut vLinkPass, it.distToCrossing);
                    tmpSpeed = v;
                    if tmpSpeed < safeSpeed {
                        safeSpeed = tmpSpeed;
                        result = it.vehAndGap;
                    }
                }
            }
            let nextInternal = !nextLane.get_link_cont()[link].get_via_lane().is_null();
            nextLane = nextLane.get_link_cont()[link].get_via_lane_or_lane();
            if nextLane.is_null() {
                break;
            }
            let leader = nextLane.get_last_any_vehicle();
            if !leader.is_null() && leader != result.0 {
                let gap = seen + leader.get_back_position_on_lane(nextLane) - veh.get_vehicle_type().get_min_gap();
                let tmpSpeed = veh.get_car_follow_model().insertion_follow_speed(Ptr::new(veh), speed, gap, leader.get_speed(), leader.get_car_follow_model().get_max_decel(), leader);
                if tmpSpeed < safeSpeed {
                    safeSpeed = tmpSpeed;
                    result = (leader, gap);
                }
            }
            if nextLane.get_vehicle_max_speed(Ptr::new(veh).cast::<SUMOTrafficObject>()) < speed {
                dist = veh.get_car_follow_model().brake_gap(nextLane.get_vehicle_max_speed(Ptr::new(veh).cast::<SUMOTrafficObject>()));
            }
            seen += nextLane.get_length();
            if seen <= dist {
                // delaying the update of arrivalTime and making it conditional to avoid possible integer overflows
                arrivalTime += utils::common::time::time2steps(nextLane.get_length() / (if speed > NUMERICAL_EPS { speed } else { NUMERICAL_EPS }));
            }
            if !nextInternal {
                view += 1;
            }
            if !(seen <= dist || nextLane.is_internal()) {
                break;
            }
        }
        result
    }

    /// C++: `MSVehicle * MSLane::getPartialBehind(const MSVehicle *) const` [const]
    // MSVehicle*
    // MSLane::getPartialBehind(const MSVehicle* ego) const {
    //     for (VehCont::const_reverse_iterator i = myPartialVehicles.rbegin(); i != myPartialVehicles.rend(); ++i) {
    //         MSVehicle* veh = *i;
    //         if (veh->isFrontOnLane(this)
    //                 && veh != ego
    //                 && veh->getPositionOnLane() <= ego->getPositionOnLane()) {
    // #ifdef DEBUG_CONTEXT
    //             if (DEBUG_COND2(ego)) {
    //                 std::cout << SIMTIME << " getPartialBehind lane=" << getID() << " ego=" << ego->getID() << " found=" << veh->getID() << "\n";
    //             }
    // #endif
    //             return veh;
    //         }
    //     }
    // #ifdef DEBUG_CONTEXT
    //     if (DEBUG_COND2(ego)) {
    //         std::cout << SIMTIME << " getPartialBehind lane=" << getID() << " ego=" << ego->getID() << " nothing found. partials=" << toString(myPartialVehicles) << "\n";
    //     }
    // #endif
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_partial_behind(&self, ego: Ptr<MSVehicle>) -> Ptr<MSVehicle> {
        for i in self.myPartialVehicles.iter().rev() {
            let veh = *i;
            if veh.is_front_on_lane(Ptr::new(self))
                && veh != ego
                && veh.get_position_on_lane() <= ego.get_position_on_lane() {
                return veh;
            }
        }
        Ptr::null()
    }

    /// C++: `MSLeaderInfo MSLane::getPartialBeyond() const` [const]
    /// get all vehicles that are inlapping from consecutive edges
    // MSLeaderInfo
    // MSLane::getPartialBeyond() const {
    //     MSLeaderInfo result(myWidth);
    //     for (VehCont::const_iterator it = myPartialVehicles.begin(); it != myPartialVehicles.end(); ++it) {
    //         MSVehicle* veh = *it;
    //         if (!veh->isFrontOnLane(this)) {
    //             result.addLeader(veh, false, veh->getLatOffset(this));
    //         } else {
    //             break;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_partial_beyond(&self) -> MSLeaderInfo {
        let mut result = MSLeaderInfo::new(self.myWidth, Ptr::null(), 0.);
        for it in self.myPartialVehicles.iter() {
            let veh = *it;
            if !veh.is_front_on_lane(Ptr::new(self)) {
                result.add_leader(veh, false, veh.get_lat_offset(Ptr::new(self)));
            } else {
                break;
            }
        }
        result
    }

    /// C++: `std::set<MSVehicle *> MSLane::getSurroundingVehicles(double, double, double, std::shared_ptr<LaneCoverageInfo>) const` [const]
    /// Returns all vehicles closer than downstreamDist along the road network starting on the given position. Predecessor lanes are searched upstream for the given upstreamDistance.
    // std::set<MSVehicle*>
    // MSLane::getSurroundingVehicles(double startPos, double downstreamDist, double upstreamDist, std::shared_ptr<LaneCoverageInfo> checkedLanes) const {
    //     assert(checkedLanes != nullptr);
    //     if (checkedLanes->find(this) != checkedLanes->end()) {
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "Skipping previously scanned lane: " << getID() << std::endl;
    // #endif
    //         return std::set<MSVehicle*>();
    //     } else {
    //         // Add this lane's coverage to the lane coverage info
    //         (*checkedLanes)[this] = std::make_pair(MAX2(0.0, startPos - upstreamDist), MIN2(startPos + downstreamDist, getLength()));
    //     }
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "Scanning on lane " << myID << "(downstr. " << downstreamDist << ", upstr. " << upstreamDist << ", startPos " << startPos << "): " << std::endl;
    // #endif
    //     std::set<MSVehicle*> foundVehicles = getVehiclesInRange(MAX2(0., startPos - upstreamDist), MIN2(myLength, startPos + downstreamDist));
    //     if (startPos < upstreamDist) {
    //         // scan incoming lanes
    //         for (const IncomingLaneInfo& incomingInfo : getIncomingLanes()) {
    //             MSLane* incoming = incomingInfo.lane;
    // #ifdef DEBUG_SURROUNDING
    //             std::cout << "Checking on incoming: " << incoming->getID() << std::endl;
    //             if (checkedLanes->find(incoming) != checkedLanes->end()) {
    //                 std::cout << "Skipping previous: " << incoming->getID() << std::endl;
    //             }
    // #endif
    //             std::set<MSVehicle*> newVehs = incoming->getSurroundingVehicles(incoming->getLength(), 0.0, upstreamDist - startPos, checkedLanes);
    //             foundVehicles.insert(newVehs.begin(), newVehs.end());
    //         }
    //     }
    //
    //     if (getLength() < startPos + downstreamDist) {
    //         // scan successive lanes
    //         const std::vector<MSLink*>& lc = getLinkCont();
    //         for (MSLink* l : lc) {
    // #ifdef DEBUG_SURROUNDING
    //             std::cout << "Checking on outgoing: " << l->getViaLaneOrLane()->getID() << std::endl;
    // #endif
    //             std::set<MSVehicle*> newVehs = l->getViaLaneOrLane()->getSurroundingVehicles(0.0, downstreamDist - (myLength - startPos), upstreamDist, checkedLanes);
    //             foundVehicles.insert(newVehs.begin(), newVehs.end());
    //         }
    //     }
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "On lane (2) " << myID << ": \nFound vehicles: " << std::endl;
    //     for (MSVehicle* v : foundVehicles) {
    //         std::cout << v->getID() << " pos = " << v->getPositionOnLane() << std::endl;
    //     }
    // #endif
    //     return foundVehicles;
    // }
    #[allow(unused_variables)]
    pub fn get_surrounding_vehicles(&self, startPos: f64, downstreamDist: f64, upstreamDist: f64, checkedLanes: Rc<std::cell::RefCell<LaneCoverageInfo>>) -> BTreeSet<Ptr<MSVehicle>> {
        // the C++ shared_ptr map is mutated by all recursive calls, hence the RefCell
        let key = Ptr::new(self);
        let already = checkedLanes.borrow().contains_key(&key);
        if already {
            return BTreeSet::new();
        }
        // Add this lane's coverage to the lane coverage info
        checkedLanes.borrow_mut().insert(key, (if 0.0 > startPos - upstreamDist { 0.0 } else { startPos - upstreamDist }, if startPos + downstreamDist < self.get_length() { startPos + downstreamDist } else { self.get_length() }));
        let mut foundVehicles = self.get_vehicles_in_range(
            if 0. > startPos - upstreamDist { 0. } else { startPos - upstreamDist },
            if self.myLength < startPos + downstreamDist { self.myLength } else { startPos + downstreamDist });
        if startPos < upstreamDist {
            // scan incoming lanes
            for incomingInfo in self.get_incoming_lanes().iter() {
                let incoming = incomingInfo.lane;
                let newVehs = incoming.get_surrounding_vehicles(incoming.get_length(), 0.0, upstreamDist - startPos, checkedLanes.clone());
                foundVehicles.extend(newVehs);
            }
        }
        if self.get_length() < startPos + downstreamDist {
            // scan successive lanes
            let lc = self.get_link_cont();
            for l in lc.iter() {
                let newVehs = l.get_via_lane_or_lane().get_surrounding_vehicles(0.0, downstreamDist - (self.myLength - startPos), upstreamDist, checkedLanes.clone());
                foundVehicles.extend(newVehs);
            }
        }
        foundVehicles
    }

    /// C++: `std::set<MSVehicle *> MSLane::getVehiclesInRange(const double, const double) const` [const]
    /// Returns all vehicles on the lane overlapping with the interval [a,b]
    // std::set<MSVehicle*>
    // MSLane::getVehiclesInRange(const double a, const double b) const {
    //     std::set<MSVehicle*> res;
    //     const VehCont& vehs = getVehiclesSecure();
    //
    //     if (!vehs.empty()) {
    //         for (MSVehicle* const veh : vehs) {
    //             if (veh->getPositionOnLane() >= a) {
    //                 if (veh->getBackPositionOnLane() > b) {
    //                     break;
    //                 }
    //                 res.insert(veh);
    //             }
    //         }
    //     }
    //     releaseVehicles();
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicles_in_range(&self, a: f64, b: f64) -> BTreeSet<Ptr<MSVehicle>> {
        let mut res: BTreeSet<Ptr<MSVehicle>> = BTreeSet::new();
        let vehs = self.get_vehicles_secure();
        if !vehs.is_empty() {
            for veh in vehs.iter() {
                if veh.get_position_on_lane() >= a {
                    if veh.get_back_position_on_lane_2() > b {
                        break;
                    }
                    res.insert(*veh);
                }
            }
        }
        self.release_vehicles();
        res
    }

    /// C++: `std::vector<const MSJunction *> MSLane::getUpcomingJunctions(double, double, const std::vector<MSLane *> &) const` [const]
    /// Returns all upcoming junctions within given range along the given (non-internal) continuation lanes measured from given position
    // std::vector<const MSJunction*>
    // MSLane::getUpcomingJunctions(double pos, double range, const std::vector<MSLane*>& contLanes) const {
    //     // set of upcoming junctions and the corresponding conflict links
    //     std::vector<const MSJunction*> junctions;
    //     for (auto l : getUpcomingLinks(pos, range, contLanes)) {
    //         junctions.insert(junctions.end(), l->getJunction());
    //     }
    //     return junctions;
    // }
    #[allow(unused_variables)]
    pub fn get_upcoming_junctions(&self, pos: f64, range: f64, contLanes: &Vec<Ptr<MSLane>>) -> Vec<Ptr<MSJunction>> {
        // set of upcoming junctions and the corresponding conflict links
        let mut junctions: Vec<Ptr<MSJunction>> = Vec::new();
        for l in self.get_upcoming_links(pos, range, contLanes) {
            junctions.push(l.get_junction());
        }
        junctions
    }

    /// C++: `std::vector<const MSLink *> MSLane::getUpcomingLinks(double, double, const std::vector<MSLane *> &) const` [const]
    /// Returns all upcoming links within given range along the given (non-internal) continuation lanes measured from given position
    // std::vector<const MSLink*>
    // MSLane::getUpcomingLinks(double pos, double range, const std::vector<MSLane*>& contLanes) const {
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "getUpcoming links on lane '" << getID() << "' with pos=" << pos
    //               << " range=" << range << std::endl;
    // #endif
    //     // set of upcoming junctions and the corresponding conflict links
    //     std::vector<const MSLink*> links;
    //
    //     // Currently scanned lane
    //     const MSLane* lane = this;
    //
    //     // continuation lanes for the vehicle
    //     std::vector<MSLane*>::const_iterator contLanesIt = contLanes.begin();
    //     // scanned distance so far
    //     double dist = 0.0;
    //     // link to be crossed by the vehicle
    //     const MSLink* link = nullptr;
    //     if (lane->isInternal()) {
    //         assert(*contLanesIt == nullptr); // is called with vehicle's bestLane structure
    //         link = lane->getEntryLink();
    //         links.insert(links.end(), link);
    //         dist += link->getInternalLengthsAfter();
    //         // next non-internal lane behind junction
    //         lane = link->getLane();
    //         pos = 0.0;
    //         assert(*(contLanesIt + 1) == lane);
    //     }
    //     while (++contLanesIt != contLanes.end()) {
    //         assert(!lane->isInternal());
    //         dist += lane->getLength() - pos;
    //         pos = 0.;
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "Distance until end of lane '" << lane->getID() << "' is " << dist << "." << std::endl;
    // #endif
    //         if (dist > range) {
    //             break;
    //         }
    //         link = lane->getLinkTo(*contLanesIt);
    //         if (link != nullptr) {
    //             links.insert(links.end(), link);
    //         }
    //         lane = *contLanesIt;
    //     }
    //     return links;
    // }
    #[allow(unused_variables)]
    pub fn get_upcoming_links(&self, pos: f64, range: f64, contLanes: &Vec<Ptr<MSLane>>) -> Vec<Ptr<MSLink>> {
        let mut pos = pos;
        // set of upcoming junctions and the corresponding conflict links
        let mut links: Vec<Ptr<MSLink>> = Vec::new();
        // Currently scanned lane
        let mut lane: Ptr<MSLane> = Ptr::new(self);
        // continuation lanes for the vehicle (index of the C++ iterator)
        let mut contLanesIt: usize = 0;
        // scanned distance so far
        let mut dist = 0.0;
        // link to be crossed by the vehicle
        let mut link: Ptr<MSLink> = Ptr::null();
        if lane.is_internal() {
            // assert(*contLanesIt == nullptr); // is called with vehicle's bestLane structure
            link = lane.get_entry_link();
            links.push(link);
            dist += link.get_internal_lengths_after();
            // next non-internal lane behind junction
            lane = link.get_lane();
            pos = 0.0;
        }
        loop {
            contLanesIt += 1;
            if contLanesIt >= contLanes.len() {
                break;
            }
            dist += lane.get_length() - pos;
            pos = 0.;
            if dist > range {
                break;
            }
            link = lane.get_link_to(contLanes[contLanesIt]);
            if !link.is_null() {
                links.push(link);
            }
            lane = contLanes[contLanesIt];
        }
        links
    }

    /// C++: `MSLane * MSLane::getLogicalPredecessorLane() const` [const]
    /// get the most likely predecessor lane (sorted using by_connections_to_sorter). The result is cached in myLogicalPredecessorLane
    // MSLane*
    // MSLane::getLogicalPredecessorLane() const {
    //     if (myLogicalPredecessorLane == nullptr) {
    //         MSEdgeVector pred = myEdge->getPredecessors();
    //         // get only those edges which connect to this lane
    //         for (MSEdgeVector::iterator i = pred.begin(); i != pred.end();) {
    //             std::vector<IncomingLaneInfo>::const_iterator j = find_if(myIncomingLanes.begin(), myIncomingLanes.end(), edge_finder(*i));
    //             if (j == myIncomingLanes.end()) {
    //                 i = pred.erase(i);
    //             } else {
    //                 ++i;
    //             }
    //         }
    //         // get the lane with the "straightest" connection
    //         if (pred.size() != 0) {
    //             std::sort(pred.begin(), pred.end(), by_connections_to_sorter(&getEdge()));
    //             MSEdge* best = *pred.begin();
    //             std::vector<IncomingLaneInfo>::const_iterator j = find_if(myIncomingLanes.begin(), myIncomingLanes.end(), edge_finder(best));
    //             myLogicalPredecessorLane = j->lane;
    //         }
    //     }
    //     return myLogicalPredecessorLane;
    // }
    #[allow(unused_variables)]
    pub fn get_logical_predecessor_lane(&self) -> Ptr<MSLane> {
        if self.myLogicalPredecessorLane.get().is_null() {
            let mut pred: MSEdgeVector = self.myEdge.get_predecessors().clone();
            // get only those edges which connect to this lane
            pred.retain(|i| self.myIncomingLanes.iter().any(|ili| MSLane_edge_finder::new(*i).call_op(ili)));
            // get the lane with the "straightest" connection
            if !pred.is_empty() {
                let sorter = MSLane_by_connections_to_sorter::new(self.myEdge);
                // determinism: std::sort is unstable, ties may be ordered differently than in C++
                pred.sort_by(|a, b| {
                    if sorter.call_op(*a, *b) != 0 {
                        std::cmp::Ordering::Less
                    } else if sorter.call_op(*b, *a) != 0 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                let best = pred[0];
                let j = self.myIncomingLanes.iter().find(|ili| MSLane_edge_finder::new(best).call_op(ili)).unwrap();
                self.myLogicalPredecessorLane.set(j.lane);
            }
        }
        self.myLogicalPredecessorLane.get()
    }

    /// C++: `const MSLane * MSLane::getNormalPredecessorLane() const` [const]
    /// get normal lane leading to this internal lane, for normal lanes, the lane itself is returned
    // const MSLane*
    // MSLane::getNormalPredecessorLane() const {
    //     if (isInternal()) {
    //         return getLogicalPredecessorLane()->getNormalPredecessorLane();
    //     } else {
    //         return this;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_normal_predecessor_lane(&self) -> Ptr<MSLane> {
        if self.is_internal() {
            self.get_logical_predecessor_lane().get_normal_predecessor_lane()
        } else {
            Ptr::new(self)
        }
    }

    /// C++: `const MSLane * MSLane::getNormalSuccessorLane() const` [const]
    /// get normal lane following this internal lane, for normal lanes, the lane itself is returned
    // const MSLane*
    // MSLane::getNormalSuccessorLane() const {
    //     if (isInternal()) {
    //         return getCanonicalSuccessorLane()->getNormalSuccessorLane();
    //     } else {
    //         return this;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_normal_successor_lane(&self) -> Ptr<MSLane> {
        if self.is_internal() {
            self.get_canonical_successor_lane().get_normal_successor_lane()
        } else {
            Ptr::new(self)
        }
    }

    /// C++: `MSLane * MSLane::getLogicalPredecessorLane(const MSEdge &) const` [const]
    /// return the (first) predecessor lane from the given edge
    // MSLane*
    // MSLane::getLogicalPredecessorLane(const MSEdge& fromEdge) const {
    //     for (const IncomingLaneInfo& cand : myIncomingLanes) {
    //         if (&(cand.lane->getEdge()) == &fromEdge) {
    //             return cand.lane;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_logical_predecessor_lane_2(&self, fromEdge: &MSEdge) -> Ptr<MSLane> {
        for cand in self.myIncomingLanes.iter() {
            if cand.lane.myEdge == Ptr::new(fromEdge) {
                return cand.lane;
            }
        }
        Ptr::null()
    }

    /// C++: `MSLane * MSLane::getCanonicalPredecessorLane() const` [const]
    /// Return the main predecessor lane for the current. If there are several incoming lanes, the first attempt is to return the priorized. If this does not yield an unambiguous lane, the one with the least angle difference to the current is selected.
    // MSLane*
    // MSLane::getCanonicalPredecessorLane() const {
    //     if (myCanonicalPredecessorLane != nullptr) {
    //         return myCanonicalPredecessorLane;
    //     }
    //     if (myIncomingLanes.empty()) {
    //         return nullptr;
    //     }
    //     // myCanonicalPredecessorLane has not yet been determined and there exist incoming lanes
    //     // get the lane with the priorized (or if this does not apply the "straightest") connection
    //     const auto bestLane = std::min_element(myIncomingLanes.begin(), myIncomingLanes.end(), incoming_lane_priority_sorter(this));
    //     {
    // #ifdef HAVE_FOX
    //         ScopedLocker<> lock(myLeaderInfoMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //         myCanonicalPredecessorLane = bestLane->lane;
    //     }
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "\nBest predecessor lane for lane '" << myID << "': '" << myCanonicalPredecessorLane->getID() << "'" << std::endl;
    // #endif
    //     return myCanonicalPredecessorLane;
    // }
    #[allow(unused_variables)]
    pub fn get_canonical_predecessor_lane(&self) -> Ptr<MSLane> {
        if !self.myCanonicalPredecessorLane.get().is_null() {
            return self.myCanonicalPredecessorLane.get();
        }
        if self.myIncomingLanes.is_empty() {
            return Ptr::null();
        }
        // myCanonicalPredecessorLane has not yet been determined and there exist incoming lanes
        // get the lane with the priorized (or if this does not apply the "straightest") connection
        // std::min_element: keep the first element that no later element is strictly less than
        let sorter = MSLane_incoming_lane_priority_sorter::new(Ptr::new(self));
        let mut best = 0;
        for i in 1..self.myIncomingLanes.len() {
            if sorter.call_op(&self.myIncomingLanes[i], &self.myIncomingLanes[best]) != 0 {
                best = i;
            }
        }
        // shortcut: C++ takes myLeaderInfoMutex (ScopedLocker) here, omitted as the port runs single-threaded, upgrade when gNumSimThreads > 1 is supported
        let bestLane = self.myIncomingLanes[best].lane;
        self.myCanonicalPredecessorLane.set(bestLane);
        bestLane
    }

    /// C++: `MSLane * MSLane::getCanonicalSuccessorLane() const` [const]
    /// Return the main successor lane for the current. If there are several outgoing lanes, the first attempt is to return the priorized. If this does not yield an unambiguous lane, the one with the least angle difference to the current is selected.
    // MSLane*
    // MSLane::getCanonicalSuccessorLane() const {
    //     if (myCanonicalSuccessorLane != nullptr) {
    //         return myCanonicalSuccessorLane;
    //     }
    //     if (myLinks.empty()) {
    //         return nullptr;
    //     }
    //     // myCanonicalSuccessorLane has not yet been determined and there exist outgoing links
    //     std::vector<MSLink*> candidateLinks = myLinks;
    //     // get the lane with the priorized (or if this does not apply the "straightest") connection
    //     std::sort(candidateLinks.begin(), candidateLinks.end(), outgoing_lane_priority_sorter(this));
    //     MSLane* best = (*candidateLinks.begin())->getViaLaneOrLane();
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "\nBest successor lane for lane '" << myID << "': '" << best->getID() << "'" << std::endl;
    // #endif
    //     myCanonicalSuccessorLane = best;
    //     return myCanonicalSuccessorLane;
    // }
    #[allow(unused_variables)]
    pub fn get_canonical_successor_lane(&self) -> Ptr<MSLane> {
        if !self.myCanonicalSuccessorLane.get().is_null() {
            return self.myCanonicalSuccessorLane.get();
        }
        if self.myLinks.is_empty() {
            return Ptr::null();
        }
        // myCanonicalSuccessorLane has not yet been determined and there exist outgoing links
        // determinism: C++ std::sort leaves the order of equal links unspecified, the first best link is used here
        let sorter = MSLane_outgoing_lane_priority_sorter::new(Ptr::new(self));
        let mut best = 0;
        for i in 1..self.myLinks.len() {
            if sorter.call_op(self.myLinks[i], self.myLinks[best]) != 0 {
                best = i;
            }
        }
        let bestLane = self.myLinks[best].get_via_lane_or_lane();
        self.myCanonicalSuccessorLane.set(bestLane);
        bestLane
    }

    /// C++: `LinkState MSLane::getIncomingLinkState() const` [const]
    /// get the state of the link from the logical predecessor to this lane
    // LinkState
    // MSLane::getIncomingLinkState() const {
    //     const MSLane* const pred = getLogicalPredecessorLane();
    //     if (pred == nullptr) {
    //         return LINKSTATE_DEADEND;
    //     } else {
    //         return pred->getLinkTo(this)->getState();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_incoming_link_state(&self) -> LinkState {
        let pred = self.get_logical_predecessor_lane();
        if pred.is_null() {
            LINKSTATE_DEADEND
        } else {
            pred.get_link_to(Ptr::new(self)).get_state()
        }
    }

    /// C++: `const std::vector<std::pair<const MSLane *, const MSEdge *>> MSLane::getOutgoingViaLanes() const` [const]
    /// get the list of outgoing lanes
    // const std::vector<std::pair<const MSLane*, const MSEdge*> >
    // MSLane::getOutgoingViaLanes() const {
    //     std::vector<std::pair<const MSLane*, const MSEdge*> > result;
    //     for (const MSLink* link : myLinks) {
    //         assert(link->getLane() != nullptr);
    //         result.push_back(std::make_pair(link->getLane(), link->getViaLane() == nullptr ? nullptr : &link->getViaLane()->getEdge()));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_outgoing_via_lanes(&self) -> Vec<(Ptr<MSLane>, Ptr<MSEdge>)> {
        let mut result: Vec<(Ptr<MSLane>, Ptr<MSEdge>)> = Vec::new();
        for link in self.myLinks.iter() {
            assert!(!link.get_lane().is_null());
            result.push((link.get_lane(), if link.get_via_lane().is_null() { Ptr::null() } else { link.get_via_lane().myEdge }));
        }
        result
    }

    /// C++: `std::vector<const MSLane *> MSLane::getNormalIncomingLanes() const` [const]
    /// get the list of all direct (disregarding internal predecessors) non-internal predecessor lanes of this lane
    // std::vector<const MSLane*>
    // MSLane::getNormalIncomingLanes() const {
    //     std::vector<const MSLane*> result = {};
    //     for (std::map<MSEdge*, std::vector<MSLane*> >::const_iterator it = myApproachingLanes.begin(); it != myApproachingLanes.end(); ++it) {
    //         for (std::vector<MSLane*>::const_iterator it_lane = (*it).second.begin(); it_lane != (*it).second.end(); ++it_lane) {
    //             if (!((*it_lane)->isInternal())) {
    //                 result.push_back(*it_lane);
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_incoming_lanes(&self) -> Vec<Ptr<MSLane>> {
        let mut result: Vec<Ptr<MSLane>> = Vec::new();
        for (_, lanes) in self.myApproachingLanes.iter() {
            for lane in lanes.iter() {
                if !lane.is_internal() {
                    result.push(*lane);
                }
            }
        }
        result
    }

    /// C++: `double MSLane::getMeanSpeed() const` [const]
    /// Returns the mean speed on this lane
    // double
    // MSLane::getMeanSpeed() const {
    //     if (myVehicles.size() == 0) {
    //         return myMaxSpeed;
    //     }
    //     double v = 0;
    //     int numVehs = 0;
    //     for (const MSVehicle* const veh : getVehiclesSecure()) {
    //         if (!veh->isStopped() || !myEdge->hasLaneChanger()) {
    //             v += veh->getSpeed();
    //             numVehs++;
    //         }
    //     }
    //     releaseVehicles();
    //     if (numVehs == 0) {
    //         return myMaxSpeed;
    //     }
    //     return v / numVehs;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed(&self) -> f64 {
        if self.myVehicles.len() == 0 {
            return self.myMaxSpeed;
        }
        let mut v = 0.;
        let mut numVehs = 0;
        for veh in self.get_vehicles_secure().iter() {
            if !veh.is_stopped() || !self.myEdge.has_lane_changer() {
                v += veh.get_speed();
                numVehs += 1;
            }
        }
        self.release_vehicles();
        if numVehs == 0 {
            return self.myMaxSpeed;
        }
        v / numVehs as f64
    }

    /// C++: `double MSLane::getMeanSpeedBike() const` [const]
    /// get the mean speed of all bicycles on this lane
    // double
    // MSLane::getMeanSpeedBike() const {
    //     // @note: redundant code with getMeanSpeed to avoid extra checks in a function that is called very often
    //     if (myVehicles.size() == 0) {
    //         return myMaxSpeed;
    //     }
    //     double v = 0;
    //     int numBikes = 0;
    //     for (MSVehicle* veh : getVehiclesSecure()) {
    //         if (veh->getVClass() == SVC_BICYCLE) {
    //             v += veh->getSpeed();
    //             numBikes++;
    //         }
    //     }
    //     double ret;
    //     if (numBikes > 0) {
    //         ret = v / (double) myVehicles.size();
    //     } else {
    //         ret = myMaxSpeed;
    //     }
    //     releaseVehicles();
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_bike(&self) -> f64 {
        // @note: redundant code with getMeanSpeed to avoid extra checks in a function that is called very often
        if self.myVehicles.len() == 0 {
            return self.myMaxSpeed;
        }
        let mut v = 0.;
        let mut numBikes = 0;
        for veh in self.get_vehicles_secure().iter() {
            if veh.get_v_class() == SVC_BICYCLE {
                v += veh.get_speed();
                numBikes += 1;
            }
        }
        let ret = if numBikes > 0 {
            v / (self.myVehicles.len() as f64)
        } else {
            self.myMaxSpeed
        };
        self.release_vehicles();
        ret
    }

    /// C++: `double MSLane::getWaitingSeconds() const` [const]
    /// Returns the overall waiting time on this lane
    // double
    // MSLane::getWaitingSeconds() const {
    //     if (myVehicles.size() == 0) {
    //         return 0;
    //     }
    //     double wtime = 0;
    //     for (VehCont::const_iterator i = myVehicles.begin(); i != myVehicles.end(); ++i) {
    //         wtime += (*i)->getWaitingSeconds();
    //     }
    //     return wtime;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        if self.myVehicles.len() == 0 {
            return 0.;
        }
        let mut wtime = 0.;
        for i in self.myVehicles.iter() {
            wtime += i.get_waiting_seconds();
        }
        wtime
    }

    /// C++: `double MSLane::getBruttoOccupancy() const` [const]
    /// Returns the brutto (including minGaps) occupancy of this lane during the last step
    // double
    // MSLane::getBruttoOccupancy() const {
    //     getVehiclesSecure();
    //     double fractions = getFractionalVehicleLength(true);
    //     if (myVehicles.size() != 0) {
    //         MSVehicle* lastVeh = myVehicles.front();
    //         if (lastVeh->getPositionOnLane() < lastVeh->getVehicleType().getLength()) {
    //             fractions -= (lastVeh->getVehicleType().getLength() - lastVeh->getPositionOnLane());
    //         }
    //     }
    //     releaseVehicles();
    //     return MIN2(1., (myBruttoVehicleLengthSum + fractions) / myLength);
    // }
    #[allow(unused_variables)]
    pub fn get_brutto_occupancy(&self) -> f64 {
        self.get_vehicles_secure();
        let mut fractions = self.get_fractional_vehicle_length(true);
        if self.myVehicles.len() != 0 {
            let lastVeh = self.myVehicles[0];
            if lastVeh.get_position_on_lane() < lastVeh.get_vehicle_type().get_length() {
                fractions -= lastVeh.get_vehicle_type().get_length() - lastVeh.get_position_on_lane();
            }
        }
        self.release_vehicles();
        let x = (self.myBruttoVehicleLengthSum + fractions) / self.myLength;
        if 1. < x { 1. } else { x }
    }

    /// C++: `double MSLane::getNettoOccupancy() const` [const]
    /// Returns the netto (excluding minGaps) occupancy of this lane during the last step (including minGaps)
    // double
    // MSLane::getNettoOccupancy() const {
    //     getVehiclesSecure();
    //     double fractions = getFractionalVehicleLength(false);
    //     if (myVehicles.size() != 0) {
    //         MSVehicle* lastVeh = myVehicles.front();
    //         if (lastVeh->getPositionOnLane() < lastVeh->getVehicleType().getLength()) {
    //             fractions -= (lastVeh->getVehicleType().getLength() - lastVeh->getPositionOnLane());
    //         }
    //     }
    //     releaseVehicles();
    //     return (myNettoVehicleLengthSum + fractions) / myLength;
    // }
    #[allow(unused_variables)]
    pub fn get_netto_occupancy(&self) -> f64 {
        self.get_vehicles_secure();
        let mut fractions = self.get_fractional_vehicle_length(false);
        if self.myVehicles.len() != 0 {
            let lastVeh = self.myVehicles[0];
            if lastVeh.get_position_on_lane() < lastVeh.get_vehicle_type().get_length() {
                fractions -= lastVeh.get_vehicle_type().get_length() - lastVeh.get_position_on_lane();
            }
        }
        self.release_vehicles();
        (self.myNettoVehicleLengthSum + fractions) / self.myLength
    }

    /// C++: `double MSLane::getBruttoVehLenSum() const` [const]
    /// Returns the sum of lengths of vehicles, including their minGaps, which were on the lane during the last step
    // inline double getBruttoVehLenSum() const {
    //         return myBruttoVehicleLengthSum;
    //     }
    #[allow(unused_variables)]
    pub fn get_brutto_veh_len_sum(&self) -> f64 {
        self.myBruttoVehicleLengthSum
    }

    /// C++: `double MSLane::getEmissions() const` [const]
    /// Returns the sum of last step emissions The value is always per 1s, so multiply by step length if necessary.
    // template<PollutantsInterface::EmissionType ET>
    //     double getEmissions() const {
    //         double ret = 0;
    //         for (MSVehicle* const v : getVehiclesSecure()) {
    //             ret += v->getEmissions<ET>();
    //         }
    //         releaseVehicles();
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn get_emissions(&self, et: PollutantsInterface_EmissionType) -> f64 {
        let mut ret = 0.;
        for v in self.get_vehicles_secure().iter() {
            ret += v.m().get_emissions(et);
        }
        self.release_vehicles();
        ret
    }

    /// C++: `double MSLane::getHarmonoise_NoiseEmissions() const` [const]
    /// Returns the sum of last step noise emissions
    // double
    // MSLane::getHarmonoise_NoiseEmissions() const {
    //     double ret = 0;
    //     const MSLane::VehCont& vehs = getVehiclesSecure();
    //     if (vehs.size() == 0) {
    //         releaseVehicles();
    //         return 0;
    //     }
    //     for (MSLane::VehCont::const_iterator i = vehs.begin(); i != vehs.end(); ++i) {
    //         double sv = (*i)->getHarmonoise_NoiseEmissions();
    //         ret += (double) pow(10., (sv / 10.));
    //     }
    //     releaseVehicles();
    //     return HelpersHarmonoise::sum(ret);
    // }
    #[allow(unused_variables)]
    pub fn get_harmonoise_noise_emissions(&self) -> f64 {
        let mut ret = 0.;
        let vehs = self.get_vehicles_secure();
        if vehs.len() == 0 {
            self.release_vehicles();
            return 0.;
        }
        for v in vehs.iter() {
            let sv = v.get_harmonoise_noise_emissions();
            ret += 10f64.powf(sv / 10.);
        }
        self.release_vehicles();
        crate::utils::emissions::helpers_harmonoise::HelpersHarmonoise::sum(ret)
    }

    /// C++: `void MSLane::setRightSideOnEdge(double, int)`
    /// @}
    // void setRightSideOnEdge(double value, int rightmostSublane) {
    //         myRightSideOnEdge = value;
    //         myRightmostSublane = rightmostSublane;
    //     }
    #[allow(unused_variables)]
    pub fn set_right_side_on_edge(&mut self, value: f64, rightmostSublane: i32) {
        self.myRightSideOnEdge = value;
        self.myRightmostSublane = rightmostSublane;
    }

    /// C++: `void MSLane::initRestrictions()`
    /// initialized vClass-specific speed limits
    // void
    // MSLane::initRestrictions() {
    //     // simplify unit testing without MSNet instance
    //     myRestrictions = MSGlobals::gUnitTests ? nullptr : MSNet::getInstance()->getRestrictions(myEdge->getEdgeType());
    // }
    #[allow(unused_variables)]
    pub fn init_restrictions(&mut self) {
        // simplify unit testing without MSNet instance
        self.myRestrictions = if MSGlobals::unit_tests() {
            Ptr::null()
        } else {
            MSNet::get_instance().get_restrictions(self.myEdge.get_edge_type())
        };
    }

    /// C++: `void MSLane::checkBufferType()`
    // void
    // MSLane::checkBufferType() {
    //     if (MSGlobals::gNumSimThreads <= 1) {
    //         myVehBuffer.unsetCondition();
    // //    } else {
    // // this is an idea for better memory locality, lanes with nearby numerical ids get the same rng and thus the same thread
    // // first tests show no visible effect though
    // //        myRNGIndex = myNumericalID * myRNGs.size() / dictSize();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_buffer_type(&mut self) {
        // shortcut: C++ myVehBuffer.unsetCondition() only matters for the MFXSynchQue wait condition; myVehBuffer is a plain Vec here, so nothing to unset, upgrade with the multi-threaded sim
        let _ = MSGlobals::num_sim_threads();
    }

    /// C++: `double MSLane::getRightSideOnEdge() const` [const]
    // double getRightSideOnEdge() const {
    //         return myRightSideOnEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_right_side_on_edge(&self) -> f64 {
        self.myRightSideOnEdge
    }

    /// C++: `int MSLane::getRightmostSublane() const` [const]
    // int getRightmostSublane() const {
    //         return myRightmostSublane;
    //     }
    #[allow(unused_variables)]
    pub fn get_rightmost_sublane(&self) -> i32 {
        self.myRightmostSublane
    }

    /// C++: `double MSLane::getCenterOnEdge() const` [const]
    // double getCenterOnEdge() const {
    //         return myRightSideOnEdge + 0.5 * myWidth;
    //     }
    #[allow(unused_variables)]
    pub fn get_center_on_edge(&self) -> f64 {
        self.myRightSideOnEdge + 0.5 * self.myWidth
    }

    /// C++: `void MSLane::sortPartialVehicles()`
    /// sorts myPartialVehicles
    // void
    // MSLane::sortPartialVehicles() {
    //     if (myPartialVehicles.size() > 1) {
    //         sort(myPartialVehicles.begin(), myPartialVehicles.end(), vehicle_natural_position_sorter(this));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sort_partial_vehicles(&mut self) {
        if self.myPartialVehicles.len() > 1 {
            // determinism: C++ std::sort is unstable on ties; sort_by is stable here
            let natSorter = MSLane_vehicle_natural_position_sorter::new(Ptr::new(self));
            self.myPartialVehicles.sort_by(|a, b| {
                if natSorter.call_op(*a, *b) != 0 {
                    std::cmp::Ordering::Less
                } else if natSorter.call_op(*b, *a) != 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
    }

    /// C++: `void MSLane::sortManeuverReservations()`
    /// sorts myManeuverReservations
    // void
    // MSLane::sortManeuverReservations() {
    //     if (myManeuverReservations.size() > 1) {
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND) {
    //             std::cout << "sortManeuverReservations on lane " << getID()
    //                       << "\nBefore sort: " << toString(myManeuverReservations) << std::endl;
    //         }
    // #endif
    //         sort(myManeuverReservations.begin(), myManeuverReservations.end(), vehicle_natural_position_sorter(this));
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND) {
    //             std::cout << "After sort: " << toString(myManeuverReservations) << std::endl;
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn sort_maneuver_reservations(&mut self) {
        if self.myManeuverReservations.len() > 1 {
            // determinism: C++ std::sort is unstable on ties; sort_by is stable here
            let natSorter = MSLane_vehicle_natural_position_sorter::new(Ptr::new(self));
            self.myManeuverReservations.sort_by(|a, b| {
                if natSorter.call_op(*a, *b) != 0 {
                    std::cmp::Ordering::Less
                } else if natSorter.call_op(*b, *a) != 0 {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
    }

    /// C++: `MSLane * MSLane::getOpposite() const` [const]
    /// return the neighboring opposite direction lane for lane changing or nullptr
    // MSLane*
    // MSLane::getOpposite() const {
    //     return myOpposite;
    // }
    #[allow(unused_variables)]
    pub fn get_opposite(&self) -> Ptr<MSLane> {
        self.myOpposite
    }

    /// C++: `MSLane * MSLane::getParallelOpposite() const` [const]
    /// return the opposite direction lane of this lanes edge or nullptr
    // MSLane*
    // MSLane::getParallelOpposite() const {
    //     return myEdge->getLanes().back()->getOpposite();
    // }
    #[allow(unused_variables)]
    pub fn get_parallel_opposite(&self) -> Ptr<MSLane> {
        self.myEdge.get_lanes().last().unwrap().get_opposite()
    }

    /// C++: `double MSLane::getOppositePos(double) const` [const]
    /// return the corresponding position on the opposite lane
    // double
    // MSLane::getOppositePos(double pos) const {
    //     return MAX2(0., myLength - pos);
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_pos(&self, pos: f64) -> f64 {
        let v = self.myLength - pos;
        if 0.0 > v { 0.0 } else { v }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getOppositeLeader(const MSVehicle *, double, bool, MinorLinkMode) const` [const]
    // std::pair<MSVehicle* const, double>
    // MSLane::getOppositeLeader(const MSVehicle* ego, double dist, bool oppositeDir, MinorLinkMode mLinkMode) const {
    // #ifdef DEBUG_OPPOSITE
    //     if (DEBUG_COND2(ego)) std::cout << SIMTIME << " getOppositeLeader lane=" << getID()
    //                                         << " ego=" << ego->getID()
    //                                         << " pos=" << ego->getPositionOnLane()
    //                                         << " posOnOpposite=" << getOppositePos(ego->getPositionOnLane())
    //                                         << " dist=" << dist
    //                                         << " oppositeDir=" << oppositeDir
    //                                         << "\n";
    // #endif
    //     if (!oppositeDir) {
    //         return getLeader(ego, getOppositePos(ego->getPositionOnLane()), ego->getBestLanesContinuation(this));
    //     } else {
    //         const double egoLength = ego->getVehicleType().getLength();
    //         const double egoPos = ego->getLaneChangeModel().isOpposite() ? ego->getPositionOnLane() : getOppositePos(ego->getPositionOnLane());
    //         std::pair<MSVehicle* const, double> result = getFollower(ego, egoPos + egoLength, dist, mLinkMode, true);
    //         if (result.first != nullptr) {
    //             result.second -= ego->getVehicleType().getMinGap();
    //             if (result.first->getLaneChangeModel().isOpposite()) {
    //                 result.second -= result.first->getVehicleType().getLength();
    //             }
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_leader(&self, ego: Ptr<MSVehicle>, dist: f64, oppositeDir: bool, mLinkMode: MSLane_MinorLinkMode) -> (Ptr<MSVehicle>, f64) {
        if !oppositeDir {
            return self.get_leader(ego, self.get_opposite_pos(ego.get_position_on_lane()), ego.get_best_lanes_continuation_2(Ptr::new(self)), -1., false);
        } else {
            let egoLength = ego.get_vehicle_type().get_length();
            let egoPos = if ego.get_lane_change_model_2().is_opposite() { ego.get_position_on_lane() } else { self.get_opposite_pos(ego.get_position_on_lane()) };
            let mut result = self.get_follower(ego, egoPos + egoLength, dist, mLinkMode, true);
            if !result.0.is_null() {
                result.1 -= ego.get_vehicle_type().get_min_gap();
                if result.0.get_lane_change_model_2().is_opposite() {
                    result.1 -= result.0.get_vehicle_type().get_length();
                }
            }
            result
        }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getOppositeFollower(const MSVehicle *) const` [const]
    // std::pair<MSVehicle* const, double>
    // MSLane::getOppositeFollower(const MSVehicle* ego) const {
    // #ifdef DEBUG_OPPOSITE
    //     if (DEBUG_COND2(ego)) std::cout << SIMTIME << " getOppositeFollower lane=" << getID()
    //                                         << " ego=" << ego->getID()
    //                                         << " backPos=" << ego->getBackPositionOnLane()
    //                                         << " posOnOpposite=" << getOppositePos(ego->getBackPositionOnLane())
    //                                         << "\n";
    // #endif
    //     if (ego->getLaneChangeModel().isOpposite()) {
    //         std::pair<MSVehicle* const, double> result = getFollower(ego, getOppositePos(ego->getPositionOnLane()), -1, MinorLinkMode::FOLLOW_NEVER);
    //         return result;
    //     } else {
    //         double vehPos = getOppositePos(ego->getPositionOnLane() - ego->getVehicleType().getLength());
    //         std::pair<MSVehicle*, double> result = getLeader(ego, vehPos, std::vector<MSLane*>());
    //         double dist = getMaximumBrakeDist() + getOppositePos(ego->getPositionOnLane() - getLength());
    //         MSLane* next = const_cast<MSLane*>(this);
    //         while (result.first == nullptr && dist > 0) {
    //             // cannot call getLeadersOnConsecutive because succLinkSec doesn't
    //             // uses the vehicle's route and doesn't work on the opposite side
    //             vehPos -= next->getLength();
    //             next = next->getCanonicalSuccessorLane();
    //             if (next == nullptr) {
    //                 break;
    //             }
    //             dist -= next->getLength();
    //             result = next->getLeader(ego, vehPos, std::vector<MSLane*>());
    //         }
    //         if (result.first != nullptr) {
    //             if (result.first->getLaneChangeModel().isOpposite()) {
    //                 result.second -= result.first->getVehicleType().getLength();
    //             } else {
    //                 if (result.second > POSITION_EPS) {
    //                     // follower can be safely ignored since it is going the other way
    //                     return std::make_pair(static_cast<MSVehicle*>(nullptr), -1);
    //                 }
    //             }
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_follower(&self, ego: Ptr<MSVehicle>) -> (Ptr<MSVehicle>, f64) {
        if ego.get_lane_change_model_2().is_opposite() {
            return self.get_follower(ego, self.get_opposite_pos(ego.get_position_on_lane()), -1., MSLane_MinorLinkMode::FOLLOW_NEVER, false);
        } else {
            let mut vehPos = self.get_opposite_pos(ego.get_position_on_lane() - ego.get_vehicle_type().get_length());
            let mut result = self.get_leader(ego, vehPos, &Vec::new(), -1., false);
            let mut dist = self.get_maximum_brake_dist() + self.get_opposite_pos(ego.get_position_on_lane() - self.get_length());
            let mut next = Ptr::new(self);
            while result.0.is_null() && dist > 0. {
                // cannot call getLeadersOnConsecutive because succLinkSec doesn't
                // uses the vehicle's route and doesn't work on the opposite side
                vehPos -= next.get_length();
                next = next.get_canonical_successor_lane();
                if next.is_null() {
                    break;
                }
                dist -= next.get_length();
                result = next.get_leader(ego, vehPos, &Vec::new(), -1., false);
            }
            if !result.0.is_null() {
                if result.0.get_lane_change_model_2().is_opposite() {
                    result.1 -= result.0.get_vehicle_type().get_length();
                } else if result.1 > POSITION_EPS {
                    // follower can be safely ignored since it is going the other way
                    return (Ptr::null(), -1.);
                }
            }
            result
        }
    }

    /// C++: `std::pair<MSVehicle *const, double> MSLane::getFollower(const MSVehicle *, double, double, MinorLinkMode, bool) const` [const]
    /// Find follower vehicle for the given ego vehicle (which may be on the opposite direction lane)
    // std::pair<MSVehicle* const, double>
    // MSLane::getFollower(const MSVehicle* ego, double egoPos, double dist, MinorLinkMode mLinkMode, bool maxSearchDist) const {
    //     for (AnyVehicleIterator first = anyVehiclesUpstreamBegin(); first != anyVehiclesUpstreamEnd(); ++first) {
    //         // XXX refactor leaderInfo to use a const vehicle all the way through the call hierarchy
    //         MSVehicle* pred = (MSVehicle*)*first;
    // #ifdef DEBUG_CONTEXT
    //         if (DEBUG_COND2(ego)) {
    //             std::cout << "   getFollower lane=" << getID() << " egoPos=" << egoPos << " pred=" << pred->getID() << " predPos=" << pred->getPositionOnLane(this) << "\n";
    //         }
    // #endif
    //         if (pred != ego && pred->getPositionOnLane(this) < egoPos) {
    //             return std::pair<MSVehicle* const, double>(pred, egoPos - pred->getPositionOnLane(this) - ego->getVehicleType().getLength() - pred->getVehicleType().getMinGap());
    //         }
    //     }
    //     const double backOffset = egoPos - ego->getVehicleType().getLength();
    //     if (dist > 0 && backOffset > dist) {
    //         return std::make_pair(nullptr, -1);
    //     }
    //     const MSLeaderDistanceInfo followers = getFollowersOnConsecutive(ego, backOffset, true,  dist, mLinkMode, maxSearchDist);
    //     CLeaderDist result = followers.getClosest();
    //     return std::make_pair(const_cast<MSVehicle*>(result.first), result.second);
    // }
    #[allow(unused_variables)]
    pub fn get_follower(&self, ego: Ptr<MSVehicle>, egoPos: f64, dist: f64, mLinkMode: MSLane_MinorLinkMode, maxSearchDist: bool) -> (Ptr<MSVehicle>, f64) {
        let end = self.any_vehicles_upstream_end();
        let mut first = self.any_vehicles_upstream_begin();
        while first.ne_op(&end) {
            let pred = first.op_();
            first.op__2();
            if pred != ego && pred.get_position_on_lane_2(Ptr::new(self)) < egoPos {
                return (pred, egoPos - pred.get_position_on_lane_2(Ptr::new(self)) - ego.get_vehicle_type().get_length() - pred.get_vehicle_type().get_min_gap());
            }
        }
        let backOffset = egoPos - ego.get_vehicle_type().get_length();
        if dist > 0. && backOffset > dist {
            return (Ptr::null(), -1.);
        }
        let followers = self.get_followers_on_consecutive(ego, backOffset, true, dist, mLinkMode, maxSearchDist);
        let result = followers.get_closest();
        (result.0, result.1)
    }

    /// C++: `void MSLane::addParking(MSBaseVehicle *)`
    /// add parking vehicle. This should only used during state loading
    // void
    // MSLane::addParking(MSBaseVehicle* veh) {
    //     myParkingVehicles.insert(veh);
    // }
    #[allow(unused_variables)]
    pub fn add_parking(&mut self, veh: Ptr<MSBaseVehicle>) {
        self.myParkingVehicles.insert(veh);
    }

    /// C++: `void MSLane::removeParking(MSBaseVehicle *)` [virtual]
    /// remove parking vehicle. This must be syncrhonized when running with GUI
    // void
    // MSLane::removeParking(MSBaseVehicle* veh) {
    //     myParkingVehicles.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn remove_parking(&mut self, veh: Ptr<MSBaseVehicle>) {
        self.myParkingVehicles.remove(&veh);
    }

    /// C++: `const std::set<const MSBaseVehicle *> & MSLane::getParkingVehicles() const` [const]
    /// retrieve the parking vehicles (see GUIParkingArea)
    // const std::set<const MSBaseVehicle*>& getParkingVehicles() const {
    //         return myParkingVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_vehicles(&self) -> &BTreeSet<Ptr<MSBaseVehicle>> {
        &self.myParkingVehicles
    }

    /// C++: `bool MSLane::isSelected() const` [const, virtual]
    /// whether this lane is selected in the GUI
    // virtual bool isSelected() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        false
    }

    /// C++: `MSLane * MSLane::getBidiLane() const` [const]
    /// retrieve bidirectional lane or nullptr
    // MSLane*
    // MSLane::getBidiLane() const {
    //     return myBidiLane;
    // }
    #[allow(unused_variables)]
    pub fn get_bidi_lane(&self) -> Ptr<MSLane> {
        self.myBidiLane
    }

    /// C++: `bool MSLane::mustCheckJunctionCollisions() const` [const]
    /// whether this lane must check for junction collisions
    // bool
    // MSLane::mustCheckJunctionCollisions() const {
    //     return myCheckJunctionCollisions && myEdge->isInternal() && (
    //                myLinks.front()->getFoeLanes().size() > 0
    //                || myLinks.front()->getWalkingAreaFoe() != nullptr
    //                || myLinks.front()->getWalkingAreaFoeExit() != nullptr);
    // }
    #[allow(unused_variables)]
    pub fn must_check_junction_collisions(&self) -> bool {
        MY_CHECK_JUNCTION_COLLISIONS.with(|c| c.get())
            && self.myEdge.is_internal()
            && (self.myLinks[0].get_foe_lanes().len() > 0
                || !self.myLinks[0].m().get_walking_area_foe().is_null()
                || !self.myLinks[0].m().get_walking_area_foe_exit().is_null())
    }

    /// C++: `std::vector<StopWatch<std::chrono::nanoseconds>> & MSLane::getStopWatch()`
    // std::vector<StopWatch<std::chrono::nanoseconds> >& getStopWatch() {
    //         return myStopWatch;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_watch(&mut self) -> &mut Vec<Unmapped /* StopWatch<std::chrono::nanoseconds> */> {
        &mut self.myStopWatch
    }

    /// C++: `void MSLane::changeLanes(const SUMOTime)`
    // void
    // MSLane::changeLanes(const SUMOTime t) {
    //     myEdge->changeLanes(t);
    // }
    #[allow(unused_variables)]
    pub fn change_lanes(&mut self, time: SUMOTime) {
        self.myEdge.change_lanes(time);
    }

    /// C++: `void MSLane::saveState(OutputDevice &)`
    /// Saves the state of this lane into the given stream
    // void
    // MSLane::saveState(OutputDevice& out) {
    //     const bool toRailJunction = myLinks.size() > 0 && (
    //                                     myEdge->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL
    //                                     || myEdge->getToJunction()->getType() == SumoXMLNodeType::RAIL_CROSSING);
    //     const bool saveApproaching = toRailJunction;
    //     const bool hasVehicles = myVehicles.size() > 0;
    //     if (hasVehicles || (toRailJunction && hasApproaching())) {
    //         out.openTag(SUMO_TAG_LANE);
    //         out.writeAttr(SUMO_ATTR_ID, getID());
    //         if (hasVehicles) {
    //             out.openTag(SUMO_TAG_VIEWSETTINGS_VEHICLES);
    //             out.writeAttr(SUMO_ATTR_VALUE, myVehicles);
    //             out.closeTag();
    //         }
    //         if (saveApproaching) {
    //             for (const MSLink* link : myLinks) {
    //                 if (link->getApproaching().size() > 0) {
    //                     out.openTag(SUMO_TAG_LINK);
    //                     out.writeAttr(SUMO_ATTR_TO, link->getViaLaneOrLane()->getID());
    //                     for (auto item : link->getApproaching()) {
    //                         out.openTag(SUMO_TAG_APPROACHING);
    //                         out.writeAttr(SUMO_ATTR_ID, item.first->getID());
    //                         out.writeAttr(SUMO_ATTR_ARRIVALTIME, item.second.arrivalTime);
    //                         out.writeAttr(SUMO_ATTR_ARRIVALSPEED, item.second.arrivalSpeed);
    //                         out.writeAttr(SUMO_ATTR_DEPARTSPEED, item.second.leaveSpeed);
    //                         out.writeAttr(SUMO_ATTR_REQUEST, item.second.willPass);
    //                         out.writeAttr(SUMO_ATTR_ARRIVALSPEEDBRAKING, item.second.arrivalSpeedBraking);
    //                         out.writeAttr(SUMO_ATTR_WAITINGTIME, item.second.waitingTime);
    //                         out.writeAttr(SUMO_ATTR_DISTANCE, item.second.dist);
    //                         if (item.second.latOffset != 0) {
    //                             out.writeAttr(SUMO_ATTR_POSITION_LAT, item.second.latOffset);
    //                         }
    //                         out.closeTag();
    //                     }
    //                     out.closeTag();
    //                 }
    //             }
    //         }
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        let toRailJunction = self.myLinks.len() > 0
            && (self.myEdge.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL
                || self.myEdge.get_to_junction().get_type() == SumoXMLNodeType::RAIL_CROSSING);
        let saveApproaching = toRailJunction;
        let hasVehicles = self.myVehicles.len() > 0;
        if hasVehicles || (saveApproaching && self.has_approaching()) {
            out.open_tag_id(SUMO_TAG_LANE);
            out.write_attr(SUMO_ATTR_ID, self.get_id());
            if hasVehicles {
                out.open_tag_id(SUMO_TAG_VIEWSETTINGS_VEHICLES);
                // C++ writes the vector as space-separated IDs
                let ids: Vec<String> = self.myVehicles.iter().map(|v| v.get_id().to_string()).collect();
                out.write_attr(SUMO_ATTR_VALUE, &ids);
                out.close_tag("");
            }
            if saveApproaching {
                for link in &self.myLinks {
                    if link.get_approaching_2().len() > 0 {
                        out.open_tag_id(SUMO_TAG_LINK);
                        out.write_attr(SUMO_ATTR_TO, link.get_via_lane_or_lane().get_id());
                        for (item_first, item_second) in link.get_approaching_2() {
                            out.open_tag_id(SUMO_TAG_APPROACHING);
                            out.write_attr(SUMO_ATTR_ID, item_first.get_id());
                            out.write_attr(SUMO_ATTR_ARRIVALTIME, &item_second.arrivalTime);
                            out.write_attr(SUMO_ATTR_ARRIVALSPEED, &item_second.arrivalSpeed);
                            out.write_attr(SUMO_ATTR_DEPARTSPEED, &item_second.leaveSpeed);
                            out.write_attr(SUMO_ATTR_REQUEST, &item_second.willPass);
                            out.write_attr(SUMO_ATTR_ARRIVALSPEEDBRAKING, &item_second.arrivalSpeedBraking);
                            out.write_attr(SUMO_ATTR_WAITINGTIME, &item_second.waitingTime);
                            out.write_attr(SUMO_ATTR_DISTANCE, &item_second.dist);
                            if item_second.latOffset != 0. {
                                out.write_attr(SUMO_ATTR_POSITION_LAT, &item_second.latOffset);
                            }
                            out.close_tag("");
                        }
                        out.close_tag("");
                    }
                }
            }
            out.close_tag("");
        }
    }

    /// C++: `void MSLane::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSLane::clearState() {
    //     myVehicles.clear();
    //     myParkingVehicles.clear();
    //     myPartialVehicles.clear();
    //     myManeuverReservations.clear();
    //     myBruttoVehicleLengthSum = 0;
    //     myNettoVehicleLengthSum = 0;
    //     myBruttoVehicleLengthSumToRemove = 0;
    //     myNettoVehicleLengthSumToRemove = 0;
    //     myLeaderInfoTime = SUMOTime_MIN;
    //     myFollowerInfoTime = SUMOTime_MIN;
    //     for (MSLink* link : myLinks) {
    //         link->clearState();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myVehicles.clear();
        self.myParkingVehicles.clear();
        self.myPartialVehicles.clear();
        self.myManeuverReservations.clear();
        self.myBruttoVehicleLengthSum = 0.;
        self.myNettoVehicleLengthSum = 0.;
        self.myBruttoVehicleLengthSumToRemove = 0.;
        self.myNettoVehicleLengthSumToRemove = 0.;
        self.myLeaderInfoTime.set(utils::common::time::SUMOTIME_MIN);
        self.myFollowerInfoTime.set(utils::common::time::SUMOTIME_MIN);
        for link in self.myLinks.iter() {
            link.m().clear_state();
        }
    }

    /// C++: `void MSLane::loadState(const std::vector<SUMOVehicle *> &)`
    /// Loads the state of this segment with the given parameters
    // void
    // MSLane::loadState(const std::vector<SUMOVehicle*>& vehs) {
    //     for (SUMOVehicle* veh : vehs) {
    //         MSVehicle* v = dynamic_cast<MSVehicle*>(veh);
    //         v->updateBestLanes(false, this);
    //         // incorporateVehicle resets the lastActionTime (which has just been loaded from state) so we must restore it
    //         const SUMOTime lastActionTime = v->getLastActionTime();
    //         incorporateVehicle(v, v->getPositionOnLane(), v->getSpeed(), v->getLateralPositionOnLane(), myVehicles.end(),
    //                            MSMoveReminder::NOTIFICATION_LOAD_STATE);
    //         v->resetActionOffset(lastActionTime - MSNet::getInstance()->getCurrentTimeStep());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, vehs: &Vec<Ptr<SUMOVehicle>>) {
        for veh in vehs.iter() {
            // shortcut: C++ dynamic_cast<MSVehicle*> is unchecked; loadState only receives MSVehicles from the state loader
            let v: Ptr<MSVehicle> = veh.cast::<MSVehicle>();
            v.m().update_best_lanes(false, Ptr::new(self));
            // incorporateVehicle resets the lastActionTime (which has just been loaded from state) so we must restore it
            let lastActionTime = v.get_last_action_time();
            let at = self.myVehicles.len();
            self.incorporate_vehicle(v, v.get_position_on_lane(), v.get_speed(), v.get_lateral_position_on_lane(), at, MSMoveReminder_Notification::NOTIFICATION_LOAD_STATE);
            v.m().reset_action_offset(lastActionTime - MSNet::get_instance().get_current_time_step());
        }
    }

    /// C++: `bool MSLane::hasApproaching() const` [const]
    // bool
    // MSLane::hasApproaching() const {
    //     for (const MSLink* link : myLinks) {
    //         if (link->getApproaching().size() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_approaching(&self) -> bool {
        for link in self.myLinks.iter() {
            if link.get_approaching_2().len() > 0 {
                return true;
            }
        }
        false
    }

    /// C++: `void MSLane::visit(const MSLane::StoringVisitor &) const` [const]
    /// Callback for visiting the lane when traversing an RTree
    // void visit(const MSLane::StoringVisitor& cont) const {
    //         cont.add(this);
    //     }
    #[allow(unused_variables)]
    pub fn visit(&self, cont: &MSLane_StoringVisitor) {
        cont.add(Ptr::new(self));
    }

    /// C++: `bool MSLane::hasPedestrians() const` [const]
    /// whether the lane has pedestrians on it
    // bool
    // MSLane::hasPedestrians() const {
    //     MSNet* const net = MSNet::getInstance();
    //     return net->hasPersons() && net->getPersonControl().getMovementModel()->hasPedestrians(this);
    // }
    #[allow(unused_variables)]
    pub fn has_pedestrians(&self) -> bool {
        let net = MSNet::get_instance();
        net.has_persons() && net.m().get_person_control().get_movement_model().m().has_pedestrians(Ptr::new(self))
    }

    /// C++: `std::pair<const MSPerson *, double> MSLane::nextBlocking(double, double, double, double, bool) const` [const]
    /// This is just a wrapper around MSPModel::nextBlocking. You should always check using hasPedestrians before calling this method.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn next_blocking(&self, minPos: f64, minRight: f64, maxLeft: f64, stopTime: f64, bidi: bool) -> (Ptr<MSPerson>, f64) {
        let net = MSNet::get_instance();
        net.m().get_person_control().get_movement_model().m().next_blocking(Ptr::new(self), minPos, minRight, maxLeft, stopTime, bidi)
    }

    /// C++: `double MSLane::getSpaceTillLastStanding(const MSVehicle *, bool &) const` [const]
    /// return the empty space up to the last standing vehicle or the empty space on the whole lane if no vehicle is standing
    // double
    // MSLane::getSpaceTillLastStanding(const MSVehicle* ego, bool& foundStopped) const {
    //     /// @todo if ego isn't on this lane, we could use a cached value
    //     double lengths = 0;
    //     for (const MSVehicle* last : myVehicles) {
    //         if (last->getSpeed() < SUMO_const_haltingSpeed && !last->getLane()->getEdge().isRoundabout()
    //                 && last != ego
    //                 // @todo recheck
    //                 && last->isFrontOnLane(this)) {
    //             foundStopped = true;
    //             const double lastBrakeGap = last->getCarFollowModel().brakeGap(last->getSpeed());
    //             const double ret = last->getBackPositionOnLane() + lastBrakeGap - lengths;
    //             return ret;
    //         }
    //         if (MSGlobals::gSublane && ego->getVehicleType().getWidth() + last->getVehicleType().getWidth() < getWidth()) {
    //             lengths += last->getVehicleType().getLengthWithGap() * (last->getVehicleType().getWidth() + last->getVehicleType().getMinGapLat()) / getWidth();
    //         } else {
    //             lengths += last->getVehicleType().getLengthWithGap();
    //         }
    //     }
    //     return getLength() - lengths;
    // }
    #[allow(unused_variables)]
    pub fn get_space_till_last_standing(&self, ego: Ptr<MSVehicle>, foundStopped: &mut bool) -> f64 {
        // @todo if ego isn't on this lane, we could use a cached value
        let mut lengths = 0.;
        for last in self.myVehicles.iter().copied() {
            if last.get_speed() < SUMO_CONST_HALTING_SPEED && !last.get_lane().get_edge().is_roundabout() && last != ego && last.is_front_on_lane(Ptr::new(self)) {
                *foundStopped = true;
                let lastBrakeGap = last.get_car_follow_model().brake_gap(last.get_speed());
                let ret = last.get_back_position_on_lane_2() + lastBrakeGap - lengths;
                return ret;
            }
            if MSGlobals::sublane() && ego.get_vehicle_type().get_width() + last.get_vehicle_type().get_width() < self.get_width() {
                lengths += last.get_vehicle_type().get_length_with_gap() * (last.get_vehicle_type().get_width() + last.get_vehicle_type().get_min_gap_lat()) / self.get_width();
            } else {
                lengths += last.get_vehicle_type().get_length_with_gap();
            }
        }
        self.get_length() - lengths
    }

    /// C++: `double MSLane::getMaximumBrakeDist() const` [const]
    /// compute maximum braking distance on this lane
    // double
    // MSLane::getMaximumBrakeDist() const {
    //     const MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     const double maxSpeed = getSpeedLimit() * vc.getMaxSpeedFactor();
    //     // NOTE: For the euler update this is an upper bound on the actual braking distance (see ticket #860)
    //     // impose a hard bound due to visibility / common sense to avoid unnecessary computation if there are strange vehicles in the fleet
    //     const double minDecel = isRailway(myPermissions) ? vc.getMinDecelerationRail() : vc.getMinDeceleration();
    //     return MIN2(maxSpeed * maxSpeed * 0.5 / minDecel + vc.getMaxMinGap(),
    //                 myPermissions == SVC_SHIP ? 10000.0 : 1000.0);
    // }
    #[allow(unused_variables)]
    pub fn get_maximum_brake_dist(&self) -> f64 {
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        let maxSpeed = self.get_speed_limit() * vc.get_max_speed_factor();
        // NOTE: For the euler update this is an upper bound on the actual braking distance (see ticket #860)
        // impose a hard bound due to visibility / common sense to avoid unnecessary computation if there are strange vehicles in the fleet
        let minDecel = if utils::common::sumo_vehicle_class::is_railway(self.myPermissions) { vc.get_min_deceleration_rail() } else { vc.get_min_deceleration() };
        let a = maxSpeed * maxSpeed * 0.5 / minDecel + vc.get_max_min_gap();
        let b = if self.myPermissions == utils::common::sumo_vehicle_class::SVC_SHIP { 10000.0 } else { 1000.0 };
        if a < b { a } else { b }
    }

    /// C++: `const PositionVector * MSLane::getOutlineShape() const` [const]
    // inline const PositionVector* getOutlineShape() const {
    //         return myOutlineShape;
    //     }
    #[allow(unused_variables)]
    pub fn get_outline_shape(&self) -> Ptr<PositionVector> {
        self.myOutlineShape
    }

    /// C++: `void MSLane::initCollisionOptions(const OptionsCont &)` [static]
    // void
    // MSLane::initCollisionOptions(const OptionsCont& oc) {
    //     initCollisionAction(oc, "collision.action", myCollisionAction);
    //     initCollisionAction(oc, "intermodal-collision.action", myIntermodalCollisionAction);
    //     myCheckJunctionCollisions = oc.getBool("collision.check-junctions");
    //     myCheckJunctionCollisionMinGap = oc.getFloat("collision.check-junctions.mingap");
    //     myCollisionStopTime = string2time(oc.getString("collision.stoptime"));
    //     myIntermodalCollisionStopTime = string2time(oc.getString("intermodal-collision.stoptime"));
    //     myCollisionMinGapFactor = oc.getFloat("collision.mingap-factor");
    //     myExtrapolateSubstepDepart = oc.getBool("extrapolate-departpos");
    // }
    #[allow(unused_variables)]
    pub fn init_collision_options(oc: &OptionsCont) {
        let mut action = MY_COLLISION_ACTION.with(|c| c.get());
        MSLane::init_collision_action(oc, "collision.action", &mut action);
        MY_COLLISION_ACTION.with(|c| c.set(action));
        let mut intermodal_action = MY_INTERMODAL_COLLISION_ACTION.with(|c| c.get());
        MSLane::init_collision_action(oc, "intermodal-collision.action", &mut intermodal_action);
        MY_INTERMODAL_COLLISION_ACTION.with(|c| c.set(intermodal_action));
        MY_CHECK_JUNCTION_COLLISIONS.with(|c| c.set(oc.get_bool("collision.check-junctions").unwrap_or_else(|e| panic!("{e}"))));
        MY_CHECK_JUNCTION_COLLISION_MIN_GAP.with(|c| c.set(oc.get_float("collision.check-junctions.mingap").unwrap_or_else(|e| panic!("{e}"))));
        MY_COLLISION_STOP_TIME.with(|c| c.set(utils::common::time::string2time(&oc.get_string("collision.stoptime").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}"))));
        MY_INTERMODAL_COLLISION_STOP_TIME.with(|c| c.set(utils::common::time::string2time(&oc.get_string("intermodal-collision.stoptime").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}"))));
        MY_COLLISION_MIN_GAP_FACTOR.with(|c| c.set(oc.get_float("collision.mingap-factor").unwrap_or_else(|e| panic!("{e}"))));
        MY_EXTRAPOLATE_SUBSTEP_DEPART.with(|c| c.set(oc.get_bool("extrapolate-departpos").unwrap_or_else(|e| panic!("{e}"))));
    }

    /// C++: `void MSLane::initCollisionAction(const OptionsCont &, const std::string &, CollisionAction &)` [static]
    // void
    // MSLane::initCollisionAction(const OptionsCont& oc, const std::string& option, CollisionAction& myAction) {
    //     const std::string action = oc.getString(option);
    //     if (action == "none") {
    //         myAction = COLLISION_ACTION_NONE;
    //     } else if (action == "warn") {
    //         myAction = COLLISION_ACTION_WARN;
    //     } else if (action == "teleport") {
    //         myAction = COLLISION_ACTION_TELEPORT;
    //     } else if (action == "remove") {
    //         myAction = COLLISION_ACTION_REMOVE;
    //     } else {
    //         WRITE_ERROR(TLF("Invalid % '%'.", option, action));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_collision_action(oc: &OptionsCont, option: &str, myAction: &mut MSLane_CollisionAction) {
        let action = oc.get_string(option).unwrap_or_else(|e| panic!("{e}"));
        if action == "none" {
            *myAction = MSLane_CollisionAction::COLLISION_ACTION_NONE;
        } else if action == "warn" {
            *myAction = MSLane_CollisionAction::COLLISION_ACTION_WARN;
        } else if action == "teleport" {
            *myAction = MSLane_CollisionAction::COLLISION_ACTION_TELEPORT;
        } else if action == "remove" {
            *myAction = MSLane_CollisionAction::COLLISION_ACTION_REMOVE;
        } else {
            utils::write_errorf!("Invalid % '%'.", option, action);
        }
    }

    /// C++: `CollisionAction MSLane::getCollisionAction()` [static]
    // static CollisionAction getCollisionAction() {
    //         return myCollisionAction;
    //     }
    #[allow(unused_variables)]
    pub fn get_collision_action() -> MSLane_CollisionAction {
        MY_COLLISION_ACTION.with(|c| c.get())
    }

    /// C++: `CollisionAction MSLane::getIntermodalCollisionAction()` [static]
    // static CollisionAction getIntermodalCollisionAction() {
    //         return myIntermodalCollisionAction;
    //     }
    #[allow(unused_variables)]
    pub fn get_intermodal_collision_action() -> MSLane_CollisionAction {
        MY_INTERMODAL_COLLISION_ACTION.with(|c| c.get())
    }

    /// C++: `DepartSpeedDefinition & MSLane::getDefaultDepartSpeedDefinition()` [static]
    // static DepartSpeedDefinition& getDefaultDepartSpeedDefinition() {
    //         return myDefaultDepartSpeedDefinition;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_depart_speed_definition() -> &'static mut DepartSpeedDefinition {
        unsafe { &mut *std::ptr::addr_of_mut!(MY_DEFAULT_DEPART_SPEED_DEFINITION) }
    }

    /// C++: `double & MSLane::getDefaultDepartSpeed()` [static]
    // static double& getDefaultDepartSpeed() {
    //         return myDefaultDepartSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_default_depart_speed() -> &'static mut f64 {
        unsafe { &mut *std::ptr::addr_of_mut!(MY_DEFAULT_DEPART_SPEED) }
    }

    /// C++: `void MSLane::swapAfterLaneChange(SUMOTime)` [virtual, protected]
    /// moves myTmpVehicles int myVehicles after a lane change procedure
    // void
    // MSLane::swapAfterLaneChange(SUMOTime) {
    //     //if (getID() == "disabled_lane") std::cout << SIMTIME << " swapAfterLaneChange lane=" << getID() << " myVehicles=" << toString(myVehicles) << " myTmpVehicles=" << toString(myTmpVehicles) << "\n";
    //     myVehicles = myTmpVehicles;
    //     myTmpVehicles.clear();
    //     // this needs to be done after finishing lane-changing for all lanes on the
    //     // current edge (MSLaneChanger::updateLanes())
    //     sortPartialVehicles();
    //     if (MSGlobals::gSublane && getOpposite() != nullptr) {
    //         getOpposite()->sortPartialVehicles();
    //     }
    //     if (myBidiLane != nullptr) {
    //         myBidiLane->sortPartialVehicles();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn swap_after_lane_change(&mut self, t: SUMOTime) {
        self.myVehicles = self.myTmpVehicles.clone();
        self.myTmpVehicles.clear();
        // this needs to be done after finishing lane-changing for all lanes on the
        // current edge (MSLaneChanger::updateLanes())
        self.sort_partial_vehicles();
        if MSGlobals::sublane() && !self.get_opposite().is_null() {
            self.get_opposite().m().sort_partial_vehicles();
        }
        if !self.myBidiLane.is_null() {
            self.myBidiLane.m().sort_partial_vehicles();
        }
    }

    /// C++: `void MSLane::incorporateVehicle(MSVehicle *, double, double, double, const MSLane::VehCont::iterator &, MSMoveReminder::Notification)` [virtual, protected]
    /// Inserts the vehicle into this lane, and informs it about entering the network
    // void
    // MSLane::incorporateVehicle(MSVehicle* veh, double pos, double speed, double posLat, const MSLane::VehCont::iterator& at, MSMoveReminder::Notification notification) {
    //     myNeedsCollisionCheck = true;
    //     assert(pos <= myLength || notification == MSMoveReminder::NOTIFICATION_LOAD_STATE);
    //     bool wasInactive = myVehicles.size() == 0;
    //     veh->enterLaneAtInsertion(this, pos, speed, posLat, notification);
    //     if (at == myVehicles.end()) {
    //         // vehicle will be the first on the lane
    //         myVehicles.push_back(veh);
    //     } else {
    //         myVehicles.insert(at, veh);
    //     }
    //     myBruttoVehicleLengthSum += veh->getVehicleType().getLengthWithGap();
    //     myNettoVehicleLengthSum += veh->getVehicleType().getLength();
    //     myEdge->markDelayed();
    //     if (wasInactive) {
    //         MSNet::getInstance()->getEdgeControl().gotActive(this);
    //     }
    //     if (getBidiLane() != nullptr && (!isRailway(veh->getVClass()) || (getPermissions() & ~SVC_RAIL_CLASSES) != 0)) {
    //         // railways don't need to "see" each other when moving in opposite directions on the same track (efficiency)
    //         getBidiLane()->setPartialOccupation(veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn incorporate_vehicle(&mut self, veh: Ptr<MSVehicle>, pos: f64, speed: f64, posLat: f64, at: usize, notification: MSMoveReminder_Notification) {
        self.myNeedsCollisionCheck = true;
        debug_assert!(pos <= self.myLength || notification == MSMoveReminder_Notification::NOTIFICATION_LOAD_STATE);
        let wasInactive = self.myVehicles.len() == 0;
        veh.m().enter_lane_at_insertion(Ptr::from_mut(self), pos, speed, posLat, notification);
        if at == self.myVehicles.len() {
            // vehicle will be the first on the lane
            self.myVehicles.push(veh);
        } else {
            self.myVehicles.insert(at, veh);
        }
        self.myBruttoVehicleLengthSum += veh.get_vehicle_type().get_length_with_gap();
        self.myNettoVehicleLengthSum += veh.get_vehicle_type().get_length();
        self.myEdge.mark_delayed();
        if wasInactive {
            MSNet::get_instance().m().get_edge_control().got_active(Ptr::from_mut(self));
        }
        if !self.get_bidi_lane().is_null() && (!utils::common::sumo_vehicle_class::is_railway(veh.get_v_class()) || (self.get_permissions() & !utils::common::sumo_vehicle_class::SVC_RAIL_CLASSES) != 0) {
            // railways don't need to "see" each other when moving in opposite directions on the same track (efficiency)
            self.get_bidi_lane().m().set_partial_occupation(veh);
        }
    }

    /// C++: `void MSLane::detectPedestrianJunctionCollision(const MSVehicle *, const PositionVector &, const MSLane *, SUMOTime, const std::string &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &)` [protected]
    /// detect whether a vehicle collids with pedestrians on the junction
    // void
    // MSLane::detectPedestrianJunctionCollision(const MSVehicle* collider, const PositionVector& colliderBoundary, const MSLane* foeLane,
    //         SUMOTime timestep, const std::string& stage,
    //         std::set<const MSVehicle*, ComparatorNumericalIdLess>& toRemove,
    //         std::set<const MSVehicle*, ComparatorNumericalIdLess>& toTeleport) {
    //     if (myIntermodalCollisionAction != COLLISION_ACTION_NONE && foeLane->getEdge().getPersons().size() > 0 && foeLane->hasPedestrians()) {
    // #ifdef DEBUG_PEDESTRIAN_COLLISIONS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " detect pedestrian junction collisions stage=" << stage << " lane=" << getID() << " foeLane=" << foeLane->getID() << "\n";
    //         }
    // #endif
    //         const std::vector<MSTransportable*>& persons = foeLane->getEdge().getSortedPersons(timestep);
    //         for (std::vector<MSTransportable*>::const_iterator it_p = persons.begin(); it_p != persons.end(); ++it_p) {
    // #ifdef DEBUG_PEDESTRIAN_COLLISIONS
    //             if (DEBUG_COND) {
    //                 std::cout << "    collider=" << collider->getID()
    //                           << " ped=" << (*it_p)->getID()
    //                           << " jammed=" << (*it_p)->isJammed()
    //                           << " colliderBoundary=" << colliderBoundary
    //                           << " pedBoundary=" << (*it_p)->getBoundingBox()
    //                           << "\n";
    //             }
    // #endif
    //             if ((*it_p)->isJammed()) {
    //                 continue;
    //             }
    //             if (colliderBoundary.overlapsWith((*it_p)->getBoundingBox())
    //                     && collider->getBoundingPoly().overlapsWith((*it_p)->getBoundingBox())) {
    //                 std::string collisionType = "junctionPedestrian";
    //                 if (foeLane->isCrossing()) {
    //                     collisionType = "crossing";
    //                 } else if (foeLane->isWalkingArea()) {
    //                     collisionType = "walkingarea";
    //                 }
    //                 handleIntermodalCollisionBetween(timestep, stage, collider, *it_p, 0, collisionType, toRemove, toTeleport);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detect_pedestrian_junction_collision(&mut self, collider: Ptr<MSVehicle>, colliderBoundary: &PositionVector, foeLane: Ptr<MSLane>, timestep: SUMOTime, stage: &str, toRemove: &mut BTreeSet<Ptr<MSVehicle>>, toTeleport: &mut BTreeSet<Ptr<MSVehicle>>) {
        if MSLane::get_intermodal_collision_action() != MSLane_CollisionAction::COLLISION_ACTION_NONE && foeLane.get_edge().get_persons().len() > 0 && foeLane.has_pedestrians() {
            let persons = foeLane.get_edge().get_sorted_persons(timestep, false);
            for p in persons {
                if p.is_jammed() {
                    continue;
                }
                if colliderBoundary.overlaps_with(&p.get_bounding_box(), 0.) && collider.get_bounding_poly(0.).overlaps_with(&p.get_bounding_box(), 0.) {
                    let mut collisionType = "junctionPedestrian";
                    if foeLane.is_crossing() {
                        collisionType = "crossing";
                    } else if foeLane.is_walking_area() {
                        collisionType = "walkingarea";
                    }
                    self.handle_intermodal_collision_between(timestep, stage, collider, p, 0., collisionType, toRemove, toTeleport);
                }
            }
        }
    }

    /// C++: `bool MSLane::detectCollisionBetween(SUMOTime, const std::string &, MSVehicle *, MSVehicle *, std::set<const MSVehicle *, ComparatorNumericalIdLess> &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &) const` [const, protected]
    /// detect whether there is a collision between the two vehicles
    // bool
    // MSLane::detectCollisionBetween(SUMOTime timestep, const std::string& stage, MSVehicle* collider, MSVehicle* victim,
    //                                std::set<const MSVehicle*, ComparatorNumericalIdLess>& toRemove,
    //                                std::set<const MSVehicle*, ComparatorNumericalIdLess>& toTeleport) const {
    //     if (myCollisionAction == COLLISION_ACTION_TELEPORT && ((victim->hasInfluencer() && victim->getInfluencer().isRemoteAffected(timestep)) ||
    //             (collider->hasInfluencer() && collider->getInfluencer().isRemoteAffected(timestep)))) {
    //         return false;
    //     }
    //
    //     // No self-collisions! (This is assumed to be ensured at caller side)
    //     if (collider == victim) {
    //         return false;
    //     }
    //
    //     const bool colliderOpposite = collider->getLaneChangeModel().isOpposite() || collider->isBidiOn(this);
    //     const bool victimOpposite = victim->getLaneChangeModel().isOpposite() || victim->isBidiOn(this);
    //     const bool bothOpposite = victimOpposite && colliderOpposite;
    //     if (bothOpposite) {
    //         std::swap(victim, collider);
    //     }
    //     const double colliderPos = colliderOpposite && !bothOpposite ? collider->getBackPositionOnLane(this) : collider->getPositionOnLane(this);
    //     const double minGapFactor = myCollisionMinGapFactor >= 0 ? myCollisionMinGapFactor : collider->getCarFollowModel().getCollisionMinGapFactor();
    //     double victimBack = victimOpposite && !bothOpposite ? victim->getPositionOnLane(this) : victim->getBackPositionOnLane(this);
    //     if (victim->getLateralOverlap() > 0 || collider->getLateralOverlap() > 0) {
    //         if (&collider->getLane()->getEdge() == myEdge && collider->getLane()->getLength() > getLength()) {
    //             // interpret victim position on the longer lane
    //             victimBack *= collider->getLane()->getLength() / getLength();
    //         }
    //     }
    //     double gap = victimBack - colliderPos - minGapFactor * collider->getVehicleType().getMinGap();
    //     if (bothOpposite) {
    //         gap = colliderPos - victimBack - minGapFactor * collider->getVehicleType().getMinGap();
    //     } else if (colliderOpposite) {
    //         // vehicles are back to back so (frontal) minGap doesn't apply
    //         gap += minGapFactor * collider->getVehicleType().getMinGap();
    //     }
    // #ifdef DEBUG_COLLISIONS
    //     if (DEBUG_COND && (DEBUG_COND2(collider) || DEBUG_COND2(victim))) {
    //         std::cout << SIMTIME
    //                   << " thisLane=" << getID()
    //                   << " collider=" << collider->getID()
    //                   << " victim=" << victim->getID()
    //                   << " colOpposite=" << colliderOpposite
    //                   << " vicOpposite=" << victimOpposite
    //                   << " colLane=" << collider->getLane()->getID()
    //                   << " vicLane=" << victim->getLane()->getID()
    //                   << " colPos=" << colliderPos
    //                   << " vicBack=" << victimBack
    //                   << " colLat=" << collider->getCenterOnEdge(this)
    //                   << " vicLat=" << victim->getCenterOnEdge(this)
    //                   << " minGap=" << collider->getVehicleType().getMinGap()
    //                   << " minGapFactor=" << minGapFactor
    //                   << " gap=" << gap
    //                   << "\n";
    //     }
    // #endif
    //     if (victimOpposite && gap < -(collider->getLength() + victim->getLength())) {
    //         // already past each other
    //         return false;
    //     }
    //     if (gap < -NUMERICAL_EPS) {
    //         double latGap = 0;
    //         if (MSGlobals::gSublane) {
    //             latGap = (fabs(victim->getCenterOnEdge(this) - collider->getCenterOnEdge(this))
    //                       - 0.5 * fabs(victim->getVehicleType().getWidth() + collider->getVehicleType().getWidth()));
    //             if (latGap + NUMERICAL_EPS > 0) {
    //                 return false;
    //             }
    //             // account for ambiguous gap computation related to partial
    //             // occupation of lanes with different lengths
    //             if (isInternal() && getEdge().getNumLanes() > 1 && victim->getLane() != collider->getLane()) {
    //                 double gapDelta = 0;
    //                 const MSVehicle* otherLaneVeh = collider->getLane() == this ? victim : collider;
    //                 if (otherLaneVeh->getLaneChangeModel().getShadowLane() == this) {
    //                     gapDelta = getLength() - otherLaneVeh->getLane()->getLength();
    //                 } else {
    //                     for (const MSLane* cand : otherLaneVeh->getFurtherLanes()) {
    //                         if (&cand->getEdge() == &getEdge()) {
    //                             gapDelta = getLength() - cand->getLength();
    //                             break;
    //                         }
    //                     }
    //                 }
    //                 if (gap + gapDelta >= 0) {
    //                     return false;
    //                 }
    //             }
    //         }
    //         if (MSGlobals::gLaneChangeDuration > DELTA_T
    //                 && collider->getLaneChangeModel().isChangingLanes()
    //                 && victim->getLaneChangeModel().isChangingLanes()
    //                 && victim->getLane() != this) {
    //             // synchroneous lane change maneuver
    //             return false;
    //         }
    // #ifdef DEBUG_COLLISIONS
    //         if (DEBUG_COND && (DEBUG_COND2(collider) || DEBUG_COND2(victim))) {
    //             std::cout << SIMTIME << " detectedCollision gap=" << gap << " latGap=" << latGap << "\n";
    //         }
    // #endif
    //         handleCollisionBetween(timestep, stage, collider, victim, gap, latGap, toRemove, toTeleport);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn detect_collision_between(&self, timestep: SUMOTime, stage: &str, collider: Ptr<MSVehicle>, victim: Ptr<MSVehicle>, toRemove: &mut BTreeSet<Ptr<MSVehicle>>, toTeleport: &mut BTreeSet<Ptr<MSVehicle>>) -> bool {
        let (mut collider, mut victim) = (collider, victim);
        if MSLane::get_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_TELEPORT
            && ((victim.has_influencer() && victim.get_influencer_2().is_remote_affected(timestep))
                || (collider.has_influencer() && collider.get_influencer_2().is_remote_affected(timestep)))
        {
            return false;
        }

        // No self-collisions! (This is assumed to be ensured at caller side)
        if collider == victim {
            return false;
        }

        let this = Ptr::new(self);
        let colliderOpposite = collider.get_lane_change_model_2().is_opposite() || collider.is_bidi_on(this);
        let victimOpposite = victim.get_lane_change_model_2().is_opposite() || victim.is_bidi_on(this);
        let bothOpposite = victimOpposite && colliderOpposite;
        if bothOpposite {
            std::mem::swap(&mut victim, &mut collider);
        }
        let colliderPos = if colliderOpposite && !bothOpposite { collider.get_back_position_on_lane(this) } else { collider.get_position_on_lane_2(this) };
        let collisionMinGapFactor = MY_COLLISION_MIN_GAP_FACTOR.with(|c| c.get());
        let minGapFactor = if collisionMinGapFactor >= 0. { collisionMinGapFactor } else { collider.get_car_follow_model().get_collision_min_gap_factor() };
        let mut victimBack = if victimOpposite && !bothOpposite { victim.get_position_on_lane_2(this) } else { victim.get_back_position_on_lane(this) };
        if victim.get_lateral_overlap() > 0. || collider.get_lateral_overlap() > 0. {
            if collider.get_lane().myEdge == self.myEdge && collider.get_lane().get_length() > self.get_length() {
                // interpret victim position on the longer lane
                victimBack *= collider.get_lane().get_length() / self.get_length();
            }
        }
        let mut gap = victimBack - colliderPos - minGapFactor * collider.get_vehicle_type().get_min_gap();
        if bothOpposite {
            gap = colliderPos - victimBack - minGapFactor * collider.get_vehicle_type().get_min_gap();
        } else if colliderOpposite {
            // vehicles are back to back so (frontal) minGap doesn't apply
            gap += minGapFactor * collider.get_vehicle_type().get_min_gap();
        }
        if victimOpposite && gap < -(collider.get_length() + victim.get_length()) {
            // already past each other
            return false;
        }
        if gap < -NUMERICAL_EPS {
            let mut latGap = 0.;
            if MSGlobals::sublane() {
                latGap = (victim.get_center_on_edge(this) - collider.get_center_on_edge(this)).abs()
                    - 0.5 * (victim.get_vehicle_type().get_width() + collider.get_vehicle_type().get_width()).abs();
                if latGap + NUMERICAL_EPS > 0. {
                    return false;
                }
                // account for ambiguous gap computation related to partial
                // occupation of lanes with different lengths
                if self.is_internal() && self.myEdge.get_num_lanes() > 1 && victim.get_lane() != collider.get_lane() {
                    let mut gapDelta = 0.;
                    let otherLaneVeh = if collider.get_lane() == this { victim } else { collider };
                    if otherLaneVeh.get_lane_change_model_2().get_shadow_lane() == this {
                        gapDelta = self.get_length() - otherLaneVeh.get_lane().get_length();
                    } else {
                        for cand in otherLaneVeh.get_further_lanes() {
                            if cand.myEdge == self.myEdge {
                                gapDelta = self.get_length() - cand.get_length();
                                break;
                            }
                        }
                    }
                    if gap + gapDelta >= 0. {
                        return false;
                    }
                }
            }
            if MSGlobals::lane_change_duration() > utils::common::time::delta_t()
                && collider.get_lane_change_model_2().is_changing_lanes()
                && victim.get_lane_change_model_2().is_changing_lanes()
                && victim.get_lane() != this
            {
                // synchroneous lane change maneuver
                return false;
            }
            self.handle_collision_between(timestep, stage, collider, victim, gap, latGap, toRemove, toTeleport);
            return true;
        }
        false
    }

    /// C++: `void MSLane::handleCollisionBetween(SUMOTime, const std::string &, const MSVehicle *, const MSVehicle *, double, double, std::set<const MSVehicle *, ComparatorNumericalIdLess> &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &) const` [const, protected]
    /// take action upon collision
    // void
    // MSLane::handleCollisionBetween(SUMOTime timestep, const std::string& stage, const MSVehicle* collider, const MSVehicle* victim,
    //                                double gap, double latGap, std::set<const MSVehicle*, ComparatorNumericalIdLess>& toRemove,
    //                                std::set<const MSVehicle*, ComparatorNumericalIdLess>& toTeleport) const {
    //     if (collider->ignoreCollision() || victim->ignoreCollision()) {
    //         return;
    //     }
    //     std::string collisionType;
    //     std::string collisionText;
    //     if (isFrontalCollision(collider, victim)) {
    //         collisionType = "frontal";
    //         collisionText = TL("frontal collision");
    //     } else if (stage == MSNet::STAGE_LANECHANGE) {
    //         collisionType = "side";
    //         collisionText = TL("side collision");
    //     } else if (isInternal()) {
    //         collisionType = "junction";
    //         collisionText = TL("junction collision");
    //     } else {
    //         collisionType = "collision";
    //         collisionText = TL("collision");
    //     }
    //
    //     // in frontal collisions the opposite vehicle is the collider
    //     if (victim->getLaneChangeModel().isOpposite() && !collider->getLaneChangeModel().isOpposite()) {
    //         std::swap(collider, victim);
    //     }
    //     std::string prefix = TLF("Vehicle '%'; % with vehicle '%", collider->getID(), collisionText, victim->getID());
    //     if (myCollisionStopTime > 0) {
    //         if (collider->collisionStopTime() >= 0 && victim->collisionStopTime() >= 0) {
    //             return;
    //         }
    //         std::string dummyError;
    //         SUMOVehicleParameter::Stop stop;
    //         stop.duration = myCollisionStopTime;
    //         stop.parametersSet |= STOP_DURATION_SET;
    //         const double collisionAngle = RAD2DEG(fabs(GeomHelper::angleDiff(victim->getAngle(), collider->getAngle())));
    //         // determine new speeds from collision angle (@todo account for vehicle mass)
    //         double victimSpeed = victim->getSpeed();
    //         double colliderSpeed = collider->getSpeed();
    //         // double victimOrigSpeed = victim->getSpeed();
    //         // double colliderOrigSpeed = collider->getSpeed();
    //         if (collisionAngle < 45) {
    //             // rear-end collisions
    //             colliderSpeed = MIN2(colliderSpeed, victimSpeed);
    //         } else if (collisionAngle < 135) {
    //             // side collision
    //             colliderSpeed /= 2;
    //             victimSpeed /= 2;
    //         } else {
    //             // frontal collision
    //             colliderSpeed = 0;
    //             victimSpeed = 0;
    //         }
    //         const double victimStopPos = MIN2(victim->getLane()->getLength(),
    //                                           victim->getPositionOnLane() + victim->getCarFollowModel().brakeGap(victimSpeed, victim->getCarFollowModel().getEmergencyDecel(), 0));
    //         if (victim->collisionStopTime() < 0) {
    //             stop.collision = true;
    //             stop.lane = victim->getLane()->getID();
    //             // @todo: push victim forward?
    //             stop.startPos = victimStopPos;
    //             stop.endPos = stop.startPos;
    //             stop.parametersSet |= STOP_START_SET | STOP_END_SET;
    //             ((MSBaseVehicle*)victim)->addStop(stop, dummyError, 0);
    //         }
    //         if (collider->collisionStopTime() < 0) {
    //             stop.collision = true;
    //             stop.lane = collider->getLane()->getID();
    //             stop.startPos = MIN2(collider->getPositionOnLane() + collider->getCarFollowModel().brakeGap(colliderSpeed, collider->getCarFollowModel().getEmergencyDecel(), 0),
    //                                  MAX3(0.0, victimStopPos - 0.75 * victim->getVehicleType().getLength(),
    //                                       collider->getPositionOnLane() - SPEED2DIST(collider->getSpeed())));
    //             stop.endPos = stop.startPos;
    //             stop.parametersSet |= STOP_START_SET | STOP_END_SET;
    //             ((MSBaseVehicle*)collider)->addStop(stop, dummyError, 0);
    //         }
    //         //std::cout << " collisionAngle=" << collisionAngle
    //         //    << "\n    vPos=" << victim->getPositionOnLane()   << " vStop=" << victimStopPos  << " vSpeed=" << victimOrigSpeed     << " vSpeed2=" << victimSpeed   << " vSpeed3=" << victim->getSpeed()
    //         //    << "\n    cPos=" << collider->getPositionOnLane() << " cStop=" << stop.startPos  << " cSpeed=" << colliderOrigSpeed   << " cSpeed2=" << colliderSpeed << " cSpeed3=" << collider->getSpeed()
    //         //    << "\n";
    //     } else {
    //         switch (myCollisionAction) {
    //             case COLLISION_ACTION_WARN:
    //                 break;
    //             case COLLISION_ACTION_TELEPORT:
    //                 prefix = TLF("Teleporting vehicle '%'; % with vehicle '%", collider->getID(), collisionText, victim->getID());
    //                 toRemove.insert(collider);
    //                 toTeleport.insert(collider);
    //                 break;
    //             case COLLISION_ACTION_REMOVE: {
    //                 prefix = TLF("Removing % participants: vehicle '%', vehicle '%", collisionText, collider->getID(), victim->getID());
    //                 bool removeCollider = true;
    //                 bool removeVictim = true;
    //                 removeVictim = !(victim->hasInfluencer() && victim->getInfluencer()->isRemoteAffected(timestep));
    //                 removeCollider = !(collider->hasInfluencer() && collider->getInfluencer()->isRemoteAffected(timestep));
    //                 if (removeVictim) {
    //                     toRemove.insert(victim);
    //                 }
    //                 if (removeCollider) {
    //                     toRemove.insert(collider);
    //                 }
    //                 if (!removeVictim) {
    //                     if (!removeCollider) {
    //                         prefix = TLF("Keeping remote-controlled % participants: vehicle '%', vehicle '%", collisionText, collider->getID(), victim->getID());
    //                     } else {
    //                         prefix = TLF("Removing % participant: vehicle '%', keeping remote-controlled vehicle '%", collisionText, collider->getID(), victim->getID());
    //                     }
    //                 } else if (!removeCollider) {
    //                     prefix = TLF("Keeping remote-controlled % participant: vehicle '%', removing vehicle '%", collisionText, collider->getID(), victim->getID());
    //                 }
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     }
    //     const bool newCollision = MSNet::getInstance()->registerCollision(collider, victim, collisionType, this, collider->getPositionOnLane(this));
    //     if (newCollision) {
    //         WRITE_WARNINGF(prefix + "', lane='%', gap=%%, time=%, stage=%.",
    //                        getID(), toString(gap), (MSGlobals::gSublane ? TL(", latGap=") + toString(latGap) : ""),
    //                        time2string(timestep), stage);
    //         MSNet::getInstance()->informVehicleStateListener(victim, MSNet::VehicleState::COLLISION);
    //         MSNet::getInstance()->informVehicleStateListener(collider, MSNet::VehicleState::COLLISION);
    //         MSNet::getInstance()->getVehicleControl().countCollision(myCollisionAction == COLLISION_ACTION_TELEPORT);
    //     }
    // #ifdef DEBUG_COLLISIONS
    //     if (DEBUG_COND2(collider)) {
    //         toRemove.erase(collider);
    //         toTeleport.erase(collider);
    //     }
    //     if (DEBUG_COND2(victim)) {
    //         toRemove.erase(victim);
    //         toTeleport.erase(victim);
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn handle_collision_between(&self, timestep: SUMOTime, stage: &str, collider: Ptr<MSVehicle>, victim: Ptr<MSVehicle>, gap: f64, latGap: f64, toRemove: &mut BTreeSet<Ptr<MSVehicle>>, toTeleport: &mut BTreeSet<Ptr<MSVehicle>>) {
        if collider.ignore_collision() || victim.ignore_collision() {
            return;
        }
        let (mut collider, mut victim) = (collider, victim);
        let collisionType: &str;
        let collisionText: &str;
        if MSLane::is_frontal_collision(collider, victim) {
            collisionType = "frontal";
            collisionText = "frontal collision";
        } else if stage == STAGE_LANECHANGE {
            collisionType = "side";
            collisionText = "side collision";
        } else if self.is_internal() {
            collisionType = "junction";
            collisionText = "junction collision";
        } else {
            collisionType = "collision";
            collisionText = "collision";
        }

        // in frontal collisions the opposite vehicle is the collider
        if victim.get_lane_change_model_2().is_opposite() && !collider.get_lane_change_model_2().is_opposite() {
            std::mem::swap(&mut collider, &mut victim);
        }
        let mut prefix = utils::tlf!("Vehicle '%'; % with vehicle '%", collider.get_id(), collisionText, victim.get_id());
        let collisionStopTime = MY_COLLISION_STOP_TIME.with(|c| c.get());
        if collisionStopTime > 0 {
            if collider.collision_stop_time() >= 0 && victim.collision_stop_time() >= 0 {
                return;
            }
            let min2 = |a: f64, b: f64| if a < b { a } else { b };
            let max2 = |a: f64, b: f64| if a > b { a } else { b };
            let mut dummyError = String::new();
            let mut stop = default_stop();
            stop.duration = collisionStopTime;
            stop.parametersSet |= STOP_DURATION_SET;
            let collisionAngle = (utils::geom::geom_helper::angle_diff(victim.get_angle(), collider.get_angle())).abs();
            let collisionAngle = utils::geom::geom_helper::rad2deg(collisionAngle);
            // determine new speeds from collision angle (@todo account for vehicle mass)
            let mut victimSpeed = victim.get_speed();
            let mut colliderSpeed = collider.get_speed();
            if collisionAngle < 45. {
                // rear-end collisions
                colliderSpeed = min2(colliderSpeed, victimSpeed);
            } else if collisionAngle < 135. {
                // side collision
                colliderSpeed /= 2.;
                victimSpeed /= 2.;
            } else {
                // frontal collision
                colliderSpeed = 0.;
                victimSpeed = 0.;
            }
            let victimStopPos = min2(
                victim.get_lane().get_length(),
                victim.get_position_on_lane() + victim.get_car_follow_model().brake_gap_2(victimSpeed, victim.get_car_follow_model().get_emergency_decel(), 0.),
            );
            if victim.collision_stop_time() < 0 {
                stop.collision = true;
                stop.lane = victim.get_lane().get_id().to_string();
                // @todo: push victim forward?
                stop.startPos = victimStopPos;
                stop.endPos = stop.startPos;
                stop.parametersSet |= STOP_START_SET | STOP_END_SET;
                victim.cast::<MSBaseVehicle>().m().add_stop(&stop, &mut dummyError, 0, Ptr::null());
            }
            if collider.collision_stop_time() < 0 {
                stop.collision = true;
                stop.lane = collider.get_lane().get_id().to_string();
                stop.startPos = min2(
                    collider.get_position_on_lane() + collider.get_car_follow_model().brake_gap_2(colliderSpeed, collider.get_car_follow_model().get_emergency_decel(), 0.),
                    max2(max2(0.0, victimStopPos - 0.75 * victim.get_vehicle_type().get_length()), collider.get_position_on_lane() - utils::common::time::speed2dist(collider.get_speed())),
                );
                stop.endPos = stop.startPos;
                stop.parametersSet |= STOP_START_SET | STOP_END_SET;
                collider.cast::<MSBaseVehicle>().m().add_stop(&stop, &mut dummyError, 0, Ptr::null());
            }
        } else {
            match MSLane::get_collision_action() {
                MSLane_CollisionAction::COLLISION_ACTION_WARN => {}
                MSLane_CollisionAction::COLLISION_ACTION_TELEPORT => {
                    prefix = utils::tlf!("Teleporting vehicle '%'; % with vehicle '%", collider.get_id(), collisionText, victim.get_id());
                    toRemove.insert(collider);
                    toTeleport.insert(collider);
                }
                MSLane_CollisionAction::COLLISION_ACTION_REMOVE => {
                    prefix = utils::tlf!("Removing % participants: vehicle '%', vehicle '%", collisionText, collider.get_id(), victim.get_id());
                    let mut removeCollider = true;
                    let mut removeVictim = true;
                    removeVictim = !(victim.has_influencer() && victim.get_influencer_2().is_remote_affected(timestep));
                    removeCollider = !(collider.has_influencer() && collider.get_influencer_2().is_remote_affected(timestep));
                    if removeVictim {
                        toRemove.insert(victim);
                    }
                    if removeCollider {
                        toRemove.insert(collider);
                    }
                    if !removeVictim {
                        if !removeCollider {
                            prefix = utils::tlf!("Keeping remote-controlled % participants: vehicle '%', vehicle '%", collisionText, collider.get_id(), victim.get_id());
                        } else {
                            prefix = utils::tlf!("Removing % participant: vehicle '%', keeping remote-controlled vehicle '%", collisionText, collider.get_id(), victim.get_id());
                        }
                    } else if !removeCollider {
                        prefix = utils::tlf!("Keeping remote-controlled % participant: vehicle '%', removing vehicle '%", collisionText, collider.get_id(), victim.get_id());
                    }
                }
                _ => {}
            }
        }
        let this = Ptr::new(self);
        let newCollision = MSNet::get_instance().m().register_collision(collider.cast::<SUMOTrafficObject>(), victim.cast::<SUMOTrafficObject>(), collisionType, this, collider.get_position_on_lane_2(this));
        if newCollision {
            let latGapText = if MSGlobals::sublane() { format!(", latGap={}", utils::common::to_string::to_string(&latGap)) } else { String::new() };
            utils::write_warningf!(
                &format!("{}', lane='%', gap=%%, time=%, stage=%.", prefix),
                self.get_id(),
                utils::common::to_string::to_string(&gap),
                latGapText,
                utils::common::time::time2string(timestep),
                stage
            );
            MSNet::get_instance().m().inform_vehicle_state_listener(victim.cast::<SUMOVehicle>(), MSNet_VehicleState::COLLISION, "");
            MSNet::get_instance().m().inform_vehicle_state_listener(collider.cast::<SUMOVehicle>(), MSNet_VehicleState::COLLISION, "");
            MSNet::get_instance().m().get_vehicle_control().count_collision(MSLane::get_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_TELEPORT);
        }
    }

    /// C++: `void MSLane::handleIntermodalCollisionBetween(SUMOTime, const std::string &, const MSVehicle *, const MSTransportable *, double, const std::string &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &, std::set<const MSVehicle *, ComparatorNumericalIdLess> &) const` [const, protected]
    // void
    // MSLane::handleIntermodalCollisionBetween(SUMOTime timestep, const std::string& stage, const MSVehicle* collider, const MSTransportable* victim,
    //         double gap, const std::string& collisionType,
    //         std::set<const MSVehicle*, ComparatorNumericalIdLess>& toRemove,
    //         std::set<const MSVehicle*, ComparatorNumericalIdLess>& toTeleport) const {
    //     if (collider->ignoreCollision()) {
    //         return;
    //     }
    //     std::string prefix = TLF("Vehicle '%'", collider->getID());
    //     if (myIntermodalCollisionStopTime > 0) {
    //         if (collider->collisionStopTime() >= 0) {
    //             return;
    //         }
    //         std::string dummyError;
    //         SUMOVehicleParameter::Stop stop;
    //         stop.duration = myIntermodalCollisionStopTime;
    //         stop.parametersSet |= STOP_DURATION_SET;
    //         // determine new speeds from collision angle (@todo account for vehicle mass)
    //         double colliderSpeed = collider->getSpeed();
    //         const double victimStopPos = victim->getEdgePos();
    //         // double victimOrigSpeed = victim->getSpeed();
    //         // double colliderOrigSpeed = collider->getSpeed();
    //         if (collider->collisionStopTime() < 0) {
    //             stop.collision = true;
    //             stop.lane = collider->getLane()->getID();
    //             stop.startPos = MIN2(collider->getPositionOnLane() + collider->getCarFollowModel().brakeGap(colliderSpeed, collider->getCarFollowModel().getEmergencyDecel(), 0),
    //                                  MAX3(0.0, victimStopPos - 0.75 * victim->getVehicleType().getLength(),
    //                                       collider->getPositionOnLane() - SPEED2DIST(collider->getSpeed())));
    //             stop.endPos = stop.startPos;
    //             stop.parametersSet |= STOP_START_SET | STOP_END_SET;
    //             ((MSBaseVehicle*)collider)->addStop(stop, dummyError, 0);
    //         }
    //     } else {
    //         switch (myIntermodalCollisionAction) {
    //             case COLLISION_ACTION_WARN:
    //                 break;
    //             case COLLISION_ACTION_TELEPORT:
    //                 prefix = TLF("Teleporting vehicle '%' after", collider->getID());
    //                 toRemove.insert(collider);
    //                 toTeleport.insert(collider);
    //                 break;
    //             case COLLISION_ACTION_REMOVE: {
    //                 prefix = TLF("Removing vehicle '%' after", collider->getID());
    //                 bool removeCollider = true;
    //                 removeCollider = !(collider->hasInfluencer() && collider->getInfluencer()->isRemoteAffected(timestep));
    //                 if (!removeCollider) {
    //                     prefix = TLF("Keeping remote-controlled vehicle '%' after", collider->getID());
    //                 } else {
    //                     toRemove.insert(collider);
    //                 }
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     }
    //     const bool newCollision = MSNet::getInstance()->registerCollision(collider, victim, collisionType, this, victim->getEdgePos());
    //     if (newCollision) {
    //         if (gap != 0) {
    //             WRITE_WARNING(prefix + TLF(" collision with person '%', lane='%', gap=%, time=%, stage=%.",
    //                                        victim->getID(), getID(), gap, time2string(timestep), stage));
    //         } else {
    //             WRITE_WARNING(prefix + TLF(" collision with person '%', lane='%', time=%, stage=%.",
    //                                        victim->getID(), getID(), time2string(timestep), stage));
    //         }
    //         MSNet::getInstance()->informVehicleStateListener(collider, MSNet::VehicleState::COLLISION);
    //         MSNet::getInstance()->getVehicleControl().countCollision(myIntermodalCollisionAction == COLLISION_ACTION_TELEPORT);
    //     }
    // #ifdef DEBUG_COLLISIONS
    //     if (DEBUG_COND2(collider)) {
    //         toRemove.erase(collider);
    //         toTeleport.erase(collider);
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn handle_intermodal_collision_between(&self, timestep: SUMOTime, stage: &str, collider: Ptr<MSVehicle>, victim: Ptr<MSTransportable>, gap: f64, collisionType: &str, toRemove: &mut BTreeSet<Ptr<MSVehicle>>, toTeleport: &mut BTreeSet<Ptr<MSVehicle>>) {
        if collider.ignore_collision() {
            return;
        }
        let mut prefix = utils::tlf!("Vehicle '%'", collider.get_id());
        let intermodalStopTime = MY_INTERMODAL_COLLISION_STOP_TIME.with(|c| c.get());
        if intermodalStopTime > 0 {
            if collider.collision_stop_time() >= 0 {
                return;
            }
            let min2 = |a: f64, b: f64| if a < b { a } else { b };
            let max2 = |a: f64, b: f64| if a > b { a } else { b };
            let mut dummyError = String::new();
            let mut stop = default_stop();
            stop.duration = intermodalStopTime;
            stop.parametersSet |= STOP_DURATION_SET;
            // determine new speeds from collision angle (@todo account for vehicle mass)
            let colliderSpeed = collider.get_speed();
            let victimStopPos = victim.get_edge_pos();
            // double victimOrigSpeed = victim->getSpeed();
            // double colliderOrigSpeed = collider->getSpeed();
            if collider.collision_stop_time() < 0 {
                stop.collision = true;
                stop.lane = collider.get_lane().get_id().to_string();
                stop.startPos = min2(
                    collider.get_position_on_lane() + collider.get_car_follow_model().brake_gap_2(colliderSpeed, collider.get_car_follow_model().get_emergency_decel(), 0.),
                    max2(max2(0.0, victimStopPos - 0.75 * victim.get_vehicle_type().get_length()), collider.get_position_on_lane() - utils::common::time::speed2dist(collider.get_speed())),
                );
                stop.endPos = stop.startPos;
                stop.parametersSet |= STOP_START_SET | STOP_END_SET;
                collider.cast::<MSBaseVehicle>().m().add_stop(&stop, &mut dummyError, 0, Ptr::null());
            }
        } else {
            match MSLane::get_intermodal_collision_action() {
                MSLane_CollisionAction::COLLISION_ACTION_WARN => {}
                MSLane_CollisionAction::COLLISION_ACTION_TELEPORT => {
                    prefix = utils::tlf!("Teleporting vehicle '%' after", collider.get_id());
                    toRemove.insert(collider);
                    toTeleport.insert(collider);
                }
                MSLane_CollisionAction::COLLISION_ACTION_REMOVE => {
                    prefix = utils::tlf!("Removing vehicle '%' after", collider.get_id());
                    let mut removeCollider = true;
                    removeCollider = !(collider.has_influencer() && collider.get_influencer_2().is_remote_affected(timestep));
                    if !removeCollider {
                        prefix = utils::tlf!("Keeping remote-controlled vehicle '%' after", collider.get_id());
                    } else {
                        toRemove.insert(collider);
                    }
                }
                _ => {}
            }
        }
        let this = Ptr::new(self);
        let newCollision = MSNet::get_instance().m().register_collision(collider.cast::<SUMOTrafficObject>(), victim.cast::<SUMOTrafficObject>(), collisionType, this, victim.get_edge_pos());
        if newCollision {
            if gap != 0. {
                utils::common::msg_handler::write_warning(&format!(
                    "{}{}",
                    prefix,
                    utils::tlf!(" collision with person '%', lane='%', gap=%, time=%, stage=%.", victim.get_id(), self.get_id(), gap, utils::common::time::time2string(timestep), stage)
                ));
            } else {
                utils::common::msg_handler::write_warning(&format!(
                    "{}{}",
                    prefix,
                    utils::tlf!(" collision with person '%', lane='%', time=%, stage=%.", victim.get_id(), self.get_id(), utils::common::time::time2string(timestep), stage)
                ));
            }
            MSNet::get_instance().m().inform_vehicle_state_listener(collider.cast::<SUMOVehicle>(), MSNet_VehicleState::COLLISION, "");
            MSNet::get_instance().m().get_vehicle_control().count_collision(MSLane::get_intermodal_collision_action() == MSLane_CollisionAction::COLLISION_ACTION_TELEPORT);
        }
    }

    /// C++: `double MSLane::getDepartSpeed(const MSVehicle &, bool &)` [protected]
    // double
    // MSLane::getDepartSpeed(const MSVehicle& veh, bool& patchSpeed) {
    //     double speed = 0;
    //     const SUMOVehicleParameter& pars = veh.getParameter();
    //     DepartSpeedDefinition dsd = pars.departSpeedProcedure;
    //     if (dsd == DepartSpeedDefinition::DEFAULT) {
    //         dsd = myDefaultDepartSpeedDefinition;
    //         if (dsd == DepartSpeedDefinition::GIVEN) {
    //             speed = myDefaultDepartSpeed;
    //         }
    //     } else if (dsd == DepartSpeedDefinition::GIVEN) {
    //         speed = pars.departSpeed;;
    //     }
    //     switch (dsd) {
    //         case DepartSpeedDefinition::GIVEN:
    //             patchSpeed = false;
    //             break;
    //         case DepartSpeedDefinition::RANDOM:
    //             speed = roundDecimal(RandHelper::rand(getVehicleMaxSpeed(&veh)), gPrecisionRandom);
    //             patchSpeed = true;
    //             break;
    //         case DepartSpeedDefinition::MAX:
    //             speed = getVehicleMaxSpeed(&veh);
    //             patchSpeed = true;
    //             break;
    //         case DepartSpeedDefinition::DESIRED:
    //             speed = getVehicleMaxSpeed(&veh);
    //             patchSpeed = false;
    //             break;
    //         case DepartSpeedDefinition::LIMIT:
    //             speed = getVehicleMaxSpeed(&veh) / veh.getChosenSpeedFactor();
    //             patchSpeed = false;
    //             break;
    //         case DepartSpeedDefinition::LAST: {
    //             MSVehicle* last = getLastAnyVehicle();
    //             speed = getVehicleMaxSpeed(&veh);
    //             if (last != nullptr) {
    //                 speed = MIN2(speed, last->getSpeed());
    //                 patchSpeed = false;
    //             }
    //             break;
    //         }
    //         case DepartSpeedDefinition::AVG: {
    //             speed = MIN2(getVehicleMaxSpeed(&veh), getMeanSpeed());
    //             if (getLastAnyVehicle() != nullptr) {
    //                 patchSpeed = false;
    //             }
    //             break;
    //         }
    //         case DepartSpeedDefinition::DEFAULT:
    //         default:
    //             // speed = 0 was set before
    //             patchSpeed = false; // @todo check
    //             break;
    //     }
    //     return speed;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_speed(&mut self, veh: &MSVehicle, patchSpeed: &mut bool) -> f64 {
        let min2 = |a: f64, b: f64| if a < b { a } else { b };
        let mut speed = 0.;
        let pars = veh.get_parameter();
        let mut dsd = pars.departSpeedProcedure;
        if dsd == DepartSpeedDefinition::DEFAULT {
            dsd = *MSLane::get_default_depart_speed_definition();
            if dsd == DepartSpeedDefinition::GIVEN {
                speed = *MSLane::get_default_depart_speed();
            }
        } else if dsd == DepartSpeedDefinition::GIVEN {
            speed = pars.departSpeed;
        }
        let vehPtr = Ptr::new(veh).cast::<SUMOTrafficObject>();
        match dsd {
            DepartSpeedDefinition::GIVEN => {
                *patchSpeed = false;
            }
            DepartSpeedDefinition::RANDOM => {
                speed = utils::common::std_defs::round_decimal(rand_max_default(self.get_vehicle_max_speed(vehPtr)), utils::common::std_defs::g_precision_random());
                *patchSpeed = true;
            }
            DepartSpeedDefinition::MAX => {
                speed = self.get_vehicle_max_speed(vehPtr);
                *patchSpeed = true;
            }
            DepartSpeedDefinition::DESIRED => {
                speed = self.get_vehicle_max_speed(vehPtr);
                *patchSpeed = false;
            }
            DepartSpeedDefinition::LIMIT => {
                speed = self.get_vehicle_max_speed(vehPtr) / veh.get_chosen_speed_factor();
                *patchSpeed = false;
            }
            DepartSpeedDefinition::LAST => {
                let last = self.get_last_any_vehicle();
                speed = self.get_vehicle_max_speed(vehPtr);
                if !last.is_null() {
                    speed = min2(speed, last.get_speed());
                    *patchSpeed = false;
                }
            }
            DepartSpeedDefinition::AVG => {
                speed = min2(self.get_vehicle_max_speed(vehPtr), self.get_mean_speed());
                if !self.get_last_any_vehicle().is_null() {
                    *patchSpeed = false;
                }
            }
            _ => {
                // speed = 0 was set before
                *patchSpeed = false; // @todo check
            }
        }
        speed
    }

    /// C++: `double MSLane::getDepartPosLat(const MSVehicle &)` [protected]
    // double
    // MSLane::getDepartPosLat(const MSVehicle& veh) {
    //     const SUMOVehicleParameter& pars = veh.getParameter();
    //     switch (pars.departPosLatProcedure) {
    //         case DepartPosLatDefinition::GIVEN:
    //             return pars.departPosLat;
    //         case DepartPosLatDefinition::RIGHT:
    //             return -getWidth() * 0.5 + veh.getVehicleType().getWidth() * 0.5;
    //         case DepartPosLatDefinition::LEFT:
    //             return getWidth() * 0.5 - veh.getVehicleType().getWidth() * 0.5;
    //         case DepartPosLatDefinition::RANDOM: {
    //             const double raw = RandHelper::rand(getWidth() - veh.getVehicleType().getWidth()) - getWidth() * 0.5 + veh.getVehicleType().getWidth() * 0.5;
    //             return roundDecimal(raw, gPrecisionRandom);
    //         }
    //         case DepartPosLatDefinition::CENTER:
    //         case DepartPosLatDefinition::DEFAULT:
    //         // @note:
    //         // case DepartPosLatDefinition::FREE
    //         // case DepartPosLatDefinition::RANDOM_FREE
    //         // are not handled here because they involve multiple insertion attempts
    //         default:
    //             return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_depart_pos_lat(&mut self, veh: &MSVehicle) -> f64 {
        let pars = veh.get_parameter();
        match pars.departPosLatProcedure {
            DepartPosLatDefinition::GIVEN => pars.departPosLat,
            DepartPosLatDefinition::RIGHT => -self.get_width() * 0.5 + veh.get_vehicle_type().get_width() * 0.5,
            DepartPosLatDefinition::LEFT => self.get_width() * 0.5 - veh.get_vehicle_type().get_width() * 0.5,
            DepartPosLatDefinition::RANDOM => {
                let raw = rand_max_default(self.get_width() - veh.get_vehicle_type().get_width()) - self.get_width() * 0.5 + veh.get_vehicle_type().get_width() * 0.5;
                utils::common::std_defs::round_decimal(raw, utils::common::std_defs::g_precision_random())
            }
            // @note: FREE and RANDOM_FREE are not handled here because they involve multiple insertion attempts
            _ => 0.,
        }
    }

    /// C++: `double MSLane::safeInsertionSpeed(const MSVehicle *, double, const MSLeaderInfo &, double)` [protected]
    /// return the maximum safe speed for insertion behind leaders (a negative value indicates that safe insertion is impossible)
    // double
    // MSLane::safeInsertionSpeed(const MSVehicle* veh, double seen, const MSLeaderInfo& leaders, double speed) {
    //     double nspeed = speed;
    // #ifdef DEBUG_INSERTION
    //     if (DEBUG_COND2(veh)) {
    //         std::cout << SIMTIME << " safeInsertionSpeed veh=" << veh->getID() << " speed=" << speed << "\n";
    //     }
    // #endif
    //     for (int i = 0; i < leaders.numSublanes(); ++i) {
    //         const MSVehicle* leader = leaders[i];
    //         if (leader != nullptr) {
    //             double gap = leader->getBackPositionOnLane(this) + seen - veh->getVehicleType().getMinGap();
    //             if (leader->getLane() == getBidiLane()) {
    //                 // use distance to front position and account for movement
    //                 gap -= (leader->getLength() + leader->getBrakeGap(true));
    //             }
    //             if (gap < 0) {
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(veh)) {
    //                     std::cout << "    leader=" << leader->getID() << " bPos=" << leader->getBackPositionOnLane(this) << " gap=" << gap << "\n";
    //                 }
    // #endif
    //                 if ((veh->getInsertionChecks() & (int)InsertionCheck::COLLISION) != 0) {
    //                     return INVALID_SPEED;
    //                 } else {
    //                     return 0;
    //                 }
    //             }
    //             nspeed = MIN2(nspeed,
    //                           veh->getCarFollowModel().insertionFollowSpeed(veh, speed, gap, leader->getSpeed(), leader->getCarFollowModel().getMaxDecel(), leader));
    // #ifdef DEBUG_INSERTION
    //             if (DEBUG_COND2(veh)) {
    //                 std::cout << "    leader=" << leader->getID() << " bPos=" << leader->getBackPositionOnLane(this) << " gap=" << gap << " nspeed=" << nspeed << "\n";
    //             }
    // #endif
    //         }
    //     }
    //     return nspeed;
    // }
    #[allow(unused_variables)]
    pub fn safe_insertion_speed(&mut self, veh: Ptr<MSVehicle>, seen: f64, leaders: &MSLeaderInfo, speed: f64) -> f64 {
        let min2 = |a: f64, b: f64| if a < b { a } else { b };
        let this = Ptr::new(self);
        let mut nspeed = speed;
        for i in 0..leaders.num_sublanes() {
            let leader = leaders.index_op(i);
            if !leader.is_null() {
                let mut gap = leader.get_back_position_on_lane(this) + seen - veh.get_vehicle_type().get_min_gap();
                if leader.get_lane() == self.get_bidi_lane() {
                    // use distance to front position and account for movement
                    gap -= leader.get_length() + leader.get_brake_gap(true);
                }
                if gap < 0. {
                    if veh.get_insertion_checks() & (InsertionCheck::COLLISION as i32) != 0 {
                        return crate::microsim::cfmodels::mscf_model::INVALID_SPEED;
                    } else {
                        return 0.;
                    }
                }
                nspeed = min2(
                    nspeed,
                    veh.get_car_follow_model().insertion_follow_speed(veh, speed, gap, leader.get_speed(), leader.get_car_follow_model().get_max_decel(), leader),
                );
            }
        }
        nspeed
    }

    /// C++: `bool MSLane::checkForPedestrians(const MSVehicle *, double &, double &, double, bool) const` [const, protected]
    /// check whether pedestrians on this lane interfere with vehicle insertion
    // bool
    // MSLane::checkForPedestrians(const MSVehicle* aVehicle, double& speed, double& dist,  double pos, bool patchSpeed) const {
    //     if (getEdge().getPersons().size() > 0 && hasPedestrians()) {
    // #ifdef DEBUG_INSERTION
    //         if (DEBUG_COND2(aVehicle)) {
    //             std::cout << SIMTIME << " check for pedestrians on lane=" << getID() << " pos=" << pos << "\n";
    //         }
    // #endif
    //         PersonDist leader = nextBlocking(pos - aVehicle->getVehicleType().getLength(),
    //                                          aVehicle->getRightSideOnLane(), aVehicle->getRightSideOnLane() + aVehicle->getVehicleType().getWidth(), ceil(speed / aVehicle->getCarFollowModel().getMaxDecel()));
    //         if (leader.first != 0) {
    //             const double gap = leader.second - aVehicle->getVehicleType().getLengthWithGap();
    //             const double stopSpeed = aVehicle->getCarFollowModel().stopSpeed(aVehicle, speed, gap, MSCFModel::CalcReason::FUTURE);
    //             if ((gap < 0 && (aVehicle->getInsertionChecks() & ((int)InsertionCheck::COLLISION | (int)InsertionCheck::PEDESTRIAN)) != 0)
    //                     || checkFailure(aVehicle, speed, dist, stopSpeed, patchSpeed, "", InsertionCheck::PEDESTRIAN)) {
    //                 // we may not drive with the given velocity - we crash into the pedestrian
    // #ifdef DEBUG_INSERTION
    //                 if (DEBUG_COND2(aVehicle)) std::cout << SIMTIME
    //                                                          << " isInsertionSuccess lane=" << getID()
    //                                                          << " veh=" << aVehicle->getID()
    //                                                          << " pos=" << pos
    //                                                          << " posLat=" << aVehicle->getLateralPositionOnLane()
    //                                                          << " patchSpeed=" << patchSpeed
    //                                                          << " speed=" << speed
    //                                                          << " stopSpeed=" << stopSpeed
    //                                                          << " pedestrianLeader=" << leader.first->getID()
    //                                                          << " failed (@796)!\n";
    // #endif
    //                 return false;
    //             }
    //         }
    //     }
    //     double backLength = aVehicle->getLength() - pos;
    //     if (backLength > 0 && MSNet::getInstance()->hasPersons()) {
    //         // look upstream for pedestrian crossings
    //         const MSLane* prev = getLogicalPredecessorLane();
    //         const MSLane* cur = this;
    //         while (backLength > 0 && prev != nullptr) {
    //             const MSLink* link = prev->getLinkTo(cur);
    //             if (link->hasFoeCrossing()) {
    //                 for (const MSLane* foe : link->getFoeLanes()) {
    //                     if (foe->isCrossing() && (foe->hasPedestrians() ||
    //                                               (foe->getIncomingLanes()[0].viaLink->getApproachingPersons() != nullptr
    //                                                && foe->getIncomingLanes()[0].viaLink->getApproachingPersons()->size() > 0))) {
    // #ifdef DEBUG_INSERTION
    //                         if (DEBUG_COND2(aVehicle)) std::cout << SIMTIME
    //                                                                  << " isInsertionSuccess lane=" << getID()
    //                                                                  << " veh=" << aVehicle->getID()
    //                                                                  << " pos=" << pos
    //                                                                  << " backCrossing=" << foe->getID()
    //                                                                  << " peds=" << joinNamedToString(foe->getEdge().getPersons(), " ")
    //                                                                  << " approaching=" << foe->getIncomingLanes()[0].viaLink->getApproachingPersons()->size()
    //                                                                  << " failed (@4550)!\n";
    // #endif
    //                         return false;
    //                     }
    //                 }
    //             }
    //             backLength -= prev->getLength();
    //             cur = prev;
    //             prev = prev->getLogicalPredecessorLane();
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_for_pedestrians(&self, aVehicle: Ptr<MSVehicle>, speed: &mut f64, dist: &mut f64, pos: f64, patchSpeed: bool) -> bool {
        if self.get_edge().get_persons().len() > 0 && self.has_pedestrians() {
            let leader = self.next_blocking(
                pos - aVehicle.get_vehicle_type().get_length(),
                aVehicle.get_right_side_on_lane(),
                aVehicle.get_right_side_on_lane() + aVehicle.get_vehicle_type().get_width(),
                (*speed / aVehicle.get_car_follow_model().get_max_decel()).ceil(),
                false,
            );
            if !leader.0.is_null() {
                let gap = leader.1 - aVehicle.get_vehicle_type().get_length_with_gap();
                let stopSpeed = aVehicle.get_car_follow_model().stop_speed(aVehicle, *speed, gap, MSCFModel_CalcReason::FUTURE);
                if (gap < 0. && (aVehicle.get_insertion_checks() & ((InsertionCheck::COLLISION as i32) | (InsertionCheck::PEDESTRIAN as i32))) != 0)
                    || self.check_failure(aVehicle, speed, dist, stopSpeed, patchSpeed, String::new(), InsertionCheck::PEDESTRIAN)
                {
                    // we may not drive with the given velocity - we crash into the pedestrian
                    return false;
                }
            }
        }
        let mut backLength = aVehicle.get_length() - pos;
        if backLength > 0. && MSNet::get_instance().has_persons() {
            // look upstream for pedestrian crossings
            let mut prev = self.get_logical_predecessor_lane();
            let mut cur = Ptr::new(self);
            while backLength > 0. && !prev.is_null() {
                let link = prev.get_link_to(cur);
                if link.has_foe_crossing() {
                    for foe in link.get_foe_lanes() {
                        let via = foe.get_incoming_lanes()[0].viaLink;
                        if foe.is_crossing() && (foe.has_pedestrians() || (!via.get_approaching_persons().is_null() && via.get_approaching_persons().len() > 0)) {
                            return false;
                        }
                    }
                }
                backLength -= prev.get_length();
                cur = prev;
                prev = prev.get_logical_predecessor_lane();
            }
        }
        true
    }

    /// C++: `bool MSLane::hasApproaching(const std::vector<MSLink *> &) const` [const, protected]
    /// check whether any of the outgoing links are being approached
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn has_approaching_2(&self, links: &Vec<Ptr<MSLink>>) -> bool {
        // shortcut: C++ declares this overload but never defines or calls it; it mirrors hasApproaching() over the given links
        for link in links.iter() {
            if link.get_approaching_2().len() > 0 {
                return true;
            }
        }
        false
    }

    /// C++: `double MSLane::getFractionalVehicleLength(bool) const` [const, protected]
    /// return length of fractional vehicles on this lane
    // double
    // MSLane::getFractionalVehicleLength(bool brutto) const {
    //     double sum = 0;
    //     if (myPartialVehicles.size() > 0) {
    //         const MSLane* bidi = getBidiLane();
    //         for (MSVehicle* cand : myPartialVehicles) {
    //             if (MSGlobals::gSublane && cand->getLaneChangeModel().getShadowLane() == this) {
    //                 continue;
    //             }
    //             if (cand->getLane() == bidi) {
    //                 sum += (brutto ? cand->getVehicleType().getLengthWithGap() : cand->getVehicleType().getLength());
    //             } else {
    //                 sum += myLength - cand->getBackPositionOnLane(this);
    //             }
    //         }
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_fractional_vehicle_length(&self, brutto: bool) -> f64 {
        let mut sum = 0.;
        if self.myPartialVehicles.len() > 0 {
            let bidi = self.get_bidi_lane();
            for cand in self.myPartialVehicles.iter() {
                if MSGlobals::sublane() && cand.get_lane_change_model_2().get_shadow_lane() == Ptr::new(self) {
                    continue;
                }
                if cand.get_lane() == bidi {
                    sum += if brutto { cand.get_vehicle_type().get_length_with_gap() } else { cand.get_vehicle_type().get_length() };
                } else {
                    sum += self.myLength - cand.get_back_position_on_lane(Ptr::new(self));
                }
            }
        }
        sum
    }

    /// C++: `bool MSLane::mayContinue(const MSVehicle *) const` [const, protected]
    /// whether the route of the give vehicle might be extended on insertion
    // bool
    // MSLane::mayContinue(const MSVehicle* veh) const {
    //     if (veh->getDevice(typeid(MSDevice_Taxi)) != nullptr) {
    //         // taxi device may assign a new route that continues past the end of the initial route
    //         return true;
    //     }
    //     for (const MSMoveReminder* rem : myMoveReminders) {
    //         if (dynamic_cast<const MSTriggeredRerouter*>(rem) != nullptr) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn may_continue(&self, veh: Ptr<MSVehicle>) -> bool {
        if !veh.get_device("taxi").is_null() {
            // taxi device may assign a new route that continues past the end of the initial route
            return true;
        }
        // dynamic_cast<MSTriggeredRerouter*> is matched by pointer identity to each rerouter's MSMoveReminder base
        let rerouterReminders: Vec<Ptr<MSMoveReminder>> = MSTriggeredRerouter::get_instances().values().map(|r| Ptr::new(&r.base2)).collect();
        if self.myMoveReminders.iter().any(|rem| rerouterReminders.contains(rem)) {
            return true;
        }
        false
    }

    /// C++: `bool MSLane::hasUnsafeLink() const` [const, protected]
    /// whether any link from this lane is unsafe
    // bool
    // MSLane::hasUnsafeLink() const {
    //     for (const MSLink* link : myLinks) {
    //         if (!link->havePriority() || link->getState() == LINKSTATE_ZIPPER) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_unsafe_link(&self) -> bool {
        for link in self.myLinks.iter() {
            if !link.have_priority() || link.get_state() == LINKSTATE_ZIPPER {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLane::isFrontalCollision(const MSVehicle *, const MSVehicle *)` [static, protected]
    /// detect frontal collisions
    // bool
    // MSLane::isFrontalCollision(const MSVehicle* collider, const MSVehicle* victim) {
    //     if (collider->getLaneChangeModel().isOpposite() != victim->getLaneChangeModel().isOpposite()) {
    //         return true;
    //     } else {
    //         const MSEdge* victimBidi = victim->getLane()->getEdge().getBidiEdge();
    //         if (&collider->getLane()->getEdge() == victimBidi) {
    //             return true;
    //         } else {
    //             for (MSLane* further : collider->getFurtherLanes()) {
    //                 if (&further->getEdge() == victimBidi) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_frontal_collision(collider: Ptr<MSVehicle>, victim: Ptr<MSVehicle>) -> bool {
        if collider.get_lane_change_model_2().is_opposite() != victim.get_lane_change_model_2().is_opposite() {
            return true;
        } else {
            let victimBidi = victim.get_lane().myEdge.get_bidi_edge();
            if collider.get_lane().myEdge == victimBidi {
                return true;
            } else {
                for further in collider.get_further_lanes() {
                    if further.myEdge == victimBidi {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++: `void MSLane::MSLane(const MSLane &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSLane) -> Self {
        panic!("MSLane copy constructor is invalidated")
    }

    /// C++: `MSLane & MSLane::operator=(const MSLane &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLane) -> &mut MSLane {
        panic!("MSLane assignment operator is invalidated")
    }

}

/// C++ `LANE_RTREE_QUAL` (microsim/MSLane.h): `RTree<MSLane*, MSLane, float, 2, MSLane::StoringVisitor>`
/// with the float 2D RectSphericalVolume specialisation
pub type LANE_RTREE_QUAL = RTree<Ptr<MSLane>, f32, 2, f32, 8, 4, foreign::rtree::rtree::Float2DVolume>;

/// C++ class `MSLane::StoringVisitor` (microsim/MSLane.h)
pub struct MSLane_StoringVisitor {
    /// The container
    pub myObjects: Ptr<BTreeSet<Ptr<Named>>>,
    pub myShape: Ptr<PositionVector>,
    pub myRange: f64,
    pub myDomain: i32,
}

impl MSLane_StoringVisitor {
    /// C++: `void StoringVisitor::StoringVisitor(std::set<const Named *> &, const PositionVector &, const double, const int)`
    /// Constructor
    // StoringVisitor(std::set<const Named*>& objects, const PositionVector& shape,
    //                        const double range, const int domain)
    //             : myObjects(objects), myShape(shape), myRange(range), myDomain(domain) {}
    #[allow(unused_variables)]
    pub fn new(objects: &mut BTreeSet<Ptr<Named>>, shape: &PositionVector, range: f64, domain: i32) -> Self {
        MSLane_StoringVisitor {
            myObjects: Ptr::from_mut(objects),
            myShape: Ptr::new(shape),
            myRange: range,
            myDomain: domain,
        }
    }

    /// C++: `void StoringVisitor::add(const MSLane *const) const` [const]
    /// Adds the given object to the container
    // void
    // MSLane::StoringVisitor::add(const MSLane* const l) const {
    //     switch (myDomain) {
    //         case libsumo::CMD_GET_VEHICLE_VARIABLE: {
    //             for (const MSVehicle* veh : l->getVehiclesSecure()) {
    //                 if (myShape.distance2D(veh->getPosition()) <= myRange) {
    //                     myObjects.insert(veh);
    //                 }
    //             }
    //             for (const MSBaseVehicle* veh : l->getParkingVehicles()) {
    //                 if (myShape.distance2D(veh->getPosition()) <= myRange) {
    //                     myObjects.insert(veh);
    //                 }
    //             }
    //             l->releaseVehicles();
    //         }
    //         break;
    //         case libsumo::CMD_GET_PERSON_VARIABLE: {
    //             l->getVehiclesSecure();
    //             std::vector<MSTransportable*> persons = l->getEdge().getSortedPersons(MSNet::getInstance()->getCurrentTimeStep(), true);
    //             for (auto p : persons) {
    //                 if (myShape.distance2D(p->getPosition()) <= myRange) {
    //                     myObjects.insert(p);
    //                 }
    //             }
    //             l->releaseVehicles();
    //         }
    //         break;
    //         case libsumo::CMD_GET_EDGE_VARIABLE: {
    //             if (myShape.size() != 1 || l->getShape().distance2D(myShape[0]) <= myRange) {
    //                 myObjects.insert(&l->getEdge());
    //             }
    //         }
    //         break;
    //         case libsumo::CMD_GET_LANE_VARIABLE: {
    //             if (myShape.size() != 1 || l->getShape().distance2D(myShape[0]) <= myRange) {
    //                 myObjects.insert(l);
    //             }
    //         }
    //         break;
    //         default:
    //             break;
    //
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add(&self, l: Ptr<MSLane>) {
        match self.myDomain {
            CMD_GET_VEHICLE_VARIABLE => {
                for veh in l.get_vehicles_secure() {
                    if self.myShape.distance2d(&veh.get_position(0.), false) <= self.myRange {
                        self.myObjects.m().insert(veh.cast::<Named>());
                    }
                }
                for veh in l.get_parking_vehicles() {
                    if self.myShape.distance2d(&veh.get_position(0.), false) <= self.myRange {
                        self.myObjects.m().insert(veh.cast::<Named>());
                    }
                }
                l.release_vehicles();
            }
            CMD_GET_PERSON_VARIABLE => {
                l.get_vehicles_secure();
                let persons = l.get_edge().get_sorted_persons(MSNet::get_instance().get_current_time_step(), true);
                for p in persons {
                    if self.myShape.distance2d(&p.get_position_2(), false) <= self.myRange {
                        self.myObjects.m().insert(p.cast::<Named>());
                    }
                }
                l.release_vehicles();
            }
            CMD_GET_EDGE_VARIABLE => {
                if self.myShape.len() != 1 || l.get_shape_2().distance2d(&self.myShape[0], false) <= self.myRange {
                    self.myObjects.m().insert(l.myEdge.cast::<Named>());
                }
            }
            CMD_GET_LANE_VARIABLE => {
                if self.myShape.len() != 1 || l.get_shape_2().distance2d(&self.myShape[0], false) <= self.myRange {
                    self.myObjects.m().insert(l.cast::<Named>());
                }
            }
            _ => {}
        }
    }

    /// C++: `void StoringVisitor::StoringVisitor(const StoringVisitor &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &MSLane_StoringVisitor) -> Self {
        panic!("StoringVisitor copy constructor is invalidated")
    }

    /// C++: `StoringVisitor & StoringVisitor::operator=(const StoringVisitor &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &MSLane_StoringVisitor) -> &mut MSLane_StoringVisitor {
        panic!("StoringVisitor assignment operator is invalidated")
    }

}

/// Sorts edges by their angle relative to the given edge (straight comes first)
/// C++ class `MSLane::by_connections_to_sorter` (microsim/MSLane.h)
pub struct MSLane_by_connections_to_sorter {
    pub myEdge: Ptr<MSEdge>,
    pub myLaneDir: f64,
}

impl MSLane_by_connections_to_sorter {
    /// C++: `void by_connections_to_sorter::by_connections_to_sorter(const MSEdge *const)`
    /// constructor
    // MSLane::by_connections_to_sorter::by_connections_to_sorter(const MSEdge* const e) :
    //     myEdge(e),
    //     myLaneDir(e->getLanes()[0]->getShape().angleAt2D(0)) {
    // }
    #[allow(unused_variables)]
    pub fn new(e: Ptr<MSEdge>) -> Self {
        MSLane_by_connections_to_sorter {
            myEdge: e,
            myLaneDir: e.get_lanes()[0].get_shape_2().angle_at2d(0),
        }
    }

    /// C++: `int by_connections_to_sorter::operator()(const MSEdge *const, const MSEdge *const) const` [const]
    /// comparing operator
    // int
    // MSLane::by_connections_to_sorter::operator()(const MSEdge* const e1, const MSEdge* const e2) const {
    // //    std::cout << "\nby_connections_to_sorter()";
    //
    //     const std::vector<MSLane*>* ae1 = e1->allowedLanes(*myEdge);
    //     const std::vector<MSLane*>* ae2 = e2->allowedLanes(*myEdge);
    //     double s1 = 0;
    //     if (ae1 != nullptr && ae1->size() != 0) {
    // //        std::cout << "\nsize 1 = " << ae1->size()
    // //        << " anglediff 1 = " << fabs(GeomHelper::angleDiff((*ae1)[0]->getShape().angleAt2D(0), myLaneDir)) / M_PI / 2.
    // //        << "\nallowed lanes: ";
    // //        for (std::vector<MSLane*>::const_iterator j = ae1->begin(); j != ae1->end(); ++j){
    // //            std::cout << "\n" << (*j)->getID();
    // //        }
    //         s1 = (double) ae1->size() + fabs(GeomHelper::angleDiff((*ae1)[0]->getShape().angleAt2D(0), myLaneDir)) / M_PI / 2.;
    //     }
    //     double s2 = 0;
    //     if (ae2 != nullptr && ae2->size() != 0) {
    // //        std::cout << "\nsize 2 = " << ae2->size()
    // //        << " anglediff 2 = " << fabs(GeomHelper::angleDiff((*ae2)[0]->getShape().angleAt2D(0), myLaneDir)) / M_PI / 2.
    // //        << "\nallowed lanes: ";
    // //        for (std::vector<MSLane*>::const_iterator j = ae2->begin(); j != ae2->end(); ++j){
    // //            std::cout << "\n" << (*j)->getID();
    // //        }
    //         s2 = (double) ae2->size() + fabs(GeomHelper::angleDiff((*ae2)[0]->getShape().angleAt2D(0), myLaneDir)) / M_PI / 2.;
    //     }
    //
    // //    std::cout << "\ne1 = " << e1->getID() << " e2 = " << e2->getID()
    // //            << "\ns1 = " << s1 << " s2 = " << s2
    // //            << std::endl;
    //
    //     return s1 < s2;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, e1: Ptr<MSEdge>, e2: Ptr<MSEdge>) -> i32 {
        let ae1 = e1.allowed_lanes(&self.myEdge, utils::common::sumo_vehicle_class::SVC_IGNORING, false);
        let ae2 = e2.allowed_lanes(&self.myEdge, utils::common::sumo_vehicle_class::SVC_IGNORING, false);
        let mut s1 = 0.;
        if !ae1.is_null() && ae1.len() != 0 {
            s1 = ae1.len() as f64 + utils::geom::geom_helper::angle_diff(ae1[0].get_shape_2().angle_at2d(0), self.myLaneDir).abs() / utils::geom::geom_helper::M_PI / 2.;
        }
        let mut s2 = 0.;
        if !ae2.is_null() && ae2.len() != 0 {
            s2 = ae2.len() as f64 + utils::geom::geom_helper::angle_diff(ae2[0].get_shape_2().angle_at2d(0), self.myLaneDir).abs() / utils::geom::geom_helper::M_PI / 2.;
        }
        if s1 < s2 { 1 } else { 0 }
    }

}

/// C++ class `MSLane::edge_finder` (microsim/MSLane.h)
pub struct MSLane_edge_finder {
    pub myEdge: Ptr<MSEdge>,
}

impl MSLane_edge_finder {
    /// C++: `void edge_finder::edge_finder(MSEdge *)`
    // edge_finder(MSEdge* e) : myEdge(e) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<MSEdge>) -> Self {
        MSLane_edge_finder { myEdge: e }
    }

    /// C++: `bool edge_finder::operator()(const IncomingLaneInfo &) const` [const]
    // bool operator()(const IncomingLaneInfo& ili) const {
    //             return &(ili.lane->getEdge()) == myEdge;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, ili: &MSLane_IncomingLaneInfo) -> bool {
        ili.lane.myEdge == self.myEdge
    }

}

/// Sorts lanes (IncomingLaneInfos) by their priority or, if this doesn't apply, wrt. the angle difference magnitude relative to the target lane's angle (straight comes first)
/// C++ class `MSLane::incoming_lane_priority_sorter` (microsim/MSLane.h)
pub struct MSLane_incoming_lane_priority_sorter {
    pub myLane: Ptr<MSLane>,
    pub myLaneDir: f64,
}

impl MSLane_incoming_lane_priority_sorter {
    /// C++: `void incoming_lane_priority_sorter::incoming_lane_priority_sorter(const MSLane *)`
    /// constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(targetLane: Ptr<MSLane>) -> Self {
        MSLane_incoming_lane_priority_sorter {
            myLane: targetLane,
            myLaneDir: targetLane.get_shape_2().angle_at2d(0),
        }
    }

    /// C++: `int incoming_lane_priority_sorter::operator()(const IncomingLaneInfo &, const IncomingLaneInfo &) const` [const]
    /// comparing operator
    // int
    // MSLane::incoming_lane_priority_sorter::operator()(const IncomingLaneInfo& laneInfo1, const IncomingLaneInfo& laneInfo2) const {
    //     const MSLane* noninternal1 = laneInfo1.lane;
    //     while (noninternal1->isInternal()) {
    //         assert(noninternal1->getIncomingLanes().size() == 1);
    //         noninternal1 = noninternal1->getIncomingLanes()[0].lane;
    //     }
    //     MSLane* noninternal2 = laneInfo2.lane;
    //     while (noninternal2->isInternal()) {
    //         assert(noninternal2->getIncomingLanes().size() == 1);
    //         noninternal2 = noninternal2->getIncomingLanes()[0].lane;
    //     }
    //
    //     const MSLink* link1 = noninternal1->getLinkTo(myLane);
    //     const MSLink* link2 = noninternal2->getLinkTo(myLane);
    //
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "\nincoming_lane_priority sorter()\n"
    //               << "noninternal predecessor for lane '" << laneInfo1.lane->getID()
    //               << "': '" << noninternal1->getID() << "'\n"
    //               << "noninternal predecessor for lane '" << laneInfo2.lane->getID()
    //               << "': '" << noninternal2->getID() << "'\n";
    // #endif
    //
    //     assert(laneInfo1.lane->isInternal() || link1 == laneInfo1.viaLink);
    //     assert(link1 != 0);
    //     assert(link2 != 0);
    //
    //     // check priority between links
    //     bool priorized1 = true;
    //     bool priorized2 = true;
    //
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "FoeLinks of '" << noninternal1->getID() << "'" << std::endl;
    // #endif
    //     for (const MSLink* const foeLink : link1->getFoeLinks()) {
    // #ifdef DEBUG_LANE_SORTER
    //         std::cout << foeLink->getLaneBefore()->getID() << std::endl;
    // #endif
    //         if (foeLink == link2) {
    //             priorized1 = false;
    //             break;
    //         }
    //     }
    //
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "FoeLinks of '" << noninternal2->getID() << "'" << std::endl;
    // #endif
    //     for (const MSLink* const foeLink : link2->getFoeLinks()) {
    // #ifdef DEBUG_LANE_SORTER
    //         std::cout << foeLink->getLaneBefore()->getID() << std::endl;
    // #endif
    //         // either link1 is priorized, or it should not appear in link2's foes
    //         if (foeLink == link1) {
    //             priorized2 = false;
    //             break;
    //         }
    //     }
    //     // if one link is subordinate, the other must be priorized (except for
    //     // traffic lights where mutual response is permitted to handle stuck-on-red
    //     // situation)
    //     if (priorized1 != priorized2) {
    //         return priorized1;
    //     }
    //
    //     // both are priorized, compare angle difference
    //     double d1 = fabs(GeomHelper::angleDiff(noninternal1->getShape().angleAt2D(0), myLaneDir));
    //     double d2 = fabs(GeomHelper::angleDiff(noninternal2->getShape().angleAt2D(0), myLaneDir));
    //
    //     return d2 > d1;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, lane1: &MSLane_IncomingLaneInfo, lane2: &MSLane_IncomingLaneInfo) -> i32 {
        let mut noninternal1 = lane1.lane;
        while noninternal1.is_internal() {
            debug_assert!(noninternal1.get_incoming_lanes().len() == 1);
            noninternal1 = noninternal1.get_incoming_lanes()[0].lane;
        }
        let mut noninternal2 = lane2.lane;
        while noninternal2.is_internal() {
            debug_assert!(noninternal2.get_incoming_lanes().len() == 1);
            noninternal2 = noninternal2.get_incoming_lanes()[0].lane;
        }

        let link1 = noninternal1.get_link_to(self.myLane);
        let link2 = noninternal2.get_link_to(self.myLane);

        debug_assert!(lane1.lane.is_internal() || link1 == lane1.viaLink);
        debug_assert!(!link1.is_null());
        debug_assert!(!link2.is_null());

        // check priority between links
        let mut priorized1 = true;
        let mut priorized2 = true;

        for foeLink in link1.get_foe_links() {
            if *foeLink == link2 {
                priorized1 = false;
                break;
            }
        }

        for foeLink in link2.get_foe_links() {
            // either link1 is priorized, or it should not appear in link2's foes
            if *foeLink == link1 {
                priorized2 = false;
                break;
            }
        }
        // if one link is subordinate, the other must be priorized (except for
        // traffic lights where mutual response is permitted to handle stuck-on-red
        // situation)
        if priorized1 != priorized2 {
            return priorized1 as i32;
        }

        // both are priorized, compare angle difference
        let d1 = utils::geom::geom_helper::angle_diff(noninternal1.get_shape_2().angle_at2d(0), self.myLaneDir).abs();
        let d2 = utils::geom::geom_helper::angle_diff(noninternal2.get_shape_2().angle_at2d(0), self.myLaneDir).abs();

        (d2 > d1) as i32
    }

}

/// Sorts lanes (their origin link) by the priority of their noninternal target edges or, if this doesn't yield an unambiguous result, wrt. the angle difference magnitude relative to the target lane's angle (straight comes first)
/// C++ class `MSLane::outgoing_lane_priority_sorter` (microsim/MSLane.h)
pub struct MSLane_outgoing_lane_priority_sorter {
    pub myLaneDir: f64,
}

impl MSLane_outgoing_lane_priority_sorter {
    /// C++: `void outgoing_lane_priority_sorter::outgoing_lane_priority_sorter(const MSLane *)`
    /// constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(sourceLane: Ptr<MSLane>) -> Self {
        MSLane_outgoing_lane_priority_sorter {
            myLaneDir: sourceLane.get_shape_2().angle_at2d(0),
        }
    }

    /// C++: `int outgoing_lane_priority_sorter::operator()(const MSLink *, const MSLink *) const` [const]
    /// comparing operator
    // int
    // MSLane::outgoing_lane_priority_sorter::operator()(const MSLink* link1, const MSLink* link2) const {
    //     const MSLane* target1 = link1->getLane();
    //     const MSLane* target2 = link2->getLane();
    //     if (target2 == nullptr) {
    //         return true;
    //     }
    //     if (target1 == nullptr) {
    //         return false;
    //     }
    //
    // #ifdef DEBUG_LANE_SORTER
    //     std::cout << "\noutgoing_lane_priority sorter()\n"
    //               << "noninternal successors for lane '" << myLane->getID()
    //               << "': '" << target1->getID() << "' and "
    //               << "'" << target2->getID() << "'\n";
    // #endif
    //
    //     // priority of targets
    //     int priority1 = target1->getEdge().getPriority();
    //     int priority2 = target2->getEdge().getPriority();
    //
    //     if (priority1 != priority2) {
    //         return priority1 > priority2;
    //     }
    //
    //     // if priority of targets coincides, use angle difference
    //
    //     // both are priorized, compare angle difference
    //     double d1 = fabs(GeomHelper::angleDiff(target1->getShape().angleAt2D(0), myLaneDir));
    //     double d2 = fabs(GeomHelper::angleDiff(target2->getShape().angleAt2D(0), myLaneDir));
    //
    //     return d2 > d1;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, link1: Ptr<MSLink>, link2: Ptr<MSLink>) -> i32 {
        let target1 = link1.get_lane();
        let target2 = link2.get_lane();
        if target2.is_null() {
            return 1;
        }
        if target1.is_null() {
            return 0;
        }

        // priority of targets
        let priority1 = target1.myEdge.get_priority();
        let priority2 = target2.myEdge.get_priority();

        if priority1 != priority2 {
            return (priority1 > priority2) as i32;
        }

        // if priority of targets coincides, use angle difference

        // both are priorized, compare angle difference
        let d1 = utils::geom::geom_helper::angle_diff(target1.get_shape_2().angle_at2d(0), self.myLaneDir).abs();
        let d2 = utils::geom::geom_helper::angle_diff(target2.get_shape_2().angle_at2d(0), self.myLaneDir).abs();

        (d2 > d1) as i32
    }

}

/// Sorts vehicles by their position (ascending)
/// C++ class `MSLane::vehicle_natural_position_sorter` (microsim/MSLane.h)
pub struct MSLane_vehicle_natural_position_sorter {
    pub myLane: Ptr<MSLane>,
}

impl MSLane_vehicle_natural_position_sorter {
    /// C++: `void vehicle_natural_position_sorter::vehicle_natural_position_sorter(const MSLane *)`
    /// Constructor
    // explicit vehicle_natural_position_sorter(const MSLane* lane) :
    //             myLane(lane) {
    //         }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>) -> Self {
        MSLane_vehicle_natural_position_sorter { myLane: lane }
    }

    /// C++: `int vehicle_natural_position_sorter::operator()(MSVehicle *, MSVehicle *) const` [const]
    /// Comparing operator
    // int
    // MSLane::vehicle_natural_position_sorter::operator()(MSVehicle* v1, MSVehicle* v2) const {
    //     const double pos1 = v1->getBackPositionOnLane(myLane);
    //     const double pos2 = v2->getBackPositionOnLane(myLane);
    //     if (pos1 != pos2) {
    //         return pos1 < pos2;
    //     } else {
    //         return v1->getLateralPositionOnLane() < v2->getLateralPositionOnLane();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, v1: Ptr<MSVehicle>, v2: Ptr<MSVehicle>) -> i32 {
        let pos1 = v1.get_back_position_on_lane(self.myLane);
        let pos2 = v2.get_back_position_on_lane(self.myLane);
        if pos1 != pos2 {
            (pos1 < pos2) as i32
        } else {
            (v1.get_lateral_position_on_lane() < v2.get_lateral_position_on_lane()) as i32
        }
    }

}

/// Sorts vehicles by their position (descending)
/// C++ class `MSLane::vehicle_position_sorter` (microsim/MSLane.h)
pub struct MSLane_vehicle_position_sorter {
    pub myLane: Ptr<MSLane>,
}

impl MSLane_vehicle_position_sorter {
    /// C++: `void vehicle_position_sorter::vehicle_position_sorter(const MSLane *)`
    /// Constructor
    // explicit vehicle_position_sorter(const MSLane* lane) :
    //             myLane(lane) {
    //         }
    #[allow(unused_variables)]
    pub fn new(lane: Ptr<MSLane>) -> Self {
        MSLane_vehicle_position_sorter { myLane: lane }
    }

    /// C++: `int vehicle_position_sorter::operator()(MSVehicle *, MSVehicle *) const` [const]
    /// Comparing operator
    // int
    // MSLane::vehicle_position_sorter::operator()(MSVehicle* v1, MSVehicle* v2) const {
    //     const double pos1 = v1->getBackPositionOnLane(myLane);
    //     const double pos2 = v2->getBackPositionOnLane(myLane);
    //     if (pos1 != pos2) {
    //         return pos1 > pos2;
    //     } else {
    //         return v1->getNumericalID() > v2->getNumericalID();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, v1: Ptr<MSVehicle>, v2: Ptr<MSVehicle>) -> i32 {
        let pos1 = v1.get_back_position_on_lane(self.myLane);
        let pos2 = v2.get_back_position_on_lane(self.myLane);
        if pos1 != pos2 {
            (pos1 > pos2) as i32
        } else {
            (v1.get_numerical_id() > v2.get_numerical_id()) as i32
        }
    }

}

