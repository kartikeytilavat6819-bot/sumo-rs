//! Port of libsumo/MultiEntryExit.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::MultiEntryExit` (libsumo/MultiEntryExit.h)
pub struct MultiEntryExit {
}

// C++ static members of MultiEntryExit (port as module-level statics, see AGENTS.md):
//   static const int MultiEntryExit::DOMAIN_ID;
//   static SubscriptionResults MultiEntryExit::mySubscriptionResults;
//   static ContextSubscriptionResults MultiEntryExit::myContextSubscriptionResults;

impl MultiEntryExit {
    /// C++: `std::vector<std::string> MultiEntryExit::getEntryLanes(const std::string &)` [static]
    // std::vector<std::string>
    // MultiEntryExit::getEntryLanes(const std::string& detID) {
    //     std::vector<std::string> ids;
    //     for (const MSCrossSection& cs : getDetector(detID)->getEntries()) {
    //         ids.push_back(cs.myLane->getID());
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_entry_lanes(detID: &str) -> Vec<String> {
        todo!("port MultiEntryExit::getEntryLanes")
    }

    /// C++: `std::vector<std::string> MultiEntryExit::getExitLanes(const std::string &)` [static]
    // std::vector<std::string>
    // MultiEntryExit::getExitLanes(const std::string& detID) {
    //     std::vector<std::string> ids;
    //     for (const MSCrossSection& cs : getDetector(detID)->getExits()) {
    //         ids.push_back(cs.myLane->getID());
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_exit_lanes(detID: &str) -> Vec<String> {
        todo!("port MultiEntryExit::getExitLanes")
    }

    /// C++: `std::vector<double> MultiEntryExit::getEntryPositions(const std::string &)` [static]
    // std::vector<double>
    // MultiEntryExit::getEntryPositions(const std::string& detID) {
    //     std::vector<double> pos;
    //     for (const MSCrossSection& cs : getDetector(detID)->getEntries()) {
    //         pos.push_back(cs.myPosition);
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_entry_positions(detID: &str) -> Vec<f64> {
        todo!("port MultiEntryExit::getEntryPositions")
    }

    /// C++: `std::vector<double> MultiEntryExit::getExitPositions(const std::string &)` [static]
    // std::vector<double>
    // MultiEntryExit::getExitPositions(const std::string& detID) {
    //     std::vector<double> pos;
    //     for (const MSCrossSection& cs : getDetector(detID)->getExits()) {
    //         pos.push_back(cs.myPosition);
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_exit_positions(detID: &str) -> Vec<f64> {
        todo!("port MultiEntryExit::getExitPositions")
    }

    /// C++: `int MultiEntryExit::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // MultiEntryExit::getLastStepVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getVehiclesWithin();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(detID: &str) -> i32 {
        todo!("port MultiEntryExit::getLastStepVehicleNumber")
    }

    /// C++: `double MultiEntryExit::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // MultiEntryExit::getLastStepMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getCurrentMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(detID: &str) -> f64 {
        todo!("port MultiEntryExit::getLastStepMeanSpeed")
    }

    /// C++: `std::vector<std::string> MultiEntryExit::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // MultiEntryExit::getLastStepVehicleIDs(const std::string& detID) {
    //     return getDetector(detID)->getCurrentVehicleIDs();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(detID: &str) -> Vec<String> {
        todo!("port MultiEntryExit::getLastStepVehicleIDs")
    }

    /// C++: `int MultiEntryExit::getLastStepHaltingNumber(const std::string &)` [static]
    // int
    // MultiEntryExit::getLastStepHaltingNumber(const std::string& detID) {
    //     return getDetector(detID)->getCurrentHaltingNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_halting_number(detID: &str) -> i32 {
        todo!("port MultiEntryExit::getLastStepHaltingNumber")
    }

    /// C++: `double MultiEntryExit::getLastIntervalMeanTravelTime(const std::string &)` [static]
    // double
    // MultiEntryExit::getLastIntervalMeanTravelTime(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanTravelTime();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_travel_time(detID: &str) -> f64 {
        todo!("port MultiEntryExit::getLastIntervalMeanTravelTime")
    }

    /// C++: `double MultiEntryExit::getLastIntervalMeanHaltsPerVehicle(const std::string &)` [static]
    // double
    // MultiEntryExit::getLastIntervalMeanHaltsPerVehicle(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanHaltsPerVehicle();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_halts_per_vehicle(detID: &str) -> f64 {
        todo!("port MultiEntryExit::getLastIntervalMeanHaltsPerVehicle")
    }

    /// C++: `double MultiEntryExit::getLastIntervalMeanTimeLoss(const std::string &)` [static]
    // double
    // MultiEntryExit::getLastIntervalMeanTimeLoss(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanTimeLoss();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_time_loss(detID: &str) -> f64 {
        todo!("port MultiEntryExit::getLastIntervalMeanTimeLoss")
    }

    /// C++: `int MultiEntryExit::getLastIntervalVehicleSum(const std::string &)` [static]
    // int
    // MultiEntryExit::getLastIntervalVehicleSum(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalVehicleSum();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_sum(detID: &str) -> i32 {
        todo!("port MultiEntryExit::getLastIntervalVehicleSum")
    }

    /// C++: `std::vector<std::string> MultiEntryExit::getIDList()` [static]
    // std::vector<std::string>
    // MultiEntryExit::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_ENTRY_EXIT_DETECTOR).insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port MultiEntryExit::getIDList")
    }

    /// C++: `int MultiEntryExit::getIDCount()` [static]
    // int
    // MultiEntryExit::getIDCount() {
    //     return (int)MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_ENTRY_EXIT_DETECTOR).size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port MultiEntryExit::getIDCount")
    }

    /// C++: `std::string MultiEntryExit::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // MultiEntryExit::getParameter(const std::string& detID, const std::string& param) {
    //     return getDetector(detID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port MultiEntryExit::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> MultiEntryExit::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(MultiEntryExit)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port MultiEntryExit::getParameterWithKey")
    }

    /// C++: `void MultiEntryExit::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // MultiEntryExit::setParameter(const std::string& detID, const std::string& name, const std::string& value) {
    //     getDetector(detID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port MultiEntryExit::setParameter")
    }

    /// C++: `void MultiEntryExit::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port MultiEntryExit::subscribe")
    }

    /// C++: `void MultiEntryExit::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port MultiEntryExit::unsubscribe")
    }

    /// C++: `void MultiEntryExit::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port MultiEntryExit::subscribeContext")
    }

    /// C++: `void MultiEntryExit::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port MultiEntryExit::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults MultiEntryExit::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port MultiEntryExit::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults MultiEntryExit::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port MultiEntryExit::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults MultiEntryExit::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port MultiEntryExit::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults MultiEntryExit::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port MultiEntryExit::getContextSubscriptionResults")
    }

    /// C++: `void MultiEntryExit::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(MultiEntryExit, MULTIENTRYEXIT)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port MultiEntryExit::subscribeParameterWithKey")
    }

    /// C++: `int MultiEntryExit::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port MultiEntryExit::domainID")
    }

    /// C++: `std::shared_ptr<VariableWrapper> MultiEntryExit::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // MultiEntryExit::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port MultiEntryExit::makeWrapper")
    }

    /// C++: `bool MultiEntryExit::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // MultiEntryExit::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case LAST_STEP_VEHICLE_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepVehicleNumber(objID));
    //         case LAST_STEP_MEAN_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getLastStepMeanSpeed(objID));
    //         case LAST_STEP_VEHICLE_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLastStepVehicleIDs(objID));
    //         case LAST_STEP_VEHICLE_HALTING_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepHaltingNumber(objID));
    //         case VAR_LAST_INTERVAL_TRAVELTIME:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanTravelTime(objID));
    //         case VAR_LAST_INTERVAL_MEAN_HALTING_NUMBER:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanHaltsPerVehicle(objID));
    //         case VAR_TIMELOSS:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanTimeLoss(objID));
    //         case VAR_LAST_INTERVAL_VEHICLE_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastIntervalVehicleSum(objID));
    //         case VAR_LANES:
    //             return wrapper->wrapStringList(objID, variable, getEntryLanes(objID));
    //         case VAR_EXIT_LANES:
    //             return wrapper->wrapStringList(objID, variable, getExitLanes(objID));
    //         case VAR_POSITION:
    //             return wrapper->wrapDoubleList(objID, variable, getEntryPositions(objID));
    //         case VAR_EXIT_POSITIONS:
    //             return wrapper->wrapDoubleList(objID, variable, getExitPositions(objID));
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
        todo!("port MultiEntryExit::handleVariable")
    }

    /// C++: `MSE3Collector * MultiEntryExit::getDetector(const std::string &)` [static]
    // MSE3Collector*
    // MultiEntryExit::getDetector(const std::string& id) {
    //     MSE3Collector* e3 = dynamic_cast<MSE3Collector*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_ENTRY_EXIT_DETECTOR).get(id));
    //     if (e3 == nullptr) {
    //         throw TraCIException("Multi entry exit detector '" + id + "' is not known");
    //     }
    //     return e3;
    // }
    #[allow(unused_variables)]
    pub fn get_detector(detID: &str) -> Ptr<MSE3Collector> {
        todo!("port MultiEntryExit::getDetector")
    }

    /// C++: `void MultiEntryExit::MultiEntryExit()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MultiEntryExit::MultiEntryExit")
    }

}

