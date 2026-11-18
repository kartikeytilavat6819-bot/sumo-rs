//! Port of microsim/MSLink.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::RefCell;
use std::collections::BTreeSet;
use utils::tlf;
use utils::geom::POSITION_EPS;
const VEHPARS_JUNCTIONMODEL_PARAMS_SET: i64 = 2 << 28;

/// C++ MAX3(a, b, c) = MAX2(c, a > b ? a : b)
fn max3(a: f64, b: f64, c: f64) -> f64 {
    max2(c, if a > b { a } else { b })
}

fn g_compute_lc() -> bool {
    crate::microsim::ms_globals::compute_lc()
}

fn g_sublane() -> bool {
    crate::microsim::ms_globals::sublane()
}

fn g_ignore_junction_blocker() -> SUMOTime {
    crate::microsim::ms_globals::ignore_junction_blocker()
}

/// C++ implicit upcast MSVehicle* -> SUMOVehicle* (MSVehicle -> MSBaseVehicle -> SUMOVehicle), null-safe
fn msveh_to_sumo_vehicle(v: Ptr<MSVehicle>) -> Ptr<SUMOVehicle> {
    if v.is_null() {
        return Ptr::null();
    }
    Ptr::from_mut(&mut v.m().base.base)
}

/// C++ `for (it = lane->anyVehiclesBegin(); it != lane->anyVehiclesEnd(); ++it)`: myVehicles, then
/// myTmpVehicles (vehicles the lane changer already processed this step), merged by position with myPartialVehicles
fn any_vehicles_of(lane: Ptr<MSLane>) -> Vec<Ptr<MSVehicle>> {
    let end = lane.any_vehicles_end();
    let mut it = lane.any_vehicles_begin();
    let mut result = Vec::new();
    while it.ne_op(&end) {
        result.push(it.op_());
        it.op__2();
    }
    result
}

/// C++ `if (!item->isVehicle()) continue; dynamic_cast<const MSVehicle*>(item)`
fn zipper_foe_as_vehicle(item: Ptr<SUMOTrafficObject>) -> Option<Ptr<MSVehicle>> {
    if item.is_vehicle() { Some(item.cast::<MSVehicle>()) } else { None }
}
use utils::common::to_string::f64_to_string;
use utils::common::std_defs::INVALID_DOUBLE;

/// C++ implicit upcast MSVehicle* -> SUMOTrafficObject* (MSVehicle -> MSBaseVehicle -> SUMOVehicle -> SUMOTrafficObject)
fn msveh_to_traffic_object(v: Ptr<MSVehicle>) -> Ptr<SUMOTrafficObject> {
    if v.is_null() { return Ptr::null(); }
    Ptr::from_mut(&mut v.m().base.base.base)
}

// shortcut: MSPModel::SAFETY_GAP (msp_model.rs) is not ported yet, this stub holds the C++ value 1.0, upgrade when MSPModel constants are ported
const MSPMODEL_SAFETY_GAP: f64 = 1.0;
use utils::common::std_defs::SUMO_CONST_HALTING_SPEED;

/// C++ implicit upcast SUMOVehicle* -> SUMOTrafficObject* (base sub-object, same address)
fn veh_to_traffic_object(v: Ptr<SUMOVehicle>) -> Ptr<SUMOTrafficObject> {
    if v.is_null() { return Ptr::null(); }
    Ptr::from_mut(&mut v.m().base)
}

/// C++ implicit upcast MSPerson* -> SUMOTrafficObject* (through MSTransportable)
fn person_to_traffic_object(p: Ptr<MSPerson>) -> Ptr<SUMOTrafficObject> {
    if p.is_null() { return Ptr::null(); }
    Ptr::from_mut(&mut p.m().base.base)
}

fn ego_rand(ego: Ptr<SUMOTrafficObject>) -> f64 {
    ego.get_rng().m().rand()
}
use utils::geom::NUMERICAL_EPS;
use utils::geom::geom_helper::{angle_diff, deg2rad, get_min_angle_diff, navi_degree, INVALID_OFFSET};

/// C++ `#define INVALID_TIME -1000` (MSLink.cpp)
const INVALID_TIME: SUMOTime = -1000;

fn g_lefthand() -> bool {
    crate::microsim::ms_globals::lefthand()
}

fn g_use_meso_sim() -> bool {
    crate::microsim::ms_globals::use_meso_sim()
}

fn g_using_internal_lanes() -> bool {
    crate::microsim::ms_globals::using_internal_lanes()
}

/// C++ `#define DIVERGENCE_MIN_WIDTH 2.5` (MSLink.cpp)
const DIVERGENCE_MIN_WIDTH: f64 = 2.5;

/// MIN2/MAX2 from StdDefs.h (differ from f64::min/max for NaN)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ `myRecheck.insert({a, b})`
fn recheck_insert(a: Ptr<MSLink>, b: Ptr<MSLink>) {
    MY_RECHECK.with(|r| r.borrow_mut().insert((a, b)));
}

/// C++ `myRecheck.erase({a, b})`
fn recheck_erase(a: Ptr<MSLink>, b: Ptr<MSLink>) {
    MY_RECHECK.with(|r| r.borrow_mut().remove(&(a, b)));
}

/// C++ enum `MSLink::ConflictFlag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSLink_ConflictFlag {
    CONFLICT_DEFAULT = 0,
    CONFLICT_DUMMY_MERGE = 1,
    CONFLICT_NO_INTERSECTION = 2,
    CONFLICT_STOP_AT_INTERNAL_JUNCTION = 3,
    CONFLICT_SIBLING_CONTINUATION = 4,
}

/// C++ enum `MSLink::LinkLeaderFlag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSLink_LinkLeaderFlag {
    LL_IN_THE_WAY = 1,
    LL_FROM_LEFT = 2,
    LL_SAME_SOURCE = 4,
    LL_SAME_TARGET = 8,
}

/// C++ typedef `std::map<const SUMOVehicle *, const ApproachingVehicleInformation, ComparatorNumericalIdLess> MSLink::ApproachInfos`
pub type MSLink_ApproachInfos = BTreeMap<Ptr<SUMOVehicle>, MSLink_ApproachingVehicleInformation>;

/// C++ typedef `std::vector<const SUMOTrafficObject *> MSLink::BlockingFoes`
pub type MSLink_BlockingFoes = Vec<Ptr<SUMOTrafficObject>>;

/// C++ typedef `std::vector<LinkLeader> MSLink::LinkLeaders`
pub type MSLink_LinkLeaders = Vec<MSLink_LinkLeader>;

/// C++ typedef `std::map<const MSPerson *, ApproachingPersonInformation> MSLink::PersonApproachInfos`
pub type MSLink_PersonApproachInfos = BTreeMap<Ptr<MSPerson>, MSLink_ApproachingPersonInformation>;

/// A structure holding the information about persons approaching a pedestrian crossing link
/// C++ class `MSLink::ApproachingPersonInformation` (microsim/MSLink.h)
pub struct MSLink_ApproachingPersonInformation {
    /// The time the vehicle's front arrives at the link
    pub arrivalTime: SUMOTime,
    /// The estimated time at which the vehicle leaves the link
    pub leavingTime: SUMOTime,
}

impl MSLink_ApproachingPersonInformation {
    /// C++: `void ApproachingPersonInformation::ApproachingPersonInformation(const SUMOTime, const SUMOTime)`
    /// Constructor
    // ApproachingPersonInformation(const SUMOTime _arrivalTime, const SUMOTime _leavingTime) :
    //             arrivalTime(_arrivalTime), leavingTime(_leavingTime) {}
    #[allow(unused_variables)]
    pub fn new(_arrivalTime: SUMOTime, _leavingTime: SUMOTime) -> Self {
        MSLink_ApproachingPersonInformation { arrivalTime: _arrivalTime, leavingTime: _leavingTime }
    }

}

/// A structure holding the information about vehicles approaching a link
/// C++ class `MSLink::ApproachingVehicleInformation` (microsim/MSLink.h)
pub struct MSLink_ApproachingVehicleInformation {
    /// The time the vehicle's front arrives at the link
    pub arrivalTime: SUMOTime,
    /// The estimated time at which the vehicle leaves the link
    pub leavingTime: SUMOTime,
    /// The estimated speed with which the vehicle arrives at the link (for headway computation)
    pub arrivalSpeed: f64,
    /// The estimated speed with which the vehicle leaves the link (for headway computation)
    pub leaveSpeed: f64,
    /// Whether the vehicle wants to pass the link (
    pub willPass: bool,
    /// The estimated speed with which the vehicle arrives at the link if it starts braking(for headway computation)
    pub arrivalSpeedBraking: f64,
    /// The waiting duration at the current link
    pub waitingTime: SUMOTime,
    /// The distance up to the current link
    pub dist: f64,
    /// The current speed
    pub speed: f64,
    /// The lateral offset from the center of the entering lane
    pub latOffset: f64,
}

impl MSLink_ApproachingVehicleInformation {
    /// C++: `void ApproachingVehicleInformation::ApproachingVehicleInformation(const SUMOTime, const SUMOTime, const double, const double, const bool, const double, const SUMOTime, const double, const double, const double)`
    /// Constructor
    // ApproachingVehicleInformation(const SUMOTime _arrivalTime, const SUMOTime _leavingTime,
    //                                       const double _arrivalSpeed, const double _leaveSpeed,
    //                                       const bool _willPass,
    //                                       const double _arrivalSpeedBraking,
    //                                       const SUMOTime _waitingTime,
    //                                       const double _dist,
    //                                       const double _speed,
    //                                       const double _latOffset
    //                                      ) :
    //             arrivalTime(_arrivalTime), leavingTime(_leavingTime),
    //             arrivalSpeed(_arrivalSpeed), leaveSpeed(_leaveSpeed),
    //             willPass(_willPass),
    //             arrivalSpeedBraking(_arrivalSpeedBraking),
    //             waitingTime(_waitingTime),
    //             dist(_dist),
    //             speed(_speed),
    //             latOffset(_latOffset) {
    //         }
    #[allow(unused_variables)]
    pub fn new(_arrivalTime: SUMOTime, _leavingTime: SUMOTime, _arrivalSpeed: f64, _leaveSpeed: f64, _willPass: bool, _arrivalSpeedBraking: f64, _waitingTime: SUMOTime, _dist: f64, _speed: f64, _latOffset: f64) -> Self {
        MSLink_ApproachingVehicleInformation {
            arrivalTime: _arrivalTime,
            leavingTime: _leavingTime,
            arrivalSpeed: _arrivalSpeed,
            leaveSpeed: _leaveSpeed,
            willPass: _willPass,
            arrivalSpeedBraking: _arrivalSpeedBraking,
            waitingTime: _waitingTime,
            dist: _dist,
            speed: _speed,
            latOffset: _latOffset,
        }
    }

}

/// Function-object for stable sorting MSLinks with numerical lane ids
/// C++ class `MSLink::ComparatorNumericalLaneIdLess` (microsim/MSLink.h)
pub struct MSLink_ComparatorNumericalLaneIdLess {
}

impl MSLink_ComparatorNumericalLaneIdLess {
    /// C++: `bool ComparatorNumericalLaneIdLess::operator()(const MSLink *const, const MSLink *const) const` [const]
    // bool operator()(const MSLink* const a, const MSLink* const b) const {
    //             return (a->getLane()->getNumericalID() < b->getLane()->getNumericalID())
    //                    || (a->getLane()->getNumericalID() == b->getLane()->getNumericalID() && a->getIndex() < b->getIndex());
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: Ptr<MSLink>, b: Ptr<MSLink>) -> bool {
        let la = a.get_lane().get_numerical_id();
        let lb = b.get_lane().get_numerical_id();
        (la < lb) || (la == lb && a.get_index() < b.get_index())
    }

}

/// pre-computed information for conflict points
/// C++ class `MSLink::ConflictInfo` (microsim/MSLink.h)
pub struct MSLink_ConflictInfo {
    /// the conflict from the perspective of the foe
    pub foeConflictIndex: i32,
    /// length of internal lane after the crossing point
    pub lengthBehindCrossing: f64,
    /// the length of the conflict space
    pub conflictSize: f64,
    pub flag: MSLink_ConflictFlag,
}

impl MSLink_ConflictInfo {
    /// C++: `void ConflictInfo::ConflictInfo(double, double, ConflictFlag)`
    // ConflictInfo(double lbc, double cs, ConflictFlag fl = CONFLICT_DEFAULT) :
    //             foeConflictIndex(-1),
    //             lengthBehindCrossing(lbc),
    //             conflictSize(cs),
    //             flag(fl)
    //         {}
    #[allow(unused_variables)]
    pub fn new(lbc: f64, cs: f64, fl: MSLink_ConflictFlag) -> Self {
        MSLink_ConflictInfo { foeConflictIndex: -1, lengthBehindCrossing: lbc, conflictSize: cs, flag: fl }
    }

    /// C++: `double ConflictInfo::getFoeLengthBehindCrossing(const MSLink *) const` [const]
    // double
    // MSLink::ConflictInfo::getFoeLengthBehindCrossing(const MSLink* foeExitLink) const {
    //     if (flag == CONFLICT_DUMMY_MERGE) {
    //         return 0;
    //     } else if (foeConflictIndex >= 0) {
    //         return foeExitLink->myConflicts[foeConflictIndex].lengthBehindCrossing;
    //     } else {
    //         return -NO_INTERSECTION;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_foe_length_behind_crossing(&self, foeExitLink: Ptr<MSLink>) -> f64 {
        if self.flag == MSLink_ConflictFlag::CONFLICT_DUMMY_MERGE {
            0.0
        } else if self.foeConflictIndex >= 0 {
            foeExitLink.myConflicts[self.foeConflictIndex as usize].lengthBehindCrossing
        } else {
            -MSLink::NO_INTERSECTION
        }
    }

    /// C++: `double ConflictInfo::getFoeConflictSize(const MSLink *) const` [const]
    // double
    // MSLink::ConflictInfo::getFoeConflictSize(const MSLink* foeExitLink) const {
    //     if (foeConflictIndex >= 0) {
    //         return foeExitLink->myConflicts[foeConflictIndex].conflictSize;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_foe_conflict_size(&self, foeExitLink: Ptr<MSLink>) -> f64 {
        if self.foeConflictIndex >= 0 {
            foeExitLink.myConflicts[self.foeConflictIndex as usize].conflictSize
        } else {
            0.0
        }
    }

    /// C++: `double ConflictInfo::getLengthBehindCrossing(const MSLink *) const` [const]
    // double
    // MSLink::ConflictInfo::getLengthBehindCrossing(const MSLink* exitLink) const {
    //     if (flag == CONFLICT_STOP_AT_INTERNAL_JUNCTION) {
    //         return exitLink->getInternalLaneBefore()->getLength();
    //     } else {
    //         return lengthBehindCrossing;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_length_behind_crossing(&self, exitLink: Ptr<MSLink>) -> f64 {
        if self.flag == MSLink_ConflictFlag::CONFLICT_STOP_AT_INTERNAL_JUNCTION {
            exitLink.get_internal_lane_before().get_length()
        } else {
            self.lengthBehindCrossing
        }
    }

}

/// holds user defined conflict positions (must be interpreted for the correct exitLink)
/// C++ class `MSLink::CustomConflict` (microsim/MSLink.h)
pub struct MSLink_CustomConflict {
    pub from: Ptr<MSLane>,
    pub to: Ptr<MSLane>,
    pub startPos: f64,
    pub endPos: f64,
}

impl MSLink_CustomConflict {
    /// C++: `void CustomConflict::CustomConflict(const MSLane *, const MSLane *, double, double)`
    // CustomConflict(const MSLane* f, const MSLane* t, double s, double e) :
    //             from(f), to(t), startPos(s), endPos(e) {}
    #[allow(unused_variables)]
    pub fn new(f: Ptr<MSLane>, t: Ptr<MSLane>, s: f64, e: f64) -> Self {
        MSLink_CustomConflict { from: f, to: t, startPos: s, endPos: e }
    }

}

/// C++ class `MSLink::LinkLeader` (microsim/MSLink.h)
pub struct MSLink_LinkLeader {
    pub vehAndGap: (Ptr<MSVehicle>, f64),
    pub distToCrossing: f64,
    pub llFlags: i32,
    pub latOffset: f64,
}

impl MSLink_LinkLeader {
    /// C++: `void LinkLeader::LinkLeader(MSVehicle *, double, double, int, double)`
    // LinkLeader(MSVehicle* _veh, double _gap, double _distToCrossing, int _llFlags = LL_FROM_LEFT, double _latOffst = 0) :
    //             vehAndGap(std::make_pair(_veh, _gap)),
    //             distToCrossing(_distToCrossing),
    //             llFlags(_llFlags),
    //             latOffset(_latOffst)
    //         { }
    #[allow(unused_variables)]
    pub fn new(_veh: Ptr<MSVehicle>, _gap: f64, _distToCrossing: f64, _llFlags: i32, _latOffst: f64) -> Self {
        MSLink_LinkLeader { vehAndGap: (_veh, _gap), distToCrossing: _distToCrossing, llFlags: _llFlags, latOffset: _latOffst }
    }

    /// C++: `bool LinkLeader::fromLeft() const` [const]
    // inline bool fromLeft() const {
    //             return (llFlags & LL_FROM_LEFT) != 0;
    //         }
    #[allow(unused_variables)]
    pub fn from_left(&self) -> bool {
        (self.llFlags & MSLink_LinkLeaderFlag::LL_FROM_LEFT as i32) != 0
    }

    /// C++: `bool LinkLeader::inTheWay() const` [const]
    // inline bool inTheWay() const {
    //             return (llFlags & LL_IN_THE_WAY) != 0;
    //         }
    #[allow(unused_variables)]
    pub fn in_the_way(&self) -> bool {
        (self.llFlags & MSLink_LinkLeaderFlag::LL_IN_THE_WAY as i32) != 0
    }

    /// C++: `bool LinkLeader::sameTarget() const` [const]
    // inline bool sameTarget() const {
    //             return (llFlags & LL_SAME_TARGET) != 0;
    //         }
    #[allow(unused_variables)]
    pub fn same_target(&self) -> bool {
        (self.llFlags & MSLink_LinkLeaderFlag::LL_SAME_TARGET as i32) != 0
    }

    /// C++: `bool LinkLeader::sameSource() const` [const]
    // inline bool sameSource() const {
    //             return (llFlags & LL_SAME_SOURCE) != 0;
    //         }
    #[allow(unused_variables)]
    pub fn same_source(&self) -> bool {
        (self.llFlags & MSLink_LinkLeaderFlag::LL_SAME_SOURCE as i32) != 0
    }

}

/// A connection between lanes
/// C++ class `MSLink` (microsim/MSLink.h)
pub struct MSLink {
    /// The lane behind the junction approached by this link
    pub myLane: Ptr<MSLane>,
    /// The lane approaching this link
    pub myLaneBefore: Ptr<MSLane>,
    pub myApproachingVehicles: MSLink_ApproachInfos,
    pub myApproachingPersons: Ptr<MSLink_PersonApproachInfos>,
    /// The position within this respond
    pub myIndex: i32,
    /// the traffic light index
    pub myTLIndex: i32,
    /// the controlling logic or 0
    pub myLogic: Ptr<MSTrafficLightLogic>,
    /// The state of the link
    pub myState: LinkState,
    /// The last green state of the link (minor or major)
    pub myLastGreenState: LinkState,
    /// The state of the link when switching of traffic light control
    pub myOffState: LinkState,
    /// The time of the last state change
    pub myLastStateChange: SUMOTime,
    /// An abstract (hopefully human readable) definition of the link's direction
    pub myDirection: LinkDirection,
    /// The length of the link
    pub myLength: f64,
    /// distance from which an approaching vehicle is able to see all relevant foes and may accelerate if the link is minor and no foe is approaching. Defaults to 4.5m. For zipper links (major) this is the distance at which zipper merging starts (and foes become "visible")
    pub myFoeVisibilityDistance: f64,
    /// distance from the stop line to the first pedestrian crossing or maxdouble
    pub myDistToFoePedCrossing: f64,
    /// Whether any foe links exist
    pub myHasFoes: bool,
    pub myAmCont: bool,
    pub myAmContOff: bool,
    pub myKeepClear: bool,
    /// The following junction-internal lane if used
    pub myInternalLane: Ptr<MSLane>,
    pub myInternalLaneBefore: Ptr<MSLane>,
    /// penalty time at tls for mesoscopic simulation
    pub myMesoTLSPenalty: SUMOTime,
    /// green fraction at tls for mesoscopic simulation
    pub myGreenFraction: f64,
    /// lateral shift to be applied when passing this link
    pub myLateralShift: f64,
    pub myConflicts: Vec<MSLink_ConflictInfo>,
    pub myCustomConflicts: Vec<MSLink_CustomConflict>,
    pub myFoeLinks: Vec<Ptr<MSLink>>,
    pub myFoeLanes: Vec<Ptr<MSLane>>,
    pub myOffFoeLinks: Ptr<Vec<Ptr<MSLink>>>,
    /// walkingArea that must be checked when entering the intersection
    pub myWalkingAreaFoe: Ptr<MSLane>,
    /// walkingArea that must be checked when leaving the intersection
    pub myWalkingAreaFoeExit: Ptr<MSLane>,
    /// whether on of myFoeLanes is a crossing
    pub myHavePedestrianCrossingFoe: bool,
    pub mySublaneFoeLinks: Vec<Ptr<MSLink>>,
    pub mySublaneFoeLinks2: Vec<Ptr<MSLink>>,
    pub mySublaneFoeLanes: Vec<Ptr<MSLane>>,
    pub myParallelRight: Ptr<MSLink>,
    pub myParallelLeft: Ptr<MSLink>,
    /// whether this connection is an indirect turning movement
    pub myAmIndirect: bool,
    /// the turning radius for this link or doublemax for straight links
    pub myRadius: f64,
    /// who may drive on this link
    pub myPermissions: SVCPermissions,
    /// the junction to which this link belongs
    pub myJunction: Ptr<MSJunction>,
}

// C++ static members of MSLink (port as module-level statics, see AGENTS.md):
//   static const double MSLink::NO_INTERSECTION;
//   static const SUMOTime MSLink::myLookaheadTime;
//   static const SUMOTime MSLink::myLookaheadTimeZipper;
//   static std::set<std::pair<MSLink *, MSLink *>> MSLink::myRecheck;

impl MSLink {
    /// C++ `MSLink::NO_INTERSECTION`
    pub const NO_INTERSECTION: f64 = 10000.;
    /// C++ `MSLink::myLookaheadTime` = TIME2STEPS(1)
    pub const myLookaheadTime: SUMOTime = 1000;
    /// C++ `MSLink::myLookaheadTimeZipper` = TIME2STEPS(16)
    pub const myLookaheadTimeZipper: SUMOTime = 16000;
}

thread_local! {
    /// C++ `MSLink::myRecheck`
    pub static MY_RECHECK: RefCell<BTreeSet<(Ptr<MSLink>, Ptr<MSLink>)>> = RefCell::new(BTreeSet::new());
}

impl MSLink {
    /// C++: `void MSLink::MSLink(MSLane *, MSLane *, MSLane *, LinkDirection, LinkState, double, double, bool, MSTrafficLightLogic *, int, bool)`
    /// Constructor for simulation which uses internal lanes
    // MSLink::MSLink(MSLane* predLane, MSLane* succLane, MSLane* via, LinkDirection dir, LinkState state,
    //                double length, double foeVisibilityDistance, bool keepClear,
    //                MSTrafficLightLogic* logic, int tlIndex,
    //                bool indirect) :
    //     myLane(succLane),
    //     myLaneBefore(predLane),
    //     myApproachingPersons(nullptr),
    //     myIndex(-1),
    //     myTLIndex(tlIndex),
    //     myLogic(logic),
    //     myState(state),
    //     myLastGreenState(LINKSTATE_TL_GREEN_MINOR),
    //     myOffState(state),
    //     myLastStateChange(SUMOTime_MIN / 2), // a large negative value, but avoid overflows when subtracting
    //     myDirection(dir),
    //     myLength(length),
    //     myFoeVisibilityDistance(foeVisibilityDistance),
    //     myDistToFoePedCrossing(std::numeric_limits<double>::max()),
    //     myHasFoes(false),
    //     myAmCont(false),
    //     myAmContOff(false),
    //     myKeepClear(keepClear),
    //     myInternalLane(via),
    //     myInternalLaneBefore(nullptr),
    //     myMesoTLSPenalty(0),
    //     myGreenFraction(1),
    //     myLateralShift(0),
    //     myOffFoeLinks(nullptr),
    //     myWalkingAreaFoe(nullptr),
    //     myWalkingAreaFoeExit(nullptr),
    //     myHavePedestrianCrossingFoe(false),
    //     myParallelRight(nullptr),
    //     myParallelLeft(nullptr),
    //     myAmIndirect(indirect),
    //     myRadius(std::numeric_limits<double>::max()),
    //     myPermissions(0),
    //     myJunction(nullptr)
    // {
    //     updatePermissions();
    //     if (MSGlobals::gLateralResolution > 0) {
    //         // detect lateral shift from lane geometries
    //         //std::cout << "DEBUG link=" << myLaneBefore->getID() << "->" << getViaLaneOrLane()->getID() << " hasInternal=" << MSNet::getInstance()->hasInternalLinks() << " shapeBefore=" << myLaneBefore->getShape().back() << " shapeFront=" << getViaLaneOrLane()->getShape().front() << "\n";
    //         if ((myInternalLane != nullptr || predLane->isInternal())
    //                 && myLaneBefore->getShape().back() != getViaLaneOrLane()->getShape().front()) {
    //             PositionVector from = myLaneBefore->getShape();
    //             const PositionVector& to = getViaLaneOrLane()->getShape();
    //             const double dist = from.back().distanceTo2D(to.front());
    //             // figure out direction of shift
    //             try {
    //                 from.move2side(dist);
    //             } catch (InvalidArgument&) {
    //             }
    //             myLateralShift = (from.back().distanceTo2D(to.front()) < dist) ? dist : -dist;
    //             if (MSGlobals::gLefthand) {
    //                 myLateralShift *= -1;
    //             }
    //             //std::cout << " lateral shift link=" << myLaneBefore->getID() << "->" << getViaLaneOrLane()->getID() << " dist=" << dist << " shift=" << myLateralShift << "\n";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(predLane: Ptr<MSLane>, succLane: Ptr<MSLane>, via: Ptr<MSLane>, dir: LinkDirection, state: LinkState, length: f64, foeVisibilityDistance: f64, keepClear: bool, logic: Ptr<MSTrafficLightLogic>, tlLinkIdx: i32, indirect: bool) -> Self {
        let mut s = MSLink {
            myLane: succLane,
            myLaneBefore: predLane,
            myApproachingVehicles: BTreeMap::new(),
            myApproachingPersons: Ptr::null(),
            myIndex: -1,
            myTLIndex: tlLinkIdx,
            myLogic: logic,
            myState: state,
            myLastGreenState: LINKSTATE_TL_GREEN_MINOR,
            myOffState: state,
            myLastStateChange: SUMOTIME_MIN / 2,
            myDirection: dir,
            myLength: length,
            myFoeVisibilityDistance: foeVisibilityDistance,
            myDistToFoePedCrossing: f64::MAX,
            myHasFoes: false,
            myAmCont: false,
            myAmContOff: false,
            myKeepClear: keepClear,
            myInternalLane: via,
            myInternalLaneBefore: Ptr::null(),
            myMesoTLSPenalty: 0,
            myGreenFraction: 1.,
            myLateralShift: 0.,
            myConflicts: Vec::new(),
            myCustomConflicts: Vec::new(),
            myFoeLinks: Vec::new(),
            myFoeLanes: Vec::new(),
            myOffFoeLinks: Ptr::null(),
            myWalkingAreaFoe: Ptr::null(),
            myWalkingAreaFoeExit: Ptr::null(),
            myHavePedestrianCrossingFoe: false,
            mySublaneFoeLinks: Vec::new(),
            mySublaneFoeLinks2: Vec::new(),
            mySublaneFoeLanes: Vec::new(),
            myParallelRight: Ptr::null(),
            myParallelLeft: Ptr::null(),
            myAmIndirect: indirect,
            myRadius: f64::MAX,
            myPermissions: 0,
            myJunction: Ptr::null(),
        };
        s.update_permissions();
        if g_lateral_resolution() > 0. {
            // detect lateral shift from lane geometries
            if (!s.myInternalLane.is_null() || predLane.is_internal())
                && s.myLaneBefore.get_shape_2().back() != s.get_via_lane_or_lane().get_shape_2().front()
            {
                let mut from = s.myLaneBefore.get_shape_2().clone();
                let via = s.get_via_lane_or_lane();
                let to = via.get_shape_2();
                let dist = from.back().distance_to2d(to.front());
                // figure out direction of shift
                from.move2side(dist, 100.);
                s.myLateralShift = if from.back().distance_to2d(to.front()) < dist { dist } else { -dist };
                if g_lefthand() {
                    s.myLateralShift *= -1.;
                }
            }
        }
        s
    }

    /// C++: `void MSLink::~MSLink()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++: delete myOffFoeLinks; delete myApproachingPersons;
        if !self.myOffFoeLinks.is_null() {
            unsafe { drop(Box::from_raw(self.myOffFoeLinks.as_raw())) };
        }
        if !self.myApproachingPersons.is_null() {
            unsafe { drop(Box::from_raw(self.myApproachingPersons.as_raw())) };
        }
    }

    /// C++: `void MSLink::addCustomConflict(const MSLane *, const MSLane *, double, double)`
    // void
    // MSLink::addCustomConflict(const MSLane* from, const MSLane* to, double startPos, double endPos) {
    //     myCustomConflicts.push_back(CustomConflict(from, to, startPos, endPos));
    // }
    #[allow(unused_variables)]
    pub fn add_custom_conflict(&mut self, from: Ptr<MSLane>, to: Ptr<MSLane>, startPos: f64, endPos: f64) {
        self.myCustomConflicts.push(MSLink_CustomConflict::new(from, to, startPos, endPos));
    }

    /// C++: `void MSLink::setRequestInformation(int, bool, bool, const std::vector<MSLink *> &, const std::vector<MSLane *> &, MSLane *)`
    /// Sets the request information
    // void
    // MSLink::setRequestInformation(int index, bool hasFoes, bool isCont,
    //                               const std::vector<MSLink*>& foeLinks,
    //                               const std::vector<MSLane*>& foeLanes,
    //                               MSLane* internalLaneBefore) {
    // //#ifdef MSLink_DEBUG_CROSSING_POINTS
    // //    std::cout << " setRequestInformation() for junction " << getViaLaneOrLane()->getEdge().getFromJunction()->getID()
    // //            << "\nInternalLanes = " << toString(getViaLaneOrLane()->getEdge().getFromJunction()->getInternalLanes())
    // //            << std::endl;
    // //#endif
    //     myIndex = index;
    //     myHasFoes = hasFoes;
    //     myAmCont = isCont && MSGlobals::gUsingInternalLanes;
    //     myFoeLinks = foeLinks;
    //     for (MSLane* foeLane : foeLanes) {
    //         // cannot assign vector due to const-ness
    //         myFoeLanes.push_back(foeLane);
    //     }
    //     myJunction = const_cast<MSJunction*>(myLane->getEdge().getFromJunction()); // junctionGraph is initialized after the whole network is loaded
    //     myAmContOff = isCont && myLogic != nullptr && internalLaneBefore == nullptr && checkContOff();
    //     myInternalLaneBefore = internalLaneBefore;
    //     MSLane* lane = nullptr;
    //     if (internalLaneBefore != nullptr) {
    //         // this is an exit link. compute crossing points with all foeLanes
    //         lane = internalLaneBefore;
    //         //} else if (myLane->isCrossing()) {
    //         //    // this is the link to a pedestrian crossing. compute crossing points with all foeLanes
    //         //    // @note not currently used by pedestrians
    //         //    lane = myLane;
    //     }
    //     const MSLink* entryLink = getCorrespondingEntryLink();
    //     if (entryLink->getOffState() == LinkState::LINKSTATE_ALLWAY_STOP && entryLink->getTLLogic() != nullptr) {
    //         // TLS has "normal" right of way rules but all conflicting links are foes when switching TLS off
    //         // (unless it's an internal junction link which should ignore all foes and should be ignored by all foes
    //         myOffFoeLinks = new std::vector<MSLink*>();
    //         if (isEntryLink()) {
    //             for (MSLane* foeLane : foeLanes) {
    //                 assert(foeLane->isInternal() || foeLane->isCrossing());
    //                 MSLink* viaLink = foeLane->getIncomingLanes().front().viaLink;
    //                 if (viaLink->getLaneBefore()->isNormal()) {
    //                     myOffFoeLinks->push_back(viaLink);
    //                 }
    //             }
    //         }
    //     }
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //     std::cout << "link " << myIndex << " to " << getViaLaneOrLane()->getID() << " internalLaneBefore=" << (lane == 0 ? "NULL" : lane->getID()) << " has foes: " << toString(foeLanes) << "\n";
    // #endif
    //     if (lane != nullptr) {
    //         const bool beforeInternalJunction = lane->getLinkCont()[0]->getViaLaneOrLane()->getEdge().isInternal();
    //         if (lane->getIncomingLanes().size() != 1) {
    //             throw ProcessError(TLF("Internal lane '%' has % predecessors", lane->getID(), toString(lane->getIncomingLanes().size())));
    //         }
    //         const MSLink* junctionEntryLink = lane->getEntryLink();
    //         const bool isSecondPart = isExitLinkAfterInternalJunction();
    //         // compute crossing points
    //         for (const MSLane* foeLane : myFoeLanes) {
    //             const CustomConflict* cc = junctionEntryLink != nullptr ? junctionEntryLink->getCustomConflict(foeLane) : nullptr;
    //             if (cc != nullptr) {
    //                 // handle custom conflict definition
    //                 double startPos = cc->startPos;
    //                 const double conflictSize = cc->endPos - cc->startPos;
    //                 if (isSecondPart) {
    //                     startPos -= junctionEntryLink->getViaLane()->getLength();
    //                 }
    //                 // the foe connection may be split at an internal
    //                 // junction, we need to figure out whether the current
    //                 // foeLane is the intended target for the custom conflict
    //                 // There are two possibilities:
    //                 // a) We have no custom conflict for the reverse pair of connections
    //                 //    -> just check whether lane and foeLane intersect
    //                 // b) We have a "reverse" custom conflict
    //                 //    -> check whether it covers the foeLane
    //                 const CustomConflict* rcc = foeLane->getEntryLink()->getCustomConflict(lane);
    //                 bool haveIntersection = false;
    //                 if (rcc == nullptr) {
    //                     // a)
    //                     haveIntersection = lane->getShape().intersectsAtLengths2D(foeLane->getShape()).size() > 0;
    //                 } else {
    //                     // b)
    //                     const bool foeIsSecondPart = foeLane->getLogicalPredecessorLane()->isInternal();
    //                     double foeStartPos = rcc->startPos;
    //                     const double foeConflictSize = rcc->endPos - rcc->startPos;
    //                     if (foeIsSecondPart) {
    //                         foeStartPos -= foeLane->getLogicalPredecessorLane()->getLength();
    //                     }
    //                     const double foeEndPos = foeStartPos + foeConflictSize;
    //                     haveIntersection = ((foeStartPos > 0 && foeStartPos < foeLane->getLength())
    //                                         || (foeEndPos > 0 && foeEndPos < foeLane->getLength()));
    //                 }
    //                 if (haveIntersection) {
    //                     myConflicts.push_back(ConflictInfo(lane->getLength() - startPos, conflictSize));
    //                 } else {
    //                     myConflicts.push_back(ConflictInfo(-NO_INTERSECTION, 0));
    //                 }
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                 std::cout << " " << lane->getID() << " custom conflict with " << foeLane->getID() << " customReverse=" << (rcc != nullptr)
    //                           << " haveIntersection=" << haveIntersection
    //                           << " startPos=" << startPos << " conflictSize=" << conflictSize
    //                           << " lbc=" << myConflicts.back().lengthBehindCrossing
    //                           << "\n";
    // #endif
    //                 continue;
    //             }
    //             myHavePedestrianCrossingFoe = myHavePedestrianCrossingFoe || foeLane->isCrossing();
    //             const bool sameTarget = myLane == foeLane->getLinkCont()[0]->getLane();
    //             if (sameTarget && !beforeInternalJunction && !contIntersect(lane, foeLane)) {
    //                 //if (myLane == foeLane->getLinkCont()[0]->getLane()) {
    //                 // this foeLane has the same target and merges at the end (lane exits the junction)
    //                 const double minDist = MIN2(DIVERGENCE_MIN_WIDTH, 0.5 * (lane->getWidth() + foeLane->getWidth()));
    //                 if (lane->getShape().back().distanceTo2D(foeLane->getShape().back()) >= minDist) {
    //                     // account for lateral shift by the entry links
    //                     if (foeLane->getEntryLink()->isIndirect()) {
    //                         myConflicts.push_back(ConflictInfo(-NO_INTERSECTION, 0)); // dummy value, never used
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                         std::cout << " " << lane->getID() << " dummy merge with indirect" << foeLane->getID() << "\n";
    // #endif
    //                     } else {
    //                         myConflicts.push_back(ConflictInfo(0, foeLane->getWidth(), CONFLICT_DUMMY_MERGE)); // dummy value, never used
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                         std::cout << " " << lane->getID() << " dummy merge with " << foeLane->getID() << "\n";
    // #endif
    //                     }
    //                 } else {
    //                     const double distAfterDivergence = computeDistToDivergence(lane, foeLane, minDist, false);
    //                     const double lbcLane = lane->interpolateGeometryPosToLanePos(distAfterDivergence);
    //                     myConflicts.push_back(ConflictInfo(lbcLane, foeLane->getWidth()));
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                     std::cout
    //                             << " " << lane->getID()
    //                             << " merges with " << foeLane->getID()
    //                             << " nextLane " << lane->getLinkCont()[0]->getViaLaneOrLane()->getID()
    //                             << " dist1=" << myConflicts.back().lengthBehindCrossing
    //                             << "\n";
    // #endif
    //                 }
    //             } else {
    //                 std::vector<double> intersections1 = lane->getShape().intersectsAtLengths2D(foeLane->getShape());
    // #ifdef MSLink_DEBUG_CROSSING_POINTS_DETAILS
    //                 std::cout << "    intersections1=" << toString(intersections1) << "\n";
    // #endif
    //                 bool haveIntersection = true;
    //                 if (intersections1.size() == 0) {
    //                     intersections1.push_back(-NO_INTERSECTION); // disregard this foe (using maxdouble leads to nasty problems down the line)
    //                     haveIntersection = false;
    //                 } else if (intersections1.size() > 1) {
    //                     std::sort(intersections1.begin(), intersections1.end());
    //                 }
    //                 std::vector<double> intersections2 = foeLane->getShape().intersectsAtLengths2D(lane->getShape());
    // #ifdef MSLink_DEBUG_CROSSING_POINTS_DETAILS
    //                 std::cout << "    intersections2=" << toString(intersections2) << "\n";
    // #endif
    //                 if (intersections2.size() == 0) {
    //                     intersections2.push_back(0);
    //                 } else if (intersections2.size() > 1) {
    //                     std::sort(intersections2.begin(), intersections2.end());
    //                 }
    //
    //                 // check for near-intersection (internal junctions for a side road which are only relevant when they have stranded vehicles))
    //                 if (!haveIntersection && foeLane->getLinkCont()[0]->getViaLane() != nullptr) {
    //                     const Position waitPos = foeLane->getShape().back();
    //                     const double dist = lane->getShape().distance2D(waitPos, true);
    //                     if (dist != GeomHelper::INVALID_OFFSET && dist < lane->getWidth() / 2) {
    //                         // risk of collision
    //                         intersections1.clear();
    //                         intersections2.clear();
    //                         intersections1.push_back(lane->getShape().nearest_offset_to_point2D(waitPos));
    //                         intersections2.push_back(foeLane->getShape().length());
    //                         haveIntersection = true;
    // #ifdef MSLink_DEBUG_CROSSING_POINTS_DETAILS
    //                         std::cout << "    link=" << myIndex << " " << getDescription() << " almostIntersection with foeLane " << foeLane->getID() << " offset=" << intersections1.back() << "\n";
    // #endif
    //                     }
    //                 }
    //
    //                 double conflictSize = foeLane->getWidth();
    //                 ConflictFlag flag = CONFLICT_NO_INTERSECTION;
    //                 if (haveIntersection) {
    //                     flag = CONFLICT_DEFAULT;
    //                     const double angle1 = GeomHelper::naviDegree(lane->getShape().rotationAtOffset(intersections1.back()));
    //                     const double angle2 = GeomHelper::naviDegree(foeLane->getShape().rotationAtOffset(intersections2.back()));
    //                     const double angleDiff = GeomHelper::getMinAngleDiff(angle1, angle2);
    //                     //const double angleDiff = MIN2(GeomHelper::getMinAngleDiff(angle1, angle2),
    //                     //                              GeomHelper::getMinAngleDiff(angle1, angle2 + 180));
    //                     const double widthFactor = 1 / MAX2(sin(DEG2RAD(angleDiff)), 0.2) * 2 - 1;
    //                     //std::cout << "  intersection of " << lane->getID() << " with " << foeLane->getID() << " angle1=" << angle1 << " angle2=" << angle2 << " angleDiff=" << angleDiff << " widthFactor=" << widthFactor << "\n";
    //                     conflictSize *= widthFactor;
    //                     conflictSize = MIN2(conflictSize, lane->getLength());
    //                     // lane width affects the crossing point
    //                     intersections1.back() -= conflictSize / 2;
    //                     // ensure non-negative offset for weird geometries
    //                     intersections1.back() = MAX2(0.0, intersections1.back());
    //
    //                     // also length/geometry factor. (XXX: Why subtract width/2 *before* converting geometric position to lane pos? refs #3031)
    //                     intersections1.back() = lane->interpolateGeometryPosToLanePos(intersections1.back());
    //
    //                     if (internalLaneBefore->getLogicalPredecessorLane()->getEdge().isInternal() && !foeLane->isCrossing())  {
    //                         flag = CONFLICT_STOP_AT_INTERNAL_JUNCTION;
    //                     }
    //
    //                     if (foeLane->isCrossing()) {
    //                         const MSLink* before = myInternalLaneBefore->getCanonicalPredecessorLane()->getLinkTo(myInternalLaneBefore);
    //                         const_cast<MSLink*>(before)->updateDistToFoePedCrossing(intersections1.back());
    //                     };
    //                 }
    //
    //                 myConflicts.push_back(ConflictInfo(
    //                                           lane->getLength() - intersections1.back(),
    //                                           conflictSize, flag));
    //
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                 std::cout
    //                         << "  intersection of " << lane->getID()
    //                         << " totalLength=" << lane->getLength()
    //                         << " with " << foeLane->getID()
    //                         << " totalLength=" << foeLane->getLength()
    //                         << " dist1=" << myConflicts.back().lengthBehindCrossing
    //                         << " widthFactor=" << myConflicts.back().conflictSize / foeLane->getWidth()
    //                         << "\n";
    // #endif
    //             }
    //         }
    //         // check for overlap with internal lanes from the same source lane
    //         const MSLane* pred = lane->getLogicalPredecessorLane();
    //         // to avoid overlap with vehicles that came from pred (especially when pred has endOffset > 0)
    //         // we add all other internal lanes from pred as foeLanes
    //         for (const MSLink* const link : pred->getLinkCont()) {
    //             const MSLane* const sibling = link->getViaLane();
    //             if (sibling != lane && sibling != nullptr) {
    //                 const double minDist = MIN2(DIVERGENCE_MIN_WIDTH, 0.5 * (lane->getWidth() + sibling->getWidth()));
    //                 if (lane->getShape().front().distanceTo2D(sibling->getShape().front()) >= minDist) {
    //                     // account for lateral shift by the entry links
    //                     continue;
    //                 }
    //                 const double distToDivergence = computeDistToDivergence(lane, sibling, minDist, true);
    //                 double lbcLane;
    //                 if (lane->getLength() == sibling->getLength() && &lane->getEdge() == &sibling->getEdge()) {
    //                     // for parallel lanes, avoid inconsistency in distance estimation (#10988)
    //                     // between forward distance (getLeaderInfo)
    //                     // and backward distance used in lane-changing (getFollowersOnConsecutive)
    //                     lbcLane = lane->getLength() - distToDivergence;
    //                 } else {
    //                     lbcLane = MAX2(0.0, lane->getLength() - lane->interpolateGeometryPosToLanePos(distToDivergence));
    //                 }
    //                 ConflictInfo ci = ConflictInfo(lbcLane, sibling->getWidth());
    //                 auto it = std::find(myFoeLanes.begin(), myFoeLanes.end(), sibling);
    //                 if (it != myFoeLanes.end()) {
    //                     // avoid duplicate foeLane
    //                     const int replacedIndex = (int)(it - myFoeLanes.begin());
    //                     myConflicts[replacedIndex] = ci;
    //                 } else {
    //                     myConflicts.push_back(ci);
    //                     myFoeLanes.push_back(sibling);
    //                 }
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                 std::cout << " adding same-origin foe" << sibling->getID()
    //                           << " dist1=" << myConflicts.back().lengthBehindCrossing
    //                           << "\n";
    // #endif
    //                 const MSLane* const siblingCont = sibling->getLinkCont().front()->getViaLaneOrLane();
    //                 if (siblingCont->isInternal() && lane->getShape().distance2D(siblingCont->getShape().front()) < minDist) {
    //                     // there may still be overlap with siblingCont (when considering vehicle widths)
    //                     const double maxCommonLength = MIN2(lane->getLength(), sibling->getLength() + siblingCont->getLength());
    //                     const double lengthBehindDivergence = MAX2(0.0, lane->getLength() - maxCommonLength);
    //                     ConflictInfo ci2 = ConflictInfo(lengthBehindDivergence, siblingCont->getWidth(), CONFLICT_SIBLING_CONTINUATION);
    //                     myConflicts.push_back(ci2);
    //                     myFoeLanes.push_back(siblingCont);
    //                     myRecheck.insert({this, siblingCont->getLinkCont().front()});
    //
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                     std::cout << " adding same-origin foeContinuation" << siblingCont->getID()
    //                               << " dist1=" << myConflicts.back().lengthBehindCrossing
    //                               << "\n";
    // #endif
    //                 }
    //             }
    //         }
    //         // init points for the symmetrical conflict
    //         // for each pair of conflicting lanes, the link that gets second, sets the pointers
    //         for (int i = 0; i < (int)myFoeLanes.size(); i++) {
    //             const MSLane* foeLane = myFoeLanes[i];
    //             MSLink* foeExitLink = foeLane->getLinkCont()[0];
    //             int foundIndex = -1;
    //             for (int i2 = 0; i2 < (int)foeExitLink->myFoeLanes.size(); i2++) {
    //                 if (foeExitLink->myFoeLanes[i2] == lane) {
    //                     myConflicts[i].foeConflictIndex = i2;
    //                     foeExitLink->myConflicts[i2].foeConflictIndex = i;
    //                     myRecheck.erase({foeExitLink, this});
    //                     foundIndex = i2;
    //                     break;
    //                 }
    //             }
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //             std::cout << lane->getID() << " foeLane=" << foeLane->getID() << " index=" << i << " foundIndex=" << foundIndex << "\n";
    // #endif
    //             if (foundIndex < 0) {
    //                 if (myConflicts[i].flag != CONFLICT_NO_INTERSECTION) {
    //                     myRecheck.insert({this, foeExitLink});
    //                 }
    //             }
    //         }
    //     }
    //     if (MSGlobals::gLateralResolution > 0) {
    //         // check for links with the same origin lane and the same destination edge
    //         const MSEdge* myTarget = &myLane->getEdge();
    //         // save foes for entry links
    //         for (MSLink* const it : myLaneBefore->getLinkCont()) {
    //             const MSEdge* target = &(it->getLane()->getEdge());
    //             if (it == this) {
    //                 continue;
    //             }
    //             if (target == myTarget) {
    //                 mySublaneFoeLinks.push_back(it);
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                 std::cout << "  sublaneFoeLink (same target): " << it->getViaLaneOrLane()->getID() << "\n";
    // #endif
    //             } else if (myDirection != LinkDirection::STRAIGHT && it->getDirection() == LinkDirection::STRAIGHT) {
    //                 // potential turn conflict
    //                 mySublaneFoeLinks2.push_back(it);
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //                 std::cout << "  sublaneFoeLink2 (other target: " << it->getViaLaneOrLane()->getID() << "\n";
    // #endif
    //             }
    //         }
    //         // save foes for exit links
    //         if (fromInternalLane()) {
    //             //std::cout << " setRequestInformation link=" << getViaLaneOrLane()->getID() << " before=" << myLaneBefore->getID() << " before2=" << myLaneBefore->getIncomingLanes().front().lane->getID() << "\n";
    //             for (const MSLink* const link : myLaneBefore->getIncomingLanes().front().lane->getLinkCont()) {
    //                 if (link->getViaLane() != myInternalLaneBefore && &link->getLane()->getEdge() == myTarget) {
    //                     //std::cout << " add sublaneFoe=" << (*it)->getViaLane()->getID() << "\n";
    //                     mySublaneFoeLanes.push_back(link->getViaLane());
    //                 }
    //             }
    //         }
    //     }
    //     if (myInternalLaneBefore != nullptr
    //             && myDirection != LinkDirection::STRAIGHT
    //             // for right turns, the curvature helps rather than restricts the linkLeader check
    //             && (
    //                 (!MSGlobals::gLefthand && myDirection != LinkDirection::RIGHT)
    //                 || (MSGlobals::gLefthand && myDirection != LinkDirection::LEFT))) {
    //         const double angle = fabs(GeomHelper::angleDiff(
    //                                       myLaneBefore->getNormalPredecessorLane()->getShape().angleAt2D(-2),
    //                                       myLane->getShape().angleAt2D(0)));
    //         if (angle > 0) {
    //             double length = myInternalLaneBefore->getShape().length2D();
    //             if (myInternalLaneBefore->getIncomingLanes().size() == 1 &&
    //                     myInternalLaneBefore->getIncomingLanes()[0].lane->isInternal()) {
    //                 length += myInternalLaneBefore->getIncomingLanes()[0].lane->getShape().length2D();
    //             } else if (myInternalLane != nullptr) {
    //                 length += myInternalLane->getShape().length2D();
    //             }
    //             myRadius = length / angle;
    //             //std::cout << getDescription() << " a=" << RAD2DEG(angle) << " l=" << length << " r=" << myRadius << "\n";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_request_information(&mut self, index: i32, hasFoes: bool, isCont: bool, foeLinks: &Vec<Ptr<MSLink>>, foeLanes: &Vec<Ptr<MSLane>>, internalLaneBefore: Ptr<MSLane>) {
        let this = Ptr::from_mut(self);
        self.myIndex = index;
        self.myHasFoes = hasFoes;
        self.myAmCont = isCont && g_using_internal_lanes();
        self.myFoeLinks = foeLinks.clone();
        for &foeLane in foeLanes.iter() {
            // cannot assign vector due to const-ness
            self.myFoeLanes.push(foeLane);
        }
        // junctionGraph is initialized after the whole network is loaded
        self.myJunction = self.myLane.get_edge().get_from_junction();
        self.myAmContOff = isCont && !self.myLogic.is_null() && internalLaneBefore.is_null() && self.check_cont_off();
        self.myInternalLaneBefore = internalLaneBefore;
        let mut lane: Ptr<MSLane> = Ptr::null();
        if !internalLaneBefore.is_null() {
            // this is an exit link. compute crossing points with all foeLanes
            lane = internalLaneBefore;
        }
        let entryLink = self.get_corresponding_entry_link();
        if entryLink.get_off_state() == LINKSTATE_ALLWAY_STOP && !entryLink.get_tl_logic().is_null() {
            // TLS has "normal" right of way rules but all conflicting links are foes when switching TLS off
            // (unless it's an internal junction link which should ignore all foes and should be ignored by all foes
            self.myOffFoeLinks = Ptr::from_mut(Box::leak(Box::new(Vec::new())));
            if self.is_entry_link() {
                for &foeLane in foeLanes.iter() {
                    debug_assert!(foeLane.is_internal() || foeLane.is_crossing());
                    let viaLink = foeLane.get_incoming_lanes()[0].viaLink;
                    if viaLink.get_lane_before().is_normal() {
                        self.myOffFoeLinks.m().push(viaLink);
                    }
                }
            }
        }
        if !lane.is_null() {
            let beforeInternalJunction = lane.get_link_cont()[0].get_via_lane_or_lane().get_edge().is_internal();
            if lane.get_incoming_lanes().len() != 1 {
                panic!("{}", ProcessError::new(tlf!("Internal lane '%' has % predecessors", lane.get_id().to_string(), lane.get_incoming_lanes().len() as i32)));
            }
            let junctionEntryLink = lane.get_entry_link();
            let isSecondPart = self.is_exit_link_after_internal_junction();
            // compute crossing points
            for i in 0..self.myFoeLanes.len() {
                let foeLane = self.myFoeLanes[i];
                let cc = if !junctionEntryLink.is_null() { junctionEntryLink.get_custom_conflict(foeLane) } else { Ptr::null() };
                if !cc.is_null() {
                    // handle custom conflict definition
                    let mut startPos = cc.startPos;
                    let conflictSize = cc.endPos - cc.startPos;
                    if isSecondPart {
                        startPos -= junctionEntryLink.get_via_lane().get_length();
                    }
                    // the foe connection may be split at an internal
                    // junction, we need to figure out whether the current
                    // foeLane is the intended target for the custom conflict
                    // There are two possibilities:
                    // a) We have no custom conflict for the reverse pair of connections
                    //    -> just check whether lane and foeLane intersect
                    // b) We have a "reverse" custom conflict
                    //    -> check whether it covers the foeLane
                    let rcc = foeLane.get_entry_link().get_custom_conflict(lane);
                    let mut haveIntersection = false;
                    if rcc.is_null() {
                        // a)
                        haveIntersection = lane.get_shape_2().intersects_at_lengths2d_pv(foeLane.get_shape_2()).len() > 0;
                    } else {
                        // b)
                        let foeIsSecondPart = foeLane.get_logical_predecessor_lane().is_internal();
                        let mut foeStartPos = rcc.startPos;
                        let foeConflictSize = rcc.endPos - rcc.startPos;
                        if foeIsSecondPart {
                            foeStartPos -= foeLane.get_logical_predecessor_lane().get_length();
                        }
                        let foeEndPos = foeStartPos + foeConflictSize;
                        haveIntersection = (foeStartPos > 0. && foeStartPos < foeLane.get_length())
                            || (foeEndPos > 0. && foeEndPos < foeLane.get_length());
                    }
                    if haveIntersection {
                        self.myConflicts.push(MSLink_ConflictInfo::new(lane.get_length() - startPos, conflictSize, MSLink_ConflictFlag::CONFLICT_DEFAULT));
                    } else {
                        self.myConflicts.push(MSLink_ConflictInfo::new(-MSLink::NO_INTERSECTION, 0., MSLink_ConflictFlag::CONFLICT_DEFAULT));
                    }
                    continue;
                }
                self.myHavePedestrianCrossingFoe = self.myHavePedestrianCrossingFoe || foeLane.is_crossing();
                let sameTarget = self.myLane == foeLane.get_link_cont()[0].get_lane();
                if sameTarget && !beforeInternalJunction && !self.cont_intersect(lane, foeLane) {
                    // this foeLane has the same target and merges at the end (lane exits the junction)
                    let minDist = min2(DIVERGENCE_MIN_WIDTH, 0.5 * (lane.get_width() + foeLane.get_width()));
                    if lane.get_shape_2().back().distance_to2d(foeLane.get_shape_2().back()) >= minDist {
                        // account for lateral shift by the entry links
                        if foeLane.get_entry_link().is_indirect() {
                            self.myConflicts.push(MSLink_ConflictInfo::new(-MSLink::NO_INTERSECTION, 0., MSLink_ConflictFlag::CONFLICT_DEFAULT)); // dummy value, never used
                        } else {
                            self.myConflicts.push(MSLink_ConflictInfo::new(0., foeLane.get_width(), MSLink_ConflictFlag::CONFLICT_DUMMY_MERGE)); // dummy value, never used
                        }
                    } else {
                        let distAfterDivergence = self.compute_dist_to_divergence(lane, foeLane, minDist, false, 0.);
                        let lbcLane = lane.interpolate_geometry_pos_to_lane_pos(distAfterDivergence);
                        self.myConflicts.push(MSLink_ConflictInfo::new(lbcLane, foeLane.get_width(), MSLink_ConflictFlag::CONFLICT_DEFAULT));
                    }
                } else {
                    let mut intersections1 = lane.get_shape_2().intersects_at_lengths2d_pv(foeLane.get_shape_2());
                    let mut haveIntersection = true;
                    if intersections1.is_empty() {
                        intersections1.push(-MSLink::NO_INTERSECTION); // disregard this foe (using maxdouble leads to nasty problems down the line)
                        haveIntersection = false;
                    } else if intersections1.len() > 1 {
                        intersections1.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    }
                    let mut intersections2 = foeLane.get_shape_2().intersects_at_lengths2d_pv(lane.get_shape_2());
                    if intersections2.is_empty() {
                        intersections2.push(0.);
                    } else if intersections2.len() > 1 {
                        intersections2.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    }

                    // check for near-intersection (internal junctions for a side road which are only relevant when they have stranded vehicles))
                    if !haveIntersection && !foeLane.get_link_cont()[0].get_via_lane().is_null() {
                        let waitPos = foeLane.get_shape_2().back().clone();
                        let dist = lane.get_shape_2().distance2d(&waitPos, true);
                        if dist != INVALID_OFFSET && dist < lane.get_width() / 2. {
                            // risk of collision
                            intersections1.clear();
                            intersections2.clear();
                            intersections1.push(lane.get_shape_2().nearest_offset_to_point2d(&waitPos, true));
                            intersections2.push(foeLane.get_shape_2().length());
                            haveIntersection = true;
                        }
                    }

                    let last1 = intersections1.len() - 1;
                    let last2 = intersections2.len() - 1;
                    let mut conflictSize = foeLane.get_width();
                    let mut flag = MSLink_ConflictFlag::CONFLICT_NO_INTERSECTION;
                    if haveIntersection {
                        flag = MSLink_ConflictFlag::CONFLICT_DEFAULT;
                        let angle1 = navi_degree(lane.get_shape_2().rotation_at_offset(intersections1[last1]));
                        let angle2 = navi_degree(foeLane.get_shape_2().rotation_at_offset(intersections2[last2]));
                        let angleDiff = get_min_angle_diff(angle1, angle2);
                        let widthFactor = 1. / max2(deg2rad(angleDiff).sin(), 0.2) * 2. - 1.;
                        conflictSize *= widthFactor;
                        conflictSize = min2(conflictSize, lane.get_length());
                        // lane width affects the crossing point
                        intersections1[last1] -= conflictSize / 2.;
                        // ensure non-negative offset for weird geometries
                        intersections1[last1] = max2(0.0, intersections1[last1]);

                        // also length/geometry factor. (XXX: Why subtract width/2 *before* converting geometric position to lane pos? refs #3031)
                        intersections1[last1] = lane.interpolate_geometry_pos_to_lane_pos(intersections1[last1]);

                        if internalLaneBefore.get_logical_predecessor_lane().get_edge().is_internal() && !foeLane.is_crossing() {
                            flag = MSLink_ConflictFlag::CONFLICT_STOP_AT_INTERNAL_JUNCTION;
                        }

                        if foeLane.is_crossing() {
                            let before = self.myInternalLaneBefore.get_canonical_predecessor_lane().get_link_to(self.myInternalLaneBefore);
                            before.m().update_dist_to_foe_ped_crossing(intersections1[last1]);
                        }
                    }

                    self.myConflicts.push(MSLink_ConflictInfo::new(lane.get_length() - intersections1[last1], conflictSize, flag));
                }
            }
            // check for overlap with internal lanes from the same source lane
            let pred = lane.get_logical_predecessor_lane();
            // to avoid overlap with vehicles that came from pred (especially when pred has endOffset > 0)
            // we add all other internal lanes from pred as foeLanes
            for &link in pred.get_link_cont().iter() {
                let sibling = link.get_via_lane();
                if sibling != lane && !sibling.is_null() {
                    let minDist = min2(DIVERGENCE_MIN_WIDTH, 0.5 * (lane.get_width() + sibling.get_width()));
                    if lane.get_shape_2().front().distance_to2d(sibling.get_shape_2().front()) >= minDist {
                        // account for lateral shift by the entry links
                        continue;
                    }
                    let distToDivergence = self.compute_dist_to_divergence(lane, sibling, minDist, true, 0.);
                    let lbcLane;
                    if lane.get_length() == sibling.get_length() && std::ptr::eq(lane.get_edge(), sibling.get_edge()) {
                        // for parallel lanes, avoid inconsistency in distance estimation (#10988)
                        // between forward distance (getLeaderInfo)
                        // and backward distance used in lane-changing (getFollowersOnConsecutive)
                        lbcLane = lane.get_length() - distToDivergence;
                    } else {
                        lbcLane = max2(0.0, lane.get_length() - lane.interpolate_geometry_pos_to_lane_pos(distToDivergence));
                    }
                    let ci = MSLink_ConflictInfo::new(lbcLane, sibling.get_width(), MSLink_ConflictFlag::CONFLICT_DEFAULT);
                    match self.myFoeLanes.iter().position(|&x| x == sibling) {
                        Some(replacedIndex) => {
                            // avoid duplicate foeLane
                            self.myConflicts[replacedIndex] = ci;
                        }
                        None => {
                            self.myConflicts.push(ci);
                            self.myFoeLanes.push(sibling);
                        }
                    }
                    let siblingCont = sibling.get_link_cont()[0].get_via_lane_or_lane();
                    if siblingCont.is_internal() && lane.get_shape_2().distance2d(siblingCont.get_shape_2().front(), false) < minDist {
                        // there may still be overlap with siblingCont (when considering vehicle widths)
                        let maxCommonLength = min2(lane.get_length(), sibling.get_length() + siblingCont.get_length());
                        let lengthBehindDivergence = max2(0.0, lane.get_length() - maxCommonLength);
                        let ci2 = MSLink_ConflictInfo::new(lengthBehindDivergence, siblingCont.get_width(), MSLink_ConflictFlag::CONFLICT_SIBLING_CONTINUATION);
                        self.myConflicts.push(ci2);
                        self.myFoeLanes.push(siblingCont);
                        recheck_insert(this, siblingCont.get_link_cont()[0]);
                    }
                }
            }
            // init points for the symmetrical conflict
            // for each pair of conflicting lanes, the link that gets second, sets the pointers
            for i in 0..self.myFoeLanes.len() {
                let foeLane = self.myFoeLanes[i];
                let foeExitLink = foeLane.get_link_cont()[0];
                let mut foundIndex: i32 = -1;
                for i2 in 0..foeExitLink.myFoeLanes.len() {
                    if foeExitLink.myFoeLanes[i2] == lane {
                        self.myConflicts[i].foeConflictIndex = i2 as i32;
                        foeExitLink.m().myConflicts[i2].foeConflictIndex = i as i32;
                        recheck_erase(foeExitLink, this);
                        foundIndex = i2 as i32;
                        break;
                    }
                }
                if foundIndex < 0 {
                    if self.myConflicts[i].flag != MSLink_ConflictFlag::CONFLICT_NO_INTERSECTION {
                        recheck_insert(this, foeExitLink);
                    }
                }
            }
        }
        if g_lateral_resolution() > 0. {
            // check for links with the same origin lane and the same destination edge
            let myTarget = self.myLane.get_edge() as *const MSEdge;
            // save foes for entry links
            let laneBefore = self.myLaneBefore;
            for &it in laneBefore.get_link_cont().iter() {
                let target = it.get_lane().get_edge() as *const MSEdge;
                if it == this {
                    continue;
                }
                if std::ptr::eq(target, myTarget) {
                    self.mySublaneFoeLinks.push(it);
                } else if self.myDirection != LinkDirection::STRAIGHT && it.get_direction() == LinkDirection::STRAIGHT {
                    // potential turn conflict
                    self.mySublaneFoeLinks2.push(it);
                }
            }
            // save foes for exit links
            if self.from_internal_lane() {
                for &link in laneBefore.get_incoming_lanes()[0].lane.get_link_cont().iter() {
                    if link.get_via_lane() != self.myInternalLaneBefore && std::ptr::eq(link.get_lane().get_edge() as *const MSEdge, myTarget) {
                        self.mySublaneFoeLanes.push(link.get_via_lane());
                    }
                }
            }
        }
        if !self.myInternalLaneBefore.is_null()
            && self.myDirection != LinkDirection::STRAIGHT
            // for right turns, the curvature helps rather than restricts the linkLeader check
            && ((!g_lefthand() && self.myDirection != LinkDirection::RIGHT)
                || (g_lefthand() && self.myDirection != LinkDirection::LEFT))
        {
            let angle = angle_diff(
                self.myLaneBefore.get_normal_predecessor_lane().get_shape_2().angle_at2d(-2),
                self.myLane.get_shape_2().angle_at2d(0),
            )
            .abs();
            if angle > 0. {
                let mut length = self.myInternalLaneBefore.get_shape_2().length2d();
                if self.myInternalLaneBefore.get_incoming_lanes().len() == 1 && self.myInternalLaneBefore.get_incoming_lanes()[0].lane.is_internal() {
                    length += self.myInternalLaneBefore.get_incoming_lanes()[0].lane.get_shape_2().length2d();
                } else if !self.myInternalLane.is_null() {
                    length += self.myInternalLane.get_shape_2().length2d();
                }
                self.myRadius = length / angle;
            }
        }
    }

    /// C++: `void MSLink::addWalkingAreaFoe(const MSLane *)`
    /// add walkingarea as foe (when entering the junction)
    // void addWalkingAreaFoe(const MSLane* lane) {
    //         myWalkingAreaFoe = lane;
    //     }
    #[allow(unused_variables)]
    pub fn add_walking_area_foe(&mut self, lane: Ptr<MSLane>) {
        self.myWalkingAreaFoe = lane;
    }

    /// C++: `void MSLink::addWalkingAreaFoeExit(const MSLane *)`
    /// add walkingarea as foe (when leaving the junction)
    // void addWalkingAreaFoeExit(const MSLane* lane) {
    //         myWalkingAreaFoeExit = lane;
    //     }
    #[allow(unused_variables)]
    pub fn add_walking_area_foe_exit(&mut self, lane: Ptr<MSLane>) {
        self.myWalkingAreaFoeExit = lane;
    }

    /// C++: `const MSLane * MSLink::getWalkingAreaFoe()`
    /// get walkingarea as foes
    // const MSLane* getWalkingAreaFoe() {
    //         return myWalkingAreaFoe;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_area_foe(&mut self) -> Ptr<MSLane> {
        self.myWalkingAreaFoe
    }

    /// C++: `const MSLane * MSLink::getWalkingAreaFoeExit()`
    // const MSLane* getWalkingAreaFoeExit() {
    //         return myWalkingAreaFoeExit;
    //     }
    #[allow(unused_variables)]
    pub fn get_walking_area_foe_exit(&mut self) -> Ptr<MSLane> {
        self.myWalkingAreaFoeExit
    }

    /// C++: `void MSLink::setApproaching(const SUMOVehicle *, const SUMOTime, const double, const double, const bool, const double, const SUMOTime, double, double)`
    /// Sets the information about an approaching vehicle
    // void
    // MSLink::setApproaching(const SUMOVehicle* approaching, const SUMOTime arrivalTime, const double arrivalSpeed, const double leaveSpeed,
    //                        const bool setRequest, const double arrivalSpeedBraking, const SUMOTime waitingTime, double dist, double latOffset) {
    //     const SUMOTime leaveTime = getLeaveTime(arrivalTime, arrivalSpeed, leaveSpeed, approaching->getVehicleType().getLength());
    // #ifdef DEBUG_APPROACHING
    //     if (DEBUG_COND2(approaching)) {
    //         std::cout << SIMTIME << " link=" << getDescription() << " setApproaching veh=" << approaching->getID();
    //         if (myApproachingVehicles.size() > 0) {
    //             std::cout << " curApproaching=";
    //             for (auto i = myApproachingVehicles.begin(); i != myApproachingVehicles.end(); ++i) {
    //                 std::cout << i->first->getID() << " ";
    //             }
    //         }
    //         std::cout << "\n";
    //     }
    // #endif
    //     if (MSGlobals::gUseMesoSim) {
    //         // - in meso, setApproaching may be called multiple times without intermediate removeApproaching (whenever a vehicle is blocked in MELoop::checkCar)
    //         // explicit erasure is necessary because emplace does nothing if the key already exists
    //         // - in micro, double registration only happens on looped routes. Here, we only wish to keep the first arrival and thus emplace has the correct behavior
    //         //   (on meso, only the next upcoming link is registered so nothing gets overwritten on looped routes)
    //         myApproachingVehicles.erase(approaching);
    //     }
    //     myApproachingVehicles.emplace(approaching,
    //                                   ApproachingVehicleInformation(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, setRequest,
    //                                           arrivalSpeedBraking, waitingTime, dist, approaching->getSpeed(), latOffset));
    // }
    #[allow(unused_variables)]
    pub fn set_approaching(&mut self, approaching: Ptr<SUMOVehicle>, arrivalTime: SUMOTime, arrivalSpeed: f64, leaveSpeed: f64, setRequest: bool, arrivalSpeedBraking: f64, waitingTime: SUMOTime, dist: f64, latOffset: f64) {
        let leaveTime = self.get_leave_time(arrivalTime, arrivalSpeed, leaveSpeed, approaching.get_vehicle_type().get_length());
        if g_use_meso_sim() {
            // - in meso, setApproaching may be called multiple times without intermediate removeApproaching (whenever a vehicle is blocked in MELoop::checkCar)
            // explicit erasure is necessary because emplace does nothing if the key already exists
            // - in micro, double registration only happens on looped routes. Here, we only wish to keep the first arrival and thus emplace has the correct behavior
            //   (on meso, only the next upcoming link is registered so nothing gets overwritten on looped routes)
            self.myApproachingVehicles.remove(&approaching);
        }
        let info = MSLink_ApproachingVehicleInformation::new(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, setRequest, arrivalSpeedBraking, waitingTime, dist, approaching.get_speed(), latOffset);
        // emplace does not overwrite an existing entry
        self.myApproachingVehicles.entry(approaching).or_insert(info);
    }

    /// C++: `void MSLink::setApproaching(const SUMOVehicle *, ApproachingVehicleInformation)`
    /// Sets the information about an approaching vehicle
    // void
    // MSLink::setApproaching(const SUMOVehicle* approaching, ApproachingVehicleInformation ai) {
    // #ifdef DEBUG_APPROACHING
    //     if (DEBUG_COND2(approaching)) {
    //         std::cout << SIMTIME << " link=" << getDescription() << " setApproaching veh=" << approaching->getID();
    //         if (myApproachingVehicles.size() > 0) {
    //             std::cout << " curApproaching=";
    //             for (auto i = myApproachingVehicles.begin(); i != myApproachingVehicles.end(); ++i) {
    //                 std::cout << i->first->getID() << " ";
    //             }
    //         }
    //         std::cout << "\n";
    //     }
    // #endif
    //     myApproachingVehicles.emplace(approaching, ai);
    // }
    #[allow(unused_variables)]
    pub fn set_approaching_2(&mut self, approaching: Ptr<SUMOVehicle>, ai: MSLink_ApproachingVehicleInformation) {
        // emplace does not overwrite an existing entry
        self.myApproachingVehicles.entry(approaching).or_insert(ai);
    }

    /// C++: `void MSLink::setApproachingPerson(const MSPerson *, const SUMOTime, const SUMOTime)`
    /// Sets the information about an approaching person (only for a pedestrian crossing)
    // void
    // MSLink::setApproachingPerson(const MSPerson* approaching, const SUMOTime arrivalTime, const SUMOTime leaveTime) {
    //     if (myApproachingPersons == nullptr) {
    //         myApproachingPersons = new PersonApproachInfos();
    //     }
    //     myApproachingPersons->emplace(approaching, ApproachingPersonInformation(arrivalTime, leaveTime));
    // }
    #[allow(unused_variables)]
    pub fn set_approaching_person(&mut self, approaching: Ptr<MSPerson>, arrivalTime: SUMOTime, leaveTime: SUMOTime) {
        if self.myApproachingPersons.is_null() {
            self.myApproachingPersons = Ptr::from_mut(Box::leak(Box::new(BTreeMap::new())));
        }
        // emplace does not overwrite an existing entry
        self.myApproachingPersons.m().entry(approaching).or_insert(MSLink_ApproachingPersonInformation::new(arrivalTime, leaveTime));
    }

    /// C++: `void MSLink::removeApproaching(const SUMOVehicle *)`
    /// removes the vehicle from myApproachingVehicles
    // void
    // MSLink::removeApproaching(const SUMOVehicle* veh) {
    // #ifdef DEBUG_APPROACHING
    //     if (DEBUG_COND2(veh)) {
    //         std::cout << SIMTIME << " link=" << getDescription() << " removeApproaching veh=" << veh->getID();
    //         if (myApproachingVehicles.size() > 0) {
    //             std::cout << " curApproaching=";
    //             for (auto i = myApproachingVehicles.begin(); i != myApproachingVehicles.end(); ++i) {
    //                 std::cout << i->first->getID() << " ";
    //             }
    //         }
    //         std::cout << "\n";
    //     }
    // #endif
    //     myApproachingVehicles.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn remove_approaching(&mut self, veh: Ptr<SUMOVehicle>) {
        self.myApproachingVehicles.remove(&veh);
    }

    /// C++: `void MSLink::removeApproachingPerson(const MSPerson *)`
    /// removes the person from myApproachingPersons
    // void
    // MSLink::removeApproachingPerson(const MSPerson* person) {
    //     if (myApproachingPersons == nullptr) {
    //         WRITE_WARNINGF("Person '%' entered crossing lane '%' without registering approach, time=%", person->getID(), myLane->getID(), time2string(SIMSTEP));
    //         return;
    //     }
    // #ifdef DEBUG_APPROACHING
    //     if (DEBUG_COND2(person)) {
    //         std::cout << SIMTIME << " Link '" << (myLaneBefore == 0 ? "NULL" : myLaneBefore->getID()) << "'->'" << (myLane == 0 ? "NULL" : myLane->getID()) << std::endl;
    //         std::cout << "' Removing approaching person '" << person->getID() << "'\nCurrently registered persons:" << std::endl;
    //         for (auto i = myApproachingPersons->begin(); i != myApproachingPersons->end(); ++i) {
    //             std::cout << "'" << i->first->getID() << "'" << std::endl;
    //         }
    //     }
    // #endif
    //     myApproachingPersons->erase(person);
    // }
    #[allow(unused_variables)]
    pub fn remove_approaching_person(&mut self, person: Ptr<MSPerson>) {
        if self.myApproachingPersons.is_null() {
            utils::write_warningf!("Person '%' entered crossing lane '%' without registering approach, time=%", person.get_id().to_string(), self.myLane.get_id().to_string(), time2string(MSNet::get_instance().get_current_time_step()));
            return;
        }
        self.myApproachingPersons.m().remove(&person);
    }

    /// C++: `ApproachingVehicleInformation MSLink::getApproaching(const SUMOVehicle *) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_approaching(&self, veh: Ptr<SUMOVehicle>) -> MSLink_ApproachingVehicleInformation {
        match self.myApproachingVehicles.get(&veh) {
            Some(i) => MSLink_ApproachingVehicleInformation::new(i.arrivalTime, i.leavingTime, i.arrivalSpeed, i.leaveSpeed, i.willPass, i.arrivalSpeedBraking, i.waitingTime, i.dist, i.speed, i.latOffset),
            None => MSLink_ApproachingVehicleInformation::new(INVALID_TIME, INVALID_TIME, 0., 0., false, 0., 0, 0., 0., 0.),
        }
    }

    /// C++: `const ApproachingVehicleInformation * MSLink::getApproachingPtr(const SUMOVehicle *) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_approaching_ptr(&self, veh: Ptr<SUMOVehicle>) -> Ptr<MSLink_ApproachingVehicleInformation> {
        match self.myApproachingVehicles.get(&veh) {
            Some(i) => Ptr::new(i),
            None => Ptr::null(),
        }
    }

    /// C++: `const ApproachInfos & MSLink::getApproaching() const` [const]
    /// return all approaching vehicles
    // const ApproachInfos& getApproaching() const {
    //         return myApproachingVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_approaching_2(&self) -> &MSLink_ApproachInfos {
        &self.myApproachingVehicles
    }

    /// C++ ApproachInfos iterates in ComparatorNumericalIdLess order; the map here is keyed by address, so sort by numerical id.
    pub fn approach_iter_by_id(&self) -> Vec<(Ptr<SUMOVehicle>, &MSLink_ApproachingVehicleInformation)> {
        let mut v: Vec<(Ptr<SUMOVehicle>, &MSLink_ApproachingVehicleInformation)> = self.myApproachingVehicles.iter().map(|(&veh, it)| (veh, it)).collect();
        v.sort_by_key(|(veh, _)| veh.get_numerical_id());
        v
    }

    /// C++: `const PersonApproachInfos * MSLink::getApproachingPersons() const` [const]
    /// return all approaching vehicles
    // const PersonApproachInfos* getApproachingPersons() const {
    //         return myApproachingPersons;
    //     }
    #[allow(unused_variables)]
    pub fn get_approaching_persons(&self) -> Ptr<MSLink_PersonApproachInfos> {
        self.myApproachingPersons
    }

    /// C++: `void MSLink::clearState()`
    /// Remove all approaching vehicles before quick-loading state
    // void
    // MSLink::clearState() {
    //     myApproachingVehicles.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myApproachingVehicles.clear();
    }

    /// C++: `bool MSLink::opened(SUMOTime, double, double, double, double, double, SUMOTime, double, BlockingFoes *, bool, const SUMOTrafficObject *, double) const` [const]
    /// Returns the information whether the link may be passed
    // bool
    // MSLink::opened(SUMOTime arrivalTime, double arrivalSpeed, double leaveSpeed, double vehicleLength,
    //                double impatience, double decel, SUMOTime waitingTime, double posLat,
    //                BlockingFoes* collectFoes, bool ignoreRed, const SUMOTrafficObject* ego, double dist) const {
    // #ifdef MSLink_DEBUG_OPENED
    //     if (gDebugFlag1) {
    //         std::cout << SIMTIME << "  opened? link=" << getDescription() << " red=" << haveRed() << " cont=" << isCont() << " numFoeLinks=" << myFoeLinks.size() << " havePrio=" << havePriority() << " lastWasContMajorGreen=" << lastWasContState(LINKSTATE_TL_GREEN_MAJOR) << "\n";
    //     }
    // #endif
    //     if (haveRed() && !ignoreRed) {
    //         return false;
    //     }
    //     if (isCont() && MSGlobals::gUsingInternalLanes) {
    //         return true;
    //     }
    //     const SUMOTime leaveTime = getLeaveTime(arrivalTime, arrivalSpeed, leaveSpeed, vehicleLength);
    //     if (MSGlobals::gLateralResolution > 0) {
    //         // check for foes on the same lane with the same target edge
    //         for (const MSLink* foeLink : mySublaneFoeLinks) {
    //             assert(myLane != foeLink->getLane());
    //             for (const auto& it : foeLink->myApproachingVehicles) {
    //                 const SUMOVehicle* foe = it.first;
    //                 if (
    //                     // there only is a conflict if the paths cross
    //                     ((posLat < foe->getLateralPositionOnLane() + it.second.latOffset && myLane->getIndex() > foeLink->myLane->getIndex())
    //                      || (posLat > foe->getLateralPositionOnLane() + it.second.latOffset && myLane->getIndex() < foeLink->myLane->getIndex()))
    //                     // the vehicle that arrives later must yield
    //                     && (arrivalTime > it.second.arrivalTime
    //                         // if both vehicles arrive at the same time, the one
    //                         // to the left must yield
    //                         || (arrivalTime == it.second.arrivalTime && posLat > foe->getLateralPositionOnLane()))) {
    //                     if (blockedByFoe(foe, it.second, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false,
    //                                      impatience, decel, waitingTime, ego)) {
    // #ifdef MSLink_DEBUG_OPENED
    //                         if (gDebugFlag1) {
    //                             std::cout << SIMTIME << " blocked by " << foe->getID() << " arrival=" << arrivalTime << " foeArrival=" << it.second.arrivalTime << "\n";
    //                         }
    // #endif
    //                         if (collectFoes == nullptr) {
    // #ifdef MSLink_DEBUG_OPENED
    //                             if (gDebugFlag1) {
    //                                 std::cout << " link=" << getViaLaneOrLane()->getID() << " blocked by sublaneFoe=" << foe->getID() << " foeLink=" << foeLink->getViaLaneOrLane()->getID() << " posLat=" << posLat << "\n";
    //                             }
    // #endif
    //                             return false;
    //                         } else {
    //                             collectFoes->push_back(it.first);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         // check for foes on the same lane with a different target edge
    //         // (straight movers take precedence if the paths cross)
    //         const int lhSign = MSGlobals::gLefthand ? -1 : 1;
    //         for (const MSLink* foeLink : mySublaneFoeLinks2) {
    //             assert(myDirection != LinkDirection::STRAIGHT);
    //             for (const auto& it : foeLink->myApproachingVehicles) {
    //                 const SUMOVehicle* foe = it.first;
    //                 // there only is a conflict if the paths cross
    //                 // and if the vehicles are not currently in a car-following relationship
    //                 const double egoWidth = ego == nullptr ? 1.8 : ego->getVehicleType().getWidth();
    //                 if (!lateralOverlap(posLat, egoWidth, foe->getLateralPositionOnLane() + it.second.latOffset, foe->getVehicleType().getWidth())
    //                         && (((myDirection == LinkDirection::RIGHT || myDirection == LinkDirection::PARTRIGHT)
    //                              && (posLat * lhSign > (foe->getLateralPositionOnLane() + it.second.latOffset) * lhSign))
    //                             || ((myDirection == LinkDirection::LEFT || myDirection == LinkDirection::PARTLEFT)
    //                                 && (posLat * lhSign < (foe->getLateralPositionOnLane() + it.second.latOffset) * lhSign)))) {
    //                     if (blockedByFoe(foe, it.second, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false,
    //                                      impatience, decel, waitingTime, ego)) {
    // #ifdef MSLink_DEBUG_OPENED
    //                         if (gDebugFlag1) {
    //                             std::cout << SIMTIME << " blocked by sublane foe " << foe->getID() << " arrival=" << arrivalTime << " foeArrival=" << it.second.arrivalTime << "\n";
    //                         }
    // #endif
    //                         if (collectFoes == nullptr) {
    // #ifdef MSLink_DEBUG_OPENED
    //                             if (gDebugFlag1) {
    //                                 std::cout << " link=" << getViaLaneOrLane()->getID() << " blocked by sublaneFoe2=" << foe->getID() << " foeLink=" << foeLink->getViaLaneOrLane()->getID() << " posLat=" << posLat << "\n";
    //                             }
    // #endif
    //                             return false;
    //                         } else {
    //                             collectFoes->push_back(it.first);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // #ifdef MSLink_DEBUG_OPENED
    //     /*
    //     if (gDebugFlag1) {
    //         std::cout << SIMTIME << " isExitLinkAfterInternalJunction=" << isExitLinkAfterInternalJunction()
    //             << " entryLink=" << getCorrespondingEntryLink()->getDescription()
    //             << " entryState=" << getCorrespondingEntryLink()->getState()
    //             << "\n";
    //     }
    //     */
    // #endif
    //     if ((havePriority()
    //             || lastWasContState(LINKSTATE_TL_GREEN_MAJOR)
    //             || (isExitLinkAfterInternalJunction() && getCorrespondingEntryLink()->getState() == LINKSTATE_TL_GREEN_MAJOR))
    //             && myState != LINKSTATE_ZIPPER) {
    //         // priority usually means the link is open but there are exceptions:
    //         // zipper still needs to collect foes
    //         // sublane model could have detected a conflict
    //         return collectFoes == nullptr || collectFoes->size() == 0;
    //     }
    //     if (myState == LINKSTATE_ALLWAY_STOP && waitingTime < TIME2STEPS(ego == nullptr ? TS : ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_ALLWAYSTOP_WAIT, TS))) {
    //         return false;
    //     } else if (myState == LINKSTATE_STOP && waitingTime < TIME2STEPS(ego == nullptr ? TS : ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_STOPSIGN_WAIT, TS))) {
    //         return false;
    //     }
    //
    //     const std::vector<MSLink*>& foeLinks = (myOffFoeLinks == nullptr || getCorrespondingEntryLink()->getState() != LINKSTATE_ALLWAY_STOP) ? myFoeLinks : *myOffFoeLinks;
    //
    //     if (MSGlobals::gUseMesoSim && impatience == 1 && !myLane->getEdge().isRoundabout()) {
    //         return true;
    //     }
    //     if (myLane->getBidiLane() != nullptr) {
    //         MSLane* bidi = myLane->getBidiLane();
    //         if (bidi->getVehicleNumber() > 0) {
    //             if (ego == nullptr) {
    //                 return false;
    //             }
    //             double maxOncomingWidth = 0;
    //             const MSLane::VehCont& vehs = bidi->getVehiclesSecure();
    //             for (MSVehicle* foe : vehs) {
    //                 maxOncomingWidth = MAX2(maxOncomingWidth, foe->getVehicleType().getWidth());
    //             }
    //             bidi->releaseVehicles();
    //             if (MSGlobals::gLateralResolution > 0) {
    //                 if (ego->getVehicleType().getWidth() + maxOncomingWidth + MSGlobals::gLateralResolution < myLane->getWidth()) {
    //                     return false;
    //                 }
    //             } else if (maxOncomingWidth > 0) {
    //                 // do not enter a lane that has any oncoming vehicles
    //                 return false;
    //             }
    //         }
    //         for (auto ili : bidi->getIncomingLanes()) {
    //             if (ili.lane->getEdge().getPriority() > myLaneBefore->getEdge().getPriority()
    //                     || (ili.lane->getEdge().getPriority() == myLaneBefore->getEdge().getPriority()
    //                         && ili.lane->getID() > myLaneBefore->getID())) {
    //                 BlockingFoes bidiApproachFoes;
    //                 double maxOncomingWidth = 0;
    //                 if (ili.viaLink->blockedAtTime(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false, 0, decel, 0,
    //                             MSGlobals::gLateralResolution ? &bidiApproachFoes : nullptr, ego) || bidiApproachFoes.size() > 0) {
    //                     if (MSGlobals::gLateralResolution > 0) {
    //                         for (const SUMOTrafficObject* foe : bidiApproachFoes) {
    //                             maxOncomingWidth = MAX2(maxOncomingWidth, foe->getVehicleType().getWidth());
    //                         }
    //                         if (ego->getVehicleType().getWidth() + maxOncomingWidth + MSGlobals::gLateralResolution < myLane->getWidth()) {
    //                             return false;
    //                         }
    //                     } else {
    //                         return false;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     const bool lastWasContRed = lastWasContState(LINKSTATE_TL_RED);
    //     for (const MSLink* const link : foeLinks) {
    //         if (MSGlobals::gUseMesoSim) {
    //             if (link->haveRed()) {
    //                 continue;
    //             }
    //         }
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag1) {
    //             std::cout << SIMTIME << "   foeLink=" << link->getViaLaneOrLane()->getID() << " numApproaching=" << link->getApproaching().size() << "\n";
    //             if (link->getLane()->isCrossing()) {
    //                 std::cout << SIMTIME << "     approachingPersons=" << (link->myApproachingPersons == nullptr ? "NULL" : toString(link->myApproachingPersons->size())) << "\n";
    //             }
    //         }
    // #endif
    //         if (link->blockedAtTime(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, myLane == link->getLane(),
    //                                 impatience, decel, waitingTime, collectFoes, ego, lastWasContRed, dist)) {
    //             return false;
    //         }
    //     }
    //     if (collectFoes != nullptr && collectFoes->size() > 0) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn opened(&self, arrivalTime: SUMOTime, arrivalSpeed: f64, leaveSpeed: f64, vehicleLength: f64, impatience: f64, decel: f64, waitingTime: SUMOTime, posLat: f64, collectFoes: Ptr<MSLink_BlockingFoes>, ignoreRed: bool, ego: Ptr<SUMOTrafficObject>, dist: f64) -> bool {
        if self.have_red() && !ignoreRed {
            return false;
        }
        if self.is_cont() && g_using_internal_lanes() {
            return true;
        }
        let leaveTime = self.get_leave_time(arrivalTime, arrivalSpeed, leaveSpeed, vehicleLength);
        if g_lateral_resolution() > 0. {
            for &foeLink in self.mySublaneFoeLinks.iter() {
                debug_assert!(self.myLane != foeLink.get_lane());
                for (foe, info) in foeLink.approach_iter_by_id() {
                    let foeLat = foe.get_lateral_position_on_lane();
                    if ((posLat < foeLat + info.latOffset && self.myLane.get_index() > foeLink.myLane.get_index())
                        || (posLat > foeLat + info.latOffset && self.myLane.get_index() < foeLink.myLane.get_index()))
                        && (arrivalTime > info.arrivalTime || (arrivalTime == info.arrivalTime && posLat > foeLat))
                    {
                        if self.blocked_by_foe(foe, info, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false, impatience, decel, waitingTime, ego) {
                            if collectFoes.is_null() {
                                return false;
                            } else {
                                collectFoes.m().push(veh_to_traffic_object(foe));
                            }
                        }
                    }
                }
            }
            let lhSign = if g_lefthand() { -1. } else { 1. };
            for &foeLink in self.mySublaneFoeLinks2.iter() {
                debug_assert!(self.myDirection != LinkDirection::STRAIGHT);
                for (foe, info) in foeLink.approach_iter_by_id() {
                    let egoWidth = if ego.is_null() { 1.8 } else { ego.get_vehicle_type().get_width() };
                    let foeLat = foe.get_lateral_position_on_lane() + info.latOffset;
                    if !MSLink::lateral_overlap(posLat, egoWidth, foeLat, foe.get_vehicle_type().get_width())
                        && (((self.myDirection == LinkDirection::RIGHT || self.myDirection == LinkDirection::PARTRIGHT) && (posLat * lhSign > foeLat * lhSign))
                            || ((self.myDirection == LinkDirection::LEFT || self.myDirection == LinkDirection::PARTLEFT) && (posLat * lhSign < foeLat * lhSign)))
                    {
                        if self.blocked_by_foe(foe, info, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false, impatience, decel, waitingTime, ego) {
                            if collectFoes.is_null() {
                                return false;
                            } else {
                                collectFoes.m().push(veh_to_traffic_object(foe));
                            }
                        }
                    }
                }
            }
        }
        if (self.have_priority()
            || self.last_was_cont_state(LINKSTATE_TL_GREEN_MAJOR)
            || (self.is_exit_link_after_internal_junction() && self.get_corresponding_entry_link().get_state() == LINKSTATE_TL_GREEN_MAJOR))
            && self.myState != LINKSTATE_ZIPPER
        {
            return collectFoes.is_null() || collectFoes.len() == 0;
        }
        if self.myState == LINKSTATE_ALLWAY_STOP
            && waitingTime < time2steps(if ego.is_null() { ts() } else { ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_ALLWAYSTOP_WAIT, ts()) })
        {
            return false;
        } else if self.myState == LINKSTATE_STOP
            && waitingTime < time2steps(if ego.is_null() { ts() } else { ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_STOPSIGN_WAIT, ts()) })
        {
            return false;
        }
        let foeLinks: Vec<Ptr<MSLink>> = if self.myOffFoeLinks.is_null() || self.get_corresponding_entry_link().get_state() != LINKSTATE_ALLWAY_STOP {
            self.myFoeLinks.clone()
        } else {
            (*self.myOffFoeLinks).clone()
        };
        if g_use_meso_sim() && impatience == 1. && !self.myLane.get_edge().is_roundabout() {
            return true;
        }
        if !self.myLane.get_bidi_lane().is_null() {
            let bidi = self.myLane.get_bidi_lane();
            if bidi.get_vehicle_number() > 0 {
                if ego.is_null() {
                    return false;
                }
                let mut maxOncomingWidth = 0.;
                let vehs = bidi.get_vehicles_secure();
                for &foe in vehs.iter() {
                    maxOncomingWidth = max2(maxOncomingWidth, foe.get_vehicle_type().get_width());
                }
                bidi.release_vehicles();
                if g_lateral_resolution() > 0. {
                    if ego.get_vehicle_type().get_width() + maxOncomingWidth + g_lateral_resolution() < self.myLane.get_width() {
                        return false;
                    }
                } else if maxOncomingWidth > 0. {
                    return false;
                }
            }
            for ili in bidi.get_incoming_lanes().iter() {
                let ilane = ili.lane;
                if ilane.get_edge().get_priority() > self.myLaneBefore.get_edge().get_priority()
                    || (ilane.get_edge().get_priority() == self.myLaneBefore.get_edge().get_priority() && ilane.get_id() > self.myLaneBefore.get_id())
                {
                    let mut bidiApproachFoes: MSLink_BlockingFoes = Vec::new();
                    let mut maxOncomingWidth = 0.;
                    let collect = if g_lateral_resolution() != 0. { Ptr::from_mut(&mut bidiApproachFoes) } else { Ptr::null() };
                    if ili.viaLink.blocked_at_time(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, false, 0., decel, 0, collect, ego, false, -1.) || bidiApproachFoes.len() > 0 {
                        if g_lateral_resolution() > 0. {
                            for &foe in bidiApproachFoes.iter() {
                                maxOncomingWidth = max2(maxOncomingWidth, foe.get_vehicle_type().get_width());
                            }
                            if ego.get_vehicle_type().get_width() + maxOncomingWidth + g_lateral_resolution() < self.myLane.get_width() {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        let lastWasContRed = self.last_was_cont_state(LINKSTATE_TL_RED);
        for &link in foeLinks.iter() {
            if g_use_meso_sim() {
                if link.have_red() {
                    continue;
                }
            }
            if link.blocked_at_time(arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, self.myLane == link.get_lane(), impatience, decel, waitingTime, collectFoes, ego, lastWasContRed, dist) {
                return false;
            }
        }
        if !collectFoes.is_null() && collectFoes.len() > 0 {
            return false;
        }
        true
    }

    /// C++: `bool MSLink::blockedAtTime(SUMOTime, SUMOTime, double, double, bool, double, double, SUMOTime, BlockingFoes *, const SUMOTrafficObject *, bool, double) const` [const]
    /// Returns the information whether this link is blocked Valid after the vehicles have set their requests
    // bool
    // MSLink::blockedAtTime(SUMOTime arrivalTime, SUMOTime leaveTime, double arrivalSpeed, double leaveSpeed,
    //                       bool sameTargetLane, double impatience, double decel, SUMOTime waitingTime,
    //                       BlockingFoes* collectFoes, const SUMOTrafficObject* ego, bool lastWasContRed, double dist) const {
    //     for (const auto& it : myApproachingVehicles) {
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag1) {
    //             if (ego != nullptr
    //                     && ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_SPEED, 0) >= it.second.speed
    //                     && ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0) > 0) {
    //                 std::stringstream stream; // to reduce output interleaving from different threads
    //                 stream << SIMTIME << " " << myApproachingVehicles.size() << "   foe link=" << getViaLaneOrLane()->getID()
    //                        << " foeVeh=" << it.first->getID() << " (below ignore speed)"
    //                        << " ignoreFoeProb=" << ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0)
    //                        << "\n";
    //                 std::cout << stream.str();
    //             }
    //         }
    // #endif
    //         if (it.first != ego
    //                 && (ego == nullptr
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0) == 0
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_SPEED, 0) < it.second.speed
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0) < RandHelper::rand(ego->getRNG()))
    //                 && !ignoreFoe(ego, it.first)
    //                 && (!lastWasContRed || it.first->getSpeed() > SUMO_const_haltingSpeed)
    //                 && blockedByFoe(it.first, it.second, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, sameTargetLane,
    //                                 impatience, decel, waitingTime, ego)) {
    //             if (collectFoes == nullptr) {
    //                 return true;
    //             } else {
    //                 collectFoes->push_back(it.first);
    //             }
    //         }
    //     }
    //     if (myApproachingPersons != nullptr && !haveRed()) {
    //         const SUMOTime lookAhead = (ego == nullptr
    //                                     ? myLookaheadTime
    //                                     : TIME2STEPS(ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_TIMEGAP_MINOR, STEPS2TIME(myLookaheadTime))));
    //         for (const auto& it : *myApproachingPersons) {
    // #ifdef MSLink_DEBUG_OPENED
    //             if (gDebugFlag1) {
    //                 std::cout << SIMTIME << ": " << ego->getID() << " check person " << it.first->getID() << " aTime=" << arrivalTime << " foeATime=" << it.second.arrivalTime
    //                           << " lTime=" << leaveTime << " foeLTime=" << it.second.leavingTime
    //                           << " dist=" << dist << "\n";
    //             }
    // #endif
    //             if ((ego == nullptr
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0) == 0
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_SPEED, 0) < it.first->getSpeed()
    //                     || ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0) < RandHelper::rand(ego->getRNG()))
    //                     && !ignoreFoe(ego, it.first)
    //                     && !((arrivalTime > it.second.leavingTime + lookAhead) || (leaveTime + lookAhead < it.second.arrivalTime))) {
    //                 if (ego == nullptr) {
    //                     // during insertion
    //                     if (myJunction->getType() == SumoXMLNodeType::RAIL_CROSSING) {
    //                         continue;
    //                     } else {
    //                         return true;
    //                     }
    //                 }
    //                 // check whether braking is feasible (ego might have started to accelerate already)
    //                 const auto& cfm = ego->getVehicleType().getCarFollowModel();
    // #ifdef MSLink_DEBUG_OPENED
    //                 if (gDebugFlag1) {
    //                     std::cout << SIMTIME << ": " << ego->getID() << " conflict with person " << it.first->getID() << " aTime=" << arrivalTime << " foeATime=" << it.second.arrivalTime << " dist=" << dist << " bGap=" << cfm.brakeGap(ego->getSpeed(), cfm.getMaxDecel(), 0) << "\n";
    //                 }
    // #endif
    //                 if (dist > cfm.brakeGap(ego->getSpeed(), cfm.getMaxDecel(), 0)) {
    // #ifdef MSLink_DEBUG_OPENED
    //                     if (gDebugFlag1) {
    //                         std::cout << SIMTIME << ": " << ego->getID() << " blocked by person " << it.first->getID() << "\n";
    //                     }
    // #endif
    //                     if (collectFoes == nullptr) {
    //                         return true;
    //                     } else {
    //                         collectFoes->push_back(it.first);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn blocked_at_time(&self, arrivalTime: SUMOTime, leaveTime: SUMOTime, arrivalSpeed: f64, leaveSpeed: f64, sameTargetLane: bool, impatience: f64, decel: f64, waitingTime: SUMOTime, collectFoes: Ptr<MSLink_BlockingFoes>, ego: Ptr<SUMOTrafficObject>, lastWasContRed: bool, dist: f64) -> bool {
        for (veh, it) in self.approach_iter_by_id() {
            let vehObj = veh_to_traffic_object(veh);
            if vehObj != ego
                && (ego.is_null()
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0.) == 0.
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_SPEED, 0.) < it.speed
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0.) < ego_rand(ego))
                && !MSLink::ignore_foe(ego, vehObj)
                && (!lastWasContRed || veh.get_speed() > SUMO_CONST_HALTING_SPEED)
                && self.blocked_by_foe(veh, it, arrivalTime, leaveTime, arrivalSpeed, leaveSpeed, sameTargetLane, impatience, decel, waitingTime, ego)
            {
                if collectFoes.is_null() {
                    return true;
                } else {
                    collectFoes.m().push(vehObj);
                }
            }
        }
        if !self.myApproachingPersons.is_null() && !self.have_red() {
            let lookAhead: SUMOTime = if ego.is_null() {
                MSLink::myLookaheadTime
            } else {
                time2steps(ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_TIMEGAP_MINOR, steps2time(MSLink::myLookaheadTime)))
            };
            for (&person, it) in self.myApproachingPersons.iter() {
                let personObj = person_to_traffic_object(person);
                if (ego.is_null()
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0.) == 0.
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_SPEED, 0.) < person.get_speed()
                    || ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_IGNORE_FOE_PROB, 0.) < ego_rand(ego))
                    && !MSLink::ignore_foe(ego, personObj)
                    && !((arrivalTime > it.leavingTime + lookAhead) || (leaveTime + lookAhead < it.arrivalTime))
                {
                    if ego.is_null() {
                        if self.myJunction.get_type() == SumoXMLNodeType::RAIL_CROSSING {
                            continue;
                        } else {
                            return true;
                        }
                    }
                    let cfm = ego.get_vehicle_type().get_car_follow_model();
                    if dist > cfm.brake_gap_2(ego.get_speed(), cfm.get_max_decel(), 0.) {
                        if collectFoes.is_null() {
                            return true;
                        } else {
                            collectFoes.m().push(personObj);
                        }
                    }
                }
            }
        }
        false
    }

    /// C++: `bool MSLink::hasApproachingFoe(SUMOTime, SUMOTime, double, double) const` [const]
    /// Returns the information whether a vehicle is approaching on one of the link's foe streams
    // bool
    // MSLink::hasApproachingFoe(SUMOTime arrivalTime, SUMOTime leaveTime, double speed, double decel) const {
    //     for (const MSLink* const link : myFoeLinks) {
    //         if (link->blockedAtTime(arrivalTime, leaveTime, speed, speed, myLane == link->getLane(), 0, decel, 0)) {
    //             return true;
    //         }
    //     }
    //     for (const MSLane* const lane : myFoeLanes) {
    //         if (lane->getVehicleNumberWithPartials() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_approaching_foe(&self, arrivalTime: SUMOTime, leaveTime: SUMOTime, speed: f64, decel: f64) -> bool {
        for &link in self.myFoeLinks.iter() {
            if link.blocked_at_time(arrivalTime, leaveTime, speed, speed, self.myLane == link.get_lane(), 0., decel, 0, Ptr::null(), Ptr::null(), false, -1.) {
                return true;
            }
        }
        for &lane in self.myFoeLanes.iter() {
            if lane.get_vehicle_number_with_partials() > 0 {
                return true;
            }
        }
        false
    }

    /// C++: `std::pair<const SUMOVehicle *, const MSLink *> MSLink::getFirstApproachingFoe(const MSLink *) const` [const]
    /// get the foe vehicle that is closest to the intersection or nullptr along with the foe link This function is used for finding circular deadlock at right_before_left junctions
    // std::pair<const SUMOVehicle*, const MSLink*>
    // MSLink::getFirstApproachingFoe(const MSLink* wrapAround) const {
    //     double closetDist = std::numeric_limits<double>::max();
    //     const SUMOVehicle* closest = nullptr;
    //     const MSLink* foeLink = nullptr;
    //     for (MSLink* link : myFoeLinks) {
    //         for (const auto& it : link->myApproachingVehicles) {
    //             //std::cout << " link=" << getDescription() << " foeLink_in=" << link->getLaneBefore()->getID() << " wrapAround=" << wrapAround->getDescription() << "\n";
    //             if (link->getLaneBefore() == wrapAround->getLaneBefore()) {
    //                 return std::make_pair(nullptr, wrapAround);
    //             } else if (it.second.dist < closetDist) {
    //                 closetDist = it.second.dist;
    //                 if (it.second.willPass) {
    //                     closest = it.first;
    //                     foeLink = link;
    //                 }
    //             }
    //         }
    //     }
    //     return std::make_pair(closest, foeLink);
    // }
    #[allow(unused_variables)]
    pub fn get_first_approaching_foe(&self, wrapAround: Ptr<MSLink>) -> (Ptr<SUMOVehicle>, Ptr<MSLink>) {
        let mut closetDist = f64::MAX;
        let mut closest: Ptr<SUMOVehicle> = Ptr::null();
        let mut foeLink: Ptr<MSLink> = Ptr::null();
        for &link in self.myFoeLinks.iter() {
            for (veh, it) in link.approach_iter_by_id() {
                if link.get_lane_before() == wrapAround.get_lane_before() {
                    return (Ptr::null(), wrapAround);
                } else if it.dist < closetDist {
                    closetDist = it.dist;
                    if it.willPass {
                        closest = veh;
                        foeLink = link;
                    }
                }
            }
        }
        (closest, foeLink)
    }

    /// C++: `MSJunction * MSLink::getJunction() const` [const]
    // MSJunction* getJunction() const {
    //         return myJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_junction(&self) -> Ptr<MSJunction> {
        self.myJunction
    }

    /// C++: `LinkState MSLink::getState() const` [const]
    /// Returns the current state of the link
    // LinkState getState() const {
    //         return myState;
    //     }
    #[allow(unused_variables)]
    pub fn get_state(&self) -> LinkState {
        self.myState
    }

    /// C++: `LinkState MSLink::getOffState() const` [const]
    /// Returns the off-state for the link
    // LinkState getOffState() const {
    //         return myOffState;
    //     }
    #[allow(unused_variables)]
    pub fn get_off_state(&self) -> LinkState {
        self.myOffState
    }

    /// C++: `LinkState MSLink::getLastGreenState() const` [const]
    /// Returns the last green state of the link
    // LinkState getLastGreenState() const {
    //         return myLastGreenState;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_green_state(&self) -> LinkState {
        self.myLastGreenState
    }

    /// C++: `SUMOTime MSLink::getLastStateChange() const` [const]
    // inline SUMOTime getLastStateChange() const {
    //         return myLastStateChange;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_state_change(&self) -> SUMOTime {
        self.myLastStateChange
    }

    /// C++: `LinkDirection MSLink::getDirection() const` [const]
    /// Returns the direction the vehicle passing this link take
    // inline LinkDirection getDirection() const {
    //         return myDirection;
    //     }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> LinkDirection {
        self.myDirection
    }

    /// C++: `void MSLink::setTLState(LinkState, SUMOTime)`
    /// Sets the current tl-state
    // void
    // MSLink::setTLState(LinkState state, SUMOTime t) {
    //     if (myState != state) {
    //         myLastStateChange = t;
    //     }
    //     myState = state;
    //     if (haveGreen()) {
    //         myLastGreenState = myState;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_tl_state(&mut self, state: LinkState, t: SUMOTime) {
        if self.myState != state {
            self.myLastStateChange = t;
        }
        self.myState = state;
        if self.have_green() {
            self.myLastGreenState = self.myState;
        }
    }

    /// C++: `void MSLink::setTLLogic(const MSTrafficLightLogic *)`
    /// Sets the currently active tlLogic
    // void
    // MSLink::setTLLogic(const MSTrafficLightLogic* logic) {
    //     myLogic = logic;
    // }
    #[allow(unused_variables)]
    pub fn set_tl_logic(&mut self, logic: Ptr<MSTrafficLightLogic>) {
        self.myLogic = logic;
    }

    /// C++: `MSLane * MSLink::getLane() const` [const]
    /// Returns the connected lane
    // inline MSLane* getLane() const {
    //         return myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        self.myLane
    }

    /// C++: `int MSLink::getIndex() const` [const]
    /// Returns the respond index (for visualization)
    // inline int getIndex() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_index(&self) -> i32 {
        self.myIndex
    }

    /// C++: `int MSLink::getTLIndex() const` [const]
    /// Returns the TLS index
    // inline int getTLIndex() const {
    //         return myTLIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_index(&self) -> i32 {
        self.myTLIndex
    }

    /// C++: `const MSTrafficLightLogic * MSLink::getTLLogic() const` [const]
    /// Returns the TLS index
    // inline const MSTrafficLightLogic* getTLLogic() const {
    //         return myLogic;
    //     }
    #[allow(unused_variables)]
    pub fn get_tl_logic(&self) -> Ptr<MSTrafficLightLogic> {
        self.myLogic
    }

    /// C++: `bool MSLink::havePriority() const` [const]
    /// Returns whether this link is a major link
    // inline bool havePriority() const {
    //         return myState >= 'A' && myState <= 'Z';
    //     }
    #[allow(unused_variables)]
    pub fn have_priority(&self) -> bool {
        self.myState >= 'A' as i32 && self.myState <= 'Z' as i32
    }

    /// C++: `bool MSLink::haveOffPriority() const` [const]
    // inline bool haveOffPriority() const {
    //         return myOffState >= 'A' && myOffState <= 'Z';
    //     }
    #[allow(unused_variables)]
    pub fn have_off_priority(&self) -> bool {
        self.myOffState >= 'A' as i32 && self.myOffState <= 'Z' as i32
    }

    /// C++: `bool MSLink::haveRed() const` [const]
    /// Returns whether this link is blocked by a red (or redyellow) traffic light
    // inline bool haveRed() const {
    //         return myState == LINKSTATE_TL_RED || myState == LINKSTATE_TL_REDYELLOW;
    //     }
    #[allow(unused_variables)]
    pub fn have_red(&self) -> bool {
        self.myState == LINKSTATE_TL_RED || self.myState == LINKSTATE_TL_REDYELLOW
    }

    /// C++: `bool MSLink::haveYellow() const` [const]
    // inline bool haveYellow() const {
    //         return myState == LINKSTATE_TL_YELLOW_MINOR || myState == LINKSTATE_TL_YELLOW_MAJOR;
    //     }
    #[allow(unused_variables)]
    pub fn have_yellow(&self) -> bool {
        self.myState == LINKSTATE_TL_YELLOW_MINOR || self.myState == LINKSTATE_TL_YELLOW_MAJOR
    }

    /// C++: `bool MSLink::haveGreen() const` [const]
    // inline bool haveGreen() const {
    //         return myState == LINKSTATE_TL_GREEN_MAJOR || myState == LINKSTATE_TL_GREEN_MINOR;
    //     }
    #[allow(unused_variables)]
    pub fn have_green(&self) -> bool {
        self.myState == LINKSTATE_TL_GREEN_MAJOR || self.myState == LINKSTATE_TL_GREEN_MINOR
    }

    /// C++: `bool MSLink::mustStop() const` [const]
    // inline bool mustStop() const {
    //         return myState == LINKSTATE_STOP || myState == LINKSTATE_ALLWAY_STOP;
    //     }
    #[allow(unused_variables)]
    pub fn must_stop(&self) -> bool {
        self.myState == LINKSTATE_STOP || self.myState == LINKSTATE_ALLWAY_STOP
    }

    /// C++: `bool MSLink::isTLSControlled() const` [const]
    // inline bool isTLSControlled() const {
    //         return myLogic != 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_tls_controlled(&self) -> bool {
        !self.myLogic.is_null()
    }

    /// C++: `bool MSLink::isTurnaround() const` [const]
    // inline bool isTurnaround() const {
    //         return myDirection == LinkDirection::TURN || myDirection == LinkDirection::TURN_LEFTHAND;
    //     }
    #[allow(unused_variables)]
    pub fn is_turnaround(&self) -> bool {
        self.myDirection == LinkDirection::TURN || self.myDirection == LinkDirection::TURN_LEFTHAND
    }

    /// C++: `double MSLink::getLength() const` [const]
    /// Returns the length of this link
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `double MSLink::getFoeVisibilityDistance() const` [const]
    /// Returns the distance on the approaching lane from which an approaching vehicle is able to see all relevant foes and may accelerate if the link is minor and no foe is approaching.
    // double getFoeVisibilityDistance() const {
    //         return myFoeVisibilityDistance;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_visibility_distance(&self) -> f64 {
        self.myFoeVisibilityDistance
    }

    /// C++: `double MSLink::getDistToFoePedCrossing() const` [const]
    // double getDistToFoePedCrossing() const {
    //         return myDistToFoePedCrossing;
    //     }
    #[allow(unused_variables)]
    pub fn get_dist_to_foe_ped_crossing(&self) -> f64 {
        self.myDistToFoePedCrossing
    }

    /// C++: `bool MSLink::hasFoes() const` [const]
    /// Returns whether this link belongs to a junction where more than one edge is incoming
    // bool hasFoes() const {
    //         return myHasFoes;
    //     }
    #[allow(unused_variables)]
    pub fn has_foes(&self) -> bool {
        self.myHasFoes
    }

    /// C++: `bool MSLink::isCont() const` [const]
    // bool
    // MSLink::isCont() const {
    //     // when a traffic light is switched off minor roads have their cont status revoked
    //     return (myState == LINKSTATE_TL_OFF_BLINKING || myState == LINKSTATE_ALLWAY_STOP || myState == LINKSTATE_STOP) ? myAmContOff : myAmCont;
    // }
    #[allow(unused_variables)]
    pub fn is_cont(&self) -> bool {
        // when a traffic light is switched off minor roads have their cont status revoked
        if self.myState == LINKSTATE_TL_OFF_BLINKING || self.myState == LINKSTATE_ALLWAY_STOP || self.myState == LINKSTATE_STOP {
            self.myAmContOff
        } else {
            self.myAmCont
        }
    }

    /// C++: `bool MSLink::keepClear() const` [const]
    /// whether the junction after this link must be kept clear
    // bool keepClear() const {
    //         return myKeepClear;
    //     }
    #[allow(unused_variables)]
    pub fn keep_clear(&self) -> bool {
        self.myKeepClear
    }

    /// C++: `bool MSLink::isIndirect() const` [const]
    /// whether this link is the start of an indirect turn
    // bool isIndirect() const {
    //         return myAmIndirect;
    //     }
    #[allow(unused_variables)]
    pub fn is_indirect(&self) -> bool {
        self.myAmIndirect
    }

    /// C++: `bool MSLink::lastWasContMajor() const` [const]
    /// whether this is a link past an internal junction which currently has priority
    // bool
    // MSLink::lastWasContMajor() const {
    //     if (isExitLinkAfterInternalJunction()) {
    //         return myInternalLaneBefore->getIncomingLanes()[0].viaLink->lastWasContMajor();
    //     }
    //     if (myInternalLane == nullptr || myAmCont) {
    //         return false;
    //     } else {
    //         MSLane* pred = myInternalLane->getLogicalPredecessorLane();
    //         if (!pred->getEdge().isInternal()) {
    //             return false;
    //         } else {
    //             const MSLane* const pred2 = pred->getLogicalPredecessorLane();
    //             assert(pred2 != nullptr);
    //             const MSLink* const predLink = pred2->getLinkTo(pred);
    //             assert(predLink != nullptr);
    //             if (predLink->havePriority()) {
    //                 return true;
    //             }
    //             if (myHavePedestrianCrossingFoe) {
    //                 return predLink->getLastGreenState() == LINKSTATE_TL_GREEN_MAJOR;
    //             } else {
    //                 return predLink->haveYellow();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn last_was_cont_major(&self) -> bool {
        if self.is_exit_link_after_internal_junction() {
            self.myInternalLaneBefore.get_incoming_lanes()[0].viaLink.last_was_cont_major()
        } else if self.myInternalLane.is_null() || self.myAmCont {
            false
        } else {
            let pred = self.myInternalLane.get_logical_predecessor_lane();
            if !pred.get_edge().is_internal() {
                false
            } else {
                let pred2 = pred.get_logical_predecessor_lane();
                debug_assert!(!pred2.is_null());
                let predLink = pred2.get_link_to(pred);
                debug_assert!(!predLink.is_null());
                if predLink.have_priority() {
                    true
                } else if self.myHavePedestrianCrossingFoe {
                    predLink.get_last_green_state() == LINKSTATE_TL_GREEN_MAJOR
                } else {
                    predLink.have_yellow()
                }
            }
        }
    }

    /// C++: `bool MSLink::lastWasContState(LinkState) const` [const]
    /// whether this is a link past an internal junction where the entry to the junction currently has the given state
    // bool
    // MSLink::lastWasContState(LinkState linkState) const {
    //     if (myInternalLane == nullptr || myAmCont || myHavePedestrianCrossingFoe) {
    //         return false;
    //     } else {
    //         MSLane* pred = myInternalLane->getLogicalPredecessorLane();
    //         if (!pred->getEdge().isInternal()) {
    //             return false;
    //         } else {
    //             const MSLane* const pred2 = pred->getLogicalPredecessorLane();
    //             assert(pred2 != nullptr);
    //             const MSLink* const predLink = pred2->getLinkTo(pred);
    //             assert(predLink != nullptr);
    //             return predLink->getState() == linkState;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn last_was_cont_state(&self, linkState: LinkState) -> bool {
        if self.myInternalLane.is_null() || self.myAmCont || self.myHavePedestrianCrossingFoe {
            false
        } else {
            let pred = self.myInternalLane.get_logical_predecessor_lane();
            if !pred.get_edge().is_internal() {
                false
            } else {
                let pred2 = pred.get_logical_predecessor_lane();
                debug_assert!(!pred2.is_null());
                let predLink = pred2.get_link_to(pred);
                debug_assert!(!predLink.is_null());
                predLink.get_state() == linkState
            }
        }
    }

    /// C++: `double MSLink::getInternalLengthsAfter() const` [const]
    /// Returns the cumulative length of all internal lanes after this link
    // double
    // MSLink::getInternalLengthsAfter() const {
    //     double len = 0.;
    //     MSLane* lane = myInternalLane;
    //
    //     while (lane != nullptr && lane->isInternal()) {
    //         len += lane->getLength();
    //         lane = lane->getLinkCont()[0]->getViaLane();
    //     }
    //     return len;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_lengths_after(&self) -> f64 {
        let mut len = 0.;
        let mut lane = self.myInternalLane;
        while !lane.is_null() && lane.is_internal() {
            len += lane.get_length();
            lane = lane.get_link_cont()[0].get_via_lane();
        }
        len
    }

    /// C++: `double MSLink::getInternalLengthsBefore() const` [const]
    /// Returns the cumulative length of all internal lanes before this link
    // double
    // MSLink::getInternalLengthsBefore() const {
    //     double len = 0.;
    //     const MSLane* lane = myInternalLane;
    //
    //     while (lane != nullptr && lane->isInternal()) {
    //         len += lane->getLength();
    //         if (lane->getIncomingLanes().size() == 1) {
    //             lane = lane->getIncomingLanes()[0].lane;
    //         } else {
    //             break;
    //         }
    //     }
    //     return len;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_lengths_before(&self) -> f64 {
        let mut len = 0.;
        let mut lane = self.myInternalLane;
        while !lane.is_null() && lane.is_internal() {
            len += lane.get_length();
            if lane.get_incoming_lanes().len() == 1 {
                lane = lane.get_incoming_lanes()[0].lane;
            } else {
                break;
            }
        }
        len
    }

    /// C++: `double MSLink::getLengthsBeforeCrossing(const MSLane *) const` [const]
    /// Returns the sum of the lengths along internal lanes following this link to the crossing with the given foe lane, if the lane is no foe lane to any of the internal lanes, INVALID_DOUBLE is returned.
    // double
    // MSLink::getLengthsBeforeCrossing(const MSLane* foeLane) const {
    //     MSLane* via = myInternalLane;
    //     double totalDist = 0.;
    //     bool foundCrossing = false;
    //     while (via != nullptr) {
    //         MSLink* link = via->getLinkCont()[0];
    //         double dist = link->getLengthBeforeCrossing(foeLane);
    //         if (dist != INVALID_DOUBLE) {
    //             // found conflicting lane
    //             totalDist += dist;
    //             foundCrossing = true;
    //             break;
    //         } else {
    //             totalDist += via->getLength();
    //             via = link->getViaLane();
    //         }
    //     }
    //     if (foundCrossing) {
    //         return totalDist;
    //     } else {
    //         return INVALID_DOUBLE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lengths_before_crossing(&self, foeLane: Ptr<MSLane>) -> f64 {
        let mut via = self.myInternalLane;
        let mut totalDist = 0.;
        let mut foundCrossing = false;
        while !via.is_null() {
            let link = via.get_link_cont()[0];
            let dist = link.get_length_before_crossing(foeLane);
            if dist != INVALID_DOUBLE {
                totalDist += dist;
                foundCrossing = true;
                break;
            } else {
                totalDist += via.get_length();
                via = link.get_via_lane();
            }
        }
        if foundCrossing {
            totalDist
        } else {
            INVALID_DOUBLE
        }
    }

    /// C++: `double MSLink::getLengthBeforeCrossing(const MSLane *) const` [const]
    /// Returns the internal length from the beginning of the link's internal lane before to the crossing with the given foe lane if applicable, if the lane is no foe lane to the link, INVALID_DOUBLE is returned.
    // double
    // MSLink::getLengthBeforeCrossing(const MSLane* foeLane) const {
    //     int foe_ix;
    //     for (foe_ix = 0; foe_ix != (int)myFoeLanes.size(); ++foe_ix) {
    //         if (myFoeLanes[foe_ix] == foeLane) {
    //             break;
    //         }
    //     }
    //     if (foe_ix == (int)myFoeLanes.size()) {
    //         // no conflict with the given lane, indicate by returning -1
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //         std::cout << "No crossing of lanes '" << foeLane->getID() << "' and '" << myInternalLaneBefore->getID() << "'" << std::endl;
    // #endif
    //         return INVALID_DOUBLE;
    //     } else {
    //         // found conflicting lane index
    //         double dist = myInternalLaneBefore->getLength() - myConflicts[foe_ix].getLengthBehindCrossing(this);
    //         if (dist == -10000.) {
    //             // this is the value in myConflicts, if the relation allows intersection but none is present for the actual geometry.
    //             return INVALID_DOUBLE;
    //         }
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //         std::cout << "Crossing of lanes '" << myInternalLaneBefore->getID() << "' and '" << foeLane->getID()
    //                   << "' at distance " << dist << " (approach along '"
    //                   <<  myInternalLaneBefore->getEntryLink()->getLaneBefore()->getID() << "')" << std::endl;
    // #endif
    //         return dist;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_length_before_crossing(&self, foeLane: Ptr<MSLane>) -> f64 {
        let mut foe_ix: usize = 0;
        while foe_ix != self.myFoeLanes.len() {
            if self.myFoeLanes[foe_ix] == foeLane {
                break;
            }
            foe_ix += 1;
        }
        if foe_ix == self.myFoeLanes.len() {
            INVALID_DOUBLE
        } else {
            let dist = self.myInternalLaneBefore.get_length() - self.myConflicts[foe_ix].get_length_behind_crossing(Ptr::new(self));
            if dist == -10000. {
                INVALID_DOUBLE
            } else {
                dist
            }
        }
    }

    /// C++: `MSLane * MSLink::getViaLane() const` [const]
    /// Returns the following inner lane
    // inline MSLane* getViaLane() const {
    //         return myInternalLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_lane(&self) -> Ptr<MSLane> {
        self.myInternalLane
    }

    /// C++: `const LinkLeaders MSLink::getLeaderInfo(const MSVehicle *, double, std::vector<const MSPerson *> *, bool) const` [const]
    /// Returns all potential link leaders (vehicles on foeLanes) Valid during the planMove() phase
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_leader_info(&self, ego: Ptr<MSVehicle>, dist: f64, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>, isShadowLink: bool) -> MSLink_LinkLeaders {
        let mut result: MSLink_LinkLeaders = Vec::new();
        if !ego.is_null() && (!self.from_internal_lane() || ego.get_lane_change_model_2().is_opposite()) {
            return result;
        }
        if g_compute_lc() && !ego.is_null() && ego.get_lane().is_normal() {
            let junctionEntry = self.get_lane_before().get_entry_link();
            if junctionEntry.have_red() && !ego.ignore_red(junctionEntry, true) && (!g_compute_lc() || junctionEntry.get_lane_before().get_bidi_lane().is_null()) {
                return result;
            }
        }
        let extraGap = if !ego.is_null() { ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_EXTRA_GAP, 0.) } else { 0. };
        let simStep = MSNet::get_instance().get_current_time_step();
        for i in 0..self.myFoeLanes.len() {
            let foeLane = self.myFoeLanes[i];
            let foeExitLink = foeLane.get_link_cont()[0];
            let mut distToCrossing = dist - self.myConflicts[i].get_length_behind_crossing(Ptr::new(self));
            let foeDistToCrossing = foeLane.get_length() - self.myConflicts[i].get_foe_length_behind_crossing(foeExitLink);
            let sameTarget = self.myLane == foeExitLink.get_lane() && !self.is_internal_junction_link() && !foeExitLink.is_internal_junction_link();
            let sameSource = !self.myInternalLaneBefore.is_null() && self.myInternalLaneBefore.get_normal_predecessor_lane() == foeLane.get_normal_predecessor_lane();
            let crossingWidth = if sameTarget || sameSource { 0. } else { self.myConflicts[i].conflictSize };
            let foeCrossingWidth = if sameTarget || sameSource { 0. } else { self.myConflicts[i].get_foe_conflict_size(foeExitLink) };
            let contLane = foeExitLink.get_via_lane_or_lane().get_edge().is_internal() && !(self.is_internal_junction_link() || self.is_exit_link_after_internal_junction());
            if distToCrossing + crossingWidth < 0. && !sameTarget && (ego.is_null() || !g_compute_lc() || distToCrossing + crossingWidth + ego.get_vehicle_type().get_length() < 0.) {
                // vehicle is behind the crossing point, continue with next foe lane
                continue;
            }
            let mut ignoreGreenCont = false;
            let mut foeIndirect = false;
            if contLane {
                let entry = self.get_lane_before().get_entry_link();
                let foeEntry = foeLane.get_entry_link();
                foeIndirect = foeEntry.myAmIndirect;
                if !entry.is_null() && entry.have_green() && !foeEntry.is_null() && foeEntry.have_green() && entry.myLaneBefore != foeEntry.myLaneBefore {
                    ignoreGreenCont = true;
                }
            }
            if foeIndirect && distToCrossing >= MSLink::NO_INTERSECTION {
                continue;
            }
            for leader in any_vehicles_of(foeLane) {
                let leaderBack = leader.get_back_position_on_lane(foeLane) - extraGap;
                let leaderBackDist = foeDistToCrossing - leaderBack;
                let l2 = if !ego.is_null() { ego.get_length() + 2. } else { 0. };
                let sagitta = if !ego.is_null() && self.myRadius != f64::MAX { self.myRadius - (self.myRadius * self.myRadius - 0.25 * l2 * l2).sqrt() } else { 0. };
                let pastTheCrossingPoint = leaderBackDist + foeCrossingWidth + sagitta < 0.;
                let enteredTheCrossingPoint = leaderBackDist < leader.get_vehicle_type().get_length();
                let foeIsBicycleTurn = leader.get_vehicle_type().get_vehicle_class() == SVC_BICYCLE && foeLane.get_incoming_lanes()[0].viaLink.get_direction() == LinkDirection::LEFT;
                let ignoreIndirectBicycleTurn = pastTheCrossingPoint && foeIsBicycleTurn;
                let cannotIgnore = ((contLane && !ignoreIndirectBicycleTurn) || sameTarget || (sameSource && !g_compute_lc())) && !ego.is_null();
                let inTheWay = (((!pastTheCrossingPoint && distToCrossing > 0.) || (sameTarget && distToCrossing > leaderBackDist - leader.get_length()))
                    && (enteredTheCrossingPoint || (sameSource && !enteredTheCrossingPoint && foeDistToCrossing < distToCrossing))
                    && (!(self.myConflicts[i].flag == MSLink_ConflictFlag::CONFLICT_DUMMY_MERGE) || foeIsBicycleTurn || sameSource))
                    || foeExitLink.get_lane_before().get_normal_predecessor_lane() == self.myLane.get_bidi_lane();
                let isOpposite = leader.get_lane_change_model_2().is_opposite();
                let avi = foeExitLink.get_approaching(msveh_to_sumo_vehicle(leader));
                let willPass = avi.willPass || (avi.arrivalTime == INVALID_TIME && sameTarget);
                if leader == ego {
                    continue;
                }
                if !inTheWay && ignoreGreenCont {
                    continue;
                }
                if (!g_compute_lc() || (!ego.is_null() && ego.get_lane() == foeLane) || (g_sublane() && !g_compute_lc()))
                    && distToCrossing < -POSITION_EPS
                    && !inTheWay
                    && (ego.is_null() || !g_compute_lc() || distToCrossing < -ego.get_vehicle_type().get_length())
                {
                    continue;
                }
                if !g_compute_lc() && sameSource && std::ptr::eq(ego.get_lane().get_edge(), self.myInternalLaneBefore.get_edge()) && leaderBack + leader.get_length() < ego.get_position_on_lane() - ego.get_length() {
                    continue;
                }
                if (!cannotIgnore || leader.is_stopped() || sameTarget)
                    && !willPass
                    && (avi.arrivalTime == INVALID_TIME || leader.get_speed() < SUMO_CONST_HALTING_SPEED)
                    && leader.is_front_on_lane(foeLane)
                    && !isOpposite
                    && !inTheWay
                    && !leader.will_stop()
                {
                    continue;
                }
                if leader.is_bidi_on(foeLane) {
                    continue;
                }
                let foeStrategicBlocked = leader.get_lane_change_model_2().is_strategic_blocked() && leader.get_car_follow_model().brake_gap(leader.get_speed()) <= foeLane.get_length() - leaderBack;
                let sameInternalEdge = std::ptr::eq(self.myInternalLaneBefore.get_edge(), foeExitLink.get_internal_lane_before().get_edge());
                let foeLaneIsBidi = self.myInternalLaneBefore.get_bidi_lane() == foeLane;
                if g_sublane() && !ego.is_null() && (sameSource || sameTarget || foeLaneIsBidi) && (!foeStrategicBlocked || sameInternalEdge) {
                    if ego.get_lane() == leader.get_lane() {
                        continue;
                    }
                    let egoLatOffset = if isShadowLink { ego.get_lat_offset(ego.m().get_lane_change_model().get_shadow_lane()) } else { 0. };
                    let posLat = ego.get_lateral_position_on_lane() + egoLatOffset;
                    let mut posLatLeader = leader.get_lateral_position_on_lane() + leader.get_lat_offset(foeLane);
                    if foeLaneIsBidi {
                        posLatLeader = foeLane.get_width() - posLatLeader;
                    }
                    let latGap = (posLat - posLatLeader).abs() - 0.5 * (ego.get_vehicle_type().get_width() + leader.get_vehicle_type().get_width());
                    let maneuverDist = leader.get_lane_change_model_2().get_maneuver_dist() * (if posLat < posLatLeader { -1. } else { 1. });
                    if latGap > 0. && (latGap > maneuverDist || !sameTarget || !g_compute_lc()) && (!g_compute_lc() || latGap > ego.get_vehicle_type().get_max_speed_lat()) {
                        let foeEntryLink = foeLane.get_incoming_lanes()[0].viaLink;
                        if sameSource {
                            let leaderFromRight = self.myIndex > foeEntryLink.get_index();
                            if (posLat > posLatLeader) == leaderFromRight {
                                continue;
                            }
                        } else if sameTarget {
                            if self.myDirection != foeEntryLink.get_direction() {
                                let mut leaderFromRight = (foeEntryLink.get_direction() as i32) < (self.myDirection as i32);
                                if g_lefthand() {
                                    leaderFromRight = !leaderFromRight;
                                }
                                if (posLat > posLatLeader) == leaderFromRight
                                    && (leader.get_lane_change_model_2().get_speed_lat() == 0. || leader.get_lane_change_model_2().get_maneuver_dist() == 0. || leaderFromRight == (leader.get_lane_change_model_2().get_speed_lat() < latGap))
                                    && (ego.get_lane_change_model_2().get_speed_lat() == 0. || ego.get_lane_change_model_2().get_maneuver_dist() == 0. || leaderFromRight == (ego.get_lane_change_model_2().get_speed_lat() > -latGap))
                                {
                                    continue;
                                }
                            }
                        } else {
                            continue;
                        }
                    }
                }
                if leader.get_waiting_time(false) < g_ignore_junction_blocker() {
                    let mut gap: f64;
                    let mut fromLeft = true;
                    if ego.is_null() {
                        gap = leaderBackDist;
                        distToCrossing += self.myConflicts[i].conflictSize / 2.;
                        if gap + foeCrossingWidth < 0. {
                            continue;
                        }
                        fromLeft = foeDistToCrossing > 0.5 * foeLane.get_length();
                    } else if (contLane && !sameSource && !ignoreIndirectBicycleTurn) || isOpposite {
                        // always break for vehicles which are on a continuation lane or for opposite-direction vehicles
                        gap = -f64::MAX;
                    } else {
                        if pastTheCrossingPoint && !sameTarget {
                            continue;
                        }
                        let mut leaderBackDist2 = leaderBackDist;
                        if sameTarget && leaderBackDist2 < 0. {
                            let mismatch = self.myConflicts[i].get_foe_length_behind_crossing(foeExitLink) - self.myConflicts[i].get_length_behind_crossing(Ptr::new(self));
                            if mismatch > 0. {
                                leaderBackDist2 += mismatch;
                            }
                        }
                        gap = distToCrossing - ego.get_vehicle_type().get_min_gap() - leaderBackDist2 - foeCrossingWidth;
                    }
                    let stopAsap = (if leader.is_front_on_lane(foeLane) { cannotIgnore } else { sameTarget || sameSource })
                        || (!ego.is_null() && ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_ADVANCE, 1.0) == 0.0);
                    if MSLink::ignore_foe(msveh_to_traffic_object(ego), msveh_to_traffic_object(leader)) {
                        continue;
                    }
                    let llFlags = (if fromLeft { MSLink_LinkLeaderFlag::LL_FROM_LEFT as i32 } else { 0 })
                        | (if inTheWay { MSLink_LinkLeaderFlag::LL_IN_THE_WAY as i32 } else { 0 })
                        | (if sameSource { MSLink_LinkLeaderFlag::LL_SAME_SOURCE as i32 } else { 0 })
                        | (if sameTarget { MSLink_LinkLeaderFlag::LL_SAME_TARGET as i32 } else { 0 });
                    result.push(MSLink_LinkLeader::new(leader, gap, if stopAsap { -1. } else { distToCrossing }, llFlags, leader.get_lat_offset(foeLane)));
                }
            }
            if !ego.is_null() && MSNet::get_instance().has_persons() {
                let distToPeds = distToCrossing - ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_STOPLINE_CROSSING_GAP, MSPMODEL_SAFETY_GAP);
                let vehWidth = ego.get_vehicle_type().get_width() + MSPMODEL_SAFETY_GAP;
                let wayIn = self.myConflicts[i].lengthBehindCrossing < self.myLaneBefore.get_length() * 0.5;
                let vehCenter = foeDistToCrossing + self.myLaneBefore.get_width() * 0.5 + ego.get_lateral_position_on_lane() * (if wayIn { -1. } else { 1. });
                // C++ short-circuits: blockedAtDist is only called when distToPeds >= -SAFETY_GAP
                let blocked = if distToPeds >= -MSPMODEL_SAFETY_GAP {
                    let movementModel = MSNet::get_instance().m().get_person_control().get_movement_model();
                    movementModel.m().blocked_at_dist(
                        msveh_to_traffic_object(ego),
                        foeLane,
                        vehCenter,
                        vehWidth,
                        ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_CROSSING_GAP, 10.),
                        collectBlockers,
                    )
                } else {
                    false
                };
                if blocked {
                    result.push(MSLink_LinkLeader::new(Ptr::null(), -1., distToPeds, MSLink_LinkLeaderFlag::LL_FROM_LEFT as i32, 0.));
                } else if foeLane.is_crossing() && ego.get_lane().is_internal() && ego.get_lane().get_edge().get_to_junction() == self.myJunction {
                    let crossingLink = foeLane.get_incoming_lanes()[0].viaLink;
                    if distToCrossing > 0. && crossingLink.have_priority() && !crossingLink.myApproachingPersons.is_null() {
                        let timeToEnterCrossing = distToCrossing / max2(ego.get_speed(), 1.0);
                        for (&person, item) in crossingLink.myApproachingPersons.iter() {
                            if !MSLink::ignore_foe(msveh_to_traffic_object(ego), person_to_traffic_object(person)) && timeToEnterCrossing > steps2time(item.arrivalTime - simStep) {
                                result.push(MSLink_LinkLeader::new(Ptr::null(), -1., distToPeds, MSLink_LinkLeaderFlag::LL_FROM_LEFT as i32, 0.));
                                break;
                            }
                        }
                    }
                }
            }
        }
        if !ego.is_null() {
            self.check_walking_area_foe(ego, self.myWalkingAreaFoe, collectBlockers, &mut result);
            self.check_walking_area_foe(ego, self.myWalkingAreaFoeExit, collectBlockers, &mut result);
        }
        if g_lateral_resolution() > 0. && !ego.is_null() && !isShadowLink {
            for &foeLane in self.mySublaneFoeLanes.iter() {
                for leader in any_vehicles_of(foeLane) {
                    if leader == ego {
                        continue;
                    }
                    if leader.get_lane().is_normal() {
                        continue;
                    }
                    let maxLength = max2(self.myInternalLaneBefore.get_length(), foeLane.get_length());
                    let gap = dist - maxLength - ego.get_vehicle_type().get_min_gap() + leader.get_back_position_on_lane(foeLane) - extraGap;
                    if gap < -(ego.get_vehicle_type().get_min_gap() + leader.get_length()) {
                        continue;
                    }
                    let posLat = ego.get_lateral_position_on_lane();
                    let posLatLeader = leader.get_lateral_position_on_lane() + leader.get_lat_offset(foeLane);
                    if (posLat < posLatLeader && self.myInternalLaneBefore.get_index() > foeLane.get_index()) || (posLat > posLatLeader && self.myInternalLaneBefore.get_index() < foeLane.get_index()) {
                        if MSLink::ignore_foe(msveh_to_traffic_object(ego), msveh_to_traffic_object(leader)) {
                            continue;
                        }
                        result.push(MSLink_LinkLeader::new(leader, gap, -1., MSLink_LinkLeaderFlag::LL_SAME_SOURCE as i32, 0.));
                    }
                }
            }
        }
        result
    }

    /// C++: `double MSLink::getZipperSpeed(const MSVehicle *, const double, double, SUMOTime, const BlockingFoes *) const` [const]
    /// return the speed at which ego vehicle must approach the zipper link
    // double
    // MSLink::getZipperSpeed(const MSVehicle* ego, const double dist, double vSafe,
    //                        SUMOTime arrivalTime,
    //                        const BlockingFoes* foes) const {
    //     if (myFoeLinks.size() == 0) {
    //         // link should have LINKSTATE_MAJOR in this case
    //         assert(false);
    //         return vSafe;
    //     }
    //     const double brakeGap = ego->getCarFollowModel().brakeGap(vSafe, ego->getCarFollowModel().getMaxDecel(), TS);
    //     if (dist > MAX2(myFoeVisibilityDistance, brakeGap)) {
    // #ifdef DEBUG_ZIPPER
    //         const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //         DEBUGOUT(DEBUG_COND_ZIPPER, SIMTIME << " getZipperSpeed ego=" << ego->getID()
    //                  << " dist=" << dist << " bGap=" << brakeGap << " ignoring foes (arrival in " << STEPS2TIME(arrivalTime - now) << ")\n")
    // #endif
    //         return vSafe;
    //     }
    // #ifdef DEBUG_ZIPPER
    //     DEBUGOUT(DEBUG_COND_ZIPPER, SIMTIME << " getZipperSpeed ego=" << ego->getID()
    //              << " egoAT=" << arrivalTime
    //              << " dist=" << dist
    //              << " brakeGap=" << brakeGap
    //              << " vSafe=" << vSafe
    //              << " numFoes=" << foes->size()
    //              << "\n")
    // #endif
    //     const bool uniqueFoeLink = myFoeLinks.size() == 1;
    //     MSLink* foeLink = myFoeLinks[0];
    //     for (const auto& item : *foes) {
    //         if (!item->isVehicle()) {
    //             continue;
    //         }
    //         const MSVehicle* foe = dynamic_cast<const MSVehicle*>(item);
    //         assert(foe != 0);
    //         const ApproachingVehicleInformation* aviPtr = nullptr;
    //         if (uniqueFoeLink) {
    //             aviPtr = foeLink->getApproachingPtr(foe);
    //         } else {
    //             // figure out which link is approached by the current foe
    //             for (MSLink* fl : myFoeLinks) {
    //                 aviPtr = fl->getApproachingPtr(foe);
    //                 if (aviPtr != nullptr) {
    //                     break;
    //                 }
    //             }
    //         }
    //         if (aviPtr == nullptr) {
    //             continue;
    //         }
    //         const ApproachingVehicleInformation& avi = *aviPtr;
    //         const double foeDist = (foe->isActive() ? avi.dist : MAX2(0.0, avi.dist -
    //                                 STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - foe->getLastActionTime()) * avi.speed));
    //
    //         if (    // ignore vehicles that arrive after us (unless they are ahead and we could easily brake for them)
    //             ((avi.arrivalTime > arrivalTime) && !couldBrakeForLeader(dist, foeDist, ego, foe)) ||
    //             // also ignore vehicles that are behind us and are able to brake for us
    //             couldBrakeForLeader(foeDist, dist, foe, ego) ||
    //             // resolve ties by lane index
    //             (avi.arrivalTime == arrivalTime && foeDist == dist && ego->getLane()->getIndex() < foe->getLane()->getIndex())) {
    // #ifdef DEBUG_ZIPPER
    //             if (DEBUG_COND_ZIPPER) std::cout
    //                         << "    ignoring foe=" << foe->getID()
    //                         << " foeAT=" << avi.arrivalTime
    //                         << " foeDist=" << avi.dist
    //                         << " foeDist2=" << foeDist
    //                         << " foeSpeed=" << avi.speed
    //                         << " egoSpeed=" << ego->getSpeed()
    //                         << " deltaDist=" << foeDist - dist
    //                         << " delteSpeed=" << avi.speed - foe->getCarFollowModel().getMaxDecel() - ego->getSpeed()
    //                         << " egoCouldBrake=" << couldBrakeForLeader(dist, foeDist, ego, foe)
    //                         << " foeCouldBrake=" << couldBrakeForLeader(foeDist, dist, foe, ego)
    //                         << "\n";
    // #endif
    //             continue;
    //         }
    //         // the idea behind speed adaption is three-fold:
    //         // 1) ego needs to be in a car-following relationship with foe eventually
    //         //    thus, the ego speed should be equal to the follow speed once the foe enters
    //         //    the zipper junction
    //         // 2) ego vehicle needs to put a certain distance beteen himself and foe (safeGap)
    //         //    achieving this distance can be spread over time but computing
    //         //    safeGap is subject to estimation errors of future speeds
    //         // 3) deceleration can be spread out over the time until true
    //         //    car-following happens, at the start of speed adaptions, smaller
    //         //    decelerations should be sufficient
    //
    //         // we cannot trust avi.arrivalSpeed if the foe has leader vehicles that are accelerating
    //         // lets try to extrapolate
    //         const double uMax = foe->getLane()->getVehicleMaxSpeed(foe);
    //         const double uAccel = foe->getCarFollowModel().estimateSpeedAfterDistance(foeDist, avi.speed, foe->getCarFollowModel().getMaxAccel());
    //         const double uEnd = MIN2(uMax, uAccel);
    //         const double uAvg = (avi.speed + uEnd) / 2;
    //         const double tf0 = foeDist / MAX2(NUMERICAL_EPS, uAvg);
    //         const double tf = MAX2(1.0, ceil((tf0) / TS) * TS);
    //
    //         const double vMax = ego->getLane()->getVehicleMaxSpeed(ego);
    //         const double vAccel = ego->getCarFollowModel().estimateSpeedAfterDistance(dist, ego->getSpeed(), ego->getCarFollowModel().getMaxAccel());
    //         const double vDecel = ego->getCarFollowModel().estimateSpeedAfterDistance(dist, ego->getSpeed(), -ego->getCarFollowModel().getMaxDecel());
    //         const double vEnd = MIN3(vMax, vAccel, MAX2(uEnd, vDecel));
    //         const double vAvg = (ego->getSpeed() + vEnd) / 2;
    //         const double te0 = dist / MAX2(NUMERICAL_EPS, vAvg);
    //         const double te = MAX2(1.0, ceil((te0) / TS) * TS);
    //
    //         const double tTarget = tf + ego->getCarFollowModel().getHeadwayTime();
    //         const double a = ego->getCarFollowModel().avoidArrivalAccel(dist, tTarget, vSafe, ego->getCarFollowModel().getMaxDecel());
    //
    //         const double gap = dist - foe->getVehicleType().getLength() - ego->getVehicleType().getMinGap() - foeDist;
    //         const double vFollow = ego->getCarFollowModel().followSpeed(
    //                                    ego, ego->getSpeed(), gap, avi.speed, foe->getCarFollowModel().getMaxDecel(), foe);
    //         const double vSafeGap = MAX2(vFollow, ego->getSpeed() + ACCEL2SPEED(a));
    //
    //         // scale behavior based on ego time to link (te)
    //         const double w = MIN2(1.0, te / 10);
    //         const double maxDecel = w * ego->getCarFollowModel().getMaxDecel() + (1 - w) * ego->getCarFollowModel().getEmergencyDecel();
    //         const double vZipper = MAX3(vFollow, ego->getSpeed() - ACCEL2SPEED(maxDecel), vSafeGap);
    //
    //         vSafe = MIN2(vSafe, vZipper);
    // #ifdef DEBUG_ZIPPER
    //         if (DEBUG_COND_ZIPPER) std::cout << "    adapting to foe=" << foe->getID()
    //                                              << " foeDist=" << foeDist
    //                                              << " foeSpeed=" << avi.speed
    //                                              << " foeAS=" << avi.arrivalSpeed
    //                                              << " egoSpeed=" << ego->getSpeed()
    //                                              << " uMax=" << uMax
    //                                              << " uAccel=" << uAccel
    //                                              << " uEnd=" << uEnd
    //                                              << " uAvg=" << uAvg
    //                                              << " gap=" << gap
    //                                              << "\n      "
    //                                              << " tf=" << tf
    //                                              << " te=" << te
    //                                              << " aSafeGap=" << a
    //                                              << " vMax=" << vMax
    //                                              << " vAccel=" << vAccel
    //                                              << " vDecel=" << vDecel
    //                                              << " vEnd=" << vEnd
    //                                              << " vSafeGap=" << vSafeGap
    //                                              << " vFollow=" << vFollow
    //                                              << " w=" << w
    //                                              << " maxDecel=" << maxDecel
    //                                              << " vZipper=" << vZipper
    //                                              << " vSafe=" << vSafe
    //                                              << "\n";
    // #endif
    //     }
    //     return vSafe;
    // }
    #[allow(unused_variables)]
    pub fn get_zipper_speed(&self, ego: Ptr<MSVehicle>, dist: f64, vSafe: f64, arrivalTime: SUMOTime, foes: Ptr<MSLink_BlockingFoes>) -> f64 {
        if self.myFoeLinks.len() == 0 {
            debug_assert!(false);
            return vSafe;
        }
        let mut vSafe = vSafe;
        let brakeGap = ego.get_car_follow_model().brake_gap_2(vSafe, ego.get_car_follow_model().get_max_decel(), ts());
        if dist > max2(self.myFoeVisibilityDistance, brakeGap) {
            return vSafe;
        }
        let uniqueFoeLink = self.myFoeLinks.len() == 1;
        let foeLink = self.myFoeLinks[0];
        for &item in foes.iter() {
            let foe = match zipper_foe_as_vehicle(item) {
                Some(foe) => foe,
                None => continue,
            };
            let footSumo = msveh_to_sumo_vehicle(foe);
            let aviPtr = if uniqueFoeLink {
                foeLink.get_approaching_ptr(footSumo)
            } else {
                let mut found: Ptr<MSLink_ApproachingVehicleInformation> = Ptr::null();
                for &fl in self.myFoeLinks.iter() {
                    found = fl.get_approaching_ptr(footSumo);
                    if !found.is_null() {
                        break;
                    }
                }
                found
            };
            if aviPtr.is_null() {
                continue;
            }
            let avi = aviPtr;
            let foeDist = if foe.is_active() {
                avi.dist
            } else {
                max2(0.0, avi.dist - steps2time(MSNet::get_instance().get_current_time_step() - foe.get_last_action_time()) * avi.speed)
            };
            // ignore vehicles that arrive after us (unless they are ahead and we could easily brake for them)
            if ((avi.arrivalTime > arrivalTime) && !MSLink::could_brake_for_leader(dist, foeDist, ego, foe))
                || MSLink::could_brake_for_leader(foeDist, dist, foe, ego)
                || (avi.arrivalTime == arrivalTime && foeDist == dist && ego.get_lane().get_index() < foe.get_lane().get_index())
            {
                continue;
            }
            let uMax = foe.get_lane().get_vehicle_max_speed(foe.cast::<SUMOTrafficObject>());
            let uAccel = foe.get_car_follow_model().estimate_speed_after_distance(foeDist, avi.speed, foe.get_car_follow_model().get_max_accel());
            let uEnd = min2(uMax, uAccel);
            let uAvg = (avi.speed + uEnd) / 2.;
            let tf0 = foeDist / max2(NUMERICAL_EPS, uAvg);
            let tf = max2(1.0, (tf0 / ts()).ceil() * ts());
            let vMax = ego.get_lane().get_vehicle_max_speed(ego.cast::<SUMOTrafficObject>());
            let vAccel = ego.get_car_follow_model().estimate_speed_after_distance(dist, ego.get_speed(), ego.get_car_follow_model().get_max_accel());
            let vDecel = ego.get_car_follow_model().estimate_speed_after_distance(dist, ego.get_speed(), -ego.get_car_follow_model().get_max_decel());
            let vEnd = min2(max2(uEnd, vDecel), min2(vMax, vAccel));
            let vAvg = (ego.get_speed() + vEnd) / 2.;
            let te0 = dist / max2(NUMERICAL_EPS, vAvg);
            let te = max2(1.0, (te0 / ts()).ceil() * ts());
            let tTarget = tf + ego.get_car_follow_model().get_headway_time();
            let a = MSCFModel::avoid_arrival_accel(dist, tTarget, vSafe, ego.get_car_follow_model().get_max_decel());
            let gap = dist - foe.get_vehicle_type().get_length() - ego.get_vehicle_type().get_min_gap() - foeDist;
            let vFollow = ego.get_car_follow_model().follow_speed(ego, ego.get_speed(), gap, avi.speed, foe.get_car_follow_model().get_max_decel(), foe, MSCFModel_CalcReason::CURRENT);
            let vSafeGap = max2(vFollow, ego.get_speed() + accel2speed(a));
            let w = min2(1.0, te / 10.);
            let maxDecel = w * ego.get_car_follow_model().get_max_decel() + (1. - w) * ego.get_car_follow_model().get_emergency_decel();
            let vZipper = max3(vFollow, ego.get_speed() - accel2speed(maxDecel), vSafeGap);
            vSafe = min2(vSafe, vZipper);
        }
        vSafe
    }

    /// C++: `MSLane * MSLink::getViaLaneOrLane() const` [const]
    /// return the via lane if it exists and the lane otherwise
    // inline MSLane* getViaLaneOrLane() const {
    //         return  myInternalLane != nullptr ? myInternalLane : myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_lane_or_lane(&self) -> Ptr<MSLane> {
        if self.myInternalLane.is_null() { self.myLane } else { self.myInternalLane }
    }

    /// C++: `const MSLane * MSLink::getLaneBefore() const` [const]
    /// return the internalLaneBefore if it exists and the laneBefore otherwise
    // inline const MSLane* getLaneBefore() const {
    //         assert(myInternalLaneBefore == nullptr || myLaneBefore == myInternalLaneBefore);  // lane before mismatch!
    //         return myLaneBefore;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_before(&self) -> Ptr<MSLane> {
        self.myLaneBefore
    }

    /// C++: `const MSLane * MSLink::getInternalLaneBefore() const` [const]
    /// return myInternalLaneBefore (always 0 when compiled without internal lanes)
    // inline const MSLane* getInternalLaneBefore() const {
    //         return myInternalLaneBefore;
    //     }
    #[allow(unused_variables)]
    pub fn get_internal_lane_before(&self) -> Ptr<MSLane> {
        self.myInternalLaneBefore
    }

    /// C++: `SUMOTime MSLink::getLeaveTime(const SUMOTime, const double, const double, const double) const` [const]
    /// return the expected time at which the given vehicle will clear the link
    // SUMOTime
    // MSLink::getLeaveTime(const SUMOTime arrivalTime, const double arrivalSpeed,
    //                      const double leaveSpeed, const double vehicleLength) const {
    //     return arrivalTime == SUMOTime_MAX ? SUMOTime_MAX : arrivalTime + TIME2STEPS((getLength() + vehicleLength) / MAX2(0.5 * (arrivalSpeed + leaveSpeed), NUMERICAL_EPS));
    // }
    #[allow(unused_variables)]
    pub fn get_leave_time(&self, arrivalTime: SUMOTime, arrivalSpeed: f64, leaveSpeed: f64, vehicleLength: f64) -> SUMOTime {
        if arrivalTime == SUMOTIME_MAX {
            SUMOTIME_MAX
        } else {
            arrivalTime + time2steps((self.get_length() + vehicleLength) / max2(0.5 * (arrivalSpeed + leaveSpeed), NUMERICAL_EPS))
        }
    }

    /// C++: `void MSLink::writeApproaching(OutputDevice &, const std::string) const` [const]
    /// write information about all approaching vehicles to the given output device
    // void
    // MSLink::writeApproaching(OutputDevice& od, const std::string fromLaneID) const {
    //     if (myApproachingVehicles.size() > 0) {
    //         od.openTag("link");
    //         od.writeAttr(SUMO_ATTR_FROM, fromLaneID);
    //         const std::string via = getViaLane() == nullptr ? "" : getViaLane()->getID();
    //         od.writeAttr(SUMO_ATTR_VIA, via);
    //         od.writeAttr(SUMO_ATTR_TO, getLane() == nullptr ? "" : getLane()->getID());
    //         std::vector<std::pair<SUMOTime, const SUMOVehicle*> > toSort; // stabilize output
    //         for (auto it : myApproachingVehicles) {
    //             toSort.push_back(std::make_pair(it.second.arrivalTime, it.first));
    //         }
    //         std::sort(toSort.begin(), toSort.end());
    //         for (std::vector<std::pair<SUMOTime, const SUMOVehicle*> >::const_iterator it = toSort.begin(); it != toSort.end(); ++it) {
    //             od.openTag("approaching");
    //             const ApproachingVehicleInformation& avi = myApproachingVehicles.find(it->second)->second;
    //             od.writeAttr(SUMO_ATTR_ID, it->second->getID());
    //             od.writeAttr(SUMO_ATTR_IMPATIENCE, it->second->getImpatience());
    //             od.writeAttr("arrivalTime", time2string(avi.arrivalTime));
    //             od.writeAttr("leaveTime", time2string(avi.leavingTime));
    //             od.writeAttr("arrivalSpeed", toString(avi.arrivalSpeed));
    //             od.writeAttr("arrivalSpeedBraking", toString(avi.arrivalSpeedBraking));
    //             od.writeAttr("leaveSpeed", toString(avi.leaveSpeed));
    //             od.writeAttr("willPass", toString(avi.willPass));
    //             od.closeTag();
    //         }
    //         od.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_approaching(&self, od: &mut OutputDevice, fromLaneID: String) {
        if self.myApproachingVehicles.len() > 0 {
            od.open_tag("link");
            od.write_attr(SUMO_ATTR_FROM, &fromLaneID);
            let via = if self.get_via_lane().is_null() { String::new() } else { self.get_via_lane().get_id().to_string() };
            od.write_attr(SUMO_ATTR_VIA, &via);
            od.write_attr(SUMO_ATTR_TO, &(if self.get_lane().is_null() { String::new() } else { self.get_lane().get_id().to_string() }));
            // stabilize output: sort by arrival time, then by vehicle address (as std::pair does)
            let mut toSort: Vec<(SUMOTime, Ptr<SUMOVehicle>)> = self.myApproachingVehicles.iter().map(|(&veh, it)| (it.arrivalTime, veh)).collect();
            toSort.sort();
            for &(_, veh) in toSort.iter() {
                od.open_tag("approaching");
                let avi = self.myApproachingVehicles.get(&veh).unwrap();
                od.write_attr(SUMO_ATTR_ID, &veh.get_id().to_string());
                od.write_attr(SUMO_ATTR_IMPATIENCE, &veh.get_impatience());
                od.write_attr("arrivalTime", &time2string(avi.arrivalTime));
                od.write_attr("leaveTime", &time2string(avi.leavingTime));
                od.write_attr("arrivalSpeed", &f64_to_string(avi.arrivalSpeed));
                od.write_attr("arrivalSpeedBraking", &f64_to_string(avi.arrivalSpeedBraking));
                od.write_attr("leaveSpeed", &f64_to_string(avi.leaveSpeed));
                od.write_attr("willPass", &avi.willPass);
                od.close_tag("");
            }
            od.close_tag("");
        }
    }

    /// C++: `MSLink * MSLink::getParallelLink(int) const` [const]
    /// return the link that is parallel to this link or 0
    // MSLink*
    // MSLink::getParallelLink(int direction) const {
    //     if (direction == -1) {
    //         return myParallelRight;
    //     } else if (direction == 1) {
    //         return myParallelLeft;
    //     } else {
    //         assert(false || myLane->getOpposite() != nullptr || MSGlobals::gComputeLC);
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parallel_link(&self, direction: i32) -> Ptr<MSLink> {
        if direction == -1 {
            self.myParallelRight
        } else if direction == 1 {
            self.myParallelLeft
        } else {
            // C++ asserts an opposite lane (or gComputeLC) here; not checked
            Ptr::null()
        }
    }

    /// C++: `MSLink * MSLink::getOppositeDirectionLink() const` [const]
    /// return the link that is the opposite entry link to this one
    // MSLink*
    // MSLink::getOppositeDirectionLink() const {
    //     if (myLane->getOpposite() != nullptr && myLaneBefore->getOpposite() != nullptr) {
    //         for (MSLink* cand : myLane->getOpposite()->getLinkCont()) {
    //             if (cand->getLane() == myLaneBefore->getOpposite()) {
    //                 return cand;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_opposite_direction_link(&self) -> Ptr<MSLink> {
        if !self.myLane.get_opposite().is_null() && !self.myLaneBefore.get_opposite().is_null() {
            for &cand in self.myLane.get_opposite().get_link_cont().iter() {
                if cand.get_lane() == self.myLaneBefore.get_opposite() {
                    return cand;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `bool MSLink::fromInternalLane() const` [const]
    /// return whether the fromLane of this link is an internal lane
    // inline bool fromInternalLane() const {
    //         return myInternalLaneBefore != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn from_internal_lane(&self) -> bool {
        !self.myInternalLaneBefore.is_null()
    }

    /// C++: `bool MSLink::isEntryLink() const` [const]
    /// return whether the toLane of this link is an internal lane and fromLane is a normal lane
    // bool
    // MSLink::isEntryLink() const {
    //     if (MSGlobals::gUsingInternalLanes) {
    //         return myInternalLane != nullptr && myInternalLaneBefore == nullptr;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_entry_link(&self) -> bool {
        if g_using_internal_lanes() {
            !self.myInternalLane.is_null() && self.myInternalLaneBefore.is_null()
        } else {
            false
        }
    }

    /// C++: `bool MSLink::isConflictEntryLink() const` [const]
    /// return whether this link enters the conflict area (not a continuation link)
    // bool
    // MSLink::isConflictEntryLink() const {
    //     // either a non-cont entry link or the link after a cont-link
    //     return !myAmCont && (isEntryLink() || (myInternalLaneBefore != nullptr && myInternalLane != nullptr));
    // }
    #[allow(unused_variables)]
    pub fn is_conflict_entry_link(&self) -> bool {
        !self.myAmCont && (self.is_entry_link() || (!self.myInternalLaneBefore.is_null() && !self.myInternalLane.is_null()))
    }

    /// C++: `bool MSLink::isExitLink() const` [const]
    /// return whether the fromLane of this link is an internal lane and toLane is a normal lane
    // bool
    // MSLink::isExitLink() const {
    //     if (MSGlobals::gUsingInternalLanes) {
    //         return myInternalLaneBefore != nullptr && myInternalLane == nullptr;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_exit_link(&self) -> bool {
        if g_using_internal_lanes() {
            !self.myInternalLaneBefore.is_null() && self.myInternalLane.is_null()
        } else {
            false
        }
    }

    /// C++: `bool MSLink::isExitLinkAfterInternalJunction() const` [const]
    /// return whether the fromLane of this link is an internal lane and its incoming lane is also an internal lane
    // bool
    // MSLink::isExitLinkAfterInternalJunction() const {
    //     if (MSGlobals::gUsingInternalLanes) {
    //         return (getInternalLaneBefore() != nullptr
    //                 && myInternalLaneBefore->getIncomingLanes().size() == 1
    //                 && myInternalLaneBefore->getIncomingLanes().front().viaLink->isInternalJunctionLink());
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_exit_link_after_internal_junction(&self) -> bool {
        if g_using_internal_lanes() {
            !self.get_internal_lane_before().is_null()
                && self.myInternalLaneBefore.get_incoming_lanes().len() == 1
                && self.myInternalLaneBefore.get_incoming_lanes()[0].viaLink.is_internal_junction_link()
        } else {
            false
        }
    }

    /// C++: `const MSLink * MSLink::getCorrespondingExitLink() const` [const]
    /// returns the corresponding exit link for entryLinks to a junction.
    // const MSLink*
    // MSLink::getCorrespondingExitLink() const {
    //     MSLane* lane = myInternalLane;
    //     const MSLink* link = this;
    //     while (lane != nullptr) {
    //         link = lane->getLinkCont()[0];
    //         lane = link->getViaLane();
    //     }
    //     return link;
    // }
    #[allow(unused_variables)]
    pub fn get_corresponding_exit_link(&self) -> Ptr<MSLink> {
        let mut lane = self.myInternalLane;
        let mut link: Ptr<MSLink> = Ptr::new(self);
        while !lane.is_null() {
            link = lane.get_link_cont()[0];
            lane = link.get_via_lane();
        }
        link
    }

    /// C++: `const MSLink * MSLink::getCorrespondingEntryLink() const` [const]
    /// returns the corresponding entry link for exitLinks to a junction.
    // const MSLink*
    // MSLink::getCorrespondingEntryLink() const {
    //     const MSLink* link = this;
    //     while (link->myLaneBefore->isInternal()) {
    //         assert(myLaneBefore->getIncomingLanes().size() == 1);
    //         link = link->myLaneBefore->getIncomingLanes().front().viaLink;
    //     }
    //     return link;
    // }
    #[allow(unused_variables)]
    pub fn get_corresponding_entry_link(&self) -> Ptr<MSLink> {
        let mut link: Ptr<MSLink> = Ptr::new(self);
        while link.myLaneBefore.is_internal() {
            debug_assert!(self.myLaneBefore.get_incoming_lanes().len() == 1);
            link = link.myLaneBefore.get_incoming_lanes()[0].viaLink;
        }
        link
    }

    /// C++: `bool MSLink::isInternalJunctionLink() const` [const]
    /// return whether the fromLane and the toLane of this link are internal lanes
    // bool
    // MSLink::isInternalJunctionLink() const {
    //     return getInternalLaneBefore() != nullptr && myInternalLane != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn is_internal_junction_link(&self) -> bool {
        !self.get_internal_lane_before().is_null() && !self.myInternalLane.is_null()
    }

    /// C++: `SUMOTime MSLink::getMesoTLSPenalty() const` [const]
    /// Returns the time penalty for passing a tls-controlled link (meso)
    // SUMOTime getMesoTLSPenalty() const {
    //         return myMesoTLSPenalty;
    //     }
    #[allow(unused_variables)]
    pub fn get_meso_tls_penalty(&self) -> SUMOTime {
        self.myMesoTLSPenalty
    }

    /// C++: `double MSLink::getGreenFraction() const` [const]
    /// Returns the average proportion of green time to cycle time
    // double getGreenFraction() const {
    //         return myGreenFraction;
    //     }
    #[allow(unused_variables)]
    pub fn get_green_fraction(&self) -> f64 {
        self.myGreenFraction
    }

    /// C++: `void MSLink::setMesoTLSPenalty(const SUMOTime)`
    /// Sets the time penalty for passing a tls-controlled link (meso)
    // void setMesoTLSPenalty(const SUMOTime penalty) {
    //         myMesoTLSPenalty = penalty;
    //     }
    #[allow(unused_variables)]
    pub fn set_meso_tls_penalty(&mut self, penalty: SUMOTime) {
        self.myMesoTLSPenalty = penalty;
    }

    /// C++: `void MSLink::setGreenFraction(const double)`
    /// Sets the green fraction for passing a tls-controlled link (meso)
    // void setGreenFraction(const double fraction) {
    //         myGreenFraction = fraction;
    //     }
    #[allow(unused_variables)]
    pub fn set_green_fraction(&mut self, fraction: f64) {
        self.myGreenFraction = fraction;
    }

    /// C++: `const std::vector<const MSLane *> & MSLink::getFoeLanes() const` [const]
    // const std::vector<const MSLane*>& getFoeLanes() const {
    //         return myFoeLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_lanes(&self) -> &Vec<Ptr<MSLane>> {
        &self.myFoeLanes
    }

    /// C++: `const std::vector<ConflictInfo> & MSLink::getConflicts() const` [const]
    // const std::vector<ConflictInfo>& getConflicts() const {
    //         return myConflicts;
    //     }
    #[allow(unused_variables)]
    pub fn get_conflicts(&self) -> &Vec<MSLink_ConflictInfo> {
        &self.myConflicts
    }

    /// C++: `const std::vector<MSLink *> & MSLink::getFoeLinks() const` [const]
    // const std::vector<MSLink*>& getFoeLinks() const {
    //         return myFoeLinks;
    //     }
    #[allow(unused_variables)]
    pub fn get_foe_links(&self) -> &Vec<Ptr<MSLink>> {
        &self.myFoeLinks
    }

    /// C++: `SVCPermissions MSLink::getPermissions() const` [const]
    /// who may use this link
    // SVCPermissions getPermissions() const {
    //         return myPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        self.myPermissions
    }

    /// C++: `void MSLink::updatePermissions()`
    // void
    // MSLink::updatePermissions() {
    //     // we only ever increase permission because transient permission reductions lead to invalid bestLanes assignment otherwise
    //     myPermissions |= myLaneBefore->getPermissions() & myLane->getPermissions() & (myInternalLane == nullptr ? SVCAll : myInternalLane->getPermissions());
    // }
    #[allow(unused_variables)]
    pub fn update_permissions(&mut self) {
        // we only ever increase permission because transient permission reductions lead to invalid bestLanes assignment otherwise
        self.myPermissions |= self.myLaneBefore.get_permissions() & self.myLane.get_permissions() & (if self.myInternalLane.is_null() { SVC_ALL } else { self.myInternalLane.get_permissions() });
    }

    /// C++: `void MSLink::initParallelLinks()`
    /// initialize parallel links (to be called after all links are loaded)
    // void
    // MSLink::initParallelLinks() {
    //     myParallelRight = computeParallelLink(-1);
    //     myParallelLeft = computeParallelLink(1);
    // }
    #[allow(unused_variables)]
    pub fn init_parallel_links(&mut self) {
        self.myParallelRight = self.compute_parallel_link(-1);
        self.myParallelLeft = self.compute_parallel_link(1);
    }

    /// C++: `double MSLink::getLateralShift() const` [const]
    /// return lateral shift that must be applied when passing this link
    // inline double getLateralShift() const {
    //         return myLateralShift;
    //     }
    #[allow(unused_variables)]
    pub fn get_lateral_shift(&self) -> f64 {
        self.myLateralShift
    }

    /// C++: `std::string MSLink::getDescription() const` [const]
    /// get string description for this link
    // std::string
    // MSLink::getDescription() const {
    //     return myLaneBefore->getID() + "->" + getViaLaneOrLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> String {
        format!("{}->{}", self.myLaneBefore.get_id(), self.get_via_lane_or_lane().get_id())
    }

    /// C++: `std::pair<const SUMOVehicle *const, const ApproachingVehicleInformation> MSLink::getClosest() const` [const]
    /// get the closest vehicle approaching this link
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_closest(&self) -> (Ptr<SUMOVehicle>, MSLink_ApproachingVehicleInformation) {
        // C++ asserts a non-empty approach list; an empty map is undefined behaviour in C++
        let mut closestIt = self.myApproachingVehicles.iter().next().expect("MSLink::getClosest on empty approach list");
        let mut minDist = f64::MAX;
        for (veh, it) in self.myApproachingVehicles.iter() {
            if it.dist < minDist {
                minDist = it.dist;
                closestIt = (veh, it);
            }
        }
        let (&veh, it) = closestIt;
        (veh, MSLink_ApproachingVehicleInformation::new(it.arrivalTime, it.leavingTime, it.arrivalSpeed, it.leaveSpeed, it.willPass, it.arrivalSpeedBraking, it.waitingTime, it.dist, it.speed, it.latOffset))
    }

    /// C++: `bool MSLink::hasFoeCrossing() const` [const]
    // inline bool hasFoeCrossing() const {
    //         return myHavePedestrianCrossingFoe;
    //     }
    #[allow(unused_variables)]
    pub fn has_foe_crossing(&self) -> bool {
        self.myHavePedestrianCrossingFoe
    }

    /// C++: `bool MSLink::railSignalWasPassed() const` [const]
    /// whether this link is for a railsignal that was passed in this step
    // bool
    // MSLink::railSignalWasPassed() const {
    //     if (myJunction != nullptr && myJunction->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         for (const auto& item : myApproachingVehicles) {
    //             if (item.second.dist < SPEED2DIST(item.first->getSpeed())) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn rail_signal_was_passed(&self) -> bool {
        if !self.myJunction.is_null() && self.myJunction.get_type() == SumoXMLNodeType::RAIL_SIGNAL {
            for (&veh, item) in self.myApproachingVehicles.iter() {
                if item.dist < speed2dist(veh.get_speed()) {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void MSLink::recheckSetRequestInformation()` [static]
    /// post-processing for legacy networks
    // void
    // MSLink::recheckSetRequestInformation() {
    //     for (auto item : myRecheck) {
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //         std::cout << " recheck l1=" << item.first->getDescription() << " l2=" << item.second->getDescription() << "\n";
    // #endif
    //         MSLink* const link = item.first;
    //         MSLink* const foeExitLink = item.second;
    //         const MSLane* const lane = link->getInternalLaneBefore();
    //         const MSLane* const foeLane = foeExitLink->getInternalLaneBefore();
    //         int conflictIndex = -1;
    //         for (int i = 0; i < (int)link->myFoeLanes.size(); i++) {
    //             if (link->myFoeLanes[i] == foeLane) {
    //                 conflictIndex = i;
    //                 break;
    //             }
    //         }
    //         if (conflictIndex == -1) {
    //             WRITE_WARNING("Could not recheck ConflictInfo for " + link->getDescription() + " and " + foeExitLink->getDescription() + "\n");
    //             continue;
    //         }
    //         ConflictInfo& ci = link->myConflicts[conflictIndex];
    //         if (ci.flag & CONFLICT_SIBLING_CONTINUATION) {
    //             const MSLane* const intLane = link->getInternalLaneBefore();
    //             const MSLane* const siblingCont = foeExitLink->getInternalLaneBefore();
    //             const MSLane* const sibling = siblingCont->getLogicalPredecessorLane();
    //             // this is an approximation because intLane and sibling+siblingCont are still close to each other but may have different curvature
    //             const double distToDivergence = intLane->getLength() - ci.lengthBehindCrossing;
    //             double lbcSibCont = MIN2(siblingCont->getLength(), MAX2(0.0, sibling->getLength() + siblingCont->getLength() - distToDivergence));
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //             std::cout << " siblingContinuation: distToDivergence=" << distToDivergence << " lbcSibCont=" << lbcSibCont << "\n";
    //             std::cout << " conflictIndex=" << conflictIndex << " foeLane=" << foeLane->getID() << " foeExitLink=" << foeExitLink->getDescription() << " intLane=" << intLane->getID() << "\n";
    // #endif
    //             ConflictInfo ci2 = ConflictInfo(lbcSibCont, intLane->getWidth());
    //             ci2.foeConflictIndex = conflictIndex;
    //             ci.foeConflictIndex = (int)foeExitLink->myConflicts.size();
    //             foeExitLink->myFoeLanes.push_back(intLane);
    //             foeExitLink->myConflicts.push_back(ci2);
    //             continue;
    //         }
    //
    //         std::vector<double> intersections1 = foeLane->getShape().intersectsAtLengths2D(lane->getShape());
    //         if (intersections1.size() == 0) {
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //             std::cout << "    no intersection\n";
    // #endif
    //             continue;
    //         }
    //         const double widthFactor = ci.conflictSize / foeLane->getWidth();
    //         const double conflictSize2 = lane->getWidth() * widthFactor;
    //         std::sort(intersections1.begin(), intersections1.end());
    //         intersections1.back() -= conflictSize2 / 2;
    //         intersections1.back() = MAX2(0.0, intersections1.back());
    //         ci.foeConflictIndex = (int)foeExitLink->myConflicts.size();
    //         foeExitLink->myConflicts.push_back(ConflictInfo(foeLane->getLength() - intersections1.back(), conflictSize2));
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //         std::cout << "    ci=" << conflictIndex << " wf=" << widthFactor << " flag=" << ci.flag << " flbc=" << foeExitLink->myConflicts.back().lengthBehindCrossing << "\n";
    // #endif
    //     }
    //     myRecheck.clear();
    // }
    #[allow(unused_variables)]
    pub fn recheck_set_request_information() {
        let items: Vec<(Ptr<MSLink>, Ptr<MSLink>)> = MY_RECHECK.with(|r| r.borrow().iter().cloned().collect());
        for (link, foeExitLink) in items {
            let lane = link.get_internal_lane_before();
            let foeLane = foeExitLink.get_internal_lane_before();
            let mut conflictIndex: i32 = -1;
            for i in 0..link.myFoeLanes.len() {
                if link.myFoeLanes[i] == foeLane {
                    conflictIndex = i as i32;
                    break;
                }
            }
            if conflictIndex == -1 {
                utils::common::msg_handler::write_warning(&format!("Could not recheck ConflictInfo for {} and {}\n", link.get_description(), foeExitLink.get_description()));
                continue;
            }
            let ci = conflictIndex as usize;
            if (link.myConflicts[ci].flag as i64 & MSLink_ConflictFlag::CONFLICT_SIBLING_CONTINUATION as i64) != 0 {
                let intLane = link.get_internal_lane_before();
                let siblingCont = foeExitLink.get_internal_lane_before();
                let sibling = siblingCont.get_logical_predecessor_lane();
                let distToDivergence = intLane.get_length() - link.myConflicts[ci].lengthBehindCrossing;
                let lbcSibCont = min2(siblingCont.get_length(), max2(0.0, sibling.get_length() + siblingCont.get_length() - distToDivergence));
                let ci2 = MSLink_ConflictInfo::new(lbcSibCont, intLane.get_width(), MSLink_ConflictFlag::CONFLICT_DEFAULT);
                // the vector index of ci2 is set before ci2 is appended to foeExitLink
                let mut ci2 = ci2;
                ci2.foeConflictIndex = conflictIndex;
                link.m().myConflicts[ci].foeConflictIndex = foeExitLink.myConflicts.len() as i32;
                foeExitLink.m().myFoeLanes.push(intLane);
                foeExitLink.m().myConflicts.push(ci2);
                continue;
            }
            let mut intersections1 = foeLane.get_shape_2().intersects_at_lengths2d_pv(lane.get_shape_2());
            if intersections1.is_empty() {
                continue;
            }
            let widthFactor = link.myConflicts[ci].conflictSize / foeLane.get_width();
            let conflictSize2 = lane.get_width() * widthFactor;
            intersections1.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let last = intersections1.len() - 1;
            intersections1[last] -= conflictSize2 / 2.;
            intersections1[last] = max2(0.0, intersections1[last]);
            link.m().myConflicts[ci].foeConflictIndex = foeExitLink.myConflicts.len() as i32;
            foeExitLink.m().myConflicts.push(MSLink_ConflictInfo::new(foeLane.get_length() - intersections1[last], conflictSize2, MSLink_ConflictFlag::CONFLICT_DEFAULT));
        }
        MY_RECHECK.with(|r| r.borrow_mut().clear());
    }

    /// C++: `bool MSLink::ignoreFoe(const SUMOTrafficObject *, const SUMOTrafficObject *)` [static]
    // bool
    // MSLink::ignoreFoe(const SUMOTrafficObject* ego, const SUMOTrafficObject* foe) {
    //     if (ego == nullptr || !ego->getParameter().wasSet(VEHPARS_JUNCTIONMODEL_PARAMS_SET)) {
    //         return false;
    //     }
    //     const SUMOVehicleParameter& param = ego->getParameter();
    //     for (const std::string& typeID : StringTokenizer(param.getParameter(toString(SUMO_ATTR_JM_IGNORE_TYPES), "")).getVector()) {
    //         if (typeID == foe->getVehicleType().getID()) {
    //             return true;
    //         }
    //     }
    //     for (const std::string& id : StringTokenizer(param.getParameter(toString(SUMO_ATTR_JM_IGNORE_IDS), "")).getVector()) {
    //         if (id == foe->getID()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn ignore_foe(ego: Ptr<SUMOTrafficObject>, foe: Ptr<SUMOTrafficObject>) -> bool {
        if ego.is_null() || !ego.get_parameter().was_set(VEHPARS_JUNCTIONMODEL_PARAMS_SET) {
            return false;
        }
        let param = ego.get_parameter();
        for typeID in param.get_parameter(&SUMO_ATTR_JM_IGNORE_TYPES.attr_name().to_string(), "").split_whitespace() {
            if typeID == foe.get_vehicle_type().get_id() {
                return true;
            }
        }
        for id in param.get_parameter(&SUMO_ATTR_JM_IGNORE_IDS.attr_name().to_string(), "").split_whitespace() {
            if id == foe.get_id() {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLink::unsafeMergeSpeeds(double, double, double, double)` [static, private]
    /// return whether the given vehicles may NOT merge safely
    // static inline bool unsafeMergeSpeeds(double leaderSpeed, double followerSpeed, double leaderDecel, double followerDecel) {
    //         // XXX mismatch between continuous an discrete deceleration
    //         return (leaderSpeed * leaderSpeed / leaderDecel) <= (followerSpeed * followerSpeed / followerDecel);
    //     }
    #[allow(unused_variables)]
    pub fn unsafe_merge_speeds(leaderSpeed: f64, followerSpeed: f64, leaderDecel: f64, followerDecel: f64) -> bool {
        // XXX mismatch between continuous an discrete deceleration
        (leaderSpeed * leaderSpeed / leaderDecel) <= (followerSpeed * followerSpeed / followerDecel)
    }

    /// C++: `bool MSLink::couldBrakeForLeader(double, double, const MSVehicle *, const MSVehicle *)` [static, private]
    /// whether follower could stay behind leader (possibly by braking)
    // bool
    // MSLink::couldBrakeForLeader(double followDist, double leaderDist, const MSVehicle* follow, const MSVehicle* leader) {
    //     return (// leader is ahead of follower
    //                followDist > leaderDist &&
    //                // and follower could brake for 1 s to stay behind leader
    //                followDist - leaderDist > follow->getSpeed() - follow->getCarFollowModel().getMaxDecel() - leader->getSpeed());
    // }
    #[allow(unused_variables)]
    pub fn could_brake_for_leader(followDist: f64, leaderDist: f64, follow: Ptr<MSVehicle>, leader: Ptr<MSVehicle>) -> bool {
        // leader is ahead of follower
        followDist > leaderDist && followDist - leaderDist > follow.get_speed() - follow.get_car_follow_model().get_max_decel() - leader.get_speed()
    }

    /// C++: `MSLink * MSLink::computeParallelLink(int)` [private]
    // MSLink*
    // MSLink::computeParallelLink(int direction) {
    //     const MSLane* const before = getLaneBefore()->getParallelLane(direction, false);
    //     const MSLane* const after = getLane()->getParallelLane(direction, false);
    //     if (before != nullptr && after != nullptr) {
    //         for (MSLink* const link : before->getLinkCont()) {
    //             if (link->getLane() == after) {
    //                 return link;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn compute_parallel_link(&mut self, direction: i32) -> Ptr<MSLink> {
        let before = self.get_lane_before().get_parallel_lane(direction, false);
        let after = self.get_lane().get_parallel_lane(direction, false);
        if !before.is_null() && !after.is_null() {
            for &link in before.get_link_cont().iter() {
                if link.get_lane() == after {
                    return link;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void MSLink::checkWalkingAreaFoe(const MSVehicle *, const MSLane *, std::vector<const MSPerson *> *, LinkLeaders &) const` [const, private]
    /// check for persons on walkingarea in the path of ego vehicle
    // void
    // MSLink::checkWalkingAreaFoe(const MSVehicle* ego, const MSLane* foeLane, std::vector<const MSPerson*>* collectBlockers, LinkLeaders& result) const {
    //     if (foeLane != nullptr && foeLane->getEdge().getPersons().size() > 0) {
    //         // pedestrians may be on an arbitrary path across this
    //         // walkingarea. make sure to keep enough distance.
    //         // This is a simple but conservative solution that could be improved
    //         // by ignoring pedestrians that are "obviously" not on a collision course
    //         double distToPeds = std::numeric_limits<double>::max();
    //         assert(myInternalLaneBefore != nullptr);
    //         PositionVector egoPath = myInternalLaneBefore->getShape();
    //         if (ego->getLateralPositionOnLane() != 0) {
    //             egoPath.move2side((MSGlobals::gLefthand ? 1 : -1) * ego->getLateralPositionOnLane());
    //         }
    //         for (MSTransportable* t : foeLane->getEdge().getPersons()) {
    //             MSPerson* p = static_cast<MSPerson*>(t);
    //             double dist = ego->getPosition().distanceTo2D(p->getPosition()) - p->getVehicleType().getLength();
    //             const bool inFront = isInFront(ego, egoPath, p->getPosition()) || isInFront(ego, egoPath, getFuturePosition(p));
    //             if (inFront) {
    //                 dist -= MAX2(ego->getVehicleType().getMinGap(), MSPModel::SAFETY_GAP);
    //             }
    // #ifdef DEBUG_WALKINGAREA
    //             if (ego->isSelected()) {
    //                 std::cout << SIMTIME << " veh=" << ego->getID() << " ped=" << p->getID()
    //                           << " pos=" << ego->getPosition() << " pedPos=" << p->getPosition()
    //                           << " futurePedPos=" << getFuturePosition(p)
    //                           << " rawDist=" << ego->getPosition().distanceTo2D(p->getPosition())
    //                           << " inFront=" << inFront
    //                           << " dist=" << dist << "\n";
    //             }
    // #endif
    //             if (dist < ego->getVehicleType().getWidth() / 2 || inFront) {
    //                 if (inFront) {
    //                     const double oncomingFactor = isOnComingPed(ego, p);
    //                     if (oncomingFactor > 0) {
    //                         // account for pedestrian movement while closing in
    //                         const double timeToStop = sqrt(dist) / 2;
    //                         const double pedDist = p->getMaxSpeed() * MAX2(timeToStop, TS) * oncomingFactor;
    //                         dist = MAX2(0.0, dist - pedDist);
    // #ifdef DEBUG_WALKINGAREA
    //                         if (ego->isSelected()) {
    //                             std::cout << "    timeToStop=" << timeToStop << " pedDist=" << pedDist << " factor=" << oncomingFactor << " dist2=" << dist << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //                 if (ignoreFoe(ego, p)) {
    //                     continue;
    //                 }
    //                 distToPeds = MIN2(distToPeds, dist);
    //                 if (collectBlockers != nullptr) {
    //                     collectBlockers->push_back(p);
    //                 }
    //             }
    //         }
    //         if (distToPeds != std::numeric_limits<double>::max()) {
    //             // leave extra space in front
    //             result.emplace_back(nullptr, -1, distToPeds);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_walking_area_foe(&self, ego: Ptr<MSVehicle>, foeLane: Ptr<MSLane>, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>, result: &mut MSLink_LinkLeaders) {
        if !foeLane.is_null() && foeLane.get_edge().get_persons().len() > 0 {
            let mut distToPeds = f64::MAX;
            debug_assert!(!self.myInternalLaneBefore.is_null());
            let mut egoPath = self.myInternalLaneBefore.get_shape_2().clone();
            if ego.get_lateral_position_on_lane() != 0. {
                egoPath.move2side((if g_lefthand() { 1. } else { -1. }) * ego.get_lateral_position_on_lane(), 100.);
            }
            for &t in foeLane.get_edge().get_persons().iter() {
                let p: Ptr<MSPerson> = t.cast();
                let mut dist = ego.get_position(0.).distance_to2d(&p.base.get_position(0.)) - p.base.get_vehicle_type().get_length();
                let inFront = self.is_in_front(ego, &egoPath, &p.base.get_position(0.)) || self.is_in_front(ego, &egoPath, &self.get_future_position(p, 1.));
                if inFront {
                    dist -= max2(ego.get_vehicle_type().get_min_gap(), MSPMODEL_SAFETY_GAP);
                }
                if dist < ego.get_vehicle_type().get_width() / 2. || inFront {
                    if inFront {
                        let oncomingFactor = self.is_on_coming_ped(ego, p);
                        if oncomingFactor > 0. {
                            let timeToStop = dist.sqrt() / 2.;
                            let pedDist = p.base.get_max_speed() * max2(timeToStop, ts()) * oncomingFactor;
                            dist = max2(0.0, dist - pedDist);
                        }
                    }
                    if MSLink::ignore_foe(msveh_to_traffic_object(ego), person_to_traffic_object(p)) {
                        continue;
                    }
                    distToPeds = min2(distToPeds, dist);
                    if !collectBlockers.is_null() {
                        collectBlockers.m().push(p);
                    }
                }
            }
            if distToPeds != f64::MAX {
                result.push(MSLink_LinkLeader::new(Ptr::null(), -1., distToPeds, MSLink_LinkLeaderFlag::LL_FROM_LEFT as i32, 0.));
            }
        }
    }

    /// C++: `bool MSLink::isInFront(const MSVehicle *, const PositionVector &, const Position &) const` [const, private]
    /// whether the given person is in front of the car
    // bool
    // MSLink::isInFront(const MSVehicle* ego, const PositionVector& egoPath, const Position& pPos) const {
    //     const double pedAngle = ego->getPosition().angleTo2D(pPos);
    //     const double angleDiff = fabs(GeomHelper::angleDiff(ego->getAngle(), pedAngle));
    // #ifdef DEBUG_WALKINGAREA
    //     if (ego->isSelected()) {
    //         std::cout << " angleDiff=" << RAD2DEG(angleDiff) << "\n";
    //     }
    // #endif
    //     if (angleDiff < DEG2RAD(75)) {
    //         return egoPath.distance2D(pPos) < ego->getVehicleType().getWidth() + MSPModel::SAFETY_GAP;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_in_front(&self, ego: Ptr<MSVehicle>, egoPath: &PositionVector, pPos: &Position) -> bool {
        let pedAngle = ego.get_position(0.).angle_to2d(pPos);
        let angleDiff = angle_diff(ego.get_angle(), pedAngle).abs();
        if angleDiff < deg2rad(75.) {
            return egoPath.distance2d(pPos, false) < ego.get_vehicle_type().get_width() + MSPMODEL_SAFETY_GAP;
        }
        false
    }

    /// C++: `double MSLink::isOnComingPed(const MSVehicle *, const MSPerson *) const` [const, private]
    /// whether the given person is walking towards the car returned as a factor in [0, 1]
    // double
    // MSLink::isOnComingPed(const MSVehicle* ego, const MSPerson* p) const {
    //     const double pedToEgoAngle = p->getPosition().angleTo2D(ego->getPosition());
    //     const double angleDiff = fabs(GeomHelper::angleDiff(p->getAngle(), pedToEgoAngle));
    // #ifdef DEBUG_WALKINGAREA
    //     if (ego->isSelected()) {
    //         std::cout << " ped-angleDiff=" << RAD2DEG(angleDiff) << " res=" << cos(angleDiff) << "\n";
    //     }
    // #endif
    //     if (angleDiff <= DEG2RAD(90)) {
    //         ;
    //         return cos(angleDiff);
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_on_coming_ped(&self, ego: Ptr<MSVehicle>, p: Ptr<MSPerson>) -> f64 {
        let pedToEgoAngle = p.base.get_position(0.).angle_to2d(&ego.get_position(0.));
        let angleDiff = angle_diff(p.base.get_angle(), pedToEgoAngle).abs();
        if angleDiff <= deg2rad(90.) {
            angleDiff.cos()
        } else {
            0.
        }
    }

    /// C++: `Position MSLink::getFuturePosition(const MSPerson *, double) const` [const, private]
    /// return extrapolated position of the given person after the given time
    // Position
    // MSLink::getFuturePosition(const MSPerson* p, double timeHorizon) const {
    //     const double a = p->getAngle();
    //     const double dist = timeHorizon * p->getMaxSpeed();
    //
    //     const Position offset(cos(a) * dist, sin(a) * dist);
    //     return p->getPosition() + offset;
    // }
    #[allow(unused_variables)]
    pub fn get_future_position(&self, p: Ptr<MSPerson>, timeHorizon: f64) -> Position {
        let a = p.base.get_angle();
        let dist = timeHorizon * p.base.get_max_speed();
        let offset = Position::new(a.cos() * dist, a.sin() * dist);
        p.base.get_position(0.) + offset
    }

    /// C++: `bool MSLink::blockedByFoe(const SUMOVehicle *, const ApproachingVehicleInformation &, SUMOTime, SUMOTime, double, double, bool, double, double, SUMOTime, const SUMOTrafficObject *) const` [const, private]
    // bool
    // MSLink::blockedByFoe(const SUMOVehicle* veh, const ApproachingVehicleInformation& avi,
    //                      SUMOTime arrivalTime, SUMOTime leaveTime, double arrivalSpeed, double leaveSpeed,
    //                      bool sameTargetLane, double impatience, double decel, SUMOTime waitingTime,
    //                      const SUMOTrafficObject* ego) const {
    // #ifdef MSLink_DEBUG_OPENED
    //     if (gDebugFlag1) {
    //         std::stringstream stream; // to reduce output interleaving from different threads
    //         stream << "    link=" << getDescription()
    //                << " foeVeh=" << veh->getID()
    //                << " req=" << avi.willPass
    //                << " aT=" << avi.arrivalTime
    //                << " lT=" << avi.leavingTime
    //                << "\n";
    //         std::cout << stream.str();
    //     }
    // #endif
    //     if (!avi.willPass) {
    //         return false;
    //     }
    //     if (myState == LINKSTATE_ALLWAY_STOP) {
    //         assert(waitingTime > 0);
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag1) {
    //             std::stringstream stream; // to reduce output interleaving from different threads
    //             stream << "    foeDist=" << avi.dist
    //                    << " foeBGap=" << veh->getBrakeGap(false)
    //                    << " foeWait=" << avi.waitingTime
    //                    << " wait=" << waitingTime
    //                    << "\n";
    //             std::cout << stream.str();
    //         }
    // #endif
    //         // when using actionSteps, the foe waiting time may be outdated
    //         const SUMOTime actionDelta = SIMSTEP - veh->getLastActionTime();
    //         if (waitingTime > avi.waitingTime + actionDelta) {
    //             return false;
    //         }
    //         if (waitingTime == (avi.waitingTime + actionDelta) && arrivalTime < avi.arrivalTime + actionDelta) {
    //             return false;
    //         }
    //     }
    //     SUMOTime foeArrivalTime = avi.arrivalTime;
    //     double foeArrivalSpeedBraking = avi.arrivalSpeedBraking;
    //     if (impatience > 0 && arrivalTime < avi.arrivalTime) {
    // #ifdef MSLink_DEBUG_OPENED
    //         gDebugFlag6 = ((ego == nullptr || ego->isSelected()) && (veh == nullptr || veh->isSelected()));
    // #endif
    //         const SUMOTime fatb = computeFoeArrivalTimeBraking(arrivalTime, veh, avi.arrivalTime, impatience, avi.dist, foeArrivalSpeedBraking);
    //         foeArrivalTime = (SUMOTime)((1. - impatience) * (double)avi.arrivalTime + impatience * (double)fatb);
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag6) {
    //             std::cout << SIMTIME << " link=" << getDescription() << " ego=" << ego->getID() << " foe=" << veh->getID()
    //                       << " at=" << STEPS2TIME(arrivalTime)
    //                       << " fat=" << STEPS2TIME(avi.arrivalTime)
    //                       << " fatb=" << STEPS2TIME(fatb)
    //                       << " fat2=" << STEPS2TIME(foeArrivalTime)
    //                       << "\n";
    //         }
    // #endif
    //     }
    //
    //
    //     const SUMOTime lookAhead = (myState == LINKSTATE_ZIPPER
    //                                 ? myLookaheadTimeZipper
    //                                 : (ego == nullptr
    //                                    ? myLookaheadTime
    //                                    : TIME2STEPS(ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_TIMEGAP_MINOR, STEPS2TIME(myLookaheadTime)))));
    //     //if (ego != 0) std::cout << SIMTIME << " ego=" << ego->getID() << " jmTimegapMinor=" << ego->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_TIMEGAP_MINOR, -1) << " lookAhead=" << lookAhead << "\n";
    // #ifdef MSLink_DEBUG_OPENED
    //     if (gDebugFlag1 || gDebugFlag6) {
    //         std::stringstream stream; // to reduce output interleaving from different threads
    //         stream << "       imp=" << impatience << " fAT2=" << foeArrivalTime << " fASb=" << foeArrivalSpeedBraking << " lA=" << lookAhead << " egoAT=" << arrivalTime << " egoLT=" << leaveTime << " egoLS=" << leaveSpeed << "\n";
    //         std::cout << stream.str();
    //     }
    // #endif
    //     if (avi.leavingTime < arrivalTime) {
    //         // ego wants to be follower
    //         if (sameTargetLane && (arrivalTime - avi.leavingTime < lookAhead
    //                                || unsafeMergeSpeeds(avi.leaveSpeed, arrivalSpeed,
    //                                        veh->getVehicleType().getCarFollowModel().getMaxDecel(), decel))) {
    // #ifdef MSLink_DEBUG_OPENED
    //             if (gDebugFlag1 || gDebugFlag6) {
    //                 std::cout << "      blocked (cannot follow)\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     } else if (foeArrivalTime > leaveTime + lookAhead) {
    //         // ego wants to be leader.
    //         if (sameTargetLane && unsafeMergeSpeeds(leaveSpeed, foeArrivalSpeedBraking,
    //                                                 decel, veh->getVehicleType().getCarFollowModel().getMaxDecel())) {
    // #ifdef MSLink_DEBUG_OPENED
    //             if (gDebugFlag1 || gDebugFlag6) {
    //                 std::cout << "      blocked (cannot lead)\n";
    //             }
    // #endif
    //             return true;
    //         }
    //     } else {
    //         // even without considering safeHeadwayTime there is already a conflict
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag1 || gDebugFlag6) {
    //             std::cout << "      blocked (hard conflict)\n";
    //         }
    // #endif
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn blocked_by_foe(&self, veh: Ptr<SUMOVehicle>, avi: &MSLink_ApproachingVehicleInformation, arrivalTime: SUMOTime, leaveTime: SUMOTime, arrivalSpeed: f64, leaveSpeed: f64, sameTargetLane: bool, impatience: f64, decel: f64, waitingTime: SUMOTime, ego: Ptr<SUMOTrafficObject>) -> bool {
        if !avi.willPass {
            return false;
        }
        if self.myState == LINKSTATE_ALLWAY_STOP {
            // EXPERIMENT (revert): debug_assert!(waitingTime > 0);
            let actionDelta = MSNet::get_instance().get_current_time_step() - veh.get_last_action_time();
            if waitingTime > avi.waitingTime + actionDelta {
                return false;
            }
            if waitingTime == (avi.waitingTime + actionDelta) && arrivalTime < avi.arrivalTime + actionDelta {
                return false;
            }
        }
        let mut foeArrivalTime = avi.arrivalTime;
        let mut foeArrivalSpeedBraking = avi.arrivalSpeedBraking;
        if impatience > 0. && arrivalTime < avi.arrivalTime {
            let fatb = MSLink::compute_foe_arrival_time_braking(arrivalTime, veh, avi.arrivalTime, impatience, avi.dist, &mut foeArrivalSpeedBraking);
            foeArrivalTime = ((1. - impatience) * (avi.arrivalTime as f64) + impatience * (fatb as f64)) as SUMOTime;
        }
        let lookAhead: SUMOTime = if self.myState == LINKSTATE_ZIPPER {
            MSLink::myLookaheadTimeZipper
        } else if ego.is_null() {
            MSLink::myLookaheadTime
        } else {
            time2steps(ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_TIMEGAP_MINOR, steps2time(MSLink::myLookaheadTime)))
        };
        if avi.leavingTime < arrivalTime {
            if sameTargetLane
                && (arrivalTime - avi.leavingTime < lookAhead
                    || MSLink::unsafe_merge_speeds(avi.leaveSpeed, arrivalSpeed, veh.get_vehicle_type().get_car_follow_model().get_max_decel(), decel))
            {
                return true;
            }
        } else if foeArrivalTime > leaveTime + lookAhead {
            if sameTargetLane
                && MSLink::unsafe_merge_speeds(leaveSpeed, foeArrivalSpeedBraking, decel, veh.get_vehicle_type().get_car_follow_model().get_max_decel())
            {
                return true;
            }
        } else {
            return true;
        }
        false
    }

    /// C++: `bool MSLink::checkContOff() const` [const, private]
    /// figure out whether the cont status remains in effect when switching off the tls
    // bool
    // MSLink::checkContOff() const {
    //     // check whether this link gets to keep its cont status switching the tls off
    //     // @note: this could also be pre-computed in netconvert
    //     // we check whether there is any major link from this edge
    //     for (const MSLane* cand : myLaneBefore->getEdge().getLanes()) {
    //         for (const MSLink* link : cand->getLinkCont()) {
    //             if (link->getOffState() == LINKSTATE_TL_OFF_NOSIGNAL) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_cont_off(&self) -> bool {
        for &cand in self.myLaneBefore.get_edge().get_lanes().iter() {
            for &link in cand.get_link_cont().iter() {
                if link.get_off_state() == LINKSTATE_TL_OFF_NOSIGNAL {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool MSLink::contIntersect(const MSLane *, const MSLane *)` [private]
    /// check if the lane intersects with a foe cont-lane
    // bool
    // MSLink::contIntersect(const MSLane* lane, const MSLane* foe) {
    //     if (foe->getLinkCont()[0]->getViaLane() != nullptr) {
    //         std::vector<double> intersections = lane->getShape().intersectsAtLengths2D(foe->getShape());
    //         return intersections.size() > 0;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn cont_intersect(&mut self, lane: Ptr<MSLane>, foe: Ptr<MSLane>) -> bool {
        if !foe.get_link_cont()[0].get_via_lane().is_null() {
            let intersections = lane.get_shape_2().intersects_at_lengths2d_pv(foe.get_shape_2());
            return intersections.len() > 0;
        }
        false
    }

    /// C++: `double MSLink::computeDistToDivergence(const MSLane *, const MSLane *, double, bool, double) const` [const, private]
    /// compute point of divergence for geomatries with a common start or end
    // double
    // MSLink::computeDistToDivergence(const MSLane* lane, const MSLane* sibling, double minDist, bool sameSource, double siblingPredLength) const {
    //     double lbcSibling = 0;
    //     double lbcLane = 0;
    //
    //     PositionVector l = lane->getShape();
    //     PositionVector s = sibling->getShape();
    //     double length = l.length2D();
    //     double sibLength = s.length2D();
    //     if (!sameSource) {
    //         l = l.reverse();
    //         s = s.reverse();
    //     } else if (sibling->getEntryLink()->myAmIndirect) {
    //         // ignore final waiting position since it may be quite close to the lane
    //         // shape but the waiting position is perpendicular (so the minDist
    //         // requirement is not necessary
    //         lbcSibling += s[-1].distanceTo2D(s[-2]);
    //         s.pop_back();
    //     } else if (lane->getEntryLink()->myAmIndirect) {
    //         // ignore final waiting position since it may be quite close to the lane
    //         // shape but the waiting position is perpendicular (so the minDist
    //         // requirement is not necessary
    //         lbcLane += l[-1].distanceTo2D(l[-2]);
    //         l.pop_back();
    //     }
    //
    // #ifdef MSLink_DEBUG_CROSSING_POINTS_DETAILS
    //     std::cout << "   sameSource=" << sameSource << " lane=" << lane->getID() << " sib=" << sibling->getID() << " minDist=" << minDist << " backDist=" << l.back().distanceTo2D(s.back()) << "\n";
    // #endif
    //     if (l.back().distanceTo2D(s.back()) > minDist) {
    //         // compute the final divergence point
    //         // this position serves two purposes:
    //         // 1) once the foe vehicle back (on sibling) has passed this point, we can safely ignore it
    //         // 2) both vehicles are put into a cf-relationship while before the point.
    //         //    Since the actual crossing point is at the start of the junction,
    //         //    we want to make sure that both vehicles have the same distance to the crossing point and thus follow each other naturally
    //         std::vector<double> distances = l.distances(s);
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //         std::cout << "   distances=" << toString(distances) << "\n";
    // #endif
    //         assert(distances.size() == l.size() + s.size());
    //         if (distances.back() > minDist && distances[l.size() - 1] > minDist) {
    //             // do a pairwise check between lane and sibling to make because we do not know which of them bends more
    //             for (int j = (int)s.size() - 2; j >= 0; j--) {
    //                 const int i = j + (int)l.size();
    //                 const double segLength = s[j].distanceTo2D(s[j + 1]);
    //                 if (distances[i] > minDist) {
    //                     lbcSibling += segLength;
    //                 } else {
    //                     // assume no sharp bends and just interpolate the last segment
    //                     lbcSibling += segLength - (minDist - distances[i]) * segLength / (distances[i + 1] - distances[i]);
    //                     break;
    //                 }
    //             }
    //             for (int i = (int)l.size() - 2; i >= 0; i--) {
    //                 const double segLength = l[i].distanceTo2D(l[i + 1]);
    //                 if (distances[i] > minDist) {
    //                     lbcLane += segLength;
    //                 } else {
    //                     // assume no sharp bends and just interpolate the last segment
    //                     lbcLane += segLength - (minDist - distances[i]) * segLength / (distances[i + 1] - distances[i]);
    //                     break;
    //                 }
    //             }
    //         }
    //         assert(lbcSibling >= -NUMERICAL_EPS);
    //         assert(lbcLane >= -NUMERICAL_EPS);
    //     }
    //     const double distToDivergence1 = sibling->getLength() + siblingPredLength - lbcSibling;
    //     const double distToDivergence2 = lane->getLength() - lbcLane;
    //     const double distToDivergence = MIN3(
    //                                         MAX2(distToDivergence1, distToDivergence2),
    //                                         sibLength, length);
    // #ifdef MSLink_DEBUG_CROSSING_POINTS
    //     std::cout << "   distToDivergence=" << distToDivergence
    //               << " distTD1=" << distToDivergence1
    //               << " distTD2=" << distToDivergence2
    //               << " length=" << length
    //               << " sibLength=" << sibLength
    //               << "\n";
    // #endif
    //     return distToDivergence;
    // }
    #[allow(unused_variables)]
    pub fn compute_dist_to_divergence(&self, lane: Ptr<MSLane>, sibling: Ptr<MSLane>, minDist: f64, sameSource: bool, siblingPredLength: f64) -> f64 {
        let mut lbcSibling = 0.;
        let mut lbcLane = 0.;
        let mut l = lane.get_shape_2().clone();
        let mut s = sibling.get_shape_2().clone();
        let length = l.length2d();
        let sibLength = s.length2d();
        if !sameSource {
            l = l.reverse();
            s = s.reverse();
        } else if sibling.get_entry_link().myAmIndirect {
            lbcSibling += s.at(-1).distance_to2d(s.at(-2));
            s.pop();
        } else if lane.get_entry_link().myAmIndirect {
            lbcLane += l.at(-1).distance_to2d(l.at(-2));
            l.pop();
        }
        if l.back().distance_to2d(s.back()) > minDist {
            let distances = l.distances(&s, false);
            debug_assert!(distances.len() == l.len() + s.len());
            if distances[distances.len() - 1] > minDist && distances[l.len() - 1] > minDist {
                for j in (0..s.len() as isize - 1).rev() {
                    let j = j as usize;
                    let i = j + l.len();
                    let segLength = s[j].distance_to2d(&s[j + 1]);
                    if distances[i] > minDist {
                        lbcSibling += segLength;
                    } else {
                        lbcSibling += segLength - (minDist - distances[i]) * segLength / (distances[i + 1] - distances[i]);
                        break;
                    }
                }
                for i in (0..l.len() as isize - 1).rev() {
                    let i = i as usize;
                    let segLength = l[i].distance_to2d(&l[i + 1]);
                    if distances[i] > minDist {
                        lbcLane += segLength;
                    } else {
                        lbcLane += segLength - (minDist - distances[i]) * segLength / (distances[i + 1] - distances[i]);
                        break;
                    }
                }
            }
            debug_assert!(lbcSibling >= -NUMERICAL_EPS);
            debug_assert!(lbcLane >= -NUMERICAL_EPS);
        }
        let distToDivergence1 = sibling.get_length() + siblingPredLength - lbcSibling;
        let distToDivergence2 = lane.get_length() - lbcLane;
        let maxDist = max2(distToDivergence1, distToDivergence2);
        // C++ MIN3(a, b, c) = MIN2(c, a < b ? a : b)
        let distToDivergence = min2(length, if maxDist < sibLength { maxDist } else { sibLength });
        distToDivergence
    }

    /// C++: `SUMOTime MSLink::computeFoeArrivalTimeBraking(SUMOTime, const SUMOVehicle *, SUMOTime, double, double, double &)` [static, private]
    /// compute arrival time if foe vehicle is braking for ego
    // SUMOTime
    // MSLink::computeFoeArrivalTimeBraking(SUMOTime arrivalTime, const SUMOVehicle* foe, SUMOTime foeArrivalTime, double impatience, double dist, double& fasb) {
    //     // a: distance saved when foe brakes from arrivalTime to foeArrivalTime
    //     // b: distance driven past foeArrivalTime
    //     // m: permitted decceleration
    //     // d: total deceleration until foeArrivalTime
    //     // dist2: distance of foe at arrivalTime
    //     // actual arrivalTime must fall on a simulation step
    //     if (arrivalTime - arrivalTime % DELTA_T == foeArrivalTime - foeArrivalTime % DELTA_T) {
    //         // foe enters the junction in the same step
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag6) {
    //             std::cout << "   foeAT before egoAT\n";
    //         }
    // #endif
    //         return foeArrivalTime;
    //     }
    //     if (arrivalTime % DELTA_T > 0) {
    //         arrivalTime = arrivalTime - (arrivalTime % DELTA_T) + DELTA_T;
    //     }
    //     //arrivalTime += DELTA_T - arrivalTime % DELTA_T;
    //     const double m = foe->getVehicleType().getCarFollowModel().getMaxDecel() * impatience;
    //     const double dt = STEPS2TIME(foeArrivalTime - arrivalTime);
    //     const double d = dt * m;
    //     const double a = dt * d / 2;
    //     const double v = dist / STEPS2TIME(foeArrivalTime - SIMSTEP + DELTA_T);
    //     const double dist2 = dist - v * STEPS2TIME(arrivalTime - SIMSTEP);
    // #ifdef MSLink_DEBUG_OPENED
    //     if (gDebugFlag6) {
    //         std::cout << "   dist=" << dist << " dist2=" << dist2
    //                   << " at=" << STEPS2TIME(arrivalTime)
    //                   << " fat=" << STEPS2TIME(foeArrivalTime)
    //                   << " dt=" << dt << " v=" << v << " m=" << m << " d=" << d << " a=" << a << "\n";
    //     }
    // #endif
    //     if (0.5 * v * v / m <= dist2) {
    // #ifdef MSLink_DEBUG_OPENED
    //         if (gDebugFlag6) {
    //             std::cout << "   canBrakeToStop\n";
    //         }
    // #endif
    //         fasb = 0;
    //         return foeArrivalTime + TIME2STEPS(30);
    //     }
    //     // a = b (foe reaches the original distance to the stop line)
    //     // x: time driven past foeArrivalTime
    //     // v: foe speed without braking
    //     // v2: average foe speed after foeArrivalTime (braking continues for time x)
    //     // v2 = (v - d - x * m / 2)
    //     // b = v2 * x
    //     // solving for x gives:
    //     const double x = (sqrt(4 * (v - d) * (v - d) - 8 * m * a) * -0.5 - d + v) / m;
    //
    // #ifdef MSLink_DEBUG_OPENED
    //     const double x2 = (sqrt(4 * (v - d) * (v - d) - 8 * m * a) * 0.5 - d + v) / m;
    //     if (gDebugFlag6 || std::isnan(x)) {
    //         std::cout << SIMTIME << "   dist=" << dist << " dist2=" << dist2  << " at=" << STEPS2TIME(arrivalTime) << " m=" << m << " d=" << d << " v=" << v << " a=" << a << " x=" << x << " x2=" << x2 << "\n";
    //     }
    // #endif
    //     fasb = v - (dt + x) * m;
    //     return foeArrivalTime + TIME2STEPS(x);
    // }
    #[allow(unused_variables)]
    pub fn compute_foe_arrival_time_braking(arrivalTime: SUMOTime, foe: Ptr<SUMOVehicle>, foeArrivalTime: SUMOTime, impatience: f64, dist: f64, fasb: &mut f64) -> SUMOTime {
        let mut arrivalTime = arrivalTime;
        let deltaT = delta_t();
        if arrivalTime - arrivalTime % deltaT == foeArrivalTime - foeArrivalTime % deltaT {
            return foeArrivalTime;
        }
        if arrivalTime % deltaT > 0 {
            arrivalTime = arrivalTime - (arrivalTime % deltaT) + deltaT;
        }
        let m = foe.get_vehicle_type().get_car_follow_model().get_max_decel() * impatience;
        let dt = steps2time(foeArrivalTime - arrivalTime);
        let d = dt * m;
        let a = dt * d / 2.;
        let simStep = MSNet::get_instance().get_current_time_step();
        let v = dist / steps2time(foeArrivalTime - simStep + deltaT);
        let dist2 = dist - v * steps2time(arrivalTime - simStep);
        if 0.5 * v * v / m <= dist2 {
            *fasb = 0.;
            return foeArrivalTime + time2steps(30.);
        }
        let x = ((4. * (v - d) * (v - d) - 8. * m * a).sqrt() * -0.5 - d + v) / m;
        *fasb = v - (dt + x) * m;
        foeArrivalTime + time2steps(x)
    }

    /// C++: `bool MSLink::lateralOverlap(double, double, double, double)` [static, private]
    /// check whether the given vehicle positions overlap laterally
    // bool
    // MSLink::lateralOverlap(double posLat, double width, double posLat2, double width2) {
    //     return fabs(posLat2 - posLat) < (width + width2) / 2;
    // }
    #[allow(unused_variables)]
    pub fn lateral_overlap(posLat: f64, width: f64, posLat2: f64, width2: f64) -> bool {
        (posLat2 - posLat).abs() < (width + width2) / 2.
    }

    /// C++: `const CustomConflict * MSLink::getCustomConflict(const MSLane *) const` [const, private]
    /// return CustomConflict with foeLane if it is defined
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_custom_conflict(&self, foeLane: Ptr<MSLane>) -> Ptr<MSLink_CustomConflict> {
        if !self.myCustomConflicts.is_empty() {
            let foeFrom = foeLane.get_normal_predecessor_lane();
            let foeTo = foeLane.get_normal_successor_lane();
            for cc in self.myCustomConflicts.iter() {
                if cc.from == foeFrom && cc.to == foeTo {
                    return Ptr::new(cc);
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void MSLink::updateDistToFoePedCrossing(double)` [private]
    /// add information about another pedestrian crossing
    // void
    // MSLink::updateDistToFoePedCrossing(double dist) {
    //     myDistToFoePedCrossing = MIN2(myDistToFoePedCrossing, dist);
    // }
    #[allow(unused_variables)]
    pub fn update_dist_to_foe_ped_crossing(&mut self, dist: f64) {
        self.myDistToFoePedCrossing = min2(self.myDistToFoePedCrossing, dist);
    }

    /// C++: `void MSLink::MSLink(const MSLink &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSLink) -> Self {
        // C++ copy constructor is declared private and never defined; copying is invalid
        panic!("MSLink copy constructor is invalidated (private and undefined in C++)")
    }

    /// C++: `MSLink & MSLink::operator=(const MSLink &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSLink) -> &mut MSLink {
        // C++ assignment operator is declared private and never defined; assignment is invalid
        panic!("MSLink assignment operator is invalidated (private and undefined in C++)")
    }

}


#[cfg(test)]
mod any_vehicles_tests {
    use super::*;

    // shortcut: MSLane::new -> init_restrictions needs the MSNet singleton, which unit tests cannot build yet
    // (see ms_vehicle.rs vt_dispatch_tests); real_dense in run_all.sh guards this meanwhile, remove #[ignore] once MSNet can be built
    #[test]
    #[ignore = "MSLane::new needs the MSNet singleton, see shortcut above"]
    fn any_vehicles_includes_vehicles_already_processed_by_the_lane_changer() {
        // real_dense v41 t=92: the foe on a sibling internal lane sat in myTmpVehicles while its edge was
        // lane-changing; C++ AnyVehicleIterator still yields it as a link leader, the old helper did not
        MSLane::init_one_rng_for_tests();
        let edge = Ptr::from_mut(Box::leak(Box::new(MSEdge::new(":j_0", 0, SumoXMLEdgeFunc::INTERNAL, "", "", "", 0, 1.0))));
        let lane = Ptr::from_mut(Box::leak(Box::new(MSLane::new(":j_0_4", 13.9, 1.0, 12.0, edge, 0, &PositionVector::new(), 3.2, utils::common::sumo_vehicle_class::SVC_PASSENGER, 0, 0, 0, false, "", &PositionVector::new()))));
        // never dereferenced: without partial vehicles the iterator only hands the pointers out
        let a: Ptr<MSVehicle> = Ptr::from_raw(0x10 as *mut MSVehicle);
        let b: Ptr<MSVehicle> = Ptr::from_raw(0x20 as *mut MSVehicle);
        lane.m().myVehicles.push(a);
        lane.m().myTmpVehicles.push(b);
        assert_eq!(any_vehicles_of(lane), vec![a, b]);
    }
}
