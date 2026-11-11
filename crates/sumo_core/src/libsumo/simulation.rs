//! Port of libsumo/Simulation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ TraCI client API implementation
/// C++ class `libsumo::Simulation` (libsumo/Simulation.h)
pub struct Simulation {
}

// C++ static members of Simulation (port as module-level statics, see AGENTS.md):
//   static const int Simulation::DOMAIN_ID;
//   static SubscriptionResults Simulation::mySubscriptionResults;
//   static ContextSubscriptionResults Simulation::myContextSubscriptionResults;

impl Simulation {
    /// C++: `std::pair<int, std::string> Simulation::init(int, int, const std::string &, const std::string &, FILE *const)` [static]
    // std::pair<int, std::string>
    // Simulation::init(int /* port */, int /* numRetries */, const std::string& /* host */, const std::string& /* label */, FILE* const /* pipe */) {
    //     throw TraCIException("Multi client support (including connection switching) is not implemented in libsumo.");
    // }
    #[allow(unused_variables)]
    pub fn init(port: i32, numRetries: i32, host: &str, label: &str, pipe: Ptr<Unmapped /* FILE */>) -> (i32, String) {
        todo!("port Simulation::init")
    }

    /// C++: `std::pair<int, std::string> Simulation::start(const std::vector<std::string> &, int, int, const std::string &, const bool, const std::string &, bool, void *)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn start(cmd: &Vec<String>, port: i32, numRetries: i32, label: &str, verbose: bool, traceFile: &str, traceGetters: bool, _stdout: Ptr<()>) -> (i32, String) {
        todo!("port Simulation::start")
    }

    /// C++: `bool Simulation::isLibsumo()` [static]
    // bool
    // Simulation::isLibsumo() {
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_libsumo() -> bool {
        todo!("port Simulation::isLibsumo")
    }

    /// C++: `void Simulation::switchConnection(const std::string &)` [static]
    // void
    // Simulation::switchConnection(const std::string& /* label */) {
    //     throw TraCIException("Multi client support (including connection switching) is not implemented in libsumo.");
    // }
    #[allow(unused_variables)]
    pub fn switch_connection(label: &str) {
        todo!("port Simulation::switchConnection")
    }

    /// C++: `const std::string & Simulation::getLabel()` [static]
    // const std::string&
    // Simulation::getLabel() {
    //     throw TraCIException("Multi client support (including connection switching) is not implemented in libsumo.");
    // }
    #[allow(unused_variables)]
    pub fn get_label() -> &'static str {
        todo!("port Simulation::getLabel")
    }

    /// C++: `void Simulation::setOrder(int)` [static]
    // void
    // Simulation::setOrder(int /* order */) {
    //     throw TraCIException("Multi client support (including connection switching) is not implemented in libsumo.");
    // }
    #[allow(unused_variables)]
    pub fn set_order(order: i32) {
        todo!("port Simulation::setOrder")
    }

    /// C++: `void Simulation::load(const std::vector<std::string> &)` [static]
    /// load a simulation with the given arguments
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn load(args: &Vec<String>) {
        todo!("port Simulation::load")
    }

    /// C++: `bool Simulation::hasGUI()` [static]
    /// whether we run with graphical user interface (sumo-gui)
    // bool
    // Simulation::hasGUI() {
    // #ifdef HAVE_LIBSUMOGUI
    //     return GUI::hasInstance();
    // #else
    //     return false;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn has_gui() -> bool {
        todo!("port Simulation::hasGUI")
    }

    /// C++: `bool Simulation::isLoaded()` [static]
    /// return whether a simulation (network) is present
    // bool
    // Simulation::isLoaded() {
    //     return MSNet::hasInstance();
    // }
    #[allow(unused_variables)]
    pub fn is_loaded() -> bool {
        todo!("port Simulation::isLoaded")
    }

    /// C++: `void Simulation::step(const double)` [static]
    /// Advances by one step (or up to the given time)
    // void
    // Simulation::step(const double time) {
    // #ifdef HAVE_FOX
    //     FXMutexLock lock(myStepMutex);
    // #endif
    //     Helper::clearStateChanges();
    //     const SUMOTime t = TIME2STEPS(time);
    // #ifdef HAVE_LIBSUMOGUI
    //     if (!GUI::step(t)) {
    // #endif
    //         if (t == 0) {
    //             MSNet::getInstance()->simulationStep();
    //         } else {
    //             while (SIMSTEP < t) {
    //                 MSNet::getInstance()->simulationStep();
    //             }
    //         }
    // #ifdef HAVE_LIBSUMOGUI
    //     }
    // #endif
    //     Helper::handleSubscriptions(SIMSTEP);
    // }
    #[allow(unused_variables)]
    pub fn step(time: f64) {
        todo!("port Simulation::step")
    }

    /// C++: `void Simulation::executeMove()` [static]
    /// Advances a "half" step
    // void
    // Simulation::executeMove() {
    //     MSNet::getInstance()->simulationStep(true);
    // }
    #[allow(unused_variables)]
    pub fn execute_move() {
        todo!("port Simulation::executeMove")
    }

    /// C++: `void Simulation::close(const std::string &)` [static]
    /// close simulation
    // void
    // Simulation::close(const std::string& reason) {
    //     Helper::clearSubscriptions();
    //     if (
    // #ifdef HAVE_LIBSUMOGUI
    //         !GUI::close(reason) &&
    // #endif
    //         MSNet::hasInstance()) {
    //         MSNet::getInstance()->closeSimulation(0, reason);
    //         delete MSNet::getInstance();
    //         SystemFrame::close();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close(reason: &str) {
        todo!("port Simulation::close")
    }

    /// C++: `std::pair<int, std::string> Simulation::getVersion()` [static]
    /// return TraCI API and SUMO version
    // std::pair<int, std::string>
    // Simulation::getVersion() {
    //     return std::make_pair(libsumo::TRACI_VERSION, "SUMO " VERSION_STRING);
    // }
    #[allow(unused_variables)]
    pub fn get_version() -> (i32, String) {
        todo!("port Simulation::getVersion")
    }

    /// C++: `std::string Simulation::getOption(const std::string &)` [static]
    /// return the SUMO option value
    // std::string
    // Simulation::getOption(const std::string& option) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (!oc.exists(option)) {
    //         throw TraCIException("The option " + option + " is unknown.");
    //     }
    //     return oc.getValueString(option);
    // }
    #[allow(unused_variables)]
    pub fn get_option(option: &str) -> String {
        todo!("port Simulation::getOption")
    }

    /// C++: `int Simulation::getCurrentTime()` [static]
    // int
    // Simulation::getCurrentTime() {
    //     return (int)MSNet::getInstance()->getCurrentTimeStep();
    // }
    #[allow(unused_variables)]
    pub fn get_current_time() -> i32 {
        todo!("port Simulation::getCurrentTime")
    }

    /// C++: `double Simulation::getTime()` [static]
    // double
    // Simulation::getTime() {
    //     return SIMTIME;
    // }
    #[allow(unused_variables)]
    pub fn get_time() -> f64 {
        todo!("port Simulation::getTime")
    }

    /// C++: `double Simulation::getEndTime()` [static]
    /// return configured end time
    // double
    // Simulation::getEndTime() {
    //     return STEPS2TIME(string2time(OptionsCont::getOptions().getString("end")));
    // }
    #[allow(unused_variables)]
    pub fn get_end_time() -> f64 {
        todo!("port Simulation::getEndTime")
    }

    /// C++: `int Simulation::getLoadedNumber()` [static]
    // int
    // Simulation::getLoadedNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::BUILT).size();
    // }
    #[allow(unused_variables)]
    pub fn get_loaded_number() -> i32 {
        todo!("port Simulation::getLoadedNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getLoadedIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_loaded_id_list() -> Vec<String> {
        todo!("port Simulation::getLoadedIDList")
    }

    /// C++: `int Simulation::getDepartedNumber()` [static]
    // int
    // Simulation::getDepartedNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::DEPARTED).size();
    // }
    #[allow(unused_variables)]
    pub fn get_departed_number() -> i32 {
        todo!("port Simulation::getDepartedNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getDepartedIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_departed_id_list() -> Vec<String> {
        todo!("port Simulation::getDepartedIDList")
    }

    /// C++: `int Simulation::getArrivedNumber()` [static]
    // int
    // Simulation::getArrivedNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::ARRIVED).size();
    // }
    #[allow(unused_variables)]
    pub fn get_arrived_number() -> i32 {
        todo!("port Simulation::getArrivedNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getArrivedIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_arrived_id_list() -> Vec<String> {
        todo!("port Simulation::getArrivedIDList")
    }

    /// C++: `int Simulation::getParkingStartingVehiclesNumber()` [static]
    // int
    // Simulation::getParkingStartingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::STARTING_PARKING).size();
    // }
    #[allow(unused_variables)]
    pub fn get_parking_starting_vehicles_number() -> i32 {
        todo!("port Simulation::getParkingStartingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getParkingStartingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_parking_starting_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getParkingStartingVehiclesIDList")
    }

    /// C++: `int Simulation::getParkingEndingVehiclesNumber()` [static]
    // int
    // Simulation::getParkingEndingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::ENDING_PARKING).size();
    // }
    #[allow(unused_variables)]
    pub fn get_parking_ending_vehicles_number() -> i32 {
        todo!("port Simulation::getParkingEndingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getParkingEndingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_parking_ending_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getParkingEndingVehiclesIDList")
    }

    /// C++: `int Simulation::getStopStartingVehiclesNumber()` [static]
    // int
    // Simulation::getStopStartingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::STARTING_STOP).size();
    // }
    #[allow(unused_variables)]
    pub fn get_stop_starting_vehicles_number() -> i32 {
        todo!("port Simulation::getStopStartingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getStopStartingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_starting_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getStopStartingVehiclesIDList")
    }

    /// C++: `int Simulation::getStopEndingVehiclesNumber()` [static]
    // int
    // Simulation::getStopEndingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::ENDING_STOP).size();
    // }
    #[allow(unused_variables)]
    pub fn get_stop_ending_vehicles_number() -> i32 {
        todo!("port Simulation::getStopEndingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getStopEndingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_ending_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getStopEndingVehiclesIDList")
    }

    /// C++: `int Simulation::getCollidingVehiclesNumber()` [static]
    // int
    // Simulation::getCollidingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::COLLISION).size();
    // }
    #[allow(unused_variables)]
    pub fn get_colliding_vehicles_number() -> i32 {
        todo!("port Simulation::getCollidingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getCollidingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_colliding_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getCollidingVehiclesIDList")
    }

    /// C++: `int Simulation::getEmergencyStoppingVehiclesNumber()` [static]
    // int
    // Simulation::getEmergencyStoppingVehiclesNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::EMERGENCYSTOP).size();
    // }
    #[allow(unused_variables)]
    pub fn get_emergency_stopping_vehicles_number() -> i32 {
        todo!("port Simulation::getEmergencyStoppingVehiclesNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getEmergencyStoppingVehiclesIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_emergency_stopping_vehicles_id_list() -> Vec<String> {
        todo!("port Simulation::getEmergencyStoppingVehiclesIDList")
    }

    /// C++: `int Simulation::getStartingTeleportNumber()` [static]
    // int
    // Simulation::getStartingTeleportNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::STARTING_TELEPORT).size();
    // }
    #[allow(unused_variables)]
    pub fn get_starting_teleport_number() -> i32 {
        todo!("port Simulation::getStartingTeleportNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getStartingTeleportIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_starting_teleport_id_list() -> Vec<String> {
        todo!("port Simulation::getStartingTeleportIDList")
    }

    /// C++: `int Simulation::getEndingTeleportNumber()` [static]
    // int
    // Simulation::getEndingTeleportNumber() {
    //     return (int)Helper::getVehicleStateChanges(MSNet::VehicleState::ENDING_TELEPORT).size();
    // }
    #[allow(unused_variables)]
    pub fn get_ending_teleport_number() -> i32 {
        todo!("port Simulation::getEndingTeleportNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getEndingTeleportIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_ending_teleport_id_list() -> Vec<String> {
        todo!("port Simulation::getEndingTeleportIDList")
    }

    /// C++: `int Simulation::getDepartedPersonNumber()` [static]
    // int
    // Simulation::getDepartedPersonNumber() {
    //     return (int)Helper::getTransportableStateChanges(MSNet::TransportableState::PERSON_DEPARTED).size();
    // }
    #[allow(unused_variables)]
    pub fn get_departed_person_number() -> i32 {
        todo!("port Simulation::getDepartedPersonNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getDepartedPersonIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_departed_person_id_list() -> Vec<String> {
        todo!("port Simulation::getDepartedPersonIDList")
    }

    /// C++: `int Simulation::getArrivedPersonNumber()` [static]
    // int
    // Simulation::getArrivedPersonNumber() {
    //     return (int)Helper::getTransportableStateChanges(MSNet::TransportableState::PERSON_ARRIVED).size();
    // }
    #[allow(unused_variables)]
    pub fn get_arrived_person_number() -> i32 {
        todo!("port Simulation::getArrivedPersonNumber")
    }

    /// C++: `std::vector<std::string> Simulation::getArrivedPersonIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_arrived_person_id_list() -> Vec<String> {
        todo!("port Simulation::getArrivedPersonIDList")
    }

    /// C++: `std::vector<std::string> Simulation::getBusStopIDList()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_bus_stop_id_list() -> Vec<String> {
        todo!("port Simulation::getBusStopIDList")
    }

    /// C++: `int Simulation::getBusStopWaiting(const std::string &)` [static]
    // int
    // Simulation::getBusStopWaiting(const std::string& stopID) {
    //     MSStoppingPlace* s = MSNet::getInstance()->getStoppingPlace(stopID, SUMO_TAG_BUS_STOP);
    //     if (s == nullptr) {
    //         throw TraCIException("Unknown bus stop '" + stopID + "'.");
    //     }
    //     return s->getTransportableNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_bus_stop_waiting(stopID: &str) -> i32 {
        todo!("port Simulation::getBusStopWaiting")
    }

    /// C++: `std::vector<std::string> Simulation::getBusStopWaitingIDList(const std::string &)` [static]
    /// Returns the IDs of the transportables on a given bus stop.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_bus_stop_waiting_id_list(stopID: &str) -> Vec<String> {
        todo!("port Simulation::getBusStopWaitingIDList")
    }

    /// C++: `std::vector<std::string> Simulation::getPendingVehicles()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_pending_vehicles() -> Vec<String> {
        todo!("port Simulation::getPendingVehicles")
    }

    /// C++: `std::vector<libsumo::TraCICollision> Simulation::getCollisions()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_collisions() -> Vec<TraCICollision> {
        todo!("port Simulation::getCollisions")
    }

    /// C++: `double Simulation::getScale()` [static]
    // double
    // Simulation::getScale() {
    //     return MSNet::getInstance()->getVehicleControl().getScale();
    // }
    #[allow(unused_variables)]
    pub fn get_scale() -> f64 {
        todo!("port Simulation::getScale")
    }

    /// C++: `double Simulation::getDeltaT()` [static]
    // double
    // Simulation::getDeltaT() {
    //     return TS;
    // }
    #[allow(unused_variables)]
    pub fn get_delta_t() -> f64 {
        todo!("port Simulation::getDeltaT")
    }

    /// C++: `libsumo::TraCIPositionVector Simulation::getNetBoundary()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_net_boundary() -> TraCIPositionVector {
        todo!("port Simulation::getNetBoundary")
    }

    /// C++: `int Simulation::getMinExpectedNumber()` [static]
    // int
    // Simulation::getMinExpectedNumber() {
    //     MSNet* net = MSNet::getInstance();
    //     return (net->getVehicleControl().getActiveVehicleCount()
    //             + net->getInsertionControl().getPendingFlowCount()
    //             + (net->hasPersons() ? net->getPersonControl().getActiveCount() : 0)
    //             + (net->hasContainers() ? net->getContainerControl().getActiveCount() : 0)
    //             + (MSDevice_Taxi::hasServableReservations() ? 1 : 0));
    // }
    #[allow(unused_variables)]
    pub fn get_min_expected_number() -> i32 {
        todo!("port Simulation::getMinExpectedNumber")
    }

    /// C++: `libsumo::TraCIPosition Simulation::convert2D(const std::string &, double, int, bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn convert2_d(edgeID: &str, pos: f64, laneIndex: i32, toGeo: bool) -> TraCIPosition {
        todo!("port Simulation::convert2D")
    }

    /// C++: `libsumo::TraCIPosition Simulation::convert3D(const std::string &, double, int, bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn convert3_d(edgeID: &str, pos: f64, laneIndex: i32, toGeo: bool) -> TraCIPosition {
        todo!("port Simulation::convert3D")
    }

    /// C++: `libsumo::TraCIRoadPosition Simulation::convertRoad(double, double, bool, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn convert_road(x: f64, y: f64, isGeo: bool, vClass: &str) -> TraCIRoadPosition {
        todo!("port Simulation::convertRoad")
    }

    /// C++: `libsumo::TraCIPosition Simulation::convertGeo(double, double, bool)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn convert_geo(x: f64, y: f64, fromGeo: bool) -> TraCIPosition {
        todo!("port Simulation::convertGeo")
    }

    /// C++: `double Simulation::getDistance2D(double, double, double, double, bool, bool)` [static]
    // double
    // Simulation::getDistance2D(double x1, double y1, double x2, double y2, bool isGeo, bool isDriving) {
    //     Position pos1(x1, y1);
    //     Position pos2(x2, y2);
    //     if (isGeo) {
    //         GeoConvHelper::getFinal().x2cartesian_const(pos1);
    //         GeoConvHelper::getFinal().x2cartesian_const(pos2);
    //     }
    //     if (isDriving) {
    //         std::pair<const MSLane*, double> roadPos1 = libsumo::Helper::convertCartesianToRoadMap(pos1, SVC_IGNORING);
    //         std::pair<const MSLane*, double> roadPos2 = libsumo::Helper::convertCartesianToRoadMap(pos2, SVC_IGNORING);
    //         return Helper::getDrivingDistance(roadPos1, roadPos2);
    //     } else {
    //         return pos1.distanceTo(pos2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_distance2_d(x1: f64, y1: f64, x2: f64, y2: f64, isGeo: bool, isDriving: bool) -> f64 {
        todo!("port Simulation::getDistance2D")
    }

    /// C++: `double Simulation::getDistanceRoad(const std::string &, double, const std::string &, double, bool)` [static]
    // double
    // Simulation::getDistanceRoad(const std::string& edgeID1, double pos1, const std::string& edgeID2, double pos2, bool isDriving) {
    //     std::pair<const MSLane*, double> roadPos1 = std::make_pair(libsumo::Helper::getLaneChecking(edgeID1, 0, pos1), pos1);
    //     std::pair<const MSLane*, double> roadPos2 = std::make_pair(libsumo::Helper::getLaneChecking(edgeID2, 0, pos2), pos2);
    //     if (isDriving) {
    //         return Helper::getDrivingDistance(roadPos1, roadPos2);
    //     } else {
    //         const Position p1 = roadPos1.first->geometryPositionAtOffset(roadPos1.second);
    //         const Position p2 = roadPos2.first->geometryPositionAtOffset(roadPos2.second);
    //         return p1.distanceTo(p2);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_distance_road(edgeID1: &str, pos1: f64, edgeID2: &str, pos2: f64, isDriving: bool) -> f64 {
        todo!("port Simulation::getDistanceRoad")
    }

    /// C++: `libsumo::TraCIStage Simulation::findRoute(const std::string &, const std::string &, const std::string &, double, int, double, double)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_route(fromEdge: &str, toEdge: &str, vType: &str, depart: f64, routingMode: i32, departPos: f64, arrivalPos: f64) -> TraCIStage {
        todo!("port Simulation::findRoute")
    }

    /// C++: `std::vector<libsumo::TraCIStage> Simulation::findIntermodalRoute(const std::string &, const std::string &, const std::string &, double, const int, double, double, double, double, const double, const std::string &, const std::string &, const std::string &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn find_intermodal_route(fromEdge: &str, toEdge: &str, modes: &str, depart: f64, routingMode: i32, speed: f64, walkFactor: f64, departPos: f64, arrivalPos: f64, departPosLat: f64, pType: &str, vType: &str, destStop: &str) -> Vec<TraCIStage> {
        todo!("port Simulation::findIntermodalRoute")
    }

    /// C++: `std::string Simulation::getParameter(const std::string &, const std::string &)` [static]
    // std::string
    // Simulation::getParameter(const std::string& objectID, const std::string& key) {
    //     if (StringUtils::startsWith(key, "chargingStation.")) {
    //         WRITE_WARNING("Retrieving chargingStation parameters via the simulation API is deprecated, please use traci.chargingstation.");
    //         const std::string attrName = key.substr(16);
    //         MSChargingStation* cs = static_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(objectID, SUMO_TAG_CHARGING_STATION));
    //         if (cs == nullptr) {
    //             throw TraCIException("Invalid chargingStation '" + objectID + "'");
    //         }
    //         if (attrName == toString(SUMO_ATTR_TOTALENERGYCHARGED)) {
    //             return toString(cs->getTotalCharged());
    //         } else if (attrName == toString(SUMO_ATTR_NAME)) {
    //             return toString(cs->getMyName());
    //         } else if (attrName == "lane") {
    //             return cs->getLane().getID();
    //         } else if (cs->hasParameter(attrName)) {
    //             return cs->getParameter(attrName);
    //         } else {
    //             throw TraCIException("Invalid chargingStation parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "overheadWire.")) {
    //         WRITE_WARNING("Retrieving overheadWire parameters via the simulation API is deprecated, please use traci.overheadwire.");
    //         const std::string attrName = key.substr(16);
    //         MSOverheadWire* cs = static_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(objectID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         if (cs == 0) {
    //             throw TraCIException("Invalid overhead wire '" + objectID + "'");
    //         }
    //         if (attrName == toString(SUMO_ATTR_TOTALENERGYCHARGED)) {
    //             return toString(cs->getTotalCharged());
    //         } else if (attrName == toString(SUMO_ATTR_NAME)) {
    //             return toString(cs->getMyName());
    //         } else {
    //             throw TraCIException("Invalid overhead wire parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "net.")) {
    //         const std::string attrName = key.substr(4);
    //         if (attrName == toString(SUMO_ATTR_NET_OFFSET)) {
    //             return toString(GeoConvHelper::getFinal().getOffsetBase());
    //         } else {
    //             throw TraCIException("Invalid net parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "stats.")) {
    //         if (objectID != "") {
    //             throw TraCIException("Simulation parameter '" + key + "' is not supported for object id '" + objectID + "'. Use empty id for stats");
    //         }
    //         const std::string attrName = key.substr(6);
    //         const MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //         const MSTransportableControl* pc = MSNet::getInstance()->hasPersons() ? &MSNet::getInstance()->getPersonControl() : nullptr;
    //         if (attrName == "vehicles.loaded") {
    //             return toString(vc.getLoadedVehicleNo());
    //         } else if (attrName == "vehicles.inserted") {
    //             return toString(vc.getDepartedVehicleNo());
    //         } else if (attrName == "vehicles.running") {
    //             return toString(vc.getRunningVehicleNo());
    //         } else if (attrName == "vehicles.waiting") {
    //             return toString(MSNet::getInstance()->getInsertionControl().getWaitingVehicleNo());
    //         } else if (attrName == "teleports.total") {
    //             return toString(vc.getTeleportCount());
    //         } else if (attrName == "teleports.jam") {
    //             return toString(vc.getTeleportsJam());
    //         } else if (attrName == "teleports.yield") {
    //             return toString(vc.getTeleportsYield());
    //         } else if (attrName == "teleports.wrongLane") {
    //             return toString(vc.getTeleportsWrongLane());
    //         } else if (attrName == "safety.collisions") {
    //             return toString(vc.getCollisionCount());
    //         } else if (attrName == "safety.emergencyStops") {
    //             return toString(vc.getEmergencyStops());
    //         } else if (attrName == "safety.emergencyBraking") {
    //             return toString(vc.getEmergencyBrakingCount());
    //         } else if (attrName == "persons.loaded") {
    //             return toString(pc != nullptr ? pc->getLoadedNumber() : 0);
    //         } else if (attrName == "persons.running") {
    //             return toString(pc != nullptr ? pc->getRunningNumber() : 0);
    //         } else if (attrName == "persons.jammed") {
    //             return toString(pc != nullptr ? pc->getJammedNumber() : 0);
    //         } else if (attrName == "personTeleports.total") {
    //             return toString(pc != nullptr ? pc->getTeleportCount() : 0);
    //         } else if (attrName == "personTeleports.abortWait") {
    //             return toString(pc != nullptr ? pc->getTeleportsAbortWait() : 0);
    //         } else if (attrName == "personTeleports.wrongDest") {
    //             return toString(pc != nullptr ? pc->getTeleportsWrongDest() : 0);
    //         } else {
    //             throw TraCIException("Invalid stats parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "parkingArea.")) {
    //         WRITE_WARNING("Retrieving parkingArea parameters via the simulation API is deprecated, please use traci.parkingarea.");
    //         const std::string attrName = key.substr(12);
    //         MSParkingArea* pa = static_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(objectID, SUMO_TAG_PARKING_AREA));
    //         if (pa == nullptr) {
    //             throw TraCIException("Invalid parkingArea '" + objectID + "'");
    //         }
    //         if (attrName == "capacity") {
    //             return toString(pa->getCapacity());
    //         } else if (attrName == "occupancy") {
    //             return toString(pa->getOccupancyIncludingBlocked());
    //         } else if (attrName == toString(SUMO_ATTR_NAME)) {
    //             return toString(pa->getMyName());
    //         } else if (attrName == "lane") {
    //             return pa->getLane().getID();
    //         } else if (pa->hasParameter(attrName)) {
    //             return pa->getParameter(attrName);
    //         } else {
    //             throw TraCIException("Invalid parkingArea parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "busStop.")) {
    //         WRITE_WARNING("Retrieving busStop parameters via the simulation API is deprecated, please use traci.busstop.");
    //         const std::string attrName = key.substr(8);
    //         MSStoppingPlace* bs = static_cast<MSStoppingPlace*>(MSNet::getInstance()->getStoppingPlace(objectID, SUMO_TAG_BUS_STOP));
    //         if (bs == nullptr) {
    //             throw TraCIException("Invalid busStop '" + objectID + "'");
    //         }
    //         if (attrName == toString(SUMO_ATTR_NAME)) {
    //             return toString(bs->getMyName());
    //         } else if (attrName == "lane") {
    //             return bs->getLane().getID();
    //         } else if (bs->hasParameter(attrName)) {
    //             return bs->getParameter(attrName);
    //         } else {
    //             throw TraCIException("Invalid busStop parameter '" + attrName + "'");
    //         }
    //     } else if (StringUtils::startsWith(key, "device.tripinfo.")) {
    //         if (objectID != "") {
    //             throw TraCIException("Simulation parameter '" + key + "' is not supported for object id '" + objectID
    //                                  + "'. Use empty id for global device parameters or vehicle domain for vehicle specific parameters");
    //         }
    //         const std::string attrName = key.substr(16);
    //         return MSDevice_Tripinfo::getGlobalParameter(attrName);
    //     } else if (StringUtils::startsWith(key, "buildConfig.")) {
    //         if (objectID != "") {
    //             throw TraCIException("Simulation parameter '" + key + "' is not supported for object id '" + objectID
    //                                  + "'. Use an empty id for buildConfig parameters");
    //         }
    //         if (key == "buildConfig.ARROW_SO_VERSION") {
    // #ifdef HAVE_PARQUET
    //             return ARROW_SO_VERSION;
    // #else
    //             return "";
    // #endif
    //         } else if (key == "buildConfig.JPS_VERSION") {
    // #ifdef HAVE_JUPEDSIM_LEGACY
    //             return toString(JPS_VERSION);
    // #else
    //             return "";
    // #endif
    //         } else if (key == "buildConfig.ENABLED") {
    //             return HAVE_ENABLED;
    //         }
    //         throw TraCIException("Unknown parameter '" + key + "'");
    //     } else if (objectID == "") {
    //         return MSNet::getInstance()->getParameter(key, "");
    //     } else {
    //         throw TraCIException("Simulation parameter '" + key + "' is not supported for object id '" + objectID + "'. Use empty id for generic network parameters");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(objectID: &str, key: &str) -> String {
        todo!("port Simulation::getParameter")
    }

    /// C++: `const std::pair<std::string, std::string> Simulation::getParameterWithKey(const std::string &, const std::string &)` [static]
    // LIBSUMO_GET_PARAMETER_WITH_KEY_IMPLEMENTATION(Simulation)
    #[allow(unused_variables)]
    pub fn get_parameter_with_key(objectID: &str, key: &str) -> (String, String) {
        todo!("port Simulation::getParameterWithKey")
    }

    /// C++: `void Simulation::setParameter(const std::string &, const std::string &, const std::string &)` [static]
    // void
    // Simulation::setParameter(const std::string& objectID, const std::string& key, const std::string& value) {
    //     if (objectID == "") {
    //         MSNet::getInstance()->setParameter(key, value);
    //     } else {
    //         throw TraCIException("Setting simulation parameter '" + key + "' is not supported for object id '" + objectID + "'. Use empty id for generic network parameters");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(objectID: &str, key: &str, value: &str) {
        todo!("port Simulation::setParameter")
    }

    /// C++: `void Simulation::setScale(double)` [static]
    // void
    // Simulation::setScale(double value) {
    //     MSNet::getInstance()->getVehicleControl().setScale(value);
    // }
    #[allow(unused_variables)]
    pub fn set_scale(value: f64) {
        todo!("port Simulation::setScale")
    }

    /// C++: `void Simulation::clearPending(const std::string &)` [static]
    // void
    // Simulation::clearPending(const std::string& routeID) {
    //     MSNet::getInstance()->getInsertionControl().clearPendingVehicles(routeID);
    // }
    #[allow(unused_variables)]
    pub fn clear_pending(routeID: &str) {
        todo!("port Simulation::clearPending")
    }

    /// C++: `void Simulation::saveState(const std::string &)` [static]
    // void
    // Simulation::saveState(const std::string& fileName) {
    //     MSStateHandler::saveState(fileName, MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn save_state(fileName: &str) {
        todo!("port Simulation::saveState")
    }

    /// C++: `double Simulation::loadState(const std::string &)` [static]
    /// quick-load simulation state from file and return the state time
    // double
    // Simulation::loadState(const std::string& fileName) {
    //     long before = PROGRESS_BEGIN_TIME_MESSAGE("Loading state from '" + fileName + "'");
    //     try {
    //         const SUMOTime newTime = MSNet::getInstance()->loadState(fileName, false);
    //         Helper::clearStateChanges();
    //         Helper::clearSubscriptions();
    //         PROGRESS_TIME_MESSAGE(before);
    //         return STEPS2TIME(newTime);
    //     } catch (const IOError& e) {
    //         throw TraCIException("Loading state from '" + fileName + "' failed. " + e.what());
    //     } catch (const ProcessError& e) {
    //         throw TraCIException("Loading state from '" + fileName + "' failed, check whether SUMO versions match. " + e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(fileName: &str) -> f64 {
        todo!("port Simulation::loadState")
    }

    /// C++: `void Simulation::writeMessage(const std::string &)` [static]
    // void
    // Simulation::writeMessage(const std::string& msg) {
    //     WRITE_MESSAGE(msg);
    // }
    #[allow(unused_variables)]
    pub fn write_message(msg: &str) {
        todo!("port Simulation::writeMessage")
    }

    /// C++: `void Simulation::subscribe(const std::string &, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe(objectID: &str, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Simulation::subscribe")
    }

    /// C++: `void Simulation::unsubscribe(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn unsubscribe(objectID: &str) {
        todo!("port Simulation::unsubscribe")
    }

    /// C++: `void Simulation::subscribeContext(const std::string &, int, double, const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_context(objectID: &str, domain: i32, dist: f64, varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Simulation::subscribeContext")
    }

    /// C++: `void Simulation::unsubscribeContext(const std::string &, int, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn unsubscribe_context(objectID: &str, domain: i32, dist: f64) {
        todo!("port Simulation::unsubscribeContext")
    }

    /// C++: `const libsumo::SubscriptionResults Simulation::getAllSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn get_all_subscription_results() -> SubscriptionResults {
        todo!("port Simulation::getAllSubscriptionResults")
    }

    /// C++: `const libsumo::TraCIResults Simulation::getSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn get_subscription_results(objectID: &str) -> TraCIResults {
        todo!("port Simulation::getSubscriptionResults")
    }

    /// C++: `const libsumo::ContextSubscriptionResults Simulation::getAllContextSubscriptionResults()` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn get_all_context_subscription_results() -> ContextSubscriptionResults {
        todo!("port Simulation::getAllContextSubscriptionResults")
    }

    /// C++: `const libsumo::SubscriptionResults Simulation::getContextSubscriptionResults(const std::string &)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn get_context_subscription_results(objectID: &str) -> SubscriptionResults {
        todo!("port Simulation::getContextSubscriptionResults")
    }

    /// C++: `void Simulation::subscribeParameterWithKey(const std::string &, const std::string &, double, double)` [static]
    // LIBSUMO_SUBSCRIPTION_IMPLEMENTATION(Simulation, SIM)
    #[allow(unused_variables)]
    pub fn subscribe_parameter_with_key(objectID: &str, key: &str, beginTime: f64, endTime: f64) {
        todo!("port Simulation::subscribeParameterWithKey")
    }

    /// C++: `int Simulation::domainID()` [static]
    // LIBSUMO_SUBSCRIPTION_API
    #[allow(unused_variables)]
    pub fn domain_id() -> i32 {
        todo!("port Simulation::domainID")
    }

    /// C++: `void Simulation::subscribe(const std::vector<int> &, double, double, const libsumo::TraCIResults &)` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn subscribe_2(varIDs: &Vec<i32>, begin: f64, end: f64, parameters: &TraCIResults) {
        todo!("port Simulation::subscribe")
    }

    /// C++: `const libsumo::TraCIResults Simulation::getSubscriptionResults()` [static]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_subscription_results_2() -> TraCIResults {
        todo!("port Simulation::getSubscriptionResults")
    }

    /// C++: `void Simulation::storeShape(PositionVector &)` [static]
    // void
    // Simulation::storeShape(PositionVector& shape) {
    //     shape = GeoConvHelper::getFinal().getConvBoundary().getShape(true);
    // }
    #[allow(unused_variables)]
    pub fn store_shape(shape: &mut PositionVector) {
        todo!("port Simulation::storeShape")
    }

    /// C++: `std::shared_ptr<VariableWrapper> Simulation::makeWrapper()` [static]
    // std::shared_ptr<VariableWrapper>
    // Simulation::makeWrapper() {
    //     return std::make_shared<Helper::SubscriptionWrapper>(handleVariable, mySubscriptionResults, myContextSubscriptionResults);
    // }
    #[allow(unused_variables)]
    pub fn make_wrapper() -> Rc<VariableWrapper> {
        todo!("port Simulation::makeWrapper")
    }

    /// C++: `bool Simulation::handleVariable(const std::string &, const int, VariableWrapper *, tcpip::Storage *)` [static]
    // bool
    // Simulation::handleVariable(const std::string& objID, const int variable, VariableWrapper* wrapper, tcpip::Storage* paramData) {
    //     switch (variable) {
    //         case VAR_TIME:
    //             return wrapper->wrapDouble(objID, variable, getTime());
    //         case VAR_TIME_STEP:
    //             return wrapper->wrapInt(objID, variable, (int)getCurrentTime());
    //         case VAR_END:
    //             return wrapper->wrapDouble(objID, variable, getEndTime());
    //         case VAR_LOADED_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getLoadedNumber());
    //         case VAR_LOADED_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getLoadedIDList());
    //         case VAR_DEPARTED_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getDepartedNumber());
    //         case VAR_DEPARTED_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getDepartedIDList());
    //         case VAR_TELEPORT_STARTING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getStartingTeleportNumber());
    //         case VAR_TELEPORT_STARTING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getStartingTeleportIDList());
    //         case VAR_TELEPORT_ENDING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getEndingTeleportNumber());
    //         case VAR_TELEPORT_ENDING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getEndingTeleportIDList());
    //         case VAR_ARRIVED_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getArrivedNumber());
    //         case VAR_ARRIVED_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getArrivedIDList());
    //         case VAR_PARKING_STARTING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getParkingStartingVehiclesNumber());
    //         case VAR_PARKING_STARTING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getParkingStartingVehiclesIDList());
    //         case VAR_PARKING_ENDING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getParkingEndingVehiclesNumber());
    //         case VAR_PARKING_ENDING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getParkingEndingVehiclesIDList());
    //         case VAR_STOP_STARTING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getStopStartingVehiclesNumber());
    //         case VAR_STOP_STARTING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getStopStartingVehiclesIDList());
    //         case VAR_STOP_ENDING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getStopEndingVehiclesNumber());
    //         case VAR_STOP_ENDING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getStopEndingVehiclesIDList());
    //         case VAR_COLLIDING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getCollidingVehiclesNumber());
    //         case VAR_COLLIDING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getCollidingVehiclesIDList());
    //         case VAR_EMERGENCYSTOPPING_VEHICLES_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getEmergencyStoppingVehiclesNumber());
    //         case VAR_EMERGENCYSTOPPING_VEHICLES_IDS:
    //             return wrapper->wrapStringList(objID, variable, getEmergencyStoppingVehiclesIDList());
    //         case VAR_DEPARTED_PERSONS_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getDepartedPersonNumber());
    //         case VAR_DEPARTED_PERSONS_IDS:
    //             return wrapper->wrapStringList(objID, variable, getDepartedPersonIDList());
    //         case VAR_ARRIVED_PERSONS_NUMBER:
    //             return wrapper->wrapInt(objID, variable, getArrivedPersonNumber());
    //         case VAR_ARRIVED_PERSONS_IDS:
    //             return wrapper->wrapStringList(objID, variable, getArrivedPersonIDList());
    //         case VAR_SCALE:
    //             return wrapper->wrapDouble(objID, variable, getScale());
    //         case VAR_DELTA_T:
    //             return wrapper->wrapDouble(objID, variable, getDeltaT());
    //         case VAR_OPTION:
    //             return wrapper->wrapString(objID, variable, getOption(objID));
    //         case VAR_MIN_EXPECTED_VEHICLES:
    //             return wrapper->wrapInt(objID, variable, getMinExpectedNumber());
    //         case VAR_BUS_STOP_ID_LIST:
    //             return wrapper->wrapStringList(objID, variable, getBusStopIDList());
    //         case VAR_BUS_STOP_WAITING:
    //             return wrapper->wrapInt(objID, variable, getBusStopWaiting(objID));
    //         case VAR_BUS_STOP_WAITING_IDS:
    //             return wrapper->wrapStringList(objID, variable, getBusStopWaitingIDList(objID));
    //         case VAR_PENDING_VEHICLES:
    //             return wrapper->wrapStringList(objID, variable, getPendingVehicles());
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
        todo!("port Simulation::handleVariable")
    }

    /// C++: `void Simulation::Simulation()` [private]
    /// invalidated standard constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port Simulation::Simulation")
    }

}

