//! Port of libsumo/Edge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Edge` (libsumo/Edge.h)
pub struct Edge {
}

// C++ static members of Edge (port as module-level statics, see AGENTS.md):
//   static const int Edge::DOMAIN_ID;
//   static SubscriptionResults Edge::mySubscriptionResults;
//   static ContextSubscriptionResults Edge::myContextSubscriptionResults;

impl Edge {
    /// C++: `double Edge::getAdaptedTraveltime(const std::string &, double)` [static]
    // double
    // Edge::getAdaptedTraveltime(const std::string& edgeID, double time) {
    //     const MSEdge* e = getEdge(edgeID);
    //     double value;
    //     if (!MSNet::getInstance()->getWeightsStorage().retrieveExistingTravelTime(e, time, value)) {
    //         return -1.;
    //     }
    //     return value;
    // }
    #[allow(unused_variables)]
    pub fn get_adapted_traveltime(edgeID: &str, time: f64) -> f64 {
        todo!("port Edge::getAdaptedTraveltime")
    }

    /// C++: `double Edge::getEffort(const std::string &, double)` [static]
    // double
    // Edge::getEffort(const std::string& edgeID, double time) {
    //     const MSEdge* e = getEdge(edgeID);
    //     double value;
    //     if (!MSNet::getInstance()->getWeightsStorage().retrieveExistingEffort(e, time, value)) {
    //         return -1.;
    //     }
    //     return value;
    // }
    #[allow(unused_variables)]
    pub fn get_effort(edgeID: &str, time: f64) -> f64 {
        todo!("port Edge::getEffort")
    }

    /// C++: `double Edge::getTraveltime(const std::string &)` [static]
    // double
    // Edge::getTraveltime(const std::string& edgeID) {
    //     return getEdge(edgeID)->getCurrentTravelTime();
    // }
    #[allow(unused_variables)]
    pub fn get_traveltime(edgeID: &str) -> f64 {
        todo!("port Edge::getTraveltime")
    }

    /// C++: `double Edge::getWaitingTime(const std::string &)` [static]
    // double
    // Edge::getWaitingTime(const std::string& edgeID) {
    //     return getEdge(edgeID)->getWaitingSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(edgeID: &str) -> f64 {
        todo!("port Edge::getWaitingTime")
    }

    /// C++: `const std::vector<std::string> Edge::getLastStepPersonIDs(const std::string &)` [static]
    // const std::vector<std::string>
    // Edge::getLastStepPersonIDs(const std::string& edgeID) {
    //     std::vector<std::string> personIDs;
    //     std::vector<MSTransportable*> persons = getEdge(edgeID)->getSortedPersons(MSNet::getInstance()->getCurrentTimeStep(), true);
    //     personIDs.reserve(persons.size());
    //     for (MSTransportable* p : persons) {
    //         personIDs.push_back(p->getID());
    //     }
    //     return personIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_person_i_ds(edgeID: &str) -> Vec<String> {
        todo!("port Edge::getLastStepPersonIDs")
    }

    /// C++: `const std::vector<std::string> Edge::getLastStepVehicleIDs(const std::string &)` [static]
    // const std::vector<std::string>
    // Edge::getLastStepVehicleIDs(const std::string& edgeID) {
    //     std::vector<std::string> vehIDs;
    //     for (const SUMOVehicle* veh : getEdge(edgeID)->getVehicles()) {
    //         vehIDs.push_back(veh->getID());
    //     }
    //     return vehIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_i_ds(edgeID: &str) -> Vec<String> {
        todo!("port Edge::getLastStepVehicleIDs")
    }

    /// C++: `double Edge::getCO2Emission(const std::string &)` [static]
    // double
    // Edge::getCO2Emission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::CO2>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_co2_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getCO2Emission")
    }

    /// C++: `double Edge::getCOEmission(const std::string &)` [static]
    // double
    // Edge::getCOEmission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::CO>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_co_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getCOEmission")
    }

    /// C++: `double Edge::getHCEmission(const std::string &)` [static]
    // double
    // Edge::getHCEmission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::HC>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_hc_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getHCEmission")
    }

    /// C++: `double Edge::getPMxEmission(const std::string &)` [static]
    // double
    // Edge::getPMxEmission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::PM_X>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_p_mx_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getPMxEmission")
    }

    /// C++: `double Edge::getNOxEmission(const std::string &)` [static]
    // double
    // Edge::getNOxEmission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::NO_X>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_n_ox_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getNOxEmission")
    }

    /// C++: `double Edge::getFuelConsumption(const std::string &)` [static]
    // double
    // Edge::getFuelConsumption(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::FUEL>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_fuel_consumption(edgeID: &str) -> f64 {
        todo!("port Edge::getFuelConsumption")
    }

    /// C++: `double Edge::getNoiseEmission(const std::string &)` [static]
    // double
    // Edge::getNoiseEmission(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += pow(10., (lane->getHarmonoise_NoiseEmissions() / 10.));
    //     }
    //     if (sum != 0) {
    //         return HelpersHarmonoise::sum(sum);
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_noise_emission(edgeID: &str) -> f64 {
        todo!("port Edge::getNoiseEmission")
    }

    /// C++: `double Edge::getElectricityConsumption(const std::string &)` [static]
    // double
    // Edge::getElectricityConsumption(const std::string& edgeID) {
    //     double sum = 0;
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         sum += lane->getEmissions<PollutantsInterface::ELEC>();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_electricity_consumption(edgeID: &str) -> f64 {
        todo!("port Edge::getElectricityConsumption")
    }

    /// C++: `int Edge::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // Edge::getLastStepVehicleNumber(const std::string& edgeID) {
    //     return getEdge(edgeID)->getVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(edgeID: &str) -> i32 {
        todo!("port Edge::getLastStepVehicleNumber")
    }

    /// C++: `double Edge::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // Edge::getLastStepMeanSpeed(const std::string& edgeID) {
    //     return getEdge(edgeID)->getMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(edgeID: &str) -> f64 {
        todo!("port Edge::getLastStepMeanSpeed")
    }

    /// C++: `double Edge::getMeanFriction(const std::string &)` [static]
    // double
    // Edge::getMeanFriction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getMeanFriction();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_friction(edgeID: &str) -> f64 {
        todo!("port Edge::getMeanFriction")
    }

    /// C++: `double Edge::getMaxSpeed(const std::string &)` [static]
    // double
    // Edge::getMaxSpeed(const std::string& edgeID) {
    //     return getEdge(edgeID)->getSpeedLimit();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(edgeID: &str) -> f64 {
        todo!("port Edge::getMaxSpeed")
    }

    /// C++: `double Edge::getLastStepOccupancy(const std::string &)` [static]
    // double
    // Edge::getLastStepOccupancy(const std::string& edgeID) {
    //     return getEdge(edgeID)->getOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(edgeID: &str) -> f64 {
        todo!("port Edge::getLastStepOccupancy")
    }

    /// C++: `int Edge::getLastStepHaltingNumber(const std::string &)` [static]
    // int
    // Edge::getLastStepHaltingNumber(const std::string& edgeID) {
    //     int result = 0;
    //     for (const SUMOVehicle* veh : getEdge(edgeID)->getVehicles()) {
    //         if (veh->getSpeed() < SUMO_const_haltingSpeed) {
    //             result++;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_halting_number(edgeID: &str) -> i32 {
        todo!("port Edge::getLastStepHaltingNumber")
    }

    /// C++: `double Edge::getLastStepLength(const std::string &)` [static]
    // double
    // Edge::getLastStepLength(const std::string& edgeID) {
    //     double lengthSum = 0;
    //     int numVehicles = 0;
    //     for (const SUMOVehicle* veh : getEdge(edgeID)->getVehicles()) {
    //         numVehicles++;
    //         lengthSum += dynamic_cast<const MSBaseVehicle*>(veh)->getVehicleType().getLength();
    //     }
    //     if (numVehicles == 0) {
    //         return 0;
    //     }
    //     return lengthSum / numVehicles;
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_length(edgeID: &str) -> f64 {
        todo!("port Edge::getLastStepLength")
    }

    /// C++: `int Edge::getLaneNumber(const std::string &)` [static]
    // int
    // Edge::getLaneNumber(const std::string& edgeID) {
    //     return (int)getEdge(edgeID)->getLanes().size();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_number(edgeID: &str) -> i32 {
        todo!("port Edge::getLaneNumber")
    }

    /// C++: `std::string Edge::getStreetName(const std::string &)` [static]
    // std::string
    // Edge::getStreetName(const std::string& edgeID) {
    //     return getEdge(edgeID)->getStreetName();
    // }
    #[allow(unused_variables)]
    pub fn get_street_name(edgeID: &str) -> String {
        todo!("port Edge::getStreetName")
    }

    /// C++: `const std::vector<std::string> Edge::getPendingVehicles(const std::string &)` [static]
    // const std::vector<std::string>
    // Edge::getPendingVehicles(const std::string& edgeID) {
    //     getEdge(edgeID); // validate edgeID
    //     std::vector<std::string> vehIDs;
    //     for (const SUMOVehicle* veh : MSNet::getInstance()->getInsertionControl().getPendingVehicles()) {
    //         if (veh->getEdge()->getID() == edgeID) {
    //             vehIDs.push_back(veh->getID());
    //         }
    //     }
    //     return vehIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_pending_vehicles(edgeID: &str) -> Vec<String> {
        todo!("port Edge::getPendingVehicles")
    }

    /// C++: `double Edge::getAngle(const std::string &, double)` [static]
    // double
    // Edge::getAngle(const std::string& edgeID, double relativePosition) {
    //     const std::vector<MSLane*>& lanes = getEdge(edgeID)->getLanes();
    //     return lanes.empty() ? libsumo::INVALID_DOUBLE_VALUE : Lane::getAngle(lanes.front()->getID(), relativePosition);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(edgeID: &str, relativePosition: f64) -> f64 {
        todo!("port Edge::getAngle")
    }

    /// C++: `std::string Edge::getFromJunction(const std::string &)` [static]
    // std::string
    // Edge::getFromJunction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getFromJunction()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_from_junction(edgeID: &str) -> String {
        todo!("port Edge::getFromJunction")
    }

    /// C++: `std::string Edge::getToJunction(const std::string &)` [static]
    // std::string
    // Edge::getToJunction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getToJunction()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_to_junction(edgeID: &str) -> String {
        todo!("port Edge::getToJunction")
    }

    /// C++: `std::string Edge::getBidiEdge(const std::string &)` [static]
    // std::string
    // Edge::getBidiEdge(const std::string& edgeID) {
    //     const MSEdge* bidi = getEdge(edgeID)->getBidiEdge();
    //     return bidi == nullptr ? "" : bidi->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(edgeID: &str) -> String {
        todo!("port Edge::getBidiEdge")
    }

    /// C++: `std::vector<std::string> Edge::getIDList()` [static]
    // std::vector<std::string>
    // Edge::getIDList() {
    //     MSNet::getInstance(); // just to check that we actually have a network
    //     std::vector<std::string> ids;
    //     MSEdge::insertIDs(ids);
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        todo!("port Edge::getIDList")
    }

    /// C++: `int Edge::getIDCount()` [static]
    // int
    // Edge::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port Edge::getIDCount")
    }

    /// C++: `std::string Edge::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Edge::getParameter(const std::string& edgeID, const std::string& param) {
    //     return getEdge(edgeID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Edge::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Edge::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Edge)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Edge::getParameterWithKey")
    }

    /// C++: `void Edge::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Edge::setParameter(const std::string& edgeID, const std::string& name, const std::string& value) {
    //     getEdge(edgeID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Edge::setParameter")
    }

    /// C++: `void Edge::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Edge::subscribe")
    }

    /// C++: `void Edge::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Edge::unsubscribe")
    }

    /// C++: `void Edge::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Edge::subscribeContext")
    }

    /// C++: `void Edge::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Edge::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Edge::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Edge::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Edge::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Edge::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Edge::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Edge::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Edge::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Edge::getContextSubscriptionResults")
    }

    /// C++: `void Edge::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Edge::subscribeParameterWithKey")
    }

    /// C++: `int Edge::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Edge::domainID")
    }

    /// C++: `void Edge::setAllowed(const std::string &, std::string)` [static]
    // void
    // Edge::setAllowed(const std::string& edgeID, std::string allowedClasses) {
    //     setAllowedSVCPermissions(edgeID, parseVehicleClasses(allowedClasses));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed(edgeID: &str, allowedClasses: String) {
        todo!("port Edge::setAllowed")
    }

    /// C++: `void Edge::setAllowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Edge::setAllowed(const std::string& edgeID, std::vector<std::string> allowedClasses) {
    //     setAllowedSVCPermissions(edgeID, parseVehicleClasses(allowedClasses));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed_2(edgeID: &str, allowedClasses: Vec<String>) {
        todo!("port Edge::setAllowed")
    }

    /// C++: `void Edge::setDisallowed(const std::string &, std::string)` [static]
    // void
    // Edge::setDisallowed(const std::string& edgeID, std::string disallowedClasses) {
    //     setAllowedSVCPermissions(edgeID, invertPermissions(parseVehicleClasses(disallowedClasses)));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed(edgeID: &str, disallowedClasses: String) {
        todo!("port Edge::setDisallowed")
    }

    /// C++: `void Edge::setDisallowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Edge::setDisallowed(const std::string& edgeID, std::vector<std::string> disallowedClasses) {
    //     setAllowedSVCPermissions(edgeID, invertPermissions(parseVehicleClasses(disallowedClasses)));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed_2(edgeID: &str, disallowedClasses: Vec<String>) {
        todo!("port Edge::setDisallowed")
    }

    /// C++: `void Edge::adaptTraveltime(const std::string &, double, double, double)` [static]
    // void
    // Edge::adaptTraveltime(const std::string& edgeID, double time, double beginSeconds, double endSeconds) {
    //     MSNet::getInstance()->getWeightsStorage().addTravelTime(getEdge(edgeID), beginSeconds, endSeconds, time);
    // }
    #[allow(unused_variables)]
    pub fn adapt_traveltime(edgeID: &str, time: f64, beginSeconds: f64, endSeconds: f64) {
        todo!("port Edge::adaptTraveltime")
    }

    /// C++: `void Edge::setEffort(const std::string &, double, double, double)` [static]
    // void
    // Edge::setEffort(const std::string& edgeID, double effort, double beginSeconds, double endSeconds) {
    //     MSNet::getInstance()->getWeightsStorage().addEffort(getEdge(edgeID), beginSeconds, endSeconds, effort);
    // }
    #[allow(unused_variables)]
    pub fn set_effort(edgeID: &str, effort: f64, beginSeconds: f64, endSeconds: f64) {
        todo!("port Edge::setEffort")
    }

    /// C++: `void Edge::setMaxSpeed(const std::string &, double)` [static]
    // void
    // Edge::setMaxSpeed(const std::string& edgeID, double speed) {
    //     getEdge(edgeID)->setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(edgeID: &str, speed: f64) {
        todo!("port Edge::setMaxSpeed")
    }

    /// C++: `void Edge::setFriction(const std::string &, double)` [static]
    // void
    // Edge::setFriction(const std::string& edgeID, double friction) {
    //     for (MSLane* lane : getEdge(edgeID)->getLanes()) {
    //         lane->setFrictionCoefficient(friction);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_friction(edgeID: &str, friction: f64) {
        todo!("port Edge::setFriction")
    }

    /// C++: `void Edge::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Edge::storeShape(const std::string& edgeID, PositionVector& shape) {
    //     const MSEdge* const e = getEdge(edgeID);
    //     for (const MSLane* lane : e->getLanes()) {
    //         copy(lane->getShape().begin(), lane->getShape().end(), back_inserter(shape));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn store_shape(edgeID: &str, shape: &mut PositionVector) {
        todo!("port Edge::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Edge::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Edge::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Edge::makeWrapper")
    }

    /// C++: `bool Edge::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Edge::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_EDGE_TRAVELTIME:
    //             return wrapper->wrapDouble(objID, variable, getAdaptedTraveltime(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_EDGE_EFFORT:
    //             return wrapper->wrapDouble(objID, variable, getEffort(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_CURRENT_TRAVELTIME:
    //             return wrapper->wrapDouble(objID, variable, getTraveltime(objID));
    //         case VAR_WAITING_TIME:
    //             return wrapper->wrapDouble(objID, variable, getWaitingTime(objID));
    //         case LAST_STEP_PERSON_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLastStepPersonIDs(objID));
    //         case LAST_STEP_VEHICLE_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLastStepVehicleIDs(objID));
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
    //         case VAR_MAXSPEED:
    //             return wrapper->wrapDouble(objID, variable, getMaxSpeed(objID));
    //         case VAR_FRICTION:
    //             return wrapper->wrapDouble(objID, variable, getMeanFriction(objID));
    //         case LAST_STEP_OCCUPANCY:
    //             return wrapper->wrapDouble(objID, variable, getLastStepOccupancy(objID));
    //         case LAST_STEP_VEHICLE_HALTING_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLastStepHaltingNumber(objID));
    //         case LAST_STEP_LENGTH:
    //             return wrapper->wrapDouble(objID, variable, getLastStepLength(objID));
    //         case VAR_LANE_INDEX:
    //             return wrapper->wrapInt(objID, variable, getLaneNumber(objID));
    //         case VAR_NAME:
    //             return wrapper->wrapString(objID, variable, getStreetName(objID));
    //         case VAR_PENDING_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getPendingVehicles(objID));
    //         case VAR_ANGLE:
    //             return wrapper->wrapDouble(objID, variable, getAngle(objID, StoHelp::readTypedDouble(*paramData)));
    //         case FROM_JUNCTION:
    //             return wrapper->wrapString(objID, variable, getFromJunction(objID));
    //         case TO_JUNCTION:
    //             return wrapper->wrapString(objID, variable, getToJunction(objID));
    //         case VAR_BIDI:
    //             return wrapper->wrapString(objID, variable, getBidiEdge(objID));
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
        todo!("port Edge::handleVariable")
    }

    /// C++: `void Edge::setAllowedSVCPermissions(const std::string &, long long)` [static, private]
    // void
    // Edge::setAllowedSVCPermissions(const std::string& edgeID, long long int permissions) {
    //     MSEdge* e = getEdge(edgeID);
    //     for (MSLane* lane : e->getLanes()) {
    //         lane->setPermissions(permissions, MSLane::CHANGE_PERMISSIONS_PERMANENT);
    //     }
    //     e->rebuildAllowedLanes(false, true);
    // }
    #[allow(unused_variables)]
    pub fn set_allowed_svc_permissions(edgeID: &str, permissions: i64) {
        todo!("port Edge::setAllowedSVCPermissions")
    }

    /// C++: `MSEdge * Edge::getEdge(const std::string &)` [static, private]
    // MSEdge*
    // Edge::getEdge(const std::string& edgeID) {
    //     MSEdge* e = MSEdge::dictionary(edgeID);
    //     if (e == nullptr) {
    //         throw TraCIException("Edge '" + edgeID + "' is not known");
    //     }
    //     return e;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(edgeID: &str) -> Ptr<MSEdge> {
        todo!("port Edge::getEdge")
    }

    /// C++: `void Edge::Edge()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Edge::Edge")
    }

}

