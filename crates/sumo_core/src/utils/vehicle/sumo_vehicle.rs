//! Port of utils/vehicle/SUMOVehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<const MSEdge *> ConstMSEdgeVector`
pub type ConstMSEdgeVector = Vec<Ptr<MSEdge>>;

/// Virtual dispatch table for the polymorphic methods of SUMOVehicle.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct SUMOVehicleVt {
    pub get_route: fn(&SUMOVehicle) -> &MSRoute,
    pub get_departure: fn(&SUMOVehicle) -> SUMOTime,
    pub get_depart_pos: fn(&SUMOVehicle) -> f64,
    pub get_odometer: fn(&SUMOVehicle) -> f64,
    pub has_departed: fn(&SUMOVehicle) -> bool,
    pub is_stopped: fn(&SUMOVehicle) -> bool,
    pub get_length: fn(&SUMOVehicle) -> f64,
    pub get_current_edge: fn(&SUMOVehicle) -> Ptr<MSEdge>,
    pub is_on_road: fn(&SUMOVehicle) -> bool,
    pub get_lateral_position_on_lane: fn(&SUMOVehicle) -> f64,
    pub get_angle: fn(&SUMOVehicle) -> f64,
    pub get_route_ptr: fn(&SUMOVehicle) -> ConstMSRoutePtr,
    pub get_route_position: fn(&SUMOVehicle) -> i32,
    pub succ_edge: fn(&SUMOVehicle, i32) -> Ptr<MSEdge>,
    pub get_reroute_origin: fn(&SUMOVehicle) -> crate::microsim::ms_route::MSRouteIterator,
    pub replace_route_edges: fn(&mut SUMOVehicle, &mut ConstMSEdgeVector, f64, f64, &str, bool, bool, bool, Ptr<String>) -> bool,
    pub reroute: fn(&mut SUMOVehicle, SUMOTime, &str, &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, bool, bool, bool, Ptr<MSEdge>) -> bool,
    pub get_router_tt: fn(&SUMOVehicle) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>,
    pub has_valid_route: fn(&SUMOVehicle, &mut String, ConstMSRoutePtr) -> bool,
    pub has_valid_route_start: fn(&mut SUMOVehicle, &mut String) -> bool,
    pub get_route_validity: fn(&mut SUMOVehicle, bool, bool, Ptr<String>) -> i32,
    pub get_current_route_edge: fn(&SUMOVehicle) -> &crate::microsim::ms_route::MSRouteIterator,
    pub get_emission_parameters: fn(&SUMOVehicle) -> Ptr<EnergyParams>,
    pub replace_parameter: fn(&mut SUMOVehicle, Ptr<SUMOVehicleParameter>),
    pub on_depart: fn(&mut SUMOVehicle),
    pub is_idling: fn(&SUMOVehicle) -> bool,
    pub is_front_on_lane: fn(&SUMOVehicle, Ptr<MSLane>) -> bool,
    pub is_parking: fn(&SUMOVehicle) -> bool,
    pub is_remote_controlled: fn(&SUMOVehicle) -> bool,
    pub was_remote_controlled: fn(&SUMOVehicle, SUMOTime) -> bool,
    pub get_arrival_pos: fn(&SUMOVehicle) -> f64,
    pub set_arrival_pos: fn(&mut SUMOVehicle, f64),
    pub get_depart_edge: fn(&SUMOVehicle) -> i32,
    pub get_number_reroutes: fn(&SUMOVehicle) -> i32,
    pub allows_boarding: fn(&SUMOVehicle, Ptr<MSTransportable>) -> bool,
    pub add_transportable: fn(&mut SUMOVehicle, Ptr<MSTransportable>),
    pub get_person_number: fn(&SUMOVehicle) -> i32,
    pub get_person_id_list: fn(&SUMOVehicle) -> Vec<String>,
    pub get_container_number: fn(&SUMOVehicle) -> i32,
    pub remove_transportable: fn(&mut SUMOVehicle, Ptr<MSTransportable>),
    pub remove_transportable_mass: fn(&mut SUMOVehicle, Ptr<MSTransportable>),
    pub get_persons: fn(&SUMOVehicle) -> &Vec<Ptr<MSTransportable>>,
    pub get_containers: fn(&SUMOVehicle) -> &Vec<Ptr<MSTransportable>>,
    pub add_stop: fn(&mut SUMOVehicle, &SUMOVehicleParameter_Stop, &mut String, SUMOTime, Ptr<crate::microsim::ms_route::MSRouteIterator>) -> bool,
    pub get_stop_indices: fn(&SUMOVehicle) -> Vec<(i32, f64)>,
    pub is_line_stop: fn(&SUMOVehicle, f64) -> bool,
    pub abort_next_stop: fn(&mut SUMOVehicle, i32) -> bool,
    pub get_next_parking_area: fn(&mut SUMOVehicle) -> Ptr<MSParkingArea>,
    pub replace_parking_area: fn(&mut SUMOVehicle, Ptr<MSParkingArea>, &mut String) -> bool,
    pub get_parking_badges: fn(&SUMOVehicle) -> &Vec<String>,
    pub remaining_stop_duration: fn(&SUMOVehicle) -> SUMOTime,
    pub is_stopped_triggered: fn(&SUMOVehicle) -> bool,
    pub is_stopped_parking: fn(&SUMOVehicle) -> bool,
    pub is_stopped_in_range: fn(&SUMOVehicle, f64, f64, bool) -> bool,
    pub stops_at: fn(&SUMOVehicle, Ptr<MSStoppingPlace>) -> bool,
    pub stops_at_edge: fn(&SUMOVehicle, Ptr<MSEdge>) -> bool,
    pub has_stops: fn(&SUMOVehicle) -> bool,
    pub get_stops: fn(&SUMOVehicle) -> &VecDeque<MSStop>,
    pub get_next_stop: fn(&SUMOVehicle) -> &MSStop,
    pub get_next_stop_mutable: fn(&mut SUMOVehicle) -> &mut MSStop,
    pub unregister_waiting: fn(&mut SUMOVehicle),
    pub get_next_stop_parameter: fn(&SUMOVehicle) -> Ptr<SUMOVehicleParameter_Stop>,
    pub get_stop_duration: fn(&SUMOVehicle) -> SUMOTime,
    pub add_traci_stop: fn(&mut SUMOVehicle, SUMOVehicleParameter_Stop, &mut String) -> bool,
    pub set_chosen_speed_factor: fn(&mut SUMOVehicle, f64),
    pub get_depart_delay: fn(&SUMOVehicle) -> SUMOTime,
    pub get_time_loss: fn(&SUMOVehicle) -> SUMOTime,
    pub get_brake_gap: fn(&SUMOVehicle, bool) -> f64,
    pub get_impatience: fn(&SUMOVehicle) -> f64,
    pub get_devices: fn(&SUMOVehicle) -> &Vec<Ptr<MSVehicleDevice>>,
    pub is_rail: fn(&SUMOVehicle) -> bool,
    pub get_last_action_time: fn(&SUMOVehicle) -> SUMOTime,
    pub get_bounding_box: fn(&SUMOVehicle, f64) -> PositionVector,
    pub get_scope_lock: fn(&mut SUMOVehicle) -> Box<Unmapped>,
    pub remember_blocked_parking_area: fn(&mut SUMOVehicle, Ptr<MSStoppingPlace>, bool),
    pub saw_blocked_parking_area: fn(&SUMOVehicle, Ptr<MSStoppingPlace>, bool) -> SUMOTime,
    pub remember_parking_area_score: fn(&mut SUMOVehicle, Ptr<MSStoppingPlace>, &str),
    pub reset_parking_area_scores: fn(&mut SUMOVehicle),
    pub get_number_parking_reroutes: fn(&SUMOVehicle) -> i32,
    pub set_number_parking_reroutes: fn(&mut SUMOVehicle, i32),
    pub remember_blocked_charging_station: fn(&mut SUMOVehicle, Ptr<MSStoppingPlace>, bool),
    pub saw_blocked_charging_station: fn(&SUMOVehicle, Ptr<MSStoppingPlace>, bool) -> SUMOTime,
    pub remember_charging_station_score: fn(&mut SUMOVehicle, Ptr<MSStoppingPlace>, &str),
    pub reset_charging_station_scores: fn(&mut SUMOVehicle),
    pub save_state: fn(&mut SUMOVehicle, &mut OutputDevice),
    pub load_state: fn(&mut SUMOVehicle, &SUMOSAXAttributes, SUMOTime),
}

fn default_get_route(_: &SUMOVehicle) -> &MSRoute {
    panic!("SUMOVehicle::getRoute is pure virtual")
}

fn default_get_departure(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::getDeparture is pure virtual")
}

fn default_get_depart_pos(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getDepartPos is pure virtual")
}

fn default_get_odometer(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getOdometer is pure virtual")
}

fn default_has_departed(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::hasDeparted is pure virtual")
}

fn default_is_stopped(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isStopped is pure virtual")
}

fn default_get_length(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getLength is pure virtual")
}

fn default_get_current_edge(_: &SUMOVehicle) -> Ptr<MSEdge> {
    panic!("SUMOVehicle::getCurrentEdge is pure virtual")
}

fn default_is_on_road(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isOnRoad is pure virtual")
}

fn default_get_lateral_position_on_lane(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getLateralPositionOnLane is pure virtual")
}

fn default_get_angle(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getAngle is pure virtual")
}

fn default_get_route_ptr(_: &SUMOVehicle) -> ConstMSRoutePtr {
    panic!("SUMOVehicle::getRoutePtr is pure virtual")
}

fn default_get_route_position(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getRoutePosition is pure virtual")
}

fn default_succ_edge(_: &SUMOVehicle, _: i32) -> Ptr<MSEdge> {
    panic!("SUMOVehicle::succEdge is pure virtual")
}

fn default_get_reroute_origin(_: &SUMOVehicle) -> crate::microsim::ms_route::MSRouteIterator {
    panic!("SUMOVehicle::getRerouteOrigin is pure virtual")
}

fn default_replace_route_edges(_: &mut SUMOVehicle, _: &mut ConstMSEdgeVector, _: f64, _: f64, _: &str, _: bool, _: bool, _: bool, _: Ptr<String>) -> bool {
    panic!("SUMOVehicle::replaceRouteEdges is pure virtual")
}

fn default_reroute(_: &mut SUMOVehicle, _: SUMOTime, _: &str, _: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, _: bool, _: bool, _: bool, _: Ptr<MSEdge>) -> bool {
    panic!("SUMOVehicle::reroute is pure virtual")
}

fn default_get_router_tt(_: &SUMOVehicle) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle> {
    panic!("SUMOVehicle::getRouterTT is pure virtual")
}

fn default_has_valid_route(_: &SUMOVehicle, _: &mut String, _: ConstMSRoutePtr) -> bool {
    panic!("SUMOVehicle::hasValidRoute is pure virtual")
}

fn default_has_valid_route_start(_: &mut SUMOVehicle, _: &mut String) -> bool {
    panic!("SUMOVehicle::hasValidRouteStart is pure virtual")
}

fn default_get_route_validity(_: &mut SUMOVehicle, _: bool, _: bool, _: Ptr<String>) -> i32 {
    panic!("SUMOVehicle::getRouteValidity is pure virtual")
}

fn default_get_current_route_edge(_: &SUMOVehicle) -> &crate::microsim::ms_route::MSRouteIterator {
    panic!("SUMOVehicle::getCurrentRouteEdge is pure virtual")
}

fn default_get_emission_parameters(_: &SUMOVehicle) -> Ptr<EnergyParams> {
    panic!("SUMOVehicle::getEmissionParameters is pure virtual")
}

fn default_replace_parameter(_: &mut SUMOVehicle, _: Ptr<SUMOVehicleParameter>) {
    panic!("SUMOVehicle::replaceParameter is pure virtual")
}

fn default_on_depart(_: &mut SUMOVehicle) {
    panic!("SUMOVehicle::onDepart is pure virtual")
}

fn default_is_idling(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isIdling is pure virtual")
}

fn default_is_front_on_lane(_: &SUMOVehicle, _: Ptr<MSLane>) -> bool {
    panic!("SUMOVehicle::isFrontOnLane is pure virtual")
}

fn default_is_parking(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isParking is pure virtual")
}

fn default_is_remote_controlled(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isRemoteControlled is pure virtual")
}

fn default_was_remote_controlled(_: &SUMOVehicle, _: SUMOTime) -> bool {
    panic!("SUMOVehicle::wasRemoteControlled is pure virtual")
}

fn default_get_arrival_pos(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getArrivalPos is pure virtual")
}

fn default_set_arrival_pos(_: &mut SUMOVehicle, _: f64) {
    panic!("SUMOVehicle::setArrivalPos is pure virtual")
}

fn default_get_depart_edge(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getDepartEdge is pure virtual")
}

fn default_get_number_reroutes(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getNumberReroutes is pure virtual")
}

fn default_allows_boarding(_: &SUMOVehicle, _: Ptr<MSTransportable>) -> bool {
    panic!("SUMOVehicle::allowsBoarding is pure virtual")
}

fn default_add_transportable(_: &mut SUMOVehicle, _: Ptr<MSTransportable>) {
    panic!("SUMOVehicle::addTransportable is pure virtual")
}

fn default_get_person_number(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getPersonNumber is pure virtual")
}

fn default_get_person_id_list(_: &SUMOVehicle) -> Vec<String> {
    panic!("SUMOVehicle::getPersonIDList is pure virtual")
}

fn default_get_container_number(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getContainerNumber is pure virtual")
}

fn default_remove_transportable(_: &mut SUMOVehicle, _: Ptr<MSTransportable>) {
    panic!("SUMOVehicle::removeTransportable is pure virtual")
}

fn default_remove_transportable_mass(_: &mut SUMOVehicle, _: Ptr<MSTransportable>) {
    panic!("SUMOVehicle::removeTransportableMass is pure virtual")
}

fn default_get_persons(_: &SUMOVehicle) -> &Vec<Ptr<MSTransportable>> {
    panic!("SUMOVehicle::getPersons is pure virtual")
}

fn default_get_containers(_: &SUMOVehicle) -> &Vec<Ptr<MSTransportable>> {
    panic!("SUMOVehicle::getContainers is pure virtual")
}

fn default_add_stop(_: &mut SUMOVehicle, _: &SUMOVehicleParameter_Stop, _: &mut String, _: SUMOTime, _: Ptr<crate::microsim::ms_route::MSRouteIterator>) -> bool {
    panic!("SUMOVehicle::addStop is pure virtual")
}

fn default_get_stop_indices(_: &SUMOVehicle) -> Vec<(i32, f64)> {
    panic!("SUMOVehicle::getStopIndices is pure virtual")
}

fn default_is_line_stop(_: &SUMOVehicle, _: f64) -> bool {
    panic!("SUMOVehicle::isLineStop is pure virtual")
}

fn default_abort_next_stop(_: &mut SUMOVehicle, _: i32) -> bool {
    panic!("SUMOVehicle::abortNextStop is pure virtual")
}

fn default_get_next_parking_area(_: &mut SUMOVehicle) -> Ptr<MSParkingArea> {
    panic!("SUMOVehicle::getNextParkingArea is pure virtual")
}

fn default_replace_parking_area(_: &mut SUMOVehicle, _: Ptr<MSParkingArea>, _: &mut String) -> bool {
    panic!("SUMOVehicle::replaceParkingArea is pure virtual")
}

fn default_get_parking_badges(_: &SUMOVehicle) -> &Vec<String> {
    panic!("SUMOVehicle::getParkingBadges is pure virtual")
}

fn default_remaining_stop_duration(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::remainingStopDuration is pure virtual")
}

fn default_is_stopped_triggered(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isStoppedTriggered is pure virtual")
}

fn default_is_stopped_parking(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isStoppedParking is pure virtual")
}

fn default_is_stopped_in_range(_: &SUMOVehicle, _: f64, _: f64, _: bool) -> bool {
    panic!("SUMOVehicle::isStoppedInRange is pure virtual")
}

fn default_stops_at(_: &SUMOVehicle, _: Ptr<MSStoppingPlace>) -> bool {
    panic!("SUMOVehicle::stopsAt is pure virtual")
}

fn default_stops_at_edge(_: &SUMOVehicle, _: Ptr<MSEdge>) -> bool {
    panic!("SUMOVehicle::stopsAtEdge is pure virtual")
}

fn default_has_stops(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::hasStops is pure virtual")
}

fn default_get_stops(_: &SUMOVehicle) -> &VecDeque<MSStop> {
    panic!("SUMOVehicle::getStops is pure virtual")
}

fn default_get_next_stop(_: &SUMOVehicle) -> &MSStop {
    panic!("SUMOVehicle::getNextStop is pure virtual")
}

fn default_get_next_stop_mutable(_: &mut SUMOVehicle) -> &mut MSStop {
    panic!("SUMOVehicle::getNextStopMutable is pure virtual")
}

fn default_unregister_waiting(_: &mut SUMOVehicle) {
    panic!("SUMOVehicle::unregisterWaiting is pure virtual")
}

fn default_get_next_stop_parameter(_: &SUMOVehicle) -> Ptr<SUMOVehicleParameter_Stop> {
    panic!("SUMOVehicle::getNextStopParameter is pure virtual")
}

fn default_get_stop_duration(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::getStopDuration is pure virtual")
}

fn default_add_traci_stop(_: &mut SUMOVehicle, _: SUMOVehicleParameter_Stop, _: &mut String) -> bool {
    panic!("SUMOVehicle::addTraciStop is pure virtual")
}

fn default_set_chosen_speed_factor(_: &mut SUMOVehicle, _: f64) {
    panic!("SUMOVehicle::setChosenSpeedFactor is pure virtual")
}

fn default_get_depart_delay(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::getDepartDelay is pure virtual")
}

fn default_get_time_loss(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::getTimeLoss is pure virtual")
}

fn default_get_brake_gap(_: &SUMOVehicle, _: bool) -> f64 {
    panic!("SUMOVehicle::getBrakeGap is pure virtual")
}

fn default_get_impatience(_: &SUMOVehicle) -> f64 {
    panic!("SUMOVehicle::getImpatience is pure virtual")
}

fn default_get_devices(_: &SUMOVehicle) -> &Vec<Ptr<MSVehicleDevice>> {
    panic!("SUMOVehicle::getDevices is pure virtual")
}

fn default_is_rail(_: &SUMOVehicle) -> bool {
    panic!("SUMOVehicle::isRail is pure virtual")
}

fn default_get_last_action_time(_: &SUMOVehicle) -> SUMOTime {
    panic!("SUMOVehicle::getLastActionTime is pure virtual")
}

fn default_get_bounding_box(_: &SUMOVehicle, _: f64) -> PositionVector {
    panic!("SUMOVehicle::getBoundingBox is pure virtual")
}

fn default_get_scope_lock(_: &mut SUMOVehicle) -> Box<Unmapped> {
    panic!("SUMOVehicle::getScopeLock is pure virtual")
}

fn default_remember_blocked_parking_area(_: &mut SUMOVehicle, _: Ptr<MSStoppingPlace>, _: bool) {
    panic!("SUMOVehicle::rememberBlockedParkingArea is pure virtual")
}

fn default_saw_blocked_parking_area(_: &SUMOVehicle, _: Ptr<MSStoppingPlace>, _: bool) -> SUMOTime {
    panic!("SUMOVehicle::sawBlockedParkingArea is pure virtual")
}

fn default_remember_parking_area_score(_: &mut SUMOVehicle, _: Ptr<MSStoppingPlace>, _: &str) {
    panic!("SUMOVehicle::rememberParkingAreaScore is pure virtual")
}

fn default_reset_parking_area_scores(_: &mut SUMOVehicle) {
    panic!("SUMOVehicle::resetParkingAreaScores is pure virtual")
}

fn default_get_number_parking_reroutes(_: &SUMOVehicle) -> i32 {
    panic!("SUMOVehicle::getNumberParkingReroutes is pure virtual")
}

fn default_set_number_parking_reroutes(_: &mut SUMOVehicle, _: i32) {
    panic!("SUMOVehicle::setNumberParkingReroutes is pure virtual")
}

fn default_remember_blocked_charging_station(_: &mut SUMOVehicle, _: Ptr<MSStoppingPlace>, _: bool) {
    panic!("SUMOVehicle::rememberBlockedChargingStation is pure virtual")
}

fn default_saw_blocked_charging_station(_: &SUMOVehicle, _: Ptr<MSStoppingPlace>, _: bool) -> SUMOTime {
    panic!("SUMOVehicle::sawBlockedChargingStation is pure virtual")
}

fn default_remember_charging_station_score(_: &mut SUMOVehicle, _: Ptr<MSStoppingPlace>, _: &str) {
    panic!("SUMOVehicle::rememberChargingStationScore is pure virtual")
}

fn default_reset_charging_station_scores(_: &mut SUMOVehicle) {
    panic!("SUMOVehicle::resetChargingStationScores is pure virtual")
}

fn default_save_state(_: &mut SUMOVehicle, _: &mut OutputDevice) {
    panic!("SUMOVehicle::saveState is pure virtual")
}

fn default_load_state(_: &mut SUMOVehicle, _: &SUMOSAXAttributes, _: SUMOTime) {
    panic!("SUMOVehicle::loadState is pure virtual")
}

static DEFAULT_SUMO_VEHICLE_VT: SUMOVehicleVt = SUMOVehicleVt {
    get_route: default_get_route,
    get_departure: default_get_departure,
    get_depart_pos: default_get_depart_pos,
    get_odometer: default_get_odometer,
    has_departed: default_has_departed,
    is_stopped: default_is_stopped,
    get_length: default_get_length,
    get_current_edge: default_get_current_edge,
    is_on_road: default_is_on_road,
    get_lateral_position_on_lane: default_get_lateral_position_on_lane,
    get_angle: default_get_angle,
    get_route_ptr: default_get_route_ptr,
    get_route_position: default_get_route_position,
    succ_edge: default_succ_edge,
    get_reroute_origin: default_get_reroute_origin,
    replace_route_edges: default_replace_route_edges,
    reroute: default_reroute,
    get_router_tt: default_get_router_tt,
    has_valid_route: default_has_valid_route,
    has_valid_route_start: default_has_valid_route_start,
    get_route_validity: default_get_route_validity,
    get_current_route_edge: default_get_current_route_edge,
    get_emission_parameters: default_get_emission_parameters,
    replace_parameter: default_replace_parameter,
    on_depart: default_on_depart,
    is_idling: default_is_idling,
    is_front_on_lane: default_is_front_on_lane,
    is_parking: default_is_parking,
    is_remote_controlled: default_is_remote_controlled,
    was_remote_controlled: default_was_remote_controlled,
    get_arrival_pos: default_get_arrival_pos,
    set_arrival_pos: default_set_arrival_pos,
    get_depart_edge: default_get_depart_edge,
    get_number_reroutes: default_get_number_reroutes,
    allows_boarding: default_allows_boarding,
    add_transportable: default_add_transportable,
    get_person_number: default_get_person_number,
    get_person_id_list: default_get_person_id_list,
    get_container_number: default_get_container_number,
    remove_transportable: default_remove_transportable,
    remove_transportable_mass: default_remove_transportable_mass,
    get_persons: default_get_persons,
    get_containers: default_get_containers,
    add_stop: default_add_stop,
    get_stop_indices: default_get_stop_indices,
    is_line_stop: default_is_line_stop,
    abort_next_stop: default_abort_next_stop,
    get_next_parking_area: default_get_next_parking_area,
    replace_parking_area: default_replace_parking_area,
    get_parking_badges: default_get_parking_badges,
    remaining_stop_duration: default_remaining_stop_duration,
    is_stopped_triggered: default_is_stopped_triggered,
    is_stopped_parking: default_is_stopped_parking,
    is_stopped_in_range: default_is_stopped_in_range,
    stops_at: default_stops_at,
    stops_at_edge: default_stops_at_edge,
    has_stops: default_has_stops,
    get_stops: default_get_stops,
    get_next_stop: default_get_next_stop,
    get_next_stop_mutable: default_get_next_stop_mutable,
    unregister_waiting: default_unregister_waiting,
    get_next_stop_parameter: default_get_next_stop_parameter,
    get_stop_duration: default_get_stop_duration,
    add_traci_stop: default_add_traci_stop,
    set_chosen_speed_factor: default_set_chosen_speed_factor,
    get_depart_delay: default_get_depart_delay,
    get_time_loss: default_get_time_loss,
    get_brake_gap: default_get_brake_gap,
    get_impatience: default_get_impatience,
    get_devices: default_get_devices,
    is_rail: default_is_rail,
    get_last_action_time: default_get_last_action_time,
    get_bounding_box: default_get_bounding_box,
    get_scope_lock: default_get_scope_lock,
    remember_blocked_parking_area: default_remember_blocked_parking_area,
    saw_blocked_parking_area: default_saw_blocked_parking_area,
    remember_parking_area_score: default_remember_parking_area_score,
    reset_parking_area_scores: default_reset_parking_area_scores,
    get_number_parking_reroutes: default_get_number_parking_reroutes,
    set_number_parking_reroutes: default_set_number_parking_reroutes,
    remember_blocked_charging_station: default_remember_blocked_charging_station,
    saw_blocked_charging_station: default_saw_blocked_charging_station,
    remember_charging_station_score: default_remember_charging_station_score,
    reset_charging_station_scores: default_reset_charging_station_scores,
    save_state: default_save_state,
    load_state: default_load_state,
};

/// Representation of a vehicle
/// C++ class `SUMOVehicle` (utils/vehicle/SUMOVehicle.h)
#[repr(C)]
pub struct SUMOVehicle {
    /// C++ base class `SUMOTrafficObject`
    pub base: SUMOTrafficObject,
    /// Virtual dispatch table; must stay after `base`
    pub vt: &'static SUMOVehicleVt,
}

impl std::ops::Deref for SUMOVehicle {
    type Target = SUMOTrafficObject;
    fn deref(&self) -> &SUMOTrafficObject {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOVehicle {
    fn deref_mut(&mut self) -> &mut SUMOTrafficObject {
        &mut self.base
    }
}

impl SUMOVehicle {
    /// C++: `void SUMOVehicle::SUMOVehicle(const std::string &)`
    /// Constructor
    // SUMOVehicle(const std::string& id) : SUMOTrafficObject(id) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        SUMOVehicle { base: SUMOTrafficObject::new(id), vt: &DEFAULT_SUMO_VEHICLE_VT }
    }

    /// C++: `void SUMOVehicle::~SUMOVehicle() noexcept` [virtual]
    /// Destructor
    // virtual ~SUMOVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double SUMOVehicle::getLateralPositionOnLane() const` [const, virtual, pure virtual]
    /// Get the vehicle's lateral position on the lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_lateral_position_on_lane(&self) -> f64 {
        (self.vt.get_lateral_position_on_lane)(self)
    }

    /// C++: `double SUMOVehicle::getAngle() const` [const, virtual, pure virtual]
    /// Get the vehicle's angle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        (self.vt.get_angle)(self)
    }

    /// C++: `const MSRoute & SUMOVehicle::getRoute() const` [const, virtual, pure virtual]
    /// Returns the current route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &MSRoute {
        (self.vt.get_route)(self)
    }

    /// C++: `ConstMSRoutePtr SUMOVehicle::getRoutePtr() const` [const, virtual, pure virtual]
    /// Returns the current route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route_ptr(&self) -> ConstMSRoutePtr {
        (self.vt.get_route_ptr)(self)
    }

    /// C++: `int SUMOVehicle::getRoutePosition() const` [const, virtual, pure virtual]
    /// return index of edge within route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        (self.vt.get_route_position)(self)
    }

    /// C++: `const MSEdge * SUMOVehicle::succEdge(int) const` [const, virtual, pure virtual]
    /// Returns the nSuccs'th successor of edge the vehicle is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn succ_edge(&self, nSuccs: i32) -> Ptr<MSEdge> {
        (self.vt.succ_edge)(self, nSuccs)
    }

    /// C++: `ConstMSEdgeVector::const_iterator SUMOVehicle::getRerouteOrigin() const` [const, virtual, pure virtual]
    /// Returns the starting point for reroutes (usually the current edge)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_reroute_origin(&self) -> crate::microsim::ms_route::MSRouteIterator {
        (self.vt.get_reroute_origin)(self)
    }

    /// C++: `bool SUMOVehicle::replaceRouteEdges(ConstMSEdgeVector &, double, double, const std::string &, bool, bool, bool, std::string *)` [virtual, pure virtual]
    /// Replaces the current route by the given edges
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_route_edges(&mut self, edges: &mut ConstMSEdgeVector, cost: f64, savings: f64, info: &str, onInit: bool, check: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        (self.vt.replace_route_edges)(self, edges, cost, savings, info, onInit, check, removeStops, msgReturn)
    }

    /// C++: `bool SUMOVehicle::reroute(SUMOTime, const std::string &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, const bool, const bool, const bool, const MSEdge *)` [virtual, pure virtual]
    /// Performs a rerouting using the given router
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reroute(&mut self, t: SUMOTime, info: &str, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, onInit: bool, withTaz: bool, silent: bool, sink: Ptr<MSEdge>) -> bool {
        (self.vt.reroute)(self, t, info, router, onInit, withTaz, silent, sink)
    }

    /// C++: `SUMOAbstractRouter<MSEdge, SUMOVehicle> & SUMOVehicle::getRouterTT() const` [const, virtual, pure virtual]
    /// returns the router according to the current routing mode of the vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_router_tt(&self) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle> {
        (self.vt.get_router_tt)(self)
    }

    /// C++: `bool SUMOVehicle::hasValidRoute(std::string &, ConstMSRoutePtr) const` [const, virtual, pure virtual]
    /// Validates the current or given route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_valid_route(&self, msg: &mut String, route: ConstMSRoutePtr) -> bool {
        (self.vt.has_valid_route)(self, msg, route)
    }

    /// C++: `bool SUMOVehicle::hasValidRouteStart(std::string &)` [virtual, pure virtual]
    /// checks whether the vehicle can depart on the first edge
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_valid_route_start(&mut self, msg: &mut String) -> bool {
        (self.vt.has_valid_route_start)(self, msg)
    }

    /// C++: `int SUMOVehicle::getRouteValidity(bool, bool, std::string *)` [virtual, pure virtual]
    /// computes validity attributes for the current route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route_validity(&mut self, update: bool, silent: bool, msgReturn: Ptr<String>) -> i32 {
        (self.vt.get_route_validity)(self, update, silent, msgReturn)
    }

    /// C++: `const ConstMSEdgeVector::const_iterator & SUMOVehicle::getCurrentRouteEdge() const` [const, virtual, pure virtual]
    /// Returns an iterator pointing to the current edge in this vehicles route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_route_edge(&self) -> &crate::microsim::ms_route::MSRouteIterator {
        (self.vt.get_current_route_edge)(self)
    }

    /// C++: `const MSEdge * SUMOVehicle::getCurrentEdge() const` [const, virtual, pure virtual]
    /// Returns the current edge which may be internal
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        (self.vt.get_current_edge)(self)
    }

    /// C++: `EnergyParams * SUMOVehicle::getEmissionParameters() const` [const, virtual, pure virtual]
    /// Returns the vehicle's emission model parameter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_emission_parameters(&self) -> Ptr<EnergyParams> {
        (self.vt.get_emission_parameters)(self)
    }

    /// C++: `void SUMOVehicle::replaceParameter(const SUMOVehicleParameter *)` [virtual, pure virtual]
    /// Replaces the vehicle's parameter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_parameter(&mut self, newParameter: Ptr<SUMOVehicleParameter>) {
        (self.vt.replace_parameter)(self, newParameter)
    }

    /// C++: `void SUMOVehicle::onDepart()` [virtual, pure virtual]
    /// Called when the vehicle is inserted into the network
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn on_depart(&mut self) {
        (self.vt.on_depart)(self)
    }

    /// C++: `bool SUMOVehicle::isOnRoad() const` [const, virtual, pure virtual]
    /// Returns the information whether the vehicle is on a road (is simulated)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_on_road(&self) -> bool {
        (self.vt.is_on_road)(self)
    }

    /// C++: `bool SUMOVehicle::isIdling() const` [const, virtual, pure virtual]
    /// Returns whether the vehicle is idling (waiting to re-enter the net
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_idling(&self) -> bool {
        (self.vt.is_idling)(self)
    }

    /// C++: `bool SUMOVehicle::isFrontOnLane(const MSLane *) const` [const, virtual, pure virtual]
    /// Returns the information whether the front of the vehhicle is on the given lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_front_on_lane(&self, arg0: Ptr<MSLane>) -> bool {
        (self.vt.is_front_on_lane)(self, arg0)
    }

    /// C++: `bool SUMOVehicle::isParking() const` [const, virtual, pure virtual]
    /// Returns the information whether the vehicle is parked
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_parking(&self) -> bool {
        (self.vt.is_parking)(self)
    }

    /// C++: `bool SUMOVehicle::isRemoteControlled() const` [const, virtual, pure virtual]
    /// Returns the information whether the vehicle is fully controlled via TraCI
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        (self.vt.is_remote_controlled)(self)
    }

    /// C++: `bool SUMOVehicle::wasRemoteControlled(SUMOTime) const` [const, virtual, pure virtual]
    /// Returns the information whether the vehicle is fully controlled via TraCI
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn was_remote_controlled(&self, lookBack: SUMOTime) -> bool {
        (self.vt.was_remote_controlled)(self, lookBack)
    }

    /// C++: `SUMOTime SUMOVehicle::getDeparture() const` [const, virtual, pure virtual]
    /// Returns this vehicle's real departure time
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_departure(&self) -> SUMOTime {
        (self.vt.get_departure)(self)
    }

    /// C++: `double SUMOVehicle::getDepartPos() const` [const, virtual, pure virtual]
    /// Returns this vehicle's real departure position
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self) -> f64 {
        (self.vt.get_depart_pos)(self)
    }

    /// C++: `double SUMOVehicle::getArrivalPos() const` [const, virtual, pure virtual]
    /// Returns this vehicle's desired arrivalPos for its current route (may change on reroute)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        (self.vt.get_arrival_pos)(self)
    }

    /// C++: `void SUMOVehicle::setArrivalPos(double)` [virtual, pure virtual]
    /// Sets this vehicle's desired arrivalPos for its current route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_arrival_pos(&mut self, arrivalPos: f64) {
        (self.vt.set_arrival_pos)(self, arrivalPos)
    }

    /// C++: `bool SUMOVehicle::hasDeparted() const` [const, virtual, pure virtual]
    /// Returns whether this vehicle has departed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_departed(&self) -> bool {
        (self.vt.has_departed)(self)
    }

    /// C++: `int SUMOVehicle::getDepartEdge() const` [const, virtual, pure virtual]
    /// Returns the edge on which this vehicle shall depart
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> i32 {
        (self.vt.get_depart_edge)(self)
    }

    /// C++: `double SUMOVehicle::getOdometer() const` [const, virtual, pure virtual]
    /// Returns the distance that was already driven by this vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_odometer(&self) -> f64 {
        (self.vt.get_odometer)(self)
    }

    /// C++: `int SUMOVehicle::getNumberReroutes() const` [const, virtual, pure virtual]
    /// Returns the number of new routes this vehicle got
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_number_reroutes(&self) -> i32 {
        (self.vt.get_number_reroutes)(self)
    }

    /// C++: `bool SUMOVehicle::allowsBoarding(const MSTransportable *) const` [const, virtual, pure virtual]
    /// whether the given transportable is allowed to board this vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn allows_boarding(&self, t: Ptr<MSTransportable>) -> bool {
        (self.vt.allows_boarding)(self, t)
    }

    /// C++: `void SUMOVehicle::addTransportable(MSTransportable *)` [virtual, pure virtual]
    /// Adds a person or container to this vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, transportable: Ptr<MSTransportable>) {
        (self.vt.add_transportable)(self, transportable)
    }

    /// C++: `int SUMOVehicle::getPersonNumber() const` [const, virtual, pure virtual]
    /// Returns the number of persons
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_person_number(&self) -> i32 {
        (self.vt.get_person_number)(self)
    }

    /// C++: `std::vector<std::string> SUMOVehicle::getPersonIDList() const` [const, virtual, pure virtual]
    /// Returns the list of persons
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_person_id_list(&self) -> Vec<String> {
        (self.vt.get_person_id_list)(self)
    }

    /// C++: `int SUMOVehicle::getContainerNumber() const` [const, virtual, pure virtual]
    /// Returns the number of containers
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_container_number(&self) -> i32 {
        (self.vt.get_container_number)(self)
    }

    /// C++: `void SUMOVehicle::removeTransportable(MSTransportable *)` [virtual, pure virtual]
    /// removes a person or container
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remove_transportable(&mut self, t: Ptr<MSTransportable>) {
        (self.vt.remove_transportable)(self, t)
    }

    /// C++: `void SUMOVehicle::removeTransportableMass(MSTransportable *)` [virtual, pure virtual]
    /// removes a person or containers mass
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remove_transportable_mass(&mut self, t: Ptr<MSTransportable>) {
        (self.vt.remove_transportable_mass)(self, t)
    }

    /// C++: `const std::vector<MSTransportable *> & SUMOVehicle::getPersons() const` [const, virtual, pure virtual]
    /// retrieve riding persons
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_persons(&self) -> &Vec<Ptr<MSTransportable>> {
        (self.vt.get_persons)(self)
    }

    /// C++: `const std::vector<MSTransportable *> & SUMOVehicle::getContainers() const` [const, virtual, pure virtual]
    /// retrieve riding containers
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_containers(&self) -> &Vec<Ptr<MSTransportable>> {
        (self.vt.get_containers)(self)
    }

    /// C++: `bool SUMOVehicle::addStop(const SUMOVehicleParameter::Stop &, std::string &, SUMOTime, ConstMSEdgeVector::const_iterator *)` [virtual, pure virtual]
    /// Adds a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, stopPar: &SUMOVehicleParameter_Stop, errorMsg: &mut String, untilOffset: SUMOTime, searchStart: Ptr<crate::microsim::ms_route::MSRouteIterator>) -> bool {
        (self.vt.add_stop)(self, stopPar, errorMsg, untilOffset, searchStart)
    }

    /// C++: `std::vector<std::pair<int, double>> SUMOVehicle::getStopIndices() const` [const, virtual, pure virtual]
    /// return list of route indices and stop positions for the remaining stops
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stop_indices(&self) -> Vec<(i32, f64)> {
        (self.vt.get_stop_indices)(self)
    }

    /// C++: `bool SUMOVehicle::isLineStop(double) const` [const, virtual, pure virtual]
    /// returns whether the vehicle serves a public transport line that serves the given stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_line_stop(&self, position: f64) -> bool {
        (self.vt.is_line_stop)(self, position)
    }

    /// C++: `bool SUMOVehicle::abortNextStop(int)` [virtual, pure virtual]
    /// deletes the next stop at the given index if it exists
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn abort_next_stop(&mut self, nextStopIndex: i32) -> bool {
        (self.vt.abort_next_stop)(self, nextStopIndex)
    }

    /// C++: `MSParkingArea * SUMOVehicle::getNextParkingArea()` [virtual, pure virtual]
    /// returns the next imminent stop in the stop queue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_parking_area(&mut self) -> Ptr<MSParkingArea> {
        (self.vt.get_next_parking_area)(self)
    }

    /// C++: `bool SUMOVehicle::replaceParkingArea(MSParkingArea *, std::string &)` [virtual, pure virtual]
    /// Replaces a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_parking_area(&mut self, parkingArea: Ptr<MSParkingArea>, errorMsg: &mut String) -> bool {
        (self.vt.replace_parking_area)(self, parkingArea, errorMsg)
    }

    /// C++: `const std::vector<std::string> & SUMOVehicle::getParkingBadges() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_parking_badges(&self) -> &Vec<String> {
        (self.vt.get_parking_badges)(self)
    }

    /// C++: `SUMOTime SUMOVehicle::remainingStopDuration() const` [const, virtual, pure virtual]
    /// Returns the remaining stop duration for a stopped vehicle or 0
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remaining_stop_duration(&self) -> SUMOTime {
        (self.vt.remaining_stop_duration)(self)
    }

    /// C++: `bool SUMOVehicle::isStopped() const` [const, virtual, pure virtual]
    /// Returns whether the vehicle is at a stop and waiting for a person or container to continue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        (self.vt.is_stopped)(self)
    }

    /// C++: `bool SUMOVehicle::isStoppedTriggered() const` [const, virtual, pure virtual]
    /// Returns whether the vehicle is at a stop and waiting for a person or container to continue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped_triggered(&self) -> bool {
        (self.vt.is_stopped_triggered)(self)
    }

    /// C++: `bool SUMOVehicle::isStoppedParking() const` [const, virtual, pure virtual]
    /// Returns whether the vehicle is at a stop and parking
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped_parking(&self) -> bool {
        (self.vt.is_stopped_parking)(self)
    }

    /// C++: `bool SUMOVehicle::isStoppedInRange(const double, const double, bool) const` [const, virtual, pure virtual]
    /// Returns whether the vehicle is stopped in the range of the given position
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped_in_range(&self, pos: f64, tolerance: f64, checkFuture: bool) -> bool {
        (self.vt.is_stopped_in_range)(self, pos, tolerance, checkFuture)
    }

    /// C++: `bool SUMOVehicle::stopsAt(MSStoppingPlace *) const` [const, virtual, pure virtual]
    /// Returns whether the vehicle stops at the given stopping place
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn stops_at(&self, stop: Ptr<MSStoppingPlace>) -> bool {
        (self.vt.stops_at)(self, stop)
    }

    /// C++: `bool SUMOVehicle::stopsAtEdge(const MSEdge *) const` [const, virtual, pure virtual]
    /// Returns whether the vehicle stops at the given edge
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn stops_at_edge(&self, edge: Ptr<MSEdge>) -> bool {
        (self.vt.stops_at_edge)(self, edge)
    }

    /// C++: `bool SUMOVehicle::hasStops() const` [const, virtual, pure virtual]
    /// Returns whether the vehicle has to stop somewhere
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_stops(&self) -> bool {
        (self.vt.has_stops)(self)
    }

    /// C++: `const std::list<MSStop> & SUMOVehicle::getStops() const` [const, virtual, pure virtual]
    /// returns the list of stops not yet reached in the stop queue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &VecDeque<MSStop> {
        (self.vt.get_stops)(self)
    }

    /// C++: `const MSStop & SUMOVehicle::getNextStop() const` [const, virtual, pure virtual]
    /// returns the next imminent stop in the stop queue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_stop(&self) -> &MSStop {
        (self.vt.get_next_stop)(self)
    }

    /// C++: `MSStop & SUMOVehicle::getNextStopMutable()` [virtual, pure virtual]
    /// returns the next imminent stop in the stop queue
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_stop_mutable(&mut self) -> &mut MSStop {
        (self.vt.get_next_stop_mutable)(self)
    }

    /// C++: `void SUMOVehicle::unregisterWaiting()` [virtual, pure virtual]
    /// mark vehicle as active
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn unregister_waiting(&mut self) {
        (self.vt.unregister_waiting)(self)
    }

    /// C++: `const SUMOVehicleParameter::Stop * SUMOVehicle::getNextStopParameter() const` [const, virtual, pure virtual]
    /// Returns parameters of the next stop or nullptr *
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_stop_parameter(&self) -> Ptr<SUMOVehicleParameter_Stop> {
        (self.vt.get_next_stop_parameter)(self)
    }

    /// C++: `SUMOTime SUMOVehicle::getStopDuration() const` [const, virtual, pure virtual]
    /// get remaining stop duration or 0 if the vehicle isn't stopped
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_stop_duration(&self) -> SUMOTime {
        (self.vt.get_stop_duration)(self)
    }

    /// C++: `bool SUMOVehicle::addTraciStop(SUMOVehicleParameter::Stop, std::string &)` [virtual, pure virtual]
    /// schedule a new stop for the vehicle; each time a stop is reached, the vehicle will wait for the given duration before continuing on its route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_traci_stop(&mut self, stop: SUMOVehicleParameter_Stop, errorMsg: &mut String) -> bool {
        (self.vt.add_traci_stop)(self, stop, errorMsg)
    }

    /// C++: `void SUMOVehicle::setChosenSpeedFactor(const double)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_chosen_speed_factor(&mut self, factor: f64) {
        (self.vt.set_chosen_speed_factor)(self, factor)
    }

    /// C++: `SUMOTime SUMOVehicle::getDepartDelay() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_depart_delay(&self) -> SUMOTime {
        (self.vt.get_depart_delay)(self)
    }

    /// C++: `SUMOTime SUMOVehicle::getTimeLoss() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_time_loss(&self) -> SUMOTime {
        (self.vt.get_time_loss)(self)
    }

    /// C++: `double SUMOVehicle::getBrakeGap(bool) const` [const, virtual, pure virtual]
    /// get distance for coming to a stop (used for rerouting checks)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_brake_gap(&self, delayed: bool) -> f64 {
        (self.vt.get_brake_gap)(self, delayed)
    }

    /// C++: `double SUMOVehicle::getImpatience() const` [const, virtual, pure virtual]
    /// Returns this vehicles impatience
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        (self.vt.get_impatience)(self)
    }

    /// C++: `const std::vector<MSVehicleDevice *> & SUMOVehicle::getDevices() const` [const, virtual, pure virtual]
    /// Returns this vehicle's devices
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_devices(&self) -> &Vec<Ptr<MSVehicleDevice>> {
        todo!("port SUMOVehicle::getDevices")
    }

    /// C++: `double SUMOVehicle::getLength() const` [const, virtual, pure virtual]
    /// Returns the vehicles's length
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        (self.vt.get_length)(self)
    }

    /// C++: `bool SUMOVehicle::isRail() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_rail(&self) -> bool {
        todo!("port SUMOVehicle::isRail")
    }

    /// C++: `SUMOTime SUMOVehicle::getLastActionTime() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_last_action_time(&self) -> SUMOTime {
        todo!("port SUMOVehicle::getLastActionTime")
    }

    /// C++: `PositionVector SUMOVehicle::getBoundingBox(double) const` [const, virtual, pure virtual]
    /// get bounding rectangle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self, offset: f64) -> PositionVector {
        todo!("port SUMOVehicle::getBoundingBox")
    }

    /// C++: `std::unique_ptr<MFXOptionalLock> SUMOVehicle::getScopeLock()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_scope_lock(&mut self) -> Box<Unmapped /* MFXOptionalLock */> {
        todo!("port SUMOVehicle::getScopeLock")
    }

    /// C++: `void SUMOVehicle::rememberBlockedParkingArea(const MSStoppingPlace *, bool)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_blocked_parking_area(&mut self, pa: Ptr<MSStoppingPlace>, local: bool) {
        todo!("port SUMOVehicle::rememberBlockedParkingArea")
    }

    /// C++: `SUMOTime SUMOVehicle::sawBlockedParkingArea(const MSStoppingPlace *, bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn saw_blocked_parking_area(&self, pa: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        todo!("port SUMOVehicle::sawBlockedParkingArea")
    }

    /// C++: `void SUMOVehicle::rememberParkingAreaScore(const MSStoppingPlace *, const std::string &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_parking_area_score(&mut self, pa: Ptr<MSStoppingPlace>, score: &str) {
        todo!("port SUMOVehicle::rememberParkingAreaScore")
    }

    /// C++: `void SUMOVehicle::resetParkingAreaScores()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_parking_area_scores(&mut self) {
        todo!("port SUMOVehicle::resetParkingAreaScores")
    }

    /// C++: `int SUMOVehicle::getNumberParkingReroutes() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_number_parking_reroutes(&self) -> i32 {
        todo!("port SUMOVehicle::getNumberParkingReroutes")
    }

    /// C++: `void SUMOVehicle::setNumberParkingReroutes(int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_number_parking_reroutes(&mut self, value: i32) {
        todo!("port SUMOVehicle::setNumberParkingReroutes")
    }

    /// C++: `void SUMOVehicle::rememberBlockedChargingStation(const MSStoppingPlace *, bool)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_blocked_charging_station(&mut self, cs: Ptr<MSStoppingPlace>, local: bool) {
        todo!("port SUMOVehicle::rememberBlockedChargingStation")
    }

    /// C++: `SUMOTime SUMOVehicle::sawBlockedChargingStation(const MSStoppingPlace *, bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn saw_blocked_charging_station(&self, cs: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        todo!("port SUMOVehicle::sawBlockedChargingStation")
    }

    /// C++: `void SUMOVehicle::rememberChargingStationScore(const MSStoppingPlace *, const std::string &)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn remember_charging_station_score(&mut self, cs: Ptr<MSStoppingPlace>, score: &str) {
        todo!("port SUMOVehicle::rememberChargingStationScore")
    }

    /// C++: `void SUMOVehicle::resetChargingStationScores()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn reset_charging_station_scores(&mut self) {
        todo!("port SUMOVehicle::resetChargingStationScores")
    }

    /// C++: `void SUMOVehicle::saveState(OutputDevice &)` [virtual, pure virtual]
    /// Saves the states of a vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        todo!("port SUMOVehicle::saveState")
    }

    /// C++: `void SUMOVehicle::loadState(const SUMOSAXAttributes &, const SUMOTime)` [virtual, pure virtual]
    /// Loads the state of this vehicle from the given description
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes, offset: SUMOTime) {
        todo!("port SUMOVehicle::loadState")
    }

}

#[cfg(test)]
mod vt_dispatch_tests {
    use super::*;

    fn stopped_true(_: &SUMOVehicle) -> bool {
        true
    }

    #[test]
    fn installed_table_is_used_by_base_method() {
        let mut v = SUMOVehicle::new("v");
        v.vt = Box::leak(Box::new(SUMOVehicleVt { is_stopped: stopped_true, ..DEFAULT_SUMO_VEHICLE_VT }));
        assert!(v.is_stopped());
    }
}

