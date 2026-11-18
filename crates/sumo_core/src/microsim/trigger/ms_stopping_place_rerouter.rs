//! Port of microsim/trigger/MSStoppingPlaceRerouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_globals::MSGlobals;
use crate::microsim::devices::ms_routing_engine::MSRoutingEngine;
use utils::common::string_utils::to_double;
use utils::common::time::time2string;
use utils::common::to_string::to_string;
use utils::common::sumo_vehicle_class::SVC_IGNORING;
use utils::geom::NUMERICAL_EPS;

type ParamMap = MSStoppingPlaceRerouter_StoppingPlaceParamMap_t;

/// C++ typedef `std::map<const MSEdge *, RouterProhibition> MSStoppingPlaceRerouter::Prohibitions`
pub type MSStoppingPlaceRerouter_Prohibitions = BTreeMap<Ptr<MSEdge>, RouterProhibition>;

/// C++ typedef `std::map<const MSStoppingPlace *, StoppingPlaceMemoryEntry, ComparatorIdLess> StoppingPlaceMemory::StoppingPlaceMap`
pub type StoppingPlaceMemory_StoppingPlaceMap = BTreeMap<Ptr<MSStoppingPlace>, StoppingPlaceMemoryEntry>;

/// C++ typedef `std::map<MSStoppingPlace *, StoppingPlaceParamMap_t, ComparatorIdLess> MSStoppingPlaceRerouter::StoppingPlaceMap_t`
pub type MSStoppingPlaceRerouter_StoppingPlaceMap_t = BTreeMap<Ptr<MSStoppingPlace>, MSStoppingPlaceRerouter_StoppingPlaceParamMap_t>;

/// C++ typedef `std::map<std::string, double> MSStoppingPlaceRerouter::StoppingPlaceParamMap_t`
pub type MSStoppingPlaceRerouter_StoppingPlaceParamMap_t = BTreeMap<String, f64>;

/// C++ typedef `std::map<std::string, bool> MSStoppingPlaceRerouter::StoppingPlaceParamSwitchMap_t`
pub type MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t = BTreeMap<String, bool>;

/// C++ typedef `std::pair<MSStoppingPlace *, bool> MSStoppingPlaceRerouter::StoppingPlaceVisible`
pub type MSStoppingPlaceRerouter_StoppingPlaceVisible = (Ptr<MSStoppingPlace>, bool);

/// C++ `operator[]` on a parameter map: inserts 0. for a missing key and returns it.
fn param_mut<'a>(m: &'a mut ParamMap, k: &str) -> &'a mut f64 {
    m.entry(k.to_string()).or_insert(0.)
}

/// C++ route color pointer: nullptr when the route has the default color (C++ compares addresses).
fn color_ptr(c: &RGBColor) -> Ptr<RGBColor> {
    if *c == RGBColor::DEFAULT_COLOR {
        Ptr::null()
    } else {
        Ptr::from_raw(Box::leak(Box::new(*c)) as *mut RGBColor)
    }
}

/// The stopping place's lane edge (C++ `&sp->getLane().getEdge()`).
fn lane_edge(sp: Ptr<MSStoppingPlace>) -> Ptr<MSEdge> {
    Ptr::from_mut(sp.get_lane().get_edge())
}

/// C++ `veh.getCurrentRouteEdge()` as a route index
fn route_cursor_index(veh: &SUMOVehicle) -> usize {
    *veh.get_current_route_edge()
}

/// C++ `MSRoute::getDistanceBetween(double, double, MSRouteIterator, MSRouteIterator, bool)`.
/// Route iterators are indices into the route edges (MSRouteIterator is Unmapped).
fn route_distance_between(route: &MSRoute, fromPos: f64, toPos: f64, fromIdx: usize, toIdx: usize, includeInternal: bool) -> f64 {
    let edges = route.get_edges();
    let end = edges.len();
    if fromIdx == toIdx {
        // destination position is on start edge
        return if fromPos <= toPos { toPos - fromPos } else { f64::MAX };
    } else if fromIdx > toIdx {
        // we don't visit the edge again
        return f64::MAX;
    }
    let mut isFirstIteration = true;
    let mut distance = -fromPos;
    let mut it = fromIdx;
    while it != end {
        if it == toIdx && !isFirstIteration {
            distance += toPos;
            break;
        }
        distance += edges[it].get_length();
        if includeInternal && it + 1 != end {
            distance += edges[it].get_internal_following_length_to(edges[it + 1], SVC_IGNORING);
        }
        isFirstIteration = false;
        it += 1;
    }
    distance
}

/// Virtual dispatch table for the pure virtual methods of MSStoppingPlaceRerouter.
/// Each subclass installs its own table in `vt`; the base methods forward through it.
pub struct MSStoppingPlaceRerouterVt {
    pub get_stopping_place_occupancy: fn(&mut MSStoppingPlaceRerouter, Ptr<MSStoppingPlace>, Ptr<SUMOVehicle>) -> f64,
    pub get_last_step_stopping_place_occupancy: fn(&mut MSStoppingPlaceRerouter, Ptr<MSStoppingPlace>, Ptr<SUMOVehicle>) -> f64,
    pub get_stopping_place_capacity: fn(&mut MSStoppingPlaceRerouter, Ptr<MSStoppingPlace>) -> f64,
    pub remember_blocked_stopping_place: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, Ptr<MSStoppingPlace>, bool),
    pub remember_stopping_place_score: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, Ptr<MSStoppingPlace>, &str),
    pub reset_stopping_place_scores: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle),
    pub saw_blocked_stopping_place: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, Ptr<MSStoppingPlace>, bool) -> SUMOTime,
    pub get_number_stopping_place_reroutes: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle) -> i32,
    pub set_number_stopping_place_reroutes: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, i32),
    pub evaluate_custom_components: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, f64, bool, Ptr<MSStoppingPlace>, f64, f64, &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, &mut ConstMSEdgeVector, &mut ConstMSEdgeVector, &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool,
    pub valid_component_values: fn(&mut MSStoppingPlaceRerouter, &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool,
    pub use_stopping_place: fn(&mut MSStoppingPlaceRerouter, Ptr<MSStoppingPlace>) -> bool,
    pub collect_weights: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle) -> MSStoppingPlaceRerouter_StoppingPlaceParamMap_t,
    /// C++ virtual `getRouter`; returns a raw pointer because the router is owned by MSNet/MSRoutingEngine.
    pub get_router: fn(&mut MSStoppingPlaceRerouter, &mut SUMOVehicle, &SUMOAbstractRouter_Prohibitions) -> Ptr<SUMOAbstractRouter<MSEdge, SUMOVehicle>>,
}

fn base_get_stopping_place_occupancy(_this: &mut MSStoppingPlaceRerouter, _stoppingPlace: Ptr<MSStoppingPlace>, _veh: Ptr<SUMOVehicle>) -> f64 {
    panic!("MSStoppingPlaceRerouter::getStoppingPlaceOccupancy is pure virtual; subclasses must override it")
}
fn base_get_last_step_stopping_place_occupancy(_this: &mut MSStoppingPlaceRerouter, _stoppingPlace: Ptr<MSStoppingPlace>, _veh: Ptr<SUMOVehicle>) -> f64 {
    panic!("MSStoppingPlaceRerouter::getLastStepStoppingPlaceOccupancy is pure virtual; subclasses must override it")
}
fn base_get_stopping_place_capacity(_this: &mut MSStoppingPlaceRerouter, _stoppingPlace: Ptr<MSStoppingPlace>) -> f64 {
    panic!("MSStoppingPlaceRerouter::getStoppingPlaceCapacity is pure virtual; subclasses must override it")
}
fn base_remember_blocked_stopping_place(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle, _stoppingPlace: Ptr<MSStoppingPlace>, _blocked: bool) {
    panic!("MSStoppingPlaceRerouter::rememberBlockedStoppingPlace is pure virtual; subclasses must override it")
}
fn base_remember_stopping_place_score(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle, _place: Ptr<MSStoppingPlace>, _score: &str) {
    panic!("MSStoppingPlaceRerouter::rememberStoppingPlaceScore is pure virtual; subclasses must override it")
}
fn base_reset_stopping_place_scores(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle) {
    panic!("MSStoppingPlaceRerouter::resetStoppingPlaceScores is pure virtual; subclasses must override it")
}
fn base_saw_blocked_stopping_place(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle, _place: Ptr<MSStoppingPlace>, _local: bool) -> SUMOTime {
    panic!("MSStoppingPlaceRerouter::sawBlockedStoppingPlace is pure virtual; subclasses must override it")
}
fn base_get_number_stopping_place_reroutes(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle) -> i32 {
    panic!("MSStoppingPlaceRerouter::getNumberStoppingPlaceReroutes is pure virtual; subclasses must override it")
}
fn base_set_number_stopping_place_reroutes(_this: &mut MSStoppingPlaceRerouter, _veh: &mut SUMOVehicle, _value: i32) {
    panic!("MSStoppingPlaceRerouter::setNumberStoppingPlaceReroutes is pure virtual; subclasses must override it")
}

/// Table used until a subclass constructor installs its own (C++ base-class vtable).
static MS_STOPPING_PLACE_REROUTER_BASE_VT: MSStoppingPlaceRerouterVt = MSStoppingPlaceRerouterVt {
    get_stopping_place_occupancy: base_get_stopping_place_occupancy,
    get_last_step_stopping_place_occupancy: base_get_last_step_stopping_place_occupancy,
    get_stopping_place_capacity: base_get_stopping_place_capacity,
    remember_blocked_stopping_place: base_remember_blocked_stopping_place,
    remember_stopping_place_score: base_remember_stopping_place_score,
    reset_stopping_place_scores: base_reset_stopping_place_scores,
    saw_blocked_stopping_place: base_saw_blocked_stopping_place,
    get_number_stopping_place_reroutes: base_get_number_stopping_place_reroutes,
    set_number_stopping_place_reroutes: base_set_number_stopping_place_reroutes,
    evaluate_custom_components: MSStoppingPlaceRerouter::evaluate_custom_components_base,
    valid_component_values: MSStoppingPlaceRerouter::valid_component_values_base,
    use_stopping_place: MSStoppingPlaceRerouter::use_stopping_place_base,
    collect_weights: MSStoppingPlaceRerouter::collect_weights_base,
    get_router: MSStoppingPlaceRerouter::get_router_base,
};

/// C++ class `MSStoppingPlaceRerouter` (microsim/trigger/MSStoppingPlaceRerouter.h)
pub struct MSStoppingPlaceRerouter {
    pub myParamPrefix: String,
    pub myCheckValidity: bool,
    pub myEvalParams: MSStoppingPlaceRerouter_StoppingPlaceParamMap_t,
    pub myNormParams: MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t,
    pub myInvertParams: MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t,
    /// Virtual dispatch table; installed by the most derived constructor
    pub vt: &'static MSStoppingPlaceRerouterVt,
}

impl MSStoppingPlaceRerouter {
    /// C++: `void MSStoppingPlaceRerouter::MSStoppingPlaceRerouter(std::string, bool, StoppingPlaceParamMap_t, StoppingPlaceParamSwitchMap_t)`
    /// Constructor
    // MSStoppingPlaceRerouter::MSStoppingPlaceRerouter(std::string paramPrefix, bool checkValidity, StoppingPlaceParamMap_t addEvalParams, StoppingPlaceParamSwitchMap_t addInvertParams) :
    //     myParamPrefix(paramPrefix), myCheckValidity(checkValidity) {
    //     myEvalParams = { {"probability", 0.}, {"capacity", 0.}, {"timefrom", 0.}, {"timeto", 0.}, {"distancefrom", 0.}, {"distanceto", 1.}, {"absfreespace", 0.}, {"relfreespace", 0.}, };
    //     myInvertParams = { {"probability", false}, { "capacity", true }, { "timefrom", false }, { "timeto", false }, { "distancefrom", false }, { "distanceto", false }, { "absfreespace", true }, { "relfreespace", true } };
    //     for (auto param : addEvalParams) {
    //         myEvalParams[param.first] = param.second;
    //         myInvertParams[param.first] = (addInvertParams.count(param.first) > 0) ? addInvertParams[param.first] : false;
    //     }
    //     for (auto param : myEvalParams) {
    //         myNormParams.insert({param.first, param.first != "probability"});
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(paramPrefix: String, checkValidity: bool, addEvalParams: MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, addInvertParams: MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t) -> Self {
        let mut this = MSStoppingPlaceRerouter {
            myParamPrefix: paramPrefix,
            myCheckValidity: checkValidity,
            myEvalParams: BTreeMap::new(),
            myNormParams: BTreeMap::new(),
            myInvertParams: BTreeMap::new(),
            vt: &MS_STOPPING_PLACE_REROUTER_BASE_VT,
        };
        this.myEvalParams = [
            ("probability".to_string(), 0.),
            ("capacity".to_string(), 0.),
            ("timefrom".to_string(), 0.),
            ("timeto".to_string(), 0.),
            ("distancefrom".to_string(), 0.),
            ("distanceto".to_string(), 1.),
            ("absfreespace".to_string(), 0.),
            ("relfreespace".to_string(), 0.),
        ].into_iter().collect();
        this.myInvertParams = [
            ("probability".to_string(), false),
            ("capacity".to_string(), true),
            ("timefrom".to_string(), false),
            ("timeto".to_string(), false),
            ("distancefrom".to_string(), false),
            ("distanceto".to_string(), false),
            ("absfreespace".to_string(), true),
            ("relfreespace".to_string(), true),
        ].into_iter().collect();
        for (key, value) in addEvalParams {
            // addInvertParams.count(key) > 0 ? addInvertParams[key] : false
            let invert = addInvertParams.get(&key).copied().unwrap_or(false);
            this.myEvalParams.insert(key.clone(), value);
            this.myInvertParams.insert(key, invert);
        }
        for key in this.myEvalParams.keys() {
            this.myNormParams.insert(key.clone(), key != "probability");
        }
        this
    }

    /// C++: `void MSStoppingPlaceRerouter::~MSStoppingPlaceRerouter() noexcept` [virtual]
    // virtual ~MSStoppingPlaceRerouter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor body is empty
    }

    /// C++: `MSStoppingPlace * MSStoppingPlaceRerouter::rerouteStoppingPlace(MSStoppingPlace *, const std::vector<StoppingPlaceVisible> &, const std::vector<double> &, SUMOVehicle &, bool &, ConstMSEdgeVector &, StoppingPlaceParamMap_t &, const Prohibitions &, const int, const bool)`
    /// main method to trigger the rerouting to the "best" StoppingPlace according to the custom evaluation function
    // MSStoppingPlace*
    // MSStoppingPlaceRerouter::rerouteStoppingPlace(MSStoppingPlace* destStoppingPlace, const std::vector<StoppingPlaceVisible>& stoppingPlaceCandidates, const std::vector<double>& probs, SUMOVehicle& veh, bool& newDestination, ConstMSEdgeVector& newRoute, StoppingPlaceParamMap_t& scores,
    //         const Prohibitions& closedEdges, const int insertStopIndex, const bool keepCurrentStop) {
    //     // Reroute destination from initial stopping place to an alternative stopping place
    //     // if the following conditions are met:
    //     // - next stop target is a stopping place of the right type
    //     // - target is included in the current alternative set
    //     // - target is visibly full
    //     // Any stopping places that are visibly full at the current location are
    //     // committed to the stopping place memory corresponding to their type
    //
    //     MSStoppingPlace* nearStoppingPlace = nullptr;
    //
    //     // get vehicle params
    //     bool destVisible = false;
    //     if (destStoppingPlace != nullptr) {
    //         destVisible = (&destStoppingPlace->getLane().getEdge() == veh.getEdge());
    //         // if the vehicle is on the destination stop edge it is always visible
    //         for (auto stoppingPlace : stoppingPlaceCandidates) {
    //             if (stoppingPlace.first == destStoppingPlace && stoppingPlace.second) {
    //                 destVisible = true;
    //                 break;
    //             }
    //         }
    //     }
    //     const MSRoute& route = veh.getRoute();
    //
    //     MSStoppingPlace* onTheWay = nullptr;
    //     const int stopAnywhere = (int)getWeight(veh, "anywhere", -1);
    //     const bool ignoreDest = getWeight(veh, "ignoreDest", destStoppingPlace != nullptr ? 0 : 1) != 0;
    //     // check whether we are ready to accept any free stopping place along the
    //     // way to our destination
    //     if (stopAnywhere < 0 || stopAnywhere > getNumberStoppingPlaceReroutes(veh)) {
    //         if (!destVisible) {
    //             // cannot determine destination occupancy, only register visibly full
    //             for (const StoppingPlaceVisible& stoppingPlace : stoppingPlaceCandidates) {
    //                 if (stoppingPlace.second && getLastStepStoppingPlaceOccupancy(stoppingPlace.first, &veh) >= getStoppingPlaceCapacity(stoppingPlace.first)) {
    //                     rememberStoppingPlaceScore(veh, stoppingPlace.first, "occupied");
    //                     rememberBlockedStoppingPlace(veh, stoppingPlace.first, &stoppingPlace.first->getLane().getEdge() == veh.getEdge());
    //                 }
    //             }
    // #ifdef DEBUG_STOPPINGPLACE
    //             if (DEBUGCOND) {
    //                 //std::cout << SIMTIME << " veh=" << veh.getID() << " dest=" << ((destStoppingPlace == nullptr)? "null" : destStoppingPlace->getID()) << " stopAnywhere=" << stopAnywhere << " reroutes=" << getNumberStoppingPlaceReroutes(veh) << " stay on original route\n";
    //             }
    // #endif
    //         }
    //     } else {
    //         double bestDist = std::numeric_limits<double>::max();
    //         const double brakeGap = veh.getBrakeGap(true);
    //         for (const StoppingPlaceVisible& item : stoppingPlaceCandidates) {
    //             if (item.second) {
    //                 if (&item.first->getLane().getEdge() == veh.getEdge()
    //                         && getLastStepStoppingPlaceOccupancy(item.first, &veh) < getStoppingPlaceCapacity(item.first)) {
    //                     const double distToStart = item.first->getBeginLanePosition() - veh.getPositionOnLane();
    //                     const double distToEnd = item.first->getEndLanePosition() - veh.getPositionOnLane();
    //                     if (distToEnd > brakeGap) {
    //                         rememberStoppingPlaceScore(veh, item.first, "dist=" + toString(distToStart));
    //                         if (distToStart < bestDist) {
    //                             bestDist = distToStart;
    //                             onTheWay = item.first;
    //                         }
    //                     } else {
    //                         rememberStoppingPlaceScore(veh, item.first, "tooClose");
    //                     }
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_STOPPINGPLACE
    //         if (DEBUGCOND) {
    //             std::cout << SIMTIME << " veh=" << veh.getID()
    //                       << " dest=" << ((destStoppingPlace == nullptr) ? "null" : destStoppingPlace->getID()) << " stopAnywhere=" << stopAnywhere << " reroutes=" << getNumberStoppingPlaceReroutes(veh) << " alongTheWay=" << Named::getIDSecure(onTheWay) << "\n";
    //         }
    // #endif
    //     }
    //     if (!ignoreDest && !destVisible && onTheWay == nullptr) {
    //         return nullptr;
    //     }
    //
    //     const bool destIsFull = destStoppingPlace != nullptr && getLastStepStoppingPlaceOccupancy(destStoppingPlace, &veh) >= getStoppingPlaceCapacity(destStoppingPlace);
    //     if (ignoreDest || destIsFull || onTheWay != nullptr) {
    //         // if the current route ends at the stopping place, the new route will
    //         // also end at the new stopping place
    //         newDestination = (destStoppingPlace != nullptr && &destStoppingPlace->getLane().getEdge() == route.getLastEdge()
    //                           && veh.getArrivalPos() >= destStoppingPlace->getBeginLanePosition()
    //                           && veh.getArrivalPos() <= destStoppingPlace->getEndLanePosition()
    //                           && veh.getStops().size() == 1);
    //
    // #ifdef DEBUG_STOPPINGPLACE
    //         if (DEBUGCOND) {
    //             std::cout << SIMTIME << " veh=" << veh.getID()
    //                       << " newDest=" << newDestination
    //                       << " destIsFull=" << destIsFull
    //                       << " onTheWay=" << Named::getIDSecure(onTheWay)
    //                       << "\n";
    //         }
    // #endif
    //         std::map<MSStoppingPlace*, ConstMSEdgeVector> newRoutes;
    //         std::map<MSStoppingPlace*, ConstMSEdgeVector> stopApproaches;
    //         StoppingPlaceParamMap_t weights = collectWeights(veh); // add option to patch values for interdependent values
    //         StoppingPlaceParamMap_t maxValues;
    //         for (auto param : weights) {
    //             maxValues[param.first] = 0.;
    //         }
    //
    //         // a map stores elegible stopping places
    //         StoppingPlaceMap_t stoppingPlaces;
    //         SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = getRouter(veh, closedEdges);
    //         const double brakeGap = veh.getBrakeGap();
    //
    //         if (onTheWay != nullptr) {
    //             // compute new route
    //             if (newDestination) {
    //                 newRoute.push_back(veh.getEdge());
    //             } else {
    //                 bool valid = evaluateDestination(veh, brakeGap, newDestination, onTheWay, getLastStepStoppingPlaceOccupancy(onTheWay, &veh), 1, router, stoppingPlaces, newRoutes, stopApproaches, maxValues, scores, insertStopIndex, keepCurrentStop);
    //                 if (!valid) {
    //                     WRITE_WARNINGF(TL("Stopping place '%' along the way cannot be used by vehicle '%' for unknown reason"), onTheWay->getID(), veh.getID());
    //                     return nullptr;
    //                 }
    //                 newRoute = newRoutes[onTheWay];
    //             }
    //             return onTheWay;
    //         }
    //         int numAlternatives = 0;
    //         std::vector<std::tuple<SUMOTime, MSStoppingPlace*, int>> blockedTimes;
    //         resetStoppingPlaceScores(veh);
    //
    //         if (destStoppingPlace != nullptr && destIsFull) {
    //             rememberStoppingPlaceScore(veh, destStoppingPlace, "occupied");
    //             rememberBlockedStoppingPlace(veh, destStoppingPlace, &destStoppingPlace->getLane().getEdge() == veh.getEdge());
    //         }
    //         const SUMOTime stoppingPlaceMemory = TIME2STEPS(getWeight(veh, "memory", 600));
    //         const double stoppingPlaceFrustration = getWeight(veh, "frustration", 100);
    //         const double stoppingPlaceKnowledge = getWeight(veh, "knowledge", 0);
    //
    //         for (int i = 0; i < (int)stoppingPlaceCandidates.size(); ++i) {
    //             // alternative occupancy is randomized (but never full) if invisible
    //             // current destination must be visible at this point
    //             if (!useStoppingPlace(stoppingPlaceCandidates[i].first)) {
    //                 continue;
    //             }
    //             const bool visible = stoppingPlaceCandidates[i].second || (stoppingPlaceCandidates[i].first == destStoppingPlace && destVisible);
    //             double occupancy = getStoppingPlaceOccupancy(stoppingPlaceCandidates[i].first, &veh);
    //             if (!visible && (stoppingPlaceKnowledge == 0 || stoppingPlaceKnowledge < RandHelper::rand(veh.getRNG()))) {
    //                 double capacity = getStoppingPlaceCapacity(stoppingPlaceCandidates[i].first);
    //                 const double minOccupancy = MIN2(capacity - NUMERICAL_EPS, (getNumberStoppingPlaceReroutes(veh) * capacity / stoppingPlaceFrustration));
    //                 occupancy = RandHelper::rand(minOccupancy, capacity);
    //                 // previously visited?
    //                 SUMOTime blockedTime = sawBlockedStoppingPlace(veh, stoppingPlaceCandidates[i].first, false);
    //                 if (blockedTime >= 0 && SIMSTEP - blockedTime < stoppingPlaceMemory) {
    //                     // assume it's still occupied
    //                     occupancy = capacity;
    //                     blockedTimes.push_back(std::make_tuple(blockedTime, stoppingPlaceCandidates[i].first, i));
    // #ifdef DEBUG_STOPPINGPLACE
    //                     if (DEBUGCOND) {
    //                         std::cout << "    altStoppingPlace=" << stoppingPlaceCandidates[i].first->getID() << " was blocked at " << time2string(blockedTime) << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //             if (occupancy < getStoppingPlaceCapacity(stoppingPlaceCandidates[i].first)) {
    //                 if (evaluateDestination(veh, brakeGap, newDestination, stoppingPlaceCandidates[i].first, occupancy, probs[i], router, stoppingPlaces, newRoutes, stopApproaches, maxValues, scores, insertStopIndex, keepCurrentStop)) {
    //                     numAlternatives++;
    //                 }
    //             } else if (visible) {
    //                 // might only be visible now (i.e. because it's on the other
    //                 // side of the street), so we should remember this for later.
    //                 rememberStoppingPlaceScore(veh, stoppingPlaceCandidates[i].first, "occupied");
    //                 rememberBlockedStoppingPlace(veh, stoppingPlaceCandidates[i].first, &stoppingPlaceCandidates[i].first->getLane().getEdge() == veh.getEdge());
    //             }
    //         }
    //
    //         if (numAlternatives == 0) {
    //             // use parkingArea with lowest blockedTime
    //             std::sort(blockedTimes.begin(), blockedTimes.end(),
    //             [](std::tuple<SUMOTime, MSStoppingPlace*, int> const & t1, std::tuple<SUMOTime, MSStoppingPlace*, int> const & t2) {
    //                 if (std::get<0>(t1) < std::get<0>(t2)) {
    //                     return true;
    //                 }
    //                 if (std::get<0>(t1) == std::get<0>(t2)) {
    //                     if (std::get<1>(t1)->getID() < std::get<1>(t2)->getID()) {
    //                         return true;
    //                     }
    //                     if (std::get<1>(t1)->getID() == std::get<1>(t2)->getID()) {
    //                         return std::get<2>(t1) < std::get<2>(t2);
    //                     }
    //                 }
    //                 return false;
    //             }
    //                      );
    //             for (auto item : blockedTimes) {
    //                 MSStoppingPlace* sp = std::get<1>(item);
    //                 double prob = probs[std::get<2>(item)];
    //                 // all stopping places are occupied. We have no good basis for
    //                 // prefering one or the other based on estimated occupancy
    //                 double occupancy = RandHelper::rand(getStoppingPlaceCapacity(sp));
    //                 if (evaluateDestination(veh, brakeGap, newDestination, sp, occupancy, prob, router, stoppingPlaces, newRoutes, stopApproaches, maxValues, scores, insertStopIndex, keepCurrentStop)) {
    // #ifdef DEBUG_STOPPINGPLACE
    //                     if (DEBUGCOND) {
    //                         std::cout << "    altStoppingPlace=" << sp->getID() << " targeting occupied stopping place based on blockTime " << STEPS2TIME(std::get<0>(item)) << " among " << blockedTimes.size() << " alternatives\n";
    //                     }
    // #endif
    //                     numAlternatives = 1;
    //                     break;
    //                 }
    //                 //std::cout << "  candidate=" << item.second->getID() << " observed=" << time2string(item.first) << "\n";
    //             }
    //             if (numAlternatives == 0) {
    //                 // take any random target but prefer one that hasn't been visited yet
    //                 std::vector<std::pair<SUMOTime, MSStoppingPlace*>> candidates;
    //                 for (const StoppingPlaceVisible& stoppingPlaceCandidate : stoppingPlaceCandidates) {
    //                     if (stoppingPlaceCandidate.first == destStoppingPlace) {
    //                         continue;
    //                     }
    //                     SUMOTime dummy = sawBlockedStoppingPlace(veh, stoppingPlaceCandidate.first, true);
    //                     if (dummy < 0) {
    //                         // randomize among the unvisited
    //                         dummy = -RandHelper::rand(1000000);
    //                     }
    //                     candidates.push_back(std::make_pair(dummy, stoppingPlaceCandidate.first));
    //                 }
    //                 std::sort(candidates.begin(), candidates.end(),
    //                 [](std::tuple<SUMOTime, MSStoppingPlace*> const & t1, std::tuple<SUMOTime, MSStoppingPlace*> const & t2) {
    //                     return std::get<0>(t1) < std::get<0>(t2) || (std::get<0>(t1) == std::get<0>(t2) && std::get<1>(t1)->getID() < std::get<1>(t2)->getID());
    //                 }
    //                          );
    //                 for (auto item : candidates) {
    //                     if (evaluateDestination(veh, brakeGap, newDestination, item.second, 0, 1, router, stoppingPlaces, newRoutes, stopApproaches, maxValues, scores, insertStopIndex, keepCurrentStop)) {
    // #ifdef DEBUG_STOPPINGPLACE
    //                         if (DEBUGCOND) {
    //                             std::cout << "    altStoppingPlace=" << item.second->getID() << " targeting occupied stopping place (based on pure randomness) among " << candidates.size() << " alternatives\n";
    //                         }
    // #endif
    //                         numAlternatives = 1;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         getRouter(veh); // reset closed edges
    //
    // #ifdef DEBUG_STOPPINGPLACE
    //         if (DEBUGCOND) {
    //             std::cout << "  maxValues=" << joinToString(maxValues, " ", ":") << "\n";
    //         }
    // #endif
    //
    //         // minimum cost to get the parking area
    //         double minStoppingPlaceCost = 0.0;
    //
    //         for (StoppingPlaceMap_t::iterator it = stoppingPlaces.begin(); it != stoppingPlaces.end(); ++it) {
    //             // get the parking values
    //             StoppingPlaceParamMap_t stoppingPlaceValues = it->second;
    //
    //             if (weights["probability"] > 0. && maxValues["probability"] > 0.) {
    //                 // random search should not drive past a usable parking area
    //                 bool dominated = false;
    //                 double endPos = it->first->getEndLanePosition();
    //                 const ConstMSEdgeVector& to1 = stopApproaches[it->first];
    //                 assert(to1.size() > 0);
    //                 for (auto altSp : stoppingPlaces) {
    //                     if (altSp.first == it->first) {
    //                         continue;
    //                     }
    //                     const ConstMSEdgeVector& to2 = stopApproaches[altSp.first];
    //                     assert(to2.size() > 0);
    //                     if (to1.size() > to2.size()) {
    //                         if (std::equal(to2.begin(), to2.end(), to1.begin())) {
    //                             // other target lies on the route to the current candidate
    //                             dominated = true;
    //                             //std::cout << SIMTIME << " rrP veh=" << veh.getID() << " full=" << destParkArea->getID() << " cand=" << it->first->getID() << " onTheWay=" << altPa.first->getID() << "\n";
    //                             break;
    //                         }
    //                     } else if (to1 == to2 && endPos > altSp.first->getEndLanePosition()) {
    //                         // other target is on the same edge but ahead of the current candidate
    //                         dominated = true;
    //                         //std::cout << SIMTIME << " rrP veh=" << veh.getID() << " full=" << destParkArea->getID() << " cand=" << it->first->getID() << " sameEdge=" << altPa.first->getID() << "\n";
    //                         break;
    //                     }
    //                 }
    //                 double prob = 0;
    //                 if (!dominated) {
    //                     prob = RandHelper::rand(stoppingPlaceValues["probability"], veh.getRNG());
    //                     stoppingPlaceValues["probability"] = 1.0 - prob / maxValues["probability"];
    //                 } else {
    //                     // worst probability score
    //                     stoppingPlaceValues["probability"] = 1.0;
    //                 }
    //             } else {
    //                 // value takes no effect due to weight=0
    //                 stoppingPlaceValues["probability"] = 0;
    //             }
    //
    //             // get the parking area cost
    //             double stoppingPlaceCost = getTargetValue(stoppingPlaceValues, maxValues, weights, myNormParams, myInvertParams);
    //             rememberStoppingPlaceScore(veh, it->first, toString(stoppingPlaceCost));
    //
    //             // get the parking area with minimum cost
    //             if (nearStoppingPlace == nullptr || stoppingPlaceCost < minStoppingPlaceCost) {
    //                 minStoppingPlaceCost = stoppingPlaceCost;
    //                 nearStoppingPlace = it->first;
    //                 newRoute = newRoutes[nearStoppingPlace];
    //             }
    // #ifdef DEBUG_STOPPINGPLACE
    //             if (DEBUGCOND) {
    //                 std::cout << "    altStoppingPlace=" << it->first->getID() << " score=" << stoppingPlaceCost << " vals=" << joinToString(stoppingPlaceValues, " ", ":") << "\n";
    //             }
    // #endif
    //         }
    //         // expose the scores of the best solution
    //         if (nearStoppingPlace != nullptr) {
    //             for (auto component : stoppingPlaces[nearStoppingPlace]) {
    //                 scores[component.first] = component.second;
    //             }
    //         }
    //         setNumberStoppingPlaceReroutes(veh, getNumberStoppingPlaceReroutes(veh) + 1);
    //     } else {
    // #ifdef DEBUG_STOPPINGPLACE
    //         if (DEBUGCOND) {
    //             std::cout << SIMTIME << " veh=" << veh.getID() << " dest=" << destStoppingPlace->getID() << " sufficient space\n";
    //         }
    // #endif
    //     }
    //
    // #ifdef DEBUG_STOPPINGPLACE
    //     if (DEBUGCOND) {
    //         std::cout << "  stoppingPlaceResult=" << Named::getIDSecure(nearStoppingPlace) << "\n";
    //     }
    // #endif
    //     return nearStoppingPlace;
    // }
    #[allow(unused_variables)]
    pub fn reroute_stopping_place(&mut self, destStoppingPlace: Ptr<MSStoppingPlace>, stoppingPlaceCandidates: &Vec<MSStoppingPlaceRerouter_StoppingPlaceVisible>, probs: &Vec<f64>, veh: &mut SUMOVehicle, newDestination: &mut bool, newRoute: &mut ConstMSEdgeVector, scores: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, closedEdges: &SUMOAbstractRouter_Prohibitions, insertStopIndex: i32, keepCurrentStop: bool) -> Ptr<MSStoppingPlace> {
        let mut nearStoppingPlace: Ptr<MSStoppingPlace> = Ptr::null();
        let now = MSNet::get_instance().get_current_time_step();
        let vp = Ptr::from_mut(veh);

        // get vehicle params
        let mut destVisible = false;
        if !destStoppingPlace.is_null() {
            destVisible = lane_edge(destStoppingPlace) == vp.get_edge();
            // if the vehicle is on the destination stop edge it is always visible
            for stoppingPlace in stoppingPlaceCandidates.iter() {
                if stoppingPlace.0 == destStoppingPlace && stoppingPlace.1 {
                    destVisible = true;
                    break;
                }
            }
        }

        let route = vp.get_route();
        let mut onTheWay: Ptr<MSStoppingPlace> = Ptr::null();
        let stopAnywhere = self.get_weight(vp.m(), "anywhere".to_string(), -1., false) as i32;
        let ignoreDest = self.get_weight(vp.m(), "ignoreDest".to_string(), if !destStoppingPlace.is_null() { 0. } else { 1. }, false) != 0.;
        // check whether we are ready to accept any free stopping place along the way to our destination
        if stopAnywhere < 0 || stopAnywhere > self.get_number_stopping_place_reroutes(vp.m()) {
            if !destVisible {
                // cannot determine destination occupancy, only register visibly full
                for stoppingPlace in stoppingPlaceCandidates.iter() {
                    if stoppingPlace.1 && self.get_last_step_stopping_place_occupancy(stoppingPlace.0, vp) >= self.get_stopping_place_capacity(stoppingPlace.0) {
                        self.remember_stopping_place_score(vp.m(), stoppingPlace.0, "occupied");
                        self.remember_blocked_stopping_place(vp.m(), stoppingPlace.0, lane_edge(stoppingPlace.0) == vp.get_edge());
                    }
                }
            }
        } else {
            let mut bestDist = f64::MAX;
            let brakeGap = vp.get_brake_gap(true);
            for item in stoppingPlaceCandidates.iter() {
                if item.1 && lane_edge(item.0) == vp.get_edge() && self.get_last_step_stopping_place_occupancy(item.0, vp) < self.get_stopping_place_capacity(item.0) {
                    let distToStart = item.0.get_begin_lane_position() - vp.get_position_on_lane();
                    let distToEnd = item.0.get_end_lane_position() - vp.get_position_on_lane();
                    if distToEnd > brakeGap {
                        self.remember_stopping_place_score(vp.m(), item.0, &format!("dist={}", to_string(&distToStart)));
                        if distToStart < bestDist {
                            bestDist = distToStart;
                            onTheWay = item.0;
                        }
                    } else {
                        self.remember_stopping_place_score(vp.m(), item.0, "tooClose");
                    }
                }
            }
        }
        if !ignoreDest && !destVisible && onTheWay.is_null() {
            return Ptr::null();
        }

        let destIsFull = !destStoppingPlace.is_null() && self.get_last_step_stopping_place_occupancy(destStoppingPlace, vp) >= self.get_stopping_place_capacity(destStoppingPlace);
        if ignoreDest || destIsFull || !onTheWay.is_null() {
            // if the current route ends at the stopping place, the new route will
            // also end at the new stopping place
            *newDestination = !destStoppingPlace.is_null()
                && lane_edge(destStoppingPlace) == route.get_last_edge()
                && vp.get_arrival_pos() >= destStoppingPlace.get_begin_lane_position()
                && vp.get_arrival_pos() <= destStoppingPlace.get_end_lane_position()
                && vp.get_stops().len() == 1;

            let mut newRoutes: BTreeMap<Ptr<MSStoppingPlace>, ConstMSEdgeVector> = BTreeMap::new();
            let mut stopApproaches: BTreeMap<Ptr<MSStoppingPlace>, ConstMSEdgeVector> = BTreeMap::new();
            let mut weights = self.collect_weights(vp.m()); // add option to patch values for interdependent values
            let mut maxValues: ParamMap = weights.keys().map(|k| (k.clone(), 0.)).collect();

            // a map stores elegible stopping places
            let mut stoppingPlaces: MSStoppingPlaceRerouter_StoppingPlaceMap_t = BTreeMap::new();
            let router = self.get_router(vp.m(), closedEdges);
            let brakeGap = vp.get_brake_gap(false);

            if !onTheWay.is_null() {
                // compute new route
                if *newDestination {
                    newRoute.push(vp.get_edge());
                } else {
                    let occ = self.get_last_step_stopping_place_occupancy(onTheWay, vp);
                    let valid = self.evaluate_destination(vp.m(), brakeGap, *newDestination, onTheWay, occ, 1., router.m(), &mut stoppingPlaces, &mut newRoutes, &mut stopApproaches, &mut maxValues, scores, insertStopIndex, keepCurrentStop);
                    if !valid {
                        write_warningf!("Stopping place '%' along the way cannot be used by vehicle '%' for unknown reason", onTheWay.get_id(), vp.get_id());
                        return Ptr::null();
                    }
                    *newRoute = newRoutes.get(&onTheWay).cloned().unwrap_or_default();
                }
                return onTheWay;
            }

            let mut numAlternatives = 0;
            let mut blockedTimes: Vec<(SUMOTime, Ptr<MSStoppingPlace>, usize)> = Vec::new();
            self.reset_stopping_place_scores(vp.m());

            if !destStoppingPlace.is_null() && destIsFull {
                self.remember_stopping_place_score(vp.m(), destStoppingPlace, "occupied");
                self.remember_blocked_stopping_place(vp.m(), destStoppingPlace, lane_edge(destStoppingPlace) == vp.get_edge());
            }
            let stoppingPlaceMemory = utils::common::time::time2steps(self.get_weight(vp.m(), "memory".to_string(), 600., false));
            let stoppingPlaceFrustration = self.get_weight(vp.m(), "frustration".to_string(), 100., false);
            let stoppingPlaceKnowledge = self.get_weight(vp.m(), "knowledge".to_string(), 0., false);

            for i in 0..stoppingPlaceCandidates.len() {
                // alternative occupancy is randomized (but never full) if invisible
                // current destination must be visible at this point
                let cand = stoppingPlaceCandidates[i];
                if !self.use_stopping_place(cand.0) {
                    continue;
                }
                let visible = cand.1 || (cand.0 == destStoppingPlace && destVisible);
                let mut occupancy = self.get_stopping_place_occupancy(cand.0, vp);
                if !visible && (stoppingPlaceKnowledge == 0. || stoppingPlaceKnowledge < vp.get_rng().m().rand()) {
                    let capacity = self.get_stopping_place_capacity(cand.0);
                    let minOccupancy = (capacity - NUMERICAL_EPS).min(self.get_number_stopping_place_reroutes(vp.m()) as f64 * capacity / stoppingPlaceFrustration);
                    occupancy = minOccupancy + (capacity - minOccupancy) * utils::common::rand_helper::rand();
                    // previously visited?
                    let blockedTime = self.saw_blocked_stopping_place(vp.m(), cand.0, false);
                    if blockedTime >= 0 && now - blockedTime < stoppingPlaceMemory {
                        // assume it's still occupied
                        occupancy = capacity;
                        blockedTimes.push((blockedTime, cand.0, i));
                    }
                }
                if occupancy < self.get_stopping_place_capacity(cand.0) {
                    if self.evaluate_destination(vp.m(), brakeGap, *newDestination, cand.0, occupancy, probs[i], router.m(), &mut stoppingPlaces, &mut newRoutes, &mut stopApproaches, &mut maxValues, scores, insertStopIndex, keepCurrentStop) {
                        numAlternatives += 1;
                    }
                } else if visible {
                    // might only be visible now (i.e. because it's on the other
                    // side of the street), so we should remember this for later.
                    self.remember_stopping_place_score(vp.m(), cand.0, "occupied");
                    self.remember_blocked_stopping_place(vp.m(), cand.0, lane_edge(cand.0) == vp.get_edge());
                }
            }

            if numAlternatives == 0 {
                // use parkingArea with lowest blockedTime
                blockedTimes.sort_by(|t1, t2| t1.0.cmp(&t2.0).then_with(|| t1.1.get_id().cmp(t2.1.get_id())).then_with(|| t1.2.cmp(&t2.2)));
                for item in blockedTimes.clone() {
                    let sp = item.1;
                    let prob = probs[item.2];
                    // all stopping places are occupied. We have no good basis for
                    // prefering one or the other based on estimated occupancy
                    let occupancy = self.get_stopping_place_capacity(sp) * utils::common::rand_helper::rand();
                    if self.evaluate_destination(vp.m(), brakeGap, *newDestination, sp, occupancy, prob, router.m(), &mut stoppingPlaces, &mut newRoutes, &mut stopApproaches, &mut maxValues, scores, insertStopIndex, keepCurrentStop) {
                        numAlternatives = 1;
                        break;
                    }
                }
            }
            if numAlternatives == 0 {
                // take any random target but prefer one that hasn't been visited yet
                let mut candidates: Vec<(SUMOTime, Ptr<MSStoppingPlace>)> = Vec::new();
                for stoppingPlaceCandidate in stoppingPlaceCandidates.iter() {
                    if stoppingPlaceCandidate.0 == destStoppingPlace {
                        continue;
                    }
                    let mut dummy = self.saw_blocked_stopping_place(vp.m(), stoppingPlaceCandidate.0, true);
                    if dummy < 0 {
                        // randomize among the unvisited
                        dummy = -(utils::common::rand_helper::with_default(|r| r.rand_int(1000000)) as SUMOTime);
                    }
                    candidates.push((dummy, stoppingPlaceCandidate.0));
                }
                candidates.sort_by(|t1, t2| t1.0.cmp(&t2.0).then_with(|| t1.1.get_id().cmp(t2.1.get_id())));
                for item in candidates.iter() {
                    if self.evaluate_destination(vp.m(), brakeGap, *newDestination, item.1, 0., 1., router.m(), &mut stoppingPlaces, &mut newRoutes, &mut stopApproaches, &mut maxValues, scores, insertStopIndex, keepCurrentStop) {
                        numAlternatives = 1;
                        break;
                    }
                }
            }
            // reset closed edges
            self.get_router(vp.m(), &Default::default());

            // minimum cost to get the parking area
            let mut minStoppingPlaceCost = 0.0;
            // determinism: C++ orders stoppingPlaces by ComparatorIdLess; here the BTreeMap is ordered by pointer address
            let entries: Vec<(Ptr<MSStoppingPlace>, ParamMap)> = stoppingPlaces.iter().map(|(k, v)| (*k, v.clone())).collect();
            for (sp, mut stoppingPlaceValues) in entries {
                // get the parking values
                if *param_mut(&mut weights, "probability") > 0. && *param_mut(&mut maxValues, "probability") > 0. {
                    // random search should not drive past a usable parking area
                    let mut dominated = false;
                    let endPos = sp.get_end_lane_position();
                    let to1 = stopApproaches.get(&sp).cloned().unwrap_or_default();
                    for altSp in stoppingPlaces.keys() {
                        if *altSp == sp {
                            continue;
                        }
                        let to2 = stopApproaches.get(altSp).cloned().unwrap_or_default();
                        if to1.len() > to2.len() {
                            if to1[..to2.len()] == to2[..] {
                                // other target lies on the route to the current candidate
                                dominated = true;
                                break;
                            }
                        } else if to1 == to2 && endPos > altSp.get_end_lane_position() {
                            // other target is on the same edge but ahead of the current candidate
                            dominated = true;
                            break;
                        }
                    }
                    if !dominated {
                        let p = *param_mut(&mut stoppingPlaceValues, "probability");
                        let drawn = p * vp.get_rng().m().rand();
                        *param_mut(&mut stoppingPlaceValues, "probability") = 1.0 - drawn / *param_mut(&mut maxValues, "probability");
                    } else {
                        // worst probability score
                        *param_mut(&mut stoppingPlaceValues, "probability") = 1.0;
                    }
                } else {
                    // value takes no effect due to weight=0
                    *param_mut(&mut stoppingPlaceValues, "probability") = 0.;
                }

                // get the parking area cost
                let stoppingPlaceCost = Self::get_target_value(&stoppingPlaceValues, &maxValues, &weights, &self.myNormParams, &self.myInvertParams);
                self.remember_stopping_place_score(vp.m(), sp, &to_string(&stoppingPlaceCost));

                // get the parking area with minimum cost
                if nearStoppingPlace.is_null() || stoppingPlaceCost < minStoppingPlaceCost {
                    minStoppingPlaceCost = stoppingPlaceCost;
                    nearStoppingPlace = sp;
                    *newRoute = newRoutes.get(&nearStoppingPlace).cloned().unwrap_or_default();
                }
            }

            // expose the scores of the best solution
            if !nearStoppingPlace.is_null() {
                if let Some(m) = stoppingPlaces.get(&nearStoppingPlace) {
                    for (k, v) in m.iter() {
                        scores.insert(k.clone(), *v);
                    }
                }
            }
            let reroutes = self.get_number_stopping_place_reroutes(vp.m());
            self.set_number_stopping_place_reroutes(vp.m(), reroutes + 1);
        }
        nearStoppingPlace
    }

    /// C++: `bool MSStoppingPlaceRerouter::evaluateDestination(SUMOVehicle &, double, bool, MSStoppingPlace *, double, double, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, StoppingPlaceMap_t &, std::map<MSStoppingPlace *, ConstMSEdgeVector> &, std::map<MSStoppingPlace *, ConstMSEdgeVector> &, StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &, const int, const bool)` [virtual]
    /// compute the target function for a single alternative
    // bool
    // MSStoppingPlaceRerouter::evaluateDestination(SUMOVehicle& veh, double brakeGap, bool newDestination, MSStoppingPlace* alternative,
    //         double occupancy, double prob, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router, StoppingPlaceMap_t& stoppingPlaces,
    //         std::map<MSStoppingPlace*, ConstMSEdgeVector>& newRoutes, std::map<MSStoppingPlace*, ConstMSEdgeVector>& stoppingPlaceApproaches,
    //         StoppingPlaceParamMap_t& maxValues, StoppingPlaceParamMap_t& addInput, const int insertStopIndex, const bool keepCurrentStop) {
    //
    //     // a map stores the stopping place values
    //     StoppingPlaceParamMap_t stoppingPlaceValues;
    //     const SUMOTime now = SIMSTEP;
    //
    //     const MSRoute& route = veh.getRoute();
    //     const RGBColor& c = route.getColor();
    //     const MSEdge* stoppingPlaceEdge = &(alternative->getLane().getEdge());
    //
    //     const bool includeInternalLengths = MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks();
    //
    //     // Compute the route from the current edge to the stopping place edge
    //     ConstMSEdgeVector edgesToStop;
    //     ConstMSEdgeVector edgesUpstream;
    //     const double targetPos = alternative->getLastFreePos(veh);
    //     MSRouteIterator rerouteOriginIt = determineRerouteOrigin(veh, insertStopIndex);
    //     double posOnLane = veh.getPositionOnLane();
    //     if (insertStopIndex > 0) {
    //         posOnLane = 0.;
    //         // determine preceding edges
    //         for (MSRouteIterator it = veh.getCurrentRouteEdge(); it != rerouteOriginIt; ++it) {
    //             if (it != rerouteOriginIt) {
    //                 edgesUpstream.push_back(*it);
    //             }
    //         }
    //     }
    //     const MSEdge* rerouteOrigin = *rerouteOriginIt;
    //     router.compute(rerouteOrigin, posOnLane, stoppingPlaceEdge, targetPos, &veh, now, edgesToStop, true);
    //     if (edgesToStop.size() > 0) {
    //         // Compute the route from the stopping place edge to the end of the route
    //         if (insertStopIndex == 0 && rerouteOrigin != veh.getEdge()) {
    //             edgesToStop.insert(edgesToStop.begin(), veh.getEdge());
    //         }
    //         // prepend preceding edges
    //         std::reverse(edgesUpstream.begin(), edgesUpstream.end());
    //         for (auto edge : edgesUpstream) {
    //             edgesToStop.insert(edgesToStop.begin(), edge);
    //         }
    //         ConstMSEdgeVector edgesFromStop;
    //         stoppingPlaceApproaches[alternative] = edgesToStop;
    //
    //         const MSEdge* nextDestination = route.getLastEdge();
    //         double nextPos = veh.getArrivalPos();
    //         int nextDestinationIndex = route.size() - 1;
    //         if (!newDestination) {
    //             std::vector<std::pair<int, double> > stopIndices = veh.getStopIndices();
    //             int nextDestStopIndex = 1 + insertStopIndex;
    //             if (!keepCurrentStop) {
    //                 nextDestStopIndex++;
    //             }
    //             if ((int)stopIndices.size() > nextDestStopIndex) {
    //                 nextDestinationIndex = stopIndices[nextDestStopIndex].first;
    //                 nextDestination = route.getEdges()[nextDestinationIndex];
    //                 nextPos = stopIndices[nextDestStopIndex].second;
    //             }
    //             router.compute(stoppingPlaceEdge, targetPos, nextDestination, nextPos, &veh, now, edgesFromStop, true);
    //         }
    //         if (edgesFromStop.size() > 0 || newDestination) {
    //             stoppingPlaceValues["probability"] = prob;
    //             if (stoppingPlaceValues["probability"] > maxValues["probability"]) {
    //                 maxValues["probability"] = stoppingPlaceValues["probability"];
    //             }
    //             stoppingPlaceValues["capacity"] = getStoppingPlaceCapacity(alternative);
    //             stoppingPlaceValues["absfreespace"] = stoppingPlaceValues["capacity"] - occupancy;
    //             // if capacity = 0 then absfreespace and relfreespace are also 0
    //             stoppingPlaceValues["relfreespace"] = stoppingPlaceValues["absfreespace"] / MAX2(1.0, stoppingPlaceValues["capacity"]);
    //             MSRoute routeToPark(route.getID() + "!to" + myParamPrefix + "#1", edgesToStop, false,
    //                                 &c == &RGBColor::DEFAULT_COLOR ? nullptr : new RGBColor(c), route.getStops());
    //
    //             // The distance from the current edge to the new parking area
    //             double toPos = alternative->getBeginLanePosition();
    //             if (&alternative->getLane().getEdge() == veh.getEdge()) {
    //                 toPos = MAX2(veh.getPositionOnLane(), toPos);
    //             }
    //             stoppingPlaceValues["distanceto"] = routeToPark.getDistanceBetween(veh.getPositionOnLane(), toPos,
    //                                                 routeToPark.begin(), routeToPark.end() - 1, includeInternalLengths);
    //
    //             if (stoppingPlaceValues["distanceto"] == std::numeric_limits<double>::max()) {
    //                 WRITE_WARNINGF(TL("Invalid distance computation for vehicle '%' to stopping place '%' at time=%."),
    //                                veh.getID(), alternative->getID(), time2string(now));
    //             }
    //             const double endPos = alternative->getLastFreePos(veh, veh.getPositionOnLane() + brakeGap);
    //             const double distToEnd = stoppingPlaceValues["distanceto"] - toPos + endPos;
    //
    //             if (distToEnd < brakeGap) {
    //                 rememberStoppingPlaceScore(veh, alternative, "tooClose");
    //                 return false;
    //             }
    //
    //             // The time to reach the new stopping place
    //             const double correctionLastEdge = ((alternative->getLane().getLength() - alternative->getEndLanePosition()) / alternative->getLane().getVehicleMaxSpeed(&veh));
    //             const double correctionFirstEdge = veh.getPositionOnLane() / edgesToStop.front()->getVehicleMaxSpeed(&veh);
    //
    //             stoppingPlaceValues["timeto"] = router.recomputeCosts(edgesToStop, &veh, SIMSTEP) - correctionLastEdge - correctionFirstEdge;
    //             ConstMSEdgeVector newEdges = edgesToStop;
    //             if (newDestination) {
    //                 stoppingPlaceValues["distancefrom"] = 0;
    //                 stoppingPlaceValues["timefrom"] = 0;
    //             } else {
    //                 MSRoute routeFromPark(route.getID() + "!from" + myParamPrefix + "#1", edgesFromStop, false,
    //                                       &c == &RGBColor::DEFAULT_COLOR ? nullptr : new RGBColor(c), route.getStops());
    //                 // The distance from the new parking area to the end of the route
    //                 stoppingPlaceValues["distancefrom"] = routeFromPark.getDistanceBetween(alternative->getBeginLanePosition(), routeFromPark.getLastEdge()->getLength(),
    //                                                       routeFromPark.begin(), routeFromPark.end() - 1, includeInternalLengths);
    //                 if (stoppingPlaceValues["distancefrom"] == std::numeric_limits<double>::max()) {
    //                     WRITE_WARNINGF(TL("Invalid distance computation for vehicle '%' from stopping place '%' at time=%."),
    //                                    veh.getID(), alternative->getID(), time2string(SIMSTEP));
    //                 }
    //                 // The time to reach this area
    //                 stoppingPlaceValues["timefrom"] = router.recomputeCosts(edgesFromStop, &veh, SIMSTEP) - (alternative->getEndLanePosition() / alternative->getLane().getSpeedLimit());
    //                 newEdges.insert(newEdges.end(), edgesFromStop.begin() + 1, edgesFromStop.end());
    //                 newEdges.insert(newEdges.end(), route.begin() + nextDestinationIndex + 1, route.end());
    //             }
    //
    //             // add some additional/custom target function components
    //             if (!evaluateCustomComponents(veh, brakeGap, newDestination, alternative, occupancy, prob, router, stoppingPlaceValues, stoppingPlaceApproaches[alternative], newEdges, maxValues, addInput)) {
    //                 return false;
    //             }
    //             if (!myCheckValidity || validComponentValues(stoppingPlaceValues)) {
    //                 updateMaxValues(stoppingPlaceValues, maxValues);
    //                 stoppingPlaces[alternative] = stoppingPlaceValues;
    //                 newRoutes[alternative] = newEdges;
    //                 return true;
    //             } else {
    //                 return false;
    //             }
    //         } else {
    //             rememberStoppingPlaceScore(veh, alternative, "unreachable");
    //         }
    //     } else {
    //         rememberStoppingPlaceScore(veh, alternative, "unreachable");
    //     }
    //     // unreachable
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn evaluate_destination(&mut self, veh: &mut SUMOVehicle, brakeGap: f64, newDestination: bool, alternative: Ptr<MSStoppingPlace>, occupancy: f64, prob: f64, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, stoppingPlaces: &mut MSStoppingPlaceRerouter_StoppingPlaceMap_t, newRoutes: &mut BTreeMap<Ptr<MSStoppingPlace>, ConstMSEdgeVector>, stoppingPlaceApproaches: &mut BTreeMap<Ptr<MSStoppingPlace>, ConstMSEdgeVector>, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, addInput: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, insertStopIndex: i32, keepCurrentStop: bool) -> bool {
        // a map stores the stopping place values
        let mut stoppingPlaceValues: ParamMap = BTreeMap::new();
        let now = MSNet::get_instance().get_current_time_step();
        let vp = Ptr::from_mut(veh);

        let route = vp.get_route();
        let c = route.get_color();
        let stoppingPlaceEdge = lane_edge(alternative);

        let includeInternalLengths = MSGlobals::using_internal_lanes() && MSNet::get_instance().has_internal_links();

        // Compute the route from the current edge to the stopping place edge
        let mut edgesToStop: ConstMSEdgeVector = Vec::new();
        let mut edgesUpstream: ConstMSEdgeVector = Vec::new();
        let targetPos = alternative.get_last_free_pos(vp.m(), 0.);
        let rerouteOriginIdx = self.determine_reroute_origin(vp.m(), insertStopIndex);
        let edges = route.get_edges();
        let mut posOnLane = vp.get_position_on_lane();
        if insertStopIndex > 0 {
            posOnLane = 0.;
            // determine preceding edges
            let cur = route_cursor_index(vp.m());
            for idx in cur..rerouteOriginIdx {
                edgesUpstream.push(edges[idx]);
            }
        }
        let rerouteOrigin = edges[rerouteOriginIdx];
        let _ = router.compute_2(rerouteOrigin, posOnLane, stoppingPlaceEdge, targetPos, vp, now, &mut edgesToStop, true);
        if edgesToStop.len() > 0 {
            // Compute the route from the stopping place edge to the end of the route
            if insertStopIndex == 0 && rerouteOrigin != vp.get_edge() {
                edgesToStop.insert(0, vp.get_edge());
            }
            // prepend preceding edges
            edgesUpstream.reverse();
            for edge in edgesUpstream.iter() {
                edgesToStop.insert(0, *edge);
            }
            let mut edgesFromStop: ConstMSEdgeVector = Vec::new();
            stoppingPlaceApproaches.insert(alternative, edgesToStop.clone());

            let mut nextDestination = route.get_last_edge();
            let mut nextPos = vp.get_arrival_pos();
            let mut nextDestinationIndex = route.size() - 1;
            if !newDestination {
                let stopIndices = vp.get_stop_indices();
                let mut nextDestStopIndex = 1 + insertStopIndex;
                if !keepCurrentStop {
                    nextDestStopIndex += 1;
                }
                if stopIndices.len() as i32 > nextDestStopIndex {
                    nextDestinationIndex = stopIndices[nextDestStopIndex as usize].0;
                    nextDestination = edges[nextDestinationIndex as usize];
                    nextPos = stopIndices[nextDestStopIndex as usize].1;
                }
                let _ = router.compute_2(stoppingPlaceEdge, targetPos, nextDestination, nextPos, vp, now, &mut edgesFromStop, true);
            }
            if edgesFromStop.len() > 0 || newDestination {
                *param_mut(&mut stoppingPlaceValues, "probability") = prob;
                if *param_mut(&mut stoppingPlaceValues, "probability") > *param_mut(maxValues, "probability") {
                    *param_mut(maxValues, "probability") = *param_mut(&mut stoppingPlaceValues, "probability");
                }
                let capacity = self.get_stopping_place_capacity(alternative);
                *param_mut(&mut stoppingPlaceValues, "capacity") = capacity;
                *param_mut(&mut stoppingPlaceValues, "absfreespace") = capacity - occupancy;
                // if capacity = 0 then absfreespace and relfreespace are also 0
                let absfree = *param_mut(&mut stoppingPlaceValues, "absfreespace");
                *param_mut(&mut stoppingPlaceValues, "relfreespace") = absfree / f64::max(1.0, capacity);
                let routeToPark = MSRoute::new(&format!("{}!to{}#1", route.base.get_id(), self.myParamPrefix), &edgesToStop, false, color_ptr(c), route.get_stops(), -1, -1);

                // The distance from the current edge to the new parking area
                let mut toPos = alternative.get_begin_lane_position();
                if lane_edge(alternative) == vp.get_edge() {
                    toPos = f64::max(vp.get_position_on_lane(), toPos);
                }
                *param_mut(&mut stoppingPlaceValues, "distanceto") = route_distance_between(&routeToPark, vp.get_position_on_lane(), toPos, 0, routeToPark.get_edges().len() - 1, includeInternalLengths);
                if *param_mut(&mut stoppingPlaceValues, "distanceto") == f64::MAX {
                    write_warningf!("Invalid distance computation for vehicle '%' to stopping place '%' at time=%.", vp.get_id(), alternative.get_id(), time2string(now));
                }
                let endPos = alternative.get_last_free_pos(vp.m(), vp.get_position_on_lane() + brakeGap);
                let distToEnd = *param_mut(&mut stoppingPlaceValues, "distanceto") - toPos + endPos;

                if distToEnd < brakeGap {
                    self.remember_stopping_place_score(vp.m(), alternative, "tooClose");
                    return false;
                }

                // The time to reach the new stopping place
                let correctionLastEdge = (alternative.get_lane().get_length() - alternative.get_end_lane_position()) / alternative.get_lane().get_vehicle_max_speed(vp.cast::<SUMOTrafficObject>());
                let correctionFirstEdge = vp.get_position_on_lane() / edgesToStop[0].get_vehicle_max_speed(vp.cast::<SUMOTrafficObject>());

                *param_mut(&mut stoppingPlaceValues, "timeto") = router.recompute_costs(&edgesToStop, vp, now, Ptr::null()) - correctionLastEdge - correctionFirstEdge;
                let mut newEdges = edgesToStop.clone();
                if newDestination {
                    *param_mut(&mut stoppingPlaceValues, "distancefrom") = 0.;
                    *param_mut(&mut stoppingPlaceValues, "timefrom") = 0.;
                } else {
                    let routeFromPark = MSRoute::new(&format!("{}!from{}#1", route.base.get_id(), self.myParamPrefix), &edgesFromStop, false, color_ptr(c), route.get_stops(), -1, -1);
                    // The distance from the new parking area to the end of the route
                    *param_mut(&mut stoppingPlaceValues, "distancefrom") = route_distance_between(&routeFromPark, alternative.get_begin_lane_position(), routeFromPark.get_last_edge().get_length(), 0, routeFromPark.get_edges().len() - 1, includeInternalLengths);
                    if *param_mut(&mut stoppingPlaceValues, "distancefrom") == f64::MAX {
                        write_warningf!("Invalid distance computation for vehicle '%' from stopping place '%' at time=%.", vp.get_id(), alternative.get_id(), time2string(now));
                    }
                    // The time to reach this area
                    *param_mut(&mut stoppingPlaceValues, "timefrom") = router.recompute_costs(&edgesFromStop, vp, now, Ptr::null()) - (alternative.get_end_lane_position() / alternative.get_lane().get_speed_limit());
                    newEdges.extend(edgesFromStop.iter().skip(1).cloned());
                    newEdges.extend(edges[(nextDestinationIndex + 1) as usize..].iter().cloned());
                }

                // add some additional/custom target function components
                let approach = stoppingPlaceApproaches.entry(alternative).or_default();
                if !self.evaluate_custom_components(vp.m(), brakeGap, newDestination, alternative, occupancy, prob, router, &mut stoppingPlaceValues, &mut newEdges, approach, maxValues, addInput) {
                    return false;
                }
                if !self.myCheckValidity || self.valid_component_values(&mut stoppingPlaceValues) {
                    Self::update_max_values(&mut stoppingPlaceValues, maxValues);
                    stoppingPlaces.insert(alternative, stoppingPlaceValues);
                    newRoutes.insert(alternative, newEdges);
                    return true;
                } else {
                    return false;
                }
            } else {
                self.remember_stopping_place_score(vp.m(), alternative, "unreachable");
            }
        } else {
            self.remember_stopping_place_score(vp.m(), alternative, "unreachable");
        }
        // unreachable
        false
    }

    /// C++: `bool MSStoppingPlaceRerouter::evaluateCustomComponents(SUMOVehicle &, double, bool, MSStoppingPlace *, double, double, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, StoppingPlaceParamMap_t &, ConstMSEdgeVector &, ConstMSEdgeVector &, StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &)` [virtual]
    /// Compute some custom target function components
    // bool
    // MSStoppingPlaceRerouter::evaluateCustomComponents(SUMOVehicle& /*veh*/, double /*brakeGap*/, bool /*newDestination*/,
    //         MSStoppingPlace* /*alternative*/, double /*occupancy*/, double /*prob*/, SUMOAbstractRouter<MSEdge, SUMOVehicle>& /*router*/,
    //         StoppingPlaceParamMap_t& /*stoppingPlaceValues*/, ConstMSEdgeVector& /*newRoute*/, ConstMSEdgeVector& /*stoppingPlaceApproach*/,
    //         StoppingPlaceParamMap_t& /*maxValues*/, StoppingPlaceParamMap_t& /*addInput*/) {
    //     return true;
    // }
    /// C++ virtual `evaluateCustomComponents`: dispatches to the derived table
    pub fn evaluate_custom_components(&mut self, veh: &mut SUMOVehicle, brakeGap: f64, newDestination: bool, alternative: Ptr<MSStoppingPlace>, occupancy: f64, prob: f64, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, newRoute: &mut ConstMSEdgeVector, stoppingPlaceApproach: &mut ConstMSEdgeVector, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, addInput: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        (self.vt.evaluate_custom_components)(self, veh, brakeGap, newDestination, alternative, occupancy, prob, router, stoppingPlaceValues, newRoute, stoppingPlaceApproach, maxValues, addInput)
    }

    #[allow(unused_variables)]
    pub fn evaluate_custom_components_base(&mut self, veh: &mut SUMOVehicle, brakeGap: f64, newDestination: bool, alternative: Ptr<MSStoppingPlace>, occupancy: f64, prob: f64, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, newRoute: &mut ConstMSEdgeVector, stoppingPlaceApproach: &mut ConstMSEdgeVector, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, addInput: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        true
    }

    /// C++: `bool MSStoppingPlaceRerouter::validComponentValues(StoppingPlaceParamMap_t &)` [virtual]
    /// Whether the stopping place should be discarded due to its results from the component evaluation (allows to check for min/max thresholds and other non-linear relations)
    // bool
    // MSStoppingPlaceRerouter::validComponentValues(StoppingPlaceParamMap_t& /* stoppingPlaceValues */) {
    //     return true;
    // }
    /// C++ virtual `validComponentValues`: dispatches to the derived table
    pub fn valid_component_values(&mut self, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        (self.vt.valid_component_values)(self, stoppingPlaceValues)
    }

    #[allow(unused_variables)]
    pub fn valid_component_values_base(&mut self, stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) -> bool {
        true
    }

    /// C++: `bool MSStoppingPlaceRerouter::useStoppingPlace(MSStoppingPlace *)` [virtual]
    /// Whether the stopping place should be included in the search (can be used to add an additional filter)
    // bool
    // MSStoppingPlaceRerouter::useStoppingPlace(MSStoppingPlace* /* stoppingPlace */) {
    //     return true;
    // }
    /// C++ virtual `useStoppingPlace`: dispatches to the derived table
    pub fn use_stopping_place(&mut self, stoppingPlace: Ptr<MSStoppingPlace>) -> bool {
        (self.vt.use_stopping_place)(self, stoppingPlace)
    }

    #[allow(unused_variables)]
    pub fn use_stopping_place_base(&mut self, stoppingPlace: Ptr<MSStoppingPlace>) -> bool {
        true
    }

    /// C++: `SUMOAbstractRouter<MSEdge, SUMOVehicle> & MSStoppingPlaceRerouter::getRouter(SUMOVehicle &, const Prohibitions &)` [virtual]
    /// Provide the router to use (MSNet::getRouterTT or MSRoutingEngine)
    // SUMOAbstractRouter<MSEdge, SUMOVehicle>&
    // MSStoppingPlaceRerouter::getRouter(SUMOVehicle& veh, const Prohibitions& prohibited) {
    //     return MSNet::getInstance()->getRouterTT(veh.getRNGIndex(), prohibited);
    // }
    /// C++ virtual `getRouter`: dispatches to the derived table (MSDevice_StationFinder uses MSRoutingEngine).
    pub fn get_router(&mut self, veh: &mut SUMOVehicle, prohibited: &SUMOAbstractRouter_Prohibitions) -> Ptr<SUMOAbstractRouter<MSEdge, SUMOVehicle>> {
        (self.vt.get_router)(self, veh, prohibited)
    }

    #[allow(unused_variables)]
    pub fn get_router_base(&mut self, veh: &mut SUMOVehicle, prohibited: &SUMOAbstractRouter_Prohibitions) -> Ptr<SUMOAbstractRouter<MSEdge, SUMOVehicle>> {
        Ptr::from_mut(MSNet::get_instance().get().unwrap().get_router_tt(veh.get_rng_index(), prohibited))
    }

    /// C++: `double MSStoppingPlaceRerouter::getStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual, pure virtual]
    /// Return the number of occupied places of the StoppingPlace
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stopping_place_occupancy(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        (self.vt.get_stopping_place_occupancy)(self, stoppingPlace, veh)
    }

    /// C++: `double MSStoppingPlaceRerouter::getLastStepStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual, pure virtual]
    /// Return the number of occupied places of the StoppingPlace from the previous time step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_last_step_stopping_place_occupancy(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        (self.vt.get_last_step_stopping_place_occupancy)(self, stoppingPlace, veh)
    }

    /// C++: `double MSStoppingPlaceRerouter::getStoppingPlaceCapacity(MSStoppingPlace *)` [virtual, pure virtual]
    /// Return the number of places the StoppingPlace provides
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stopping_place_capacity(&mut self, stoppingPlace: Ptr<MSStoppingPlace>) -> f64 {
        (self.vt.get_stopping_place_capacity)(self, stoppingPlace)
    }

    /// C++: `void MSStoppingPlaceRerouter::rememberBlockedStoppingPlace(SUMOVehicle &, const MSStoppingPlace *, bool)` [virtual, pure virtual]
    /// store the blocked stopping place in the vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, stoppingPlace: Ptr<MSStoppingPlace>, blocked: bool) {
        (self.vt.remember_blocked_stopping_place)(self, veh, stoppingPlace, blocked)
    }

    /// C++: `void MSStoppingPlaceRerouter::rememberStoppingPlaceScore(SUMOVehicle &, MSStoppingPlace *, const std::string &)` [virtual, pure virtual]
    /// store the stopping place score in the vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_stopping_place_score(&mut self, veh: &mut SUMOVehicle, place: Ptr<MSStoppingPlace>, score: &str) {
        (self.vt.remember_stopping_place_score)(self, veh, place, score)
    }

    /// C++: `void MSStoppingPlaceRerouter::resetStoppingPlaceScores(SUMOVehicle &)` [virtual, pure virtual]
    /// forget all stopping place score for this vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_stopping_place_scores(&mut self, veh: &mut SUMOVehicle) {
        (self.vt.reset_stopping_place_scores)(self, veh)
    }

    /// C++: `SUMOTime MSStoppingPlaceRerouter::sawBlockedStoppingPlace(SUMOVehicle &, MSStoppingPlace *, bool)` [virtual, pure virtual]
    /// ask the vehicle when it has seen the stopping place
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn saw_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, place: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        (self.vt.saw_blocked_stopping_place)(self, veh, place, local)
    }

    /// C++: `int MSStoppingPlaceRerouter::getNumberStoppingPlaceReroutes(SUMOVehicle &)` [virtual, pure virtual]
    /// ask how many times already the vehicle has been rerouted to another stopping place
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle) -> i32 {
        (self.vt.get_number_stopping_place_reroutes)(self, veh)
    }

    /// C++: `void MSStoppingPlaceRerouter::setNumberStoppingPlaceReroutes(SUMOVehicle &, int)` [virtual, pure virtual]
    /// update the number of reroutes for the vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle, value: i32) {
        (self.vt.set_number_stopping_place_reroutes)(self, veh, value)
    }

    /// C++: `StoppingPlaceParamMap_t MSStoppingPlaceRerouter::collectWeights(SUMOVehicle &)` [virtual]
    /// read target function weights for this vehicle
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    /// C++ virtual `collectWeights`: dispatches to the derived table
    pub fn collect_weights(&mut self, veh: &mut SUMOVehicle) -> MSStoppingPlaceRerouter_StoppingPlaceParamMap_t {
        (self.vt.collect_weights)(self, veh)
    }

    #[allow(unused_variables)]
    pub fn collect_weights_base(&mut self, veh: &mut SUMOVehicle) -> MSStoppingPlaceRerouter_StoppingPlaceParamMap_t {
        let mut result: ParamMap = BTreeMap::new();
        let d = *param_mut(&mut self.myEvalParams, "distanceto");
        let w = self.get_weight(veh, "distance.weight".to_string(), d, false);
        self.myEvalParams.insert("distanceto".to_string(), w);
        let evalParams = self.myEvalParams.clone();
        for (k, v) in evalParams.iter() {
            let value = self.get_weight(veh, format!("{}.weight", k), *v, false);
            result.insert(k.clone(), value);
        }
        let p = self.get_weight(veh, "probability.weight".to_string(), 0., false);
        result.insert("probability".to_string(), p);
        result
    }

    /// C++: `double MSStoppingPlaceRerouter::getWeight(SUMOVehicle &, const std::string, const double, const bool)`
    /// read the value of a stopping place search param, e.g. a component weight factor
    // double
    // MSStoppingPlaceRerouter::getWeight(SUMOVehicle& veh, const std::string param, const double defaultWeight, const bool warn) {
    //     // get custom vehicle parameter
    //     const std::string key = myParamPrefix + "." + param;
    //     if (veh.getParameter().hasParameter(key)) {
    //         try {
    //             return StringUtils::toDouble(veh.getParameter().getParameter(key, "-1"));
    //         } catch (...) {
    //             WRITE_WARNINGF(TL("Invalid value '%' for vehicle parameter '%'"), veh.getParameter().getParameter(key, "-1"), key);
    //         }
    //     } else {
    //         // get custom vType parameter
    //         if (veh.getVehicleType().getParameter().hasParameter(key)) {
    //             try {
    //                 return StringUtils::toDouble(veh.getVehicleType().getParameter().getParameter(key, "-1"));
    //             } catch (...) {
    //                 WRITE_WARNINGF(TL("Invalid value '%' for vType parameter '%'"), veh.getVehicleType().getParameter().getParameter(key, "-1"), key);
    //             }
    //         }
    //     }
    //     if (warn) {
    //         WRITE_MESSAGEF("Vehicle '%' does not supply vehicle parameter '%'. Using default of %\n", veh.getID(), key, toString(defaultWeight));
    //     }
    //     return defaultWeight;
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&mut self, veh: &mut SUMOVehicle, param: String, defaultWeight: f64, warn: bool) -> f64 {
        // get custom vehicle parameter
        let key = format!("{}.{}", self.myParamPrefix, param);
        if veh.get_parameter().has_parameter(&key) {
            match to_double(&veh.get_parameter().get_parameter(&key, "-1")) {
                Ok(v) => return v,
                Err(_) => write_warningf!("Invalid value '%' for vehicle parameter '%'", veh.get_parameter().get_parameter(&key, "-1"), key),
            }
        } else {
            // get custom vType parameter
            if veh.get_vehicle_type().get_parameter().has_parameter(&key) {
                match to_double(&veh.get_vehicle_type().get_parameter().get_parameter(&key, "-1")) {
                    Ok(v) => return v,
                    Err(_) => write_warningf!("Invalid value '%' for vType parameter '%'", veh.get_vehicle_type().get_parameter().get_parameter(&key, "-1"), key),
                }
            }
        }
        if warn {
            write_messagef!("Vehicle '%' does not supply vehicle parameter '%'. Using default of %\n", veh.get_id(), key, to_string(&defaultWeight));
        }
        defaultWeight
    }

    /// C++: `void MSStoppingPlaceRerouter::updateMaxValues(StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &)` [static]
    /// keep track of the maximum values of each component
    // void
    // MSStoppingPlaceRerouter::updateMaxValues(StoppingPlaceParamMap_t& stoppingPlaceValues, StoppingPlaceParamMap_t& maxValues) {
    //     for (auto it = maxValues.begin(); it != maxValues.end(); ++it) {
    //         if (stoppingPlaceValues[it->first] > it->second) {
    //             it->second = stoppingPlaceValues[it->first];
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_max_values(stoppingPlaceValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) {
        for (k, max) in maxValues.iter_mut() {
            let v = *param_mut(stoppingPlaceValues, k);
            if v > *max {
                *max = v;
            }
        }
    }

    /// C++: `double MSStoppingPlaceRerouter::getTargetValue(const StoppingPlaceParamMap_t &, const StoppingPlaceParamMap_t &, const StoppingPlaceParamMap_t &, const StoppingPlaceParamSwitchMap_t &, const StoppingPlaceParamSwitchMap_t &)` [static]
    /// compute the scalar target function value by means of a linear combination of all components/weights after normalising and optionally inverting the values
    // double
    // MSStoppingPlaceRerouter::getTargetValue(const StoppingPlaceParamMap_t& absValues, const StoppingPlaceParamMap_t& maxValues, const StoppingPlaceParamMap_t& weights, const StoppingPlaceParamSwitchMap_t& norm, const StoppingPlaceParamSwitchMap_t& invert) {
    //     double cost = 0.;
    //     for (StoppingPlaceParamMap_t::const_iterator sc = absValues.begin(); sc != absValues.end(); ++sc) {
    //         double weight = weights.at(sc->first);
    //         double val = sc->second;
    //         if (norm.at(sc->first) && maxValues.at(sc->first) > 0.) {
    //             val /= maxValues.at(sc->first);
    //         }
    //         cost += (invert.at(sc->first)) ? weight * (1. - val) : weight * val;
    //     }
    //     return cost;
    // }
    #[allow(unused_variables)]
    pub fn get_target_value(absValues: &MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, maxValues: &MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, weights: &MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, norm: &MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t, invert: &MSStoppingPlaceRerouter_StoppingPlaceParamSwitchMap_t) -> f64 {
        let mut cost = 0.;
        for (k, &v0) in absValues.iter() {
            let weight = weights[k];
            let mut val = v0;
            if norm[k] && maxValues[k] > 0. {
                val /= maxValues[k];
            }
            cost += if invert[k] { weight * (1. - val) } else { weight * val };
        }
        cost
    }

    /// C++: `void MSStoppingPlaceRerouter::readEvaluationWeights(SUMOVehicle &, StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &, StoppingPlaceParamMap_t &)` [protected]
    /// Ask the vehicle about the relevant rerouting parameters and initiate the maximum value data structure
    // void readEvaluationWeights(SUMOVehicle& veh, StoppingPlaceParamMap_t& stoppingPlaceParams, StoppingPlaceParamMap_t& stoppingPlaceDefaults, StoppingPlaceParamMap_t& maxValues) {
    //         for (auto& it : stoppingPlaceParams) {
    //             const double value = getWeight(veh, it.first, stoppingPlaceDefaults[it.first]);
    //             it.second = value;
    //             if (value > maxValues[it.first]) {
    //                 maxValues[it.first] = value;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_evaluation_weights(&mut self, veh: &mut SUMOVehicle, stoppingPlaceParams: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, stoppingPlaceDefaults: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t, maxValues: &mut MSStoppingPlaceRerouter_StoppingPlaceParamMap_t) {
        for (k, it) in stoppingPlaceParams.iter_mut() {
            let def = *param_mut(stoppingPlaceDefaults, k);
            let value = self.get_weight(veh, k.clone(), def, false);
            *it = value;
            let m = param_mut(maxValues, k);
            if value > *m {
                *m = value;
            }
        }
    }

    /// C++: `const MSRouteIterator MSStoppingPlaceRerouter::determineRerouteOrigin(SUMOVehicle &, int)` [protected]
    /// Determine the rerouting origin edge (not necessarily the current edge of the vehicle!)
    // const MSRouteIterator determineRerouteOrigin(SUMOVehicle& veh, int insertStopIndex) {
    //         const int stopCount = (int)veh.getStops().size();
    //         if (insertStopIndex == 0 || stopCount == 0) {
    //             return veh.getRerouteOrigin();
    //         }
    //         // if the stop index is too high cap to route end
    //         const int stopIndex = MIN2(insertStopIndex, stopCount - 1);
    //         std::list<MSStop>::const_iterator it = veh.getStops().begin();
    //         std::advance(it, stopIndex);
    //         MSStop relevantStop = *it;
    //         return relevantStop.edge;
    //     }
    #[allow(unused_variables)]
    pub fn determine_reroute_origin(&mut self, veh: &mut SUMOVehicle, insertStopIndex: i32) -> usize {
        let stopCount = veh.get_stops().len() as i32;
        if insertStopIndex == 0 || stopCount == 0 {
            return veh.get_reroute_origin();
        }
        // if the stop index is too high cap to route end
        let stopIndex = insertStopIndex.min(stopCount - 1) as usize;
        veh.get_stops()[stopIndex].edge
    }

    /// C++: `void MSStoppingPlaceRerouter::MSStoppingPlaceRerouter()` [private]
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        panic!("the private default constructor of MSStoppingPlaceRerouter is not callable")
    }

}

/// C++ class `StoppingPlaceMemory` (microsim/trigger/MSStoppingPlaceRerouter.h)
pub struct StoppingPlaceMemory {
    /// The map from StoppingPlace to single evaluation
    pub myMap: StoppingPlaceMemory_StoppingPlaceMap,
}

impl StoppingPlaceMemory {
    /// C++: `void StoppingPlaceMemory::StoppingPlaceMemory()`
    /// Constructor
    // StoppingPlaceMemory() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        StoppingPlaceMemory { myMap: BTreeMap::new() }
    }

    /// C++: `void StoppingPlaceMemory::~StoppingPlaceMemory() noexcept` [virtual]
    // virtual ~StoppingPlaceMemory() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // the C++ destructor body is empty
    }

    /// C++: `bool StoppingPlaceMemory::remove(MSStoppingPlace *)`
    /// Removes an item
    // bool remove(MSStoppingPlace* id) {
    //         auto it = myMap.find(id);
    //         if (it == myMap.end()) {
    //             return false;
    //         } else {
    //             myMap.erase(it);
    //             return true;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn remove(&mut self, id: Ptr<MSStoppingPlace>) -> bool {
        self.myMap.remove(&id).is_some()
    }

    /// C++: `void StoppingPlaceMemory::clear()`
    /// Removes all data about evaluated StoppingPlace items
    // void clear() {
    //         myMap.clear();
    //     }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        self.myMap.clear();
    }

    /// C++: `int StoppingPlaceMemory::size() const` [const]
    /// Returns the number of stored items within the container
    // int size() const {
    //         return (int)myMap.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myMap.len() as i32
    }

    /// C++: `void StoppingPlaceMemory::rememberBlockedStoppingPlace(const MSStoppingPlace *, bool)`
    /// Store the time the StoppingPlace was confirmed to be blocked
    // void rememberBlockedStoppingPlace(const MSStoppingPlace* stoppingPlace, bool local) {
    //         myMap[stoppingPlace].blockedAtTime = SIMSTEP;
    //         if (local) {
    //             myMap[stoppingPlace].blockedAtTimeLocal = SIMSTEP;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn remember_blocked_stopping_place(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, local: bool) {
        let now = MSNet::get_instance().get_current_time_step();
        let entry = self.myMap.entry(stoppingPlace).or_insert_with(StoppingPlaceMemoryEntry::new);
        entry.blockedAtTime = now;
        if local {
            entry.blockedAtTimeLocal = now;
        }
    }

    /// C++: `SUMOTime StoppingPlaceMemory::sawBlockedStoppingPlace(const MSStoppingPlace *, bool) const` [const]
    /// Get the time the StoppingPlace was confirmed to be blocked
    // SUMOTime sawBlockedStoppingPlace(const MSStoppingPlace* stoppingPlace, bool local) const {
    //         auto it = myMap.find(stoppingPlace);
    //         if (it == myMap.end()) {
    //             return -1;
    //         } else {
    //             return local ? it->second.blockedAtTimeLocal : it->second.blockedAtTime;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn saw_blocked_stopping_place(&self, stoppingPlace: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        match self.myMap.get(&stoppingPlace) {
            None => -1,
            Some(it) => if local { it.blockedAtTimeLocal } else { it.blockedAtTime },
        }
    }

    /// C++: `void StoppingPlaceMemory::rememberStoppingPlaceScore(const MSStoppingPlace *, const std::string &)`
    /// score only needed when running with gui
    // void rememberStoppingPlaceScore(const MSStoppingPlace* stoppingPlace, const std::string& score) {
    //         myMap[stoppingPlace].score = score;
    //     }
    #[allow(unused_variables)]
    pub fn remember_stopping_place_score(&mut self, stoppingPlace: Ptr<MSStoppingPlace>, score: &str) {
        self.myMap.entry(stoppingPlace).or_insert_with(StoppingPlaceMemoryEntry::new).score = score.to_string();
    }

    /// C++: `void StoppingPlaceMemory::resetStoppingPlaceScores()`
    // void resetStoppingPlaceScores() {
    //         for (auto& item : myMap) {
    //             item.second.score = "";
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn reset_stopping_place_scores(&mut self) {
        for item in self.myMap.values_mut() {
            item.score = String::new();
        }
    }

    /// C++: `typename StoppingPlaceMap::const_iterator StoppingPlaceMemory::begin() const` [const]
    /// Returns a reference to the begin iterator for the internal map
    // typename StoppingPlaceMap::const_iterator begin() const {
    //         return myMap.begin();
    //     }
    #[allow(unused_variables)]
    pub fn begin(&self) -> Box<dyn Iterator<Item = (&Ptr<MSStoppingPlace>, &StoppingPlaceMemoryEntry)> + '_> {
        Box::new(self.myMap.iter())
    }

    /// C++: `typename StoppingPlaceMap::const_iterator StoppingPlaceMemory::end() const` [const]
    /// Returns a reference to the end iterator for the internal map
    // typename StoppingPlaceMap::const_iterator end() const {
    //         return myMap.end();
    //     }
    #[allow(unused_variables)]
    pub fn end(&self) -> Box<dyn Iterator<Item = (&Ptr<MSStoppingPlace>, &StoppingPlaceMemoryEntry)> + '_> {
        // Rust iterators carry their own end: callers iterate begin() to exhaustion, so end() is the empty rest
        Box::new(std::iter::empty())
    }

}

/// store information for a single stopping place
/// C++ class `StoppingPlaceMemoryEntry` (microsim/trigger/MSStoppingPlaceRerouter.h)
pub struct StoppingPlaceMemoryEntry {
    pub blockedAtTime: SUMOTime,
    pub blockedAtTimeLocal: SUMOTime,
    pub score: String,
}

impl StoppingPlaceMemoryEntry {
    /// C++: `void StoppingPlaceMemoryEntry::StoppingPlaceMemoryEntry()`
    // StoppingPlaceMemoryEntry() : blockedAtTime(-1), blockedAtTimeLocal(-1), score("") {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        StoppingPlaceMemoryEntry { blockedAtTime: -1, blockedAtTimeLocal: -1, score: String::new() }
    }

}

