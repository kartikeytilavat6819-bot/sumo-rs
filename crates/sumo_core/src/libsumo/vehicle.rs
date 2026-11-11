//! Port of libsumo/Vehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `libsumo::Vehicle` (libsumo/Vehicle.h)
pub struct Vehicle {
}

// C++ static members of Vehicle (port as module-level statics, see AGENTS.md):
//   static const int Vehicle::DOMAIN_ID;
//   static SubscriptionResults Vehicle::mySubscriptionResults;
//   static ContextSubscriptionResults Vehicle::myContextSubscriptionResults;

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
        todo!("port Vehicle::getSpeed")
    }

    /// C++: `double Vehicle::getLateralSpeed(const std::string &)` [static]
    // double
    // Vehicle::getLateralSpeed(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getLaneChangeModel().getSpeedLat(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_speed(vehID: &str) -> f64 {
        todo!("port Vehicle::getLateralSpeed")
    }

    /// C++: `double Vehicle::getAcceleration(const std::string &)` [static]
    // double
    // Vehicle::getAcceleration(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getAcceleration(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_acceleration(vehID: &str) -> f64 {
        todo!("port Vehicle::getAcceleration")
    }

    /// C++: `double Vehicle::getSpeedWithoutTraCI(const std::string &)` [static]
    // double
    // Vehicle::getSpeedWithoutTraCI(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getSpeedWithoutTraciInfluence(), veh->getSpeed()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_speed_without_tra_ci(vehID: &str) -> f64 {
        todo!("port Vehicle::getSpeedWithoutTraCI")
    }

    /// C++: `libsumo::TraCIPosition Vehicle::getPosition(const std::string &, const bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(vehID: &str, includeZ: bool) -> TraCIPosition {
        todo!("port Vehicle::getPosition")
    }

    /// C++: `libsumo::TraCIPosition Vehicle::getPosition3D(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position3_d(vehID: &str) -> TraCIPosition {
        todo!("port Vehicle::getPosition3D")
    }

    /// C++: `double Vehicle::getAngle(const std::string &)` [static]
    // double
    // Vehicle::getAngle(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? GeomHelper::naviDegree(veh->getAngle()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_angle(vehID: &str) -> f64 {
        todo!("port Vehicle::getAngle")
    }

    /// C++: `double Vehicle::getSlope(const std::string &)` [static]
    // double
    // Vehicle::getSlope(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return (veh->isOnRoad() || veh->isParking()) ? veh->getSlope() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_slope(vehID: &str) -> f64 {
        todo!("port Vehicle::getSlope")
    }

    /// C++: `std::string Vehicle::getRoadID(const std::string &)` [static]
    // std::string
    // Vehicle::getRoadID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? CALL_MICRO_FUN(veh, getLane()->getEdge().getID(), veh->getEdge()->getID()) : "";
    // }
    #[allow(unused_variables)]
    pub fn get_road_id(vehID: &str) -> String {
        todo!("port Vehicle::getRoadID")
    }

    /// C++: `std::string Vehicle::getLaneID(const std::string &)` [static]
    // std::string
    // Vehicle::getLaneID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLane()->getID(), "") : "";
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(vehID: &str) -> String {
        todo!("port Vehicle::getLaneID")
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
        todo!("port Vehicle::getShadowLaneID")
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
        todo!("port Vehicle::getLaneIndex")
    }

    /// C++: `std::string Vehicle::getSegmentID(const std::string &)` [static]
    // std::string
    // Vehicle::getSegmentID(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MESO_FUN(veh, getSegment()->getID(), "") : "";
    // }
    #[allow(unused_variables)]
    pub fn get_segment_id(vehID: &str) -> String {
        todo!("port Vehicle::getSegmentID")
    }

    /// C++: `int Vehicle::getSegmentIndex(const std::string &)` [static]
    // int
    // Vehicle::getSegmentIndex(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MESO_FUN(veh, getSegment()->getIndex(), INVALID_INT_VALUE) : INVALID_INT_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_segment_index(vehID: &str) -> i32 {
        todo!("port Vehicle::getSegmentIndex")
    }

    /// C++: `std::string Vehicle::getTypeID(const std::string &)` [static]
    // std::string
    // Vehicle::getTypeID(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getID();
    // }
    #[allow(unused_variables)]
    pub fn get_type_id(vehID: &str) -> String {
        todo!("port Vehicle::getTypeID")
    }

    /// C++: `std::string Vehicle::getRouteID(const std::string &)` [static]
    // std::string
    // Vehicle::getRouteID(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getRoute().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_route_id(vehID: &str) -> String {
        todo!("port Vehicle::getRouteID")
    }

    /// C++: `double Vehicle::getDeparture(const std::string &)` [static]
    // double
    // Vehicle::getDeparture(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->hasDeparted() ? STEPS2TIME(veh->getDeparture()) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_departure(vehID: &str) -> f64 {
        todo!("port Vehicle::getDeparture")
    }

    /// C++: `double Vehicle::getDepartDelay(const std::string &)` [static]
    // double
    // Vehicle::getDepartDelay(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicle(vehID)->getDepartDelay());
    // }
    #[allow(unused_variables)]
    pub fn get_depart_delay(vehID: &str) -> f64 {
        todo!("port Vehicle::getDepartDelay")
    }

    /// C++: `int Vehicle::getRouteIndex(const std::string &)` [static]
    // int
    // Vehicle::getRouteIndex(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->hasDeparted() ? veh->getRoutePosition() : INVALID_INT_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_route_index(vehID: &str) -> i32 {
        todo!("port Vehicle::getRouteIndex")
    }

    /// C++: `double Vehicle::getLanePosition(const std::string &)` [static]
    // double
    // Vehicle::getLanePosition(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return (veh->isOnRoad() || veh->isParking()) ? veh->getPositionOnLane() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lane_position(vehID: &str) -> f64 {
        todo!("port Vehicle::getLanePosition")
    }

    /// C++: `double Vehicle::getArrivalPos(const std::string &)` [static]
    // double
    // Vehicle::getArrivalPos(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getArrivalPos();
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(vehID: &str) -> f64 {
        todo!("port Vehicle::getArrivalPos")
    }

    /// C++: `double Vehicle::getLateralLanePosition(const std::string &)` [static]
    // double
    // Vehicle::getLateralLanePosition(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLateralPositionOnLane(), 0) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_lane_position(vehID: &str) -> f64 {
        todo!("port Vehicle::getLateralLanePosition")
    }

    /// C++: `double Vehicle::getCO2Emission(const std::string &)` [static]
    // double
    // Vehicle::getCO2Emission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::CO2>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_co2_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getCO2Emission")
    }

    /// C++: `double Vehicle::getCOEmission(const std::string &)` [static]
    // double
    // Vehicle::getCOEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::CO>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_co_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getCOEmission")
    }

    /// C++: `double Vehicle::getHCEmission(const std::string &)` [static]
    // double
    // Vehicle::getHCEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::HC>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_hc_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getHCEmission")
    }

    /// C++: `double Vehicle::getPMxEmission(const std::string &)` [static]
    // double
    // Vehicle::getPMxEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::PM_X>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_p_mx_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getPMxEmission")
    }

    /// C++: `double Vehicle::getNOxEmission(const std::string &)` [static]
    // double
    // Vehicle::getNOxEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::NO_X>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_n_ox_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getNOxEmission")
    }

    /// C++: `double Vehicle::getFuelConsumption(const std::string &)` [static]
    // double
    // Vehicle::getFuelConsumption(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::FUEL>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_fuel_consumption(vehID: &str) -> f64 {
        todo!("port Vehicle::getFuelConsumption")
    }

    /// C++: `double Vehicle::getNoiseEmission(const std::string &)` [static]
    // double
    // Vehicle::getNoiseEmission(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getHarmonoise_NoiseEmissions() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_noise_emission(vehID: &str) -> f64 {
        todo!("port Vehicle::getNoiseEmission")
    }

    /// C++: `double Vehicle::getElectricityConsumption(const std::string &)` [static]
    // double
    // Vehicle::getElectricityConsumption(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return isVisible(veh) ? veh->getEmissions<PollutantsInterface::ELEC>() : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_electricity_consumption(vehID: &str) -> f64 {
        todo!("port Vehicle::getElectricityConsumption")
    }

    /// C++: `int Vehicle::getPersonNumber(const std::string &)` [static]
    // int
    // Vehicle::getPersonNumber(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getPersonNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_person_number(vehID: &str) -> i32 {
        todo!("port Vehicle::getPersonNumber")
    }

    /// C++: `std::vector<std::string> Vehicle::getPersonIDList(const std::string &)` [static]
    // std::vector<std::string>
    // Vehicle::getPersonIDList(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getPersonIDList();
    // }
    #[allow(unused_variables)]
    pub fn get_person_id_list(vehID: &str) -> Vec<String> {
        todo!("port Vehicle::getPersonIDList")
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
        todo!("port Vehicle::getLeader")
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
        todo!("port Vehicle::getFollower")
    }

    /// C++: `std::vector<libsumo::TraCIJunctionFoe> Vehicle::getJunctionFoes(const std::string &, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_junction_foes(vehID: &str, dist: f64) -> Vec<TraCIJunctionFoe> {
        todo!("port Vehicle::getJunctionFoes")
    }

    /// C++: `double Vehicle::getWaitingTime(const std::string &)` [static]
    // double
    // Vehicle::getWaitingTime(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicle(vehID)->getWaitingTime());
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(vehID: &str) -> f64 {
        todo!("port Vehicle::getWaitingTime")
    }

    /// C++: `double Vehicle::getAccumulatedWaitingTime(const std::string &)` [static]
    // double
    // Vehicle::getAccumulatedWaitingTime(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getAccumulatedWaitingSeconds(), INVALID_DOUBLE_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_accumulated_waiting_time(vehID: &str) -> f64 {
        todo!("port Vehicle::getAccumulatedWaitingTime")
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
        todo!("port Vehicle::getAdaptedTraveltime")
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
        todo!("port Vehicle::getEffort")
    }

    /// C++: `bool Vehicle::isRouteValid(const std::string &)` [static]
    // bool
    // Vehicle::isRouteValid(const std::string& vehID) {
    //     std::string msg;
    //     return Helper::getVehicle(vehID)->hasValidRoute(msg);
    // }
    #[allow(unused_variables)]
    pub fn is_route_valid(vehID: &str) -> bool {
        todo!("port Vehicle::isRouteValid")
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
        todo!("port Vehicle::getRoute")
    }

    /// C++: `int Vehicle::getSignals(const std::string &)` [static]
    // int
    // Vehicle::getSignals(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getSignals(), MSVehicle::VEH_SIGNAL_NONE);
    // }
    #[allow(unused_variables)]
    pub fn get_signals(vehID: &str) -> i32 {
        todo!("port Vehicle::getSignals")
    }

    /// C++: `std::vector<libsumo::TraCIBestLanesData> Vehicle::getBestLanes(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_best_lanes(vehID: &str) -> Vec<TraCIBestLanesData> {
        todo!("port Vehicle::getBestLanes")
    }

    /// C++: `std::vector<libsumo::TraCINextTLSData> Vehicle::getNextTLS(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_next_tls(vehID: &str) -> Vec<TraCINextTLSData> {
        todo!("port Vehicle::getNextTLS")
    }

    /// C++: `std::vector<libsumo::TraCINextStopData> Vehicle::getNextStops(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_next_stops(vehID: &str) -> Vec<TraCINextStopData> {
        todo!("port Vehicle::getNextStops")
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
        todo!("port Vehicle::getNextLinks")
    }

    /// C++: `std::vector<libsumo::TraCINextStopData> Vehicle::getStops(const std::string &, int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stops(vehID: &str, limit: i32) -> Vec<TraCINextStopData> {
        todo!("port Vehicle::getStops")
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
        todo!("port Vehicle::getStopState")
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
    pub fn get_stop_parameter(vehID: &str, nextStopIndex: i32, param: &str, customParam: bool) -> String {
        todo!("port Vehicle::getStopParameter")
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
        todo!("port Vehicle::getDistance")
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
        todo!("port Vehicle::getDrivingDistance")
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
        todo!("port Vehicle::getDrivingDistance2D")
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
        todo!("port Vehicle::getReferenceDistance")
    }

    /// C++: `double Vehicle::getAllowedSpeed(const std::string &)` [static]
    // double
    // Vehicle::getAllowedSpeed(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return veh->isOnRoad() ? CALL_MICRO_FUN(veh, getLane()->getVehicleMaxSpeed(veh), veh->getEdge()->getVehicleMaxSpeed(veh)) : INVALID_DOUBLE_VALUE;
    // }
    #[allow(unused_variables)]
    pub fn get_allowed_speed(vehID: &str) -> f64 {
        todo!("port Vehicle::getAllowedSpeed")
    }

    /// C++: `int Vehicle::getSpeedMode(const std::string &)` [static]
    // int
    // Vehicle::getSpeedMode(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getInfluencer().getSpeedMode(), INVALID_INT_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_mode(vehID: &str) -> i32 {
        todo!("port Vehicle::getSpeedMode")
    }

    /// C++: `int Vehicle::getLaneChangeMode(const std::string &)` [static]
    // int
    // Vehicle::getLaneChangeMode(const std::string& vehID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     return CALL_MICRO_FUN(veh, getInfluencer().getLaneChangeMode(), INVALID_INT_VALUE);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_change_mode(vehID: &str) -> i32 {
        todo!("port Vehicle::getLaneChangeMode")
    }

    /// C++: `int Vehicle::getRoutingMode(const std::string &)` [static]
    // int
    // Vehicle::getRoutingMode(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getRoutingMode();
    // }
    #[allow(unused_variables)]
    pub fn get_routing_mode(vehID: &str) -> i32 {
        todo!("port Vehicle::getRoutingMode")
    }

    /// C++: `std::string Vehicle::getLine(const std::string &)` [static]
    // std::string
    // Vehicle::getLine(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getParameter().line;
    // }
    #[allow(unused_variables)]
    pub fn get_line(vehID: &str) -> String {
        todo!("port Vehicle::getLine")
    }

    /// C++: `std::vector<std::string> Vehicle::getVia(const std::string &)` [static]
    // std::vector<std::string>
    // Vehicle::getVia(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getParameter().via;
    // }
    #[allow(unused_variables)]
    pub fn get_via(vehID: &str) -> Vec<String> {
        todo!("port Vehicle::getVia")
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
        todo!("port Vehicle::getLaneChangeState")
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
        todo!("port Vehicle::getLastActionTime")
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
        todo!("port Vehicle::getNeighbors")
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
        todo!("port Vehicle::getFollowSpeed")
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
        todo!("port Vehicle::getSecureGap")
    }

    /// C++: `double Vehicle::getStopSpeed(const std::string &, double, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_speed(vehID: &str, speed: f64, gap: f64) -> f64 {
        todo!("port Vehicle::getStopSpeed")
    }

    /// C++: `double Vehicle::getStopDelay(const std::string &)` [static]
    // double
    // Vehicle::getStopDelay(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getStopDelay();
    // }
    #[allow(unused_variables)]
    pub fn get_stop_delay(vehID: &str) -> f64 {
        todo!("port Vehicle::getStopDelay")
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
        todo!("port Vehicle::getStopArrivalDelay")
    }

    /// C++: `double Vehicle::getTimeLoss(const std::string &)` [static]
    // double
    // Vehicle::getTimeLoss(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getTimeLossSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_time_loss(vehID: &str) -> f64 {
        todo!("port Vehicle::getTimeLoss")
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
        todo!("port Vehicle::getTaxiFleet")
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
        todo!("port Vehicle::getLoadedIDList")
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
        todo!("port Vehicle::getTeleportingIDList")
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
        todo!("port Vehicle::getIDList")
    }

    /// C++: `int Vehicle::getIDCount()` [static]
    /// @}
    // int
    // Vehicle::getIDCount() {
    //     return (int)getIDList().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        todo!("port Vehicle::getIDCount")
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
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Vehicle::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Vehicle::getParameterWithKey(const std::string &, const std::string &)` [static]
    /// @}
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Vehicle)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Vehicle::getParameterWithKey")
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
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Vehicle::setParameter")
    }

    /// C++: `double Vehicle::getLength(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getLength(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(typeID: &str) -> f64 {
        todo!("port Vehicle::getLength")
    }

    /// C++: `double Vehicle::getMaxSpeed(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMaxSpeed(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMaxSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(typeID: &str) -> f64 {
        todo!("port Vehicle::getMaxSpeed")
    }

    /// C++: `std::string Vehicle::getVehicleClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getVehicleClass(const std::string& vehID) {
    //     return toString(Helper::getVehicleType(vehID).getVehicleClass());
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(typeID: &str) -> String {
        todo!("port Vehicle::getVehicleClass")
    }

    /// C++: `double Vehicle::getSpeedFactor(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getSpeedFactor(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getChosenSpeedFactor();
    // }
    #[allow(unused_variables)]
    pub fn get_speed_factor(typeID: &str) -> f64 {
        todo!("port Vehicle::getSpeedFactor")
    }

    /// C++: `double Vehicle::getAccel(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getAccel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getMaxAccel();
    // }
    #[allow(unused_variables)]
    pub fn get_accel(typeID: &str) -> f64 {
        todo!("port Vehicle::getAccel")
    }

    /// C++: `double Vehicle::getDecel(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getMaxDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_decel(typeID: &str) -> f64 {
        todo!("port Vehicle::getDecel")
    }

    /// C++: `double Vehicle::getEmergencyDecel(const std::string &)` [static]
    /// @}
    // double Vehicle::getEmergencyDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getEmergencyDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_emergency_decel(typeID: &str) -> f64 {
        todo!("port Vehicle::getEmergencyDecel")
    }

    /// C++: `double Vehicle::getApparentDecel(const std::string &)` [static]
    /// @}
    // double Vehicle::getApparentDecel(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getApparentDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_apparent_decel(typeID: &str) -> f64 {
        todo!("port Vehicle::getApparentDecel")
    }

    /// C++: `double Vehicle::getImperfection(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getImperfection(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getImperfection();
    // }
    #[allow(unused_variables)]
    pub fn get_imperfection(typeID: &str) -> f64 {
        todo!("port Vehicle::getImperfection")
    }

    /// C++: `double Vehicle::getTau(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getTau(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getCarFollowModel().getHeadwayTime();
    // }
    #[allow(unused_variables)]
    pub fn get_tau(typeID: &str) -> f64 {
        todo!("port Vehicle::getTau")
    }

    /// C++: `std::string Vehicle::getEmissionClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getEmissionClass(const std::string& vehID) {
    //     return PollutantsInterface::getName(Helper::getVehicleType(vehID).getEmissionClass());
    // }
    #[allow(unused_variables)]
    pub fn get_emission_class(typeID: &str) -> String {
        todo!("port Vehicle::getEmissionClass")
    }

    /// C++: `std::string Vehicle::getShapeClass(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getShapeClass(const std::string& vehID) {
    //     return getVehicleShapeName(Helper::getVehicleType(vehID).getGuiShape());
    // }
    #[allow(unused_variables)]
    pub fn get_shape_class(typeID: &str) -> String {
        todo!("port Vehicle::getShapeClass")
    }

    /// C++: `double Vehicle::getMinGap(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMinGap(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(typeID: &str) -> f64 {
        todo!("port Vehicle::getMinGap")
    }

    /// C++: `double Vehicle::getWidth(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getWidth(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(typeID: &str) -> f64 {
        todo!("port Vehicle::getWidth")
    }

    /// C++: `double Vehicle::getHeight(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getHeight(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(typeID: &str) -> f64 {
        todo!("port Vehicle::getHeight")
    }

    /// C++: `double Vehicle::getMass(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMass(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMass();
    // }
    #[allow(unused_variables)]
    pub fn get_mass(typeID: &str) -> f64 {
        todo!("port Vehicle::getMass")
    }

    /// C++: `libsumo::TraCIColor Vehicle::getColor(const std::string &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(typeID: &str) -> TraCIColor {
        todo!("port Vehicle::getColor")
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
        todo!("port Vehicle::getMinGapLat")
    }

    /// C++: `double Vehicle::getMaxSpeedLat(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getMaxSpeedLat(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getMaxSpeedLat();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(typeID: &str) -> f64 {
        todo!("port Vehicle::getMaxSpeedLat")
    }

    /// C++: `std::string Vehicle::getLateralAlignment(const std::string &)` [static]
    /// @}
    // std::string
    // Vehicle::getLateralAlignment(const std::string& vehID) {
    //     return toString(Helper::getVehicleType(vehID).getPreferredLateralAlignment());
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_alignment(typeID: &str) -> String {
        todo!("port Vehicle::getLateralAlignment")
    }

    /// C++: `int Vehicle::getPersonCapacity(const std::string &)` [static]
    /// @}
    // int
    // Vehicle::getPersonCapacity(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getPersonCapacity();
    // }
    #[allow(unused_variables)]
    pub fn get_person_capacity(typeID: &str) -> i32 {
        todo!("port Vehicle::getPersonCapacity")
    }

    /// C++: `double Vehicle::getActionStepLength(const std::string &)` [static]
    /// @}
    // double Vehicle::getActionStepLength(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getActionStepLengthSecs();
    // }
    #[allow(unused_variables)]
    pub fn get_action_step_length(typeID: &str) -> f64 {
        todo!("port Vehicle::getActionStepLength")
    }

    /// C++: `double Vehicle::getSpeedDeviation(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getSpeedDeviation(const std::string& vehID) {
    //     return Helper::getVehicleType(vehID).getSpeedFactor().getParameter(1);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_deviation(typeID: &str) -> f64 {
        todo!("port Vehicle::getSpeedDeviation")
    }

    /// C++: `double Vehicle::getBoardingDuration(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getBoardingDuration(const std::string& vehID) {
    //     return STEPS2TIME(Helper::getVehicleType(vehID).getLoadingDuration(true));
    // }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(typeID: &str) -> f64 {
        todo!("port Vehicle::getBoardingDuration")
    }

    /// C++: `double Vehicle::getImpatience(const std::string &)` [static]
    /// @}
    // double
    // Vehicle::getImpatience(const std::string& vehID) {
    //     return Helper::getVehicle(vehID)->getImpatience();
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(typeID: &str) -> f64 {
        todo!("port Vehicle::getImpatience")
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
        todo!("port Vehicle::setStop")
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
        todo!("port Vehicle::replaceStop")
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
        todo!("port Vehicle::insertStop")
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
    //             pars.arrival = string2time(value);
    //             pars.parametersSet |= STOP_ARRIVAL_SET;
    //         } else if (param == toString(SUMO_ATTR_DURATION)) {
    //             pars.duration = string2time(value);
    //             pars.parametersSet |= STOP_DURATION_SET;
    //             // also update dynamic value
    //             stop.initPars(pars);
    //         } else if (param == toString(SUMO_ATTR_UNTIL)) {
    //             pars.until = string2time(value);
    //             pars.parametersSet |= STOP_UNTIL_SET;
    //         } else if (param == toString(SUMO_ATTR_EXTENSION)) {
    //             pars.extension = string2time(value);
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
    //             pars.started = string2time(value);
    //             pars.parametersSet |= STOP_STARTED_SET;
    //         } else if (param == toString(SUMO_ATTR_ENDED)) {
    //             pars.ended = string2time(value);
    //             pars.parametersSet |= STOP_ENDED_SET;
    //         } else if (param == toString(SUMO_ATTR_ONDEMAND)) {
    //             pars.onDemand = StringUtils::toBool(value);
    //             pars.parametersSet |= STOP_ONDEMAND_SET;
    //         } else if (param == toString(SUMO_ATTR_JUMP)) {
    //             pars.jump = string2time(value);
    //             pars.parametersSet |= STOP_JUMP_SET;
    //         } else if (param == toString(SUMO_ATTR_JUMP_UNTIL)) {
    //             pars.jumpUntil = string2time(value);
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
        todo!("port Vehicle::setStopParameter")
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
        todo!("port Vehicle::rerouteParkingArea")
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
        todo!("port Vehicle::resume")
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
        todo!("port Vehicle::add")
    }

    /// C++: `void Vehicle::changeTarget(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::changeTarget(const std::string& vehID, const std::string& edgeID) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     const MSEdge* destEdge = MSEdge::dictionary(edgeID);
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
    #[allow(unused_variables)]
    pub fn change_target(vehID: &str, edgeID: &str) {
        todo!("port Vehicle::changeTarget")
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
        todo!("port Vehicle::changeLane")
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
        todo!("port Vehicle::changeLaneRelative")
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
        todo!("port Vehicle::changeSublane")
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
        todo!("port Vehicle::slowDown")
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
    pub fn open_gap(vehID: &str, newTimeHeadway: f64, newSpaceHeadway: f64, duration: f64, changeRate: f64, maxDecel: f64, referenceVehID: &str) {
        todo!("port Vehicle::openGap")
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
        todo!("port Vehicle::deactivateGapControl")
    }

    /// C++: `void Vehicle::requestToC(const std::string &, double)` [static]
    // void
    // Vehicle::requestToC(const std::string& vehID, double leadTime) {
    //     setParameter(vehID, "device.toc.requestToC", toString(leadTime));
    // }
    #[allow(unused_variables)]
    pub fn request_to_c(vehID: &str, leadTime: f64) {
        todo!("port Vehicle::requestToC")
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
        todo!("port Vehicle::setSpeed")
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
        todo!("port Vehicle::setAcceleration")
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
    pub fn set_previous_speed(vehID: &str, prevSpeed: f64, prevAcceleration: f64) {
        todo!("port Vehicle::setPreviousSpeed")
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
        todo!("port Vehicle::setSpeedMode")
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
        todo!("port Vehicle::setLaneChangeMode")
    }

    /// C++: `void Vehicle::setRoutingMode(const std::string &, int)` [static]
    // void
    // Vehicle::setRoutingMode(const std::string& vehID, int routingMode) {
    //     Helper::getVehicle(vehID)->setRoutingMode(routingMode);
    // }
    #[allow(unused_variables)]
    pub fn set_routing_mode(vehID: &str, routingMode: i32) {
        todo!("port Vehicle::setRoutingMode")
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
        todo!("port Vehicle::setType")
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
    //         if (MSGlobals::gCheckRoutes) {
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
        todo!("port Vehicle::setRouteID")
    }

    /// C++: `void Vehicle::setRoute(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setRoute(const std::string& vehID, const std::string& edgeID) {
    //     setRoute(vehID, std::vector<std::string>({edgeID}));
    // }
    #[allow(unused_variables)]
    pub fn set_route(vehID: &str, edgeID: &str) {
        todo!("port Vehicle::setRoute")
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
        todo!("port Vehicle::setRoute")
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
        todo!("port Vehicle::setLateralLanePosition")
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
        todo!("port Vehicle::updateBestLanes")
    }

    /// C++: `void Vehicle::setAdaptedTraveltime(const std::string &, const std::string &, double, double, double)` [static]
    // void
    // Vehicle::setAdaptedTraveltime(const std::string& vehID, const std::string& edgeID,
    //                               double time, double begSeconds, double endSeconds) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSEdge* edge = MSEdge::dictionary(edgeID);
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
        todo!("port Vehicle::setAdaptedTraveltime")
    }

    /// C++: `void Vehicle::setEffort(const std::string &, const std::string &, double, double, double)` [static]
    // void
    // Vehicle::setEffort(const std::string& vehID, const std::string& edgeID,
    //                    double effort, double begSeconds, double endSeconds) {
    //     MSBaseVehicle* veh = Helper::getVehicle(vehID);
    //     MSEdge* edge = MSEdge::dictionary(edgeID);
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
        todo!("port Vehicle::setEffort")
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
        todo!("port Vehicle::rerouteTraveltime")
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
        todo!("port Vehicle::rerouteEffort")
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
        todo!("port Vehicle::setSignals")
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
    //     MSLane* l = MSLane::dictionary(laneID);
    //     if (l == nullptr) {
    //         throw TraCIException("Unknown lane '" + laneID + "'.");
    //     }
    //     if (veh->getLane() == l) {
    //         veh->setTentativeLaneAndPosition(l, pos, veh->getLateralPositionOnLane());
    //         return;
    //     }
    //     MSEdge* destinationEdge = &l->getEdge();
    //     const MSEdge* destinationRouteEdge = destinationEdge->getNormalBefore();
    //     if (!veh->isOnRoad() && veh->getParameter().wasSet(VEHPARS_FORCE_REROUTE) && veh->getRoute().getEdges().size() == 2) {
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
        todo!("port Vehicle::moveTo")
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
    pub fn move_to_xy(vehID: &str, edgeID: &str, laneIndex: i32, x: f64, y: f64, angle: f64, keepRoute: i32, matchThreshold: f64) {
        todo!("port Vehicle::moveToXY")
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
    pub fn remove(vehID: &str, reason: i8) {
        todo!("port Vehicle::remove")
    }

    /// C++: `void Vehicle::setLine(const std::string &, const std::string &)` [static]
    // void
    // Vehicle::setLine(const std::string& vehID, const std::string& line) {
    //     Helper::getVehicle(vehID)->getParameter().line = line;
    // }
    #[allow(unused_variables)]
    pub fn set_line(vehID: &str, line: &str) {
        todo!("port Vehicle::setLine")
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
        todo!("port Vehicle::setVia")
    }

    /// C++: `void Vehicle::highlight(const std::string &, const libsumo::TraCIColor &, double, const int, const double, const int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn highlight(vehID: &str, col: &TraCIColor, size: f64, alphaMax: i32, duration: f64, type_: i32) {
        todo!("port Vehicle::highlight")
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
        todo!("port Vehicle::dispatchTaxi")
    }

    /// C++: `void Vehicle::setLength(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setLength(const std::string& vehID, double length) {
    //     Helper::getVehicle(vehID)->getSingularType().setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(typeID: &str, length: f64) {
        todo!("port Vehicle::setLength")
    }

    /// C++: `void Vehicle::setMaxSpeed(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMaxSpeed(const std::string& vehID, double speed) {
    //     Helper::getVehicle(vehID)->getSingularType().setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(typeID: &str, speed: f64) {
        todo!("port Vehicle::setMaxSpeed")
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
    pub fn set_vehicle_class(typeID: &str, clazz: &str) {
        todo!("port Vehicle::setVehicleClass")
    }

    /// C++: `void Vehicle::setSpeedFactor(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setSpeedFactor(const std::string& vehID, double factor) {
    //     Helper::getVehicle(vehID)->setChosenSpeedFactor(factor);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor(typeID: &str, factor: f64) {
        todo!("port Vehicle::setSpeedFactor")
    }

    /// C++: `void Vehicle::setAccel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setAccel(const std::string& vehID, double accel) {
    //     Helper::getVehicle(vehID)->getSingularType().setAccel(accel);
    // }
    #[allow(unused_variables)]
    pub fn set_accel(typeID: &str, accel: f64) {
        todo!("port Vehicle::setAccel")
    }

    /// C++: `void Vehicle::setDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setDecel(const std::string& vehID, double decel) {
    //     VehicleType::setDecel(Helper::getVehicle(vehID)->getSingularType().getID(), decel);
    // }
    #[allow(unused_variables)]
    pub fn set_decel(typeID: &str, decel: f64) {
        todo!("port Vehicle::setDecel")
    }

    /// C++: `void Vehicle::setEmergencyDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setEmergencyDecel(const std::string& vehID, double decel) {
    //     VehicleType::setEmergencyDecel(Helper::getVehicle(vehID)->getSingularType().getID(), decel);
    // }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(typeID: &str, decel: f64) {
        todo!("port Vehicle::setEmergencyDecel")
    }

    /// C++: `void Vehicle::setApparentDecel(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setApparentDecel(const std::string& vehID, double decel) {
    //     Helper::getVehicle(vehID)->getSingularType().setApparentDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(typeID: &str, decel: f64) {
        todo!("port Vehicle::setApparentDecel")
    }

    /// C++: `void Vehicle::setImperfection(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setImperfection(const std::string& vehID, double imperfection) {
    //     Helper::getVehicle(vehID)->getSingularType().setImperfection(imperfection);
    // }
    #[allow(unused_variables)]
    pub fn set_imperfection(typeID: &str, imperfection: f64) {
        todo!("port Vehicle::setImperfection")
    }

    /// C++: `void Vehicle::setTau(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setTau(const std::string& vehID, double tau) {
    //     Helper::getVehicle(vehID)->getSingularType().setTau(tau);
    // }
    #[allow(unused_variables)]
    pub fn set_tau(typeID: &str, tau: f64) {
        todo!("port Vehicle::setTau")
    }

    /// C++: `void Vehicle::setEmissionClass(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setEmissionClass(const std::string& vehID, const std::string& clazz) {
    //     Helper::getVehicle(vehID)->getSingularType().setEmissionClass(PollutantsInterface::getClassByName(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_emission_class(typeID: &str, clazz: &str) {
        todo!("port Vehicle::setEmissionClass")
    }

    /// C++: `void Vehicle::setShapeClass(const std::string &, const std::string &)` [static]
    /// @}
    // void
    // Vehicle::setShapeClass(const std::string& vehID, const std::string& clazz) {
    //     Helper::getVehicle(vehID)->getSingularType().setShape(getVehicleShapeID(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_shape_class(typeID: &str, shapeClass: &str) {
        todo!("port Vehicle::setShapeClass")
    }

    /// C++: `void Vehicle::setWidth(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setWidth(const std::string& vehID, double width) {
    //     Helper::getVehicle(vehID)->getSingularType().setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(typeID: &str, width: f64) {
        todo!("port Vehicle::setWidth")
    }

    /// C++: `void Vehicle::setHeight(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setHeight(const std::string& vehID, double height) {
    //     Helper::getVehicle(vehID)->getSingularType().setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(typeID: &str, height: f64) {
        todo!("port Vehicle::setHeight")
    }

    /// C++: `void Vehicle::setMass(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMass(const std::string& vehID, double mass) {
    //     Helper::getVehicle(vehID)->getSingularType().setMass(mass);
    // }
    #[allow(unused_variables)]
    pub fn set_mass(typeID: &str, mass: f64) {
        todo!("port Vehicle::setMass")
    }

    /// C++: `void Vehicle::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(typeID: &str, color: &TraCIColor) {
        todo!("port Vehicle::setColor")
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
    pub fn set_min_gap(typeID: &str, minGap: f64) {
        todo!("port Vehicle::setMinGap")
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
    pub fn set_min_gap_lat(typeID: &str, minGapLat: f64) {
        todo!("port Vehicle::setMinGapLat")
    }

    /// C++: `void Vehicle::setMaxSpeedLat(const std::string &, double)` [static]
    /// @}
    // void
    // Vehicle::setMaxSpeedLat(const std::string& vehID, double speed) {
    //     Helper::getVehicle(vehID)->getSingularType().setMaxSpeedLat(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed_lat(typeID: &str, speed: f64) {
        todo!("port Vehicle::setMaxSpeedLat")
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
    pub fn set_lateral_alignment(typeID: &str, latAlignment: &str) {
        todo!("port Vehicle::setLateralAlignment")
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
    pub fn set_action_step_length(typeID: &str, actionStepLength: f64, resetActionOffset: bool) {
        todo!("port Vehicle::setActionStepLength")
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
    pub fn set_boarding_duration(typeID: &str, boardingDuration: f64) {
        todo!("port Vehicle::setBoardingDuration")
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
    pub fn set_impatience(typeID: &str, impatience: f64) {
        todo!("port Vehicle::setImpatience")
    }

    /// C++: `void Vehicle::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Vehicle::subscribe")
    }

    /// C++: `void Vehicle::unsubscribe(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Vehicle::unsubscribe")
    }

    /// C++: `void Vehicle::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    /// @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Vehicle::subscribeContext")
    }

    /// C++: `void Vehicle::unsubscribeContext(const std::string &, int, double)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Vehicle::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Vehicle::getAllSubscriptionResults()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Vehicle::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Vehicle::getSubscriptionResults(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Vehicle::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Vehicle::getAllContextSubscriptionResults()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Vehicle::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Vehicle::getContextSubscriptionResults(const std::string &)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Vehicle::getContextSubscriptionResults")
    }

    /// C++: `void Vehicle::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Vehicle, VEHICLE)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Vehicle::subscribeParameterWithKey")
    }

    /// C++: `int Vehicle::domainID()` [static]
    /// @}
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Vehicle::domainID")
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
        todo!("port Vehicle::subscribeLeader")
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
        todo!("port Vehicle::addSubscriptionFilterLanes")
    }

    /// C++: `void Vehicle::addSubscriptionFilterNoOpposite()` [static]
    // void
    // Vehicle::addSubscriptionFilterNoOpposite() {
    //     Helper::addSubscriptionFilter(SUBS_FILTER_NOOPPOSITE);
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_no_opposite() {
        todo!("port Vehicle::addSubscriptionFilterNoOpposite")
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
        todo!("port Vehicle::addSubscriptionFilterDownstreamDistance")
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
        todo!("port Vehicle::addSubscriptionFilterUpstreamDistance")
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
        todo!("port Vehicle::addSubscriptionFilterCFManeuver")
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
        todo!("port Vehicle::addSubscriptionFilterLCManeuver")
    }

    /// C++: `void Vehicle::addSubscriptionFilterLeadFollow(const std::vector<int> &)` [static]
    // void
    // Vehicle::addSubscriptionFilterLeadFollow(const std::vector<int>& lanes) {
    //     Helper::addSubscriptionFilter(SUBS_FILTER_LEAD_FOLLOW);
    //     addSubscriptionFilterLanes(lanes);
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lead_follow(lanes: &Vec<i32>) {
        todo!("port Vehicle::addSubscriptionFilterLeadFollow")
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
        todo!("port Vehicle::addSubscriptionFilterTurn")
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
        todo!("port Vehicle::addSubscriptionFilterVClass")
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
        todo!("port Vehicle::addSubscriptionFilterVType")
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
        todo!("port Vehicle::addSubscriptionFilterFieldOfVision")
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
        todo!("port Vehicle::addSubscriptionFilterLateralDistance")
    }

    /// C++: `void Vehicle::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Vehicle::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(Helper::getVehicle(id)->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        todo!("port Vehicle::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Vehicle::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Vehicle::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Vehicle::makeWrapper")
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
        todo!("port Vehicle::handleVariable")
    }

    /// C++: `bool Vehicle::isVisible(const SUMOVehicle *)` [static, private]
    // bool
    // Vehicle::isVisible(const SUMOVehicle* veh) {
    //     return veh->isOnRoad() || veh->isParking() || veh->wasRemoteControlled();
    // }
    #[allow(unused_variables)]
    pub fn is_visible(veh: Ptr<SUMOVehicle>) -> bool {
        todo!("port Vehicle::isVisible")
    }

    /// C++: `bool Vehicle::isOnInit(const std::string &)` [static, private]
    // bool
    // Vehicle::isOnInit(const std::string& vehID) {
    //     SUMOVehicle* sumoVehicle = MSNet::getInstance()->getVehicleControl().getVehicle(vehID);
    //     return sumoVehicle == nullptr || sumoVehicle->getLane() == nullptr;
    // }
    #[allow(unused_variables)]
    pub fn is_on_init(vehID: &str) -> bool {
        todo!("port Vehicle::isOnInit")
    }

    /// C++: `void Vehicle::Vehicle()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Vehicle::Vehicle")
    }

}

