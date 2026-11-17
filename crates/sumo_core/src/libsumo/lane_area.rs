//! Port of libsumo/LaneArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_error;
use crate::utils::named_rtree::{delete_named_rtree, new_named_rtree, NamedRTree};

/// C++ class `libsumo::LaneArea` (libsumo/LaneArea.h)
pub struct LaneArea {
}

// shortcut: C++ throws TraCIException(msg) for unknown detectors (here panic!("{}", msg)), and libsumo functions are not Result-based yet; upgrade when they return Result
// C++ static members of LaneArea (port as module-level statics, see AGENTS.md):
//   static const int LaneArea::DOMAIN_ID;
//   static NamedRTree * LaneArea::myTree;
//   static SubscriptionResults LaneArea::mySubscriptionResults;
//   static ContextSubscriptionResults LaneArea::myContextSubscriptionResults;

thread_local! {
    /// C++ static `LaneArea::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `LaneArea::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
    /// C++ static `LaneArea::myTree`
    static MY_TREE: std::cell::Cell<Ptr<NamedRTree>> = std::cell::Cell::new(Ptr::null());
}

impl LaneArea {
    /// C++: `int LaneArea::getJamLengthVehicle(const std::string &)` [static]
    // int
    // LaneArea::getJamLengthVehicle(const std::string& detID) {
    //     return getDetector(detID)->getCurrentJamLengthInVehicles();
    // }
    #[allow(unused_variables)]
    pub fn get_jam_length_vehicle(detID: &str) -> i32 {
        Self::get_detector(detID).get_current_jam_length_in_vehicles()
    }

    /// C++: `double LaneArea::getJamLengthMeters(const std::string &)` [static]
    // double
    // LaneArea::getJamLengthMeters(const std::string& detID) {
    //     return getDetector(detID)->getCurrentJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_jam_length_meters(detID: &str) -> f64 {
        Self::get_detector(detID).get_current_jam_length_in_meters()
    }

    /// C++: `double LaneArea::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getLastStepMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getCurrentMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(detID: &str) -> f64 {
        Self::get_detector(detID).get_current_mean_speed()
    }

    /// C++: `std::vector<std::string> LaneArea::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // LaneArea::getLastStepVehicleIDs(const std::string& detID) {
    //     return getDetector(detID)->getCurrentVehicleIDs();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(detID: &str) -> Vec<String> {
        Self::get_detector(detID).get_current_vehicle_i_ds()
    }

    /// C++: `double LaneArea::getLastStepOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getLastStepOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getCurrentOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(detID: &str) -> f64 {
        Self::get_detector(detID).get_current_occupancy()
    }

    /// C++: `double LaneArea::getPosition(const std::string &)` [static]
    // double
    // LaneArea::getPosition(const std::string& detID) {
    //     return getDetector(detID)->getStartPos();
    // }
    #[allow(unused_variables)]
    pub fn get_position(detID: &str) -> f64 {
        Self::get_detector(detID).get_start_pos()
    }

    /// C++: `std::string LaneArea::getLaneID(const std::string &)` [static]
    // std::string
    // LaneArea::getLaneID(const std::string& detID) {
    //     return getDetector(detID)->getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(detID: &str) -> String {
        Self::get_detector(detID).get_lane().get_id().to_string()
    }

    /// C++: `double LaneArea::getLength(const std::string &)` [static]
    // double
    // LaneArea::getLength(const std::string& detID) {
    //     const MSE2Collector* const e2 = getDetector(detID);
    //     return e2->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(detID: &str) -> f64 {
        let e2 = Self::get_detector(detID);
        e2.get_length()
    }

    /// C++: `int LaneArea::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastStepVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getCurrentVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(detID: &str) -> i32 {
        Self::get_detector(detID).get_current_vehicle_number()
    }

    /// C++: `int LaneArea::getLastStepHaltingNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastStepHaltingNumber(const std::string& detID) {
    //     return getDetector(detID)->getCurrentHaltingNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_halting_number(detID: &str) -> i32 {
        Self::get_detector(detID).get_current_halting_number()
    }

    /// C++: `double LaneArea::getIntervalOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getIntervalOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getIntervalOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_occupancy(detID: &str) -> f64 {
        Self::get_detector(detID).get_interval_occupancy()
    }

    /// C++: `double LaneArea::getIntervalMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_speed(detID: &str) -> f64 {
        Self::get_detector(detID).get_interval_mean_speed()
    }

    /// C++: `double LaneArea::getIntervalMeanTimeLoss(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMeanTimeLoss(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMeanTimeLoss();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_time_loss(detID: &str) -> f64 {
        Self::get_detector(detID).get_interval_mean_time_loss()
    }

    /// C++: `double LaneArea::getIntervalMaxJamLengthInMeters(const std::string &)` [static]
    // double
    // LaneArea::getIntervalMaxJamLengthInMeters(const std::string& detID) {
    //     return getDetector(detID)->getIntervalMaxJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_max_jam_length_in_meters(detID: &str) -> f64 {
        Self::get_detector(detID).get_interval_max_jam_length_in_meters()
    }

    /// C++: `int LaneArea::getIntervalVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getIntervalVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getIntervalVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_number(detID: &str) -> i32 {
        Self::get_detector(detID).get_interval_vehicle_number()
    }

    /// C++: `double LaneArea::getLastIntervalOccupancy(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalOccupancy(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_occupancy(detID: &str) -> f64 {
        Self::get_detector(detID).get_last_interval_occupancy()
    }

    /// C++: `double LaneArea::getLastIntervalMeanSpeed(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMeanSpeed(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_speed(detID: &str) -> f64 {
        Self::get_detector(detID).get_last_interval_mean_speed()
    }

    /// C++: `double LaneArea::getLastIntervalMeanTimeLoss(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMeanTimeLoss(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMeanTimeLoss();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_time_loss(detID: &str) -> f64 {
        Self::get_detector(detID).get_last_interval_mean_time_loss()
    }

    /// C++: `double LaneArea::getLastIntervalMaxJamLengthInMeters(const std::string &)` [static]
    // double
    // LaneArea::getLastIntervalMaxJamLengthInMeters(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalMaxJamLengthInMeters();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_max_jam_length_in_meters(detID: &str) -> f64 {
        Self::get_detector(detID).get_last_interval_max_jam_length_in_meters()
    }

    /// C++: `int LaneArea::getLastIntervalVehicleNumber(const std::string &)` [static]
    // int
    // LaneArea::getLastIntervalVehicleNumber(const std::string& detID) {
    //     return getDetector(detID)->getLastIntervalVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_number(detID: &str) -> i32 {
        Self::get_detector(detID).get_last_interval_vehicle_number()
    }

    /// C++: `void LaneArea::overrideVehicleNumber(const std::string &, int)` [static]
    // void
    // LaneArea::overrideVehicleNumber(const std::string& detID, int vehNum) {
    //     getDetector(detID)->overrideVehicleNumber(vehNum);
    // }
    #[allow(unused_variables)]
    pub fn override_vehicle_number(detID: &str, vehNum: i32) {
        Self::get_detector(detID).m().override_vehicle_number(vehNum);
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
        let mut ids: Vec<String> = Vec::new();
        for id in MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR).keys() {
            ids.push(id.clone());
        }
        ids
    }

    /// C++: `int LaneArea::getIDCount()` [static]
    // int
    // LaneArea::getIDCount() {
    //     std::vector<std::string> ids;
    //     return (int)MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR).len() as i32
    }

    /// C++: `std::string LaneArea::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // LaneArea::getParameter(const std::string& detID, const std::string& param) {
    //     return getDetector(detID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(detID: &str, key: &str) -> String {
        // shortcut: MSE2Collector has no ported getParameter, upgrade when MSDetectorFileOutput parameter accessors are ported
        let _ = (detID, key);
        panic!("LaneArea::getParameter: detector parameters not ported")
    }

    /// C++: `const std::pair<std::string, std::string> LaneArea::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(LaneArea)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void LaneArea::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // LaneArea::setParameter(const std::string& detID, const std::string& name, const std::string& value) {
    //     getDetector(detID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(detID: &str, key: &str, value: &str) {
        // shortcut: MSE2Collector has no ported setParameter, upgrade when MSDetectorFileOutput parameter accessors are ported
        let _ = (detID, key, value);
        panic!("LaneArea::setParameter: detector parameters not ported")
    }

    /// C++: `void LaneArea::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_LANEAREA_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void LaneArea::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_LANEAREA_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void LaneArea::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_LANEAREA_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void LaneArea::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_LANEAREA_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults LaneArea::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults LaneArea::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults LaneArea::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults LaneArea::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void LaneArea::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(LaneArea, LANEAREA)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_LANEAREA_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int LaneArea::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_LANEAREA_VARIABLE
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
    pub fn get_tree() -> Ptr<NamedRTree> {
        if MY_TREE.with(|t| t.get()).is_null() {
            let tree = new_named_rtree();
            MY_TREE.with(|t| t.set(tree));
            for id in Self::get_id_list() {
                let mut shape = PositionVector(Vec::new());
                Self::store_shape(&id, &mut shape);
                let b = shape.get_box_boundary();
                let cmin = [b.xmin() as f32, b.ymin() as f32];
                let cmax = [b.xmax() as f32, b.ymax() as f32];
                let det = Self::get_detector(&id);
                tree.m().insert(&cmin, &cmax, &Ptr::from_mut(&mut det.m().base2.base));
            }
        }
        MY_TREE.with(|t| t.get())
    }

    /// C++: `void LaneArea::cleanup()` [static]
    // void
    // LaneArea::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        delete_named_rtree(MY_TREE.with(|t| t.replace(Ptr::null())));
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
    pub fn store_shape(detID: &str, shape: &mut PositionVector) {
        let det = Self::get_detector(detID);
        let lanes = det.m().get_lanes();
        shape.push(lanes.first().unwrap().get_shape_2().position_at_offset(det.get_start_pos(), 0.0));
        shape.push(lanes.last().unwrap().get_shape_2().position_at_offset(det.get_end_pos(), 0.0));
    }

    /// C++: `std::shared_ptr<VariableWrapper> LaneArea::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // LaneArea::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            LAST_STEP_VEHICLE_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_vehicle_number(objID)),
            LAST_STEP_MEAN_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_last_step_mean_speed(objID)),
            LAST_STEP_VEHICLE_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_step_vehicle_i_ds(objID)),
            LAST_STEP_VEHICLE_HALTING_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_halting_number(objID)),
            JAM_LENGTH_VEHICLE => wrapper.m().wrap_int(objID, variable, Self::get_jam_length_vehicle(objID)),
            JAM_LENGTH_METERS => wrapper.m().wrap_double(objID, variable, Self::get_jam_length_meters(objID)),
            LAST_STEP_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_step_occupancy(objID)),
            VAR_POSITION => wrapper.m().wrap_double(objID, variable, Self::get_position(objID)),
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            VAR_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_length(objID)),
            VAR_INTERVAL_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_interval_occupancy(objID)),
            VAR_INTERVAL_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_interval_mean_speed(objID)),
            VAR_INTERVAL_TIMELOSS => wrapper.m().wrap_double(objID, variable, Self::get_interval_mean_time_loss(objID)),
            VAR_INTERVAL_MAX_JAM_LENGTH_METERS => wrapper.m().wrap_double(objID, variable, Self::get_interval_max_jam_length_in_meters(objID)),
            VAR_INTERVAL_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_interval_vehicle_number(objID)),
            VAR_LAST_INTERVAL_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_occupancy(objID)),
            VAR_LAST_INTERVAL_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_mean_speed(objID)),
            VAR_LAST_INTERVAL_TIMELOSS => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_mean_time_loss(objID)),
            VAR_LAST_INTERVAL_MAX_JAM_LENGTH_METERS => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_max_jam_length_in_meters(objID)),
            VAR_LAST_INTERVAL_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_interval_vehicle_number(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte().expect("shortcut: malformed TraCI parameter header");
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte().expect("shortcut: malformed TraCI parameter header");
                let key = paramData.m().read_string().expect("shortcut: malformed TraCI parameter string");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
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
        // shortcut: C++ dynamic_cast<MSE2Collector*> cannot be checked at runtime; a non-MSE2Collector typed detector is cast unchecked, upgrade with a type tag on MSDetectorFileOutput
        let e2 = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR).get(detID).copied();
        match e2 {
            Some(d) => d.cast::<MSE2Collector>(),
            None => {
                let msg = format!("Lane area detector '{}' is not known", detID);
                panic!("{}", msg);
            }
        }
    }

    /// C++: `void LaneArea::LaneArea()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::LaneArea cannot be constructed: the standard constructor is invalidated in C++")
    }

}

