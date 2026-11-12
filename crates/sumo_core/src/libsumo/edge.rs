//! Port of libsumo/Edge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{NUMERICAL_EPS, SUMO_CONST_HALTING_SPEED};
use utils::common::sumo_vehicle_class::{invert_permissions, parse_vehicle_classes, parse_vehicle_classes_vec};
// shortcut: C++ `throw TraCIException(msg)` is mapped to `panic!("{}", msg)` (unwinds where C++ throws), upgrade when libsumo has a typed TraCI error that callers can catch

/// C++ class `libsumo::Edge` (libsumo/Edge.h)
pub struct Edge {
}

// C++ static members of Edge (port as module-level statics, see AGENTS.md):
//   static const int Edge::DOMAIN_ID;
//   static SubscriptionResults Edge::mySubscriptionResults;
//   static ContextSubscriptionResults Edge::myContextSubscriptionResults;

thread_local! {
    /// C++ static `Edge::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Edge::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

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
        let e = Self::get_edge(edgeID);
        let mut value: f64 = 0.0;
        if !MSNet::get_instance().m().get_weights_storage().retrieve_existing_travel_time(e, time, &mut value) {
            return -1.0;
        }
        value
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
        let e = Self::get_edge(edgeID);
        let mut value: f64 = 0.0;
        if !MSNet::get_instance().m().get_weights_storage().retrieve_existing_effort(e, time, &mut value) {
            return -1.0;
        }
        value
    }

    /// C++: `double Edge::getTraveltime(const std::string &)` [static]
    // double
    // Edge::getTraveltime(const std::string& edgeID) {
    //     return getEdge(edgeID)->getCurrentTravelTime();
    // }
    #[allow(unused_variables)]
    pub fn get_traveltime(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_current_travel_time(NUMERICAL_EPS)
    }

    /// C++: `double Edge::getWaitingTime(const std::string &)` [static]
    // double
    // Edge::getWaitingTime(const std::string& edgeID) {
    //     return getEdge(edgeID)->getWaitingSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_waiting_seconds()
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
        let mut personIDs: Vec<String> = Vec::new();
        let persons = Self::get_edge(edgeID).get_sorted_persons(MSNet::get_instance().get_current_time_step(), true);
        personIDs.reserve(persons.len());
        for p in persons.iter() {
            personIDs.push(p.get_id().to_string());
        }
        personIDs
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
        let mut vehIDs: Vec<String> = Vec::new();
        for veh in Self::get_edge(edgeID).get_vehicles().iter() {
            vehIDs.push(veh.get_id().to_string());
        }
        vehIDs
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::CO2>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getCO2Emission needs MSLane::getEmissions<CO2>")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::CO>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getCOEmission needs MSLane::getEmissions<CO>")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::HC>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getHCEmission needs MSLane::getEmissions<HC>")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::PM_X>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getPMxEmission needs MSLane::getEmissions<PM_X>")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::NO_X>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getNOxEmission needs MSLane::getEmissions<NO_X>")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::FUEL>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getFuelConsumption needs MSLane::getEmissions<FUEL>")
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
        // shortcut: needs MSLane::getHarmonoise_NoiseEmissions, which is still unported in ms_lane.rs, upgrade when it is ported
        panic!("shortcut: Edge::getNoiseEmission needs MSLane::getHarmonoise_NoiseEmissions")
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
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::ELEC>, a template that the Rust MSLane::get_emissions does not carry (and is still unported), upgrade with the typed emission port
        panic!("shortcut: Edge::getElectricityConsumption needs MSLane::getEmissions<ELEC>")
    }

    /// C++: `int Edge::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // Edge::getLastStepVehicleNumber(const std::string& edgeID) {
    //     return getEdge(edgeID)->getVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(edgeID: &str) -> i32 {
        Self::get_edge(edgeID).get_vehicle_number()
    }

    /// C++: `double Edge::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // Edge::getLastStepMeanSpeed(const std::string& edgeID) {
    //     return getEdge(edgeID)->getMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_mean_speed()
    }

    /// C++: `double Edge::getMeanFriction(const std::string &)` [static]
    // double
    // Edge::getMeanFriction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getMeanFriction();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_friction(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_mean_friction()
    }

    /// C++: `double Edge::getMaxSpeed(const std::string &)` [static]
    // double
    // Edge::getMaxSpeed(const std::string& edgeID) {
    //     return getEdge(edgeID)->getSpeedLimit();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_speed_limit()
    }

    /// C++: `double Edge::getLastStepOccupancy(const std::string &)` [static]
    // double
    // Edge::getLastStepOccupancy(const std::string& edgeID) {
    //     return getEdge(edgeID)->getOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(edgeID: &str) -> f64 {
        Self::get_edge(edgeID).get_occupancy()
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
        let mut result: i32 = 0;
        for veh in Self::get_edge(edgeID).get_vehicles().iter() {
            if veh.get_speed() < SUMO_CONST_HALTING_SPEED {
                result += 1;
            }
        }
        result
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
        // shortcut: C++ dynamic_cast<const MSBaseVehicle*> from SUMOVehicle needs #[repr(C)] on MSBaseVehicle in microsim/ms_base_vehicle.rs, upgrade when added
        panic!("shortcut: Edge::getLastStepLength needs dynamic_cast to MSBaseVehicle")
    }

    /// C++: `int Edge::getLaneNumber(const std::string &)` [static]
    // int
    // Edge::getLaneNumber(const std::string& edgeID) {
    //     return (int)getEdge(edgeID)->getLanes().size();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_number(edgeID: &str) -> i32 {
        Self::get_edge(edgeID).get_lanes().len() as i32
    }

    /// C++: `std::string Edge::getStreetName(const std::string &)` [static]
    // std::string
    // Edge::getStreetName(const std::string& edgeID) {
    //     return getEdge(edgeID)->getStreetName();
    // }
    #[allow(unused_variables)]
    pub fn get_street_name(edgeID: &str) -> String {
        Self::get_edge(edgeID).get_street_name().to_string()
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
        Self::get_edge(edgeID); // validate edgeID
        let mut vehIDs: Vec<String> = Vec::new();
        for veh in MSNet::get_instance().m().get_insertion_control().get_pending_vehicles().iter() {
            if veh.get_edge().get_id() == edgeID {
                vehIDs.push(veh.get_id().to_string());
            }
        }
        vehIDs
    }

    /// C++: `double Edge::getAngle(const std::string &, double)` [static]
    // double
    // Edge::getAngle(const std::string& edgeID, double relativePosition) {
    //     const std::vector<MSLane*>& lanes = getEdge(edgeID)->getLanes();
    //     return lanes.empty() ? libsumo::INVALID_DOUBLE_VALUE : Lane::getAngle(lanes.front()->getID(), relativePosition);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(edgeID: &str, relativePosition: f64) -> f64 {
        let e = Self::get_edge(edgeID);
        let lanes = e.get_lanes();
        if lanes.is_empty() { INVALID_DOUBLE_VALUE } else { Lane::get_angle(lanes[0].get_id(), relativePosition) }
    }

    /// C++: `std::string Edge::getFromJunction(const std::string &)` [static]
    // std::string
    // Edge::getFromJunction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getFromJunction()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_from_junction(edgeID: &str) -> String {
        Self::get_edge(edgeID).get_from_junction().get_id().to_string()
    }

    /// C++: `std::string Edge::getToJunction(const std::string &)` [static]
    // std::string
    // Edge::getToJunction(const std::string& edgeID) {
    //     return getEdge(edgeID)->getToJunction()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_to_junction(edgeID: &str) -> String {
        Self::get_edge(edgeID).get_to_junction().get_id().to_string()
    }

    /// C++: `std::string Edge::getBidiEdge(const std::string &)` [static]
    // std::string
    // Edge::getBidiEdge(const std::string& edgeID) {
    //     const MSEdge* bidi = getEdge(edgeID)->getBidiEdge();
    //     return bidi == nullptr ? "" : bidi->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(edgeID: &str) -> String {
        let bidi = Self::get_edge(edgeID).get_bidi_edge();
        if bidi.is_null() { String::new() } else { bidi.get_id().to_string() }
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
        let _ = MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        MSEdge::insert_i_ds(&mut ids);
        ids
    }

    /// C++: `int Edge::getIDCount()` [static]
    // int
    // Edge::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Edge::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Edge::getParameter(const std::string& edgeID, const std::string& param) {
    //     return getEdge(edgeID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_edge(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Edge::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Edge)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Edge::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Edge::setParameter(const std::string& edgeID, const std::string& name, const std::string& value) {
    //     getEdge(edgeID)->setParameter(name, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_edge(objectID).m().base2.set_parameter(key, value);
    }

    /// C++: `void Edge::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_EDGE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Edge::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_EDGE_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Edge::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_EDGE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Edge::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_EDGE_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Edge::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Edge::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Edge::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Edge::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Edge::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Edge, EDGE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        // shortcut: TraCIString is a subclass wrapper (base: TraCIResult), so std::make_shared<TraCIString> cannot be stored as Rc<TraCIResult> yet, upgrade with the TraCIResult class hierarchy
        panic!("shortcut: Edge::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult>")
    }

    /// C++: `int Edge::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_EDGE_VARIABLE
    }

    /// C++: `void Edge::setAllowed(const std::string &, std::string)` [static]
    // void
    // Edge::setAllowed(const std::string& edgeID, std::string allowedClasses) {
    //     setAllowedSVCPermissions(edgeID, parseVehicleClasses(allowedClasses));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed(edgeID: &str, allowedClasses: String) {
        Self::set_allowed_svc_permissions(edgeID, parse_vehicle_classes(&allowedClasses));
    }

    /// C++: `void Edge::setAllowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Edge::setAllowed(const std::string& edgeID, std::vector<std::string> allowedClasses) {
    //     setAllowedSVCPermissions(edgeID, parseVehicleClasses(allowedClasses));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed_2(edgeID: &str, allowedClasses: Vec<String>) {
        Self::set_allowed_svc_permissions(edgeID, parse_vehicle_classes_vec(&allowedClasses).unwrap_or_else(|e| panic!("{}", e)));
    }

    /// C++: `void Edge::setDisallowed(const std::string &, std::string)` [static]
    // void
    // Edge::setDisallowed(const std::string& edgeID, std::string disallowedClasses) {
    //     setAllowedSVCPermissions(edgeID, invertPermissions(parseVehicleClasses(disallowedClasses)));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed(edgeID: &str, disallowedClasses: String) {
        Self::set_allowed_svc_permissions(edgeID, invert_permissions(parse_vehicle_classes(&disallowedClasses)));
    }

    /// C++: `void Edge::setDisallowed(const std::string &, std::vector<std::string>)` [static]
    // void
    // Edge::setDisallowed(const std::string& edgeID, std::vector<std::string> disallowedClasses) {
    //     setAllowedSVCPermissions(edgeID, invertPermissions(parseVehicleClasses(disallowedClasses)));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed_2(edgeID: &str, disallowedClasses: Vec<String>) {
        Self::set_allowed_svc_permissions(edgeID, invert_permissions(parse_vehicle_classes_vec(&disallowedClasses).unwrap_or_else(|e| panic!("{}", e))));
    }

    /// C++: `void Edge::adaptTraveltime(const std::string &, double, double, double)` [static]
    // void
    // Edge::adaptTraveltime(const std::string& edgeID, double time, double beginSeconds, double endSeconds) {
    //     MSNet::getInstance()->getWeightsStorage().addTravelTime(getEdge(edgeID), beginSeconds, endSeconds, time);
    // }
    #[allow(unused_variables)]
    pub fn adapt_traveltime(edgeID: &str, time: f64, beginSeconds: f64, endSeconds: f64) {
        let e = Self::get_edge(edgeID);
        MSNet::get_instance().m().get_weights_storage().add_travel_time(e, beginSeconds, endSeconds, time);
    }

    /// C++: `void Edge::setEffort(const std::string &, double, double, double)` [static]
    // void
    // Edge::setEffort(const std::string& edgeID, double effort, double beginSeconds, double endSeconds) {
    //     MSNet::getInstance()->getWeightsStorage().addEffort(getEdge(edgeID), beginSeconds, endSeconds, effort);
    // }
    #[allow(unused_variables)]
    pub fn set_effort(edgeID: &str, effort: f64, beginSeconds: f64, endSeconds: f64) {
        let e = Self::get_edge(edgeID);
        MSNet::get_instance().m().get_weights_storage().add_effort(e, beginSeconds, endSeconds, effort);
    }

    /// C++: `void Edge::setMaxSpeed(const std::string &, double)` [static]
    // void
    // Edge::setMaxSpeed(const std::string& edgeID, double speed) {
    //     getEdge(edgeID)->setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(edgeID: &str, speed: f64) {
        // C++ default arguments: modified = true, jamThreshold = -1
        Self::get_edge(edgeID).m().set_max_speed(speed, true, -1.0);
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
        let e = Self::get_edge(edgeID);
        for lane in e.get_lanes().iter() {
            lane.m().set_friction_coefficient(friction);
        }
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
        let e = Self::get_edge(edgeID);
        for lane in e.get_lanes().iter() {
            for p in lane.get_shape(false).iter() {
                shape.0.push(p.clone());
            }
        }
    }

    /// C++: `std::shared_ptr<VariableWrapper> Edge::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Edge::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        // shortcut: Helper::SubscriptionWrapper borrows the class-level subscription statics by reference, which thread_local RefCells cannot hand out yet
        panic!("shortcut: Edge::makeWrapper needs the subscription wrapper over class statics")
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_EDGE_TRAVELTIME => {
                let t = StoHelp::read_typed_double(paramData.m(), "");
                wrapper.m().wrap_double(objID, variable, Self::get_adapted_traveltime(objID, t))
            }
            VAR_EDGE_EFFORT => {
                let t = StoHelp::read_typed_double(paramData.m(), "");
                wrapper.m().wrap_double(objID, variable, Self::get_effort(objID, t))
            }
            VAR_CURRENT_TRAVELTIME => wrapper.m().wrap_double(objID, variable, Self::get_traveltime(objID)),
            VAR_WAITING_TIME => wrapper.m().wrap_double(objID, variable, Self::get_waiting_time(objID)),
            LAST_STEP_PERSON_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_step_person_i_ds(objID)),
            LAST_STEP_VEHICLE_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_step_vehicle_i_ds(objID)),
            VAR_CO2EMISSION => wrapper.m().wrap_double(objID, variable, Self::get_co2_emission(objID)),
            VAR_COEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_co_emission(objID)),
            VAR_HCEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_hc_emission(objID)),
            VAR_PMXEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_p_mx_emission(objID)),
            VAR_NOXEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_n_ox_emission(objID)),
            VAR_FUELCONSUMPTION => wrapper.m().wrap_double(objID, variable, Self::get_fuel_consumption(objID)),
            VAR_NOISEEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_noise_emission(objID)),
            VAR_ELECTRICITYCONSUMPTION => wrapper.m().wrap_double(objID, variable, Self::get_electricity_consumption(objID)),
            LAST_STEP_VEHICLE_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_vehicle_number(objID)),
            LAST_STEP_MEAN_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_last_step_mean_speed(objID)),
            VAR_MAXSPEED => wrapper.m().wrap_double(objID, variable, Self::get_max_speed(objID)),
            VAR_FRICTION => wrapper.m().wrap_double(objID, variable, Self::get_mean_friction(objID)),
            LAST_STEP_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_step_occupancy(objID)),
            LAST_STEP_VEHICLE_HALTING_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_halting_number(objID)),
            LAST_STEP_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_last_step_length(objID)),
            VAR_LANE_INDEX => wrapper.m().wrap_int(objID, variable, Self::get_lane_number(objID)),
            VAR_NAME => wrapper.m().wrap_string(objID, variable, &Self::get_street_name(objID)),
            VAR_PENDING_VEHICLES => wrapper.m().wrap_string_list(objID, variable, &Self::get_pending_vehicles(objID)),
            VAR_ANGLE => {
                let relPos = StoHelp::read_typed_double(paramData.m(), "");
                wrapper.m().wrap_double(objID, variable, Self::get_angle(objID, relPos))
            }
            FROM_JUNCTION => wrapper.m().wrap_string(objID, variable, &Self::get_from_junction(objID)),
            TO_JUNCTION => wrapper.m().wrap_string(objID, variable, &Self::get_to_junction(objID)),
            VAR_BIDI => wrapper.m().wrap_string(objID, variable, &Self::get_bidi_edge(objID)),
            VAR_PARAMETER => {
                let key = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                let key = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => false,
        }
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
        let e = Self::get_edge(edgeID);
        for lane in e.get_lanes().iter() {
            lane.m().set_permissions(permissions, MSLane_CHANGE_PERMISSIONS_PERMANENT);
        }
        e.m().rebuild_allowed_lanes(false, true);
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
        // shortcut: MSEdge::dictionary_2 takes an Unmapped std::string_view, so the id is looked up by a linear scan over MSEdgeControl::get_edges (normal edges only), upgrade when dictionary_2 takes &str
        for e in MSNet::get_instance().m().get_edge_control().get_edges().iter() {
            if e.get_id() == edgeID {
                return *e;
            }
        }
        panic!("{}", format!("Edge '{}' is not known", edgeID));
    }

    /// C++: `void Edge::Edge()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Edge cannot be constructed: the standard constructor is invalidated in C++")
    }

}

