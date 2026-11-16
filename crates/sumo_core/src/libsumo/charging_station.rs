//! Port of libsumo/ChargingStation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::ChargingStation` (libsumo/ChargingStation.h)
pub struct ChargingStation {
}

// C++ static members of ChargingStation (port as module-level statics, see AGENTS.md):
//   static const int ChargingStation::DOMAIN_ID;
//   static SubscriptionResults ChargingStation::mySubscriptionResults;
//   static ContextSubscriptionResults ChargingStation::myContextSubscriptionResults;

thread_local! {
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl ChargingStation {
    /// C++: `std::string ChargingStation::getLaneID(const std::string &)` [static]
    // std::string
    // ChargingStation::getLaneID(const std::string& stopID) {
    //     return getChargingStation(stopID)->getLane().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(stopID: &str) -> String {
        Self::get_charging_station(stopID).get_lane().get_id().to_string()
    }

    /// C++: `double ChargingStation::getStartPos(const std::string &)` [static]
    // double
    // ChargingStation::getStartPos(const std::string& stopID) {
    //     return getChargingStation(stopID)->getBeginLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_start_pos(stopID: &str) -> f64 {
        Self::get_charging_station(stopID).get_begin_lane_position()
    }

    /// C++: `double ChargingStation::getEndPos(const std::string &)` [static]
    // double
    // ChargingStation::getEndPos(const std::string& stopID) {
    //     return getChargingStation(stopID)->getEndLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_end_pos(stopID: &str) -> f64 {
        Self::get_charging_station(stopID).get_end_lane_position()
    }

    /// C++: `std::string ChargingStation::getName(const std::string &)` [static]
    // std::string
    // ChargingStation::getName(const std::string& stopID) {
    //     return getChargingStation(stopID)->getMyName();
    // }
    #[allow(unused_variables)]
    pub fn get_name(stopID: &str) -> String {
        Self::get_charging_station(stopID).get_my_name().to_string()
    }

    /// C++: `int ChargingStation::getVehicleCount(const std::string &)` [static]
    // int
    // ChargingStation::getVehicleCount(const std::string& stopID) {
    //     return (int)getChargingStation(stopID)->getStoppedVehicles().size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_count(stopID: &str) -> i32 {
        Self::get_charging_station(stopID).get_stopped_vehicles().len() as i32
    }

    /// C++: `std::vector<std::string> ChargingStation::getVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // ChargingStation::getVehicleIDs(const std::string& stopID) {
    //     std::vector<std::string> result;
    //     for (const SUMOVehicle* veh : getChargingStation(stopID)->getStoppedVehicles()) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_i_ds(stopID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for veh in Self::get_charging_station(stopID).get_stopped_vehicles() {
            result.push(veh.get_id().to_string());
        }
        result
    }

    /// C++: `double ChargingStation::getChargingPower(const std::string &)` [static]
    // double
    // ChargingStation::getChargingPower(const std::string& stopID) {
    //     return dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->getChargingPower(true);
    // }
    #[allow(unused_variables)]
    pub fn get_charging_power(stopID: &str) -> f64 {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().get_charging_power(true)
    }

    /// C++: `double ChargingStation::getEfficiency(const std::string &)` [static]
    // double
    // ChargingStation::getEfficiency(const std::string& stopID) {
    //     return dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->getEfficency();
    // }
    #[allow(unused_variables)]
    pub fn get_efficiency(stopID: &str) -> f64 {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().get_efficency()
    }

    /// C++: `double ChargingStation::getChargeDelay(const std::string &)` [static]
    // double
    // ChargingStation::getChargeDelay(const std::string& stopID) {
    //     return STEPS2TIME(dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->getChargeDelay());
    // }
    #[allow(unused_variables)]
    pub fn get_charge_delay(stopID: &str) -> f64 {
        utils::common::time::steps2time(Self::get_charging_station(stopID).cast::<MSChargingStation>().get_charge_delay())
    }

    /// C++: `int ChargingStation::getChargeInTransit(const std::string &)` [static]
    // int
    // ChargingStation::getChargeInTransit(const std::string& stopID) {
    //     return dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->getChargeInTransit();
    // }
    #[allow(unused_variables)]
    pub fn get_charge_in_transit(stopID: &str) -> i32 {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().get_charge_in_transit() as i32
    }

    /// C++: `double ChargingStation::getTotalPower(const std::string &)` [static]
    // double
    // ChargingStation::getTotalPower(const std::string& stopID) {
    //     return dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->getTotalChargingPower();
    // }
    #[allow(unused_variables)]
    pub fn get_total_power(stopID: &str) -> f64 {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().get_total_charging_power()
    }

    /// C++: `std::vector<std::string> ChargingStation::getIDList()` [static]
    // std::vector<std::string>
    // ChargingStation::getIDList() {
    //     std::vector<std::string> ids;
    //     for (auto& item : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_CHARGING_STATION)) {
    //         ids.push_back(item.first);
    //     }
    //     std::sort(ids.begin(), ids.end());
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let mut ids = Vec::new();
        for item in MSNet::get_instance().get_stopping_places(SUMO_TAG_CHARGING_STATION) {
            ids.push(item.0.to_string());
        }
        ids.sort();
        ids
    }

    /// C++: `int ChargingStation::getIDCount()` [static]
    // int
    // ChargingStation::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string ChargingStation::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // ChargingStation::getParameter(const std::string& stopID, const std::string& param) {
    //     return getChargingStation(stopID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_charging_station(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> ChargingStation::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(ChargingStation)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void ChargingStation::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // ChargingStation::setParameter(const std::string& stopID, const std::string& key, const std::string& value) {
    //     getChargingStation(stopID)->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_charging_station(objectID).m().base2.set_parameter(key, value);
    }

    /// C++: `void ChargingStation::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_CHARGINGSTATION_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void ChargingStation::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_CHARGINGSTATION_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void ChargingStation::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        // No body provided for this method in the original C++
    }

    /// C++: `void ChargingStation::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults ChargingStation::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults ChargingStation::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults ChargingStation::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults ChargingStation::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void ChargingStation::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ChargingStation, CHARGINGSTATION)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_CHARGINGSTATION_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int ChargingStation::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        return 0;
    }

    /// C++: `void ChargingStation::setChargingPower(const std::string &, double)` [static]
    // void
    // ChargingStation::setChargingPower(const std::string& stopID, double power) {
    //     dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->setChargingPower(power);
    // }
    #[allow(unused_variables)]
    pub fn set_charging_power(stopID: &str, power: f64) {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().m().set_charging_power(power);
    }

    /// C++: `void ChargingStation::setEfficiency(const std::string &, double)` [static]
    // void
    // ChargingStation::setEfficiency(const std::string& stopID, double efficiency) {
    //     dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->setEfficiency(efficiency);
    // }
    #[allow(unused_variables)]
    pub fn set_efficiency(stopID: &str, efficiency: f64) {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().m().set_efficiency(efficiency);
    }

    /// C++: `void ChargingStation::setChargeDelay(const std::string &, double)` [static]
    // void
    // ChargingStation::setChargeDelay(const std::string& stopID, double delay) {
    //     dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->setChargeDelay(TIME2STEPS(delay));
    // }
    #[allow(unused_variables)]
    pub fn set_charge_delay(stopID: &str, delay: f64) {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().m().set_charge_delay(utils::common::time::time2steps(delay));
    }

    /// C++: `void ChargingStation::setChargeInTransit(const std::string &, bool)` [static]
    // void
    // ChargingStation::setChargeInTransit(const std::string& stopID, bool inTransit) {
    //     dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->setChargeInTransit(inTransit);
    // }
    #[allow(unused_variables)]
    pub fn set_charge_in_transit(stopID: &str, inTransit: bool) {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().m().set_charge_in_transit(inTransit);
    }

    /// C++: `void ChargingStation::setTotalPower(const std::string &, double)` [static]
    // void
    // ChargingStation::setTotalPower(const std::string& stopID, double totalPower) {
    //     dynamic_cast<MSChargingStation*>(getChargingStation(stopID))->setTotalChargingPower(totalPower);
    // }
    #[allow(unused_variables)]
    pub fn set_total_power(stopID: &str, totalPower: f64) {
        Self::get_charging_station(stopID).cast::<MSChargingStation>().m().set_total_charging_power(totalPower);
    }

    /// C++: `std::shared_ptr<VariableWrapper> ChargingStation::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // ChargingStation::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool ChargingStation::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // ChargingStation::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
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
    //         case VAR_CS_POWER:
    //             return wrapper->wrapDouble(objID, variable, getChargingPower(objID));
    //         case VAR_CS_EFFICIENCY:
    //             return wrapper->wrapDouble(objID, variable, getEfficiency(objID));
    //         case VAR_CS_CHARGE_DELAY:
    //             return wrapper->wrapDouble(objID, variable, getChargeDelay(objID));
    //         case VAR_CS_CHARGE_IN_TRANSIT:
    //             return wrapper->wrapInt(objID, variable, getChargeInTransit(objID));
    //         case VAR_CS_TOTAL_POWER:
    //             return wrapper->wrapDouble(objID, variable, getTotalPower(objID));
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
            VAR_CS_POWER => wrapper.m().wrap_double(objID, variable, Self::get_charging_power(objID)),
            VAR_CS_EFFICIENCY => wrapper.m().wrap_double(objID, variable, Self::get_efficiency(objID)),
            VAR_CS_CHARGE_DELAY => wrapper.m().wrap_double(objID, variable, Self::get_charge_delay(objID)),
            VAR_CS_CHARGE_IN_TRANSIT => wrapper.m().wrap_int(objID, variable, Self::get_charge_in_transit(objID)),
            VAR_CS_TOTAL_POWER => wrapper.m().wrap_double(objID, variable, Self::get_total_power(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte();
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte();
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
    }

    /// C++: `MSStoppingPlace * ChargingStation::getChargingStation(const std::string &)` [static, private]
    // MSStoppingPlace*
    // ChargingStation::getChargingStation(const std::string& id) {
    //     return Helper::getStoppingPlace(id, SUMO_TAG_CHARGING_STATION);
    // }
    #[allow(unused_variables)]
    pub fn get_charging_station(id: &str) -> Ptr<MSStoppingPlace> {
        Helper::get_stopping_place(id, SUMO_TAG_CHARGING_STATION)
    }

    /// C++: `void ChargingStation::ChargingStation()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            // Initialize fields here if needed
        }
    }

}

