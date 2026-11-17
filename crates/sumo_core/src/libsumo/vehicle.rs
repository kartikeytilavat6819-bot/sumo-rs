//! Port of libsumo/Vehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Vehicle` (libsumo/Vehicle.h)
pub struct Vehicle {
}

// C++ static members of Vehicle (port as module-level statics, see AGENTS.md):
//   static const int Vehicle::DOMAIN_ID;

thread_local! {
    /// C++ static `Vehicle::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Vehicle::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

use utils::geom::geom_helper::navi_degree;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::time::{delta_t, speed2dist, SUMOTIME_MAX};
use crate::libsumo::helper::check_time_bounds;
use utils::common::to_string::to_string;
use utils::xml::sumo_xml_definitions as SUMOXMLDefinitions;
use utils::common::msg_handler::write_warning;
use utils::common::sumo_vehicle_class::SVC_IGNORING;
use utils::geom::geom_helper;
use utils::traci::traci_constants::{REMOVE_ARRIVED, REMOVE_PARKING, REMOVE_TELEPORT, REMOVE_TELEPORT_ARRIVED, REMOVE_VAPORIZED};

// SUMOVehicleParameter.h / SUMOVehicleParameter::Stop parametersSet bits (not exported by the ported module yet)
const VEHPARS_COLOR_SET: i64 = 1;
const VEHPARS_FORCE_REROUTE: i64 = 2 << 17;
const VEHPARS_VTYPE_SET: i64 = 2;
const VEHPARS_DEPARTLANE_SET: i64 = 2 << 1;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const VEHPARS_DEPARTSPEED_SET: i64 = 2 << 3;
const VEHPARS_ARRIVALLANE_SET: i64 = 2 << 11;
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;
const VEHPARS_ARRIVALSPEED_SET: i64 = 2 << 13;
const VEHPARS_LINE_SET: i64 = 2 << 14;
const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
const VEHPARS_PERSON_NUMBER_SET: i64 = 2 << 19;
const VEHPARS_DEPARTPOSLAT_SET: i64 = 2 << 21;
const VEHPARS_ARRIVALPOSLAT_SET: i64 = 2 << 22;
const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
const VEHPARS_INSERTION_CHECKS_SET: i64 = 2 << 31;
const STOP_START_SET: i32 = 1;
const STOP_END_SET: i32 = 2;
const STOP_DURATION_SET: i32 = 2 << 1;
const STOP_UNTIL_SET: i32 = 2 << 2;
const STOP_EXTENSION_SET: i32 = 2 << 3;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;
const STOP_EXPECTED_SET: i32 = 2 << 6;
const STOP_EXPECTED_CONTAINERS_SET: i32 = 2 << 8;
const STOP_TRIP_ID_SET: i32 = 2 << 9;
const STOP_LINE_SET: i32 = 2 << 10;
const STOP_SPEED_SET: i32 = 2 << 11;
const STOP_SPLIT_SET: i32 = 2 << 12;
const STOP_JOIN_SET: i32 = 2 << 13;
const STOP_ARRIVAL_SET: i32 = 2 << 14;
const STOP_PERMITTED_SET: i32 = 2 << 15;
const STOP_ENDED_SET: i32 = 2 << 16;
const STOP_STARTED_SET: i32 = 2 << 17;
const STOP_POSLAT_SET: i32 = 2 << 18;
const STOP_ONDEMAND_SET: i32 = 2 << 19;
const STOP_JUMP_SET: i32 = 2 << 20;
const STOP_JUMP_UNTIL_SET: i32 = 2 << 21;
// shortcut: dynamic_cast<MSVehicle*> / dynamic_cast<MEVehicle*> cannot be checked at runtime (no type tag on
// MSBaseVehicle, MSGlobals::gUseMesoSim not ported), so every vehicle is treated as micro. A MEVehicle would be
// misread. Upgrade when a vehicle type tag or gUseMesoSim exists. The casts also need #[repr(C)] with `base`
// first on MSBaseVehicle and MSVehicle (not yet present on those structs).
fn micro_veh(veh: Ptr<MSBaseVehicle>) -> Ptr<MSVehicle> {
    veh.cast::<MSVehicle>()
}

/// C++ `veh->getEmissions<ET>()` (MSBaseVehicle template; the skeleton's get_emissions() has no ET parameter).
fn get_emissions_et(veh: Ptr<MSBaseVehicle>, et: PollutantsInterface_EmissionType) -> f64 {
    veh.m().get_emissions(et)
}

/// Mutable access to a vehicle type; C++ Helper::getVehicleType returns a non-const reference.
fn vtype_mut(typeID: &str) -> Ptr<MSVehicleType> {
    Ptr::new(Helper::get_vehicle_type(typeID))
}

/// Body of the per-link loop in C++ `Vehicle::getNextLinks`: one TraCIConnection for `link`, leaving `lane`.
fn next_link_connection(veh: Ptr<MSVehicle>, lane: Ptr<MSLane>, link: Ptr<MSLink>, currTime: SUMOTime) -> TraCIConnection {
    let approachedLane = if !link.get_lane().is_null() { link.get_lane().get_id().to_string() } else { String::new() };
    let hasPrio = link.have_priority();
    let speed = {
        let a = lane.get_speed_limit();
        let b = link.get_lane().get_speed_limit();
        if a < b { a } else { b }
    };
    let isOpen = link.opened(currTime, speed, speed, veh.get_length(), veh.get_impatience(), veh.get_vehicle_type().get_car_follow_model().get_max_decel(), veh.get_waiting_time(false), veh.get_lateral_position_on_lane(), Ptr::null(), false, veh.cast::<SUMOTrafficObject>(), -1.0);
    let hasFoe = link.has_approaching_foe(currTime, currTime, 0.0, SUMOVTypeParameter::get_default_decel(SVC_IGNORING));
    let approachedInternal = if !link.get_via_lane().is_null() { link.get_via_lane().get_id().to_string() } else { String::new() };
    let state = LinkStates.get_string(link.get_state()).to_string();
    let direction = LinkDirections.get_string(link.get_direction()).to_string();
    TraCIConnection { approachedLane, hasPrio, isOpen, hasFoe, approachedInternal, state, direction, length: link.get_length() }
}

/// C++ setStopParameter: ProcessError rethrown as TraCIException with this prefix.
fn set_stop_param_error(vehID: &str, msg: String) -> ! {
    panic!("{}", format!("Could not set stop parameter for vehicle '{}' ({})", vehID, msg))
}

/// Mutable view of a vehicle's parameter block (C++ const_cast-style access used by setColor/setLine/setParameter).
fn param_mut(p: &SUMOVehicleParameter) -> &mut SUMOVehicleParameter {
    unsafe { &mut *Ptr::new(p).as_raw() }
}

impl Vehicle {
    /// C++: `double Vehicle::getSpeed(const std::string &)` [static]
    /// @{
    // double
    // Vehicle::getSpeed(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getSpeed() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { veh.get_speed() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getLateralSpeed(const std::string &)` [static]
    // double
    // Vehicle::getLateralSpeed(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getLaneChangeModel().getSpeedLat(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_speed(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { micro_veh(veh).m().get_lane_change_model().get_speed_lat() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getAcceleration(const std::string &)` [static]
    // double
    // Vehicle::getAcceleration(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getAcceleration(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_acceleration(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { micro_veh(veh).get_acceleration() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getSpeedWithoutTraCI(const std::string &)` [static]
    // double
    // Vehicle::getSpeedWithoutTraCI(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getSpeedWithoutTraciInfluence(), veh->getSpeed()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_speed_without_tra_ci(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { micro_veh(veh).get_speed_without_traci_influence() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `libsumo::TraCIPosition Vehicle::getPosition(const std::string &, const bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(vehID: &str, includeZ: bool) -> TraCIPosition {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) {
            return Helper::make_tra_ci_position(&veh.get_position(0.), includeZ);
        }
        TraCIPosition { base: TraCIResult::new(), x: 0.0, y: 0.0, z: 0.0 }
    }

    /// C++: `libsumo::TraCIPosition Vehicle::getPosition3D(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position3_d(vehID: &str) -> TraCIPosition {
        Self::get_position(vehID, true)
    }

    /// C++: `double Vehicle::getAngle(const std::string &)` [static]
    // double
    // Vehicle::getAngle(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? GeomHelper::naviDegree(veh->getAngle()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { navi_degree(veh.get_angle()) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getSlope(const std::string &)` [static]
    // double
    // Vehicle::getSlope(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return (veh->isOnRoad() || veh->isParking()) ? veh->getSlope() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_slope(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() || veh.is_parking() { veh.get_slope() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `std::string Vehicle::getRoadID(const std::string &)` [static]
    // std::string
    // Vehicle::getRoadID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getLane()->getEdge().getID(), veh->getEdge()->getID()) : "";
    // }
    #[allow(unused_variables)]
    pub fn get_road_id(vehID: &str) -> String {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() { micro_veh(veh).get_lane().get_edge().get_id().to_string() } else { String::new() }
    }

    /// C++: `std::string Vehicle::getLaneID(const std::string &)` [static]
    // std::string
    // Vehicle::getLaneID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLane()->getID(), "") : "";
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(vehID: &str) -> String {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() { micro_veh(veh).get_lane().get_id().to_string() } else { String::new() }
    }

    /// C++: `std::string Vehicle::getShadowLaneID(const std::string &)` [static]
    // std::string
    // Vehicle::getShadowLaneID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     return (microVeh != nullptr && microVeh->isOnRoad()) ? (Named::getIDSecure(microVeh->getLaneChangeModel().getShadowLane(),"")) : "";
    // }
    #[allow(unused_variables)]
    pub fn get_shadow_lane_id(vehID: &str) -> String {
        let veh = Helper::get_vehicle(vehID);
        let microVeh = micro_veh(veh);
        if veh.is_on_road() {
            match microVeh.m().get_lane_change_model().get_shadow_lane().get() {
                Some(lane) => lane.get_id().to_string(),
                None => String::new(),
            }
        } else {
            String::new()
        }
    }

    /// C++: `int Vehicle::getLaneIndex(const std::string &)` [static]
    // int
    // Vehicle::getLaneIndex(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh->isOnRoad()) {
    //         MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //         if (microVeh != nullptr) {
    //             return microVeh->getLane()->getIndex();
    //         } else {
    //             MEVehicle* mesoVeh = dynamic_cast<MEVehicle*>(veh);
    //             return mesoVeh->getQueIndex();
    //         }
    //     } else {
    //         return INVALID_INT_VALUE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_index(vehID: &str) -> i32 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() { micro_veh(veh).get_lane().get_index() } else { INVALID_INT_VALUE }
    }

    /// C++: `std::string Vehicle::getSegmentID(const std::string &)` [static]
    // std::string
    // Vehicle::getSegmentID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MESO_FUN(veh, getSegment()->getID(), "") : "";
    // }
    #[allow(unused_variables)]
    pub fn get_segment_id(vehID: &str) -> String {
        // shortcut: CALL_MESO_FUN; MEVehicle is never detected (see micro_veh), so the micro fallback "" is returned.
        let _veh = Helper::get_vehicle(vehID);
        String::new()
    }

    /// C++: `int Vehicle::getSegmentIndex(const std::string &)` [static]
    // int
    // Vehicle::getSegmentIndex(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MESO_FUN(veh, getSegment()->getIndex(), INVALID_INT_VALUE) : INVALID_INT_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_segment_index(vehID: &str) -> i32 {
        // shortcut: CALL_MESO_FUN; MEVehicle is never detected (see micro_veh), so the micro fallback is returned.
        let _veh = Helper::get_vehicle(vehID);
        INVALID_INT_VALUE
    }

    /// C++: `std::string Vehicle::getTypeID(const std::string &)` [static]
    // std::string
    // Vehicle::getTypeID(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getID();
    // }
    #[allow(unused_variables)]
    pub fn get_type_id(vehID: &str) -> String {
        Helper::get_vehicle_type(vehID).get_id().to_string()
    }

    /// C++: `std::string Vehicle::getRouteID(const std::string &)` [static]
    // std::string
    // Vehicle::getRouteID(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getRoute().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_route_id(vehID: &str) -> String {
        Helper::get_vehicle(vehID).get_route().get_id().to_string()
    }

    /// C++: `double Vehicle::getDeparture(const std::string &)` [static]
    // double
    // Vehicle::getDeparture(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->hasDeparted() ? STEPS2TIME(veh->getDeparture()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_departure(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.has_departed() { steps2time(veh.get_departure()) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getDepartDelay(const std::string &)` [static]
    // double
    // Vehicle::getDepartDelay(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicle(vehID)->getDepartDelay());
    // }
    #[allow(unused_variables)]
    pub fn get_depart_delay(vehID: &str) -> f64 {
        steps2time(Helper::get_vehicle(vehID).get_depart_delay())
    }

    /// C++: `int Vehicle::getRouteIndex(const std::string &)` [static]
    // int
    // Vehicle::getRouteIndex(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->hasDeparted() ? veh->getRoutePosition() : INVALID_INT_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_route_index(vehID: &str) -> i32 {
        let veh = Helper::get_vehicle(vehID);
        if veh.has_departed() { veh.get_route_position() } else { INVALID_INT_VALUE }
    }

    /// C++: `double Vehicle::getLanePosition(const std::string &)` [static]
    // double
    // Vehicle::getLanePosition(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return (veh->isOnRoad() || veh->isParking()) ? veh->getPositionOnLane() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_position(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() || veh.is_parking() { veh.get_position_on_lane() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getArrivalPos(const std::string &)` [static]
    // double
    // Vehicle::getArrivalPos(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getArrivalPos();
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(vehID: &str) -> f64 {
        Helper::get_vehicle(vehID).get_arrival_pos()
    }

    /// C++: `double Vehicle::getLateralLanePosition(const std::string &)` [static]
    // double
    // Vehicle::getLateralLanePosition(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLateralPositionOnLane(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_lane_position(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() { micro_veh(veh).get_lateral_position_on_lane() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getCO2Emission(const std::string &)` [static]
    // double
    // Vehicle::getCO2Emission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::CO2>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_co2_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::CO2) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getCOEmission(const std::string &)` [static]
    // double
    // Vehicle::getCOEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::CO>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_co_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::CO) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getHCEmission(const std::string &)` [static]
    // double
    // Vehicle::getHCEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::HC>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_hc_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::HC) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getPMxEmission(const std::string &)` [static]
    // double
    // Vehicle::getPMxEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::PM_X>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_p_mx_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::PM_X) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getNOxEmission(const std::string &)` [static]
    // double
    // Vehicle::getNOxEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::NO_X>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_n_ox_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::NO_X) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getFuelConsumption(const std::string &)` [static]
    // double
    // Vehicle::getFuelConsumption(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::FUEL>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_fuel_consumption(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::FUEL) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getNoiseEmission(const std::string &)` [static]
    // double
    // Vehicle::getNoiseEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getHarmonoise_NoiseEmissions() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_noise_emission(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { veh.get_harmonoise_noise_emissions() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getElectricityConsumption(const std::string &)` [static]
    // double
    // Vehicle::getElectricityConsumption(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::ELEC>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_electricity_consumption(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if Self::is_visible(veh.cast::<SUMOVehicle>()) { get_emissions_et(veh, PollutantsInterface_EmissionType::ELEC) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `int Vehicle::getPersonNumber(const std::string &)` [static]
    // int
    // Vehicle::getPersonNumber(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getPersonNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_person_number(vehID: &str) -> i32 {
        Helper::get_vehicle(vehID).get_person_number()
    }

    /// C++: `std::vector<std::string> Vehicle::getPersonIDList(const std::string &)` [static]
    // std::vector<std::string>
    // Vehicle::getPersonIDList(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getPersonIDList();
    // }
    #[allow(unused_variables)]
    pub fn get_person_id_list(vehID: &str) -> Vec<String> {
        Helper::get_vehicle(vehID).get_person_id_list()
    }

    /// C++: `std::pair<std::string, double> Vehicle::getLeader(const std::string &, double)` [static]
    // std::pair<std::string, double>
    // Vehicle::getLeader(const std::string& vehID, double dist) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh->isOnRoad()) {
    //         std::pair<const MSVehicle* const, double> leaderInfo = veh->getLeader(dist, false);
    //         const std::string leaderID = leaderInfo.first != nullptr ? leaderInfo.first->getID() : "";
    //         double gap = leaderInfo.second;
    //         if (leaderInfo.first != nullptr
    //                 && leaderInfo.first->getLane() != nullptr
    //                 && leaderInfo.first->getLane()->isInternal()
    //                 && veh->getLane() != nullptr
    //                 && (!veh->getLane()->isInternal()
    //                     || (veh->getLane()->getLinkCont().front()->getIndex() != leaderInfo.first->getLane()->getLinkCont().front()->getIndex()))) {
    //             // leader is a linkLeader (see MSLink::getLeaderInfo)
    //             // avoid internal gap values which may be negative (or -inf)
    //             gap = MAX2(0.0, gap);
    //         }
    //         return std::make_pair(leaderID, gap);
    //     } else {
    //         return std::make_pair("", -1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_leader(vehID: &str, dist: f64) -> (String, f64) {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() {
            // C++ MSBaseVehicle::getLeader is virtual; MSVehicle overrides it, so dispatch to the micro override
            let (leader, mut gap) = micro_veh(veh).get_leader(dist, false);
            let leaderID = if !leader.is_null() { leader.get_id().to_string() } else { String::new() };
            if !leader.is_null()
                && !leader.get_lane().is_null()
                && leader.get_lane().is_internal()
                && !veh.get_lane().is_null()
                && (!veh.get_lane().is_internal()
                    || (veh.get_lane().get_link_cont()[0].get_index() != leader.get_lane().get_link_cont()[0].get_index()))
            {
                // leader is a linkLeader (see MSLink::getLeaderInfo)
                // avoid internal gap values which may be negative (or -inf)
                gap = if 0.0 > gap { 0.0 } else { gap }; // MAX2(0.0, gap)
            }
            (leaderID, gap)
        } else {
            (String::new(), -1.0)
        }
    }

    /// C++: `std::pair<std::string, double> Vehicle::getFollower(const std::string &, double)` [static]
    // std::pair<std::string, double>
    // Vehicle::getFollower(const std::string& vehID, double dist) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh->isOnRoad()) {
    //         std::pair<const MSVehicle* const, double> leaderInfo = veh->getFollower(dist);
    //         return std::make_pair(
    //                    leaderInfo.first != nullptr ? leaderInfo.first->getID() : "",
    //                    leaderInfo.second);
    //     } else {
    //         return std::make_pair("", -1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_follower(vehID: &str, dist: f64) -> (String, f64) {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() {
            let (follower, gap) = veh.get_follower(dist);
            (if !follower.is_null() { follower.get_id().to_string() } else { String::new() }, gap)
        } else {
            (String::new(), -1.0)
        }
    }

    /// C++: `std::vector<libsumo::TraCIJunctionFoe> Vehicle::getJunctionFoes(const std::string &, double)` [static]
    // std::vector<TraCIJunctionFoe>
    // Vehicle::getJunctionFoes(const std::string& vehID, double dist) {
    //     std::vector<TraCIJunctionFoe> result;
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("getJunctionFoes not applicable for meso");
    //     } else if (veh->isOnRoad()) {
    //         if (dist == 0) {
    //             dist = veh->getCarFollowModel().brakeGap(veh->getSpeed()) + veh->getVehicleType().getMinGap();
    //         }
    //         const std::vector<const MSLane*> internalLanes;
    //         // distance to the end of the lane
    //         double curDist = -veh->getPositionOnLane();
    //         for (const MSLane* lane : veh->getUpcomingLanesUntil(dist)) {
    //             curDist += lane->getLength();
    //             if (lane->isInternal()) {
    //                 const MSLink* exitLink = lane->getLinkCont().front();
    //                 int foeIndex = 0;
    //                 const std::vector<MSLink::ConflictInfo>& conflicts = exitLink->getConflicts();
    //                 const MSJunctionLogic* logic = exitLink->getJunction()->getLogic();
    //                 for (const MSLane* foeLane : exitLink->getFoeLanes()) {
    //                     const MSLink::ConflictInfo& ci = conflicts[foeIndex];
    //                     if (ci.flag == MSLink::CONFLICT_NO_INTERSECTION) {
    //                         continue;
    //                     }
    //                     const double distBehindCrossing = ci.lengthBehindCrossing;
    //                     const MSLink* foeExitLink = foeLane->getLinkCont().front();
    //                     const double distToCrossing = curDist - distBehindCrossing;
    //                     const double foeDistBehindCrossing = ci.getFoeLengthBehindCrossing(foeExitLink);
    //                     for (auto item : foeExitLink->getApproaching()) {
    //                         const SUMOVehicle* foe = item.first;
    //                         TraCIJunctionFoe jf;
    //                         jf.foeId = foe->getID();
    //                         jf.egoDist = distToCrossing;
    //                         // approach information is from the start of the previous step
    //                         // but the foe vehicle then moved within that steop
    //                         const double prevFoeDist = SPEED2DIST(MSGlobals::gSemiImplicitEulerUpdate
    //                                                               ? foe->getSpeed()
    //                                                               : (foe->getSpeed() + foe->getPreviousSpeed()) / 2);
    //                         jf.foeDist = item.second.dist - foeDistBehindCrossing - prevFoeDist;
    //                         jf.egoExitDist = jf.egoDist + ci.conflictSize;
    //                         jf.foeExitDist = jf.foeDist + ci.getFoeConflictSize(foeExitLink);
    //                         jf.egoLane = lane->getID();
    //                         jf.foeLane = foeLane->getID();
    //                         jf.egoResponse = logic->getResponseFor(exitLink->getIndex()).test(foeExitLink->getIndex());
    //                         jf.foeResponse = logic->getResponseFor(foeExitLink->getIndex()).test(exitLink->getIndex());
    //                         result.push_back(jf);
    //                     }
    //                     foeIndex++;
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_junction_foes(vehID: &str, mut dist: f64) -> Vec<TraCIJunctionFoe> {
        let mut result: Vec<TraCIJunctionFoe> = Vec::new();
        // shortcut: meso warning unreachable, micro_veh cannot tell MEVehicle apart, upgrade with a vehicle type tag (see micro_veh)
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if veh.is_on_road() {
            if dist == 0.0 {
                dist = veh.get_car_follow_model().brake_gap(veh.get_speed()) + veh.get_vehicle_type().get_min_gap();
            }
            // distance to the end of the lane
            let mut curDist = -veh.get_position_on_lane();
            for lane in veh.get_upcoming_lanes_until(dist) {
                curDist += lane.get_length();
                if lane.is_internal() {
                    let exitLink = lane.get_link_cont()[0];
                    let mut foeIndex: usize = 0;
                    let logic = exitLink.get_junction().get_logic();
                    for foeLane in exitLink.get_foe_lanes().iter().copied() {
                        let ci = &exitLink.get_conflicts()[foeIndex];
                        // C++ `continue` skips the `foeIndex++` below, so a skipped conflict does not advance the index
                        if ci.flag == MSLink_ConflictFlag::CONFLICT_NO_INTERSECTION {
                            continue;
                        }
                        let distBehindCrossing = ci.lengthBehindCrossing;
                        let foeExitLink = foeLane.get_link_cont()[0];
                        let distToCrossing = curDist - distBehindCrossing;
                        let foeDistBehindCrossing = ci.get_foe_length_behind_crossing(foeExitLink);
                        // determinism: C++ std::map is keyed by SUMOVehicle*, BTreeMap<Ptr<SUMOVehicle>> iterates in the same address order
                        for (foe, item) in foeExitLink.myApproachingVehicles.iter() {
                            // approach information is from the start of the previous step
                            // but the foe vehicle then moved within that steop
                            let prevFoeDist = speed2dist(if MSGlobals::semi_implicit_euler_update() {
                                foe.get_speed()
                            } else {
                                (foe.get_speed() + foe.get_previous_speed()) / 2.0
                            });
                            let egoDist = distToCrossing;
                            let foeDist = item.dist - foeDistBehindCrossing - prevFoeDist;
                            result.push(TraCIJunctionFoe {
                                foeId: foe.get_id().to_string(),
                                egoDist,
                                foeDist,
                                egoExitDist: egoDist + ci.conflictSize,
                                foeExitDist: foeDist + ci.get_foe_conflict_size(foeExitLink),
                                egoLane: lane.get_id().to_string(),
                                foeLane: foeLane.get_id().to_string(),
                                egoResponse: logic.get_response_for(exitLink.get_index()).test(foeExitLink.get_index()),
                                foeResponse: logic.get_response_for(foeExitLink.get_index()).test(exitLink.get_index()),
                            });
                        }
                        foeIndex += 1;
                    }
                }
            }
        }
        result
    }

    /// C++: `double Vehicle::getWaitingTime(const std::string &)` [static]
    // double
    // Vehicle::getWaitingTime(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicle(vehID)->getWaitingTime());
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(vehID: &str) -> f64 {
        steps2time(Helper::get_vehicle(vehID).get_waiting_time(false))
    }

    /// C++: `double Vehicle::getAccumulatedWaitingTime(const std::string &)` [static]
    // double
    // Vehicle::getAccumulatedWaitingTime(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getAccumulatedWaitingSeconds(), INVALID_DOUBLE_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_accumulated_waiting_time(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        micro_veh(veh).get_accumulated_waiting_seconds()
    }

    /// C++: `double Vehicle::getAdaptedTraveltime(const std::string &, double, const std::string &)` [static]
    // double
    // Vehicle::getAdaptedTraveltime(const std::string& vehID, double time, const std::string& edgeID) {
    //     MSEdge* edge = Helper::getEdge(edgeID);
    //     double value = INVALID_DOUBLE_VALUE;
    //     Helper::getVehicle(vehID)->getWeightsStorage().retrieveExistingTravelTime(edge, time, value);
    //     return value;
    // }
    #[allow(unused_variables)]
    pub fn get_adapted_traveltime(vehID: &str, time: f64, edgeID: &str) -> f64 {
        let edge = Helper::get_edge(edgeID);
        let mut value = INVALID_DOUBLE_VALUE;
        Helper::get_vehicle(vehID).get_weights_storage().retrieve_existing_travel_time(edge, time, &mut value);
        value
    }

    /// C++: `double Vehicle::getEffort(const std::string &, double, const std::string &)` [static]
    // double
    // Vehicle::getEffort(const std::string& vehID, double time, const std::string& edgeID) {
    //     MSEdge* edge = Helper::getEdge(edgeID);
    //     double value = INVALID_DOUBLE_VALUE;
    //     Helper::getVehicle(vehID)->getWeightsStorage().retrieveExistingEffort(edge, time, value);
    //     return value;
    // }
    #[allow(unused_variables)]
    pub fn get_effort(vehID: &str, time: f64, edgeID: &str) -> f64 {
        let edge = Helper::get_edge(edgeID);
        let mut value = INVALID_DOUBLE_VALUE;
        Helper::get_vehicle(vehID).get_weights_storage().retrieve_existing_effort(edge, time, &mut value);
        value
    }

    /// C++: `bool Vehicle::isRouteValid(const std::string &)` [static]
    // bool
    // Vehicle::isRouteValid(const std::string& vehID) {
    //     std::string msg;
    //     return Helper::getVehicle(vehID)->hasValidRoute(msg);
    // }
    #[allow(unused_variables)]
    pub fn is_route_valid(vehID: &str) -> bool {
        let mut msg = String::new();
        let veh = Helper::get_vehicle(vehID);
        // C++ hasValidRoute(msg) with the default route == nullptr, i.e. myRoute
        veh.has_valid_route(&mut msg, veh.get_route_ptr())
    }

    /// C++: `std::vector<std::string> Vehicle::getRoute(const std::string &)` [static]
    // std::vector<std::string>
    // Vehicle::getRoute(const std::string& vehID) {
    //     std::vector<std::string> result;
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     const MSRoute& r = veh->getRoute();
    //     for (MSRouteIterator i = r.begin(); i != r.end(); ++i) {
    //         result.push_back((*i)->getID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_route(vehID: &str) -> Vec<String> {
        let mut result = Vec::new();
        let veh = Helper::get_vehicle(vehID);
        let r = veh.get_route();
        for e in r.get_edges() {
            result.push(e.get_id().to_string());
        }
        result
    }

    /// C++: `int Vehicle::getSignals(const std::string &)` [static]
    // int
    // Vehicle::getSignals(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getSignals(), MSVehicle::VEH_SIGNAL_NONE);
    // }
    #[allow(unused_variables)]
    pub fn get_signals(vehID: &str) -> i32 {
        let veh = Helper::get_vehicle(vehID);
        micro_veh(veh).get_signals()
    }

    /// C++: `std::vector<libsumo::TraCIBestLanesData> Vehicle::getBestLanes(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_best_lanes(vehID: &str) -> Vec<TraCIBestLanesData> {
        let mut result = Vec::new();
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if !veh.is_null() && veh.is_on_road() {
            for lq in veh.get_best_lanes().iter() {
                let mut bld = TraCIBestLanesData { laneID: lq.lane.get_id().to_string(), length: lq.length, occupation: lq.nextOccupation, bestLaneOffset: lq.bestLaneOffset, allowsContinuation: lq.allowsContinuation, continuationLanes: Vec::new() };
                for lane in lq.bestContinuations.iter() {
                    if !lane.is_null() {
                        bld.continuationLanes.push(lane.get_id().to_string());
                    }
                }
                result.push(bld);
            }
        }
        result
    }

    /// C++: `std::vector<libsumo::TraCINextTLSData> Vehicle::getNextTLS(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_next_tls(vehID: &str) -> Vec<TraCINextTLSData> {
        let mut result: Vec<TraCINextTLSData> = Vec::new();
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        if !veh.is_null() {
            let mut view: i32 = 1;
            let mut seen = veh.get_edge().get_length() - veh.get_position_on_lane();
            if vehicle.is_on_road() {
                let mut lane = veh.get_lane();
                let bestLaneConts = veh.get_best_lanes_continuation_2(lane).clone();
                seen = lane.get_length() - veh.get_position_on_lane();
                let mut linkIt = MSLane::succ_link_sec(&*veh, view, &*lane, &bestLaneConts);
                while !lane.is_link_end(&mut linkIt) {
                    let link = lane.get_link_cont()[linkIt];
                    if !lane.get_edge().is_internal() && link.is_tls_controlled() {
                        result.push(TraCINextTLSData { id: link.get_tl_logic().get_id().to_string(), tlIndex: link.get_tl_index(), dist: seen, state: link.get_state() as i8 });
                    }
                    lane = link.get_via_lane_or_lane();
                    if !lane.get_edge().is_internal() {
                        view += 1;
                    }
                    seen += lane.get_length();
                    linkIt = MSLane::succ_link_sec(&*veh, view, &*lane, &bestLaneConts);
                }
            }
            // consider edges beyond bestLanes
            let edges = veh.get_route().get_edges().clone();
            let curIdx = *veh.get_current_route_edge() as i32;
            let remainingEdges = edges.len() as i32 - curIdx - view;
            for i in 0..remainingEdges {
                let prev = edges[(curIdx + view + i - 1) as usize];
                let next = edges[(curIdx + view + i) as usize];
                let allowed = prev.allowed_lanes(&*next, veh.get_v_class(), false);
                if !allowed.is_null() && allowed.len() != 0 {
                    for link in allowed[0].get_link_cont().iter() {
                        if std::ptr::eq(&*link.get_lane().get_edge(), &*next) {
                            if link.is_tls_controlled() {
                                result.push(TraCINextTLSData { id: link.get_tl_logic().get_id().to_string(), tlIndex: link.get_tl_index(), dist: seen, state: link.get_state() as i8 });
                            }
                            seen += next.get_length() + link.get_internal_lengths_after();
                            break;
                        }
                    }
                } else {
                    // invalid route, cannot determine nextTLS
                    break;
                }
            }
        } else {
            write_warning("getNextTLS not yet implemented for meso");
        }
        result
    }

    /// C++: `std::vector<libsumo::TraCINextStopData> Vehicle::getNextStops(const std::string &)` [static]
    // std::vector<libsumo::TraCINextStopData>
    // Vehicle::getNextStops(const std::string& vehID) {
    //     return getStops(vehID, 0);
    // }
    #[allow(unused_variables)]
    pub fn get_next_stops(vehID: &str) -> Vec<TraCINextStopData> {
        Self::get_stops(vehID, 0)
    }

    /// C++: `std::vector<libsumo::TraCIConnection> Vehicle::getNextLinks(const std::string &)` [static]
    // std::vector<libsumo::TraCIConnection>
    // Vehicle::getNextLinks(const std::string& vehID) {
    //     std::vector<libsumo::TraCIConnection> result;
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (!vehicle->isOnRoad()) {
    //         return result;
    //     }
    //     if (veh != nullptr) {
    //         const MSLane* lane = veh->getLane();
    //         const std::vector<MSLane*>& bestLaneConts = veh->getBestLanesContinuation(lane);
    //         int view = 1;
    //         const SUMOTime currTime = MSNet::getInstance()->getCurrentTimeStep();
    //         std::vector<MSLink*>::const_iterator linkIt = MSLane::succLinkSec(*veh, view, *lane, bestLaneConts);
    //         while (!lane->isLinkEnd(linkIt)) {
    //             if (!lane->getEdge().isInternal()) {
    //                 const MSLink* link = (*linkIt);
    //                 const std::string approachedLane = link->getLane() != nullptr ? link->getLane()->getID() : "";
    //                 const bool hasPrio = link->havePriority();
    //                 const double speed = MIN2(lane->getSpeedLimit(), link->getLane()->getSpeedLimit());
    //                 const bool isOpen = link->opened(currTime, speed, speed, veh->getLength(),
    //                                                  veh->getImpatience(), veh->getVehicleType().getCarFollowModel().getMaxDecel(),
    //                                                  veh->getWaitingTime(), veh->getLateralPositionOnLane(), nullptr, false, veh);
    //                 const bool hasFoe = link->hasApproachingFoe(currTime, currTime, 0, SUMOVTypeParameter::getDefaultDecel());
    //                 const std::string approachedInternal = link->getViaLane() != nullptr ? link->getViaLane()->getID() : "";
    //                 const std::string state = SUMOXMLDefinitions::LinkStates.getString(link->getState());
    //                 const std::string direction = SUMOXMLDefinitions::LinkDirections.getString(link->getDirection());
    //                 const double length = link->getLength();
    //                 result.push_back(TraCIConnection(approachedLane, hasPrio, isOpen, hasFoe, approachedInternal, state, direction, length));
    //             }
    //             lane = (*linkIt)->getViaLaneOrLane();
    //             if (!lane->getEdge().isInternal()) {
    //                 view++;
    //             }
    //             linkIt = MSLane::succLinkSec(*veh, view, *lane, bestLaneConts);
    //         }
    //         // consider edges beyond bestLanes
    //         const int remainingEdges = (int)(veh->getRoute().end() - veh->getCurrentRouteEdge()) - view;
    //         for (int i = 0; i < remainingEdges; i++) {
    //             const MSEdge* prev = *(veh->getCurrentRouteEdge() + view + i - 1);
    //             const MSEdge* next = *(veh->getCurrentRouteEdge() + view + i);
    //             const std::vector<MSLane*>* allowed = prev->allowedLanes(*next, veh->getVClass());
    //             if (allowed != nullptr && allowed->size() != 0) {
    //                 for (const MSLink* const link : allowed->front()->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == next) {
    //                         const std::string approachedLane = link->getLane() != nullptr ? link->getLane()->getID() : "";
    //                         const bool hasPrio = link->havePriority();
    //                         const double speed = MIN2(lane->getSpeedLimit(), link->getLane()->getSpeedLimit());
    //                         const bool isOpen = link->opened(currTime, speed, speed, veh->getLength(),
    //                                                          veh->getImpatience(), veh->getVehicleType().getCarFollowModel().getMaxDecel(),
    //                                                          veh->getWaitingTime(), veh->getLateralPositionOnLane(), nullptr, false, veh);
    //                         const bool hasFoe = link->hasApproachingFoe(currTime, currTime, 0, SUMOVTypeParameter::getDefaultDecel());
    //                         const std::string approachedInternal = link->getViaLane() != nullptr ? link->getViaLane()->getID() : "";
    //                         const std::string state = SUMOXMLDefinitions::LinkStates.getString(link->getState());
    //                         const std::string direction = SUMOXMLDefinitions::LinkDirections.getString(link->getDirection());
    //                         const double length = link->getLength();
    //                         result.push_back(TraCIConnection(approachedLane, hasPrio, isOpen, hasFoe, approachedInternal, state, direction, length));
    //                     }
    //                 }
    //             } else {
    //                 // invalid route, cannot determine nextTLS
    //                 break;
    //             }
    //         }
    //     } else {
    //         WRITE_WARNING("getNextLinks not yet implemented for meso");
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_next_links(vehID: &str) -> Vec<TraCIConnection> {
        let vehicle = Helper::get_vehicle(vehID);
        if !vehicle.is_on_road() {
            return Vec::new();
        }
        let mut result: Vec<TraCIConnection> = Vec::new();
        let veh = micro_veh(vehicle);
        if !veh.is_null() {
            let lane = veh.get_lane();
            let bestLaneConts = veh.get_best_lanes_continuation_2(lane).clone();
            let mut view: i32 = 1;
            let currTime = MSNet::get_instance().get_current_time_step();
            let mut linkIt = MSLane::succ_link_sec(&*veh, view, &*lane, &bestLaneConts);
            let mut lane = lane;
            while !lane.is_link_end(&mut linkIt) {
                let link = lane.get_link_cont()[linkIt];
                if !lane.get_edge().is_internal() {
                    result.push(next_link_connection(veh, lane, link, currTime));
                }
                lane = link.get_via_lane_or_lane();
                if !lane.get_edge().is_internal() {
                    view += 1;
                }
                linkIt = MSLane::succ_link_sec(&*veh, view, &*lane, &bestLaneConts);
            }
            // consider edges beyond bestLanes
            let edges = veh.get_route().get_edges().clone();
            let curIdx = *veh.get_current_route_edge() as i32;
            let remainingEdges = edges.len() as i32 - curIdx - view;
            for i in 0..remainingEdges {
                let prev = edges[(curIdx + view + i - 1) as usize];
                let next = edges[(curIdx + view + i) as usize];
                let allowed = prev.allowed_lanes(&*next, veh.get_v_class(), false);
                if !allowed.is_null() && allowed.len() != 0 {
                    for link in allowed[0].get_link_cont().iter() {
                        if std::ptr::eq(&*link.get_lane().get_edge(), &*next) {
                            result.push(next_link_connection(veh, lane, *link, currTime));
                        }
                    }
                } else {
                    // invalid route, cannot determine nextTLS
                    break;
                }
            }
        } else {
            write_warning("getNextLinks not yet implemented for meso");
        }
        result
    }

    /// C++: `std::vector<libsumo::TraCINextStopData> Vehicle::getStops(const std::string &, int)` [static]
    // std::vector<libsumo::TraCINextStopData>
    // Vehicle::getStops(const std::string& vehID, int limit) {
    //     std::vector<TraCINextStopData> result;
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     if (limit < 0) {
    //         // return past stops up to the given limit
    //         const StopParVector& pastStops = vehicle->getPastStops();
    //         const int n = (int)pastStops.size();
    //         for (int i = MAX2(0, n + limit); i < n; i++) {
    //             result.push_back(Helper::buildStopData(pastStops[i]));
    //         }
    //     } else {
    //         for (const MSStop& stop : vehicle->getStops()) {
    //             if (!stop.pars.collision) {
    //                 TraCINextStopData nsd = Helper::buildStopData(stop.pars);
    //                 nsd.duration = STEPS2TIME(stop.duration);
    //                 result.push_back(nsd);
    //                 if (limit > 0 && (int)result.size() >= limit) {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_stops(vehID: &str, limit: i32) -> Vec<TraCINextStopData> {
        let mut result = Vec::new();
        let vehicle = Helper::get_vehicle(vehID);
        if limit < 0 {
            let pastStops = vehicle.get_past_stops();
            let n = pastStops.len() as i32;
            for i in std::cmp::max(0, n + limit)..n {
                result.push(Helper::build_stop_data(&pastStops[i as usize]));
            }
        } else {
            for stop in vehicle.get_stops().iter() {
                if !stop.pars.collision {
                    let mut nsd = Helper::build_stop_data(&stop.pars);
                    nsd.duration = steps2time(stop.duration);
                    result.push(nsd);
                    if limit > 0 && result.len() as i32 >= limit {
                        break;
                    }
                }
            }
        }
        result
    }

    /// C++: `int Vehicle::getStopState(const std::string &)` [static]
    // int
    // Vehicle::getStopState(const std::string& vehID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("getStopState not yet implemented for meso");
    //         return 0;
    //     }
    //     int result = 0;
    //     if (veh->isStopped()) {
    //         const MSStop& stop = veh->getNextStop();
    //         result = stop.getStateFlagsOld();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_stop_state(vehID: &str) -> i32 {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let mut result = 0;
        if veh.is_stopped() {
            let stop = veh.get_next_stop();
            result = stop.get_state_flags_old();
        }
        result
    }

    /// C++: `std::string Vehicle::getStopParameter(const std::string &, int, const std::string &, bool)` [static]
    // std::string
    // Vehicle::getStopParameter(const std::string& vehID, int nextStopIndex, const std::string& param, bool customParam) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     try {
    //         if (nextStopIndex >= (int)vehicle->getStops().size() || (nextStopIndex < 0 && -nextStopIndex > (int)vehicle->getPastStops().size())) {
    //             throw ProcessError("Invalid stop index " + toString(nextStopIndex)
    //                                + " (has " + toString(vehicle->getPastStops().size()) + " past stops and " + toString(vehicle->getStops().size()) + " remaining stops)");
    //
    //         }
    //         const SUMOVehicleParameter::Stop& pars = (nextStopIndex >= 0
    //                 ? vehicle->getStop(nextStopIndex).pars
    //                 : vehicle->getPastStops()[(int)vehicle->getPastStops().size() + nextStopIndex]);
    //         if (customParam) {
    //             // custom user parameter
    //             return pars.getParameter(param, "");
    //         }
    //
    //         if (param == toString(SUMO_ATTR_EDGE)) {
    //             return pars.edge;
    //         } else if (param == toString(SUMO_ATTR_LANE)) {
    //             return toString(SUMOXMLDefinitions::getIndexFromLane(pars.lane));
    //         } else if (param == toString(SUMO_ATTR_BUS_STOP)
    //                    || param == toString(SUMO_ATTR_TRAIN_STOP)) {
    //             return pars.busstop;
    //         } else if (param == toString(SUMO_ATTR_CONTAINER_STOP)) {
    //             return pars.containerstop;
    //         } else if (param == toString(SUMO_ATTR_CHARGING_STATION)) {
    //             return pars.chargingStation;
    //         } else if (param == toString(SUMO_ATTR_PARKING_AREA)) {
    //             return pars.parkingarea;
    //         } else if (param == toString(SUMO_ATTR_STARTPOS)) {
    //             return toString(pars.startPos);
    //         } else if (param == toString(SUMO_ATTR_ENDPOS)) {
    //             return toString(pars.endPos);
    //         } else if (param == toString(SUMO_ATTR_POSITION_LAT)) {
    //             return toString(pars.posLat == INVALID_DOUBLE ? INVALID_DOUBLE_VALUE : pars.posLat);
    //         } else if (param == toString(SUMO_ATTR_ARRIVAL)) {
    //             return pars.arrival < 0 ? "-1" : time2string(pars.arrival);
    //         } else if (param == toString(SUMO_ATTR_DURATION)) {
    //             return pars.duration < 0 ? "-1" : time2string(pars.duration);
    //         } else if (param == toString(SUMO_ATTR_UNTIL)) {
    //             return pars.until < 0 ? "-1" : time2string(pars.until);
    //         } else if (param == toString(SUMO_ATTR_EXTENSION)) {
    //             return pars.extension < 0 ? "-1" : time2string(pars.extension);
    //         } else if (param == toString(SUMO_ATTR_INDEX)) {
    //             return toString(nextStopIndex + (int)vehicle->getPastStops().size());
    //         } else if (param == toString(SUMO_ATTR_PARKING)) {
    //             return toString(pars.parking);
    //         } else if (param == toString(SUMO_ATTR_TRIGGERED)) {
    //             return joinToString(pars.getTriggers(), " ");
    //         } else if (param == toString(SUMO_ATTR_EXPECTED)) {
    //             return joinToString(pars.awaitedPersons, " ");
    //         } else if (param == toString(SUMO_ATTR_EXPECTED_CONTAINERS)) {
    //             return joinToString(pars.awaitedContainers, " ");
    //         } else if (param == toString(SUMO_ATTR_PERMITTED)) {
    //             return joinToString(pars.permitted, " ");
    //         } else if (param == toString(SUMO_ATTR_ACTTYPE)) {
    //             return pars.actType;
    //         } else if (param == toString(SUMO_ATTR_TRIP_ID)) {
    //             return pars.tripId;
    //         } else if (param == toString(SUMO_ATTR_SPLIT)) {
    //             return pars.split;
    //         } else if (param == toString(SUMO_ATTR_JOIN)) {
    //             return pars.join;
    //         } else if (param == toString(SUMO_ATTR_LINE)) {
    //             return pars.line;
    //         } else if (param == toString(SUMO_ATTR_SPEED)) {
    //             return toString(pars.speed);
    //         } else if (param == toString(SUMO_ATTR_STARTED)) {
    //             return pars.started < 0 ? "-1" : time2string(pars.started);
    //         } else if (param == toString(SUMO_ATTR_ENDED)) {
    //             return pars.ended < 0 ? "-1" : time2string(pars.ended);
    //         } else if (param == toString(SUMO_ATTR_ONDEMAND)) {
    //             return toString(pars.onDemand);
    //         } else if (param == toString(SUMO_ATTR_JUMP)) {
    //             return pars.jump < 0 ? "-1" : time2string(pars.jump);
    //         } else if (param == toString(SUMO_ATTR_JUMP_UNTIL)) {
    //             return pars.jumpUntil < 0 ? "-1" : time2string(pars.jumpUntil);
    //         } else {
    //             throw ProcessError(TLF("Unsupported parameter '%'", param));
    //         }
    //     } catch (ProcessError& e) {
    //         throw TraCIException("Could not get stop parameter for vehicle '" + vehID + "' (" + e.what() + ")");
    //     }
    // }
    #[allow(unused_variables)]
    /// Joins a std::set<std::string> with spaces (C++ joinToString(set, " "))
fn join_set(s: &BTreeSet<String>) -> String {
    s.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(" ")
}

pub fn get_stop_parameter(vehID: &str, nextStopIndex: i32, param: &str, customParam: bool) -> String {
        let vehicle = Helper::get_vehicle(vehID);
        if nextStopIndex >= vehicle.get_stops().len() as i32 || (nextStopIndex < 0 && -nextStopIndex > vehicle.get_past_stops().len() as i32) {
            panic!("{}", format!("Could not get stop parameter for vehicle '{}' ({})", vehID,
                format!("Invalid stop index {} (has {} past stops and {} remaining stops)", nextStopIndex, vehicle.get_past_stops().len(), vehicle.get_stops().len())));
        }
        let pars: &SUMOVehicleParameter_Stop = if nextStopIndex >= 0 {
            &vehicle.m().get_stop(nextStopIndex).pars
        } else {
            &vehicle.get_past_stops()[(vehicle.get_past_stops().len() as i32 + nextStopIndex) as usize]
        };
        if customParam {
            // custom user parameter
            return pars.get_parameter(param, "");
        }
        match param {
            "edge" => pars.edge.clone(),
            "lane" => SUMOXMLDefinitions::get_index_from_lane(&pars.lane).unwrap_or_else(|e| panic!("{}", e)).to_string(),
            "busStop" | "trainStop" => pars.busstop.clone(),
            "containerStop" => pars.containerstop.clone(),
            "chargingStation" => pars.chargingStation.clone(),
            "parkingArea" => pars.parkingarea.clone(),
            "startPos" => to_string(&pars.startPos),
            "endPos" => to_string(&pars.endPos),
            "posLat" => to_string(&if pars.posLat == INVALID_DOUBLE { INVALID_DOUBLE_VALUE } else { pars.posLat }),
            "arrival" => if pars.arrival < 0 { "-1".to_string() } else { time2string(pars.arrival) },
            "duration" => if pars.duration < 0 { "-1".to_string() } else { time2string(pars.duration) },
            "until" => if pars.until < 0 { "-1".to_string() } else { time2string(pars.until) },
            "extension" => if pars.extension < 0 { "-1".to_string() } else { time2string(pars.extension) },
            "index" => to_string(&(nextStopIndex + vehicle.get_past_stops().len() as i32)),
            "parking" => to_string(&(pars.parking as i32)),
            "triggered" => pars.get_triggers().join(" "),
            "expected" => Self::join_set(&pars.awaitedPersons),
            "expectedContainers" => Self::join_set(&pars.awaitedContainers),
            "permitted" => Self::join_set(&pars.permitted),
            "actType" => pars.actType.clone(),
            "tripId" => pars.tripId.clone(),
            "split" => pars.split.clone(),
            "join" => pars.join.clone(),
            "line" => pars.line.clone(),
            "speed" => to_string(&pars.speed),
            "started" => if pars.started < 0 { "-1".to_string() } else { time2string(pars.started) },
            "ended" => if pars.ended < 0 { "-1".to_string() } else { time2string(pars.ended) },
            "onDemand" => to_string(&pars.onDemand),
            "jump" => if pars.jump < 0 { "-1".to_string() } else { time2string(pars.jump) },
            "jumpUntil" => if pars.jumpUntil < 0 { "-1".to_string() } else { time2string(pars.jumpUntil) },
            _ => panic!("{}", format!("Could not get stop parameter for vehicle '{}' ({})", vehID, format!("Unsupported parameter '{}'", param))),
        }
    }

    /// C++: `double Vehicle::getDistance(const std::string &)` [static]
    // double
    // Vehicle::getDistance(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh->hasDeparted()) {
    //         return veh->getOdometer();
    //     } else {
    //         return INVALID_DOUBLE_VALUE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_distance(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.has_departed() { veh.get_odometer() } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `double Vehicle::getDrivingDistance(const std::string &, const std::string &, double, int)` [static]
    // double
    // Vehicle::getDrivingDistance(const std::string& vehID, const std::string& edgeID, double pos, int laneIndex) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (veh->isOnRoad()) {
    //         const MSLane* lane = microVeh != nullptr ? veh->getLane() : veh->getEdge()->getLanes()[0];
    //         double distance = veh->getRoute().getDistanceBetween(veh->getPositionOnLane(), pos,
    //                           lane, Helper::getLaneChecking(edgeID, laneIndex, pos), veh->getRoutePosition());
    //         if (distance == std::numeric_limits<double>::max()) {
    //             return INVALID_DOUBLE_VALUE;
    //         }
    //         return distance;
    //     } else {
    //         return INVALID_DOUBLE_VALUE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_driving_distance(vehID: &str, edgeID: &str, pos: f64, laneIndex: i32) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        let microVeh = micro_veh(veh);
        if veh.is_on_road() {
            let lane = microVeh.get_lane();
            let distance = veh.get_route().get_distance_between(veh.get_position_on_lane(), pos,
                               lane, Helper::get_lane_checking(edgeID, laneIndex, pos), veh.get_route_position());
            if distance == f64::MAX {
                return INVALID_DOUBLE_VALUE;
            }
            distance
        } else {
            INVALID_DOUBLE_VALUE
        }
    }

    /// C++: `double Vehicle::getDrivingDistance2D(const std::string &, double, double)` [static]
    // double
    // Vehicle::getDrivingDistance2D(const std::string& vehID, double x, double y) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh == nullptr) {
    //         return INVALID_DOUBLE_VALUE;
    //     }
    //     if (veh->isOnRoad()) {
    //         MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //         const MSLane* lane = microVeh != nullptr ? veh->getLane() : veh->getEdge()->getLanes()[0];
    //         std::pair<MSLane*, double> roadPos = Helper::convertCartesianToRoadMap(Position(x, y), veh->getVehicleType().getVehicleClass());
    //         double distance = veh->getRoute().getDistanceBetween(veh->getPositionOnLane(), roadPos.second,
    //                           lane, roadPos.first, veh->getRoutePosition());
    //         if (distance == std::numeric_limits<double>::max()) {
    //             return INVALID_DOUBLE_VALUE;
    //         }
    //         return distance;
    //     } else {
    //         return INVALID_DOUBLE_VALUE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_driving_distance2_d(vehID: &str, x: f64, y: f64) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() {
            let lane = micro_veh(veh).get_lane();
            let mut roadPos = Helper::convert_cartesian_to_road_map(&Position::new(x, y), veh.get_vehicle_type().get_vehicle_class());
            let distance = veh.get_route().get_distance_between(veh.get_position_on_lane(), roadPos.1,
                               lane, roadPos.0, veh.get_route_position());
            if distance == f64::MAX {
                return INVALID_DOUBLE_VALUE;
            }
            distance
        } else {
            INVALID_DOUBLE_VALUE
        }
    }

    /// C++: `double Vehicle::getReferenceDistance(const std::string &)` [static]
    // double
    // Vehicle::getReferenceDistance(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     if (veh->hasDeparted()) {
    //         double lanePos = veh->getPositionOnLane();
    //         MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //         if (microVeh != nullptr && microVeh->getLane()->isInternal()) {
    //             lanePos = microVeh->getRoute().getDistanceBetween(0., lanePos, microVeh->getEdge()->getLanes()[0],
    //                       microVeh->getLane(), microVeh->getRoutePosition());
    //         }
    //         return veh->getEdge()->getDistanceAt(lanePos);
    //     } else {
    //         return INVALID_DOUBLE_VALUE;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_reference_distance(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.has_departed() {
            let mut lanePos = veh.get_position_on_lane();
            let microVeh = micro_veh(veh);
            if microVeh.get_lane().is_internal() {
                lanePos = microVeh.get_route().get_distance_between(0., lanePos, microVeh.get_edge().get_lanes()[0],
                          microVeh.get_lane(), microVeh.get_route_position());
            }
            veh.get_edge().get_distance_at(lanePos)
        } else {
            INVALID_DOUBLE_VALUE
        }
    }

    /// C++: `double Vehicle::getAllowedSpeed(const std::string &)` [static]
    // double
    // Vehicle::getAllowedSpeed(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLane()->getVehicleMaxSpeed(veh), veh->getEdge()->getVehicleMaxSpeed(veh)) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_allowed_speed(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        if veh.is_on_road() { micro_veh(veh).get_lane().get_vehicle_max_speed(veh.cast::<SUMOTrafficObject>()) } else { INVALID_DOUBLE_VALUE }
    }

    /// C++: `int Vehicle::getSpeedMode(const std::string &)` [static]
    // int
    // Vehicle::getSpeedMode(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getInfluencer().getSpeedMode(), INVALID_INT_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_mode(vehID: &str) -> i32 {
        let veh = Helper::get_vehicle(vehID);
        micro_veh(veh).m().get_influencer().get_speed_mode()
    }

    /// C++: `int Vehicle::getLaneChangeMode(const std::string &)` [static]
    // int
    // Vehicle::getLaneChangeMode(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getInfluencer().getLaneChangeMode(), INVALID_INT_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_change_mode(vehID: &str) -> i32 {
        let veh = Helper::get_vehicle(vehID);
        micro_veh(veh).m().get_influencer().get_lane_change_mode()
    }

    /// C++: `int Vehicle::getRoutingMode(const std::string &)` [static]
    // int
    // Vehicle::getRoutingMode(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getRoutingMode();
    // }
    #[allow(unused_variables)]
    pub fn get_routing_mode(vehID: &str) -> i32 {
        Helper::get_vehicle(vehID).get_routing_mode()
    }

    /// C++: `std::string Vehicle::getLine(const std::string &)` [static]
    // std::string
    // Vehicle::getLine(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getParameter().line;
    // }
    #[allow(unused_variables)]
    pub fn get_line(vehID: &str) -> String {
        Helper::get_vehicle(vehID).get_parameter().line.clone()
    }

    /// C++: `std::vector<std::string> Vehicle::getVia(const std::string &)` [static]
    // std::vector<std::string>
    // Vehicle::getVia(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getParameter().via;
    // }
    #[allow(unused_variables)]
    pub fn get_via(vehID: &str) -> Vec<String> {
        Helper::get_vehicle(vehID).get_parameter().via.clone()
    }

    /// C++: `std::pair<int, int> Vehicle::getLaneChangeState(const std::string &, int)` [static]
    // std::pair<int, int>
    // Vehicle::getLaneChangeState(const std::string& vehID, int direction) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     auto undefined = std::make_pair((int)LCA_UNKNOWN, (int)LCA_UNKNOWN);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLaneChangeModel().getSavedState(direction), undefined) : undefined;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_change_state(vehID: &str, direction: i32) -> (i32, i32) {
        let veh = Helper::get_vehicle(vehID);
        let undefined = (LCA_UNKNOWN as i32, LCA_UNKNOWN as i32);
        if veh.is_on_road() { *micro_veh(veh).m().get_lane_change_model().get_saved_state(direction) } else { undefined }
    }

    /// C++: `double Vehicle::getLastActionTime(const std::string &)` [static]
    // double Vehicle::getLastActionTime(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (microVeh != nullptr) {
    //         return STEPS2TIME(microVeh->getLastActionTime());
    //     } else {
    //         MEVehicle* mesoVeh = dynamic_cast<MEVehicle*>(veh);
    //         return STEPS2TIME(mesoVeh->getEventTime());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_last_action_time(vehID: &str) -> f64 {
        let veh = Helper::get_vehicle(vehID);
        steps2time(micro_veh(veh).get_last_action_time())
    }

    /// C++: `std::vector<std::pair<std::string, double>> Vehicle::getNeighbors(const std::string &, const int)` [static]
    // std::vector<std::pair<std::string, double> >
    // Vehicle::getNeighbors(const std::string& vehID, const int mode) {
    //     int dir = (1 & mode) != 0 ? -1 : 1;
    //     bool queryLeaders = (2 & mode) != 0;
    //     bool blockersOnly = (4 & mode) != 0;
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     std::vector<std::pair<std::string, double> > result;
    //     if (veh == nullptr) {
    //         return result;
    //     }
    // #ifdef DEBUG_NEIGHBORS
    //     if (DEBUG_COND) {
    //         std::cout << "getNeighbors() for veh '" << vehID << "': dir=" << dir
    //                   << ", queryLeaders=" << queryLeaders
    //                   << ", blockersOnly=" << blockersOnly << std::endl;
    //     }
    // #endif
    //     if (veh->getLaneChangeModel().isOpposite()) {
    //         // getParallelLane works relative to lane forward direction
    //         dir *= -1;
    //     }
    //
    //     MSLane* targetLane = veh->getLane()->getParallelLane(dir);
    //     if (targetLane == nullptr) {
    //         return result;
    //     }
    //     // need to recompute leaders and followers (#8119)
    //     const bool opposite = &veh->getLane()->getEdge() != &targetLane->getEdge();
    //     MSLeaderDistanceInfo neighbors(targetLane->getWidth(), nullptr, 0.);
    //     if (queryLeaders) {
    //         if (opposite) {
    //             double pos = targetLane->getOppositePos(veh->getPositionOnLane());
    //             neighbors = targetLane->getFollowersOnConsecutive(veh, pos, true);
    //         } else {
    //             targetLane->addLeaders(veh, veh->getPositionOnLane(), neighbors);
    //         }
    //     } else {
    //         if (opposite) {
    //             double pos = targetLane->getOppositePos(veh->getPositionOnLane());
    //             targetLane->addLeaders(veh, pos, neighbors);
    //             neighbors.fixOppositeGaps(true);
    //         } else {
    //             neighbors = targetLane->getFollowersOnConsecutive(veh, veh->getBackPositionOnLane(), true);
    //         }
    //     }
    //     if (blockersOnly) {
    //         // filter out vehicles that aren't blocking
    //         MSLeaderDistanceInfo blockers(targetLane->getWidth(), nullptr, 0.);
    //         for (int i = 0; i < neighbors.numSublanes(); i++) {
    //             CLeaderDist n = neighbors[i];
    //             if (n.first != nullptr) {
    //                 const MSVehicle* follower = veh;
    //                 const MSVehicle* leader = n.first;
    //                 if (!queryLeaders) {
    //                     std::swap(follower, leader);
    //                 }
    //                 const double secureGap = (follower->getCarFollowModel().getSecureGap(
    //                                               follower, leader, follower->getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel())
    //                                           * follower->getLaneChangeModel().getSafetyFactor());
    //                 if (n.second < secureGap) {
    //                     blockers.addLeader(n.first, n.second, 0, i);
    //                 }
    //             }
    //         }
    //         neighbors = blockers;
    //     }
    //
    //     if (neighbors.hasVehicles()) {
    //         for (int i = 0; i < neighbors.numSublanes(); i++) {
    //             CLeaderDist n = neighbors[i];
    //             if (n.first != nullptr &&
    //                     // avoid duplicates
    //                     (result.size() == 0 || result.back().first != n.first->getID())) {
    //                 result.push_back(std::make_pair(n.first->getID(), n.second));
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_neighbors(vehID: &str, mode: i32) -> Vec<(String, f64)> {
        let mut dir: i32 = if (1 & mode) != 0 { -1 } else { 1 };
        let queryLeaders = (2 & mode) != 0;
        let blockersOnly = (4 & mode) != 0;
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let mut result: Vec<(String, f64)> = Vec::new();
        if veh.m().get_lane_change_model().is_opposite() {
            // getParallelLane works relative to lane forward direction
            dir *= -1;
        }
        let targetLane = veh.get_lane().get_parallel_lane(dir, true);
        if targetLane.is_null() {
            return result;
        }
        // need to recompute leaders and followers (#8119)
        let opposite = veh.get_lane().get_edge() as *const MSEdge != targetLane.get_edge() as *const MSEdge;
        let mut neighbors = MSLeaderDistanceInfo::new(targetLane.get_width(), Ptr::default(), 0.);
        if queryLeaders {
            if opposite {
                let pos = targetLane.get_opposite_pos(veh.get_position_on_lane());
                neighbors = targetLane.get_followers_on_consecutive(veh, pos, true, -1., MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            } else {
                targetLane.m().add_leaders(veh, veh.get_position_on_lane(), &mut neighbors, false);
            }
        } else {
            if opposite {
                let pos = targetLane.get_opposite_pos(veh.get_position_on_lane());
                targetLane.m().add_leaders(veh, pos, &mut neighbors, false);
                neighbors.fix_opposite_gaps(true);
            } else {
                neighbors = targetLane.get_followers_on_consecutive(veh, veh.get_back_position_on_lane(veh.get_lane()), true, -1., MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            }
        }
        if blockersOnly {
            // filter out vehicles that aren't blocking
            let mut blockers = MSLeaderDistanceInfo::new(targetLane.get_width(), Ptr::default(), 0.);
            for i in 0..neighbors.num_sublanes() {
                let n = neighbors.index_op(i);
                if !n.0.is_null() {
                    let mut follower = veh;
                    let mut leader = n.0;
                    if !queryLeaders {
                        std::mem::swap(&mut follower, &mut leader);
                    }
                    let secureGap = follower.get_car_follow_model().get_secure_gap(follower, leader, follower.get_speed(), leader.get_speed(), leader.get_car_follow_model().get_max_decel())
                        * follower.m().get_lane_change_model().get_safety_factor();
                    if n.1 < secureGap {
                        blockers.add_leader(n.0, n.1, 0., i);
                    }
                }
            }
            neighbors = blockers;
        }
        if neighbors.has_vehicles() {
            for i in 0..neighbors.num_sublanes() {
                let n = neighbors.index_op(i);
                // avoid duplicates
                if !n.0.is_null() && (result.is_empty() || result.last().unwrap().0 != n.0.get_id()) {
                    result.push((n.0.get_id().to_string(), n.1));
                }
            }
        }
        result
    }

    /// C++: `double Vehicle::getFollowSpeed(const std::string &, double, double, double, double, const std::string &)` [static]
    // double
    // Vehicle::getFollowSpeed(const std::string& vehID, double speed, double gap, double leaderSpeed, double leaderMaxDecel, const std::string& leaderID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("getFollowSpeed not applicable for meso");
    //         return INVALID_DOUBLE_VALUE;
    //     }
    //     MSVehicle* leader = dynamic_cast<MSVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle(leaderID));
    //     return veh->getCarFollowModel().followSpeed(veh, speed, gap, leaderSpeed, leaderMaxDecel, leader, MSCFModel::CalcReason::FUTURE);
    // }
    #[allow(unused_variables)]
    pub fn get_follow_speed(vehID: &str, speed: f64, gap: f64, leaderSpeed: f64, leaderMaxDecel: f64, leaderID: &str) -> f64 {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let leader = micro_veh(MSNet::get_instance().get_vehicle_control().get_vehicle(leaderID).cast::<MSBaseVehicle>());
        veh.get_car_follow_model().follow_speed(veh, speed, gap, leaderSpeed, leaderMaxDecel, leader, MSCFModel_CalcReason::FUTURE)
    }

    /// C++: `double Vehicle::getSecureGap(const std::string &, double, double, double, const std::string &)` [static]
    // double
    // Vehicle::getSecureGap(const std::string& vehID, double speed, double leaderSpeed, double leaderMaxDecel, const std::string& leaderID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("getSecureGap not applicable for meso");
    //         return INVALID_DOUBLE_VALUE;
    //     }
    //     MSVehicle* leader = dynamic_cast<MSVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle(leaderID));
    //     return veh->getCarFollowModel().getSecureGap(veh, leader, speed, leaderSpeed, leaderMaxDecel);
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(vehID: &str, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64, leaderID: &str) -> f64 {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let leader = micro_veh(MSNet::get_instance().get_vehicle_control().get_vehicle(leaderID).cast::<MSBaseVehicle>());
        veh.get_car_follow_model().get_secure_gap(veh, leader, speed, leaderSpeed, leaderMaxDecel)
    }

    /// C++: `double Vehicle::getStopSpeed(const std::string &, double, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_speed(vehID: &str, speed: f64, gap: f64) -> f64 {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        veh.get_car_follow_model().stop_speed(veh, speed, gap, MSCFModel_CalcReason::FUTURE)
    }

    /// C++: `double Vehicle::getStopDelay(const std::string &)` [static]
    // double
    // Vehicle::getStopDelay(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getStopDelay();
    // }
    #[allow(unused_variables)]
    pub fn get_stop_delay(vehID: &str) -> f64 {
        Helper::get_vehicle(vehID).get_stop_delay()
    }

    /// C++: `double Vehicle::getStopArrivalDelay(const std::string &)` [static]
    // double
    // Vehicle::getStopArrivalDelay(const std::string& vehID) {
    //     double result = Helper::getVehicle(vehID)->getStopArrivalDelay();
    //     if (result == INVALID_DOUBLE) {
    //         return INVALID_DOUBLE_VALUE;
    //     } else {
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stop_arrival_delay(vehID: &str) -> f64 {
        let result = Helper::get_vehicle(vehID).get_stop_arrival_delay();
        if result == INVALID_DOUBLE {
            INVALID_DOUBLE_VALUE
        } else {
            result
        }
    }

    /// C++: `double Vehicle::getTimeLoss(const std::string &)` [static]
    // double
    // Vehicle::getTimeLoss(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getTimeLossSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(vehID: &str) -> f64 {
        Helper::get_vehicle(vehID).get_time_loss_seconds()
    }

    /// C++: `std::vector<std::string> Vehicle::getTaxiFleet(int)` [static]
    // std::vector<std::string>
    // Vehicle::getTaxiFleet(int taxiState) {
    //     std::vector<std::string> result;
    //     for (MSDevice_Taxi* taxi : MSDevice_Taxi::getFleet()) {
    //         if (taxi->getHolder().hasDeparted()) {
    //             if (taxiState == -1
    //                     || (taxiState == 0 && taxi->getState() == 0)
    //                     || (taxiState != 0 && (taxi->getState() & taxiState) == taxiState)) {
    //                 result.push_back(taxi->getHolder().getID());
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_taxi_fleet(taxiState: i32) -> Vec<String> {
        let mut result = Vec::new();
        for taxi in MSDevice_Taxi::get_fleet() {
            if taxi.get_holder().has_departed() {
                if taxiState == -1
                    || (taxiState == 0 && taxi.get_state() == 0)
                    || (taxiState != 0 && (taxi.get_state() & taxiState) == taxiState) {
                    result.push(taxi.get_holder().get_id().to_string());
                }
            }
        }
        result
    }

    /// C++: `std::vector<std::string> Vehicle::getLoadedIDList()` [static]
    // std::vector<std::string>
    // Vehicle::getLoadedIDList() {
    //     std::vector<std::string> ids;
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         ids.push_back(i->first);
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_loaded_id_list() -> Vec<String> {
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        c.myVehicleDict.keys().cloned().collect()
    }

    /// C++: `std::vector<std::string> Vehicle::getTeleportingIDList()` [static]
    // std::vector<std::string>
    // Vehicle::getTeleportingIDList() {
    //     std::vector<std::string> ids;
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         SUMOVehicle* veh = i->second;
    //         if (veh->hasDeparted() && !isVisible(veh)) {
    //             ids.push_back(veh->getID());
    //         }
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_teleporting_id_list() -> Vec<String> {
        let mut ids = Vec::new();
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        for veh in c.myVehicleDict.values() {
            if veh.has_departed() && !Self::is_visible(*veh) {
                ids.push(veh.get_id().to_string());
            }
        }
        ids
    }

    /// C++: `std::vector<std::string> Vehicle::getIDList()` [static]
    /// @}
    // std::vector<std::string>
    // Vehicle::getIDList() {
    //     std::vector<std::string> ids;
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         if (isVisible((*i).second)) {
    //             ids.push_back((*i).first);
    //         }
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        let mut ids = Vec::new();
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        for (id, veh) in c.myVehicleDict.iter() {
            if Self::is_visible(*veh) {
                ids.push(id.clone());
            }
        }
        ids
    }

    /// C++: `int Vehicle::getIDCount()` [static]
    /// @}
    // int
    // Vehicle::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        Self::get_id_list().len() as i32
    }

    /// C++: `std::string Vehicle::getParameter(const std::string &, const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getParameter(const std::string& vehID, const std::string& key) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     std::string error;
    //     std::string result = veh->getPrefixedParameter(key, error);
    //     if (error != "") {
    //         throw TraCIException(error);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(vehID: &str, key: &str) -> String {
        let veh = Helper::get_vehicle(vehID);
        let mut error = String::new();
        let result = veh.get_prefixed_parameter(key, &mut error);
        if error != "" {
            panic!("{}", error);
        }
        result
    }

    /// C++: `const std::pair<std::string, std::string> Vehicle::getParameterWithKey(const std::string &, const std::string &)` [static]
    /// @}
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Vehicle)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Vehicle::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setParameter(const std::string& vehID, const std::string& key, const std::string& value) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (StringUtils::startsWith(key, "device.")) {
    //         StringTokenizer tok(key, ".");
    //         if (tok.size() < 3) {
    //             throw TraCIException("Invalid device parameter '" + key + "' for vehicle '" + vehID + "'");
    //         }
    //         try {
    //             veh->setDeviceParameter(tok.get(1), key.substr(tok.get(0).size() + tok.get(1).size() + 2), value);
    //         } catch (InvalidArgument& e) {
    //             throw TraCIException("Vehicle '" + vehID + "' does not support device parameter '" + key + "' (" + e.what() + ").");
    //         }
    //     } else if (StringUtils::startsWith(key, "laneChangeModel.")) {
    //         if (microVeh == nullptr) {
    //             throw TraCIException("Meso Vehicle '" + vehID + "' does not support laneChangeModel parameters.");
    //         }
    //         const std::string attrName = key.substr(16);
    //         if (attrName == toString(SUMO_ATTR_LCA_CONTRIGHT)) {
    //             // special case: not used within lcModel
    //             veh->getSingularType().setLcContRight(value);
    //         } else {
    //             try {
    //                 microVeh->getLaneChangeModel().setParameter(attrName, value);
    //             } catch (InvalidArgument& e) {
    //                 throw TraCIException("Vehicle '" + vehID + "' does not support laneChangeModel parameter '" + key + "' (" + e.what() + ").");
    //             }
    //         }
    //     } else if (StringUtils::startsWith(key, "carFollowModel.")) {
    //         if (microVeh == nullptr) {
    //             throw TraCIException("Meso Vehicle '" + vehID + "' does not support carFollowModel parameters.");
    //         }
    //         try {
    //             veh->setCarFollowModelParameter(key, value);
    //         } catch (InvalidArgument& e) {
    //             throw TraCIException("Vehicle '" + vehID + "' does not support carFollowModel parameter '" + key + "' (" + e.what() + ").");
    //         }
    //     } else if (StringUtils::startsWith(key, "junctionModel.")) {
    //         try {
    //             // use the whole key (including junctionModel prefix)
    //             veh->setJunctionModelParameter(key, value);
    //         } catch (InvalidArgument& e) {
    //             // error message includes id since it is also used for xml input
    //             throw TraCIException(e.what());
    //         }
    //     } else if (StringUtils::startsWith(key, "has.") && StringUtils::endsWith(key, ".device")) {
    //         StringTokenizer tok(key, ".");
    //         if (tok.size() != 3) {
    //             throw TraCIException("Invalid request for device status change. Expected format is 'has.DEVICENAME.device'");
    //         }
    //         const std::string deviceName = tok.get(1);
    //         bool create;
    //         try {
    //             create = StringUtils::toBool(value);
    //         } catch (BoolFormatException&) {
    //             throw TraCIException("Changing device status requires a 'true' or 'false'");
    //         }
    //         if (!create) {
    //             throw TraCIException("Device removal is not supported for device of type '" + deviceName + "'");
    //         }
    //         try {
    //             veh->createDevice(deviceName);
    //         } catch (InvalidArgument& e) {
    //             throw TraCIException("Cannot create vehicle device (" + std::string(e.what()) + ").");
    //         }
    //     } else {
    //         ((SUMOVehicleParameter&)veh->getParameter()).setParameter(key, value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(vehID: &str, key: &str, value: &str) {
        let veh = Helper::get_vehicle(vehID);
        let microVeh = micro_veh(veh);
        if key.starts_with("device.") {
            let tok: Vec<&str> = key.split('.').filter(|s| !s.is_empty()).collect();
            if tok.len() < 3 {
                panic!("{}", format!("Invalid device parameter '{}' for vehicle '{}'", key, vehID));
            }
            // InvalidArgument from setDeviceParameter is rethrown as TraCIException
            veh.m().set_device_parameter(tok[1], &key[tok[0].len() + tok[1].len() + 2..], value);
        } else if key.starts_with("laneChangeModel.") {
            let attrName = &key[16..];
            if attrName == "lcContRight" {
                // special case: not used within lcModel
                veh.m().get_singular_type().set_lc_cont_right(value);
            } else {
                microVeh.m().get_lane_change_model().set_parameter(attrName, value);
            }
        } else if key.starts_with("carFollowModel.") {
            veh.m().set_car_follow_model_parameter(key, value);
        } else if key.starts_with("junctionModel.") {
            // use the whole key (including junctionModel prefix)
            veh.m().set_junction_model_parameter(key, value);
        } else if key.starts_with("has.") && key.ends_with(".device") {
            let tok: Vec<&str> = key.split('.').filter(|s| !s.is_empty()).collect();
            if tok.len() != 3 {
                panic!("Invalid request for device status change. Expected format is 'has.DEVICENAME.device'");
            }
            let deviceName = tok[1];
            let create = match value {
                "true" => true,
                "false" => false,
                _ => panic!("Changing device status requires a 'true' or 'false'"),
            };
            if !create {
                panic!("{}", format!("Device removal is not supported for device of type '{}'", deviceName));
            }
            veh.m().create_device(deviceName);
        } else {
            param_mut(veh.get_parameter()).set_parameter(key, value);
        }
    }

    /// C++: `double Vehicle::getLength(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getLength(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_length()
    }

    /// C++: `double Vehicle::getMaxSpeed(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMaxSpeed(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMaxSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_max_speed()
    }

    /// C++: `std::string Vehicle::getVehicleClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getVehicleClass(const std::string& vehID) {
    //     return toString(Helper::getVehicleType(vehID).getVehicleClass());
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(typeID: &str) -> String {
        to_string(&Helper::get_vehicle_type(typeID).get_vehicle_class())
    }

    /// C++: `double Vehicle::getSpeedFactor(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getSpeedFactor(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getChosenSpeedFactor();
    // }
    #[allow(unused_variables)]
    pub fn get_speed_factor(vehID: &str) -> f64 {
        Helper::get_vehicle(vehID).get_chosen_speed_factor()
    }

    /// C++: `double Vehicle::getAccel(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getAccel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getMaxAccel();
    // }
    #[allow(unused_variables)]
    pub fn get_accel(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_max_accel()
    }

    /// C++: `double Vehicle::getDecel(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getMaxDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_decel(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_max_decel()
    }

    /// C++: `double Vehicle::getEmergencyDecel(const std::string &)` [static]
    /// @}
    // double Vehicle::getEmergencyDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getEmergencyDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_emergency_decel(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_emergency_decel()
    }

    /// C++: `double Vehicle::getApparentDecel(const std::string &)` [static]
    /// @}
    // double Vehicle::getApparentDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getApparentDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_apparent_decel(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_apparent_decel()
    }

    /// C++: `double Vehicle::getImperfection(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getImperfection(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getImperfection();
    // }
    #[allow(unused_variables)]
    pub fn get_imperfection(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_imperfection()
    }

    /// C++: `double Vehicle::getTau(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getTau(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getHeadwayTime();
    // }
    #[allow(unused_variables)]
    pub fn get_tau(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_car_follow_model().get_headway_time()
    }

    /// C++: `std::string Vehicle::getEmissionClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getEmissionClass(const std::string& vehID) {
    //     return PollutantsInterface::getName(Helper::getVehicleType(vehID).getEmissionClass());
    // }
    #[allow(unused_variables)]
    pub fn get_emission_class(typeID: &str) -> String {
        PollutantsInterface::get_name(Helper::get_vehicle_type(typeID).get_emission_class())
    }

    /// C++: `std::string Vehicle::getShapeClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getShapeClass(const std::string& vehID) {
    //     return getVehicleShapeName(Helper::getVehicleType(vehID).getGuiShape());
    // }
    #[allow(unused_variables)]
    pub fn get_shape_class(typeID: &str) -> String {
        get_vehicle_shape_name(Helper::get_vehicle_type(typeID).get_gui_shape())
    }

    /// C++: `double Vehicle::getMinGap(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMinGap(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_min_gap()
    }

    /// C++: `double Vehicle::getWidth(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getWidth(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_width()
    }

    /// C++: `double Vehicle::getHeight(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getHeight(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_height()
    }

    /// C++: `double Vehicle::getMass(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMass(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMass();
    // }
    #[allow(unused_variables)]
    pub fn get_mass(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_mass()
    }

    /// C++: `libsumo::TraCIColor Vehicle::getColor(const std::string &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(vehID: &str) -> TraCIColor {
        Helper::make_tra_ci_color(&Helper::get_vehicle(vehID).get_parameter().color)
    }

    /// C++: `double Vehicle::getMinGapLat(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMinGapLat(const std::string& vehID) {
    //     try {
    //         return StringUtils::toDouble(getParameter(vehID, "laneChangeModel.minGapLat"));
    //     } catch (const TraCIException&) {
    //         // legacy behavior
    //         return Helper::getVehicleType(vehID).getMinGapLat();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap_lat(typeID: &str) -> f64 {
        let mut err = String::new();
        let param = Helper::get_vehicle(typeID).get_prefixed_parameter("laneChangeModel.minGapLat", &mut err);
        if err.is_empty() {
            // StringUtils::toDouble
            param.trim().parse::<f64>().unwrap_or_else(|_| panic!("{}", format!("Not a valid double: '{}'", param)))
        } else {
            // legacy behavior
            Helper::get_vehicle_type(typeID).get_min_gap_lat()
        }
    }

    /// C++: `double Vehicle::getMaxSpeedLat(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMaxSpeedLat(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMaxSpeedLat();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_max_speed_lat()
    }

    /// C++: `std::string Vehicle::getLateralAlignment(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getLateralAlignment(const std::string& vehID) {
    //     return toString(Helper::getVehicleType(vehID).getPreferredLateralAlignment());
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_alignment(typeID: &str) -> String {
        crate::libsumo::vehicle_type::lat_alignment_to_string(*Helper::get_vehicle_type(typeID).get_preferred_lateral_alignment()).to_string()
    }

    /// C++: `int Vehicle::getPersonCapacity(const std::string &)` [static]
    /// @}
    // int
    // Vehicle::getPersonCapacity(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getPersonCapacity();
    // }
    #[allow(unused_variables)]
    pub fn get_person_capacity(typeID: &str) -> i32 {
        Helper::get_vehicle_type(typeID).get_person_capacity()
    }

    /// C++: `double Vehicle::getActionStepLength(const std::string &)` [static]
    /// @}
    // double Vehicle::getActionStepLength(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getActionStepLengthSecs();
    // }
    #[allow(unused_variables)]
    pub fn get_action_step_length(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_action_step_length_secs()
    }

    /// C++: `double Vehicle::getSpeedDeviation(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getSpeedDeviation(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getSpeedFactor().getParameter(1);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_deviation(typeID: &str) -> f64 {
        Helper::get_vehicle_type(typeID).get_speed_factor().get_parameter(1)
    }

    /// C++: `double Vehicle::getBoardingDuration(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getBoardingDuration(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicleType(vehID).getLoadingDuration(true));
    // }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(typeID: &str) -> f64 {
        steps2time(Helper::get_vehicle_type(typeID).get_loading_duration(true))
    }

    /// C++: `double Vehicle::getImpatience(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getImpatience(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getImpatience();
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(vehID: &str) -> f64 {
        Helper::get_vehicle(vehID).get_impatience()
    }

    /// C++: `void Vehicle::setStop(const std::string &, const std::string &, double, int, double, int, double, double)` [static]
    /// @{
    // void
    // Vehicle::setStop(const std::string& vehID,
    //                  const std::string& edgeID,
    //                  double pos,
    //                  int laneIndex,
    //                  double duration,
    //                  int flags,
    //                  double startPos,
    //                  double until) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     SUMOVehicleParameter::Stop stopPars = Helper::buildStopParameters(edgeID,
    //                                           pos, laneIndex, startPos, flags, duration, until);
    //     std::string error;
    //     if (!vehicle->addTraciStop(stopPars, error)) {
    //         throw TraCIException(error);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_stop(vehID: &str, edgeID: &str, pos: f64, laneIndex: i32, duration: f64, flags: i32, startPos: f64, until: f64) {
        let vehicle = Helper::get_vehicle(vehID);
        let stopPars = Helper::build_stop_parameters(edgeID, pos, laneIndex, startPos, flags, duration, until);
        let mut error = String::new();
        if !vehicle.m().add_traci_stop(stopPars, &mut error) {
            panic!("{}", error);
        }
    }

    /// C++: `void Vehicle::replaceStop(const std::string &, int, const std::string &, double, int, double, int, double, double, int)` [static]
    // void
    // Vehicle::replaceStop(const std::string& vehID,
    //                      int nextStopIndex,
    //                      const std::string& edgeID,
    //                      double pos,
    //                      int laneIndex,
    //                      double duration,
    //                      int flags,
    //                      double startPos,
    //                      double until,
    //                      int teleport) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     std::string error;
    //     if (edgeID == "") {
    //         // only remove stop
    //         const bool ok = vehicle->abortNextStop(nextStopIndex);
    //         if (teleport != 0) {
    //             if (!vehicle->rerouteBetweenStops(nextStopIndex, "traci:replaceStop", (teleport & 1), error)) {
    //                 throw TraCIException("Stop replacement failed for vehicle '" + vehID + "' (" + error + ").");
    //             }
    //         } else {
    //             MSVehicle* msVeh = dynamic_cast<MSVehicle*>(vehicle);
    //             if (msVeh->getLane() != nullptr) {
    //                 msVeh->updateBestLanes(true);
    //             }
    //         }
    //         if (!ok) {
    //             throw TraCIException("Stop replacement failed for vehicle '" + vehID + "' (invalid nextStopIndex).");
    //         }
    //     } else {
    //         SUMOVehicleParameter::Stop stopPars = Helper::buildStopParameters(edgeID,
    //                                               pos, laneIndex, startPos, flags, duration, until);
    //
    //         if (!vehicle->replaceStop(nextStopIndex, stopPars, "traci:replaceStop", teleport != 0, error)) {
    //             throw TraCIException("Stop replacement failed for vehicle '" + vehID + "' (" + error + ").");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_stop(vehID: &str, nextStopIndex: i32, edgeID: &str, pos: f64, laneIndex: i32, duration: f64, flags: i32, startPos: f64, until: f64, teleport: i32) {
        let vehicle = Helper::get_vehicle(vehID);
        let mut error = String::new();
        if edgeID == "" {
            // only remove stop
            let ok = vehicle.m().abort_next_stop(nextStopIndex);
            if teleport != 0 {
                if !vehicle.m().reroute_between_stops(nextStopIndex, "traci:replaceStop", (teleport & 1) != 0, &mut error) {
                    panic!("{}", format!("Stop replacement failed for vehicle '{}' ({}).", vehID, error));
                }
            } else {
                let msVeh = micro_veh(vehicle);
                if !msVeh.get_lane().is_null() {
                    msVeh.m().update_best_lanes(true, Ptr::default());
                }
            }
            if !ok {
                panic!("{}", format!("Stop replacement failed for vehicle '{}' (invalid nextStopIndex).", vehID));
            }
        } else {
            let stopPars = Helper::build_stop_parameters(edgeID, pos, laneIndex, startPos, flags, duration, until);
            if !vehicle.m().replace_stop(nextStopIndex, stopPars, "traci:replaceStop", teleport != 0, &mut error) {
                panic!("{}", format!("Stop replacement failed for vehicle '{}' ({}).", vehID, error));
            }
        }
    }

    /// C++: `void Vehicle::insertStop(const std::string &, int, const std::string &, double, int, double, int, double, double, int)` [static]
    // void
    // Vehicle::insertStop(const std::string& vehID,
    //                     int nextStopIndex,
    //                     const std::string& edgeID,
    //                     double pos,
    //                     int laneIndex,
    //                     double duration,
    //                     int flags,
    //                     double startPos,
    //                     double until,
    //                     int teleport) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     SUMOVehicleParameter::Stop stopPars = Helper::buildStopParameters(edgeID,
    //                                           pos, laneIndex, startPos, flags, duration, until);
    //
    //     std::string error;
    //     if (!vehicle->insertStop(nextStopIndex, stopPars, "traci:insertStop", teleport != 0, error)) {
    //         throw TraCIException("Stop insertion failed for vehicle '" + vehID + "' (" + error + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_stop(vehID: &str, nextStopIndex: i32, edgeID: &str, pos: f64, laneIndex: i32, duration: f64, flags: i32, startPos: f64, until: f64, teleport: i32) {
        let vehicle = Helper::get_vehicle(vehID);
        let stopPars = Helper::build_stop_parameters(edgeID, pos, laneIndex, startPos, flags, duration, until);
        let mut error = String::new();
        if !vehicle.m().insert_stop(nextStopIndex, stopPars, "traci:insertStop", teleport != 0, &mut error) {
            panic!("{}", format!("Stop insertion failed for vehicle '{}' ({}).", vehID, error));
        }
    }

    /// C++: `void Vehicle::setStopParameter(const std::string &, int, const std::string &, const std::string &, bool)` [static]
    // void
    // Vehicle::setStopParameter(const std::string& vehID, int nextStopIndex,
    //                           const std::string& param, const std::string& value,
    //                           bool customParam) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     try {
    //         MSStop& stop = vehicle->getStop(nextStopIndex);
    //         SUMOVehicleParameter::Stop& pars = const_cast<SUMOVehicleParameter::Stop&>(stop.pars);
    //         if (customParam) {
    //             pars.setParameter(param, value);
    //             return;
    //         }
    //         std::string error;
    //         if (param == toString(SUMO_ATTR_EDGE)
    //                 || param == toString(SUMO_ATTR_BUS_STOP)
    //                 || param == toString(SUMO_ATTR_TRAIN_STOP)
    //                 || param == toString(SUMO_ATTR_CONTAINER_STOP)
    //                 || param == toString(SUMO_ATTR_CHARGING_STATION)
    //                 || param == toString(SUMO_ATTR_PARKING_AREA)
    //                 || param == toString(SUMO_ATTR_LANE)
    //            ) {
    //             int laneIndex = stop.lane->getIndex();
    //             int flags = pars.getFlags() & 3;
    //             std::string edgeOrStopID = value;
    //             if (param == toString(SUMO_ATTR_LANE)) {
    //                 laneIndex = StringUtils::toInt(value);
    //                 edgeOrStopID = pars.edge;
    //             } else if (param == toString(SUMO_ATTR_BUS_STOP)
    //                        || param == toString(SUMO_ATTR_TRAIN_STOP)) {
    //                 flags |= 8;
    //             } else if (param == toString(SUMO_ATTR_CONTAINER_STOP)) {
    //                 flags |= 16;
    //             } else if (param == toString(SUMO_ATTR_CHARGING_STATION)) {
    //                 flags |= 32;
    //             } else if (param == toString(SUMO_ATTR_PARKING_AREA)) {
    //                 flags |= 64;
    //             }
    //             // special case: replace stop
    //             replaceStop(vehID, nextStopIndex, edgeOrStopID, pars.endPos, laneIndex, STEPS2TIME(pars.duration),
    //                         flags, pars.startPos, STEPS2TIME(pars.until), 0);
    //
    //         } else if (param == toString(SUMO_ATTR_STARTPOS)) {
    //             pars.startPos = StringUtils::toDouble(value);
    //             pars.parametersSet |= STOP_START_SET;
    //         } else if (param == toString(SUMO_ATTR_ENDPOS)) {
    //             pars.endPos = StringUtils::toDouble(value);
    //             pars.parametersSet |= STOP_END_SET;
    //         } else if (param == toString(SUMO_ATTR_POSITION_LAT)) {
    //             pars.posLat = StringUtils::toDouble(value);
    //             pars.parametersSet |= STOP_POSLAT_SET;
    //         } else if (param == toString(SUMO_ATTR_ARRIVAL)) {
    //             pars.arrival = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_ARRIVAL_SET;
    //         } else if (param == toString(SUMO_ATTR_DURATION)) {
    //             pars.duration = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_DURATION_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_UNTIL)) {
    //             pars.until = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_UNTIL_SET;
    //         } else if (param == toString(SUMO_ATTR_EXTENSION)) {
    //             pars.extension = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_EXTENSION_SET;
    //         } else if (param == toString(SUMO_ATTR_INDEX)) {
    //             throw TraCIException("Changing stop index is not supported");
    //         } else if (param == toString(SUMO_ATTR_PARKING)) {
    //             pars.parking = SUMOVehicleParameter::parseParkingType(value);
    //             pars.parametersSet |= STOP_PARKING_SET;
    //         } else if (param == toString(SUMO_ATTR_TRIGGERED)) {
    //             if (pars.speed > 0 && value != "") {
    //                 throw ProcessError(TLF("Waypoint (speed = %) at index % does not support triggers", pars.speed, nextStopIndex));
    //             }
    //             SUMOVehicleParameter::parseStopTriggers(StringTokenizer(value).getVector(), false, pars);
    //             pars.parametersSet |= STOP_TRIGGER_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_EXPECTED)) {
    //             pars.awaitedPersons = StringTokenizer(value).getSet();
    //             pars.parametersSet |= STOP_EXPECTED_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_EXPECTED_CONTAINERS)) {
    //             pars.awaitedContainers = StringTokenizer(value).getSet();
    //             pars.parametersSet |= STOP_EXPECTED_CONTAINERS_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_PERMITTED)) {
    //             pars.permitted = StringTokenizer(value).getSet();
    //             pars.parametersSet |= STOP_PERMITTED_SET;
    //         } else if (param == toString(SUMO_ATTR_ACTTYPE)) {
    //             pars.actType = value;
    //         } else if (param == toString(SUMO_ATTR_TRIP_ID)) {
    //             pars.tripId = value;
    //             pars.parametersSet |= STOP_TRIP_ID_SET;
    //         } else if (param == toString(SUMO_ATTR_SPLIT)) {
    //             pars.split = value;
    //             pars.parametersSet |= STOP_SPLIT_SET;
    //         } else if (param == toString(SUMO_ATTR_JOIN)) {
    //             pars.join = value;
    //             pars.parametersSet |= STOP_JOIN_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_LINE)) {
    //             pars.line = value;
    //             pars.parametersSet |= STOP_LINE_SET;
    //         } else if (param == toString(SUMO_ATTR_SPEED)) {
    //             const double speed = StringUtils::toDouble(value);
    //             if (speed > 0 && pars.getTriggers().size() > 0) {
    //                 throw ProcessError(TLF("Triggered stop at index % cannot be changed into a waypoint by setting speed to %", nextStopIndex, speed));
    //             }
    //             pars.speed = speed;
    //             pars.parametersSet |= STOP_SPEED_SET;
    //         } else if (param == toString(SUMO_ATTR_STARTED)) {
    //             pars.started = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_STARTED_SET;
    //         } else if (param == toString(SUMO_ATTR_ENDED)) {
    //             pars.ended = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_ENDED_SET;
    //         } else if (param == toString(SUMO_ATTR_ONDEMAND)) {
    //             pars.onDemand = StringUtils::toBool(value);
    //             pars.parametersSet |= STOP_ONDEMAND_SET;
    //         } else if (param == toString(SUMO_ATTR_JUMP)) {
    //             pars.jump = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_JUMP_SET;
    //         } else if (param == toString(SUMO_ATTR_JUMP_UNTIL)) {
    //             pars.jumpUntil = string2time(value).unwrap_or_else(|e| panic!("{}", e));
    //             pars.parametersSet |= STOP_JUMP_UNTIL_SET;
    //         } else {
    //             throw ProcessError(TLF("Unsupported parameter '%'", param));
    //         }
    //     } catch (ProcessError& e) {
    //         throw TraCIException("Could not set stop parameter for vehicle '" + vehID + "' (" + e.what() + ")");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_stop_parameter(vehID: &str, nextStopIndex: i32, param: &str, value: &str, customParam: bool) {
        let vehicle = Helper::get_vehicle(vehID);
        let stop = vehicle.m().get_stop(nextStopIndex);
        let pars = &mut stop.pars;
        if customParam {
            pars.set_parameter(param, value);
            return;
        }
        if param == "edge" || param == "busStop" || param == "trainStop" || param == "containerStop"
            || param == "chargingStation" || param == "parkingArea" || param == "lane" {
            let mut laneIndex = stop.lane.get_index();
            let mut flags = pars.get_flags() & 3;
            let mut edgeOrStopID = value.to_string();
            if param == "lane" {
                laneIndex = value.parse::<i32>().unwrap_or_else(|_| panic!("{}", format!("Not a valid integer: '{}'", value)));
                edgeOrStopID = pars.edge.clone();
            } else if param == "busStop" || param == "trainStop" {
                flags |= 8;
            } else if param == "containerStop" {
                flags |= 16;
            } else if param == "chargingStation" {
                flags |= 32;
            } else if param == "parkingArea" {
                flags |= 64;
            }
            // special case: replace stop
            Self::replace_stop(vehID, nextStopIndex, &edgeOrStopID, pars.endPos, laneIndex, steps2time(pars.duration),
                               flags, pars.startPos, steps2time(pars.until), 0);
        } else if param == "startPos" {
            pars.startPos = value.parse::<f64>().unwrap_or_else(|_| panic!("{}", format!("Not a valid double: '{}'", value)));
            pars.parametersSet |= STOP_START_SET;
        } else if param == "endPos" {
            pars.endPos = value.parse::<f64>().unwrap_or_else(|_| panic!("{}", format!("Not a valid double: '{}'", value)));
            pars.parametersSet |= STOP_END_SET;
        } else if param == "posLat" {
            pars.posLat = value.parse::<f64>().unwrap_or_else(|_| panic!("{}", format!("Not a valid double: '{}'", value)));
            pars.parametersSet |= STOP_POSLAT_SET;
        } else if param == "arrival" {
            pars.arrival = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_ARRIVAL_SET;
        } else if param == "duration" {
            pars.duration = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_DURATION_SET;
            // also update dynamic value
            let parsPtr = pars as *const SUMOVehicleParameter_Stop;
            stop.init_pars(unsafe { &*parsPtr });
        } else if param == "until" {
            pars.until = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_UNTIL_SET;
        } else if param == "extension" {
            pars.extension = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_EXTENSION_SET;
        } else if param == "index" {
            panic!("Changing stop index is not supported");
        } else if param == "parking" {
            pars.parking = SUMOVehicleParameter::parse_parking_type(value);
            pars.parametersSet |= STOP_PARKING_SET;
        } else if param == "triggered" {
            if pars.speed > 0. && value != "" {
                set_stop_param_error(vehID, format!("Waypoint (speed = {}) at index {} does not support triggers", pars.speed, nextStopIndex));
            }
            let tokens: Vec<String> = value.split_whitespace().map(String::from).collect();
            SUMOVehicleParameter::parse_stop_triggers(&tokens, false, pars);
            pars.parametersSet |= STOP_TRIGGER_SET;
            // also update dynamic value
            let parsPtr = pars as *const SUMOVehicleParameter_Stop;
            stop.init_pars(unsafe { &*parsPtr });
        } else if param == "expected" {
            pars.awaitedPersons = value.split_whitespace().map(String::from).collect();
            pars.parametersSet |= STOP_EXPECTED_SET;
            // also update dynamic value
            let parsPtr = pars as *const SUMOVehicleParameter_Stop;
            stop.init_pars(unsafe { &*parsPtr });
        } else if param == "expectedContainers" {
            pars.awaitedContainers = value.split_whitespace().map(String::from).collect();
            pars.parametersSet |= STOP_EXPECTED_CONTAINERS_SET;
            // also update dynamic value
            let parsPtr = pars as *const SUMOVehicleParameter_Stop;
            stop.init_pars(unsafe { &*parsPtr });
        } else if param == "permitted" {
            pars.permitted = value.split_whitespace().map(String::from).collect();
            pars.parametersSet |= STOP_PERMITTED_SET;
        } else if param == "actType" {
            pars.actType = value.to_string();
        } else if param == "tripId" {
            pars.tripId = value.to_string();
            pars.parametersSet |= STOP_TRIP_ID_SET;
        } else if param == "split" {
            pars.split = value.to_string();
            pars.parametersSet |= STOP_SPLIT_SET;
        } else if param == "join" {
            pars.join = value.to_string();
            pars.parametersSet |= STOP_JOIN_SET;
            // also update dynamic value
            let parsPtr = pars as *const SUMOVehicleParameter_Stop;
            stop.init_pars(unsafe { &*parsPtr });
        } else if param == "line" {
            pars.line = value.to_string();
            pars.parametersSet |= STOP_LINE_SET;
        } else if param == "speed" {
            let speed = value.parse::<f64>().unwrap_or_else(|_| panic!("{}", format!("Not a valid double: '{}'", value)));
            if speed > 0. && pars.get_triggers().len() > 0 {
                set_stop_param_error(vehID, format!("Triggered stop at index {} cannot be changed into a waypoint by setting speed to {}", nextStopIndex, speed));
            }
            pars.speed = speed;
            pars.parametersSet |= STOP_SPEED_SET;
        } else if param == "started" {
            pars.started = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_STARTED_SET;
        } else if param == "ended" {
            pars.ended = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_ENDED_SET;
        } else if param == "onDemand" {
            pars.onDemand = utils::common::string_utils::to_bool(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_ONDEMAND_SET;
        } else if param == "jump" {
            pars.jump = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_JUMP_SET;
        } else if param == "jumpUntil" {
            pars.jumpUntil = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            pars.parametersSet |= STOP_JUMP_UNTIL_SET;
        } else {
            set_stop_param_error(vehID, format!("Unsupported parameter '{}'", param));
        }
    }

    /// C++: `void Vehicle::rerouteParkingArea(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::rerouteParkingArea(const std::string& vehID, const std::string& parkingAreaID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("rerouteParkingArea not yet implemented for meso");
    //         return;
    //     }
    //     std::string error;
    //     // Forward command to vehicle
    //     if (!veh->rerouteParkingArea(parkingAreaID, error)) {
    //         throw TraCIException(error);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute_parking_area(vehID: &str, parkingAreaID: &str) {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let mut error = String::new();
        // Forward command to vehicle
        if !veh.m().reroute_parking_area(parkingAreaID, &mut error) {
            panic!("{}", error);
        }
    }

    /// C++: `void Vehicle::resume(const std::string &)` [static]
    // void
    // Vehicle::resume(const std::string& vehID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("resume not yet implemented for meso");
    //         return;
    //     }
    //     if (!veh->hasStops()) {
    //         throw TraCIException("Failed to resume vehicle '" + veh->getID() + "', it has no stops.");
    //     }
    //     if (!veh->resumeFromStopping()) {
    //         const MSStop& sto = veh->getNextStop();
    //         std::ostringstream strs;
    //         strs << "reached: " << sto.reached;
    //         strs << ", duration:" << sto.duration;
    //         strs << ", edge:" << (*sto.edge)->getID();
    //         strs << ", startPos: " << sto.pars.startPos;
    //         std::string posStr = strs.str();
    //         throw TraCIException("Failed to resume from stopping for vehicle '" + veh->getID() + "', " + posStr);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn resume(vehID: &str) {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        if !veh.has_stops() {
            panic!("{}", format!("Failed to resume vehicle '{}', it has no stops.", veh.get_id()));
        }
        if !veh.m().resume_from_stopping() {
            let sto = veh.get_next_stop();
            // shortcut: MSStop.edge is not typed yet (Unmapped), the stop's parameter edge is used instead
            let posStr = format!("reached: {}, duration:{}, edge:{}, startPos: {}", sto.reached, sto.duration, sto.pars.edge, sto.pars.startPos);
            panic!("{}", format!("Failed to resume from stopping for vehicle '{}', {}", veh.get_id(), posStr));
        }
    }

    /// C++: `void Vehicle::add(const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, const std::string &, int, int, const std::string &, const std::string &, double, const std::string &)` [static]
    // void
    // Vehicle::add(const std::string& vehID,
    //              const std::string& routeID,
    //              const std::string& typeID,
    //              const std::string& depart,
    //              const std::string& departLane,
    //              const std::string& departPos,
    //              const std::string& departSpeed,
    //              const std::string& arrivalLane,
    //              const std::string& arrivalPos,
    //              const std::string& arrivalSpeed,
    //              const std::string& fromTaz,
    //              const std::string& toTaz,
    //              const std::string& line,
    //              int /*personCapacity*/,
    //              int personNumber,
    //              const std::string& departPosLat,
    //              const std::string& arrivalPosLat,
    //              double speedFactor,
    //              const std::string& insertionChecks) {
    //     SUMOVehicle* veh = MSNet::getInstance()->getVehicleControl().getVehicle(vehID);
    //     if (veh != nullptr) {
    //         throw TraCIException("The vehicle '" + vehID + "' to add already exists.");
    //     }
    //
    //     SUMOVehicleParameter vehicleParams;
    //     vehicleParams.id = vehID;
    //     MSVehicleType* vehicleType = MSNet::getInstance()->getVehicleControl().getVType(typeID);
    //     if (!vehicleType) {
    //         throw TraCIException("Invalid type '" + typeID + "' for vehicle '" + vehID + "'.");
    //     }
    //     if (typeID != "DEFAULT_VEHTYPE") {
    //         vehicleParams.vtypeid = typeID;
    //         vehicleParams.parametersSet |= VEHPARS_VTYPE_SET;
    //     }
    //     if (SUMOVehicleParserHelper::isInternalRouteID(routeID)) {
    //         WRITE_WARNINGF(TL("Internal routes receive an ID starting with '!' and must not be referenced in other vehicle or flow definitions. Please remove all references to route '%' in case it is internal."), routeID);
    //     }
    //     ConstMSRoutePtr route = MSRoute::dictionary(routeID);
    //     if (!route) {
    //         if (routeID == "") {
    //             // assume, route was intentionally left blank because the caller
    //             // intends to control the vehicle remotely
    //             SUMOVehicleClass vclass = vehicleType->getVehicleClass();
    //             const std::string dummyRouteID = "DUMMY_ROUTE_" + SumoVehicleClassStrings.getString(vclass);
    //             route = MSRoute::dictionary(dummyRouteID);
    //             if (route == nullptr) {
    //                 for (MSEdge* e : MSEdge::getAllEdges()) {
    //                     if (e->getFunction() == SumoXMLEdgeFunc::NORMAL && (e->getPermissions() & vclass) == vclass) {
    //                         std::vector<std::string>  edges;
    //                         edges.push_back(e->getID());
    //                         libsumo::Route::add(dummyRouteID, edges);
    //                         break;
    //                     }
    //                 }
    //             }
    //             route = MSRoute::dictionary(dummyRouteID);
    //             if (!route) {
    //                 throw TraCIException("Could not build dummy route for vehicle class: '" + SumoVehicleClassStrings.getString(vehicleType->getVehicleClass()) + "'");
    //             }
    //         } else {
    //             throw TraCIException("Invalid route '" + routeID + "' for vehicle '" + vehID + "'.");
    //         }
    //     }
    //     // check if the route implies a trip
    //     if (route->getEdges().size() == 2) {
    //         const MSEdgeVector& succ = route->getEdges().front()->getSuccessors();
    //         if (std::find(succ.begin(), succ.end(), route->getEdges().back()) == succ.end()) {
    //             vehicleParams.parametersSet |= VEHPARS_FORCE_REROUTE;
    //         }
    //     }
    //     if (fromTaz != "" || toTaz != "") {
    //         vehicleParams.parametersSet |= VEHPARS_FORCE_REROUTE;
    //     }
    //     std::string error;
    //     if (!SUMOVehicleParameter::parseDepart(depart, "vehicle", vehID, vehicleParams.depart, vehicleParams.departProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (vehicleParams.departProcedure == DepartDefinition::GIVEN && vehicleParams.depart < MSNet::getInstance()->getCurrentTimeStep()) {
    //         vehicleParams.depart = MSNet::getInstance()->getCurrentTimeStep();
    //         WRITE_WARNINGF(TL("Departure time for vehicle '%' is in the past; using current time instead."), vehID);
    //     } else if (vehicleParams.departProcedure == DepartDefinition::NOW) {
    //         vehicleParams.depart = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    //     if (!SUMOVehicleParameter::parseDepartLane(departLane, "vehicle", vehID, vehicleParams.departLane, vehicleParams.departLaneProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseDepartPos(departPos, "vehicle", vehID, vehicleParams.departPos, vehicleParams.departPosProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseDepartSpeed(departSpeed, "vehicle", vehID, vehicleParams.departSpeed, vehicleParams.departSpeedProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseArrivalLane(arrivalLane, "vehicle", vehID, vehicleParams.arrivalLane, vehicleParams.arrivalLaneProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseArrivalPos(arrivalPos, "vehicle", vehID, vehicleParams.arrivalPos, vehicleParams.arrivalPosProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!SUMOVehicleParameter::parseArrivalSpeed(arrivalSpeed, "vehicle", vehID, vehicleParams.arrivalSpeed, vehicleParams.arrivalSpeedProcedure, error)) {
    //         throw TraCIException(error);
    //     }
    //     if (!departPosLat.empty()) {
    //         if (!SUMOVehicleParameter::parseDepartPosLat(departPosLat, "vehicle", vehID, vehicleParams.departPosLat, vehicleParams.departPosLatProcedure, error)) {
    //             throw TraCIException(error);
    //         }
    //         vehicleParams.parametersSet |= VEHPARS_DEPARTPOSLAT_SET;
    //     }
    //     if (!arrivalPosLat.empty()) {
    //         if (!SUMOVehicleParameter::parseArrivalPosLat(arrivalPosLat, "vehicle", vehID, vehicleParams.arrivalPosLat, vehicleParams.arrivalPosLatProcedure, error)) {
    //             throw TraCIException(error);
    //         }
    //         vehicleParams.parametersSet |= VEHPARS_ARRIVALPOSLAT_SET;
    //     }
    //     if (speedFactor > 0) {
    //         vehicleParams.speedFactor = speedFactor;
    //         vehicleParams.parametersSet |= VEHPARS_SPEEDFACTOR_SET;
    //     }
    //     if (!insertionChecks.empty()) {
    //         try {
    //             vehicleParams.insertionChecks = SUMOVehicleParameter::parseInsertionChecks(insertionChecks);
    //             vehicleParams.parametersSet |= VEHPARS_INSERTION_CHECKS_SET;
    //         } catch (const InvalidArgument& e) {
    //             throw TraCIException(e.what());
    //         }
    //     }
    //     // mark non-default attributes
    //     if (departLane != "first") {
    //         vehicleParams.parametersSet |= VEHPARS_DEPARTLANE_SET;
    //     }
    //     if (departPos != "base") {
    //         vehicleParams.parametersSet |= VEHPARS_DEPARTPOS_SET;
    //     }
    //     if (departSpeed != "0") {
    //         vehicleParams.parametersSet |= VEHPARS_DEPARTSPEED_SET;
    //     }
    //     if (arrivalLane != "current") {
    //         vehicleParams.parametersSet |= VEHPARS_ARRIVALLANE_SET;
    //     }
    //     if (arrivalPos != "max") {
    //         vehicleParams.parametersSet |= VEHPARS_ARRIVALPOS_SET;
    //     }
    //     if (arrivalSpeed != "current") {
    //         vehicleParams.parametersSet |= VEHPARS_ARRIVALSPEED_SET;
    //     }
    //     if (fromTaz != "") {
    //         vehicleParams.parametersSet |= VEHPARS_FROM_TAZ_SET;
    //     }
    //     if (toTaz != "") {
    //         vehicleParams.parametersSet |= VEHPARS_TO_TAZ_SET;
    //     }
    //     if (line != "") {
    //         vehicleParams.parametersSet |= VEHPARS_LINE_SET;
    //     }
    //     if (personNumber != 0) {
    //         vehicleParams.parametersSet |= VEHPARS_PERSON_NUMBER_SET;
    //     }
    //     // build vehicle
    //     vehicleParams.fromTaz = fromTaz;
    //     vehicleParams.toTaz = toTaz;
    //     vehicleParams.line = line;
    //     //vehicleParams.personCapacity = personCapacity;
    //     vehicleParams.personNumber = personNumber;
    //
    //     SUMOVehicleParameter* params = new SUMOVehicleParameter(vehicleParams);
    //     SUMOVehicle* vehicle = nullptr;
    //     try {
    //         vehicle = MSNet::getInstance()->getVehicleControl().buildVehicle(params, route, vehicleType, true, MSVehicleControl::VehicleDefinitionSource::LIBSUMO);
    //         if (fromTaz == "" && !route->getEdges().front()->validateDepartSpeed(*vehicle)) {
    //             MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //             throw TraCIException("Departure speed for vehicle '" + vehID + "' is too high for the departure edge '" + route->getEdges().front()->getID() + "'.");
    //         }
    //         std::string msg;
    //         if (vehicle->getRouteValidity(true, true, &msg) != MSBaseVehicle::ROUTE_VALID) {
    //             MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //             throw TraCIException("Vehicle '" + vehID + "' has no valid route (" + msg + "). ");
    //         }
    //         MSNet::getInstance()->getVehicleControl().addVehicle(vehicleParams.id, vehicle);
    //         if (vehicleParams.departProcedure != DepartDefinition::TRIGGERED && vehicleParams.departProcedure != DepartDefinition::CONTAINER_TRIGGERED) {
    //             MSNet::getInstance()->getInsertionControl().add(vehicle);
    //         }
    //     } catch (ProcessError& e) {
    //         if (vehicle != nullptr) {
    //             MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //         }
    //         throw TraCIException(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add(vehID: &str, routeID: &str, typeID: &str, depart: &str, departLane: &str, departPos: &str, departSpeed: &str, arrivalLane: &str, arrivalPos: &str, arrivalSpeed: &str, fromTaz: &str, toTaz: &str, line: &str, personCapacity: i32, personNumber: i32, departPosLat: &str, arrivalPosLat: &str, speedFactor: f64, insertionChecks: &str) {
        if !MSNet::get_instance().m().get_vehicle_control().get_vehicle(vehID).is_null() {
            panic!("{}", format!("The vehicle '{}' to add already exists.", vehID));
        }
        let mut vehicleParams = SUMOVehicleParameter::new();
        vehicleParams.id = vehID.to_string();
        let vehicleType = MSNet::get_instance().m().get_vehicle_control().get_v_type(typeID, Ptr::null(), false);
        if vehicleType.is_null() {
            panic!("{}", format!("Invalid type '{}' for vehicle '{}'.", typeID, vehID));
        }
        if typeID != "DEFAULT_VEHTYPE" {
            vehicleParams.vtypeid = typeID.to_string();
            vehicleParams.parametersSet |= VEHPARS_VTYPE_SET;
        }
        if SUMOVehicleParserHelper::is_internal_route_id(routeID) {
            write_warning(&format!("Internal routes receive an ID starting with '!' and must not be referenced in other vehicle or flow definitions. Please remove all references to route '{}' in case it is internal.", routeID));
        }
        let mut route = MSRoute::dictionary_3(routeID, Ptr::null());
        if !MSRoute::has_route(routeID) {
            if routeID.is_empty() {
                // assume, route was intentionally left blank because the caller
                // intends to control the vehicle remotely
                let vclass = vehicleType.get_vehicle_class();
                let dummyRouteID = format!("DUMMY_ROUTE_{}", SUMO_VEHICLE_CLASS_STRINGS.get_string(vclass).unwrap());
                if !MSRoute::has_route(&dummyRouteID) {
                    for e in MSEdge::get_all_edges().iter() {
                        if e.get_function() == SumoXMLEdgeFunc::NORMAL && (e.get_permissions() & vclass) == vclass {
                            let edges: Vec<String> = vec![e.get_id().to_string()];
                            super::route::Route::add(&dummyRouteID, &edges);
                            break;
                        }
                    }
                }
                route = MSRoute::dictionary_3(&dummyRouteID, Ptr::null());
                if !MSRoute::has_route(&dummyRouteID) {
                    panic!("{}", format!("Could not build dummy route for vehicle class: '{}'", SUMO_VEHICLE_CLASS_STRINGS.get_string(vehicleType.get_vehicle_class()).unwrap()));
                }
            } else {
                panic!("{}", format!("Invalid route '{}' for vehicle '{}'.", routeID, vehID));
            }
        }
        // check if the route implies a trip
        if route.size() == 2 {
            let edges = route.get_edges();
            if !edges[0].get_successors(SVC_IGNORING).contains(&edges[1]) {
                vehicleParams.parametersSet |= VEHPARS_FORCE_REROUTE;
            }
        }
        if !fromTaz.is_empty() || !toTaz.is_empty() {
            vehicleParams.parametersSet |= VEHPARS_FORCE_REROUTE;
        }
        let mut error = String::new();
        if !SUMOVehicleParameter::parse_depart(depart, "vehicle", vehID, &mut vehicleParams.depart, &mut vehicleParams.departProcedure, &mut error, "depart") {
            panic!("{}", error);
        }
        if vehicleParams.departProcedure == DepartDefinition::GIVEN && vehicleParams.depart < MSNet::get_instance().get_current_time_step() {
            vehicleParams.depart = MSNet::get_instance().get_current_time_step();
            write_warning(&format!("Departure time for vehicle '{}' is in the past; using current time instead.", vehID));
        } else if vehicleParams.departProcedure == DepartDefinition::NOW {
            vehicleParams.depart = MSNet::get_instance().get_current_time_step();
        }
        if !SUMOVehicleParameter::parse_depart_lane(departLane, "vehicle", vehID, &mut vehicleParams.departLane, &mut vehicleParams.departLaneProcedure, &mut error) {
            panic!("{}", error);
        }
        if !SUMOVehicleParameter::parse_depart_pos(departPos, "vehicle", vehID, &mut vehicleParams.departPos, &mut vehicleParams.departPosProcedure, &mut error) {
            panic!("{}", error);
        }
        if !SUMOVehicleParameter::parse_depart_speed(departSpeed, "vehicle", vehID, &mut vehicleParams.departSpeed, &mut vehicleParams.departSpeedProcedure, &mut error) {
            panic!("{}", error);
        }
        if !SUMOVehicleParameter::parse_arrival_lane(arrivalLane, "vehicle", vehID, &mut vehicleParams.arrivalLane, &mut vehicleParams.arrivalLaneProcedure, &mut error) {
            panic!("{}", error);
        }
        if !SUMOVehicleParameter::parse_arrival_pos(arrivalPos, "vehicle", vehID, &mut vehicleParams.arrivalPos, &mut vehicleParams.arrivalPosProcedure, &mut error) {
            panic!("{}", error);
        }
        if !SUMOVehicleParameter::parse_arrival_speed(arrivalSpeed, "vehicle", vehID, &mut vehicleParams.arrivalSpeed, &mut vehicleParams.arrivalSpeedProcedure, &mut error) {
            panic!("{}", error);
        }
        if !departPosLat.is_empty() {
            if !SUMOVehicleParameter::parse_depart_pos_lat(departPosLat, "vehicle", vehID, &mut vehicleParams.departPosLat, &mut vehicleParams.departPosLatProcedure, &mut error) {
                panic!("{}", error);
            }
            vehicleParams.parametersSet |= VEHPARS_DEPARTPOSLAT_SET;
        }
        if !arrivalPosLat.is_empty() {
            if !SUMOVehicleParameter::parse_arrival_pos_lat(arrivalPosLat, "vehicle", vehID, &mut vehicleParams.arrivalPosLat, &mut vehicleParams.arrivalPosLatProcedure, &mut error) {
                panic!("{}", error);
            }
            vehicleParams.parametersSet |= VEHPARS_ARRIVALPOSLAT_SET;
        }
        if speedFactor > 0. {
            vehicleParams.speedFactor = speedFactor;
            vehicleParams.parametersSet |= VEHPARS_SPEEDFACTOR_SET;
        }
        if !insertionChecks.is_empty() {
            vehicleParams.insertionChecks = SUMOVehicleParameter::parse_insertion_checks(insertionChecks);
            vehicleParams.parametersSet |= VEHPARS_INSERTION_CHECKS_SET;
        }
        // mark non-default attributes
        if departLane != "first" {
            vehicleParams.parametersSet |= VEHPARS_DEPARTLANE_SET;
        }
        if departPos != "base" {
            vehicleParams.parametersSet |= VEHPARS_DEPARTPOS_SET;
        }
        if departSpeed != "0" {
            vehicleParams.parametersSet |= VEHPARS_DEPARTSPEED_SET;
        }
        if arrivalLane != "current" {
            vehicleParams.parametersSet |= VEHPARS_ARRIVALLANE_SET;
        }
        if arrivalPos != "max" {
            vehicleParams.parametersSet |= VEHPARS_ARRIVALPOS_SET;
        }
        if arrivalSpeed != "current" {
            vehicleParams.parametersSet |= VEHPARS_ARRIVALSPEED_SET;
        }
        if !fromTaz.is_empty() {
            vehicleParams.parametersSet |= VEHPARS_FROM_TAZ_SET;
        }
        if !toTaz.is_empty() {
            vehicleParams.parametersSet |= VEHPARS_TO_TAZ_SET;
        }
        if !line.is_empty() {
            vehicleParams.parametersSet |= VEHPARS_LINE_SET;
        }
        if personNumber != 0 {
            vehicleParams.parametersSet |= VEHPARS_PERSON_NUMBER_SET;
        }
        // build vehicle
        vehicleParams.fromTaz = fromTaz.to_string();
        vehicleParams.toTaz = toTaz.to_string();
        vehicleParams.line = line.to_string();
        vehicleParams.personNumber = personNumber;
        let departProcedure = vehicleParams.departProcedure;
        // shortcut: the parameters are leaked; C++ hands ownership to the vehicle, which would free them with itself
        let params = Ptr::from_raw(Box::leak(Box::new(vehicleParams)));
        let vehicle = MSNet::get_instance().m().get_vehicle_control().build_vehicle(params, route.clone(), vehicleType, true, MSVehicleControl_VehicleDefinitionSource::LIBSUMO, true);
        if fromTaz.is_empty() && !route.get_edges()[0].validate_depart_speed(vehicle.m()) {
            MSNet::get_instance().m().get_vehicle_control().delete_vehicle(vehicle, true, false);
            panic!("{}", format!("Departure speed for vehicle '{}' is too high for the departure edge '{}'.", vehID, route.get_edges()[0].get_id()));
        }
        let mut msg = String::new();
        if vehicle.m().get_route_validity(true, true, Ptr::from_mut(&mut msg)) != crate::microsim::ms_base_vehicle::MSBaseVehicle_RouteValidity::ROUTE_VALID as i32 {
            MSNet::get_instance().m().get_vehicle_control().delete_vehicle(vehicle, true, false);
            panic!("{}", format!("Vehicle '{}' has no valid route ({}). ", vehID, msg));
        }
        MSNet::get_instance().m().get_vehicle_control().add_vehicle(vehID, vehicle);
        if departProcedure != DepartDefinition::TRIGGERED && departProcedure != DepartDefinition::CONTAINER_TRIGGERED {
            MSNet::get_instance().m().get_insertion_control().add(vehicle);
        }
    }

    /// C++: `void Vehicle::changeTarget(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::changeTarget(const std::string& vehID, const std::string& edgeID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     const MSEdge* destEdge = MSEdge::dictionary_2(edgeID);
    //     const bool onInit = isOnInit(vehID);
    //     if (destEdge == nullptr) {
    //         throw TraCIException("Destination edge '" + edgeID + "' is not known.");
    //     }
    //     // change the final edge of the route and reroute
    //     try {
    //         const bool success = veh->reroute(MSNet::getInstance()->getCurrentTimeStep(), "traci:changeTarget",
    //                                           veh->getRouterTT(), onInit, false, false, destEdge);
    //         if (!success) {
    //             throw TraCIException("ChangeTarget failed for vehicle '" + veh->getID() + "', destination edge '" + edgeID + "' unreachable.");
    //         }
    //     } catch (ProcessError& e) {
    //         throw TraCIException(e.what());
    //     }
    // }
    /// C++ `MSBaseVehicle::getRouterTT() const`; the ms_base_vehicle.rs method is still a stub, so the mode branch is here
    fn router_tt_of(veh: &Ptr<MSBaseVehicle>) -> &'static mut MSVehicleRouter {
        if veh.get_routing_mode() == ROUTING_MODE_AGGREGATED {
            MSRoutingEngine::get_router_tt(veh.get_rng_index(), veh.get_v_class(), &Default::default())
        } else {
            MSNet::get_instance().get_mut().unwrap().get_router_tt(veh.get_rng_index(), &Default::default())
        }
    }

    #[allow(unused_variables)]
    pub fn change_target(vehID: &str, edgeID: &str) {
        let veh = Helper::get_vehicle(vehID);
        let destEdge = MSEdge::dictionary_2(edgeID);
        let onInit = Self::is_on_init(vehID);
        if destEdge.is_null() {
            panic!("{}", format!("Destination edge '{}' is not known.", edgeID));
        }
        // change the final edge of the route and reroute
        let success = veh.m().reroute(MSNet::get_instance().get_current_time_step(), "traci:changeTarget",
                                      Self::router_tt_of(&veh), onInit, false, false, destEdge);
        if !success {
            panic!("{}", format!("ChangeTarget failed for vehicle '{}', destination edge '{}' unreachable.", veh.get_id(), edgeID));
        }
    }

    /// C++: `void Vehicle::changeLane(const std::string &, int, double)` [static]
    // void
    // Vehicle::changeLane(const std::string& vehID, int laneIndex, double duration) {
    //     try {
    //         checkTimeBounds(duration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration parameter exceeds the time value range.");
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("changeLane not applicable for meso");
    //         return;
    //     }
    //
    //     std::vector<std::pair<SUMOTime, int> > laneTimeLine;
    //     laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), laneIndex));
    //     laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(duration), laneIndex));
    //     veh->getInfluencer().setLaneTimeLine(laneTimeLine);
    // }
    #[allow(unused_variables)]
    pub fn change_lane(vehID: &str, laneIndex: i32, duration: f64) {
        if !check_time_bounds(duration) {
            panic!("Duration parameter exceeds the time value range.");
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let mut laneTimeLine: Vec<(SUMOTime, i32)> = Vec::new();
        laneTimeLine.push((MSNet::get_instance().get_current_time_step(), laneIndex));
        laneTimeLine.push((MSNet::get_instance().get_current_time_step() + time2steps(duration), laneIndex));
        veh.m().get_influencer().set_lane_time_line(&laneTimeLine);
    }

    /// C++: `void Vehicle::changeLaneRelative(const std::string &, int, double)` [static]
    // void
    // Vehicle::changeLaneRelative(const std::string& vehID, int indexOffset, double duration) {
    //     try {
    //         checkTimeBounds(duration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration parameter exceeds the time value range.");
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("changeLaneRelative not applicable for meso");
    //         return;
    //     }
    //
    //     std::vector<std::pair<SUMOTime, int> > laneTimeLine;
    //     int laneIndex = veh->getLaneIndex() + indexOffset;
    //     if (laneIndex < 0 && !veh->getLaneChangeModel().isOpposite()) {
    //         if (veh->getLaneIndex() == -1) {
    //             WRITE_WARNINGF(TL("Ignoring changeLaneRelative for vehicle '%' that isn't on the road"), vehID);
    //         } else {
    //             WRITE_WARNINGF(TL("Ignoring indexOffset % for vehicle '%' on laneIndex %."), indexOffset, vehID, veh->getLaneIndex());
    //         }
    //     } else {
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), laneIndex));
    //         laneTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(duration), laneIndex));
    //         veh->getInfluencer().setLaneTimeLine(laneTimeLine);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_lane_relative(vehID: &str, indexOffset: i32, duration: f64) {
        if !check_time_bounds(duration) {
            panic!("Duration parameter exceeds the time value range.");
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let mut laneTimeLine: Vec<(SUMOTime, i32)> = Vec::new();
        let laneIndex = veh.get_lane_index() + indexOffset;
        if laneIndex < 0 && !veh.m().get_lane_change_model().is_opposite() {
            if veh.get_lane_index() == -1 {
                write_warning(&format!("Ignoring changeLaneRelative for vehicle '{}' that isn't on the road", vehID));
            } else {
                write_warning(&format!("Ignoring indexOffset {} for vehicle '{}' on laneIndex {}.", indexOffset, vehID, veh.get_lane_index()));
            }
        } else {
            laneTimeLine.push((MSNet::get_instance().get_current_time_step(), laneIndex));
            laneTimeLine.push((MSNet::get_instance().get_current_time_step() + time2steps(duration), laneIndex));
            veh.m().get_influencer().set_lane_time_line(&laneTimeLine);
        }
    }

    /// C++: `void Vehicle::changeSublane(const std::string &, double)` [static]
    // void
    // Vehicle::changeSublane(const std::string& vehID, double latDist) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("changeSublane not applicable for meso");
    //         return;
    //     }
    //
    //     veh->getInfluencer().setSublaneChange(latDist);
    // }
    #[allow(unused_variables)]
    pub fn change_sublane(vehID: &str, latDist: f64) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        veh.m().get_influencer().set_sublane_change(latDist);
    }

    /// C++: `void Vehicle::slowDown(const std::string &, double, double)` [static]
    // void
    // Vehicle::slowDown(const std::string& vehID, double speed, double duration) {
    //     try {
    //         checkTimeBounds(duration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration parameter exceeds the time value range.");
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("slowDown not applicable for meso");
    //         return;
    //     }
    //
    //     std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //     speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), veh->getSpeed()));
    //     speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(duration), speed));
    //     veh->getInfluencer().setSpeedTimeLine(speedTimeLine);
    // }
    #[allow(unused_variables)]
    pub fn slow_down(vehID: &str, speed: f64, duration: f64) {
        if !check_time_bounds(duration) {
            panic!("Duration parameter exceeds the time value range.");
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
        speedTimeLine.push((MSNet::get_instance().get_current_time_step(), veh.get_speed()));
        speedTimeLine.push((MSNet::get_instance().get_current_time_step() + time2steps(duration), speed));
        veh.m().get_influencer().set_speed_time_line(&speedTimeLine);
    }

    /// C++: `void Vehicle::openGap(const std::string &, double, double, double, double, double, const std::string &)` [static]
    // void
    // Vehicle::openGap(const std::string& vehID, double newTimeHeadway, double newSpaceHeadway, double duration, double changeRate, double maxDecel, const std::string& referenceVehID) {
    //     try {
    //         checkTimeBounds(duration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration parameter exceeds the time value range.");
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("openGap not applicable for meso");
    //         return;
    //     }
    //
    //     MSVehicle* refVeh = nullptr;
    //     if (referenceVehID != "") {
    //         refVeh = dynamic_cast<MSVehicle*>(Helper::getVehicle(referenceVehID));
    //     }
    //     const double originalTau = veh->getVehicleType().getCarFollowModel().getHeadwayTime();
    //     if (newTimeHeadway == -1) {
    //         newTimeHeadway = originalTau;
    //     }
    //     if (originalTau > newTimeHeadway) {
    //         WRITE_WARNING("Ignoring openGap(). New time headway must not be smaller than the original.");
    //         return;
    //     }
    //     veh->getInfluencer().activateGapController(originalTau, newTimeHeadway, newSpaceHeadway, duration, changeRate, maxDecel, refVeh);
    // }
    #[allow(unused_variables)]
    pub fn open_gap(vehID: &str, mut newTimeHeadway: f64, newSpaceHeadway: f64, duration: f64, changeRate: f64, maxDecel: f64, referenceVehID: &str) {
        if !check_time_bounds(duration) {
            panic!("Duration parameter exceeds the time value range.");
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let mut refVeh: Ptr<MSVehicle> = Ptr::default();
        if referenceVehID != "" {
            refVeh = micro_veh(Helper::get_vehicle(referenceVehID));
        }
        let originalTau = veh.get_vehicle_type().get_car_follow_model().get_headway_time();
        if newTimeHeadway == -1. {
            newTimeHeadway = originalTau;
        }
        if originalTau > newTimeHeadway {
            write_warning("Ignoring openGap(). New time headway must not be smaller than the original.");
            return;
        }
        veh.m().get_influencer().activate_gap_controller(originalTau, newTimeHeadway, newSpaceHeadway, duration, changeRate, maxDecel, refVeh);
    }

    /// C++: `void Vehicle::deactivateGapControl(const std::string &)` [static]
    // void
    // Vehicle::deactivateGapControl(const std::string& vehID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("deactivateGapControl not applicable for meso");
    //         return;
    //     }
    //
    //     if (veh->hasInfluencer()) {
    //         veh->getInfluencer().deactivateGapController();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_gap_control(vehID: &str) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if veh.has_influencer() {
            veh.m().get_influencer().deactivate_gap_controller();
        }
    }

    /// C++: `void Vehicle::requestToC(const std::string &, double)` [static]
    // void
    // Vehicle::requestToC(const std::string& vehID, double leadTime) {
    //     setParameter(vehID, "device.toc.requestToC", toString(leadTime));
    // }
    #[allow(unused_variables)]
    pub fn request_to_c(vehID: &str, leadTime: f64) {
        Self::set_parameter(vehID, "device.toc.requestToC", &to_string(&leadTime));
    }

    /// C++: `void Vehicle::setSpeed(const std::string &, double)` [static]
    // void
    // Vehicle::setSpeed(const std::string& vehID, double speed) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("setSpeed not yet implemented for meso");
    //         return;
    //     }
    //
    //     std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //     if (speed >= 0) {
    //         speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), speed));
    //         speedTimeLine.push_back(std::make_pair(SUMOTime_MAX - DELTA_T, speed));
    //     }
    //     veh->getInfluencer().setSpeedTimeLine(speedTimeLine);
    // }
    #[allow(unused_variables)]
    pub fn set_speed(vehID: &str, speed: f64) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
        if speed >= 0. {
            speedTimeLine.push((MSNet::get_instance().get_current_time_step(), speed));
            speedTimeLine.push((SUMOTIME_MAX - delta_t(), speed));
        }
        veh.m().get_influencer().set_speed_time_line(&speedTimeLine);
    }

    /// C++: `void Vehicle::setAcceleration(const std::string &, double, double)` [static]
    // void
    // Vehicle::setAcceleration(const std::string& vehID, double acceleration, double duration) {
    //     try {
    //         checkTimeBounds(duration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("Duration parameter exceeds the time value range.");
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("setAcceleration not yet implemented for meso");
    //         return;
    //     }
    //
    //     double targetSpeed = std::max(veh->getSpeed() + acceleration * duration, 0.0);
    //     std::vector<std::pair<SUMOTime, double>> speedTimeLine;
    //     speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), veh->getSpeed()));
    //     speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(duration), targetSpeed));
    //     veh->getInfluencer().setSpeedTimeLine(speedTimeLine);
    // }
    #[allow(unused_variables)]
    pub fn set_acceleration(vehID: &str, acceleration: f64, duration: f64) {
        if !check_time_bounds(duration) {
            panic!("Duration parameter exceeds the time value range.");
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        let targetSpeed = f64::max(veh.get_speed() + acceleration * duration, 0.0);
        let mut speedTimeLine: Vec<(SUMOTime, f64)> = Vec::new();
        speedTimeLine.push((MSNet::get_instance().get_current_time_step(), veh.get_speed()));
        speedTimeLine.push((MSNet::get_instance().get_current_time_step() + time2steps(duration), targetSpeed));
        veh.m().get_influencer().set_speed_time_line(&speedTimeLine);
    }

    /// C++: `void Vehicle::setPreviousSpeed(const std::string &, double, double)` [static]
    // void
    // Vehicle::setPreviousSpeed(const std::string& vehID, double prevSpeed, double prevAcceleration) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("setPreviousSpeed not yet implemented for meso");
    //         return;
    //     }
    //     if (prevAcceleration == INVALID_DOUBLE_VALUE) {
    //         prevAcceleration = std::numeric_limits<double>::min();
    //     }
    //     veh->setPreviousSpeed(prevSpeed, prevAcceleration);
    // }
    #[allow(unused_variables)]
    pub fn set_previous_speed(vehID: &str, prevSpeed: f64, mut prevAcceleration: f64) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if prevAcceleration == INVALID_DOUBLE_VALUE {
            prevAcceleration = f64::MIN_POSITIVE; // std::numeric_limits<double>::min()
        }
        veh.m().set_previous_speed(prevSpeed, prevAcceleration);
    }

    /// C++: `void Vehicle::setSpeedMode(const std::string &, int)` [static]
    // void
    // Vehicle::setSpeedMode(const std::string& vehID, int speedMode) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("setSpeedMode not yet implemented for meso");
    //         return;
    //     }
    //
    //     veh->getInfluencer().setSpeedMode(speedMode);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_mode(vehID: &str, speedMode: i32) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        veh.m().get_influencer().set_speed_mode(speedMode);
    }

    /// C++: `void Vehicle::setLaneChangeMode(const std::string &, int)` [static]
    // void
    // Vehicle::setLaneChangeMode(const std::string& vehID, int laneChangeMode) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("setLaneChangeMode not applicable for meso");
    //         return;
    //     }
    //
    //     veh->getInfluencer().setLaneChangeMode(laneChangeMode);
    // }
    #[allow(unused_variables)]
    pub fn set_lane_change_mode(vehID: &str, laneChangeMode: i32) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        veh.m().get_influencer().set_lane_change_mode(laneChangeMode);
    }

    /// C++: `void Vehicle::setRoutingMode(const std::string &, int)` [static]
    // void
    // Vehicle::setRoutingMode(const std::string& vehID, int routingMode) {
    //     Helper::getVehicle(vehID)->setRoutingMode(routingMode);
    // }
    #[allow(unused_variables)]
    pub fn set_routing_mode(vehID: &str, routingMode: i32) {
        Helper::get_vehicle(vehID).m().set_routing_mode(routingMode);
    }

    /// C++: `void Vehicle::setType(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setType(const std::string& vehID, const std::string& typeID) {
    //     MSVehicleType* vehicleType = MSNet::getInstance()->getVehicleControl().getVType(typeID);
    //     if (vehicleType == nullptr) {
    //         throw TraCIException("Vehicle type '" + typeID + "' is not known");
    //     }
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     veh->replaceVehicleType(vehicleType);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (microVeh != nullptr && microVeh->isOnRoad()) {
    //         microVeh->updateBestLanes(true);
    //         microVeh->updateLaneBruttoSum();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_type(vehID: &str, typeID: &str) {
        let vehicleType = MSNet::get_instance().m().get_vehicle_control().get_v_type(typeID, Ptr::null(), false);
        if vehicleType.is_null() {
            panic!("{}", format!("Vehicle type '{}' is not known", typeID));
        }
        let veh = Helper::get_vehicle(vehID);
        veh.m().replace_vehicle_type(vehicleType);
        let microVeh = micro_veh(veh);
        if microVeh.is_on_road() {
            microVeh.m().update_best_lanes(true, Ptr::default());
            microVeh.m().update_lane_brutto_sum();
        }
    }

    /// C++: `void Vehicle::setRouteID(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setRouteID(const std::string& vehID, const std::string& routeID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     ConstMSRoutePtr r = MSRoute::dictionary(routeID);
    //     if (r == nullptr) {
    //         throw TraCIException("The route '" + routeID + "' is not known.");
    //     }
    //     if (SUMOVehicleParserHelper::isInternalRouteID(routeID)) {
    //         WRITE_WARNINGF(TL("Internal routes receive an ID starting with '!' and must not be referenced in other vehicle or flow definitions. Please remove all references to route '%' in case it is internal."), routeID);
    //     }
    //     std::string msg;
    //     if (!veh->hasValidRoute(msg, r)) {
    //         WRITE_WARNINGF(TL("Invalid route replacement for vehicle '%'. %"), veh->getID(), msg);
    //         if (false /* shortcut: MSGlobals::gCheckRoutes not ported yet (ms_globals.rs empty), upgrade when MSGlobals is filled */) {
    //             throw TraCIException("Route replacement failed for " + veh->getID());
    //         }
    //     }
    //
    //     std::string errorMsg;
    //     if (!veh->replaceRoute(r, "traci:setRouteID", veh->getLane() == nullptr, 0, true, true, &errorMsg)) {
    //         throw TraCIException("Route replacement failed for vehicle '" + veh->getID() + "' (" + errorMsg + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_route_id(vehID: &str, routeID: &str) {
        let veh = Helper::get_vehicle(vehID);
        if !MSRoute::has_route(routeID) {
            panic!("{}", format!("The route '{}' is not known.", routeID));
        }
        let r = MSRoute::dictionary_3(routeID, Ptr::null());
        if SUMOVehicleParserHelper::is_internal_route_id(routeID) {
            write_warning(&format!("Internal routes receive an ID starting with '!' and must not be referenced in other vehicle or flow definitions. Please remove all references to route '{}' in case it is referenced.", routeID));
        }
        let mut msg = String::new();
        if !veh.has_valid_route(&mut msg, r.clone()) {
            if crate::microsim::ms_globals::check_routes() {
                panic!("{}", format!("Route replacement failed for {}", veh.get_id()));
            }
        }
        let mut errorMsg = String::new();
        if !veh.m().replace_route(r, "traci:setRouteID", veh.get_lane().is_null(), 0, true, true, Ptr::from_mut(&mut errorMsg)) {
            panic!("{}", format!("Route replacement failed for vehicle '{}' ({}).", veh.get_id(), errorMsg));
        }
    }

    /// C++: `void Vehicle::setRoute(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setRoute(const std::string& vehID, const std::string& edgeID) {
    //     setRoute(vehID, std::vector<std::string>({edgeID}));
    // }
    #[allow(unused_variables)]
    pub fn set_route(vehID: &str, edgeID: &str) {
        Self::set_route_2(vehID, &vec![edgeID.to_string()]);
    }

    /// C++: `void Vehicle::setRoute(const std::string &, const std::vector<std::string> &)` [static]
    // void
    // Vehicle::setRoute(const std::string& vehID, const std::vector<std::string>& edgeIDs) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     ConstMSEdgeVector edges;
    //     const bool onInit = veh->getLane() == nullptr;
    //     try {
    //         MSEdge::parseEdgesList(edgeIDs, edges, "<unknown>");
    //         if (edges.size() > 0 && edges.front()->isInternal()) {
    //             if (edges.size() == 1) {
    //                 // avoid crashing due to lack of normal edges in route (#5390)
    //                 edges.push_back(edges.back()->getLanes()[0]->getNextNormal());
    //             } else {
    //                 // avoid internal edge in final route
    //                 if (edges.front() == &veh->getLane()->getEdge()) {
    //                     edges.erase(edges.begin());
    //                 }
    //             }
    //         }
    //     } catch (ProcessError& e) {
    //         throw TraCIException("Invalid edge list for vehicle '" + veh->getID() + "' (" + e.what() + ")");
    //     }
    //     std::string errorMsg;
    //     if (!veh->replaceRouteEdges(edges, -1, 0, "traci:setRoute", onInit, true, true, &errorMsg)) {
    //         throw TraCIException("Route replacement failed for vehicle '" + veh->getID() + "' (" + errorMsg + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_route_2(vehID: &str, edgeIDs: &Vec<String>) {
        let veh = Helper::get_vehicle(vehID);
        let mut edges: ConstMSEdgeVector = ConstMSEdgeVector::new();
        let onInit = veh.get_lane().is_null();
        // MSEdge::parseEdgesList(edgeIDs, edges, "<unknown>") throws ProcessError, rethrown as TraCIException below
        MSEdge::parse_edges_list_2(edgeIDs, &mut edges, "<unknown>");
        if edges.len() > 0 && edges[0].is_internal() {
            if edges.len() == 1 {
                // avoid crashing due to lack of normal edges in route (#5390)
                let next = edges[edges.len() - 1].get_lanes()[0].get_next_normal();
                edges.push(next);
            } else {
                // avoid internal edge in final route
                if edges[0] == Ptr::new(veh.get_lane().get_edge()) {
                    edges.remove(0);
                }
            }
        }
        let mut errorMsg = String::new();
        if !veh.m().replace_route_edges(&mut edges, -1., 0., "traci:setRoute", onInit, true, true, Ptr::from_mut(&mut errorMsg)) {
            panic!("{}", format!("Route replacement failed for vehicle '{}' ({}).", veh.get_id(), errorMsg));
        }
    }

    /// C++: `void Vehicle::setLateralLanePosition(const std::string &, double)` [static]
    // void
    // Vehicle::setLateralLanePosition(const std::string& vehID, double posLat) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh != nullptr) {
    //         veh->setLateralPositionOnLane(posLat);
    //     } else {
    //         WRITE_ERROR("setLateralLanePosition not applicable for meso");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_lateral_lane_position(vehID: &str, posLat: f64) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        veh.m().set_lateral_position_on_lane(posLat);
    }

    /// C++: `void Vehicle::updateBestLanes(const std::string &)` [static]
    // void
    // Vehicle::updateBestLanes(const std::string& vehID) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("updateBestLanes not applicable for meso");
    //         return;
    //     }
    //     if (veh->isOnRoad()) {
    //         veh->updateBestLanes(true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_best_lanes(vehID: &str) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if veh.is_on_road() {
            veh.m().update_best_lanes(true, Ptr::default());
        }
    }

    /// C++: `void Vehicle::setAdaptedTraveltime(const std::string &, const std::string &, double, double, double)` [static]
    // void
    // Vehicle::setAdaptedTraveltime(const std::string& vehID, const std::string& edgeID,
    //                               double time, double begSeconds, double endSeconds) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSEdge* edge = MSEdge::dictionary_2(edgeID);
    //     if (edge == nullptr) {
    //         throw TraCIException("Edge '" + edgeID + "' is not known.");
    //     }
    //     if (time != INVALID_DOUBLE_VALUE) {
    //         // add time
    //         if (begSeconds == 0 && endSeconds == std::numeric_limits<double>::max()) {
    //             // clean up old values before setting whole range
    //             while (veh->getWeightsStorage().knowsTravelTime(edge)) {
    //                 veh->getWeightsStorage().removeTravelTime(edge);
    //             }
    //         }
    //         veh->getWeightsStorage().addTravelTime(edge, begSeconds, endSeconds, time);
    //     } else {
    //         // remove time
    //         while (veh->getWeightsStorage().knowsTravelTime(edge)) {
    //             veh->getWeightsStorage().removeTravelTime(edge);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_adapted_traveltime(vehID: &str, edgeID: &str, time: f64, begSeconds: f64, endSeconds: f64) {
        let mut veh = Helper::get_vehicle(vehID);
        let edge = MSEdge::dictionary_2(edgeID);
        if edge.is_null() {
            panic!("{}", format!("Edge '{}' is not known.", edgeID));
        }
        if time != INVALID_DOUBLE_VALUE {
            // add time
            if begSeconds == 0. && endSeconds == f64::MAX {
                // clean up old values before setting whole range
                while veh.get_weights_storage_2().knows_travel_time(edge) {
                    veh.m().get_weights_storage_2().remove_travel_time(edge);
                }
            }
            veh.m().get_weights_storage_2().add_travel_time(edge, begSeconds, endSeconds, time);
        } else {
            // remove time
            while veh.get_weights_storage_2().knows_travel_time(edge) {
                veh.m().get_weights_storage_2().remove_travel_time(edge);
            }
        }
    }

    /// C++: `void Vehicle::setEffort(const std::string &, const std::string &, double, double, double)` [static]
    // void
    // Vehicle::setEffort(const std::string& vehID, const std::string& edgeID,
    //                    double effort, double begSeconds, double endSeconds) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSEdge* edge = MSEdge::dictionary_2(edgeID);
    //     if (edge == nullptr) {
    //         throw TraCIException("Edge '" + edgeID + "' is not known.");
    //     }
    //     if (effort != INVALID_DOUBLE_VALUE) {
    //         // add effort
    //         if (begSeconds == 0 && endSeconds == std::numeric_limits<double>::max()) {
    //             // clean up old values before setting whole range
    //             while (veh->getWeightsStorage().knowsEffort(edge)) {
    //                 veh->getWeightsStorage().removeEffort(edge);
    //             }
    //         }
    //         veh->getWeightsStorage().addEffort(edge, begSeconds, endSeconds, effort);
    //     } else {
    //         // remove effort
    //         while (veh->getWeightsStorage().knowsEffort(edge)) {
    //             veh->getWeightsStorage().removeEffort(edge);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_effort(vehID: &str, edgeID: &str, effort: f64, begSeconds: f64, endSeconds: f64) {
        let mut veh = Helper::get_vehicle(vehID);
        let edge = MSEdge::dictionary_2(edgeID);
        if edge.is_null() {
            panic!("{}", format!("Edge '{}' is not known.", edgeID));
        }
        if effort != INVALID_DOUBLE_VALUE {
            // add effort
            if begSeconds == 0. && endSeconds == f64::MAX {
                // clean up old values before setting whole range
                while veh.get_weights_storage_2().knows_effort(edge) {
                    veh.m().get_weights_storage_2().remove_effort(edge);
                }
            }
            veh.m().get_weights_storage_2().add_effort(edge, begSeconds, endSeconds, effort);
        } else {
            // remove effort
            while veh.get_weights_storage_2().knows_effort(edge) {
                veh.m().get_weights_storage_2().remove_effort(edge);
            }
        }
    }

    /// C++: `void Vehicle::rerouteTraveltime(const std::string &, const bool)` [static]
    // void
    // Vehicle::rerouteTraveltime(const std::string& vehID, const bool currentTravelTimes) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     const int routingMode = veh->getRoutingMode();
    //     if (currentTravelTimes && routingMode == ROUTING_MODE_DEFAULT) {
    //         veh->setRoutingMode(ROUTING_MODE_AGGREGATED_CUSTOM);
    //     }
    //     veh->reroute(MSNet::getInstance()->getCurrentTimeStep(), "traci:rerouteTraveltime",
    //                  veh->getRouterTT(), isOnInit(vehID));
    //     if (currentTravelTimes && routingMode == ROUTING_MODE_DEFAULT) {
    //         veh->setRoutingMode(routingMode);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute_traveltime(vehID: &str, currentTravelTimes: bool) {
        let veh = Helper::get_vehicle(vehID);
        let routingMode = veh.get_routing_mode();
        if currentTravelTimes && routingMode == ROUTING_MODE_DEFAULT {
            veh.m().set_routing_mode(ROUTING_MODE_AGGREGATED_CUSTOM);
        }
        veh.m().reroute(MSNet::get_instance().get_current_time_step(), "traci:rerouteTraveltime",
                        Self::router_tt_of(&veh), Self::is_on_init(vehID), false, false, Ptr::default());
        if currentTravelTimes && routingMode == ROUTING_MODE_DEFAULT {
            veh.m().set_routing_mode(routingMode);
        }
    }

    /// C++: `void Vehicle::rerouteEffort(const std::string &)` [static]
    // void
    // Vehicle::rerouteEffort(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     veh->reroute(MSNet::getInstance()->getCurrentTimeStep(), "traci:rerouteEffort",
    //                  MSNet::getInstance()->getRouterEffort(veh->getRNGIndex()), isOnInit(vehID));
    // }
    #[allow(unused_variables)]
    pub fn reroute_effort(vehID: &str) {
        let veh = Helper::get_vehicle(vehID);
        veh.m().reroute(MSNet::get_instance().get_current_time_step(), "traci:rerouteEffort",
                        MSNet::get_instance().get_router_effort(veh.get_rng_index(), &Default::default()), Self::is_on_init(vehID), false, false, Ptr::default());
    }

    /// C++: `void Vehicle::setSignals(const std::string &, int)` [static]
    // void
    // Vehicle::setSignals(const std::string& vehID, int signals) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("setSignals not applicable for meso");
    //         return;
    //     }
    //
    //     // set influencer to make the change persistent
    //     veh->getInfluencer().setSignals(signals);
    //     // set them now so that getSignals returns the correct value
    //     veh->switchOffSignal(0x0fffffff);
    //     if (signals >= 0) {
    //         veh->switchOnSignal(signals);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_signals(vehID: &str, signals: i32) {
        let veh = micro_veh(Helper::get_vehicle(vehID));
        // set influencer to make the change persistent
        veh.m().get_influencer().set_signals(signals);
        // set them now so that getSignals returns the correct value
        veh.m().switch_off_signal(0x0fffffff);
        if signals >= 0 {
            veh.m().switch_on_signal(signals);
        }
    }

    /// C++: `void Vehicle::moveTo(const std::string &, const std::string &, double, int)` [static]
    // void
    // Vehicle::moveTo(const std::string& vehID, const std::string& laneID, double pos, int reason) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("moveTo not yet implemented for meso");
    //         return;
    //     }
    //
    //     MSLane* l = MSLane::dictionary_2(laneID);
    //     if (l == nullptr) {
    //         throw TraCIException("Unknown lane '" + laneID + "'.");
    //     }
    //     if (veh->getLane() == l) {
    //         veh->setTentativeLaneAndPosition(l, pos, veh->getLateralPositionOnLane());
    //         return;
    //     }
    //     MSEdge* destinationEdge = &l->getEdge();
    //     const MSEdge* destinationRouteEdge = destinationEdge->getNormalBefore();
    //     if (!veh->isOnRoad() && veh->getParameter().was_set(VEHPARS_FORCE_REROUTE) && veh->getRoute().getEdges().size() == 2) {
    //         // it's a trip that wasn't routeted yet (likely because the vehicle was added in this step. Find a route now
    //         veh->reroute(MSNet::getInstance()->getCurrentTimeStep(), "traci:moveTo-tripInsertion",
    //                      veh->getRouterTT(), true);
    //     }
    //     // find edge in the remaining route
    //     MSRouteIterator it = std::find(veh->getCurrentRouteEdge(), veh->getRoute().end(), destinationRouteEdge);
    //     if (it == veh->getRoute().end()) {
    //         // find edge in the edges that were already passed
    //         it = std::find(veh->getRoute().begin(), veh->getRoute().end(), destinationRouteEdge);
    //     }
    //     if (it == veh->getRoute().end() ||
    //             // internal edge must continue the route
    //             (destinationEdge->isInternal() &&
    //              ((it + 1) == veh->getRoute().end()
    //               || l->getNextNormal() != *(it + 1)))) {
    //         throw TraCIException("Lane '" + laneID + "' is not on the route of vehicle '" + vehID + "'.");
    //     }
    //     Position oldPos = vehicle->getPosition();
    //     veh->onRemovalFromNet(MSMoveReminder::NOTIFICATION_TELEPORT);
    //     if (veh->getLane() != nullptr) {
    //         // correct odometer which gets incremented via onRemovalFromNet->leaveLane
    //         veh->addToOdometer(-veh->getLane()->getLength());
    //         veh->getMutableLane()->removeVehicle(veh, MSMoveReminder::NOTIFICATION_TELEPORT, false);
    //     } else {
    //         veh->setTentativeLaneAndPosition(l, pos);
    //     }
    //     const int oldRouteIndex = veh->getRoutePosition();
    //     const int newRouteIndex = (int)(it - veh->getRoute().begin());
    //     if (oldRouteIndex > newRouteIndex) {
    //         // more odometer correction needed
    //         veh->addToOdometer(-l->getLength());
    //     }
    //     veh->resetRoutePosition(newRouteIndex, veh->getParameter().departLaneProcedure);
    //     if (!veh->isOnRoad()) {
    //         MSNet::getInstance()->getInsertionControl().alreadyDeparted(veh);
    //         MSVehicleTransfer::getInstance()->remove(veh);
    //     }
    //     MSMoveReminder::Notification moveReminderReason;
    //     if (veh->hasDeparted()) {
    //         if (reason == MOVE_TELEPORT) {
    //             moveReminderReason = MSMoveReminder::NOTIFICATION_TELEPORT;
    //         } else if (reason == MOVE_NORMAL) {
    //             moveReminderReason = MSMoveReminder::NOTIFICATION_JUNCTION;
    //         } else if (reason == MOVE_AUTOMATIC) {
    //             Position newPos = l->geometryPositionAtOffset(pos);
    //             const double dist = newPos.distanceTo2D(oldPos);
    //             if (dist < SPEED2DIST(veh->getMaxSpeed())) {
    //                 moveReminderReason = MSMoveReminder::NOTIFICATION_JUNCTION;
    //             } else {
    //                 moveReminderReason = MSMoveReminder::NOTIFICATION_TELEPORT;
    //             }
    //         } else {
    //             throw TraCIException("Invalid moveTo reason '" + toString(reason) + "' for vehicle '" + vehID + "'.");
    //         }
    //     } else {
    //         moveReminderReason = MSMoveReminder::NOTIFICATION_DEPARTED;
    //     }
    //     l->forceVehicleInsertion(veh, pos, moveReminderReason);
    //     if (veh->getLaneChangeModel().isOpposite()) {
    //         veh->getLaneChangeModel().changedToOpposite();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to(vehID: &str, laneID: &str, pos: f64, reason: i32) {
        let vehicle = Helper::get_vehicle(vehID);
        let veh = micro_veh(vehicle);
        let mut l = MSLane::dictionary_2(laneID);
        if l.is_null() {
            panic!("{}", format!("Unknown lane '{}'.", laneID));
        }
        if veh.get_lane() == l {
            veh.m().set_tentative_lane_and_position(l, pos, veh.get_lateral_position_on_lane());
            return;
        }
        let destinationEdge = l.get_edge();
        let destinationRouteEdge = destinationEdge.get_normal_before();
        if !veh.is_on_road() && veh.get_parameter().was_set(VEHPARS_FORCE_REROUTE) && veh.get_route().get_edges().len() == 2 {
            // it's a trip that wasn't routeted yet (likely because the vehicle was added in this step. Find a route now
            veh.m().reroute(MSNet::get_instance().get_current_time_step(), "traci:moveTo-tripInsertion",
                            Self::router_tt_of(&veh.cast::<MSBaseVehicle>()), true, false, false, Ptr::default());
        }
        // find edge in the remaining route
        let edges = veh.get_route().get_edges();
        let start = veh.get_route_position() as usize;
        let mut it: Option<usize> = edges[start..].iter().position(|e| *e == destinationRouteEdge).map(|p| p + start);
        if it.is_none() {
            // find edge in the edges that were already passed
            it = edges.iter().position(|e| *e == destinationRouteEdge);
        }
        let invalid = match it {
            None => true,
            Some(i) => destinationEdge.is_internal() && (i + 1 == edges.len() || l.get_next_normal() != edges[i + 1]),
        };
        if invalid {
            panic!("{}", format!("Lane '{}' is not on the route of vehicle '{}'.", laneID, vehID));
        }
        let it = it.unwrap();
        let oldPos = veh.get_position(0.);
        veh.m().on_removal_from_net(MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
        if !veh.get_lane().is_null() {
            // correct odometer which gets incremented via onRemovalFromNet->leaveLane
            veh.m().add_to_odometer(-veh.get_lane().get_length());
            veh.m().get_mutable_lane().remove_vehicle(veh, MSMoveReminder_Notification::NOTIFICATION_TELEPORT, false);
        } else {
            veh.m().set_tentative_lane_and_position(l, pos, 0.);
        }
        let oldRouteIndex = veh.get_route_position();
        let newRouteIndex = it as i32;
        if oldRouteIndex > newRouteIndex {
            // more odometer correction needed
            veh.m().add_to_odometer(-l.get_length());
        }
        veh.m().reset_route_position(newRouteIndex, veh.get_parameter().departLaneProcedure);
        if !veh.is_on_road() {
            MSNet::get_instance().get_insertion_control().already_departed(veh.cast::<SUMOVehicle>());
            MSVehicleTransfer::get_instance().remove(veh);
        }
        let moveReminderReason;
        if veh.has_departed() {
            if reason == MOVE_TELEPORT {
                moveReminderReason = MSMoveReminder_Notification::NOTIFICATION_TELEPORT;
            } else if reason == MOVE_NORMAL {
                moveReminderReason = MSMoveReminder_Notification::NOTIFICATION_JUNCTION;
            } else if reason == MOVE_AUTOMATIC {
                let newPos = l.geometry_position_at_offset(pos, 0.);
                let dist = newPos.distance_to2d(&oldPos);
                if dist < (veh.get_max_speed() * (delta_t() as f64 / 1000.)) {
                    moveReminderReason = MSMoveReminder_Notification::NOTIFICATION_JUNCTION;
                } else {
                    moveReminderReason = MSMoveReminder_Notification::NOTIFICATION_TELEPORT;
                }
            } else {
                panic!("{}", format!("Invalid moveTo reason '{}' for vehicle '{}'.", reason, vehID));
            }
        } else {
            moveReminderReason = MSMoveReminder_Notification::NOTIFICATION_DEPARTED;
        }
        l.force_vehicle_insertion(veh, pos, moveReminderReason, 0.);
        if veh.m().get_lane_change_model().is_opposite() {
            veh.m().get_lane_change_model().changed_to_opposite();
        }
    }

    /// C++: `void Vehicle::moveToXY(const std::string &, const std::string &, const int, const double, const double, double, const int, double)` [static]
    // void
    // Vehicle::moveToXY(const std::string& vehID, const std::string& edgeID, const int laneIndex,
    //                   const double x, const double y, double angle, const int keepRoute, double matchThreshold) {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_WARNING("moveToXY not yet implemented for meso");
    //         return;
    //     }
    //     const bool doKeepRoute = (keepRoute & 1) != 0 && veh->getID() != "VTD_EGO";
    //     const bool mayLeaveNetwork = (keepRoute & 2) != 0;
    //     const bool ignorePermissions = (keepRoute & 4) != 0;
    //     const bool setLateralPos = (MSGlobals::gLateralResolution > 0 || mayLeaveNetwork);
    //     SUMOVehicleClass vClass = ignorePermissions ? SVC_IGNORING : veh->getVClass();
    //     // process
    //     const std::string origID = edgeID + "_" + toString(laneIndex);
    //     // @todo add an interpretation layer for OSM derived origID values (without lane index)
    //     Position pos(x, y);
    // #ifdef DEBUG_MOVEXY
    //     const double origAngle = angle;
    // #endif
    //     // angle must be in [0,360] because it will be compared against those returned by naviDegree()
    //     // angle set to INVALID_DOUBLE_VALUE is ignored in the evaluated and later set to the angle of the matched lane
    //     if (angle != INVALID_DOUBLE_VALUE) {
    //         while (angle >= 360.) {
    //             angle -= 360.;
    //         }
    //         while (angle < 0.) {
    //             angle += 360.;
    //         }
    //     }
    //
    // #ifdef DEBUG_MOVEXY
    //     std::cout << std::endl << SIMTIME << " moveToXY veh=" << veh->getID() << " vehPos=" << veh->getPosition()
    //               << " lane=" << Named::getIDSecure(veh->getLane()) << " lanePos=" << vehicle->getPositionOnLane() << std::endl;
    //     std::cout << " wantedPos=" << pos << " origID=" << origID << " laneIndex=" << laneIndex << " origAngle=" << origAngle << " angle=" << angle << " keepRoute=" << keepRoute << std::endl;
    // #endif
    //
    //     ConstMSEdgeVector edges;
    //     MSLane* lane = nullptr;
    //     double lanePos;
    //     double lanePosLat = 0;
    //     double bestDistance = std::numeric_limits<double>::max();
    //     int routeOffset = 0;
    //     bool found;
    //     double maxRouteDistance = matchThreshold;
    //     /* EGO vehicle is known to have a fixed route. @todo make this into a parameter of the TraCI call */
    //     if (doKeepRoute) {
    //         // case a): vehicle is on its earlier route
    //         //  we additionally assume it is moving forward (SUMO-limit);
    //         //  note that the route ("edges") is not changed in this case
    //
    //         found = Helper::moveToXYMap_matchingRoutePosition(pos, origID,
    //                 veh->getRoute().getEdges(), veh->getRoutePosition(),
    //                 vClass, setLateralPos,
    //                 bestDistance, &lane, lanePos, routeOffset);
    //         // @note silenty ignoring mapping failure
    //     } else {
    //         const double speed = pos.distanceTo2D(veh->getPosition()); // !!!veh->getSpeed();
    //         found = Helper::moveToXYMap(pos, maxRouteDistance, mayLeaveNetwork, origID, angle,
    //                                     speed, veh->getRoute().getEdges(), veh->getRoutePosition(), veh->getLane(), veh->getPositionOnLane(), veh->isOnRoad(),
    //                                     vClass, GeomHelper::naviDegree(veh->getAngle()), setLateralPos,
    //                                     bestDistance, &lane, lanePos, routeOffset, edges);
    //     }
    //     if ((found && bestDistance <= maxRouteDistance) || mayLeaveNetwork) {
    //         // optionally compute lateral offset
    //         pos.setz(veh->getPosition().z());
    //         if (found && setLateralPos) {
    //             const double perpDist = lane->getShape().distance2D(pos, false);
    //             if (perpDist != GeomHelper::INVALID_OFFSET) {
    //                 lanePosLat = perpDist;
    //                 if (!mayLeaveNetwork) {
    //                     lanePosLat = MIN2(lanePosLat, 0.5 * (lane->getWidth() + veh->getVehicleType().getWidth() - MSGlobals::gLateralResolution));
    //                 }
    //                 // figure out whether the offset is to the left or to the right
    //                 PositionVector tmp = lane->getShape();
    //                 try {
    //                     tmp.move2side(-lanePosLat); // moved to left
    //                 } catch (ProcessError&) {
    //                     WRITE_WARNINGF(TL("Could not determine position on lane '%' at lateral position %."), lane->getID(), toString(-lanePosLat));
    //                 }
    //                 //std::cout << " lane=" << lane->getID() << " posLat=" << lanePosLat << " shape=" << lane->getShape() << " tmp=" << tmp << " tmpDist=" << tmp.distance2D(pos) << "\n";
    //                 if (tmp.distance2D(pos) > perpDist) {
    //                     lanePosLat = -lanePosLat;
    //                 }
    //             }
    //             pos.setz(lane->geometryPositionAtOffset(lanePos).z());
    //         }
    //         if (found && !mayLeaveNetwork && MSGlobals::gLateralResolution < 0) {
    //             // mapped position may differ from pos
    //             pos = lane->geometryPositionAtOffset(lanePos, -lanePosLat);
    //         }
    //         assert((found && lane != 0) || (!found && lane == 0));
    //         assert(!std::isnan(lanePos));
    //         if (angle == INVALID_DOUBLE_VALUE) {
    //             if (lane != nullptr) {
    //                 angle = GeomHelper::naviDegree(lane->getShape().rotationAtOffset(lanePos));
    //             } else {
    //                 // compute angle outside road network from old and new position
    //                 angle = GeomHelper::naviDegree(veh->getPosition().angleTo2D(pos));
    //             }
    //         }
    //         // use the best we have
    // #ifdef DEBUG_MOVEXY
    //         std::cout << SIMTIME << " veh=" << vehID + " moveToXYResult lane='" << Named::getIDSecure(lane) << "' lanePos=" << lanePos << " lanePosLat=" << lanePosLat << "\n";
    // #endif
    //         Helper::setRemoteControlled(veh, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, MSNet::getInstance()->getCurrentTimeStep());
    //         if (!veh->isOnRoad()) {
    //             MSNet::getInstance()->getInsertionControl().alreadyDeparted(veh);
    //         }
    //     } else {
    //         if (lane == nullptr) {
    //             throw TraCIException("Could not map vehicle '" + vehID + "', no road found within " + toString(maxRouteDistance) + "m.");
    //         } else {
    //             throw TraCIException("Could not map vehicle '" + vehID + "', distance to road is " + toString(bestDistance) + ".");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to_xy(vehID: &str, edgeID: &str, laneIndex: i32, x: f64, y: f64, mut angle: f64, keepRoute: i32, mut matchThreshold: f64) {
        let mut veh = micro_veh(Helper::get_vehicle(vehID));
        if veh.is_null() {
            utils::common::msg_handler::write_warning("moveToXY not yet implemented for meso");
            return;
        }
        let doKeepRoute = (keepRoute & 1) != 0 && veh.get_id() != "VTD_EGO";
        let mayLeaveNetwork = (keepRoute & 2) != 0;
        let ignorePermissions = (keepRoute & 4) != 0;
        let setLateralPos = MSGlobals::lateral_resolution() > 0. || mayLeaveNetwork;
        let vClass = if ignorePermissions { SVC_IGNORING } else { veh.get_v_class() };
        let origID = format!("{}_{}", edgeID, laneIndex);
        let mut pos = Position::new(x, y);
        if angle != INVALID_DOUBLE_VALUE {
            while angle >= 360. {
                angle -= 360.;
            }
            while angle < 0. {
                angle += 360.;
            }
        }

        let mut edges: ConstMSEdgeVector = Vec::new();
        let mut lane: Ptr<MSLane> = Ptr::null();
        let mut lanePos: f64 = 0.;
        let mut lanePosLat: f64 = 0.;
        let mut bestDistance: f64 = f64::MAX;
        let mut routeOffset: i32 = 0;
        let found: bool;
        let maxRouteDistance = matchThreshold;
        if doKeepRoute {
            found = Helper::move_to_xy_map_matching_route_position(&pos, &origID, veh.get_route().get_edges(), veh.get_route_position(), vClass, setLateralPos, &mut bestDistance, Ptr::from_mut(&mut lane), &mut lanePos, &mut routeOffset);
        } else {
            let speed = pos.distance_to2d(&veh.get_position(0.));
            found = Helper::move_to_xy_map(&pos, maxRouteDistance, mayLeaveNetwork, &origID, angle, speed, veh.get_route().get_edges(), veh.get_route_position(), veh.get_lane(), veh.get_position_on_lane(), veh.is_on_road(), vClass, geom_helper::navi_degree(veh.get_angle()), setLateralPos, &mut bestDistance, Ptr::from_mut(&mut lane), &mut lanePos, &mut routeOffset, &mut edges);
        }
        if (found && bestDistance <= maxRouteDistance) || mayLeaveNetwork {
            pos.setz(veh.get_position(0.).z());
            if found && setLateralPos {
                let perpDist = lane.get_shape(false).distance2d(&pos, false);
                if perpDist != geom_helper::INVALID_OFFSET {
                    lanePosLat = perpDist;
                    if !mayLeaveNetwork {
                        lanePosLat = f64::min(lanePosLat, 0.5 * (lane.get_width() + veh.get_vehicle_type().get_width() - MSGlobals::lateral_resolution()));
                    }
                    let mut tmp = lane.get_shape(false).clone();
                    // shortcut: move2side has no Result in this port, so the ProcessError warning at Vehicle.cpp:1856 cannot fire; upgrade when move2side reports failure
                    tmp.move2side(-lanePosLat, 100.);
                    if tmp.distance2d(&pos, false) > perpDist {
                        lanePosLat = -lanePosLat;
                    }
                }
                pos.setz(lane.geometry_position_at_offset(lanePos, 0.).z());
            }
            if found && !mayLeaveNetwork && MSGlobals::lateral_resolution() < 0. {
                pos = lane.geometry_position_at_offset(lanePos, -lanePosLat);
            }
            assert!((found && !lane.is_null()) || (!found && lane.is_null()));
            assert!(!lanePos.is_nan());
            if angle == INVALID_DOUBLE_VALUE {
                if !lane.is_null() {
                    angle = geom_helper::navi_degree(lane.get_shape(false).rotation_at_offset(lanePos));
                } else {
                    angle = geom_helper::navi_degree(veh.get_position(0.).angle_to2d(&pos));
                }
            }
            Helper::set_remote_controlled(veh, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, MSNet::get_instance().get_current_time_step());
            if !veh.is_on_road() {
                MSNet::get_instance().m().get_insertion_control().already_departed(veh.cast::<SUMOVehicle>());
            }
        } else {
            if lane.is_null() {
                panic!("Could not map vehicle '{}', no road found within {}m.", vehID, utils::common::to_string::to_string(&maxRouteDistance));
            } else {
                panic!("Could not map vehicle '{}', distance to road is {}.", vehID, utils::common::to_string::to_string(&bestDistance));
            }
        }
    }

    /// C++: `void Vehicle::remove(const std::string &, char)` [static]
    // void
    // Vehicle::remove(const std::string& vehID, char reason) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSMoveReminder::Notification n = MSMoveReminder::NOTIFICATION_ARRIVED;
    //     switch (reason) {
    //         case REMOVE_TELEPORT:
    //             // XXX semantics unclear
    //             // n = MSMoveReminder::NOTIFICATION_TELEPORT;
    //             n = MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED;
    //             break;
    //         case REMOVE_PARKING:
    //             // XXX semantics unclear
    //             // n = MSMoveReminder::NOTIFICATION_PARKING;
    //             n = MSMoveReminder::NOTIFICATION_ARRIVED;
    //             break;
    //         case REMOVE_ARRIVED:
    //             n = MSMoveReminder::NOTIFICATION_ARRIVED;
    //             break;
    //         case REMOVE_VAPORIZED:
    //             n = MSMoveReminder::NOTIFICATION_VAPORIZED_TRACI;
    //             break;
    //         case REMOVE_TELEPORT_ARRIVED:
    //             n = MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED;
    //             break;
    //         default:
    //             throw TraCIException("Unknown removal status.");
    //     }
    //     if (veh->hasDeparted()) {
    //         veh->onRemovalFromNet(n);
    //         MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //         if (microVeh != nullptr) {
    //             if (veh->getLane() != nullptr) {
    //                 microVeh->getMutableLane()->removeVehicle(dynamic_cast<MSVehicle*>(veh), n);
    //             }
    //             MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         }
    //         MSNet::getInstance()->getVehicleControl().removePending();
    //     } else {
    //         MSNet::getInstance()->getInsertionControl().alreadyDeparted(veh);
    //         MSNet::getInstance()->getVehicleControl().deleteVehicle(veh, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove(vehID: &str, reason: i32) {
        let veh = Helper::get_vehicle(vehID);
        let n = match reason {
            REMOVE_TELEPORT => MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED,
            REMOVE_PARKING => MSMoveReminder_Notification::NOTIFICATION_ARRIVED,
            REMOVE_ARRIVED => MSMoveReminder_Notification::NOTIFICATION_ARRIVED,
            REMOVE_VAPORIZED => MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_TRACI,
            REMOVE_TELEPORT_ARRIVED => MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED,
            _ => panic!("Unknown removal status."),
        };
        if veh.has_departed() {
            veh.m().on_removal_from_net(n);
            let microVeh = micro_veh(veh);
            if !veh.get_lane().is_null() {
                microVeh.m().get_mutable_lane().remove_vehicle(microVeh, n, true);
            }
            MSNet::get_instance().get_vehicle_control().schedule_vehicle_removal(veh.cast::<SUMOVehicle>(), false);
            MSNet::get_instance().get_vehicle_control().remove_pending();
        } else {
            MSNet::get_instance().get_insertion_control().already_departed(veh.cast::<SUMOVehicle>());
            MSNet::get_instance().get_vehicle_control().delete_vehicle(veh.cast::<SUMOVehicle>(), true, false);
        }
    }

    /// C++: `void Vehicle::setLine(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setLine(const std::string& vehID, const std::string& line) {
    //     Helper::getVehicle(vehID)->getParameter().line = line;
    // }
    #[allow(unused_variables)]
    pub fn set_line(vehID: &str, line: &str) {
        param_mut(Helper::get_vehicle(vehID).get_parameter()).line = line.to_string();
    }

    /// C++: `void Vehicle::setVia(const std::string &, const std::vector<std::string> &)` [static]
    // void
    // Vehicle::setVia(const std::string& vehID, const std::vector<std::string>& edgeList) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     try {
    //         // ensure edges exist
    //         ConstMSEdgeVector edges;
    //         MSEdge::parseEdgesList(edgeList, edges, "<via-edges>");
    //     } catch (ProcessError& e) {
    //         throw TraCIException(e.what());
    //     }
    //     veh->getParameter().via = edgeList;
    // }
    #[allow(unused_variables)]
    pub fn set_via(vehID: &str, edgeList: &Vec<String>) {
        let veh = Helper::get_vehicle(vehID);
        // ensure edges exist (ProcessError from parseEdgesList is rethrown as TraCIException)
        let mut edges: ConstMSEdgeVector = ConstMSEdgeVector::new();
        MSEdge::parse_edges_list_2(edgeList, &mut edges, "<via-edges>");
        param_mut(veh.get_parameter()).via = edgeList.clone();
    }

    /// C++: `void Vehicle::highlight(const std::string &, const libsumo::TraCIColor &, double, const int, const double, const int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn highlight(vehID: &str, col: &TraCIColor, size: f64, alphaMax: i32, duration: f64, type_: i32) {
        // shortcut: Polygon::exists / Polygon::addHighlightPolygon / Polygon::addDynamics (libsumo/Polygon, GUI highlight) are not ported in this pass; upgrade when polygon.rs is filled
        panic!("Vehicle::highlight not yet ported: depends on Polygon highlight helpers")
    }

    /// C++: `void Vehicle::dispatchTaxi(const std::string &, const std::vector<std::string> &)` [static]
    // void
    // Vehicle::dispatchTaxi(const std::string& vehID,  const std::vector<std::string>& reservations) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSDevice_Taxi* taxi = static_cast<MSDevice_Taxi*>(veh->getDevice(typeid(MSDevice_Taxi)));
    //     if (!veh->hasDeparted()) {
    //         throw TraCIException("Vehicle '" + vehID + "' has not yet departed");
    //     }
    //     if (taxi == nullptr) {
    //         throw TraCIException("Vehicle '" + vehID + "' is not a taxi");
    //     }
    //     MSDispatch* dispatcher = MSDevice_Taxi::getDispatchAlgorithm();
    //     if (dispatcher == nullptr) {
    //         throw TraCIException("Cannot dispatch taxi because no reservations have been made");
    //     }
    //     MSDispatch_TraCI* traciDispatcher = dynamic_cast<MSDispatch_TraCI*>(dispatcher);
    //     if (traciDispatcher == nullptr) {
    //         throw TraCIException("device.taxi.dispatch-algorithm 'traci' has not been loaded");
    //     }
    //     if (reservations.size() == 0) {
    //         throw TraCIException("No reservations have been specified for vehicle '" + vehID + "'");
    //     }
    //     try {
    //         traciDispatcher->interpretDispatch(taxi, reservations);
    //     } catch (InvalidArgument& e) {
    //         throw TraCIException("Could not interpret reservations for vehicle '" + vehID + "' (" + e.what() + ").");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dispatch_taxi(vehID: &str, reservations: &Vec<String>) {
        // shortcut: MSDevice_Taxi::getDispatchAlgorithm and MSDispatch_TraCI::interpretDispatch are not ported in this pass; upgrade when ms_dispatch.rs is filled
        panic!("Vehicle::dispatchTaxi not yet ported: depends on MSDispatch_TraCI")
    }

    /// C++: `void Vehicle::setLength(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setLength(const std::string& vehID, double length) {
    //     Helper::getVehicle(vehID)->getSingularType().setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(vehID: &str, length: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_length(&length);
    }

    /// C++: `void Vehicle::setMaxSpeed(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMaxSpeed(const std::string& vehID, double speed) {
    //     Helper::getVehicle(vehID)->getSingularType().setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(vehID: &str, speed: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_max_speed(&speed);
    }

    /// C++: `void Vehicle::setVehicleClass(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setVehicleClass(const std::string& vehID, const std::string& clazz) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     veh->getSingularType().setVClass(getVehicleClassID(clazz));
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (microVeh != nullptr && microVeh->isOnRoad()) {
    //         microVeh->updateBestLanes(true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_class(vehID: &str, clazz: &str) {
        let veh = Helper::get_vehicle(vehID);
        veh.m().get_singular_type().set_v_class(get_vehicle_class_id(clazz).unwrap_or_else(|e| panic!("{}", e)));
        let microVeh = micro_veh(veh);
        if microVeh.is_on_road() {
            microVeh.m().update_best_lanes(true, Ptr::default());
        }
    }

    /// C++: `void Vehicle::setSpeedFactor(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setSpeedFactor(const std::string& vehID, double factor) {
    //     Helper::getVehicle(vehID)->setChosenSpeedFactor(factor);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor(vehID: &str, factor: f64) {
        Helper::get_vehicle(vehID).m().set_chosen_speed_factor(factor);
    }

    /// C++: `void Vehicle::setAccel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setAccel(const std::string& vehID, double accel) {
    //     Helper::getVehicle(vehID)->getSingularType().setAccel(accel);
    // }
    #[allow(unused_variables)]
    pub fn set_accel(vehID: &str, accel: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_accel(accel);
    }

    /// C++: `void Vehicle::setDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setDecel(const std::string& vehID, double decel) {
    //     VehicleType::setDecel(Helper::getVehicle(vehID)->getSingularType().getID(), decel);
    // }
    #[allow(unused_variables)]
    pub fn set_decel(vehID: &str, decel: f64) {
        let typeID = Helper::get_vehicle(vehID).m().get_singular_type().get_id().to_string();
        VehicleType::set_decel(&typeID, decel);
    }

    /// C++: `void Vehicle::setEmergencyDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setEmergencyDecel(const std::string& vehID, double decel) {
    //     VehicleType::setEmergencyDecel(Helper::getVehicle(vehID)->getSingularType().getID(), decel);
    // }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(vehID: &str, decel: f64) {
        let typeID = Helper::get_vehicle(vehID).m().get_singular_type().get_id().to_string();
        VehicleType::set_emergency_decel(&typeID, decel);
    }

    /// C++: `void Vehicle::setApparentDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setApparentDecel(const std::string& vehID, double decel) {
    //     Helper::getVehicle(vehID)->getSingularType().setApparentDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(vehID: &str, decel: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_apparent_decel(decel);
    }

    /// C++: `void Vehicle::setImperfection(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setImperfection(const std::string& vehID, double imperfection) {
    //     Helper::getVehicle(vehID)->getSingularType().setImperfection(imperfection);
    // }
    #[allow(unused_variables)]
    pub fn set_imperfection(vehID: &str, imperfection: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_imperfection(imperfection);
    }

    /// C++: `void Vehicle::setTau(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setTau(const std::string& vehID, double tau) {
    //     Helper::getVehicle(vehID)->getSingularType().setTau(tau);
    // }
    #[allow(unused_variables)]
    pub fn set_tau(vehID: &str, tau: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_tau(tau);
    }

    /// C++: `void Vehicle::setEmissionClass(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setEmissionClass(const std::string& vehID, const std::string& clazz) {
    //     Helper::getVehicle(vehID)->getSingularType().setEmissionClass(PollutantsInterface::getClassByName(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_emission_class(vehID: &str, clazz: &str) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_emission_class(PollutantsInterface::get_class_by_name(clazz, SVC_IGNORING));
    }

    /// C++: `void Vehicle::setShapeClass(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setShapeClass(const std::string& vehID, const std::string& clazz) {
    //     Helper::getVehicle(vehID)->getSingularType().setShape(getVehicleShapeID(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_shape_class(vehID: &str, clazz: &str) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_shape(get_vehicle_shape_id(clazz).unwrap_or_else(|e| panic!("{}", e)));
    }

    /// C++: `void Vehicle::setWidth(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setWidth(const std::string& vehID, double width) {
    //     Helper::getVehicle(vehID)->getSingularType().setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(vehID: &str, width: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_width(&width);
    }

    /// C++: `void Vehicle::setHeight(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setHeight(const std::string& vehID, double height) {
    //     Helper::getVehicle(vehID)->getSingularType().setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(vehID: &str, height: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_height(&height);
    }

    /// C++: `void Vehicle::setMass(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMass(const std::string& vehID, double mass) {
    //     Helper::getVehicle(vehID)->getSingularType().setMass(mass);
    // }
    #[allow(unused_variables)]
    pub fn set_mass(vehID: &str, mass: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_mass(mass);
    }

    /// C++: `void Vehicle::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(vehID: &str, col: &TraCIColor) {
        let veh = Helper::get_vehicle(vehID);
        let p = param_mut(veh.get_parameter());
        p.color.set(col.r as u8, col.g as u8, col.b as u8, col.a as u8);
        p.parametersSet |= VEHPARS_COLOR_SET;
    }

    /// C++: `void Vehicle::setMinGap(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMinGap(const std::string& vehID, double minGap) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     veh->getSingularType().setMinGap(minGap);
    //     MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //     if (microVeh != nullptr && microVeh->isOnRoad()) {
    //         microVeh->updateLaneBruttoSum();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap(vehID: &str, minGap: f64) {
        let veh = Helper::get_vehicle(vehID);
        veh.m().get_singular_type().set_min_gap(&minGap);
        let microVeh = micro_veh(veh);
        if microVeh.is_on_road() {
            microVeh.m().update_lane_brutto_sum();
        }
    }

    /// C++: `void Vehicle::setMinGapLat(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMinGapLat(const std::string& vehID, double minGapLat) {
    //     try {
    //         setParameter(vehID, "laneChangeModel.minGapLat", toString(minGapLat));
    //     } catch (TraCIException&) {
    //         // legacy behavior
    //         Helper::getVehicle(vehID)->getSingularType().setMinGapLat(minGapLat);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap_lat(vehID: &str, minGapLat: f64) {
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            Self::set_parameter(vehID, "laneChangeModel.minGapLat", &to_string(&minGapLat))
        }))
        .is_err();
        if failed {
            // legacy behavior (the TraCIException from setParameter is caught by catch_unwind above)
            Helper::get_vehicle(vehID).m().get_singular_type().set_min_gap_lat(&minGapLat);
        }
    }

    /// C++: `void Vehicle::setMaxSpeedLat(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMaxSpeedLat(const std::string& vehID, double speed) {
    //     Helper::getVehicle(vehID)->getSingularType().setMaxSpeedLat(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed_lat(vehID: &str, speed: f64) {
        Helper::get_vehicle(vehID).m().get_singular_type().set_max_speed_lat(&speed);
    }

    /// C++: `void Vehicle::setLateralAlignment(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setLateralAlignment(const std::string& vehID, const std::string& latAlignment) {
    //     double lao;
    //     LatAlignmentDefinition lad;
    //     if (SUMOVTypeParameter::parseLatAlignment(latAlignment, lao, lad)) {
    //         Helper::getVehicle(vehID)->getSingularType().setPreferredLateralAlignment(lad, lao);
    //     } else {
    //         throw TraCIException("Unknown value '" + latAlignment + "' when setting latAlignment for vehID '" + vehID + "';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_lateral_alignment(vehID: &str, latAlignment: &str) {
        let mut lao: f64 = 0.;
        let mut lad: LatAlignmentDefinition = LatAlignmentDefinition::DEFAULT;
        if SUMOVTypeParameter::parse_lat_alignment(latAlignment, &mut lao, &mut lad) {
            Helper::get_vehicle(vehID).m().get_singular_type().set_preferred_lateral_alignment(&lad, lao);
        } else {
            panic!("{}", format!("Unknown value '{}' when setting latAlignment for vehID '{}';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)", latAlignment, vehID));
        }
    }

    /// C++: `void Vehicle::setActionStepLength(const std::string &, double, bool)` [static]
    /// @}
    // void
    // Vehicle::setActionStepLength(const std::string& vehID, double actionStepLength, bool resetActionOffset) {
    //     if (actionStepLength < 0.0) {
    //         WRITE_ERROR("Invalid action step length (<0). Ignoring command setActionStepLength().");
    //         return;
    //     }
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     MSVehicle* veh = dynamic_cast<MSVehicle*>(vehicle);
    //     if (veh == nullptr) {
    //         WRITE_ERROR("setActionStepLength not applicable for meso");
    //         return;
    //     }
    //
    //     if (actionStepLength == 0.) {
    //         veh->resetActionOffset();
    //     } else {
    //         veh->setActionStepLength(actionStepLength, resetActionOffset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_action_step_length(vehID: &str, actionStepLength: f64, resetActionOffset: bool) {
        if actionStepLength < 0.0 {
            return;
        }
        let veh = micro_veh(Helper::get_vehicle(vehID));
        if actionStepLength == 0. {
            veh.m().reset_action_offset(0);
        } else {
            veh.m().set_action_step_length(actionStepLength, resetActionOffset);
        }
    }

    /// C++: `void Vehicle::setBoardingDuration(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setBoardingDuration(const std::string& vehID, double boardingDuration)  {
    //     try {
    //         checkTimeBounds(boardingDuration);
    //     } catch (ProcessError&) {
    //         throw TraCIException("BoardingDuration parameter exceeds the time value range.");
    //     }
    //     Helper::getVehicle(vehID)->getSingularType().setBoardingDuration(TIME2STEPS(boardingDuration));
    // }
    #[allow(unused_variables)]
    pub fn set_boarding_duration(vehID: &str, boardingDuration: f64) {
        if !check_time_bounds(boardingDuration) {
            panic!("BoardingDuration parameter exceeds the time value range.");
        }
        Helper::get_vehicle(vehID).m().get_singular_type().set_boarding_duration(time2steps(boardingDuration), true);
    }

    /// C++: `void Vehicle::setImpatience(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setImpatience(const std::string& vehID, double impatience)  {
    //     MSBaseVehicle* vehicle = Helper::getVehicle(vehID);
    //     const double normalImpatience = vehicle->getImpatience();
    //     vehicle->getBaseInfluencer().setExtraImpatience(impatience - normalImpatience);
    // }
    #[allow(unused_variables)]
    pub fn set_impatience(vehID: &str, impatience: f64) {
        let vehicle = Helper::get_vehicle(vehID);
        let normalImpatience = vehicle.get_impatience();
        vehicle.m().get_base_influencer().set_extra_impatience(impatience - normalImpatience);
    }

    /// C++: `void Vehicle::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLE_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.);
    }

    /// C++: `void Vehicle::unsubscribe(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLE_VARIABLE, objectID, &Vec::new(), 0., 0., &TraCIResults::new(), 0, 0.);
    }

    /// C++: `void Vehicle::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLE_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Vehicle::unsubscribeContext(const std::string &, int, double)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLE_CONTEXT, objectID, &Vec::new(), 0., 0., &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Vehicle::getAllSubscriptionResults()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Vehicle::getSubscriptionResults(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Vehicle::getAllContextSubscriptionResults()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Vehicle::getContextSubscriptionResults(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Vehicle::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_PARAMETER_WITH_KEY, crate::libsumo::helper::as_traci_result(TraCIString { base: TraCIResult { result_type: TYPE_STRING }, value: key.to_string() }));
        Helper::subscribe(CMD_SUBSCRIBE_VEHICLE_VARIABLE, objectID, &vec![VAR_PARAMETER_WITH_KEY], beginTime, endTime, &params, 0, 0.0);
    }

    /// C++: `int Vehicle::domainID()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_VEHICLE_VARIABLE
    }

    /// C++: `void Vehicle::subscribeLeader(const std::string &, double, double, double)` [static]
    /// @}
    // void
    // Vehicle::subscribeLeader(const std::string& vehID, double dist, double begin, double end) {
    //     subscribe(vehID, std::vector<int>({ libsumo::VAR_LEADER }), begin, end,
    //     libsumo::TraCIResults({ {libsumo::VAR_LEADER, std::make_shared<libsumo::TraCIDouble>(dist)} }));
    // }
    #[allow(unused_variables)]
    pub fn subscribe_leader(vehID: &str, dist: f64, begin: f64, end: f64) {
        let mut params: TraCIResults = TraCIResults::new();
        params.insert(VAR_LEADER, crate::libsumo::helper::as_traci_result(TraCIDouble { base: TraCIResult { result_type: TYPE_DOUBLE }, value: dist }));
        Self::subscribe(vehID, &vec![VAR_LEADER], begin, end, &params);
    }

    /// C++: `void Vehicle::addSubscriptionFilterLanes(const std::vector<int> &, bool, double, double)` [static]
    // void
    // Vehicle::addSubscriptionFilterLanes(const std::vector<int>& lanes, bool noOpposite, double downstreamDist, double upstreamDist) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_LANES);
    //     if (s != nullptr) {
    //         s->filterLanes = lanes;
    //     }
    //     if (noOpposite) {
    //         addSubscriptionFilterNoOpposite();
    //     }
    //     if (downstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterDownstreamDistance(downstreamDist);
    //     }
    //     if (upstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterUpstreamDistance(upstreamDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lanes(lanes: &Vec<i32>, noOpposite: bool, downstreamDist: f64, upstreamDist: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_LANES);
        if !s.is_null() {
            s.m().filterLanes = lanes.clone();
        }
        if noOpposite {
            Self::add_subscription_filter_no_opposite();
        }
        if downstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_downstream_distance(downstreamDist);
        }
        if upstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_upstream_distance(upstreamDist);
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterNoOpposite()` [static]
    // void
    // Vehicle::addSubscriptionFilterNoOpposite() {
    //     Helper::addSubscriptionFilter(SUBS_FILTER_NOOPPOSITE);
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_no_opposite() {
Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_NOOPPOSITE);
    }

    /// C++: `void Vehicle::addSubscriptionFilterDownstreamDistance(double)` [static]
    // void
    // Vehicle::addSubscriptionFilterDownstreamDistance(double dist) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_DOWNSTREAM_DIST);
    //     if (s != nullptr) {
    //         s->filterDownstreamDist = dist;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_downstream_distance(dist: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_DOWNSTREAM_DIST);
        if !s.is_null() {
            s.m().filterDownstreamDist = dist;
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterUpstreamDistance(double)` [static]
    // void
    // Vehicle::addSubscriptionFilterUpstreamDistance(double dist) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_UPSTREAM_DIST);
    //     if (s != nullptr) {
    //         s->filterUpstreamDist = dist;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_upstream_distance(dist: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_UPSTREAM_DIST);
        if !s.is_null() {
            s.m().filterUpstreamDist = dist;
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterCFManeuver(double, double)` [static]
    // void
    // Vehicle::addSubscriptionFilterCFManeuver(double downstreamDist, double upstreamDist) {
    //     addSubscriptionFilterLeadFollow(std::vector<int>({0}));
    //     if (downstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterDownstreamDistance(downstreamDist);
    //     }
    //     if (upstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterUpstreamDistance(upstreamDist);
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_cf_maneuver(downstreamDist: f64, upstreamDist: f64) {
Self::add_subscription_filter_lead_follow(&vec![0]);
        if downstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_downstream_distance(downstreamDist);
        }
        if upstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_upstream_distance(upstreamDist);
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterLCManeuver(int, bool, double, double)` [static]
    // void
    // Vehicle::addSubscriptionFilterLCManeuver(int direction, bool noOpposite, double downstreamDist, double upstreamDist) {
    //     std::vector<int> lanes;
    //     if (direction == INVALID_INT_VALUE) {
    //         // Using default: both directions
    //         lanes = std::vector<int>({-1, 0, 1});
    //     } else if (direction != -1 && direction != 1) {
    //         WRITE_WARNINGF(TL("Ignoring lane change subscription filter with non-neighboring lane offset direction=%."), direction);
    //     } else {
    //         lanes = std::vector<int>({0, direction});
    //     }
    //     addSubscriptionFilterLeadFollow(lanes);
    //     if (noOpposite) {
    //         addSubscriptionFilterNoOpposite();
    //     }
    //     if (downstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterDownstreamDistance(downstreamDist);
    //     }
    //     if (upstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterUpstreamDistance(upstreamDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lc_maneuver(direction: i32, noOpposite: bool, downstreamDist: f64, upstreamDist: f64) {
let mut lanes: Vec<i32> = Vec::new();
        if direction == INVALID_INT_VALUE {
            lanes = vec![-1, 0, 1];
        } else if direction != -1 && direction != 1 {
            utils::write_warningf!("Ignoring lane change subscription filter with non-neighboring lane offset direction=%.", direction);
        } else {
            lanes = vec![0, direction];
        }
        Self::add_subscription_filter_lead_follow(&lanes);
        if noOpposite {
            Self::add_subscription_filter_no_opposite();
        }
        if downstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_downstream_distance(downstreamDist);
        }
        if upstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_upstream_distance(upstreamDist);
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterLeadFollow(const std::vector<int> &)` [static]
    // void
    // Vehicle::addSubscriptionFilterLeadFollow(const std::vector<int>& lanes) {
    //     Helper::addSubscriptionFilter(SUBS_FILTER_LEAD_FOLLOW);
    //     addSubscriptionFilterLanes(lanes);
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lead_follow(lanes: &Vec<i32>) {
Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_LEAD_FOLLOW);
        Self::add_subscription_filter_lanes(lanes, false, INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE);
    }

    /// C++: `void Vehicle::addSubscriptionFilterTurn(double, double)` [static]
    // void
    // Vehicle::addSubscriptionFilterTurn(double downstreamDist, double foeDistToJunction) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_TURN);
    //     if (downstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterDownstreamDistance(downstreamDist);
    //     }
    //     if (foeDistToJunction != INVALID_DOUBLE_VALUE) {
    //         s->filterFoeDistToJunction = foeDistToJunction;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_turn(downstreamDist: f64, foeDistToJunction: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_TURN);
        if downstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_downstream_distance(downstreamDist);
        }
        if foeDistToJunction != INVALID_DOUBLE_VALUE {
            s.m().filterFoeDistToJunction = foeDistToJunction;
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterVClass(const std::vector<std::string> &)` [static]
    // void
    // Vehicle::addSubscriptionFilterVClass(const std::vector<std::string>& vClasses) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_VCLASS);
    //     if (s != nullptr) {
    //         s->filterVClasses = parseVehicleClasses(vClasses);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_v_class(vClasses: &Vec<String>) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_VCLASS);
        if !s.is_null() {
            s.m().filterVClasses = utils::common::sumo_vehicle_class::parse_vehicle_classes_vec(vClasses).unwrap_or_else(|e| panic!("{}", e));
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterVType(const std::vector<std::string> &)` [static]
    // void
    // Vehicle::addSubscriptionFilterVType(const std::vector<std::string>& vTypes) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_VTYPE);
    //     if (s != nullptr) {
    //         s->filterVTypes.insert(vTypes.begin(), vTypes.end());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_v_type(vTypes: &Vec<String>) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_VTYPE);
        if !s.is_null() {
            s.m().filterVTypes.extend(vTypes.iter().cloned());
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterFieldOfVision(double)` [static]
    // void
    // Vehicle::addSubscriptionFilterFieldOfVision(double openingAngle) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_FIELD_OF_VISION);
    //     if (s != nullptr) {
    //         s->filterFieldOfVisionOpeningAngle = openingAngle;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_field_of_vision(openingAngle: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_FIELD_OF_VISION);
        if !s.is_null() {
            s.m().filterFieldOfVisionOpeningAngle = openingAngle;
        }
    }

    /// C++: `void Vehicle::addSubscriptionFilterLateralDistance(double, double, double)` [static]
    // void
    // Vehicle::addSubscriptionFilterLateralDistance(double lateralDist, double downstreamDist, double upstreamDist) {
    //     Subscription* s = Helper::addSubscriptionFilter(SUBS_FILTER_LATERAL_DIST);
    //     if (s != nullptr) {
    //         s->filterLateralDist = lateralDist;
    //     }
    //     if (downstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterDownstreamDistance(downstreamDist);
    //     }
    //     if (upstreamDist != INVALID_DOUBLE_VALUE) {
    //         addSubscriptionFilterUpstreamDistance(upstreamDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lateral_distance(lateralDist: f64, downstreamDist: f64, upstreamDist: f64) {
let s = Helper::add_subscription_filter(SubscriptionFilterType::SUBS_FILTER_LATERAL_DIST);
        if !s.is_null() {
            s.m().filterLateralDist = lateralDist;
        }
        if downstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_downstream_distance(downstreamDist);
        }
        if upstreamDist != INVALID_DOUBLE_VALUE {
            Self::add_subscription_filter_upstream_distance(upstreamDist);
        }
    }

    /// C++: `void Vehicle::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Vehicle::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(Helper::getVehicle(id)->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        shape.0.push(Helper::get_vehicle(id).get_position(0.));
    }

    /// C++: `std::shared_ptr<VariableWrapper> Vehicle::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Vehicle::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        Helper::make_subscription_wrapper(Some(Self::handle_variable), unsafe { &mut *MY_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) }, unsafe { &mut *MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.as_ptr()) })
    }

    /// C++: `bool Vehicle::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Vehicle::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case TRACI_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getIDList());
    //         case ID_COUNT:
    //             return wrapper->wrapInt(objID, variable, getIDCount());
    //         case VAR_POSITION:
    //             return wrapper->wrapPosition(objID, variable, getPosition(objID));
    //         case VAR_POSITION3D:
    //             return wrapper->wrapPosition(objID, variable, getPosition(objID, true));
    //         case VAR_ANGLE:
    //             return wrapper->wrapDouble(objID, variable, getAngle(objID));
    //         case VAR_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getSpeed(objID));
    //         case VAR_SPEED_LAT:
    //             return wrapper->wrapDouble(objID, variable, getLateralSpeed(objID));
    //         case VAR_ROAD_ID:
    //             return wrapper->wrapString(objID, variable, getRoadID(objID));
    //         case VAR_SPEED_WITHOUT_TRACI:
    //             return wrapper->wrapDouble(objID, variable, getSpeedWithoutTraCI(objID));
    //         case VAR_SLOPE:
    //             return wrapper->wrapDouble(objID, variable, getSlope(objID));
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case VAR_SHADOW_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getShadowLaneID(objID));
    //         case VAR_LANE_INDEX:
    //             return wrapper->wrapInt(objID, variable, getLaneIndex(objID));
    //         case VAR_SEGMENT_ID:
    //             return wrapper->wrapString(objID, variable, getSegmentID(objID));
    //         case VAR_SEGMENT_INDEX:
    //             return wrapper->wrapInt(objID, variable, getSegmentIndex(objID));
    //         case VAR_TYPE:
    //             return wrapper->wrapString(objID, variable, getTypeID(objID));
    //         case VAR_ROUTE_ID:
    //             return wrapper->wrapString(objID, variable, getRouteID(objID));
    //         case VAR_DEPARTURE:
    //             return wrapper->wrapDouble(objID, variable, getDeparture(objID));
    //         case VAR_DEPART_DELAY:
    //             return wrapper->wrapDouble(objID, variable, getDepartDelay(objID));
    //         case VAR_ROUTE_INDEX:
    //             return wrapper->wrapInt(objID, variable, getRouteIndex(objID));
    //         case VAR_COLOR:
    //             return wrapper->wrapColor(objID, variable, getColor(objID));
    //         case VAR_LANEPOSITION:
    //             return wrapper->wrapDouble(objID, variable, getLanePosition(objID));
    //         case VAR_ARRIVALPOS:
    //             return wrapper->wrapDouble(objID, variable, getArrivalPos(objID));
    //         case VAR_LANEPOSITION_LAT:
    //             return wrapper->wrapDouble(objID, variable, getLateralLanePosition(objID));
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
    //         case VAR_PERSON_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getPersonNumber(objID));
    //         case VAR_PERSON_CAPACITY:
    //             return wrapper->wrapInt(objID, variable, getPersonCapacity(objID));
    //         case VAR_BOARDING_DURATION:
    //             return wrapper->wrapDouble(objID, variable, getBoardingDuration(objID));
    //         case LAST_STEP_PERSON_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getPersonIDList(objID));
    //         case VAR_WAITING_TIME:
    //             return wrapper->wrapDouble(objID, variable, getWaitingTime(objID));
    //         case VAR_ACCUMULATED_WAITING_TIME:
    //             return wrapper->wrapDouble(objID, variable, getAccumulatedWaitingTime(objID));
    //         case VAR_EDGE_TRAVELTIME: {
    //             StoHelp::readCompound(*paramData);
    //             const double time = StoHelp::readTypedDouble(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getAdaptedTraveltime(objID, time, StoHelp::readTypedString(*paramData)));
    //         }
    //         case VAR_EDGE_EFFORT: {
    //             StoHelp::readCompound(*paramData);
    //             const double time = StoHelp::readTypedDouble(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getEffort(objID, time, StoHelp::readTypedString(*paramData)));
    //         }
    //         case VAR_ROUTE_VALID:
    //             return wrapper->wrapInt(objID, variable, isRouteValid(objID));
    //         case VAR_EDGES:
    //             return wrapper->wrapStringList(objID, variable, getRoute(objID));
    //         case VAR_SIGNALS:
    //             return wrapper->wrapInt(objID, variable, getSignals(objID));
    //         case VAR_BEST_LANES:
    //             return wrapper->wrapBestLanesDataVector(objID, variable, getBestLanes(objID));
    //         case VAR_NEXT_LINKS:
    //             return wrapper->wrapConnectionVector(objID, variable, getNextLinks(objID));
    //         case VAR_STOPSTATE:
    //             return wrapper->wrapInt(objID, variable, getStopState(objID));
    //         case VAR_DISTANCE:
    //             return wrapper->wrapDouble(objID, variable, getDistance(objID));
    //         case VAR_REFERENCE_DISTANCE:
    //             return wrapper->wrapDouble(objID, variable, getReferenceDistance(objID));
    //         case VAR_ALLOWED_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getAllowedSpeed(objID));
    //         case VAR_SPEED_FACTOR:
    //             return wrapper->wrapDouble(objID, variable, getSpeedFactor(objID));
    //         case VAR_SPEEDSETMODE:
    //             return wrapper->wrapInt(objID, variable, getSpeedMode(objID));
    //         case VAR_LANECHANGE_MODE:
    //             return wrapper->wrapInt(objID, variable, getLaneChangeMode(objID));
    //         case VAR_ROUTING_MODE:
    //             return wrapper->wrapInt(objID, variable, getRoutingMode(objID));
    //         case VAR_LINE:
    //             return wrapper->wrapString(objID, variable, getLine(objID));
    //         case VAR_VIA:
    //             return wrapper->wrapStringList(objID, variable, getVia(objID));
    //         case VAR_ACCELERATION:
    //             return wrapper->wrapDouble(objID, variable, getAcceleration(objID));
    //         case VAR_LASTACTIONTIME:
    //             return wrapper->wrapDouble(objID, variable, getLastActionTime(objID));
    //         case VAR_STOP_DELAY:
    //             return wrapper->wrapDouble(objID, variable, getStopDelay(objID));
    //         case VAR_IMPATIENCE:
    //             return wrapper->wrapDouble(objID, variable, getImpatience(objID));
    //         case VAR_STOP_ARRIVALDELAY:
    //             return wrapper->wrapDouble(objID, variable, getStopArrivalDelay(objID));
    //         case VAR_TIMELOSS:
    //             return wrapper->wrapDouble(objID, variable, getTimeLoss(objID));
    //         case VAR_MINGAP_LAT:
    //             return wrapper->wrapDouble(objID, variable, getMinGapLat(objID));
    //         case VAR_LEADER:
    //             return wrapper->wrapStringDoublePair(objID, variable, getLeader(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_FOLLOWER:
    //             return wrapper->wrapStringDoublePair(objID, variable, getFollower(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_LOADED_LIST:
    //             return wrapper->wrapStringList(objID, variable, getLoadedIDList());
    //         case VAR_TELEPORTING_LIST:
    //             return wrapper->wrapStringList(objID, variable, getTeleportingIDList());
    //         case VAR_FOLLOW_SPEED: {
    //             StoHelp::readCompound(*paramData);
    //             const double speed = StoHelp::readTypedDouble(*paramData);
    //             const double gap = StoHelp::readTypedDouble(*paramData);
    //             const double leaderSpeed = StoHelp::readTypedDouble(*paramData);
    //             const double leaderMaxDecel = StoHelp::readTypedDouble(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getFollowSpeed(objID, speed, gap, leaderSpeed, leaderMaxDecel, StoHelp::readTypedString(*paramData)));
    //         }
    //         case VAR_SECURE_GAP: {
    //             StoHelp::readCompound(*paramData);
    //             const double speed = StoHelp::readTypedDouble(*paramData);
    //             const double leaderSpeed = StoHelp::readTypedDouble(*paramData);
    //             const double leaderMaxDecel = StoHelp::readTypedDouble(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getSecureGap(objID, speed, leaderSpeed, leaderMaxDecel, StoHelp::readTypedString(*paramData)));
    //         }
    //         case VAR_STOP_SPEED: {
    //             StoHelp::readCompound(*paramData);
    //             const double speed = StoHelp::readTypedDouble(*paramData);
    //             return wrapper->wrapDouble(objID, variable, getStopSpeed(objID, speed, StoHelp::readTypedDouble(*paramData)));
    //         }
    //         case VAR_FOES:
    //             return wrapper->wrapJunctionFoeVector(objID, variable, getJunctionFoes(objID, StoHelp::readTypedDouble(*paramData)));
    //         case VAR_NEIGHBORS:
    //             return wrapper->wrapStringDoublePairList(objID, variable, getNeighbors(objID, StoHelp::readTypedByte(*paramData)));
    //         case CMD_CHANGELANE:
    //             return wrapper->wrapIntPair(objID, variable, getLaneChangeState(objID, StoHelp::readTypedInt(*paramData)));
    //         case VAR_STOP_PARAMETER: {
    //             const int count = StoHelp::readCompound(*paramData);
    //             const int nextStopIndex = StoHelp::readTypedInt(*paramData);
    //             const std::string param = StoHelp::readTypedString(*paramData);
    //             const bool customParam = count == 3 && StoHelp::readTypedByte(*paramData) != 0;
    //             return wrapper->wrapString(objID, variable, getStopParameter(objID, nextStopIndex, param, customParam));
    //         }
    //         case VAR_NEXT_TLS:
    //             return wrapper->wrapNextTLSDataVector(objID, variable, getNextTLS(objID));
    //         case VAR_NEXT_STOPS:
    //             return wrapper->wrapNextStopDataVector(objID, variable, getNextStops(objID));
    //         case VAR_NEXT_STOPS2:
    //             return wrapper->wrapNextStopDataVector(objID, variable, getStops(objID, StoHelp::readTypedInt(*paramData)));
    //         case DISTANCE_REQUEST: {
    //             TraCIRoadPosition roadPos;
    //             Position pos;
    //             if (Helper::readDistanceRequest(*paramData, roadPos, pos) == libsumo::POSITION_ROADMAP) {
    //                 return wrapper->wrapDouble(objID, variable, getDrivingDistance(objID, roadPos.edgeID, roadPos.pos, roadPos.laneIndex));
    //             }
    //             return wrapper->wrapDouble(objID, variable, getDrivingDistance2D(objID, pos.x(), pos.y()));
    //         }
    //         case VAR_TAXI_FLEET:
    //             return wrapper->wrapStringList(objID, variable, getTaxiFleet(StoHelp::readTypedInt(*paramData)));
    //         case VAR_PARAMETER:
    //             return wrapper->wrapString(objID, variable, getParameter(objID, StoHelp::readTypedString(*paramData)));
    //         case VAR_PARAMETER_WITH_KEY:
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, StoHelp::readTypedString(*paramData)));
    //         default:
    //             return VehicleType::handleVariableWithID(objID, getTypeID(objID), variable, wrapper, paramData);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        let p = paramData.m();
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_POSITION => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, false)),
            VAR_POSITION3D => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, true)),
            VAR_ANGLE => wrapper.m().wrap_double(objID, variable, Self::get_angle(objID)),
            VAR_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_speed(objID)),
            VAR_SPEED_LAT => wrapper.m().wrap_double(objID, variable, Self::get_lateral_speed(objID)),
            VAR_ROAD_ID => wrapper.m().wrap_string(objID, variable, &Self::get_road_id(objID)),
            VAR_SPEED_WITHOUT_TRACI => wrapper.m().wrap_double(objID, variable, Self::get_speed_without_tra_ci(objID)),
            VAR_SLOPE => wrapper.m().wrap_double(objID, variable, Self::get_slope(objID)),
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            VAR_SHADOW_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_shadow_lane_id(objID)),
            VAR_LANE_INDEX => wrapper.m().wrap_int(objID, variable, Self::get_lane_index(objID)),
            VAR_SEGMENT_ID => wrapper.m().wrap_string(objID, variable, &Self::get_segment_id(objID)),
            VAR_SEGMENT_INDEX => wrapper.m().wrap_int(objID, variable, Self::get_segment_index(objID)),
            VAR_TYPE => wrapper.m().wrap_string(objID, variable, &Self::get_type_id(objID)),
            VAR_ROUTE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_route_id(objID)),
            VAR_DEPARTURE => wrapper.m().wrap_double(objID, variable, Self::get_departure(objID)),
            VAR_DEPART_DELAY => wrapper.m().wrap_double(objID, variable, Self::get_depart_delay(objID)),
            VAR_ROUTE_INDEX => wrapper.m().wrap_int(objID, variable, Self::get_route_index(objID)),
            VAR_COLOR => wrapper.m().wrap_color(objID, variable, &Self::get_color(objID)),
            VAR_LANEPOSITION => wrapper.m().wrap_double(objID, variable, Self::get_lane_position(objID)),
            VAR_ARRIVALPOS => wrapper.m().wrap_double(objID, variable, Self::get_arrival_pos(objID)),
            VAR_LANEPOSITION_LAT => wrapper.m().wrap_double(objID, variable, Self::get_lateral_lane_position(objID)),
            VAR_CO2EMISSION => wrapper.m().wrap_double(objID, variable, Self::get_co2_emission(objID)),
            VAR_COEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_co_emission(objID)),
            VAR_HCEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_hc_emission(objID)),
            VAR_PMXEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_p_mx_emission(objID)),
            VAR_NOXEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_n_ox_emission(objID)),
            VAR_FUELCONSUMPTION => wrapper.m().wrap_double(objID, variable, Self::get_fuel_consumption(objID)),
            VAR_NOISEEMISSION => wrapper.m().wrap_double(objID, variable, Self::get_noise_emission(objID)),
            VAR_ELECTRICITYCONSUMPTION => wrapper.m().wrap_double(objID, variable, Self::get_electricity_consumption(objID)),
            VAR_PERSON_NUMBER => wrapper.m().wrap_int(objID, variable, Self::get_person_number(objID)),
            VAR_PERSON_CAPACITY => wrapper.m().wrap_int(objID, variable, Self::get_person_capacity(objID)),
            VAR_BOARDING_DURATION => wrapper.m().wrap_double(objID, variable, Self::get_boarding_duration(objID)),
            LAST_STEP_PERSON_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_person_id_list(objID)),
            VAR_WAITING_TIME => wrapper.m().wrap_double(objID, variable, Self::get_waiting_time(objID)),
            VAR_ACCUMULATED_WAITING_TIME => wrapper.m().wrap_double(objID, variable, Self::get_accumulated_waiting_time(objID)),
            VAR_EDGE_TRAVELTIME => {
                StoHelp::read_compound(p, -1, "");
                let time = StoHelp::read_typed_double(p, "");
                let edge = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_double(objID, variable, Self::get_adapted_traveltime(objID, time, &edge))
            }
            VAR_EDGE_EFFORT => {
                StoHelp::read_compound(p, -1, "");
                let time = StoHelp::read_typed_double(p, "");
                let edge = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_double(objID, variable, Self::get_effort(objID, time, &edge))
            }
            VAR_ROUTE_VALID => wrapper.m().wrap_int(objID, variable, Self::is_route_valid(objID) as i32),
            VAR_EDGES => wrapper.m().wrap_string_list(objID, variable, &Self::get_route(objID)),
            VAR_SIGNALS => wrapper.m().wrap_int(objID, variable, Self::get_signals(objID)),
            VAR_BEST_LANES => wrapper.m().wrap_best_lanes_data_vector(objID, variable, &Self::get_best_lanes(objID)),
            VAR_NEXT_LINKS => wrapper.m().wrap_connection_vector(objID, variable, &Self::get_next_links(objID)),
            VAR_STOPSTATE => wrapper.m().wrap_int(objID, variable, Self::get_stop_state(objID)),
            VAR_DISTANCE => wrapper.m().wrap_double(objID, variable, Self::get_distance(objID)),
            VAR_REFERENCE_DISTANCE => wrapper.m().wrap_double(objID, variable, Self::get_reference_distance(objID)),
            VAR_ALLOWED_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_allowed_speed(objID)),
            VAR_SPEED_FACTOR => wrapper.m().wrap_double(objID, variable, Self::get_speed_factor(objID)),
            VAR_SPEEDSETMODE => wrapper.m().wrap_int(objID, variable, Self::get_speed_mode(objID)),
            VAR_LANECHANGE_MODE => wrapper.m().wrap_int(objID, variable, Self::get_lane_change_mode(objID)),
            VAR_ROUTING_MODE => wrapper.m().wrap_int(objID, variable, Self::get_routing_mode(objID)),
            VAR_LINE => wrapper.m().wrap_string(objID, variable, &Self::get_line(objID)),
            VAR_VIA => wrapper.m().wrap_string_list(objID, variable, &Self::get_via(objID)),
            VAR_ACCELERATION => wrapper.m().wrap_double(objID, variable, Self::get_acceleration(objID)),
            VAR_LASTACTIONTIME => wrapper.m().wrap_double(objID, variable, Self::get_last_action_time(objID)),
            VAR_STOP_DELAY => wrapper.m().wrap_double(objID, variable, Self::get_stop_delay(objID)),
            VAR_IMPATIENCE => wrapper.m().wrap_double(objID, variable, Self::get_impatience(objID)),
            VAR_STOP_ARRIVALDELAY => wrapper.m().wrap_double(objID, variable, Self::get_stop_arrival_delay(objID)),
            VAR_TIMELOSS => wrapper.m().wrap_double(objID, variable, Self::get_time_loss(objID)),
            VAR_MINGAP_LAT => wrapper.m().wrap_double(objID, variable, Self::get_min_gap_lat(objID)),
            VAR_LEADER => {
                let dist = StoHelp::read_typed_double(p, "");
                wrapper.m().wrap_string_double_pair(objID, variable, &Self::get_leader(objID, dist))
            }
            VAR_FOLLOWER => {
                let dist = StoHelp::read_typed_double(p, "");
                wrapper.m().wrap_string_double_pair(objID, variable, &Self::get_follower(objID, dist))
            }
            VAR_LOADED_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_loaded_id_list()),
            VAR_TELEPORTING_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_teleporting_id_list()),
            VAR_FOLLOW_SPEED => {
                StoHelp::read_compound(p, -1, "");
                let speed = StoHelp::read_typed_double(p, "");
                let gap = StoHelp::read_typed_double(p, "");
                let leaderSpeed = StoHelp::read_typed_double(p, "");
                let leaderMaxDecel = StoHelp::read_typed_double(p, "");
                let leaderID = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_double(objID, variable, Self::get_follow_speed(objID, speed, gap, leaderSpeed, leaderMaxDecel, &leaderID))
            }
            VAR_SECURE_GAP => {
                StoHelp::read_compound(p, -1, "");
                let speed = StoHelp::read_typed_double(p, "");
                let leaderSpeed = StoHelp::read_typed_double(p, "");
                let leaderMaxDecel = StoHelp::read_typed_double(p, "");
                let leaderID = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_double(objID, variable, Self::get_secure_gap(objID, speed, leaderSpeed, leaderMaxDecel, &leaderID))
            }
            VAR_STOP_SPEED => {
                StoHelp::read_compound(p, -1, "");
                let speed = StoHelp::read_typed_double(p, "");
                let gap = StoHelp::read_typed_double(p, "");
                wrapper.m().wrap_double(objID, variable, Self::get_stop_speed(objID, speed, gap))
            }
            VAR_FOES => {
                let dist = StoHelp::read_typed_double(p, "");
                wrapper.m().wrap_junction_foe_vector(objID, variable, &Self::get_junction_foes(objID, dist))
            }
            VAR_NEIGHBORS => {
                let mode = StoHelp::read_typed_byte(p, "");
                wrapper.m().wrap_string_double_pair_list(objID, variable, &Self::get_neighbors(objID, mode))
            }
            CMD_CHANGELANE => {
                let direction = StoHelp::read_typed_int(p, "");
                wrapper.m().wrap_int_pair(objID, variable, &Self::get_lane_change_state(objID, direction))
            }
            VAR_STOP_PARAMETER => {
                let count = StoHelp::read_compound(p, -1, "");
                let nextStopIndex = StoHelp::read_typed_int(p, "");
                let param = StoHelp::read_typed_string(p, "");
                let customParam = count == 3 && StoHelp::read_typed_byte(p, "") != 0;
                wrapper.m().wrap_string(objID, variable, &Self::get_stop_parameter(objID, nextStopIndex, &param, customParam))
            }
            VAR_NEXT_TLS => wrapper.m().wrap_next_tls_data_vector(objID, variable, &Self::get_next_tls(objID)),
            VAR_NEXT_STOPS => wrapper.m().wrap_next_stop_data_vector(objID, variable, &Self::get_next_stops(objID)),
            VAR_NEXT_STOPS2 => {
                let limit = StoHelp::read_typed_int(p, "");
                wrapper.m().wrap_next_stop_data_vector(objID, variable, &Self::get_stops(objID, limit))
            }
            DISTANCE_REQUEST => {
                let mut roadPos = TraCIRoadPosition { base: TraCIResult::new(), edgeID: String::new(), pos: 0., laneIndex: 0 };
                let mut pos = Position::default();
                if Helper::read_distance_request(p, &mut roadPos, &mut pos) == POSITION_ROADMAP {
                    wrapper.m().wrap_double(objID, variable, Self::get_driving_distance(objID, &roadPos.edgeID, roadPos.pos, roadPos.laneIndex))
                } else {
                    wrapper.m().wrap_double(objID, variable, Self::get_driving_distance2_d(objID, pos.x(), pos.y()))
                }
            }
            VAR_TAXI_FLEET => {
                let taxiState = StoHelp::read_typed_int(p, "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_taxi_fleet(taxiState))
            }
            VAR_PARAMETER => {
                let key = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                let key = StoHelp::read_typed_string(p, "");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            _ => VehicleType::handle_variable_with_id(objID, &Self::get_type_id(objID), variable, wrapper, paramData),
        }
    }

    /// C++: `bool Vehicle::isVisible(const SUMOVehicle *)` [static, private]
    // bool
    // Vehicle::isVisible(const SUMOVehicle* veh) {
    //     return veh->isOnRoad() || veh->isParking() || veh->wasRemoteControlled();
    // }
    #[allow(unused_variables)]
    pub fn is_visible(veh: Ptr<SUMOVehicle>) -> bool {
        veh.is_on_road() || veh.is_parking() || veh.was_remote_controlled(delta_t())
    }

    /// C++: `bool Vehicle::isOnInit(const std::string &)` [static, private]
    // bool
    // Vehicle::isOnInit(const std::string& vehID) {
    //     SUMOVehicle* sumoVehicle = MSNet::getInstance()->getVehicleControl().getVehicle(vehID);
    //     return sumoVehicle == nullptr || sumoVehicle->getLane() == nullptr;
    // }
    #[allow(unused_variables)]
    pub fn is_on_init(vehID: &str) -> bool {
        let sumoVehicle = MSNet::get_instance().get_vehicle_control().get_vehicle(vehID);
        sumoVehicle.is_null() || sumoVehicle.get_lane().is_null()
    }

    /// C++: `void Vehicle::Vehicle()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        // invalidated standard constructor (C++ declares it private without a body)
        Self {}
    }

}

