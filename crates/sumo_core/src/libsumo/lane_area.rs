//! Port of libsumo/LaneArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::LaneArea` (libsumo/LaneArea.h)
pub struct LaneArea {
}

// C++ static members of LaneArea (port as module-level statics, see AGENTS.md):
//   static const int LaneArea::DOMAIN_ID;
//   static NamedRTree * LaneArea::myTree;
//   static SubscriptionResults LaneArea::mySubscriptionResults;
//   static ContextSubscriptionResults LaneArea::myContextSubscriptionResults;

impl LaneArea {
    /// C++: `int LaneArea::getJamLengthVehicle(const std::string &)` [static]
    // int
    // LaneArea::getJamLengthVehicle(const std::string& detID) {
    //     return getDetector(detID)->getCurrentJamLengthInVehicles();
    // }
    #[allow(unused_variables)]
    pub fn get_jam_length_vehicle(detID: &str) -> i32 {
        todo!("port LaneArea::getJamLengthVehicle")
    }

    /// C++: `double LaneArea::getJamLengthMeters(const std::string &)` [static]
    // double
    // LaneArea::getJamLengthMeters(const std::string& detID) {
    //     return getDetector(detID)->getCurrentJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_jam_length_meters(detID: &str) -> f64 {
        todo!("port LaneArea::getJamLengthMeters")
    }

    /// C++: `double LaneArea::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getLastStepMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getCurrentMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(detID: &str) -> f64 {
        todo!("port LaneArea::getLastStepMeanSpeed")
    }

    /// C++: `std::vector<std::string> LaneArea::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // LaneArea::getLastStepVehicleIDs(const std::string& detID) {
    //     return getDetector(detID)->getCurrentVehicleIDs();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(detID: &str) -> Vec<String> {
        todo!("port LaneArea::getLastStepVehicleIDs")
    }

    /// C++: `double LaneArea::getLastStepOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getLastStepOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getCurrentOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(detID: &str) -> f64 {
        todo!("port LaneArea::getLastStepOccupancy")
    }

    /// C++: `double LaneArea::getPosition(const std::string &)` [static]
    // double
    // LaneArea::getPosition(const std::string& detID) {
    //     return getDetector(detID)->getStartPos();
    // }
    #[allow(unused_variables)]
    pub fn get_position(detID: &str) -> f64 {
        todo!("port LaneArea::getPosition")
    }

    /// C++: `std::string LaneArea::getLaneID(const std::string &)` [static]
    // std::string
    // LaneArea::getLaneID(const std::string& detID) {
    //     return getDetector(detID)->getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(detID: &str) -> String {
        todo!("port LaneArea::getLaneID")
    }

    /// C++: `double LaneArea::getLength(const std::string &)` [static]
    // double
    // LaneArea::getLength(const std::string& detID) {
    //     const MSE2Collector* const e2 = getDetector(detID);
    //     return e2->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(detID: &str) -> f64 {
        todo!("port LaneArea::getLength")
    }

    /// C++: `int LaneArea::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastStepVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getCurrentVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(detID: &str) -> i32 {
        todo!("port LaneArea::getLastStepVehicleNumber")
    }

    /// C++: `int LaneArea::getLastStepHaltingNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastStepHaltingNumber(const std::string& detID) {
    //     return getDetector(detID)->getCurrentHaltingNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_halting_number(detID: &str) -> i32 {
        todo!("port LaneArea::getLastStepHaltingNumber")
    }

    /// C++: `double LaneArea::getIntervalOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getIntervalOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getIntervalOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_occupancy(detID: &str) -> f64 {
        todo!("port LaneArea::getIntervalOccupancy")
    }

    /// C++: `double LaneArea::getIntervalMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_speed(detID: &str) -> f64 {
        todo!("port LaneArea::getIntervalMeanSpeed")
    }

    /// C++: `double LaneArea::getIntervalMeanTimeLoss(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMeanTimeLoss(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMeanTimeLoss();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_time_loss(detID: &str) -> f64 {
        todo!("port LaneArea::getIntervalMeanTimeLoss")
    }

    /// C++: `double LaneArea::getIntervalMaxJamLengthInMeters(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMaxJamLengthInMeters(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMaxJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_max_jam_length_in_meters(detID: &str) -> f64 {
        todo!("port LaneArea::getIntervalMaxJamLengthInMeters")
    }

    /// C++: `int LaneArea::getIntervalVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getIntervalVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getIntervalVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_number(detID: &str) -> i32 {
        todo!("port LaneArea::getIntervalVehicleNumber")
    }

    /// C++: `double LaneArea::getLastIntervalOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_occupancy(detID: &str) -> f64 {
        todo!("port LaneArea::getLastIntervalOccupancy")
    }

    /// C++: `double LaneArea::getLastIntervalMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_speed(detID: &str) -> f64 {
        todo!("port LaneArea::getLastIntervalMeanSpeed")
    }

    /// C++: `double LaneArea::getLastIntervalMeanTimeLoss(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMeanTimeLoss(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanTimeLoss();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_time_loss(detID: &str) -> f64 {
        todo!("port LaneArea::getLastIntervalMeanTimeLoss")
    }

    /// C++: `double LaneArea::getLastIntervalMaxJamLengthInMeters(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMaxJamLengthInMeters(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMaxJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_max_jam_length_in_meters(detID: &str) -> f64 {
        todo!("port LaneArea::getLastIntervalMaxJamLengthInMeters")
    }

    /// C++: `int LaneArea::getLastIntervalVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastIntervalVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_number(detID: &str) -> i32 {
        todo!("port LaneArea::getLastIntervalVehicleNumber")
    }

    /// C++: `void LaneArea::overrideVehicleNumber(const std::string &, int)` [static]
    // void
    // LaneArea::overrideVehicleNumber(const std::string& detID, int vehNum) {
    //     getDetector(detID)->overrideVehicleNumber(vehNum);
    // }
    #[allow(unused_variables)]
    pub fn override_vehicle_number(detID: &str, vehNum: i32) {
        todo!("port LaneArea::overrideVehicleNumber")
    }

    /// C++: `std::vector<std::string> LaneArea::getIDList()` [static]
    // std::vector<std::string>
    // LaneArea::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port LaneArea::getIDList")
    }

    /// C++: `int LaneArea::getIDCount()` [static]
    // int
    // LaneArea::getIDCount() {
    //     std::vector<std::string> ids;
    //     return (int)MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port LaneArea::getIDCount")
    }

    /// C++: `std::string LaneArea::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // LaneArea::getParameter(const std::string& detID, const std::string& param) {
    //     return getDetector(detID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port LaneArea::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> LaneArea::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(LaneArea)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port LaneArea::getParameterWithKey")
    }

    /// C++: `void LaneArea::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // LaneArea::setParameter(const std::string& detID, const std::string& name, const std::string& value) {
    //     getDetector(detID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port LaneArea::setParameter")
    }

    /// C++: `void LaneArea::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port LaneArea::subscribe")
    }

    /// C++: `void LaneArea::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port LaneArea::unsubscribe")
    }

    /// C++: `void LaneArea::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port LaneArea::subscribeContext")
    }

    /// C++: `void LaneArea::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port LaneArea::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults LaneArea::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port LaneArea::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults LaneArea::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port LaneArea::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults LaneArea::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port LaneArea::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults LaneArea::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port LaneArea::getContextSubscriptionResults")
    }

    /// C++: `void LaneArea::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port LaneArea::subscribeParameterWithKey")
    }

    /// C++: `int LaneArea::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port LaneArea::domainID")
    }

    /// C++: `NamedRTree * LaneArea::getTree()` [static]
    /// Returns a tree filled with inductive loop instances
    // NamedRTree*
    // LaneArea::getTree() {
    //     if (myTree == nullptr) {
    //         myTree = new NamedRTree();
    //         for (const std::string& id : getIDList()) {
    //             PositionVector shape;
    //             storeShape(id, shape);
    //             Boundary b = shape.getBoxBoundary();
    //             const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //             const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //             myTree->Insert(cmin, cmax, getDetector(id));
    //         }
    //     }
    //     return myTree;
    // }
    #[allow(unused_variables)]
    pub fn get_tree() -> Ptr<Unmapped /* NamedRTree */> {
        todo!("port LaneArea::getTree")
    }

    /// C++: `void LaneArea::cleanup()` [static]
    // void
    // LaneArea::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port LaneArea::cleanup")
    }

    /// C++: `void LaneArea::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // LaneArea::storeShape(const std::string& id, PositionVector& shape) {
    //     MSE2Collector* const det = getDetector(id);
    //     shape.push_back(det->getLanes().front()->getShape().positionAtOffset(det->getStartPos()));
    //     shape.push_back(det->getLanes().back()->getShape().positionAtOffset(det->getEndPos()));
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port LaneArea::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> LaneArea::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // LaneArea::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port LaneArea::makeWrapper")
    }

    /// C++: `bool LaneArea::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // LaneArea::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
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
    //         case JAM_LENGTH_VEHICLE:
    //             return wrapper->wrapInt(objID, variable, getJamLengthVehicle(objID));
    //         case JAM_LENGTH_METERS:
    //             return wrapper->wrapDouble(objID, variable, getJamLengthMeters(objID));
    //         case LAST_STEP_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastStepOccupancy(objID));
    //         case VAR_POSITION:
    //             return wrapper->wrapDouble(objID, variable, getPosition(objID));
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case VAR_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLength(objID));
    //         case VAR_INTERVAL_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getIntervalOccupancy(objID));
    //         case VAR_INTERVAL_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getIntervalMeanSpeed(objID));
    //         case VAR_INTERVAL_TIMELOSS:
    //             return wrapper->wrapDouble(objID, variable, getIntervalMeanTimeLoss(objID));
    //         case VAR_INTERVAL_MAX_JAM_LENGTH_METERS:
    //             return wrapper->wrapDouble(objID, variable, getIntervalMaxJamLengthInMeters(objID));
    //         case VAR_INTERVAL_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getIntervalVehicleNumber(objID));
    //         case VAR_LAST_INTERVAL_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalOccupancy(objID));
    //         case VAR_LAST_INTERVAL_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanSpeed(objID));
    //         case VAR_LAST_INTERVAL_TIMELOSS:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanTimeLoss(objID));
    //         case VAR_LAST_INTERVAL_MAX_JAM_LENGTH_METERS:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMaxJamLengthInMeters(objID));
    //         case VAR_LAST_INTERVAL_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastIntervalVehicleNumber(objID));
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
        todo!("port LaneArea::handleVariable")
    }

    /// C++: `MSE2Collector * LaneArea::getDetector(const std::string &)` [static, private]
    // MSE2Collector*
    // LaneArea::getDetector(const std::string& id) {
    //     MSE2Collector* e2 = dynamic_cast<MSE2Collector*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).get(id));
    //     if (e2 == nullptr) {
    //         throw TraCIException("Lane area detector '" + id + "' is not known");
    //     }
    //     return e2;
    // }
    #[allow(unused_variables)]
    pub fn get_detector(detID: &str) -> Ptr<MSE2Collector> {
        todo!("port LaneArea::getDetector")
    }

    /// C++: `void LaneArea::LaneArea()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port LaneArea::LaneArea")
    }

}

