//! Port of libsumo/Person.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Every method is a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
// shortcut: C++ `throw TraCIException(msg)` is ported as panic! with the same message; panics marked
// `shortcut:` are bodies whose unported dependency (taxi dispatch, stage conversion, pedestrian router,
// sidewalk template, SUMOVehicleParameter mutation) is not in the port yet

/// C++ class `libsumo::Person` (libsumo/Person.h)
pub struct Person {
}

// C++ static members of Person (port as module-level statics, see AGENTS.md):
//   static const int Person::DOMAIN_ID;
//   static SubscriptionResults Person::mySubscriptionResults;
//   static ContextSubscriptionResults Person::myContextSubscriptionResults;

thread_local! {
    /// C++ static `Person::mySubscriptionResults`
    static MY_SUBSCRIPTION_RESULTS: std::cell::RefCell<SubscriptionResults> = std::cell::RefCell::new(SubscriptionResults::new());
    /// C++ static `Person::myContextSubscriptionResults`
    static MY_CONTEXT_SUBSCRIPTION_RESULTS: std::cell::RefCell<ContextSubscriptionResults> = std::cell::RefCell::new(ContextSubscriptionResults::new());
}

impl Person {
    /// C++: `double Person::getSpeed(const std::string &)` [static]
    // double
    // Person::getSpeed(const std::string& personID) {
    //     return getPerson(personID)->getSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(personID: &str) -> f64 {
        Self::get_person(personID).get_speed()
    }

    /// C++: `libsumo::TraCIPosition Person::getPosition(const std::string &, const bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position(personID: &str, includeZ: bool) -> TraCIPosition {
        Helper::make_tra_ci_position(&Self::get_person(personID).get_position_2(), includeZ)
    }

    /// C++: `libsumo::TraCIPosition Person::getPosition3D(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_position3_d(personID: &str) -> TraCIPosition {
        Helper::make_tra_ci_position(&Self::get_person(personID).get_position_2(), true)
    }

    /// C++: `std::string Person::getRoadID(const std::string &)` [static]
    // std::string
    // Person::getRoadID(const std::string& personID) {
    //     return getPerson(personID)->getEdge()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_road_id(personID: &str) -> String {
        Self::get_person(personID).get_edge().get_id().to_string()
    }

    /// C++: `std::string Person::getLaneID(const std::string &)` [static]
    // std::string
    // Person::getLaneID(const std::string& personID) {
    //     return Named::getIDSecure(getPerson(personID)->getLane(), "");
    // }
    #[allow(unused_variables)]
    pub fn get_lane_id(personID: &str) -> String {
        let lane = Self::get_person(personID).get_lane();
        if lane.is_null() { String::new() } else { lane.get_id().to_string() }
    }

    /// C++: `std::string Person::getTypeID(const std::string &)` [static]
    // std::string
    // Person::getTypeID(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getID();
    // }
    #[allow(unused_variables)]
    pub fn get_type_id(personID: &str) -> String {
        Self::get_person(personID).get_vehicle_type().get_id().to_string()
    }

    /// C++: `double Person::getWaitingTime(const std::string &)` [static]
    // double
    // Person::getWaitingTime(const std::string& personID) {
    //     return getPerson(personID)->getWaitingSeconds();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(personID: &str) -> f64 {
        Self::get_person(personID).get_waiting_seconds()
    }

    /// C++: `double Person::getArrivalPos(const std::string &)` [static]
    // double
    // Person::getArrivalPos(const std::string& personID) {
    //     return getPerson(personID)->getArrivalPos();
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(personID: &str) -> f64 {
        Self::get_person(personID).get_arrival_pos()
    }

    /// C++: `std::string Person::getNextEdge(const std::string &)` [static]
    // std::string
    // Person::getNextEdge(const std::string& personID) {
    //     return getPerson(personID)->getNextEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(personID: &str) -> String {
        Self::get_person(personID).get_next_edge().to_string()
    }

    /// C++: `std::string Person::getVehicle(const std::string &)` [static]
    // std::string
    // Person::getVehicle(const std::string& personID) {
    //     const SUMOVehicle* veh = getPerson(personID)->getVehicle();
    //     if (veh == nullptr) {
    //         return "";
    //     } else {
    //         return veh->getID();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle(personID: &str) -> String {
        let veh = Self::get_person(personID).get_vehicle();
        if veh.is_null() { String::new() } else { veh.get_id().to_string() }
    }

    /// C++: `int Person::getRemainingStages(const std::string &)` [static]
    // int
    // Person::getRemainingStages(const std::string& personID) {
    //     return getPerson(personID)->getNumRemainingStages();
    // }
    #[allow(unused_variables)]
    pub fn get_remaining_stages(personID: &str) -> i32 {
        Self::get_person(personID).get_num_remaining_stages()
    }

    /// C++: `libsumo::TraCIStage Person::getStage(const std::string &, int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stage(personID: &str, nextStageIndex: i32) -> TraCIStage {
        panic!("shortcut: Person::getStage needs convertTraCIStage-style TraCIStage construction from MSStage (not ported)")
    }

    /// C++: `std::vector<std::string> Person::getEdges(const std::string &, int)` [static]
    // std::vector<std::string>
    // Person::getEdges(const std::string& personID, int nextStageIndex) {
    //     MSTransportable* p = getPerson(personID);
    //     if (nextStageIndex >= p->getNumRemainingStages()) {
    //         throw TraCIException("The stage index must be lower than the number of remaining stages.");
    //     }
    //     if (nextStageIndex < (p->getNumRemainingStages() - p->getNumStages())) {
    //         throw TraCIException("The negative stage index must refer to a valid previous stage.");
    //     }
    //     std::vector<std::string> edgeIDs;
    //     for (auto& e : p->getNextStage(nextStageIndex)->getEdges()) {
    //         if (e != nullptr) {
    //             edgeIDs.push_back(e->getID());
    //         }
    //     }
    //     return edgeIDs;
    // }
    #[allow(unused_variables)]
    pub fn get_edges(personID: &str, nextStageIndex: i32) -> Vec<String> {
        let p = Self::get_person(personID);
        if nextStageIndex >= p.get_num_remaining_stages() {
            panic!("The stage index must be lower than the number of remaining stages.");
        }
        if nextStageIndex < (p.get_num_remaining_stages() - p.get_num_stages()) {
            panic!("The negative stage index must refer to a valid previous stage.");
        }
        let mut edgeIDs: Vec<String> = Vec::new();
        for e in p.get_next_stage(nextStageIndex).get_edges() {
            if !e.is_null() {
                edgeIDs.push(e.get_id().to_string());
            }
        }
        edgeIDs
    }

    /// C++: `double Person::getAngle(const std::string &)` [static]
    // double
    // Person::getAngle(const std::string& personID) {
    //     return GeomHelper::naviDegree(getPerson(personID)->getAngle());
    // }
    #[allow(unused_variables)]
    pub fn get_angle(personID: &str) -> f64 {
        utils::geom::geom_helper::navi_degree(Self::get_person(personID).get_angle())
    }

    /// C++: `double Person::getSlope(const std::string &)` [static]
    // double
    // Person::getSlope(const std::string& personID) {
    //     MSPerson* person = getPerson(personID);
    //     const double ep = person->getEdgePos();
    //     const MSLane* lane = getSidewalk<MSEdge, MSLane>(person->getEdge());
    //     if (lane == nullptr) {
    //         lane = person->getEdge()->getLanes()[0];
    //     }
    //     const double gp = lane->interpolateLanePosToGeometryPos(ep);
    //     return lane->getShape().slopeDegreeAtOffset(gp);
    // }
    #[allow(unused_variables)]
    pub fn get_slope(personID: &str) -> f64 {
        panic!("shortcut: Person::getSlope needs the getSidewalk<MSEdge, MSLane> template (not ported)")
    }

    /// C++: `double Person::getLanePosition(const std::string &)` [static]
    // double
    // Person::getLanePosition(const std::string& personID) {
    //     return getPerson(personID)->getEdgePos();
    // }
    #[allow(unused_variables)]
    pub fn get_lane_position(personID: &str) -> f64 {
        Self::get_person(personID).get_edge_pos()
    }

    /// C++: `double Person::getWalkingDistance(const std::string &, const std::string &, double, int)` [static]
    // double
    // Person::getWalkingDistance(const std::string& personID, const std::string& edgeID, double pos, int laneIndex) {
    //     MSPerson* p = getPerson(personID);
    //     if (p->getCurrentStageType() == MSStageType::WALKING) {
    //         const MSStageWalking* walk = dynamic_cast<const MSStageWalking*>(p->getCurrentStage());
    //         ConstMSEdgeVector edges = walk->getEdges();
    //         edges.erase(edges.begin(), edges.begin() + walk->getRoutePosition());
    //         const MSLane* lane = Helper::getLaneChecking(edgeID, laneIndex, pos);
    //         auto it = std::find(edges.begin(), edges.end(), &lane->getEdge());
    //         if (it == edges.end()) {
    //             // Vehicle would return INVALID_DOUBLE_VALUE;
    //             throw TraCIException(TLF("Edge '%' does not occur within the remaining walk of person '%'.", edgeID, personID));
    //
    //         }
    //         edges.erase(it + 1, edges.end());
    //         double distance = 0;
    //         MSPedestrianRouter& router = MSNet::getInstance()->getPedestrianRouter(0);
    //         router.recomputeWalkCosts(edges, p->getMaxSpeed(), p->getEdgePos(), pos, SIMSTEP, p->getVTypeParameter(), distance);
    //         if (distance == std::numeric_limits<double>::max()) {
    //             return INVALID_DOUBLE_VALUE;
    //         }
    //         return distance;
    //     } else {
    //         // Vehicle would return INVALID_DOUBLE_VALUE;
    //         throw TraCIException(TLF("Person '%' is not walking", personID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_walking_distance(personID: &str, edgeID: &str, pos: f64, laneIndex: i32) -> f64 {
        panic!("shortcut: Person::getWalkingDistance needs MSStageWalking casts and MSPedestrianRouter::recomputeWalkCosts (not ported)")
    }

    /// C++: `double Person::getWalkingDistance2D(const std::string &, double, double)` [static]
    // double
    // Person::getWalkingDistance2D(const std::string& personID, double x, double y) {
    //     MSPerson* p = getPerson(personID);
    //     std::pair<MSLane*, double> roadPos = Helper::convertCartesianToRoadMap(Position(x, y), p->getVehicleType().getVehicleClass());
    //     return getWalkingDistance(personID, roadPos.first->getEdge().getID(), roadPos.second, roadPos.first->getIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_walking_distance2_d(personID: &str, x: f64, y: f64) -> f64 {
        panic!("shortcut: Person::getWalkingDistance2D depends on getWalkingDistance (not ported)")
    }

    /// C++: `std::vector<libsumo::TraCIReservation> Person::getTaxiReservations(int)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_taxi_reservations(onlyNew: i32) -> Vec<TraCIReservation> {
        panic!("shortcut: Person::getTaxiReservations needs MSDispatch and Reservation (taxi dispatch not ported)")
    }

    /// C++: `std::string Person::splitTaxiReservation(std::string, const std::vector<std::string> &)` [static]
    // std::string
    // Person::splitTaxiReservation(std::string reservationID, const std::vector<std::string>& personIDs) {
    //     MSDispatch* dispatcher = MSDevice_Taxi::getDispatchAlgorithm();
    //     if (dispatcher != nullptr) {
    //         MSDispatch_TraCI* traciDispatcher = dynamic_cast<MSDispatch_TraCI*>(dispatcher);
    //         if (traciDispatcher != nullptr) {
    //             return traciDispatcher->splitReservation(reservationID, personIDs);
    //         }
    //     }
    //     throw TraCIException("device.taxi.dispatch-algorithm 'traci' has not been loaded");
    // }
    #[allow(unused_variables)]
    pub fn split_taxi_reservation(reservationID: String, personIDs: &Vec<String>) -> String {
        panic!("shortcut: Person::splitTaxiReservation needs MSDispatch_TraCI (taxi dispatch not ported)")
    }

    /// C++: `std::vector<std::string> Person::getIDList()` [static]
    // std::vector<std::string>
    // Person::getIDList() {
    //     MSTransportableControl& c = MSNet::getInstance()->getPersonControl();
    //     std::vector<std::string> ids;
    //     for (MSTransportableControl::constVehIt i = c.loadedBegin(); i != c.loadedEnd(); ++i) {
    //         if (i->second->getCurrentStageType() != MSStageType::WAITING_FOR_DEPART) {
    //             ids.push_back(i->first);
    //         }
    //     }
    //     return ids;
    // }
    #[allow(unused_variables)]
    pub fn get_id_list() -> Vec<String> {
        panic!("shortcut: Person::getIDList iterates MSTransportableControl loaded persons, whose iterator type is still Unmapped")
    }

    /// C++: `int Person::getIDCount()` [static]
    // int
    // Person::getIDCount() {
    //     return MSNet::getInstance()->getPersonControl().size();
    // }
    #[allow(unused_variables)]
    pub fn get_id_count() -> i32 {
        MSNet::get_instance().m().get_person_control().size()
    }

    /// C++: `std::string Person::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Person::getParameter(const std::string& personID, const std::string& param) {
    //     return getPerson(personID)->getParameter().getParameter(param, "");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        Self::get_person(objectID).get_parameter().get_parameter(key, "")
    }

    /// C++: `const std::pair<std::string, std::string> Person::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Person)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        (key.to_string(), Self::get_parameter(objectID, key))
    }

    /// C++: `void Person::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Person::setParameter(const std::string& personID, const std::string& key, const std::string& value) {
    //     MSTransportable* p = getPerson(personID);
    //     if (StringUtils::startsWith(key, "device.")) {
    //         throw TraCIException("Person '" + personID + "' does not support device parameters\n");
    //     } else if (StringUtils::startsWith(key, "laneChangeModel.")) {
    //         throw TraCIException("Person '" + personID + "' does not support laneChangeModel parameters\n");
    //     } else if (StringUtils::startsWith(key, "carFollowModel.")) {
    //         throw TraCIException("Person '" + personID + "' does not support carFollowModel parameters\n");
    //     } else if (StringUtils::startsWith(key, "junctionModel.")) {
    //         try {
    //             // use the whole key (including junctionModel prefix)
    //             p->setJunctionModelParameter(key, value);
    //         } catch (InvalidArgument& e) {
    //             // error message includes id since it is also used for xml input
    //             throw TraCIException(e.what());
    //         }
    //     } else if (StringUtils::startsWith(key, "has.") && StringUtils::endsWith(key, ".device")) {
    //         throw TraCIException("Person '" + personID + "' does not support chanigng device status\n");
    //     } else {
    //         ((SUMOVehicleParameter&)p->getParameter()).setParameter(key, value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        let p = Self::get_person(objectID);
        if key.starts_with("device.") {
            panic!("Person '{}' does not support device parameters\n", objectID);
        } else if key.starts_with("laneChangeModel.") {
            panic!("Person '{}' does not support laneChangeModel parameters\n", objectID);
        } else if key.starts_with("carFollowModel.") {
            panic!("Person '{}' does not support carFollowModel parameters\n", objectID);
        } else if key.starts_with("junctionModel.") {
            // use the whole key (including junctionModel prefix); InvalidArgument panics inside
            p.m().set_junction_model_parameter(key, value);
        } else if key.starts_with("has.") && key.ends_with(".device") {
            panic!("Person '{}' does not support chanigng device status\n", objectID);
        } else {
            panic!("shortcut: Person::setParameter writes SUMOVehicleParameter through a const path, needs mutable parameter access");
        }
    }

    /// C++: `double Person::getLength(const std::string &)` [static]
    // double
    // Person::getLength(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_length()
    }

    /// C++: `double Person::getMaxSpeed(const std::string &)` [static]
    // double
    // Person::getMaxSpeed(const std::string& personID) {
    //     return getPerson(personID)->getMaxSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(typeID: &str) -> f64 {
        Self::get_person(typeID).get_max_speed()
    }

    /// C++: `std::string Person::getVehicleClass(const std::string &)` [static]
    // std::string
    // Person::getVehicleClass(const std::string& personID) {
    //     return toString(getPerson(personID)->getVehicleType().getVehicleClass());
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_class(typeID: &str) -> String {
        panic!("shortcut: Person::getVehicleClass needs toString(SUMOVehicleClass) (not ported)")
    }

    /// C++: `double Person::getSpeedFactor(const std::string &)` [static]
    // double
    // Person::getSpeedFactor(const std::string& personID) {
    //     return getPerson(personID)->getChosenSpeedFactor();
    // }
    #[allow(unused_variables)]
    pub fn get_speed_factor(typeID: &str) -> f64 {
        Self::get_person(typeID).get_chosen_speed_factor()
    }

    /// C++: `double Person::getAccel(const std::string &)` [static]
    // double
    // Person::getAccel(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getMaxAccel();
    // }
    #[allow(unused_variables)]
    pub fn get_accel(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_max_accel()
    }

    /// C++: `double Person::getDecel(const std::string &)` [static]
    // double
    // Person::getDecel(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getMaxDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_decel(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_max_decel()
    }

    /// C++: `double Person::getEmergencyDecel(const std::string &)` [static]
    // double Person::getEmergencyDecel(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getEmergencyDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_emergency_decel(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_emergency_decel()
    }

    /// C++: `double Person::getApparentDecel(const std::string &)` [static]
    // double Person::getApparentDecel(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getApparentDecel();
    // }
    #[allow(unused_variables)]
    pub fn get_apparent_decel(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_apparent_decel()
    }

    /// C++: `double Person::getImperfection(const std::string &)` [static]
    // double
    // Person::getImperfection(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getImperfection();
    // }
    #[allow(unused_variables)]
    pub fn get_imperfection(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_imperfection()
    }

    /// C++: `double Person::getTau(const std::string &)` [static]
    // double
    // Person::getTau(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getCarFollowModel().getHeadwayTime();
    // }
    #[allow(unused_variables)]
    pub fn get_tau(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_car_follow_model().get_headway_time()
    }

    /// C++: `std::string Person::getEmissionClass(const std::string &)` [static]
    // std::string
    // Person::getEmissionClass(const std::string& personID) {
    //     return PollutantsInterface::getName(getPerson(personID)->getVehicleType().getEmissionClass());
    // }
    #[allow(unused_variables)]
    pub fn get_emission_class(typeID: &str) -> String {
        PollutantsInterface::get_name(Self::get_person(typeID).get_vehicle_type().get_emission_class())
    }

    /// C++: `std::string Person::getShapeClass(const std::string &)` [static]
    // std::string
    // Person::getShapeClass(const std::string& personID) {
    //     return getVehicleShapeName(getPerson(personID)->getVehicleType().getGuiShape());
    // }
    #[allow(unused_variables)]
    pub fn get_shape_class(typeID: &str) -> String {
        get_vehicle_shape_name(Self::get_person(typeID).get_vehicle_type().get_gui_shape())
    }

    /// C++: `double Person::getMinGap(const std::string &)` [static]
    // double
    // Person::getMinGap(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getMinGap();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_min_gap()
    }

    /// C++: `double Person::getWidth(const std::string &)` [static]
    // double
    // Person::getWidth(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getWidth();
    // }
    #[allow(unused_variables)]
    pub fn get_width(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_width()
    }

    /// C++: `double Person::getHeight(const std::string &)` [static]
    // double
    // Person::getHeight(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getHeight();
    // }
    #[allow(unused_variables)]
    pub fn get_height(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_height()
    }

    /// C++: `double Person::getMass(const std::string &)` [static]
    // double
    // Person::getMass(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getMass();
    // }
    #[allow(unused_variables)]
    pub fn get_mass(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_mass()
    }

    /// C++: `libsumo::TraCIColor Person::getColor(const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_color(typeID: &str) -> TraCIColor {
        Helper::make_tra_ci_color(&Self::get_person(typeID).get_parameter().color)
    }

    /// C++: `double Person::getMinGapLat(const std::string &)` [static]
    // double
    // Person::getMinGapLat(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getMinGapLat();
    // }
    #[allow(unused_variables)]
    pub fn get_min_gap_lat(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_min_gap_lat()
    }

    /// C++: `double Person::getMaxSpeedLat(const std::string &)` [static]
    // double
    // Person::getMaxSpeedLat(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getMaxSpeedLat();
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_max_speed_lat()
    }

    /// C++: `std::string Person::getLateralAlignment(const std::string &)` [static]
    // std::string
    // Person::getLateralAlignment(const std::string& personID) {
    //     return toString(getPerson(personID)->getVehicleType().getPreferredLateralAlignment());
    // }
    #[allow(unused_variables)]
    pub fn get_lateral_alignment(typeID: &str) -> String {
        panic!("shortcut: Person::getLateralAlignment needs toString(LatAlignmentDefinition) (not ported)")
    }

    /// C++: `int Person::getPersonCapacity(const std::string &)` [static]
    // int
    // Person::getPersonCapacity(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getPersonCapacity();
    // }
    #[allow(unused_variables)]
    pub fn get_person_capacity(typeID: &str) -> i32 {
        Self::get_person(typeID).get_vehicle_type().get_person_capacity()
    }

    /// C++: `double Person::getActionStepLength(const std::string &)` [static]
    // double Person::getActionStepLength(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getActionStepLengthSecs();
    // }
    #[allow(unused_variables)]
    pub fn get_action_step_length(typeID: &str) -> f64 {
        Self::get_person(typeID).get_vehicle_type().get_action_step_length_secs()
    }

    /// C++: `double Person::getSpeedDeviation(const std::string &)` [static]
    // double
    // Person::getSpeedDeviation(const std::string& personID) {
    //     return getPerson(personID)->getVehicleType().getSpeedFactor().getParameter(1);
    // }
    #[allow(unused_variables)]
    pub fn get_speed_deviation(typeID: &str) -> f64 {
        panic!("shortcut: Person::getSpeedDeviation needs the speed factor distribution (Unmapped in MSVehicleType)")
    }

    /// C++: `double Person::getBoardingDuration(const std::string &)` [static]
    // double
    // Person::getBoardingDuration(const std::string& personID) {
    //     return STEPS2TIME(getPerson(personID)->getVehicleType().getBoardingDuration(true));
    // }
    #[allow(unused_variables)]
    pub fn get_boarding_duration(typeID: &str) -> f64 {
        steps2time(Self::get_person(typeID).get_vehicle_type().get_boarding_duration(true))
    }

    /// C++: `double Person::getImpatience(const std::string &)` [static]
    // double
    // Person::getImpatience(const std::string& personID) {
    //     return getPerson(personID)->getImpatience();
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(typeID: &str) -> f64 {
        Self::get_person(typeID).get_impatience()
    }

    /// C++: `void Person::add(const std::string &, const std::string &, double, double, const std::string)` [static]
    // void
    // Person::add(const std::string& personID, const std::string& edgeID, double pos, double departInSecs, const std::string typeID) {
    //     MSTransportable* p;
    //     try {
    //         p = getPerson(personID);
    //     } catch (TraCIException&) {
    //         p = nullptr;
    //     }
    //
    //     if (p != nullptr) {
    //         throw TraCIException("The person " + personID + " to add already exists.");
    //     }
    //
    //     SUMOTime depart = TIME2STEPS(departInSecs);
    //     SUMOVehicleParameter vehicleParams;
    //     vehicleParams.id = personID;
    //
    //     MSVehicleType* vehicleType = MSNet::getInstance()->getVehicleControl().getVType(typeID);
    //     if (!vehicleType) {
    //         throw TraCIException("Invalid type '" + typeID + "' for person '" + personID + "'");
    //     }
    //
    //     const MSEdge* edge = MSEdge::dictionary(edgeID);
    //     if (!edge) {
    //         throw TraCIException("Invalid edge '" + edgeID + "' for person: '" + personID + "'");
    //     }
    //
    //     if (departInSecs < 0.) {
    //         const int proc = (int) - departInSecs;
    //         if (proc >= static_cast<int>(DepartDefinition::DEF_MAX)) {
    //             throw TraCIException("Invalid departure time." + toString(depart) + " " + toString(proc));
    //         }
    //         vehicleParams.departProcedure = (DepartDefinition)proc;
    //         vehicleParams.depart = MSNet::getInstance()->getCurrentTimeStep();
    //     } else if (depart < MSNet::getInstance()->getCurrentTimeStep()) {
    //         vehicleParams.depart = MSNet::getInstance()->getCurrentTimeStep();
    //         WRITE_WARNINGF(TL("Departure time=% for person '%' is in the past; using current time=% instead."),
    //                        toString(departInSecs), personID, time2string(vehicleParams.depart));
    //     } else {
    //         vehicleParams.depart = depart;
    //     }
    //
    //     vehicleParams.departPosProcedure = DepartPosDefinition::GIVEN;
    //     if (fabs(pos) > edge->getLength()) {
    //         throw TraCIException("Invalid departure position.");
    //     }
    //     if (pos < 0) {
    //         pos += edge->getLength();
    //     }
    //     vehicleParams.departPos = pos;
    //
    //     SUMOVehicleParameter* params = new SUMOVehicleParameter(vehicleParams);
    //     MSTransportable::MSTransportablePlan* plan = new MSTransportable::MSTransportablePlan();
    //     plan->push_back(new MSStageWaiting(edge, nullptr, 0, depart, pos, "awaiting departure", true));
    //
    //     try {
    //         MSTransportable* person = MSNet::getInstance()->getPersonControl().buildPerson(params, vehicleType, plan, nullptr);
    //         MSNet::getInstance()->getPersonControl().add(person);
    //     } catch (ProcessError& e) {
    //         delete params;
    //         delete plan;
    //         throw TraCIException(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add(personID: &str, edgeID: &str, pos: f64, depart: f64, typeID: String) {
        panic!("shortcut: Person::add needs SUMOVehicleParameter construction, MSTransportablePlan and buildPerson (not ported)")
    }

    /// C++: `void Person::appendStage(const std::string &, const libsumo::TraCIStage &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn append_stage(personID: &str, stage: &TraCIStage) {
        panic!("shortcut: Person::appendStage needs convertTraCIStage (not ported)")
    }

    /// C++: `void Person::replaceStage(const std::string &, const int, const libsumo::TraCIStage &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn replace_stage(personID: &str, stageIndex: i32, stage: &TraCIStage) {
        panic!("shortcut: Person::replaceStage needs convertTraCIStage (not ported)")
    }

    /// C++: `void Person::appendWaitingStage(const std::string &, double, const std::string &, const std::string &)` [static]
    // void
    // Person::appendWaitingStage(const std::string& personID, double duration, const std::string& description, const std::string& stopID) {
    //     MSTransportable* p = getPerson(personID);
    //     if (duration < 0) {
    //         throw TraCIException("Duration for person: '" + personID + "' must not be negative");
    //     }
    //     MSStoppingPlace* bs = nullptr;
    //     if (stopID != "") {
    //         bs = MSNet::getInstance()->getStoppingPlace(stopID, SUMO_TAG_BUS_STOP);
    //         if (bs == nullptr) {
    //             throw TraCIException("Invalid stopping place id '" + stopID + "' for person: '" + personID + "'");
    //         }
    //     }
    //     p->appendStage(new MSStageWaiting(p->getArrivalEdge(), nullptr, TIME2STEPS(duration), 0, p->getArrivalPos(), description, false));
    // }
    #[allow(unused_variables)]
    pub fn append_waiting_stage(personID: &str, duration: f64, description: &str, stopID: &str) {
        panic!("shortcut: Person::appendWaitingStage needs MSStageWaiting with the jumpDuration default (not ported)")
    }

    /// C++: `void Person::appendWalkingStage(const std::string &, const std::vector<std::string> &, double, double, double, const std::string &)` [static]
    // void
    // Person::appendWalkingStage(const std::string& personID, const std::vector<std::string>& edgeIDs, double arrivalPos, double duration, double speed, const std::string& stopID) {
    //     MSTransportable* p = getPerson(personID);
    //     ConstMSEdgeVector edges;
    //     try {
    //         MSEdge::parseEdgesList(edgeIDs, edges, "<unknown>");
    //     } catch (ProcessError& e) {
    //         throw TraCIException(e.what());
    //     }
    //     if (edges.empty()) {
    //         throw TraCIException("Empty edge list for walking stage of person '" + personID + "'.");
    //     }
    //     if (fabs(arrivalPos) > edges.back()->getLength()) {
    //         throw TraCIException("Invalid arrivalPos for walking stage of person '" + personID + "'.");
    //     }
    //     if (arrivalPos < 0) {
    //         arrivalPos += edges.back()->getLength();
    //     }
    //     MSStoppingPlace* bs = nullptr;
    //     if (stopID != "") {
    //         bs = MSNet::getInstance()->getStoppingPlace(stopID, SUMO_TAG_BUS_STOP);
    //         if (bs == nullptr) {
    //             throw TraCIException("Invalid stopping place id '" + stopID + "' for person: '" + personID + "'");
    //         }
    //     }
    //     p->appendStage(new MSStageWalking(p->getID(), edges, bs, TIME2STEPS(duration), speed, p->getArrivalPos(), arrivalPos, MSPModel::UNSPECIFIED_POS_LAT));
    // }
    #[allow(unused_variables)]
    pub fn append_walking_stage(personID: &str, edges: &Vec<String>, arrivalPos: f64, duration: f64, speed: f64, stopID: &str) {
        panic!("shortcut: Person::appendWalkingStage needs MSStageWalking with routeID/departLane defaults (not ported)")
    }

    /// C++: `void Person::appendDrivingStage(const std::string &, const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Person::appendDrivingStage(const std::string& personID, const std::string& toEdge, const std::string& lines, const std::string& stopID) {
    //     MSTransportable* p = getPerson(personID);
    //     const MSEdge* edge = MSEdge::dictionary(toEdge);
    //     if (!edge) {
    //         throw TraCIException("Invalid edge '" + toEdge + "' for person: '" + personID + "'");
    //     }
    //     if (lines.size() == 0) {
    //         throw TraCIException("Empty lines parameter for person: '" + personID + "'");
    //     }
    //     MSStoppingPlace* bs = nullptr;
    //     if (stopID != "") {
    //         bs = MSNet::getInstance()->getStoppingPlace(stopID);
    //         if (bs == nullptr) {
    //             throw TraCIException("Invalid stopping place id '" + stopID + "' for person: '" + personID + "'");
    //         }
    //     }
    //     p->appendStage(new MSStageDriving(nullptr, edge, bs, edge->getLength() - NUMERICAL_EPS, 0.0, StringTokenizer(lines).getVector()));
    // }
    #[allow(unused_variables)]
    pub fn append_driving_stage(personID: &str, toEdge: &str, lines: &str, stopID: &str) {
        panic!("shortcut: Person::appendDrivingStage needs MSStageDriving group/intendedVeh defaults (not ported)")
    }

    /// C++: `void Person::removeStage(const std::string &, int)` [static]
    // void
    // Person::removeStage(const std::string& personID, int nextStageIndex) {
    //     MSTransportable* p = getPerson(personID);
    //     if (nextStageIndex >= p->getNumRemainingStages()) {
    //         throw TraCIException("The stage index must be lower than the number of remaining stages.");
    //     }
    //     if (nextStageIndex < 0) {
    //         throw TraCIException("The stage index may not be negative.");
    //     }
    //     p->removeStage(nextStageIndex);
    // }
    #[allow(unused_variables)]
    pub fn remove_stage(personID: &str, nextStageIndex: i32) {
        let p = Self::get_person(personID);
        if nextStageIndex >= p.get_num_remaining_stages() {
            panic!("The stage index must be lower than the number of remaining stages.");
        }
        if nextStageIndex < 0 {
            panic!("The stage index may not be negative.");
        }
        p.m().remove_stage(nextStageIndex, true);
    }

    /// C++: `void Person::rerouteTraveltime(const std::string &)` [static]
    // void
    // Person::rerouteTraveltime(const std::string& personID) {
    //     MSPerson* p = getPerson(personID);
    //     if (p->getNumRemainingStages() == 0) {
    //         throw TraCIException("Person '" + personID + "' has no remaining stages.");
    //     }
    //     const MSEdge* from = p->getEdge();
    //     double  departPos = p->getEdgePos();
    //     // reroute to the start of the next-non-walking stage
    //     int firstIndex;
    //     if (p->getCurrentStageType() == MSStageType::WALKING) {
    //         firstIndex = 0;
    //     } else if (p->getCurrentStageType() == MSStageType::WAITING) {
    //         if (p->getNumRemainingStages() < 2 || p->getStageType(1) != MSStageType::WALKING) {
    //             throw TraCIException("Person '" + personID + "' cannot reroute after the current stop.");
    //         }
    //         firstIndex = 1;
    //     } else {
    //         throw TraCIException("Person '" + personID + "' cannot reroute in stage type '" + toString((int)p->getCurrentStageType()) + "'.");
    //     }
    //     int nextIndex = firstIndex + 1;
    //     for (; nextIndex < p->getNumRemainingStages(); nextIndex++) {
    //         if (p->getStageType(nextIndex) != MSStageType::WALKING) {
    //             break;
    //         }
    //     }
    //     MSStage* destStage = p->getNextStage(nextIndex - 1);
    //     const MSEdge* to = destStage->getEdges().back();
    //     double arrivalPos = destStage->getArrivalPos();
    //     double speed = p->getMaxSpeed();
    //     ConstMSEdgeVector newEdges;
    //     MSNet::getInstance()->getPedestrianRouter(0).compute(from, to, departPos, arrivalPos, speed, 0, nullptr, p->getVTypeParameter(), newEdges);
    //     if (newEdges.empty()) {
    //         throw TraCIException("Could not find new route for person '" + personID + "'.");
    //     }
    //     ConstMSEdgeVector oldEdges = p->getNextStage(firstIndex)->getEdges();
    //     assert(!oldEdges.empty());
    //     if (oldEdges.front()->getFunction() != SumoXMLEdgeFunc::NORMAL) {
    //         oldEdges.erase(oldEdges.begin());
    //     }
    //     //std::cout << " remainingStages=" << p->getNumRemainingStages() << " oldEdges=" << toString(oldEdges) << " newEdges=" << toString(newEdges) << " firstIndex=" << firstIndex << " nextIndex=" << nextIndex << "\n";
    //     if (newEdges == oldEdges && (firstIndex + 1 == nextIndex)) {
    //         return;
    //     }
    //     if (newEdges.front() != from) {
    //         // @note: maybe this should be done automatically by the router
    //         newEdges.insert(newEdges.begin(), from);
    //     }
    //     p->replaceWalk(newEdges, departPos, firstIndex, nextIndex);
    // }
    #[allow(unused_variables)]
    pub fn reroute_traveltime(personID: &str) {
        panic!("shortcut: Person::rerouteTraveltime needs MSPedestrianRouter::compute (not ported)")
    }

    /// C++: `void Person::moveTo(const std::string &, const std::string &, double, double)` [static]
    // void
    // Person::moveTo(const std::string& personID, const std::string& laneID, double pos, double posLat) {
    //     MSPerson* p = getPerson(personID);
    //     MSLane* l = MSLane::dictionary(laneID);
    //     if (l == nullptr) {
    //         throw TraCIException("Unknown lane '" + laneID + "'.");
    //     }
    //     if (posLat == INVALID_DOUBLE_VALUE) {
    //         posLat = 0;
    //     } else if (fabs(posLat) >= (0.5 * (l->getWidth() + p->getVehicleType().getWidth()) + MSPModel::SIDEWALK_OFFSET)) {
    //         // see MSPModel_Striping::moveToXY
    //         throw TraCIException("Invalid lateral position " + toString(posLat) + " on lane '" + laneID + "'.");
    //     }
    //     switch (p->getStageType(0)) {
    //         case MSStageType::WALKING: {
    //             MSStageWalking* s = dynamic_cast<MSStageWalking*>(p->getCurrentStage());
    //             assert(s != 0);
    //             s->getPState()->moveTo(p, l, pos, posLat, SIMSTEP);
    //             break;
    //         }
    //         default:
    //             throw TraCIException("Command moveTo is not supported for person '" + personID + "' while " + p->getCurrentStageDescription() + ".");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to(personID: &str, laneID: &str, pos: f64, posLat: f64) {
        panic!("shortcut: Person::moveTo needs MSStageWalking::getPState()->moveTo (not ported)")
    }

    /// C++: `void Person::moveToXY(const std::string &, const std::string &, const double, const double, double, const int, double)` [static]
    // void
    // Person::moveToXY(const std::string& personID, const std::string& edgeID, const double x, const double y, double angle, const int keepRoute, double matchThreshold) {
    //     MSPerson* p = getPerson(personID);
    //     const bool doKeepRoute = (keepRoute & 1) != 0;
    //     const bool mayLeaveNetwork = (keepRoute & 2) != 0;
    //     const bool ignorePermissions = (keepRoute & 4) != 0;
    //     SUMOVehicleClass vClass = ignorePermissions ? SVC_IGNORING : p->getVClass();
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
    // #ifdef DEBUG_MOVEXY
    //     std::cout << std::endl << "begin person " << p->getID() << " lanePos:" << p->getEdgePos() << " edge:" << Named::getIDSecure(p->getEdge()) << "\n";
    //     std::cout << " want pos:" << pos << " edgeID:" << edgeID <<  " origAngle:" << origAngle << " angle:" << angle << " keepRoute:" << keepRoute << std::endl;
    // #endif
    //
    //     ConstMSEdgeVector edges;
    //     MSLane* lane = nullptr;
    //     double lanePos;
    //     double lanePosLat = 0;
    //     double bestDistance = std::numeric_limits<double>::max();
    //     int routeOffset = 0;
    //     bool found = false;
    //     double maxRouteDistance = matchThreshold;
    //
    //     ConstMSEdgeVector ev;
    //     ev.push_back(p->getEdge());
    //     int routeIndex = 0;
    //     MSLane* currentLane = const_cast<MSLane*>(getSidewalk<MSEdge, MSLane>(p->getEdge()));
    //     switch (p->getStageType(0)) {
    //         case MSStageType::WALKING: {
    //             MSStageWalking* s = dynamic_cast<MSStageWalking*>(p->getCurrentStage());
    //             assert(s != 0);
    //             ev = s->getEdges();
    //             routeIndex = (int)(s->getRouteStep() - s->getRoute().begin());
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    //     if (doKeepRoute) {
    //         // case a): vehicle is on its earlier route
    //         //  we additionally assume it is moving forward (SUMO-limit);
    //         //  note that the route ("edges") is not changed in this case
    //         found = Helper::moveToXYMap_matchingRoutePosition(pos, edgeID,
    //                 ev, routeIndex, vClass, true,
    //                 bestDistance, &lane, lanePos, routeOffset);
    //         if (bestDistance > maxRouteDistance) {
    //             found = false;
    //             lane = nullptr;
    //         }
    //     } else {
    //         double speed = pos.distanceTo2D(p->getPosition()); // !!!veh->getSpeed();
    //         found = Helper::moveToXYMap(pos, maxRouteDistance, mayLeaveNetwork, edgeID, angle,
    //                                     speed, ev, routeIndex, currentLane, p->getEdgePos(), currentLane != nullptr,
    //                                     vClass, GeomHelper::naviDegree(p->getAngle()), true,
    //                                     bestDistance, &lane, lanePos, routeOffset, edges);
    //         if (edges.size() != 0 && ev.size() > 1) {
    //             // try to rebuild the route
    //             const MSEdge* origEdge = p->getEdge();
    //             assert(lane != nullptr);
    //             const MSJunction* originalTarget = nullptr;
    //             if (origEdge->isNormal()) {
    //                 if (routeIndex == 0) {
    //                     if (origEdge->getToJunction() == ev[1]->getToJunction() || origEdge->getToJunction() == ev[1]->getFromJunction()) {
    //                         originalTarget = origEdge->getToJunction();
    //                     } else {
    //                         originalTarget = origEdge->getFromJunction();
    //                     }
    //                 } else {
    //                     if (origEdge->getToJunction() == ev[routeIndex - 1]->getToJunction() || origEdge->getToJunction() == ev[routeIndex - 1]->getFromJunction()) {
    //                         originalTarget = origEdge->getFromJunction();
    //                     } else {
    //                         originalTarget = origEdge->getToJunction();
    //                     }
    //                 }
    //             } else {
    //                 originalTarget = origEdge->getToJunction();
    //                 assert(originalTarget == origEdge->getFromJunction());
    //             }
    //             const MSEdge* newEdge = edges[0];
    //             if (edges[0]->getFromJunction() == originalTarget || edges[0]->getToJunction() == originalTarget) {
    //                 edges = ev;
    //                 edges[routeIndex] = newEdge;
    //             }
    //         }
    //     }
    //     if (found || mayLeaveNetwork) {
    //         // compute lateral offset
    //         if (found) {
    //             const double perpDist = lane->getShape().distance2D(pos, false);
    //             if (perpDist != GeomHelper::INVALID_OFFSET) {
    //                 lanePosLat = perpDist;
    //                 if (!mayLeaveNetwork) {
    //                     lanePosLat = MIN2(lanePosLat, 0.5 * (lane->getWidth() + p->getVehicleType().getWidth()));
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
    //         }
    //         if (found && !mayLeaveNetwork && MSGlobals::gLateralResolution < 0) {
    //             // mapped position may differ from pos
    //             pos = lane->geometryPositionAtOffset(lanePos, -lanePosLat);
    //         }
    //         assert((found && lane != nullptr) || (!found && lane == nullptr));
    //         switch (p->getStageType(0)) {
    //             case MSStageType::WALKING: {
    //                 if (angle == INVALID_DOUBLE_VALUE) {
    //                     // walking angle cannot be deduced from road angle so we always use the last pos
    //                     angle = GeomHelper::naviDegree(p->getPosition().angleTo2D(pos));
    //                 }
    //                 break;
    //             }
    //             case MSStageType::WAITING_FOR_DEPART:
    //             case MSStageType::WAITING: {
    //                 if (p->getNumRemainingStages() <= 1 || p->getStageType(1) != MSStageType::WALKING) {
    //                     // insert walking stage after the current stage
    //                     ConstMSEdgeVector route({p->getEdge()});
    //                     const double departPos = p->getCurrentStage()->getArrivalPos();
    //                     p->appendStage(new MSStageWalking(p->getID(), route, nullptr, -1, -1, departPos, departPos, MSPModel::UNSPECIFIED_POS_LAT), 1);
    //                 }
    //                 // abort waiting stage and proceed to walking stage
    //                 p->removeStage(0);
    //                 assert(p->getStageType(0) == MSStageType::WALKING);
    //                 if (angle == INVALID_DOUBLE_VALUE) {
    //                     if (lane != nullptr && !lane->isWalkingArea()) {
    //                         angle = GeomHelper::naviDegree(lane->getShape().rotationAtOffset(lanePos));
    //                     } else {
    //                         // compute angle outside road network or on walkingarea from old and new position
    //                         angle = GeomHelper::naviDegree(p->getPosition().angleTo2D(pos));
    //                     }
    //                 }
    //                 break;
    //             }
    //             default:
    //                 throw TraCIException("Command moveToXY is not supported for person '" + personID + "' while " + p->getCurrentStageDescription() + ".");
    //         }
    //         Helper::setRemoteControlled(p, pos, lane, lanePos, lanePosLat, angle, routeOffset, edges, MSNet::getInstance()->getCurrentTimeStep());
    //     } else {
    //         if (lane == nullptr) {
    //             throw TraCIException("Could not map person '" + personID + "' no road found within " + toString(maxRouteDistance) + "m.");
    //         } else {
    //             throw TraCIException("Could not map person '" + personID + "' distance to road is " + toString(bestDistance) + ".");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_to_xy(personID: &str, edgeID: &str, x: f64, y: f64, angle: f64, keepRoute: i32, matchThreshold: f64) {
        panic!("shortcut: Person::moveToXY is not ported yet (needs the full Helper::moveToXYMap flow)")
    }

    /// C++: `void Person::remove(const std::string &, char)` [static]
    // void
    // Person::remove(const std::string& personID, char /*reason*/) {
    //     MSPerson* person = getPerson(personID);
    //     // remove all stages after the current and then abort the current stage
    //     // (without adding a zero-length waiting stage)
    //     while (person->getNumRemainingStages() > 1) {
    //         person->removeStage(1);
    //     }
    //     person->removeStage(0, false);
    // }
    #[allow(unused_variables)]
    pub fn remove(personID: &str, reason: i8) {
        let person = Self::get_person(personID);
        // remove all stages after the current and then abort the current stage
        while person.get_num_remaining_stages() > 1 {
            person.m().remove_stage(1, true);
        }
        person.m().remove_stage(0, false);
    }

    /// C++: `void Person::setSpeed(const std::string &, double)` [static]
    // void
    // Person::setSpeed(const std::string& personID, double speed) {
    //     getPerson(personID)->setSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_speed(personID: &str, speed: f64) {
        Self::get_person(personID).m().set_speed(speed);
    }

    /// C++: `void Person::setType(const std::string &, const std::string &)` [static]
    // void
    // Person::setType(const std::string& personID, const std::string& typeID) {
    //     MSVehicleType* vehicleType = MSNet::getInstance()->getVehicleControl().getVType(typeID);
    //     if (vehicleType == nullptr) {
    //         throw TraCIException("The vehicle type '" + typeID + "' is not known.");
    //     }
    //     getPerson(personID)->replaceVehicleType(vehicleType);
    // }
    #[allow(unused_variables)]
    pub fn set_type(personID: &str, typeID: &str) {
        let vehicleType = MSNet::get_instance().m().get_vehicle_control().get_v_type(typeID, Ptr::null(), false);
        if vehicleType.is_null() {
            panic!("The vehicle type '{}' is not known.", typeID);
        }
        Self::get_person(personID).m().replace_vehicle_type(vehicleType);
    }

    /// C++: `void Person::setLength(const std::string &, double)` [static]
    // void
    // Person::setLength(const std::string& personID, double length) {
    //     getPerson(personID)->getSingularType().setLength(length);
    // }
    #[allow(unused_variables)]
    pub fn set_length(typeID: &str, length: f64) {
        Self::get_person(typeID).m().get_singular_type().set_length(&length);
    }

    /// C++: `void Person::setMaxSpeed(const std::string &, double)` [static]
    // void
    // Person::setMaxSpeed(const std::string& personID, double speed) {
    //     getPerson(personID)->getSingularType().setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed(typeID: &str, speed: f64) {
        Self::get_person(typeID).m().get_singular_type().set_max_speed(&speed);
    }

    /// C++: `void Person::setVehicleClass(const std::string &, const std::string &)` [static]
    // void
    // Person::setVehicleClass(const std::string& personID, const std::string& clazz) {
    //     getPerson(personID)->getSingularType().setVClass(getVehicleClassID(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_vehicle_class(typeID: &str, clazz: &str) {
        panic!("shortcut: Person::setVehicleClass needs getVehicleClassID error handling (ProcessError not translated)")
    }

    /// C++: `void Person::setSpeedFactor(const std::string &, double)` [static]
    // void
    // Person::setSpeedFactor(const std::string& personID, double factor) {
    //     getPerson(personID)->setChosenSpeedFactor(factor);
    // }
    #[allow(unused_variables)]
    pub fn set_speed_factor(typeID: &str, factor: f64) {
        Self::get_person(typeID).m().set_chosen_speed_factor(factor);
    }

    /// C++: `void Person::setAccel(const std::string &, double)` [static]
    // void
    // Person::setAccel(const std::string& personID, double accel) {
    //     getPerson(personID)->getSingularType().setAccel(accel);
    // }
    #[allow(unused_variables)]
    pub fn set_accel(typeID: &str, accel: f64) {
        Self::get_person(typeID).m().get_singular_type().set_accel(accel);
    }

    /// C++: `void Person::setDecel(const std::string &, double)` [static]
    // void
    // Person::setDecel(const std::string& personID, double decel) {
    //     getPerson(personID)->getSingularType().setDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_decel(typeID: &str, decel: f64) {
        Self::get_person(typeID).m().get_singular_type().set_decel(decel);
    }

    /// C++: `void Person::setEmergencyDecel(const std::string &, double)` [static]
    // void
    // Person::setEmergencyDecel(const std::string& personID, double decel) {
    //     getPerson(personID)->getSingularType().setEmergencyDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(typeID: &str, decel: f64) {
        Self::get_person(typeID).m().get_singular_type().set_emergency_decel(decel);
    }

    /// C++: `void Person::setApparentDecel(const std::string &, double)` [static]
    // void
    // Person::setApparentDecel(const std::string& personID, double decel) {
    //     getPerson(personID)->getSingularType().setApparentDecel(decel);
    // }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(typeID: &str, decel: f64) {
        Self::get_person(typeID).m().get_singular_type().set_apparent_decel(decel);
    }

    /// C++: `void Person::setImperfection(const std::string &, double)` [static]
    // void
    // Person::setImperfection(const std::string& personID, double imperfection) {
    //     getPerson(personID)->getSingularType().setImperfection(imperfection);
    // }
    #[allow(unused_variables)]
    pub fn set_imperfection(typeID: &str, imperfection: f64) {
        Self::get_person(typeID).m().get_singular_type().set_imperfection(imperfection);
    }

    /// C++: `void Person::setTau(const std::string &, double)` [static]
    // void
    // Person::setTau(const std::string& personID, double tau) {
    //     getPerson(personID)->getSingularType().setTau(tau);
    // }
    #[allow(unused_variables)]
    pub fn set_tau(typeID: &str, tau: f64) {
        Self::get_person(typeID).m().get_singular_type().set_tau(tau);
    }

    /// C++: `void Person::setEmissionClass(const std::string &, const std::string &)` [static]
    // void
    // Person::setEmissionClass(const std::string& personID, const std::string& clazz) {
    //     getPerson(personID)->getSingularType().setEmissionClass(PollutantsInterface::getClassByName(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_emission_class(typeID: &str, clazz: &str) {
        Self::get_person(typeID).m().get_singular_type().set_emission_class(PollutantsInterface::get_class_by_name(clazz, SVC_IGNORING));
    }

    /// C++: `void Person::setShapeClass(const std::string &, const std::string &)` [static]
    // void
    // Person::setShapeClass(const std::string& personID, const std::string& clazz) {
    //     getPerson(personID)->getSingularType().setShape(getVehicleShapeID(clazz));
    // }
    #[allow(unused_variables)]
    pub fn set_shape_class(typeID: &str, shapeClass: &str) {
        panic!("shortcut: Person::setShapeClass needs getVehicleShapeID error handling (ProcessError not translated)")
    }

    /// C++: `void Person::setWidth(const std::string &, double)` [static]
    // void
    // Person::setWidth(const std::string& personID, double width) {
    //     getPerson(personID)->getSingularType().setWidth(width);
    // }
    #[allow(unused_variables)]
    pub fn set_width(typeID: &str, width: f64) {
        Self::get_person(typeID).m().get_singular_type().set_width(&width);
    }

    /// C++: `void Person::setHeight(const std::string &, double)` [static]
    // void
    // Person::setHeight(const std::string& personID, double height) {
    //     getPerson(personID)->getSingularType().setHeight(height);
    // }
    #[allow(unused_variables)]
    pub fn set_height(typeID: &str, height: f64) {
        Self::get_person(typeID).m().get_singular_type().set_height(&height);
    }

    /// C++: `void Person::setMass(const std::string &, double)` [static]
    // void
    // Person::setMass(const std::string& personID, double mass) {
    //     getPerson(personID)->getSingularType().setMass(mass);
    // }
    #[allow(unused_variables)]
    pub fn set_mass(typeID: &str, mass: f64) {
        Self::get_person(typeID).m().get_singular_type().set_mass(mass);
    }

    /// C++: `void Person::setColor(const std::string &, const libsumo::TraCIColor &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_color(typeID: &str, color: &TraCIColor) {
        panic!("shortcut: Person::setColor writes SUMOVehicleParameter through a const path, needs mutable parameter access")
    }

    /// C++: `void Person::setMinGap(const std::string &, double)` [static]
    // void
    // Person::setMinGap(const std::string& personID, double minGap) {
    //     getPerson(personID)->getSingularType().setMinGap(minGap);
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap(typeID: &str, minGap: f64) {
        Self::get_person(typeID).m().get_singular_type().set_min_gap(&minGap);
    }

    /// C++: `void Person::setMinGapLat(const std::string &, double)` [static]
    // void
    // Person::setMinGapLat(const std::string& personID, double minGapLat) {
    //     getPerson(personID)->getSingularType().setMinGapLat(minGapLat);
    // }
    #[allow(unused_variables)]
    pub fn set_min_gap_lat(typeID: &str, minGapLat: f64) {
        Self::get_person(typeID).m().get_singular_type().set_min_gap_lat(&minGapLat);
    }

    /// C++: `void Person::setMaxSpeedLat(const std::string &, double)` [static]
    // void
    // Person::setMaxSpeedLat(const std::string& personID, double speed) {
    //     getPerson(personID)->getSingularType().setMaxSpeedLat(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_max_speed_lat(typeID: &str, speed: f64) {
        Self::get_person(typeID).m().get_singular_type().set_max_speed_lat(&speed);
    }

    /// C++: `void Person::setLateralAlignment(const std::string &, const std::string &)` [static]
    // void
    // Person::setLateralAlignment(const std::string& personID, const std::string& latAlignment) {
    //     double lao;
    //     LatAlignmentDefinition lad;
    //     if (SUMOVTypeParameter::parseLatAlignment(latAlignment, lao, lad)) {
    //         getPerson(personID)->getSingularType().setPreferredLateralAlignment(lad, lao);
    //     } else {
    //         throw TraCIException("Unknown value '" + latAlignment + "' when setting latAlignment for person '" + personID + "';\n must be one of (\"right\", \"center\", \"arbitrary\", \"nice\", \"compact\", \"left\" or a float)");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_lateral_alignment(typeID: &str, latAlignment: &str) {
        panic!("shortcut: Person::setLateralAlignment needs a LatAlignmentDefinition default (not ported)")
    }

    /// C++: `void Person::setActionStepLength(const std::string &, double, bool)` [static]
    // void
    // Person::setActionStepLength(const std::string& personID, double actionStepLength, bool resetActionOffset) {
    //     getPerson(personID)->getSingularType().setActionStepLength(SUMOVehicleParserHelper::processActionStepLength(actionStepLength), resetActionOffset);
    // }
    #[allow(unused_variables)]
    pub fn set_action_step_length(typeID: &str, actionStepLength: f64, resetActionOffset: bool) {
        Self::get_person(typeID).m().get_singular_type().set_action_step_length(SUMOVehicleParserHelper::process_action_step_length(actionStepLength), resetActionOffset);
    }

    /// C++: `void Person::setBoardingDuration(const std::string &, double)` [static]
    // void
    // Person::setBoardingDuration(const std::string& personID, double boardingDuration)  {
    //     Helper::getPerson(personID)->getSingularType().setBoardingDuration(TIME2STEPS(boardingDuration));
    // }
    #[allow(unused_variables)]
    pub fn set_boarding_duration(typeID: &str, boardingDuration: f64) {
        Self::get_person(typeID).m().get_singular_type().set_boarding_duration(time2steps(boardingDuration), true);
    }

    /// C++: `void Person::setImpatience(const std::string &, double)` [static]
    // void
    // Person::setImpatience(const std::string& personID, double impatience)  {
    //     Helper::getVehicle(personID)->getSingularType().setImpatience(impatience);
    // }
    #[allow(unused_variables)]
    pub fn set_impatience(typeID: &str, impatience: f64) {
        Helper::get_vehicle(typeID).m().get_singular_type().set_impatience(impatience);
    }

    /// C++: `void Person::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_PERSON_VARIABLE, objectID, varIDs, begin, end, parameters, 0, 0.0);
    }

    /// C++: `void Person::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        Helper::subscribe(CMD_SUBSCRIBE_PERSON_VARIABLE, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), 0, 0.0);
    }

    /// C++: `void Person::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        Helper::subscribe(CMD_SUBSCRIBE_PERSON_CONTEXT, objectID, varIDs, begin, end, parameters, domain, dist);
    }

    /// C++: `void Person::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        Helper::subscribe(CMD_SUBSCRIBE_PERSON_CONTEXT, objectID, &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, &TraCIResults::new(), domain, dist);
    }

    /// C++: `const libsumo::SubscriptionResults Person::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::TraCIResults Person::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        MY_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `const libsumo::ContextSubscriptionResults Person::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow().clone())
    }

    /// C++: `const libsumo::SubscriptionResults Person::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        MY_CONTEXT_SUBSCRIPTION_RESULTS.with(|r| r.borrow_mut().entry(objectID.to_string()).or_default().clone())
    }

    /// C++: `void Person::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Person, PERSON)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        panic!("shortcut: Person::subscribeParameterWithKey needs TraCIString as Rc<TraCIResult> (class hierarchy)")
    }

    /// C++: `int Person::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        CMD_GET_PERSON_VARIABLE
    }

    /// C++: `void Person::storeShape(const std::string &, PositionVector &)` [static]
    /// Saves the shape of the requested object in the given container
    // void
    // Person::storeShape(const std::string& id, PositionVector& shape) {
    //     shape.push_back(getPerson(id)->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn store_shape(id: &str, shape: &mut PositionVector) {
        shape.push(Self::get_person(id).get_position_2());
    }

    /// C++: `std::shared_ptr<VariableWrapper> Person::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Person::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        panic!("shortcut: Person::makeWrapper needs the subscription wrapper over class statics (not ported)")
    }

    /// C++: `bool Person::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Person::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
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
    //         case VAR_SLOPE:
    //             return wrapper->wrapDouble(objID, variable, getSlope(objID));
    //         case VAR_SPEED:
    //             return wrapper->wrapDouble(objID, variable, getSpeed(objID));
    //         case VAR_ROAD_ID:
    //             return wrapper->wrapString(objID, variable, getRoadID(objID));
    //         case VAR_LANE_ID:
    //             return wrapper->wrapString(objID, variable, getLaneID(objID));
    //         case VAR_LANEPOSITION:
    //             return wrapper->wrapDouble(objID, variable, getLanePosition(objID));
    //         case VAR_ARRIVALPOS:
    //             return wrapper->wrapDouble(objID, variable, getArrivalPos(objID));
    //         case VAR_COLOR:
    //             return wrapper->wrapColor(objID, variable, getColor(objID));
    //         case VAR_WAITING_TIME:
    //             return wrapper->wrapDouble(objID, variable, getWaitingTime(objID));
    //         case VAR_IMPATIENCE:
    //             return wrapper->wrapDouble(objID, variable, getImpatience(objID));
    //         case VAR_TYPE:
    //             return wrapper->wrapString(objID, variable, getTypeID(objID));
    //         case VAR_SPEED_FACTOR:
    //             return wrapper->wrapDouble(objID, variable, getSpeedFactor(objID));
    //         case VAR_NEXT_EDGE:
    //             return wrapper->wrapString(objID, variable, getNextEdge(objID));
    //         case VAR_EDGES:
    //             return wrapper->wrapStringList(objID, variable, getEdges(objID, StoHelp::readTypedInt(*paramData)));
    //         case VAR_STAGE:
    //             return wrapper->wrapStage(objID, variable, getStage(objID, StoHelp::readTypedInt(*paramData)));
    //         case VAR_STAGES_REMAINING:
    //             return wrapper->wrapInt(objID, variable, getRemainingStages(objID));
    //         case VAR_VEHICLE:
    //             return wrapper->wrapString(objID, variable, getVehicle(objID));
    //         case VAR_MAXSPEED:
    //             // integrate desiredMaxSpeed and individual speedFactor
    //             return wrapper->wrapDouble(objID, variable, getMaxSpeed(objID));
    //         case DISTANCE_REQUEST: {
    //             TraCIRoadPosition roadPos;
    //             Position pos;
    //             if (Helper::readDistanceRequest(*paramData, roadPos, pos) == libsumo::POSITION_ROADMAP) {
    //                 return wrapper->wrapDouble(objID, variable, getWalkingDistance(objID, roadPos.edgeID, roadPos.pos, roadPos.laneIndex));
    //             }
    //             return wrapper->wrapDouble(objID, variable, getWalkingDistance2D(objID, pos.x(), pos.y()));
    //         }
    //         case VAR_PARAMETER:
    //             return wrapper->wrapString(objID, variable, getParameter(objID, StoHelp::readTypedString(*paramData)));
    //         case VAR_PARAMETER_WITH_KEY:
    //             return wrapper->wrapStringPair(objID, variable, getParameterWithKey(objID, StoHelp::readTypedString(*paramData)));
    //         case VAR_TAXI_RESERVATIONS:
    //             return wrapper->wrapReservationVector(objID, variable, getTaxiReservations(StoHelp::readTypedInt(*paramData)));
    //         case SPLIT_TAXI_RESERVATIONS:
    //             return wrapper->wrapString(objID, variable, splitTaxiReservation(objID, StoHelp::readTypedStringList(*paramData)));
    //         default:
    //             return libsumo::VehicleType::handleVariable(getTypeID(objID), variable, wrapper, paramData);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_variable(objID: &str, variable: i32, wrapper: Ptr<VariableWrapper>, paramData: Ptr<Storage>) -> bool {
        match variable {
            TRACI_ID_LIST => wrapper.m().wrap_string_list(objID, variable, &Self::get_id_list()),
            ID_COUNT => wrapper.m().wrap_int(objID, variable, Self::get_id_count()),
            VAR_POSITION => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, false)),
            VAR_POSITION3D => wrapper.m().wrap_position(objID, variable, &Self::get_position(objID, true)),
            VAR_ANGLE => wrapper.m().wrap_double(objID, variable, Self::get_angle(objID)),
            VAR_SLOPE => wrapper.m().wrap_double(objID, variable, Self::get_slope(objID)),
            VAR_SPEED => wrapper.m().wrap_double(objID, variable, Self::get_speed(objID)),
            VAR_ROAD_ID => wrapper.m().wrap_string(objID, variable, &Self::get_road_id(objID)),
            VAR_LANE_ID => wrapper.m().wrap_string(objID, variable, &Self::get_lane_id(objID)),
            VAR_LANEPOSITION => wrapper.m().wrap_double(objID, variable, Self::get_lane_position(objID)),
            VAR_ARRIVALPOS => wrapper.m().wrap_double(objID, variable, Self::get_arrival_pos(objID)),
            VAR_COLOR => wrapper.m().wrap_color(objID, variable, &Self::get_color(objID)),
            VAR_WAITING_TIME => wrapper.m().wrap_double(objID, variable, Self::get_waiting_time(objID)),
            VAR_IMPATIENCE => wrapper.m().wrap_double(objID, variable, Self::get_impatience(objID)),
            VAR_TYPE => wrapper.m().wrap_string(objID, variable, &Self::get_type_id(objID)),
            VAR_SPEED_FACTOR => wrapper.m().wrap_double(objID, variable, Self::get_speed_factor(objID)),
            VAR_NEXT_EDGE => wrapper.m().wrap_string(objID, variable, &Self::get_next_edge(objID)),
            VAR_EDGES => {
                let n = StorageHelper::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_string_list(objID, variable, &Self::get_edges(objID, n))
            }
            VAR_STAGE => panic!("shortcut: Person::handleVariable VAR_STAGE needs getStage (not ported)"),
            VAR_STAGES_REMAINING => wrapper.m().wrap_int(objID, variable, Self::get_remaining_stages(objID)),
            VAR_VEHICLE => wrapper.m().wrap_string(objID, variable, &Self::get_vehicle(objID)),
            VAR_MAXSPEED => wrapper.m().wrap_double(objID, variable, Self::get_max_speed(objID)),
            DISTANCE_REQUEST => panic!("shortcut: Person::handleVariable DISTANCE_REQUEST needs default Position/TraCIRoadPosition construction (not ported)"),
            VAR_PARAMETER => {
                let key = StorageHelper::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string(objID, variable, &Self::get_parameter(objID, &key))
            }
            VAR_PARAMETER_WITH_KEY => {
                let key = StorageHelper::read_typed_string(paramData.m(), "");
                wrapper.m().wrap_string_pair(objID, variable, &Self::get_parameter_with_key(objID, &key))
            }
            VAR_TAXI_RESERVATIONS => {
                let n = StorageHelper::read_typed_int(paramData.m(), "");
                wrapper.m().wrap_reservation_vector(objID, variable, &Self::get_taxi_reservations(n))
            }
            SPLIT_TAXI_RESERVATIONS => {
                let ids = StorageHelper::read_typed_string_list(paramData.m(), "");
                wrapper.m().wrap_string(objID, variable, &Self::split_taxi_reservation(objID.to_string(), &ids))
            }
            _ => VehicleType::handle_variable(&Self::get_type_id(objID), variable, wrapper, paramData),
        }
    }

    /// C++: `MSPerson * Person::getPerson(const std::string &)` [static, private]
    // MSPerson*
    // Person::getPerson(const std::string& personID) {
    //     return Helper::getPerson(personID);
    // }
    #[allow(unused_variables)]
    pub fn get_person(id: &str) -> Ptr<MSPerson> {
        Helper::get_person(id)
    }

    /// C++: `MSStage * Person::convertTraCIStage(const TraCIStage &, const std::string)` [static, private]
    // MSStage*
    // Person::convertTraCIStage(const TraCIStage& stage, const std::string personID) {
    //     MSStoppingPlace* bs = nullptr;
    //     if (!stage.destStop.empty()) {
    //         bs = MSNet::getInstance()->getStoppingPlace(stage.destStop);
    //         if (bs == nullptr) {
    //             throw TraCIException("Invalid stopping place id '" + stage.destStop + "' for person: '" + personID + "'");
    //         }
    //     }
    //     switch (stage.type) {
    //         case STAGE_DRIVING: {
    //             if (stage.edges.empty()) {
    //                 throw TraCIException("The stage should have at least one edge");
    //             }
    //             std::string toId = stage.edges.back();
    //             MSEdge* to = MSEdge::dictionary(toId);
    //             if (!to) {
    //                 throw TraCIException("Invalid edge '" + toId + "' for person: '" + personID + "'");
    //             }
    //             //std::string fromId = stage.edges.front();
    //             //MSEdge* from = MSEdge::dictionary(fromId);
    //             //if (!from) {
    //             //    throw TraCIException("Invalid edge '" + fromId + "' for person: '" + personID + "'");
    //             //}
    //             if (stage.line.empty()) {
    //                 throw TraCIException("Empty lines parameter for person: '" + personID + "'");
    //             }
    //             double arrivalPos = stage.arrivalPos;
    //             if (arrivalPos == INVALID_DOUBLE_VALUE) {
    //                 if (bs != nullptr) {
    //                     arrivalPos = bs->getEndLanePosition();
    //                 } else {
    //                     arrivalPos = to->getLength();
    //                 }
    //             }
    //             return new MSStageDriving(nullptr, to, bs, arrivalPos, 0.0, StringTokenizer(stage.line).getVector());
    //         }
    //
    //         case STAGE_WALKING: {
    //             MSTransportable* p = getPerson(personID);
    //             ConstMSEdgeVector edges;
    //             try {
    //                 MSEdge::parseEdgesList(stage.edges, edges, "<unknown>");
    //             } catch (ProcessError& e) {
    //                 throw TraCIException(e.what());
    //             }
    //             if (edges.empty()) {
    //                 throw TraCIException("Empty edge list for walking stage of person '" + personID + "'.");
    //             }
    //             double arrivalPos = stage.arrivalPos;
    //             if (fabs(arrivalPos) > edges.back()->getLength()) {
    //                 throw TraCIException("Invalid arrivalPos for walking stage of person '" + personID + "'.");
    //             }
    //             if (arrivalPos < 0) {
    //                 arrivalPos += edges.back()->getLength();
    //             }
    //             return new MSStageWalking(p->getID(), edges, bs, -1, -1, p->getArrivalPos(), arrivalPos, MSPModel::UNSPECIFIED_POS_LAT);
    //         }
    //
    //         case STAGE_WAITING: {
    //             MSTransportable* p = getPerson(personID);
    //             if (stage.travelTime < 0) {
    //                 throw TraCIException("Duration for person: '" + personID + "' must not be negative");
    //             }
    //             return new MSStageWaiting(p->getArrivalEdge(), nullptr, TIME2STEPS(stage.travelTime), 0, p->getArrivalPos(), stage.description, false);
    //         }
    //         case STAGE_TRIP: {
    //             MSTransportable* p = getPerson(personID);
    //             ConstMSEdgeVector edges;
    //             try {
    //                 MSEdge::parseEdgesList(stage.edges, edges, "<unknown>");
    //             } catch (ProcessError& e) {
    //                 throw TraCIException(e.what());
    //             }
    //             if ((edges.size() == 0 && bs == nullptr) || edges.size() > 1) {
    //                 throw TraCIException("A trip should be defined with a destination edge or a destination stop for person '" + personID + "'.");
    //             }
    //             const MSEdge* to = nullptr;
    //             if (bs != nullptr) {
    //                 to = &bs->getLane().getEdge();
    //                 if (edges.size() > 0 && to != edges.back()) {
    //                     throw TraCIException("Mismatching destination edge and destination stop edge for person '" + personID + "'.");
    //                 }
    //             } else {
    //                 to = edges.back();
    //             }
    //             SVCPermissions modeSet = 0;
    //             MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //             for (std::string vtypeid : StringTokenizer(stage.vType).getVector()) {
    //                 const MSVehicleType* const vType = vehControl.getVType(vtypeid);
    //                 if (vType == nullptr) {
    //                     throw TraCIException("The vehicle type '" + vtypeid + "' in a trip for person '" + personID + "' is not known.");
    //                 }
    //                 modeSet |= (vType->getVehicleClass() == SVC_BICYCLE) ? SVC_BICYCLE : SVC_PASSENGER;
    //             }
    //             if (stage.line.empty()) {
    //                 modeSet = p->getParameter().modes;
    //             } else {
    //                 std::string errorMsg;
    //                 if (!SUMOVehicleParameter::parsePersonModes(stage.line, "person", personID, modeSet, errorMsg)) {
    //                     throw TraCIException(errorMsg);
    //                 }
    //             }
    //             bool hasArrivalPos = stage.arrivalPos != INVALID_DOUBLE_VALUE;
    //             double arrivalPos = stage.arrivalPos;
    //             if (hasArrivalPos) {
    //                 if (fabs(arrivalPos) > to->getLength()) {
    //                     throw TraCIException("Invalid arrivalPos for walking stage of person '" + personID + "'.");
    //                 }
    //                 if (arrivalPos < 0) {
    //                     arrivalPos += to->getLength();
    //                 }
    //             }
    //             const MSStage* cur = p->getCurrentStage();
    //             double walkfactor = OptionsCont::getOptions().getFloat("persontrip.walkfactor");
    //             std::string group = stage.intended; //OptionsCont::getOptions().getString("persontrip.default.group");
    //             const SUMOTime duration = -1;
    //             const double speed = -1;
    //             return new MSStageTrip(cur->getDestination(), cur->getDestinationStop(), to, bs,
    //                                    duration, modeSet, stage.vType, speed, walkfactor, group,
    //                                    MSPModel::UNSPECIFIED_POS_LAT, hasArrivalPos, arrivalPos);
    //         }
    //         default:
    //             return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn convert_tra_ci_stage(stage: &TraCIStage, personID: String) -> Ptr<MSStage> {
        panic!("shortcut: Person::convertTraCIStage needs the MSStage constructors with unported defaults")
    }

    /// C++: `bool Person::filterReservation(int, const Reservation *, std::vector<libsumo::TraCIReservation> &)` [static, private]
    // bool
    // Person::filterReservation(int onlyNew, const Reservation* res, std::vector<libsumo::TraCIReservation>& reservations) {
    //     if (onlyNew != 0 && (onlyNew & res->state) == 0) {
    //         return false;
    //     }
    //     std::vector<std::string> personIDs;
    //     for (const MSTransportable* p : res->persons) {
    //         personIDs.push_back(p->getID());
    //     }
    //     std::sort(personIDs.begin(), personIDs.end());
    //     reservations.push_back(TraCIReservation(res->id,
    //                                             personIDs,
    //                                             res->group,
    //                                             res->from->getID(),
    //                                             res->to->getID(),
    //                                             res->fromPos,
    //                                             res->toPos,
    //                                             STEPS2TIME(res->pickupTime),
    //                                             STEPS2TIME(res->reservationTime),
    //                                             res->state
    //                                            ));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn filter_reservation(stateFilter: i32, res: Ptr<Reservation>, reservations: &mut Vec<TraCIReservation>) -> bool {
        panic!("shortcut: Person::filterReservation needs the Reservation type from MSDispatch (not ported)")
    }

    /// C++: `void Person::Person()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Person {}
    }

}

/// clase for CW Sorter
/// C++ class `libsumo::Person::reservation_by_id_sorter` (libsumo/Person.h)
pub struct Person_reservation_by_id_sorter {
}

impl Person_reservation_by_id_sorter {
    /// C++: `void reservation_by_id_sorter::reservation_by_id_sorter()`
    /// constructor
    // reservation_by_id_sorter() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Person_reservation_by_id_sorter {}
    }

    /// C++: `int reservation_by_id_sorter::operator()(const TraCIReservation &, const TraCIReservation &) const` [const]
    /// comparing operation for sort
    // int
    // Person::reservation_by_id_sorter::operator()(const TraCIReservation& r1, const TraCIReservation& r2) const {
    //     return r1.id < r2.id;
    // }
    #[allow(unused_variables)]
    pub fn call_op(&self, r1: &TraCIReservation, r2: &TraCIReservation) -> i32 {
        if r1.id < r2.id { 1 } else { 0 }
    }

}

