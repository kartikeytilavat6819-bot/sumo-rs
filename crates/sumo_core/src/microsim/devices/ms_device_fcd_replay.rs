//! Port of microsim/devices/MSDevice_FCDReplay.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use utils::xml::generic_sax_handler::SaxHandler;
use crate::microsim::ms_event_control::Command;
use crate::prelude::*;
use crate::microsim::ms_router_defs::MSMapMatcher;
use crate::utils::map_matcher::{MapMatcher, MapMatcherHost};
use crate::utils::named_rtree::NamedRTree;
use utils::common::sumo_vehicle_class::{SVC_IGNORING, SVC_PASSENGER};

/// C++ typedef `std::vector<TrajectoryEntry> MSDevice_FCDReplay::Trajectory`
pub type MSDevice_FCDReplay_Trajectory = Vec<MSDevice_FCDReplay_TrajectoryEntry>;

/// C++ class `MSDevice_FCDReplay::FCDHandler` (microsim/devices/MSDevice_FCDReplay.h)
#[repr(C)]
pub struct MSDevice_FCDReplay_FCDHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// C++ base class `MapMatcher<MSEdge, MSLane, MSJunction>`
    pub base2: MSMapMatcher,
    pub myTime: SUMOTime,
    pub myTrajectories: BTreeMap<String, Box<MSDevice_FCDReplay_Trajectory>>,
    pub myRoutes: BTreeMap<String, (SUMOTime, String, bool, ConstMSEdgeVector, Vec<FCDHandler_StageStart>)>, // C++: std::map<std::string, std::tuple<SUMOTime, std::string, bool, ConstMSEdgeVector, std::vector<StageStart>>>
    pub myPositions: BTreeMap<Position, String>,
}

impl std::ops::Deref for MSDevice_FCDReplay_FCDHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_FCDReplay_FCDHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl MapMatcherHost<MSEdge> for MSDevice_FCDReplay_FCDHandler {
    fn init_lane_tree(&self, tree: Ptr<NamedRTree>) {
        MSDevice_FCDReplay_FCDHandler::init_lane_tree(self, tree)
    }
    fn retrieve_edge(&self, id: &str) -> Ptr<MSEdge> {
        MSDevice_FCDReplay_FCDHandler::retrieve_edge(self, id)
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
        MSDevice_FCDReplay_FCDHandler {
            base: utils::xml::sumo_sax_handler::new(file, ""),
            base2: MapMatcher::new(
                false,
                false,
                utils::options::options_cont::get_float("mapmatch.distance"),
                Ptr::from_raw(utils::common::msg_handler::with_instance(MsgType::Error, |h| h as *mut MsgHandler)),
            ),
            myTime: 0,
            myTrajectories: BTreeMap::new(),
            myRoutes: BTreeMap::new(),
            myPositions: BTreeMap::new(),
        }
    }

    /// C++: `SUMOTime FCDHandler::getTime() const` [const]
    // SUMOTime getTime() const {
    //             return myTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_time(&self) -> SUMOTime {
        self.myTime
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
        let ids: Vec<String> = self.myRoutes.keys().cloned().collect();
        for id in ids {
            let (depart, vType0, isPerson, routeEdges, stages) = self.myRoutes[&id].clone();
            let t: Ptr<MSDevice_FCDReplay_Trajectory> = Ptr::from_mut(&mut **self.myTrajectories.entry(id.clone()).or_default());
            if t[0].time >= intervalStart {
                // new vehicle or person
                let mut params = SUMOVehicleParameter::new();
                params.id = id.clone();
                params.depart = depart;
                params.departPos = t[0].lanePos;
                params.departSpeed = t[0].speed;
                // params->arrivalPos = t.back().lanePos;
                let mut vType = vType0.clone();
                if vType == "" {
                    vType = if isPerson { DEFAULT_PEDTYPE_ID } else { DEFAULT_VTYPE_ID }.to_string();
                }
                let vehicleType = MSNet::get_instance().m().get_vehicle_control().get_v_type(&vType, Ptr::null(), false);
                if vehicleType.is_null() {
                    panic!("Unknown vType '{}'.", vType);
                }
                if routeEdges[0].is_null() {
                    if isPerson {
                        write_warningf!("No edge in fcd replay file for person '%' at time %.", id, time2string(t[0].time));
                    } else {
                        write_warningf!("No lane in fcd replay file for vehicle '%' at time %.", id, time2string(t[0].time));
                    }
                    continue;
                }
                if isPerson {
                    let plan = self.make_plan(&params, &routeEdges, &stages, &*t);
                    let person = MSNet::get_instance().m().get_person_control().build_person(Ptr::from_mut(&mut params), vehicleType, plan, Ptr::null());
                    person.m().get_singular_type().set_v_class(SVC_IGNORING);
                    if !MSNet::get_instance().m().get_person_control().add(person) {
                        panic!("Duplicate person '{}'.", id);
                    }
                    let device = person.get_devices().iter().find(|d| d.device_name() == "fcd-replay").map(|d| d.cast::<MSTransportableDevice_FCDReplay>());
                    match device {
                        // shortcut: C++ deletes the person when it has no replay device; the port leaves it registered, upgrade when person deletion is ported
                        None => continue,
                        Some(device) => device.m().set_trajectory(t),
                    }
                } else {
                    let dummyRouteID = format!("DUMMY_ROUTE_{}", id);
                    let stops: StopParVector = Vec::new();
                    let route: ConstMSRoutePtr = Rc::new(MSRoute::new(&dummyRouteID, &routeEdges, true, Ptr::null(), &stops, -1, -1));
                    if !MSRoute::dictionary(&dummyRouteID, route.clone()) {
                        panic!("Could not add route '{}'.", dummyRouteID);
                    }
                    if t[0].edgeOrLane != "" {
                        params.departPosProcedure = DepartPosDefinition::GIVEN;
                        params.departLaneProcedure = DepartLaneDefinition::GIVEN;
                        params.departLane = utils::xml::sumo_xml_definitions::get_index_from_lane(&t[0].edgeOrLane).unwrap_or_else(|e| panic!("{}", e));
                    }
                    let vehicle = MSNet::get_instance().m().get_vehicle_control().build_vehicle(Ptr::from_mut(&mut params), route, vehicleType, false, MSVehicleControl_VehicleDefinitionSource::OTHER, true);
                    if !MSNet::get_instance().m().get_vehicle_control().add_vehicle(&id, vehicle) {
                        panic!("Duplicate vehicle '{}'.", id);
                    }
                    MSNet::get_instance().m().get_insertion_control().add(vehicle);
                    let device = fcd_replay_device(vehicle.get_devices());
                    if device.is_null() {
                        // Vehicle did not get a replay device
                        MSNet::get_instance().m().get_vehicle_control().delete_vehicle(vehicle, true, false);
                        continue;
                    }
                    device.m().set_trajectory(t);

                    // repair the route, cannot do this on parsing because a vehicle is needed
                    let mut checkedRoute = self.check_route(&routeEdges, vehicle);
                    if checkedRoute.len() != routeEdges.len() {
                        vehicle.m().replace_route_edges(&mut checkedRoute, -1., 0., "FCDReplay", true, false, true, Ptr::null());
                    }
                }
            } else if t[t.len() - 1].time >= intervalStart {
                // new data for existing person / vehicle
                if isPerson {
                    let person = MSNet::get_instance().m().get_person_control().get(&id);
                    if person.is_null() {
                        // TODO this should not happen
                        continue;
                    }
                    // TODO optimize: no need to regenerate the whole plan
                    let plan = self.make_plan(&person.get_parameter(), &routeEdges, &stages, &*t);
                    let stageIndex = person.get_num_remaining_stages() - 1;
                    let finalStage = person.get_next_stage(stageIndex);
                    let mut append = false;
                    for stage in plan.iter() {
                        let stage = *stage;
                        if stage.get_stage_type() == finalStage.get_stage_type() && stage.get_from_edge() == finalStage.get_from_edge() {
                            // TODO: circular plans?
                            append = true;
                        }
                        if append {
                            person.m().append_stage(stage, 0);
                        }
                        // else: C++ deletes the stage here; the port leaves it allocated (shortcut: stage ownership is not ported)
                    }
                    person.m().remove_stage(stageIndex, false);
                    // C++ `delete plan`: the plan vector is not freed here (shortcut: plan ownership is not ported)
                } else {
                    let vehicle = MSNet::get_instance().m().get_vehicle_control().get_vehicle(&id);
                    let mut checkedRoute = self.check_route(&routeEdges, vehicle);
                    if checkedRoute.len() as i32 != vehicle.get_route().size() {
                        vehicle.m().replace_route_edges(&mut checkedRoute, -1., 0., "FCDReplay", true, false, true, Ptr::null());
                    }
                }
            }
        }
    }

    /// C++: `const std::map<std::string, Trajectory> & FCDHandler::getTrajectories()`
    // const std::map<std::string, Trajectory>& getTrajectories() {
    //             return myTrajectories;
    //         }
    #[allow(unused_variables)]
    pub fn get_trajectories(&mut self) -> &BTreeMap<String, Box<MSDevice_FCDReplay_Trajectory>> {
        &self.myTrajectories
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
        let mut ok = true;
        match element {
            SUMO_TAG_TIMESTEP => {
                self.myTime = attrs.get_sumo_time_reporting(SUMO_ATTR_TIME, "", &mut ok, true);
                self.myPositions.clear();
                return;
            }
            SUMO_TAG_VEHICLE | SUMO_TAG_PERSON => {
                if self.myTime >= MSNet::get_instance().get_current_time_step() {
                    let isPerson = element == SUMO_TAG_PERSON;
                    let id = attrs.get_string(SUMO_ATTR_ID);
                    let xy = Position::new(fcd_opt_double(attrs, SUMO_ATTR_X, &id, &mut ok), fcd_opt_double(attrs, SUMO_ATTR_Y, &id, &mut ok));
                    let type_ = if fcd_used(SUMO_ATTR_TYPE) { attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true) } else { String::new() };
                    let mut edgeOrLane = String::new();
                    if fcd_used(SUMO_ATTR_EDGE) || fcd_used(SUMO_ATTR_LANE) {
                        edgeOrLane = attrs.get_opt::<String>(if isPerson { SUMO_ATTR_EDGE } else { SUMO_ATTR_LANE }, &id, &mut ok, String::new(), true);
                    }
                    let speed = fcd_opt_double(attrs, SUMO_ATTR_SPEED, &id, &mut ok);
                    let pos = fcd_opt_double(attrs, SUMO_ATTR_POSITION, &id, &mut ok);
                    let angle = fcd_opt_double(attrs, SUMO_ATTR_ANGLE, &id, &mut ok);
                    let mut vehicle = attrs.get_opt::<String>(SUMO_ATTR_VEHICLE, &id, &mut ok, String::new(), true);
                    if isPerson {
                        if vehicle == "" {
                            if let Some(veh) = self.myPositions.get(&xy) {
                                vehicle = veh.clone();
                            }
                        }
                    } else {
                        self.myPositions.insert(xy, id.clone());
                    }
                    if !ok {
                        write_warningf!("Invalid  FCD data.");
                        return;
                    }
                    let entry = MSDevice_FCDReplay_TrajectoryEntry { time: self.myTime, pos: xy, edgeOrLane: edgeOrLane.clone(), lanePos: pos, speed, angle };
                    self.myTrajectories.entry(id.clone()).or_default().push(entry);
                    let edge: Ptr<MSEdge> = MSEdge::dictionary_2(&if isPerson { edgeOrLane.clone() } else { utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&edgeOrLane) });
                    if edge.is_null() && edgeOrLane != "" {
                        if isPerson {
                            write_warningf!("Unknown edge '%' in fcd replay file for person '%'.", edgeOrLane, id);
                        } else {
                            write_warningf!("Unknown lane '%' in fcd replay file for vehicle '%'.", edgeOrLane, id);
                        }
                    }
                    if !self.myRoutes.contains_key(&id) {
                        let mut edge = edge;
                        if edgeOrLane == "" {
                            let lane = self.base2.get_closest_lane(&*self, &xy, SVC_PASSENGER, -1.0);
                            if !lane.is_null() {
                                let laneID = lane.m().base.get_id().to_string();
                                let lanePos = lane.m().interpolate_geometry_pos_to_lane_pos(lane.m().get_shape(false).nearest_offset_to_point25d(&xy, false));
                                let trajectory = self.myTrajectories.get_mut(&id).unwrap();
                                trajectory.last_mut().unwrap().edgeOrLane = laneID;
                                trajectory.last_mut().unwrap().lanePos = lanePos;
                                edge = Ptr::from_mut(lane.m().get_edge());
                                while edge.m().is_internal() {
                                    // we cannot use an internal edge for a route
                                    edge = edge.m().get_successors(SVC_IGNORING)[0];
                                }
                            }
                        }
                        self.myRoutes.insert(id.clone(), (self.myTime, type_, isPerson, vec![edge], Vec::new()));
                    } else {
                        let route = &mut self.myRoutes.get_mut(&id).unwrap().3;
                        if !edge.is_null() && !edge.is_internal() && edge != *route.last().unwrap() {
                            route.push(edge);
                        }
                    }
                    let trajLen = self.myTrajectories[&id].len() as i32 - 1;
                    let routeLen = self.myRoutes[&id].3.len() as i32 - 1;
                    let vehicleUsage = &mut self.myRoutes.get_mut(&id).unwrap().4;
                    if (vehicleUsage.is_empty() && vehicle != "") || (!vehicleUsage.is_empty() && vehicle != vehicleUsage.last().unwrap().vehicle) {
                        vehicleUsage.push(FCDHandler_StageStart { vehicle: vehicle.clone(), trajectoryOffset: trajLen, routeOffset: routeLen });
                    }
                }
                return;
            }
            _ => {}
        }
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
    pub fn init_lane_tree(&self, tree: Ptr<NamedRTree>) {
        for edge in MSEdge::get_all_edges().iter() {
            for lane in edge.m().get_lanes().iter() {
                let b = lane.m().get_shape(false).get_box_boundary();
                let cmin = [b.xmin() as f32, b.ymin() as f32];
                let cmax = [b.xmax() as f32, b.ymax() as f32];
                tree.m().insert(&cmin, &cmax, &Ptr::new(&lane.base));
            }
        }
    }

    /// C++: `MSEdge * FCDHandler::retrieveEdge(const std::string &)` [virtual, protected]
    // MSEdge*
    // MSDevice_FCDReplay::FCDHandler::retrieveEdge(const std::string& id) {
    //     return MSEdge::dictionary(id);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_edge(&self, id: &str) -> Ptr<MSEdge> {
        MSEdge::dictionary_2(id)
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
    pub fn make_plan(&mut self, params: &SUMOVehicleParameter, route: &ConstMSEdgeVector, stages: &Vec<FCDHandler_StageStart>, t: &MSDevice_FCDReplay_Trajectory) -> Ptr<MSTransportable_MSTransportablePlan> {
        // shortcut: stages are heap-leaked like the C++ new, plan ownership is not ported; upgrade when MSTransportable owns its plan
        let mut plan: Vec<Ptr<MSStage>> = Vec::new();
        let w: &mut MSStageWaiting = Box::leak(Box::new(MSStageWaiting::new(route[0], Ptr::null(), 0, params.depart, params.departPos, "awaiting departure", true, -1)));
        plan.push(Ptr::from_mut(&mut w.base));
        let mut prevRouteOffset: i32 = 0;
        let mut start: Ptr<MSEdge> = route[0];
        let mut prevVeh = String::new();
        for stageStart in stages {
            if stageStart.vehicle != prevVeh {
                if stageStart.trajectoryOffset != 0 {
                    let prevEdge = MSEdge::dictionary_2(&t[(stageStart.trajectoryOffset - 1) as usize].edgeOrLane);
                    if prevVeh == "" {
                        let mut finalStop: Ptr<MSStoppingPlace> = Ptr::null();
                        if prevRouteOffset < route.len() as i32 - 1 && (route[prevRouteOffset as usize].get_permissions() & SVC_PEDESTRIAN) == 0 {
                            prevRouteOffset += 1; // skip the access
                        }
                        let mut offset = stageStart.routeOffset;
                        if offset < route.len() as i32 - 1 && (route[offset as usize].get_permissions() & SVC_PEDESTRIAN) == SVC_PEDESTRIAN {
                            offset += 1; // a bus stop or two consecutive walks so include the edge in both
                        } else {
                            // may have an access, let's find the stop
                            let stop = MSNet::get_instance().get_stopping_place_id(route[offset as usize].get_lanes()[0], t[stageStart.trajectoryOffset as usize].lanePos, SUMO_TAG_BUS_STOP);
                            if stop != "" {
                                finalStop = MSNet::get_instance().get_stopping_place(&stop, SUMO_TAG_BUS_STOP);
                            }
                        }
                        let subRoute: ConstMSEdgeVector = route[prevRouteOffset as usize..offset as usize].to_vec();
                        let s: &mut MSStageWalking = Box::leak(Box::new(MSStageWalking::new(&params.id, &subRoute, finalStop, -1, params.departSpeed, params.departPos, 0., 0., -1, "")));
                        plan.push(Ptr::from_mut(&mut s.base.base));
                    } else {
                        let s: &mut MSStageDriving = Box::leak(Box::new(MSStageDriving::new(start, prevEdge, Ptr::null(), -1., 0., &vec![prevVeh.clone()], "", "", 0)));
                        plan.push(Ptr::from_mut(&mut s.base));
                    }
                    start = MSEdge::dictionary_2(&t[stageStart.trajectoryOffset as usize].edgeOrLane);
                    prevVeh = stageStart.vehicle.clone();
                    prevRouteOffset = stageStart.routeOffset;
                }
            }
        }
        // final stage
        if prevVeh == "" {
            if prevRouteOffset < route.len() as i32 - 1 && (route[prevRouteOffset as usize].get_permissions() & SVC_PEDESTRIAN) == 0 {
                prevRouteOffset += 1;
            }
            let mut offset = route.len() as i32 - 1;
            if (route[offset as usize].get_permissions() & SVC_PEDESTRIAN) == SVC_PEDESTRIAN {
                offset += 1;
            }
            if prevRouteOffset < offset {
                // otherwise we may be still in a vehicle or in access, skip the stage for now
                let subRoute: ConstMSEdgeVector = route[prevRouteOffset as usize..offset as usize].to_vec();
                let s: &mut MSStageWalking = Box::leak(Box::new(MSStageWalking::new(&params.id, &subRoute, Ptr::null(), -1, params.departSpeed, params.departPos, 0., 0., -1, "")));
                plan.push(Ptr::from_mut(&mut s.base.base));
            }
        } else {
            let s: &mut MSStageDriving = Box::leak(Box::new(MSStageDriving::new(start, *route.last().unwrap(), Ptr::null(), -1., 0., &vec![prevVeh.clone()], "", "", 0)));
            plan.push(Ptr::from_mut(&mut s.base));
        }
        Ptr::from_raw(Box::into_raw(Box::new(plan)))
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
        let mut checkedRoute: ConstMSEdgeVector = Vec::new();
        for e in edges {
            if checkedRoute.is_empty() || checkedRoute.last().unwrap().is_connected_to(&**e, vehicle.get_vehicle_type().get_vehicle_class(), false) {
                checkedRoute.push(*e);
            } else {
                let fromEdge = *checkedRoute.last().unwrap();
                checkedRoute.pop();
                if !MSNet::get_instance().get_router_tt(0, &Default::default()).compute(fromEdge, *e, vehicle, self.myTime, &mut checkedRoute, false) {
                    // TODO maybe warn about disconnected route
                    checkedRoute.push(fromEdge);
                    checkedRoute.push(*e);
                }
                // TODO check whether we introduced a big detour
            }
        }
        checkedRoute
    }

}

/// A device which replays a vehicle trajectory from an fcd file
/// C++ class `MSDevice_FCDReplay` (microsim/devices/MSDevice_FCDReplay.h)
#[repr(C)]
pub struct MSDevice_FCDReplay {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    pub myTrajectory: Ptr<MSDevice_FCDReplay_Trajectory>,
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

thread_local! {
    /// C++ `static std::vector<std::unique_ptr<FCDHandler>> MSDevice_FCDReplay::myHandlers`
    static MY_HANDLERS: std::cell::RefCell<Vec<Box<MSDevice_FCDReplay_FCDHandler>>> = std::cell::RefCell::new(Vec::new());
    /// C++ `static std::vector<std::unique_ptr<SUMOSAXReader>> MSDevice_FCDReplay::myParsers` (None is nullptr)
    static MY_PARSERS: std::cell::RefCell<Vec<Option<utils::xml::sumo_sax_reader::SUMOSAXReader>>> = std::cell::RefCell::new(Vec::new());
    /// C++ `static SumoXMLAttrMask MSDevice_FCDReplay::myUsedAttributes` (set of SumoXMLAttr ids)
    static MY_USED_ATTRIBUTES: std::cell::RefCell<BTreeSet<i32>> = std::cell::RefCell::new(BTreeSet::new());
}

/// C++ `myUsedAttributes.test(attr)`
fn fcd_used(attr: i32) -> bool {
    MY_USED_ATTRIBUTES.with(|m| m.borrow().contains(&attr))
}

/// C++ `getOptDouble` lambda of myStartElement
fn fcd_opt_double(attrs: &SUMOSAXAttributes, attr: i32, id: &str, ok: &mut bool) -> f64 {
    if fcd_used(attr) {
        attrs.get_opt::<f64>(attr, id, ok, INVALID_DOUBLE_VALUE, true)
    } else {
        INVALID_DOUBLE_VALUE
    }
}

/// C++ `StaticCommand<MSDevice_FCDReplay>(&MSDevice_FCDReplay::parseNext)` scheduled by init()
struct MSDevice_FCDReplay_ParseNextCommand;

impl Command for MSDevice_FCDReplay_ParseNextCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSDevice_FCDReplay::parse_next(currentTime)
    }
}

impl Command for MSDevice_FCDReplay_MoveVehicles {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSDevice_FCDReplay_MoveVehicles::execute(self, currentTime)
    }
}

/// C++ `getDevice(typeid(MSDevice_FCDReplay))`
fn fcd_replay_device(devices: &Vec<Ptr<MSVehicleDevice>>) -> Ptr<MSDevice_FCDReplay> {
    MSDevice::get_device_by_name(devices, "fcd-replay").cast::<MSDevice_FCDReplay>()
}

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
        oc.add_option_sub_topic("FCD Replay Device");
        MSDevice::insert_default_assignment_options("fcd-replay", "FCD Replay Device", oc, false);

        oc.do_register("device.fcd-replay.files", SumoOption::file_name());
        oc.add_synonyme("device.fcd-replay.files", "device.fcd-replay.file", false);
        oc.add_description("device.fcd-replay.files", "FCD Replay Device", "FCD files to read");
        oc.do_register("device.fcd-replay.consistency", SumoOption::string_vector_with(vec!["all".to_string()]));
        oc.add_description("device.fcd-replay.consistency", "FCD Replay Device", "Values to use when replaying ('all' or any combination of 'x', 'y', 'edge', 'lane', 'speed', 'position', 'angle', 'type')");
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
        let filesSet = utils::options::options_cont::is_set("device.fcd-replay.files");
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "fcd-replay", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), filesSet, false)
        });
        if equipped {
            // shortcut: the device is leaked like the C++ new, so the vehicle's device list keeps a stable pointer; upgrade when MSVehicle owns its devices as Box
            let device: &mut MSDevice_FCDReplay = Box::leak(Box::new(MSDevice_FCDReplay::new(v, &format!("fcdReplay_{}", v.get_id()))));
            into.push(Ptr::from_mut(&mut device.base));
        }
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
        let consistency = utils::options::options_cont::get_string_vector("device.fcd-replay.consistency");
        MY_USED_ATTRIBUTES.with(|m| m.borrow_mut().clear());
        for attr in consistency.iter() {
            let mut set = |a: i32| MY_USED_ATTRIBUTES.with(|m| m.borrow_mut().insert(a));
            if attr == "x" || attr == "all" {
                set(SUMO_ATTR_X);
            }
            if attr == "y" || attr == "all" {
                set(SUMO_ATTR_Y);
            }
            if attr == "edge" || attr == "all" {
                set(SUMO_ATTR_EDGE);
            }
            if attr == "lane" || attr == "all" {
                set(SUMO_ATTR_LANE);
            }
            if attr == "speed" || attr == "all" {
                set(SUMO_ATTR_SPEED);
            }
            if attr == "position" || attr == "pos" || attr == "all" {
                set(SUMO_ATTR_POSITION);
            }
            if attr == "angle" || attr == "all" {
                set(SUMO_ATTR_ANGLE);
            }
            if attr == "type" || attr == "all" {
                set(SUMO_ATTR_TYPE);
            }
        }
        MY_HANDLERS.with(|h| h.borrow_mut().clear());
        MY_PARSERS.with(|p| p.borrow_mut().clear());
        if utils::options::options_cont::is_set("device.fcd-replay.files") {
            for filename in utils::options::options_cont::get_string_vector("device.fcd-replay.files").iter() {
                // C++ keeps the handler in myHandlers before parseFirst; the port parses first and stores both once they are ready
                let mut handler = MSDevice_FCDReplay_FCDHandler::new(filename);
                let mut reader = utils::xml::xml_sub_sys::get_sax_reader(false, false);
                if !reader.parse_first(&mut handler, filename).unwrap_or_else(|e| panic!("{}", e)) {
                    panic!("Can not read XML-file '{}'.", filename);
                }
                MY_HANDLERS.with(|h| h.borrow_mut().push(Box::new(handler)));
                MY_PARSERS.with(|p| p.borrow_mut().push(Some(reader)));
                let inc = MSDevice_FCDReplay::parse_next(MSNet::get_instance().get_current_time_step());
                let moveCmd: Box<MSDevice_FCDReplay_MoveVehicles> = Box::new(MSDevice_FCDReplay_MoveVehicles { base: () });
                MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(Box::into_raw(moveCmd) as *mut dyn Command), MSNet::get_instance().get_current_time_step() + utils::common::time::delta_t());
                if inc > 0 {
                    let parseCmd: Box<MSDevice_FCDReplay_ParseNextCommand> = Box::new(MSDevice_FCDReplay_ParseNextCommand);
                    MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(Box::into_raw(parseCmd) as *mut dyn Command), MSNet::get_instance().get_current_time_step() + inc);
                }
            }
        }
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
        let inc = utils::common::time::string2time(&utils::options::options_cont::get_string("route-steps")).unwrap_or_else(|e| panic!("{}", e));
        let mut haveData = false;
        MY_HANDLERS.with(|hs| {
            let mut hs = hs.borrow_mut();
            MY_PARSERS.with(|ps| {
                let mut ps = ps.borrow_mut();
                for i in 0..hs.len() {
                    if ps[i].is_some() {
                        haveData = true;
                        // make sure that we have always at least inc time steps buffered, so at time 200 we will parse 400 to 600
                        let start = hs[i].get_time();
                        while hs[i].get_time() < t + 2 * inc {
                            let more = ps[i].as_mut().unwrap().parse_next(&mut *hs[i]).unwrap_or_else(|e| panic!("{}", e));
                            if !more {
                                ps[i] = None;
                                break;
                            }
                        }
                        hs[i].update_traffic_objects(start);
                    }
                }
            });
        });
        if haveData { inc } else { 0 }
    }

    /// C++: `void MSDevice_FCDReplay::~MSDevice_FCDReplay() noexcept` [virtual]
    /// Destructor.
    // MSDevice_FCDReplay::~MSDevice_FCDReplay() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
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
        let holderID = self.get_holder().get_id().to_string();
        if self.myTrajectory.is_null() {
            let mut found: Ptr<MSDevice_FCDReplay_Trajectory> = Ptr::null();
            MY_HANDLERS.with(|hs| {
                for handler in hs.borrow_mut().iter_mut() {
                    let trajs = handler.get_trajectories();
                    if let Some(t) = trajs.get(&holderID) {
                        found = Ptr::new(&**t);
                        break;
                    }
                }
            });
            if found.is_null() {
                return;
            }
            self.set_trajectory(found);
        } else if self.myTrajectoryIndex == self.myTrajectory.len() as i32 {
            // removal happens via the usual MSVehicle::hasArrived mechanism
            // TODO we may need to set an arrivalPos
            return;
        }
        let v = Ptr::from_mut(self.get_holder()).cast::<MSVehicle>();
        let te = self.myTrajectory[self.myTrajectoryIndex as usize].clone();
        if v.is_null() || te.time > currentTime {
            return;
        }
        if te.edgeOrLane != "" {
            let edgeID = utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&te.edgeOrLane);
            let laneIdx = utils::xml::sumo_xml_definitions::get_index_from_lane(&te.edgeOrLane).unwrap_or_else(|e| panic!("{}", e));
            crate::libsumo::vehicle::Vehicle::move_to_xy(&holderID, &edgeID, laneIdx, te.pos.x(), te.pos.y(), te.angle, 7, -1.);
        } else if te.pos.x() != INVALID_DOUBLE_VALUE {
            crate::libsumo::vehicle::Vehicle::move_to_xy(&holderID, "", -1, te.pos.x(), te.pos.y(), te.angle, 0, -1.);
        }
        v.m().get_influencer().set_speed_mode(0);
        crate::libsumo::vehicle::Vehicle::set_speed(&holderID, te.speed);
        // libsumo::Vehicle::changeLane(myHolder.getID(), laneIdx, TS);
        self.myTrajectoryIndex += 1;
    }

    /// C++: `const std::string MSDevice_FCDReplay::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const override {
    //         return "fcd-replay";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "fcd-replay".to_string()
    }

    /// C++: `void MSDevice_FCDReplay::setTrajectory(const Trajectory *const)`
    // void setTrajectory(const Trajectory* const t) {
    //         myTrajectory = t;
    //         myTrajectoryIndex = 1;
    //     }
    #[allow(unused_variables)]
    pub fn set_trajectory(&mut self, t: Ptr<MSDevice_FCDReplay_Trajectory>) {
        self.myTrajectory = t;
        self.myTrajectoryIndex = 1;
    }

    /// C++: `void MSDevice_FCDReplay::MSDevice_FCDReplay(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_FCDReplay::MSDevice_FCDReplay(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        let mut this = Self::new_device(holder, id);
        // C++ deviceName() is "fcd-replay", which the id prefix "fcdReplay_" does not give
        this.base.base2.name = "fcd-replay".to_string();
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str) -> Self {
        MSDevice_FCDReplay {
            base: MSVehicleDevice::new(holder, id),
            myTrajectory: Ptr::null(),
            myTrajectoryIndex: 0,
        }
    }

    /// C++: `void MSDevice_FCDReplay::MSDevice_FCDReplay(const MSDevice_FCDReplay &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_FCDReplay) -> Self {
        panic!("MSDevice_FCDReplay copy constructor is private and invalid")
    }

    /// C++: `MSDevice_FCDReplay & MSDevice_FCDReplay::operator=(const MSDevice_FCDReplay &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCDReplay) -> &mut MSDevice_FCDReplay {
        panic!("MSDevice_FCDReplay assignment operator is private and invalid")
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
        let net = MSNet::get_instance();
        let c = net.m().get_vehicle_control();
        for (_, veh) in c.myVehicleDict.iter() {
            let device = fcd_replay_device(veh.get_devices());
            if !device.is_null() && veh.cast::<MSBaseVehicle>().has_departed() {
                device.m().move_(currentTime);
            }
        }
        utils::common::time::delta_t()
    }

    /// C++: `MoveVehicles & MoveVehicles::operator=(const MoveVehicles &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_FCDReplay_MoveVehicles) -> &mut MSDevice_FCDReplay_MoveVehicles {
        panic!("MoveVehicles assignment operator is private and invalid")
    }

}

/// C++ class `FCDHandler::StageStart` (microsim/devices/MSDevice_FCDReplay.h)
#[derive(Clone)]
pub struct FCDHandler_StageStart {
    pub vehicle: String,
    pub trajectoryOffset: i32,
    pub routeOffset: i32,
}

impl FCDHandler_StageStart {
}

impl SaxHandler for MSDevice_FCDReplay_FCDHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler { &mut self.base }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        MSDevice_FCDReplay_FCDHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}

/// C++ class `MSDevice_FCDReplay::TrajectoryEntry` (microsim/devices/MSDevice_FCDReplay.h)
#[derive(Clone)]
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


