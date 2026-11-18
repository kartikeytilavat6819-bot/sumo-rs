//! Port of libsumo/InductionLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_error;
use utils::common::time::delta_t;
use crate::utils::named_rtree::{delete_named_rtree, new_named_rtree, NamedRTree};

/// C++ class `libsumo::InductionLoop` (libsumo/InductionLoop.h)
pub struct InductionLoop {
}

// shortcut: C++ throws TraCIException(msg) for unknown detectors (here panic!("{}", msg)), and libsumo functions are not Result-based yet; upgrade when they return Result
// C++ static members of InductionLoop (port as module-level statics, see AGENTS.md):
//   static const int InductionLoop::DOMAIN_ID;
//   static SubscriptionResults InductionLoop::mySubscriptionResults;
//   static ContextSubscriptionResults InductionLoop::myContextSubscriptionResults;
//   static NamedRTree * InductionLoop::myTree;

thread_local! {
    /// C++ static `InductionLoop::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `InductionLoop::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
    /// C++ static `InductionLoop::myTree`
    static MY_TREE: std::cell::Cell<Ptr<NamedRTree>> = std::cell::Cell::new(Ptr::null());
}

/// C++ `MSGlobals::gUseMesoSim`
fn use_meso_sim() -> bool {
    crate::microsim::ms_globals::use_meso_sim()
}

impl InductionLoop {
    /// C++: `double InductionLoop::getPosition(const std::string &)` [static]
    // double
    // InductionLoop::getPosition(const std::string& loopID) {
    //     return getDetector(loopID)->getPosition();
    // }
    #[allow(unused_variables)]
    pub fn get_position(loopID: &str) -> f64 {
        Self::get_detector(loopID).get_position()
    }

    /// C++: `std::string InductionLoop::getLaneID(const std::string &)` [static]
    // std::string
    // InductionLoop::getLaneID(const std::string& loopID) {
    //     return getDetector(loopID)->getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(loopID: &str) -> String {
        Self::get_detector(loopID).get_lane().get_id().to_string()
    }

    /// C++: `int InductionLoop::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // InductionLoop::getLastStepVehicleNumber(const std::string& loopID) {
    //     return (int)getDetector(loopID)->getEnteredNumber((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(loopID: &str) -> i32 {
        Self::get_detector(loopID).get_entered_number(delta_t() as i32) as i32
    }

    /// C++: `double InductionLoop::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepMeanSpeed(const std::string& loopID) {
    //     return getDetector(loopID)->getSpeed((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(loopID: &str) -> f64 {
        Self::get_detector(loopID).get_speed(delta_t() as i32)
    }

    /// C++: `std::vector<std::string> InductionLoop::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // InductionLoop::getLastStepVehicleIDs(const std::string& loopID) {
    //     return getDetector(loopID)->getVehicleIDs((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(loopID: &str) -> Vec<String> {
        Self::get_detector(loopID).get_vehicle_i_ds(delta_t() as i32)
    }

    /// C++: `double InductionLoop::getLastStepOccupancy(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepOccupancy(const std::string& loopID) {
    //     return getDetector(loopID)->getOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(loopID: &str) -> f64 {
        Self::get_detector(loopID).get_occupancy()
    }

    /// C++: `double InductionLoop::getLastStepMeanLength(const std::string &)` [static]
    // double
    // InductionLoop::getLastStepMeanLength(const std::string& loopID) {
    //     return getDetector(loopID)->getVehicleLength((int)DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_length(loopID: &str) -> f64 {
        Self::get_detector(loopID).get_vehicle_length(delta_t() as i32)
    }

    /// C++: `double InductionLoop::getTimeSinceDetection(const std::string &)` [static]
    // double
    // InductionLoop::getTimeSinceDetection(const std::string& loopID) {
    //     return getDetector(loopID)->getTimeSinceLastDetection();
    // }
    #[allow(unused_variables)]
    pub fn get_time_since_detection(loopID: &str) -> f64 {
        Self::get_detector(loopID).get_time_since_last_detection()
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
        let det = Self::get_detector(loopID);
        let vd = det.collect_vehicles_on_det(MSNet::get_instance().get_current_time_step() - delta_t(), true, true, false, false);
        let mut tvd: Vec<TraCIVehicleData> = Vec::new();
        for vdi in vd.iter() {
            tvd.push(TraCIVehicleData {
                id: vdi.idM.clone(),
                length: vdi.lengthM,
                entryTime: vdi.entryTimeM,
                leaveTime: vdi.leaveTimeM,
                typeID: vdi.typeIDM.clone(),
            });
        }
        tvd
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
        if use_meso_sim() {
            let det = Self::get_me_detector(loopID);
            let meanData = det.get_mean_data();
            meanData.get_occupancy(MSNet::get_instance().get_current_time_step() - meanData.get_reset_time(), det.get_edge().get_num_lanes())
        } else {
            Self::get_detector(loopID).get_interval_occupancy(false)
        }
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
        if use_meso_sim() {
            let det = Self::get_me_detector(loopID);
            let meanData = det.get_mean_data();
            if meanData.get_samples() != 0.0 {
                meanData.get_travelled_distance() / meanData.get_samples()
            } else {
                let defaultTravelTime = det.get_edge().get_length() / det.get_edge().get_speed_limit();
                meanData.get_lane_length() / defaultTravelTime
            }
        } else {
            Self::get_detector(loopID).get_interval_mean_speed(false)
        }
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
        if use_meso_sim() {
            let det = Self::get_me_detector(loopID);
            let meanData = det.get_mean_data();
            meanData.nVehDeparted + meanData.nVehEntered
        } else {
            Self::get_detector(loopID).get_interval_vehicle_number(false)
        }
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
        if use_meso_sim() {
            write_error("getIntervalVehicleIDs not applicable for meso");
            return Vec::new();
        }
        Self::get_detector(loopID).get_interval_vehicle_i_ds(false)
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
        if use_meso_sim() {
            write_error("getLastIntervalOccupancy not applicable for meso");
            return INVALID_DOUBLE_VALUE;
        }
        Self::get_detector(loopID).get_interval_occupancy(true)
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
        if use_meso_sim() {
            write_error("getLastIntervalMeanSpeed not applicable for meso");
            return INVALID_DOUBLE_VALUE;
        }
        Self::get_detector(loopID).get_interval_mean_speed(true)
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
        if use_meso_sim() {
            write_error("getLastIntervalVehicleNumber not applicable for meso");
            return INVALID_INT_VALUE;
        }
        Self::get_detector(loopID).get_interval_vehicle_number(true)
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
        if use_meso_sim() {
            write_error("getLastIntervalVehicleIDs not applicable for meso");
            return Vec::new();
        }
        Self::get_detector(loopID).get_interval_vehicle_i_ds(true)
    }

    /// C++: `void InductionLoop::overrideTimeSinceDetection(const std::string &, double)` [static]
    // void
    // InductionLoop::overrideTimeSinceDetection(const std::string& loopID, double time) {
    //     getDetector(loopID)->overrideTimeSinceDetection(time);
    // }
    #[allow(unused_variables)]
    pub fn override_time_since_detection(loopID: &str, time: f64) {
        Self::get_detector(loopID).m().override_time_since_detection(time);
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
        let mut ids: Vec<String> = Vec::new();
        for id in MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).keys() {
            ids.push(id.clone());
        }
        ids
    }

    /// C++: `int InductionLoop::getIDCount()` [static]
    // int
    // InductionLoop::getIDCount() {
    //     std::vector<std::string> ids;
    //     return (int)MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).len() as i32
    }

    /// C++: `std::string InductionLoop::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // InductionLoop::getParameter(const std::string& loopID, const std::string& param) {
    //     return getDetector(loopID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(loopID: &str, key: &str) -> String {
        Self::get_detector(loopID).base2.base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> InductionLoop::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(InductionLoop)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void InductionLoop::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // InductionLoop::setParameter(const std::string& loopID, const std::string& name, const std::string& value) {
    //     getDetector(loopID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(loopID: &str, key: &str, value: &str) {
        Self::get_detector(loopID).m().base2.base2.set_parameter(key, value);
    }

    /// C++: `void InductionLoop::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void InductionLoop::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void InductionLoop::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void InductionLoop::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults InductionLoop::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults InductionLoop::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults InductionLoop::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults InductionLoop::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void InductionLoop::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(InductionLoop, INDUCTIONLOOP)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int InductionLoop::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_INDUCTIONLOOP_VARIABLE
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
    pub fn get_tree() -> Ptr<NamedRTree> {
        if MY_TREE.with(|t| t.get()).is_null() {
            let tree = new_named_rtree();
            MY_TREE.with(|t| t.set(tree));
            let net = MSNet::get_instance();
            for (_, i) in net.m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).iter() {
                let il = crate::microsim::output::ms_induct_loop::induct_loop_of(i.cast::<crate::microsim::output::ms_detector_file_output::MSDetectorFileOutput>());
                let p = il.get_lane().get_shape_2().position_at_offset(il.get_position(), 0.0);
                let cmin = [p.x() as f32, p.y() as f32];
                let cmax = [p.x() as f32, p.y() as f32];
                tree.m().insert(&cmin, &cmax, &Ptr::from_mut(&mut il.m().base2.base));
            }
        }
        MY_TREE.with(|t| t.get())
    }

    /// C++: `void InductionLoop::cleanup()` [static]
    // void
    // InductionLoop::cleanup() {
    //     delete myTree;
    //     myTree = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        delete_named_rtree(MY_TREE.with(|t| t.replace(Ptr::null())));
    }

    /// C++: `void InductionLoop::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // InductionLoop::storeShape(const std::string& id, PositionVector& shape) {
    //     MSInductLoop* const il = getDetector(id);
    //     shape.push_back(il->getLane()->getShape().positionAtOffset(il->getPosition()));
    // }
    #[allow(unused_variables)]
    pub fn store_shape(loopID: &str, shape: &mut PositionVector) {
        let il = Self::get_detector(loopID);
        let lane = il.get_lane();
        shape.push(lane.get_shape_2().position_at_offset(il.get_position(), 0.0));
    }

    /// C++: `std::shared_ptr<VariableWrapper> InductionLoop::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // InductionLoop::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_POSITION => wrapper.m().wrap_double(objID, variable, Self::get_position(objID)),
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            LAST_STEP_VEHICLE_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_vehicle_number(objID)),
            LAST_STEP_MEAN_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_last_step_mean_speed(objID)),
            LAST_STEP_VEHICLE_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_step_vehicle_i_ds(objID)),
            LAST_STEP_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_step_occupancy(objID)),
            LAST_STEP_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_last_step_mean_length(objID)),
            LAST_STEP_TIME_SINCE_DETECTION => wrapper.m().wrap_double(objID, variable, Self::get_time_since_detection(objID)),
            VAR_INTERVAL_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_interval_occupancy(objID)),
            VAR_INTERVAL_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_interval_mean_speed(objID)),
            VAR_INTERVAL_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_interval_vehicle_number(objID)),
            VAR_INTERVAL_IDS => wrapper.m().wrap_string_list(objID, variable, &Self::get_interval_vehicle_i_ds(objID)),
            VAR_LAST_INTERVAL_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_occupancy(objID)),
            VAR_LAST_INTERVAL_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_last_interval_mean_speed(objID)),
            VAR_LAST_INTERVAL_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_interval_vehicle_number(objID)),
            VAR_LAST_INTERVAL_IDS => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_interval_vehicle_i_ds(objID)),
            LAST_STEP_VEHICLE_DATA => wrapper.m().wrap_vehicle_data_vector(objID, variable, &Self::get_vehicle_data(objID)),
            VAR_PARAMETER => {
                paramData.m().read_unsigned_byte().unwrap_or_else(|e| std::panic::panic_any(e));
                let key = paramData.m().read_string().unwrap_or_else(|e| std::panic::panic_any(e));
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                paramData.m().read_unsigned_byte().unwrap_or_else(|e| std::panic::panic_any(e));
                let key = paramData.m().read_string().unwrap_or_else(|e| std::panic::panic_any(e));
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
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
        // shortcut: C++ dynamic_cast<MSInductLoop*> cannot be checked at runtime; a non-MSInductLoop typed detector is cast unchecked, upgrade with a type tag on MSDetectorFileOutput
        let il = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).get(loopID).copied();
        match il {
            Some(d) => crate::microsim::output::ms_induct_loop::induct_loop_of(d),
            None => {
                let msg = format!("Induction loop '{}' is not known", loopID);
                panic!("{}", msg);
            }
        }
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
        // shortcut: C++ dynamic_cast<MEInductLoop*> cannot be checked at runtime; a non-MEInductLoop typed detector is cast unchecked, upgrade with a type tag on MSDetectorFileOutput
        let il = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).get(loopID).copied();
        match il {
            Some(d) => d.cast::<MEInductLoop>(),
            None => {
                let msg = format!("Induction loop '{}' is not known", loopID);
                panic!("{}", msg);
            }
        }
    }

    /// C++: `void InductionLoop::InductionLoop()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::InductionLoop cannot be constructed: the standard constructor is invalidated in C++")
    }

}

