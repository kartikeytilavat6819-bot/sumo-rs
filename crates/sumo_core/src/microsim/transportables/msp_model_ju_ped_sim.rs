//! Port of microsim/transportables/MSPModel_JuPedSim.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSPModel_JuPedSim::JPS_Model`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSPModel_JuPedSim_JPS_Model {
    CollisionFreeSpeed = 0,
    CollisionFreeSpeedV2 = 1,
    GeneralizedCentrifugalForce = 2,
    SocialForce = 3,
}

/// C++ typedef `int64_t MSPModel_JuPedSim::JPS_AgentId`
pub type MSPModel_JuPedSim_JPS_AgentId = i64;

/// C++ typedef `int64_t MSPModel_JuPedSim::JPS_JourneyId`
pub type MSPModel_JuPedSim_JPS_JourneyId = i64;

/// C++ typedef `int64_t MSPModel_JuPedSim::JPS_StageId`
pub type MSPModel_JuPedSim_JPS_StageId = i64;

/// C++ typedef `std::tuple<JPS_StageId, const MSLane *, Position, double> MSPModel_JuPedSim::WaypointDesc`
pub type MSPModel_JuPedSim_WaypointDesc = (MSPModel_JuPedSim_JPS_StageId, Ptr<MSLane>, Position, f64);

/// Structure that keeps data related to vanishing areas (and other types of areas).
/// C++ class `MSPModel_JuPedSim::AreaData` (microsim/transportables/MSPModel_JuPedSim.h)
pub struct MSPModel_JuPedSim_AreaData {
    pub id: String,
    pub areaType: String,
    // shortcut: C++ sumo_jupedsim_api::Polygon (protobuf) -> Vec<Position>, convert to the tonic-build generated Polygon at the gRPC call once gRPC is wired
    pub areaBoundary: Vec<Position>,
    /// C++ `const Parameterised::Map&` (non-owning)
    pub params: Ptr<ParameterisedMap>,
    /// The last time a pedestrian was removed in a vanishing area.
    pub lastRemovalTime: SUMOTime,
}

impl MSPModel_JuPedSim_AreaData {
}

/// C++ class `MSPModel_JuPedSim::Event` (microsim/transportables/MSPModel_JuPedSim.h)
#[repr(C)]
pub struct MSPModel_JuPedSim_Event {
    /// C++ base class `Command`
    // shortcut: Command has no Rust type yet, keep () until microsim Command is ported
    pub base: Unmapped /* Command */,
    pub myModel: Ptr<MSPModel_JuPedSim>,
}

impl std::ops::Deref for MSPModel_JuPedSim_Event {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim_Event {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSPModel_JuPedSim_Event {
    /// C++: `void Event::Event(MSPModel_JuPedSim *)`
    // explicit Event(MSPModel_JuPedSim* model)
    //             : myModel(model) { }
    #[allow(unused_variables)]
    pub fn new(model: Ptr<MSPModel_JuPedSim>) -> Self {
        Self { base: (), myModel: model }
    }

    /// C++: `SUMOTime Event::execute(SUMOTime)` [virtual]
    // SUMOTime execute(SUMOTime currentTime) override {
    //             return myModel->execute(currentTime);
    //         }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.myModel.m().execute(currentTime)
    }

}

/// A pedestrian following model that acts as a proxy for pedestrian models provided by the JuPedSim third-party simulation framework.
/// C++ class `MSPModel_JuPedSim` (microsim/transportables/MSPModel_JuPedSim.h)
#[repr(C)]
pub struct MSPModel_JuPedSim {
    /// C++ base class `MSPModel_Interacting`
    pub base: MSPModel_Interacting,
    /// C++ base class `ShapeListener`
    pub base2: ShapeListener,
    // shortcut: C++ std::vector<StopWatch<nanoseconds>>, StopWatch is unported; holds accumulated ns, upgrade to the ported StopWatch
    pub myStopWatch: Vec<i64>,
    /// The random number generator for fluctuations on insertion
    pub myRNG: SumoRNG,
    /// The network on which the simulation runs.
    pub myNetwork: Ptr<MSNet>,
    /// The shape container used to add polygons to the rendering pipeline.
    pub myShapeContainer: Ptr<ShapeContainer>,
    /// Timestep used in the JuPedSim simulator.
    pub myJPSDeltaT: SUMOTime,
    /// Threshold to decide if a pedestrian has ended its journey or not.
    pub myExitTolerance: f64,
    pub myPedestrianStates: Vec<Ptr<MSPModel_JuPedSim_PState>>,
    /// The GEOS polygon containing all computed connected components of the pedestrian network.
    // shortcut: owned GEOSGeometry* -> geos::Geometry (crate geos 11.3.1), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
    pub myGEOSPedestrianNetwork: Option<Box<dyn std::any::Any>>,
    /// The GEOS polygon representing the largest (by area) connected component of the pedestrian network.
    // shortcut: non-owning const GEOSGeometry* -> Ptr<geos::Geometry>, upgrade with the geos crate
    pub myGEOSPedestrianNetworkLargestComponent: Option<Ptr<dyn std::any::Any>>,
    pub myHaveAdditionalWalkableAreas: bool,
    /// The JPS polygon representing the largest connected component of the pedestrian network.
    pub myJPSGeometry: i64,
    /// The JPS polygon representing the largest connected component plus carriages and ramps.
    pub myJPSGeometryWithTrainsAndRamps: i64,
    pub myJPSModel: String,
    pub myJPSSimulation: i64,
    // shortcut: std::shared_ptr<grpc::Channel> -> tonic::transport::Channel (crate tonic 0.14.9), upgrade when gRPC is wired
    pub myGrpcChannel: Option<Box<dyn std::any::Any>>,
    // shortcut: std::unique_ptr<JuPedSimService::Stub> -> tonic-build generated client over Channel (prost 0.14.6), generate from the .proto when gRPC is wired
    pub myGrpcStub: Option<Box<dyn std::any::Any>>,
    /// Array of special areas.
    pub myAreas: Vec<Box<MSPModel_JuPedSim_AreaData>>,
    /// Array of stopped trains, used to detect whether to add carriages and ramps to the geometry.
    pub myAllStoppedTrainIDs: Vec<SUMOTrafficObject_NumericalID>,
    // determinism: C++ std::map ordered by ComparatorNumericalIdLess (lane numerical id); a Vec keeps insertion order, sort by lane id when iterating
    pub myCrossingWaits: Vec<(Ptr<MSLane>, (MSPModel_JuPedSim_JPS_StageId, MSPModel_JuPedSim_JPS_StageId))>,
    pub myCrossings: BTreeMap<MSPModel_JuPedSim_JPS_StageId, (Ptr<MSLane>, bool)>,
    // determinism: C++ std::map ordered by ComparatorNumericalIdLess (lane numerical id); same caveat as myCrossingWaits
    pub myLaneCenters: Vec<(Ptr<MSLane>, MSPModel_JuPedSim_JPS_StageId)>,
}

impl std::ops::Deref for MSPModel_JuPedSim {
    type Target = MSPModel_Interacting;
    fn deref(&self) -> &MSPModel_Interacting {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim {
    fn deref_mut(&mut self) -> &mut MSPModel_Interacting {
        &mut self.base
    }
}

// C++ static members of MSPModel_JuPedSim (port as module-level statics, see AGENTS.md):
//   static const int MSPModel_JuPedSim::GEOS_QUADRANT_SEGMENTS;
//   static const double MSPModel_JuPedSim::GEOS_MITRE_LIMIT;
//   static const double MSPModel_JuPedSim::GEOS_MIN_AREA;
//   static const double MSPModel_JuPedSim::GEOS_BUFFERED_SEGMENT_WIDTH;
//   static const RGBColor MSPModel_JuPedSim::PEDESTRIAN_NETWORK_COLOR;
//   static const RGBColor MSPModel_JuPedSim::PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_COLOR;
//   static const std::string MSPModel_JuPedSim::PEDESTRIAN_NETWORK_ID;
//   static const std::string MSPModel_JuPedSim::PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_ID;
//   static const int MSPModel_JuPedSim::noPedestrians;

impl MSPModel_JuPedSim {
    /// C++: `void MSPModel_JuPedSim::MSPModel_JuPedSim(const OptionsCont &, MSNet *)`
    // MSPModel_JuPedSim::MSPModel_JuPedSim(const OptionsCont& oc, MSNet* net) :
    //     myStopWatch(10), myRNG("JuPedSim"), myNetwork(net), myShapeContainer(net->getShapeContainer()), myJPSDeltaT(string2time(oc.getString("pedestrian.jupedsim.step-length"))),
    //     myExitTolerance(oc.getFloat("pedestrian.jupedsim.exit-tolerance")), myGEOSPedestrianNetworkLargestComponent(nullptr),
    //     myHaveAdditionalWalkableAreas(false) {
    //     RandHelper::initRandGlobal(&myRNG);
    //     myJPSModel = oc.getString("pedestrian.jupedsim.model");
    //     if (myJPSModel.find("Model") == std::string::npos) {
    //         // TODO warn about outdated model name
    //         myJPSModel += "Model";
    //         if (myJPSModel == "CollisionFreeSpeedV2Model") {
    //             myJPSModel = "CollisionFreeSpeedModelV2";
    //         }
    //     }
    //     std::string command;
    //     std::string address = oc.getString("pedestrian.jupedsim.address");
    //     // address = "localhost:50051"; // for debugging
    //     if (address == "") {
    //         const char* pythonEnv = getenv("PYTHON");
    //         const std::string python = (pythonEnv == nullptr) ? "python" : pythonEnv;
    //         const char* sumoHomeEnv = getenv("SUMO_HOME");
    //         const std::string sumoHome = (sumoHomeEnv == nullptr) ? "." : sumoHomeEnv;
    //         command = python + " " + sumoHome + "/tools/jupedsim_grpc/servicer.py";
    //         if (oc.isSet("pedestrian.jupedsim.py")) {
    //             command += " --debug " + oc.getString("pedestrian.jupedsim.py");
    //         }
    //     }
    //     for (int i = 0; i < 2; i++) {  // try twice: initial attempt + one restart
    //         if (command != "") {
    // #ifdef HAVE_BOOST
    //             const std::string port = toString(tcpip::Socket::getFreeSocketPort());
    //             myJuPedSimServer = std::unique_ptr<bp::child>(new bp::child(command + " --port " + port));
    //             address = "localhost:" + port;
    // #else
    //             WRITE_WARNINGF(TL("No boost process support, you will need to start the JuPedSim server manually using %."), command);
    // #endif
    //         }
    //
    //         myGrpcChannel = grpc::CreateChannel(address, grpc::InsecureChannelCredentials());
    //         if (myGrpcChannel->WaitForConnected(std::chrono::system_clock::now() + std::chrono::seconds(5))) {
    //             WRITE_MESSAGEF(TL("Connected to JuPedSim gRPC server on '%'."), address);
    //             break;
    //         } else {
    //             WRITE_WARNINGF(TL("Could not connect to JuPedSim gRPC server on '%' (will retry on demand)."), address);
    // #ifdef HAVE_BOOST
    //             if (myJuPedSimServer == nullptr || myJuPedSimServer->running()) {
    //                 break;
    //             }
    // #else
    //             break;
    // #endif
    //         }
    //     }
    //     myGrpcStub = sumo_jupedsim_api::JuPedSimService::NewStub(myGrpcChannel);
    //     initialize(oc);
    //     net->getBeginOfTimestepEvents()->addEvent(new Event(this), net->getCurrentTimeStep() + DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont, net: Ptr<MSNet>) -> Self {
        // shortcut: needs gRPC (tonic 0.14.9 + prost 0.14.6, tonic-build needs protoc) for the JuPedSim channel and stub, upgrade when tonic/prost are in Cargo.toml and protoc is installed (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::new needs gRPC (tonic + prost, protoc for tonic-build)")
    }

    /// C++: `void MSPModel_JuPedSim::~MSPModel_JuPedSim() noexcept` [virtual]
    // MSPModel_JuPedSim::~MSPModel_JuPedSim() {
    //     clearState();
    // #ifdef HAVE_BOOST
    //     if (myJuPedSimServer != nullptr) {
    //         myJuPedSimServer->terminate();
    //     }
    // #endif
    //
    //     // TODO: clear simulation, operational model and geometries?
    //
    //     GEOSGeom_destroy(myGEOSPedestrianNetwork);
    //     finishGEOS();
    // #ifdef DEBUG_GRPC_TIMING
    //     std::cout << myStopWatch[0].getTotal() / (1e6) << "ms, iteration: "
    //               << myStopWatch[1].getTotal() / (1e6) << "ms, agent data retrieval: "
    //               << myStopWatch[2].getTotal() / (1e6) << "ms, journey creation: "
    //               << myStopWatch[3].getTotal() / (1e6) << "ms, insertion: "
    //               << myStopWatch[4].getTotal() / (1e6) << "ms, removal: "
    //               << myStopWatch[5].getTotal() / (1e6) << "ms, add waiting set: "
    //               << myStopWatch[6].getTotal() / (1e6) << "ms, speed: "
    //               << myStopWatch[7].getTotal() / (1e6) << "ms " << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: needs GEOS (geos 11.3.1, GEOSGeom_destroy/finishGEOS) and the gRPC server process handle, upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::~MSPModel_JuPedSim needs GEOS (geos crate) and the gRPC server process handle")
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_JuPedSim::add(MSTransportable *, MSStageMoving *, SUMOTime)` [virtual]
    // MSTransportableStateAdapter*
    // MSPModel_JuPedSim::add(MSTransportable* person, MSStageMoving* stage, SUMOTime now) {
    //     assert(person->getCurrentStageType() == MSStageType::WALKING);
    //     const double radius = getRadius(person->getVehicleType());
    //     Position departurePosition = Position::INVALID;
    //     const MSLane* const departureLane = getSidewalk<MSEdge, MSLane>(stage->getRoute().front());
    //     if (departureLane == nullptr) {
    //         const char* error = TL("Person '%' could not find sidewalk on edge '%', time=%.");
    //         if (OptionsCont::getOptions().getBool("ignore-route-errors")) {
    //             WRITE_WARNINGF(error, person->getID(), person->getEdge()->getID(), time2string(SIMSTEP));
    //             return nullptr;
    //         } else {
    //             throw ProcessError(TLF(error, person->getID(), person->getEdge()->getID(), time2string(SIMSTEP)));
    //         }
    //     }
    //     // First real stage, stage 0 is waiting.
    //     if (person->getCurrentStageIndex() == 2 && person->getParameter().departPosProcedure == DepartPosDefinition::RANDOM_LOCATION) {
    //         const MSEdge* const tripOrigin = person->getNextStage(-1)->getEdge();
    //         if (tripOrigin->isTazConnector()) {
    //             const SUMOPolygon* tazShape = myNetwork->getShapeContainer().getPolygons().get(tripOrigin->getParameter("taz"));
    //             if (tazShape == nullptr) {
    //                 WRITE_WARNINGF(TL("FromTaz '%' for person '%' has no shape information."), tripOrigin->getParameter("taz"), person->getID());
    //             } else {
    //                 const Boundary& bbox = tazShape->getShape().getBoxBoundary();
    //                 while (!tazShape->getShape().around(departurePosition)) {
    //                     // TODO: Optimize for speed if necessary or at least abort trying to find a point.
    //                     departurePosition.setx(RandHelper::rand(bbox.xmin(), bbox.xmax(), &myRNG));
    //                     departurePosition.sety(RandHelper::rand(bbox.ymin(), bbox.ymax(), &myRNG));
    //                 }
    //             }
    //         }
    //     }
    //     if (departurePosition == Position::INVALID) {
    //         const double halfDepartureLaneWidth = departureLane->getWidth() / 2.0;
    //         double departureRelativePositionY = stage->getDepartPosLat();
    //         if (departureRelativePositionY == UNSPECIFIED_POS_LAT) {
    //             departureRelativePositionY = 0.0;
    //         }
    //         if (departureRelativePositionY == MSPModel::RANDOM_POS_LAT) {
    //             departureRelativePositionY = RandHelper::rand(-halfDepartureLaneWidth, halfDepartureLaneWidth, &myRNG);
    //         }
    //         departurePosition = departureLane->getShape().positionAtOffset(stage->getDepartPos() + radius + POSITION_EPS, -departureRelativePositionY); // Minus sign is here for legacy reasons.
    //     }
    //
    //     std::vector<WaypointDesc> waypoints;
    //     const MSEdge* prev = nullptr;
    //     for (const MSEdge* const e : stage->getEdges()) {
    //         const MSLane* const lane = getSidewalk<MSEdge, MSLane>(e);
    //         JPS_StageId waitingStage = 0;
    //         if (prev != nullptr) {
    //             int dir = UNDEFINED_DIRECTION;
    //             if (prev->getToJunction() == e->getFromJunction() || prev->getToJunction() == e->getToJunction()) {
    //                 dir = FORWARD;
    //             } else if (prev->getFromJunction() == e->getFromJunction() || prev->getFromJunction() == e->getToJunction()) {
    //                 dir = BACKWARD;
    //             }
    //             if (dir != UNDEFINED_DIRECTION) {
    //                 ConstMSEdgeVector crossingRoute;
    //                 MSNet::getInstance()->getPedestrianRouter(0).compute(prev, e, 0, 0, stage->getMaxSpeed(person), 0,
    //                         dir == FORWARD ? prev->getToJunction() : prev->getFromJunction(),
    //                         person->getVTypeParameter(), crossingRoute, true);
    //                 const MSEdge* wa = nullptr;
    //                 for (const MSEdge* const ce : crossingRoute) {
    //                     if (ce->isCrossing()) {
    //                         const MSLane* const crossing = getSidewalk<MSEdge, MSLane>(ce);
    //                         auto crossingWait = myCrossingWaits.find(crossing);
    //                         if (crossingWait != myCrossingWaits.end()) {
    //                             if (waitingStage != 0 && wa != nullptr) {
    //                                 // we already have a waiting stage we need an intermediate waypoint
    //                                 waypoints.push_back({waitingStage, wa->getLanes()[0], {}, {}});
    //                             }
    //                             const MSLane* const prevLane = getSidewalk<MSEdge, MSLane>(prev);
    //                             const Position& startPos = dir == FORWARD ? prevLane->getShape().back() : prevLane->getShape().front();
    //                             // choose the waiting set closer to the lane "end"
    //                             if (crossing->getShape().front().distanceSquaredTo(startPos) < crossing->getShape().back().distanceSquaredTo(startPos)) {
    //                                 if (crossingWait->second.first == 0) {
    //                                     crossingWait->second.first = addWaitingSet(crossing, true);
    //                                 }
    //                                 waitingStage = crossingWait->second.first;
    //                             } else {
    //                                 if (crossingWait->second.second == 0) {
    //                                     crossingWait->second.second = addWaitingSet(crossing, false);
    //                                 }
    //                                 waitingStage = crossingWait->second.second;
    //                             }
    //                         } else {
    //                             throw ProcessError(TLF("No waiting set for crossing at %.", ce->getID()));
    //                         }
    //                     }
    //                     if (ce->isWalkingArea()) {
    //                         wa = ce;
    //                     }
    //                 }
    //             }
    //         }
    //         waypoints.push_back({waitingStage, lane, {}, {}});
    //         prev = e;
    //     }
    //     const JPS_StageId finalWait = std::get<0>(waypoints.back());
    //     waypoints.pop_back();  // arrival waypoint comes later
    //     if (!waypoints.empty()) {
    //         waypoints.erase(waypoints.begin());  // departure edge also does not need a waypoint
    //     }
    //     waypoints.push_back({finalWait, getSidewalk<MSEdge, MSLane>(stage->getDestination()), {}, {}});
    //
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[3].start();
    // #endif
    //     sumo_jupedsim_api::AddJourneyRequest journeyRequest;
    //     JPS_StageId startingStage = 0;
    //     for (WaypointDesc& p : waypoints) {
    //         const JPS_StageId waiting = std::get<0>(p);
    //         if (waiting != 0) {
    //             journeyRequest.add_stage_ids(waiting);
    //         }
    //         JPS_StageId waypoint = addWaypoint(person->getID(), p, &p == &waypoints.back() ? stage : nullptr);
    //         if (waypoint == 0) {
    //             return nullptr;
    //         }
    //         journeyRequest.add_stage_ids(waypoint);
    //         if (startingStage == 0) {
    //             startingStage = waiting != 0 ? waiting : waypoint;
    //         }
    //     }
    //     const sumo_jupedsim_api::AddJourneyResponse journeyResponse = callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::AddJourney, journeyRequest,
    //             TLF("Error while adding journey for agent '%': ", person->getID()));
    //     const JPS_JourneyId journeyId = journeyResponse.journey_id();
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[3].stop();
    // #endif
    //
    //     PState* state = nullptr;
    //     for (PState* const pstate : myPedestrianStates) {  // TODO transform myPedestrianStates into a map for faster lookup
    //         if (pstate->getPerson() == person) {
    //             state = pstate;
    //             break;
    //         }
    //     }
    //     if (state == nullptr) {
    //         state = new PState(static_cast<MSPerson*>(person), stage, journeyId, startingStage, waypoints);
    //         state->setLanePosition(stage->getDepartPos() + radius + POSITION_EPS);
    //         state->setPosition(departurePosition.x(), departurePosition.y());
    //         state->setAngle(departureLane->getShape().rotationAtOffset(stage->getDepartPos()));
    //         myPedestrianStates.push_back(state);
    //         myNumActivePedestrians++;
    //     } else {
    //         state->reinit(stage, journeyId, startingStage, waypoints);
    //     }
    //     if (state->isWaitingToEnter()) {
    //         const Position p = state->getPosition(*state->getStage(), now);
    //         tryPedestrianInsertion(state, p);
    //     } else {
    //         sumo_jupedsim_api::SwitchAgentJourneyRequest switchRequest;
    //         switchRequest.set_agent_id(state->getAgentId());
    //         switchRequest.set_journey_id(journeyId);
    //         switchRequest.set_stage_id(startingStage);
    //         callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SwitchAgentJourney, switchRequest,
    //                  TLF("Error while switching journey for agent '%': ", person->getID()));
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, person: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
        // shortcut: needs gRPC (tonic + prost) for AddJourney and SwitchAgentJourney, upgrade when tonic/prost are in Cargo.toml and protoc is installed
        panic!("MSPModel_JuPedSim::add needs gRPC (tonic + prost, protoc for tonic-build)")
    }

    /// C++: `void MSPModel_JuPedSim::remove(MSTransportableStateAdapter *)` [virtual]
    // void
    // MSPModel_JuPedSim::remove(MSTransportableStateAdapter* state) {
    //     PState* pstate = static_cast<PState*>(state);
    //     if (pstate->getLane() != nullptr) {
    //         auto& peds = myActiveLanes[pstate->getLane()];
    //         peds.erase(std::find(peds.begin(), peds.end(), pstate));
    //         pstate->setLane(nullptr);
    //     }
    //     if (pstate->getStage() != nullptr) {
    //         pstate->getStage()->setPState(nullptr);  // we need to remove the old state reference to avoid double deletion
    //     }
    //     pstate->setStage(nullptr);
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, state: Ptr<MSTransportableStateAdapter>) {
        // shortcut: needs the myActiveLanes member (C++ std::map<MSLane*, std::vector<PState*>>), which the generated struct omits, add it then port remove, upgrade when the struct is regenerated with it
        panic!("MSPModel_JuPedSim::remove needs the myActiveLanes member, missing from the generated struct")
    }

    /// C++: `SUMOTime MSPModel_JuPedSim::execute(SUMOTime)`
    // SUMOTime
    // MSPModel_JuPedSim::execute(SUMOTime time) {
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[1].start();
    // #endif
    //     sumo_jupedsim_api::IterateRequest iterateRequest;
    //     iterateRequest.set_count((int)(DELTA_T / myJPSDeltaT));
    //     callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::Iterate, iterateRequest, TL("JuPedSim gRPC Iterate failed: "));
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[1].stop();
    // #endif
    //
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[2].start();
    // #endif
    //     sumo_jupedsim_api::GetCorePropertiesOfAllAgentsRequest propertiesRequest;
    //     sumo_jupedsim_api::GetCorePropertiesOfAllAgentsResponse propertiesResponse =
    //         callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::GetCorePropertiesOfAllAgents, propertiesRequest, TL("JuPedSim gRPC GetCorePropertiesOfAllAgents failed: "));
    //     const auto& properties = propertiesResponse.properties();
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[2].stop();
    // #endif
    //
    //     for (auto stateIt = myPedestrianStates.begin(); stateIt != myPedestrianStates.end();) {
    //         PState* const state = *stateIt;
    //
    //         if (state->isWaitingToEnter()) {
    //             // insertion failed at first try so we retry with some noise
    //             Position p = state->getPosition(*state->getStage(), time);
    //             p.setx(p.x() + RandHelper::rand(-.5, .5, &myRNG));  // we do this separately to avoid evaluation order problems
    //             p.sety(p.y() + RandHelper::rand(-.5, .5, &myRNG));
    //             tryPedestrianInsertion(state, p);
    //             ++stateIt;
    //             continue;
    //         }
    //
    //         MSPerson* const person = state->getPerson();
    //         MSStageWalking* const stage = dynamic_cast<MSStageWalking*>(person->getCurrentStage());
    //         if (stage == nullptr) {
    //             // It seems we kept the state for another stage but the new stage is not a walk.
    //             // So let's remove the state because after the new stage we will be elsewhere and need to be reinserted for JuPedSim anyway.
    //             // We cannot check this earlier because when the old stage ends the next stage might not know yet whether it will be a walk.
    //             registerArrived(state->getAgentId());
    //             stateIt = myPedestrianStates.erase(stateIt);
    //             continue;
    //         }
    //
    //         // Updates the agent position.
    //         const auto agentIt = properties.find(state->getAgentId());
    //         if (agentIt == properties.end()) {
    //             WRITE_WARNINGF(TL("JuPedSim gRPC unknown agent: %"), state->getAgentId());
    //             continue;
    //         }
    //         const sumo_jupedsim_api::Point& pos = agentIt->second.position();
    //         state->setPosition(pos.x(), pos.y());
    //         const sumo_jupedsim_api::Point& orientation = agentIt->second.orientation();
    //         state->setAngle(atan2(orientation.y(), orientation.x()));
    //
    //         // Find on which edge the pedestrian is, using route's forward-looking edges because of how moveToXY is written.
    //         Position newPosition(pos.x(), pos.y());
    //         ConstMSEdgeVector route = stage->getEdges();
    //         const int routeIndex = (int)(stage->getRouteStep() - stage->getRoute().begin());
    //         const double oldLanePos = state->getEdgePos(time);
    //         ConstMSEdgeVector forwardRoute = ConstMSEdgeVector(route.begin() + routeIndex, route.end());
    //         double bestDistance = std::numeric_limits<double>::max();
    //         MSLane* candidateLane = nullptr;
    //         double candidateLaneLongitudinalPosition = 0.0;
    //         int routeOffset = 0;
    //         const bool found = libsumo::Helper::moveToXYMap_matchingRoutePosition(newPosition, "",
    //                            forwardRoute, 0, person->getVClass(), true, bestDistance, &candidateLane, candidateLaneLongitudinalPosition, routeOffset);
    //
    //         if (found) {
    //             if (candidateLane != state->getLane()) {
    //                 if (state->getLane() != nullptr) {
    //                     auto& peds = myActiveLanes[state->getLane()];
    //                     peds.erase(std::find(peds.begin(), peds.end(), state));
    //                 }
    //                 myActiveLanes[candidateLane].push_back(state);
    //                 state->setLane(candidateLane);
    //             }
    //             state->setLanePosition(candidateLaneLongitudinalPosition);
    //         }
    //
    //         const MSEdge* const expectedEdge = stage->getEdge();
    //         if (found && expectedEdge->isNormal() && candidateLane->getEdge().isNormal() && &candidateLane->getEdge() != expectedEdge) {
    //             const bool arrived = stage->moveToNextEdge(person, time, 1, nullptr);
    //             UNUSED_PARAMETER(arrived);
    //             assert(!arrived); // The person has not arrived yet.
    //             stage->activateEntryReminders(person);
    //             // Adapt speed to lane's speed limit.
    //             // TODO make this a bulk request
    //             sumo_jupedsim_api::SetDesiredSpeedRequest desiredSpeedRequest;
    //             auto speeds = desiredSpeedRequest.mutable_desired_speeds();
    //             (*speeds)[state->getAgentId()] = MIN2(candidateLane->getSpeedLimit(), person->getMaxSpeed());
    // #ifdef DEBUG_GRPC_TIMING
    //             myStopWatch[7].start();
    // #endif
    //             callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SetDesiredSpeedOfAgents, desiredSpeedRequest,
    //                      TLF("Error while setting desired speed for %: ", person->getID()));
    // #ifdef DEBUG_GRPC_TIMING
    //             myStopWatch[7].stop();
    // #endif
    //         }
    //         const double speed = person->getSpeed();
    //         for (int offset = 0; offset < 2; offset++) {
    //             const WaypointDesc* const waypoint = state->getNextWaypoint(offset);
    //             if (waypoint != nullptr && std::get<0>(*waypoint) != 0) {
    //                 const JPS_StageId waitingStage = std::get<0>(*waypoint);
    //                 auto& crossingState = myCrossings[waitingStage];
    //                 const MSLane* const crossing = crossingState.first;
    //                 const MSLink* link = crossing->getIncomingLanes().front().viaLink;
    //                 if (waitingStage == myCrossingWaits[crossing].second && link->getTLLogic() != nullptr) {
    //                     // we are walking backwards on a traffic light, there are different links to check
    //                     link = crossing->getLinkCont().front();
    //                     if (link->getTLLogic() == nullptr) {
    //                         link = crossing->getLogicalPredecessorLane()->getLinkTo(crossing);
    //                     }
    //                 }
    //                 // compare to and maybe adapt for MSPModel_Striping.cpp:1264
    //                 const double passingClearanceTime = person->getFloatParam("pedestrian.timegap-crossing");
    //                 const bool open = link->opened(time - DELTA_T, speed, speed, person->getVehicleType().getLength() + passingClearanceTime * speed,
    //                                                person->getImpatience(), speed, 0, 0, nullptr, false, person);
    //                 if (open != crossingState.second) {
    //                     sumo_jupedsim_api::SetWaitingSetStateRequest waitingSetStateRequest;
    //                     waitingSetStateRequest.set_stage_id(waitingStage);
    //                     waitingSetStateRequest.set_state(open ? sumo_jupedsim_api::INACTIVE : sumo_jupedsim_api::ACTIVE);
    //                     callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SetWaitingSetState, waitingSetStateRequest,
    //                              TLF("Error while setting waiting status for %:", crossing->getID()));
    //                     crossingState.second = open;
    //                 }
    //             }
    //         }
    //         stage->activateMoveReminders(person, oldLanePos, state->getEdgePos(time), state->getSpeed(*stage));
    //         // In the worst case during one SUMO step the person touches the waypoint radius and walks immediately into a different direction,
    //         // but at some simstep it should have a maximum distance of v * delta_t / 2 to the waypoint circle.
    //         const double slack = person->getMaxSpeed() * TS / 2. + POSITION_EPS;
    //         if (newPosition.distanceTo2D(std::get<2>(*state->getNextWaypoint())) < std::get<3>(*state->getNextWaypoint()) + slack) {
    //             // If near the last waypoint, remove the agent.
    //             if (state->advanceNextWaypoint()) {
    //                 // TODO this only works if the final stage is actually a walk
    //                 const bool finalStage = person->getNumRemainingStages() == 1;
    //                 const JPS_AgentId agentID = state->getAgentId();
    //                 while (!stage->moveToNextEdge(person, time, 1, nullptr));
    //                 if (finalStage) {
    //                     registerArrived(agentID);
    //                     stateIt = myPedestrianStates.erase(stateIt);
    //                     continue;
    //                 }
    //             }
    //         }
    //         ++stateIt;
    //     }
    //
    //     // Remove pedestrians that are in a predefined area, at a predefined rate.
    //     for (const auto& area : myAreas) {
    //         sumo_jupedsim_api::GetAgentsInRegionRequest agentsRequest;
    //         *agentsRequest.mutable_region() = area->areaBoundary;
    //         const sumo_jupedsim_api::AgentIdsResponse agentsResponse = callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::GetAgentsInRegion,
    //                 agentsRequest, TL("Error while retrieving agents in area: "));
    //         const int numAgents = agentsResponse.agent_ids_size();
    //         if (numAgents == 0) {
    //             continue;
    //         }
    //         if (area->areaType == "vanishing_area") {
    //             const SUMOTime period = area->params.count("period") > 0 ? string2time(area->params.at("period")) : 1000;
    //             const int nbrPeriodsCoveringTimestep = (int)ceil(TS / STEPS2TIME(period));
    //             if (time - area->lastRemovalTime >= nbrPeriodsCoveringTimestep * period) {
    //                 for (int k = 0; k < MIN2(nbrPeriodsCoveringTimestep, numAgents); k++) {
    //                     const JPS_AgentId agentID = agentsResponse.agent_ids(k);
    //                     auto lambda = [agentID](const PState * const p) {
    //                         return p->getAgentId() == agentID;
    //                     };
    //                     std::vector<PState*>::const_iterator iterator = std::find_if(myPedestrianStates.begin(), myPedestrianStates.end(), lambda);
    //                     if (iterator != myPedestrianStates.end()) {
    //                         const PState* const state = *iterator;
    //                         MSPerson* const person = state->getPerson();
    //                         // Code below only works if the removal happens at the last stage.
    //                         const bool finalStage = person->getNumRemainingStages() == 1;
    //                         if (finalStage) {
    //                             WRITE_MESSAGEF(TL("Person '%' in vanishing area '%' was removed from the simulation."), person->getID(), area->id);
    //                             while (!state->getStage()->moveToNextEdge(person, time, 1, nullptr));
    //                             registerArrived(agentID);
    //                             myPedestrianStates.erase(iterator);
    //                             area->lastRemovalTime = time;
    //                         }
    //                     }
    //                 }
    //             }
    //         } else {  // areaType == "influencer"
    //             if (area->params.count("speed") == 0) {
    //                 continue;
    //             }
    //             const double newSpeed = StringUtils::toDouble(area->params.at("speed"));
    //             sumo_jupedsim_api::SetDesiredSpeedRequest desiredSpeedRequest;
    //             auto speeds = desiredSpeedRequest.mutable_desired_speeds();
    //             for (int i = 0; i < numAgents; i++) {
    //                 (*speeds)[agentsResponse.agent_ids(i)] = newSpeed;
    //             }
    //             callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SetDesiredSpeedOfAgents, desiredSpeedRequest,
    //                      TL("Error while setting desired speed for area: "));
    //         }
    //     }
    //
    //     // Add dynamically additional geometry from train carriages that are stopped.
    //     const auto& stoppingPlaces = myNetwork->getStoppingPlaces(SumoXMLTag::SUMO_TAG_BUS_STOP);
    //     std::vector<SUMOTrafficObject::NumericalID> allStoppedTrainIDs;
    //     std::vector<const MSVehicle*> allStoppedTrains;
    //     for (const auto& stop : stoppingPlaces) {
    //         std::vector<const SUMOVehicle*> stoppedTrains = stop.second->getStoppedVehicles();
    //         for (const SUMOVehicle* train : stoppedTrains) {
    //             allStoppedTrainIDs.push_back(train->getNumericalID());
    //             allStoppedTrains.push_back(dynamic_cast<const MSVehicle*>(train));
    //         }
    //     }
    //     if (allStoppedTrainIDs != myAllStoppedTrainIDs) {
    //         removePolygonFromDrawing(PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_ID);
    //         if (!allStoppedTrainIDs.empty()) {
    //             std::vector<GEOSGeometry*> carriagePolygons;
    //             std::vector<GEOSGeometry*> rampPolygons;
    //             for (const MSVehicle* train : allStoppedTrains) {
    //                 if (train->getLeavingPersonNumber() > 0) {
    //                     const SUMOVTypeParameter& vTypeParam = train->getVehicleType().getParameter();
    //                     MSTrainHelper trainHelper = MSTrainHelper(train);
    //                     trainHelper.computeDoorPositions();
    //                     const std::vector<MSTrainHelper::Carriage*>& carriages = trainHelper.getCarriages();
    //                     for (const MSTrainHelper::Carriage* carriage : carriages) {
    //                         Position dir = carriage->front - carriage->back;
    //                         if (dir.length2D() == 0.0) {
    //                             continue;
    //                         }
    //                         dir.norm2D();
    //                         Position perp = Position(-dir.y(), dir.x());
    //                         // Create carriages geometry.
    //                         double p = trainHelper.getHalfWidth();
    //                         PositionVector carriageShape;
    //                         carriageShape.push_back(carriage->front + perp * p);
    //                         carriageShape.push_back(carriage->back + perp * p);
    //                         carriageShape.push_back(carriage->back - perp * p);
    //                         carriageShape.push_back(carriage->front - perp * p);
    //                         carriagePolygons.push_back(createGeometryFromShape(carriageShape));
    //                         // Create ramps geometry.
    //                         p += vTypeParam.maxPlatformDistance;
    //                         const double d = 0.5 * vTypeParam.carriageDoorWidth;
    //                         for (const Position& door : carriage->doorPositions) {
    //                             PositionVector rampShape;
    //                             rampShape.push_back(door - perp * p + dir * d);
    //                             rampShape.push_back(door - perp * p - dir * d);
    //                             rampShape.push_back(door + perp * p - dir * d);
    //                             rampShape.push_back(door + perp * p + dir * d);
    //                             rampPolygons.push_back(createGeometryFromShape(rampShape));
    //                         }
    //                     }
    //                 }
    //             }
    //             if (!carriagePolygons.empty()) {
    //                 GEOSGeometry* carriagesCollection = GEOSGeom_createCollection(GEOS_MULTIPOLYGON, carriagePolygons.data(), (unsigned int)carriagePolygons.size());
    //                 GEOSGeometry* rampsCollection = GEOSGeom_createCollection(GEOS_MULTIPOLYGON, rampPolygons.data(), (unsigned int)rampPolygons.size());
    //                 GEOSGeometry* carriagesAndRampsUnion = GEOSUnion(carriagesCollection, rampsCollection);
    //                 if (carriagesAndRampsUnion == nullptr) {
    //                     WRITE_WARNING(TL("Error while generating geometry for carriages."));
    //                 } else {
    //                     GEOSGeometry* pedestrianNetworkWithTrainsAndRamps = GEOSUnion(carriagesAndRampsUnion, myGEOSPedestrianNetworkLargestComponent);
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //                     //dumpGeometry(pedestrianNetworkWithTrainsAndRamps, "pedestrianNetworkWithTrainsAndRamps.wkt");
    // #endif
    //                     int nbrComponents = 0;
    //                     double maxArea = 0.0;
    //                     double totalArea = 0.0;
    //                     const GEOSGeometry* pedestrianNetworkWithTrainsAndRampsLargestComponent = getLargestComponent(pedestrianNetworkWithTrainsAndRamps, nbrComponents, maxArea, totalArea);
    //                     if (nbrComponents > 1) {
    //                         WRITE_WARNINGF(TL("While generating geometry % connected components were detected, %% of total pedestrian area is covered by the largest."),
    //                                        nbrComponents, maxArea / totalArea * 100.0, "%");
    //                     }
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //                     //dumpGeometry(pedestrianNetworkWithTrainsAndRampsLargestComponent, "pedestrianNetworkWithTrainsAndRamps.wkt");
    // #endif
    //                     myJPSGeometryWithTrainsAndRamps = buildJPSGeometryFromGEOSGeometry(pedestrianNetworkWithTrainsAndRampsLargestComponent);
    //                     sumo_jupedsim_api::SwitchGeometryRequest switchRequest;
    //                     switchRequest.set_geometry_id(myJPSGeometryWithTrainsAndRamps);
    //                     callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SwitchGeometry, switchRequest, TL("While switching to train geometry: "), true);
    //                     removePolygonFromDrawing(PEDESTRIAN_NETWORK_ID);
    //                     preparePolygonForDrawing(pedestrianNetworkWithTrainsAndRampsLargestComponent, PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_ID, PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_COLOR);
    //                     GEOSGeom_destroy(pedestrianNetworkWithTrainsAndRamps);
    //                 }
    //                 GEOSGeom_destroy(rampsCollection);
    //                 GEOSGeom_destroy(carriagesCollection);
    //             }
    //         } else {
    //             if (myJPSGeometryWithTrainsAndRamps != -1) {
    //                 sumo_jupedsim_api::SwitchGeometryRequest switchRequest;
    //                 switchRequest.set_geometry_id(myJPSGeometry);
    //                 callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::SwitchGeometry, switchRequest, TL("While switching to default geometry: "), true);
    //                 preparePolygonForDrawing(myGEOSPedestrianNetworkLargestComponent, PEDESTRIAN_NETWORK_ID, PEDESTRIAN_NETWORK_COLOR);
    //                 myJPSGeometryWithTrainsAndRamps = -1;
    //             }
    //         }
    //         myAllStoppedTrainIDs = allStoppedTrainIDs;
    //     }
    //
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, time: SUMOTime) -> SUMOTime {
        // shortcut: needs gRPC (tonic + prost) for Iterate and GetCorePropertiesOfAllAgents, plus GEOS for train geometry, upgrade when both are wired (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::execute needs gRPC (tonic + prost) and GEOS")
    }

    /// C++: `bool MSPModel_JuPedSim::usingShortcuts()` [virtual]
    // bool usingShortcuts() override {
    //         return myHaveAdditionalWalkableAreas;
    //     }
    #[allow(unused_variables)]
    pub fn using_shortcuts(&mut self) -> bool {
        self.myHaveAdditionalWalkableAreas
    }

    /// C++: `bool MSPModel_JuPedSim::usingDoors()` [virtual]
    /// precise lateral positioning when leaving trains is supported by JuPedSim
    // bool usingDoors() override {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn using_doors(&mut self) -> bool {
        true
    }

    /// C++: `void MSPModel_JuPedSim::registerArrived(const JPS_AgentId)`
    // void MSPModel_JuPedSim::registerArrived(const JPS_AgentId agentID) {
    //     myNumActivePedestrians--;
    //     sumo_jupedsim_api::RemoveAgentsRequest removeRequest;
    //     removeRequest.add_agent_ids(agentID);
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[5].start();
    // #endif
    //     callGrpc(&sumo_jupedsim_api::JuPedSimService::Stub::RemoveAgents, removeRequest, TL("Error while removing agent: "));
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[5].stop();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn register_arrived(&mut self, agentID: MSPModel_JuPedSim_JPS_AgentId) {
        // shortcut: needs gRPC (tonic + prost) for RemoveAgents, upgrade when tonic/prost are in Cargo.toml and protoc is installed
        panic!("MSPModel_JuPedSim::registerArrived needs gRPC (tonic + prost, protoc for tonic-build)")
    }

    /// C++: `void MSPModel_JuPedSim::clearState()` [virtual]
    // void MSPModel_JuPedSim::clearState() {
    //     myPedestrianStates.clear();
    //     myNumActivePedestrians = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myPedestrianStates.clear();
        self.myNumActivePedestrians = 0;
    }

    /// C++: `void MSPModel_JuPedSim::polygonChanged(const SUMOPolygon *const, const bool, const bool)` [virtual]
    // void
    // MSPModel_JuPedSim::polygonChanged(const SUMOPolygon* const poly, const bool added, const bool removed) {
    //     if (poly->getShapeType() == "jupedsim.vanishing_area" || poly->getShapeType() == "jupedsim.influencer") {
    //         SUMOTime lastRemovalTime = 0;
    //         if (!added) {
    //             for (auto areaIt = myAreas.begin(); areaIt != myAreas.end(); ++areaIt) {
    //                 if (poly->getID() == (*areaIt)->id) {
    //                     lastRemovalTime = (*areaIt)->lastRemovalTime;
    //                     myAreas.erase(areaIt);
    //                     break;
    //                 }
    //             }
    //         }
    //         if (!removed) {
    //             sumo_jupedsim_api::Polygon areaBoundary;
    //             for (const Position& p : poly->getShape()) {
    //                 sumo_jupedsim_api::Point* point = areaBoundary.add_points();
    //                 point->set_x(p.x());
    //                 point->set_y(p.y());
    //             }
    //             // // Make sure the shape is not repeating the first point.
    //             // if (areaBoundary.back().x == areaBoundary.front().x && areaBoundary.back().y == areaBoundary.front().y) {
    //             //     areaBoundary.pop_back();
    //             // }
    //             const std::string type = StringTokenizer(poly->getShapeType(), ".").getVector()[1];
    //             myAreas.emplace_back(std::unique_ptr<AreaData>(new AreaData{poly->getID(), type, areaBoundary, poly->getParametersMap(), lastRemovalTime}));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn polygon_changed(&mut self, poly: Ptr<SUMOPolygon>, added: bool, removed: bool) {
        if poly.get_shape_type() == "jupedsim.vanishing_area" || poly.get_shape_type() == "jupedsim.influencer" {
            let mut lastRemovalTime: SUMOTime = 0;
            if !added {
                if let Some(areaIt) = self.myAreas.iter().position(|a| poly.get_id() == a.id) {
                    lastRemovalTime = self.myAreas[areaIt].lastRemovalTime;
                    self.myAreas.remove(areaIt);
                }
            }
            if !removed {
                let areaBoundary: Vec<Position> = poly.get_shape().iter().copied().collect();
                let type_ = StringTokenizer::with_token(poly.get_shape_type(), ".", false).get_vector()[1].clone();
                self.myAreas.push(Box::new(MSPModel_JuPedSim_AreaData {
                    id: poly.get_id().to_string(),
                    areaType: type_,
                    areaBoundary,
                    params: Ptr::new(poly.base2.get_parameters_map()),
                    lastRemovalTime,
                }));
            }
        }
    }

    /// C++: `void MSPModel_JuPedSim::initialize(const OptionsCont &)` [private]
    // void
    // MSPModel_JuPedSim::initialize(const OptionsCont& oc) {
    //     initGEOS(nullptr, nullptr);
    //     PROGRESS_BEGIN_MESSAGE("Generating initial JuPedSim geometry for pedestrian network");
    //     myGEOSPedestrianNetwork = buildPedestrianNetwork(myNetwork);
    //     int nbrComponents = 0;
    //     double maxArea = 0.0;
    //     double totalArea = 0.0;
    //     myGEOSPedestrianNetworkLargestComponent = getLargestComponent(myGEOSPedestrianNetwork, nbrComponents, maxArea, totalArea);
    //     if (nbrComponents > 1) {
    //         WRITE_WARNINGF(TL("While generating geometry % connected components were detected, %% of total pedestrian area is covered by the largest."),
    //                        nbrComponents, maxArea / totalArea * 100.0, "%");
    //     }
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     dumpGeometry(myGEOSPedestrianNetworkLargestComponent, "pedestrianNetwork.wkt");
    // #endif
    //     const std::string filename = oc.getString("pedestrian.jupedsim.wkt");
    //     if (!filename.empty()) {
    //         dumpGeometry(myGEOSPedestrianNetworkLargestComponent, filename, oc.getBool("pedestrian.jupedsim.wkt.geo"));
    //     }
    //     // For the moment, only one connected component is supported.
    //     myJPSGeometry = buildJPSGeometryFromGEOSGeometry(myGEOSPedestrianNetworkLargestComponent);
    //     myJPSGeometryWithTrainsAndRamps = -1;
    //     preparePolygonForDrawing(myGEOSPedestrianNetworkLargestComponent, PEDESTRIAN_NETWORK_ID, PEDESTRIAN_NETWORK_COLOR);
    //     PROGRESS_DONE_MESSAGE();
    //
    //     double strengthGeometryRepulsion = oc.getFloat("pedestrian.jupedsim.strength-geometry-repulsion");
    //     double rangeGeometryRepulsion = oc.getFloat("pedestrian.jupedsim.range-geometry-repulsion");
    //     if (myJPSDeltaT == 20) {
    //         if (oc.isDefault("pedestrian.jupedsim.strength-geometry-repulsion") && oc.isDefault("pedestrian.jupedsim.range-geometry-repulsion")) {
    //             WRITE_MESSAGE(TL("Adapting geometry repulsion default values for jupedsim timestep of 0.02."));
    //             strengthGeometryRepulsion = 35.;
    //             rangeGeometryRepulsion = 0.019;
    //         }
    //     }
    //     sumo_jupedsim_api::CreateSimulationRequest simulationRequest;
    //     simulationRequest.set_geometry_id(myJPSGeometry);
    //     simulationRequest.set_model_class(myJPSModel);
    //     simulationRequest.set_delta_t(STEPS2TIME(myJPSDeltaT));
    //     // Default server side arguments for the models.
    //     // TODO: Better handling of this as this is still hard-coded for each model class.
    //     auto& modelParams = *simulationRequest.mutable_model_parameters()->mutable_fields();
    //     if (myJPSModel == "CollisionFreeSpeedModel") {
    //         modelParams["strength_neighbor_repulsion"].set_number_value(oc.getFloat("pedestrian.jupedsim.strength-neighbor-repulsion"));
    //         modelParams["range_neighbor_repulsion"].set_number_value(oc.getFloat("pedestrian.jupedsim.range-neighbor-repulsion"));
    //         modelParams["strength_geometry_repulsion"].set_number_value(strengthGeometryRepulsion);
    //         modelParams["range_geometry_repulsion"].set_number_value(rangeGeometryRepulsion);
    //     } else if (myJPSModel == "CollisionFreeSpeedModelV2") {
    //         // Empty - did not set any parameters in old implementation
    //     } else if (myJPSModel == "GeneralizedCentrifugalForceModel") {
    //         modelParams["strength_neighbor_repulsion"].set_number_value(oc.getFloat("pedestrian.jupedsim.strength-neighbor-repulsion"));
    //         modelParams["strength_geometry_repulsion"].set_number_value(strengthGeometryRepulsion);
    //     } else if (myJPSModel == "SocialForceModel") {
    //         modelParams["body_force"].set_number_value(120000.0);
    //         modelParams["friction"].set_number_value(240000.0);
    //     }
    //     grpc::ClientContext simulationContext;
    //     sumo_jupedsim_api::CreateSimulationResponse simulationResponse;
    //     const grpc::Status simulationStatus = myGrpcStub->CreateSimulation(&simulationContext, simulationRequest, &simulationResponse);
    //     if (simulationStatus.ok()) {
    //         myJPSSimulation = simulationResponse.simulation_id();
    //     } else {
    //         throw ProcessError(TLF("JuPedSim gRPC CreateSimulation failed: %", simulationStatus.error_message()));
    //     }
    //     // Polygons that define vanishing areas aren't part of the regular JuPedSim geometry.
    //     for (const auto& polygonWithID : myNetwork->getShapeContainer().getPolygons()) {
    //         polygonChanged(polygonWithID.second, true, false);
    //     }
    //     myNetwork->getShapeContainer().addShapeListener(this);
    // }
    #[allow(unused_variables)]
    pub fn initialize(&mut self, oc: &OptionsCont) {
        // shortcut: needs GEOS (geos 11.3.1) for the pedestrian network and gRPC for CreateSimulation, upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::initialize needs GEOS (geos crate) and gRPC (CreateSimulation)")
    }

    /// C++: `void MSPModel_JuPedSim::tryPedestrianInsertion(PState *, const Position &)` [private]
    // void
    // MSPModel_JuPedSim::tryPedestrianInsertion(PState* state, const Position& p) {
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[4].start();
    // #endif
    //     const MSVehicleType& type = state->getPerson()->getVehicleType();
    //     sumo_jupedsim_api::CreateAgentRequest agentRequest;
    //     agentRequest.set_simulation_id(myJPSSimulation);
    //     agentRequest.set_agent_model_class(myJPSModel);
    //     auto& agentParams = *agentRequest.mutable_agent_model_parameters()->mutable_fields();
    //     agentParams["journey_id"].set_number_value(static_cast<double>(state->getJourneyId()));
    //     agentParams["stage_id"].set_number_value(static_cast<double>(state->getStageId()));
    //     agentParams["desired_speed"].set_number_value(state->getPerson()->getMaxSpeed());
    //     if (myJPSModel != "GeneralizedCentrifugalForceModel") {
    //         agentParams["radius"].set_number_value(getRadius(type));
    //     }
    //     google::protobuf::ListValue* position = agentParams["position"].mutable_list_value();
    //     position->add_values()->set_number_value(p.x());
    //     position->add_values()->set_number_value(p.y());
    //     if (myJPSModel == "GeneralizedCentrifugalForceModel" || myJPSModel == "SocialForceModel") {
    //         const double angle = state->getAngle(*state->getStage(), 0);
    //         double orientation_x = 0.;
    //         double orientation_y = 0.;
    //         if (fabs(angle - M_PI / 2) < NUMERICAL_EPS) {
    //             orientation_y = 1.;
    //         } else if (fabs(angle + M_PI / 2) < NUMERICAL_EPS) {
    //             orientation_y = -1.;
    //         } else {
    //             orientation_x = 1.;
    //             orientation_y = tan(angle);
    //         }
    //         google::protobuf::ListValue* orientation = agentParams["orientation"].mutable_list_value();
    //         orientation->add_values()->set_number_value(orientation_x);
    //         orientation->add_values()->set_number_value(orientation_y);
    //     }
    //     grpc::ClientContext agentContext;
    //     sumo_jupedsim_api::CreateAgentResponse agentResponse;
    //     const grpc::Status agentStatus = myGrpcStub->CreateAgent(&agentContext, agentRequest, &agentResponse);
    //     if (agentStatus.ok()) {
    //         state->setAgentId(agentResponse.agent_id());
    //     } else {
    //         WRITE_WARNINGF(TL("Error while adding person '%' as JuPedSim agent: %"), state->getPerson()->getID(), agentStatus.error_message());
    //     }
    // #ifdef DEBUG_GRPC_TIMING
    //     myStopWatch[4].stop();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn try_pedestrian_insertion(&mut self, state: Ptr<MSPModel_Striping_PState>, p: &Position) {
        // shortcut: needs gRPC (tonic + prost) for CreateAgent, upgrade when tonic/prost are in Cargo.toml and protoc is installed
        panic!("MSPModel_JuPedSim::tryPedestrianInsertion needs gRPC (tonic + prost, protoc for tonic-build)")
    }

    /// C++: `JPS_StageId MSPModel_JuPedSim::addWaypoint(const std::string &, WaypointDesc &, const MSStageMoving *const)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_waypoint(&mut self, agentID: &str, waypoint: &mut MSPModel_JuPedSim_WaypointDesc, finalStage: Ptr<MSStageMoving>) -> MSPModel_JuPedSim_JPS_StageId {
        // shortcut: needs gRPC (tonic + prost) for AddWaypointStage, upgrade when tonic/prost are in Cargo.toml and protoc is installed
        panic!("MSPModel_JuPedSim::addWaypoint needs gRPC (tonic + prost, protoc for tonic-build)")
    }

    /// C++: `int * MSPModel_JuPedSim::createGeometryFromCenterLine(PositionVector, double, int)` [static, private]
    // GEOSGeometry*
    // MSPModel_JuPedSim::createGeometryFromCenterLine(PositionVector centerLine, double width, int capStyle) {
    //     GEOSCoordSequence* coordinateSequence = convertToGEOSPoints(centerLine);
    //     GEOSGeometry* lineString = GEOSGeom_createLineString(coordinateSequence);
    //     GEOSGeometry* dilatedLineString = GEOSBufferWithStyle(lineString, width, GEOS_QUADRANT_SEGMENTS, capStyle, GEOSBUF_JOIN_ROUND, GEOS_MITRE_LIMIT);
    //     GEOSGeom_destroy(lineString);
    //     return dilatedLineString;
    // }
    #[allow(unused_variables)]
    pub fn create_geometry_from_center_line(centerLine: PositionVector, width: f64, capStyle: i32) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSBufferWithStyle), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::createGeometryFromCenterLine needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim::createGeometryFromShape(PositionVector, std::string, std::string, bool)` [static, private]
    // GEOSGeometry*
    // MSPModel_JuPedSim::createGeometryFromShape(PositionVector shape, std::string junctionID, std::string shapeID, bool isInternalShape) {
    //     // Corner case.
    //     if (shape.size() == 1) {
    //         WRITE_WARNINGF(TL("Polygon '%' at junction '%' will be skipped as it is just a point."), shapeID, junctionID);
    //         return nullptr;
    //     }
    //     // Make sure the shape is closed.
    //     if (shape.back() != shape.front()) {
    //         shape.push_back(shape.front());
    //     }
    //     // Replace consecutive points that are equal with just one.
    //     PositionVector cleanShape;
    //     cleanShape.push_back(shape[0]);
    //     PositionVector duplicates;
    //     for (int i = 1; i < (int)shape.size(); i++) {
    //         if (shape[i] != shape[i - 1]) {
    //             cleanShape.push_back(shape[i]);
    //         } else {
    //             duplicates.push_back(shape[i]);
    //         }
    //     }
    //     if (cleanShape.size() < shape.size()) {
    //         WRITE_WARNINGF(TL("Polygon '%' at junction '%' had some equal consecutive points removed: %"), shapeID, junctionID, toString(duplicates, 9));
    //     }
    //     GEOSCoordSequence* coordinateSequence = convertToGEOSPoints(cleanShape);
    //     GEOSGeometry* linearRing = GEOSGeom_createLinearRing(coordinateSequence);
    //     GEOSGeometry* polygon = GEOSGeom_createPolygon(linearRing, nullptr, 0);
    //
    //     if (!GEOSisValid(polygon)) {
    //         if (cleanShape.size() == 3) {
    //             if (isInternalShape) {
    //                 WRITE_WARNINGF(TL("Polygon '%' at junction '%' has been dilated as it is just a segment."), shapeID, junctionID);
    //             } else {
    //                 WRITE_WARNINGF(TL("Polygon '%' has been dilated as it is just a segment."), shapeID);
    //             }
    //             GEOSGeometry* lineString = GEOSGeom_createLineString(GEOSCoordSeq_clone(coordinateSequence));
    //             GEOSGeom_destroy(polygon);
    //             polygon = GEOSBufferWithStyle(lineString, GEOS_BUFFERED_SEGMENT_WIDTH, GEOS_QUADRANT_SEGMENTS, GEOSBUF_CAP_FLAT, GEOSBUF_JOIN_BEVEL, GEOS_MITRE_LIMIT);
    //             GEOSGeom_destroy(lineString);
    //         } else {
    //             if (isInternalShape) {
    //                 WRITE_WARNINGF(TL("Polygon '%' at junction '%' has been replaced by its convex hull as it is not simple."), shapeID, junctionID);
    //                 GEOSGeometry* hull = GEOSConvexHull(polygon);
    //                 GEOSGeom_destroy(polygon);
    //                 polygon = GEOSBufferWithStyle(hull, POSITION_EPS, GEOS_QUADRANT_SEGMENTS, GEOSBUF_CAP_FLAT, GEOSBUF_JOIN_ROUND, GEOS_MITRE_LIMIT);;
    //             } else {
    //                 WRITE_WARNINGF(TL("Polygon '%' at junction '%' will be skipped as it is not simple."), shapeID, junctionID);
    //                 polygon = nullptr;
    //             }
    //         }
    //
    //         return polygon;
    //     }
    //
    //     // Some junctions need to be buffered a little so that edges are properly connected.
    //     if (isInternalShape) {
    //         polygon = GEOSBufferWithStyle(polygon, POSITION_EPS, GEOS_QUADRANT_SEGMENTS, GEOSBUF_CAP_FLAT, GEOSBUF_JOIN_ROUND, GEOS_MITRE_LIMIT);
    //     }
    //
    //     return polygon;
    // }
    #[allow(unused_variables)]
    pub fn create_geometry_from_shape(shape: PositionVector, junctionID: String, shapeID: String, isInternalShape: bool) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSisValid/GEOSConvexHull/GEOSBufferWithStyle), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::createGeometryFromShape needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim::buildPedestrianNetwork(MSNet *)` [private]
    // GEOSGeometry*
    // MSPModel_JuPedSim::buildPedestrianNetwork(MSNet* network) {
    //     std::vector<GEOSGeometry*> walkableAreas;
    //     for (const MSEdge* const edge : network->getEdgeControl().getEdges()) {
    //         const MSLane* const lane = getSidewalk<MSEdge, MSLane>(edge);
    //         if (lane != nullptr) {
    //             GEOSGeometry* dilatedLane = nullptr;
    //             if (edge->isWalkingArea()) {
    //                 dilatedLane = createGeometryFromShape(lane->getShape(), edge->getFromJunction()->getID(), edge->getID(), true);
    //             } else if (edge->isCrossing()) {
    //                 const PositionVector* outlineShape = lane->getOutlineShape();
    //                 if (outlineShape != nullptr) {
    //                     // this is mainly to ensure that we have at least the "normal" crossing, see #15037
    //                     dilatedLane = createGeometryFromCenterLine(lane->getShape(), lane->getWidth() / 2.0, GEOSBUF_CAP_FLAT);
    //                     if (dilatedLane != nullptr) {
    //                         walkableAreas.push_back(dilatedLane);
    //                     }
    //                     dilatedLane = createGeometryFromShape(*outlineShape, edge->getFromJunction()->getID(), edge->getID(), true);
    //                 } else {
    //                     dilatedLane = createGeometryFromCenterLine(lane->getShape(), lane->getWidth() / 2.0, GEOSBUF_CAP_ROUND);
    //                 }
    //                 myCrossingWaits[lane] = {0, 0};
    //             } else {  // regular sidewalk
    //                 dilatedLane = createGeometryFromCenterLine(lane->getShape(), lane->getWidth() / 2.0 + POSITION_EPS, GEOSBUF_CAP_FLAT);
    //             }
    //             if (dilatedLane != nullptr) {
    //                 walkableAreas.push_back(dilatedLane);
    //             }
    //         }
    //     }
    //     for (const auto& junctionWithID : network->getJunctionControl()) {
    //         const MSJunction* const junction = junctionWithID.second;
    //         int pedEdgeCount = 0;
    //         bool hasWalkingArea = false;
    //         for (const ConstMSEdgeVector& edges : {
    //                     junction->getIncoming(), junction->getOutgoing()
    //                 }) {
    //             for (const MSEdge* const edge : edges) {
    //                 if (edge->isWalkingArea()) {
    //                     hasWalkingArea = true;
    //                     break;
    //                 }
    //                 if (getSidewalk<MSEdge, MSLane>(edge) != nullptr) {
    //                     pedEdgeCount++;
    //                 }
    //             }
    //             if (hasWalkingArea) {
    //                 break;
    //             }
    //         }
    //         if (!hasWalkingArea && pedEdgeCount > 1) {
    //             // there is something to connect but no walking area, let's assume peds are allowed everywhere
    //             GEOSGeometry* walkingAreaGeom = createGeometryFromShape(junction->getShape(), junction->getID(), junction->getID(), true);
    //             if (walkingAreaGeom != nullptr) {
    //                 walkableAreas.push_back(walkingAreaGeom);
    //             }
    //         }
    //     }
    //
    //     // Retrieve additional walkable areas and obstacles (walkable areas and obstacles in the sense of JuPedSim).
    //     std::vector<GEOSGeometry*> additionalObstacles;
    //     for (const auto& polygonWithID : myNetwork->getShapeContainer().getPolygons()) {
    //         if (polygonWithID.second->getShapeType() == "jupedsim.walkable_area" || polygonWithID.second->getShapeType() == "taz") {
    //             GEOSGeometry* walkableArea = createGeometryFromShape(polygonWithID.second->getShape(), std::string(""), polygonWithID.first);
    //             if (walkableArea != nullptr) {
    //                 walkableAreas.push_back(walkableArea);
    //                 myHaveAdditionalWalkableAreas = true;
    //             }
    //         } else if (polygonWithID.second->getShapeType() == "jupedsim.obstacle") {
    //             GEOSGeometry* additionalObstacle = createGeometryFromShape(polygonWithID.second->getShape(), std::string(""), polygonWithID.first);
    //             if (additionalObstacle != nullptr) {
    //                 additionalObstacles.push_back(additionalObstacle);
    //             }
    //         }
    //     }
    //
    //     // Take the union of all walkable areas.
    //     GEOSGeometry* disjointWalkableAreas = GEOSGeom_createCollection(GEOS_MULTIPOLYGON, walkableAreas.data(), (unsigned int)walkableAreas.size());
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     // dumpGeometry(disjointWalkableAreas, "disjointWalkableAreas.wkt");
    // #endif
    //     GEOSGeometry* initialWalkableAreas = GEOSUnaryUnion(disjointWalkableAreas);
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     dumpGeometry(initialWalkableAreas, "initialWalkableAreas.wkt");
    // #endif
    //     GEOSGeom_destroy(disjointWalkableAreas);
    //
    //     // At last, remove additional obstacles from the merged walkable areas.
    //     GEOSGeometry* disjointAdditionalObstacles = GEOSGeom_createCollection(GEOS_MULTIPOLYGON, additionalObstacles.data(), (unsigned int)additionalObstacles.size());
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     // dumpGeometry(disjointAdditionalObstacles, "disjointAdditionalObstacles.wkt");
    // #endif
    //     GEOSGeometry* additionalObstaclesUnion = GEOSUnaryUnion(disjointAdditionalObstacles); // Obstacles may overlap, e.g. if they were loaded from separate files.
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     dumpGeometry(additionalObstaclesUnion, "additionalObstaclesUnion.wkt");
    // #endif
    //     GEOSGeometry* finalWalkableAreas = GEOSDifference(initialWalkableAreas, additionalObstaclesUnion);
    // #ifdef DEBUG_GEOMETRY_GENERATION
    //     dumpGeometry(finalWalkableAreas, "finalWalkableAreas.wkt");
    // #endif
    //     GEOSGeom_destroy(initialWalkableAreas);
    //     GEOSGeom_destroy(additionalObstaclesUnion);
    //     GEOSGeom_destroy(disjointAdditionalObstacles);
    //
    //     if (!GEOSisSimple(finalWalkableAreas)) {
    //         throw ProcessError(TL("Union of walkable areas minus union of obstacles is not a simple polygon."));
    //     }
    //
    //     return finalWalkableAreas;
    // }
    #[allow(unused_variables)]
    pub fn build_pedestrian_network(&mut self, network: Ptr<MSNet>) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSUnaryUnion/GEOSDifference/GEOSisSimple), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::buildPedestrianNetwork needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim::convertToGEOSPoints(PositionVector)` [static, private]
    // GEOSCoordSequence*
    // MSPModel_JuPedSim::convertToGEOSPoints(PositionVector shape) {
    //     GEOSCoordSequence* coordinateSequence = GEOSCoordSeq_create((unsigned int)shape.size(), 2);
    //     for (int i = 0; i < (int)shape.size(); i++) {
    //         GEOSCoordSeq_setX(coordinateSequence, i, shape[i].x());
    //         GEOSCoordSeq_setY(coordinateSequence, i, shape[i].y());
    //     }
    //     return coordinateSequence;
    // }
    #[allow(unused_variables)]
    pub fn convert_to_geos_points(shape: PositionVector) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSCoordSeq), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::convertToGEOSPoints needs GEOS (geos crate)")
    }

    /// C++: `void MSPModel_JuPedSim::convertToJPSPoints(const int *, int *)` [static, private]
    // void
    // MSPModel_JuPedSim::convertToJPSPoints(const GEOSGeometry* geometry, sumo_jupedsim_api::Polygon* into) {
    //     const GEOSCoordSequence* coordinateSequence = GEOSGeom_getCoordSeq(geometry);
    //     unsigned int coordinateSequenceSize;
    //     GEOSCoordSeq_getSize(coordinateSequence, &coordinateSequenceSize);
    //     double x;
    //     double y;
    //     // Remove the last point so that CGAL doesn't complain about the simplicity of the polygon downstream.
    //     for (unsigned int i = 0; i < coordinateSequenceSize - 1; i++) {
    //         GEOSCoordSeq_getX(coordinateSequence, i, &x);
    //         GEOSCoordSeq_getY(coordinateSequence, i, &y);
    //         sumo_jupedsim_api::Point* point = into->add_points();
    //         point->set_x(x);
    //         point->set_y(y);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn convert_to_jps_points(geometry: Ptr<i32>, into: Ptr<i32>) {
        // shortcut: needs GEOS (geos 11.3.1, reads coordinates) and protobuf sumo_jupedsim_api::Polygon (prost), upgrade once both are in Cargo.toml (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::convertToJPSPoints needs GEOS (geos crate) and protobuf Polygon (prost)")
    }

    /// C++: `PositionVector MSPModel_JuPedSim::convertToSUMOPoints(const int *)` [static, private]
    // PositionVector
    // MSPModel_JuPedSim::convertToSUMOPoints(const GEOSGeometry* geometry) {
    //     PositionVector coordinateVector;
    //     const GEOSCoordSequence* coordinateSequence = GEOSGeom_getCoordSeq(geometry);
    //     unsigned int coordinateSequenceSize;
    //     GEOSCoordSeq_getSize(coordinateSequence, &coordinateSequenceSize);
    //     double x;
    //     double y;
    //     for (unsigned int i = 0; i < coordinateSequenceSize; i++) {
    //         GEOSCoordSeq_getX(coordinateSequence, i, &x);
    //         GEOSCoordSeq_getY(coordinateSequence, i, &y);
    //         coordinateVector.push_back(Position(x, y));
    //     }
    //     return coordinateVector;
    // }
    #[allow(unused_variables)]
    pub fn convert_to_sumo_points(geometry: Ptr<i32>) -> PositionVector {
        // shortcut: needs GEOS (geos 11.3.1, reads coordinates), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::convertToSUMOPoints needs GEOS (geos crate)")
    }

    /// C++: `double MSPModel_JuPedSim::getLinearRingArea(const int *)` [static, private]
    // double
    // MSPModel_JuPedSim::getLinearRingArea(const GEOSGeometry* linearRing) {
    //     double area;
    //     GEOSGeometry* linearRingAsPolygon = GEOSGeom_createPolygon(GEOSGeom_clone(linearRing), nullptr, 0);
    //     GEOSArea(linearRingAsPolygon, &area);
    //     GEOSGeom_destroy(linearRingAsPolygon);
    //     return area;
    // }
    #[allow(unused_variables)]
    pub fn get_linear_ring_area(linearRing: Ptr<i32>) -> f64 {
        // shortcut: needs GEOS (geos 11.3.1, GEOSArea), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::getLinearRingArea needs GEOS (geos crate)")
    }

    /// C++: `void MSPModel_JuPedSim::removePolygonFromDrawing(const std::string &)` [private]
    // void
    // MSPModel_JuPedSim::removePolygonFromDrawing(const std::string& polygonId) {
    //     myShapeContainer.removePolygon(polygonId);
    // }
    #[allow(unused_variables)]
    pub fn remove_polygon_from_drawing(&mut self, polygonId: &str) {
        self.myShapeContainer.m().remove_polygon(polygonId, true);
    }

    /// C++: `void MSPModel_JuPedSim::preparePolygonForDrawing(const int *, const std::string &, const RGBColor &)` [private]
    // void
    // MSPModel_JuPedSim::preparePolygonForDrawing(const GEOSGeometry* polygon, const std::string& polygonId, const RGBColor& color) {
    //     const GEOSGeometry* exterior = GEOSGetExteriorRing(polygon);
    //     bool added = myShapeContainer.addPolygon(polygonId, std::string("jupedsim.pedestrian_network"), color, 10.0, 0.0,
    //                  std::string(), convertToSUMOPoints(exterior), false, true, 1.0);
    //     if (added) {
    //         std::vector<PositionVector> holes;
    //         int nbrInteriorRings = GEOSGetNumInteriorRings(polygon);
    //         if (nbrInteriorRings != -1) {
    //             for (int k = 0; k < nbrInteriorRings; k++) {
    //                 const GEOSGeometry* linearRing = GEOSGetInteriorRingN(polygon, k);
    //                 double area = getLinearRingArea(linearRing);
    //                 if (area > GEOS_MIN_AREA) {
    //                     holes.push_back(convertToSUMOPoints(linearRing));
    //                 }
    //             }
    //             myShapeContainer.getPolygons().get(polygonId)->setHoles(holes);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prepare_polygon_for_drawing(&mut self, polygon: Ptr<i32>, polygonId: &str, color: &RGBColor) {
        // shortcut: needs GEOS (geos 11.3.1, GEOSGetExteriorRing/GEOSGetInteriorRingN), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::preparePolygonForDrawing needs GEOS (geos crate)")
    }

    /// C++: `const int * MSPModel_JuPedSim::getLargestComponent(const int *, int &, double &, double &)` [static, private]
    // const GEOSGeometry*
    // MSPModel_JuPedSim::getLargestComponent(const GEOSGeometry* polygon, int& nbrComponents, double& maxArea, double& totalArea) {
    //     nbrComponents = GEOSGetNumGeometries(polygon);
    //     const GEOSGeometry* largestComponent = nullptr;
    //     maxArea = 0.0;
    //     totalArea = 0.0;
    //     for (int i = 0; i < nbrComponents; i++) {
    //         const GEOSGeometry* componentPolygon = GEOSGetGeometryN(polygon, i);
    //         double area;
    //         GEOSArea(componentPolygon, &area);
    //         totalArea += area;
    //         if (area > maxArea) {
    //             maxArea = area;
    //             largestComponent = componentPolygon;
    //         }
    //     }
    //     return largestComponent;
    // }
    #[allow(unused_variables)]
    pub fn get_largest_component(polygon: Ptr<i32>, nbrComponents: &mut i32, maxArea: &mut f64, totalArea: &mut f64) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSGetNumGeometries/GEOSArea), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::getLargestComponent needs GEOS (geos crate)")
    }

    /// C++: `int64_t MSPModel_JuPedSim::buildJPSGeometryFromGEOSGeometry(const int *)` [private]
    // int64_t
    // MSPModel_JuPedSim::buildJPSGeometryFromGEOSGeometry(const GEOSGeometry* polygon) {
    //     sumo_jupedsim_api::CreateGeometryRequest geometryRequest;
    //     sumo_jupedsim_api::Polygon* boundary = geometryRequest.mutable_boundary();
    //
    //     // Handle the exterior polygon.
    //     const GEOSGeometry* exterior =  GEOSGetExteriorRing(polygon);
    //     convertToJPSPoints(exterior, boundary);
    //
    //     // Handle the interior polygons (holes).
    //     int nbrInteriorRings = GEOSGetNumInteriorRings(polygon);
    //     if (nbrInteriorRings != -1) {
    //         for (int k = 0; k < nbrInteriorRings; k++) {
    //             const GEOSGeometry* linearRing = GEOSGetInteriorRingN(polygon, k);
    //             double area = getLinearRingArea(linearRing);
    //             if (area > GEOS_MIN_AREA) {
    //                 sumo_jupedsim_api::Polygon* hole = geometryRequest.add_obstacles();
    //                 convertToJPSPoints(linearRing, hole);
    //             }
    //         }
    //     }
    //     grpc::ClientContext geometryContext;
    //     sumo_jupedsim_api::CreateGeometryResponse geometryResponse;
    //     const grpc::Status geometryStatus = myGrpcStub->CreateGeometry(&geometryContext, geometryRequest, &geometryResponse);
    //     if (!geometryStatus.ok()) {
    //         WRITE_WARNINGF(TL("JuPedSim gRPC CreateGeometry failed: %"), geometryStatus.error_message());
    //         return -1;
    //     }
    //
    //     return geometryResponse.geometry_id();
    // }
    #[allow(unused_variables)]
    pub fn build_jps_geometry_from_geos_geometry(&mut self, polygon: Ptr<i32>) -> i64 {
        // shortcut: needs GEOS (geos 11.3.1, reads rings) and gRPC CreateGeometry (tonic + prost), upgrade once both are wired (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::buildJPSGeometryFromGEOSGeometry needs GEOS (geos crate) and gRPC CreateGeometry")
    }

    /// C++: `void MSPModel_JuPedSim::dumpGeometry(const int *, const std::string &, bool)` [static, private]
    // void
    // MSPModel_JuPedSim::dumpGeometry(const GEOSGeometry* polygon, const std::string& filename, bool useGeoCoordinates) {
    //     GEOSGeometry* polygonGeoCoordinates = nullptr;
    //     if (useGeoCoordinates) {
    //         const GEOSGeometry* exterior =  GEOSGetExteriorRing(polygon);
    //         PositionVector exteriorPoints = convertToSUMOPoints(exterior);
    //         for (Position& position : exteriorPoints) {
    //             GeoConvHelper::getFinal().cartesian2geo(position);
    //         }
    //         const int nbrInteriorRings = GEOSGetNumInteriorRings(polygon);
    //         std::vector<GEOSGeometry*> holeRings;
    //         if (nbrInteriorRings != -1) {
    //             for (int k = 0; k < nbrInteriorRings; k++) {
    //                 const GEOSGeometry* linearRing = GEOSGetInteriorRingN(polygon, k);
    //                 PositionVector holePoints = convertToSUMOPoints(linearRing);
    //                 for (Position& position : holePoints) {
    //                     GeoConvHelper::getFinal().cartesian2geo(position);
    //                 }
    //                 GEOSCoordSequence* holeRingCoords = convertToGEOSPoints(holePoints);
    //                 GEOSGeometry* holeRing = GEOSGeom_createLinearRing(holeRingCoords);
    //                 holeRings.push_back(holeRing);
    //             }
    //         }
    //         GEOSCoordSequence* exteriorRingCoords = convertToGEOSPoints(exteriorPoints);
    //         GEOSGeometry* exteriorRing = GEOSGeom_createLinearRing(exteriorRingCoords);
    //         polygonGeoCoordinates = GEOSGeom_createPolygon(exteriorRing, holeRings.data(), nbrInteriorRings);
    //     }
    //     std::ofstream dumpFile;
    //     dumpFile.open(filename);
    //     GEOSWKTWriter* writer = GEOSWKTWriter_create();
    //     GEOSWKTWriter_setRoundingPrecision(writer, gPrecisionGeo);
    //     char* wkt = GEOSWKTWriter_write(writer, polygonGeoCoordinates == nullptr ? polygon : polygonGeoCoordinates);
    //     dumpFile << wkt << std::endl;
    //     dumpFile.close();
    //     GEOSFree(wkt);
    //     GEOSWKTWriter_destroy(writer);
    //     GEOSGeom_destroy(polygonGeoCoordinates);
    // }
    #[allow(unused_variables)]
    pub fn dump_geometry(polygon: Ptr<i32>, filename: &str, useGeoCoordinates: bool) {
        // shortcut: needs GEOS (geos 11.3.1, GEOSWKTWriter), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::dumpGeometry needs GEOS (geos crate)")
    }

    /// C++: `double MSPModel_JuPedSim::getRadius(const MSVehicleType &)` [static, private]
    // double
    // MSPModel_JuPedSim::getRadius(const MSVehicleType& vehType) {
    //     return 0.5 * MAX2(vehType.getLength(), vehType.getWidth());
    // }
    #[allow(unused_variables)]
    pub fn get_radius(vehType: &MSVehicleType) -> f64 {
        let length = vehType.get_length();
        let width = vehType.get_width();
        0.5 * (if length > width { length } else { width })
    }

    /// C++: `JPS_StageId MSPModel_JuPedSim::addWaitingSet(const MSLane *const, const bool)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_waiting_set(&mut self, crossing: Ptr<MSLane>, entry: bool) -> MSPModel_JuPedSim_JPS_StageId {
        // shortcut: needs GEOS (geos 11.3.1, GEOSContains on the pedestrian network) and gRPC AddWaitingSetStage (tonic + prost), upgrade once both are wired (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim::addWaitingSet needs GEOS (geos crate) and gRPC AddWaitingSetStage")
    }

    /// C++: `Response MSPModel_JuPedSim::callGrpc(int (*)(int *, const Request &, Response *), Request &, const std::string &, const bool)` [private]
    // template <class Request, class Response>
    //     Response
    //     callGrpc(grpc::Status(sumo_jupedsim_api::JuPedSimService::Stub::*method)(grpc::ClientContext*, const Request&, Response*),
    //              Request& request, const std::string& what, const bool warnOnly = false) {
    //         myStopWatch[0].start();
    //         grpc::ClientContext context;
    //         Response response;
    //         request.set_simulation_id(myJPSSimulation);
    //         const grpc::Status status = ((*myGrpcStub).*method)(&context, request, &response);
    //         myStopWatch[0].stop();
    //         if (!status.ok()) {
    //             if (!warnOnly) {
    //                 throw ProcessError(what + status.error_message());
    //             }
    //             WRITE_WARNING(what + status.error_message());
    //         }
    //         return response;
    //     }
    #[allow(unused_variables)]
    pub fn call_grpc(&mut self) -> Unmapped /* Response */ {
        // shortcut: needs gRPC (tonic + prost) for every JuPedSimService stub call, upgrade when tonic/prost are in Cargo.toml and protoc is installed
        panic!("MSPModel_JuPedSim::callGrpc needs gRPC (tonic + prost, protoc for tonic-build)")
    }

}

/// Holds pedestrian state and performs updates
/// C++ class `MSPModel_JuPedSim::PState` (microsim/transportables/MSPModel_JuPedSim.h)
#[repr(C)]
pub struct MSPModel_JuPedSim_PState {
    /// C++ base class `MSPModel_InteractingState`
    pub base: MSPModel_InteractingState,
    /// id of the journey, needed for modifying it
    pub myJourneyId: MSPModel_JuPedSim_JPS_JourneyId,
    pub myStageId: MSPModel_JuPedSim_JPS_StageId,
    pub myWaypoints: Vec<MSPModel_JuPedSim_WaypointDesc>,
    pub myAgentId: MSPModel_JuPedSim_JPS_AgentId,
}

impl std::ops::Deref for MSPModel_JuPedSim_PState {
    type Target = MSPModel_InteractingState;
    fn deref(&self) -> &MSPModel_InteractingState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim_PState {
    fn deref_mut(&mut self) -> &mut MSPModel_InteractingState {
        &mut self.base
    }
}

impl MSPModel_JuPedSim_PState {
    /// C++: `void PState::PState(MSPerson *, MSStageMoving *, JPS_JourneyId, JPS_StageId, const int &)`
    // MSPModel_JuPedSim::PState::PState(MSPerson* person, MSStageMoving* stage,
    //                                   JPS_JourneyId journeyId, JPS_StageId stageId,
    //                                   const std::vector<WaypointDesc>& waypoints) :
    //     MSPModel_InteractingState(person, stage, nullptr),
    //     myJourneyId(journeyId), myStageId(stageId), myWaypoints(waypoints), myAgentId(0) {
    //     myDir = FORWARD;
    // }
    #[allow(unused_variables)]
    pub fn new(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, journeyId: MSPModel_JuPedSim_JPS_JourneyId, stageId: MSPModel_JuPedSim_JPS_StageId, waypoints: &[MSPModel_JuPedSim_WaypointDesc]) -> Self {
        let mut state = Self {
            base: MSPModel_InteractingState::new(person, stage, Ptr::null()),
            myJourneyId: journeyId,
            myStageId: stageId,
            myWaypoints: waypoints.to_vec(),
            myAgentId: 0,
        };
        state.myDir = 1; // MSPModel::FORWARD
        state
    }

    /// C++: `void PState::~PState() noexcept` [virtual]
    // MSPModel_JuPedSim::PState::~PState() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {

    }

    /// C++: `void PState::reinit(MSStageMoving *, JPS_JourneyId, JPS_StageId, const int &)`
    // void
    // MSPModel_JuPedSim::PState::reinit(MSStageMoving* stage, JPS_JourneyId journeyId, JPS_StageId stageId,
    //                                   const std::vector<WaypointDesc>& waypoints) {
    //     if (myStage != nullptr) {
    //         myStage->setPState(nullptr);  // we need to remove the old state reference to avoid double deletion
    //     }
    //     myStage = stage;
    //     myJourneyId = journeyId;
    //     myStageId = stageId;
    //     myWaypoints = waypoints;
    // }
    #[allow(unused_variables)]
    pub fn reinit(&mut self, stage: Ptr<MSStageMoving>, journeyId: MSPModel_JuPedSim_JPS_JourneyId, stageId: MSPModel_JuPedSim_JPS_StageId, waypoints: &[MSPModel_JuPedSim_WaypointDesc]) {
        if !self.myStage.is_null() {
            self.myStage.m().set_p_state(Ptr::null());
        }
        self.myStage = stage;
        self.myJourneyId = journeyId;
        self.myStageId = stageId;
        self.myWaypoints = waypoints.to_vec();
    }

    /// C++: `Position PState::getPosition(const MSStageMoving &, SUMOTime) const` [const, virtual]
    // inline Position getPosition(const MSStageMoving&, SUMOTime) const override {
    //             return myRemoteXYPos;
    //         }
    #[allow(unused_variables)]
    pub fn get_position(&self, arg0: &MSStageMoving, arg1: SUMOTime) -> Position {
        self.myRemoteXYPos
    }

    /// C++: `void PState::setPosition(const double, const double, const double)`
    // void MSPModel_JuPedSim::PState::setPosition(const double x, const double y, const double z) {
    //     if (myRemoteXYPos != Position::INVALID) {
    //         mySpeed = myRemoteXYPos.distanceTo2D(Position(x, y, z)) / STEPS2TIME(DELTA_T);
    //     } else {
    //         mySpeed = 0.;
    //     }
    //     myRemoteXYPos.set(x, y, z);
    // }
    #[allow(unused_variables)]
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        if self.myRemoteXYPos != Position::INVALID {
            self.mySpeed = self.myRemoteXYPos.distance_to2d(&Position::new3(x, y, z)) / utils::common::time::steps2time(utils::common::time::delta_t());
        } else {
            self.mySpeed = 0.;
        }
        self.myRemoteXYPos.set3(x, y, z);
    }

    /// C++: `void PState::setAngle(double)`
    // inline void setAngle(double angle) {
    //             myAngle = angle;
    //         }
    #[allow(unused_variables)]
    pub fn set_angle(&mut self, angle: f64) {
        self.myAngle.set(angle);
    }

    /// C++: `void PState::setStage(MSStageMoving *const)`
    // inline void setStage(MSStageMoving* const stage) {
    //             myStage = stage;
    //         }
    #[allow(unused_variables)]
    pub fn set_stage(&mut self, stage: Ptr<MSStageMoving>) {
        self.myStage = stage;
    }

    /// C++: `void PState::setLane(MSLane *)`
    // inline void setLane(MSLane* lane) {
    //             myLane = lane;
    //         }
    #[allow(unused_variables)]
    pub fn set_lane(&mut self, lane: Ptr<MSLane>) {
        self.myLane = lane;
    }

    /// C++: `void PState::setLanePosition(double)`
    // inline void setLanePosition(double lanePosition) {
    //             myEdgePos = lanePosition;
    //         }
    #[allow(unused_variables)]
    pub fn set_lane_position(&mut self, lanePosition: f64) {
        self.myEdgePos = lanePosition;
    }

    /// C++: `const MSEdge * PState::getNextEdge(const MSStageMoving &) const` [const, virtual]
    // const MSEdge* MSPModel_JuPedSim::PState::getNextEdge(const MSStageMoving& stage) const {
    //     return stage.getNextRouteEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        stage.get_next_route_edge()
    }

    /// C++: `const MSPModel_JuPedSim::WaypointDesc * PState::getNextWaypoint(const int) const` [const]
    // const MSPModel_JuPedSim::WaypointDesc*
    // MSPModel_JuPedSim::PState::getNextWaypoint(const int offset) const {
    //     return offset < (int)myWaypoints.size() ? &myWaypoints[offset] : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_waypoint(&self, offset: i32) -> Ptr<MSPModel_JuPedSim_WaypointDesc> {
        if (offset as usize) < self.myWaypoints.len() {
            Ptr::new(&self.myWaypoints[offset as usize])
        } else {
            Ptr::null()
        }
    }

    /// C++: `JPS_AgentId PState::getAgentId() const` [const]
    // inline JPS_AgentId getAgentId() const {
    //             return myAgentId;
    //         }
    #[allow(unused_variables)]
    pub fn get_agent_id(&self) -> MSPModel_JuPedSim_JPS_AgentId {
        self.myAgentId
    }

    /// C++: `void PState::setAgentId(JPS_AgentId)`
    // void setAgentId(JPS_AgentId id) {
    //             myAgentId = id;
    //             myWaitingToEnter = false;
    //         }
    #[allow(unused_variables)]
    pub fn set_agent_id(&mut self, id: MSPModel_JuPedSim_JPS_AgentId) {
        self.myAgentId = id;
        self.myWaitingToEnter = false;
    }

    /// C++: `bool PState::isFinished() const` [const, virtual]
    /// whether the transportable has finished walking
    // bool isFinished() const override {
    //             return myWaypoints.empty();
    //         }
    #[allow(unused_variables)]
    pub fn is_finished(&self) -> bool {
        self.myWaypoints.is_empty()
    }

    /// C++: `JPS_JourneyId PState::getJourneyId() const` [const]
    // JPS_JourneyId getJourneyId() const {
    //             return myJourneyId;
    //         }
    #[allow(unused_variables)]
    pub fn get_journey_id(&self) -> MSPModel_JuPedSim_JPS_JourneyId {
        self.myJourneyId
    }

    /// C++: `JPS_StageId PState::getStageId() const` [const]
    /// first stage of the journey
    // JPS_StageId getStageId() const {
    //             return myStageId;
    //         }
    #[allow(unused_variables)]
    pub fn get_stage_id(&self) -> MSPModel_JuPedSim_JPS_StageId {
        self.myStageId
    }

    /// C++: `bool PState::advanceNextWaypoint()`
    // bool advanceNextWaypoint() {
    //             myWaypoints.erase(myWaypoints.begin());
    //             return myWaypoints.empty();
    //         }
    #[allow(unused_variables)]
    pub fn advance_next_waypoint(&mut self) -> bool {
        self.myWaypoints.remove(0);
        self.myWaypoints.is_empty()
    }

}

