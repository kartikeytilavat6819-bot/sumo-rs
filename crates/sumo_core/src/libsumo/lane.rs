//! Port of libsumo/Lane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Lane` (libsumo/Lane.h)
pub struct Lane {
}

// C++ static members of Lane (port as module-level statics, see AGENTS.md):
//   static const int Lane::DOMAIN_ID;
//   static SubscriptionResults Lane::mySubscriptionResults;
//   static ContextSubscriptionResults Lane::myContextSubscriptionResults;

impl Lane {
    /// C++: `int Lane::getLinkNumber(const std::string &)` [static]
    // int
    // Lane::getLinkNumber(const std::string& laneID) {
    //     return (int)getLane(laneID)->getLinkCont().size();
    // }
    #[allow(unused_variables)]
    pub fn get_link_number(laneID: &str) -> i32 {
        todo!("port Lane::getLinkNumber")
    }

    /// C++: `std::string Lane::getEdgeID(const std::string &)` [static]
    // std::string
    // Lane::getEdgeID(const std::string& laneID) {
    //     return getLane(laneID)->getEdge().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(laneID: &str) -> String {
        todo!("port Lane::getEdgeID")
    }

    /// C++: `double Lane::getLength(const std::string &)` [static]
    // double
    // Lane::getLength(const std::string& laneID) {
    //     return getLane(laneID)->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(laneID: &str) -> f64 {
        todo!("port Lane::getLength")
    }

    /// C++: `double Lane::getMaxSpeed(const std::string &)` [static]
    // double
    // Lane::getMaxSpeed(const std::string& laneID) {
    //     return getLane(laneID)->getSpeedLimit();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(laneID: &str) -> f64 {
        todo!("port Lane::getMaxSpeed")
    }

    /// C++: `double Lane::getFriction(const std::string &)` [static]
    // double
    // Lane::getFriction(const std::string& laneID) {
    //     return getLane(laneID)->getFrictionCoefficient();
    // }
    #[allow(unused_variables)]
    pub fn get_friction(laneID: &str) -> f64 {
        todo!("port Lane::getFriction")
    }

    /// C++: `std::vector<std::string> Lane::getAllowed(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getAllowed(const std::string& laneID) {
    //     return getVehicleClassNamesList(getLane(laneID)->getPermissions());
    // }
    #[allow(unused_variables)]
    pub fn get_allowed(laneID: &str) -> Vec<String> {
        todo!("port Lane::getAllowed")
    }

    /// C++: `std::vector<std::string> Lane::getDisallowed(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getDisallowed(const std::string& laneID) {
    //     return getVehicleClassNamesList(invertPermissions((getLane(laneID)->getPermissions()))); // negation yields disallowed
    // }
    #[allow(unused_variables)]
    pub fn get_disallowed(laneID: &str) -> Vec<String> {
        todo!("port Lane::getDisallowed")
    }

    /// C++: `std::vector<std::string> Lane::getChangePermissions(const std::string &, const int)` [static]
    // std::vector<std::string>
    // Lane::getChangePermissions(const std::string& laneID, const int direction) {
    //     if (direction == libsumo::LANECHANGE_LEFT) {
    //         return getVehicleClassNamesList(getLane(laneID)->getChangeLeft());
    //     } else if (direction == libsumo::LANECHANGE_RIGHT) {
    //         return getVehicleClassNamesList(getLane(laneID)->getChangeRight());
    //     } else {
    //         throw TraCIException("Invalid direction for change permission (must be " + toString(libsumo::LANECHANGE_LEFT) + " or " + toString(libsumo::LANECHANGE_RIGHT));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_change_permissions(laneID: &str, direction: i32) -> Vec<String> {
        todo!("port Lane::getChangePermissions")
    }

    /// C++: `std::vector<libsumo::TraCIConnection> Lane::getLinks(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_links(laneID: &str) -> Vec<TraCIConnection> {
        todo!("port Lane::getLinks")
    }

    /// C++: `libsumo::TraCIPositionVector Lane::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(laneID: &str) -> TraCIPositionVector {
        todo!("port Lane::getShape")
    }

    /// C++: `double Lane::getWidth(const std::string &)` [static]
    // double
    // Lane::getWidth(const std::string& laneID) {
    //     return getLane(laneID)->getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(laneID: &str) -> f64 {
        todo!("port Lane::getWidth")
    }

    /// C++: `double Lane::getCO2Emission(const std::string &)` [static]
    // double
    // Lane::getCO2Emission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::CO2>();
    // }
    #[allow(unused_variables)]
    pub fn get_co2_emission(laneID: &str) -> f64 {
        todo!("port Lane::getCO2Emission")
    }

    /// C++: `double Lane::getCOEmission(const std::string &)` [static]
    // double
    // Lane::getCOEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::CO>();
    // }
    #[allow(unused_variables)]
    pub fn get_co_emission(laneID: &str) -> f64 {
        todo!("port Lane::getCOEmission")
    }

    /// C++: `double Lane::getHCEmission(const std::string &)` [static]
    // double
    // Lane::getHCEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::HC>();
    // }
    #[allow(unused_variables)]
    pub fn get_hc_emission(laneID: &str) -> f64 {
        todo!("port Lane::getHCEmission")
    }

    /// C++: `double Lane::getPMxEmission(const std::string &)` [static]
    // double
    // Lane::getPMxEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::PM_X>();
    // }
    #[allow(unused_variables)]
    pub fn get_p_mx_emission(laneID: &str) -> f64 {
        todo!("port Lane::getPMxEmission")
    }

    /// C++: `double Lane::getNOxEmission(const std::string &)` [static]
    // double
    // Lane::getNOxEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::NO_X>();
    // }
    #[allow(unused_variables)]
    pub fn get_n_ox_emission(laneID: &str) -> f64 {
        todo!("port Lane::getNOxEmission")
    }

    /// C++: `double Lane::getFuelConsumption(const std::string &)` [static]
    // double
    // Lane::getFuelConsumption(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::FUEL>();
    // }
    #[allow(unused_variables)]
    pub fn get_fuel_consumption(laneID: &str) -> f64 {
        todo!("port Lane::getFuelConsumption")
    }

    /// C++: `double Lane::getNoiseEmission(const std::string &)` [static]
    // double
    // Lane::getNoiseEmission(const std::string& laneID) {
    //     return getLane(laneID)->getHarmonoise_NoiseEmissions();
    // }
    #[allow(unused_variables)]
    pub fn get_noise_emission(laneID: &str) -> f64 {
        todo!("port Lane::getNoiseEmission")
    }

    /// C++: `double Lane::getElectricityConsumption(const std::string &)` [static]
    // double
    // Lane::getElectricityConsumption(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::ELEC>();
    // }
    #[allow(unused_variables)]
    pub fn get_electricity_consumption(laneID: &str) -> f64 {
        todo!("port Lane::getElectricityConsumption")
    }

    /// C++: `double Lane::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // Lane::getLastStepMeanSpeed(const std::string& laneID) {
    //     return getLane(laneID)->getMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(laneID: &str) -> f64 {
        todo!("port Lane::getLastStepMeanSpeed")
    }

    /// C++: `double Lane::getLastStepOccupancy(const std::string &)` [static]
    // double
    // Lane::getLastStepOccupancy(const std::string& laneID) {
    //     return getLane(laneID)->getNettoOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(laneID: &str) -> f64 {
        todo!("port Lane::getLastStepOccupancy")
    }

    /// C++: `double Lane::getLastStepLength(const std::string &)` [static]
    // double
    // Lane::getLastStepLength(const std::string& laneID) {
    //     const MSLane* lane = getLane(laneID);
    //     double length = 0;
    //     const MSLane::VehCont& vehs = lane->getVehiclesSecure();
    //     for (MSLane::VehCont::const_iterator j = vehs.begin(); j != vehs.end(); ++j) {
    //         length += (*j)->getVehicleType().getLength();
    //     }
    //     if (vehs.size() > 0) {
    //         length = length / (double)vehs.size();
    //     }
    //     lane->releaseVehicles();
    //     return length;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_length(laneID: &str) -> f64 {
        todo!("port Lane::getLastStepLength")
    }

    /// C++: `double Lane::getWaitingTime(const std::string &)` [static]
    // double
    // Lane::getWaitingTime(const std::string& laneID) {
    //     return getLane(laneID)->getWaitingSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(laneID: &str) -> f64 {
        todo!("port Lane::getWaitingTime")
    }

    /// C++: `double Lane::getTraveltime(const std::string &)` [static]
    // double
    // Lane::getTraveltime(const std::string& laneID) {
    //     const MSLane* lane = getLane(laneID);
    //     double meanSpeed = lane->getMeanSpeed();
    //     if (meanSpeed != 0) {
    //         return lane->getLength() / meanSpeed;
    //     } else {
    //         return 1000000.;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_traveltime(laneID: &str) -> f64 {
        todo!("port Lane::getTraveltime")
    }

    /// C++: `int Lane::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // Lane::getLastStepVehicleNumber(const std::string& laneID) {
    //     return (int)getLane(laneID)->getVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(laneID: &str) -> i32 {
        todo!("port Lane::getLastStepVehicleNumber")
    }

    /// C++: `int Lane::getLastStepHaltingNumber(const std::string &)` [static]
    // int
    // Lane::getLastStepHaltingNumber(const std::string& laneID) {
    //     const MSLane* lane = getLane(laneID);
    //     int halting = 0;
    //     const MSLane::VehCont& vehs = lane->getVehiclesSecure();
    //     for (MSLane::VehCont::const_iterator j = vehs.begin(); j != vehs.end(); ++j) {
    //         if ((*j)->getSpeed() < SUMO_const_haltingSpeed) {
    //             ++halting;
    //         }
    //     }
    //     lane->releaseVehicles();
    //     return halting;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_halting_number(laneID: &str) -> i32 {
        todo!("port Lane::getLastStepHaltingNumber")
    }

    /// C++: `std::vector<std::string> Lane::getLastStepVehicleIDs(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getLastStepVehicleIDs(const std::string& laneID) {
    //     const MSLane* lane = getLane(laneID);
    //     std::vector<std::string> vehIDs;
    //     const MSLane::VehCont& vehs = lane->getVehiclesSecure();
    //     for (MSLane::VehCont::const_iterator j = vehs.begin(); j != vehs.end(); ++j) {
    //         vehIDs.push_back((*j)->getID());
    //     }
    //     lane->releaseVehicles();
    //     return vehIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(laneID: &str) -> Vec<String> {
        todo!("port Lane::getLastStepVehicleIDs")
    }

    /// C++: `std::vector<std::string> Lane::getFoes(const std::string &, const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getFoes(const std::string& laneID, const std::string& toLaneID) {
    //     if (toLaneID == "") {
    //         return getInternalFoes(laneID);
    //     }
    //     std::vector<std::string> foeIDs;
    //     const MSLink* const link = getLane(laneID)->getLinkTo(getLane(toLaneID));
    //     if (link == nullptr) {
    //         throw TraCIException("No connection from lane '" + laneID + "' to lane '" + toLaneID + "'");
    //     }
    //     for (const MSLink* foe : link->getFoeLinks()) {
    //         foeIDs.push_back(foe->getLaneBefore()->getID());
    //     }
    //     return foeIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_foes(laneID: &str, toLaneID: &str) -> Vec<String> {
        todo!("port Lane::getFoes")
    }

    /// C++: `std::vector<std::string> Lane::getInternalFoes(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getInternalFoes(const std::string& laneID) {
    //     const MSLane* lane = getLane(laneID);
    //     const std::vector<const MSLane*>* foeLanes;
    //     std::vector<const MSLane*>::const_iterator it;
    //     std::vector<std::string> foeIDs;
    //
    //     if ((lane->isInternal() || lane->isCrossing()) && lane->getLinkCont().size() > 0) {
    //         MSLink* link = lane->getLinkCont().front();
    //         foeLanes = &link->getFoeLanes();
    //
    //         for (it = foeLanes->begin(); foeLanes->end() != it; ++it) {
    //             foeIDs.push_back((*it)->getID());
    //         }
    //     }
    //     return foeIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_foes(laneID: &str) -> Vec<String> {
        todo!("port Lane::getInternalFoes")
    }

    /// C++: `const std::vector<std::string> Lane::getPendingVehicles(const std::string &)` [static]
    // const std::vector<std::string>
    // Lane::getPendingVehicles(const std::string& laneID) {
    //     MSLane* const l = getLane(laneID); // validate laneID
    //     std::vector<std::string> vehIDs;
    //     for (const SUMOVehicle* veh : MSNet::getInstance()->getInsertionControl().getPendingVehicles()) {
    //         if (veh->getLane() == l) {
    //             vehIDs.push_back(veh->getID());
    //         }
    //     }
    //     return vehIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_pending_vehicles(laneID: &str) -> Vec<String> {
        todo!("port Lane::getPendingVehicles")
    }

    /// C++: `double Lane::getAngle(const std::string &, double)` [static]
    // double
    // Lane::getAngle(const std::string& laneID, double relativePosition) {
    //     double angle;
    //     MSLane* lane = getLane(laneID);
    //     if (relativePosition == libsumo::INVALID_DOUBLE_VALUE) {
    //         Position start = lane->getShape().front();
    //         Position end = lane->getShape().back();
    //         angle = start.angleTo2D(end);
    //     } else {
    //         angle = lane->getShape().rotationAtOffset(lane->interpolateLanePosToGeometryPos(relativePosition));
    //     }
    //
    //     return GeomHelper::naviDegree(angle);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(laneID: &str, relativePosition: f64) -> f64 {
        todo!("port Lane::getAngle")
    }

    /// C++: `std::string Lane::getBidiLane(const std::string &)` [static]
    // std::string
    // Lane::getBidiLane(const std::string& laneID) {
    //     const MSLane* bidi = getLane(laneID)->getBidiLane();
    //     return bidi == nullptr ? "" : bidi->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_bidi_lane(laneID: &str) -> String {
        todo!("port Lane::getBidiLane")
    }

    /// C++: `std::vector<std::string> Lane::getIDList()` [static]
    // std::vector<std::string>
    // Lane::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     MSLane::insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port Lane::getIDList")
    }

    /// C++: `int Lane::getIDCount()` [static]
    // int
    // Lane::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port Lane::getIDCount")
    }

    /// C++: `std::string Lane::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Lane::getParameter(const std::string& laneID, const std::string& param) {
    //     return getLane(laneID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Lane::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Lane::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Lane)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Lane::getParameterWithKey")
    }

    /// C++: `void Lane::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Lane::setParameter(const std::string& laneID, const std::string& key, const std::string& value) {
    //     getLane(laneID)->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Lane::setParameter")
    }

    /// C++: `void Lane::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Lane::subscribe")
    }

    /// C++: `void Lane::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Lane::unsubscribe")
    }

    /// C++: `void Lane::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Lane::subscribeContext")
    }

    /// C++: `void Lane::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Lane::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Lane::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Lane::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Lane::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Lane::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Lane::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Lane::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Lane::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Lane::getContextSubscriptionResults")
    }

    /// C++: `void Lane::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Lane::subscribeParameterWithKey")
    }

    /// C++: `int Lane::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Lane::domainID")
    }

    /// C++: `void Lane::setAllowed(const std::string &, std::string)` [static]
    // void
    // Lane::setAllowed(const std::string& laneID, std::string allowedClass) {
    //     setAllowed(laneID, std::vector<std::string>({allowedClass}));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed(laneID: &str, allowedClasses: String) {
        todo!("port Lane::setAllowed")
    }

    /// C++: `void Lane::setAllowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Lane::setAllowed(const std::string& laneID, std::vector<std::string> allowedClasses) {
    //     MSLane* const l = getLane(laneID);
    //     l->setPermissions(parseVehicleClasses(allowedClasses), MSLane::CHANGE_PERMISSIONS_PERMANENT);
    //     l->getEdge().rebuildAllowedLanes(false, true);
    // }
    #[allow(unused_variables)]
    pub fn set_allowed_2(laneID: &str, allowedClasses: Vec<String>) {
        todo!("port Lane::setAllowed")
    }

    /// C++: `void Lane::setDisallowed(const std::string &, std::string)` [static]
    // void
    // Lane::setDisallowed(const std::string& laneID, std::string disallowedClasses) {
    //     setDisallowed(laneID, std::vector<std::string>({disallowedClasses}));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed(laneID: &str, disallowedClasses: String) {
        todo!("port Lane::setDisallowed")
    }

    /// C++: `void Lane::setDisallowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Lane::setDisallowed(const std::string& laneID, std::vector<std::string> disallowedClasses) {
    //     MSLane* const l = getLane(laneID);
    //     l->setPermissions(invertPermissions(parseVehicleClasses(disallowedClasses)), MSLane::CHANGE_PERMISSIONS_PERMANENT); // negation yields allowed
    //     l->getEdge().rebuildAllowedLanes(false, true);
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed_2(laneID: &str, disallowedClasses: Vec<String>) {
        todo!("port Lane::setDisallowed")
    }

    /// C++: `void Lane::setChangePermissions(const std::string &, std::vector<std::string>, const int)` [static]
    // void
    // Lane::setChangePermissions(const std::string& laneID, std::vector<std::string> allowedClasses, const int direction) {
    //     MSLane* const l = getLane(laneID);
    //     if (direction == libsumo::LANECHANGE_LEFT) {
    //         l->setChangeLeft(parseVehicleClasses(allowedClasses));
    //     } else if (direction == libsumo::LANECHANGE_RIGHT) {
    //         l->setChangeRight(parseVehicleClasses(allowedClasses));
    //     } else {
    //         throw TraCIException("Invalid direction for change permission (must be " + toString(libsumo::LANECHANGE_LEFT) + " or " + toString(libsumo::LANECHANGE_RIGHT));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_change_permissions(laneID: &str, allowedClasses: Vec<String>, direction: i32) {
        todo!("port Lane::setChangePermissions")
    }

    /// C++: `void Lane::setMaxSpeed(const std::string &, double)` [static]
    // void
    // Lane::setMaxSpeed(const std::string& laneID, double speed) {
    //     getLane(laneID)->setMaxSpeed(speed, false, true);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(laneID: &str, speed: f64) {
        todo!("port Lane::setMaxSpeed")
    }

    /// C++: `void Lane::setLength(const std::string &, double)` [static]
    // void
    // Lane::setLength(const std::string& laneID, double length) {
    //     getLane(laneID)->setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(laneID: &str, length: f64) {
        todo!("port Lane::setLength")
    }

    /// C++: `void Lane::setFriction(const std::string &, double)` [static]
    // void
    // Lane::setFriction(const std::string& laneID, double friction) {
    //     getLane(laneID)->setFrictionCoefficient(friction);
    // }
    #[allow(unused_variables)]
    pub fn set_friction(laneID: &str, friction: f64) {
        todo!("port Lane::setFriction")
    }

    /// C++: `void Lane::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Lane::storeShape(const std::string& id, PositionVector& shape) {
    //     shape = getLane(id)->getShape();
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port Lane::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Lane::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Lane::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Lane::makeWrapper")
    }

    /// C++: `bool Lane::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Lane::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case LANE_LINK_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLinkNumber(objID));
    //         case LANE_EDGE_ID:
    //             return wrapper->wrapString(objID, variable, getEdgeID(objID));
    //         case VAR_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLength(objID));
    //         case VAR_MAXSPEED:
    //             return wrapper->wrapDouble(objID, variable, getMaxSpeed(objID));
    //         case VAR_FRICTION:
    //             return wrapper->wrapDouble(objID, variable, getFriction(objID));
    //         case LANE_ALLOWED:
    //             return wrapper->wrapStringList(objID, variable, getAllowed(objID));
    //         case LANE_DISALLOWED:
    //             return wrapper->wrapStringList(objID, variable, getDisallowed(objID));
    //         case LANE_CHANGES:
    //             return wrapper->wrapStringList(objID, variable, getChangePermissions(objID, StoHelp::readTypedByte(*paramData)));
    //         case VAR_CO2EMISSION:
    //             return wrapper->wrapDouble(objID, variable, getCO2Emission(objID));
    //         case VAR_COEMISSION:
    //             return wrapper->wrapDouble(objID, variable, getCOEmission(objID));
    //         case VAR_HCEMISSION:
    //             return wrapper->wrapDouble(objID, variable, getHCEmission(objID));
    //         case VAR_PMXEMISSION:
    //             return wrapper->wrapDouble(objID, variable, getPMxEmission(objID));
    //         case VAR_NOXEMISSION:
    //             return wrapper->wrapDouble(objID, variable, getNOxEmission(objID));
    //         case VAR_FUELCONSUMPTION:
    //             return wrapper->wrapDouble(objID, variable, getFuelConsumption(objID));
    //         case VAR_NOISEEMISSION:
    //             return wrapper->wrapDouble(objID, variable, getNoiseEmission(objID));
    //         case VAR_ELECTRICITYCONSUMPTION:
    //             return wrapper->wrapDouble(objID, variable, getElectricityConsumption(objID));
    //         case LAST_STEP_VEHICLE_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepVehicleNumber(objID));
    //         case LAST_STEP_MEAN_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getLastStepMeanSpeed(objID));
    //         case LAST_STEP_VEHICLE_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLastStepVehicleIDs(objID));
    //         case LAST_STEP_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastStepOccupancy(objID));
    //         case LAST_STEP_VEHICLE_HALTING_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepHaltingNumber(objID));
    //         case LAST_STEP_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLastStepLength(objID));
    //         case VAR_WAITING_TIME:
    //             return wrapper->wrapDouble(objID, variable, getWaitingTime(objID));
    //         case VAR_CURRENT_TRAVELTIME:
    //             return wrapper->wrapDouble(objID, variable, getTraveltime(objID));
    //         case VAR_WIDTH:
    //             return wrapper->wrapDouble(objID, variable, getWidth(objID));
    //         case VAR_SHAPE:
    //             return wrapper->wrapPositionVector(objID, variable, getShape(objID));
    //         case VAR_PENDING_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getPendingVehicles(objID));
    //         case VAR_ANGLE:
    //             return wrapper->wrapDouble(objID, variable, getAngle(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_BIDI:
    //             return wrapper->wrapString(objID, variable, getBidiLane(objID));
    //         case VAR_FOES:
    //             return wrapper->wrapStringList(objID, variable, getFoes(objID, StoHelp::readTypedString(*paramData)));
    //         case LANE_LINKS:
    //             return wrapper->wrapConnectionVector(objID, variable, getLinks(objID));
    //         case VAR_PARAMETER:
    //             return wrapper->wrapString(objID, variable, getParameter(objID, StoHelp::readTypedString(*paramData)));
    //         case VAR_PARAMETER_WITH_KEY:
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, StoHelp::readTypedString(*paramData)));
    //         default:
    //             return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        todo!("port Lane::handleVariable")
    }

    /// C++: `MSLane * Lane::getLane(const std::string &)` [static, private]
    // MSLane*
    // Lane::getLane(const std::string& id) {
    //     MSLane* const lane = MSLane::dictionary(id);
    //     if (lane == nullptr) {
    //         throw TraCIException("Lane '" + id + "' is not known");
    //     }
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn get_lane(id: &str) -> Ptr<MSLane> {
        todo!("port Lane::getLane")
    }

    /// C++: `void Lane::Lane()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Lane::Lane")
    }

}

