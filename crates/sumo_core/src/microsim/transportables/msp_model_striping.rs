//! Port of microsim/transportables/MSPModel_Striping.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{SVC_IGNORING, SVC_PEDESTRIAN};

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

/// C++ getSidewalk<MSEdge, MSLane>(edge, svc) from utils/router/IntermodalEdge.h
// shortcut: local copy because the Rust getSidewalk template (utils/router/intermodal_edge.rs) is still todo!(); use it once ported
fn get_sidewalk(edge: Ptr<MSEdge>, svc: SUMOVehicleClass) -> Ptr<MSLane> {
    if edge.is_null() {
        return Ptr::null();
    }
    // prefer lanes that are exclusive to pedestrians
    let lanes = edge.get_lanes();
    for lane in lanes.iter() {
        if lane.get_permissions() == svc {
            return *lane;
        }
    }
    for lane in lanes.iter() {
        if lane.allows_vehicle_class(svc) {
            return *lane;
        }
    }
    if svc != SVC_PEDESTRIAN {
        // persons should always be able to use the sidewalk
        for lane in lanes.iter() {
            if lane.get_permissions() == SVC_PEDESTRIAN {
                return *lane;
            }
        }
        for lane in lanes.iter() {
            if lane.allows_vehicle_class(SVC_PEDESTRIAN) {
                return *lane;
            }
        }
    }
    Ptr::null()
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
                base: MSPModel {},
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
        // shortcut: MMVersion is not ported (MSNet::myVersion is a () placeholder), so the net version check
        // (networkVersion >= 1.20) is assumed to hold; upgrade when MMVersion is ported and compare it here
        set_use_net_speeds(true);

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
        debug_assert!(person.get_current_stage_type() == MSStageType::WALKING);
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
        self.myActiveLanes.entry(lane).or_default().push(ped);
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
    pub fn load_state(&mut self, transportable: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, in_: &mut Unmapped /* std::istringstream */) -> Ptr<MSTransportableStateAdapter> {
        todo!("port MSPModel_Striping::loadState")
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
        todo!("port MSPModel_Striping::moveInDirection")
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
        todo!("port MSPModel_Striping::moveInDirectionOnLane")
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
                    self.myActiveLanes.entry(p.get_lane()).or_default().push(p.cast::<MSPModel_InteractingState>());
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
        todo!("port MSPModel_Striping::getNextLane")
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
        todo!("port MSPModel_Striping::initWalkingAreaPaths")
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
        todo!("port MSPModel_Striping::insertWalkArePaths")
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
        todo!("port MSPModel_Striping::getNeighboringObstacles")
    }

    /// C++: `const Obstacles & MSPModel_Striping::getNextLaneObstacles(NextLanesObstacles &, const MSLane *, const MSLane *, int, int, double, int)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_next_lane_obstacles(&mut self, nextLanesObs: &mut MSPModel_Striping_NextLanesObstacles, lane: Ptr<MSLane>, nextLane: Ptr<MSLane>, stripes: i32, nextDir: i32, currentLength: f64, currentDir: i32) -> &MSPModel_Striping_Obstacles {
        todo!("port MSPModel_Striping::getNextLaneObstacles")
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
        todo!("port MSPModel_Striping::transformToCurrentLanePositions")
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
        todo!("port MSPModel_Striping::addCloserObstacle")
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
        todo!("port MSPModel_Striping::getStripeOffset")
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
        todo!("port MSPModel_Striping::addCrossingVehs")
    }

    /// C++: `Obstacles MSPModel_Striping::getVehicleObstacles(const MSLane *, int, PState *)` [static, private]
    /// retrieve vehicle obstacles on the given lane
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_vehicle_obstacles(lane: Ptr<MSLane>, dir: i32, ped: Ptr<MSPModel_Striping_PState>) -> MSPModel_Striping_Obstacles {
        todo!("port MSPModel_Striping::getVehicleObstacles")
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
    pub fn add_vehicle_foe(veh: Ptr<MSVehicle>, walkingarea: Ptr<MSLane>, relPos: &Position, xWidth: f64, yWidth: f64, lateral_offset: f64, minY: f64, maxY: f64, toDelete: &mut MSPModel_Interacting_Pedestrians, transformedPeds: &mut MSPModel_Interacting_Pedestrians) -> bool {
        todo!("port MSPModel_Striping::addVehicleFoe")
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
        todo!("port MSPModel_Striping::getReserved")
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
        todo!("port MSPModel_Striping::registerCrossingApproach")
    }

}

/// C++ class `MSPModel_Striping::MovePedestrians` (microsim/transportables/MSPModel_Striping.h)
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
        todo!("port MovePedestrians::MovePedestrians")
    }

    /// C++: `void MovePedestrians::~MovePedestrians() noexcept` [virtual]
    // ~MovePedestrians() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MovePedestrians::~MovePedestrians")
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
        todo!("port MovePedestrians::execute")
    }

    /// C++: `MovePedestrians & MovePedestrians::operator=(const MovePedestrians &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCDReplay_MovePedestrians) -> &mut MSTransportableDevice_FCDReplay_MovePedestrians {
        todo!("port MovePedestrians::operator=")
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
        todo!("port NextLaneInfo::NextLaneInfo")
    }

    /// C++: `void NextLaneInfo::NextLaneInfo()`
    // NextLaneInfo() :
    //             lane(0),
    //             link(0),
    //             dir(UNDEFINED_DIRECTION) {
    //         }
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        todo!("port NextLaneInfo::NextLaneInfo")
    }

}

/// information regarding surround Pedestrians (and potentially other things)
/// C++ class `MSPModel_Striping::Obstacle` (microsim/transportables/MSPModel_Striping.h)
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
        todo!("port Obstacle::Obstacle")
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
        todo!("port Obstacle::Obstacle")
    }

    /// C++: `void Obstacle::Obstacle(double, double, ObstacleType, const std::string &, const double, const SUMOVehicle *)`
    /// create an obstacle from explicit values
    // Obstacle(double _x, double _speed, ObstacleType _type, const std::string& _description, const double width = 0., const SUMOVehicle* veh = nullptr)
    //             : xFwd(_x + width / 2.), xBack(_x - width / 2.), speed(_speed), type(_type), description(_description), vehicle(veh) {}
    #[allow(unused_variables)]
    pub fn new_3(_x: f64, _speed: f64, _type: MSPModel_Striping_ObstacleType, _description: &str, width: f64, veh: Ptr<SUMOVehicle>) -> Self {
        todo!("port Obstacle::Obstacle")
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
        todo!("port Obstacle::closer")
    }

}

/// Container for pedestrian state and individual position update function
/// C++ class `MSPModel_Striping::PState` (microsim/transportables/MSPModel_Striping.h)
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
        todo!("port PState::PState")
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
    pub fn new_2(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, in_: Ptr<Unmapped /* std::istringstream */>) -> Self {
        todo!("port PState::PState")
    }

    /// C++: `void PState::~PState() noexcept` [virtual]
    // ~PState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PState::~PState")
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
        todo!("port PState::getPosition")
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
        todo!("port PState::getAngle")
    }

    /// C++: `const MSEdge * PState::getNextEdge(const MSStageMoving &) const` [const, virtual]
    // const MSEdge*
    // MSPModel_Striping::PState::getNextEdge(const MSStageMoving&) const {
    //     return myNLI.lane == nullptr ? nullptr : &myNLI.lane->getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        todo!("port PState::getNextEdge")
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
        todo!("port PState::moveTo")
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
        todo!("port PState::moveToXY")
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
    #[allow(unused_variables)]
    pub fn get_min_x(&self, includeMinGap: bool) -> f64 {
        todo!("port PState::getMinX")
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
    #[allow(unused_variables)]
    pub fn get_max_x(&self, includeMinGap: bool) -> f64 {
        todo!("port PState::getMaxX")
    }

    /// C++: `double PState::getLength() const` [const]
    /// return the length of the pedestrian
    // double
    // MSPModel_Striping::PState::getLength() const {
    //     return myPerson->getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port PState::getLength")
    }

    /// C++: `double PState::getMinGap() const` [const]
    /// return the minimum gap of the pedestrian
    // double
    // MSPModel_Striping::PState::getMinGap() const {
    //     return myPerson->getVehicleType().getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(&self) -> f64 {
        todo!("port PState::getMinGap")
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
        todo!("port PState::distToLaneEnd")
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
        todo!("port PState::moveToNextLane")
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
        todo!("port PState::walk")
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
        todo!("port PState::getImpatience")
    }

    /// C++: `int PState::stripe() const` [const]
    // int
    // MSPModel_Striping::PState::stripe() const {
    //     return MIN2(MAX2(0, stripe(myPosLat)), numStripes(myLane) - 1);
    // }
    #[allow(unused_variables)]
    pub fn stripe(&self) -> i32 {
        todo!("port PState::stripe")
    }

    /// C++: `int PState::otherStripe() const` [const]
    // int
    // MSPModel_Striping::PState::otherStripe() const {
    //     return MIN2(MAX2(0, otherStripe(myPosLat)), numStripes(myLane) - 1);
    // }
    #[allow(unused_variables)]
    pub fn other_stripe(&self) -> i32 {
        todo!("port PState::otherStripe")
    }

    /// C++: `int PState::stripe(const double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn stripe_2(relY: f64) -> i32 {
        todo!("port PState::stripe")
    }

    /// C++: `int PState::otherStripe(const double) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn other_stripe_2(&self, relY: f64) -> i32 {
        todo!("port PState::otherStripe")
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
        todo!("port PState::distanceTo")
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
        todo!("port PState::mergeObstacles")
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
        todo!("port PState::mergeObstacles")
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
        todo!("port PState::ignoreRed")
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
        todo!("port PState::stopForYellow")
    }

    /// C++: `double PState::getWidth() const` [const, virtual]
    /// return the person width
    // double
    // MSPModel_Striping::PState::getWidth() const {
    //     return myPerson->getVehicleType().getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port PState::getWidth")
    }

    /// C++: `ObstacleType PState::getOType() const` [const, virtual]
    // virtual ObstacleType getOType() const {
    //             return OBSTACLE_PED;
    //         }
    #[allow(unused_variables)]
    pub fn get_o_type(&self) -> MSPModel_Striping_ObstacleType {
        todo!("port PState::getOType")
    }

    /// C++: `bool PState::isRemoteControlled() const` [const]
    /// whether the person is currently being controlled via TraCI
    // bool
    // MSPModel_Striping::PState::isRemoteControlled() const {
    //     return myPerson->hasInfluencer() && myPerson->getInfluencer().isRemoteControlled();
    // }
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        todo!("port PState::isRemoteControlled")
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
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */) {
        todo!("port PState::saveState")
    }

    /// C++: `const MSLane * PState::getNextCrossing() const` [const, virtual]
    // const MSLane*
    // MSPModel_Striping::PState::getNextCrossing() const {
    //     return myNLI.lane != nullptr && myNLI.lane->isCrossing() ? myNLI.lane : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_crossing(&self) -> Ptr<MSLane> {
        todo!("port PState::getNextCrossing")
    }

    /// C++: `double PState::getLatOffset() const` [const, virtual]
    /// return the lateral offset to the lane center
    // double getLatOffset() const {
    //             return posLatConversion(myPosLat, myLane->getWidth());
    //         }
    #[allow(unused_variables)]
    pub fn get_lat_offset(&self) -> f64 {
        todo!("port PState::getLatOffset")
    }

    /// C++: `double PState::getPosLat() const` [const]
    // inline double getPosLat() const {
    //             return myPosLat;
    //         }
    #[allow(unused_variables)]
    pub fn get_pos_lat(&self) -> f64 {
        todo!("port PState::getPosLat")
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
        todo!("port PState::getPathLength")
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
        todo!("port PState::reverse")
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
        todo!("port PState::reset")
    }

    /// C++: `void PState::PState()` [protected]
    /// constructor for PStateVehicle
    // MSPModel_Striping::PState::PState() :
    //     MSPModel_InteractingState(nullptr, nullptr, nullptr) {}
    #[allow(unused_variables)]
    pub fn new_3() -> Self {
        todo!("port PState::PState")
    }

    /// C++: `PState & PState::operator=(const PState &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPModel_Striping_PState) -> &mut MSPModel_Striping_PState {
        todo!("port PState::operator=")
    }

}

/// C++ class `MSPModel_Striping::PStateVehicle` (microsim/transportables/MSPModel_Striping.h)
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
        todo!("port PStateVehicle::PStateVehicle")
    }

    /// C++: `const std::string & PStateVehicle::getID() const` [const, virtual]
    // const std::string&
    // MSPModel_Striping::PStateVehicle::getID() const {
    //     return myVehicle->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        todo!("port PStateVehicle::getID")
    }

    /// C++: `double PStateVehicle::getMinX(const bool) const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getMinX(const bool /*includeMinGap*/) const {
    //     return myXWidth > 0 ? myEdgePos - myXWidth : myEdgePos;
    // }
    #[allow(unused_variables)]
    pub fn get_min_x(&self, includeMinGap: bool) -> f64 {
        todo!("port PStateVehicle::getMinX")
    }

    /// C++: `double PStateVehicle::getMaxX(const bool) const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getMaxX(const bool /*includeMinGap*/) const {
    //     return myXWidth > 0 ? myEdgePos : myEdgePos - myXWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_max_x(&self, includeMinGap: bool) -> f64 {
        todo!("port PStateVehicle::getMaxX")
    }

    /// C++: `double PStateVehicle::getWidth() const` [const, virtual]
    // double
    // MSPModel_Striping::PStateVehicle::getWidth() const {
    //     return myYWidth;
    // }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        todo!("port PStateVehicle::getWidth")
    }

    /// C++: `ObstacleType PStateVehicle::getOType() const` [const, virtual]
    // ObstacleType getOType() const {
    //             return OBSTACLE_VEHICLE;
    //         }
    #[allow(unused_variables)]
    pub fn get_o_type(&self) -> MSPModel_Striping_ObstacleType {
        todo!("port PStateVehicle::getOType")
    }

    /// C++: `const MSVehicle * PStateVehicle::getVehicle() const` [const]
    // const MSVehicle* getVehicle() const {
    //             return myVehicle;
    //         }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<MSVehicle> {
        todo!("port PStateVehicle::getVehicle")
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
        todo!("port WalkingAreaPath::WalkingAreaPath")
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

