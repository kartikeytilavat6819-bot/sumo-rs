//! Port of microsim/transportables/MSPModel_Striping.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{SVC_IGNORING, SVC_PEDESTRIAN};
use utils::common::to_string::f64_to_string_prec;

/// C++ enum `MSPModel_Striping::ObstacleType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSPModel_Striping_ObstacleType {
    OBSTACLE_NONE = 0,
    OBSTACLE_PED = 1,
    OBSTACLE_VEHICLE = 3,
    OBSTACLE_END = 4,
    OBSTACLE_NEXTEND = 5,
    OBSTACLE_LINKCLOSED = 6,
    OBSTACLE_ARRIVALPOS = 7,
}

/// C++ typedef `std::map<const MSLane *, double> MSPModel_Striping::MinNextLengths`
pub type MSPModel_Striping_MinNextLengths = BTreeMap<Ptr<MSLane>, f64>;

/// C++ typedef `std::map<const MSLane *, Obstacles, ComparatorNumericalIdLess> MSPModel_Striping::NextLanesObstacles`
pub type MSPModel_Striping_NextLanesObstacles = BTreeMap<Ptr<MSLane>, MSPModel_Striping_Obstacles>;

/// C++ typedef `std::vector<Obstacle> MSPModel_Striping::Obstacles`
pub type MSPModel_Striping_Obstacles = Vec<MSPModel_Striping_Obstacle>;

/// C++ typedef `std::map<std::pair<const MSLane *, const MSLane *>, const WalkingAreaPath> MSPModel_Striping::WalkingAreaPaths`
pub type MSPModel_Striping_WalkingAreaPaths = BTreeMap<(Ptr<MSLane>, Ptr<MSLane>), MSPModel_Striping_WalkingAreaPath>;

/// The pedestrian movement model using stripes on sidewalks
/// C++ class `MSPModel_Striping` (microsim/transportables/MSPModel_Striping.h)
#[repr(C)]
pub struct MSPModel_Striping {
    /// C++ base class `MSPModel_Interacting`
    pub base: MSPModel_Interacting,
}

impl std::ops::Deref for MSPModel_Striping {
    type Target = MSPModel_Interacting;
    fn deref(&self) -> &MSPModel_Interacting {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_Striping {
    fn deref_mut(&mut self) -> &mut MSPModel_Interacting {
        &mut self.base
    }
}

// C++ static members of MSPModel_Striping (port as module-level statics, see AGENTS.md):
//   static double MSPModel_Striping::stripeWidth;
//   static double MSPModel_Striping::dawdling;
//   static double MSPModel_Striping::minGapToVehicle;
//   static int MSPModel_Striping::myWalkingAreaDetail;
//   static SUMOTime MSPModel_Striping::jamTime;
//   static SUMOTime MSPModel_Striping::jamTimeCrossing;
//   static SUMOTime MSPModel_Striping::jamTimeNarrow;
//   static double MSPModel_Striping::jamFactor;
//   static bool MSPModel_Striping::myLegacyPosLat;
//   static const double MSPModel_Striping::LOOKAHEAD_SAMEDIR;
//   static const double MSPModel_Striping::LOOKAHEAD_ONCOMING;
//   static const double MSPModel_Striping::LOOKAROUND_VEHICLES;
//   static const double MSPModel_Striping::LOOKAHEAD_ONCOMING_DIST;
//   static const double MSPModel_Striping::LATERAL_PENALTY;
//   static const double MSPModel_Striping::OBSTRUCTED_PENALTY;
//   static const double MSPModel_Striping::INAPPROPRIATE_PENALTY;
//   static const double MSPModel_Striping::ONCOMING_CONFLICT_PENALTY;
//   static const double MSPModel_Striping::OBSTRUCTION_THRESHOLD;
//   static const double MSPModel_Striping::SQUEEZE;
//   static double MSPModel_Striping::RESERVE_FOR_ONCOMING_FACTOR;
//   static double MSPModel_Striping::RESERVE_FOR_ONCOMING_FACTOR_JUNCTIONS;
//   static double MSPModel_Striping::RESERVE_FOR_ONCOMING_MAX;
//   static bool MSPModel_Striping::USE_NET_SPEEDS;
//   static const double MSPModel_Striping::MAX_WAIT_TOLERANCE;
//   static const double MSPModel_Striping::LATERAL_SPEED_FACTOR;
//   static const double MSPModel_Striping::MIN_STARTUP_DIST;
//   static const double MSPModel_Striping::DIST_FAR_AWAY;
//   static const double MSPModel_Striping::DIST_BEHIND;
//   static const double MSPModel_Striping::DIST_OVERLAP;
//   static WalkingAreaPaths MSPModel_Striping::myWalkingAreaPaths;
//   static std::map<const MSEdge *, std::vector<const MSLane *>> MSPModel_Striping::myWalkingAreaFoes;
//   static MinNextLengths MSPModel_Striping::myMinNextLengths;

/// Module-level storage for the mutable C++ static members (get/set pairs, like MSGlobals)
macro_rules! striping_static {
    ($name:ident, $get:ident, $set:ident, $ty:ty, $default:expr) => {
        thread_local! { static $name: std::cell::Cell<$ty> = const { std::cell::Cell::new($default) }; }
        pub fn $get() -> $ty {
            $name.with(|c| c.get())
        }
        pub fn $set(v: $ty) {
            $name.with(|c| c.set(v))
        }
    };
}
striping_static!(STRIPE_WIDTH, stripe_width, set_stripe_width, f64, 0.);
striping_static!(DAWDLING, dawdling, set_dawdling, f64, 0.);
striping_static!(MIN_GAP_TO_VEHICLE, min_gap_to_vehicle, set_min_gap_to_vehicle, f64, 0.);
striping_static!(MY_WALKING_AREA_DETAIL, my_walking_area_detail, set_my_walking_area_detail, i32, 0);
striping_static!(JAM_TIME, jam_time, set_jam_time, SUMOTime, 0);
striping_static!(JAM_TIME_CROSSING, jam_time_crossing, set_jam_time_crossing, SUMOTime, 0);
striping_static!(JAM_TIME_NARROW, jam_time_narrow, set_jam_time_narrow, SUMOTime, 0);
striping_static!(JAM_FACTOR, jam_factor, set_jam_factor, f64, 0.);
striping_static!(MY_LEGACY_POS_LAT, my_legacy_pos_lat, set_my_legacy_pos_lat, bool, false);
striping_static!(RESERVE_FOR_ONCOMING_FACTOR, reserve_for_oncoming_factor, set_reserve_for_oncoming_factor, f64, 0.);
striping_static!(RESERVE_FOR_ONCOMING_FACTOR_JUNCTIONS, reserve_for_oncoming_factor_junctions, set_reserve_for_oncoming_factor_junctions, f64, 0.);
striping_static!(RESERVE_FOR_ONCOMING_MAX, reserve_for_oncoming_max, set_reserve_for_oncoming_max, f64, 0.);
striping_static!(USE_NET_SPEEDS, use_net_speeds, set_use_net_speeds, bool, false);

// determinism: the C++ maps are ordered by pointer value; Ptr's Ord compares addresses the same way
thread_local! {
    static MY_WALKING_AREA_PATHS: std::cell::RefCell<MSPModel_Striping_WalkingAreaPaths> = std::cell::RefCell::new(BTreeMap::new());
    static MY_WALKING_AREA_FOES: std::cell::RefCell<BTreeMap<Ptr<MSEdge>, Vec<Ptr<MSLane>>>> = std::cell::RefCell::new(BTreeMap::new());
    static MY_MIN_NEXT_LENGTHS: std::cell::RefCell<MSPModel_Striping_MinNextLengths> = std::cell::RefCell::new(BTreeMap::new());
}

/// Lookup in myWalkingAreaPaths. Returned pointers stay valid because the map is only filled in initWalkingAreaPaths.
fn lookup_walking_area_path(before: Ptr<MSLane>, after: Ptr<MSLane>) -> Option<Ptr<MSPModel_Striping_WalkingAreaPath>> {
    MY_WALKING_AREA_PATHS.with(|m| m.borrow().get(&(before, after)).map(|p| Ptr::new(p)))
}

/// Token readers for the whitespace-separated state format (C++ `std::istringstream >>`)
fn ps_next<T: std::str::FromStr + Default>(state: &mut std::str::SplitWhitespace<'_>) -> T {
    state.next().and_then(|t| t.parse::<T>().ok()).unwrap_or_default()
}
fn ps_next_str(state: &mut std::str::SplitWhitespace<'_>) -> String {
    state.next().unwrap_or("").to_string()
}
fn ps_next_bool(state: &mut std::str::SplitWhitespace<'_>) -> bool {
    ps_next::<i32>(state) != 0
}

/// C++ implicit copy constructor `PState(const PState &)` (used as `new PState(*p)`)
fn copy_pstate(p: &MSPModel_Striping_PState) -> MSPModel_Striping_PState {
    MSPModel_Striping_PState {
        base: MSPModel_InteractingState {
            base: MSTransportableStateAdapter { vt: &MSPMODEL_STRIPING_PSTATE_VT },
            myPerson: p.myPerson,
            myStage: p.myStage,
            myLane: p.myLane,
            myEdgePos: p.myEdgePos,
            myPosLat: p.myPosLat,
            myDir: p.myDir,
            mySpeed: p.mySpeed,
            mySpeedLat: p.mySpeedLat,
            myWaitingToEnter: p.myWaitingToEnter,
            myWaitingTime: p.myWaitingTime,
            myTotalWaitingTime: p.myTotalWaitingTime,
            myAmJammed: p.myAmJammed,
            myRemoteXYPos: p.myRemoteXYPos,
            myAngle: std::cell::Cell::new(p.myAngle.get()),
        },
        myNLI: MSPModel_Striping_NextLaneInfo::new(p.myNLI.lane, p.myNLI.link, p.myNLI.dir),
        myWalkingAreaPath: p.myWalkingAreaPath,
    }
}

/// C++ NUMERICAL_EPS-style constant `MSPModel::SAFETY_GAP` (MSPModel.cpp)
const SAFETY_GAP: f64 = 1.0;
/// C++ `MSPModel::SIDEWALK_OFFSET` (MSPModel.cpp)
const SIDEWALK_OFFSET: f64 = 3.0;
/// C++ `MSPModel::UNSPECIFIED_POS_LAT` / `RANDOM_POS_LAT` (MSPModel.cpp)
const UNSPECIFIED_POS_LAT: f64 = f64::MAX;
const RANDOM_POS_LAT: f64 = -f64::MAX;

/// Sidewalk lookup for the walking stage; a uniquely named wrapper avoids clashing with the router's get_sidewalk in the prelude glob
pub fn walking_sidewalk(edge: Ptr<MSEdge>, svc: SUMOVehicleClass) -> Ptr<MSLane> {
    get_sidewalk(edge, svc)
}

/// C++ getSidewalk<MSEdge, MSLane>(edge, svc) from utils/router/IntermodalEdge.h
fn get_sidewalk(edge: Ptr<MSEdge>, svc: SUMOVehicleClass) -> Ptr<MSLane> {
    crate::utils::router::intermodal_edge::get_sidewalk::<MSEdge, MSLane, SUMOVehicle>(edge, svc)
}

// C++ constants (static const double members)
pub const LOOKAHEAD_SAMEDIR: f64 = 4.0; // seconds
pub const LOOKAHEAD_ONCOMING: f64 = 10.0; // seconds
pub const LOOKAHEAD_ONCOMING_DIST: f64 = 10.0; // m
pub const LOOKAROUND_VEHICLES: f64 = 60.0; // meters
pub const LATERAL_PENALTY: f64 = -1.; // meters
pub const OBSTRUCTED_PENALTY: f64 = -300000.; // meters
pub const INAPPROPRIATE_PENALTY: f64 = -20000.; // meters
pub const ONCOMING_CONFLICT_PENALTY: f64 = -1000.; // meters
pub const OBSTRUCTION_THRESHOLD: f64 = OBSTRUCTED_PENALTY * 0.5; // despite obstruction, additional utility may have been added
pub const SQUEEZE: f64 = 0.7;
pub const MAX_WAIT_TOLERANCE: f64 = 120.; // seconds
pub const LATERAL_SPEED_FACTOR: f64 = 0.4;
pub const MIN_STARTUP_DIST: f64 = 0.4; // meters
pub const DIST_FAR_AWAY: f64 = 10000.;
pub const DIST_BEHIND: f64 = 1000.;
pub const DIST_OVERLAP: f64 = -1.;
// C++ MSPModel::FORWARD / BACKWARD / UNDEFINED_DIRECTION
const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;

// C++ virtual overrides of MSPModel, dispatched through MSPModel::vt
static MSPMODEL_STRIPING_VT: MSPModelVt = MSPModelVt {
    add: striping_vt_add,
    clear_state: striping_vt_clear_state,
    remove: striping_vt_remove,
    using_internal_lanes: striping_vt_using_internal_lanes,
    get_active_number: striping_vt_get_active_number,
    load_state: striping_vt_load_state,
    has_pedestrians: striping_vt_has_pedestrians,
    blocked_at_dist: striping_vt_blocked_at_dist,
    next_blocking: striping_vt_next_blocking,
    using_shortcuts: msp_model_default_using_shortcuts,
};
fn striping_vt_blocked_at_dist(m: &mut MSPModel, ego: Ptr<SUMOTrafficObject>, lane: Ptr<MSLane>, vehCenter: f64, vehWidth: f64, oncomingGap: f64, collectBlockers: Ptr<Vec<Ptr<MSPerson>>>) -> bool {
    Ptr::from_mut(m).cast::<MSPModel_Interacting>().m().blocked_at_dist(ego, lane, vehCenter, vehWidth, oncomingGap, collectBlockers)
}
fn striping_vt_has_pedestrians(m: &mut MSPModel, lane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(m).cast::<MSPModel_Interacting>().m().has_pedestrians(lane)
}
fn striping_vt_next_blocking(m: &mut MSPModel, lane: Ptr<MSLane>, minPos: f64, minRight: f64, maxLeft: f64, stopTime: f64, bidi: bool) -> PersonDist {
    Ptr::from_mut(m).cast::<MSPModel_Interacting>().m().next_blocking(lane, minPos, minRight, maxLeft, stopTime, bidi)
}
fn striping_vt_load_state(m: &mut MSPModel, t: Ptr<MSTransportable>, s: Ptr<MSStageMoving>, state: &mut std::str::SplitWhitespace<'_>) -> Ptr<MSTransportableStateAdapter> {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().load_state(t, s, state)
}

/// PState overrides of the MSTransportableStateAdapter virtual methods (the others come from MSPModel_InteractingState)
static MSPMODEL_STRIPING_PSTATE_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_lat_offset: |a| Ptr::new(a).cast::<MSPModel_Striping_PState>().get_lat_offset(),
    get_edge_pos: |a, now| Ptr::new(a).cast::<MSPModel_InteractingState>().get_edge_pos(now),
    get_direction: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_direction(),
    get_position: |a, stage, now| Ptr::new(a).cast::<MSPModel_Striping_PState>().get_position(stage, now),
    get_angle: |a, stage, now| Ptr::new(a).cast::<MSPModel_Striping_PState>().get_angle(stage, now),
    get_waiting_time: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_waiting_time(),
    get_total_waiting_time: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_total_waiting_time(),
    get_speed: |a, _| Ptr::new(a).cast::<MSPModel_InteractingState>().get_speed(),
    get_next_edge: |a, stage| Ptr::new(a).cast::<MSPModel_Striping_PState>().get_next_edge(stage),
    move_to: |a, p, lane, lanePos, lanePosLat, t| Ptr::from_mut(a).cast::<MSPModel_Striping_PState>().m().move_to(p, lane, lanePos, lanePosLat, t),
    move_to_xy: |a, p, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, t| Ptr::from_mut(a).cast::<MSPModel_Striping_PState>().m().move_to_xy(p, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, t),
    is_jammed: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().is_jammed(),
    is_finished: adapter_default_is_finished,
    get_lane: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get_lane(),
    get_path_length: |a| Ptr::new(a).cast::<MSPModel_Striping_PState>().get_path_length(),
    save_state: |a, out| Ptr::from_mut(a).cast::<MSPModel_Striping_PState>().m().save_state(out),
    get_id: |a| Ptr::new(a).cast::<MSPModel_InteractingState>().get().unwrap().myPerson.get_id(),
    get_min_x: |a, g| Ptr::new(a).cast::<MSPModel_Striping_PState>().get().unwrap().get_min_x_base(g),
    get_max_x: |a, g| Ptr::new(a).cast::<MSPModel_Striping_PState>().get().unwrap().get_max_x_base(g),
    get_width: |a| Ptr::new(a).cast::<MSPModel_Striping_PState>().get().unwrap().get_width_base(),
    get_o_type: |a| Ptr::new(a).cast::<MSPModel_Striping_PState>().get().unwrap().get_o_type_base(),
};
/// PStateVehicle overrides getID, getMinX, getMaxX, getWidth, getOType (C++ PStateVehicle)
static MSPMODEL_STRIPING_PSTATE_VEHICLE_VT: MSTransportableStateAdapterVt = MSTransportableStateAdapterVt {
    get_id: |a| Ptr::new(a).cast::<MSPModel_Striping_PStateVehicle>().get().unwrap().get_id(),
    get_min_x: |a, g| Ptr::new(a).cast::<MSPModel_Striping_PStateVehicle>().get().unwrap().get_min_x(g),
    get_max_x: |a, g| Ptr::new(a).cast::<MSPModel_Striping_PStateVehicle>().get().unwrap().get_max_x(g),
    get_width: |a| Ptr::new(a).cast::<MSPModel_Striping_PStateVehicle>().get().unwrap().get_width(),
    get_o_type: |a| Ptr::new(a).cast::<MSPModel_Striping_PStateVehicle>().get().unwrap().get_o_type(),
    ..MSPMODEL_STRIPING_PSTATE_VT
};
fn striping_vt_add(m: &mut MSPModel, t: Ptr<MSTransportable>, s: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().add(t, s, now)
}
fn striping_vt_clear_state(m: &mut MSPModel) {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().clear_state()
}
fn striping_vt_remove(m: &mut MSPModel, state: Ptr<MSTransportableStateAdapter>) {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().remove(state)
}
fn striping_vt_using_internal_lanes(m: &mut MSPModel) -> bool {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().using_internal_lanes()
}
fn striping_vt_get_active_number(m: &mut MSPModel) -> i32 {
    Ptr::from_mut(m).cast::<MSPModel_Striping>().m().get_active_number()
}

impl MSPModel_Striping {
    /// C++: `void MSPModel_Striping::MSPModel_Striping(const OptionsCont &, MSNet *)`
    /// Constructor (it should not be necessary to construct more than one instance)
    // MSPModel_Striping::MSPModel_Striping(const OptionsCont& oc, MSNet* net) {
    //     myWalkingAreaDetail = oc.getInt("pedestrian.striping.walkingarea-detail");
    //     initWalkingAreaPaths(net);
    //     // configurable parameters
    //     stripeWidth = oc.getFloat("pedestrian.striping.stripe-width");
    //     MSVehicleType* defaultPedType = MSNet::getInstance()->getVehicleControl().getVType(DEFAULT_PEDTYPE_ID, nullptr, true);
    //     if (defaultPedType != nullptr && defaultPedType->getWidth() > stripeWidth) {
    //         WRITE_WARNINGF(TL("Pedestrian vType '%' width % is larger than pedestrian.striping.stripe-width and this may cause collisions with vehicles."),
    //                        DEFAULT_PEDTYPE_ID, defaultPedType->getWidth());
    //     }
    //
    //     dawdling = oc.getFloat("pedestrian.striping.dawdling");
    //     minGapToVehicle = oc.getFloat("pedestrian.striping.mingap-to-vehicle");
    //     RESERVE_FOR_ONCOMING_FACTOR = oc.getFloat("pedestrian.striping.reserve-oncoming");
    //     RESERVE_FOR_ONCOMING_FACTOR_JUNCTIONS = oc.getFloat("pedestrian.striping.reserve-oncoming.junctions");
    //     RESERVE_FOR_ONCOMING_MAX = oc.getFloat("pedestrian.striping.reserve-oncoming.max");
    //     // beginning with 1.20.0, sensible default speeds were set for crossings and walkingareas
    //     USE_NET_SPEEDS = net->getNetworkVersion() >= MMVersion(1, 20);
    //
    //     jamTime = string2time(oc.getString("pedestrian.striping.jamtime"));
    //     if (jamTime <= 0) {
    //         jamTime = SUMOTime_MAX;
    //     }
    //     jamTimeCrossing = string2time(oc.getString("pedestrian.striping.jamtime.crossing"));
    //     if (jamTimeCrossing <= 0) {
    //         jamTimeCrossing = SUMOTime_MAX;
    //     }
    //     jamTimeNarrow = string2time(oc.getString("pedestrian.striping.jamtime.narrow"));
    //     if (jamTimeNarrow <= 0) {
    //         jamTimeNarrow = SUMOTime_MAX;
    //     }
    //     jamFactor = oc.getFloat("pedestrian.striping.jamfactor");
    //     myLegacyPosLat = oc.getBool("pedestrian.striping.legacy-departposlat");
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont, net: Ptr<MSNet>) -> Self {
        let s = MSPModel_Striping {
            base: MSPModel_Interacting {
                base: MSPModel { vt: &MSPMODEL_STRIPING_VT },
                myNumActivePedestrians: 0,
                myActiveLanes: BTreeMap::new(),
                myAmActive: false,
                myApproachedCrossings: BTreeSet::new(),
            },
        };
        set_my_walking_area_detail(oc.get_int("pedestrian.striping.walkingarea-detail").unwrap());
        Self::init_walking_area_paths(net);
        // configurable parameters
        set_stripe_width(oc.get_float("pedestrian.striping.stripe-width").unwrap());
        let defaultPedType = net.m().get_vehicle_control().get_v_type(utils::common::sumo_vehicle_class::DEFAULT_PEDTYPE_ID, Ptr::null(), true);
        if !defaultPedType.is_null() && defaultPedType.get_width() > stripe_width() {
            utils::write_warningf!("Pedestrian vType '%' width % is larger than pedestrian.striping.stripe-width and this may cause collisions with vehicles.",
                utils::common::sumo_vehicle_class::DEFAULT_PEDTYPE_ID, defaultPedType.get_width());
        }

        set_dawdling(oc.get_float("pedestrian.striping.dawdling").unwrap());
        set_min_gap_to_vehicle(oc.get_float("pedestrian.striping.mingap-to-vehicle").unwrap());
        set_reserve_for_oncoming_factor(oc.get_float("pedestrian.striping.reserve-oncoming").unwrap());
        set_reserve_for_oncoming_factor_junctions(oc.get_float("pedestrian.striping.reserve-oncoming.junctions").unwrap());
        set_reserve_for_oncoming_max(oc.get_float("pedestrian.striping.reserve-oncoming.max").unwrap());
        // beginning with 1.20.0, sensible default speeds were set for crossings and walkingareas
        set_use_net_speeds(net.get_network_version() >= (1, 20.0));

        set_jam_time(utils::common::time::string2time(&oc.get_string("pedestrian.striping.jamtime").unwrap()).unwrap());
        if jam_time() <= 0 {
            set_jam_time(utils::common::time::SUMOTIME_MAX);
        }
        set_jam_time_crossing(utils::common::time::string2time(&oc.get_string("pedestrian.striping.jamtime.crossing").unwrap()).unwrap());
        if jam_time_crossing() <= 0 {
            set_jam_time_crossing(utils::common::time::SUMOTIME_MAX);
        }
        set_jam_time_narrow(utils::common::time::string2time(&oc.get_string("pedestrian.striping.jamtime.narrow").unwrap()).unwrap());
        if jam_time_narrow() <= 0 {
            set_jam_time_narrow(utils::common::time::SUMOTIME_MAX);
        }
        set_jam_factor(oc.get_float("pedestrian.striping.jamfactor").unwrap());
        set_my_legacy_pos_lat(oc.get_bool("pedestrian.striping.legacy-departposlat").unwrap());
        s
    }

    /// C++: `void MSPModel_Striping::~MSPModel_Striping() noexcept` [virtual]
    // MSPModel_Striping::~MSPModel_Striping() {
    //     myWalkingAreaPaths.clear(); // need to recompute when lane pointers change
    //     myWalkingAreaFoes.clear();
    //     myMinNextLengths.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        MY_WALKING_AREA_PATHS.with(|m| m.borrow_mut().clear()); // need to recompute when lane pointers change
        MY_WALKING_AREA_FOES.with(|m| m.borrow_mut().clear());
        MY_MIN_NEXT_LENGTHS.with(|m| m.borrow_mut().clear());
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_Striping::add(MSTransportable *, MSStageMoving *, SUMOTime)` [virtual]
    /// register the given person as a pedestrian
    // MSTransportableStateAdapter*
    // MSPModel_Striping::add(MSTransportable* transportable, MSStageMoving* stage, SUMOTime) {
    //     if (!transportable->isPerson()) {
    //         // containers are not supported (TODO add a warning here?)
    //         return nullptr;
    //     }
    //     MSPerson* person = static_cast<MSPerson*>(transportable);
    //     MSNet* net = MSNet::getInstance();
    //     if (!myAmActive) {
    //         net->getBeginOfTimestepEvents()->addEvent(new MovePedestrians(this), net->getCurrentTimeStep() + DELTA_T);
    //         myAmActive = true;
    //     }
    //     assert(person->getCurrentStageType() == MSStageType::WALKING);
    //     const MSLane* lane = stage->checkDepartLane(person->getEdge(), person->getVClass(), stage->getDepartLane(), person->getID());
    //     if (lane == nullptr) {
    //         const char* error = TL("Person '%' could not find sidewalk on edge '%', time=%.");
    //         if (OptionsCont::getOptions().getBool("ignore-route-errors")) {
    //             WRITE_WARNINGF(error, person->getID(), person->getEdge()->getID(), time2string(net->getCurrentTimeStep()));
    //             return nullptr;
    //         } else {
    //             throw ProcessError(TLF(error, person->getID(), person->getEdge()->getID(), time2string(net->getCurrentTimeStep())));
    //         }
    //     }
    //     PState* ped = new PState(person, stage, lane);
    //     myActiveLanes[lane].push_back(ped);
    //     myNumActivePedestrians++;
    //     return ped;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
        if !transportable.is_person() {
            // containers are not supported (TODO add a warning here?)
            return Ptr::null();
        }
        let person: Ptr<MSPerson> = transportable.cast::<MSPerson>();
        let net = MSNet::get_instance();
        if !self.myAmActive {
            let cmd: Ptr<dyn crate::microsim::ms_event_control::Command> =
                Ptr::from_mut(Box::leak(Box::new(MSPModel_Striping_MovePedestrians::new(Ptr::from_mut(self)))) as &mut dyn crate::microsim::ms_event_control::Command);
            net.m().get_begin_of_timestep_events().m().add_event(cmd, net.get_current_time_step() + utils::common::time::delta_t());
            self.myAmActive = true;
        }
        // C++ `assert(person->getCurrentStageType() == MSStageType::WALKING)` is
        // compiled out in the official (NDEBUG) binary; multi-modal stage changes
        // can reach here with a non-walking current stage, so keep release behaviour.
        let lane = MSStageMoving::check_depart_lane(person.get_edge(), person.get_v_class(), stage.get_depart_lane(), person.get_id());
        if lane.is_null() {
            if utils::options::options_cont::get_bool("ignore-route-errors") {
                utils::write_warningf!("Person '%' could not find sidewalk on edge '%', time=%.", person.get_id(), person.get_edge().get_id(), utils::common::time::time2string(net.get_current_time_step()));
                return Ptr::null();
            } else {
                panic!("{}", utils::tlf!("Person '%' could not find sidewalk on edge '%', time=%.", person.get_id(), person.get_edge().get_id(), utils::common::time::time2string(net.get_current_time_step())));
            }
        }
        let ped: Ptr<MSPModel_InteractingState> = Ptr::from_mut(Box::leak(Box::new(MSPModel_Striping_PState::new(person, stage, lane)))).cast::<MSPModel_InteractingState>();
        self.myActiveLanes.entry(MSPModel_Interacting_LaneKey(lane)).or_default().push(ped);
        self.myNumActivePedestrians += 1;
        ped.cast::<MSTransportableStateAdapter>()
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_Striping::loadState(MSTransportable *, MSStageMoving *, std::istringstream &)` [virtual]
    /// load the state of the given transportable
    // MSTransportableStateAdapter*
    // MSPModel_Striping::loadState(MSTransportable* transportable, MSStageMoving* stage, std::istringstream& in) {
    //     MSPerson* person = static_cast<MSPerson*>(transportable);
    //     MSNet* net = MSNet::getInstance();
    //     if (!myAmActive) {
    //         net->getBeginOfTimestepEvents()->addEvent(new MovePedestrians(this), SIMSTEP);
    //         myAmActive = true;
    //     }
    //     PState* ped = new PState(person, stage, &in);
    //     myActiveLanes[ped->getLane()].push_back(ped);
    //     myNumActivePedestrians++;
    //     return ped;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, in_: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) -> Ptr<MSTransportableStateAdapter> {
        let person: Ptr<MSPerson> = transportable.cast::<MSPerson>();
        let net = MSNet::get_instance();
        if !self.myAmActive {
            let cmd: Ptr<dyn crate::microsim::ms_event_control::Command> =
                Ptr::from_mut(Box::leak(Box::new(MSPModel_Striping_MovePedestrians::new(Ptr::from_mut(self)))) as &mut dyn crate::microsim::ms_event_control::Command);
            net.m().get_begin_of_timestep_events().m().add_event(cmd, net.get_current_time_step());
            self.myAmActive = true;
        }
        let state = MSPModel_Striping_PState::new_2(person, stage, in_);
        let lane = state.base.myLane;
        let ped: Ptr<MSPModel_InteractingState> = Ptr::from_mut(Box::leak(Box::new(state))).cast::<MSPModel_InteractingState>();
        self.myActiveLanes.entry(MSPModel_Interacting_LaneKey(lane)).or_default().push(ped);
        self.myNumActivePedestrians += 1;
        ped.cast::<MSTransportableStateAdapter>()
    }

    /// C++: `double MSPModel_Striping::posLatConversion(const double, const double)` [static]
    /// Convert the striping to the vehicle lateral position and vice versa.
    // inline static double posLatConversion(const double posLat, const double laneWidth) {
    //         return .5 * (laneWidth - stripeWidth) - posLat;
    //     }
    #[allow(unused_variables)]
    pub fn pos_lat_conversion(posLat: f64, laneWidth: f64) -> f64 {
        0.5 * (laneWidth - stripe_width()) - posLat
    }

    /// C++: `void MSPModel_Striping::moveInDirection(SUMOTime, std::set<MSPerson *> &, int)` [protected]
    /// move all pedestrians forward and advance to the next lane if applicable
    // void
    // MSPModel_Striping::moveInDirection(SUMOTime currentTime, std::set<MSPerson*>& changedLane, int dir) {
    //     for (ActiveLanes::iterator it_lane = myActiveLanes.begin(); it_lane != myActiveLanes.end(); ++it_lane) {
    //         const MSLane* lane = it_lane->first;
    //         Pedestrians& pedestrians = it_lane->second;
    //         if (pedestrians.size() == 0) {
    //             continue;
    //         }
    //         //std::cout << SIMTIME << ">>> lane=" << lane->getID() << " numPeds=" << pedestrians.size() << "\n";
    //         if (lane->isWalkingArea()) {
    //             const double lateral_offset = (lane->getWidth() - stripeWidth) * 0.5;
    //             const double minY = stripeWidth * - 0.5 + NUMERICAL_EPS;
    //             const double maxY = stripeWidth * (numStripes(lane) - 0.5) - NUMERICAL_EPS;
    //             const WalkingAreaPath* debugPath = nullptr;
    //             // need to handle each walkingAreaPath separately and transform
    //             // coordinates beforehand
    //             std::set<const WalkingAreaPath*, walkingarea_path_sorter> paths;
    //             for (Pedestrians::iterator it = pedestrians.begin(); it != pedestrians.end(); ++it) {
    //                 const PState* p = static_cast<PState*>(*it);
    //                 assert(p->myWalkingAreaPath != 0);
    //                 if (p->getDirection() == dir) {
    //                     paths.insert(p->myWalkingAreaPath);
    //                     if DEBUGCOND(*p) {
    //                         debugPath = p->myWalkingAreaPath;
    //                         std::cout << SIMTIME << " debugging WalkingAreaPath from=" << debugPath->from->getID() << " to=" << debugPath->to->getID() << " minY=" << minY << " maxY=" << maxY << " latOffset=" << lateral_offset << "\n";
    //                     }
    //                 }
    //             }
    //             const double usableWidth = (numStripes(lane) - 1) * stripeWidth;
    //             for (std::set<const WalkingAreaPath*, walkingarea_path_sorter>::iterator it = paths.begin(); it != paths.end(); ++it) {
    //                 const WalkingAreaPath* path = *it;
    //                 Pedestrians toDelete;
    //                 Pedestrians transformedPeds;
    //                 transformedPeds.reserve(pedestrians.size());
    //                 for (Pedestrians::iterator it_p = pedestrians.begin(); it_p != pedestrians.end(); ++it_p) {
    //                     PState* p = static_cast<PState*>(*it_p);
    //                     if (p->myWalkingAreaPath == path) {
    //                         transformedPeds.push_back(p);
    //                         if (path == debugPath) std::cout << "  ped=" << p->getPerson()->getID() << "  relX=" << p->getEdgePos(0) << " relY=" << p->getPosLat() << " (untransformed), vecCoord="
    //                                                              << path->shape.transformToVectorCoordinates(p->getPosition(*p->getStage(), -1)) << "\n";
    //                     } else if (p->myWalkingAreaPath->from == path->to && p->myWalkingAreaPath->to == path->from) {
    //                         if (p->myWalkingAreaPath->dir != path->dir) {
    //                             // opposite direction is already in the correct coordinate system
    //                             transformedPeds.push_back(p);
    //                             if (path == debugPath) std::cout << "  ped=" << p->getPerson()->getID() << "  relX=" << p->getEdgePos(0) << " relY=" << p->getPosLat() << " (untransformed), vecCoord="
    //                                                                  << path->shape.transformToVectorCoordinates(p->getPosition(*p->getStage(), -1)) << "\n";
    //                         } else {
    //                             // x position must be reversed
    //                             PState* tp = new PState(*p);
    //                             tp->reverse(path->length, usableWidth);
    //                             toDelete.push_back(tp);
    //                             transformedPeds.push_back(tp);
    //                             if (path == debugPath) std::cout << "  ped=" << p->getPerson()->getID() << "  relX=" << p->getEdgePos(0) << " relY=" << p->getPosLat() << " (semi-transformed), vecCoord="
    //                                                                  << path->shape.transformToVectorCoordinates(p->getPosition(*p->getStage(), -1)) << "\n";
    //                         }
    //                     } else {
    //                         const Position relPos = path->shape.transformToVectorCoordinates(p->getPosition(*p->getStage(), -1));
    //                         const double newY = relPos.y() + lateral_offset;
    //                         if (relPos != Position::INVALID && newY >= minY && newY <= maxY) {
    //                             PState* tp = new PState(*p);
    //                             tp->reset(relPos.x(), newY);
    //                             toDelete.push_back(tp);
    //                             transformedPeds.push_back(tp);
    //                             if (path == debugPath) {
    //                                 std::cout << "  ped=" << p->getPerson()->getID() << "  relX=" << relPos.x() << " relY=" << newY << " (transformed), vecCoord=" << relPos << "\n";
    //                             }
    //                         } else {
    //                             if (path == debugPath) {
    //                                 std::cout << "  ped=" << p->getPerson()->getID() << "  relX=" << relPos.x() << " relY=" << newY << " (invalid), vecCoord=" << relPos << "\n";
    //                             }
    //                         }
    //                     }
    //                 }
    //                 auto itFoe = myWalkingAreaFoes.find(&lane->getEdge());
    //                 if (itFoe != myWalkingAreaFoes.end()) {
    //                     // add vehicle foes on paths which cross this walkingarea
    //                     // translate the vehicle into a number of dummy-pedestrians
    //                     // that occupy the same space
    //                     for (const MSLane* foeLane : itFoe->second) {
    //                         for (auto itVeh = foeLane->anyVehiclesBegin(); itVeh != foeLane->anyVehiclesEnd(); ++itVeh) {
    //                             const MSVehicle* veh = *itVeh;
    //                             const double vehWidth = veh->getVehicleType().getWidth();
    //                             Boundary relCorners;
    //                             Position relFront = path->shape.transformToVectorCoordinates(veh->getPosition(), true);
    //                             Position relBack = path->shape.transformToVectorCoordinates(veh->getBackPosition(), true);
    //                             if (relFront == Position::INVALID) {
    //                                 WRITE_WARNINGF("Could not vehicle '%' front position % onto walkingarea '%' path=%, time=%.",
    //                                                veh->getID(), veh->getPosition(), lane->getID(), path->shape, time2string(SIMSTEP));
    //                             }
    //                             if (relBack == Position::INVALID) {
    //                                 WRITE_WARNINGF("Could not vehicle '%' back position % onto walkingarea '%' path=%, time=%.",
    //                                                veh->getID(), veh->getBackPosition(), lane->getID(), path->shape, time2string(SIMSTEP));
    //                             }
    //                             PositionVector relCenter;
    //                             relCenter.push_back(relFront);
    //                             relCenter.push_back(relBack);
    //                             relCenter.move2side(vehWidth / 2);
    //                             relCorners.add(relCenter[0]);
    //                             relCorners.add(relCenter[1]);
    //                             relCenter.move2side(-vehWidth);
    //                             relCorners.add(relCenter[0]);
    //                             relCorners.add(relCenter[1]);
    //                             // persons should require less gap than the vehicles to prevent getting stuck
    //                             // when a vehicles moves towards them
    //                             relCorners.growWidth(SAFETY_GAP / 2);
    //                             const double xWidth = relCorners.getWidth();
    //                             const double vehYmin = MAX2(minY - lateral_offset, relCorners.ymin());
    //                             const double vehYmax = MIN2(maxY - lateral_offset, relCorners.ymax());
    //                             const double xCenter = relCorners.getCenter().x();
    //                             Position yMinPos(xCenter, vehYmin);
    //                             Position yMaxPos(xCenter, vehYmax);
    //                             const bool addFront = addVehicleFoe(veh, lane, yMinPos, dir * xWidth, 0, lateral_offset, minY, maxY, toDelete, transformedPeds);
    //                             const bool addBack = addVehicleFoe(veh, lane, yMaxPos, dir * xWidth, 0, lateral_offset, minY, maxY, toDelete, transformedPeds);
    //                             if (path == debugPath) {
    //                                 std::cout << "  veh=" << veh->getID()
    //                                           << " corners=" << relCorners
    //                                           << " xWidth=" << xWidth
    //                                           << " ymin=" << relCorners.ymin()
    //                                           << " ymax=" << relCorners.ymax()
    //                                           << " vehYmin=" << vehYmin
    //                                           << " vehYmax=" << vehYmax
    //                                           << "\n";
    //                             }
    //                             if (addFront && addBack) {
    //                                 // add in-between positions
    //                                 const double yDist = vehYmax - vehYmin;
    //                                 for (double dist = stripeWidth; dist < yDist; dist += stripeWidth) {
    //                                     const double relDist = dist / yDist;
    //                                     Position between = (yMinPos * relDist) + (yMaxPos * (1 - relDist));
    //                                     if (path == debugPath) {
    //                                         std::cout << "  vehBetween=" << veh->getID() << " pos=" << between << "\n";
    //                                     }
    //                                     addVehicleFoe(veh, lane, between, dir * xWidth, stripeWidth, lateral_offset, minY, maxY, toDelete, transformedPeds);
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 moveInDirectionOnLane(transformedPeds, lane, currentTime, changedLane, dir, path == debugPath);
    //                 arriveAndAdvance(pedestrians, currentTime, changedLane, dir);
    //                 // clean up
    //                 for (Pedestrians::iterator it_p = toDelete.begin(); it_p != toDelete.end(); ++it_p) {
    //                     delete *it_p;
    //                 }
    //             }
    //         } else {
    //             moveInDirectionOnLane(pedestrians, lane, currentTime, changedLane, dir, false);
    //             arriveAndAdvance(pedestrians, currentTime, changedLane, dir);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_in_direction(&mut self, currentTime: SUMOTime, changedLane: &mut BTreeSet<Ptr<MSPerson>>, dir: i32) {
        // GUI-only debug dump: DEBUGCOND is person/lane isSelected(), always false in a headless run, so debugPath stays null
        // shortcut: the C++ holds a reference into myActiveLanes; here a lane's vector is taken out while it is advanced and put back afterwards
        // C++ iterates the live std::map: lanes inserted by arriveAndAdvance are visited when their key is larger than the current one
        let mut cursor: Option<MSPModel_Interacting_LaneKey> = None;
        loop {
            let from = match cursor {
                Some(c) => std::ops::Bound::Excluded(c),
                None => std::ops::Bound::Unbounded,
            };
            let (key, lane) = match self.myActiveLanes.range((from, std::ops::Bound::Unbounded)).next() {
                Some((k, _)) => (*k, k.0),
                None => break,
            };
            cursor = Some(key);
            let mut pedestrians = std::mem::take(self.myActiveLanes.get_mut(&key).unwrap());
            if pedestrians.len() == 0 {
                self.put_back_lane(lane, pedestrians);
                continue;
            }
            //std::cout << SIMTIME << ">>> lane=" << lane->getID() << " numPeds=" << pedestrians.size() << "\n";
            if lane.is_walking_area() {
                let lateral_offset = (lane.get_width() - stripe_width()) * 0.5;
                let minY = stripe_width() * -0.5 + utils::geom::NUMERICAL_EPS;
                let maxY = stripe_width() * (Self::num_stripes(lane) as f64 - 0.5) - utils::geom::NUMERICAL_EPS;
                // need to handle each walkingAreaPath separately and transform
                // coordinates beforehand
                // determinism: std::set ordered by walkingarea_path_sorter (numerical IDs of from/to), first insertion wins
                let mut paths: BTreeMap<(i32, i32), Ptr<MSPModel_Striping_WalkingAreaPath>> = BTreeMap::new();
                for it in pedestrians.iter() {
                    let p = (*it).cast::<MSPModel_Striping_PState>();
                    debug_assert!(!p.myWalkingAreaPath.is_null());
                    if p.get_direction() == dir {
                        let wap = p.myWalkingAreaPath;
                        paths.entry((wap.from.get_numerical_id(), wap.to.get_numerical_id())).or_insert(wap);
                    }
                }
                let usableWidth = (Self::num_stripes(lane) - 1) as f64 * stripe_width();
                let pathList: Vec<Ptr<MSPModel_Striping_WalkingAreaPath>> = paths.values().copied().collect();
                for path in pathList {
                    // owners of the PState copies (C++ `delete` after the path is processed)
                    let mut owned: Vec<Box<dyn std::any::Any>> = Vec::new();
                    let mut toDelete: MSPModel_Interacting_Pedestrians = Vec::new();
                    let mut transformedPeds: MSPModel_Interacting_Pedestrians = Vec::with_capacity(pedestrians.len());
                    for it_p in pedestrians.iter() {
                        let p = (*it_p).cast::<MSPModel_Striping_PState>();
                        if p.myWalkingAreaPath == path {
                            transformedPeds.push(*it_p);
                        } else if p.myWalkingAreaPath.from == path.to && p.myWalkingAreaPath.to == path.from {
                            if p.myWalkingAreaPath.dir != path.dir {
                                // opposite direction is already in the correct coordinate system
                                transformedPeds.push(*it_p);
                            } else {
                                // x position must be reversed
                                let mut tp = Box::new(copy_pstate(&*p));
                                tp.reverse(path.length, usableWidth);
                                let tpp = Ptr::from_mut(&mut *tp).cast::<MSPModel_InteractingState>();
                                owned.push(tp);
                                toDelete.push(tpp);
                                transformedPeds.push(tpp);
                            }
                        } else {
                            let relPos = path.shape.transform_to_vector_coordinates(&p.get_position(&*p.get_stage(), -1), false);
                            let newY = relPos.y() + lateral_offset;
                            if relPos != Position::INVALID && newY >= minY && newY <= maxY {
                                let mut tp = Box::new(copy_pstate(&*p));
                                tp.reset(relPos.x(), newY);
                                let tpp = Ptr::from_mut(&mut *tp).cast::<MSPModel_InteractingState>();
                                owned.push(tp);
                                toDelete.push(tpp);
                                transformedPeds.push(tpp);
                            }
                        }
                    }
                    let foes: Option<Vec<Ptr<MSLane>>> = MY_WALKING_AREA_FOES.with(|m| m.borrow().get(&Ptr::from_mut(lane.m().get_edge())).cloned());
                    if let Some(foeLanes) = foes {
                        // add vehicle foes on paths which cross this walkingarea
                        // translate the vehicle into a number of dummy-pedestrians
                        // that occupy the same space
                        for foeLane in foeLanes {
                            let end = foeLane.any_vehicles_end();
                            let mut itVeh = foeLane.any_vehicles_begin();
                            while itVeh.ne_op(&end) {
                                let veh = itVeh.mul_op();
                                let vehWidth = veh.get_vehicle_type().get_width();
                                let mut relCorners = Boundary::new();
                                let relFront = path.shape.transform_to_vector_coordinates(&veh.get_position(0.), true);
                                let relBack = path.shape.transform_to_vector_coordinates(&veh.get_back_position(), true);
                                if relFront == Position::INVALID {
                                    utils::write_warningf!("Could not vehicle '%' front position % onto walkingarea '%' path=%, time=%.",
                                        veh.get_id(), veh.get_position(0.), lane.get_id(), path.shape.to_string(), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                                }
                                if relBack == Position::INVALID {
                                    utils::write_warningf!("Could not vehicle '%' back position % onto walkingarea '%' path=%, time=%.",
                                        veh.get_id(), veh.get_back_position(), lane.get_id(), path.shape.to_string(), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                                }
                                let mut relCenter = PositionVector::new();
                                relCenter.push(relFront);
                                relCenter.push(relBack);
                                relCenter.move2side(vehWidth / 2., 100.);
                                relCorners.add_pos(&relCenter[0]);
                                relCorners.add_pos(&relCenter[1]);
                                relCenter.move2side(-vehWidth, 100.);
                                relCorners.add_pos(&relCenter[0]);
                                relCorners.add_pos(&relCenter[1]);
                                // persons should require less gap than the vehicles to prevent getting stuck
                                // when a vehicles moves towards them
                                relCorners.grow_width(SAFETY_GAP / 2.);
                                let xWidth = relCorners.get_width();
                                let vehYmin = f64::max(minY - lateral_offset, relCorners.ymin());
                                let vehYmax = f64::min(maxY - lateral_offset, relCorners.ymax());
                                let xCenter = relCorners.get_center().x();
                                let yMinPos = Position::new(xCenter, vehYmin);
                                let yMaxPos = Position::new(xCenter, vehYmax);
                                let addFront = Self::add_vehicle_foe(veh, lane, &yMinPos, dir as f64 * xWidth, 0., lateral_offset, minY, maxY, &mut toDelete, &mut transformedPeds, &mut owned);
                                let addBack = Self::add_vehicle_foe(veh, lane, &yMaxPos, dir as f64 * xWidth, 0., lateral_offset, minY, maxY, &mut toDelete, &mut transformedPeds, &mut owned);
                                if addFront && addBack {
                                    // add in-between positions
                                    let yDist = vehYmax - vehYmin;
                                    let mut dist = stripe_width();
                                    while dist < yDist {
                                        let relDist = dist / yDist;
                                        let between = (yMinPos * relDist) + (yMaxPos * (1. - relDist));
                                        Self::add_vehicle_foe(veh, lane, &between, dir as f64 * xWidth, stripe_width(), lateral_offset, minY, maxY, &mut toDelete, &mut transformedPeds, &mut owned);
                                        dist += stripe_width();
                                    }
                                }
                                itVeh.op__2();
                            }
                        }
                    }
                    self.move_in_direction_on_lane(&mut transformedPeds, lane, currentTime, changedLane, dir, false);
                    self.arrive_and_advance(&mut pedestrians, currentTime, changedLane, dir);
                    // clean up: the owned PState copies are dropped here
                    drop(owned);
                }
            } else {
                self.move_in_direction_on_lane(&mut pedestrians, lane, currentTime, changedLane, dir, false);
                self.arrive_and_advance(&mut pedestrians, currentTime, changedLane, dir);
            }
            self.put_back_lane(lane, pedestrians);
        }
    }

    /// Returns a lane's pedestrian vector to myActiveLanes (see the shortcut in moveInDirection)
    fn put_back_lane(&mut self, lane: Ptr<MSLane>, pedestrians: MSPModel_Interacting_Pedestrians) {
        let key = MSPModel_Interacting_LaneKey(lane);
        let extra = std::mem::replace(self.myActiveLanes.entry(key).or_default(), pedestrians);
        self.myActiveLanes.get_mut(&key).unwrap().extend(extra);
    }

    /// C++: `void MSPModel_Striping::moveInDirectionOnLane(Pedestrians &, const MSLane *, SUMOTime, std::set<MSPerson *> &, int, bool)` [protected]
    /// move pedestrians forward on one lane
    // void
    // MSPModel_Striping::moveInDirectionOnLane(Pedestrians& pedestrians, const MSLane* lane, SUMOTime currentTime, std::set<MSPerson*>& changedLane, int dir, bool debug) {
    //     const int stripes = numStripes(lane);
    //     //std::cout << " laneWidth=" << lane->getWidth() << " stripeWidth=" << stripeWidth << " stripes=" << stripes << "\n";
    //     Obstacles obs(stripes, Obstacle(dir)); // continuously updated
    //     NextLanesObstacles nextLanesObs; // continuously updated
    //     sort(pedestrians.begin(), pedestrians.end(), by_xpos_sorter(dir));
    //
    //     Obstacles crossingVehs(stripes, Obstacle(dir));
    //     bool hasCrossingVehObs = false;
    //     if (lane->isCrossing()) {
    //         // assume that vehicles will brake when already on the crossing
    //         hasCrossingVehObs = addCrossingVehs(lane, stripes, 0, dir, crossingVehs, true, false);
    //     }
    //
    //     for (int ii = 0; ii < (int)pedestrians.size(); ++ii) {
    //         PState& p = *static_cast<PState*>(pedestrians[ii]);
    //         UNUSED_PARAMETER(debug);
    //         //if (debug) {
    //         //    std::cout << SIMTIME << " CHECKING d=" << dir << " p=" << p.getID() << " relX=" << p.myRelX << " xmin=" << p.getMinX() << " xmax=" << p.getMaxX() << " pdir=" << p.getDirection() << "\n";
    //         //}
    //         Obstacles currentObs = obs;
    //         if (p.getDirection() != dir || changedLane.count(p.getPerson()) != 0 || p.getRemotePosition() != Position::INVALID) {
    //             if (!p.isWaitingToEnter() && !p.isJammed()) {
    //                 //if DEBUGCOND(p) {
    //                 //    std::cout << "   obs=" << p.getPerson()->getID() << "  y=" << p.getPosLat() << "  stripe=" << p.stripe() << " oStripe=" << p.otherStripe() << "\n";
    //                 //}
    //                 Obstacle o(p);
    //                 if (p.getDirection() != dir && p.getSpeed() == 0.) {
    //                     // ensure recognition of oncoming
    //                     o.speed = (p.getDirection() == FORWARD ? 0.1 : -0.1);
    //                 }
    //                 if (o.closer(obs[p.stripe()], dir)) {
    //                     obs[p.stripe()] = o;
    //                 }
    //                 if (o.closer(obs[p.otherStripe()], dir)) {
    //                     obs[p.otherStripe()] = o;
    //                 }
    //             }
    //             continue;
    //         }
    //         if DEBUGCOND(p) {
    //             std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  currentObs=";
    //             gDebugFlag1 = true;
    //             DEBUG_PRINT(currentObs);
    //         }
    //         const MSLane* nextLane = p.myNLI.lane;
    //         const MSLink* link = p.myNLI.link;
    //         const double dist = p.distToLaneEnd();
    //         const double speed(p.getStage()->getConfiguredSpeed() >= 0
    //                            ? p.getStage()->getConfiguredSpeed()
    //                            : ((nextLane != nullptr && (USE_NET_SPEEDS || nextLane->isNormal() || nextLane->isInternal()))
    //                               ? nextLane->getVehicleMaxSpeed(p.getPerson())
    //                               : p.getStage()->getMaxSpeed(p.getPerson())));
    //
    //
    //         if (nextLane != nullptr && dist <= LOOKAHEAD_ONCOMING_DIST) {
    //             const double currentLength = (p.myWalkingAreaPath == nullptr ? lane->getLength() : p.myWalkingAreaPath->length);
    //             const Obstacles& nextObs = getNextLaneObstacles(
    //                                            nextLanesObs, lane, nextLane, stripes,
    //                                            p.myNLI.dir, currentLength, dir);
    //
    //             if DEBUGCOND(p) {
    //                 std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  nextObs=";
    //                 DEBUG_PRINT(nextObs);
    //             }
    //             p.mergeObstacles(currentObs, nextObs);
    //         }
    //         if DEBUGCOND(p) {
    //             std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  obsWithNext=";
    //             DEBUG_PRINT(currentObs);
    //         }
    //         p.mergeObstacles(currentObs, getNeighboringObstacles(pedestrians, ii, stripes));
    //         if DEBUGCOND(p) {
    //             std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  obsWithNeigh=";
    //             DEBUG_PRINT(currentObs);
    //         }
    //         // time gap to pass the intersection ahead of a vehicle.
    //         const double passingLength = p.getLength() + p.getPerson()->getTimegapCrossing() * speed;
    //         // check link state
    //         if DEBUGCOND(p) {
    //             gDebugFlag1 = true; // get debug output from MSLink
    //             std::cout << "   link=" << (link == nullptr ? "NULL" : link->getViaLaneOrLane()->getID())
    //                       << " dist=" << dist << " d2=" << dist - p.getMinGap() << " la=" << LOOKAHEAD_SAMEDIR* speed << "\n";
    //         }
    //         if (link != nullptr
    //                 // only check close before junction, @todo we should take deceleration into account here
    //                 && dist - p.getMinGap() < LOOKAHEAD_SAMEDIR * speed
    //                 // persons move before vehicles so we subtract DELTA_TO because they cannot rely on vehicles having passed the intersection in the current time step
    //                 && (!link->opened(currentTime - DELTA_T, speed, speed, passingLength, p.getImpatience(), speed, 0, 0, nullptr, p.ignoreRed(link), p.getPerson())
    //                     || p.stopForYellow(link))) {
    //             // prevent movement passed a closed link
    //             Obstacles closedLink(stripes, Obstacle(p.getEdgePos(0) + dir * (dist - NUMERICAL_EPS), 0, OBSTACLE_LINKCLOSED, "closedLink_" + link->getViaLaneOrLane()->getID(), 0));
    //             p.mergeObstacles(currentObs, closedLink);
    //             if DEBUGCOND(p) {
    //                 std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  obsWithTLS=";
    //                 DEBUG_PRINT(currentObs);
    //             }
    //             // consider rerouting over another crossing
    //             if (p.myWalkingAreaPath != nullptr) {
    //                 // @todo actually another path would be needed starting at the current position
    //                 const MSLane* oldNext = p.myNLI.lane;
    //                 p.myNLI = getNextLane(p, p.getLane(), p.myWalkingAreaPath->from);
    //                 if (p.myNLI.lane != oldNext && oldNext->isPriorityCrossing()) {
    //                     unregisterCrossingApproach(p, oldNext);
    //                 }
    //             }
    //         }
    //         if DEBUGCOND(p) {
    //             gDebugFlag1 = false;
    //         }
    //         if (&lane->getEdge() == p.getStage()->getDestination() && p.getStage()->getDestinationStop() != nullptr) {
    //             Obstacles arrival;
    //             if (p.getStage()->getDestinationStop()->getWaitingCapacity() > p.getStage()->getDestinationStop()->getNumWaitingPersons() ||
    //                     (!p.getStage()->getDestinationStop()->checkPersonCapacity())) {
    //                 arrival = Obstacles(stripes, Obstacle(p.getStage()->getArrivalPos() + dir * p.getMinGap(), 0, OBSTACLE_ARRIVALPOS, "arrival", 0));
    //             } else {
    //                 arrival = Obstacles(stripes, Obstacle(p.getStage()->getArrivalPos() - dir * p.getMinGap(), 0, OBSTACLE_ARRIVALPOS, "arrival_blocked", 0));
    //             }
    //             p.mergeObstacles(currentObs, arrival);
    //         }
    //
    //         if (lane->getVehicleNumberWithPartials() > 0) {
    //             // react to vehicles on the same lane
    //             // @todo: improve efficiency by using the same iterator for all pedestrians on this lane
    //             Obstacles vehObs = getVehicleObstacles(lane, dir, &p);
    //             p.mergeObstacles(currentObs, vehObs);
    //             if DEBUGCOND(p) {
    //                 std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  obsWithVehs=";
    //                 DEBUG_PRINT(currentObs);
    //             }
    //         }
    //         if (hasCrossingVehObs) {
    //             p.mergeObstacles(currentObs, crossingVehs);
    //             if DEBUGCOND(p) {
    //                 std::cout << SIMTIME << " ped=" << p.getPerson()->getID() << "  obsWithVehs2=";
    //                 DEBUG_PRINT(currentObs);
    //             }
    //         }
    //
    //         // walk, taking into account all obstacles
    //         p.walk(currentObs);
    //         gDebugFlag1 = false;
    //         if (!p.isWaitingToEnter() && !p.isJammed()) {
    //             Obstacle o(p);
    //             if (o.closer(obs[p.stripe()], dir)) {
    //                 obs[p.stripe()] = o;
    //             }
    //             if (o.closer(obs[p.otherStripe()], dir)) {
    //                 obs[p.otherStripe()] = o;
    //             }
    //             if (MSGlobals::gCheck4Accidents && p.myWalkingAreaPath == nullptr && !p.isJammed()) {
    //                 for (int coll = 0; coll < ii; ++coll) {
    //                     PState& c = *static_cast<PState*>(pedestrians[coll]);
    //                     if (!c.isWaitingToEnter() && c.myWalkingAreaPath == nullptr && !c.isJammed()) {
    //                         if (c.stripe() == p.stripe() || p.stripe() == c.otherStripe() || p.otherStripe() == c.stripe() || p.otherStripe() == c.otherStripe()) {
    //                             Obstacle cObs(c);
    //                             // we check only for real collisions, no min gap violations
    //                             if (p.distanceTo(cObs, false) == DIST_OVERLAP) {
    //                                 WRITE_WARNING("Collision of person '" + p.getPerson()->getID() + "' and person '" + c.getPerson()->getID()
    //                                               + "', lane='" + lane->getID() + "', time=" + time2string(currentTime) + ".");
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         //std::cout << SIMTIME << p.getPerson()->getID() << " lane=" << lane->getID() << " x=" << p.myRelX << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_in_direction_on_lane(&mut self, pedestrians: &mut MSPModel_Interacting_Pedestrians, lane: Ptr<MSLane>, currentTime: SUMOTime, changedLane: &mut BTreeSet<Ptr<MSPerson>>, dir: i32, debug: bool) {
        // GUI-only debug dump: DEBUGCOND is person isSelected(), always false in a headless run, so not ported
        let _ = debug;
        let stripes = Self::num_stripes(lane);
        //std::cout << " laneWidth=" << lane->getWidth() << " stripeWidth=" << stripeWidth << " stripes=" << stripes << "\n";
        let mut obs: MSPModel_Striping_Obstacles = (0..stripes).map(|_| MSPModel_Striping_Obstacle::new(dir, DIST_FAR_AWAY)).collect(); // continuously updated
        let mut nextLanesObs: MSPModel_Striping_NextLanesObstacles = BTreeMap::new(); // continuously updated
        let sorter = MSPModel_Striping_by_xpos_sorter::new(dir);
        pedestrians.sort_by(|a, b| {
            if sorter.call_op(*a, *b) {
                std::cmp::Ordering::Less
            } else if sorter.call_op(*b, *a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mut crossingVehs: MSPModel_Striping_Obstacles = (0..stripes).map(|_| MSPModel_Striping_Obstacle::new(dir, DIST_FAR_AWAY)).collect();
        let mut hasCrossingVehObs = false;
        if lane.is_crossing() {
            // assume that vehicles will brake when already on the crossing
            hasCrossingVehObs = Self::add_crossing_vehs(lane, stripes, 0., dir, &mut crossingVehs, true, false);
        }

        for ii in 0..pedestrians.len() {
            let p = pedestrians[ii].cast::<MSPModel_Striping_PState>();
            let mut currentObs: MSPModel_Striping_Obstacles = obs.clone();
            if p.get_direction() != dir || changedLane.contains(&p.get_person()) || *p.get_remote_position() != Position::INVALID {
                if !p.is_waiting_to_enter() && !p.is_jammed() {
                    let mut o = MSPModel_Striping_Obstacle::new_2(&*p);
                    if p.get_direction() != dir && p.get_speed() == 0. {
                        // ensure recognition of oncoming
                        o.speed = if p.get_direction() == MSPMODEL_FORWARD { 0.1 } else { -0.1 };
                    }
                    if o.closer(&obs[p.stripe() as usize], dir) {
                        obs[p.stripe() as usize] = o.clone();
                    }
                    if o.closer(&obs[p.other_stripe() as usize], dir) {
                        obs[p.other_stripe() as usize] = o.clone();
                    }
                }
                continue;
            }
            let nextLane = p.myNLI.lane;
            let link = p.myNLI.link;
            let dist = p.dist_to_lane_end();
            let speed = if p.get_stage().get_configured_speed() >= 0. {
                p.get_stage().get_configured_speed()
            } else if !nextLane.is_null() && (use_net_speeds() || nextLane.is_normal() || nextLane.is_internal()) {
                nextLane.get_vehicle_max_speed(p.get_person().cast::<SUMOTrafficObject>())
            } else {
                p.get_stage().get_max_speed(p.get_person().cast::<MSTransportable>())
            };

            if !nextLane.is_null() && dist <= LOOKAHEAD_ONCOMING_DIST {
                let currentLength = if p.myWalkingAreaPath.is_null() { lane.get_length() } else { p.myWalkingAreaPath.length };
                let nextObs = self.get_next_lane_obstacles(&mut nextLanesObs, lane, nextLane, stripes, p.myNLI.dir, currentLength, dir);
                p.m().merge_obstacles(&mut currentObs, nextObs);
            }
            let neighbors = Self::get_neighboring_obstacles(pedestrians, ii as i32, stripes);
            p.m().merge_obstacles(&mut currentObs, &neighbors);
            // time gap to pass the intersection ahead of a vehicle.
            let passingLength = p.get_length() + p.get_person().get_timegap_crossing() * speed;
            // check link state
            if !link.is_null()
                // only check close before junction, @todo we should take deceleration into account here
                && dist - p.get_min_gap() < LOOKAHEAD_SAMEDIR * speed
                // persons move before vehicles so we subtract DELTA_TO because they cannot rely on vehicles having passed the intersection in the current time step
                && (!link.opened(currentTime - utils::common::time::delta_t(), speed, speed, passingLength, p.get_impatience(), speed, 0, 0., Ptr::null(), p.ignore_red(link), p.get_person().cast::<SUMOTrafficObject>(), -1.)
                    || p.stop_for_yellow(link))
            {
                // prevent movement passed a closed link
                let closedLink = MSPModel_Striping_Obstacle::new_3(p.get_edge_pos(0) + dir as f64 * (dist - utils::geom::NUMERICAL_EPS), 0., MSPModel_Striping_ObstacleType::OBSTACLE_LINKCLOSED, &format!("closedLink_{}", link.get_via_lane_or_lane().get_id()), 0., Ptr::null());
                let closedLinkObs: MSPModel_Striping_Obstacles = vec![closedLink; stripes as usize];
                p.m().merge_obstacles(&mut currentObs, &closedLinkObs);
                // consider rerouting over another crossing
                if !p.myWalkingAreaPath.is_null() {
                    // @todo actually another path would be needed starting at the current position
                    let oldNext = p.myNLI.lane;
                    p.m().myNLI = Self::get_next_lane(&*p, p.get_lane(), p.myWalkingAreaPath.from);
                    if p.myNLI.lane != oldNext && oldNext.is_priority_crossing() {
                        MSPModel_Interacting::unregister_crossing_approach(&*p, oldNext);
                    }
                }
            }
            if p.get_stage().get_destination() == Ptr::from_mut(lane.m().get_edge()) && !p.get_stage().get_destination_stop().is_null() {
                let arrival: MSPModel_Striping_Obstacles;
                let stop = p.get_stage().get_destination_stop();
                if stop.get_waiting_capacity() > stop.get_num_waiting_persons() || !stop.check_person_capacity() {
                    let a = MSPModel_Striping_Obstacle::new_3(p.get_stage().get_arrival_pos() + dir as f64 * p.get_min_gap(), 0., MSPModel_Striping_ObstacleType::OBSTACLE_ARRIVALPOS, "arrival", 0., Ptr::null());
                    arrival = vec![a; stripes as usize];
                } else {
                    let a = MSPModel_Striping_Obstacle::new_3(p.get_stage().get_arrival_pos() - dir as f64 * p.get_min_gap(), 0., MSPModel_Striping_ObstacleType::OBSTACLE_ARRIVALPOS, "arrival_blocked", 0., Ptr::null());
                    arrival = vec![a; stripes as usize];
                }
                p.m().merge_obstacles(&mut currentObs, &arrival);
            }

            if lane.get_vehicle_number_with_partials() > 0 {
                // react to vehicles on the same lane
                // @todo: improve efficiency by using the same iterator for all pedestrians on this lane
                let vehObs = Self::get_vehicle_obstacles(lane, dir, p);
                p.m().merge_obstacles(&mut currentObs, &vehObs);
            }
            if hasCrossingVehObs {
                p.m().merge_obstacles(&mut currentObs, &crossingVehs);
            }

            // walk, taking into account all obstacles
            p.m().walk(&currentObs);
            if !p.is_waiting_to_enter() && !p.is_jammed() {
                let mut o = MSPModel_Striping_Obstacle::new_2(&*p);
                if o.closer(&obs[p.stripe() as usize], dir) {
                    obs[p.stripe() as usize] = o.clone();
                }
                if o.closer(&obs[p.other_stripe() as usize], dir) {
                    obs[p.other_stripe() as usize] = o.clone();
                }
                if MSGlobals::check4_accidents() && p.myWalkingAreaPath.is_null() && !p.is_jammed() {
                    for coll in 0..ii {
                        let c = pedestrians[coll].cast::<MSPModel_Striping_PState>();
                        if !c.is_waiting_to_enter() && c.myWalkingAreaPath.is_null() && !c.is_jammed() {
                            if c.stripe() == p.stripe() || p.stripe() == c.other_stripe() || p.other_stripe() == c.stripe() || p.other_stripe() == c.other_stripe() {
                                let cObs = MSPModel_Striping_Obstacle::new_2(&*c);
                                // we check only for real collisions, no min gap violations
                                if p.distance_to(&cObs, false) == DIST_OVERLAP {
                                    utils::common::msg_handler::write_warning(&format!("Collision of person '{}' and person '{}', lane='{}', time={}.",
                                        p.get_person().get_id(), c.get_person().get_id(), lane.get_id(), utils::common::time::time2string(currentTime)));
                                }
                            }
                        }
                    }
                }
            }
            //std::cout << SIMTIME << p.getPerson()->getID() << " lane=" << lane->getID() << " x=" << p.myRelX << "\n";
        }
    }

    /// C++: `void MSPModel_Striping::arriveAndAdvance(Pedestrians &, SUMOTime, std::set<MSPerson *> &, int)` [protected]
    /// handle arrivals and lane advancement
    // void
    // MSPModel_Striping::arriveAndAdvance(Pedestrians& pedestrians, SUMOTime currentTime, std::set<MSPerson*>& changedLane, int dir) {
    //     // advance to the next lane / arrive at destination
    //     sort(pedestrians.begin(), pedestrians.end(), by_xpos_sorter(dir));
    //     // can't use iterators because we do concurrent modification
    //     for (int i = 0; i < (int)pedestrians.size(); i++) {
    //         PState* const p = static_cast<PState*>(pedestrians[i]);
    //         if (p->isRemoteControlled()) {
    //             continue;
    //         }
    //         if (p->getDirection() == dir && p->distToLaneEnd() < 0) {
    //             // moveToNextLane may trigger re-insertion (for consecutive
    //             // walks) so erase must be called first
    //             pedestrians.erase(pedestrians.begin() + i);
    //             i--;
    //             p->moveToNextLane(currentTime);
    //             if (p->getLane() != nullptr) {
    //                 changedLane.insert(p->getPerson());
    //                 myActiveLanes[p->getLane()].push_back(p);
    //                 p->getStage()->activateEntryReminders(p->getPerson());
    //             } else {
    //                 // end walking stage and destroy PState
    //                 p->getStage()->moveToNextEdge(p->getPerson(), currentTime, dir);
    //                 myNumActivePedestrians--;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn arrive_and_advance(&mut self, pedestrians: &mut MSPModel_Interacting_Pedestrians, currentTime: SUMOTime, changedLane: &mut BTreeSet<Ptr<MSPerson>>, dir: i32) {
        // advance to the next lane / arrive at destination
        // determinism: C++ std::sort is unstable; ties are resolved by the id comparison in by_xpos_sorter
        let sorter = MSPModel_Striping_by_xpos_sorter::new(dir);
        pedestrians.sort_by(|a, b| {
            if sorter.call_op(*a, *b) {
                std::cmp::Ordering::Less
            } else if sorter.call_op(*b, *a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        // can't use iterators because we do concurrent modification
        let mut i: usize = 0;
        while i < pedestrians.len() {
            let p = pedestrians[i].cast::<MSPModel_Striping_PState>();
            if p.is_remote_controlled() {
                i += 1;
                continue;
            }
            if p.get_direction() == dir && p.dist_to_lane_end() < 0. {
                // moveToNextLane may trigger re-insertion (for consecutive
                // walks) so erase must be called first
                pedestrians.remove(i);
                // C++ does i-- and the loop's ++i, so the next pedestrian is now at index i
                p.m().move_to_next_lane(currentTime);
                if !p.get_lane().is_null() {
                    changedLane.insert(p.get_person());
                    self.myActiveLanes.entry(MSPModel_Interacting_LaneKey(p.get_lane())).or_default().push(p.cast::<MSPModel_InteractingState>());
                    p.get_stage().m().activate_entry_reminders(p.get_person().cast::<MSTransportable>(), false);
                } else {
                    // end walking stage and destroy PState
                    p.get_stage().m().move_to_next_edge(p.get_person().cast::<MSTransportable>(), currentTime, dir, Ptr::null(), false);
                    self.myNumActivePedestrians -= 1;
                }
                continue;
            }
            i += 1;
        }
    }

    /// C++: `const ActiveLanes & MSPModel_Striping::getActiveLanes()` [protected]
    // const ActiveLanes& getActiveLanes() {
    //         return myActiveLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_active_lanes(&mut self) -> &MSPModel_Interacting_ActiveLanes {
        &self.myActiveLanes
    }

    /// C++: `void MSPModel_Striping::DEBUG_PRINT(const Obstacles &)` [static, private]
    // void MSPModel_Striping::DEBUG_PRINT(const Obstacles& obs) {
    //     for (int i = 0; i < (int)obs.size(); ++i) {
    //         std::cout
    //                 << "(" << obs[i].description
    //                 << " x=(" << obs[i].xBack << "," << obs[i].xFwd
    //                 << ") s=" << obs[i].speed
    //                 << ")   ";
    //     }
    //     std::cout << "\n";
    // }
    #[allow(unused_variables)]
    pub fn debug_print(obs: &MSPModel_Striping_Obstacles) {
        // shortcut: C++ ostream default float formatting is not reproduced; Rust Display is used (debug output only)
        for o in obs.iter() {
            print!("({} x=({},{}) s={})   ", o.description, o.xBack, o.xFwd, o.speed);
        }
        println!();
    }

    /// C++: `int MSPModel_Striping::connectedDirection(const MSLane *, const MSLane *)` [static, private]
    /// returns the direction in which these lanes are connectioned or 0 if they are not
    // int
    // MSPModel_Striping::connectedDirection(const MSLane* from, const MSLane* to) {
    //     if (from == nullptr || to == nullptr) {
    //         return UNDEFINED_DIRECTION;
    //     } else if (from->getLinkTo(to) != nullptr) {
    //         return FORWARD;
    //     } else if (to->getLinkTo(from) != nullptr) {
    //         return BACKWARD;
    //     } else {
    //         return UNDEFINED_DIRECTION;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn connected_direction(from: Ptr<MSLane>, to: Ptr<MSLane>) -> i32 {
        if from.is_null() || to.is_null() {
            return MSPMODEL_UNDEFINED_DIRECTION;
        } else if !from.get_link_to(to).is_null() {
            return MSPMODEL_FORWARD;
        } else if !to.get_link_to(from).is_null() {
            return MSPMODEL_BACKWARD;
        } else {
            return MSPMODEL_UNDEFINED_DIRECTION;
        }
    }

    /// C++: `NextLaneInfo MSPModel_Striping::getNextLane(const PState &, const MSLane *, const MSLane *)` [static, private]
    /// computes the successor lane for the given pedestrian and sets the link as well as the direction to use on the succesor lane
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_next_lane(ped: &MSPModel_Striping_PState, currentLane: Ptr<MSLane>, prevLane: Ptr<MSLane>) -> MSPModel_Striping_NextLaneInfo {
        // MSPModel_Striping::getNextLane(const PState& ped, const MSLane* currentLane, const MSLane* prevLane)
        let currentEdge = Ptr::from_mut(currentLane.m().get_edge());
        let junction = if ped.get_direction() == MSPMODEL_FORWARD { currentEdge.get_to_junction() } else { currentEdge.get_from_junction() };
        let nextRouteEdge = ped.get_stage().get_next_route_edge();
        let mut nextRouteLane = get_sidewalk(nextRouteEdge, ped.get_person().get_v_class());
        // result values
        let mut nextLane = nextRouteLane;
        let mut link: Ptr<MSLink> = Ptr::null();
        let mut nextDir = MSPMODEL_UNDEFINED_DIRECTION;

        if nextRouteLane.is_null() && !nextRouteEdge.is_null() {
            let error = format!("Person '{}' could not find sidewalk on edge '{}', time={}.", ped.get_person().get_id(), nextRouteEdge.get_id(),
                utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
            if utils::options::options_cont::get_bool("ignore-route-errors") {
                utils::common::msg_handler::write_warning(&error);
                nextRouteLane = nextRouteEdge.get_lanes()[0];
            } else {
                panic!("{}", error);
            }
        }

        if !nextRouteLane.is_null() {
            if currentEdge.is_internal() {
                debug_assert!(junction == currentEdge.get_from_junction());
                nextDir = if junction == nextRouteEdge.get_from_junction() { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                if nextDir == MSPMODEL_FORWARD {
                    nextLane = currentLane.get_link_cont()[0].get_via_lane_or_lane();
                } else {
                    nextLane = currentLane.get_logical_predecessor_lane();
                }
            } else if currentEdge.is_crossing() {
                nextDir = ped.get_direction();
                if nextDir == MSPMODEL_FORWARD {
                    nextLane = currentLane.get_link_cont()[0].get_lane();
                } else {
                    nextLane = currentLane.get_logical_predecessor_lane();
                }
                if currentLane.is_priority_crossing() {
                    MSPModel_Interacting::unregister_crossing_approach(ped, currentLane);
                }
            } else if currentEdge.is_walking_area() {
                let mut crossingRoute: Vec<Ptr<MSEdge>> = Vec::new();
                // departPos can be 0 because the direction of the walkingArea does not matter
                // for the arrivalPos, we need to make sure that the route does not deviate across other junctions
                let nextRouteEdgeDir = if nextRouteEdge.get_from_junction() == junction { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                let arrivalPos = if Some(nextRouteEdge) == ped.get_stage().get_route().last().copied() {
                    ped.get_stage().get_arrival_pos()
                } else if nextRouteEdgeDir == MSPMODEL_FORWARD {
                    0.
                } else {
                    nextRouteEdge.get_length()
                };
                let mut prohibited: MSNet_Prohibitions = BTreeMap::new();
                if !prevLane.is_null() {
                    // C++ operator[] default-constructs RouterProhibition (begin 0, end max, SVC_IGNORING)
                    prohibited.entry(Ptr::from_mut(prevLane.m().get_edge())).or_insert(RouterProhibition { begin: 0., end: f64::MAX, permissions: SVC_IGNORING }).end = f64::MAX;
                }
                MSNet::get_instance().get_pedestrian_router(0, &prohibited).compute(currentEdge, nextRouteEdge, 0., arrivalPos,
                    ped.get_stage().get_max_speed(ped.get_person().cast::<MSTransportable>()),
                    0, junction, ped.get_person().get_v_type_parameter(), &mut crossingRoute, true);
                if crossingRoute.len() > 1 {
                    let nextEdge = crossingRoute[1];
                    nextLane = get_sidewalk(crossingRoute[1], ped.get_person().get_v_class());
                    debug_assert!(nextEdge.get_from_junction() == junction || nextEdge.get_to_junction() == junction);
                    debug_assert!(nextLane != prevLane);
                    nextDir = Self::connected_direction(currentLane, nextLane);
                    debug_assert!(nextDir != MSPMODEL_UNDEFINED_DIRECTION);
                    if nextDir == MSPMODEL_FORWARD {
                        link = currentLane.get_link_to(nextLane);
                    } else {
                        link = nextLane.get_link_to(currentLane);
                        if nextEdge.is_crossing() && link.get_tl_logic().is_null() {
                            let oppositeWalkingArea = nextLane.get_logical_predecessor_lane();
                            link = oppositeWalkingArea.get_link_to(nextLane);
                        }
                    }
                    debug_assert!(!link.is_null());
                    if nextLane.is_priority_crossing() {
                        Self::register_crossing_approach(ped, nextLane, prevLane);
                    }
                } else {
                    // check if a direct connection exists (moving onto the walkingarea was the wrong choice)
                    if ped.get_direction() == MSPMODEL_FORWARD {
                        link = prevLane.get_link_to(nextRouteLane);
                    } else {
                        link = nextRouteLane.get_link_to(prevLane);
                    }
                    if !link.is_null() {
                        // leave direction as UNDEFINED_DIRECTION to signal that currentLane must be changed
                        nextLane = link.get_via_lane_or_lane();
                    } else {
                        utils::common::msg_handler::write_warning(&format!("Person '{}' could not find route across junction '{}' from walkingArea '{}' to edge '{}', time={}.",
                            ped.get_person().get_id(), junction.get_id(), currentEdge.get_id(), nextRouteEdge.get_id(),
                            utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                        // error indicated by nextDir == UNDEFINED_DIRECTION
                        nextLane = nextRouteLane;
                    }
                }
            } else if currentEdge == nextRouteEdge {
                // strange loop in this route. No need to use walkingArea
                nextDir = -ped.get_direction();
            } else {
                // normal edge. by default use next / previous walking area
                nextDir = ped.get_direction();
                nextLane = Self::get_next_walking_area(currentLane, ped.get_direction(), &mut link);
                if nextLane.is_null() {
                    // walk forward by default
                    if junction == nextRouteEdge.get_to_junction() {
                        nextDir = MSPMODEL_BACKWARD;
                    } else if junction == nextRouteEdge.get_from_junction() {
                        nextDir = MSPMODEL_FORWARD;
                    } else {
                        // topological disconnect, find a direction that makes sense
                        // for the future part of the route
                        let mut futureRoute: Vec<Ptr<MSEdge>> = ped.get_stage().get_route().clone();
                        futureRoute.drain(0..(ped.get_stage().get_route_position() + 1) as usize);
                        let mut passedFwd = 0;
                        let mut passedBwd = 0;
                        MSPModel::can_traverse(MSPMODEL_FORWARD, &futureRoute, &mut passedFwd);
                        MSPModel::can_traverse(MSPMODEL_BACKWARD, &futureRoute, &mut passedBwd);
                        nextDir = if passedFwd >= passedBwd { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                    }
                    // try to use a direct link as fallback
                    // direct links only exist if built explicitly. They are used to model tl-controlled links if there are no crossings
                    if ped.get_direction() == MSPMODEL_FORWARD {
                        link = currentLane.get_link_to(nextRouteLane);
                        if !link.is_null() {
                            nextLane = currentLane.get_internal_following_lane(nextRouteLane);
                        }
                    } else {
                        link = nextRouteLane.get_link_to(currentLane);
                        if !link.is_null() {
                            nextLane = nextRouteLane.get_internal_following_lane(currentLane);
                            if !nextLane.is_null() {
                                // advance to the end of consecutive internal lanes
                                while nextLane.get_link_cont()[0].get_via_lane_or_lane().is_internal() {
                                    nextLane = nextLane.get_link_cont()[0].get_via_lane_or_lane();
                                }
                            }
                        }
                    }
                }
                if nextLane.is_null() {
                    // no internal lane found
                    nextLane = nextRouteLane;
                    if MSPModel_Interacting::using_internal_lanes_static() && currentLane.get_link_cont().len() > 0 {
                        utils::common::msg_handler::write_warning(&format!("Person '{}' could not find route across junction '{}' from edge '{}' to edge '{}', time={}.",
                            ped.get_person().get_id(), junction.get_id(), currentEdge.get_id(), nextRouteEdge.get_id(),
                            utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                    }
                } else if nextLane.get_length() <= utils::geom::POSITION_EPS {
                    // internal lane too short
                    // most often this is due to a zero-size junction. However, if
                    // the person needs to pass a crossing we cannot skip ahead
                    if (nextLane.get_canonical_successor_lane().is_null() || !nextLane.get_canonical_successor_lane().is_crossing())
                        && (nextLane.get_logical_predecessor_lane().is_null() || !nextLane.get_logical_predecessor_lane().is_crossing())
                    {
                        nextLane = nextRouteLane;
                        nextDir = if nextRouteEdge.get_from_junction() == junction { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                    }
                }
            }
        }
        debug_assert!(!nextLane.is_null() || nextRouteLane.is_null());
        MSPModel_Striping_NextLaneInfo::new(nextLane, link, nextDir)
    }

    /// C++: `const MSLane * MSPModel_Striping::getNextWalkingArea(const MSLane *, const int, const MSLink *&)` [static, private]
    /// return the next walkingArea in the given direction
    // const MSLane*
    // MSPModel_Striping::getNextWalkingArea(const MSLane* currentLane, const int dir, const MSLink*& link) {
    //     if (dir == FORWARD) {
    //         for (const MSLink* const l : currentLane->getLinkCont()) {
    //             if (l->getLane()->isWalkingArea()) {
    //                 link = l;
    //                 return l->getLane();
    //             }
    //         }
    //     } else {
    //         const std::vector<MSLane::IncomingLaneInfo>& laneInfos = currentLane->getIncomingLanes();
    //         for (std::vector<MSLane::IncomingLaneInfo>::const_iterator it = laneInfos.begin(); it != laneInfos.end(); ++it) {
    //             if ((*it).lane->isWalkingArea()) {
    //                 link = (*it).viaLink;
    //                 return (*it).lane;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_walking_area(currentLane: Ptr<MSLane>, dir: i32, link: &mut Ptr<MSLink>) -> Ptr<MSLane> {
        if dir == MSPMODEL_FORWARD {
            for l in currentLane.get_link_cont().iter() {
                if l.get_lane().is_walking_area() {
                    *link = *l;
                    return l.get_lane();
                }
            }
        } else {
            let laneInfos = currentLane.get_incoming_lanes();
            for it in laneInfos.iter() {
                if it.lane.is_walking_area() {
                    *link = it.viaLink;
                    return it.lane;
                }
            }
        }
        Ptr::null()
    }

    /// C++: `void MSPModel_Striping::initWalkingAreaPaths(const MSNet *)` [static, private]
    // void
    // MSPModel_Striping::initWalkingAreaPaths(const MSNet*) {
    //     if (myWalkingAreaPaths.size() > 0) {
    //         return;
    //     }
    //     // collect vehicle lanes that cross walkingareas
    //     for (MSEdgeVector::const_iterator i = MSEdge::getAllEdges().begin(); i != MSEdge::getAllEdges().end(); ++i) {
    //         const MSEdge* edge = *i;
    //         if (!edge->isWalkingArea() && !edge->isCrossing()) {
    //             for (MSLane* lane : edge->getLanes()) {
    //                 for (MSLink* link : lane->getLinkCont()) {
    //                     if (link->getWalkingAreaFoe() != nullptr) {
    //                         // link is an exit link
    //                         myWalkingAreaFoes[&link->getWalkingAreaFoe()->getEdge()].push_back(link->getLaneBefore());
    //                         //std::cout << " wa=" << link->getWalkingAreaFoe()->getEdge().getID() << " foe=" << link->getLaneBefore()->getID() << "\n";
    //                     }
    //                     if (link->getWalkingAreaFoeExit() != nullptr) {
    //                         // link is an exit link
    //                         myWalkingAreaFoes[&link->getWalkingAreaFoeExit()->getEdge()].push_back(link->getLaneBefore());
    //                         //std::cout << " wa=" << link->getWalkingAreaFoeExit()->getEdge().getID() << " foe=" << link->getLaneBefore()->getID() << "\n";
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     // build walkingareaPaths
    //     for (MSEdgeVector::const_iterator i = MSEdge::getAllEdges().begin(); i != MSEdge::getAllEdges().end(); ++i) {
    //         insertWalkArePaths(*i, myWalkingAreaPaths);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_walking_area_paths(net: Ptr<MSNet>) {
        // (the C++ net parameter is unused)
        let _ = net;
        if MY_WALKING_AREA_PATHS.with(|m| m.borrow().len() > 0) {
            return;
        }
        // collect vehicle lanes that cross walkingareas
        for edge in MSEdge::get_all_edges().iter().copied() {
            if !edge.is_walking_area() && !edge.is_crossing() {
                for lane in edge.get_lanes().iter().copied() {
                    for link in lane.get_link_cont().iter().copied() {
                        let foe = link.m().get_walking_area_foe();
                        if !foe.is_null() {
                            // link is an exit link
                            let link_before = link.get_lane_before();
                            MY_WALKING_AREA_FOES.with(|m| m.borrow_mut().entry(Ptr::from_mut(foe.m().get_edge())).or_default().push(link_before));
                        }
                        let foeExit = link.m().get_walking_area_foe_exit();
                        if !foeExit.is_null() {
                            // link is an exit link
                            let link_before = link.get_lane_before();
                            MY_WALKING_AREA_FOES.with(|m| m.borrow_mut().entry(Ptr::from_mut(foeExit.m().get_edge())).or_default().push(link_before));
                        }
                    }
                }
            }
        }

        // build walkingareaPaths
        // (built in a local map and moved into the thread-local afterwards, so no RefCell borrow is held while inserting)
        let mut into: MSPModel_Striping_WalkingAreaPaths = BTreeMap::new();
        for edge in MSEdge::get_all_edges().iter().copied() {
            Self::insert_walk_are_paths(edge, &mut into);
        }
        MY_WALKING_AREA_PATHS.with(|m| *m.borrow_mut() = into);
    }

    /// C++: `void MSPModel_Striping::insertWalkArePaths(const MSEdge *, WalkingAreaPaths &)` [static, private]
    /// creates and inserts all paths into the given map
    // void
    // MSPModel_Striping::insertWalkArePaths(const MSEdge* edge, WalkingAreaPaths& into) {
    //     if (edge->isWalkingArea()) {
    //         const MSLane* walkingArea = getSidewalk<MSEdge, MSLane>(edge);
    //         myMinNextLengths[walkingArea] = walkingArea->getLength();
    //         // build all possible paths across this walkingArea
    //         // gather all incident lanes
    //         std::vector<const MSLane*> lanes;
    //         for (const MSEdge* in : edge->getPredecessors()) {
    //             if (!in->isTazConnector()) {
    //                 lanes.push_back(getSidewalk<MSEdge, MSLane>(in));
    //                 if (lanes.back() == nullptr) {
    //                     throw ProcessError("Invalid connection from edge '" + in->getID() + "' to walkingarea edge '" + edge->getID() + "'");
    //                 }
    //             }
    //         }
    //         for (const MSEdge* out : edge->getSuccessors()) {
    //             if (!out->isTazConnector()) {
    //                 lanes.push_back(getSidewalk<MSEdge, MSLane>(out));
    //                 if (lanes.back() == nullptr) {
    //                     throw ProcessError("Invalid connection from walkingarea edge '" + edge->getID() + "' to edge '" + out->getID() + "'");
    //                 }
    //             }
    //         }
    //         // build all combinations
    //         for (int j = 0; j < (int)lanes.size(); ++j) {
    //             for (int k = 0; k < (int)lanes.size(); ++k) {
    //                 if (j != k) {
    //                     // build the walkingArea
    //                     const MSLane* const from = lanes[j];
    //                     const MSLane* const to = lanes[k];
    //                     const int fromDir = from->getLinkTo(walkingArea) != nullptr ? FORWARD : BACKWARD;
    //                     const int toDir = walkingArea->getLinkTo(to) != nullptr ? FORWARD : BACKWARD;
    //                     PositionVector shape;
    //                     Position fromPos = from->getShape()[fromDir == FORWARD ? -1 : 0];
    //                     Position toPos = to->getShape()[toDir == FORWARD ? 0 : -1];
    //                     const double maxExtent = fromPos.distanceTo2D(toPos) / 4; // prevent sharp corners
    //                     const double extrapolateBy = MIN2(maxExtent, walkingArea->getWidth() / 2);
    //                     // assemble shape
    //                     shape.push_back(fromPos);
    //                     if (extrapolateBy > POSITION_EPS) {
    //                         PositionVector fromShp = from->getShape();
    //                         fromShp.extrapolate(extrapolateBy);
    //                         shape.push_back_noDoublePos(fromDir == FORWARD ? fromShp.back() : fromShp.front());
    //                         PositionVector nextShp = to->getShape();
    //                         nextShp.extrapolate(extrapolateBy);
    //                         shape.push_back_noDoublePos(toDir == FORWARD ? nextShp.front() : nextShp.back());
    //                     }
    //                     shape.push_back_noDoublePos(toPos);
    //                     if (shape.size() < 2) {
    //                         PositionVector fromShp = from->getShape();
    //                         fromShp.extrapolate(1.5 * POSITION_EPS); // noDoublePos requires a difference of POSITION_EPS in at least one coordinate
    //                         shape.push_back_noDoublePos(fromDir == FORWARD ? fromShp.back() : fromShp.front());
    //                         assert(shape.size() == 2);
    //                     } else if (myWalkingAreaDetail > 4) {
    //                         shape = shape.bezier(myWalkingAreaDetail);
    //                     }
    //                     double angleOverride = INVALID_DOUBLE;
    //                     if (shape.size() >= 4 && shape.length() < walkingArea->getWidth()) {
    //                         const double aStart = shape.angleAt2D(0);
    //                         const double aEnd = shape.angleAt2D((int)shape.size() - 2);
    //                         if (fabs(aStart - aEnd) < DEG2RAD(10)) {
    //                             angleOverride = (aStart + aEnd) / 2;
    //                         }
    //                     }
    //                     if (fromDir == BACKWARD) {
    //                         // will be walking backward on walkingArea
    //                         shape = shape.reverse();
    //                     }
    //                     WalkingAreaPath wap = WalkingAreaPath(from, walkingArea, to, shape, fromDir, angleOverride);
    //                     into.insert(std::make_pair(std::make_pair(from, to), wap));
    //                     myMinNextLengths[walkingArea] = MIN2(myMinNextLengths[walkingArea], wap.length);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_walk_are_paths(edge: Ptr<MSEdge>, into: &mut MSPModel_Striping_WalkingAreaPaths) {
        // MSPModel_Striping::insertWalkArePaths(const MSEdge* edge, WalkingAreaPaths& into)
        if edge.is_walking_area() {
            let walkingArea = get_sidewalk(edge, SVC_PEDESTRIAN);
            MY_MIN_NEXT_LENGTHS.with(|m| m.borrow_mut().insert(walkingArea, walkingArea.get_length()));
            // build all possible paths across this walkingArea
            // gather all incident lanes
            let mut lanes: Vec<Ptr<MSLane>> = Vec::new();
            for in_ in edge.get_predecessors().iter().copied() {
                if !in_.is_taz_connector() {
                    lanes.push(get_sidewalk(in_, SVC_PEDESTRIAN));
                    if lanes.last().unwrap().is_null() {
                        panic!("{}", format!("Invalid connection from edge '{}' to walkingarea edge '{}'", in_.get_id(), edge.get_id()));
                    }
                }
            }
            for out in edge.get_successors(SVC_IGNORING).iter().copied() {
                if !out.is_taz_connector() {
                    lanes.push(get_sidewalk(out, SVC_PEDESTRIAN));
                    if lanes.last().unwrap().is_null() {
                        panic!("{}", format!("Invalid connection from walkingarea edge '{}' to edge '{}'", edge.get_id(), out.get_id()));
                    }
                }
            }
            // build all combinations
            for j in 0..lanes.len() {
                for k in 0..lanes.len() {
                    if j != k {
                        // build the walkingArea
                        let from = lanes[j];
                        let to = lanes[k];
                        let fromDir = if !from.get_link_to(walkingArea).is_null() { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                        let toDir = if !walkingArea.get_link_to(to).is_null() { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                        let mut shape = PositionVector::new();
                        let fromPos = *from.get_shape_2().at(if fromDir == MSPMODEL_FORWARD { -1 } else { 0 });
                        let toPos = *to.get_shape_2().at(if toDir == MSPMODEL_FORWARD { 0 } else { -1 });
                        let maxExtent = fromPos.distance_to2d(&toPos) / 4.; // prevent sharp corners
                        let extrapolateBy = f64::min(maxExtent, walkingArea.get_width() / 2.);
                        // assemble shape
                        shape.push(fromPos);
                        if extrapolateBy > utils::geom::POSITION_EPS {
                            let mut fromShp = from.get_shape_2().clone();
                            fromShp.extrapolate(extrapolateBy, false, false);
                            shape.push_back_no_double_pos(if fromDir == MSPMODEL_FORWARD { fromShp.back() } else { fromShp.front() });
                            let mut nextShp = to.get_shape_2().clone();
                            nextShp.extrapolate(extrapolateBy, false, false);
                            shape.push_back_no_double_pos(if toDir == MSPMODEL_FORWARD { nextShp.front() } else { nextShp.back() });
                        }
                        shape.push_back_no_double_pos(&toPos);
                        if shape.len() < 2 {
                            let mut fromShp = from.get_shape_2().clone();
                            fromShp.extrapolate(1.5 * utils::geom::POSITION_EPS, false, false); // noDoublePos requires a difference of POSITION_EPS in at least one coordinate
                            shape.push_back_no_double_pos(if fromDir == MSPMODEL_FORWARD { fromShp.back() } else { fromShp.front() });
                            debug_assert!(shape.len() == 2);
                        } else if my_walking_area_detail() > 4 {
                            shape = shape.bezier(my_walking_area_detail());
                        }
                        let mut angleOverride = utils::common::std_defs::INVALID_DOUBLE;
                        if shape.len() >= 4 && shape.length() < walkingArea.get_width() {
                            let aStart = shape.angle_at2d(0);
                            let aEnd = shape.angle_at2d(shape.len() as i32 - 2);
                            if f64::abs(aStart - aEnd) < utils::geom::geom_helper::deg2rad(10.) {
                                angleOverride = (aStart + aEnd) / 2.;
                            }
                        }
                        if fromDir == MSPMODEL_BACKWARD {
                            // will be walking backward on walkingArea
                            shape = shape.reverse();
                        }
                        let wap = MSPModel_Striping_WalkingAreaPath::new(from, walkingArea, to, &shape, fromDir, angleOverride);
                        let wapLength = wap.length;
                        into.entry((from, to)).or_insert(wap);
                        MY_MIN_NEXT_LENGTHS.with(|m| {
                            let mut m = m.borrow_mut();
                            let cur = *m.entry(walkingArea).or_insert(0.);
                            let v = if cur < wapLength { cur } else { wapLength };
                            m.insert(walkingArea, v);
                        });
                    }
                }
            }
        }
    }

    /// C++: `const WalkingAreaPath * MSPModel_Striping::getWalkingAreaPath(const MSEdge *, const MSLane *, const MSLane *)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_walking_area_path(walkingArea: Ptr<MSEdge>, before: Ptr<MSLane>, after: Ptr<MSLane>) -> Ptr<MSPModel_Striping_WalkingAreaPath> {
        debug_assert!(walkingArea.is_walking_area());
        if let Some(p) = lookup_walking_area_path(before, after) {
            return p;
        }
        // this can happen in case of moveToXY where before can point anywhere
        // or when a person starts directly on a walking area (before == nullptr)
        for pred in walkingArea.get_predecessors().iter() {
            if let Some(p) = lookup_walking_area_path(get_sidewalk(*pred, SVC_PEDESTRIAN), after) {
                return p;
            }
        }
        Self::get_arbitrary_path(walkingArea)
    }

    /// C++: `const WalkingAreaPath * MSPModel_Striping::getArbitraryPath(const MSEdge *)` [static, private]
    /// return an arbitrary path across the given walkingArea
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_arbitrary_path(walkingArea: Ptr<MSEdge>) -> Ptr<MSPModel_Striping_WalkingAreaPath> {
        debug_assert!(walkingArea.is_walking_area());
        let mut lanes: Vec<Ptr<MSLane>> = Vec::new();
        for pred in walkingArea.get_predecessors().iter() {
            lanes.push(get_sidewalk(*pred, SVC_PEDESTRIAN));
        }
        for succ in walkingArea.get_successors(SVC_IGNORING).iter() {
            lanes.push(get_sidewalk(*succ, SVC_PEDESTRIAN));
        }
        if lanes.len() < 1 {
            panic!("{}", utils::tlf!("Invalid walkingarea '%' does not allow continuation.", walkingArea.get_id()));
        }
        // shortcut: C++ dereferences end() when the key is missing (undefined behaviour); this panics instead
        lookup_walking_area_path(lanes[0], lanes[lanes.len() - 1]).unwrap_or_else(|| panic!("walking area path not found"))
    }

    /// C++: `const WalkingAreaPath * MSPModel_Striping::guessPath(const MSEdge *, const MSEdge *, const MSEdge *)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn guess_path(walkingArea: Ptr<MSEdge>, before: Ptr<MSEdge>, after: Ptr<MSEdge>) -> Ptr<MSPModel_Striping_WalkingAreaPath> {
        debug_assert!(walkingArea.is_walking_area());
        let swBefore = get_sidewalk(before, SVC_PEDESTRIAN);
        let swAfter = get_sidewalk(after, SVC_PEDESTRIAN);
        if let Some(p) = lookup_walking_area_path(swBefore, swAfter) {
            return p;
        }
        let preds = walkingArea.get_predecessors();
        let succs = walkingArea.get_successors(SVC_IGNORING);
        let useBefore = !swBefore.is_null() && preds.contains(&before);
        let useAfter = !swAfter.is_null() && succs.contains(&after);
        if useBefore {
            if useAfter {
                return Self::get_walking_area_path(walkingArea, swBefore, swAfter);
            } else if succs.len() > 0 {
                // could also try to exploit direction
                return Self::get_walking_area_path(walkingArea, swBefore, get_sidewalk(succs[0], SVC_PEDESTRIAN));
            }
        } else if useAfter && preds.len() > 0 {
            // could also try to exploit direction
            return Self::get_walking_area_path(walkingArea, get_sidewalk(preds[0], SVC_PEDESTRIAN), swAfter);
        }
        Self::get_arbitrary_path(walkingArea)
    }

    /// C++: `int MSPModel_Striping::numStripes(const MSLane *)` [static, private]
    /// return the maximum number of pedestrians walking side by side
    // int
    // MSPModel_Striping::numStripes(const MSLane* lane) {
    //     return MAX2(1, (int)floor(lane->getWidth() / stripeWidth));
    // }
    #[allow(unused_variables)]
    pub fn num_stripes(lane: Ptr<MSLane>) -> i32 {
        std::cmp::max(1, (lane.get_width() / stripe_width()).floor() as i32)
    }

    /// C++: `Obstacles MSPModel_Striping::getNeighboringObstacles(const Pedestrians &, int, int)` [static, private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_neighboring_obstacles(pedestrians: &MSPModel_Interacting_Pedestrians, egoIndex: i32, stripes: i32) -> MSPModel_Striping_Obstacles {
        let ego = pedestrians[egoIndex as usize].cast::<MSPModel_Striping_PState>();
        let egoStripe = ego.stripe();
        let mut obs: MSPModel_Striping_Obstacles = (0..stripes).map(|_| MSPModel_Striping_Obstacle::new(ego.get_direction(), DIST_FAR_AWAY)).collect();
        let mut haveBlocker = vec![false; stripes as usize];
        for index in ((egoIndex + 1) as usize)..pedestrians.len() {
            let p = pedestrians[index].cast::<MSPModel_Striping_PState>();
            if !p.is_waiting_to_enter() && !p.is_jammed() {
                let o = MSPModel_Striping_Obstacle::new_2(&*p);
                if ego.distance_to(&o, true) == DIST_BEHIND {
                    break;
                }
                if ego.distance_to(&o, true) == DIST_OVERLAP {
                    if p.stripe() != egoStripe || p.get_direction() != ego.get_direction() {
                        obs[p.stripe() as usize] = o.clone();
                        haveBlocker[p.stripe() as usize] = true;
                    } else {
                        //std::cout << SIMTIME << "   ignoring overlap between " << ego.getID() << " and " << p.getID() << " on stripe=" << egoStripe << "\n";
                    }
                    if p.other_stripe() != egoStripe || p.get_direction() != ego.get_direction() {
                        obs[p.other_stripe() as usize] = o.clone();
                        haveBlocker[p.other_stripe() as usize] = true;
                    } else {
                        //std::cout << SIMTIME << "   ignoring overlap between " << ego.getID() << " and " << p.getID() << " on stripe2=" << egoStripe << "\n";
                    }
                } else {
                    if !haveBlocker[p.stripe() as usize] {
                        obs[p.stripe() as usize] = o.clone();
                    }
                    if !haveBlocker[p.other_stripe() as usize] {
                        obs[p.other_stripe() as usize] = o.clone();
                    }
                }
            }
        }
        // GUI-only debug dump: DEBUGCOND is person isSelected(), always false in a headless run, so not ported
        obs
    }

    /// C++: `const Obstacles & MSPModel_Striping::getNextLaneObstacles(NextLanesObstacles &, const MSLane *, const MSLane *, int, int, double, int)` [private]
    /// returns the cached obstacles for nextLane, computing them on first use (the reference lives in nextLanesObs)
    #[allow(unused_variables)]
    pub fn get_next_lane_obstacles<'a>(&mut self, nextLanesObs: &'a mut MSPModel_Striping_NextLanesObstacles, lane: Ptr<MSLane>, nextLane: Ptr<MSLane>, stripes: i32, mut nextDir: i32, currentLength: f64, currentDir: i32) -> &'a MSPModel_Striping_Obstacles {
        if !nextLanesObs.contains_key(&nextLane) {
            let nextLength = if nextLane.is_walking_area() {
                MY_MIN_NEXT_LENGTHS.with(|m| *m.borrow_mut().entry(nextLane).or_insert(0.))
            } else {
                nextLane.get_length()
            };
            // figure out the which pedestrians are ahead on the next lane
            let nextStripes = Self::num_stripes(nextLane);
            // do not move past the end of the next lane in a single step
            let mut obs: MSPModel_Striping_Obstacles = (0..stripes)
                .map(|_| MSPModel_Striping_Obstacle::new_3(if nextDir == MSPMODEL_FORWARD { nextLength } else { 0. }, 0., MSPModel_Striping_ObstacleType::OBSTACLE_NEXTEND, "nextEnd", 0., Ptr::null()))
                .collect();

            let offset = Self::get_stripe_offset(nextStripes, stripes, currentDir != nextDir && nextStripes > stripes);
            if nextStripes < stripes {
                // some stripes do not continue
                for ii in 0..stripes {
                    if ii < offset || ii >= nextStripes + offset {
                        obs[ii as usize] = MSPModel_Striping_Obstacle::new_3(if nextDir == MSPMODEL_FORWARD { 0. } else { nextLength }, 0., MSPModel_Striping_ObstacleType::OBSTACLE_END, "stripeEnd", 0., Ptr::null());
                    }
                }
            }
            let pedestrians: &mut MSPModel_Interacting_Pedestrians = self.get_pedestrians(nextLane);
            if nextLane.is_walking_area() {
                Self::transform_to_current_lane_positions(&mut obs, currentDir, nextDir, currentLength, nextLength);
                // complex transformation into the coordinate system of the current lane
                // (pedestrians on next lane may walk at arbitrary angles relative to the current lane)
                let mut lateral_offset = (lane.get_width() - stripe_width()) * 0.5;
                if (stripes - nextStripes) % 2 != 0 {
                    lateral_offset += 0.5 * stripe_width();
                }
                nextDir = currentDir;
                // transform pedestrians into the current coordinate system
                for ii in 0..pedestrians.len() {
                    let p = pedestrians[ii].cast::<MSPModel_Striping_PState>();
                    if p.is_waiting_to_enter() || p.is_jammed() {
                        continue;
                    }
                    let pPos = p.get_position(&*p.get_stage(), -1);
                    let relPos = lane.get_shape_2().transform_to_vector_coordinates(&pPos, true);
                    if relPos == Position::INVALID {
                        utils::write_warningf!("Could not map position % onto lane '%'", pPos, lane.get_id());
                    }
                    let newY = relPos.y() + lateral_offset;
                    if (currentDir == MSPMODEL_FORWARD && relPos.x() >= lane.get_length()) || (currentDir == MSPMODEL_BACKWARD && relPos.x() < 0.) {
                        Self::add_closer_obstacle(&mut obs, relPos.x(), MSPModel_Striping_PState::stripe_2(newY), stripes, p.get_person().get_id(),
                            p.get_person().get_vehicle_type().get_width(), currentDir, MSPModel_Striping_ObstacleType::OBSTACLE_PED);
                        Self::add_closer_obstacle(&mut obs, relPos.x(), p.other_stripe_2(newY), stripes, p.get_person().get_id(),
                            p.get_person().get_vehicle_type().get_width(), currentDir, MSPModel_Striping_ObstacleType::OBSTACLE_PED);
                    }
                }
            } else {
                // simple transformation into the coordinate system of the current lane
                // (only need to worry about currentDir and nextDir)
                // XXX consider waitingToEnter on nextLane
                // determinism: C++ std::sort is unstable; ties are resolved by the id comparison in by_xpos_sorter
                let sorter = MSPModel_Striping_by_xpos_sorter::new(nextDir);
                pedestrians.sort_by(|a, b| {
                    if sorter.call_op(*a, *b) {
                        std::cmp::Ordering::Less
                    } else if sorter.call_op(*b, *a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                for ii in 0..pedestrians.len() {
                    let p = pedestrians[ii].cast::<MSPModel_Striping_PState>();
                    if p.is_waiting_to_enter() || p.is_jammed() {
                        continue;
                    }
                    let mut newY = p.get_pos_lat();
                    let mut pObs = MSPModel_Striping_Obstacle::new_2(&*p);
                    if nextDir != currentDir {
                        newY = (nextStripes - 1) as f64 * stripe_width() - newY;
                        pObs.speed *= -1.;
                    }
                    newY += offset as f64 * stripe_width();
                    let stripe = MSPModel_Striping_PState::stripe_2(newY);
                    if stripe >= 0 && stripe < stripes {
                        obs[stripe as usize] = pObs.clone();
                    }
                    let otherStripe = p.other_stripe_2(newY);
                    if otherStripe >= 0 && otherStripe < stripes {
                        obs[otherStripe as usize] = pObs.clone();
                    }
                }
                if nextLane.is_crossing() {
                    // add vehicle obstacles
                    let crossingEntryLink = nextLane.get_incoming_lanes()[0].viaLink;
                    let prio = crossingEntryLink.have_priority() || !crossingEntryLink.get_tl_logic().is_null();
                    Self::add_crossing_vehs(nextLane, stripes, offset as f64, nextDir, &mut obs, prio, currentDir != nextDir);
                }
                if nextLane.get_vehicle_number_with_partials() > 0 {
                    let vehObs = Self::get_vehicle_obstacles(nextLane, nextDir, Ptr::null());
                    MSPModel_Striping_PState::merge_obstacles_2(&mut obs, &vehObs, nextDir, offset);
                }
                Self::transform_to_current_lane_positions(&mut obs, currentDir, nextDir, currentLength, nextLength);
            }
            nextLanesObs.insert(nextLane, obs);
        }
        nextLanesObs.get(&nextLane).unwrap()
    }

    /// C++: `void MSPModel_Striping::transformToCurrentLanePositions(Obstacles &, int, int, double, double)` [static, private]
    // void
    // MSPModel_Striping::transformToCurrentLanePositions(Obstacles& obs, int currentDir, int nextDir, double currentLength, double nextLength) {
    //     for (Obstacle& o : obs) {
    //         if (currentDir == FORWARD) {
    //             if (nextDir == FORWARD) {
    //                 o.xFwd += currentLength;
    //                 o.xBack += currentLength;
    //             } else {
    //                 const double tmp = o.xFwd;
    //                 o.xFwd = currentLength + nextLength - o.xBack;
    //                 o.xBack = currentLength + nextLength - tmp;
    //             }
    //         } else {
    //             if (nextDir == FORWARD) {
    //                 const double tmp = o.xFwd;
    //                 o.xFwd = -o.xBack;
    //                 o.xBack = -tmp;
    //             } else {
    //                 o.xFwd -= nextLength;
    //                 o.xBack -= nextLength;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn transform_to_current_lane_positions(o: &mut MSPModel_Striping_Obstacles, currentDir: i32, nextDir: i32, currentLength: f64, nextLength: f64) {
        for obs in o.iter_mut() {
            if currentDir == MSPMODEL_FORWARD {
                if nextDir == MSPMODEL_FORWARD {
                    obs.xFwd += currentLength;
                    obs.xBack += currentLength;
                } else {
                    let tmp = obs.xFwd;
                    obs.xFwd = currentLength + nextLength - obs.xBack;
                    obs.xBack = currentLength + nextLength - tmp;
                }
            } else {
                if nextDir == MSPMODEL_FORWARD {
                    let tmp = obs.xFwd;
                    obs.xFwd = -obs.xBack;
                    obs.xBack = -tmp;
                } else {
                    obs.xFwd -= nextLength;
                    obs.xBack -= nextLength;
                }
            }
        }
    }

    /// C++: `void MSPModel_Striping::addCloserObstacle(Obstacles &, double, int, int, const std::string &, double, int, ObstacleType)` [static, private]
    // void
    // MSPModel_Striping::addCloserObstacle(Obstacles& obs, double x, int stripe, int numStripes, const std::string& id, double width, int dir, ObstacleType type) {
    //     if (stripe >= 0 && stripe < numStripes) {
    //         if ((dir == FORWARD && x - width / 2. < obs[stripe].xBack) || (dir == BACKWARD && x + width / 2. > obs[stripe].xFwd)) {
    //             obs[stripe] = Obstacle(x, 0, type, id, width);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_closer_obstacle(obs: &mut MSPModel_Striping_Obstacles, x: f64, stripe: i32, numStripes: i32, id: &str, width: f64, dir: i32, type_: MSPModel_Striping_ObstacleType) {
        if stripe >= 0 && stripe < numStripes {
            let s = stripe as usize;
            if (dir == MSPMODEL_FORWARD && x - width / 2. < obs[s].xBack) || (dir == MSPMODEL_BACKWARD && x + width / 2. > obs[s].xFwd) {
                obs[s] = MSPModel_Striping_Obstacle::new_3(x, 0., type_, id, width, Ptr::null());
            }
        }
    }

    /// C++: `int MSPModel_Striping::getStripeOffset(int, int, bool)` [static, private]
    // int
    // MSPModel_Striping::getStripeOffset(int origStripes, int destStripes, bool addRemainder) {
    //     int offset = (destStripes - origStripes) / 2;
    //     if (addRemainder) {
    //         offset += (destStripes - origStripes) % 2;
    //     }
    //     return offset;
    // }
    #[allow(unused_variables)]
    pub fn get_stripe_offset(origStripes: i32, destStripes: i32, addRemainder: bool) -> i32 {
        let mut offset = (destStripes - origStripes) / 2;
        if addRemainder {
            offset += (destStripes - origStripes) % 2;
        }
        offset
    }

    /// C++: `bool MSPModel_Striping::addCrossingVehs(const MSLane *, int, double, int, Obstacles &, bool, bool)` [static, private]
    /// add vehicles driving across
    // bool
    // MSPModel_Striping::addCrossingVehs(const MSLane* crossing, int stripes, double lateral_offset, int dir, Obstacles& obs, bool prio, bool flipY) {
    //     bool hasCrossingVehObs = false;
    //     const MSLink* crossingExitLink = crossing->getLinkCont().front();
    //     gDebugFlag1 = DEBUGCOND2(crossing);
    //     const MSLink::LinkLeaders linkLeaders = crossingExitLink->getLeaderInfo(nullptr, crossing->getLength());
    //     gDebugFlag1 = false;
    //     if (linkLeaders.size() > 0) {
    //         for (MSLink::LinkLeaders::const_iterator it = linkLeaders.begin(); it != linkLeaders.end(); ++it) {
    //             // the vehicle to enter the junction first has priority
    //             const MSVehicle* veh = (*it).vehAndGap.first;
    //             if (veh != nullptr) {
    //                 Obstacle vo((*it).distToCrossing, 0, OBSTACLE_VEHICLE, veh->getID(), veh->getVehicleType().getWidth() + 2 * minGapToVehicle, veh);
    //                 // block entry to the crossing in walking direction but allow leaving it
    //                 Obstacle voBlock = vo;
    //                 if (dir == FORWARD) {
    //                     voBlock.xBack = NUMERICAL_EPS;
    //                 } else {
    //                     voBlock.xFwd = crossing->getLength() - NUMERICAL_EPS;
    //                 }
    //                 // when approaching a priority crossings, vehicles must be able
    //                 // to brake, otherwise the person must be able to cross in time
    //                 const double distToCrossBeforeVeh = (dir == FORWARD ? vo.xFwd : crossing->getLength() - vo.xBack);
    //                 const double bGap = (prio
    //                                      ? veh->getCarFollowModel().brakeGap(veh->getSpeed(), veh->getCarFollowModel().getMaxDecel(), 0)
    //                                      : veh->getSpeed() * distToCrossBeforeVeh); // walking 1m/s
    //                 double vehYmin;
    //                 double vehYmax;
    //                 // relY increases from left to right (the other way around from vehicles)
    //                 if ((*it).fromLeft()) {
    //                     vehYmin = -(*it).vehAndGap.second + lateral_offset; // vehicle back
    //                     vehYmax = vehYmin + veh->getVehicleType().getLength() + bGap + minGapToVehicle;
    //                     vehYmin -= minGapToVehicle;
    //                 } else {
    //                     vehYmax = crossing->getWidth() + (*it).vehAndGap.second - lateral_offset; // vehicle back
    //                     vehYmin = vehYmax - veh->getVehicleType().getLength() - bGap - minGapToVehicle;
    //                     vehYmax += minGapToVehicle;
    //
    //                 }
    //                 for (int s = MAX2(0, PState::stripe(vehYmin)); s < MIN2(PState::stripe(vehYmax), stripes); ++s) {
    //                     if ((dir == FORWARD && obs[s].xBack > vo.xBack)
    //                             || (dir == BACKWARD && obs[s].xFwd < vo.xFwd)) {
    //                         if (!prio && veh->getSpeed() > SUMO_const_haltingSpeed) {
    //                             // do not enter the crossing
    //                             obs[s] = voBlock;
    //                         } else {
    //                             obs[s] = vo;
    //                         }
    //                         hasCrossingVehObs = true;
    //                     }
    //                 }
    //                 if (flipY) {
    //                     Obstacles tmp = obs;
    //                     for (int i = 0; i < (int)obs.size(); i++) {
    //                         obs[i] = tmp[obs.size() - 1 - i];
    //                     }
    //                 }
    //                 if (DEBUGCOND2(crossing)) {
    //                     std::cout << SIMTIME
    //                               << " crossingVeh=" << veh->getID()
    //                               << " lane=" << crossing->getID()
    //                               << " prio=" << prio
    //                               << " latOffset=" << lateral_offset
    //                               << " dir=" << dir
    //                               << " flipY=" << flipY
    //                               << " stripes=" << stripes
    //                               << " dist=" << (*it).distToCrossing
    //                               << " gap=" << (*it).vehAndGap.second
    //                               << " brakeGap=" << bGap
    //                               << " fromLeft=" << (*it).fromLeft()
    //                               << " distToCrossBefore=" << distToCrossBeforeVeh
    //                               << " ymin=" << vehYmin
    //                               << " ymax=" << vehYmax
    //                               << " smin=" << PState::stripe(vehYmin)
    //                               << " smax=" << PState::stripe(vehYmax)
    //                               << "\n";
    //                     DEBUG_PRINT(obs);
    //                 }
    //             }
    //         }
    //         if (hasCrossingVehObs) {
    //             // check whether the crossing is fully blocked
    //             const int reserved = getReserved((int)obs.size(), RESERVE_FOR_ONCOMING_FACTOR_JUNCTIONS);
    //             bool allBlocked = true;
    //
    //             for (int i = 0; i < (int)obs.size(); i++) {
    //                 const Obstacle& o = obs[i];
    //                 if (o.type != OBSTACLE_VEHICLE && (
    //                             (dir == FORWARD && i >= reserved) ||
    //                             (dir == BACKWARD && i < (int)obs.size() - reserved))) {
    //                     allBlocked = false;
    //                     break;
    //                 }
    //             }
    //             if (allBlocked) {
    //                 if (DEBUGCOND2(crossing)) {
    //                     std::cout << SIMTIME << " crossing=" << crossing->getID() << " allBlocked\n";
    //                 }
    //                 for (Obstacle& o : obs) {
    //                     if (dir == FORWARD) {
    //                         o.xBack = NUMERICAL_EPS;
    //                     } else {
    //                         o.xFwd = crossing->getLength() - NUMERICAL_EPS;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return hasCrossingVehObs;
    // }
    #[allow(unused_variables)]
    pub fn add_crossing_vehs(crossing: Ptr<MSLane>, stripes: i32, lateral_offset: f64, dir: i32, crossingVehs: &mut MSPModel_Striping_Obstacles, prio: bool, flipY: bool) -> bool {
        let mut hasCrossingVehObs = false;
        let crossingExitLink = crossing.get_link_cont()[0];
        // GUI-only debug dump: DEBUGCOND2 is lane isSelected(), always false in a headless run, so not ported
        let linkLeaders = crossingExitLink.get_leader_info(Ptr::null(), crossing.get_length(), Ptr::null(), false);
        if linkLeaders.len() > 0 {
            for it in linkLeaders.iter() {
                // the vehicle to enter the junction first has priority
                let veh = it.vehAndGap.0;
                if !veh.is_null() {
                    let vo = MSPModel_Striping_Obstacle::new_3(it.distToCrossing, 0., MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE, veh.get_id(),
                        veh.get_vehicle_type().get_width() + 2. * min_gap_to_vehicle(), veh.cast::<SUMOVehicle>());
                    // block entry to the crossing in walking direction but allow leaving it
                    let mut voBlock = vo.clone();
                    if dir == MSPMODEL_FORWARD {
                        voBlock.xBack = utils::geom::NUMERICAL_EPS;
                    } else {
                        voBlock.xFwd = crossing.get_length() - utils::geom::NUMERICAL_EPS;
                    }
                    // when approaching a priority crossings, vehicles must be able
                    // to brake, otherwise the person must be able to cross in time
                    let distToCrossBeforeVeh = if dir == MSPMODEL_FORWARD { vo.xFwd } else { crossing.get_length() - vo.xBack };
                    let bGap = if prio {
                        veh.get_car_follow_model().brake_gap_2(veh.get_speed(), veh.get_car_follow_model().get_max_decel(), 0.)
                    } else {
                        veh.get_speed() * distToCrossBeforeVeh // walking 1m/s
                    };
                    let mut vehYmin: f64;
                    let mut vehYmax: f64;
                    // relY increases from left to right (the other way around from vehicles)
                    if it.from_left() {
                        vehYmin = -it.vehAndGap.1 + lateral_offset; // vehicle back
                        vehYmax = vehYmin + veh.get_vehicle_type().get_length() + bGap + min_gap_to_vehicle();
                        vehYmin -= min_gap_to_vehicle();
                    } else {
                        vehYmax = crossing.get_width() + it.vehAndGap.1 - lateral_offset; // vehicle back
                        vehYmin = vehYmax - veh.get_vehicle_type().get_length() - bGap - min_gap_to_vehicle();
                        vehYmax += min_gap_to_vehicle();
                    }
                    let sEnd = std::cmp::min(MSPModel_Striping_PState::stripe_2(vehYmax), stripes);
                    for s in std::cmp::max(0, MSPModel_Striping_PState::stripe_2(vehYmin))..sEnd {
                        let su = s as usize;
                        if (dir == MSPMODEL_FORWARD && crossingVehs[su].xBack > vo.xBack) || (dir == MSPMODEL_BACKWARD && crossingVehs[su].xFwd < vo.xFwd) {
                            if !prio && veh.get_speed() > utils::common::std_defs::SUMO_CONST_HALTING_SPEED {
                                // do not enter the crossing
                                crossingVehs[su] = voBlock.clone();
                            } else {
                                crossingVehs[su] = vo.clone();
                            }
                            hasCrossingVehObs = true;
                        }
                    }
                    if flipY {
                        let tmp = crossingVehs.clone();
                        for i in 0..crossingVehs.len() {
                            crossingVehs[i] = tmp[crossingVehs.len() - 1 - i].clone();
                        }
                    }
                }
            }
            if hasCrossingVehObs {
                // check whether the crossing is fully blocked
                let reserved = Self::get_reserved(crossingVehs.len() as i32, reserve_for_oncoming_factor_junctions());
                let mut allBlocked = true;
                for i in 0..crossingVehs.len() as i32 {
                    let o = &crossingVehs[i as usize];
                    if o.type_ != MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE
                        && ((dir == MSPMODEL_FORWARD && i >= reserved) || (dir == MSPMODEL_BACKWARD && i < crossingVehs.len() as i32 - reserved))
                    {
                        allBlocked = false;
                        break;
                    }
                }
                if allBlocked {
                    for o in crossingVehs.iter_mut() {
                        if dir == MSPMODEL_FORWARD {
                            o.xBack = utils::geom::NUMERICAL_EPS;
                        } else {
                            o.xFwd = crossing.get_length() - utils::geom::NUMERICAL_EPS;
                        }
                    }
                }
            }
        }
        hasCrossingVehObs
    }

    /// C++: `Obstacles MSPModel_Striping::getVehicleObstacles(const MSLane *, int, PState *)` [static, private]
    /// retrieve vehicle obstacles on the given lane
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_vehicle_obstacles(lane: Ptr<MSLane>, dir: i32, ped: Ptr<MSPModel_Striping_PState>) -> MSPModel_Striping_Obstacles {
        // GUI-only debug dump: DEBUGCOND / DEBUGCOND2 are isSelected(), always false in a headless run, so not ported
        let stripes = Self::num_stripes(lane);
        let mut vehObs: MSPModel_Striping_Obstacles = (0..stripes).map(|_| MSPModel_Striping_Obstacle::new(dir, DIST_FAR_AWAY)).collect();
        let mut current: i32 = -1;
        let mut minX = 0.;
        let mut maxX = 0.;
        let mut pRelY = -1.;
        let mut pWidth = 0.;
        if !ped.is_null() {
            current = ped.stripe();
            minX = ped.get_min_x(true);
            maxX = ped.get_max_x(true);
            pRelY = ped.get_pos_lat();
            pWidth = ped.get_person().get_vehicle_type().get_width();
        } else if dir == MSPMODEL_BACKWARD {
            // checking vehicles on the next lane. Use entry point as reference
            minX = lane.get_length();
            maxX = lane.get_length();
        }

        let (begin, end) = if dir == MSPMODEL_FORWARD {
            (lane.any_vehicles_upstream_begin(), lane.any_vehicles_upstream_end())
        } else {
            (lane.any_vehicles_begin(), lane.any_vehicles_end())
        };
        let mut it = begin;
        while it.ne_op(&end) {
            let veh = it.mul_op();
            let bidi = veh.get_lane() == lane.get_bidi_lane();
            let vehBack = veh.get_back_position_on_lane(lane);
            let mut vehFront = vehBack + veh.get_vehicle_type().get_length();
            // ensure that vehicles are not blocked
            let vehNextSpeed = if veh.get_waiting_time(false) > utils::common::time::delta_t() { 0. } else { f64::max(veh.get_speed(), 1.0) };
            let clearance = SAFETY_GAP + vehNextSpeed * LOOKAHEAD_SAMEDIR;
            // boundaries for range checking
            let vehXMax: f64;
            let vehXMin: f64;
            let vehXMaxCheck: f64;
            let vehXMinCheck: f64;
            if bidi {
                vehFront = vehBack - veh.get_vehicle_type().get_length();
                vehXMax = vehBack + SAFETY_GAP;
                vehXMin = vehFront - clearance;
                if dir == MSPMODEL_FORWARD {
                    vehXMaxCheck = vehBack + utils::geom::NUMERICAL_EPS;
                    vehXMinCheck = vehFront - LOOKAROUND_VEHICLES;
                } else {
                    vehXMaxCheck = vehBack + LOOKAHEAD_SAMEDIR;
                    vehXMinCheck = vehFront - clearance;
                }
            } else {
                vehXMax = vehFront + clearance;
                vehXMin = vehBack - SAFETY_GAP;
                if dir == MSPMODEL_FORWARD {
                    vehXMaxCheck = vehFront + clearance;
                    vehXMinCheck = vehBack - LOOKAHEAD_SAMEDIR;
                } else {
                    vehXMaxCheck = vehFront + LOOKAROUND_VEHICLES;
                    vehXMinCheck = vehBack - utils::geom::NUMERICAL_EPS;
                }
            }
            // C++ tests the double vehXMinCheck as a bool (non-zero is true)
            if vehXMaxCheck > minX && vehXMinCheck != 0. && vehXMinCheck <= maxX {
                let mut vo = MSPModel_Striping_Obstacle::new_3(vehBack, veh.get_speed() * if bidi { -1. } else { 1. }, MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE,
                    veh.get_id(), 0., veh.cast::<SUMOVehicle>());
                // moving vehicles block space along their path
                vo.xFwd = vehXMax;
                vo.xBack = vehXMin;
                // relY increases from left to right (the other way around from vehicles)
                // XXX lateral offset for partial vehicles
                let posLat = veh.get_lateral_position_on_lane() * if bidi { -1. } else { 1. };
                let vehYmax = 0.5 * (lane.get_width() + veh.get_vehicle_type().get_width() - stripe_width()) - posLat;
                let vehYmin = vehYmax - veh.get_vehicle_type().get_width();
                let sEnd = std::cmp::min(MSPModel_Striping_PState::stripe_2(vehYmax) + 1, stripes);
                for s in std::cmp::max(0, MSPModel_Striping_PState::stripe_2(vehYmin))..sEnd {
                    let su = s as usize;
                    let prior = vehObs[su].clone();
                    vehObs[su] = vo.clone();
                    if s == current && vehFront + SAFETY_GAP < minX {
                        // ignore if already overlapping while vehicle is still behind
                        if pRelY - pWidth < vehYmax && pRelY + pWidth > vehYmin && dir == MSPMODEL_FORWARD {
                            if dir == MSPMODEL_FORWARD {
                                vehObs[su] = prior;
                            } else {
                                vehObs[su].xFwd = f64::min(vo.xFwd, vehFront + SAFETY_GAP);
                            }
                        }
                    }
                }
            }
            it.op__2();
        }
        vehObs
    }

    /// C++: `bool MSPModel_Striping::addVehicleFoe(const MSVehicle *, const MSLane *, const Position &, double, double, double, double, double, Pedestrians &, Pedestrians &)` [static, private]
    // bool
    // MSPModel_Striping::addVehicleFoe(const MSVehicle* veh, const MSLane* walkingarea, const Position& relPos, double xWidth, double yWidth, double lateral_offset,
    //                                  double minY, double maxY, Pedestrians& toDelete, Pedestrians& transformedPeds) {
    //     if (relPos != Position::INVALID) {
    //         const double newY = relPos.y() + lateral_offset;
    //         if (newY >= minY && newY <= maxY) {
    //             PState* tp = new PStateVehicle(veh, walkingarea, relPos.x(), newY, xWidth, yWidth);
    //             //std::cout << SIMTIME << " addVehicleFoe=" << veh->getID() << " rx=" << relPos.x() << " ry=" << newY << " s=" << tp->stripe() << " o=" << tp->otherStripe() << "\n";
    //             toDelete.push_back(tp);
    //             transformedPeds.push_back(tp);
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_foe(veh: Ptr<MSVehicle>, walkingarea: Ptr<MSLane>, relPos: &Position, xWidth: f64, yWidth: f64, lateral_offset: f64, minY: f64, maxY: f64, toDelete: &mut MSPModel_Interacting_Pedestrians, transformedPeds: &mut MSPModel_Interacting_Pedestrians, owned: &mut Vec<Box<dyn std::any::Any>>) -> bool {
        // `owned` keeps the PStateVehicle alive; C++ allocates it with new and deletes it via toDelete
        if *relPos != Position::INVALID {
            let newY = relPos.y() + lateral_offset;
            if newY >= minY && newY <= maxY {
                let mut tp = Box::new(MSPModel_Striping_PStateVehicle::new(veh, walkingarea, relPos.x(), newY, xWidth, yWidth));
                //std::cout << SIMTIME << " addVehicleFoe=" << veh->getID() << " rx=" << relPos.x() << " ry=" << newY << " s=" << tp->stripe() << " o=" << tp->otherStripe() << "\n";
                let tpp = Ptr::from_mut(&mut *tp).cast::<MSPModel_InteractingState>();
                owned.push(tp);
                toDelete.push(tpp);
                transformedPeds.push(tpp);
            }
            true
        } else {
            false
        }
    }

    /// C++: `int MSPModel_Striping::getReserved(int, double)` [static, private]
    // int
    // MSPModel_Striping::getReserved(int stripes, double factor) {
    //     return MIN2(
    //                (int)floor(stripes * factor),
    //                (int)floor(RESERVE_FOR_ONCOMING_MAX / stripeWidth));
    // }
    #[allow(unused_variables)]
    pub fn get_reserved(stripes: i32, factor: f64) -> i32 {
        std::cmp::min((stripes as f64 * factor).floor() as i32, (reserve_for_oncoming_max() / stripe_width()).floor() as i32)
    }

    /// C++: `void MSPModel_Striping::registerCrossingApproach(const PState &, const MSLane *, const MSLane *)` [static, private]
    /// register pedestrian approach with the junction model
    // void
    // MSPModel_Striping::registerCrossingApproach(const PState& ped, const MSLane* crossing, const MSLane* beforeWA) {
    //     // person has entered the walkingarea
    //     SUMOTime arrivalTime = SIMSTEP;
    //     assert(ped.getLane()->isWalkingArea());
    //     const WalkingAreaPath* wa = getWalkingAreaPath(&ped.getLane()->getEdge(), beforeWA, crossing);
    //     const double speed = ped.getStage()->getMaxSpeed(ped.getPerson()) * (1 - dawdling / 2);
    //     arrivalTime += TIME2STEPS(wa->length / speed);
    //     SUMOTime leavingTime = arrivalTime + TIME2STEPS(crossing->getLength() / speed);
    //     crossing->getIncomingLanes()[0].viaLink->setApproachingPerson(ped.getPerson(), arrivalTime, leavingTime);
    //     if DEBUGCOND(ped) {
    //         std::cout << SIMTIME << " register " << ped.getPerson()->getID() << " at crossing " << crossing->getID() << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn register_crossing_approach(ped: &MSPModel_Striping_PState, crossing: Ptr<MSLane>, beforeWA: Ptr<MSLane>) {
        // person has entered the walkingarea
        let mut arrivalTime: SUMOTime = MSNet::get_instance().get_current_time_step();
        debug_assert!(ped.get_lane().is_walking_area());
        let wa = Self::get_walking_area_path(Ptr::from_mut(ped.get_lane().get_edge()), beforeWA, crossing);
        let speed = ped.get_stage().get_max_speed(ped.get_person().cast::<MSTransportable>()) * (1. - dawdling() / 2.);
        arrivalTime += utils::common::time::time2steps(wa.length / speed);
        let leavingTime = arrivalTime + utils::common::time::time2steps(crossing.get_length() / speed);
        crossing.get_incoming_lanes()[0].viaLink.m().set_approaching_person(ped.get_person(), arrivalTime, leavingTime);
        // GUI-only debug dump: DEBUGCOND is person isSelected(), always false in a headless run, so not ported
    }

}

/// C++ class `MSPModel_Striping::MovePedestrians` (microsim/transportables/MSPModel_Striping.h)
#[repr(C)]
pub struct MSPModel_Striping_MovePedestrians {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myModel: Ptr<MSPModel_Striping>,
}

impl std::ops::Deref for MSPModel_Striping_MovePedestrians {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_Striping_MovePedestrians {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSPModel_Striping_MovePedestrians {
    /// C++: `void MovePedestrians::MovePedestrians(MSPModel_Striping *)`
    // MovePedestrians(MSPModel_Striping* model) : myModel(model) {}
    #[allow(unused_variables)]
    pub fn new(model: Ptr<MSPModel_Striping>) -> Self {
        MSPModel_Striping_MovePedestrians { base: (), myModel: model }
    }

    /// C++: `void MovePedestrians::~MovePedestrians() noexcept` [virtual]
    // ~MovePedestrians() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `SUMOTime MovePedestrians::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSPModel_Striping::MovePedestrians::execute(SUMOTime currentTime) {
    //     std::set<MSPerson*> changedLane;
    //     myModel->moveInDirection(currentTime, changedLane, FORWARD);
    //     myModel->moveInDirection(currentTime, changedLane, BACKWARD);
    //     // DEBUG
    // #ifdef LOG_ALL
    //     for (ActiveLanes::const_iterator it_lane = myModel->getActiveLanes().begin(); it_lane != myModel->getActiveLanes().end(); ++it_lane) {
    //         const MSLane* lane = it_lane->first;
    //         Pedestrians pedestrians = it_lane->second;
    //         if (pedestrians.size() == 0) {
    //             continue;
    //         }
    //         sort(pedestrians.begin(), pedestrians.end(), by_xpos_sorter(FORWARD));
    //         std::cout << SIMTIME << " lane=" << lane->getID();
    //         for (int ii = 0; ii < (int)pedestrians.size(); ++ii) {
    //             const PState& p = *pedestrians[ii];
    //             std::cout << " (" << p.getPerson()->getID() << " " << p.myEdgePos << "," << p.myPosLat << " " << p.getDirection() << ")";
    //         }
    //         std::cout << "\n";
    //     }
    // #endif
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        let mut changedLane: BTreeSet<Ptr<MSPerson>> = BTreeSet::new();
        self.myModel.m().move_in_direction(currentTime, &mut changedLane, MSPMODEL_FORWARD);
        self.myModel.m().move_in_direction(currentTime, &mut changedLane, MSPMODEL_BACKWARD);
        // LOG_ALL is a compile-time option, commented out in C++ 1.28.0 (off by default), so not ported
        utils::common::time::delta_t()
    }

    /// C++: `MovePedestrians & MovePedestrians::operator=(const MovePedestrians &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_Striping_MovePedestrians) -> &mut MSPModel_Striping_MovePedestrians {
        // private and never defined in C++ (invalidated), so it must not be called
        panic!("MSPModel_Striping::MovePedestrians::operator= is invalidated")
    }

}

impl crate::microsim::ms_event_control::Command for MSPModel_Striping_MovePedestrians {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSPModel_Striping_MovePedestrians::execute(self, currentTime)
    }
}

/// C++ class `MSPModel_Striping::NextLaneInfo` (microsim/transportables/MSPModel_Striping.h)
pub struct MSPModel_Striping_NextLaneInfo {
    /// the next lane to be used
    pub lane: Ptr<MSLane>,
    /// the link from the current lane to the next lane
    pub link: Ptr<MSLink>,
    /// the direction on the next lane
    pub dir: i32,
}

impl MSPModel_Striping_NextLaneInfo {
    /// C++: `void NextLaneInfo::NextLaneInfo(const MSLane *, const MSLink *, int)`
    // NextLaneInfo(const MSLane* _lane, const MSLink* _link, int _dir) :
    //             lane(_lane),
    //             link(_link),
    //             dir(_dir) {
    //         }
    #[allow(unused_variables)]
    pub fn new(_lane: Ptr<MSLane>, _link: Ptr<MSLink>, _dir: i32) -> Self {
        MSPModel_Striping_NextLaneInfo { lane: _lane, link: _link, dir: _dir }
    }

    /// C++: `void NextLaneInfo::NextLaneInfo()`
    // NextLaneInfo() :
    //             lane(0),
    //             link(0),
    //             dir(UNDEFINED_DIRECTION) {
    //         }
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        MSPModel_Striping_NextLaneInfo { lane: Ptr::null(), link: Ptr::null(), dir: MSPMODEL_UNDEFINED_DIRECTION }
    }

}

/// information regarding surround Pedestrians (and potentially other things)
/// C++ class `MSPModel_Striping::Obstacle` (microsim/transportables/MSPModel_Striping.h)
#[derive(Clone)]
pub struct MSPModel_Striping_Obstacle {
    /// maximal position on the current lane in forward direction
    pub xFwd: f64,
    /// maximal position on the current lane in backward direction
    pub xBack: f64,
    /// speed relative to lane direction (positive means in the same direction)
    pub speed: f64,
    /// whether this obstacle denotes a border, a vehicle or a pedestrian
    pub type_: MSPModel_Striping_ObstacleType,
    /// the id / description of the obstacle
    pub description: String,
    /// a pointer to the vehicle if this obstacle is one
    pub vehicle: Ptr<SUMOVehicle>,
}

impl MSPModel_Striping_Obstacle {
    /// C++: `void Obstacle::Obstacle(int, double)`
    /// create No-Obstacle
    // MSPModel_Striping::Obstacle::Obstacle(int dir, double dist) :
    //     xFwd(dir * dist),  // by default, far away when seen in dir
    //     xBack(dir * dist),  // by default, far away when seen in dir
    //     speed(0),
    //     type(OBSTACLE_NONE),
    //     description("") {
    // }
    #[allow(unused_variables)]
    pub fn new(dir: i32, dist: f64) -> Self {
        MSPModel_Striping_Obstacle {
            xFwd: dir as f64 * dist, // by default, far away when seen in dir
            xBack: dir as f64 * dist, // by default, far away when seen in dir
            speed: 0.,
            type_: MSPModel_Striping_ObstacleType::OBSTACLE_NONE,
            description: String::new(),
            vehicle: Ptr::null(),
        }
    }

    /// C++: `void Obstacle::Obstacle(const PState &)`
    /// create an obstacle from ped for ego moving in dir
    // MSPModel_Striping::Obstacle::Obstacle(const PState& ped) :
    //     xFwd(ped.getMaxX()),
    //     xBack(ped.getMinX()),
    //     speed(ped.getDirection() * ped.getSpeed()),
    //     type(ped.getOType()),
    //     description(ped.getID()) {
    //     assert(!ped.isWaitingToEnter());
    //     if (type == OBSTACLE_VEHICLE) {
    //         vehicle = static_cast<const PStateVehicle&>(ped).getVehicle();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(ped: &MSPModel_Striping_PState) -> Self {
        debug_assert!(!ped.is_waiting_to_enter());
        let type_ = ped.get_o_type();
        let mut vehicle: Ptr<SUMOVehicle> = Ptr::null();
        if type_ == MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE {
            vehicle = Ptr::new(ped).cast::<MSPModel_Striping_PStateVehicle>().get_vehicle().cast::<SUMOVehicle>();
        }
        MSPModel_Striping_Obstacle {
            xFwd: ped.get_max_x(true),
            xBack: ped.get_min_x(true),
            speed: ped.get_direction() as f64 * ped.get_speed(),
            type_,
            description: ped.get_id().to_string(),
            vehicle,
        }
    }

    /// C++: `void Obstacle::Obstacle(double, double, ObstacleType, const std::string &, const double, const SUMOVehicle *)`
    /// create an obstacle from explicit values
    // Obstacle(double _x, double _speed, ObstacleType _type, const std::string& _description, const double width = 0., const SUMOVehicle* veh = nullptr)
    //             : xFwd(_x + width / 2.), xBack(_x - width / 2.), speed(_speed), type(_type), description(_description), vehicle(veh) {}
    #[allow(unused_variables)]
    pub fn new_3(_x: f64, _speed: f64, _type: MSPModel_Striping_ObstacleType, _description: &str, width: f64, veh: Ptr<SUMOVehicle>) -> Self {
        MSPModel_Striping_Obstacle {
            xFwd: _x + width / 2.,
            xBack: _x - width / 2.,
            speed: _speed,
            type_: _type,
            description: _description.to_string(),
            vehicle: veh,
        }
    }

    /// C++: `bool Obstacle::closer(const Obstacle &, int)`
    // bool
    // MSPModel_Striping::Obstacle::closer(const Obstacle& o, int dir) {
    //     if (dir == FORWARD) {
    //         return xBack <= o.xBack;
    //     } else {
    //         return xFwd >= o.xFwd;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn closer(&mut self, o: &MSPModel_Striping_Obstacle, dir: i32) -> bool {
        if dir == MSPMODEL_FORWARD {
            self.xBack <= o.xBack
        } else {
            self.xFwd >= o.xFwd
        }
    }

}

/// Container for pedestrian state and individual position update function
/// C++ class `MSPModel_Striping::PState` (microsim/transportables/MSPModel_Striping.h)
#[repr(C)]
pub struct MSPModel_Striping_PState {
    /// C++ base class `MSPModel_InteractingState`
    pub base: MSPModel_InteractingState,
    /// information about the upcoming lane
    pub myNLI: MSPModel_Striping_NextLaneInfo,
    /// the current walkingAreaPath or 0
    pub myWalkingAreaPath: Ptr<MSPModel_Striping_WalkingAreaPath>,
}

impl std::ops::Deref for MSPModel_Striping_PState {
    type Target = MSPModel_InteractingState;
    fn deref(&self) -> &MSPModel_InteractingState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_Striping_PState {
    fn deref_mut(&mut self) -> &mut MSPModel_InteractingState {
        &mut self.base
    }
}

impl MSPModel_Striping_PState {
    /// C++: `void PState::PState(MSPerson *, MSStageMoving *, const MSLane *)`
    // MSPModel_Striping::PState::PState(MSPerson* person, MSStageMoving* stage, const MSLane* lane):
    //     MSPModel_InteractingState(person, stage, lane),
    //     myWalkingAreaPath(nullptr) {
    //     const MSEdge* currentEdge = &lane->getEdge();
    //     const ConstMSEdgeVector& route = myStage->getRoute();
    //     assert(!route.empty());
    //     myDir = FORWARD;
    //     if (route.size() == 1) {
    //         // only a single edge, move towards end pos
    //         myDir = (myEdgePos <= myStage->getArrivalPos()) ? FORWARD : BACKWARD;
    //     } else if (route.front()->getFunction() != SumoXMLEdgeFunc::NORMAL) {
    //         // start on an intersection
    //         if (route.front()->isWalkingArea()) {
    //             myWalkingAreaPath = getArbitraryPath(route.front());
    //         }
    //     } else {
    //         int passedFwd = 0;
    //         int passedBwd = 0;
    //         const bool mayStartForward = canTraverse(FORWARD, route, passedFwd) != UNDEFINED_DIRECTION;
    //         const bool mayStartBackward = canTraverse(BACKWARD, route, passedBwd) != UNDEFINED_DIRECTION;
    //         if DEBUGCOND(*this) {
    //             std::cout << "  initialize dir for " << myPerson->getID() << " forward=" << mayStartForward << " backward=" << mayStartBackward << "\n";
    //         }
    //         if (mayStartForward && mayStartBackward) {
    //             // figure out the best direction via routing
    //             ConstMSEdgeVector crossingRoute;
    //             MSNet::getInstance()->getPedestrianRouter(0).compute(currentEdge, route.back(), myEdgePos, myStage->getArrivalPos(),
    //                     myStage->getMaxSpeed(person), 0, nullptr, person->getVTypeParameter(), crossingRoute, true);
    //             if (crossingRoute.size() > 1) {
    //                 // route found
    //                 const MSEdge* nextEdge = crossingRoute[1];
    //                 if (nextEdge->getFromJunction() == currentEdge->getFromJunction() || nextEdge->getToJunction() == currentEdge->getFromJunction()) {
    //                     myDir = BACKWARD;
    //                 }
    //             }
    //             if DEBUGCOND(*this) {
    //                 std::cout << " crossingRoute=" << toString(crossingRoute) << "\n";
    //             }
    //         } else if (!mayStartForward && !mayStartBackward) {
    //             int lastDisconnect = passedFwd >= passedBwd ? passedFwd : passedBwd;
    //             std::string dLoc;
    //             if (route.size() > 2) {
    //                 dLoc = TLF(" between edge '%' and edge '%'", route[lastDisconnect - 1]->getID(), route[lastDisconnect]->getID());
    //             }
    //             WRITE_WARNINGF(TL("Person '%' walking from edge '%' to edge '%' has a disconnect%, time=%."),
    //                            myPerson->getID(), route.front()->getID(), route.back()->getID(), dLoc, SIMTIME);
    //             myDir =  passedFwd >= passedBwd ? FORWARD : BACKWARD;
    //         } else {
    //             myDir = !mayStartBackward ? FORWARD : BACKWARD;
    //         }
    //     }
    //     if (myPosLat == UNSPECIFIED_POS_LAT || myLegacyPosLat) {
    //         if (myPosLat == UNSPECIFIED_POS_LAT) {
    //             myPosLat = 0;
    //         }
    //         if (lane->getVehicleNumberWithPartials() > 0 && myPosLat == 0) {
    //             // better start next to the road if nothing was specified
    //             myPosLat -= stripeWidth;
    //         }
    //         if (myDir == FORWARD || lane->getPermissions() != SVC_PEDESTRIAN) {
    //             // start at the right side of the sidewalk on shared roads
    //             myPosLat = stripeWidth * (numStripes(lane) - 1) - myPosLat;
    //         }
    //     } else if (myPosLat == RANDOM_POS_LAT) {
    //         myPosLat = RandHelper::rand() * stripeWidth * (numStripes(lane) - 1);
    //     } else {
    //         // vehicle to striping coordinate system
    //         myPosLat = posLatConversion(myPosLat, lane->getWidth());
    //     }
    //     if DEBUGCOND(*this) {
    //         std::cout << "  added new pedestrian " << myPerson->getID() << " on " << lane->getID() << " myEdgePos=" << myEdgePos << " myPosLat=" << myPosLat << " dir=" << myDir << " route=" << toString(myStage->getRoute()) << "\n";
    //     }
    //
    //     myNLI = getNextLane(*this, lane, nullptr);
    // }
    #[allow(unused_variables)]
    pub fn new(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, lane: Ptr<MSLane>) -> Self {
        // MSPModel_InteractingState(person, stage, lane) inlined: its ctor is still todo!() in msp_model_interacting.rs
        let mut s = MSPModel_Striping_PState {
            base: MSPModel_InteractingState {
                base: MSTransportableStateAdapter { vt: &MSPMODEL_STRIPING_PSTATE_VT },
                myPerson: person,
                myStage: stage,
                myLane: lane,
                myEdgePos: if !stage.is_null() { stage.get_depart_pos() } else { 0. },
                myPosLat: if !stage.is_null() { stage.get_depart_pos_lat() } else { 0. },
                myDir: MSPMODEL_UNDEFINED_DIRECTION,
                mySpeed: 0.,
                mySpeedLat: 0.,
                myWaitingToEnter: !person.is_null(),
                myWaitingTime: 0,
                myTotalWaitingTime: 0,
                myAmJammed: false,
                myRemoteXYPos: Position::INVALID,
                myAngle: std::cell::Cell::new(f64::MAX),
            },
            myNLI: MSPModel_Striping_NextLaneInfo::new_2(),
            myWalkingAreaPath: Ptr::null(),
        };
        let currentEdge = Ptr::from_mut(lane.get_edge());
        let route = stage.get_route();
        debug_assert!(!route.is_empty());
        s.myDir = MSPMODEL_FORWARD;
        if route.len() == 1 {
            // only a single edge, move towards end pos
            s.myDir = if s.myEdgePos <= stage.get_arrival_pos() { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
        } else if route[0].get_function() != SumoXMLEdgeFunc::NORMAL {
            // start on an intersection
            if route[0].is_walking_area() {
                s.myWalkingAreaPath = MSPModel_Striping::get_arbitrary_path(route[0]);
            }
        } else {
            let mut passedFwd = 0;
            let mut passedBwd = 0;
            let mayStartForward = MSPModel::can_traverse(MSPMODEL_FORWARD, route, &mut passedFwd) != MSPMODEL_UNDEFINED_DIRECTION;
            let mayStartBackward = MSPModel::can_traverse(MSPMODEL_BACKWARD, route, &mut passedBwd) != MSPMODEL_UNDEFINED_DIRECTION;
            if mayStartForward && mayStartBackward {
                // figure out the best direction via routing
                let mut crossingRoute: Vec<Ptr<MSEdge>> = Vec::new();
                MSNet::get_instance().get_pedestrian_router(0, &MSNet_Prohibitions::new()).compute(currentEdge, *route.last().unwrap(), s.myEdgePos, stage.get_arrival_pos(),
                    stage.get_max_speed(person.cast::<MSTransportable>()), 0, Ptr::null(), person.get_v_type_parameter(), &mut crossingRoute, true);
                if crossingRoute.len() > 1 {
                    // route found
                    let nextEdge = crossingRoute[1];
                    if nextEdge.get_from_junction() == currentEdge.get_from_junction() || nextEdge.get_to_junction() == currentEdge.get_from_junction() {
                        s.myDir = MSPMODEL_BACKWARD;
                    }
                }
            } else if !mayStartForward && !mayStartBackward {
                let lastDisconnect = (if passedFwd >= passedBwd { passedFwd } else { passedBwd }) as usize;
                let mut dLoc = String::new();
                if route.len() > 2 {
                    dLoc = utils::tlf!(" between edge '%' and edge '%'", route[lastDisconnect - 1].get_id(), route[lastDisconnect].get_id());
                }
                utils::write_warningf!("Person '%' walking from edge '%' to edge '%' has a disconnect%, time=%.", person.get_id(), route[0].get_id(),
                    route[route.len() - 1].get_id(), dLoc.as_str(), MSNet::get_instance().get_current_time_step());
                s.myDir = if passedFwd >= passedBwd { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
            } else {
                s.myDir = if !mayStartBackward { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
            }
        }
        // GUI-only debug dump: DEBUGCOND is person isSelected(), always false in a headless run, so not ported
        if s.myPosLat == UNSPECIFIED_POS_LAT || my_legacy_pos_lat() {
            if s.myPosLat == UNSPECIFIED_POS_LAT {
                s.myPosLat = 0.;
            }
            if lane.get_vehicle_number_with_partials() > 0 && s.myPosLat == 0. {
                // better start next to the road if nothing was specified
                s.myPosLat -= stripe_width();
            }
            if s.myDir == MSPMODEL_FORWARD || lane.get_permissions() != SVC_PEDESTRIAN {
                // start at the right side of the sidewalk on shared roads
                s.myPosLat = stripe_width() * (MSPModel_Striping::num_stripes(lane) - 1) as f64 - s.myPosLat;
            }
        } else if s.myPosLat == RANDOM_POS_LAT {
            s.myPosLat = utils::common::rand_helper::rand() * stripe_width() * (MSPModel_Striping::num_stripes(lane) - 1) as f64;
        } else {
            // vehicle to striping coordinate system
            s.myPosLat = MSPModel_Striping::pos_lat_conversion(s.myPosLat, lane.get_width());
        }
        s.myNLI = MSPModel_Striping::get_next_lane(&s, lane, Ptr::null());
        s
    }

    /// C++: `void PState::PState(MSPerson *, MSStageMoving *, std::istringstream *)`
    /// constructor for loading state
    // MSPModel_Striping::PState::PState(MSPerson* person, MSStageMoving* stage, std::istringstream* in) :
    //     MSPModel_InteractingState(person, stage, nullptr),
    //     myWalkingAreaPath(nullptr) {
    //     if (in != nullptr) {
    //         std::string laneID;
    //         std::string wapLaneFrom;
    //         std::string wapLaneTo;
    //         std::string nextLaneID;
    //         std::string nextLinkFrom;
    //         std::string nextLinkTo;
    //         int nextDir;
    //
    //         (*in) >> laneID
    //               >> myEdgePos >> myPosLat >> myDir >> mySpeed >> mySpeedLat >> myWaitingToEnter
    //               >> myWaitingTime >> myTotalWaitingTime
    //               >> wapLaneFrom >> wapLaneTo
    //               >> myAmJammed
    //               >> nextLaneID
    //               >> nextLinkFrom
    //               >> nextLinkTo
    //               >> nextDir;
    //
    //
    //         myLane = MSLane::dictionary(laneID);
    //         if (myLane == nullptr) {
    //             throw ProcessError("Unknown lane '" + laneID + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //         }
    //
    //         MSLane* nextLane = nullptr;
    //         if (nextLaneID != "null") {
    //             nextLane = MSLane::dictionary(nextLaneID);
    //             if (nextLane == nullptr) {
    //                 throw ProcessError("Unknown next lane '" + nextLaneID + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //         }
    //         const MSLink* link = nullptr;
    //         if (nextLinkFrom != "null") {
    //             MSLane* from = MSLane::dictionary(nextLinkFrom);
    //             MSLane* to = MSLane::dictionary(nextLinkTo);
    //             if (from == nullptr) {
    //                 throw ProcessError("Unknown link origin lane '" + nextLinkFrom + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //             if (to == nullptr) {
    //                 throw ProcessError("Unknown link destination lane '" + nextLinkTo + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //             link = from->getLinkTo(to);
    //         }
    //         myNLI =  NextLaneInfo(nextLane, link, nextDir);
    //
    //         if (wapLaneFrom != "null") {
    //             MSLane* from = MSLane::dictionary(wapLaneFrom);
    //             MSLane* to = MSLane::dictionary(wapLaneTo);
    //             if (from == nullptr) {
    //                 throw ProcessError("Unknown walkingAreaPath origin lane '" + wapLaneFrom + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //             if (to == nullptr) {
    //                 throw ProcessError("Unknown walkingAreaPath destination lane '" + wapLaneTo + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //             const auto pathIt = myWalkingAreaPaths.find(std::make_pair(from, to));
    //             if (pathIt != myWalkingAreaPaths.end()) {
    //                 myWalkingAreaPath = &pathIt->second;
    //             } else {
    //                 throw ProcessError("Unknown walkingAreaPath from lane '" + wapLaneFrom + "' to lane '" + wapLaneTo + "' when loading walk for person '" + myPerson->getID() + "' from state.");
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, in_: &mut std::str::SplitWhitespace<'_>) -> Self {
        let mut s = MSPModel_Striping_PState {
            base: MSPModel_InteractingState {
                base: MSTransportableStateAdapter { vt: &MSPMODEL_STRIPING_PSTATE_VT },
                myPerson: person,
                myStage: stage,
                myLane: Ptr::null(),
                myEdgePos: if !stage.is_null() { stage.get_depart_pos() } else { 0. },
                myPosLat: if !stage.is_null() { stage.get_depart_pos_lat() } else { 0. },
                myDir: MSPMODEL_UNDEFINED_DIRECTION,
                mySpeed: 0.,
                mySpeedLat: 0.,
                myWaitingToEnter: !person.is_null(),
                myWaitingTime: 0,
                myTotalWaitingTime: 0,
                myAmJammed: false,
                myRemoteXYPos: Position::INVALID,
                myAngle: std::cell::Cell::new(f64::MAX),
            },
            myNLI: MSPModel_Striping_NextLaneInfo::new_2(),
            myWalkingAreaPath: Ptr::null(),
        };
        let laneID = ps_next_str(in_);
        s.base.myEdgePos = ps_next(in_);
        s.base.myPosLat = ps_next(in_);
        s.base.myDir = ps_next(in_);
        s.base.mySpeed = ps_next(in_);
        s.base.mySpeedLat = ps_next(in_);
        s.base.myWaitingToEnter = ps_next_bool(in_);
        s.base.myWaitingTime = ps_next(in_);
        s.base.myTotalWaitingTime = ps_next(in_);
        let wapLaneFrom = ps_next_str(in_);
        let wapLaneTo = ps_next_str(in_);
        s.base.myAmJammed = ps_next_bool(in_);
        let nextLaneID = ps_next_str(in_);
        let nextLinkFrom = ps_next_str(in_);
        let nextLinkTo = ps_next_str(in_);
        let nextDir: i32 = ps_next(in_);

        s.base.myLane = MSLane::dictionary_2(&laneID);
        if s.base.myLane.is_null() {
            panic!("Unknown lane '{}' when loading walk for person '{}' from state.", laneID, person.get_id());
        }
        let mut nextLane: Ptr<MSLane> = Ptr::null();
        if nextLaneID != "null" {
            nextLane = MSLane::dictionary_2(&nextLaneID);
            if nextLane.is_null() {
                panic!("Unknown next lane '{}' when loading walk for person '{}' from state.", nextLaneID, person.get_id());
            }
        }
        let mut link: Ptr<MSLink> = Ptr::null();
        if nextLinkFrom != "null" {
            let from = MSLane::dictionary_2(&nextLinkFrom);
            let to = MSLane::dictionary_2(&nextLinkTo);
            if from.is_null() {
                panic!("Unknown link origin lane '{}' when loading walk for person '{}' from state.", nextLinkFrom, person.get_id());
            }
            if to.is_null() {
                panic!("Unknown link destination lane '{}' when loading walk for person '{}' from state.", nextLinkTo, person.get_id());
            }
            link = from.get_link_to(to);
        }
        s.myNLI = MSPModel_Striping_NextLaneInfo::new(nextLane, link, nextDir);

        if wapLaneFrom != "null" {
            let from = MSLane::dictionary_2(&wapLaneFrom);
            let to = MSLane::dictionary_2(&wapLaneTo);
            if from.is_null() {
                panic!("Unknown walkingAreaPath origin lane '{}' when loading walk for person '{}' from state.", wapLaneFrom, person.get_id());
            }
            if to.is_null() {
                panic!("Unknown walkingAreaPath destination lane '{}' when loading walk for person '{}' from state.", wapLaneTo, person.get_id());
            }
            match lookup_walking_area_path(from, to) {
                Some(p) => s.myWalkingAreaPath = p,
                None => panic!("Unknown walkingAreaPath from lane '{}' to lane '{}' when loading walk for person '{}' from state.", wapLaneFrom, wapLaneTo, person.get_id()),
            }
        }
        s
    }

    /// C++: `void PState::~PState() noexcept` [virtual]
    // ~PState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ `~PState() {}` has an empty body
        let _ = self;
    }

    /// C++: `Position PState::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual]
    // Position
    // MSPModel_Striping::PState::getPosition(const MSStageMoving& stage, SUMOTime) const {
    //     if (myRemoteXYPos != Position::INVALID) {
    //         return myRemoteXYPos;
    //     }
    //     if (myLane == nullptr) {
    //         // pedestrian has already finished
    //         return Position::INVALID;
    //     }
    //     const double lateral_offset = -getLatOffset();  // the minus is hunting me in my dreams but seems to be here for historical reasons
    //     if (myWalkingAreaPath == nullptr) {
    //         return stage.getLanePosition(myLane, myEdgePos, lateral_offset);
    //     } else {
    //         //if DEBUGCOND(*this) {
    //         //    std::cout << SIMTIME
    //         //        << " getPosition (walkingArea)"
    //         //        << " p=" << myPerson->getID()
    //         //        << " x=" << myEdgePos
    //         //        << " y=" << myPosLat
    //         //        << " latOffset=" << lateral_offset
    //         //        << " shape=" << myWalkingAreaPath->shape
    //         //        << " pos=" << myWalkingAreaPath->shape.positionAtOffset(myEdgePos, lateral_offset)
    //         //        << "\n";
    //         //}
    //         if (myWalkingAreaPath->angleOverride == INVALID_DOUBLE) {
    //             return myWalkingAreaPath->shape.positionAtOffset(myEdgePos, lateral_offset);
    //         } else {
    //             const double rotationOffset = myDir == FORWARD ? 0 : DEG2RAD(180);
    //             return myWalkingAreaPath->shape.sidePositionAtAngle(myEdgePos, lateral_offset, myWalkingAreaPath->angleOverride + rotationOffset);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, stage: &MSStageMoving, now: SUMOTime) -> Position {
        if self.myRemoteXYPos != Position::INVALID {
            return self.myRemoteXYPos;
        }
        if self.myLane.is_null() {
            // pedestrian has already finished
            return Position::INVALID;
        }
        let lateral_offset = -self.get_lat_offset(); // the minus is hunting me in my dreams but seems to be here for historical reasons
        if self.myWalkingAreaPath.is_null() {
            stage.get_lane_position(self.myLane, self.myEdgePos, lateral_offset)
        } else if self.myWalkingAreaPath.angleOverride == utils::common::std_defs::INVALID_DOUBLE {
            self.myWalkingAreaPath.shape.position_at_offset(self.myEdgePos, lateral_offset)
        } else {
            let rotationOffset = if self.myDir == MSPMODEL_FORWARD { 0. } else { utils::geom::geom_helper::deg2rad(180.) };
            self.myWalkingAreaPath.shape.side_position_at_angle(self.myEdgePos, lateral_offset, self.myWalkingAreaPath.angleOverride + rotationOffset)
        }
    }

    /// C++: `double PState::getAngle(const MSStageMoving &, SUMOTime) const` [const, virtual]
    // double
    // MSPModel_Striping::PState::getAngle(const MSStageMoving&, SUMOTime) const {
    //     if (myAngle != std::numeric_limits<double>::max()) {
    //         return myAngle;
    //     }
    //     if (myLane == nullptr) {
    //         // pedestrian has already finished
    //         return 0;
    //     }
    //     if (myWalkingAreaPath != nullptr && myWalkingAreaPath->angleOverride != INVALID_DOUBLE) {
    //         return myWalkingAreaPath->angleOverride;
    //     }
    //     const PositionVector& shp = myWalkingAreaPath == nullptr ? myLane->getShape() : myWalkingAreaPath->shape;
    //     double geomX = myWalkingAreaPath == nullptr ? myLane->interpolateLanePosToGeometryPos(myEdgePos) : myEdgePos;
    //     double angle = shp.rotationAtOffset(geomX) + (myDir == MSPModel::BACKWARD ? M_PI : 0);
    //     if (myDir == MSPModel::BACKWARD) {
    //         angle += atan2(mySpeedLat, MAX2(mySpeed, NUMERICAL_EPS));
    //     } else { // myDir == MSPModel::FORWARD
    //         angle -= atan2(mySpeedLat, MAX2(mySpeed, NUMERICAL_EPS));
    //     }
    //     if (angle > M_PI) {
    //         angle -= 2 * M_PI;
    //     }
    //     myAngle = angle;
    //     return angle;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, stage: &MSStageMoving, now: SUMOTime) -> f64 {
        if self.myAngle.get() != f64::MAX {
            return self.myAngle.get();
        }
        if self.myLane.is_null() {
            // pedestrian has already finished
            return 0.;
        }
        if !self.myWalkingAreaPath.is_null() && self.myWalkingAreaPath.angleOverride != utils::common::std_defs::INVALID_DOUBLE {
            return self.myWalkingAreaPath.angleOverride;
        }
        let shp: &PositionVector = if self.myWalkingAreaPath.is_null() { self.myLane.get_shape_2() } else { &self.myWalkingAreaPath.shape };
        let geomX = if self.myWalkingAreaPath.is_null() { self.myLane.interpolate_lane_pos_to_geometry_pos(self.myEdgePos) } else { self.myEdgePos };
        let mut angle = shp.rotation_at_offset(geomX) + (if self.myDir == MSPMODEL_BACKWARD { std::f64::consts::PI } else { 0. });
        if self.myDir == MSPMODEL_BACKWARD {
            angle += self.mySpeedLat.atan2(f64::max(self.mySpeed, utils::geom::NUMERICAL_EPS));
        } else {
            // myDir == MSPModel::FORWARD
            angle -= self.mySpeedLat.atan2(f64::max(self.mySpeed, utils::geom::NUMERICAL_EPS));
        }
        if angle > std::f64::consts::PI {
            angle -= 2. * std::f64::consts::PI;
        }
        // C++ `mutable` cache
        self.myAngle.set(angle);
        angle
    }

    /// C++: `const MSEdge * PState::getNextEdge(const MSStageMoving &) const` [const, virtual]
    // const MSEdge*
    // MSPModel_Striping::PState::getNextEdge(const MSStageMoving&) const {
    //     return myNLI.lane == nullptr ? nullptr : &myNLI.lane->getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        if self.myNLI.lane.is_null() {
            Ptr::null()
        } else {
            Ptr::from_mut(self.myNLI.lane.get_edge())
        }
    }

    /// C++: `void PState::moveTo(MSPerson *, MSLane *, double, double, SUMOTime)` [virtual]
    // void
    // MSPModel_Striping::PState::moveTo(MSPerson* p, MSLane* lane, double lanePos, double lanePosLat, SUMOTime t) {
    //     ConstMSEdgeVector newEdges; // keep route
    //     int routeOffset = 0;
    //     bool laneOnRoute = false;
    //     const MSJunction* laneOnJunction = lane->isNormal() ? nullptr : lane->getEdge().getToJunction();
    //     for (const MSEdge* edge : myStage->getRoute()) {
    //         if (edge == &lane->getEdge()
    //                 || edge->getToJunction() == laneOnJunction
    //                 || edge->getFromJunction() == laneOnJunction) {
    //             laneOnRoute = true;
    //             break;
    //         }
    //         routeOffset++;
    //     }
    //     if (!laneOnRoute) {
    //         throw ProcessError("Lane '" + lane->getID() + "' is not on the route of person '" + getID() + "'.");
    //     }
    //     Position pos = lane->geometryPositionAtOffset(lanePos, lanePosLat);
    //     if (lane->isWalkingArea() && (myWalkingAreaPath == nullptr || myWalkingAreaPath->lane != lane)) {
    //         // entered new walkingarea. Determine path to guess position
    //         const MSEdge* prevEdge = myStage->getRoute()[routeOffset];
    //         const MSEdge* nextEdge = routeOffset + 1 < (int)myStage->getRoute().size() ? myStage->getRoute()[routeOffset + 1] : nullptr;
    //         const WalkingAreaPath* guessed = guessPath(&lane->getEdge(), prevEdge, nextEdge);
    //         const double maxPos = guessed->shape.length() - NUMERICAL_EPS;
    //         if (lanePos > maxPos + POSITION_EPS || lanePos < -POSITION_EPS) {
    //             throw ProcessError("Lane position " + toString(lanePos) + " cannot be mapped onto walkingarea '" + lane->getID()
    //                                + "' (fromLane='" + guessed->from->getID()
    //                                + "' toLane='" + guessed->to->getID() + "') for person '" + getID() + "' time=" + time2string(t) + ".");
    //         }
    //         // give some slack
    //         lanePos = MIN2(maxPos, MAX2(NUMERICAL_EPS, lanePos));
    //         pos = guessed->shape.positionAtOffset(lanePos, lanePosLat);
    //     }
    //     const double angle = GeomHelper::naviDegree(p->getPosition().angleTo2D(pos));
    //     moveToXY(p, pos, lane, lanePos, lanePosLat, angle, routeOffset, newEdges, t);
    // }
    #[allow(unused_variables)]
    pub fn move_to(&mut self, p: Ptr<MSPerson>, lane: Ptr<MSLane>, lanePos: f64, lanePosLat: f64, t: SUMOTime) {
        let mut newEdges: ConstMSEdgeVector = Vec::new(); // keep route
        let mut routeOffset: i32 = 0;
        let mut laneOnRoute = false;
        let mut lanePos = lanePos;
        let laneEdge = Ptr::from_mut(lane.get_edge());
        let laneOnJunction = if lane.is_normal() { Ptr::null() } else { laneEdge.get_to_junction() };
        let route: Vec<Ptr<MSEdge>> = self.myStage.get_route().clone();
        for edge in route.iter() {
            if *edge == laneEdge || edge.get_to_junction() == laneOnJunction || edge.get_from_junction() == laneOnJunction {
                laneOnRoute = true;
                break;
            }
            routeOffset += 1;
        }
        if !laneOnRoute {
            panic!("{}", format!("Lane '{}' is not on the route of person '{}'.", lane.get_id(), self.get_id()));
        }
        let mut pos = lane.geometry_position_at_offset(lanePos, lanePosLat);
        if lane.is_walking_area() && (self.myWalkingAreaPath.is_null() || self.myWalkingAreaPath.lane != lane) {
            // entered new walkingarea. Determine path to guess position
            let prevEdge = route[routeOffset as usize];
            let nextEdge = if ((routeOffset + 1) as usize) < route.len() { route[(routeOffset + 1) as usize] } else { Ptr::null() };
            let guessed = MSPModel_Striping::guess_path(laneEdge, prevEdge, nextEdge);
            let maxPos = guessed.shape.length() - utils::geom::NUMERICAL_EPS;
            if lanePos > maxPos + utils::geom::POSITION_EPS || lanePos < -utils::geom::POSITION_EPS {
                panic!("{}", format!("Lane position {} cannot be mapped onto walkingarea '{}' (fromLane='{}' toLane='{}') for person '{}' time={}.",
                    utils::common::to_string::f64_to_string_prec(lanePos, 6), lane.get_id(), guessed.from.get_id(), guessed.to.get_id(), self.get_id(),
                    utils::common::time::time2string(t)));
            }
            // give some slack
            lanePos = f64::min(maxPos, f64::max(utils::geom::NUMERICAL_EPS, lanePos));
            pos = guessed.shape.position_at_offset(lanePos, lanePosLat);
        }
        let angle = utils::geom::geom_helper::navi_degree(p.cast::<MSTransportable>().get_position(0.).angle_to2d(&pos));
        self.move_to_xy(p, pos, lane, lanePos, lanePosLat, angle, routeOffset, &newEdges, t);
    }

    /// C++: `void PState::moveToXY(MSPerson *, Position, MSLane *, double, double, double, int, const ConstMSEdgeVector &, SUMOTime)` [virtual]
    // void
    // MSPModel_Striping::PState::moveToXY(MSPerson* p, Position pos, MSLane* lane, double lanePos,
    //                                     double lanePosLat, double angle, int routeOffset,
    //                                     const ConstMSEdgeVector& edges, SUMOTime t) {
    //     MSPModel_Striping* pm = dynamic_cast<MSPModel_Striping*>(MSNet::getInstance()->getPersonControl().getMovementModel());
    //     assert(p == myPerson);
    //     assert(pm != nullptr);
    //     const double oldAngle = GeomHelper::naviDegree(getAngle(*myStage, t));
    //     // person already walking in this step. undo this to obtain the previous position
    //     const double oldX = myEdgePos - SPEED2DIST(mySpeed * myDir);
    //     const double tmp = myEdgePos;
    //     myEdgePos = oldX;
    //     Position oldPos = getPosition(*myStage, t);
    //     myEdgePos = tmp;
    //     //if (oldPos == Position::INVALID) {
    //     //    oldPos = pos
    //     //}
    //     myAngle = GeomHelper::fromNaviDegree(angle);
    // #ifdef DEBUG_MOVETOXY
    //     std::cout << SIMTIME << " ped=" << p->getID()
    //               << " moveToXY"
    //               << " pos=" << pos
    //               << " lane=" << lane->getID()
    //               << " lanePos=" << lanePos
    //               << " lanePosLat=" << lanePosLat
    //               << " angle=" << angle
    //               << " routeOffset=" << routeOffset
    //               << " edges=" << toString(edges)
    //               << " oldLane=" << Named::getIDSecure(myLane)
    //               << " path=" << (myWalkingAreaPath == nullptr ? "null" : (myWalkingAreaPath->from->getID() + "->" + myWalkingAreaPath->to->getID())) << "\n";
    // #endif
    //
    //     if (lane != myLane && myLane != nullptr && lane != nullptr) {
    //         pm->remove(this);
    //         pm->registerActive();
    //     }
    //     if (lane != nullptr &&
    //             fabs(lanePosLat) < (0.5 * (lane->getWidth() + p->getVehicleType().getWidth()) + SIDEWALK_OFFSET)) {
    //         myRemoteXYPos = Position::INVALID;
    //         const MSEdge* old = myStage->getEdge();
    //         const MSLane* oldLane = myLane;
    //         if (lane != myLane) {
    //             // implicitly adds new active lane if necessary
    //             pm->myActiveLanes[lane].push_back(this);
    //         }
    //         if (edges.empty()) {
    //             // map within route
    //             myStage->setRouteIndex(myPerson, routeOffset);
    //         } else {
    //             myStage->replaceRoute(myPerson, edges, routeOffset);
    //         }
    //         if (!lane->getEdge().isNormal()) {
    //             myStage->moveToNextEdge(myPerson, t, myDir, &lane->getEdge());
    //         }
    //
    //         myLane = lane;
    //         const double lateral_offset = (lane->getWidth() - stripeWidth) * 0.5;
    //         if (lane->isWalkingArea()) {
    //             if (myWalkingAreaPath == nullptr || myWalkingAreaPath->lane != lane) {
    //                 // entered new walkingarea. Determine path
    //                 myWalkingAreaPath = guessPath(&lane->getEdge(), old, myStage->getNextRouteEdge());
    // #ifdef DEBUG_MOVETOXY
    //                 std::cout << " guessPath old=" << old->getID() << " next=" << Named::getIDSecure(myStage->getNextRouteEdge())
    //                           << " path=" << myWalkingAreaPath->from->getID() << "->" << myWalkingAreaPath->to->getID() << "\n";
    // #endif
    //             }
    //             // lanePos and lanePosLat are matched onto the circumference of the
    //             // walkingarea. Use pos instead
    //             const Position relPos = myWalkingAreaPath->shape.transformToVectorCoordinates(pos);
    //             if (relPos == Position::INVALID) {
    //                 WRITE_WARNING("Could not map position " + toString(pos) + " onto lane '" + myLane->getID()
    //                               + "' (fromLane='" + myWalkingAreaPath->from->getID()
    //                               + "' toLane='" + myWalkingAreaPath->to->getID() + "') for person '" + getID() + "' time=" + time2string(t) + ".");
    //                 myRemoteXYPos = pos;
    //             } else {
    //                 myEdgePos = relPos.x();
    //                 myPosLat = lateral_offset + relPos.y();
    //             }
    //         } else {
    //             myWalkingAreaPath = nullptr;
    //             myEdgePos = lanePos;
    //             myPosLat = lateral_offset - lanePosLat;
    //             lane->requireCollisionCheck();
    //         }
    //         // guess direction
    //         const double angleDiff = GeomHelper::getMinAngleDiff(angle, oldAngle);
    //         if (myStage->getNextRouteEdge() != nullptr) {
    //             if (myStage->getEdge()->getToJunction() == myStage->getNextRouteEdge()->getFromJunction() ||
    //                     myStage->getEdge()->getToJunction() == myStage->getNextRouteEdge()->getToJunction()) {
    //                 myDir = FORWARD;
    //             } else {
    //                 myDir = BACKWARD;
    //             }
    //         } else {
    //             // guess from angle
    //             if (angleDiff <= 90) {
    //                 // keep direction
    //                 if (myDir == UNDEFINED_DIRECTION) {
    //                     myDir = FORWARD;
    //                 }
    //             } else {
    //                 // change direction
    //                 myDir = (myDir == BACKWARD) ? FORWARD : BACKWARD;
    //             }
    //         }
    //         // update next lane info (after guessing direction)
    //         if (oldLane == nullptr || &oldLane->getEdge() != &myLane->getEdge()) {
    //             const MSLane* sidewalk = getSidewalk<MSEdge, MSLane>(&myLane->getEdge(), p->getVClass());
    //             // assume that we will eventually move back onto the sidewalk if
    //             // there is one
    //             myNLI = getNextLane(*this, sidewalk == nullptr ? myLane : sidewalk, nullptr);
    //             myStage->activateEntryReminders(myPerson);
    // #ifdef DEBUG_MOVETOXY
    //             std::cout << " myNLI=" << Named::getIDSecure(myNLI.lane) << " link=" << (myNLI.link == nullptr ? "NULL" : myNLI.link->getDescription()) << " dir=" << myNLI.dir << "\n";
    // #endif
    //         }
    // #ifdef DEBUG_MOVETOXY
    //         std::cout << " newRelPos=" << Position(myEdgePos, myPosLat) << " edge=" << myPerson->getEdge()->getID()
    //                   << " newPos=" << myPerson->getPosition()
    //                   << " latOffset=" << getLatOffset()
    //                   << " oldAngle=" << oldAngle << " angleDiff=" << angleDiff << " newDir=" << myDir << "\n";
    // #endif
    //         if (oldLane == myLane) {
    //             mySpeed = DIST2SPEED(fabs(oldX - myEdgePos));
    //         } else {
    //             //std::cout << SIMTIME << " oldX=" << oldX << " oldSpeed=" << mySpeed << " oldPos=" << oldPos << " pos=" << pos << " dist=" << oldPos.distanceTo2D(pos) << " oldLane=" << Named::getIDSecure(oldLane) << " lane=" << lane->getID() << "\n";
    //             mySpeed = DIST2SPEED(oldPos.distanceTo2D(pos));
    //         }
    //     } else {
    //         // map outside the network
    //         myRemoteXYPos = pos;
    //         mySpeed = DIST2SPEED(oldPos.distanceTo2D(pos));
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn move_to_xy(&mut self, p: Ptr<MSPerson>, pos: Position, lane: Ptr<MSLane>, lanePos: f64, lanePosLat: f64, angle: f64, routeOffset: i32, edges: &ConstMSEdgeVector, t: SUMOTime) {
        let pm: Ptr<MSPModel_Striping> = MSNet::get_instance().m().get_person_control().get_movement_model().cast::<MSPModel_Striping>();
        debug_assert!(p == self.myPerson);
        let oldAngle = utils::geom::geom_helper::navi_degree(self.get_angle(&*self.myStage, t));
        let oldX = self.myEdgePos - utils::common::time::speed2dist(self.mySpeed * self.myDir as f64);
        let tmp = self.myEdgePos;
        self.myEdgePos = oldX;
        let oldPos = self.get_position(&*self.myStage, t);
        self.myEdgePos = tmp;
        self.myAngle.set(utils::geom::geom_helper::from_navi_degree(angle));
        if lane != self.myLane && !self.myLane.is_null() && !lane.is_null() {
            pm.m().remove(Ptr::from_mut(self).cast::<MSTransportableStateAdapter>());
            pm.m().register_active();
        }
        if !lane.is_null() && lanePosLat.abs() < (0.5 * (lane.get_width() + p.get_vehicle_type().get_width()) + SIDEWALK_OFFSET) {
            self.myRemoteXYPos = Position::INVALID;
            let old = self.myStage.get_edge();
            let oldLane = self.myLane;
            if lane != self.myLane {
                pm.m().myActiveLanes.entry(MSPModel_Interacting_LaneKey(lane)).or_default().push(Ptr::from_mut(self).cast::<MSPModel_InteractingState>());
            }
            if edges.is_empty() {
                self.myStage.m().set_route_index(p.cast::<MSTransportable>(), routeOffset);
            } else {
                self.myStage.m().replace_route(p.cast::<MSTransportable>(), edges, routeOffset);
            }
            if !lane.get_edge().is_normal() {
                self.myStage.m().move_to_next_edge(p.cast::<MSTransportable>(), t, self.myDir, Ptr::from_mut(lane.m().get_edge()), false);
            }
            self.myLane = lane;
            let lateral_offset = (lane.get_width() - stripe_width()) * 0.5;
            if lane.is_walking_area() {
                if self.myWalkingAreaPath.is_null() || self.myWalkingAreaPath.lane != lane {
                    self.myWalkingAreaPath = MSPModel_Striping::guess_path(Ptr::from_mut(lane.m().get_edge()), old, self.myStage.get_next_route_edge());
                }
                let relPos = self.myWalkingAreaPath.shape.transform_to_vector_coordinates(&pos, false);
                if relPos == Position::INVALID {
                    utils::common::msg_handler::write_warning(&format!("Could not map position {} onto lane '{}' (fromLane='{}' toLane='{}') for person '{}' time={}.",
                        pos, self.myLane.get_id(), self.myWalkingAreaPath.from.get_id(), self.myWalkingAreaPath.to.get_id(), self.get_id(),
                        utils::common::time::time2string(t)));
                    self.myRemoteXYPos = pos;
                } else {
                    self.myEdgePos = relPos.x();
                    self.myPosLat = lateral_offset + relPos.y();
                }
            } else {
                self.myWalkingAreaPath = Ptr::null();
                self.myEdgePos = lanePos;
                self.myPosLat = lateral_offset - lanePosLat;
                lane.m().require_collision_check();
            }
            let angleDiff = utils::geom::geom_helper::get_min_angle_diff(angle, oldAngle);
            let nre = self.myStage.get_next_route_edge();
            if !nre.is_null() {
                if self.myStage.get_edge().get_to_junction() == nre.get_from_junction() || self.myStage.get_edge().get_to_junction() == nre.get_to_junction() {
                    self.myDir = MSPMODEL_FORWARD;
                } else {
                    self.myDir = MSPMODEL_BACKWARD;
                }
            } else {
                // guess from angle
                if angleDiff <= 90. {
                    // keep direction
                    if self.myDir == MSPMODEL_UNDEFINED_DIRECTION {
                        self.myDir = MSPMODEL_FORWARD;
                    }
                } else {
                    // change direction
                    self.myDir = if self.myDir == MSPMODEL_BACKWARD { MSPMODEL_FORWARD } else { MSPMODEL_BACKWARD };
                }
            }
            // update next lane info (after guessing direction)
            if oldLane.is_null() || Ptr::from_mut(oldLane.m().get_edge()) != Ptr::from_mut(self.myLane.m().get_edge()) {
                let sidewalk = get_sidewalk(Ptr::from_mut(self.myLane.m().get_edge()), p.get_v_class());
                // assume that we will eventually move back onto the sidewalk if there is one
                let nli = MSPModel_Striping::get_next_lane(&*self, if sidewalk.is_null() { self.myLane } else { sidewalk }, Ptr::null());
                self.myNLI = nli;
                self.myStage.m().activate_entry_reminders(p.cast::<MSTransportable>(), false);
            }
            if oldLane == self.myLane {
                self.mySpeed = utils::common::time::dist2speed((oldX - self.myEdgePos).abs());
            } else {
                self.mySpeed = utils::common::time::dist2speed(oldPos.distance_to2d(&pos));
            }
        } else {
            self.myRemoteXYPos = pos;
            self.mySpeed = utils::common::time::dist2speed(oldPos.distance_to2d(&pos));
        }
    }

    /// C++: `double PState::getMinX(const bool) const` [const, virtual]
    /// return the minimum position on the lane
    // double
    // MSPModel_Striping::PState::getMinX(const bool includeMinGap) const {
    //     // @todo speed should have an influence here because faster persons need more space
    //     if (myDir == FORWARD) {
    //         return myEdgePos - getLength();
    //     }
    //     return myEdgePos - (includeMinGap ? getMinGap() : 0.);
    // }
    pub fn get_min_x(&self, includeMinGap: bool) -> f64 {
        (self.base.base.vt.get_min_x)(&self.base.base, includeMinGap)
    }
    #[allow(unused_variables)]
    pub fn get_min_x_base(&self, includeMinGap: bool) -> f64 {
        if self.myDir == MSPMODEL_FORWARD {
            return self.myEdgePos - self.get_length();
        }
        self.myEdgePos - (if includeMinGap { self.get_min_gap() } else { 0. })
    }

    /// C++: `double PState::getMaxX(const bool) const` [const, virtual]
    /// return the maximum position on the lane
    // double
    // MSPModel_Striping::PState::getMaxX(const bool includeMinGap) const {
    //     // @todo speed should have an influence here because faster persons need more space
    //     if (myDir == FORWARD) {
    //         return myEdgePos + (includeMinGap ? getMinGap() : 0.);
    //     }
    //     return myEdgePos + getLength();
    // }
    pub fn get_max_x(&self, includeMinGap: bool) -> f64 {
        (self.base.base.vt.get_max_x)(&self.base.base, includeMinGap)
    }
    #[allow(unused_variables)]
    pub fn get_max_x_base(&self, includeMinGap: bool) -> f64 {
        if self.myDir == MSPMODEL_FORWARD {
            return self.myEdgePos + (if includeMinGap { self.get_min_gap() } else { 0. });
        }
        self.myEdgePos + self.get_length()
    }

    /// C++: `double PState::getLength() const` [const]
    /// return the length of the pedestrian
    // double
    // MSPModel_Striping::PState::getLength() const {
    //     return myPerson->getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myPerson.get_vehicle_type().get_length()
    }

    /// C++: `double PState::getMinGap() const` [const]
    /// return the minimum gap of the pedestrian
    // double
    // MSPModel_Striping::PState::getMinGap() const {
    //     return myPerson->getVehicleType().getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(&self) -> f64 {
        self.myPerson.get_vehicle_type().get_min_gap()
    }

    /// C++: `double PState::distToLaneEnd() const` [const]
    /// the absolute distance to the end of the lane in walking direction (or to the arrivalPos)
    // double
    // MSPModel_Striping::PState::distToLaneEnd() const {
    //     if (myStage->getNextRouteEdge() == nullptr) {
    //         return myDir * (myStage->getArrivalPos() - myEdgePos) - POSITION_EPS - (
    //                    (myWaitingTime > DELTA_T && (myStage->getDestinationStop() == nullptr ||
    //                                                 myStage->getDestinationStop()->getWaitingCapacity() > myStage->getDestinationStop()->getNumWaitingPersons()))
    //                    ? getMinGap() : 0);
    //     } else {
    //         const double length = myWalkingAreaPath == nullptr ? myLane->getLength() : myWalkingAreaPath->length;
    //         return myDir == FORWARD ? length - myEdgePos : myEdgePos;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dist_to_lane_end(&self) -> f64 {
        if self.myStage.get_next_route_edge().is_null() {
            self.myDir as f64 * (self.myStage.get_arrival_pos() - self.myEdgePos) - utils::geom::POSITION_EPS - (
                if self.myWaitingTime > utils::common::time::delta_t()
                    && (self.myStage.get_destination_stop().is_null()
                        || self.myStage.get_destination_stop().get_waiting_capacity() > self.myStage.get_destination_stop().get_num_waiting_persons())
                {
                    self.get_min_gap()
                } else {
                    0.
                })
        } else {
            let length = if self.myWalkingAreaPath.is_null() { self.myLane.get_length() } else { self.myWalkingAreaPath.length };
            if self.myDir == MSPMODEL_FORWARD { length - self.myEdgePos } else { self.myEdgePos }
        }
    }

    /// C++: `bool PState::moveToNextLane(SUMOTime)`
    /// return whether this pedestrian has passed the end of the current lane and update myRelX if so
    // bool
    // MSPModel_Striping::PState::moveToNextLane(SUMOTime currentTime) {
    //     double dist = distToLaneEnd();
    //     if (DEBUGCOND(*this)) {
    //         std::cout << SIMTIME << " ped=" << myPerson->getID() << " myEdgePos=" << myEdgePos << " dist=" << dist << "\n";
    //     }
    //     if (dist <= 0) {
    //         //if (ped.getPerson()->getID() == DEBUG1) {
    //         //    std::cout << SIMTIME << " addToLane x=" << ped.myEdgePos << " newDir=" << newDir << " newLane=" << newLane->getID() << " walkingAreaShape=" << walkingAreaShape << "\n";
    //         //}
    //         //std::cout << " changing to " << newLane->getID() << " myPosLat=" << ped.myPosLat << " oldStripes=" << numStripes(myLane) << " newStripes=" << numStripes(newLane);
    //         //std::cout << " newY=" << ped.myPosLat << " myDir=" << ped.getDirection() << " newDir=" << newDir;
    //         const int oldDir = myDir;
    //         const MSLane* oldLane = myLane;
    //         myLane = myNLI.lane;
    //         myDir = myNLI.dir;
    //         const bool normalLane = (myLane == nullptr || myLane->getEdge().getFunction() == SumoXMLEdgeFunc::NORMAL || &myLane->getEdge() == myStage->getNextRouteEdge());
    //         if DEBUGCOND(*this) {
    //             std::cout << SIMTIME
    //                       << " ped=" << myPerson->getID()
    //                       << " moveToNextLane old=" << oldLane->getID()
    //                       << " new=" << (myLane == nullptr ? "NULL" : myLane->getID())
    //                       << " oldDir=" << oldDir
    //                       << " newDir=" << myDir
    //                       << " myEdgePos=" << myEdgePos
    //                       << " dist=" << dist
    //                       << "\n";
    //         }
    //         if (myLane == nullptr) {
    //             myEdgePos = myStage->getArrivalPos();
    //         }
    //         // moveToNextEdge might destroy the person and thus mess up the heap. Better check first
    //         if (myStage->getRouteStep() == myStage->getRoute().end() - 1) {
    //             myLane = nullptr;
    //         } else {
    //             const bool arrived = myStage->moveToNextEdge(myPerson, currentTime, oldDir, normalLane ? nullptr : &myLane->getEdge());
    //             UNUSED_PARAMETER(arrived);
    //             assert(!arrived);
    //             assert(myDir != UNDEFINED_DIRECTION);
    //             myNLI = getNextLane(*this, myLane, oldLane);
    //             assert(myNLI.lane != oldLane); // do not turn around
    //             if DEBUGCOND(*this) {
    //                 std::cout << "    nextLane=" << (myNLI.lane == nullptr ? "NULL" : myNLI.lane->getID()) << "\n";
    //             }
    //             if (myLane->isWalkingArea()) {
    //                 if (myNLI.dir != UNDEFINED_DIRECTION) {
    //                     myWalkingAreaPath = getWalkingAreaPath(&myLane->getEdge(), oldLane, myNLI.lane);
    //                     assert(myWalkingAreaPath->shape.size() >= 2);
    //                     if DEBUGCOND(*this) {
    //                         std::cout << "  mWAPath shape=" << myWalkingAreaPath->shape << " length=" << myWalkingAreaPath->length << "\n";
    //                     }
    //                 } else if (myNLI.link != nullptr) {
    //                     // using direct connection (instead of using walkingarea)
    //                     myLane = myNLI.lane;
    //                     assert(!myLane->isWalkingArea());
    //                     myStage->moveToNextEdge(myPerson, currentTime, myDir, &myLane->getEdge());
    //                     myWalkingAreaPath = nullptr;
    //                     myNLI = getNextLane(*this, myLane, oldLane);
    //                 } else {
    //                     // disconnected route. move to the next edge
    //                     if (OptionsCont::getOptions().getBool("ignore-route-errors")) {
    //                         // try to determine direction from topology, otherwise maintain current direction
    //                         const MSEdge* currRouteEdge = *myStage->getRouteStep();
    //                         const MSEdge* nextRouteEdge = myStage->getNextRouteEdge();
    //                         if ((nextRouteEdge->getToJunction() == currRouteEdge->getFromJunction())
    //                                 || nextRouteEdge->getToJunction() == currRouteEdge->getToJunction()) {
    //                             myDir = BACKWARD;
    //                         } else if ((nextRouteEdge->getFromJunction() == currRouteEdge->getFromJunction())
    //                                    || nextRouteEdge->getFromJunction() == currRouteEdge->getToJunction()) {
    //                             myDir = FORWARD;
    //                         }
    //                         myStage->moveToNextEdge(myPerson, currentTime, oldDir, nullptr);
    //                         myLane = myNLI.lane;
    //                         assert(myLane != 0);
    //                         assert(myLane->getEdge().getFunction() == SumoXMLEdgeFunc::NORMAL);
    //                         myNLI = getNextLane(*this, myLane, oldLane);
    //                         myWalkingAreaPath = nullptr;
    //                     } else {
    //                         throw ProcessError(TLF("Disconnected walk for person '%'.", myPerson->getID()));
    //                     }
    //                 }
    //             } else {
    //                 myWalkingAreaPath = nullptr;
    //             }
    //             // adapt x to fit onto the new lane
    //             // (make sure we do not move past the end of the new lane since that
    //             // lane was not checked for obstacles)
    //             const double newLength = (myWalkingAreaPath == nullptr ? myLane->getLength() : myWalkingAreaPath->length);
    //             if (-dist > newLength) {
    //                 assert(OptionsCont::getOptions().getBool("ignore-route-errors"));
    //                 // should not happen because the end of myLane should have been an obstacle as well
    //                 // (only when the route is broken)
    //                 dist = -newLength;
    //             }
    //             if (myDir == BACKWARD) {
    //                 myEdgePos = newLength + dist;
    //             } else {
    //                 myEdgePos = -dist;
    //             }
    //             if DEBUGCOND(*this) {
    //                 std::cout << SIMTIME << " update myEdgePos ped=" << myPerson->getID()
    //                           << " newLength=" << newLength
    //                           << " dist=" << dist
    //                           << " myEdgePos=" << myEdgePos
    //                           << "\n";
    //             }
    //             // adjust to change in direction
    //             if (myDir != oldDir) {
    //                 myPosLat = (numStripes(oldLane) - 1) * stripeWidth - myPosLat;
    //             }
    //             // adjust to differences in sidewalk width
    //             const int offset = getStripeOffset(numStripes(oldLane), numStripes(myLane), oldDir != myDir && numStripes(myLane) < numStripes(oldLane));
    //             myPosLat += offset * stripeWidth;
    //             if DEBUGCOND(*this) {
    //                 std::cout << SIMTIME << " transformY ped=" << myPerson->getID()
    //                           << " newLane=" << Named::getIDSecure(myLane)
    //                           << " newY=" << myPosLat
    //                           << " os=" << numStripes(oldLane) << " ns=" << numStripes(myLane)
    //                           << " od=" << oldDir << " nd=" << myDir
    //                           << " offset=" << offset << "\n";
    //             }
    //         }
    //         myAngle = std::numeric_limits<double>::max(); // see #9014
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to_next_lane(&mut self, currentTime: SUMOTime) -> bool {
        let mut dist = self.dist_to_lane_end();
        let person = self.myPerson.cast::<MSTransportable>();
        if dist <= 0. {
            let oldDir = self.myDir;
            let oldLane = self.myLane;
            self.myLane = self.myNLI.lane;
            self.myDir = self.myNLI.dir;
            let normalLane = self.myLane.is_null()
                || self.myLane.get_edge().get_function() == SumoXMLEdgeFunc::NORMAL
                || Ptr::from_mut(self.myLane.get_edge()) == self.myStage.get_next_route_edge();
            if self.myLane.is_null() {
                self.myEdgePos = self.myStage.get_arrival_pos();
            }
            // moveToNextEdge might destroy the person and thus mess up the heap. Better check first
            if self.myStage.get_route_step() == self.myStage.get_route().len() - 1 {
                self.myLane = Ptr::null();
            } else {
                let nextInternal = if normalLane { Ptr::null() } else { Ptr::from_mut(self.myLane.get_edge()) };
                let arrived = self.myStage.m().move_to_next_edge(person, currentTime, oldDir, nextInternal, false);
                debug_assert!(!arrived);
                debug_assert!(self.myDir != MSPMODEL_UNDEFINED_DIRECTION);
                let nli = MSPModel_Striping::get_next_lane(&*self, self.myLane, oldLane);
                self.myNLI = nli;
                debug_assert!(self.myNLI.lane != oldLane); // do not turn around
                if self.myLane.is_walking_area() {
                    if self.myNLI.dir != MSPMODEL_UNDEFINED_DIRECTION {
                        self.myWalkingAreaPath = MSPModel_Striping::get_walking_area_path(Ptr::from_mut(self.myLane.get_edge()), oldLane, self.myNLI.lane);
                        debug_assert!(self.myWalkingAreaPath.shape.len() >= 2);
                    } else if !self.myNLI.link.is_null() {
                        // using direct connection (instead of using walkingarea)
                        self.myLane = self.myNLI.lane;
                        debug_assert!(!self.myLane.is_walking_area());
                        let nextInternal = Ptr::from_mut(self.myLane.get_edge());
                        self.myStage.m().move_to_next_edge(person, currentTime, self.myDir, nextInternal, false);
                        self.myWalkingAreaPath = Ptr::null();
                        let nli = MSPModel_Striping::get_next_lane(&*self, self.myLane, oldLane);
                        self.myNLI = nli;
                    } else {
                        // disconnected route. move to the next edge
                        if utils::options::options_cont::get_bool("ignore-route-errors") {
                            // try to determine direction from topology, otherwise maintain current direction
                            let currRouteEdge = self.myStage.get_route()[self.myStage.get_route_step()];
                            let nextRouteEdge = self.myStage.get_next_route_edge();
                            if nextRouteEdge.get_to_junction() == currRouteEdge.get_from_junction() || nextRouteEdge.get_to_junction() == currRouteEdge.get_to_junction() {
                                self.myDir = MSPMODEL_BACKWARD;
                            } else if nextRouteEdge.get_from_junction() == currRouteEdge.get_from_junction() || nextRouteEdge.get_from_junction() == currRouteEdge.get_to_junction() {
                                self.myDir = MSPMODEL_FORWARD;
                            }
                            self.myStage.m().move_to_next_edge(person, currentTime, oldDir, Ptr::null(), false);
                            self.myLane = self.myNLI.lane;
                            debug_assert!(!self.myLane.is_null());
                            debug_assert!(self.myLane.get_edge().get_function() == SumoXMLEdgeFunc::NORMAL);
                            let nli = MSPModel_Striping::get_next_lane(&*self, self.myLane, oldLane);
                            self.myNLI = nli;
                            self.myWalkingAreaPath = Ptr::null();
                        } else {
                            panic!("{}", utils::tlf!("Disconnected walk for person '%'.", self.myPerson.get_id()));
                        }
                    }
                } else {
                    self.myWalkingAreaPath = Ptr::null();
                }
                // adapt x to fit onto the new lane
                // (make sure we do not move past the end of the new lane since that
                // lane was not checked for obstacles)
                let newLength = if self.myWalkingAreaPath.is_null() { self.myLane.get_length() } else { self.myWalkingAreaPath.length };
                if -dist > newLength {
                    // should not happen because the end of myLane should have been an obstacle as well
                    // (only when the route is broken)
                    dist = -newLength;
                }
                if self.myDir == MSPMODEL_BACKWARD {
                    self.myEdgePos = newLength + dist;
                } else {
                    self.myEdgePos = -dist;
                }
                // adjust to change in direction
                if self.myDir != oldDir {
                    self.myPosLat = (MSPModel_Striping::num_stripes(oldLane) - 1) as f64 * stripe_width() - self.myPosLat;
                }
                // adjust to differences in sidewalk width
                let offset = MSPModel_Striping::get_stripe_offset(MSPModel_Striping::num_stripes(oldLane), MSPModel_Striping::num_stripes(self.myLane),
                    oldDir != self.myDir && MSPModel_Striping::num_stripes(self.myLane) < MSPModel_Striping::num_stripes(oldLane));
                self.myPosLat += offset as f64 * stripe_width();
            }
            self.myAngle.set(f64::MAX); // see #9014
            true
        } else {
            false
        }
    }

    /// C++: `void PState::walk(const Obstacles &)`
    /// perform position update
    // void
    // MSPModel_Striping::PState::walk(const Obstacles& obs) {
    //     const int stripes = (int)obs.size();
    //     const int sMax =  stripes - 1;
    //     assert(stripes == numStripes(myLane));
    //     // account stage-specific max speed but also for normal lane speed limit
    //     // (speed limits on crossings and walkingareas ignored due to #11527)
    //     const double vMax = (myStage->getConfiguredSpeed() >= 0
    //                          ? myStage->getConfiguredSpeed()
    //                          : (USE_NET_SPEEDS || myLane->isNormal() || myLane->isInternal()
    //                             ? myLane->getVehicleMaxSpeed(myPerson)
    //                             : myStage->getMaxSpeed(myPerson)));
    //     // ultimate goal is to choose the preferred stripe (chosen)
    //     const int current = stripe();
    //     const int other = otherStripe();
    //     // compute distances
    //     std::vector<double> distance(stripes);
    //     for (int i = 0; i < stripes; ++i) {
    //         distance[i] = distanceTo(obs[i], obs[i].type == OBSTACLE_PED);
    //     }
    //     // compute utility for all stripes
    //     std::vector<double> utility(stripes, 0);
    //     // forbid stripes which are blocked and also all stripes behind them
    //     for (int i = 0; i < stripes; ++i) {
    //         if (distance[i] == DIST_OVERLAP) {
    //             if (i == current && (!isWaitingToEnter() || stripe() != stripe(myPosLat))) {
    //                 utility[i] += OBSTRUCTED_PENALTY;
    //             }
    //             if (i < current) {
    //                 for (int j = 0; j <= i; ++j) {
    //                     utility[j] += OBSTRUCTED_PENALTY;
    //                 }
    //             }
    //             if (i > current) {
    //                 for (int j = i; j < stripes; ++j) {
    //                     utility[j] += OBSTRUCTED_PENALTY;
    //                 }
    //             }
    //         }
    //     }
    //     // forbid a portion of the leftmost stripes (in walking direction).
    //     // lanes with stripes less than 1 / RESERVE_FOR_ONCOMING_FACTOR
    //     // may still deadlock in heavy pedestrian traffic
    //     const bool onJunction = myLane->isWalkingArea() || myLane->isCrossing();
    //     const int reserved = getReserved(stripes, (onJunction ? RESERVE_FOR_ONCOMING_FACTOR_JUNCTIONS : RESERVE_FOR_ONCOMING_FACTOR));
    //     if (myDir == FORWARD) {
    //         for (int i = 0; i < reserved; ++i) {
    //             utility[i] += INAPPROPRIATE_PENALTY * (i == current ? 0.5 : 1);
    //         }
    //     } else {
    //         for (int i = sMax; i > sMax - reserved; --i) {
    //             utility[i] += INAPPROPRIATE_PENALTY * (i == current ? 0.5 : 1);
    //         }
    //     }
    //     // adapt utility based on obstacles
    //     for (int i = 0; i < stripes; ++i) {
    //         if (obs[i].speed * myDir < 0) {
    //             // penalize evasion to the left unless the obstacle is a vehicle
    //             if ((myDir == FORWARD || obs[i].type == OBSTACLE_VEHICLE) && i > 0) {
    //                 utility[i - 1] -= 0.5;
    //             } else if (myDir == BACKWARD && i < sMax) {
    //                 utility[i + 1] -= 0.5;
    //             }
    //         }
    //         // compute expected distance achievable by staying on this stripe for a time horizon
    //         const double walkDist = MAX2(0., distance[i]); // disregard special distance flags
    //         const double lookAhead = obs[i].speed * myDir >= 0 ? LOOKAHEAD_SAMEDIR : LOOKAHEAD_ONCOMING;
    //         const double expectedDist = MIN2(vMax * LOOKAHEAD_SAMEDIR, walkDist + obs[i].speed * myDir * lookAhead);
    //         if (expectedDist >= 0) {
    //             utility[i] += expectedDist;
    //         } else {
    //             // let only the distance count
    //             utility[i] += ONCOMING_CONFLICT_PENALTY + distance[i];
    //         }
    //     }
    //     // discourage use of the leftmost stripe (in walking direction) if there are oncoming
    //     if (myDir == FORWARD && obs[0].speed < 0) {
    //         utility[0] += ONCOMING_CONFLICT_PENALTY;
    //     } else if (myDir == BACKWARD && obs[sMax].speed > 0) {
    //         utility[sMax] += ONCOMING_CONFLICT_PENALTY;
    //     }
    //     // penalize lateral movement (if the current stripe permits walking)
    //     if (distance[current] > 0 && myWaitingTime == 0) {
    //         for (int i = 0; i < stripes; ++i) {
    //             utility[i] += abs(i - current) * LATERAL_PENALTY;
    //         }
    //     }
    //     // walk on the right side on shared space
    //     if (myLane->getPermissions() != SVC_PEDESTRIAN && myDir == BACKWARD) {
    //         for (int i = 0; i < stripes; ++i) {
    //             if (i <= current) {
    //                 utility[i] += (sMax - i + 1) * LATERAL_PENALTY;
    //             }
    //         }
    //     }
    //
    //     // select best stripe
    //     int chosen = current;
    //     for (int i = 0; i < stripes; ++i) {
    //         if (utility[i] > utility[chosen] && utility[i] >= INAPPROPRIATE_PENALTY * 0.5) {
    //             chosen = i;
    //         }
    //     }
    //     // compute speed components along both axes
    //     const int next = (chosen == current ? current : (chosen < current ? current - 1 : current + 1));
    //     double xDist = MIN3(distance[current], distance[other], distance[next]);
    //     if (next != chosen) {
    //         // ensure that we do not collide with an obstacle in the stripe beyond
    //         // next as this might become the 'other' stripe in the next step
    //         const int nextOther = chosen < current ? current - 2 : current + 2;
    //         xDist = MIN2(xDist, distance[nextOther]);
    //     }
    //     // XXX preferred gap differs between approaching a standing obstacle or a moving obstacle
    //     const double preferredGap = NUMERICAL_EPS;
    //     double xSpeed = MIN2(vMax, MAX2(0., DIST2SPEED(xDist - preferredGap)));
    //     if (xSpeed < NUMERICAL_EPS) {
    //         xSpeed = 0.;
    //     }
    //     if (DEBUGCOND(*this)) {
    //         std::cout << " xSpeedPotential=" << xSpeed << "\n";
    //     }
    //     // avoid tiny steps
    //     // XXX pressure from behind?
    //     if (mySpeed == 0 && xDist < MIN_STARTUP_DIST &&
    //             // unless walking towards a short lane
    //             !(
    //                 (xDist == distance[current] && obs[current].type >= OBSTACLE_END)
    //                 || (xDist == distance[other] && obs[other].type >= OBSTACLE_END)
    //                 || (xDist == distance[next] && obs[next].type >= OBSTACLE_END))
    //        ) {
    //         xSpeed = 0;
    //     }
    //     if (xSpeed == 0) {
    //         if (DEBUGCOND(*this)) {
    //             std::cout << " sharedWA=" << (myWalkingAreaFoes.find(&myLane->getEdge()) != myWalkingAreaFoes.end())
    //                       << " vehObs=" << Named::getIDSecure(obs[current].vehicle)
    //                       << " vehWait=" << STEPS2TIME(obs[current].vehicle ? obs[current].vehicle->getWaitingTime() : 0)
    //                       << "\n";
    //         }
    //         if (myWaitingTime > ((myLane->isCrossing()
    //                               // treat shared walkingarea like a crossing to avoid deadlocking vehicles
    //                               || (myLane->isWalkingArea() && obs[current].vehicle != nullptr && obs[current].vehicle->getWaitingTime() > jamTimeCrossing
    //                                   && myWalkingAreaFoes.find(&myLane->getEdge()) != myWalkingAreaFoes.end())) ? jamTimeCrossing : jamTime)
    //                 || (sMax == 0 && obs[0].speed * myDir < 0 && myWaitingTime > jamTimeNarrow)
    //                 || myAmJammed) {
    //             // squeeze slowly through the crowd ignoring others
    //             if (!myAmJammed) {
    //                 MSNet::getInstance()->getPersonControl().registerJammed();
    //                 WRITE_WARNINGF(TL("Person '%' is jammed on edge '%', time=%."),
    //                                myPerson->getID(), myStage->getEdge()->getID(), time2string(SIMSTEP));
    //                 myAmJammed = true;
    //             }
    //             xSpeed = vMax * jamFactor;
    //         }
    //     } else if (myAmJammed && stripe(myPosLat) >= 0 && stripe(myPosLat) <= sMax && xDist >= MIN_STARTUP_DIST)  {
    //         myAmJammed = false;
    //     }
    //     // dawdling
    //     const double dawdle = MIN2(xSpeed, RandHelper::rand() * vMax * dawdling);
    //     xSpeed -= dawdle;
    //
    //     // XXX ensure that diagonal speed <= vMax
    //     // avoid deadlocks on narrow sidewalks
    //     //if (oncoming && xSpeed == 0 && myStage->getWaitingTime(currentTime) > TIME2STEPS(ONCOMIN_PATIENCE)) {
    //     //    if DEBUGCOND(*this) std::cout << "  stepping aside to resolve oncoming deadlock\n";
    //     //    xSpeed = POSITION_EPS; // reset myWaitingTime
    //     //     if (myDir == FORWARD && chosen < sMax) {
    //     //         chosen += 1;
    //     //     } else if (myDir == BACKWARD && chosen > 0) {
    //     //         chosen -= 1;
    //     //     }
    //     //}
    //     const double maxYSpeed = MIN2(MAX2(vMax * LATERAL_SPEED_FACTOR, vMax - xSpeed), stripeWidth);
    //     double ySpeed = 0;
    //     double yDist = 0;
    //     if (utility[next] > OBSTRUCTION_THRESHOLD && utility[chosen] > OBSTRUCTION_THRESHOLD) {
    //         // don't move laterally if the stripes are blocked
    //         yDist = (chosen * stripeWidth) - myPosLat;
    //         if (fabs(yDist) > NUMERICAL_EPS) {
    //             ySpeed = (yDist > 0 ?
    //                       MIN2(maxYSpeed, DIST2SPEED(yDist)) :
    //                       MAX2(-maxYSpeed, DIST2SPEED(yDist)));
    //         }
    //     } else if (utility[next] <= OBSTRUCTION_THRESHOLD && obs[next].type == OBSTACLE_VEHICLE
    //                // still on the road
    //                && stripe() == stripe(myPosLat)
    //                // only when the vehicle is moving on the same lane
    //                && !(myLane->isCrossing() || myLane->isWalkingArea())) {
    //         // step aside to let the vehicle pass
    //         int stepAsideDir = myDir;
    //         if (myLane->getEdge().getLanes().size() > 1 || current > sMax / 2) {
    //             // always step to the right on multi-lane edges or when closer to
    //             // the right side
    //             stepAsideDir = FORWARD;
    //         }
    //         myAmJammed = true; // ignore pedestrian-pedestrian collisions
    //         ySpeed = stepAsideDir * vMax;
    //     }
    //
    //     // DEBUG
    //     if DEBUGCOND(*this) {
    //         std::cout << SIMTIME
    //                   << " ped=" << myPerson->getID()
    //                   << " edge=" << myStage->getEdge()->getID()
    //                   << " x=" << myEdgePos
    //                   << " y=" << myPosLat
    //                   << " d=" << myDir
    //                   << " pvx=" << mySpeed
    //                   << " cur=" << current
    //                   << " cho=" << chosen
    //                   << " oth=" << other
    //                   << " nxt=" << next
    //                   << " vx=" << xSpeed
    //                   << " dawdle=" << dawdle
    //                   << " vy=" << ySpeed
    //                   << " xd=" << xDist
    //                   << " yd=" << yDist
    //                   << " vMax=" << vMax
    //                   << " wTime=" << myStage->getWaitingTime()
    //                   << " jammed=" << myAmJammed
    //                   << "\n";
    //         if (DEBUGCOND(*this)) {
    //             for (int i = 0; i < stripes; ++i) {
    //                 const Obstacle& o = obs[i];
    //                 std::cout << " util=" << utility[i] << " dist=" << distance[i] << " o=" << o.description;
    //                 if (o.description != "") {
    //                     std::cout << " xF=" << o.xFwd << " xB=" << o.xBack << " v=" << o.speed;
    //                 }
    //                 if (i == current) {
    //                     std::cout << " current";
    //                 }
    //                 if (i == other && i != current) {
    //                     std::cout << " other";
    //                 }
    //                 if (i == chosen) {
    //                     std::cout << " chosen";
    //                 }
    //                 if (i == next) {
    //                     std::cout << " next";
    //                 }
    //                 std::cout << "\n";
    //             }
    //         }
    //     }
    //     myEdgePos += SPEED2DIST(xSpeed * myDir);
    //     myPosLat += SPEED2DIST(ySpeed);
    //     mySpeedLat = ySpeed;
    //     mySpeed = xSpeed;
    //     if (xSpeed >= SUMO_const_haltingSpeed) {
    //         myWaitingToEnter = false;
    //         myWaitingTime = 0;
    //     } else {
    //         myWaitingTime += DELTA_T;
    //         myTotalWaitingTime += DELTA_T;
    //     }
    //     myAngle = std::numeric_limits<double>::max(); // set on first access or via remote control
    // }
    #[allow(unused_variables)]
    pub fn walk(&mut self, obs: &MSPModel_Striping_Obstacles) {
        // GUI-only debug dump: DEBUGCOND is person isSelected(), always false in a headless run, so not ported
        let stripes = obs.len() as i32;
        let sMax = stripes - 1;
        debug_assert!(stripes == MSPModel_Striping::num_stripes(self.myLane));
        // account stage-specific max speed but also for normal lane speed limit
        // (speed limits on crossings and walkingareas ignored due to #11527)
        let vMax = if self.myStage.get_configured_speed() >= 0. {
            self.myStage.get_configured_speed()
        } else if use_net_speeds() || self.myLane.is_normal() || self.myLane.is_internal() {
            self.myLane.get_vehicle_max_speed(self.myPerson.cast::<SUMOTrafficObject>())
        } else {
            self.myStage.get_max_speed(self.myPerson.cast::<MSTransportable>())
        };
        // ultimate goal is to choose the preferred stripe (chosen)
        let current = self.stripe();
        let other = self.other_stripe();
        // compute distances
        let mut distance: Vec<f64> = Vec::with_capacity(stripes as usize);
        for i in 0..stripes as usize {
            distance.push(self.distance_to(&obs[i], obs[i].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_PED));
        }
        // compute utility for all stripes
        let mut utility: Vec<f64> = vec![0.; stripes as usize];
        // forbid stripes which are blocked and also all stripes behind them
        for i in 0..stripes {
            let iu = i as usize;
            if distance[iu] == DIST_OVERLAP {
                if i == current && (!self.is_waiting_to_enter() || self.stripe() != Self::stripe_2(self.myPosLat)) {
                    utility[iu] += OBSTRUCTED_PENALTY;
                }
                if i < current {
                    for j in 0..=i {
                        utility[j as usize] += OBSTRUCTED_PENALTY;
                    }
                }
                if i > current {
                    for j in i..stripes {
                        utility[j as usize] += OBSTRUCTED_PENALTY;
                    }
                }
            }
        }
        // forbid a portion of the leftmost stripes (in walking direction).
        // lanes with stripes less than 1 / RESERVE_FOR_ONCOMING_FACTOR
        // may still deadlock in heavy pedestrian traffic
        let onJunction = self.myLane.is_walking_area() || self.myLane.is_crossing();
        let reserved = MSPModel_Striping::get_reserved(stripes, if onJunction { reserve_for_oncoming_factor_junctions() } else { reserve_for_oncoming_factor() });
        if self.myDir == MSPMODEL_FORWARD {
            for i in 0..reserved {
                utility[i as usize] += INAPPROPRIATE_PENALTY * (if i == current { 0.5 } else { 1. });
            }
        } else {
            let mut i = sMax;
            while i > sMax - reserved {
                utility[i as usize] += INAPPROPRIATE_PENALTY * (if i == current { 0.5 } else { 1. });
                i -= 1;
            }
        }
        // adapt utility based on obstacles
        for i in 0..stripes {
            let iu = i as usize;
            if obs[iu].speed * (self.myDir as f64) < 0. {
                // penalize evasion to the left unless the obstacle is a vehicle
                if (self.myDir == MSPMODEL_FORWARD || obs[iu].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE) && i > 0 {
                    utility[iu - 1] -= 0.5;
                } else if self.myDir == MSPMODEL_BACKWARD && i < sMax {
                    utility[iu + 1] -= 0.5;
                }
            }
            // compute expected distance achievable by staying on this stripe for a time horizon
            let walkDist = f64::max(0., distance[iu]); // disregard special distance flags
            let lookAhead = if obs[iu].speed * self.myDir as f64 >= 0. { LOOKAHEAD_SAMEDIR } else { LOOKAHEAD_ONCOMING };
            let expectedDist = f64::min(vMax * LOOKAHEAD_SAMEDIR, walkDist + obs[iu].speed * self.myDir as f64 * lookAhead);
            if expectedDist >= 0. {
                utility[iu] += expectedDist;
            } else {
                // let only the distance count
                utility[iu] += ONCOMING_CONFLICT_PENALTY + distance[iu];
            }
        }
        // discourage use of the leftmost stripe (in walking direction) if there are oncoming
        if self.myDir == MSPMODEL_FORWARD && obs[0].speed < 0. {
            utility[0] += ONCOMING_CONFLICT_PENALTY;
        } else if self.myDir == MSPMODEL_BACKWARD && obs[sMax as usize].speed > 0. {
            utility[sMax as usize] += ONCOMING_CONFLICT_PENALTY;
        }
        // penalize lateral movement (if the current stripe permits walking)
        if distance[current as usize] > 0. && self.myWaitingTime == 0 {
            for i in 0..stripes {
                utility[i as usize] += (i - current).abs() as f64 * LATERAL_PENALTY;
            }
        }
        // walk on the right side on shared space
        if self.myLane.get_permissions() != SVC_PEDESTRIAN && self.myDir == MSPMODEL_BACKWARD {
            for i in 0..stripes {
                if i <= current {
                    utility[i as usize] += (sMax - i + 1) as f64 * LATERAL_PENALTY;
                }
            }
        }
        // select best stripe
        let mut chosen = current;
        for i in 0..stripes {
            if utility[i as usize] > utility[chosen as usize] && utility[i as usize] >= INAPPROPRIATE_PENALTY * 0.5 {
                chosen = i;
            }
        }
        // compute speed components along both axes
        let next = if chosen == current {
            current
        } else if chosen < current {
            current - 1
        } else {
            current + 1
        };
        let mut xDist = f64::min(f64::min(distance[current as usize], distance[other as usize]), distance[next as usize]);
        if next != chosen {
            // ensure that we do not collide with an obstacle in the stripe beyond
            // next as this might become the 'other' stripe in the next step
            let nextOther = if chosen < current { current - 2 } else { current + 2 };
            xDist = f64::min(xDist, distance[nextOther as usize]);
        }
        // XXX preferred gap differs between approaching a standing obstacle or a moving obstacle
        let preferredGap = utils::geom::NUMERICAL_EPS;
        let mut xSpeed = f64::min(vMax, f64::max(0., utils::common::time::dist2speed(xDist - preferredGap)));
        if xSpeed < utils::geom::NUMERICAL_EPS {
            xSpeed = 0.;
        }
        // avoid tiny steps
        // XXX pressure from behind?
        if self.mySpeed == 0. && xDist < MIN_STARTUP_DIST
            // unless walking towards a short lane
            && !((xDist == distance[current as usize] && obs[current as usize].type_ >= MSPModel_Striping_ObstacleType::OBSTACLE_END)
                || (xDist == distance[other as usize] && obs[other as usize].type_ >= MSPModel_Striping_ObstacleType::OBSTACLE_END)
                || (xDist == distance[next as usize] && obs[next as usize].type_ >= MSPModel_Striping_ObstacleType::OBSTACLE_END))
        {
            xSpeed = 0.;
        }
        if xSpeed == 0. {
            let sharedWA = MY_WALKING_AREA_FOES.with(|m| m.borrow().contains_key(&Ptr::from_mut(self.myLane.get_edge())));
            let jamTimeHere = if self.myLane.is_crossing()
                // treat shared walkingarea like a crossing to avoid deadlocking vehicles
                || (self.myLane.is_walking_area()
                    && !obs[current as usize].vehicle.is_null()
                    && obs[current as usize].vehicle.cast::<MSVehicle>().get_waiting_time(false) > jam_time_crossing()
                    && sharedWA)
            {
                jam_time_crossing()
            } else {
                jam_time()
            };
            if self.myWaitingTime > jamTimeHere
                || (sMax == 0 && obs[0].speed * (self.myDir as f64) < 0. && self.myWaitingTime > jam_time_narrow())
                || self.myAmJammed
            {
                // squeeze slowly through the crowd ignoring others
                if !self.myAmJammed {
                    MSNet::get_instance().m().get_person_control().register_jammed();
                    utils::write_warningf!("Person '%' is jammed on edge '%', time=%.", self.myPerson.get_id(), self.myStage.get_edge().get_id(),
                        utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                    self.myAmJammed = true;
                }
                xSpeed = vMax * jam_factor();
            }
        } else if self.myAmJammed && Self::stripe_2(self.myPosLat) >= 0 && Self::stripe_2(self.myPosLat) <= sMax && xDist >= MIN_STARTUP_DIST {
            self.myAmJammed = false;
        }
        // dawdling
        let dawdle = f64::min(xSpeed, utils::common::rand_helper::rand() * vMax * dawdling());
        xSpeed -= dawdle;
        // XXX ensure that diagonal speed <= vMax
        let maxYSpeed = f64::min(f64::max(vMax * LATERAL_SPEED_FACTOR, vMax - xSpeed), stripe_width());
        let mut ySpeed = 0.;
        let mut yDist = 0.;
        if utility[next as usize] > OBSTRUCTION_THRESHOLD && utility[chosen as usize] > OBSTRUCTION_THRESHOLD {
            // don't move laterally if the stripes are blocked
            yDist = (chosen as f64 * stripe_width()) - self.myPosLat;
            if yDist.abs() > utils::geom::NUMERICAL_EPS {
                ySpeed = if yDist > 0. {
                    f64::min(maxYSpeed, utils::common::time::dist2speed(yDist))
                } else {
                    f64::max(-maxYSpeed, utils::common::time::dist2speed(yDist))
                };
            }
        } else if utility[next as usize] <= OBSTRUCTION_THRESHOLD
            && obs[next as usize].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE
            // still on the road
            && self.stripe() == Self::stripe_2(self.myPosLat)
            // only when the vehicle is moving on the same lane
            && !(self.myLane.is_crossing() || self.myLane.is_walking_area())
        {
            // step aside to let the vehicle pass
            let mut stepAsideDir = self.myDir;
            if self.myLane.get_edge().get_lanes().len() > 1 || current > sMax / 2 {
                // always step to the right on multi-lane edges or when closer to
                // the right side
                stepAsideDir = MSPMODEL_FORWARD;
            }
            self.myAmJammed = true; // ignore pedestrian-pedestrian collisions
            ySpeed = stepAsideDir as f64 * vMax;
        }
        self.myEdgePos += utils::common::time::speed2dist(xSpeed * self.myDir as f64);
        self.myPosLat += utils::common::time::speed2dist(ySpeed);
        self.mySpeedLat = ySpeed;
        self.mySpeed = xSpeed;
        if xSpeed >= utils::common::std_defs::SUMO_CONST_HALTING_SPEED {
            self.myWaitingToEnter = false;
            self.myWaitingTime = 0;
        } else {
            self.myWaitingTime += utils::common::time::delta_t();
            self.myTotalWaitingTime += utils::common::time::delta_t();
        }
        self.myAngle.set(f64::MAX); // set on first access or via remote control
    }

    /// C++: `double PState::getImpatience() const` [const]
    /// returns the impatience
    // double
    // MSPModel_Striping::PState::getImpatience() const {
    //     return MAX2(0., MIN2(1., myPerson->getVehicleType().getImpatience()
    //                          + STEPS2TIME(myStage->getWaitingTime()) / MAX_WAIT_TOLERANCE));
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        let x = self.myPerson.get_vehicle_type().get_impatience() + utils::common::time::steps2time(self.myStage.get_waiting_time()) / MAX_WAIT_TOLERANCE;
        let m = if 1. < x { 1. } else { x };
        if 0. > m { 0. } else { m }
    }

    /// C++: `int PState::stripe() const` [const]
    // int
    // MSPModel_Striping::PState::stripe() const {
    //     return MIN2(MAX2(0, stripe(myPosLat)), numStripes(myLane) - 1);
    // }
    #[allow(unused_variables)]
    pub fn stripe(&self) -> i32 {
        std::cmp::min(std::cmp::max(0, Self::stripe_2(self.myPosLat)), MSPModel_Striping::num_stripes(self.myLane) - 1)
    }

    /// C++: `int PState::otherStripe() const` [const]
    // int
    // MSPModel_Striping::PState::otherStripe() const {
    //     return MIN2(MAX2(0, otherStripe(myPosLat)), numStripes(myLane) - 1);
    // }
    #[allow(unused_variables)]
    pub fn other_stripe(&self) -> i32 {
        std::cmp::min(std::cmp::max(0, self.other_stripe_2(self.myPosLat)), MSPModel_Striping::num_stripes(self.myLane) - 1)
    }

    /// C++: `int PState::stripe(const double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn stripe_2(relY: f64) -> i32 {
        (relY / stripe_width() + 0.5).floor() as i32
    }

    /// C++: `int PState::otherStripe(const double) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn other_stripe_2(&self, relY: f64) -> i32 {
        let s = Self::stripe_2(relY);
        let offset = relY - s as f64 * stripe_width();
        let threshold = f64::max(utils::geom::NUMERICAL_EPS, stripe_width() - SQUEEZE * self.get_width());
        if offset > threshold {
            s + 1
        } else if offset < -threshold {
            s - 1
        } else {
            s
        }
    }

    /// C++: `double PState::distanceTo(const Obstacle &, const bool) const` [const]
    // double
    // MSPModel_Striping::PState::distanceTo(const Obstacle& obs, const bool includeMinGap) const {
    //     // check for overlap
    //     const double maxX = getMaxX(includeMinGap);
    //     const double minX = getMinX(includeMinGap);
    //     //if (DEBUGCOND(*this)) {
    //     //    std::cout << std::setprecision(2) <<   "   distanceTo=" << obs.description << " maxX=" << maxX << " minX=" << minX << " obs.xFwd=" << obs.xFwd << " obs.xBack=" << obs.xBack << "\n";
    //     //}
    //     if ((obs.xFwd >= maxX && obs.xBack <= maxX) || (obs.xFwd <= maxX && obs.xFwd >= minX)) {
    //         // avoid blocking by itself on looped route
    //         return (obs.type == OBSTACLE_PED && obs.description == myPerson->getID()) ? DIST_FAR_AWAY : DIST_OVERLAP;
    //     }
    //     if (myDir == FORWARD) {
    //         return obs.xFwd < minX ? DIST_BEHIND : obs.xBack - maxX;
    //     } else {
    //         return obs.xBack > maxX ? DIST_BEHIND : minX - obs.xFwd;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn distance_to(&self, obs: &MSPModel_Striping_Obstacle, includeMinGap: bool) -> f64 {
        // check for overlap
        let maxX = self.get_max_x(includeMinGap);
        let minX = self.get_min_x(includeMinGap);
        if (obs.xFwd >= maxX && obs.xBack <= maxX) || (obs.xFwd <= maxX && obs.xFwd >= minX) {
            // avoid blocking by itself on looped route
            return if obs.type_ == MSPModel_Striping_ObstacleType::OBSTACLE_PED && obs.description == self.myPerson.get_id() { DIST_FAR_AWAY } else { DIST_OVERLAP };
        }
        if self.myDir == MSPMODEL_FORWARD {
            if obs.xFwd < minX { DIST_BEHIND } else { obs.xBack - maxX }
        } else {
            if obs.xBack > maxX { DIST_BEHIND } else { minX - obs.xFwd }
        }
    }

    /// C++: `void PState::mergeObstacles(Obstacles &, const Obstacles &)`
    /// replace obstacles in the first vector with obstacles from the second if they are closer to me
    // void
    // MSPModel_Striping::PState::mergeObstacles(Obstacles& into, const Obstacles& obs2) {
    //     for (int i = 0; i < (int)into.size(); ++i) {
    //         if (gDebugFlag1) {
    //             std::cout << "     i=" << i << " maxX=" << getMaxX(true) << " minX=" << getMinX(true)
    //                       << " into=" << into[i].description << " iDist=" << distanceTo(into[i], into[i].type == OBSTACLE_PED)
    //                       << " obs2=" << obs2[i].description << " oDist=" << distanceTo(obs2[i], obs2[i].type == OBSTACLE_PED) << "\n";
    //         }
    //         const double dO = distanceTo(obs2[i], obs2[i].type == OBSTACLE_PED);
    //         const double dI = distanceTo(into[i], into[i].type == OBSTACLE_PED);
    //         if (dO < dI) {
    //             into[i] = obs2[i];
    //         } else if (dO == dI
    //                    && into[i].type != OBSTACLE_PED
    //                    && into[i].type != OBSTACLE_VEHICLE
    //                    && (obs2[i].type == OBSTACLE_PED ||
    //                        obs2[i].type == OBSTACLE_VEHICLE)) {
    //             into[i] = obs2[i];
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn merge_obstacles(&mut self, into: &mut MSPModel_Striping_Obstacles, obs2: &MSPModel_Striping_Obstacles) {
        // GUI-only debug dump: gDebugFlag1 is set only from DEBUGCOND (isSelected(), always false in a headless run), so not ported
        for i in 0..into.len() {
            let dO = self.distance_to(&obs2[i], obs2[i].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_PED);
            let dI = self.distance_to(&into[i], into[i].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_PED);
            if dO < dI {
                into[i] = obs2[i].clone();
            } else if dO == dI
                && into[i].type_ != MSPModel_Striping_ObstacleType::OBSTACLE_PED
                && into[i].type_ != MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE
                && (obs2[i].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_PED || obs2[i].type_ == MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE)
            {
                into[i] = obs2[i].clone();
            }
        }
    }

    /// C++: `void PState::mergeObstacles(Obstacles &, const Obstacles &, int, int)` [static]
    /// replace obstacles in the first vector with obstacles from the second if they are closer in the given direction
    // void
    // MSPModel_Striping::PState::mergeObstacles(Obstacles& into, const Obstacles& obs2, int dir, int offset) {
    //     for (int i = 0; i < (int)into.size(); ++i) {
    //         int i2 = i + offset;
    //         if (i2 >= 0 && i2 < (int)obs2.size()) {
    //             if (dir == FORWARD) {
    //                 if (obs2[i2].xBack < into[i].xBack) {
    //                     into[i] = obs2[i2];
    //                 }
    //             } else {
    //                 if (obs2[i2].xFwd > into[i].xFwd) {
    //                     into[i] = obs2[i2];
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn merge_obstacles_2(into: &mut MSPModel_Striping_Obstacles, obs2: &MSPModel_Striping_Obstacles, dir: i32, offset: i32) {
        for i in 0..into.len() {
            let i2 = i as i32 + offset;
            if i2 >= 0 && (i2 as usize) < obs2.len() {
                if dir == MSPMODEL_FORWARD {
                    if obs2[i2 as usize].xBack < into[i].xBack {
                        into[i] = obs2[i2 as usize].clone();
                    }
                } else {
                    if obs2[i2 as usize].xFwd > into[i].xFwd {
                        into[i] = obs2[i2 as usize].clone();
                    }
                }
            }
        }
    }

    /// C++: `bool PState::ignoreRed(const MSLink *) const` [const]
    /// whether the pedestrian may ignore a red light
    // bool
    // MSPModel_Striping::PState::ignoreRed(const MSLink* link) const {
    //     if (link->haveRed()) {
    //         const double ignoreRedTime = myPerson->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_DRIVE_AFTER_RED_TIME, -1);
    //         if (ignoreRedTime >= 0) {
    //             const double redDuration = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - link->getLastStateChange());
    //             if (DEBUGCOND(*this)) {
    //                 std::cout << SIMTIME << "  ignoreRedTime=" << ignoreRedTime << " redDuration=" << redDuration << "\n";
    //             }
    //             return ignoreRedTime > redDuration;
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn ignore_red(&self, link: Ptr<MSLink>) -> bool {
        if link.have_red() {
            let ignoreRedTime = self.myPerson.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_DRIVE_AFTER_RED_TIME, -1.);
            if ignoreRedTime >= 0. {
                let redDuration = utils::common::time::steps2time(MSNet::get_instance().get_current_time_step() - link.get_last_state_change());
                ignoreRedTime > redDuration
            } else {
                false
            }
        } else {
            false
        }
    }

    /// C++: `bool PState::stopForYellow(const MSLink *) const` [const]
    /// whether the pedestrian should stop at a yellow light
    // bool
    // MSPModel_Striping::PState::stopForYellow(const MSLink* link) const {
    //     // main use case is at rail_crossing
    //     if (link->haveYellow()) {
    //         const double ignoreYellowTime = myPerson->getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME, -1);
    //         if (ignoreYellowTime >= 0) {
    //             const double yellowDuration = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - link->getLastStateChange());
    //             if (DEBUGCOND(*this)) {
    //                 std::cout << SIMTIME << "  ignoreYellowTime=" << ignoreYellowTime << " yellowDuration=" << yellowDuration << "\n";
    //             }
    //             return ignoreYellowTime < yellowDuration;
    //         } else {
    //             return true;
    //         }
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn stop_for_yellow(&self, link: Ptr<MSLink>) -> bool {
        // main use case is at rail_crossing
        if link.have_yellow() {
            let ignoreYellowTime = self.myPerson.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_DRIVE_AFTER_YELLOW_TIME, -1.);
            if ignoreYellowTime >= 0. {
                let yellowDuration = utils::common::time::steps2time(MSNet::get_instance().get_current_time_step() - link.get_last_state_change());
                ignoreYellowTime < yellowDuration
            } else {
                true
            }
        } else {
            false
        }
    }

    /// C++: `double PState::getWidth() const` [const, virtual]
    /// return the person width
    // double
    // MSPModel_Striping::PState::getWidth() const {
    //     return myPerson->getVehicleType().getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        (self.base.base.vt.get_width)(&self.base.base)
    }
    pub fn get_width_base(&self) -> f64 {
        self.myPerson.get_vehicle_type().get_width()
    }

    /// C++: `ObstacleType PState::getOType() const` [const, virtual]
    // virtual ObstacleType getOType() const {
    //             return OBSTACLE_PED;
    //         }
    #[allow(unused_variables)]
    pub fn get_o_type(&self) -> MSPModel_Striping_ObstacleType {
        (self.base.base.vt.get_o_type)(&self.base.base)
    }
    pub fn get_o_type_base(&self) -> MSPModel_Striping_ObstacleType {
        MSPModel_Striping_ObstacleType::OBSTACLE_PED
    }

    /// C++: `bool PState::isRemoteControlled() const` [const]
    /// whether the person is currently being controlled via TraCI
    // bool
    // MSPModel_Striping::PState::isRemoteControlled() const {
    //     return myPerson->hasInfluencer() && myPerson->getInfluencer().isRemoteControlled();
    // }
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        self.myPerson.has_influencer() && self.myPerson.m().get_influencer().is_remote_controlled()
    }

    /// C++: `void PState::saveState(std::ostringstream &)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSPModel_Striping::PState::saveState(std::ostringstream& out) {
    //     std::string wapLaneFrom = "null";
    //     std::string wapLaneTo = "null";
    //     if (myWalkingAreaPath != nullptr) {
    //         wapLaneFrom = myWalkingAreaPath->from->getID();
    //         wapLaneTo = myWalkingAreaPath->to->getID();
    //     }
    //     std::string nextLaneID = "null";
    //     std::string nextLinkFrom = "null";
    //     std::string nextLinkTo = "null";
    //     if (myNLI.lane != nullptr) {
    //         nextLaneID = myNLI.lane->getID();
    //     }
    //     if (myNLI.link != nullptr) {
    //         nextLinkFrom = myNLI.link->getLaneBefore()->getID();
    //         nextLinkTo = myNLI.link->getViaLaneOrLane()->getID();
    //     }
    //     out << " " << myLane->getID()
    //         << " " << myEdgePos
    //         << " " << myPosLat
    //         << " " << myDir
    //         << " " << mySpeed
    //         << " " << mySpeedLat
    //         << " " << myWaitingToEnter
    //         << " " << myWaitingTime
    //         << " " << myTotalWaitingTime
    //         << " " << wapLaneFrom
    //         << " " << wapLaneTo
    //         << " " << myAmJammed
    //         << " " << nextLaneID
    //         << " " << nextLinkFrom
    //         << " " << nextLinkTo
    //         << " " << myNLI.dir;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */) {
        // ostream default precision (6 significant digits) for doubles, as in the C++ `<<`
        let wapLaneFrom = if !self.myWalkingAreaPath.is_null() { self.myWalkingAreaPath.from.get_id().to_string() } else { "null".to_string() };
        let wapLaneTo = if !self.myWalkingAreaPath.is_null() { self.myWalkingAreaPath.to.get_id().to_string() } else { "null".to_string() };
        let nextLaneID = if !self.myNLI.lane.is_null() { self.myNLI.lane.get_id().to_string() } else { "null".to_string() };
        let (nextLinkFrom, nextLinkTo) = if !self.myNLI.link.is_null() {
            (self.myNLI.link.get_lane_before().get_id().to_string(), self.myNLI.link.get_via_lane_or_lane().get_id().to_string())
        } else {
            ("null".to_string(), "null".to_string())
        };
        let b = &self.base;
        out.push_str(&format!(
            " {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            b.myLane.get_id(),
            f64_to_string_prec(b.myEdgePos, 6),
            f64_to_string_prec(b.myPosLat, 6),
            b.myDir,
            f64_to_string_prec(b.mySpeed, 6),
            f64_to_string_prec(b.mySpeedLat, 6),
            b.myWaitingToEnter as i32,
            b.myWaitingTime,
            b.myTotalWaitingTime,
            wapLaneFrom,
            wapLaneTo,
            b.myAmJammed as i32,
            nextLaneID,
            nextLinkFrom,
            nextLinkTo,
            self.myNLI.dir
        ));
    }

    /// C++: `const MSLane * PState::getNextCrossing() const` [const, virtual]
    // const MSLane*
    // MSPModel_Striping::PState::getNextCrossing() const {
    //     return myNLI.lane != nullptr && myNLI.lane->isCrossing() ? myNLI.lane : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_crossing(&self) -> Ptr<MSLane> {
        if !self.myNLI.lane.is_null() && self.myNLI.lane.is_crossing() {
            self.myNLI.lane
        } else {
            Ptr::null()
        }
    }

    /// C++: `double PState::getLatOffset() const` [const, virtual]
    /// return the lateral offset to the lane center
    // double getLatOffset() const {
    //             return posLatConversion(myPosLat, myLane->getWidth());
    //         }
    #[allow(unused_variables)]
    pub fn get_lat_offset(&self) -> f64 {
        MSPModel_Striping::pos_lat_conversion(self.myPosLat, self.myLane.get_width())
    }

    /// C++: `double PState::getPosLat() const` [const]
    // inline double getPosLat() const {
    //             return myPosLat;
    //         }
    #[allow(unused_variables)]
    pub fn get_pos_lat(&self) -> f64 {
        self.myPosLat
    }

    /// C++: `double PState::getPathLength() const` [const, virtual]
    // double
    // MSPModel_Striping::PState::getPathLength() const {
    //     if (myWalkingAreaPath != nullptr) {
    //         return myWalkingAreaPath->length;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_path_length(&self) -> f64 {
        if !self.myWalkingAreaPath.is_null() {
            self.myWalkingAreaPath.length
        } else {
            0.
        }
    }

    /// C++: `void PState::reverse(const double, const double)`
    // void
    // MSPModel_Striping::PState::reverse(const double pathLength, const double usableWidth) {
    //     myEdgePos = pathLength - myEdgePos;
    //     myPosLat = usableWidth - myPosLat;
    //     myDir = -myWalkingAreaPath->dir;
    //     mySpeedLat = -mySpeedLat;
    // }
    #[allow(unused_variables)]
    pub fn reverse(&mut self, pathLength: f64, usableWidth: f64) {
        self.myEdgePos = pathLength - self.myEdgePos;
        self.myPosLat = usableWidth - self.myPosLat;
        self.myDir = -self.myWalkingAreaPath.dir;
        self.mySpeedLat = -self.mySpeedLat;
    }

    /// C++: `void PState::reset(const double, const double)`
    // void
    // MSPModel_Striping::PState::reset(const double edgePos, const double latPos) {
    //     myEdgePos = edgePos;
    //     myPosLat = latPos;
    //     myDir = UNDEFINED_DIRECTION;  // only an obstacle, speed may be orthogonal to dir
    //     mySpeed = 0.;
    //     mySpeedLat = 0.;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self, edgePos: f64, latPos: f64) {
        self.myEdgePos = edgePos;
        self.myPosLat = latPos;
        self.myDir = MSPMODEL_UNDEFINED_DIRECTION; // only an obstacle, speed may be orthogonal to dir
        self.mySpeed = 0.;
        self.mySpeedLat = 0.;
    }

    /// C++: `void PState::PState()` [protected]
    /// constructor for PStateVehicle
    // MSPModel_Striping::PState::PState() :
    //     MSPModel_InteractingState(nullptr, nullptr, nullptr) {}
    #[allow(unused_variables)]
    pub fn new_3() -> Self {
        // MSPModel_InteractingState(nullptr, nullptr, nullptr) inlined: all-default state
        MSPModel_Striping_PState {
            base: MSPModel_InteractingState {
                base: MSTransportableStateAdapter { vt: &MSPMODEL_STRIPING_PSTATE_VT },
                myPerson: Ptr::null(),
                myStage: Ptr::null(),
                myLane: Ptr::null(),
                myEdgePos: 0.,
                myPosLat: 0.,
                myDir: MSPMODEL_UNDEFINED_DIRECTION,
                mySpeed: 0.,
                mySpeedLat: 0.,
                myWaitingToEnter: false,
                myWaitingTime: 0,
                myTotalWaitingTime: 0,
                myAmJammed: false,
                myRemoteXYPos: Position::INVALID,
                myAngle: std::cell::Cell::new(f64::MAX),
            },
            myNLI: MSPModel_Striping_NextLaneInfo::new_2(),
            myWalkingAreaPath: Ptr::null(),
        }
    }

    /// C++: `PState & PState::operator=(const PState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_Striping_PState) -> &mut MSPModel_Striping_PState {
        // C++ declares this private and never defines it (never used); behaves like the implicit copy
        *self = copy_pstate(arg0);
        self
    }

}

/// C++ class `MSPModel_Striping::PStateVehicle` (microsim/transportables/MSPModel_Striping.h)
#[repr(C)]
pub struct MSPModel_Striping_PStateVehicle {
    /// C++ base class `PState`
    pub base: MSPModel_Striping_PState,
    pub myVehicle: Ptr<MSVehicle>,
    pub myXWidth: f64,
    pub myYWidth: f64,
}

impl std::ops::Deref for MSPModel_Striping_PStateVehicle {
    type Target = MSPModel_Striping_PState;
    fn deref(&self) -> &MSPModel_Striping_PState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_Striping_PStateVehicle {
    fn deref_mut(&mut self) -> &mut MSPModel_Striping_PState {
        &mut self.base
    }
}

impl MSPModel_Striping_PStateVehicle {
    /// C++: `void PStateVehicle::PStateVehicle(const MSVehicle *, const MSLane *, double, double, double, double)`
    // MSPModel_Striping::PStateVehicle::PStateVehicle(const MSVehicle* veh, const MSLane* walkingarea, double relX, double relY, double xWidth, double yWidth):
    //     myVehicle(veh), myXWidth(xWidth), myYWidth(yWidth) {
    //     myLane = walkingarea; // to ensure correct limits when calling otherStripe()
    //     // relX is the center but we want it to be the max value if the movement direction is forward
    //     // and the min value otherwise (indicated by xWidth sign)
    //     myEdgePos = relX + xWidth / 2;
    //     myPosLat = relY;
    // }
    #[allow(unused_variables)]
    pub fn new(veh: Ptr<MSVehicle>, walkingarea: Ptr<MSLane>, relX: f64, relY: f64, xWidth: f64, yWidth: f64) -> Self {
        let mut s = MSPModel_Striping_PStateVehicle {
            base: MSPModel_Striping_PState::new_3(),
            myVehicle: veh,
            myXWidth: xWidth,
            myYWidth: yWidth,
        };
        s.base.base.base.vt = &MSPMODEL_STRIPING_PSTATE_VEHICLE_VT;
        s.myLane = walkingarea; // to ensure correct limits when calling otherStripe()
        // relX is the center but we want it to be the max value if the movement direction is forward
        // and the min value otherwise (indicated by xWidth sign)
        s.myEdgePos = relX + xWidth / 2.;
        s.myPosLat = relY;
        s
    }

    /// C++: `const std::string & PStateVehicle::getID() const` [const, virtual]
    // const std::string&
    // MSPModel_Striping::PStateVehicle::getID() const {
    //     return myVehicle->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        self.myVehicle.get_id()
    }

    /// C++: `double PStateVehicle::getMinX(const bool) const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getMinX(const bool /*includeMinGap*/) const {
    //     return myXWidth > 0 ? myEdgePos - myXWidth : myEdgePos;
    // }
    #[allow(unused_variables)]
    pub fn get_min_x(&self, includeMinGap: bool) -> f64 {
        if self.myXWidth > 0. { self.myEdgePos - self.myXWidth } else { self.myEdgePos }
    }

    /// C++: `double PStateVehicle::getMaxX(const bool) const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getMaxX(const bool /*includeMinGap*/) const {
    //     return myXWidth > 0 ? myEdgePos : myEdgePos - myXWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_max_x(&self, includeMinGap: bool) -> f64 {
        if self.myXWidth > 0. { self.myEdgePos } else { self.myEdgePos - self.myXWidth }
    }

    /// C++: `double PStateVehicle::getWidth() const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getWidth() const {
    //     return myYWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        self.myYWidth
    }

    /// C++: `ObstacleType PStateVehicle::getOType() const` [const, virtual]
    // ObstacleType getOType() const {
    //             return OBSTACLE_VEHICLE;
    //         }
    #[allow(unused_variables)]
    pub fn get_o_type(&self) -> MSPModel_Striping_ObstacleType {
        MSPModel_Striping_ObstacleType::OBSTACLE_VEHICLE
    }

    /// C++: `const MSVehicle * PStateVehicle::getVehicle() const` [const]
    // const MSVehicle* getVehicle() const {
    //             return myVehicle;
    //         }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<MSVehicle> {
        self.myVehicle
    }

}

/// C++ class `MSPModel_Striping::WalkingAreaPath` (microsim/transportables/MSPModel_Striping.h)
pub struct MSPModel_Striping_WalkingAreaPath {
    pub from: Ptr<MSLane>,
    pub to: Ptr<MSLane>,
    pub lane: Ptr<MSLane>,
    pub shape: PositionVector,
    pub dir: i32,
    pub angleOverride: f64,
    pub length: f64,
}

impl MSPModel_Striping_WalkingAreaPath {
    /// C++: `void WalkingAreaPath::WalkingAreaPath(const MSLane *, const MSLane *, const MSLane *, const PositionVector &, int, double)`
    // WalkingAreaPath(const MSLane* _from, const MSLane* _walkingArea, const MSLane* _to, const PositionVector& _shape, int _dir, double _angleOverride) :
    //             from(_from),
    //             to(_to),
    //             lane(_walkingArea),
    //             shape(_shape),
    //             dir(_dir),
    //             angleOverride(_angleOverride),
    //             length(_shape.length()) {
    //         }
    #[allow(unused_variables)]
    pub fn new(_from: Ptr<MSLane>, _walkingArea: Ptr<MSLane>, _to: Ptr<MSLane>, _shape: &PositionVector, _dir: i32, _angleOverride: f64) -> Self {
        MSPModel_Striping_WalkingAreaPath {
            from: _from,
            to: _to,
            lane: _walkingArea,
            shape: _shape.clone(),
            dir: _dir,
            angleOverride: _angleOverride,
            length: _shape.length(),
        }
    }

}

/// sorts the persons by position on the lane. If dir is forward, higher x positions come first.
/// C++ class `MSPModel_Striping::by_xpos_sorter` (microsim/transportables/MSPModel_Striping.h)
pub struct MSPModel_Striping_by_xpos_sorter {
    pub myDir: i32,
}

impl MSPModel_Striping_by_xpos_sorter {
    /// C++: `void by_xpos_sorter::by_xpos_sorter(int)`
    /// constructor
    // by_xpos_sorter(int dir): myDir(dir) {}
    #[allow(unused_variables)]
    pub fn new(dir: i32) -> Self {
        Self { myDir: dir }
    }

    /// C++: `bool by_xpos_sorter::operator()(const MSPModel_InteractingState *, const MSPModel_InteractingState *) const` [const]
    /// comparing operation
    // bool operator()(const MSPModel_InteractingState* p1, const MSPModel_InteractingState* p2) const {
    //             if (p1->getEdgePos(0) != p2->getEdgePos(0)) {
    //                 return myDir * p1->getEdgePos(0) > myDir * p2->getEdgePos(0);
    //             }
    //             return p1->getID() < p2->getID();
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, p1: Ptr<MSPModel_InteractingState>, p2: Ptr<MSPModel_InteractingState>) -> bool {
        if p1.get_edge_pos(0) != p2.get_edge_pos(0) {
            return self.myDir as f64 * p1.get_edge_pos(0) > self.myDir as f64 * p2.get_edge_pos(0);
        }
        p1.get_id() < p2.get_id()
    }

}

/// C++ class `MSPModel_Striping::walkingarea_path_sorter` (microsim/transportables/MSPModel_Striping.h)
pub struct MSPModel_Striping_walkingarea_path_sorter {
}

impl MSPModel_Striping_walkingarea_path_sorter {
    /// C++: `bool walkingarea_path_sorter::operator()(const WalkingAreaPath *, const WalkingAreaPath *) const` [const]
    /// comparing operation
    // bool operator()(const WalkingAreaPath* p1, const WalkingAreaPath* p2) const {
    //             if (p1->from->getNumericalID() < p2->from->getNumericalID()) {
    //                 return true;
    //             }
    //             if (p1->from->getNumericalID() == p2->from->getNumericalID()) {
    //                 if (p1->to->getNumericalID() < p2->to->getNumericalID()) {
    //                     return true;
    //                 }
    //             }
    //             return false;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, p1: Ptr<MSPModel_Striping_WalkingAreaPath>, p2: Ptr<MSPModel_Striping_WalkingAreaPath>) -> bool {
        if p1.from.get_numerical_id() < p2.from.get_numerical_id() {
            return true;
        }
        if p1.from.get_numerical_id() == p2.from.get_numerical_id() {
            if p1.to.get_numerical_id() < p2.to.get_numerical_id() {
                return true;
            }
        }
        false
    }

}

