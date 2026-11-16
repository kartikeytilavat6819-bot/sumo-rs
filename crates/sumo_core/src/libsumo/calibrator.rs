//! Port of libsumo/Calibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::traci::traci_constants::*;

/// C++ class `libsumo::Calibrator` (libsumo/Calibrator.h)
pub struct Calibrator {
}

// C++ static members of Calibrator (port as module-level statics, see AGENTS.md):
//   static const int Calibrator::DOMAIN_ID;
//   static SubscriptionResults Calibrator::mySubscriptionResults;
//   static ContextSubscriptionResults Calibrator::myContextSubscriptionResults;

thread_local! {
    /// C++ static `Calibrator::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Calibrator::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl Calibrator {
    /// C++: `std::string Calibrator::getEdgeID(const std::string &)` [static]
    // std::string
    // Calibrator::getEdgeID(const std::string& calibratorID) {
    //     return getCalibrator(calibratorID)->getEdge()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(calibratorID: &str) -> String {
        Self::get_calibrator(calibratorID).get_edge().get_id().to_string()
    }

    /// C++: `std::string Calibrator::getLaneID(const std::string &)` [static]
    // std::string
    // Calibrator::getLaneID(const std::string& calibratorID) {
    //     const MSLane* lane = getCalibrator(calibratorID)->getLane();
    //     if (lane == nullptr) {
    //         return "";
    //     } else {
    //         return lane->getID();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(calibratorID: &str) -> String {
        let lane = Self::get_calibrator(calibratorID).get_lane();
        if lane.is_null() {
            String::new()
        } else {
            lane.get_id().to_string()
        }
    }

    /// C++: `double Calibrator::getVehsPerHour(const std::string &)` [static]
    // double
    // Calibrator::getVehsPerHour(const std::string& calibratorID) {
    //     return Helper::getCalibratorState(getCalibrator(calibratorID)).q;
    // }
    #[allow(unused_variables)]
    pub fn get_vehs_per_hour(calibratorID: &str) -> f64 {
        Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).q
    }

    /// C++: `double Calibrator::getSpeed(const std::string &)` [static]
    // double
    // Calibrator::getSpeed(const std::string& calibratorID) {
    //     return Helper::getCalibratorState(getCalibrator(calibratorID)).v;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(calibratorID: &str) -> f64 {
        Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).v
    }

    /// C++: `std::string Calibrator::getTypeID(const std::string &)` [static]
    // std::string
    // Calibrator::getTypeID(const std::string& calibratorID) {
    //     return Helper::getCalibratorState(getCalibrator(calibratorID)).vehicleParameter->vtypeid;
    // }
    #[allow(unused_variables)]
    pub fn get_type_id(calibratorID: &str) -> String {
        Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).vehicleParameter.vtypeid.clone()
    }

    /// C++: `double Calibrator::getBegin(const std::string &)` [static]
    // double
    // Calibrator::getBegin(const std::string& calibratorID) {
    //     return STEPS2TIME(Helper::getCalibratorState(getCalibrator(calibratorID)).begin);
    // }
    #[allow(unused_variables)]
    pub fn get_begin(calibratorID: &str) -> f64 {
        steps2time(Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).begin)
    }

    /// C++: `double Calibrator::getEnd(const std::string &)` [static]
    // double
    // Calibrator::getEnd(const std::string& calibratorID) {
    //     return STEPS2TIME(Helper::getCalibratorState(getCalibrator(calibratorID)).end);
    // }
    #[allow(unused_variables)]
    pub fn get_end(calibratorID: &str) -> f64 {
        steps2time(Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).end)
    }

    /// C++: `std::string Calibrator::getRouteID(const std::string &)` [static]
    // std::string
    // Calibrator::getRouteID(const std::string& calibratorID) {
    //     return Helper::getCalibratorState(getCalibrator(calibratorID)).vehicleParameter->routeid;
    // }
    #[allow(unused_variables)]
    pub fn get_route_id(calibratorID: &str) -> String {
        Helper::get_calibrator_state(Self::get_calibrator(calibratorID)).vehicleParameter.routeid.clone()
    }

    /// C++: `std::string Calibrator::getRouteProbeID(const std::string &)` [static]
    // std::string
    // Calibrator::getRouteProbeID(const std::string& calibratorID) {
    //     const MSRouteProbe* rp = getCalibrator(calibratorID)->getRouteProbe();
    //     if (rp == nullptr) {
    //         return "";
    //     } else {
    //         return rp->getID();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_route_probe_id(calibratorID: &str) -> String {
        let rp = Self::get_calibrator(calibratorID).get_route_probe();
        if rp.is_null() {
            String::new()
        } else {
            rp.get_id().to_string()
        }
    }

    /// C++: `std::vector<std::string> Calibrator::getVTypes(const std::string &)` [static]
    // std::vector<std::string>
    // Calibrator::getVTypes(const std::string& calibratorID) {
    //     std::vector<std::string> result;
    //     const std::set<std::string>& vTypes = getCalibrator(calibratorID)->getVehicleTypes();
    //     result.insert(result.begin(), vTypes.begin(), vTypes.end());
    //     std::sort(result.begin(), result.end());
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_v_types(calibratorID: &str) -> Vec<String> {
        // C++ MSCalibrator inherits getVehicleTypes() from MSDetectorFileOutput (std::set, already sorted)
        Self::get_calibrator(calibratorID).m().base3.get_vehicle_types().iter().cloned().collect()
    }

    /// C++: `int Calibrator::getPassed(const std::string &)` [static]
    // int
    // Calibrator::getPassed(const std::string& calibratorID) {
    //     return getCalibrator(calibratorID)->passed();
    // }
    #[allow(unused_variables)]
    pub fn get_passed(calibratorID: &str) -> i32 {
        Self::get_calibrator(calibratorID).passed()
    }

    /// C++: `int Calibrator::getInserted(const std::string &)` [static]
    // int
    // Calibrator::getInserted(const std::string& calibratorID) {
    //     return getCalibrator(calibratorID)->getInserted();
    // }
    #[allow(unused_variables)]
    pub fn get_inserted(calibratorID: &str) -> i32 {
        Self::get_calibrator(calibratorID).get_inserted()
    }

    /// C++: `int Calibrator::getRemoved(const std::string &)` [static]
    // int
    // Calibrator::getRemoved(const std::string& calibratorID) {
    //     return getCalibrator(calibratorID)->getRemoved();
    // }
    #[allow(unused_variables)]
    pub fn get_removed(calibratorID: &str) -> i32 {
        Self::get_calibrator(calibratorID).get_removed()
    }

    /// C++: `std::vector<std::string> Calibrator::getIDList()` [static]
    // std::vector<std::string>
    // Calibrator::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     for (const auto& item : MSCalibrator::getInstances()) {
    //         ids.push_back(item.first);
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        for (key, _) in MSCalibrator::get_instances() {
            ids.push(key.clone());
        }
        ids
    }

    /// C++: `int Calibrator::getIDCount()` [static]
    // int
    // Calibrator::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Calibrator::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Calibrator::getParameter(const std::string& calibratorID, const std::string& param) {
    //     const MSCalibrator* c = getCalibrator(calibratorID);
    //     return c->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        // Parameterised is reached through MSDetectorFileOutput (MSCalibrator : MSDetectorFileOutput)
        Self::get_calibrator(objectID).m().base3.base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Calibrator::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Calibrator)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Calibrator::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Calibrator::setParameter(const std::string& calibratorID, const std::string& key, const std::string& value) {
    //     MSCalibrator* c = getCalibrator(calibratorID);
    //     c->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_calibrator(objectID).m().base3.base2.set_parameter(key, value);
    }

    /// C++: `void Calibrator::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_CALIBRATOR_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Calibrator::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_CALIBRATOR_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Calibrator::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_CALIBRATOR_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Calibrator::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_CALIBRATOR_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Calibrator::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Calibrator::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Calibrator::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Calibrator::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Calibrator::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Calibrator, CALIBRATOR)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        // shortcut: TraCIString is a subclass wrapper (base: TraCIResult), so std::make_shared<TraCIString> cannot be stored as Rc<TraCIResult> yet, upgrade with the TraCIResult class hierarchy
        panic!("shortcut: Calibrator::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult>")
    }

    /// C++: `int Calibrator::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_CALIBRATOR_VARIABLE
    }

    /// C++: `void Calibrator::setFlow(const std::string &, double, double, double, double, const std::string &, const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Calibrator::setFlow(const std::string& calibratorID, double begin, double end, double vehsPerHour, double speed, const std::string& typeID,
    //                     const std::string& routeID,
    //                     const std::string& departLane,
    //                     const std::string& departSpeed) {
    //     std::string error;
    //     SUMOVehicleParameter vehicleParams;
    //     vehicleParams.vtypeid = typeID;
    //     vehicleParams.routeid = routeID;
    //     MSVehicleType* t = MSNet::getInstance()->getVehicleControl().getVType(typeID);
    //     if (t == nullptr) {
    //         throw TraCIException("Vehicle type '" + typeID + "' is not known");
    //     }
    //     if (!SUMOVehicleParameter::parseDepartLane(departLane, "calibrator", calibratorID, vehicleParams.departLane, vehicleParams.departLaneProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseDepartSpeed(departSpeed, "calibrator", calibratorID, vehicleParams.departSpeed, vehicleParams.departSpeedProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     getCalibrator(calibratorID)->setFlow(TIME2STEPS(begin), TIME2STEPS(end), vehsPerHour, speed, vehicleParams);
    // }
    #[allow(unused_variables)]
    pub fn set_flow(calibratorID: &str, begin: f64, end: f64, vehsPerHour: f64, speed: f64, typeID: &str, routeID: &str, departLane: &str, departSpeed: &str) {
        // shortcut: C++ throws TraCIException for an unknown type or bad depart lane/speed (here it panics), upgrade when libsumo functions return Result
        let mut error = String::new();
        let mut vehicleParams = SUMOVehicleParameter::new();
        vehicleParams.vtypeid = typeID.to_string();
        vehicleParams.routeid = routeID.to_string();
        if MSNet::get_instance().m().get_vehicle_control().get_v_type(typeID, Ptr::null(), false).is_null() {
            panic!("shortcut: Vehicle type '{typeID}' is not known (C++ TraCIException)");
        }
        if !SUMOVehicleParameter::parse_depart_lane(departLane, "calibrator", calibratorID, &mut vehicleParams.departLane, &mut vehicleParams.departLaneProcedure, &mut error) {
            panic!("shortcut: {error} (C++ TraCIException)");
        }
        if !SUMOVehicleParameter::parse_depart_speed(departSpeed, "calibrator", calibratorID, &mut vehicleParams.departSpeed, &mut vehicleParams.departSpeedProcedure, &mut error) {
            panic!("shortcut: {error} (C++ TraCIException)");
        }
        Self::get_calibrator(calibratorID).set_flow(time2steps(begin), time2steps(end), vehsPerHour, speed, vehicleParams);
    }

    /// C++: `std::shared_ptr<VariableWrapper> Calibrator::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Calibrator::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        // shortcut: Helper::SubscriptionWrapper borrows the class-level subscription statics by reference, which thread_local RefCells cannot hand out yet
        panic!("shortcut: Calibrator::makeWrapper needs the subscription wrapper over class statics")
    }

    /// C++: `bool Calibrator::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Calibrator::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_ROAD_ID:
    //             return wrapper->wrapString(objID, variable, getEdgeID(objID));
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case VAR_VEHSPERHOUR:
    //             return wrapper->wrapDouble(objID, variable, getVehsPerHour(objID));
    //         case VAR_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getSpeed(objID));
    //         case VAR_TYPE:
    //             return wrapper->wrapString(objID, variable, getTypeID(objID));
    //         case VAR_BEGIN:
    //             return wrapper->wrapDouble(objID, variable, getBegin(objID));
    //         case VAR_END:
    //             return wrapper->wrapDouble(objID, variable, getEnd(objID));
    //         case VAR_ROUTE_ID:
    //             return wrapper->wrapString(objID, variable, getRouteID(objID));
    //         case VAR_ROUTE_PROBE:
    //             return wrapper->wrapString(objID, variable, getRouteProbeID(objID));
    //         case VAR_VTYPES:
    //             return wrapper->wrapStringList(objID, variable, getVTypes(objID));
    //         case VAR_PASSED:
    //             return wrapper->wrapInt(objID, variable, getPassed(objID));
    //         case VAR_INSERTED:
    //             return wrapper->wrapInt(objID, variable, getInserted(objID));
    //         case VAR_REMOVED:
    //             return wrapper->wrapInt(objID, variable, getRemoved(objID));
    //         case libsumo::VAR_PARAMETER:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapString(objID, variable, getParameter(objID, paramData->readString()));
    //         case libsumo::VAR_PARAMETER_WITH_KEY:
    //             paramData->readUnsignedByte();
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, paramData->readString()));
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_ROAD_ID => wrapper.m().wrap_string(objID, variable, &Self::get_edge_id(objID)),
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            VAR_VEHSPERHOUR => wrapper.m().wrap_double(objID, variable, Self::get_vehs_per_hour(objID)),
            VAR_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_speed(objID)),
            VAR_TYPE => wrapper.m().wrap_string(objID, variable, &Self::get_type_id(objID)),
            VAR_BEGIN => wrapper.m().wrap_double(objID, variable, Self::get_begin(objID)),
            VAR_END => wrapper.m().wrap_double(objID, variable, Self::get_end(objID)),
            VAR_ROUTE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_route_id(objID)),
            VAR_ROUTE_PROBE => wrapper.m().wrap_string(objID, variable, &Self::get_route_probe_id(objID)),
            VAR_VTYPES => wrapper.m().wrap_string_list(objID, variable, &Self::get_v_types(objID)),
            VAR_PASSED => wrapper.m().wrap_int(objID, variable, Self::get_passed(objID)),
            VAR_INSERTED => wrapper.m().wrap_int(objID, variable, Self::get_inserted(objID)),
            VAR_REMOVED => wrapper.m().wrap_int(objID, variable, Self::get_removed(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte();
                // shortcut: C++ readString() throws ProcessError on malformed storage, here it panics, upgrade when handle_variable returns Result
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `MSCalibrator * Calibrator::getCalibrator(const std::string &)` [static]
    // MSCalibrator*
    // Calibrator::getCalibrator(const std::string& id) {
    //     const auto& dict = MSCalibrator::getInstances();
    //     auto it = dict.find(id);
    //     if (it == dict.end()) {
    //         throw TraCIException("Calibrator '" + id + "' is not known");
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_calibrator(id: &str) -> Ptr<MSCalibrator> {
        // shortcut: C++ throws TraCIException for an unknown id (here it panics), upgrade when libsumo functions return Result
        match MSCalibrator::get_instances().get(id) {
            Some(c) => *c,
            None => panic!("shortcut: Calibrator '{id}' is not known (C++ TraCIException)"),
        }
    }

    /// C++: `void Calibrator::Calibrator()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Calibrator cannot be constructed: the standard constructor is invalidated in C++")
    }

}

