//! Port of libsumo/Helper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::Cell;
use utils::geom::geom_helper;

// shortcut: C++ `throw TraCIException(msg)` is `panic!("{}", msg)` in this file (the TraCI server catches panics per command).

const FAR_AWAY: f64 = 1000.0; // C++ #define in Helper.cpp
const SUMO_CONST_LANE_WIDTH: f64 = 3.2; // utils/common/StdDefs.h, no Rust const yet
const STOP_INDEX_FIT: i32 = -2; // SUMOVehicleParameter.h
const STOP_START_SET: i64 = 1;
const STOP_END_SET: i64 = 2;
const STOP_DURATION_SET: i64 = 2 << 1;
const STOP_UNTIL_SET: i64 = 2 << 2;
const STOP_TRIGGER_SET: i64 = 2 << 4;
const STOP_PARKING_SET: i64 = 2 << 5;
const STOP_CONTAINER_TRIGGER_SET: i64 = 2 << 7;

fn leak<T>(v: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(v)))
}

thread_local! {
    // static std::vector<Subscription> Helper::mySubscriptions (never freed, like the C++ global)
    static MY_SUBSCRIPTIONS: Ptr<Vec<Subscription>> = leak(Vec::new());
    // static Subscription * Helper::myLastContextSubscription
    static MY_LAST_CONTEXT_SUBSCRIPTION: Cell<Ptr<Subscription>> = Cell::new(Ptr::null());
    // static std::map<int, std::shared_ptr<VariableWrapper>> Helper::myWrapper
    static MY_WRAPPER: Ptr<BTreeMap<i32, Rc<VariableWrapper>>> = leak(BTreeMap::new());
    // static VehicleStateListener Helper::myVehicleStateListener
    static MY_VEHICLE_STATE_LISTENER: Ptr<Helper_VehicleStateListener> = leak(Helper_VehicleStateListener { base: MSNet_VehicleStateListener {}, myVehicleStateChanges: BTreeMap::new() });
    // static TransportableStateListener Helper::myTransportableStateListener
    static MY_TRANSPORTABLE_STATE_LISTENER: Ptr<Helper_TransportableStateListener> = leak(Helper_TransportableStateListener { base: MSNet_TransportableStateListener {}, myTransportableStateChanges: BTreeMap::new() });
    // static std::map<std::string, MSVehicle *> Helper::myRemoteControlledVehicles
    static MY_REMOTE_CONTROLLED_VEHICLES: Ptr<BTreeMap<String, Ptr<MSVehicle>>> = leak(BTreeMap::new());
    // static std::map<std::string, MSPerson *> Helper::myRemoteControlledPersons
    static MY_REMOTE_CONTROLLED_PERSONS: Ptr<BTreeMap<String, Ptr<MSPerson>>> = leak(BTreeMap::new());
}
// shortcut: static myLaneTree (RTree<MSLane*>) is not ported yet, lane searches panic in collectObjectsInRange

fn my_subscriptions() -> &'static mut Vec<Subscription> {
    unsafe { &mut *MY_SUBSCRIPTIONS.with(|p| p.as_raw()) }
}
fn my_wrapper() -> &'static mut BTreeMap<i32, Rc<VariableWrapper>> {
    unsafe { &mut *MY_WRAPPER.with(|p| p.as_raw()) }
}
fn my_vehicle_state_listener() -> &'static mut Helper_VehicleStateListener {
    unsafe { &mut *MY_VEHICLE_STATE_LISTENER.with(|p| p.as_raw()) }
}
fn my_transportable_state_listener() -> &'static mut Helper_TransportableStateListener {
    unsafe { &mut *MY_TRANSPORTABLE_STATE_LISTENER.with(|p| p.as_raw()) }
}
fn my_remote_controlled_vehicles() -> &'static mut BTreeMap<String, Ptr<MSVehicle>> {
    unsafe { &mut *MY_REMOTE_CONTROLLED_VEHICLES.with(|p| p.as_raw()) }
}
fn my_remote_controlled_persons() -> &'static mut BTreeMap<String, Ptr<MSPerson>> {
    unsafe { &mut *MY_REMOTE_CONTROLLED_PERSONS.with(|p| p.as_raw()) }
}

/// C++ `Subscription` copy constructor (the C++ code copies `s` into the vector)
fn copy_subscription(s: &Subscription) -> Subscription {
    Subscription {
        commandId: s.commandId,
        id: s.id.clone(),
        variables: s.variables.clone(),
        parameters: s.parameters.clone(),
        beginTime: s.beginTime,
        endTime: s.endTime,
        contextDomain: s.contextDomain,
        range: s.range,
        activeFilters: s.activeFilters,
        filterLanes: s.filterLanes.clone(),
        filterDownstreamDist: s.filterDownstreamDist,
        filterUpstreamDist: s.filterUpstreamDist,
        filterFoeDistToJunction: s.filterFoeDistToJunction,
        filterVTypes: s.filterVTypes.clone(),
        filterVClasses: s.filterVClasses,
        filterFieldOfVisionOpeningAngle: s.filterFieldOfVisionOpeningAngle,
        filterLateralDist: s.filterLateralDist,
    }
}

/// C++ `std::shared_ptr<TraCIResult>` built from a subclass (std::make_shared<TraCIDouble>(...) stored as a base pointer)
// shortcut: relies on #[repr(C)] with `base: TraCIResult` first on the TraCIResult subclasses in tra_ci_defs.rs, add it there
fn as_traci_result<T>(v: T) -> Rc<TraCIResult> {
    let p = Rc::into_raw(Rc::new(v)) as *const TraCIResult;
    unsafe { Rc::from_raw(p) }
}

/// C++ `libsumo::Helper::getEdge` lookup by id (MSEdge::dictionary)
fn edge_dictionary(id: &str) -> Ptr<MSEdge> {
    // shortcut: MSEdge::dictionary_2 still takes an Unmapped (string_view) argument, pass the id once ms_edge.rs maps it to &str
    let _ = id;
    MSEdge::dictionary_2(id)
}

/// C++ `checkTimeBounds(time)` returns false where it throws TimeFormatException
fn check_time_bounds(time: f64) -> bool {
    !(time > steps2time(SUMOTIME_MAX))
}

/// C++ `SubscriptionFilterType` used as an int bitmask
fn sf(f: SubscriptionFilterType) -> i32 {
    f as i32
}

/// C++ TraCI client API implementation
/// C++ class `libsumo::Helper` (libsumo/Helper.h)
pub struct Helper {
}

// C++ static members of Helper (port as module-level statics, see AGENTS.md):
//   static std::vector<Subscription> Helper::mySubscriptions;
//   static Subscription * Helper::myLastContextSubscription;
//   static std::map<int, std::shared_ptr<VariableWrapper>> Helper::myWrapper;
//   static VehicleStateListener Helper::myVehicleStateListener;
//   static TransportableStateListener Helper::myTransportableStateListener;
//   static RTree<MSLane *, MSLane, float, 2, MSLane::StoringVisitor> * Helper::myLaneTree;
//   static std::map<std::string, MSVehicle *> Helper::myRemoteControlledVehicles;
//   static std::map<std::string, MSPerson *> Helper::myRemoteControlledPersons;

impl Helper {
    /// C++: `void Helper::subscribe(const int, const std::string &, const std::vector<int> &, const double, const double, const libsumo::TraCIResults &, const int, const double)` [static]
    // void
    // Helper::subscribe(const int commandId, const std::string& id, const std::vector<int>& variables,
    //                   const double beginTime, const double endTime, const libsumo::TraCIResults& params,
    //                   const int contextDomain, const double range) {
    //     myLastContextSubscription = nullptr;
    //     if (variables.empty()) {
    //         for (std::vector<libsumo::Subscription>::iterator j = mySubscriptions.begin(); j != mySubscriptions.end();) {
    //             if (j->id == id && j->commandId == commandId && j->contextDomain == contextDomain) {
    //                 j = mySubscriptions.erase(j);
    //             } else {
    //                 ++j;
    //             }
    //         }
    //         return;
    //     }
    //     std::vector<std::shared_ptr<tcpip::Storage> > parameters;
    //     for (const int var : variables) {
    //         const auto& p = params.find(var);
    //         if (p == params.end()) {
    //             parameters.push_back(std::make_shared<tcpip::Storage>());
    //         } else {
    //             parameters.push_back(libsumo::StorageHelper::toStorage(*p->second));
    //         }
    //     }
    //     const SUMOTime begin = beginTime == INVALID_DOUBLE_VALUE ? 0 : TIME2STEPS(beginTime);
    //     const SUMOTime end = endTime == INVALID_DOUBLE_VALUE || endTime > STEPS2TIME(SUMOTime_MAX) ? SUMOTime_MAX : TIME2STEPS(endTime);
    //     libsumo::Subscription s(commandId, id, variables, parameters, begin, end, contextDomain, range);
    //     if (commandId == libsumo::CMD_SUBSCRIBE_SIM_CONTEXT) {
    //         s.range = std::numeric_limits<double>::max();
    //     }
    //     if (s.variables.size() == 1 && s.variables.front() == -1) {
    //         if (contextDomain == 0) {
    //             if (commandId == libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE) {
    //                 // default for vehicles is edge id and lane position
    //                 s.variables = {libsumo::VAR_ROAD_ID, libsumo::VAR_LANEPOSITION};
    //                 s.parameters.push_back(std::make_shared<tcpip::Storage>());
    //             } else if (commandId == libsumo::CMD_SUBSCRIBE_EDGE_VARIABLE ||
    //                        commandId == libsumo::CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE ||
    //                        commandId == libsumo::CMD_SUBSCRIBE_LANE_VARIABLE ||
    //                        commandId == libsumo::CMD_SUBSCRIBE_LANEAREA_VARIABLE ||
    //                        commandId == libsumo::CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE) {
    //                 // default for detectors, edges and lanes is vehicle number
    //                 s.variables[0] = libsumo::LAST_STEP_VEHICLE_NUMBER;
    //             } else {
    //                 // for all others id list
    //                 s.variables[0] = libsumo::TRACI_ID_LIST;
    //             }
    //         } else {
    //             s.variables.clear();
    //             s.parameters.clear();
    //         }
    //     }
    //     handleSingleSubscription(s);
    //     libsumo::Subscription* modifiedSubscription = nullptr;
    //     needNewSubscription(s, mySubscriptions, modifiedSubscription);
    //     if (modifiedSubscription->isVehicleToVehicleContextSubscription()
    //             || modifiedSubscription->isVehicleToPersonContextSubscription()) {
    //         // Set last modified vehicle context subscription active for filter modifications
    //         myLastContextSubscription = modifiedSubscription;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn subscribe(commandId: i32, id: &str, variables: &Vec<i32>, beginTime: f64, endTime: f64, params: &TraCIResults, contextDomain: i32, range: f64) {
        MY_LAST_CONTEXT_SUBSCRIPTION.with(|c| c.set(Ptr::null()));
        if variables.is_empty() {
            my_subscriptions().retain(|j| !(j.id == id && j.commandId == commandId && j.contextDomain == contextDomain));
            return;
        }
        let mut parameters: Vec<Rc<Storage>> = Vec::new();
        for var in variables.iter() {
            match params.get(var) {
                None => parameters.push(Rc::new(Storage::new())),
                Some(p) => parameters.push(StorageHelper::to_storage(p)),
            }
        }
        let begin: SUMOTime = if beginTime == INVALID_DOUBLE_VALUE { 0 } else { time2steps(beginTime) };
        let end: SUMOTime = if endTime == INVALID_DOUBLE_VALUE || endTime > steps2time(SUMOTIME_MAX) { SUMOTIME_MAX } else { time2steps(endTime) };
        let mut s = Subscription::new(commandId, id, variables, &parameters, begin, end, contextDomain, range);
        if commandId == CMD_SUBSCRIBE_SIM_CONTEXT {
            s.range = f64::MAX;
        }
        if s.variables.len() == 1 && s.variables[0] == -1 {
            if contextDomain == 0 {
                if commandId == CMD_SUBSCRIBE_VEHICLE_VARIABLE {
                    // default for vehicles is edge id and lane position
                    s.variables = vec![VAR_ROAD_ID, VAR_LANEPOSITION];
                    s.parameters.push(Rc::new(Storage::new()));
                } else if commandId == CMD_SUBSCRIBE_EDGE_VARIABLE
                    || commandId == CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE
                    || commandId == CMD_SUBSCRIBE_LANE_VARIABLE
                    || commandId == CMD_SUBSCRIBE_LANEAREA_VARIABLE
                    || commandId == CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE
                {
                    // default for detectors, edges and lanes is vehicle number
                    s.variables[0] = LAST_STEP_VEHICLE_NUMBER;
                } else {
                    // for all others id list
                    s.variables[0] = TRACI_ID_LIST;
                }
            } else {
                s.variables.clear();
                s.parameters.clear();
            }
        }
        Helper::handle_single_subscription(&s);
        let mut modifiedSubscription: Ptr<Subscription> = Ptr::null();
        Helper::need_new_subscription(&mut s, my_subscriptions(), &mut modifiedSubscription);
        if modifiedSubscription.is_vehicle_to_vehicle_context_subscription() || modifiedSubscription.is_vehicle_to_person_context_subscription() {
            // Set last modified vehicle context subscription active for filter modifications
            MY_LAST_CONTEXT_SUBSCRIPTION.with(|c| c.set(modifiedSubscription));
        }
    }

    /// C++: `void Helper::handleSubscriptions(const SUMOTime)` [static]
    // void
    // Helper::handleSubscriptions(const SUMOTime t) {
    //     for (auto& wrapper : myWrapper) {
    //         wrapper.second->clear();
    //     }
    //     for (std::vector<libsumo::Subscription>::iterator i = mySubscriptions.begin(); i != mySubscriptions.end();) {
    //         const libsumo::Subscription& s = *i;
    //         const bool isArrivedVehicle = (s.commandId == CMD_SUBSCRIBE_VEHICLE_VARIABLE || s.commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT)
    //                                       && (find(getVehicleStateChanges(MSNet::VehicleState::ARRIVED).begin(), getVehicleStateChanges(MSNet::VehicleState::ARRIVED).end(), s.id) != getVehicleStateChanges(MSNet::VehicleState::ARRIVED).end());
    //         const bool isArrivedPerson = (s.commandId == libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE || s.commandId == libsumo::CMD_SUBSCRIBE_PERSON_CONTEXT)
    //                                      && MSNet::getInstance()->getPersonControl().get(s.id) == nullptr;
    //         if (s.endTime < t || isArrivedVehicle || isArrivedPerson) {
    //             i = mySubscriptions.erase(i);
    //             continue;
    //         }
    //         ++i;
    //     }
    //     for (const libsumo::Subscription& s : mySubscriptions) {
    //         if (s.beginTime <= t) {
    //             handleSingleSubscription(s);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_subscriptions(t: SUMOTime) {
        for (_, w) in my_wrapper().iter() {
            Ptr::new(&**w).m().clear();
        }
        let subs = my_subscriptions();
        let mut i = 0;
        while i < subs.len() {
            let s = &subs[i];
            let is_arrived_vehicle = (s.commandId == CMD_SUBSCRIBE_VEHICLE_VARIABLE || s.commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT)
                && Helper::get_vehicle_state_changes(MSNet_VehicleState::ARRIVED).contains(&s.id);
            let is_arrived_person = (s.commandId == CMD_SUBSCRIBE_PERSON_VARIABLE || s.commandId == CMD_SUBSCRIBE_PERSON_CONTEXT)
                && MSNet::get_instance().m().get_person_control().get(&s.id).is_null();
            if s.endTime < t || is_arrived_vehicle || is_arrived_person {
                subs.remove(i);
                continue;
            }
            i += 1;
        }
        for s in subs.iter() {
            if s.beginTime <= t {
                Helper::handle_single_subscription(s);
            }
        }
    }

    /// C++: `bool Helper::needNewSubscription(libsumo::Subscription &, std::vector<Subscription> &, libsumo::Subscription *&)` [static]
    // bool
    // Helper::needNewSubscription(libsumo::Subscription& s, std::vector<Subscription>& subscriptions, libsumo::Subscription*& modifiedSubscription) {
    //     for (libsumo::Subscription& o : subscriptions) {
    //         if (s.commandId == o.commandId && s.id == o.id &&
    //                 s.beginTime == o.beginTime && s.endTime == o.endTime &&
    //                 s.contextDomain == o.contextDomain && s.range == o.range) {
    //             std::vector<std::shared_ptr<tcpip::Storage> >::const_iterator k = s.parameters.begin();
    //             for (const int v : s.variables) {
    //                 const int offset = (int)(std::find(o.variables.begin(), o.variables.end(), v) - o.variables.begin());
    //                 if (offset == (int)o.variables.size() || o.parameters[offset]->size() != (*k)->size() || !std::equal((*k)->begin(), (*k)->end(), o.parameters[offset]->begin())) {
    //                     o.variables.push_back(v);
    //                     o.parameters.push_back(*k);
    //                 }
    //                 ++k;
    //             }
    //             modifiedSubscription = &o;
    //             return false;
    //         }
    //     }
    //     subscriptions.push_back(s);
    //     modifiedSubscription = &subscriptions.back();
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn need_new_subscription(s: &mut Subscription, subscriptions: &mut Vec<Subscription>, modifiedSubscription: &mut Ptr<Subscription>) -> bool {
        for oi in 0..subscriptions.len() {
            if s.commandId == subscriptions[oi].commandId && s.id == subscriptions[oi].id && s.beginTime == subscriptions[oi].beginTime && s.endTime == subscriptions[oi].endTime && s.contextDomain == subscriptions[oi].contextDomain && s.range == subscriptions[oi].range {
                let mut k = 0usize;
                for &v in s.variables.iter() {
                    let offset = subscriptions[oi].variables.iter().position(|&x| x == v).unwrap_or(subscriptions[oi].variables.len());
                    let differs = offset == subscriptions[oi].variables.len()
                        || subscriptions[oi].parameters[offset].size() != s.parameters[k].size()
                        || subscriptions[oi].parameters[offset].as_bytes() != s.parameters[k].as_bytes();
                    if differs {
                        let param = s.parameters[k].clone();
                        subscriptions[oi].variables.push(v);
                        subscriptions[oi].parameters.push(param);
                    }
                    k += 1;
                }
                *modifiedSubscription = Ptr::from_mut(&mut subscriptions[oi]);
                return false;
            }
        }
        subscriptions.push(copy_subscription(s));
        *modifiedSubscription = Ptr::from_mut(subscriptions.last_mut().unwrap());
        true
    }

    /// C++: `void Helper::clearSubscriptions()` [static]
    // void
    // Helper::clearSubscriptions() {
    //     mySubscriptions.clear();
    //     myLastContextSubscription = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn clear_subscriptions() {
        my_subscriptions().clear();
        MY_LAST_CONTEXT_SUBSCRIPTION.with(|c| c.set(Ptr::null()));
    }

    /// C++: `Subscription * Helper::addSubscriptionFilter(SubscriptionFilterType)` [static]
    // Subscription*
    // Helper::addSubscriptionFilter(SubscriptionFilterType filter) {
    //     if (myLastContextSubscription != nullptr) {
    //         myLastContextSubscription->activeFilters |= filter;
    //     } else {
    //         // The following code relies on the fact that the filter is 2^(filterType-1),
    //         // see Subscription.h and the corresponding TraCIConstants.h.
    //         // It is only for getting similar error messages with libsumo and traci.
    //         int index = (int)filter;
    //         int filterType = 0;
    //         if (index != 0) {
    //             ++filterType;
    //             while (index >>= 1) {
    //                 ++filterType;
    //             }
    //         }
    //         throw TraCIException("No previous vehicle context subscription exists to apply filter type " + toHex(filterType, 2));
    //     }
    //     return myLastContextSubscription;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter(filter: SubscriptionFilterType) -> Ptr<Subscription> {
        let last = MY_LAST_CONTEXT_SUBSCRIPTION.with(|c| c.get());
        if !last.is_null() {
            last.m().activeFilters |= filter as i32;
            return last;
        }
        // The following code relies on the fact that the filter is 2^(filterType-1),
        // see Subscription.h and the corresponding TraCIConstants.h.
        // It is only for getting similar error messages with libsumo and traci.
        let mut index = filter as i32;
        let mut filterType = 0;
        if index != 0 {
            filterType += 1;
            index >>= 1;
            while index != 0 {
                filterType += 1;
                index >>= 1;
            }
        }
        panic!("No previous vehicle context subscription exists to apply filter type {}", to_hex(filterType as u64, 2, 8));
    }

    /// C++: `TraCIPositionVector Helper::makeTraCIPositionVector(const PositionVector &)` [static]
    /// helper functions
    // TraCIPositionVector
    // Helper::makeTraCIPositionVector(const PositionVector& positionVector) {
    //     TraCIPositionVector tp;
    //     for (int i = 0; i < (int)positionVector.size(); ++i) {
    //         tp.value.push_back(makeTraCIPosition(positionVector[i]));
    //     }
    //     return tp;
    // }
    #[allow(unused_variables)]
    pub fn make_tra_ci_position_vector(positionVector: &PositionVector) -> TraCIPositionVector {
        let mut tp = TraCIPositionVector { base: TraCIResult::new(), value: Vec::new() };
        for i in 0..positionVector.len() {
            tp.value.push(Helper::make_tra_ci_position(&positionVector[i], false));
        }
        tp
    }

    /// C++: `TraCIPosition Helper::makeTraCIPosition(const Position &, const bool)` [static]
    // TraCIPosition
    // Helper::makeTraCIPosition(const Position& position, const bool includeZ) {
    //     TraCIPosition p;
    //     p.x = position.x();
    //     p.y = position.y();
    //     p.z = includeZ ? position.z() : INVALID_DOUBLE_VALUE;
    //     return p;
    // }
    #[allow(unused_variables)]
    pub fn make_tra_ci_position(position: &Position, includeZ: bool) -> TraCIPosition {
        let z = if includeZ { position.z() } else { INVALID_DOUBLE_VALUE };
        TraCIPosition {
            base: TraCIResult { result_type: if z != INVALID_DOUBLE_VALUE { POSITION_3D } else { POSITION_2D } },
            x: position.x(),
            y: position.y(),
            z,
        }
    }

    /// C++: `Position Helper::makePosition(const TraCIPosition &)` [static]
    // Position
    // Helper::makePosition(const TraCIPosition& tpos) {
    //     return Position(tpos.x, tpos.y, tpos.z);
    // }
    #[allow(unused_variables)]
    pub fn make_position(position: &TraCIPosition) -> Position {
        Position::new3(position.x, position.y, position.z)
    }

    /// C++: `PositionVector Helper::makePositionVector(const TraCIPositionVector &)` [static]
    // PositionVector
    // Helper::makePositionVector(const TraCIPositionVector& vector) {
    //     PositionVector pv;
    //     for (const TraCIPosition& pos : vector.value) {
    //         if (std::isnan(pos.x) || std::isnan(pos.y)) {
    //             throw libsumo::TraCIException("NaN-Value in shape.");
    //         }
    //         pv.push_back(Position(pos.x, pos.y));
    //     }
    //     return pv;
    // }
    #[allow(unused_variables)]
    pub fn make_position_vector(vector: &TraCIPositionVector) -> PositionVector {
        let mut pv = PositionVector(Vec::new());
        for pos in vector.value.iter() {
            if pos.x.is_nan() || pos.y.is_nan() {
                panic!("NaN-Value in shape.");
            }
            pv.push(Position::new(pos.x, pos.y));
        }
        pv
    }

    /// C++: `TraCIColor Helper::makeTraCIColor(const RGBColor &)` [static]
    // TraCIColor
    // Helper::makeTraCIColor(const RGBColor& color) {
    //     TraCIColor tc;
    //     tc.a = color.alpha();
    //     tc.b = color.blue();
    //     tc.g = color.green();
    //     tc.r = color.red();
    //     return tc;
    // }
    #[allow(unused_variables)]
    pub fn make_tra_ci_color(color: &RGBColor) -> TraCIColor {
        TraCIColor {
            base: TraCIResult::new(),
            r: color.red() as i32,
            g: color.green() as i32,
            b: color.blue() as i32,
            a: color.alpha() as i32,
        }
    }

    /// C++: `RGBColor Helper::makeRGBColor(const TraCIColor &)` [static]
    // RGBColor
    // Helper::makeRGBColor(const TraCIColor& c) {
    //     return RGBColor((unsigned char)c.r, (unsigned char)c.g, (unsigned char)c.b, (unsigned char)c.a);
    // }
    #[allow(unused_variables)]
    pub fn make_rgb_color(color: &TraCIColor) -> RGBColor {
        RGBColor::new(color.r as u8, color.g as u8, color.b as u8, color.a as u8)
    }

    /// C++: `MSEdge * Helper::getEdge(const std::string &)` [static]
    // MSEdge*
    // Helper::getEdge(const std::string& edgeID) {
    //     MSEdge* edge = MSEdge::dictionary(edgeID);
    //     if (edge == nullptr) {
    //         throw TraCIException("Referenced edge '" + edgeID + "' is not known.");
    //     }
    //     return edge;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(edgeID: &str) -> Ptr<MSEdge> {
        let edge = edge_dictionary(edgeID);
        if edge.is_null() {
            panic!("Referenced edge '{}' is not known.", edgeID);
        }
        edge
    }

    /// C++: `const MSLane * Helper::getLaneChecking(const std::string &, int, double)` [static]
    // const MSLane*
    // Helper::getLaneChecking(const std::string& edgeID, int laneIndex, double pos) {
    //     const MSEdge* edge = MSEdge::dictionary(edgeID);
    //     if (edge == nullptr) {
    //         throw TraCIException("Unknown edge " + edgeID);
    //     }
    //     if (laneIndex < 0 || laneIndex >= (int)edge->getLanes().size()) {
    //         throw TraCIException("Invalid lane index for " + edgeID);
    //     }
    //     const MSLane* lane = edge->getLanes()[laneIndex];
    //     if (pos < 0 || pos > lane->getLength()) {
    //         throw TraCIException("Position on lane invalid");
    //     }
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_checking(edgeID: &str, laneIndex: i32, pos: f64) -> Ptr<MSLane> {
        let edge = edge_dictionary(edgeID);
        if edge.is_null() {
            panic!("Unknown edge {}", edgeID);
        }
        if laneIndex < 0 || laneIndex >= edge.get_lanes().len() as i32 {
            panic!("Invalid lane index for {}", edgeID);
        }
        let lane = edge.get_lanes()[laneIndex as usize];
        if pos < 0.0 || pos > lane.get_length() {
            panic!("Position on lane invalid");
        }
        lane
    }

    /// C++: `std::pair<MSLane *, double> Helper::convertCartesianToRoadMap(const Position &, const SUMOVehicleClass)` [static]
    // std::pair<MSLane*, double>
    // Helper::convertCartesianToRoadMap(const Position& pos, const SUMOVehicleClass vClass) {
    //     const PositionVector shape({ pos });
    //     std::pair<MSLane*, double> result(nullptr, -1);
    //     double range = 1000.;
    //     const Boundary& netBounds = GeoConvHelper::getFinal().getConvBoundary();
    //     const double maxRange = MAX2(1001., netBounds.getWidth() + netBounds.getHeight() + netBounds.distanceTo2D(pos));
    //     while (range < maxRange) {
    //         std::set<const Named*> lanes;
    //         collectObjectsInRange(libsumo::CMD_GET_LANE_VARIABLE, shape, range, lanes);
    //         double minDistance = std::numeric_limits<double>::max();
    //         for (const Named* named : lanes) {
    //             MSLane* lane = const_cast<MSLane*>(dynamic_cast<const MSLane*>(named));
    //             if (lane->allowsVehicleClass(vClass)) {
    //                 // @todo this may be a place where 3D is required but 2D is used
    //                 double newDistance = lane->getShape().distance2D(pos);
    //                 newDistance = patchShapeDistance(lane, pos, newDistance, false);
    //                 if (newDistance < minDistance ||
    //                         (newDistance == minDistance
    //                          && result.first != nullptr
    //                          && lane->getID() < result.first->getID())) {
    //                     minDistance = newDistance;
    //                     result.first = lane;
    //                 }
    //             }
    //         }
    //         if (minDistance < std::numeric_limits<double>::max()) {
    //             result.second = result.first->interpolateGeometryPosToLanePos(result.first->getShape().nearest_offset_to_point2D(pos, false));
    //             break;
    //         }
    //         range *= 2;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn convert_cartesian_to_road_map(pos: &Position, vClass: SUMOVehicleClass) -> (Ptr<MSLane>, f64) {
        let shape = PositionVector(vec![*pos]);
        let mut result: (Ptr<MSLane>, f64) = (Ptr::null(), -1.0);
        let mut range = 1000.;
        let netBounds: Boundary = GeoConvHelper::with_final(|g| g.get_conv_boundary().clone());
        let maxRange = f64::max(1001., netBounds.get_width() + netBounds.get_height() + netBounds.distance_to2d(pos));
        while range < maxRange {
            let mut lanes: BTreeSet<Ptr<Named>> = BTreeSet::new();
            Helper::collect_objects_in_range(CMD_GET_LANE_VARIABLE, &shape, range, &mut lanes);
            let mut minDistance = f64::MAX;
            for named in lanes.iter() {
                let lane: Ptr<MSLane> = named.cast::<MSLane>(); // dynamic_cast<const MSLane*>
                if lane.allows_vehicle_class(vClass) {
                    // @todo this may be a place where 3D is required but 2D is used
                    let mut newDistance = lane.get_shape(false).distance2d(pos, false);
                    newDistance = Helper::patch_shape_distance(lane, pos, newDistance, false);
                    if newDistance < minDistance || (newDistance == minDistance && !result.0.is_null() && lane.get_id() < result.0.get_id()) {
                        minDistance = newDistance;
                        result.0 = lane;
                    }
                }
            }
            if minDistance < f64::MAX {
                let off = result.0.get_shape(false).nearest_offset_to_point2d(pos, false);
                result.1 = result.0.interpolate_geometry_pos_to_lane_pos(off);
                break;
            }
            range *= 2.;
        }
        result
    }

    /// C++: `double Helper::getDrivingDistance(std::pair<const MSLane *, double> &, std::pair<const MSLane *, double> &)` [static]
    // double
    // Helper::getDrivingDistance(std::pair<const MSLane*, double>& roadPos1, std::pair<const MSLane*, double>& roadPos2) {
    //     if (roadPos1.first == roadPos2.first && roadPos1.second <= roadPos2.second) {
    //         // same edge
    //         return roadPos2.second - roadPos1.second;
    //     }
    //     double distance = 0.0;
    //     ConstMSEdgeVector newRoute;
    //     while (roadPos2.first->isInternal() && roadPos2.first != roadPos1.first) {
    //         distance += roadPos2.second;
    //         roadPos2.first = roadPos2.first->getLogicalPredecessorLane();
    //         roadPos2.second = roadPos2.first->getLength();
    //     }
    //     MSNet::getInstance()->getRouterTT(0).compute(&roadPos1.first->getEdge(), &roadPos2.first->getEdge(), nullptr, SIMSTEP, newRoute, true);
    //     if (newRoute.empty()) {
    //         return libsumo::INVALID_DOUBLE_VALUE;
    //     }
    //     MSRoute route("", newRoute, false, nullptr, StopParVector());
    //     return distance + route.getDistanceBetween(roadPos1.second, roadPos2.second, roadPos1.first, roadPos2.first);
    // }
    #[allow(unused_variables)]
    pub fn get_driving_distance(roadPos1: &mut (Ptr<MSLane>, f64), roadPos2: &mut (Ptr<MSLane>, f64)) -> f64 {
        if roadPos1.0 == roadPos2.0 && roadPos1.1 <= roadPos2.1 {
            // same edge
            return roadPos2.1 - roadPos1.1;
        }
        let mut distance = 0.0;
        let mut newRoute: ConstMSEdgeVector = Vec::new();
        while roadPos2.0.is_internal() && roadPos2.0 != roadPos1.0 {
            distance += roadPos2.1;
            roadPos2.0 = roadPos2.0.get_logical_predecessor_lane();
            roadPos2.1 = roadPos2.0.get_length();
        }
        let from = Ptr::from_mut(roadPos1.0.m().get_edge());
        let to = Ptr::from_mut(roadPos2.0.m().get_edge());
        // shortcut: the prohibitions map is Unmapped (), an empty map is passed
        let now = MSNet::get_instance().get_current_time_step();
        MSNet::get_instance().get_router_tt(0, &()).compute(from, to, Ptr::null(), now, &mut newRoute, true);
        if newRoute.is_empty() {
            return INVALID_DOUBLE_VALUE;
        }
        let route = MSRoute::new("", &newRoute, false, Ptr::null(), &Vec::new(), -1, 0);
        distance + route.get_distance_between(roadPos1.1, roadPos2.1, roadPos1.0, roadPos2.0, 0)
    }

    /// C++: `MSBaseVehicle * Helper::getVehicle(const std::string &)` [static]
    // MSBaseVehicle*
    // Helper::getVehicle(const std::string& id) {
    //     SUMOVehicle* sumoVehicle = MSNet::getInstance()->getVehicleControl().getVehicle(id);
    //     if (sumoVehicle == nullptr) {
    //         throw TraCIException("Vehicle '" + id + "' is not known.");
    //     }
    //     MSBaseVehicle* v = dynamic_cast<MSBaseVehicle*>(sumoVehicle);
    //     if (v == nullptr) {
    //         throw TraCIException("Vehicle '" + id + "' is not a proper vehicle.");
    //     }
    //     return v;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle(id: &str) -> Ptr<MSBaseVehicle> {
        let sumoVehicle = MSNet::get_instance().get_vehicle_control().get_vehicle(id);
        if sumoVehicle.is_null() {
            panic!("Vehicle '{}' is not known.", id);
        }
        // shortcut: dynamic_cast<MSBaseVehicle*> is not checked at runtime, the caller must guarantee a base vehicle
        let v: Ptr<MSBaseVehicle> = sumoVehicle.cast::<MSBaseVehicle>();
        v
    }

    /// C++: `MSPerson * Helper::getPerson(const std::string &)` [static]
    // MSPerson*
    // Helper::getPerson(const std::string& personID) {
    //     MSTransportableControl& c = MSNet::getInstance()->getPersonControl();
    //     MSPerson* p = dynamic_cast<MSPerson*>(c.get(personID));
    //     if (p == nullptr) {
    //         throw TraCIException("Person '" + personID + "' is not known");
    //     }
    //     return p;
    // }
    #[allow(unused_variables)]
    pub fn get_person(id: &str) -> Ptr<MSPerson> {
        let p: Ptr<MSPerson> = MSNet::get_instance().get_person_control().get(id).cast::<MSPerson>();
        if p.is_null() {
            panic!("Person '{}' is not known", id);
        }
        p
    }

    /// C++: `SUMOTrafficObject * Helper::getTrafficObject(int, const std::string &)` [static]
    // SUMOTrafficObject*
    // Helper::getTrafficObject(int domain, const std::string& id) {
    //     if (domain == CMD_GET_VEHICLE_VARIABLE) {
    //         return getVehicle(id);
    //     } else if (domain == CMD_GET_PERSON_VARIABLE) {
    //         return getPerson(id);
    //     } else {
    //         throw TraCIException("Cannot retrieve traffic object for domain " + toString(domain));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_traffic_object(domain: i32, id: &str) -> Ptr<SUMOTrafficObject> {
        if domain == CMD_GET_VEHICLE_VARIABLE {
            Helper::get_vehicle(id).cast::<SUMOTrafficObject>()
        } else if domain == CMD_GET_PERSON_VARIABLE {
            Helper::get_person(id).cast::<SUMOTrafficObject>()
        } else {
            panic!("Cannot retrieve traffic object for domain {}", domain);
        }
    }

    /// C++: `const MSVehicleType & Helper::getVehicleType(const std::string &)` [static]
    // const MSVehicleType&
    // Helper::getVehicleType(const std::string& vehicleID) {
    //     return getVehicle(vehicleID)->getVehicleType();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(vehicleID: &str) -> &'static MSVehicleType {
        Helper::get_vehicle(vehicleID).get_mut().unwrap().get_vehicle_type()
    }

    /// C++: `MSTLLogicControl::TLSLogicVariants & Helper::getTLS(const std::string &)` [static]
    // MSTLLogicControl::TLSLogicVariants&
    // Helper::getTLS(const std::string& id) {
    //     if (!MSNet::getInstance()->getTLSControl().knows(id)) {
    //         throw TraCIException("Traffic light '" + id + "' is not known");
    //     }
    //     return MSNet::getInstance()->getTLSControl().get(id);
    // }
    #[allow(unused_variables)]
    pub fn get_tls(id: &str) -> &'static mut MSTLLogicControl_TLSLogicVariants {
        if !MSNet::get_instance().get_mut().unwrap().get_tls_control().knows(id) {
            panic!("Traffic light '{}' is not known", id);
        }
        MSNet::get_instance().get_mut().unwrap().get_tls_control().get(id)
    }

    /// C++: `MSStoppingPlace * Helper::getStoppingPlace(const std::string &, const SumoXMLTag)` [static]
    // MSStoppingPlace*
    // Helper::getStoppingPlace(const std::string& id, const SumoXMLTag type) {
    //     MSStoppingPlace* s = MSNet::getInstance()->getStoppingPlace(id, type);
    //     if (s == nullptr) {
    //         throw TraCIException(toString(type) + " '" + id + "' is not known");
    //     }
    //     return s;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place(id: &str, type_: SumoXMLTag) -> Ptr<MSStoppingPlace> {
        let s = MSNet::get_instance().get_stopping_place(id, type_);
        if s.is_null() {
            panic!("{} '{}' is not known", utils::common::to_string::to_string(&type_), id);
        }
        s
    }

    /// C++: `SUMOVehicleParameter::Stop Helper::buildStopParameters(const std::string &, double, int, double, int, double, double)` [static]
    // SUMOVehicleParameter::Stop
    // Helper::buildStopParameters(const std::string& edgeOrStoppingPlaceID,
    //                             double pos, int laneIndex, double startPos, int flags, double duration, double until) {
    //     SUMOVehicleParameter::Stop newStop;
    //     try {
    //         checkTimeBounds(duration);
    //         checkTimeBounds(until);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration or until parameter exceed the time value range.");
    //     }
    //     newStop.duration = duration == INVALID_DOUBLE_VALUE ? SUMOTime_MAX : TIME2STEPS(duration);
    //     newStop.until = until == INVALID_DOUBLE_VALUE ? -1 : TIME2STEPS(until);
    //     newStop.index = STOP_INDEX_FIT;
    //     if (newStop.duration >= 0) {
    //         newStop.parametersSet |= STOP_DURATION_SET as i32;
    //     }
    //     if (newStop.until >= 0) {
    //         newStop.parametersSet |= STOP_UNTIL_SET as i32;
    //     }
    //     if ((flags & 1) != 0) {
    //         newStop.parking = ParkingType::OFFROAD;
    //         newStop.parametersSet |= STOP_PARKING_SET as i32;
    //     }
    //     if ((flags & 2) != 0) {
    //         newStop.triggered = true;
    //         newStop.parametersSet |= STOP_TRIGGER_SET as i32;
    //     }
    //     if ((flags & 4) != 0) {
    //         newStop.containerTriggered = true;
    //         newStop.parametersSet |= STOP_CONTAINER_TRIGGER_SET as i32;
    //     }
    //
    //     SumoXMLTag stoppingPlaceType = SUMO_TAG_NOTHING;
    //     if ((flags & 8) != 0) {
    //         stoppingPlaceType = SUMO_TAG_BUS_STOP;
    //     }
    //     if ((flags & 16) != 0) {
    //         stoppingPlaceType = SUMO_TAG_CONTAINER_STOP;
    //     }
    //     if ((flags & 32) != 0) {
    //         stoppingPlaceType = SUMO_TAG_CHARGING_STATION;
    //     }
    //     if ((flags & 64) != 0) {
    //         stoppingPlaceType = SUMO_TAG_PARKING_AREA;
    //     }
    //     if ((flags & 128) != 0) {
    //         stoppingPlaceType = SUMO_TAG_OVERHEAD_WIRE_SEGMENT;
    //     }
    //
    //     if (stoppingPlaceType != SUMO_TAG_NOTHING) {
    //         MSStoppingPlace* bs = MSNet::getInstance()->getStoppingPlace(edgeOrStoppingPlaceID, stoppingPlaceType);
    //         if (bs == nullptr) {
    //             throw TraCIException("The " + toString(stoppingPlaceType) + " '" + edgeOrStoppingPlaceID + "' is not known");
    //         }
    //         newStop.lane = bs->getLane().getID();
    //         newStop.edge = bs->getLane().getEdge().getID();
    //         newStop.endPos = bs->getEndLanePosition();
    //         newStop.startPos = bs->getBeginLanePosition();
    //         switch (stoppingPlaceType) {
    //             case SUMO_TAG_BUS_STOP:
    //                 newStop.busstop = edgeOrStoppingPlaceID;
    //                 break;
    //             case SUMO_TAG_CONTAINER_STOP:
    //                 newStop.containerstop = edgeOrStoppingPlaceID;
    //                 break;
    //             case SUMO_TAG_CHARGING_STATION:
    //                 newStop.chargingStation = edgeOrStoppingPlaceID;
    //                 break;
    //             case SUMO_TAG_PARKING_AREA:
    //                 newStop.parkingarea = edgeOrStoppingPlaceID;
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //                 newStop.overheadWireSegment = edgeOrStoppingPlaceID;
    //                 break;
    //             default:
    //                 throw TraCIException("Unknown stopping place type '" + toString(stoppingPlaceType) + "'.");
    //         }
    //     } else {
    //         if (startPos == INVALID_DOUBLE_VALUE) {
    //             startPos = MAX2(0.0, pos - POSITION_EPS);
    //         }
    //         if (startPos < 0.) {
    //             throw TraCIException("Position on lane must not be negative.");
    //         }
    //         if (pos < startPos) {
    //             throw TraCIException("End position on lane must be after start position.");
    //         }
    //         // get the actual lane that is referenced by laneIndex
    //         MSEdge* road = MSEdge::dictionary(edgeOrStoppingPlaceID);
    //         if (road == nullptr) {
    //             throw TraCIException("Edge '" + edgeOrStoppingPlaceID + "' is not known.");
    //         }
    //         const std::vector<MSLane*>& allLanes = road->getLanes();
    //         if ((laneIndex < 0) || laneIndex >= (int)(allLanes.size())) {
    //             throw TraCIException("No lane with index '" + toString(laneIndex) + "' on edge '" + edgeOrStoppingPlaceID + "'.");
    //         }
    //         newStop.lane = allLanes[laneIndex]->getID();
    //         newStop.edge = allLanes[laneIndex]->getEdge().getID();
    //         newStop.endPos = pos;
    //         newStop.startPos = startPos;
    //         newStop.parametersSet |= (STOP_START_SET | STOP_END_SET) as i32;
    //     }
    //     return newStop;
    // }
    #[allow(unused_variables)]
    pub fn build_stop_parameters(edgeOrStoppingPlaceID: &str, pos: f64, laneIndex: i32, startPos: f64, flags: i32, duration: f64, until: f64) -> SUMOVehicleParameter_Stop {
        // shortcut: SUMOVehicleParameter::Stop has no Rust default constructor yet (C++ `Stop newStop;`), add one in sumo_vehicle_parameter.rs
    let mut newStop: SUMOVehicleParameter_Stop = panic!("shortcut: SUMOVehicleParameter::Stop default constructor is not ported yet");
        if !check_time_bounds(duration) || !check_time_bounds(until) {
            panic!("Duration or until parameter exceed the time value range.");
        }
        newStop.duration = if duration == INVALID_DOUBLE_VALUE { SUMOTIME_MAX } else { time2steps(duration) };
        newStop.until = if until == INVALID_DOUBLE_VALUE { -1 } else { time2steps(until) };
        newStop.index = STOP_INDEX_FIT;
        if newStop.duration >= 0 {
            newStop.parametersSet |= STOP_DURATION_SET as i32;
        }
        if newStop.until >= 0 {
            newStop.parametersSet |= STOP_UNTIL_SET as i32;
        }
        if (flags & 1) != 0 {
            newStop.parking = ParkingType::OFFROAD;
            newStop.parametersSet |= STOP_PARKING_SET as i32;
        }
        if (flags & 2) != 0 {
            newStop.triggered = true;
            newStop.parametersSet |= STOP_TRIGGER_SET as i32;
        }
        if (flags & 4) != 0 {
            newStop.containerTriggered = true;
            newStop.parametersSet |= STOP_CONTAINER_TRIGGER_SET as i32;
        }

        let mut stoppingPlaceType = SUMO_TAG_NOTHING;
        if (flags & 8) != 0 {
            stoppingPlaceType = SUMO_TAG_BUS_STOP;
        }
        if (flags & 16) != 0 {
            stoppingPlaceType = SUMO_TAG_CONTAINER_STOP;
        }
        if (flags & 32) != 0 {
            stoppingPlaceType = SUMO_TAG_CHARGING_STATION;
        }
        if (flags & 64) != 0 {
            stoppingPlaceType = SUMO_TAG_PARKING_AREA;
        }
        if (flags & 128) != 0 {
            stoppingPlaceType = SUMO_TAG_OVERHEAD_WIRE_SEGMENT;
        }

        if stoppingPlaceType != SUMO_TAG_NOTHING {
            let bs = MSNet::get_instance().get_stopping_place(edgeOrStoppingPlaceID, stoppingPlaceType);
            if bs.is_null() {
                panic!("The {} '{}' is not known", utils::common::to_string::to_string(&stoppingPlaceType), edgeOrStoppingPlaceID);
            }
            newStop.lane = bs.get_lane().get_id().to_string();
            newStop.edge = bs.get_lane().get_edge().get_id().to_string();
            newStop.endPos = bs.get_end_lane_position();
            newStop.startPos = bs.get_begin_lane_position();
            match stoppingPlaceType {
                SUMO_TAG_BUS_STOP => newStop.busstop = edgeOrStoppingPlaceID.to_string(),
                SUMO_TAG_CONTAINER_STOP => newStop.containerstop = edgeOrStoppingPlaceID.to_string(),
                SUMO_TAG_CHARGING_STATION => newStop.chargingStation = edgeOrStoppingPlaceID.to_string(),
                SUMO_TAG_PARKING_AREA => newStop.parkingarea = edgeOrStoppingPlaceID.to_string(),
                SUMO_TAG_OVERHEAD_WIRE_SEGMENT => newStop.overheadWireSegment = edgeOrStoppingPlaceID.to_string(),
                _ => panic!("Unknown stopping place type '{}'.", utils::common::to_string::to_string(&stoppingPlaceType)),
            }
        } else {
            if startPos == INVALID_DOUBLE_VALUE {
                startPos = f64::max(0.0, pos - utils::geom::POSITION_EPS);
            }
            if startPos < 0. {
                panic!("Position on lane must not be negative.");
            }
            if pos < startPos {
                panic!("End position on lane must be after start position.");
            }
            // get the actual lane that is referenced by laneIndex
            let road = edge_dictionary(edgeOrStoppingPlaceID);
            if road.is_null() {
                panic!("Edge '{}' is not known.", edgeOrStoppingPlaceID);
            }
            let allLanes = road.get_lanes();
            if laneIndex < 0 || laneIndex >= allLanes.len() as i32 {
                panic!("No lane with index '{}' on edge '{}'.", laneIndex, edgeOrStoppingPlaceID);
            }
            newStop.lane = allLanes[laneIndex as usize].get_id().to_string();
            newStop.edge = allLanes[laneIndex as usize].get_edge().get_id().to_string();
            newStop.endPos = pos;
            newStop.startPos = startPos;
            newStop.parametersSet |= (STOP_START_SET | STOP_END_SET) as i32;
        }
        newStop
    }

    /// C++: `TraCINextStopData Helper::buildStopData(const SUMOVehicleParameter::Stop &)` [static]
    // TraCINextStopData
    // Helper::buildStopData(const SUMOVehicleParameter::Stop& stopPar) {
    //     std::string stoppingPlaceID = "";
    //     if (stopPar.busstop != "") {
    //         stoppingPlaceID = stopPar.busstop;
    //     }
    //     if (stopPar.containerstop != "") {
    //         stoppingPlaceID = stopPar.containerstop;
    //     }
    //     if (stopPar.parkingarea != "") {
    //         stoppingPlaceID = stopPar.parkingarea;
    //     }
    //     if (stopPar.chargingStation != "") {
    //         stoppingPlaceID = stopPar.chargingStation;
    //     }
    //     if (stopPar.overheadWireSegment != "") {
    //         stoppingPlaceID = stopPar.overheadWireSegment;
    //     }
    //
    //     return TraCINextStopData(stopPar.lane,
    //                              stopPar.startPos,
    //                              stopPar.endPos,
    //                              stoppingPlaceID,
    //                              stopPar.getFlags(),
    //                              // negative duration is permitted to indicate that a vehicle cannot
    //                              // re-enter traffic after parking
    //                              stopPar.duration != -1 ? STEPS2TIME(stopPar.duration) : INVALID_DOUBLE_VALUE,
    //                              stopPar.until >= 0 ? STEPS2TIME(stopPar.until) : INVALID_DOUBLE_VALUE,
    //                              stopPar.arrival >= 0 ? STEPS2TIME(stopPar.arrival) : INVALID_DOUBLE_VALUE,
    //                              stopPar.started >= 0 ? STEPS2TIME(stopPar.started) : INVALID_DOUBLE_VALUE,
    //                              stopPar.ended >= 0 ? STEPS2TIME(stopPar.ended) : INVALID_DOUBLE_VALUE,
    //                              stopPar.split,
    //                              stopPar.join,
    //                              stopPar.actType,
    //                              stopPar.tripId,
    //                              stopPar.line,
    //                              stopPar.speed);
    // }
    #[allow(unused_variables)]
    pub fn build_stop_data(stopPar: &SUMOVehicleParameter_Stop) -> TraCINextStopData {
        let mut stoppingPlaceID = String::new();
        if stopPar.busstop != "" {
            stoppingPlaceID = stopPar.busstop.clone();
        }
        if stopPar.containerstop != "" {
            stoppingPlaceID = stopPar.containerstop.clone();
        }
        if stopPar.parkingarea != "" {
            stoppingPlaceID = stopPar.parkingarea.clone();
        }
        if stopPar.chargingStation != "" {
            stoppingPlaceID = stopPar.chargingStation.clone();
        }
        if stopPar.overheadWireSegment != "" {
            stoppingPlaceID = stopPar.overheadWireSegment.clone();
        }
        // negative duration is permitted to indicate that a vehicle cannot
        // re-enter traffic after parking
        TraCINextStopData::new(
            &stopPar.lane,
            stopPar.startPos,
            stopPar.endPos,
            &stoppingPlaceID,
            stopPar.get_flags(),
            if stopPar.duration != -1 { steps2time(stopPar.duration) } else { INVALID_DOUBLE_VALUE },
            if stopPar.until >= 0 { steps2time(stopPar.until) } else { INVALID_DOUBLE_VALUE },
            if stopPar.arrival >= 0 { steps2time(stopPar.arrival) } else { INVALID_DOUBLE_VALUE },
            if stopPar.started >= 0 { steps2time(stopPar.started) } else { INVALID_DOUBLE_VALUE },
            if stopPar.ended >= 0 { steps2time(stopPar.ended) } else { INVALID_DOUBLE_VALUE },
            &stopPar.split,
            &stopPar.join,
            &stopPar.actType,
            &stopPar.tripId,
            &stopPar.line,
            stopPar.speed,
        )
    }

    /// C++: `void Helper::findObjectShape(int, const std::string &, PositionVector &)` [static]
    // void
    // Helper::findObjectShape(int domain, const std::string& id, PositionVector& shape) {
    //     switch (domain) {
    //         case libsumo::CMD_SUBSCRIBE_BUSSTOP_CONTEXT: {
    //             MSStoppingPlace* const stop = getStoppingPlace(id, SUMO_TAG_BUS_STOP);
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getBeginLanePosition()));
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getEndLanePosition()));
    //             break;
    //         }
    //         case libsumo::CMD_SUBSCRIBE_CALIBRATOR_CONTEXT: {
    //             MSCalibrator* const calib = Calibrator::getCalibrator(id);
    //             shape.push_back(calib->getLane()->getShape()[0]);
    //             break;
    //         }
    //         case libsumo::CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT: {
    //             MSStoppingPlace* const stop = getStoppingPlace(id, SUMO_TAG_CHARGING_STATION);
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getBeginLanePosition()));
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getEndLanePosition()));
    //             break;
    //         }
    //         case libsumo::CMD_SUBSCRIBE_EDGE_CONTEXT:
    //             Edge::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT:
    //             InductionLoop::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_JUNCTION_CONTEXT:
    //             Junction::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_LANE_CONTEXT:
    //             Lane::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_LANEAREA_CONTEXT:
    //             LaneArea::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT: {
    //             MSE3Collector* const det = MultiEntryExit::getDetector(id);
    //             for (const MSCrossSection& cs : det->getEntries()) {
    //                 shape.push_back(cs.myLane->getShape().positionAtOffset(cs.myPosition));
    //             }
    //             for (const MSCrossSection& cs : det->getExits()) {
    //                 shape.push_back(cs.myLane->getShape().positionAtOffset(cs.myPosition));
    //             }
    //             break;
    //         }
    //         case libsumo::CMD_SUBSCRIBE_PARKINGAREA_CONTEXT: {
    //             MSStoppingPlace* const stop = getStoppingPlace(id, SUMO_TAG_PARKING_AREA);
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getBeginLanePosition()));
    //             shape.push_back(stop->getLane().getShape().positionAtOffset(stop->getEndLanePosition()));
    //             break;
    //         }
    //         case libsumo::CMD_SUBSCRIBE_PERSON_CONTEXT:
    //             Person::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_POI_CONTEXT:
    //             POI::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_POLYGON_CONTEXT:
    //             Polygon::storeShape(id, shape);
    //             break;
    //         case libsumo::CMD_SUBSCRIBE_VEHICLE_CONTEXT:
    //             Vehicle::storeShape(id, shape);
    //             break;
    //         default:
    //             Simulation::storeShape(shape);
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_object_shape(domain: i32, id: &str, shape: &mut PositionVector) {
        match domain {
            CMD_SUBSCRIBE_BUSSTOP_CONTEXT | CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT | CMD_SUBSCRIBE_PARKINGAREA_CONTEXT => {
                let tag = match domain {
                    CMD_SUBSCRIBE_BUSSTOP_CONTEXT => SUMO_TAG_BUS_STOP,
                    CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT => SUMO_TAG_CHARGING_STATION,
                    _ => SUMO_TAG_PARKING_AREA,
                };
                let stop = Helper::get_stopping_place(id, tag);
                shape.push(stop.get_lane().get_shape(false).position_at_offset(stop.get_begin_lane_position(), 0.0));
                shape.push(stop.get_lane().get_shape(false).position_at_offset(stop.get_end_lane_position(), 0.0));
            }
            CMD_SUBSCRIBE_CALIBRATOR_CONTEXT => {
                let calib = Calibrator::get_calibrator(id);
                shape.push(calib.get_lane().get_shape(false)[0]);
            }
            CMD_SUBSCRIBE_EDGE_CONTEXT => Edge::store_shape(id, shape),
            CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT => InductionLoop::store_shape(id, shape),
            CMD_SUBSCRIBE_JUNCTION_CONTEXT => Junction::store_shape(id, shape),
            CMD_SUBSCRIBE_LANE_CONTEXT => Lane::store_shape(id, shape),
            CMD_SUBSCRIBE_LANEAREA_CONTEXT => LaneArea::store_shape(id, shape),
            CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT => {
                let det = MultiEntryExit::get_detector(id);
                for cs in det.get_entries().iter() {
                    shape.push(cs.myLane.get_shape(false).position_at_offset(cs.myPosition, 0.0));
                }
                for cs in det.get_exits().iter() {
                    shape.push(cs.myLane.get_shape(false).position_at_offset(cs.myPosition, 0.0));
                }
            }
            CMD_SUBSCRIBE_PERSON_CONTEXT => Person::store_shape(id, shape),
            CMD_SUBSCRIBE_POI_CONTEXT => POI::store_shape(id, shape),
            CMD_SUBSCRIBE_POLYGON_CONTEXT => Polygon::store_shape(id, shape),
            CMD_SUBSCRIBE_VEHICLE_CONTEXT => Vehicle::store_shape(id, shape),
            _ => Simulation::store_shape(shape),
        }
    }

    /// C++: `void Helper::collectObjectsInRange(int, const PositionVector &, double, std::set<const Named *> &)` [static]
    // void
    // Helper::collectObjectsInRange(int domain, const PositionVector& shape, double range, std::set<const Named*>& into) {
    //     const Boundary b = shape.getBoxBoundary().grow(range);
    //     const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //     const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //     switch (domain) {
    //         case libsumo::CMD_GET_BUSSTOP_VARIABLE:
    //             for (const auto& stop : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_BUS_STOP)) {
    //                 if (shape.distance2D(stop.second->getCenterPos()) <= range) {
    //                     into.insert(stop.second);
    //                 }
    //             }
    //             break;
    //         case libsumo::CMD_GET_CHARGINGSTATION_VARIABLE:
    //             for (const auto& stop : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_CHARGING_STATION)) {
    //                 if (shape.distance2D(stop.second->getCenterPos()) <= range) {
    //                     into.insert(stop.second);
    //                 }
    //             }
    //             break;
    //         case libsumo::CMD_GET_CALIBRATOR_VARIABLE:
    //             for (const auto& calib : MSCalibrator::getInstances()) {
    //                 if (shape.distance2D(calib.second->getLane()->getShape()[0]) <= range) {
    //                     into.insert(calib.second);
    //                 }
    //             }
    //             break;
    //         case libsumo::CMD_GET_INDUCTIONLOOP_VARIABLE:
    //             InductionLoop::getTree()->Search(cmin, cmax, Named::StoringVisitor(into));
    //             break;
    //         case libsumo::CMD_GET_JUNCTION_VARIABLE:
    //             Junction::getTree()->Search(cmin, cmax, Named::StoringVisitor(into));
    //             break;
    //         case libsumo::CMD_GET_LANEAREA_VARIABLE:
    //             LaneArea::getTree()->Search(cmin, cmax, Named::StoringVisitor(into));
    //             break;
    //         case libsumo::CMD_GET_PARKINGAREA_VARIABLE: {
    //             for (const auto& stop : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_PARKING_AREA)) {
    //                 if (shape.distance2D(stop.second->getCenterPos()) <= range) {
    //                     into.insert(stop.second);
    //                 }
    //             }
    //             break;
    //         }
    //         case libsumo::CMD_GET_POI_VARIABLE:
    //             POI::getTree()->Search(cmin, cmax, Named::StoringVisitor(into));
    //             break;
    //         case libsumo::CMD_GET_POLYGON_VARIABLE:
    //             Polygon::getTree()->Search(cmin, cmax, Named::StoringVisitor(into));
    //             break;
    //         case libsumo::CMD_GET_EDGE_VARIABLE:
    //         case libsumo::CMD_GET_LANE_VARIABLE:
    //         case libsumo::CMD_GET_PERSON_VARIABLE:
    //         case libsumo::CMD_GET_VEHICLE_VARIABLE: {
    //             if (myLaneTree == nullptr) {
    //                 myLaneTree = new LANE_RTREE_QUAL(&MSLane::visit);
    //                 MSLane::fill(*myLaneTree);
    //             }
    //             MSLane::StoringVisitor lsv(into, shape, range, domain);
    //             myLaneTree->Search(cmin, cmax, lsv);
    //         }
    //         break;
    //         default:
    //             throw TraCIException("Infeasible context domain (" + toString(domain) + ")");
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_objects_in_range(domain: i32, shape: &PositionVector, range: f64, into: &mut BTreeSet<Ptr<Named>>) {
        let mut b = shape.get_box_boundary();
        b.grow(range);
        let cmin = [b.xmin() as f32, b.ymin() as f32];
        let cmax = [b.xmax() as f32, b.ymax() as f32];
        match domain {
            CMD_GET_BUSSTOP_VARIABLE | CMD_GET_CHARGINGSTATION_VARIABLE | CMD_GET_PARKINGAREA_VARIABLE => {
                // shortcut: MSNet::get_stopping_places returns Unmapped (NamedObjectCont), iteration needs ms_net.rs mapped
                panic!("shortcut: MSNet::get_stopping_places is not mapped yet, cannot collect stopping places in range");
            }
            CMD_GET_CALIBRATOR_VARIABLE => {
                // shortcut: MSCalibrator::getInstances is not mapped yet
                panic!("shortcut: MSCalibrator::getInstances is not mapped yet, cannot collect calibrators in range");
            }
            CMD_GET_INDUCTIONLOOP_VARIABLE | CMD_GET_JUNCTION_VARIABLE | CMD_GET_LANEAREA_VARIABLE | CMD_GET_POI_VARIABLE | CMD_GET_POLYGON_VARIABLE => {
                // shortcut: NamedRTree is Unmapped in the detector and shape ports, Search cannot be called yet
                let _ = (cmin, cmax);
                panic!("shortcut: NamedRTree is Unmapped, Search not available yet");
            }
            CMD_GET_EDGE_VARIABLE | CMD_GET_LANE_VARIABLE | CMD_GET_PERSON_VARIABLE | CMD_GET_VEHICLE_VARIABLE => {
                // shortcut: the MSLane R-tree (myLaneTree) is not ported yet
                panic!("shortcut: MSLane R-tree is not ported yet, cannot search lanes in range");
            }
            _ => panic!("Infeasible context domain ({})", domain),
        }
    }

    /// C++: `void Helper::collectObjectIDsInRange(int, const PositionVector &, double, std::set<std::string> &)` [static]
    // void
    // Helper::collectObjectIDsInRange(int domain, const PositionVector& shape, double range, std::set<std::string>& into) {
    //     std::set<const Named*> objects;
    //     collectObjectsInRange(domain, shape, range, objects);
    //     for (const Named* obj : objects) {
    //         into.insert(obj->getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn collect_object_i_ds_in_range(domain: i32, shape: &PositionVector, range: f64, into: &mut BTreeSet<String>) {
        let mut objects: BTreeSet<Ptr<Named>> = BTreeSet::new();
        Helper::collect_objects_in_range(domain, shape, range, &mut objects);
        for obj in objects.iter() {
            into.insert(obj.get_id().to_string());
        }
    }

    /// C++: `void Helper::applySubscriptionFilters(const Subscription &, std::set<std::string> &)` [static]
    /// Filter the given ID-Set (which was obtained from an R-Tree search) according to the filters set by the subscription or firstly build the object ID list if the filters rather demand searching along the road network than considering a geometric range.
    // void
    // Helper::applySubscriptionFilters(const Subscription& s, std::set<std::string>& objIDs) {
    // #ifdef DEBUG_SURROUNDING
    //     MSBaseVehicle* _veh = getVehicle(s.id);
    //     std::cout << SIMTIME << " applySubscriptionFilters for vehicle '" << _veh->getID() << "' on lane '" << _veh->getLane()->getID() << "'"
    //               << "\n       on edge '" << _veh->getLane()->getEdge().getID() << "' (" << toString(_veh->getLane()->getEdge().getLanes()) << ")\n"
    //               << "objIDs = " << toString(objIDs) << std::endl;
    // #endif
    //
    //     if (s.activeFilters == 0) {
    //         // No filters set
    //         return;
    //     }
    //
    //     MSVehicle* v = dynamic_cast<MSVehicle*>(getVehicle(s.id));
    //
    //     // Whether vehicles on opposite lanes shall be taken into account
    //     const bool disregardOppositeDirection = (s.activeFilters & SUBS_FILTER_NOOPPOSITE) != 0;
    //
    //     // Check filter specification consistency
    //     if (disregardOppositeDirection && (s.activeFilters & SUBS_FILTER_NO_RTREE) == 0) {
    //         WRITE_WARNINGF(TL("Ignoring veh '%' no-opposite subscription filter for geographic range object collection. Consider using the 'lanes' filter."), v->getID())
    //     }
    //     // TODO: Treat case, where ego vehicle is currently on opposite lane
    //
    //     std::set<const SUMOTrafficObject*> vehs;
    //     if (s.activeFilters & SUBS_FILTER_NO_RTREE) {
    //         // Set defaults for upstream/downstream/lateral distances
    //         double downstreamDist = s.range, upstreamDist = s.range, lateralDist = s.range;
    //         if (s.activeFilters & SUBS_FILTER_DOWNSTREAM_DIST) {
    //             // Specifies maximal downstream distance for vehicles in context subscription result
    //             downstreamDist = s.filterDownstreamDist;
    //         }
    //         if (s.activeFilters & SUBS_FILTER_UPSTREAM_DIST) {
    //             // Specifies maximal downstream distance for vehicles in context subscription result
    //             upstreamDist = s.filterUpstreamDist;
    //         }
    //         if (s.activeFilters & SUBS_FILTER_LATERAL_DIST) {
    //             // Specifies maximal lateral distance for vehicles in context subscription result
    //             lateralDist = s.filterLateralDist;
    //         }
    //         if (v == nullptr) {
    //             throw TraCIException("Subscription filter not yet implemented for meso vehicle");
    //         }
    //         if (!v->isOnRoad()) {
    //             return;
    //         }
    //         const MSLane* vehLane = v->getLane();
    //         if (vehLane == nullptr) {
    //             return;
    //         }
    //         MSEdge* vehEdge = &vehLane->getEdge();
    //         std::vector<int> filterLanes;
    //         if ((s.activeFilters & SUBS_FILTER_LANES) == 0) {
    //             // No lane indices are specified (but downstream and/or upstream distance)
    //             //   -> use only vehicle's current lane as origin for the searches
    //             filterLanes = {0};
    //             // Lane indices must be specified when leader/follower information is requested.
    //             assert((s.activeFilters & SUBS_FILTER_LEAD_FOLLOW) == 0);
    //         } else {
    //             filterLanes = s.filterLanes;
    //         }
    //
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "Filter lanes: " << toString(filterLanes) << std::endl;
    //         std::cout << "Downstream distance: " << downstreamDist << std::endl;
    //         std::cout << "Upstream distance: " << upstreamDist << std::endl;
    //         std::cout << "Lateral distance: " << lateralDist << std::endl;
    // #endif
    //
    //         if ((s.activeFilters & SUBS_FILTER_MANEUVER) != 0) {
    //             // Maneuver filters disables road net search for all surrounding vehicles
    //             if ((s.activeFilters & SUBS_FILTER_LEAD_FOLLOW) != 0) {
    //                 // Return leader and follower on the specified lanes in context subscription result.
    //                 for (int offset : filterLanes) {
    //                     MSLane* lane = v->getLane()->getParallelLane(offset, false);
    //                     if (lane != nullptr) {
    //                         // this is a non-opposite lane
    //                         MSVehicle* leader = lane->getLeader(v, v->getPositionOnLane(), v->getBestLanesContinuation(lane), downstreamDist).first;
    //                         MSVehicle* follower = lane->getFollower(v, v->getPositionOnLane(), upstreamDist,
    //                                                                 MSLane::MinorLinkMode::FOLLOW_ALWAYS).first;
    //                         vehs.insert(vehs.end(), leader);
    //                         vehs.insert(vehs.end(), follower);
    //
    // #ifdef DEBUG_SURROUNDING
    //                         std::cout << "Lane at index " << offset << ": '" << lane->getID() << std::endl;
    //                         std::cout << "Leader: '" << (leader != nullptr ? leader->getID() : "NULL") << "'" << std::endl;
    //                         std::cout << "Follower: '" << (follower != nullptr ? follower->getID() : "NULL") << "'" << std::endl;
    // #endif
    //
    //                     } else if (!disregardOppositeDirection && offset > 0) { // TODO: offset<0 may indicate opposite query when vehicle is on opposite itself
    //                         // check whether ix points to an opposite lane
    //                         const MSEdge* opposite = vehEdge->getOppositeEdge();
    //                         if (opposite == nullptr) {
    // #ifdef DEBUG_SURROUNDING
    //                             std::cout << "No lane at index " << offset << std::endl;
    // #endif
    //                             // no opposite edge
    //                             continue;
    //                         }
    //                         // Index of opposite lane at relative offset
    //                         const int ix_opposite = (int)opposite->getLanes().size() - 1 - (vehLane->getIndex() + offset - (int)vehEdge->getLanes().size());
    //                         if (ix_opposite < 0) {
    // #ifdef DEBUG_SURROUNDING
    //                             std::cout << "No lane on opposite at index " << ix_opposite << std::endl;
    // #endif
    //                             // no opposite edge
    //                             continue;
    //                         }
    //                         lane = opposite->getLanes()[ix_opposite];
    //                         // Search vehs along opposite lanes (swap upstream and downstream distance)
    //                         // XXX transformations for curved geometries
    //                         double posOnOpposite = MAX2(0., opposite->getLength() - v->getPositionOnLane());
    //                         // Get leader on opposite
    //                         vehs.insert(vehs.end(), lane->getFollower(v, posOnOpposite, downstreamDist, MSLane::MinorLinkMode::FOLLOW_NEVER).first);
    //                         // Get follower (no search on consecutive lanes
    //                         vehs.insert(vehs.end(), lane->getLeader(v, posOnOpposite - v->getLength(), std::vector<MSLane*>()).first);
    //                     }
    //                 }
    //             }
    //
    //             if (s.activeFilters & SUBS_FILTER_TURN) {
    //                 applySubscriptionFilterTurn(s, vehs);
    //                 if (s.activeFilters & SUBS_FILTER_LANES) {
    //                     applySubscriptionFilterLanes(s, vehs, filterLanes, downstreamDist, upstreamDist, disregardOppositeDirection);
    //                 }
    //                 if (s.activeFilters & SUBS_FILTER_LATERAL_DIST) {
    //                     applySubscriptionFilterLateralDistance(s, vehs, downstreamDist, upstreamDist, lateralDist);
    //                 }
    //             }
    // #ifdef DEBUG_SURROUNDING
    //             std::cout << SIMTIME << " applySubscriptionFilters() for veh '" << v->getID() << "'. Found the following vehicles:\n";
    //             for (auto veh : vehs) {
    //                 debugPrint(veh);
    //             }
    // #endif
    //         } else if (s.activeFilters & SUBS_FILTER_LATERAL_DIST) {
    //             applySubscriptionFilterLateralDistance(s, vehs, downstreamDist, upstreamDist, lateralDist);
    //         } else {
    //             // No maneuver or lateral distance filters requested, but only lanes filter (directly, or indirectly by specifying downstream or upstream distance)
    //             applySubscriptionFilterLanes(s, vehs, filterLanes, downstreamDist, upstreamDist, disregardOppositeDirection);
    //         }
    //         // Write vehs IDs in objIDs
    //         for (const SUMOTrafficObject* veh : vehs) {
    //             if (veh != nullptr) {
    //                 objIDs.insert(objIDs.end(), veh->getID());
    //             }
    //         }
    //     }
    //
    //     if (s.activeFilters & SUBS_FILTER_VCLASS) {
    //         // Only return vehicles of the given vClass in context subscription result
    //         auto i = objIDs.begin();
    //         while (i != objIDs.end()) {
    //             MSBaseVehicle* veh = getVehicle(*i);
    //             if ((veh->getVehicleType().getVehicleClass() & s.filterVClasses) == 0) {
    //                 i = objIDs.erase(i);
    //             } else {
    //                 ++i;
    //             }
    //         }
    //     }
    //     if (s.activeFilters & SUBS_FILTER_VTYPE) {
    //         // Only return vehicles of the given vType in context subscription result
    //         auto i = objIDs.begin();
    //         while (i != objIDs.end()) {
    //             MSBaseVehicle* veh = getVehicle(*i);
    //             if (s.filterVTypes.find(veh->getVehicleType().getID()) == s.filterVTypes.end()) {
    //                 i = objIDs.erase(i);
    //             } else {
    //                 ++i;
    //             }
    //         }
    //     }
    //     if (s.activeFilters & SUBS_FILTER_FIELD_OF_VISION) {
    //         // Only return vehicles within field of vision in context subscription result
    //         applySubscriptionFilterFieldOfVision(s, objIDs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filters(s: &Subscription, objIDs: &mut BTreeSet<String>) {
        if s.activeFilters == 0 {
            // No filters set
            return;
        }
        let v: Ptr<MSVehicle> = Helper::get_vehicle(&s.id).cast::<MSVehicle>();
        // Whether vehicles on opposite lanes shall be taken into account
        let disregardOppositeDirection = (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_NOOPPOSITE)) != 0;
        // Check filter specification consistency
        if disregardOppositeDirection && (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_NO_RTREE)) == 0 {
            utils::write_warningf!("Ignoring veh '%' no-opposite subscription filter for geographic range object collection. Consider using the 'lanes' filter.", v.get_id());
        }
        // TODO: Treat case, where ego vehicle is currently on opposite lane

        let mut vehs: BTreeSet<Ptr<SUMOTrafficObject>> = BTreeSet::new();
        if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_NO_RTREE)) != 0 {
            // Set defaults for upstream/downstream/lateral distances
            let mut downstreamDist = s.range;
            let mut upstreamDist = s.range;
            let mut lateralDist = s.range;
            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_DOWNSTREAM_DIST)) != 0 {
                // Specifies maximal downstream distance for vehicles in context subscription result
                downstreamDist = s.filterDownstreamDist;
            }
            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_UPSTREAM_DIST)) != 0 {
                // Specifies maximal downstream distance for vehicles in context subscription result
                upstreamDist = s.filterUpstreamDist;
            }
            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LATERAL_DIST)) != 0 {
                // Specifies maximal lateral distance for vehicles in context subscription result
                lateralDist = s.filterLateralDist;
            }
            if v.is_null() {
                panic!("Subscription filter not yet implemented for meso vehicle");
            }
            if !v.is_on_road() {
                return;
            }
            let vehLane = v.get_lane();
            if vehLane.is_null() {
                return;
            }
            let vehEdge = Ptr::from_mut(vehLane.m().get_edge());
            let mut filterLanes: Vec<i32>;
            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LANES)) == 0 {
                // No lane indices are specified (but downstream and/or upstream distance)
                //   -> use only vehicle's current lane as origin for the searches
                filterLanes = vec![0];
                // Lane indices must be specified when leader/follower information is requested.
                assert!((s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LEAD_FOLLOW)) == 0);
            } else {
                filterLanes = s.filterLanes.clone();
            }

            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_MANEUVER)) != 0 {
                // Maneuver filters disables road net search for all surrounding vehicles
                if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LEAD_FOLLOW)) != 0 {
                    // Return leader and follower on the specified lanes in context subscription result.
                    for &offset in filterLanes.iter() {
                        let lane = vehLane.get_parallel_lane(offset, false);
                        if !lane.is_null() {
                            // this is a non-opposite lane
                            let leader = lane.get_leader(v, v.get_position_on_lane(), v.get_best_lanes_continuation(), downstreamDist, false).0;
                            let follower = lane.get_follower(v, v.get_position_on_lane(), upstreamDist, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false).0;
                            vehs.insert(leader.cast::<SUMOTrafficObject>());
                            vehs.insert(follower.cast::<SUMOTrafficObject>());
                        } else if !disregardOppositeDirection && offset > 0 {
                            // TODO: offset<0 may indicate opposite query when vehicle is on opposite itself
                            // check whether ix points to an opposite lane
                            let opposite = vehEdge.get_opposite_edge();
                            if opposite.is_null() {
                                // no opposite edge
                                continue;
                            }
                            // Index of opposite lane at relative offset
                            let ix_opposite = opposite.get_lanes().len() as i32 - 1 - (vehLane.get_index() + offset - vehEdge.get_lanes().len() as i32);
                            if ix_opposite < 0 {
                                // no opposite edge
                                continue;
                            }
                            let lane = opposite.get_lanes()[ix_opposite as usize];
                            // Search vehs along opposite lanes (swap upstream and downstream distance)
                            // XXX transformations for curved geometries
                            let posOnOpposite = f64::max(0., opposite.get_length() - v.get_position_on_lane());
                            // Get leader on opposite
                            vehs.insert(lane.get_follower(v, posOnOpposite, downstreamDist, MSLane_MinorLinkMode::FOLLOW_NEVER, false).0.cast::<SUMOTrafficObject>());
                            // Get follower (no search on consecutive lanes
                            vehs.insert(lane.get_leader(v, posOnOpposite - v.get_length(), &Vec::new(), -1., false).0.cast::<SUMOTrafficObject>());
                        }
                    }
                }

                if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_TURN)) != 0 {
                    Helper::apply_subscription_filter_turn(s, &mut vehs);
                    if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LANES)) != 0 {
                        Helper::apply_subscription_filter_lanes(s, &mut vehs, &mut filterLanes, downstreamDist, upstreamDist, disregardOppositeDirection);
                    }
                    if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LATERAL_DIST)) != 0 {
                        Helper::apply_subscription_filter_lateral_distance(s, &mut vehs, downstreamDist, upstreamDist, lateralDist);
                    }
                }
            } else if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_LATERAL_DIST)) != 0 {
                Helper::apply_subscription_filter_lateral_distance(s, &mut vehs, downstreamDist, upstreamDist, lateralDist);
            } else {
                // No maneuver or lateral distance filters requested, but only lanes filter (directly, or indirectly by specifying downstream or upstream distance)
                Helper::apply_subscription_filter_lanes(s, &mut vehs, &mut filterLanes, downstreamDist, upstreamDist, disregardOppositeDirection);
            }
            // Write vehs IDs in objIDs
            for veh in vehs.iter() {
                if !veh.is_null() {
                    objIDs.insert(veh.get_id().to_string());
                }
            }
        }

        if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_VCLASS)) != 0 {
            // Only return vehicles of the given vClass in context subscription result
            objIDs.retain(|id| {
                let veh = Helper::get_vehicle(id);
                (veh.get_vehicle_type().get_vehicle_class() & s.filterVClasses) != 0
            });
        }
        if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_VTYPE)) != 0 {
            // Only return vehicles of the given vType in context subscription result
            objIDs.retain(|id| {
                let veh = Helper::get_vehicle(id);
                s.filterVTypes.contains(veh.get_vehicle_type().get_id())
            });
        }
        if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_FIELD_OF_VISION)) != 0 {
            // Only return vehicles within field of vision in context subscription result
            Helper::apply_subscription_filter_field_of_vision(s, objIDs);
        }
    }

    /// C++: `void Helper::applySubscriptionFilterLanes(const Subscription &, std::set<const SUMOTrafficObject *> &, std::vector<int> &, double, double, bool)` [static]
    /// Apply the subscription filter "lanes": Only return vehicles on list of lanes relative to ego vehicle. Search all predecessor and successor lanes along the road network up until upstreamDist and downstreamDist, respectively,
    // void
    // Helper::applySubscriptionFilterLanes(const Subscription& s, std::set<const SUMOTrafficObject*>& vehs, std::vector<int>& filterLanes, double downstreamDist,
    //                                      double upstreamDist, bool disregardOppositeDirection) {
    //     if (!s.isVehicleToVehicleContextSubscription()) {
    //         WRITE_WARNINGF(TL("Lanes filter is only feasible for context domain 'vehicle' (current is '%'), ignoring filter..."), toHex(s.contextDomain, 2));
    //         return;
    //     }
    //     assert(filterLanes.size() > 0);
    //     MSVehicle* v = dynamic_cast<MSVehicle*>(getVehicle(s.id));
    //     const MSLane* vehLane = v->getLane();
    //     MSEdge* vehEdge = &vehLane->getEdge();
    //     // This is to remember the lanes checked in the driving direction of the vehicle (their opposites can be added in a second pass)
    //     auto checkedLanesInDrivingDir = std::make_shared<LaneCoverageInfo>();
    //     for (int offset : filterLanes) {
    //         MSLane* lane = vehLane->getParallelLane(offset, false);
    //         if (lane != nullptr) {
    // #ifdef DEBUG_SURROUNDING
    //             std::cout << "Checking for surrounding vehicles starting on lane '" << lane->getID() << "' at index " << offset << std::endl;
    // #endif
    //             // Search vehs along this lane
    //             // (Coverage info is collected per origin lane since lanes reached from neighboring lanes may have different distances
    //             // and aborting at previously scanned when coming from a closer origin may prevent scanning of parts that should be included.)
    //             std::shared_ptr<LaneCoverageInfo> checkedLanes = std::make_shared<LaneCoverageInfo>();
    //             const std::set<MSVehicle*> new_vehs =
    //                 lane->getSurroundingVehicles(v->getPositionOnLane(), downstreamDist, upstreamDist + v->getLength(), checkedLanes);
    //             vehs.insert(new_vehs.begin(), new_vehs.end());
    //             fuseLaneCoverage(checkedLanesInDrivingDir, checkedLanes);
    //         } else if (!disregardOppositeDirection && offset > 0) {
    //             // Check opposite edge, too
    //             assert(vehLane->getIndex() + offset >= (int)vehEdge->getLanes().size());  // index points beyond this edge
    //             const MSEdge* opposite = vehEdge->getOppositeEdge();
    //             if (opposite == nullptr) {
    // #ifdef DEBUG_SURROUNDING
    //                 std::cout << "No opposite edge, thus no lane at index " << offset << std::endl;
    // #endif
    //                 // no opposite edge
    //                 continue;
    //             }
    //             // Index of opposite lane at relative offset
    //             const int ix_opposite = (int)opposite->getLanes().size() - 1 - (vehLane->getIndex() + offset - (int)vehEdge->getLanes().size());
    //             if (ix_opposite < 0) {
    // #ifdef DEBUG_SURROUNDING
    //                 std::cout << "No lane on opposite at index " << ix_opposite << std::endl;
    // #endif
    //                 // no opposite edge
    //                 continue;
    //             }
    //             lane = opposite->getLanes()[ix_opposite];
    //             // Search vehs along opposite lanes (swap upstream and downstream distance)
    //             const std::set<MSVehicle*> new_vehs = lane->getSurroundingVehicles(lane->getLength() - v->getPositionOnLane(), upstreamDist + v->getLength(),
    //                                                   downstreamDist, std::make_shared<LaneCoverageInfo>());
    //             vehs.insert(new_vehs.begin(), new_vehs.end());
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         else {
    //             std::cout << "No lane at index " << offset << std::endl;
    //         }
    // #endif
    //
    //         if (!disregardOppositeDirection) {
    //             // If opposite should be checked, do this for each lane of the search tree in checkedLanesInDrivingDir
    //             // (For instance, some opposite lanes of these would not be obtained if the ego lane does not have an opposite.)
    //
    //             // Number of opposite lanes to be checked (assumes filterLanes.size()>0, see assertion above) determined as hypothetical offset
    //             // overlap into opposing edge from the vehicle's current lane.
    //             // TODO: offset<0 may indicate opposite query when vehicle is on opposite itself (-> use min_element(filterLanes...) instead, etc)
    //             const int nOpp = MAX2(0, (*std::max_element(filterLanes.begin(), filterLanes.end())) - ((int)vehEdge->getLanes().size() - 1 - vehLane->getIndex()));
    //             // Collect vehicles from opposite lanes
    //             if (nOpp > 0) {
    //                 for (auto& laneCov : *checkedLanesInDrivingDir) {
    //                     const MSLane* const l = laneCov.first;
    //                     if (l == nullptr || l->getEdge().getOppositeEdge() == nullptr) {
    //                         continue;
    //                     }
    //                     const MSEdge* opposite = l->getEdge().getOppositeEdge();
    //                     const std::pair<double, double>& range = laneCov.second;
    //                     auto leftMostOppositeLaneIt = opposite->getLanes().rbegin();
    //                     for (auto oppositeLaneIt = leftMostOppositeLaneIt; oppositeLaneIt != opposite->getLanes().rend(); ++oppositeLaneIt) {
    //                         if ((int)(oppositeLaneIt - leftMostOppositeLaneIt) == nOpp) {
    //                             break;
    //                         }
    //                         // Add vehicles from corresponding range on opposite direction
    //                         const MSLane* oppositeLane = *oppositeLaneIt;
    //                         auto new_vehs = oppositeLane->getVehiclesInRange(l->getLength() - range.second, l->getLength() - range.first);
    //                         vehs.insert(new_vehs.begin(), new_vehs.end());
    //                     }
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << SIMTIME << " applySubscriptionFilterLanes() for veh '" << v->getID() << "', lane offset '" << offset << "'. Found the following vehicles so far:\n";
    //         for (auto veh : vehs) {
    //             debugPrint(veh);
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filter_lanes(s: &Subscription, vehs: &mut BTreeSet<Ptr<SUMOTrafficObject>>, filterLanes: &mut Vec<i32>, downstreamDist: f64, upstreamDist: f64, disregardOppositeDirection: bool) {
        if !s.is_vehicle_to_vehicle_context_subscription() {
            utils::write_warningf!("Lanes filter is only feasible for context domain 'vehicle' (current is '%'), ignoring filter...", to_hex(s.contextDomain as u64, 2, 8));
            return;
        }
        assert!(filterLanes.len() > 0);
        let v: Ptr<MSVehicle> = Helper::get_vehicle(&s.id).cast::<MSVehicle>();
        let vehLane = v.get_lane();
        let vehEdge = Ptr::from_mut(vehLane.m().get_edge());
        // This is to remember the lanes checked in the driving direction of the vehicle (their opposites can be added in a second pass)
        let checkedLanesInDrivingDir: Rc<LaneCoverageInfo> = Rc::new(LaneCoverageInfo::new());
        for &offset in filterLanes.iter() {
            let mut lane = vehLane.get_parallel_lane(offset, false);
            if !lane.is_null() {
                // Search vehs along this lane
                // (Coverage info is collected per origin lane since lanes reached from neighboring lanes may have different distances
                // and aborting at previously scanned when coming from a closer origin may prevent scanning of parts that should be included.)
                let checkedLanes: Rc<std::cell::RefCell<LaneCoverageInfo>> = Rc::new(std::cell::RefCell::new(LaneCoverageInfo::new()));
                let newVehs = lane.get_surrounding_vehicles(v.get_position_on_lane(), downstreamDist, upstreamDist + v.get_length(), checkedLanes.clone());
                for nv in newVehs.iter() {
                    vehs.insert(nv.cast::<SUMOTrafficObject>());
                }
                Helper::fuse_lane_coverage(checkedLanesInDrivingDir.clone(), Rc::new(checkedLanes.borrow().clone()));
            } else if !disregardOppositeDirection && offset > 0 {
                // Check opposite edge, too
                // index points beyond this edge
                let opposite = vehEdge.get_opposite_edge();
                if opposite.is_null() {
                    // no opposite edge
                    continue;
                }
                // Index of opposite lane at relative offset
                let ix_opposite = opposite.get_lanes().len() as i32 - 1 - (vehLane.get_index() + offset - vehEdge.get_lanes().len() as i32);
                if ix_opposite < 0 {
                    // no opposite edge
                    continue;
                }
                lane = opposite.get_lanes()[ix_opposite as usize];
                // Search vehs along opposite lanes (swap upstream and downstream distance)
                let newVehs = lane.get_surrounding_vehicles(lane.get_length() - v.get_position_on_lane(), upstreamDist + v.get_length(), downstreamDist, Rc::new(std::cell::RefCell::new(LaneCoverageInfo::new())));
                for nv in newVehs.iter() {
                    vehs.insert(nv.cast::<SUMOTrafficObject>());
                }
            }

            if !disregardOppositeDirection {
                // If opposite should be checked, do this for each lane of the search tree in checkedLanesInDrivingDir
                // (For instance, some opposite lanes of these would not be obtained if the ego lane does not have an opposite.)

                // Number of opposite lanes to be checked (assumes filterLanes.size()>0, see assertion above) determined as hypothetical offset
                // overlap into opposing edge from the vehicle's current lane.
                let maxOffset = *filterLanes.iter().max().unwrap();
                let nOpp = i32::max(0, maxOffset - (vehEdge.get_lanes().len() as i32 - 1 - vehLane.get_index()));
                // Collect vehicles from opposite lanes
                if nOpp > 0 {
                    for (l, range) in checkedLanesInDrivingDir.iter() {
                        let l = *l;
                        if l.is_null() || l.get_edge().get_opposite_edge().is_null() {
                            continue;
                        }
                        let opposite = l.get_edge().get_opposite_edge();
                        let leftMostOppositeLaneIt = opposite.get_lanes().len();
                        // iterate opposite lanes from the left-most (last) one, like the C++ reverse iterator
                        let oppLanes = opposite.get_lanes();
                        for k in 0..leftMostOppositeLaneIt {
                            let oppositeLaneIt = leftMostOppositeLaneIt - 1 - k;
                            if k as i32 == nOpp {
                                break;
                            }
                            // Add vehicles from corresponding range on opposite direction
                            let oppositeLane = oppLanes[oppositeLaneIt];
                            let newVehs = oppositeLane.get_vehicles_in_range(l.get_length() - range.1, l.get_length() - range.0);
                            for nv in newVehs.iter() {
                                vehs.insert(nv.cast::<SUMOTrafficObject>());
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++: `void Helper::applySubscriptionFilterTurn(const Subscription &, std::set<const SUMOTrafficObject *> &)` [static]
    /// Apply the subscription filter "turn": Gather upcoming junctions and vialanes within downstream distance and find approaching foes within foeDistToJunction.
    // void
    // Helper::applySubscriptionFilterTurn(const Subscription& s, std::set<const SUMOTrafficObject*>& vehs) {
    //     if (!s.isVehicleToVehicleContextSubscription()) {
    //         WRITE_WARNINGF(TL("Turn filter is only feasible for context domain 'vehicle' (current is '%'), ignoring filter..."), toHex(s.contextDomain, 2));
    //         return;
    //     }
    //     // Get upcoming junctions and vialanes within downstream distance, where foe links exist or at least the link direction is not straight
    //     MSVehicle* v = dynamic_cast<MSVehicle*>(getVehicle(s.id));
    //     const MSLane* lane = v->getLane();
    //     std::vector<const MSLink*> links = lane->getUpcomingLinks(v->getPositionOnLane(), s.filterDownstreamDist, v->getBestLanesContinuation());
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "Applying turn filter for vehicle '" << v->getID() << "'\n Gathering foes ..." << std::endl;
    // #endif
    //     // Iterate through junctions and find approaching foes within foeDistToJunction.
    //     for (auto& l : links) {
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "  On junction '" << l->getJunction()->getID() << "' (no. foe links = " << l->getFoeLinks().size() << "):" << std::endl;
    // #endif
    //         for (auto& foeLane : l->getFoeLanes()) {
    //             if (foeLane->isCrossing()) {
    // #ifdef DEBUG_SURROUNDING
    //                 std::cout << "   skipping crossing foeLane '" << foeLane->getID() << "'" << std::endl;
    // #endif
    //                 continue;
    //             }
    // #ifdef DEBUG_SURROUNDING
    //             std::cout << "   foeLane '" << foeLane->getID() << "'" << std::endl;
    // #endif
    //             // Check vehicles approaching the entry link corresponding to this lane
    //             const MSLink* foeLink = foeLane->getEntryLink();
    //             for (auto& vi : foeLink->getApproaching()) {
    //                 if (vi.second.dist <= s.filterFoeDistToJunction) {
    // #ifdef DEBUG_SURROUNDING
    //                     std::cout << "    Approaching foeLane entry link '" << vi.first->getID() << "'" << std::endl;
    // #endif
    //                     vehs.insert(vehs.end(), dynamic_cast<const MSVehicle*>(vi.first));
    //                 }
    //             }
    //             // add vehicles currently on the junction
    //             for (const MSVehicle* foe : foeLane->getVehiclesSecure()) {
    // #ifdef DEBUG_SURROUNDING
    //                 std::cout << "    On foeLane '" << foe->getID() << "'" << std::endl;
    // #endif
    //                 vehs.insert(vehs.end(), foe);
    //             }
    //             foeLane->releaseVehicles();
    //             for (auto& laneInfo : foeLane->getIncomingLanes()) {
    //                 const MSLane* foeLanePred = laneInfo.lane;
    //                 if (foeLanePred->isInternal()) {
    // #ifdef DEBUG_SURROUNDING
    //                     std::cout << "     foeLanePred '" << foeLanePred->getID() << "'" << std::endl;
    // #endif
    //                     for (const MSVehicle* foe : foeLanePred->getVehiclesSecure()) {
    // #ifdef DEBUG_SURROUNDING
    //                         std::cout << "      On foeLanePred '" << foe->getID() << "'" << std::endl;
    // #endif
    //                         vehs.insert(vehs.end(), foe);
    //                     }
    //                     foeLanePred->releaseVehicles();
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filter_turn(s: &Subscription, vehs: &mut BTreeSet<Ptr<SUMOTrafficObject>>) {
        if !s.is_vehicle_to_vehicle_context_subscription() {
            utils::write_warningf!("Turn filter is only feasible for context domain 'vehicle' (current is '%'), ignoring filter...", to_hex(s.contextDomain as u64, 2, 8));
            return;
        }
        // Get upcoming junctions and vialanes within downstream distance, where foe links exist or at least the link direction is not straight
        let v: Ptr<MSVehicle> = Helper::get_vehicle(&s.id).cast::<MSVehicle>();
        let lane = v.get_lane();
        let links = lane.get_upcoming_links(v.get_position_on_lane(), s.filterDownstreamDist, v.get_best_lanes_continuation());
        // Iterate through junctions and find approaching foes within foeDistToJunction.
        for l in links.iter() {
            for foeLane in l.get_foe_lanes().iter() {
                if foeLane.is_crossing() {
                    // skipping crossing foeLane
                    continue;
                }
                // Check vehicles approaching the entry link corresponding to this lane
                let foeLink = foeLane.get_entry_link();
                // shortcut: MSLink exposes no iterator over its approaching vehicles map yet (getApproaching(veh) only)
                let _ = foeLink;
                panic!("shortcut: MSLink approaching-vehicles iteration is not mapped yet, turn filter unavailable");
            }
        }
    }

    /// C++: `void Helper::applySubscriptionFilterFieldOfVision(const Subscription &, std::set<std::string> &)` [static]
    // void
    // Helper::applySubscriptionFilterFieldOfVision(const Subscription& s, std::set<std::string>& objIDs) {
    //     if (s.filterFieldOfVisionOpeningAngle <= 0. || s.filterFieldOfVisionOpeningAngle >= 360.) {
    //         WRITE_WARNINGF(TL("Field of vision opening angle ('%') should be within interval (0, 360), ignoring filter..."), s.filterFieldOfVisionOpeningAngle);
    //         return;
    //     }
    //
    //     MSBaseVehicle* egoVehicle = getVehicle(s.id);
    //     Position egoPosition = egoVehicle->getPosition();
    //     double openingAngle = DEG2RAD(s.filterFieldOfVisionOpeningAngle);
    //
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "FOVFILTER: ego direction = " << toString(RAD2DEG(egoVehicle->getAngle())) << " (deg)" << std::endl;
    // #endif
    //
    //     auto i = objIDs.begin();
    //     while (i != objIDs.end()) {
    //         if (s.id.compare(*i) == 0) { // skip if this is the ego vehicle
    //             ++i;
    //             continue;
    //         }
    //         SUMOTrafficObject* obj = getTrafficObject(s.contextDomain, *i);
    //         double angleEgoToVeh = egoPosition.angleTo2D(obj->getPosition());
    //         double alpha = GeomHelper::angleDiff(egoVehicle->getAngle(), angleEgoToVeh);
    //
    // #ifdef DEBUG_SURROUNDING
    //         const std::string objType = s.isVehicleToPersonContextSubscription() ? "person" : "veh";
    //         std::cout << "FOVFILTER: " << objType << " '" << *i << "' dist  = " << toString(egoPosition.distanceTo2D(obj->getPosition())) << std::endl;
    //         std::cout << "FOVFILTER: " << objType << " '" << *i << "' alpha = " << toString(RAD2DEG(alpha)) << " (deg)" << std::endl;
    // #endif
    //
    //         if (abs(alpha) > openingAngle * 0.5) {
    //             i = objIDs.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filter_field_of_vision(s: &Subscription, objIDs: &mut BTreeSet<String>) {
        if s.filterFieldOfVisionOpeningAngle <= 0. || s.filterFieldOfVisionOpeningAngle >= 360. {
            utils::write_warningf!("Field of vision opening angle ('%') should be within interval (0, 360), ignoring filter...", s.filterFieldOfVisionOpeningAngle);
            return;
        }

        let egoVehicle = Helper::get_vehicle(&s.id);
        let egoPosition = egoVehicle.get_position(0.0);
        let openingAngle = geom_helper::deg2rad(s.filterFieldOfVisionOpeningAngle);

        let ids: Vec<String> = objIDs.iter().cloned().collect();
        for id in ids.iter() {
            if s.id.as_str().cmp(id.as_str()) == std::cmp::Ordering::Equal {
                // skip if this is the ego vehicle
                continue;
            }
            let obj = Helper::get_traffic_object(s.contextDomain, id);
            let angleEgoToVeh = egoPosition.angle_to2d(&obj.get_position(0.0));
            let alpha = geom_helper::angle_diff(egoVehicle.get_angle(), angleEgoToVeh);
            if alpha.abs() > openingAngle * 0.5 {
                objIDs.remove(id);
            }
        }
    }

    /// C++: `void Helper::applySubscriptionFilterLateralDistance(const Subscription &, std::set<const SUMOTrafficObject *> &, double, double, double)` [static]
    /// Apply the subscription filter "lateral distance": Only return vehicles within the given lateral distance. Search myRoute (right-most lane) upstream and bestLanesCont downstream up until upstreamDist and downstreamDist, respectively.
    // void
    // Helper::applySubscriptionFilterLateralDistance(const Subscription& s, std::set<const SUMOTrafficObject*>& vehs, double downstreamDist, double upstreamDist,
    //         double lateralDist) {
    //     // collect all vehicles within maximum range of interest to get an upper bound
    //     PositionVector vehShape;
    //     findObjectShape(s.commandId, s.id, vehShape);
    //     double range = MAX3(downstreamDist, upstreamDist, lateralDist);
    //     std::set<std::string> objIDs;
    //     collectObjectIDsInRange(s.contextDomain, vehShape, range, objIDs);
    //
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "FILTER_LATERAL_DIST: collected object IDs (range " << range << "):" << std::endl;
    //     for (std::string i : objIDs) {
    //         std::cout << i << std::endl;
    //     }
    // #endif
    //
    //     MSVehicle* v = dynamic_cast<MSVehicle*>(getVehicle(s.id));
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << "FILTER_LATERAL_DIST: myLane is '" << v->getLane()->getID() << "', pos " << v->getPositionOnLane() << std::endl;
    //     std::cout << "FILTER_LATERAL_DIST: opposite lane is '" << v->getLane()->getParallelOpposite()->getID() << "'" << std::endl;
    // #endif
    //     double frontPosOnLane = v->getPositionOnLane();
    //     if (v->getLaneChangeModel().isOpposite()) {
    //         frontPosOnLane = v->getLane()->getOppositePos(frontPosOnLane);
    //     }
    //     // 1st pass: downstream (make sure that the whole length of the vehicle is included in the match)
    //     const double backPosOnLane = MAX2(0.0, frontPosOnLane - v->getVehicleType().getLength());
    //     applySubscriptionFilterLateralDistanceSinglePass(s, objIDs, vehs, v->getUpcomingLanesUntil(downstreamDist), backPosOnLane, v->getLateralPositionOnLane(),
    //             true);
    //     // 2nd pass: upstream
    //     applySubscriptionFilterLateralDistanceSinglePass(s, objIDs, vehs, v->getPastLanesUntil(upstreamDist), frontPosOnLane, v->getLateralPositionOnLane(), false);
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filter_lateral_distance(s: &Subscription, vehs: &mut BTreeSet<Ptr<SUMOTrafficObject>>, downstreamDist: f64, upstreamDist: f64, lateralDist: f64) {
        // collect all vehicles within maximum range of interest to get an upper bound
        let mut vehShape = PositionVector(Vec::new());
        Helper::find_object_shape(s.commandId, &s.id, &mut vehShape);
        let range = f64::max(f64::max(downstreamDist, upstreamDist), lateralDist);
        let mut objIDs: BTreeSet<String> = BTreeSet::new();
        Helper::collect_object_i_ds_in_range(s.contextDomain, &vehShape, range, &mut objIDs);

        let v: Ptr<MSVehicle> = Helper::get_vehicle(&s.id).cast::<MSVehicle>();
        let mut frontPosOnLane = v.get_position_on_lane();
        if v.m().get_lane_change_model().is_opposite() {
            frontPosOnLane = v.get_lane().get_opposite_pos(frontPosOnLane);
        }
        // 1st pass: downstream (make sure that the whole length of the vehicle is included in the match)
        let backPosOnLane = f64::max(0.0, frontPosOnLane - v.get_vehicle_type().get_length());
        Helper::apply_subscription_filter_lateral_distance_single_pass(s, &mut objIDs, vehs, &v.get_upcoming_lanes_until(downstreamDist), backPosOnLane, v.get_lateral_position_on_lane(), true);
        // 2nd pass: upstream
        Helper::apply_subscription_filter_lateral_distance_single_pass(s, &mut objIDs, vehs, &v.get_past_lanes_until(upstreamDist), frontPosOnLane, v.get_lateral_position_on_lane(), false);
    }

    /// C++: `void Helper::applySubscriptionFilterLateralDistanceSinglePass(const Subscription &, std::set<std::string> &, std::set<const SUMOTrafficObject *> &, const std::vector<const MSLane *> &, double, double, bool)` [static]
    // void
    // Helper::applySubscriptionFilterLateralDistanceSinglePass(const Subscription& s, std::set<std::string>& objIDs,
    //         std::set<const SUMOTrafficObject*>& vehs,
    //         const std::vector<const MSLane*>& lanes, double posOnLane, double posLat, bool isDownstream) {
    //     const double streamDist = isDownstream ? s.filterDownstreamDist : s.filterUpstreamDist;
    //     double distRemaining = streamDist;
    //     bool isFirstLane = true;
    //     PositionVector combinedShape;
    //     for (const MSLane* lane : lanes) {
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "FILTER_LATERAL_DIST: current lane " << (isDownstream ? "down" : "up") << " is '" << lane->getID() << "', length " << lane->getLength()
    //                   << ", pos " << posOnLane << ", distRemaining " << distRemaining << std::endl;
    // #endif
    //         PositionVector laneShape = lane->getShape();
    //         if (isFirstLane) {
    //             isFirstLane = false;
    //             double geometryPos = lane->interpolateLanePosToGeometryPos(posOnLane);
    //             if (geometryPos <= POSITION_EPS) {
    //                 if (!isDownstream) {
    //                     continue;
    //                 }
    //             } else {
    //                 if (geometryPos >= laneShape.length() - POSITION_EPS) {
    //                     laneShape = isDownstream ? PositionVector() : laneShape;
    //                 } else {
    //                     auto pair = laneShape.splitAt(geometryPos, false);
    //                     laneShape = isDownstream ? pair.second : pair.first;
    //                 }
    //             }
    //         }
    //         double laneLength = lane->interpolateGeometryPosToLanePos(laneShape.length());
    //         if (distRemaining - laneLength < 0.) {
    //             double geometryPos = lane->interpolateLanePosToGeometryPos(isDownstream ? distRemaining : laneLength - distRemaining);
    //             if (geometryPos > POSITION_EPS && geometryPos < laneShape.length() - POSITION_EPS) {
    //                 auto pair = laneShape.splitAt(geometryPos, false);
    //                 laneShape = isDownstream ? pair.first : pair.second;
    //             }
    //         }
    //         distRemaining -= laneLength;
    //         try {
    //             laneShape.move2side(-posLat);
    //         } catch (ProcessError&) {
    //             WRITE_WARNINGF(TL("addSubscriptionFilterLateralDistance could not determine shape of lane '%' with a lateral shift of %."),
    //                            lane->getID(), toString(posLat));
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << "   posLat=" << posLat << " laneShape=" << laneShape << "\n";
    // #endif
    //         if (isDownstream) {
    //             combinedShape.append(laneShape);
    //         } else {
    //             combinedShape.prepend(laneShape);
    //         }
    //         if (distRemaining <= POSITION_EPS) {
    //             break;
    //         }
    //     }
    // #ifdef DEBUG_SURROUNDING
    //     std::cout << " combinedShape=" << combinedShape << "\n";
    // #endif
    //     // check remaining objects' distances to the combined shape
    //     auto i = objIDs.begin();
    //     while (i != objIDs.end()) {
    //         SUMOTrafficObject* obj = getTrafficObject(s.contextDomain, *i);
    //         double minPerpendicularDist = combinedShape.distance2D(obj->getPosition(), true);
    // #ifdef DEBUG_SURROUNDING
    //         std::cout << (isDownstream ? "DOWN" : "UP") << " obj " << obj->getID() << " perpendicular dist=" << minPerpendicularDist << " filterLateralDist=" << s.filterLateralDist << "\n";
    // #endif
    //         if ((minPerpendicularDist != GeomHelper::INVALID_OFFSET) && (minPerpendicularDist <= s.filterLateralDist)) {
    //             vehs.insert(obj);
    //             i = objIDs.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn apply_subscription_filter_lateral_distance_single_pass(s: &Subscription, objIDs: &mut BTreeSet<String>, vehs: &mut BTreeSet<Ptr<SUMOTrafficObject>>, lanes: &Vec<Ptr<MSLane>>, posOnLane: f64, posLat: f64, isDownstream: bool) {
        let streamDist = if isDownstream { s.filterDownstreamDist } else { s.filterUpstreamDist };
        let mut distRemaining = streamDist;
        let mut isFirstLane = true;
        let mut combinedShape = PositionVector(Vec::new());
        for lane in lanes.iter() {
            let mut laneShape: PositionVector = lane.get_shape(false).clone();
            if isFirstLane {
                isFirstLane = false;
                let geometryPos = lane.interpolate_lane_pos_to_geometry_pos(posOnLane);
                if geometryPos <= utils::geom::POSITION_EPS {
                    if !isDownstream {
                        continue;
                    }
                } else {
                    if geometryPos >= laneShape.length() - utils::geom::POSITION_EPS {
                        laneShape = if isDownstream { PositionVector(Vec::new()) } else { laneShape };
                    } else {
                        let pair = laneShape.split_at(geometryPos, false).unwrap();
                        laneShape = if isDownstream { pair.1 } else { pair.0 };
                    }
                }
            }
            let laneLength = lane.interpolate_geometry_pos_to_lane_pos(laneShape.length());
            if distRemaining - laneLength < 0. {
                let geometryPos = lane.interpolate_lane_pos_to_geometry_pos(if isDownstream { distRemaining } else { laneLength - distRemaining });
                if geometryPos > utils::geom::POSITION_EPS && geometryPos < laneShape.length() - utils::geom::POSITION_EPS {
                    let pair = laneShape.split_at(geometryPos, false).unwrap();
                    laneShape = if isDownstream { pair.0 } else { pair.1 };
                }
            }
            distRemaining -= laneLength;
            // shortcut: PositionVector::move2side does not report ProcessError, so the C++ warning path is not reachable
            laneShape.move2side(-posLat, 100.0);
            if isDownstream {
                combinedShape.append(&laneShape, 2.0);
            } else {
                combinedShape.prepend(&laneShape, 2.0);
            }
            if distRemaining <= utils::geom::POSITION_EPS {
                break;
            }
        }
        // check remaining objects' distances to the combined shape
        let ids: Vec<String> = objIDs.iter().cloned().collect();
        for id in ids.iter() {
            let obj = Helper::get_traffic_object(s.contextDomain, id);
            let minPerpendicularDist = combinedShape.distance2d(&obj.get_position(0.0), true);
            if minPerpendicularDist != geom_helper::INVALID_OFFSET && minPerpendicularDist <= s.filterLateralDist {
                vehs.insert(obj);
                objIDs.remove(id);
            }
        }
    }

    /// C++: `void Helper::setRemoteControlled(MSVehicle *, Position, MSLane *, double, double, double, int, ConstMSEdgeVector, SUMOTime)` [static]
    // void
    // Helper::setRemoteControlled(MSVehicle* v, Position xyPos, MSLane* l, double pos, double posLat, double angle,
    //                             int edgeOffset, ConstMSEdgeVector route, SUMOTime t) {
    //     myRemoteControlledVehicles[v->getID()] = v;
    //     v->getInfluencer().setRemoteControlled(xyPos, l, pos, posLat, angle, edgeOffset, route, t);
    // }
    #[allow(unused_variables)]
    pub fn set_remote_controlled(v: Ptr<MSVehicle>, xyPos: Position, l: Ptr<MSLane>, pos: f64, posLat: f64, angle: f64, edgeOffset: i32, route: ConstMSEdgeVector, t: SUMOTime) {
        my_remote_controlled_vehicles().insert(v.get_id().to_string(), v);
        v.m().get_influencer().set_remote_controlled(xyPos, l, pos, posLat, angle, edgeOffset, &route, t);
    }

    /// C++: `void Helper::setRemoteControlled(MSPerson *, Position, MSLane *, double, double, double, int, ConstMSEdgeVector, SUMOTime)` [static]
    // void
    // Helper::setRemoteControlled(MSPerson* p, Position xyPos, MSLane* l, double pos, double posLat, double angle,
    //                             int edgeOffset, ConstMSEdgeVector route, SUMOTime t) {
    //     myRemoteControlledPersons[p->getID()] = p;
    //     p->getInfluencer().setRemoteControlled(xyPos, l, pos, posLat, angle, edgeOffset, route, t);
    // }
    #[allow(unused_variables)]
    pub fn set_remote_controlled_2(p: Ptr<MSPerson>, xyPos: Position, l: Ptr<MSLane>, pos: f64, posLat: f64, angle: f64, edgeOffset: i32, route: ConstMSEdgeVector, t: SUMOTime) {
        my_remote_controlled_persons().insert(p.get_id().to_string(), p);
        p.m().get_influencer().set_remote_controlled(xyPos, l, pos, posLat, angle, edgeOffset, &route, t);
    }

    /// C++: `int Helper::postProcessRemoteControl()` [static]
    /// return number of remote-controlled entities
    // int
    // Helper::postProcessRemoteControl() {
    //     int numControlled = 0;
    //     for (auto& controlled : myRemoteControlledVehicles) {
    //         if (MSNet::getInstance()->getVehicleControl().getVehicle(controlled.first) != nullptr) {
    //             controlled.second->getInfluencer().postProcessRemoteControl(controlled.second);
    //             numControlled++;
    //         } else {
    //             WRITE_WARNINGF(TL("Vehicle '%' was removed though being controlled by TraCI"), controlled.first);
    //         }
    //     }
    //     myRemoteControlledVehicles.clear();
    //     for (auto& controlled : myRemoteControlledPersons) {
    //         if (MSNet::getInstance()->getPersonControl().get(controlled.first) != nullptr) {
    //             controlled.second->getInfluencer().postProcessRemoteControl(controlled.second);
    //             numControlled++;
    //         } else {
    //             WRITE_WARNINGF(TL("Person '%' was removed though being controlled by TraCI"), controlled.first);
    //         }
    //     }
    //     myRemoteControlledPersons.clear();
    //     return numControlled;
    // }
    #[allow(unused_variables)]
    pub fn post_process_remote_control() -> i32 {
        let mut numControlled = 0;
        let vehs = my_remote_controlled_vehicles();
        for (id, controlled) in vehs.iter() {
            if !MSNet::get_instance().m().get_vehicle_control().get_vehicle(id).is_null() {
                controlled.m().get_influencer().post_process_remote_control(*controlled);
                numControlled += 1;
            } else {
                utils::write_warningf!("Vehicle '%' was removed though being controlled by TraCI", id);
            }
        }
        vehs.clear();
        let persons = my_remote_controlled_persons();
        for (id, controlled) in persons.iter() {
            if !MSNet::get_instance().m().get_person_control().get(id).is_null() {
                // shortcut: person remote-control post-processing (MSPerson influencer) is not ported, upgrade when it is
                panic!("shortcut: person remote control post-processing not ported");
                numControlled += 1;
            } else {
                utils::write_warningf!("Person '%' was removed though being controlled by TraCI", id);
            }
        }
        persons.clear();
        numControlled
    }

    /// C++: `void Helper::cleanup()` [static]
    // void
    // Helper::cleanup() {
    //     // clean up NamedRTrees
    //     InductionLoop::cleanup();
    //     Junction::cleanup();
    //     LaneArea::cleanup();
    //     POI::cleanup();
    //     Polygon::cleanup();
    //     Helper::clearStateChanges();
    //     Helper::clearSubscriptions();
    //     delete myLaneTree;
    //     myLaneTree = nullptr;
    //     myRemoteControlledVehicles.clear();
    //     myRemoteControlledPersons.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        // clean up NamedRTrees
        InductionLoop::cleanup();
        Junction::cleanup();
        LaneArea::cleanup();
        POI::cleanup();
        Polygon::cleanup();
        Helper::clear_state_changes();
        Helper::clear_subscriptions();
        // shortcut: myLaneTree is not ported yet, nothing to delete
        my_remote_controlled_vehicles().clear();
        my_remote_controlled_persons().clear();
    }

    /// C++: `void Helper::registerStateListener()` [static]
    // void
    // Helper::registerStateListener() {
    //     if (MSNet::hasInstance()) {
    //         MSNet::getInstance()->addVehicleStateListener(&myVehicleStateListener);
    //         MSNet::getInstance()->addTransportableStateListener(&myTransportableStateListener);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn register_state_listener() {
        if MSNet::has_instance() {
            MSNet::get_instance().m().add_vehicle_state_listener(Ptr::from_mut(&mut my_vehicle_state_listener().base));
            MSNet::get_instance().m().add_transportable_state_listener(Ptr::from_mut(&mut my_transportable_state_listener().base));
        }
    }

    /// C++: `const std::vector<std::string> & Helper::getVehicleStateChanges(const MSNet::VehicleState)` [static]
    // const std::vector<std::string>&
    // Helper::getVehicleStateChanges(const MSNet::VehicleState state) {
    //     return myVehicleStateListener.myVehicleStateChanges[state];
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_state_changes(state: MSNet_VehicleState) -> &'static Vec<String> {
        let l = my_vehicle_state_listener();
        l.myVehicleStateChanges.entry(state).or_default();
        l.myVehicleStateChanges.get(&state).unwrap()
    }

    /// C++: `const std::vector<std::string> & Helper::getTransportableStateChanges(const MSNet::TransportableState)` [static]
    // const std::vector<std::string>&
    // Helper::getTransportableStateChanges(const MSNet::TransportableState state) {
    //     return myTransportableStateListener.myTransportableStateChanges[state];
    // }
    #[allow(unused_variables)]
    pub fn get_transportable_state_changes(state: MSNet_TransportableState) -> &'static Vec<String> {
        let l = my_transportable_state_listener();
        l.myTransportableStateChanges.entry(state).or_default();
        l.myTransportableStateChanges.get(&state).unwrap()
    }

    /// C++: `void Helper::clearStateChanges()` [static]
    // void
    // Helper::clearStateChanges() {
    //     for (auto& i : myVehicleStateListener.myVehicleStateChanges) {
    //         i.second.clear();
    //     }
    //     for (auto& i : myTransportableStateListener.myTransportableStateChanges) {
    //         i.second.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state_changes() {
        for i in my_vehicle_state_listener().myVehicleStateChanges.values_mut() {
            i.clear();
        }
        for i in my_transportable_state_listener().myTransportableStateChanges.values_mut() {
            i.clear();
        }
    }

    /// C++: `MSCalibrator::AspiredState Helper::getCalibratorState(const MSCalibrator *)` [static]
    // MSCalibrator::AspiredState
    // Helper::getCalibratorState(const MSCalibrator* c) {
    //     try {
    //         return c->getCurrentStateInterval();
    //     } catch (ProcessError& e) {
    //         throw TraCIException(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_calibrator_state(c: Ptr<MSCalibrator>) -> MSCalibrator_AspiredState {
        // C++ wraps ProcessError into TraCIException, which is a panic here
        c.get_current_state_interval()
    }

    /// C++: `bool Helper::moveToXYMap(const Position &, double, bool, const std::string &, const double, double, const ConstMSEdgeVector &, const int, const MSLane *, double, bool, SUMOVehicleClass, double, bool, double &, MSLane **, double &, int &, ConstMSEdgeVector &)` [static]
    /// @{
    // bool
    // Helper::moveToXYMap(const Position& pos, double maxRouteDistance, bool mayLeaveNetwork, const std::string& origID, const double angle,
    //                     double speed, const ConstMSEdgeVector& currentRoute, const int routePosition, const MSLane* currentLane, double currentLanePos, bool onRoad,
    //                     SUMOVehicleClass vClass, double currentAngle, bool setLateralPos,
    //                     double& bestDistance, MSLane** lane, double& lanePos, int& routeOffset, ConstMSEdgeVector& edges) {
    //     // collect edges around the vehicle/person
    // #ifdef DEBUG_MOVEXY
    //     std::cout << SIMTIME << " moveToXYMap pos=" << pos << " angle=" << angle << " vClass=" << toString(vClass) << "\n";
    // #endif
    //     const MSEdge* const currentRouteEdge = currentRoute[routePosition];
    //     std::set<const Named*> into;
    //     PositionVector shape;
    //     shape.push_back(pos);
    //     collectObjectsInRange(libsumo::CMD_GET_EDGE_VARIABLE, shape, maxRouteDistance, into);
    //     double maxDist = 0;
    //     std::map<MSLane*, LaneUtility, ComparatorNumericalIdLess> lane2utility;
    //     // compute utility for all candidate edges
    //     for (const Named* namedEdge : into) {
    //         const MSEdge* e = dynamic_cast<const MSEdge*>(namedEdge);
    //         if ((e->getPermissions() & vClass) != vClass) {
    //             continue;
    //         }
    //         const MSEdge* prevEdge = nullptr;
    //         const MSEdge* nextEdge = nullptr;
    //         bool onRoute = false;
    //         bool useCurrentAngle = false;
    //         // the next if/the clause sets "onRoute", "prevEdge", and "nextEdge", depending on
    //         //  whether the currently seen edge is an internal one or a normal one
    //         if (e->isWalkingArea() || e->isCrossing()) {
    //             // find current intersection along the route
    //             const MSJunction* junction = e->getFromJunction();
    //             for (int i = routePosition; i < (int)currentRoute.size(); i++) {
    //                 const MSEdge* cand = currentRoute[i];
    //                 if (cand->getToJunction() == junction) {
    //                     prevEdge = cand;
    //                     if (i + 1 < (int)currentRoute.size()) {
    //                         onRoute = true;
    //                         nextEdge = currentRoute[i + 1];
    //                     }
    //                     break;
    //                 }
    //             }
    //             if (!onRoute) {
    //                 // search backward
    //                 for (int i = routePosition - 1; i >= 0; i--) {
    //                     const MSEdge* cand = currentRoute[i];
    //                     if (cand->getToJunction() == junction) {
    //                         onRoute = true;
    //                         prevEdge = cand;
    //                         nextEdge = currentRoute[i + 1];
    //                         break;
    //                     }
    //                 }
    //             }
    //             if (prevEdge == nullptr) {
    //                 // use arbitrary predecessor
    //                 if (e->getPredecessors().size() > 0) {
    //                     prevEdge = e->getPredecessors().front();
    //                 } else if (e->getSuccessors().size() > 1) {
    //                     for (MSEdge* e2 : e->getSuccessors()) {
    //                         if (e2 != nextEdge) {
    //                             prevEdge = e2;
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    //             if (nextEdge == nullptr) {
    //                 if (e->getSuccessors().size() > 0) {
    //                     nextEdge = e->getSuccessors().front();
    //                 } else if (e->getPredecessors().size() > 1) {
    //                     for (MSEdge* e2 : e->getPredecessors()) {
    //                         if (e2 != prevEdge) {
    //                             nextEdge = e2;
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    // #ifdef DEBUG_MOVEXY_ANGLE
    //             std::cout << "walkingarea/crossing:" << e->getID() << " prev:" << Named::getIDSecure(prevEdge) << " next:" << Named::getIDSecure(nextEdge)
    //                       << " pred=" << toString(e->getPredecessors()) << " succ=" << toString(e->getSuccessors())
    //                       << "\n";
    // #endif
    //         } else if (e->isNormal()) {
    //             // a normal edge
    //             //
    //             // check whether the currently seen edge is in the vehicle's route
    //             //  - either the one it's on or one of the next edges
    //             ConstMSEdgeVector::const_iterator searchStart = currentRoute.begin() + routePosition;
    //             if (onRoad && currentLane->getEdge().isInternal()) {
    //                 ++searchStart;
    //             }
    //             ConstMSEdgeVector::const_iterator edgePos = std::find(searchStart, currentRoute.end(), e);
    //             onRoute = edgePos != currentRoute.end(); // no? -> onRoute is false
    //             if (edgePos == currentRoute.end() - 1 && currentRouteEdge == e) {
    //                 // onRoute is false as well if the vehicle is beyond the edge
    //                 onRoute &= currentRouteEdge->getLength() > currentLanePos + SPEED2DIST(speed);
    //             }
    //             // save prior and next edges
    //             prevEdge = e;
    //             nextEdge = !onRoute || edgePos == currentRoute.end() - 1 ? nullptr : *(edgePos + 1);
    // #ifdef DEBUG_MOVEXY_ANGLE
    //             std::cout << "normal:" << e->getID() << " prev:" << Named::getIDSecure(prevEdge) << " next:" << Named::getIDSecure(nextEdge) << "\n";
    // #endif
    //         } else if (e->isInternal()) {
    //             // an internal edge
    //             // get the previous edge
    //             prevEdge = e;
    //             while (prevEdge != nullptr && prevEdge->isInternal()) {
    //                 MSLane* l = prevEdge->getLanes()[0];
    //                 l = l->getLogicalPredecessorLane();
    //                 prevEdge = l == nullptr ? nullptr : &l->getEdge();
    //             }
    //             // check whether the previous edge is on the route (was on the route)
    //             ConstMSEdgeVector::const_iterator prevEdgePos = std::find(currentRoute.begin() + routePosition, currentRoute.end(), prevEdge);
    //             nextEdge = e;
    //             while (nextEdge != nullptr && nextEdge->isInternal()) {
    //                 nextEdge = nextEdge->getSuccessors()[0]; // should be only one for an internal edge
    //             }
    //             if (prevEdgePos != currentRoute.end() && (prevEdgePos + 1) != currentRoute.end()) {
    //                 onRoute = *(prevEdgePos + 1) == nextEdge;
    //             } else {
    //                 // we cannot make use of route information and should make
    //                 // use of the current angle if the user did not supply an angle
    //                 useCurrentAngle = angle == INVALID_DOUBLE_VALUE;
    //             }
    // #ifdef DEBUG_MOVEXY_ANGLE
    //             std::cout << "internal:" << e->getID() << " prev:" << Named::getIDSecure(prevEdge) << " next:" << Named::getIDSecure(nextEdge) << "\n";
    // #endif
    //         }
    //
    //
    //         // weight the lanes...
    //         const bool perpendicular = false;
    //         for (MSLane* const l : e->getLanes()) {
    //             if (!l->allowsVehicleClass(vClass)) {
    //                 continue;
    //             }
    //             if (l->getShape().length() == 0) {
    //                 // mapping to shapeless lanes is a bad idea
    //                 continue;
    //             }
    //             double langle = 180.;
    //             double dist = FAR_AWAY;
    //             double perpendicularDist = FAR_AWAY;
    //             // add some slack to avoid issues from tiny gaps between consecutive lanes
    //             // except when simulating with high precision where the slack can throw of mapping
    //             const double slack = POSITION_EPS * TS;
    //             PositionVector laneShape = l->getShape();
    //             laneShape.extrapolate2D(slack);
    //             double off = laneShape.nearest_offset_to_point2D(pos, true);
    //             if (off != GeomHelper::INVALID_OFFSET) {
    //                 perpendicularDist = laneShape.distance2D(pos, true);
    //                 perpendicularDist = patchShapeDistance(l, pos, perpendicularDist, true);
    //             }
    //             off = l->getShape().nearest_offset_to_point2D(pos, perpendicular);
    //             if (off != GeomHelper::INVALID_OFFSET) {
    //                 dist = l->getShape().distance2D(pos, perpendicular);
    //                 dist = patchShapeDistance(l, pos, dist, perpendicular);
    //                 langle = GeomHelper::naviDegree(l->getShape().rotationAtOffset(off));
    //             }
    //             // cannot trust lanePos on walkingArea
    //             bool sameEdge = onRoad && e == &currentLane->getEdge() && currentRouteEdge->getLength() > currentLanePos + SPEED2DIST(speed) && !e->isWalkingArea();
    //             /*
    //             const MSEdge* rNextEdge = nextEdge;
    //             while(rNextEdge==0&&lane->getEdge().getPurpose()==MSEdge::EDGEFUNCTION_INTERNAL) {
    //                 MSLane* next = lane->getLinkCont()[0]->getLane();
    //                 rNextEdge = next == 0 ? 0 : &next->getEdge();
    //             }
    //             */
    //             double dist2 = dist;
    //             if (mayLeaveNetwork && fabs(dist - perpendicularDist) > slack) {
    //                 // ambiguous mapping. Don't trust this
    //                 dist2 = FAR_AWAY;
    //             }
    //             const double angle2 = useCurrentAngle ? currentAngle : angle;
    //             const double angleDiff = (angle2 == INVALID_DOUBLE_VALUE || l->isWalkingArea() ? 0 : GeomHelper::getMinAngleDiff(angle2, langle));
    // #ifdef DEBUG_MOVEXY_ANGLE
    //             std::cout << std::setprecision(gPrecision)
    //                       << " candLane=" << l->getID() << " lAngle=" << langle << " lLength=" << l->getLength()
    //                       << " angleDiff=" << angleDiff
    //                       << " off=" << off
    //                       << " pDist=" << perpendicularDist
    //                       << " dist=" << dist
    //                       << " dist2=" << dist2
    //                       << "\n";
    //             std::cout << l->getID() << " param=" << l->getParameter(SUMO_PARAM_ORIGID, "") << " origID='" << origID << "\n";
    // #endif
    //
    //             bool origIDMatch = l->getParameter(SUMO_PARAM_ORIGID, l->getID()) == origID;
    //             if (origIDMatch && setLateralPos
    //                     && perpendicularDist > l->getWidth() / 2) {
    //                 origIDMatch = false;
    //             }
    //             lane2utility.emplace(l, LaneUtility(
    //                                      dist2, perpendicularDist, off, angleDiff,
    //                                      origIDMatch,
    //                                      onRoute, sameEdge, prevEdge, nextEdge));
    //             // update scaling value
    //             maxDist = MAX2(maxDist, MIN2(dist, SUMO_const_laneWidth));
    //
    //         }
    //     }
    //
    //     // get the best lane given the previously computed values
    //     double bestValue = 0;
    //     MSLane* bestLane = nullptr;
    //     for (const auto& it : lane2utility) {
    //         MSLane* const l = it.first;
    //         const LaneUtility& u = it.second;
    //         double distN = u.dist > 999 ? -10 : 1. - (u.dist / maxDist);
    //         double angleDiffN = 1. - (u.angleDiff / 180.);
    //         double idN = u.ID ? 1 : 0;
    //         double onRouteN = u.onRoute ? 1 : 0;
    //         double sameEdgeN = u.sameEdge ? MIN2(currentRouteEdge->getLength() / MAX2(NUMERICAL_EPS, speed), (double)1.) : 0;
    //         // distance is more important than angle because the vehicle might be driving in the opposite direction
    //         // also, distance becomes increasingly more important with lower step lengths (because position errors from one step to the next can result in large acceleration/speed errors)
    //         double value = (distN * .5 / TS
    //                         + angleDiffN * 0.35 /*.5 */
    //                         + idN * 1
    //                         + onRouteN * 0.1
    //                         + sameEdgeN * 0.1);
    // #ifdef DEBUG_MOVEXY
    //         std::cout << " x; l:" << l->getID() << " d:" << u.dist << " dN:" << distN << " aD:" << angleDiffN <<
    //                   " ID:" << idN << " oRN:" << onRouteN << " sEN:" << sameEdgeN << " value:" << value << std::endl;
    // #endif
    //         if (value > bestValue || bestLane == nullptr) {
    //             bestValue = value;
    //             if (u.dist == FAR_AWAY) {
    //                 bestLane = nullptr;
    //             } else {
    //                 bestLane = l;
    //             }
    //         }
    //     }
    //     // no best lane found, return
    //     if (bestLane == nullptr) {
    //         return false;
    //     }
    //     const LaneUtility& u = lane2utility.find(bestLane)->second;
    //     bestDistance = u.dist;
    //     *lane = bestLane;
    //     lanePos = MAX2(0., MIN2(double((*lane)->getLength() - NUMERICAL_EPS),
    //                             bestLane->interpolateGeometryPosToLanePos(
    //                                 bestLane->getShape().nearest_offset_to_point25D(pos, false))));
    //     const MSEdge* prevEdge = u.prevEdge;
    //     if (u.onRoute) {
    //         ConstMSEdgeVector::const_iterator prevEdgePos = std::find(currentRoute.begin(), currentRoute.end(), prevEdge);
    //         routeOffset = (int)std::distance(currentRoute.begin(), prevEdgePos);
    //         //std::cout << SIMTIME << "moveToXYMap currLane=" << currentLane->getID() << " routeOffset=" << routeOffset << " edges=" << toString(edges) << " bestLane=" << bestLane->getID() << " prevEdge=" << prevEdge->getID() << "\n";
    //     } else {
    //         edges.push_back(u.prevEdge);
    //         /*
    //            if(bestLane->getEdge().getPurpose()!=MSEdge::EDGEFUNCTION_INTERNAL) {
    //            edges.push_back(&bestLane->getEdge());
    //            }
    //         */
    //         if (u.nextEdge != nullptr) {
    //             edges.push_back(u.nextEdge);
    //         }
    //         routeOffset = 0;
    // #ifdef DEBUG_MOVEXY_ANGLE
    //         std::cout << SIMTIME << " internal2: lane=" << bestLane->getID() << " prev=" << Named::getIDSecure(u.prevEdge) << " next=" << Named::getIDSecure(u.nextEdge) << "\n";
    // #endif
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn move_to_xy_map(pos: &Position, maxRouteDistance: f64, mayLeaveNetwork: bool, origID: &str, angle: f64, speed: f64, currentRoute: &ConstMSEdgeVector, routePosition: i32, currentLane: Ptr<MSLane>, currentLanePos: f64, onRoad: bool, vClass: SUMOVehicleClass, currentAngle: f64, setLateralPos: bool, bestDistance: &mut f64, lane: Ptr<Ptr<MSLane>>, lanePos: &mut f64, routeOffset: &mut i32, edges: &mut ConstMSEdgeVector) -> bool {
        // collect edges around the vehicle/person
        let currentRouteEdge = currentRoute[routePosition as usize];
        let mut into: BTreeSet<Ptr<Named>> = BTreeSet::new();
        let mut shape = PositionVector(Vec::new());
        shape.push(*pos);
        Helper::collect_objects_in_range(CMD_GET_EDGE_VARIABLE, &shape, maxRouteDistance, &mut into);
        let mut maxDist = 0.0;
        let mut lane2utility: Vec<(Ptr<MSLane>, Helper_LaneUtility)> = Vec::new();
        // compute utility for all candidate edges
        for namedEdge in into.iter() {
            let e: Ptr<MSEdge> = namedEdge.cast::<MSEdge>();
            if (e.get_permissions() & vClass) != vClass {
                continue;
            }
            let mut prevEdge: Ptr<MSEdge> = Ptr::null();
            let mut nextEdge: Ptr<MSEdge> = Ptr::null();
            let mut onRoute = false;
            let mut useCurrentAngle = false;
            // the next if/the clause sets "onRoute", "prevEdge", and "nextEdge", depending on
            //  whether the currently seen edge is an internal one or a normal one
            if e.is_walking_area() || e.is_crossing() {
                // find current intersection along the route
                let junction = e.get_from_junction();
                for i in routePosition..(currentRoute.len() as i32) {
                    let cand = currentRoute[i as usize];
                    if cand.get_to_junction() == junction {
                        prevEdge = cand;
                        if i + 1 < currentRoute.len() as i32 {
                            onRoute = true;
                            nextEdge = currentRoute[(i + 1) as usize];
                        }
                        break;
                    }
                }
                if !onRoute {
                    // search backward
                    let mut i = routePosition - 1;
                    while i >= 0 {
                        let cand = currentRoute[i as usize];
                        if cand.get_to_junction() == junction {
                            onRoute = true;
                            prevEdge = cand;
                            nextEdge = currentRoute[(i + 1) as usize];
                            break;
                        }
                        i -= 1;
                    }
                }
                if prevEdge.is_null() {
                    // use arbitrary predecessor
                    if e.get_predecessors().len() > 0 {
                        prevEdge = e.get_predecessors()[0];
                    } else if e.get_successors(SVC_IGNORING).len() > 1 {
                        for &e2 in e.get_successors(SVC_IGNORING).iter() {
                            if e2 != nextEdge {
                                prevEdge = e2;
                                break;
                            }
                        }
                    }
                }
                if nextEdge.is_null() {
                    if e.get_successors(SVC_IGNORING).len() > 0 {
                        nextEdge = e.get_successors(SVC_IGNORING)[0];
                    } else if e.get_predecessors().len() > 1 {
                        for &e2 in e.get_predecessors().iter() {
                            if e2 != prevEdge {
                                nextEdge = e2;
                                break;
                            }
                        }
                    }
                }
            } else if e.is_normal() {
                // a normal edge
                //
                // check whether the currently seen edge is in the vehicle's route
                //  - either the one it's on or one of the next edges
                let mut searchStart = routePosition as usize;
                if onRoad && currentLane.get_edge().is_internal() {
                    searchStart += 1;
                }
                let edgePos = currentRoute[searchStart.min(currentRoute.len())..].iter().position(|&x| x == e).map(|p| p + searchStart);
                onRoute = edgePos.is_some(); // no? -> onRoute is false
                if edgePos == Some(currentRoute.len() - 1) && currentRouteEdge == e {
                    // onRoute is false as well if the vehicle is beyond the edge
                    onRoute &= currentRouteEdge.get_length() > currentLanePos + speed2dist(speed);
                }
                // save prior and next edges
                prevEdge = e;
                nextEdge = match edgePos {
                    Some(p) if onRoute && p + 1 < currentRoute.len() => currentRoute[p + 1],
                    _ => Ptr::null(),
                };
            } else if e.is_internal() {
                // an internal edge
                // get the previous edge
                prevEdge = e;
                while !prevEdge.is_null() && prevEdge.is_internal() {
                    let mut l = prevEdge.get_lanes()[0];
                    l = l.get_logical_predecessor_lane();
                    prevEdge = if l.is_null() { Ptr::null() } else { Ptr::from_mut(l.m().get_edge()) };
                }
                // check whether the previous edge is on the route (was on the route)
                let prevEdgePos = currentRoute[(routePosition as usize).min(currentRoute.len())..].iter().position(|&x| x == prevEdge).map(|p| p + routePosition as usize);
                nextEdge = e;
                while !nextEdge.is_null() && nextEdge.is_internal() {
                    nextEdge = nextEdge.get_successors(SVC_IGNORING)[0]; // should be only one for an internal edge
                }
                match prevEdgePos {
                    Some(p) if p + 1 < currentRoute.len() => onRoute = currentRoute[p + 1] == nextEdge,
                    _ => {
                        // we cannot make use of route information and should make
                        // use of the current angle if the user did not supply an angle
                        useCurrentAngle = angle == INVALID_DOUBLE_VALUE;
                    }
                }
            }

            // weight the lanes...
            let perpendicular = false;
            for l in e.get_lanes().clone().iter() {
                let l = *l;
                if !l.allows_vehicle_class(vClass) {
                    continue;
                }
                if l.get_shape(false).length() == 0. {
                    // mapping to shapeless lanes is a bad idea
                    continue;
                }
                let mut langle = 180.;
                let mut dist = FAR_AWAY;
                let mut perpendicularDist = FAR_AWAY;
                // add some slack to avoid issues from tiny gaps between consecutive lanes
                // except when simulating with high precision where the slack can throw of mapping
                let slack = utils::geom::POSITION_EPS * ts();
                let mut laneShape: PositionVector = l.get_shape(false).clone();
                laneShape.extrapolate2d(slack, false);
                let mut off = laneShape.nearest_offset_to_point2d(pos, true);
                if off != geom_helper::INVALID_OFFSET {
                    perpendicularDist = laneShape.distance2d(pos, true);
                    perpendicularDist = Helper::patch_shape_distance(l, pos, perpendicularDist, true);
                }
                off = l.get_shape(false).nearest_offset_to_point2d(pos, perpendicular);
                if off != geom_helper::INVALID_OFFSET {
                    dist = l.get_shape(false).distance2d(pos, perpendicular);
                    dist = Helper::patch_shape_distance(l, pos, dist, perpendicular);
                    langle = geom_helper::navi_degree(l.get_shape(false).rotation_at_offset(off));
                }
                // cannot trust lanePos on walkingArea
                let sameEdge = onRoad && e == Ptr::from_mut(currentLane.m().get_edge()) && currentRouteEdge.get_length() > currentLanePos + speed2dist(speed) && !e.is_walking_area();
                let mut dist2 = dist;
                if mayLeaveNetwork && (dist - perpendicularDist).abs() > slack {
                    // ambiguous mapping. Don't trust this
                    dist2 = FAR_AWAY;
                }
                let angle2 = if useCurrentAngle { currentAngle } else { angle };
                let angleDiff = if angle2 == INVALID_DOUBLE_VALUE || l.is_walking_area() { 0. } else { geom_helper::get_min_angle_diff(angle2, langle) };

                let mut origIDMatch: bool = panic!("shortcut: MSLane parameters not ported, upgrade when MSLane is Parameterised");
                if origIDMatch && setLateralPos && perpendicularDist > l.get_width() / 2. {
                    origIDMatch = false;
                }
                lane2utility.push((l, Helper_LaneUtility::new(dist2, perpendicularDist, off, angleDiff, origIDMatch, onRoute, sameEdge, prevEdge, nextEdge)));
                // update scaling value
                maxDist = f64::max(maxDist, f64::min(dist, SUMO_CONST_LANE_WIDTH));
            }
        }

        // get the best lane given the previously computed values
        let mut bestValue = 0.0;
        let mut bestLane: Ptr<MSLane> = Ptr::null();
        // determinism: C++ orders lane2utility by numerical lane id (ComparatorNumericalIdLess)
            lane2utility.sort_by_key(|(l, _)| l.get_numerical_id());
            for (l, u) in lane2utility.iter() {
            let distN = if u.dist > 999. { -10. } else { 1. - (u.dist / maxDist) };
            let angleDiffN = 1. - (u.angleDiff / 180.);
            let idN = if u.ID { 1. } else { 0. };
            let onRouteN = if u.onRoute { 1. } else { 0. };
            let sameEdgeN = if u.sameEdge { f64::min(currentRouteEdge.get_length() / f64::max(utils::geom::NUMERICAL_EPS, speed), 1.) } else { 0. };
            // distance is more important than angle because the vehicle might be driving in the opposite direction
            // also, distance becomes increasingly more important with lower step lengths (because position errors from one step to the next can result in large acceleration/speed errors)
            let value = distN * 0.5 / ts() + angleDiffN * 0.35 + idN * 1. + onRouteN * 0.1 + sameEdgeN * 0.1;
            if value > bestValue || bestLane.is_null() {
                bestValue = value;
                if u.dist == FAR_AWAY {
                    bestLane = Ptr::null();
                } else {
                    bestLane = *l;
                }
            }
        }
        // no best lane found, return
        if bestLane.is_null() {
            return false;
        }
        let u = &lane2utility.iter().find(|(l, _)| *l == bestLane).unwrap().1;
        *bestDistance = u.dist;
        *lane.m() = bestLane;
        *lanePos = f64::max(0., f64::min(bestLane.get_length() - utils::geom::NUMERICAL_EPS, bestLane.interpolate_geometry_pos_to_lane_pos(bestLane.get_shape(false).nearest_offset_to_point25d(pos, false))));
        let prevEdge = u.prevEdge;
        if u.onRoute {
            let prevEdgePos = currentRoute.iter().position(|&x| x == prevEdge).unwrap_or(currentRoute.len());
            *routeOffset = prevEdgePos as i32;
        } else {
            edges.push(u.prevEdge);
            if !u.nextEdge.is_null() {
                edges.push(u.nextEdge);
            }
            *routeOffset = 0;
        }
        true
    }

    /// C++: `bool Helper::moveToXYMap_matchingRoutePosition(const Position &, const std::string &, const ConstMSEdgeVector &, int, SUMOVehicleClass, bool, double &, MSLane **, double &, int &)` [static]
    // bool
    // Helper::moveToXYMap_matchingRoutePosition(const Position& pos, const std::string& origID,
    //         const ConstMSEdgeVector& currentRoute, int routeIndex,
    //         SUMOVehicleClass vClass, bool setLateralPos,
    //         double& bestDistance, MSLane** lane, double& lanePos, int& routeOffset) {
    // #ifdef DEBUG_MOVEXY
    //     std::cout << SIMTIME << " moveToXYMap_matchingRoutePosition pos=" << pos << " vClass=" << toString(vClass) << "\n";
    // #endif
    //     //std::cout << "moveToXYMap_matchingRoutePosition pos=" << pos << "\n";
    //     routeOffset = 0;
    //     // routes may be looped which makes routeOffset ambiguous. We first try to
    //     // find the closest upcoming edge on the route and then look for closer passed edges
    //
    //     // look forward along the route
    //     const MSEdge* prev = nullptr;
    //     for (int i = routeIndex; i < (int)currentRoute.size(); ++i) {
    //         const MSEdge* cand = currentRoute[i];
    //         while (prev != nullptr) {
    //             // check internal edge(s)
    //             const MSEdge* internalCand = prev->getInternalFollowingEdge(cand, vClass);
    // #ifdef DEBUG_MOVEXY
    //             std::cout << SIMTIME << "    prev=" << Named::getIDSecure(prev) << " cand=" << Named::getIDSecure(cand) << " internal=" << Named::getIDSecure(internalCand) << "\n";
    // #endif
    //             if (findCloserLane(internalCand, pos, vClass, bestDistance, lane)) {
    //                 routeOffset = i - 1;
    //             }
    //             prev = internalCand;
    //         }
    //         if (findCloserLane(cand, pos, vClass, bestDistance, lane)) {
    //             routeOffset = i;
    //         }
    //         prev = cand;
    //     }
    //     // look backward along the route
    //     const MSEdge* next = currentRoute[routeIndex];
    //     for (int i = routeIndex; i >= 0; --i) {
    //         const MSEdge* cand = currentRoute[i];
    //         //std::cout << "  next=" << next->getID() << " cand=" << cand->getID() << " i=" << i << " routeIndex=" << routeIndex << "\n";
    //         prev = cand;
    //         while (prev != nullptr) {
    //             // check internal edge(s)
    //             const MSEdge* internalCand = prev->getInternalFollowingEdge(next, vClass);
    //             if (findCloserLane(internalCand, pos, vClass, bestDistance, lane)) {
    //                 routeOffset = i;
    //             }
    //             prev = internalCand;
    //         }
    //         if (findCloserLane(cand, pos, vClass, bestDistance, lane)) {
    //             routeOffset = i;
    //         }
    //         next = cand;
    //     }
    //     if (vClass == SVC_PEDESTRIAN) {
    //         // consider all crossings and walkingareas along the route
    //         std::map<const MSJunction*, int> routeJunctions;
    //         for (int i = 0; i < (int)currentRoute.size() - 1; ++i) {
    //             routeJunctions[currentRoute[i]->getToJunction()] = i;
    //         }
    //         std::set<const Named*> into;
    //         PositionVector shape;
    //         shape.push_back(pos);
    //         collectObjectsInRange(libsumo::CMD_GET_LANE_VARIABLE, shape, 100, into);
    //         for (const Named* named : into) {
    //             const MSLane* cand = dynamic_cast<const MSLane*>(named);
    //             if ((cand->isWalkingArea() || cand->isCrossing())
    //                     && routeJunctions.count(cand->getEdge().getToJunction()) != 0) {
    //                 if (findCloserLane(&cand->getEdge(), pos, vClass, bestDistance, lane)) {
    //                     routeOffset = routeJunctions[cand->getEdge().getToJunction()];
    //                 }
    //             }
    //         }
    //     }
    //
    //     assert(lane != nullptr);
    //     // quit if no solution was found, reporting a failure
    //     if (*lane == nullptr) {
    // #ifdef DEBUG_MOVEXY
    //         std::cout << "  b failed - no best route lane" << std::endl;
    // #endif
    //         return false;
    //     }
    //
    //
    //     // position may be inaccurate; let's check the given index, too
    //     // @note: this is enabled for non-internal lanes only, as otherwise the position information may ambiguous
    //     if (!(*lane)->getEdge().isInternal()) {
    //         const std::vector<MSLane*>& lanes = (*lane)->getEdge().getLanes();
    //         for (std::vector<MSLane*>::const_iterator i = lanes.begin(); i != lanes.end(); ++i) {
    //             if ((*i)->getParameter(SUMO_PARAM_ORIGID, (*i)->getID()) == origID) {
    //                 if (setLateralPos) {
    //                     // vehicle might end up on top of another lane with a big
    //                     // lateral offset to the lane with origID.
    //                     const double dist = (*i)->getShape().distance2D(pos); // get distance
    //                     if (dist < (*i)->getWidth() / 2) {
    //                         *lane = *i;
    //                         break;
    //                     }
    //                 } else {
    //                     *lane = *i;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     // check position, stuff, we should have the best lane along the route
    //     lanePos = MAX2(0., MIN2(double((*lane)->getLength() - NUMERICAL_EPS),
    //                             (*lane)->interpolateGeometryPosToLanePos(
    //                                 (*lane)->getShape().nearest_offset_to_point25D(pos, false))));
    //     //std::cout << SIMTIME << " moveToXYMap_matchingRoutePosition vehicle=" << veh.getID() << " currLane=" << veh.getLane()->getID() << " routeOffset=" << routeOffset << " edges=" << toString(edges) << " lane=" << (*lane)->getID() << "\n";
    // #ifdef DEBUG_MOVEXY
    //     std::cout << "  b ok lane " << (*lane)->getID() << " lanePos:" << lanePos << std::endl;
    // #endif
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn move_to_xy_map_matching_route_position(pos: &Position, origID: &str, currentRoute: &ConstMSEdgeVector, routeIndex: i32, vClass: SUMOVehicleClass, setLateralPos: bool, bestDistance: &mut f64, lane: Ptr<Ptr<MSLane>>, lanePos: &mut f64, routeOffset: &mut i32) -> bool {
        *routeOffset = 0;
        // routes may be looped which makes routeOffset ambiguous. We first try to
        // find the closest upcoming edge on the route and then look for closer passed edges

        // look forward along the route
        let mut prev: Ptr<MSEdge> = Ptr::null();
        for i in routeIndex..(currentRoute.len() as i32) {
            let cand = currentRoute[i as usize];
            while !prev.is_null() {
                // check internal edge(s)
                let internalCand = prev.get_internal_following_edge(cand, vClass);
                if Helper::find_closer_lane(internalCand, pos, vClass, bestDistance, lane) {
                    *routeOffset = i - 1;
                }
                prev = internalCand;
            }
            if Helper::find_closer_lane(cand, pos, vClass, bestDistance, lane) {
                *routeOffset = i;
            }
            prev = cand;
        }
        // look backward along the route
        let mut next = currentRoute[routeIndex as usize];
        let mut i = routeIndex;
        while i >= 0 {
            let cand = currentRoute[i as usize];
            prev = cand;
            while !prev.is_null() {
                // check internal edge(s)
                let internalCand = prev.get_internal_following_edge(next, vClass);
                if Helper::find_closer_lane(internalCand, pos, vClass, bestDistance, lane) {
                    *routeOffset = i;
                }
                prev = internalCand;
            }
            if Helper::find_closer_lane(cand, pos, vClass, bestDistance, lane) {
                *routeOffset = i;
            }
            next = cand;
            i -= 1;
        }
        if vClass == SVC_PEDESTRIAN {
            // consider all crossings and walkingareas along the route
            let mut routeJunctions: BTreeMap<Ptr<MSJunction>, i32> = BTreeMap::new();
            for i in 0..(currentRoute.len() as i32 - 1) {
                routeJunctions.insert(currentRoute[i as usize].get_to_junction(), i);
            }
            let mut into: BTreeSet<Ptr<Named>> = BTreeSet::new();
            let mut shape = PositionVector(Vec::new());
            shape.push(*pos);
            Helper::collect_objects_in_range(CMD_GET_LANE_VARIABLE, &shape, 100., &mut into);
            for named in into.iter() {
                let cand: Ptr<MSLane> = named.cast::<MSLane>();
                if (cand.is_walking_area() || cand.is_crossing()) && routeJunctions.contains_key(&cand.get_edge().get_to_junction()) {
                    let edge = Ptr::from_mut(cand.m().get_edge());
                    if Helper::find_closer_lane(edge, pos, vClass, bestDistance, lane) {
                        *routeOffset = routeJunctions[&cand.get_edge().get_to_junction()];
                    }
                }
            }
        }

        assert!(!lane.is_null());
        // quit if no solution was found, reporting a failure
        if lane.is_null() || (*lane).is_null() {
            return false;
        }

        // position may be inaccurate; let's check the given index, too
        // @note: this is enabled for non-internal lanes only, as otherwise the position information may ambiguous
        if !(*lane).get_edge().is_internal() {
            let lanes = (*lane).get_edge().get_lanes().clone();
            for li in lanes.iter() {
                if { panic!("shortcut: MSLane parameters not ported, upgrade when MSLane is Parameterised") } {
                    if setLateralPos {
                        // vehicle might end up on top of another lane with a big
                        // lateral offset to the lane with origID.
                        let dist = li.get_shape(false).distance2d(pos, false); // get distance
                        if dist < li.get_width() / 2. {
                            *lane.m() = *li;
                            break;
                        }
                    } else {
                        *lane.m() = *li;
                        break;
                    }
                }
            }
        }
        // check position, stuff, we should have the best lane along the route
        let l = *lane.m();
        *lanePos = f64::max(0., f64::min(l.get_length() - utils::geom::NUMERICAL_EPS, l.interpolate_geometry_pos_to_lane_pos(l.get_shape(false).nearest_offset_to_point25d(pos, false))));
        true
    }

    /// C++: `bool Helper::findCloserLane(const MSEdge *, const Position &, SUMOVehicleClass, double &, MSLane **)` [static]
    // bool
    // Helper::findCloserLane(const MSEdge* edge, const Position& pos, SUMOVehicleClass vClass, double& bestDistance, MSLane** lane) {
    //     // TODO maybe there is a way to abort this early if the lane already found is good enough but simply
    //     // checking for bestDistance < POSITON_EPS gives ugly order dependencies (#7933), so think twice and profile first
    //     if (edge == nullptr) {
    //         return false;
    //     }
    //     bool newBest = false;
    //     for (MSLane* const candidateLane : edge->getLanes()) {
    //         if (!candidateLane->allowsVehicleClass(vClass)) {
    //             continue;
    //         }
    //         if (candidateLane->getShape().length() == 0) {
    //             // mapping to shapeless lanes is a bad idea
    //             continue;
    //         }
    //         double dist = candidateLane->getShape().distance2D(pos);
    //         dist = patchShapeDistance(candidateLane, pos, dist, false);
    // #ifdef DEBUG_MOVEXY
    //         std::cout << "   b at lane " << candidateLane->getID() << " dist:" << dist << " best:" << bestDistance << std::endl;
    // #endif
    //         if (dist < bestDistance || (dist == bestDistance && candidateLane->getNumericalID() < (*lane)->getNumericalID())) {
    //             // is the new distance the best one? keep then...
    //             bestDistance = dist;
    //             *lane = candidateLane;
    //             newBest = true;
    //         }
    //     }
    //     if (edge->isInternal() && edge->getNumLanes() > 1) {
    //         // there is a parallel internal edge that isn't returned by getInternalFollowingEdge but is also usable for the same route
    //         for (const MSLane* const l : edge->getLanes()) {
    //             if (l->getIndex() == 0) {
    //                 continue;
    //             }
    //             for (const MSLink* const link : l->getLinkCont()) {
    //                 if (link->isInternalJunctionLink()) {
    //                     if (findCloserLane(&link->getViaLane()->getEdge(), pos, vClass, bestDistance, lane)) {
    //                         newBest = true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return newBest;
    // }
    #[allow(unused_variables)]
    pub fn find_closer_lane(edge: Ptr<MSEdge>, pos: &Position, vClass: SUMOVehicleClass, bestDistance: &mut f64, lane: Ptr<Ptr<MSLane>>) -> bool {
        // TODO maybe there is a way to abort this early if the lane already found is good enough but simply
        // checking for bestDistance < POSITON_EPS gives ugly order dependencies (#7933), so think twice and profile first
        if edge.is_null() {
            return false;
        }
        let mut newBest = false;
        for candidateLane in edge.get_lanes().clone().iter() {
            let candidateLane = *candidateLane;
            if !candidateLane.allows_vehicle_class(vClass) {
                continue;
            }
            if candidateLane.get_shape(false).length() == 0. {
                // mapping to shapeless lanes is a bad idea
                continue;
            }
            let mut dist = candidateLane.get_shape(false).distance2d(pos, false);
            dist = Helper::patch_shape_distance(candidateLane, pos, dist, false);
            if dist < *bestDistance || (dist == *bestDistance && candidateLane.get_numerical_id() < (*lane.m()).get_numerical_id()) {
                // is the new distance the best one? keep then...
                *bestDistance = dist;
                *lane.m() = candidateLane;
                newBest = true;
            }
        }
        if edge.is_internal() && edge.get_num_lanes() > 1 {
            // there is a parallel internal edge that isn't returned by getInternalFollowingEdge but is also usable for the same route
            for l in edge.get_lanes().clone().iter() {
                if l.get_index() == 0 {
                    continue;
                }
                for link in l.get_link_cont().clone().iter() {
                    if link.is_internal_junction_link() {
                        if Helper::find_closer_lane(Ptr::from_mut(link.get_via_lane().m().get_edge()), pos, vClass, bestDistance, lane) {
                            newBest = true;
                        }
                    }
                }
            }
        }
        newBest
    }

    /// C++: `double Helper::patchShapeDistance(const MSLane *, const Position &, double, bool)` [static]
    /// return the distance of pos from the area covered by this lane
    // double
    // Helper::patchShapeDistance(const MSLane* lane, const Position& pos, double dist, bool wasPerpendicular) {
    //     if (!lane->isWalkingArea() && (wasPerpendicular || lane->getShape().nearest_offset_to_point25D(pos, true) != GeomHelper::INVALID_OFFSET)) {
    //         dist = MAX2(0.0, dist - lane->getWidth() * 0.5);
    //     }
    //     return dist;
    // }
    #[allow(unused_variables)]
    pub fn patch_shape_distance(lane: Ptr<MSLane>, pos: &Position, dist: f64, wasPerpendicular: bool) -> f64 {
        let mut dist = dist;
        if !lane.is_walking_area() && (wasPerpendicular || lane.get_shape(false).nearest_offset_to_point25d(pos, true) != geom_helper::INVALID_OFFSET) {
            dist = f64::max(0.0, dist - lane.get_width() * 0.5);
        }
        dist
    }

    /// C++: `int Helper::readDistanceRequest(tcpip::Storage &, TraCIRoadPosition &, Position &)` [static]
    // int
    // Helper::readDistanceRequest(tcpip::Storage& data, TraCIRoadPosition& roadPos, Position& pos) {
    //     int distType = 0;
    //     StoHelp::readCompound(data, 2, "Retrieval of distance requires two parameter as compound.");
    //     const int posType = data.readUnsignedByte();
    //     switch (posType) {
    //         case libsumo::POSITION_ROADMAP: {
    //             roadPos.edgeID = data.readString();
    //             roadPos.pos = data.readDouble();
    //             roadPos.laneIndex = data.readUnsignedByte();
    //             break;
    //         }
    //         case libsumo::POSITION_2D:
    //         case libsumo::POSITION_3D: {
    //             pos.setx(data.readDouble());
    //             pos.sety(data.readDouble());
    //             if (posType == libsumo::POSITION_3D) {
    //                 pos.setz(data.readDouble());
    //             }
    //             break;
    //         }
    //         default:
    //             throw TraCIException("Unknown position format used for distance request.");
    //     }
    //     distType = data.readUnsignedByte();
    //     if (distType != libsumo::REQUEST_DRIVINGDIST) {
    //         throw TraCIException("Only driving distance is supported.");
    //     }
    //     return posType;
    // }
    #[allow(unused_variables)]
    pub fn read_distance_request(data: &mut Storage, roadPos: &mut TraCIRoadPosition, pos: &mut Position) -> i32 {
        let mut distType = 0;
        StorageHelper::read_compound(data, 2, "Retrieval of distance requires two parameter as compound.");
        let posType = data.read_unsigned_byte().unwrap();
        match posType {
            POSITION_ROADMAP => {
                roadPos.edgeID = data.read_string().unwrap();
                roadPos.pos = data.read_double().unwrap();
                roadPos.laneIndex = data.read_unsigned_byte().unwrap();
            }
            POSITION_2D | POSITION_3D => {
                pos.setx(data.read_double().unwrap());
                pos.sety(data.read_double().unwrap());
                if posType == POSITION_3D {
                    pos.setz(data.read_double().unwrap());
                }
            }
            _ => panic!("Unknown position format used for distance request."),
        }
        distType = data.read_unsigned_byte().unwrap();
        if distType != REQUEST_DRIVINGDIST {
            panic!("Only driving distance is supported.");
        }
        posType
    }

    /// C++: `void Helper::handleSingleSubscription(const Subscription &)` [static, private]
    // void
    // Helper::handleSingleSubscription(const Subscription& s) {
    //     const int getCommandId = s.contextDomain > 0 ? s.contextDomain : s.commandId - 0x30;
    //     std::set<std::string> objIDs;
    //     if (s.contextDomain > 0) {
    //         if ((s.activeFilters & SUBS_FILTER_NO_RTREE) == 0) {
    //             PositionVector shape;
    //             findObjectShape(s.commandId, s.id, shape);
    //             collectObjectIDsInRange(s.contextDomain, shape, s.range, objIDs);
    //         }
    //         applySubscriptionFilters(s, objIDs);
    //     } else {
    //         objIDs.insert(s.id);
    //     }
    //     if (myWrapper.empty()) {
    //         myWrapper[libsumo::CMD_GET_BUSSTOP_VARIABLE] = BusStop::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_CALIBRATOR_VARIABLE] = Calibrator::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_CHARGINGSTATION_VARIABLE] = ChargingStation::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_EDGE_VARIABLE] = Edge::makeWrapper();
    // #ifdef HAVE_LIBSUMOGUI
    //         myWrapper[libsumo::CMD_GET_GUI_VARIABLE] = GUI::makeWrapper();
    // #endif
    //         myWrapper[libsumo::CMD_GET_INDUCTIONLOOP_VARIABLE] = InductionLoop::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_JUNCTION_VARIABLE] = Junction::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_LANE_VARIABLE] = Lane::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_LANEAREA_VARIABLE] = LaneArea::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_MEANDATA_VARIABLE] = MeanData::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_MULTIENTRYEXIT_VARIABLE] = MultiEntryExit::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_OVERHEADWIRE_VARIABLE] = OverheadWire::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_PARKINGAREA_VARIABLE] = ParkingArea::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_PERSON_VARIABLE] = Person::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_POI_VARIABLE] = POI::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_POLYGON_VARIABLE] = Polygon::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_REROUTER_VARIABLE] = Rerouter::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_ROUTE_VARIABLE] = Route::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_ROUTEPROBE_VARIABLE] = RouteProbe::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_SIM_VARIABLE] = Simulation::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_TL_VARIABLE] = TrafficLight::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_VARIABLESPEEDSIGN_VARIABLE] = VariableSpeedSign::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_VEHICLE_VARIABLE] = Vehicle::makeWrapper();
    //         myWrapper[libsumo::CMD_GET_VEHICLETYPE_VARIABLE] = VehicleType::makeWrapper();
    //     }
    //     auto wrapper = myWrapper.find(getCommandId);
    //     if (wrapper == myWrapper.end()) {
    //         throw TraCIException("Unsupported command " + toHex(getCommandId, 2) + " specified");
    //     }
    //     std::shared_ptr<VariableWrapper> handler = wrapper->second;
    //     VariableWrapper* container = handler.get();
    //     if (s.contextDomain > 0) {
    //         auto containerWrapper = myWrapper.find(s.commandId + 0x20);
    //         if (containerWrapper == myWrapper.end()) {
    //             throw TraCIException("Unsupported domain " + toHex(s.commandId + 0x20, 2) + " specified");
    //         }
    //         container = containerWrapper->second.get();
    //         container->setContext(&s.id);
    //     } else {
    //         container->setContext(nullptr);
    //     }
    //     for (const std::string& objID : objIDs) {
    //         if (!s.variables.empty()) {
    //             std::vector<std::shared_ptr<tcpip::Storage> >::const_iterator k = s.parameters.begin();
    //             for (const int variable : s.variables) {
    //                 if (s.contextDomain > 0 && variable == libsumo::TRACI_ID_LIST) {
    //                     container->empty(objID);
    //                 } else {
    //                     (*k)->resetPos();
    //                     try {
    //                         if (!handler->handle(objID, variable, container, k->get())) {
    //                             throw TraCIException("Unsupported variable " + toHex(variable, 2) + " specified");
    //                         }
    //                     } catch (const std::invalid_argument&) {
    //                         throw TraCIException("Unsupported variable " + toHex(variable, 2) + " specified");
    //                     }
    //                     ++k;
    //                 }
    //             }
    //         } else if (s.contextDomain > 0) {
    //             // default for contexts is an empty map (similar to id list)
    //             container->empty(objID);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_single_subscription(s: &Subscription) {
        let getCommandId = if s.contextDomain > 0 { s.contextDomain } else { s.commandId - 0x30 };
        let mut objIDs: BTreeSet<String> = BTreeSet::new();
        if s.contextDomain > 0 {
            if (s.activeFilters & sf(SubscriptionFilterType::SUBS_FILTER_NO_RTREE)) == 0 {
                let mut shape = PositionVector(Vec::new());
                Helper::find_object_shape(s.commandId, &s.id, &mut shape);
                Helper::collect_object_i_ds_in_range(s.contextDomain, &shape, s.range, &mut objIDs);
            }
            Helper::apply_subscription_filters(s, &mut objIDs);
        } else {
            objIDs.insert(s.id.clone());
        }
        if my_wrapper().is_empty() {
            let w = my_wrapper();
            w.insert(CMD_GET_BUSSTOP_VARIABLE, BusStop::make_wrapper());
            w.insert(CMD_GET_CALIBRATOR_VARIABLE, Calibrator::make_wrapper());
            w.insert(CMD_GET_CHARGINGSTATION_VARIABLE, ChargingStation::make_wrapper());
            w.insert(CMD_GET_EDGE_VARIABLE, Edge::make_wrapper());
            w.insert(CMD_GET_INDUCTIONLOOP_VARIABLE, InductionLoop::make_wrapper());
            w.insert(CMD_GET_JUNCTION_VARIABLE, Junction::make_wrapper());
            w.insert(CMD_GET_LANE_VARIABLE, Lane::make_wrapper());
            w.insert(CMD_GET_LANEAREA_VARIABLE, LaneArea::make_wrapper());
            w.insert(CMD_GET_MEANDATA_VARIABLE, MeanData::make_wrapper());
            w.insert(CMD_GET_MULTIENTRYEXIT_VARIABLE, MultiEntryExit::make_wrapper());
            w.insert(CMD_GET_OVERHEADWIRE_VARIABLE, OverheadWire::make_wrapper());
            w.insert(CMD_GET_PARKINGAREA_VARIABLE, ParkingArea::make_wrapper());
            w.insert(CMD_GET_PERSON_VARIABLE, Person::make_wrapper());
            w.insert(CMD_GET_POI_VARIABLE, POI::make_wrapper());
            w.insert(CMD_GET_POLYGON_VARIABLE, Polygon::make_wrapper());
            w.insert(CMD_GET_REROUTER_VARIABLE, Rerouter::make_wrapper());
            w.insert(CMD_GET_ROUTE_VARIABLE, Route::make_wrapper());
            w.insert(CMD_GET_ROUTEPROBE_VARIABLE, RouteProbe::make_wrapper());
            w.insert(CMD_GET_SIM_VARIABLE, Simulation::make_wrapper());
            w.insert(CMD_GET_TL_VARIABLE, TrafficLight::make_wrapper());
            w.insert(CMD_GET_VARIABLESPEEDSIGN_VARIABLE, VariableSpeedSign::make_wrapper());
            w.insert(CMD_GET_VEHICLE_VARIABLE, Vehicle::make_wrapper());
            w.insert(CMD_GET_VEHICLETYPE_VARIABLE, VehicleType::make_wrapper());
        }
        let handler: Rc<VariableWrapper> = match my_wrapper().get(&getCommandId) {
            Some(h) => h.clone(),
            None => panic!("Unsupported command {} specified", to_hex(getCommandId as u64, 2, 8)),
        };
        let container: Ptr<VariableWrapper>;
        if s.contextDomain > 0 {
            let containerWrapper = match my_wrapper().get(&(s.commandId + 0x20)) {
                Some(c) => c.clone(),
                None => panic!("Unsupported domain {} specified", to_hex((s.commandId + 0x20) as u64, 2, 8)),
            };
            container = Ptr::new(&*containerWrapper);
            container.m().set_context(Ptr::new(&s.id));
        } else {
            container = Ptr::new(&*handler);
            container.m().set_context(Ptr::null());
        }
        let handlerPtr = Ptr::new(&*handler);
        for objID in objIDs.iter() {
            if !s.variables.is_empty() {
                let mut k = 0usize;
                for &variable in s.variables.iter() {
                    if s.contextDomain > 0 && variable == TRACI_ID_LIST {
                        container.m().empty(objID);
                    } else {
                        Ptr::new(&*s.parameters[k]).m().reset_pos();
                        // shortcut: VariableWrapper::handle is Unmapped (), the variable handler dispatch is not ported yet
                        panic!("shortcut: VariableWrapper::handle is Unmapped, cannot dispatch variable {}", to_hex(variable as u64, 2, 8));
                    }
                }
            } else if s.contextDomain > 0 {
                // default for contexts is an empty map (similar to id list)
                container.m().empty(objID);
            }
        }
    }

    /// C++: `void Helper::fuseLaneCoverage(std::shared_ptr<LaneCoverageInfo>, const std::shared_ptr<LaneCoverageInfo>)` [static, private]
    /// Adds lane coverage information from newLaneCoverage into aggregatedLaneCoverage
    // void
    // Helper::fuseLaneCoverage(std::shared_ptr<LaneCoverageInfo> aggregatedLaneCoverage, const std::shared_ptr<LaneCoverageInfo> newLaneCoverage) {
    //     for (auto& p : *newLaneCoverage) {
    //         const MSLane* lane = p.first;
    //         if (aggregatedLaneCoverage->find(lane) == aggregatedLaneCoverage->end()) {
    //             // Lane has no coverage in aggregatedLaneCoverage, yet
    //             (*aggregatedLaneCoverage)[lane] = (*newLaneCoverage)[lane];
    //         } else {
    //             // Lane is covered in aggregatedLaneCoverage as well
    //             std::pair<double, double>& range1 = (*aggregatedLaneCoverage)[lane];
    //             std::pair<double, double>& range2 = (*newLaneCoverage)[lane];
    //             std::pair<double, double> hull = std::make_pair(MIN2(range1.first, range2.first), MAX2(range1.second, range2.second));
    //             (*aggregatedLaneCoverage)[lane] = hull;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fuse_lane_coverage(aggregatedLaneCoverage: Rc<LaneCoverageInfo>, newLaneCoverage: Rc<LaneCoverageInfo>) {
        for (lane, range2) in newLaneCoverage.iter() {
            // shortcut: Rc<LaneCoverageInfo> is mutated in place like the C++ shared_ptr, needs interior mutability in ms_lane.rs to be sound
            let agg: &mut LaneCoverageInfo = unsafe { &mut *(Rc::as_ptr(&aggregatedLaneCoverage) as *mut LaneCoverageInfo) };
            match agg.get(lane).copied() {
                None => {
                    // Lane has no coverage in aggregatedLaneCoverage, yet
                    agg.insert(*lane, *range2);
                }
                Some(range1) => {
                    // Lane is covered in aggregatedLaneCoverage as well
                    let hull = (f64::min(range1.0, range2.0), f64::max(range1.1, range2.1));
                    agg.insert(*lane, hull);
                }
            }
        }
    }

    /// C++: `void Helper::debugPrint(const SUMOTrafficObject *)` [static, private]
    // void
    // Helper::debugPrint(const SUMOTrafficObject* veh) {
    //     if (veh != nullptr) {
    //         if (veh->isVehicle()) {
    //             std::cout << "  '" << veh->getID() << "' on lane '" << ((SUMOVehicle*)veh)->getLane()->getID() << "'\n";
    //         } else {
    //             std::cout << "  '" << veh->getID() << "' on edge '" << veh->getEdge()->getID() << "'\n";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn debug_print(veh: Ptr<SUMOTrafficObject>) {
        // debug output is compiled out unless DEBUG_SURROUNDING is defined in C++
        let _ = veh;
    }

    /// C++: `void Helper::Helper()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Helper {}
    }

}

/// C++ class `libsumo::Helper::LaneUtility` (libsumo/Helper.h)
pub struct Helper_LaneUtility {
    pub dist: f64,
    pub perpendicularDist: f64,
    pub lanePos: f64,
    pub angleDiff: f64,
    pub ID: bool,
    pub onRoute: bool,
    pub sameEdge: bool,
    pub prevEdge: Ptr<MSEdge>,
    pub nextEdge: Ptr<MSEdge>,
}

impl Helper_LaneUtility {
    /// C++: `void LaneUtility::LaneUtility(double, double, double, double, bool, bool, bool, const MSEdge *, const MSEdge *)`
    // LaneUtility(double dist_, double perpendicularDist_, double lanePos_, double angleDiff_, bool ID_,
    //                     bool onRoute_, bool sameEdge_, const MSEdge* prevEdge_, const MSEdge* nextEdge_) :
    //             dist(dist_), perpendicularDist(perpendicularDist_), lanePos(lanePos_), angleDiff(angleDiff_), ID(ID_),
    //             onRoute(onRoute_), sameEdge(sameEdge_), prevEdge(prevEdge_), nextEdge(nextEdge_) {}
    #[allow(unused_variables)]
    pub fn new(dist_: f64, perpendicularDist_: f64, lanePos_: f64, angleDiff_: f64, ID_: bool, onRoute_: bool, sameEdge_: bool, prevEdge_: Ptr<MSEdge>, nextEdge_: Ptr<MSEdge>) -> Self {
        Helper_LaneUtility {
            dist: dist_,
            perpendicularDist: perpendicularDist_,
            lanePos: lanePos_,
            angleDiff: angleDiff_,
            ID: ID_,
            onRoute: onRoute_,
            sameEdge: sameEdge_,
            prevEdge: prevEdge_,
            nextEdge: nextEdge_,
        }
    }

    /// C++: `void LaneUtility::~LaneUtility() noexcept`
    // ~LaneUtility() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body: nothing to release
    }

}

/// @}
/// C++ class `libsumo::Helper::SubscriptionWrapper` (libsumo/Helper.h)
#[repr(C)] // base must stay first: Simulation::handleVariable downcasts the VariableWrapper to this type
pub struct Helper_SubscriptionWrapper {
    /// C++ base class `VariableWrapper`
    pub base: VariableWrapper,
    pub myResults: Ptr<SubscriptionResults>,
    pub myContextResults: Ptr<ContextSubscriptionResults>,
    pub myActiveResults: Ptr<SubscriptionResults>,
}

impl std::ops::Deref for Helper_SubscriptionWrapper {
    type Target = VariableWrapper;
    fn deref(&self) -> &VariableWrapper {
        &self.base
    }
}
impl std::ops::DerefMut for Helper_SubscriptionWrapper {
    fn deref_mut(&mut self) -> &mut VariableWrapper {
        &mut self.base
    }
}

impl Helper_SubscriptionWrapper {
    /// C++: `void SubscriptionWrapper::SubscriptionWrapper(VariableWrapper::SubscriptionHandler, SubscriptionResults &, ContextSubscriptionResults &)`
    // Helper::SubscriptionWrapper::SubscriptionWrapper(VariableWrapper::SubscriptionHandler handler, SubscriptionResults& into, ContextSubscriptionResults& context)
    //     : VariableWrapper(handler), myResults(into), myContextResults(context), myActiveResults(&into) {
    //
    // }
    #[allow(unused_variables)]
    pub fn new(handler: VariableWrapper_SubscriptionHandler, into: &mut SubscriptionResults, context: &mut ContextSubscriptionResults) -> Self {
        Helper_SubscriptionWrapper {
            base: VariableWrapper::new(handler),
            myResults: Ptr::from_mut(into),
            myContextResults: Ptr::from_mut(context),
            myActiveResults: Ptr::from_mut(into),
        }
    }

    /// C++: `void SubscriptionWrapper::setContext(const std::string *const)` [virtual]
    // void
    // Helper::SubscriptionWrapper::setContext(const std::string* const refID) {
    //     myActiveResults = refID == nullptr ? &myResults : &myContextResults[*refID];
    // }
    #[allow(unused_variables)]
    pub fn set_context(&mut self, refID: Ptr<String>) {
        // myActiveResults = refID == nullptr ? &myResults : &myContextResults[*refID];
        self.myActiveResults = if refID.is_null() {
            self.myResults
        } else {
            let key: String = (*refID).clone();
            Ptr::from_mut(self.myContextResults.m().entry(key).or_default())
        };
    }

    /// C++: `void SubscriptionWrapper::clear()` [virtual]
    // void
    // Helper::SubscriptionWrapper::clear() {
    //     myActiveResults = &myResults;
    //     myResults.clear();
    //     myContextResults.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        self.myActiveResults = self.myResults;
        self.myResults.m().clear();
        self.myContextResults.m().clear();
    }

    /// C++: `bool SubscriptionWrapper::wrapConnectionVector(const std::string &, const int, const std::vector<TraCIConnection> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapConnectionVector(const std::string& objID, const int variable, const std::vector<TraCIConnection>& value) {
    //     auto sl = std::make_shared<TraCIConnectionVectorWrapped>();
    //     sl->value = value;
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_connection_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIConnection>) -> bool {
        let sl = TraCIConnectionVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapDouble(const std::string &, const int, const double)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapDouble(const std::string& objID, const int variable, const double value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIDouble>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_double(&mut self, objID: &str, variable: i32, value: f64) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIDouble { base: TraCIResult { result_type: TYPE_DOUBLE }, value }));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapInt(const std::string &, const int, const int)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapInt(const std::string& objID, const int variable, const int value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIInt>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_int(&mut self, objID: &str, variable: i32, value: i32) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIInt::new(value, TYPE_INTEGER)));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapString(const std::string &, const int, const std::string &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapString(const std::string& objID, const int variable, const std::string& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIString>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string(&mut self, objID: &str, variable: i32, value: &str) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: value.to_string() }));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapStringList(const std::string &, const int, const std::vector<std::string> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapStringList(const std::string& objID, const int variable, const std::vector<std::string>& value) {
    //     auto sl = std::make_shared<TraCIStringList>();
    //     sl->value = value;
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_list(&mut self, objID: &str, variable: i32, value: &Vec<String>) -> bool {
        let sl = TraCIStringList { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapDoubleList(const std::string &, const int, const std::vector<double> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapDoubleList(const std::string& objID, const int variable, const std::vector<double>& value) {
    //     auto sl = std::make_shared<TraCIDoubleList>();
    //     sl->value = value;
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_double_list(&mut self, objID: &str, variable: i32, value: &Vec<f64>) -> bool {
        let sl = TraCIDoubleList { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapPosition(const std::string &, const int, const TraCIPosition &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapPosition(const std::string& objID, const int variable, const TraCIPosition& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIPosition>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_position(&mut self, objID: &str, variable: i32, value: &TraCIPosition) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIPosition { base: TraCIResult { result_type: if value.z != INVALID_DOUBLE_VALUE { POSITION_3D } else { POSITION_2D } }, x: value.x, y: value.y, z: value.z }));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapPositionVector(const std::string &, const int, const TraCIPositionVector &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapPositionVector(const std::string& objID, const int variable, const TraCIPositionVector& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIPositionVector>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_position_vector(&mut self, objID: &str, variable: i32, value: &TraCIPositionVector) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIPositionVector { base: TraCIResult::new(), value: value.value.clone() }));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapColor(const std::string &, const int, const TraCIColor &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapColor(const std::string& objID, const int variable, const TraCIColor& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIColor>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_color(&mut self, objID: &str, variable: i32, value: &TraCIColor) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIColor { base: TraCIResult::new(), r: value.r, g: value.g, b: value.b, a: value.a }));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapStringDoublePair(const std::string &, const int, const std::pair<std::string, double> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapStringDoublePair(const std::string& objID, const int variable, const std::pair<std::string, double>& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIRoadPosition>(value.first, value.second);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair(&mut self, objID: &str, variable: i32, value: &(String, f64)) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(TraCIRoadPosition::new(value.0.clone(), value.1, INVALID_INT_VALUE)));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapStringDoublePairList(const std::string &, const int, const std::vector<std::pair<std::string, double>> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapStringDoublePairList(const std::string& objID, const int variable, const std::vector<std::pair<std::string, double> >& value) {
    //     auto sl = std::make_shared<TraCIStringDoublePairList>();
    //     sl->value = value;
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair_list(&mut self, objID: &str, variable: i32, value: &Vec<(String, f64)>) -> bool {
        let sl = TraCIStringDoublePairList { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapStringPair(const std::string &, const int, const std::pair<std::string, std::string> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapStringPair(const std::string& objID, const int variable, const std::pair<std::string, std::string>& value) {
    //     auto sl = std::make_shared<TraCIStringList>();
    //     sl->value.push_back(value.first);
    //     sl->value.push_back(value.second);
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_pair(&mut self, objID: &str, variable: i32, value: &(String, String)) -> bool {
        let sl = TraCIStringList { base: TraCIResult::new(), value: vec![value.0.clone(), value.1.clone()] };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapIntPair(const std::string &, const int, const std::pair<int, int> &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapIntPair(const std::string& objID, const int variable, const std::pair<int, int>& value) {
    //     auto sl = std::make_shared<TraCIIntList>();
    //     sl->value.push_back(value.first);
    //     sl->value.push_back(value.second);
    //     (*myActiveResults)[objID][variable] = sl;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_int_pair(&mut self, objID: &str, variable: i32, value: &(i32, i32)) -> bool {
        let sl = TraCIIntList { base: TraCIResult::new(), value: vec![value.0, value.1] };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapStage(const std::string &, const int, const TraCIStage &)` [virtual]
    // bool
    // Helper::SubscriptionWrapper::wrapStage(const std::string& objID, const int variable, const TraCIStage& value) {
    //     (*myActiveResults)[objID][variable] = std::make_shared<TraCIStage>(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_stage(&mut self, objID: &str, variable: i32, value: &TraCIStage) -> bool {
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(value.clone()));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapReservationVector(const std::string &, const int, const std::vector<TraCIReservation> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_reservation_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIReservation>) -> bool {
        let sl = TraCIReservationVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapLogicVector(const std::string &, const int, const std::vector<TraCILogic> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_logic_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCILogic>) -> bool {
        let sl = TraCILogicVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapLinkVectorVector(const std::string &, const int, const std::vector<std::vector<TraCILink>> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_link_vector_vector(&mut self, objID: &str, variable: i32, value: &Vec<Vec<TraCILink>>) -> bool {
        let sl = TraCILinkVectorVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapSignalConstraintVector(const std::string &, const int, const std::vector<TraCISignalConstraint> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_signal_constraint_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCISignalConstraint>) -> bool {
        let sl = TraCISignalConstraintVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapJunctionFoeVector(const std::string &, const int, const std::vector<TraCIJunctionFoe> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_junction_foe_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIJunctionFoe>) -> bool {
        let sl = TraCIJunctionFoeVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapNextStopDataVector(const std::string &, const int, const std::vector<TraCINextStopData> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_next_stop_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCINextStopData>) -> bool {
        let sl = TraCINextStopDataVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapVehicleDataVector(const std::string &, const int, const std::vector<TraCIVehicleData> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_vehicle_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIVehicleData>) -> bool {
        let sl = TraCIVehicleDataVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapBestLanesDataVector(const std::string &, const int, const std::vector<TraCIBestLanesData> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_best_lanes_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCIBestLanesData>) -> bool {
        let sl = TraCIBestLanesDataVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `bool SubscriptionWrapper::wrapNextTLSDataVector(const std::string &, const int, const std::vector<TraCINextTLSData> &)` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn wrap_next_tls_data_vector(&mut self, objID: &str, variable: i32, value: &Vec<TraCINextTLSData>) -> bool {
        let sl = TraCINextTLSDataVectorWrapped { base: TraCIResult::new(), value: value.clone() };
        self.myActiveResults.m().entry(objID.to_string()).or_default().insert(variable, as_traci_result(sl));
        true
    }

    /// C++: `void SubscriptionWrapper::empty(const std::string &)` [virtual]
    // void
    // Helper::SubscriptionWrapper::empty(const std::string& objID) {
    //     (*myActiveResults)[objID]; // initiate the empty map to track the objectID for TRACI_ID_LIST context subscriptions
    // }
    #[allow(unused_variables)]
    pub fn empty(&mut self, objID: &str) {
        // initiate the empty map to track the objectID for TRACI_ID_LIST context subscriptions
        self.myActiveResults.m().entry(objID.to_string()).or_default();
    }

    /// C++: `SubscriptionWrapper & SubscriptionWrapper::operator=(const SubscriptionWrapper &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &Helper_SubscriptionWrapper) -> &mut Helper_SubscriptionWrapper {
        // C++ copy assignment: the subscription wrapper is not copyable in practice, copy the active result pointer and the result maps
        self.myResults = s.myResults;
        self.myContextResults = s.myContextResults;
        self.myActiveResults = s.myActiveResults;
        self
    }

}

/// C++ class `libsumo::Helper::TransportableStateListener` (libsumo/Helper.h)
#[repr(C)]
pub struct Helper_TransportableStateListener {
    /// C++ base class `MSNet::TransportableStateListener`
    pub base: MSNet_TransportableStateListener,
    /// Changes in the states of simulated transportables
    pub myTransportableStateChanges: BTreeMap<MSNet_TransportableState, Vec<String>>,
}

impl std::ops::Deref for Helper_TransportableStateListener {
    type Target = MSNet_TransportableStateListener;
    fn deref(&self) -> &MSNet_TransportableStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for Helper_TransportableStateListener {
    fn deref_mut(&mut self) -> &mut MSNet_TransportableStateListener {
        &mut self.base
    }
}

impl Helper_TransportableStateListener {
    /// C++: `void TransportableStateListener::transportableStateChanged(const MSTransportable *const, MSNet::TransportableState, const std::string &)` [virtual]
    // void
    // Helper::TransportableStateListener::transportableStateChanged(const MSTransportable* const transportable, MSNet::TransportableState to, const std::string& /*info*/) {
    //     myTransportableStateChanges[to].push_back(transportable->getID());
    // }
    #[allow(unused_variables)]
    pub fn transportable_state_changed(&mut self, transportable: Ptr<MSTransportable>, to: MSNet_TransportableState, info: &str) {
        self.myTransportableStateChanges.entry(to).or_default().push(transportable.get_id().to_string());
    }

}

/// C++ class `libsumo::Helper::VehicleStateListener` (libsumo/Helper.h)
#[repr(C)]
pub struct Helper_VehicleStateListener {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// Changes in the states of simulated vehicles
    pub myVehicleStateChanges: BTreeMap<MSNet_VehicleState, Vec<String>>,
}

impl std::ops::Deref for Helper_VehicleStateListener {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for Helper_VehicleStateListener {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

impl Helper_VehicleStateListener {
    /// C++: `void VehicleStateListener::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    // void
    // Helper::VehicleStateListener::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& /*info*/) {
    //     myVehicleStateChanges[to].push_back(vehicle->getID());
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        self.myVehicleStateChanges.entry(to).or_default().push(vehicle.get_id().to_string());
    }

}

