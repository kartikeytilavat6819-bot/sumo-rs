//! Port of libsumo/ParkingArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
// shortcut: C++ throw TraCIException(msg) is ported as panic!("{}", msg), upgrade when TraCIException is ported
use crate::prelude::*;

/// C++ class `libsumo::ParkingArea` (libsumo/ParkingArea.h)
pub struct ParkingArea {
}

// C++ static members of ParkingArea (port as module-level statics, see AGENTS.md):
//   static const int ParkingArea::DOMAIN_ID;
//   static SubscriptionResults ParkingArea::mySubscriptionResults;
//   static ContextSubscriptionResults ParkingArea::myContextSubscriptionResults;
thread_local! {
    /// C++ static `ParkingArea::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `ParkingArea::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}


impl ParkingArea {
    /// C++: `std::string ParkingArea::getLaneID(const std::string &)` [static]
    // std::string
    // ParkingArea::getLaneID(const std::string& stopID) {
    //     return getParkingArea(stopID)->getLane().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(stopID: &str) -> String {
        Self::get_parking_area(stopID).get_lane().get_id().to_string()
    }

    /// C++: `double ParkingArea::getStartPos(const std::string &)` [static]
    // double
    // ParkingArea::getStartPos(const std::string& stopID) {
    //     return getParkingArea(stopID)->getBeginLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_start_pos(stopID: &str) -> f64 {
        Self::get_parking_area(stopID).get_begin_lane_position()
    }

    /// C++: `double ParkingArea::getEndPos(const std::string &)` [static]
    // double
    // ParkingArea::getEndPos(const std::string& stopID) {
    //     return getParkingArea(stopID)->getEndLanePosition();
    // }
    #[allow(unused_variables)]
    pub fn get_end_pos(stopID: &str) -> f64 {
        Self::get_parking_area(stopID).get_end_lane_position()
    }

    /// C++: `std::string ParkingArea::getName(const std::string &)` [static]
    // std::string
    // ParkingArea::getName(const std::string& stopID) {
    //     return getParkingArea(stopID)->getMyName();
    // }
    #[allow(unused_variables)]
    pub fn get_name(stopID: &str) -> String {
        Self::get_parking_area(stopID).get_my_name().to_string()
    }

    /// C++: `int ParkingArea::getVehicleCount(const std::string &)` [static]
    // int
    // ParkingArea::getVehicleCount(const std::string& stopID) {
    //     return (int)getParkingArea(stopID)->getStoppedVehicles().size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_count(stopID: &str) -> i32 {
        Self::get_parking_area(stopID).get_stopped_vehicles().len() as i32
    }

    /// C++: `std::vector<std::string> ParkingArea::getVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // ParkingArea::getVehicleIDs(const std::string& stopID) {
    //     std::vector<std::string> result;
    //     for (const SUMOVehicle* veh : getParkingArea(stopID)->getStoppedVehicles()) {
    //         result.push_back(veh->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_i_ds(stopID: &str) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for veh in Self::get_parking_area(stopID).get_stopped_vehicles() {
            result.push(veh.get_id().to_string());
        }
        result
    }

    /// C++: `std::vector<std::string> ParkingArea::getAcceptedBadges(const std::string &)` [static]
    // std::vector<std::string>
    // ParkingArea::getAcceptedBadges(const std::string& stopID) {
    //     return dynamic_cast<MSParkingArea*>(getParkingArea(stopID))->getAcceptedBadges();
    // }
    #[allow(unused_variables)]
    pub fn get_accepted_badges(stopID: &str) -> Vec<String> {
        // shortcut: C++ dynamic_cast is replaced by a cast, valid because the lookup is for a PARKING_AREA stopping place; MSParkingArea needs #[repr(C)] (base first), upgrade when ms_parking_area.rs adds it
        Self::get_parking_area(stopID).cast::<MSParkingArea>().get_accepted_badges()
    }

    /// C++: `std::vector<std::string> ParkingArea::getIDList()` [static]
    // std::vector<std::string>
    // ParkingArea::getIDList() {
    //     std::vector<std::string> ids;
    //     for (auto& item : MSNet::getInstance()->getStoppingPlaces(SUMO_TAG_PARKING_AREA)) {
    //         ids.push_back(item.first);
    //     }
    //     std::sort(ids.begin(), ids.end());
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let mut ids: Vec<String> = Vec::new();
        for item in MSNet::get_instance().get_stopping_places(SUMO_TAG_PARKING_AREA) {
            ids.push(item.0.to_string());
        }
        ids.sort();
        ids
    }

    /// C++: `int ParkingArea::getIDCount()` [static]
    // int
    // ParkingArea::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string ParkingArea::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // ParkingArea::getParameter(const std::string& stopID, const std::string& param) {
    //     return getParkingArea(stopID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_parking_area(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> ParkingArea::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(ParkingArea)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void ParkingArea::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // ParkingArea::setParameter(const std::string& stopID, const std::string& key, const std::string& value) {
    //     getParkingArea(stopID)->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_parking_area(objectID).base2.set_parameter(key, value);
    }

    /// C++: `void ParkingArea::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_PARKINGAREA_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void ParkingArea::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_PARKINGAREA_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void ParkingArea::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_PARKINGAREA_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void ParkingArea::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_PARKINGAREA_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults ParkingArea::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults ParkingArea::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults ParkingArea::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults ParkingArea::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void ParkingArea::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(ParkingArea, PARKINGAREA)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_PARKINGAREA_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int ParkingArea::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_PARKINGAREA_VARIABLE
    }

    /// C++: `void ParkingArea::setAcceptedBadges(const std::string &, const std::vector<std::string> &)` [static]
    // void
    // ParkingArea::setAcceptedBadges(const std::string& stopID, const std::vector<std::string>& badges) {
    //     dynamic_cast<MSParkingArea*>(getParkingArea(stopID))->setAcceptedBadges(badges);
    // }
    #[allow(unused_variables)]
    pub fn set_accepted_badges(stopID: &str, badges: &Vec<String>) {
        // shortcut: C++ dynamic_cast is replaced by a cast, valid because the lookup is for a PARKING_AREA stopping place; MSParkingArea needs #[repr(C)] (base first), upgrade when ms_parking_area.rs adds it
        let pa = Self::get_parking_area(stopID).cast::<MSParkingArea>();
        pa.m().myAcceptedBadges.clear();
        pa.m().myAcceptedBadges.extend(badges.iter().cloned());
    }

    /// C++: `std::shared_ptr<VariableWrapper> ParkingArea::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // ParkingArea::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool ParkingArea::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // ParkingArea::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
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
    //         case VAR_ACCESS_BADGE:
    //             return wrapper->wrapStringList(objID, variable, getAcceptedBadges(objID));
    //         case VAR_STOP_STARTING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getVehicleCount(objID));
    //         case VAR_STOP_STARTING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getVehicleIDs(objID));
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
            VAR_ACCESS_BADGE => wrapper.m().wrap_string_list(objID, variable, &Self::get_accepted_badges(objID)),
            VAR_STOP_STARTING_VEHICLES_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_vehicle_count(objID)),
            VAR_STOP_STARTING_VEHICLES_IDS => wrapper.m().wrap_string_list(objID, variable, &Self::get_vehicle_i_ds(objID)),
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

    /// C++: `MSStoppingPlace * ParkingArea::getParkingArea(const std::string &)` [static, private]
    // MSStoppingPlace*
    // ParkingArea::getParkingArea(const std::string& id) {
    //     return Helper::getStoppingPlace(id, SUMO_TAG_PARKING_AREA);
    // }
    #[allow(unused_variables)]
    pub fn get_parking_area(id: &str) -> Ptr<MSStoppingPlace> {
        Helper::get_stopping_place(id, SUMO_TAG_PARKING_AREA)
    }

    /// C++: `void ParkingArea::ParkingArea()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::ParkingArea cannot be constructed: the standard constructor is invalidated in C++")
    }

}

