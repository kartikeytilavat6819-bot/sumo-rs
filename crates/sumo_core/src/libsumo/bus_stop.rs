//! Port of libsumo/BusStop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::traci::traci_constants::*;

/// C++ class `libsumo::BusStop` (libsumo/BusStop.h)
pub struct BusStop {
}

// C++ static members of BusStop (port as module-level statics, see AGENTS.md):
//   static const int BusStop::DOMAIN_ID;
//   static SubscriptionResults BusStop::mySubscriptionResults;
//   static ContextSubscriptionResults BusStop::myContextSubscriptionResults;

thread_local! {
    /// C++ static `BusStop::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `BusStop::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl BusStop {
    /// C++: `std::string BusStop::getLaneID(const std::string &)` [static]
    // std::string
    // BusStop::getLaneID(const std::string& stopID) {
    //     return getBusStop(stopID)->getLane().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(stopID: &str) -> String {
        Self::get_bus_stop(stopID).get_lane().get_id().to_string()
    }

    /// C++: `double BusStop::getStartPos(const std::string &)` [static]
    // double
    // BusStop::getStartPos(const std::string& stopID) {
    //     return getBusStop(stopID)->getBeginLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_start_pos(stopID: &str) -> f64 {
        Self::get_bus_stop(stopID).get_begin_lane_position()
    }

    /// C++: `double BusStop::getEndPos(const std::string &)` [static]
    // double
    // BusStop::getEndPos(const std::string& stopID) {
    //     return getBusStop(stopID)->getEndLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_end_pos(stopID: &str) -> f64 {
        Self::get_bus_stop(stopID).get_end_lane_position()
    }

    /// C++: `std::string BusStop::getName(const std::string &)` [static]
    // std::string
    // BusStop::getName(const std::string& stopID) {
    //     return getBusStop(stopID)->getMyName();
    // }
    #[allow(unused_variables)]
    pub fn get_name(stopID: &str) -> String {
        Self::get_bus_stop(stopID).get_my_name().to_string()
    }

    /// C++: `int BusStop::getVehicleCount(const std::string &)` [static]
    // int
    // BusStop::getVehicleCount(const std::string& stopID) {
    //     return (int)getBusStop(stopID)->getStoppedVehicles().size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_count(stopID: &str) -> i32 {
        Self::get_bus_stop(stopID).get_stopped_vehicles().len() as i32
    }

    /// C++: `std::vector<std::string> BusStop::getVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // BusStop::getVehicleIDs(const std::string& stopID) {
    //     std::vector<std::string> result;
    //     for (const SUMOVehicle* veh : getBusStop(stopID)->getStoppedVehicles()) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_i_ds(stopID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for veh in Self::get_bus_stop(stopID).get_stopped_vehicles() {
            result.push(veh.get_id().to_string());
        }
        result
    }

    /// C++: `int BusStop::getPersonCount(const std::string &)` [static]
    // int
    // BusStop::getPersonCount(const std::string& stopID) {
    //     return (int)getBusStop(stopID)->getNumWaitingPersons();
    // }
    #[allow(unused_variables)]
    pub fn get_person_count(stopID: &str) -> i32 {
        Self::get_bus_stop(stopID).get_num_waiting_persons()
    }

    /// C++: `std::vector<std::string> BusStop::getPersonIDs(const std::string &)` [static]
    // std::vector<std::string>
    // BusStop::getPersonIDs(const std::string& stopID) {
    //     std::vector<std::string> result;
    //     getBusStop(stopID)->getWaitingPersonIDs(result);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_person_i_ds(stopID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Self::get_bus_stop(stopID).get_waiting_person_i_ds(&mut result);
        result
    }

    /// C++: `std::vector<std::string> BusStop::getIDList()` [static]
    // std::vector<std::string>
    // BusStop::getIDList() {
    //     std::vector<std::string> ids;
    //     for (auto& item : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_BUS_STOP)) {
    //         ids.push_back(item.first);
    //     }
    //     std::sort(ids.begin(), ids.end());
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        // shortcut: needs MSNet::getStoppingPlaces, which is an unmapped NamedObjectCont in ms_net.rs, iterate it sorted by id once typed
        panic!("shortcut: BusStop::getIDList needs MSNet::getStoppingPlaces (unmapped type)")
    }

    /// C++: `int BusStop::getIDCount()` [static]
    // int
    // BusStop::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string BusStop::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // BusStop::getParameter(const std::string& stopID, const std::string& param) {
    //     return getBusStop(stopID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_bus_stop(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> BusStop::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(BusStop)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void BusStop::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // BusStop::setParameter(const std::string& stopID, const std::string& key, const std::string& value) {
    //     getBusStop(stopID)->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_bus_stop(objectID).base2.set_parameter(key, value);
    }

    /// C++: `void BusStop::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_BUSSTOP_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void BusStop::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_BUSSTOP_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void BusStop::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_BUSSTOP_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void BusStop::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_BUSSTOP_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults BusStop::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults BusStop::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults BusStop::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults BusStop::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void BusStop::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(BusStop, BUSSTOP)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        // shortcut: TraCIString is a subclass wrapper (base: TraCIResult), so std::make_shared<TraCIString> cannot be stored as Rc<TraCIResult> yet, upgrade with the TraCIResult class hierarchy
        panic!("shortcut: BusStop::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult>")
    }

    /// C++: `int BusStop::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_BUSSTOP_VARIABLE
    }

    /// C++: `std::shared_ptr<VariableWrapper> BusStop::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // BusStop::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        // shortcut: Helper::SubscriptionWrapper borrows the class-level subscription statics by reference, which thread_local RefCells cannot hand out yet
        panic!("shortcut: BusStop::makeWrapper needs the subscription wrapper over class statics")
    }

    /// C++: `bool BusStop::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // BusStop::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case VAR_POSITION:
    //             return wrapper->wrapDouble(objID, variable, getStartPos(objID));
    //         case VAR_LANEPOSITION:
    //             return wrapper->wrapDouble(objID, variable, getEndPos(objID));
    //         case VAR_NAME:
    //             return wrapper->wrapString(objID, variable, getName(objID));
    //         case VAR_STOP_STARTING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getVehicleCount(objID));
    //         case VAR_STOP_STARTING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getVehicleIDs(objID));
    //         case VAR_BUS_STOP_WAITING:
    //             return wrapper->wrapInt(objID, variable, getPersonCount(objID));
    //         case VAR_BUS_STOP_WAITING_IDS:
    //             return wrapper->wrapStringList(objID, variable, getPersonIDs(objID));
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
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            VAR_POSITION => wrapper.m().wrap_double(objID, variable, Self::get_start_pos(objID)),
            VAR_LANEPOSITION => wrapper.m().wrap_double(objID, variable, Self::get_end_pos(objID)),
            VAR_NAME => wrapper.m().wrap_string(objID, variable, &Self::get_name(objID)),
            VAR_STOP_STARTING_VEHICLES_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_vehicle_count(objID)),
            VAR_STOP_STARTING_VEHICLES_IDS => wrapper.m().wrap_string_list(objID, variable, &Self::get_vehicle_i_ds(objID)),
            VAR_BUS_STOP_WAITING => wrapper.m().wrap_int(objID, variable, Self::get_person_count(objID)),
            VAR_BUS_STOP_WAITING_IDS => wrapper.m().wrap_string_list(objID, variable, &Self::get_person_i_ds(objID)),
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

    /// C++: `MSStoppingPlace * BusStop::getBusStop(const std::string &)` [static, private]
    // MSStoppingPlace*
    // BusStop::getBusStop(const std::string& id) {
    //     return Helper::getStoppingPlace(id, SUMO_TAG_BUS_STOP);
    // }
    #[allow(unused_variables)]
    pub fn get_bus_stop(id: &str) -> Ptr<MSStoppingPlace> {
        Helper::get_stopping_place(id, SUMO_TAG_BUS_STOP)
    }

    /// C++: `void BusStop::BusStop()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::BusStop cannot be constructed: the standard constructor is invalidated in C++")
    }

}

