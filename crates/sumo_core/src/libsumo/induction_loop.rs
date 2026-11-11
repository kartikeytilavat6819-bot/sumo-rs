//! Port of libsumo/InductionLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::InductionLoop` (libsumo/InductionLoop.h)
pub struct InductionLoop {
}

// C++ static members of InductionLoop (port as module-level statics, see AGENTS.md):
//   static const int InductionLoop::DOMAIN_ID;
//   static SubscriptionResults InductionLoop::mySubscriptionResults;
//   static ContextSubscriptionResults InductionLoop::myContextSubscriptionResults;
//   static NamedRTree * InductionLoop::myTree;

impl InductionLoop {
    /// C++: `double InductionLoop::getPosition(const std::string &)` [static]
    // double
    // InductionLoop::getPosition(const std::string& loopID) {
    //     return getDetector(loopID)->getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_position(loopID: &str) -> f64 {
        todo!("port InductionLoop::getPosition")
    }

    /// C++: `std::string InductionLoop::getLaneID(const std::string &)` [static]
    // std::string
    // InductionLoop::getLaneID(const std::string& loopID) {
    //     return getDetector(loopID)->getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(loopID: &str) -> String {
        todo!("port InductionLoop::getLaneID")
    }

    /// C++: `int InductionLoop::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // InductionLoop::getLastStepVehicleNumber(const std::string& loopID) {
    //     return (int)getDetector(loopID)->getEnteredNumber((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(loopID: &str) -> i32 {
        todo!("port InductionLoop::getLastStepVehicleNumber")
    }

    /// C++: `double InductionLoop::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepMeanSpeed(const std::string& loopID) {
    //     return getDetector(loopID)->getSpeed((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(loopID: &str) -> f64 {
        todo!("port InductionLoop::getLastStepMeanSpeed")
    }

    /// C++: `std::vector<std::string> InductionLoop::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // InductionLoop::getLastStepVehicleIDs(const std::string& loopID) {
    //     return getDetector(loopID)->getVehicleIDs((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(loopID: &str) -> Vec<String> {
        todo!("port InductionLoop::getLastStepVehicleIDs")
    }

    /// C++: `double InductionLoop::getLastStepOccupancy(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepOccupancy(const std::string& loopID) {
    //     return getDetector(loopID)->getOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(loopID: &str) -> f64 {
        todo!("port InductionLoop::getLastStepOccupancy")
    }

    /// C++: `double InductionLoop::getLastStepMeanLength(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepMeanLength(const std::string& loopID) {
    //     return getDetector(loopID)->getVehicleLength((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_length(loopID: &str) -> f64 {
        todo!("port InductionLoop::getLastStepMeanLength")
    }

    /// C++: `double InductionLoop::getTimeSinceDetection(const std::string &)` [static]
    // double
    // InductionLoop::getTimeSinceDetection(const std::string& loopID) {
    //     return getDetector(loopID)->getTimeSinceLastDetection();
    // }
    #[allow(unused_variables)]
    pub fn get_time_since_detection(loopID: &str) -> f64 {
        todo!("port InductionLoop::getTimeSinceDetection")
    }

    /// C++: `std::vector<libsumo::TraCIVehicleData> InductionLoop::getVehicleData(const std::string &)` [static]
    // std::vector<libsumo::TraCIVehicleData>
    // InductionLoop::getVehicleData(const std::string& loopID) {
    //     const std::vector<MSInductLoop::VehicleData> vd = getDetector(loopID)->collectVehiclesOnDet(SIMSTEP - DELTA_T, true, true);
    //     std::vector<libsumo::TraCIVehicleData> tvd;
    //     for (const MSInductLoop::VehicleData& vdi : vd) {
    //         tvd.push_back(libsumo::TraCIVehicleData());
    //         tvd.back().id = vdi.idM;
    //         tvd.back().length = vdi.lengthM;
    //         tvd.back().entryTime = vdi.entryTimeM;
    //         tvd.back().leaveTime = vdi.leaveTimeM;
    //         tvd.back().typeID = vdi.typeIDM;
    //     }
    //     return tvd;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_data(loopID: &str) -> Vec<TraCIVehicleData> {
        todo!("port InductionLoop::getVehicleData")
    }

    /// C++: `double InductionLoop::getIntervalOccupancy(const std::string &)` [static]
    // double
    // InductionLoop::getIntervalOccupancy(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         const MEInductLoop* det = getMEDetector(loopID);
    //         const auto& meanData = det->getMeanData();
    //         return meanData.getOccupancy(SIMSTEP - meanData.getResetTime(), det->getEdge().getNumLanes());
    //     } else {
    //         return getDetector(loopID)->getIntervalOccupancy();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_interval_occupancy(loopID: &str) -> f64 {
        todo!("port InductionLoop::getIntervalOccupancy")
    }

    /// C++: `double InductionLoop::getIntervalMeanSpeed(const std::string &)` [static]
    // double
    // InductionLoop::getIntervalMeanSpeed(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         const MEInductLoop* det = getMEDetector(loopID);
    //         const auto& meanData = det->getMeanData();
    //         if (meanData.getSamples() != 0) {
    //             return meanData.getTravelledDistance() / meanData.getSamples();
    //         } else {
    //             const double defaultTravelTime = det->getEdge().getLength() / det->getEdge().getSpeedLimit();
    //             return meanData.getLaneLength() / defaultTravelTime;
    //         }
    //     } else {
    //         return getDetector(loopID)->getIntervalMeanSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_speed(loopID: &str) -> f64 {
        todo!("port InductionLoop::getIntervalMeanSpeed")
    }

    /// C++: `int InductionLoop::getIntervalVehicleNumber(const std::string &)` [static]
    // int
    // InductionLoop::getIntervalVehicleNumber(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         const auto& meanData = getMEDetector(loopID)->getMeanData();
    //         return meanData.nVehDeparted + meanData.nVehEntered;
    //     } else {
    //         return getDetector(loopID)->getIntervalVehicleNumber();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_number(loopID: &str) -> i32 {
        todo!("port InductionLoop::getIntervalVehicleNumber")
    }

    /// C++: `std::vector<std::string> InductionLoop::getIntervalVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // InductionLoop::getIntervalVehicleIDs(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         WRITE_ERROR("getIntervalVehicleIDs not applicable for meso");
    //         return std::vector<std::string>();
    //     }
    //     return getDetector(loopID)->getIntervalVehicleIDs();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_i_ds(loopID: &str) -> Vec<String> {
        todo!("port InductionLoop::getIntervalVehicleIDs")
    }

    /// C++: `double InductionLoop::getLastIntervalOccupancy(const std::string &)` [static]
    // double
    // InductionLoop::getLastIntervalOccupancy(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         WRITE_ERROR("getLastIntervalOccupancy not applicable for meso");
    //         return INVALID_DOUBLE_VALUE;
    //     }
    //     return getDetector(loopID)->getIntervalOccupancy(true);
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_occupancy(loopID: &str) -> f64 {
        todo!("port InductionLoop::getLastIntervalOccupancy")
    }

    /// C++: `double InductionLoop::getLastIntervalMeanSpeed(const std::string &)` [static]
    // double
    // InductionLoop::getLastIntervalMeanSpeed(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         WRITE_ERROR("getLastIntervalMeanSpeed not applicable for meso");
    //         return INVALID_DOUBLE_VALUE;
    //     }
    //     return getDetector(loopID)->getIntervalMeanSpeed(true);
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_speed(loopID: &str) -> f64 {
        todo!("port InductionLoop::getLastIntervalMeanSpeed")
    }

    /// C++: `int InductionLoop::getLastIntervalVehicleNumber(const std::string &)` [static]
    // int
    // InductionLoop::getLastIntervalVehicleNumber(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         WRITE_ERROR("getLastIntervalVehicleNumber not applicable for meso");
    //         return INVALID_INT_VALUE;
    //     }
    //     return getDetector(loopID)->getIntervalVehicleNumber(true);
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_number(loopID: &str) -> i32 {
        todo!("port InductionLoop::getLastIntervalVehicleNumber")
    }

    /// C++: `std::vector<std::string> InductionLoop::getLastIntervalVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // InductionLoop::getLastIntervalVehicleIDs(const std::string& loopID) {
    //     if (MSGlobals::gUseMesoSim) {
    //         WRITE_ERROR("getLastIntervalVehicleIDs not applicable for meso");
    //         return std::vector<std::string>();
    //     }
    //     return getDetector(loopID)->getIntervalVehicleIDs(true);
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_i_ds(loopID: &str) -> Vec<String> {
        todo!("port InductionLoop::getLastIntervalVehicleIDs")
    }

    /// C++: `void InductionLoop::overrideTimeSinceDetection(const std::string &, double)` [static]
    // void
    // InductionLoop::overrideTimeSinceDetection(const std::string& loopID, double time) {
    //     getDetector(loopID)->overrideTimeSinceDetection(time);
    // }
    #[allow(unused_variables)]
    pub fn override_time_since_detection(loopID: &str, time: f64) {
        todo!("port InductionLoop::overrideTimeSinceDetection")
    }

    /// C++: `std::vector<std::string> InductionLoop::getIDList()` [static]
    // std::vector<std::string>
    // InductionLoop::getIDList() {
    //     std::vector<std::string> ids;
    //     MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port InductionLoop::getIDList")
    }

    /// C++: `int InductionLoop::getIDCount()` [static]
    // int
    // InductionLoop::getIDCount() {
    //     std::vector<std::string> ids;
    //     return (int)MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port InductionLoop::getIDCount")
    }

    /// C++: `std::string InductionLoop::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // InductionLoop::getParameter(const std::string& loopID, const std::string& param) {
    //     return getDetector(loopID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port InductionLoop::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> InductionLoop::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(InductionLoop)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port InductionLoop::getParameterWithKey")
    }

    /// C++: `void InductionLoop::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // InductionLoop::setParameter(const std::string& loopID, const std::string& name, const std::string& value) {
    //     getDetector(loopID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port InductionLoop::setParameter")
    }

    /// C++: `void InductionLoop::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port InductionLoop::subscribe")
    }

    /// C++: `void InductionLoop::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port InductionLoop::unsubscribe")
    }

    /// C++: `void InductionLoop::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port InductionLoop::subscribeContext")
    }

    /// C++: `void InductionLoop::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port InductionLoop::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults InductionLoop::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port InductionLoop::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults InductionLoop::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port InductionLoop::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults InductionLoop::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port InductionLoop::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults InductionLoop::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port InductionLoop::getContextSubscriptionResults")
    }

    /// C++: `void InductionLoop::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port InductionLoop::subscribeParameterWithKey")
    }

    /// C++: `int InductionLoop::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port InductionLoop::domainID")
    }

    /// C++: `NamedRTree * InductionLoop::getTree()` [static]
    /// Returns a tree filled with inductive loop instances
    // NamedRTree*
    // InductionLoop::getTree() {
    //     if (myTree == nullptr) {
    //         myTree = new NamedRTree();
    //         for (const auto& i : MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP)) {
    //             MSInductLoop* il = static_cast<MSInductLoop*>(i.second);
    //             Position p = il->getLane()->getShape().positionAtOffset(il->getPosition());
    //             const float cmin[2] = {(float) p.x(), (float) p.y()};
    //             const float cmax[2] = {(float) p.x(), (float) p.y()};
    //             myTree->Insert(cmin, cmax, il);
    //         }
    //     }
    //     return myTree;
    // }
    #[allow(unused_variables)]
    pub fn get_tree() -> Ptr<Unmapped /* NamedRTree */> {
        todo!("port InductionLoop::getTree")
    }

    /// C++: `void InductionLoop::cleanup()` [static]
    // void
    // InductionLoop::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port InductionLoop::cleanup")
    }

    /// C++: `void InductionLoop::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // InductionLoop::storeShape(const std::string& id, PositionVector& shape) {
    //     MSInductLoop* const il = getDetector(id);
    //     shape.push_back(il->getLane()->getShape().positionAtOffset(il->getPosition()));
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port InductionLoop::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> InductionLoop::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // InductionLoop::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port InductionLoop::makeWrapper")
    }

    /// C++: `bool InductionLoop::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // InductionLoop::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_POSITION:
    //             return wrapper->wrapDouble(objID, variable, getPosition(objID));
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case LAST_STEP_VEHICLE_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepVehicleNumber(objID));
    //         case LAST_STEP_MEAN_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getLastStepMeanSpeed(objID));
    //         case LAST_STEP_VEHICLE_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLastStepVehicleIDs(objID));
    //         case LAST_STEP_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastStepOccupancy(objID));
    //         case LAST_STEP_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLastStepMeanLength(objID));
    //         case LAST_STEP_TIME_SINCE_DETECTION:
    //             return wrapper->wrapDouble(objID, variable, getTimeSinceDetection(objID));
    //         case VAR_INTERVAL_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getIntervalOccupancy(objID));
    //         case VAR_INTERVAL_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getIntervalMeanSpeed(objID));
    //         case VAR_INTERVAL_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getIntervalVehicleNumber(objID));
    //         case VAR_INTERVAL_IDS:
    //             return wrapper->wrapStringList(objID, variable, getIntervalVehicleIDs(objID));
    //         case VAR_LAST_INTERVAL_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalOccupancy(objID));
    //         case VAR_LAST_INTERVAL_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getLastIntervalMeanSpeed(objID));
    //         case VAR_LAST_INTERVAL_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastIntervalVehicleNumber(objID));
    //         case VAR_LAST_INTERVAL_IDS:
    //             return wrapper->wrapStringList(objID, variable, getLastIntervalVehicleIDs(objID));
    //         case LAST_STEP_VEHICLE_DATA:
    //             return wrapper->wrapVehicleDataVector(objID, variable, getVehicleData(objID));
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
        todo!("port InductionLoop::handleVariable")
    }

    /// C++: `MSInductLoop * InductionLoop::getDetector(const std::string &)` [static, private]
    // MSInductLoop*
    // InductionLoop::getDetector(const std::string& id) {
    //     MSInductLoop* il = dynamic_cast<MSInductLoop*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).get(id));
    //     if (il == nullptr) {
    //         throw TraCIException("Induction loop '" + id + "' is not known");
    //     }
    //     return il;
    // }
    #[allow(unused_variables)]
    pub fn get_detector(loopID: &str) -> Ptr<MSInductLoop> {
        todo!("port InductionLoop::getDetector")
    }

    /// C++: `MEInductLoop * InductionLoop::getMEDetector(const std::string &)` [static, private]
    // MEInductLoop*
    // InductionLoop::getMEDetector(const std::string& id) {
    //     MEInductLoop* il = dynamic_cast<MEInductLoop*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).get(id));
    //     if (il == nullptr) {
    //         throw TraCIException("Induction loop '" + id + "' is not known");
    //     }
    //     return il;
    // }
    #[allow(unused_variables)]
    pub fn get_me_detector(loopID: &str) -> Ptr<MEInductLoop> {
        todo!("port InductionLoop::getMEDetector")
    }

    /// C++: `void InductionLoop::InductionLoop()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port InductionLoop::InductionLoop")
    }

}

