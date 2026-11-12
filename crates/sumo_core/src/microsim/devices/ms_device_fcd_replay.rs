//! Port of microsim/devices/MSDevice_FCDReplay.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<TrajectoryEntry> MSDevice_FCDReplay::Trajectory`
pub type MSDevice_FCDReplay_Trajectory = Vec<MSDevice_FCDReplay_TrajectoryEntry>;

/// C++ class `MSDevice_FCDReplay::FCDHandler` (microsim/devices/MSDevice_FCDReplay.h)
#[repr(C)]
pub struct MSDevice_FCDReplay_FCDHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// C++ base class `MapMatcher<MSEdge, MSLane, MSJunction>`
    pub base2: Unmapped /* MapMatcher<MSEdge, MSLane, MSJunction> */,
    pub myTime: SUMOTime,
    pub myTrajectories: BTreeMap<String, Encounter_Trajectory>,
    pub myRoutes: BTreeMap<String, Unmapped /* std::tuple<SUMOTime, std::string, bool, ConstMSEdgeVector, std::vector<StageStart>> */>, // C++: std::map<std::string, std::tuple<SUMOTime, std::string, bool, ConstMSEdgeVector, std::vector<StageStart>>>
    pub myPositions: BTreeMap<Position, String>,
}

impl std::ops::Deref for MSDevice_FCDReplay_FCDHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_FCDReplay_FCDHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

impl MSDevice_FCDReplay_FCDHandler {
    /// C++: `void FCDHandler::FCDHandler(const std::string &)`
    // MSDevice_FCDReplay::FCDHandler::FCDHandler(const std::string& file) :
    //     SUMOSAXHandler(file),
    //     MapMatcher(false, false,
    //                OptionsCont::getOptions().getFloat("mapmatch.distance"),
    //                MsgHandler::getErrorInstance()) {}
    #[allow(unused_variables)]
    pub fn new(file: &str) -> Self {
        todo!("port FCDHandler::FCDHandler")
    }

    /// C++: `SUMOTime FCDHandler::getTime() const` [const]
    // SUMOTime getTime() const {
    //             return myTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_time(&self) -> SUMOTime {
        todo!("port FCDHandler::getTime")
    }

    /// C++: `void FCDHandler::updateTrafficObjects(const SUMOTime)`
    // void
    // MSDevice_FCDReplay::FCDHandler::updateTrafficObjects(const SUMOTime intervalStart) {
    //     for (const auto& desc : myRoutes) {
    //         const std::string& id = desc.first;
    //         const bool isPerson = std::get<2>(desc.second);
    //         const ConstMSEdgeVector& routeEdges = std::get<3>(desc.second);
    //         Trajectory& t = myTrajectories[id];
    //         if (t.front().time >= intervalStart) {
    //             // new vehicle or person
    //             SUMOVehicleParameter* params = new SUMOVehicleParameter();
    //             params->id = id;
    //             params->depart = std::get<0>(desc.second);
    //             params->departPos = t.front().lanePos;
    //             params->departSpeed = t.front().speed;
    //             // params->arrivalPos = t.back().lanePos;
    //             std::string vType = std::get<1>(desc.second);
    //             if (vType == "") {
    //                 vType = isPerson ? DEFAULT_PEDTYPE_ID : DEFAULT_VTYPE_ID;
    //             }
    //             MSVehicleType* vehicleType = MSNet::getInstance()->getVehicleControl().getVType(vType);
    //             if (vehicleType == nullptr) {
    //                 delete params;
    //                 throw ProcessError(TLF("Unknown vType '%'.", vType));
    //             }
    //             if (routeEdges.front() == nullptr) {
    //                 if (isPerson) {
    //                     WRITE_WARNINGF(TL("No edge in fcd replay file for person '%' at time %."), id, time2string(t.front().time));
    //                 } else {
    //                     WRITE_WARNINGF(TL("No lane in fcd replay file for vehicle '%' at time %."), id, time2string(t.front().time));
    //                 }
    //                 delete params;
    //                 continue;
    //             }
    //             if (isPerson) {
    //                 MSTransportable::MSTransportablePlan* plan = makePlan(*params, routeEdges, std::get<4>(desc.second), t);
    //                 // plan completed, now build the person
    //                 MSTransportable* person = MSNet::getInstance()->getPersonControl().buildPerson(params, vehicleType, plan, nullptr);
    //                 person->getSingularType().setVClass(SVC_IGNORING);
    //                 if (!MSNet::getInstance()->getPersonControl().add(person)) {
    //                     delete person;
    //                     throw ProcessError(TLF("Duplicate person '%'.", id));
    //                 }
    //                 MSTransportableDevice_FCDReplay* device = static_cast<MSTransportableDevice_FCDReplay*>(person->getDevice(typeid(MSTransportableDevice_FCDReplay)));
    //                 if (device == nullptr) {  // Person did not get a replay device
    //                     delete person;
    //                     continue;
    //                 }
    //                 device->setTrajectory(&t);
    //             } else {
    //                 const std::string dummyRouteID = "DUMMY_ROUTE_" + id;
    //                 const StopParVector stops;
    //                 ConstMSRoutePtr route = std::make_shared<MSRoute>(dummyRouteID, routeEdges, true, nullptr, stops);
    //                 if (!MSRoute::dictionary(dummyRouteID, route)) {
    //                     throw ProcessError(TLF("Could not add route '%'.", dummyRouteID));
    //                 }
    //                 if (t.front().edgeOrLane != "") {
    //                     params->departPosProcedure = DepartPosDefinition::GIVEN;
    //                     params->departLaneProcedure = DepartLaneDefinition::GIVEN;
    //                     params->departLane = SUMOXMLDefinitions::getIndexFromLane(t.front().edgeOrLane);
    //                 }
    //                 SUMOVehicle* vehicle = MSNet::getInstance()->getVehicleControl().buildVehicle(params, route, vehicleType, false, MSVehicleControl::VehicleDefinitionSource::OTHER);
    //                 if (!MSNet::getInstance()->getVehicleControl().addVehicle(id, vehicle)) {
    //                     throw ProcessError(TLF("Duplicate vehicle '%'.", id));
    //                 }
    //                 MSNet::getInstance()->getInsertionControl().add(vehicle);
    //                 MSDevice_FCDReplay* device = static_cast<MSDevice_FCDReplay*>(vehicle->getDevice(typeid(MSDevice_FCDReplay)));
    //                 if (device == nullptr) {  // Vehicle did not get a replay device
    //                     MSNet::getInstance()->getVehicleControl().deleteVehicle(vehicle, true);
    //                     continue;
    //                 }
    //                 device->setTrajectory(&t);
    //
    //                 // repair the route, cannot do this on parsing because a vehicle is needed
    //                 ConstMSEdgeVector checkedRoute = checkRoute(routeEdges, vehicle);
    //                 if (checkedRoute.size() != routeEdges.size()) {
    //                     vehicle->replaceRouteEdges(checkedRoute, -1, 0, "FCDReplay", true);
    //                 }
    //             }
    //         } else if (t.back().time >= intervalStart) {
    //             // new data for existing person / vehicle
    //             if (isPerson) {
    //                 MSTransportable* person = MSNet::getInstance()->getPersonControl().get(id);
    //                 if (person == nullptr) {
    //                     // TODO this should not happen
    //                     continue;
    //                 }
    //                 // TODO optimize: no need to regenerate the whole plan
    //                 MSTransportable::MSTransportablePlan* plan = makePlan(person->getParameter(), routeEdges, std::get<4>(desc.second), t);
    //                 const int stageIndex = person->getNumRemainingStages() - 1;
    //                 const MSStage* const finalStage = person->getNextStage(stageIndex);
    //                 bool append = false;
    //                 for (MSStage* const stage : *plan) {
    //                     if (stage->getStageType() == finalStage->getStageType() && stage->getFromEdge() == finalStage->getFromEdge()) {
    //                         // TODO: circular plans?
    //                         append = true;
    //                     }
    //                     if (append) {
    //                         person->appendStage(stage);
    //                     } else {
    //                         delete stage;
    //                     }
    //                 }
    //                 person->removeStage(stageIndex);
    //                 delete plan;
    //             } else {
    //                 SUMOVehicle* vehicle = MSNet::getInstance()->getVehicleControl().getVehicle(id);
    //                 ConstMSEdgeVector checkedRoute = checkRoute(routeEdges, vehicle);
    //                 if ((int)checkedRoute.size() != vehicle->getRoute().size()) {
    //                     vehicle->replaceRouteEdges(checkedRoute, -1, 0, "FCDReplay", true);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_traffic_objects(&mut self, intervalStart: SUMOTime) {
        todo!("port FCDHandler::updateTrafficObjects")
    }

    /// C++: `const std::map<std::string, Trajectory> & FCDHandler::getTrajectories()`
    // const std::map<std::string, Trajectory>& getTrajectories() {
    //             return myTrajectories;
    //         }
    #[allow(unused_variables)]
    pub fn get_trajectories(&mut self) -> &BTreeMap<String, Encounter_Trajectory> {
        todo!("port FCDHandler::getTrajectories")
    }

    /// C++: `void FCDHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag
    // void
    // MSDevice_FCDReplay::FCDHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     switch (element) {
    //         case SUMO_TAG_TIMESTEP:
    //             myTime = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME, "", ok);
    //             myPositions.clear();
    //             return;
    //         case SUMO_TAG_VEHICLE:
    //         case SUMO_TAG_PERSON:
    //             if (myTime >= SIMSTEP) {
    //                 const bool isPerson = element == SUMO_TAG_PERSON;
    //                 const std::string id = attrs.getString(SUMO_ATTR_ID);
    //                 auto getOptDouble = [&](const SumoXMLAttr attr) {
    //                     return myUsedAttributes.test(attr) ? attrs.getOpt<double>(attr, id.c_str(), ok, libsumo::INVALID_DOUBLE_VALUE) : libsumo::INVALID_DOUBLE_VALUE;
    //                 };
    //                 const Position xy = Position(getOptDouble(SUMO_ATTR_X), getOptDouble(SUMO_ATTR_Y));
    //                 const std::string type = myUsedAttributes.test(SUMO_ATTR_TYPE) ? attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "") : "";
    //                 std::string edgeOrLane;
    //                 if (myUsedAttributes.test(SUMO_ATTR_EDGE) || myUsedAttributes.test(SUMO_ATTR_LANE)) {
    //                     edgeOrLane = attrs.getOpt<std::string>(isPerson ? SUMO_ATTR_EDGE : SUMO_ATTR_LANE, id.c_str(), ok, "");
    //                 }
    //                 const double speed = getOptDouble(SUMO_ATTR_SPEED);
    //                 const double pos = getOptDouble(SUMO_ATTR_POSITION);
    //                 const double angle = getOptDouble(SUMO_ATTR_ANGLE);
    //                 std::string vehicle = attrs.getOpt<std::string>(SUMO_ATTR_VEHICLE, id.c_str(), ok, "");
    //                 if (isPerson) {
    //                     if (vehicle == "") {
    //                         const auto& veh = myPositions.find(xy);
    //                         if (veh != myPositions.end()) {
    //                             vehicle = veh->second;
    //                         }
    //                     }
    //                 } else {
    //                     myPositions[xy] = id;
    //                 }
    //                 if (!ok) {
    //                     WRITE_WARNING("Invalid  FCD data.");
    //                     return;
    //                 }
    //                 myTrajectories[id].push_back({myTime, xy, edgeOrLane, pos, speed, angle});
    //                 const MSEdge* edge = MSEdge::dictionary(isPerson ? edgeOrLane : SUMOXMLDefinitions::getEdgeIDFromLane(edgeOrLane));
    //                 if (edge == nullptr && edgeOrLane != "") {
    //                     WRITE_WARNINGF(isPerson ? TL("Unknown edge '%' in fcd replay file for person '%'.") : TL("Unknown lane '%' in fcd replay file for vehicle '%'."), edgeOrLane, id);
    //                 }
    //                 if (myRoutes.count(id) == 0) {
    //                     if (edgeOrLane == "") {
    //                         const MSLane* const lane = getClosestLane(xy, SVC_PASSENGER);
    //                         if (lane != nullptr) {
    //                             myTrajectories[id].back().edgeOrLane = lane->getID();
    //                             myTrajectories[id].back().lanePos = lane->interpolateGeometryPosToLanePos(
    //                                                                     lane->getShape().nearest_offset_to_point25D(xy, false));
    //                             edge = &lane->getEdge();
    //                             while (edge->isInternal()) {  // we cannot use an internal edge for a route
    //                                 edge = edge->getSuccessors().front();
    //                             }
    //                         }
    //                     }
    //                     myRoutes[id] = std::make_tuple(myTime, type, isPerson, ConstMSEdgeVector{edge}, std::vector<StageStart>());
    //                 } else {
    //                     ConstMSEdgeVector& route = std::get<3>(myRoutes[id]);
    //                     if (edge != nullptr && !edge->isInternal() && edge != route.back()) {
    //                         route.push_back(edge);
    //                     }
    //                 }
    //                 std::vector<StageStart>& vehicleUsage = std::get<4>(myRoutes[id]);
    //                 if ((vehicleUsage.empty() && vehicle != "") || (!vehicleUsage.empty() && vehicle != vehicleUsage.back().vehicle)) {
    //                     vehicleUsage.push_back({vehicle, (int)myTrajectories[id].size() - 1, (int)std::get<3>(myRoutes[id]).size() - 1});
    //                 }
    //             }
    //             return;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        todo!("port FCDHandler::myStartElement")
    }

    /// C++: `void FCDHandler::initLaneTree(NamedRTree *)` [virtual, protected]
    // void
    // MSDevice_FCDReplay::FCDHandler::initLaneTree(NamedRTree* tree) {
    //     for (const auto& edge : MSEdge::getAllEdges()) {
    //         for (MSLane* lane : edge->getLanes()) {
    //             Boundary b = lane->getShape().getBoxBoundary();
    //             const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //             const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //             tree->Insert(cmin, cmax, lane);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_lane_tree(&mut self, tree: Ptr<Unmapped /* NamedRTree */>) {
        todo!("port FCDHandler::initLaneTree")
    }

    /// C++: `MSEdge * FCDHandler::retrieveEdge(const std::string &)` [virtual, protected]
    // MSEdge*
    // MSDevice_FCDReplay::FCDHandler::retrieveEdge(const std::string& id) {
    //     return MSEdge::dictionary(id);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_edge(&mut self, id: &str) -> Ptr<MSEdge> {
        todo!("port FCDHandler::retrieveEdge")
    }

    /// C++: `MSTransportable::MSTransportablePlan * FCDHandler::makePlan(const SUMOVehicleParameter &, const ConstMSEdgeVector &, const std::vector<StageStart> &, const Trajectory &)` [private]
    // MSTransportable::MSTransportablePlan*
    // MSDevice_FCDReplay::FCDHandler::makePlan(const SUMOVehicleParameter& params, const ConstMSEdgeVector& route,
    //         const std::vector<StageStart>& stages, const Trajectory& t) {
    //     MSTransportable::MSTransportablePlan* plan = new MSTransportable::MSTransportablePlan();
    //     plan->push_back(new MSStageWaiting(route.front(), nullptr, 0, params.depart, params.departPos, "awaiting departure", true));
    //     int prevRouteOffset = 0;
    //     const MSEdge* start = route.front();
    //     std::string prevVeh;
    //     for (const auto& stageStart : stages) {
    //         if (stageStart.vehicle != prevVeh) {
    //             if (stageStart.trajectoryOffset != 0) {
    //                 const MSEdge* prevEdge = MSEdge::dictionary(t[stageStart.trajectoryOffset - 1].edgeOrLane);
    //                 if (prevVeh == "") {
    //                     MSStoppingPlace* finalStop = nullptr;
    //                     if (prevRouteOffset < (int)route.size() - 1 && (route[prevRouteOffset]->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //                         prevRouteOffset++;  // skip the access
    //                     }
    //                     int offset = stageStart.routeOffset;
    //                     if (offset < (int)route.size() - 1 && (route[offset]->getPermissions() & SVC_PEDESTRIAN) == SVC_PEDESTRIAN) {
    //                         offset++;  // a bus stop or two consecutive walks so include the edge in both
    //                     } else {
    //                         // may have an access, let's find the stop
    //                         const std::string& stop = MSNet::getInstance()->getStoppingPlaceID(route[offset]->getLanes()[0], t[stageStart.trajectoryOffset].lanePos, SUMO_TAG_BUS_STOP);
    //                         if (stop != "") {
    //                             finalStop = MSNet::getInstance()->getStoppingPlace(stop, SUMO_TAG_BUS_STOP);
    //                         }
    //                     }
    //                     ConstMSEdgeVector subRoute = ConstMSEdgeVector(route.begin() + prevRouteOffset, route.begin() + offset);
    //                     plan->push_back(new MSStageWalking(params.id, subRoute, finalStop, -1, params.departSpeed, params.departPos, 0, 0));
    //                 } else {
    //                     plan->push_back(new MSStageDriving(start, prevEdge, nullptr, -1, 0, {prevVeh}));
    //                 }
    //                 start = MSEdge::dictionary(t[stageStart.trajectoryOffset].edgeOrLane);
    //                 prevVeh = stageStart.vehicle;
    //                 prevRouteOffset = stageStart.routeOffset;
    //             }
    //         }
    //     }
    //     // final stage
    //     if (prevVeh == "") {
    //         if (prevRouteOffset < (int)route.size() - 1 && (route[prevRouteOffset]->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //             prevRouteOffset++;
    //         }
    //         int offset = (int)route.size() - 1;
    //         if ((route[offset]->getPermissions() & SVC_PEDESTRIAN) == SVC_PEDESTRIAN) {
    //             offset++;
    //         }
    //         if (prevRouteOffset < offset) {
    //             // otherwise we may be still in a vehicle or in access, skip the stage for now
    //             ConstMSEdgeVector subRoute = ConstMSEdgeVector(route.begin() + prevRouteOffset, route.begin() + offset);
    //             plan->push_back(new MSStageWalking(params.id, subRoute, nullptr, -1, params.departSpeed, params.departPos, 0, 0));
    //         }
    //     } else {
    //         plan->push_back(new MSStageDriving(start, route.back(), nullptr, -1, 0, {prevVeh}));
    //     }
    //     return plan;
    // }
    #[allow(unused_variables)]
    pub fn make_plan(&mut self, params: &SUMOVehicleParameter, route: &ConstMSEdgeVector, stages: &Vec<FCDHandler_StageStart>, t: &Encounter_Trajectory) -> Ptr<MSTransportable_MSTransportablePlan> {
        todo!("port FCDHandler::makePlan")
    }

    /// C++: `ConstMSEdgeVector FCDHandler::checkRoute(const ConstMSEdgeVector &, const SUMOVehicle *const)` [private]
    // ConstMSEdgeVector
    // MSDevice_FCDReplay::FCDHandler::checkRoute(const ConstMSEdgeVector& edges, const SUMOVehicle* const vehicle) {
    //     ConstMSEdgeVector checkedRoute;
    //     for (const MSEdge* const e : edges) {
    //         if (checkedRoute.empty() || checkedRoute.back()->isConnectedTo(*e, vehicle->getVehicleType().getVehicleClass())) {
    //             checkedRoute.push_back(e);
    //         } else {
    //             const MSEdge* fromEdge = checkedRoute.back();
    //             checkedRoute.pop_back();
    //             if (!MSNet::getInstance()->getRouterTT(0).compute(fromEdge, e, vehicle, myTime, checkedRoute)) {
    //                 // TODO maybe warn about disconnected route
    //                 checkedRoute.push_back(fromEdge);
    //                 checkedRoute.push_back(e);
    //             }
    //             // TODO check whether we introduced a big detour
    //         }
    //     }
    //     return checkedRoute;
    // }
    #[allow(unused_variables)]
    pub fn check_route(&mut self, edges: &ConstMSEdgeVector, vehicle: Ptr<SUMOVehicle>) -> ConstMSEdgeVector {
        todo!("port FCDHandler::checkRoute")
    }

}

/// A device which replays a vehicle trajectory from an fcd file
/// C++ class `MSDevice_FCDReplay` (microsim/devices/MSDevice_FCDReplay.h)
#[repr(C)]
pub struct MSDevice_FCDReplay {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    pub myTrajectory: Ptr<Encounter_Trajectory>,
    pub myTrajectoryIndex: i32,
}

impl std::ops::Deref for MSDevice_FCDReplay {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_FCDReplay {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_FCDReplay (port as module-level statics, see AGENTS.md):
//   static std::vector<std::unique_ptr<FCDHandler>> MSDevice_FCDReplay::myHandlers;
//   static std::vector<std::unique_ptr<SUMOSAXReader>> MSDevice_FCDReplay::myParsers;
//   static SumoXMLAttrMask MSDevice_FCDReplay::myUsedAttributes;

impl MSDevice_FCDReplay {
    /// C++: `void MSDevice_FCDReplay::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_FCDReplay-options
    // void
    // MSDevice_FCDReplay::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("FCD Replay Device");
    //     insertDefaultAssignmentOptions("fcd-replay", "FCD Replay Device", oc);
    //
    //     oc.doRegister("device.fcd-replay.files", new Option_FileName());
    //     oc.addSynonyme("device.fcd-replay.files", "device.fcd-replay.file");
    //     oc.addDescription("device.fcd-replay.files", "FCD Replay Device", TL("FCD files to read"));
    //     oc.doRegister("device.fcd-replay.consistency", new Option_StringVector(StringVector({ "all" })));
    //     oc.addDescription("device.fcd-replay.consistency", "FCD Replay Device", TL("Values to use when replaying ('all' or any combination of 'x', 'y', 'edge', 'lane', 'speed', 'position', 'angle', 'type')"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_FCDReplay::insertOptions")
    }

    /// C++: `void MSDevice_FCDReplay::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_FCDReplay::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "fcd-replay", v, oc.isSet("device.fcd-replay.files"))) {
    //         MSDevice_FCDReplay* device = new MSDevice_FCDReplay(v, "fcdReplay_" + v.getID());
    //         into.push_back(device);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_FCDReplay::buildVehicleDevices")
    }

    /// C++: `void MSDevice_FCDReplay::init()` [static]
    /// Static intialization
    // void
    // MSDevice_FCDReplay::init() {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     myUsedAttributes.reset();
    //     for (const std::string& attr : oc.getStringVector("device.fcd-replay.consistency")) {
    //         if (attr == "x" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_X);
    //         }
    //         if (attr == "y" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_Y);
    //         }
    //         if (attr == "edge" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_EDGE);
    //         }
    //         if (attr == "lane" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_LANE);
    //         }
    //         if (attr == "speed" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_SPEED);
    //         }
    //         if (attr == "position" || attr == "pos" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_POSITION);
    //         }
    //         if (attr == "angle" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_ANGLE);
    //         }
    //         if (attr == "type" || attr == "all") {
    //             myUsedAttributes.set(SUMO_ATTR_TYPE);
    //         }
    //     }
    //     myHandlers.clear();
    //     if (oc.isSet("device.fcd-replay.files")) {
    //         for (const std::string& filename : oc.getStringVector("device.fcd-replay.files")) {
    //             myHandlers.push_back(std::unique_ptr<MSDevice_FCDReplay::FCDHandler>(new MSDevice_FCDReplay::FCDHandler(filename)));
    //             myParsers.push_back(std::unique_ptr<SUMOSAXReader>(XMLSubSys::getSAXReader(*myHandlers.back())));
    //             if (!myParsers.back()->parseFirst(filename)) {
    //                 throw ProcessError(TLF("Can not read XML-file '%'.", filename));
    //             }
    //             const SUMOTime inc = parseNext(SIMSTEP);
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(new MoveVehicles(), SIMSTEP + DELTA_T);
    //             if (inc > 0) {
    //                 MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(new StaticCommand<MSDevice_FCDReplay>(&MSDevice_FCDReplay::parseNext),
    //                         SIMSTEP + inc);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init() {
        todo!("port MSDevice_FCDReplay::init")
    }

    /// C++: `SUMOTime MSDevice_FCDReplay::parseNext(SUMOTime)` [static]
    // SUMOTime
    // MSDevice_FCDReplay::parseNext(SUMOTime t) {
    //     SUMOTime inc = string2time(OptionsCont::getOptions().getString("route-steps"));
    //     bool haveData = false;
    //     for (int i = 0; i < (int)myHandlers.size(); i++) {
    //         if (myParsers[i] != nullptr) {
    //             haveData = true;
    //             // make sure that we have always at least inc time steps buffered, so at time 200 we will parse 400 to 600
    //             const SUMOTime start = myHandlers[i]->getTime();
    //             while (myHandlers[i]->getTime() < t + 2 * inc) {
    //                 if (!myParsers[i]->parseNext()) {
    //                     myParsers[i] = nullptr;
    //                     break;
    //                 }
    //             }
    //             myHandlers[i]->updateTrafficObjects(start);
    //         }
    //     }
    //     return haveData ? inc : 0;
    // }
    #[allow(unused_variables)]
    pub fn parse_next(t: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_FCDReplay::parseNext")
    }

    /// C++: `void MSDevice_FCDReplay::~MSDevice_FCDReplay() noexcept` [virtual]
    /// Destructor.
    // MSDevice_FCDReplay::~MSDevice_FCDReplay() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_FCDReplay::~MSDevice_FCDReplay")
    }

    /// C++: `void MSDevice_FCDReplay::move(SUMOTime)`
    // void
    // MSDevice_FCDReplay::move(SUMOTime currentTime) {
    //     if (myTrajectory == nullptr) {
    //         for (auto& handler : myHandlers) {
    //             auto it = handler->getTrajectories().find(myHolder.getID());
    //             if (it != handler->getTrajectories().end()) {
    //                 setTrajectory(&it->second);
    //                 break;
    //             }
    //         }
    //         if (myTrajectory == nullptr) {
    //             return;
    //         }
    //     } else if (myTrajectoryIndex == (int)myTrajectory->size()) {
    //         // removal happens via the usual MSVehicle::hasArrived mechanism
    //         // TODO we may need to set an arrivalPos
    //         return;
    //     }
    //     MSVehicle* v = dynamic_cast<MSVehicle*>(&myHolder);
    //     const TrajectoryEntry& te = myTrajectory->at(myTrajectoryIndex);
    //     if (v == nullptr || te.time > currentTime) {
    //         return;
    //     }
    //     if (te.edgeOrLane != "") {
    //         const std::string& edgeID = SUMOXMLDefinitions::getEdgeIDFromLane(te.edgeOrLane);
    //         const int laneIdx = SUMOXMLDefinitions::getIndexFromLane(te.edgeOrLane);
    //         libsumo::Vehicle::moveToXY(myHolder.getID(), edgeID, laneIdx, te.pos.x(), te.pos.y(), te.angle, 7);
    //     } else if (te.pos.x() != libsumo::INVALID_DOUBLE_VALUE) {
    //         libsumo::Vehicle::moveToXY(myHolder.getID(), "", -1, te.pos.x(), te.pos.y(), te.angle, 0);
    //     }
    //     v->getInfluencer().setSpeedMode(0);
    //     libsumo::Vehicle::setSpeed(myHolder.getID(), te.speed);
    //     // libsumo::Vehicle::changeLane(myHolder.getID(), laneIdx, TS);
    //     myTrajectoryIndex++;
    // }
    #[allow(unused_variables)]
    pub fn move_(&mut self, currentTime: SUMOTime) {
        todo!("port MSDevice_FCDReplay::move")
    }

    /// C++: `const std::string MSDevice_FCDReplay::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "fcd-replay";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_FCDReplay::deviceName")
    }

    /// C++: `void MSDevice_FCDReplay::setTrajectory(const Trajectory *const)`
    // void setTrajectory(const Trajectory* const t) {
    //         myTrajectory = t;
    //         myTrajectoryIndex = 1;
    //     }
    #[allow(unused_variables)]
    pub fn set_trajectory(&mut self, t: Ptr<Encounter_Trajectory>) {
        todo!("port MSDevice_FCDReplay::setTrajectory")
    }

    /// C++: `void MSDevice_FCDReplay::MSDevice_FCDReplay(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_FCDReplay::MSDevice_FCDReplay(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSDevice_FCDReplay::MSDevice_FCDReplay")
    }

    /// C++: `void MSDevice_FCDReplay::MSDevice_FCDReplay(const MSDevice_FCDReplay &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_FCDReplay) -> Self {
        todo!("port MSDevice_FCDReplay::MSDevice_FCDReplay")
    }

    /// C++: `MSDevice_FCDReplay & MSDevice_FCDReplay::operator=(const MSDevice_FCDReplay &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCDReplay) -> &mut MSDevice_FCDReplay {
        todo!("port MSDevice_FCDReplay::operator=")
    }

}

/// C++ class `MSDevice_FCDReplay::MoveVehicles` (microsim/devices/MSDevice_FCDReplay.h)
#[repr(C)]
pub struct MSDevice_FCDReplay_MoveVehicles {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
}

impl std::ops::Deref for MSDevice_FCDReplay_MoveVehicles {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_FCDReplay_MoveVehicles {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSDevice_FCDReplay_MoveVehicles {
    /// C++: `SUMOTime MoveVehicles::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSDevice_FCDReplay::MoveVehicles::execute(SUMOTime currentTime) {
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt i = c.loadedVehBegin(); i != c.loadedVehEnd(); ++i) {
    //         MSDevice_FCDReplay* device = static_cast<MSDevice_FCDReplay*>(i->second->getDevice(typeid(MSDevice_FCDReplay)));
    //         if (device != nullptr && i->second->hasDeparted()) {
    //             device->move(currentTime);
    //         }
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MoveVehicles::execute")
    }

    /// C++: `MoveVehicles & MoveVehicles::operator=(const MoveVehicles &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCDReplay_MoveVehicles) -> &mut MSDevice_FCDReplay_MoveVehicles {
        todo!("port MoveVehicles::operator=")
    }

}

/// C++ class `FCDHandler::StageStart` (microsim/devices/MSDevice_FCDReplay.h)
pub struct FCDHandler_StageStart {
    pub vehicle: String,
    pub trajectoryOffset: i32,
    pub routeOffset: i32,
}

impl FCDHandler_StageStart {
}

/// C++ class `MSDevice_FCDReplay::TrajectoryEntry` (microsim/devices/MSDevice_FCDReplay.h)
pub struct MSDevice_FCDReplay_TrajectoryEntry {
    pub time: SUMOTime,
    pub pos: Position,
    pub edgeOrLane: String,
    pub lanePos: f64,
    pub speed: f64,
    pub angle: f64,
}

impl MSDevice_FCDReplay_TrajectoryEntry {
}

