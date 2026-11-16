//! Port of libsumo/Lane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{get_vehicle_class_names_list, invert_permissions, parse_vehicle_classes_vec};
use utils::common::std_defs::SUMO_CONST_HALTING_SPEED;
use utils::geom::geom_helper::navi_degree;
// shortcut: C++ `throw TraCIException(msg)` is mapped to `panic!("{}", msg)` (unwinds where C++ throws), upgrade when libsumo has a typed TraCI error that callers can catch

/// C++ class `libsumo::Lane` (libsumo/Lane.h)
pub struct Lane {
}

// C++ static members of Lane (port as module-level statics, see AGENTS.md):
//   static const int Lane::DOMAIN_ID;
//   static SubscriptionResults Lane::mySubscriptionResults;
//   static ContextSubscriptionResults Lane::myContextSubscriptionResults;

thread_local! {
    /// C++ static `Lane::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Lane::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl Lane {
    /// C++: `int Lane::getLinkNumber(const std::string &)` [static]
    // int
    // Lane::getLinkNumber(const std::string& laneID) {
    //     return (int)getLane(laneID)->getLinkCont().size();
    // }
    #[allow(unused_variables)]
    pub fn get_link_number(laneID: &str) -> i32 {
        Self::get_lane(laneID).get_link_cont().len() as i32
    }

    /// C++: `std::string Lane::getEdgeID(const std::string &)` [static]
    // std::string
    // Lane::getEdgeID(const std::string& laneID) {
    //     return getLane(laneID)->getEdge().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_id(laneID: &str) -> String {
        Self::get_lane(laneID).get_edge().get_id().to_string()
    }

    /// C++: `double Lane::getLength(const std::string &)` [static]
    // double
    // Lane::getLength(const std::string& laneID) {
    //     return getLane(laneID)->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_length()
    }

    /// C++: `double Lane::getMaxSpeed(const std::string &)` [static]
    // double
    // Lane::getMaxSpeed(const std::string& laneID) {
    //     return getLane(laneID)->getSpeedLimit();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_speed_limit()
    }

    /// C++: `double Lane::getFriction(const std::string &)` [static]
    // double
    // Lane::getFriction(const std::string& laneID) {
    //     return getLane(laneID)->getFrictionCoefficient();
    // }
    #[allow(unused_variables)]
    pub fn get_friction(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_friction_coefficient()
    }

    /// C++: `std::vector<std::string> Lane::getAllowed(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getAllowed(const std::string& laneID) {
    //     return getVehicleClassNamesList(getLane(laneID)->getPermissions());
    // }
    #[allow(unused_variables)]
    pub fn get_allowed(laneID: &str) -> Vec<String> {
        get_vehicle_class_names_list(Self::get_lane(laneID).get_permissions())
    }

    /// C++: `std::vector<std::string> Lane::getDisallowed(const std::string &)` [static]
    // std::vector<std::string>
    // Lane::getDisallowed(const std::string& laneID) {
    //     return getVehicleClassNamesList(invertPermissions((getLane(laneID)->getPermissions()))); // negation yields disallowed
    // }
    #[allow(unused_variables)]
    pub fn get_disallowed(laneID: &str) -> Vec<String> {
        // negation yields disallowed
        get_vehicle_class_names_list(invert_permissions(Self::get_lane(laneID).get_permissions()))
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
        if direction == LANECHANGE_LEFT {
            get_vehicle_class_names_list(Self::get_lane(laneID).get_change_left())
        } else if direction == LANECHANGE_RIGHT {
            get_vehicle_class_names_list(Self::get_lane(laneID).get_change_right())
        } else {
            panic!("{}", format!("Invalid direction for change permission (must be {} or {}", LANECHANGE_LEFT, LANECHANGE_RIGHT));
        }
    }

    /// C++: `std::vector<libsumo::TraCIConnection> Lane::getLinks(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_links(laneID: &str) -> Vec<TraCIConnection> {
        let mut v: Vec<TraCIConnection> = Vec::new();
        let lane = Self::get_lane(laneID);
        let currTime = MSNet::get_instance().get_current_time_step();
        for link in lane.get_link_cont().iter() {
            let approachedLane = if !link.get_lane().is_null() { link.get_lane().get_id().to_string() } else { String::new() };
            let hasPrio = link.have_priority();
            let speed = {
                let a = lane.get_speed_limit();
                let b = link.get_lane().get_speed_limit();
                if a < b { a } else { b }
            };
            let defaults = SUMOVTypeParameter::get_default();
            // C++ defaults of MSLink::opened: posLat=0, collectFoes=nullptr, ignoreRed=false, ego=nullptr, dist=-1
            let isOpen = link.opened(currTime, speed, speed, defaults.length, defaults.impatience, SUMOVTypeParameter::get_default_decel(SVC_IGNORING), 0, 0.0, Ptr::null(), false, Ptr::null(), -1.0);
            let hasFoe = link.has_approaching_foe(currTime, currTime, 0.0, SUMOVTypeParameter::get_default_decel(SVC_IGNORING));
            let approachedInternal = if !link.get_via_lane().is_null() { link.get_via_lane().get_id().to_string() } else { String::new() };
            let state = LinkStates.get_string(link.get_state()).to_string();
            let direction = LinkDirections.get_string(link.get_direction()).to_string();
            let length = link.get_length();
            v.push(TraCIConnection { approachedLane, hasPrio, isOpen, hasFoe, approachedInternal, state, direction, length });
        }
        v
    }

    /// C++: `libsumo::TraCIPositionVector Lane::getShape(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_shape(laneID: &str) -> TraCIPositionVector {
        let shp = Self::get_lane(laneID).get_shape(false).clone();
        let mut pv = TraCIPositionVector { base: TraCIResult::new(), value: Vec::new() };
        for pi in shp.iter() {
            pv.value.push(TraCIPosition { base: TraCIResult::new(), x: pi.x(), y: pi.y(), z: pi.z() });
        }
        pv
    }

    /// C++: `double Lane::getWidth(const std::string &)` [static]
    // double
    // Lane::getWidth(const std::string& laneID) {
    //     return getLane(laneID)->getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_width()
    }

    /// C++: `double Lane::getCO2Emission(const std::string &)` [static]
    // double
    // Lane::getCO2Emission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::CO2>();
    // }
    #[allow(unused_variables)]
    pub fn get_co2_emission(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::CO2>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getCO2Emission needs MSLane::getEmissions<CO2>")
    }

    /// C++: `double Lane::getCOEmission(const std::string &)` [static]
    // double
    // Lane::getCOEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::CO>();
    // }
    #[allow(unused_variables)]
    pub fn get_co_emission(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::CO>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getCOEmission needs MSLane::getEmissions<CO>")
    }

    /// C++: `double Lane::getHCEmission(const std::string &)` [static]
    // double
    // Lane::getHCEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::HC>();
    // }
    #[allow(unused_variables)]
    pub fn get_hc_emission(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::HC>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getHCEmission needs MSLane::getEmissions<HC>")
    }

    /// C++: `double Lane::getPMxEmission(const std::string &)` [static]
    // double
    // Lane::getPMxEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::PM_X>();
    // }
    #[allow(unused_variables)]
    pub fn get_p_mx_emission(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::PM_X>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getPMxEmission needs MSLane::getEmissions<PM_X>")
    }

    /// C++: `double Lane::getNOxEmission(const std::string &)` [static]
    // double
    // Lane::getNOxEmission(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::NO_X>();
    // }
    #[allow(unused_variables)]
    pub fn get_n_ox_emission(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::NO_X>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getNOxEmission needs MSLane::getEmissions<NO_X>")
    }

    /// C++: `double Lane::getFuelConsumption(const std::string &)` [static]
    // double
    // Lane::getFuelConsumption(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::FUEL>();
    // }
    #[allow(unused_variables)]
    pub fn get_fuel_consumption(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::FUEL>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getFuelConsumption needs MSLane::getEmissions<FUEL>")
    }

    /// C++: `double Lane::getNoiseEmission(const std::string &)` [static]
    // double
    // Lane::getNoiseEmission(const std::string& laneID) {
    //     return getLane(laneID)->getHarmonoise_NoiseEmissions();
    // }
    #[allow(unused_variables)]
    pub fn get_noise_emission(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_harmonoise_noise_emissions()
    }

    /// C++: `double Lane::getElectricityConsumption(const std::string &)` [static]
    // double
    // Lane::getElectricityConsumption(const std::string& laneID) {
    //     return getLane(laneID)->getEmissions<PollutantsInterface::ELEC>();
    // }
    #[allow(unused_variables)]
    pub fn get_electricity_consumption(laneID: &str) -> f64 {
        // shortcut: needs MSLane::getEmissions<PollutantsInterface::ELEC>, a template that the Rust MSLane::get_emissions does not carry (and is still todo!), upgrade with the typed emission port
        panic!("shortcut: Lane::getElectricityConsumption needs MSLane::getEmissions<ELEC>")
    }

    /// C++: `double Lane::getLastStepMeanSpeed(const std::string &)` [static]
    // double
    // Lane::getLastStepMeanSpeed(const std::string& laneID) {
    //     return getLane(laneID)->getMeanSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_mean_speed(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_mean_speed()
    }

    /// C++: `double Lane::getLastStepOccupancy(const std::string &)` [static]
    // double
    // Lane::getLastStepOccupancy(const std::string& laneID) {
    //     return getLane(laneID)->getNettoOccupancy();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_occupancy(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_netto_occupancy()
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
        let lane = Self::get_lane(laneID);
        let mut length: f64 = 0.0;
        let vehs = lane.get_vehicles_secure();
        for veh in vehs.iter() {
            length += veh.get_vehicle_type().get_length();
        }
        if vehs.len() > 0 {
            length = length / (vehs.len() as f64);
        }
        lane.release_vehicles();
        length
    }

    /// C++: `double Lane::getWaitingTime(const std::string &)` [static]
    // double
    // Lane::getWaitingTime(const std::string& laneID) {
    //     return getLane(laneID)->getWaitingSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(laneID: &str) -> f64 {
        Self::get_lane(laneID).get_waiting_seconds()
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
        let lane = Self::get_lane(laneID);
        let meanSpeed = lane.get_mean_speed();
        if meanSpeed != 0.0 {
            lane.get_length() / meanSpeed
        } else {
            1000000.0
        }
    }

    /// C++: `int Lane::getLastStepVehicleNumber(const std::string &)` [static]
    // int
    // Lane::getLastStepVehicleNumber(const std::string& laneID) {
    //     return (int)getLane(laneID)->getVehicleNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_vehicle_number(laneID: &str) -> i32 {
        Self::get_lane(laneID).get_vehicle_number()
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
        let lane = Self::get_lane(laneID);
        let mut halting: i32 = 0;
        let vehs = lane.get_vehicles_secure();
        for veh in vehs.iter() {
            if veh.get_speed() < SUMO_CONST_HALTING_SPEED {
                halting += 1;
            }
        }
        lane.release_vehicles();
        halting
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
        let lane = Self::get_lane(laneID);
        let mut vehIDs: Vec<String> = Vec::new();
        let vehs = lane.get_vehicles_secure();
        for veh in vehs.iter() {
            vehIDs.push(veh.get_id().to_string());
        }
        lane.release_vehicles();
        vehIDs
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
        if toLaneID == "" {
            return Self::get_internal_foes(laneID);
        }
        let mut foeIDs: Vec<String> = Vec::new();
        let link = Self::get_lane(laneID).get_link_to(Self::get_lane(toLaneID));
        if link.is_null() {
            panic!("{}", format!("No connection from lane '{}' to lane '{}'", laneID, toLaneID));
        }
        for foe in link.get_foe_links().iter() {
            foeIDs.push(foe.get_lane_before().get_id().to_string());
        }
        foeIDs
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
        let lane = Self::get_lane(laneID);
        let mut foeIDs: Vec<String> = Vec::new();
        if (lane.is_internal() || lane.is_crossing()) && lane.get_link_cont().len() > 0 {
            let link = lane.get_link_cont()[0];
            for foe in link.get_foe_lanes().iter() {
                foeIDs.push(foe.get_id().to_string());
            }
        }
        foeIDs
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
        let l = Self::get_lane(laneID); // validate laneID
        let mut vehIDs: Vec<String> = Vec::new();
        for veh in MSNet::get_instance().m().get_insertion_control().get_pending_vehicles().iter() {
            if veh.get_lane().as_raw() == l.as_raw() {
                vehIDs.push(veh.get_id().to_string());
            }
        }
        vehIDs
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
        let angle: f64;
        let lane = Self::get_lane(laneID);
        if relativePosition == INVALID_DOUBLE_VALUE {
            let shape = lane.get_shape(false);
            let start = shape.front();
            let end = shape.back();
            angle = start.angle_to2d(end);
        } else {
            angle = lane.get_shape(false).rotation_at_offset(lane.interpolate_lane_pos_to_geometry_pos(relativePosition));
        }
        navi_degree(angle)
    }

    /// C++: `std::string Lane::getBidiLane(const std::string &)` [static]
    // std::string
    // Lane::getBidiLane(const std::string& laneID) {
    //     const MSLane* bidi = getLane(laneID)->getBidiLane();
    //     return bidi == nullptr ? "" : bidi->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_bidi_lane(laneID: &str) -> String {
        let bidi = Self::get_lane(laneID).get_bidi_lane();
        if bidi.is_null() { String::new() } else { bidi.get_id().to_string() }
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
        let _ = MSNet::get_instance(); // just to check that we actually have a network
        let mut ids: Vec<String> = Vec::new();
        MSLane::insert_i_ds(&mut ids);
        ids
    }

    /// C++: `int Lane::getIDCount()` [static]
    // int
    // Lane::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Lane::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Lane::getParameter(const std::string& laneID, const std::string& param) {
    //     return getLane(laneID)->getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_lane(objectID).base2.get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Lane::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Lane)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Lane::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Lane::setParameter(const std::string& laneID, const std::string& key, const std::string& value) {
    //     getLane(laneID)->setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        Self::get_lane(objectID).m().base2.set_parameter(key, value);
    }

    /// C++: `void Lane::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_LANE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Lane::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_LANE_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Lane::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_LANE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Lane::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_LANE_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Lane::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Lane::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Lane::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Lane::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Lane::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Lane, LANE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_LANE_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int Lane::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_LANE_VARIABLE
    }

    /// C++: `void Lane::setAllowed(const std::string &, std::string)` [static]
    // void
    // Lane::setAllowed(const std::string& laneID, std::string allowedClass) {
    //     setAllowed(laneID, std::vector<std::string>({allowedClass}));
    // }
    #[allow(unused_variables)]
    pub fn set_allowed(laneID: &str, allowedClasses: String) {
        Self::set_allowed_2(laneID, vec![allowedClasses]);
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
        let l = Self::get_lane(laneID);
        let perms = parse_vehicle_classes_vec(&allowedClasses).unwrap_or_else(|e| panic!("{}", e));
        l.m().set_permissions(perms, MSLane_CHANGE_PERMISSIONS_PERMANENT);
        l.get_edge().rebuild_allowed_lanes(false, true);
    }

    /// C++: `void Lane::setDisallowed(const std::string &, std::string)` [static]
    // void
    // Lane::setDisallowed(const std::string& laneID, std::string disallowedClasses) {
    //     setDisallowed(laneID, std::vector<std::string>({disallowedClasses}));
    // }
    #[allow(unused_variables)]
    pub fn set_disallowed(laneID: &str, disallowedClasses: String) {
        Self::set_disallowed_2(laneID, vec![disallowedClasses]);
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
        let l = Self::get_lane(laneID);
        // negation yields allowed
        let perms = invert_permissions(parse_vehicle_classes_vec(&disallowedClasses).unwrap_or_else(|e| panic!("{}", e)));
        l.m().set_permissions(perms, MSLane_CHANGE_PERMISSIONS_PERMANENT);
        l.get_edge().rebuild_allowed_lanes(false, true);
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
        let l = Self::get_lane(laneID);
        if direction == LANECHANGE_LEFT {
            l.m().set_change_left(parse_vehicle_classes_vec(&allowedClasses).unwrap_or_else(|e| panic!("{}", e)));
        } else if direction == LANECHANGE_RIGHT {
            l.m().set_change_right(parse_vehicle_classes_vec(&allowedClasses).unwrap_or_else(|e| panic!("{}", e)));
        } else {
            panic!("{}", format!("Invalid direction for change permission (must be {} or {}", LANECHANGE_LEFT, LANECHANGE_RIGHT));
        }
    }

    /// C++: `void Lane::setMaxSpeed(const std::string &, double)` [static]
    // void
    // Lane::setMaxSpeed(const std::string& laneID, double speed) {
    //     getLane(laneID)->setMaxSpeed(speed, false, true);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(laneID: &str, speed: f64) {
        // C++ passes `true` as the double jamThreshold argument, which converts to 1.0
        Self::get_lane(laneID).m().set_max_speed(speed, false, 1.0);
    }

    /// C++: `void Lane::setLength(const std::string &, double)` [static]
    // void
    // Lane::setLength(const std::string& laneID, double length) {
    //     getLane(laneID)->setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(laneID: &str, length: f64) {
        Self::get_lane(laneID).m().set_length(length);
    }

    /// C++: `void Lane::setFriction(const std::string &, double)` [static]
    // void
    // Lane::setFriction(const std::string& laneID, double friction) {
    //     getLane(laneID)->setFrictionCoefficient(friction);
    // }
    #[allow(unused_variables)]
    pub fn set_friction(laneID: &str, friction: f64) {
        Self::get_lane(laneID).m().set_friction_coefficient(friction);
    }

    /// C++: `void Lane::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Lane::storeShape(const std::string& id, PositionVector& shape) {
    //     shape = getLane(id)->getShape();
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        *shape = Self::get_lane(id).get_shape(false).clone();
    }

    /// C++: `std::shared_ptr<VariableWrapper> Lane::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Lane::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
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
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            LANE_LINK_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_link_number(objID)),
            LANE_EDGE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_edge_id(objID)),
            VAR_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_length(objID)),
            VAR_MAXSPEED => wrapper.m().wrap_double(objID, variable, Self::get_max_speed(objID)),
            VAR_FRICTION => wrapper.m().wrap_double(objID, variable, Self::get_friction(objID)),
            LANE_ALLOWED => wrapper.m().wrap_string_list(objID, variable, &Self::get_allowed(objID)),
            LANE_DISALLOWED => wrapper.m().wrap_string_list(objID, variable, &Self::get_disallowed(objID)),
            LANE_CHANGES => {
                let direction = StoHelp::read_typed_byte(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_change_permissions(objID, direction))
            }
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
            LAST_STEP_VEHICLE_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_last_step_vehicle_i_ds(objID)),
            LAST_STEP_OCCUPANCY => wrapper.m().wrap_double(objID, variable, Self::get_last_step_occupancy(objID)),
            LAST_STEP_VEHICLE_HALTING_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_last_step_halting_number(objID)),
            LAST_STEP_LENGTH => wrapper.m().wrap_double(objID, variable, Self::get_last_step_length(objID)),
            VAR_WAITING_TIME => wrapper.m().wrap_double(objID, variable, Self::get_waiting_time(objID)),
            VAR_CURRENT_TRAVELTIME => wrapper.m().wrap_double(objID, variable, Self::get_traveltime(objID)),
            VAR_WIDTH => wrapper.m().wrap_double(objID, variable, Self::get_width(objID)),
            VAR_SHAPE => wrapper.m().wrap_position_vector(objID, variable, &Self::get_shape(objID)),
            VAR_PENDING_VEHICLES => wrapper.m().wrap_string_list(objID, variable, &Self::get_pending_vehicles(objID)),
            VAR_ANGLE => {
                let relPos = StoHelp::read_typed_double(paramData.m(), "");
                wrapper.m().wrap_double(objID, variable, Self::get_angle(objID, relPos))
            }
            VAR_BIDI => wrapper.m().wrap_string(objID, variable, &Self::get_bidi_lane(objID)),
            VAR_FOES => {
                let toLane = StoHelp::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_foes(objID, &toLane))
            }
            LANE_LINKS => wrapper.m().wrap_connection_vector(objID, variable, &Self::get_links(objID)),
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
        let lane = MSLane::dictionary_2(id);
        if lane.is_null() {
            panic!("{}", format!("Lane '{}' is not known", id));
        }
        lane
    }

    /// C++: `void Lane::Lane()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        panic!("libsumo::Lane cannot be constructed: the standard constructor is invalidated in C++")
    }

}

