//! Port of microsim/transportables/MSPModel_JuPedSim_legacy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSPModel_JuPedSim_legacy::JPS_Model`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSPModel_JuPedSim_legacy_JPS_Model {
    CollisionFreeSpeed = 0,
    CollisionFreeSpeedV2 = 1,
    GeneralizedCentrifugalForce = 2,
    SocialForce = 3,
}

/// C++ typedef `std::tuple<JPS_StageId, Position, double> MSPModel_JuPedSim_legacy::WaypointDesc`
pub type MSPModel_JuPedSim_legacy_WaypointDesc = (i64, Position, f64);

/// Structure that keeps data related to vanishing areas (and other types of areas).
/// C++ class `MSPModel_JuPedSim_legacy::AreaData` (microsim/transportables/MSPModel_JuPedSim_legacy.h)
pub struct MSPModel_JuPedSim_legacy_AreaData {
    pub id: String,
    pub areaType: String,
    /// C++ `std::vector<JPS_Point>`, JPS_Point is {x, y}
    pub areaBoundary: Vec<(f64, f64)>,
    /// C++ `const Parameterised::Map&` (non-owning)
    pub params: Ptr<ParameterisedMap>,
    /// The last time a pedestrian was removed in a vanishing area.
    pub lastRemovalTime: SUMOTime,
}

impl MSPModel_JuPedSim_legacy_AreaData {
}

/// C++ class `MSPModel_JuPedSim_legacy::Event` (microsim/transportables/MSPModel_JuPedSim_legacy.h)
#[repr(C)]
pub struct MSPModel_JuPedSim_legacy_Event {
    /// C++ base class `Command`
    // shortcut: Command has no Rust type yet, keep () until microsim Command is ported
    pub base: Unmapped /* Command */,
    pub myModel: Ptr<MSPModel_JuPedSim_legacy>,
}

impl std::ops::Deref for MSPModel_JuPedSim_legacy_Event {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim_legacy_Event {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSPModel_JuPedSim_legacy_Event {
    /// C++: `void Event::Event(MSPModel_JuPedSim_legacy *)`
    // explicit Event(MSPModel_JuPedSim_legacy* model)
    //             : myModel(model) { }
    #[allow(unused_variables)]
    pub fn new(model: Ptr<MSPModel_JuPedSim_legacy>) -> Self {
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
/// C++ class `MSPModel_JuPedSim_legacy` (microsim/transportables/MSPModel_JuPedSim_legacy.h)
#[repr(C)]
pub struct MSPModel_JuPedSim_legacy {
    /// C++ base class `MSPModel_Interacting`
    pub base: MSPModel_Interacting,
    /// C++ base class `ShapeListener`
    pub base2: ShapeListener,
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
    pub myPedestrianStates: Vec<Ptr<MSPModel_JuPedSim_legacy_PState>>,
    /// The GEOS polygon containing all computed connected components of the pedestrian network.
    // shortcut: owned GEOSGeometry* -> geos::Geometry (crate geos 11.3.1), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
    pub myGEOSPedestrianNetwork: Option<Box<dyn std::any::Any>>,
    /// The GEOS polygon representing the largest (by area) connected component of the pedestrian network.
    // shortcut: non-owning const GEOSGeometry* -> Ptr<geos::Geometry>, upgrade with the geos crate
    pub myGEOSPedestrianNetworkLargestComponent: Option<Ptr<dyn std::any::Any>>,
    pub myHaveAdditionalWalkableAreas: bool,
    /// The JPS polygon representing the largest connected component of the pedestrian network.
    // shortcut: JPS_Geometry is an opaque jupedsim.h pointer; no jupedsim crate found on crates.io, bindgen over jupedsim/jupedsim.h when wiring it
    pub myJPSGeometry: Option<Box<dyn std::any::Any>>,
    /// The JPS polygon representing the largest connected component plus carriages and ramps.
    // shortcut: same as myJPSGeometry (JPS_Geometry opaque pointer), upgrade with the jupedsim bindings
    pub myJPSGeometryWithTrainsAndRamps: Option<Box<dyn std::any::Any>>,
    pub myJPSModel: MSPModel_JuPedSim_JPS_Model,
    // shortcut: JPS_OperationalModel opaque pointer, upgrade with the jupedsim bindings
    pub myJPSOperationalModel: Option<Box<dyn std::any::Any>>,
    // shortcut: JPS_Simulation opaque pointer, upgrade with the jupedsim bindings
    pub myJPSSimulation: Option<Box<dyn std::any::Any>>,
    pub myPythonScript: Ptr<OutputDevice>,
    /// Array of special areas.
    pub myAreas: Vec<Box<MSPModel_JuPedSim_legacy_AreaData>>,
    /// Array of stopped trains, used to detect whether to add carriages and ramps to the geometry.
    pub myAllStoppedTrainIDs: Vec<SUMOTrafficObject_NumericalID>,
    // determinism: C++ std::map ordered by ComparatorNumericalIdLess (lane numerical id); a Vec keeps insertion order, sort by lane id when iterating
    pub myCrossingWaits: Vec<(Ptr<MSLane>, (i64, i64))>,
    pub myCrossings: BTreeMap<i64, Ptr<MSLane>>,
}

impl std::ops::Deref for MSPModel_JuPedSim_legacy {
    type Target = MSPModel_Interacting;
    fn deref(&self) -> &MSPModel_Interacting {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim_legacy {
    fn deref_mut(&mut self) -> &mut MSPModel_Interacting {
        &mut self.base
    }
}

// C++ static members of MSPModel_JuPedSim_legacy (port as module-level statics, see AGENTS.md):
//   static const int MSPModel_JuPedSim_legacy::GEOS_QUADRANT_SEGMENTS;
//   static const double MSPModel_JuPedSim_legacy::GEOS_MITRE_LIMIT;
//   static const double MSPModel_JuPedSim_legacy::GEOS_MIN_AREA;
//   static const double MSPModel_JuPedSim_legacy::GEOS_BUFFERED_SEGMENT_WIDTH;
//   static const RGBColor MSPModel_JuPedSim_legacy::PEDESTRIAN_NETWORK_COLOR;
//   static const RGBColor MSPModel_JuPedSim_legacy::PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_COLOR;
//   static const std::string MSPModel_JuPedSim_legacy::PEDESTRIAN_NETWORK_ID;
//   static const std::string MSPModel_JuPedSim_legacy::PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_ID;
//   static const int MSPModel_JuPedSim_legacy::noPedestrians;

impl MSPModel_JuPedSim_legacy {
    /// C++: `void MSPModel_JuPedSim_legacy::MSPModel_JuPedSim_legacy(const OptionsCont &, MSNet *)`
    // MSPModel_JuPedSim_legacy::MSPModel_JuPedSim_legacy(const OptionsCont& oc, MSNet* net) :
    //     myRNG("JuPedSim_legacy"), myNetwork(net), myShapeContainer(net->getShapeContainer()), myJPSDeltaT(string2time(oc.getString("pedestrian.jupedsim.step-length"))),
    //     myExitTolerance(oc.getFloat("pedestrian.jupedsim.exit-tolerance")), myGEOSPedestrianNetworkLargestComponent(nullptr),
    //     myHaveAdditionalWalkableAreas(false) {
    //     RandHelper::initRandGlobal(&myRNG);
    //     const std::string model = oc.getString("pedestrian.jupedsim.model");
    //     if (model == "CollisionFreeSpeed" || model == "CollisionFreeSpeedModel") {
    //         myJPSModel = JPS_Model::CollisionFreeSpeed;
    //     } else if (model == "CollisionFreeSpeedV2" || model == "CollisionFreeSpeedModelV2") {
    //         myJPSModel = JPS_Model::CollisionFreeSpeedV2;
    //     } else if (model == "GeneralizedCentrifugalForce" || model == "GeneralizedCentrifugalForceModel") {
    //         myJPSModel = JPS_Model::GeneralizedCentrifugalForce;
    //     } else if (model == "SocialForce" || model == "SocialForceModel") {
    //         myJPSModel = JPS_Model::SocialForce;
    //     } else {
    //         throw ProcessError(TLF("Unknown JuPedSim model '%'.", model));
    //     }
    //     initialize(oc);
    //     net->getBeginOfTimestepEvents()->addEvent(new Event(this), net->getCurrentTimeStep() + DELTA_T);
    // }
    #[allow(unused_variables)]
    pub fn new(oc: &OptionsCont, net: Ptr<MSNet>) -> Self {
        // shortcut: needs the JuPedSim C API (jupedsim/jupedsim.h, no crate chosen, bindgen planned) for the operational model and simulation, and GEOS via initialize(), upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::new needs the JuPedSim C API (jupedsim.h) and GEOS (geos crate)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::~MSPModel_JuPedSim_legacy() noexcept` [virtual]
    // MSPModel_JuPedSim_legacy::~MSPModel_JuPedSim_legacy() {
    //     clearState();
    //     if (myPythonScript != nullptr) {
    //         (*myPythonScript) << "while simulation.agent_count() > 0 and simulation.iteration_count() < 160 * 100: simulation.iterate()\n";
    //     }
    //
    //     JPS_Simulation_Free(myJPSSimulation);
    //     JPS_OperationalModel_Free(myJPSOperationalModel);
    //     JPS_Geometry_Free(myJPSGeometry);
    //     if (myJPSGeometryWithTrainsAndRamps != nullptr) {
    //         JPS_Geometry_Free(myJPSGeometryWithTrainsAndRamps);
    //     }
    //
    //     GEOSGeom_destroy(myGEOSPedestrianNetwork);
    //     finishGEOS();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: needs GEOS (geos 11.3.1, GEOSGeom_destroy/finishGEOS) and JuPedSim C API frees (JPS_Simulation_Free, JPS_Geometry_Free), upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::~MSPModel_JuPedSim_legacy needs GEOS (geos crate) and the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `MSTransportableStateAdapter * MSPModel_JuPedSim_legacy::add(MSTransportable *, MSStageMoving *, SUMOTime)` [virtual]
    // MSTransportableStateAdapter*
    // MSPModel_JuPedSim_legacy::add(MSTransportable* person, MSStageMoving* stage, SUMOTime now) {
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
    //                                 waypoints.push_back({waitingStage, wa->getLanes()[0]->getShape().getCentroid(), wa->getWidth() / 2.});
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
    //         waypoints.push_back({waitingStage, lane->getShape().positionAtOffset(e->getLength() / 2.), lane->getWidth() / 2.});
    //         prev = e;
    //     }
    //     const JPS_StageId finalWait = std::get<0>(waypoints.back());
    //     waypoints.pop_back();  // arrival waypoint comes later
    //     if (!waypoints.empty()) {
    //         waypoints.erase(waypoints.begin());  // departure edge also does not need a waypoint
    //     }
    //     const MSLane* const arrivalLane = getSidewalk<MSEdge, MSLane>(stage->getDestination());
    //     const Position arrivalPosition = arrivalLane->getShape().positionAtOffset(stage->getArrivalPos());
    //     waypoints.push_back({finalWait, arrivalPosition, stage->getDouble("jupedsim.waypoint.radius", myExitTolerance)});
    //
    //     JPS_JourneyDescription journeyDesc = JPS_JourneyDescription_Create();
    //     if (myPythonScript != nullptr) {
    //         (*myPythonScript) << "\njourney = jps.JourneyDescription()\n";
    //     }
    //     JPS_StageId startingStage = 0;
    //     JPS_StageId predecessor = 0;
    //     for (const auto& p : waypoints) {
    //         if (!addWaypoint(journeyDesc, predecessor, person->getID(), p)) {
    //             JPS_JourneyDescription_Free(journeyDesc);
    //             return nullptr;
    //         }
    //         if (startingStage == 0) {
    //             startingStage = std::get<0>(waypoints.front()) == 0 ? predecessor : std::get<0>(waypoints.front());
    //         }
    //     }
    //     JPS_ErrorMessage message = nullptr;
    //     JPS_JourneyId journeyId = JPS_Simulation_AddJourney(myJPSSimulation, journeyDesc, &message);
    //     JPS_JourneyDescription_Free(journeyDesc);
    //     if (message != nullptr) {
    //         WRITE_WARNINGF(TL("Error while adding a journey for person '%': %"), person->getID(), JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         return nullptr;
    //     }
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
    //         if (myPythonScript != nullptr) {
    //             (*myPythonScript) << "journey_id = simulation.add_journey(journey)\n"
    //                               "simulation.add_agent(jps.CollisionFreeSpeedModelAgentParameters(journey_id=journey_id,stage_id="
    //                               << startingStage << ",position=(" << departurePosition.x() << "," << departurePosition.y()
    //                               << "),radius=" << getRadius(person->getVehicleType()) << ",v0=" << person->getMaxSpeed() << "))\n";
    //         }
    //     } else {
    //         JPS_Simulation_SwitchAgentJourney(myJPSSimulation, state->getAgentId(), journeyId, startingStage, &message);
    //         if (message != nullptr) {
    //             WRITE_WARNINGF(TL("Error while switching to a new journey for person '%': %"), person->getID(), JPS_ErrorMessage_GetMessage(message));
    //             JPS_ErrorMessage_Free(message);
    //             return nullptr;
    //         }
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, person: Ptr<MSTransportable>, stage: Ptr<MSStageMoving>, now: SUMOTime) -> Ptr<MSTransportableStateAdapter> {
        // shortcut: needs the JuPedSim C API (jupedsim.h: AddJourney, AddAgent, SwitchAgentJourney), upgrade once the C API is bound via bindgen
        panic!("MSPModel_JuPedSim_legacy::add needs the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::remove(MSTransportableStateAdapter *)` [virtual]
    // void
    // MSPModel_JuPedSim_legacy::remove(MSTransportableStateAdapter* state) {
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
        panic!("MSPModel_JuPedSim_legacy::remove needs the myActiveLanes member, missing from the generated struct")
    }

    /// C++: `SUMOTime MSPModel_JuPedSim_legacy::execute(SUMOTime)`
    // SUMOTime
    // MSPModel_JuPedSim_legacy::execute(SUMOTime time) {
    //     const int nbrIterations = (int)(DELTA_T / myJPSDeltaT);
    //     JPS_ErrorMessage message = nullptr;
    //     for (int i = 0; i < nbrIterations; ++i) {
    //         // Perform one JuPedSim iteration.
    //         bool ok = JPS_Simulation_Iterate(myJPSSimulation, &message);
    //         if (!ok) {
    //             WRITE_ERRORF(TL("Error during iteration %: %"), i, JPS_ErrorMessage_GetMessage(message));
    //         }
    //     }
    //
    //     // Update the state of all pedestrians.
    //     // If necessary, this could be done more often in the loop above but the more precise positions are probably never visible.
    //     // If it is needed for model correctness (precise stopping / arrivals) we should rather reduce SUMO's step-length.
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
    //         const JPS_Agent agent = JPS_Simulation_GetAgent(myJPSSimulation, state->getAgentId(), nullptr);
    //         const JPS_Point position = JPS_Agent_GetPosition(agent);
    //         state->setPosition(position.x, position.y);
    //
    //         // Updates the agent direction.
    //         const JPS_Point orientation = JPS_Agent_GetOrientation(agent);
    //         state->setAngle(atan2(orientation.y, orientation.x));
    //
    //         // Find on which edge the pedestrian is, using route's forward-looking edges because of how moveToXY is written.
    //         Position newPosition(position.x, position.y);
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
    //             const double newMaxSpeed = MIN2(candidateLane->getSpeedLimit(), person->getMaxSpeed());
    //             switch (myJPSModel) {
    //                 case JPS_Model::CollisionFreeSpeed: {
    //                     JPS_CollisionFreeSpeedModelState modelState = JPS_Agent_GetCollisionFreeSpeedModelState(agent, nullptr);
    //                     if (newMaxSpeed != JPS_CollisionFreeSpeedModelState_GetV0(modelState)) {
    //                         JPS_CollisionFreeSpeedModelState_SetV0(modelState, newMaxSpeed);
    //                     }
    //                     break;
    //                 }
    //                 case JPS_Model::CollisionFreeSpeedV2: {
    //                     JPS_CollisionFreeSpeedModelV2State modelState = JPS_Agent_GetCollisionFreeSpeedModelV2State(agent, nullptr);
    //                     if (newMaxSpeed != JPS_CollisionFreeSpeedModelV2State_GetV0(modelState)) {
    //                         JPS_CollisionFreeSpeedModelV2State_SetV0(modelState, newMaxSpeed);
    //                     }
    //                     break;
    //                 }
    //                 case JPS_Model::GeneralizedCentrifugalForce: {
    //                     JPS_GeneralizedCentrifugalForceModelState modelState = JPS_Agent_GetGeneralizedCentrifugalForceModelState(agent, nullptr);
    //                     if (newMaxSpeed != JPS_GeneralizedCentrifugalForceModelState_GetV0(modelState)) {
    //                         JPS_GeneralizedCentrifugalForceModelState_SetV0(modelState, newMaxSpeed);
    //                     }
    //                     break;
    //                 }
    //                 case JPS_Model::SocialForce: {
    //                     JPS_SocialForceModelState modelState = JPS_Agent_GetSocialForceModelState(agent, nullptr);
    //                     if (newMaxSpeed != JPS_SocialForceModelState_GetDesiredSpeed(modelState)) {
    //                         JPS_SocialForceModelState_SetDesiredSpeed(modelState, newMaxSpeed);
    //                     }
    //                     break;
    //                 }
    //             }
    //         }
    //         const double speed = person->getSpeed();
    //         for (int offset = 0; offset < 2; offset++) {
    //             const WaypointDesc* const waypoint = state->getNextWaypoint(offset);
    //             if (waypoint != nullptr && std::get<0>(*waypoint) != 0) {
    //                 const JPS_StageId waitingStage = std::get<0>(*waypoint);
    //                 const MSLane* const crossing = myCrossings[waitingStage];
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
    //                 const auto proxy = JPS_Simulation_GetWaitingSetProxy(myJPSSimulation, waitingStage, &message);
    //                 JPS_WaitingSetProxy_SetWaitingSetState(proxy, open ? JPS_WaitingSet_Inactive : JPS_WaitingSet_Active);
    //             }
    //         }
    //         stage->activateMoveReminders(person, oldLanePos, state->getEdgePos(time), state->getSpeed(*stage));
    //         // In the worst case during one SUMO step the person touches the waypoint radius and walks immediately into a different direction,
    //         // but at some simstep it should have a maximum distance of v * delta_t / 2 to the waypoint circle.
    //         const double slack = person->getMaxSpeed() * TS / 2. + POSITION_EPS;
    //         if (newPosition.distanceTo2D(std::get<1>(*state->getNextWaypoint())) < std::get<2>(*state->getNextWaypoint()) + slack) {
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
    //         const std::vector<JPS_Point>& areaBoundary = area->areaBoundary;
    //         JPS_AgentIdIterator agentsInArea = JPS_Simulation_AgentsInPolygon(myJPSSimulation, areaBoundary.data(), areaBoundary.size());
    //         if (area->areaType == "vanishing_area") {
    //             const SUMOTime period = area->params.count("period") > 0 ? string2time(area->params.at("period")) : 1000;
    //             const int nbrPeriodsCoveringTimestep = (int)ceil(TS / STEPS2TIME(period));
    //             if (time - area->lastRemovalTime >= nbrPeriodsCoveringTimestep * period) {
    //                 for (int k = 0; k < nbrPeriodsCoveringTimestep; k++) {
    //                     const JPS_AgentId agentID = JPS_AgentIdIterator_Next(agentsInArea);
    //                     if (agentID != 0) {
    //                         auto lambda = [agentID](const PState * const p) {
    //                             return p->getAgentId() == agentID;
    //                         };
    //                         std::vector<PState*>::const_iterator iterator = std::find_if(myPedestrianStates.begin(), myPedestrianStates.end(), lambda);
    //                         if (iterator != myPedestrianStates.end()) {
    //                             const PState* const state = *iterator;
    //                             MSPerson* const person = state->getPerson();
    //                             // Code below only works if the removal happens at the last stage.
    //                             const bool finalStage = person->getNumRemainingStages() == 1;
    //                             if (finalStage) {
    //                                 WRITE_MESSAGEF(TL("Person '%' in vanishing area '%' was removed from the simulation."), person->getID(), area->id);
    //                                 while (!state->getStage()->moveToNextEdge(person, time, 1, nullptr));
    //                                 registerArrived(agentID);
    //                                 myPedestrianStates.erase(iterator);
    //                                 area->lastRemovalTime = time;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         } else {  // areaType == "influencer"
    //             for (JPS_AgentId agentID = JPS_AgentIdIterator_Next(agentsInArea); agentID != 0; agentID = JPS_AgentIdIterator_Next(agentsInArea)) {
    //                 if (area->params.count("speed") > 0) {
    //                     const JPS_Agent agent = JPS_Simulation_GetAgent(myJPSSimulation, agentID, nullptr);
    //                     const double newMaxSpeed = StringUtils::toDouble(area->params.at("speed"));
    //                     switch (myJPSModel) {
    //                         case JPS_Model::CollisionFreeSpeed: {
    //                             JPS_CollisionFreeSpeedModelState modelState = JPS_Agent_GetCollisionFreeSpeedModelState(agent, nullptr);
    //                             if (newMaxSpeed != JPS_CollisionFreeSpeedModelState_GetV0(modelState)) {
    //                                 JPS_CollisionFreeSpeedModelState_SetV0(modelState, newMaxSpeed);
    //                             }
    //                             break;
    //                         }
    //                         case JPS_Model::CollisionFreeSpeedV2: {
    //                             JPS_CollisionFreeSpeedModelV2State modelState = JPS_Agent_GetCollisionFreeSpeedModelV2State(agent, nullptr);
    //                             if (newMaxSpeed != JPS_CollisionFreeSpeedModelV2State_GetV0(modelState)) {
    //                                 JPS_CollisionFreeSpeedModelV2State_SetV0(modelState, newMaxSpeed);
    //                             }
    //                             break;
    //                         }
    //                         case JPS_Model::GeneralizedCentrifugalForce: {
    //                             JPS_GeneralizedCentrifugalForceModelState modelState = JPS_Agent_GetGeneralizedCentrifugalForceModelState(agent, nullptr);
    //                             if (newMaxSpeed != JPS_GeneralizedCentrifugalForceModelState_GetV0(modelState)) {
    //                                 JPS_GeneralizedCentrifugalForceModelState_SetV0(modelState, newMaxSpeed);
    //                             }
    //                             break;
    //                         }
    //                         case JPS_Model::SocialForce: {
    //                             JPS_SocialForceModelState modelState = JPS_Agent_GetSocialForceModelState(agent, nullptr);
    //                             if (newMaxSpeed != JPS_SocialForceModelState_GetDesiredSpeed(modelState)) {
    //                                 JPS_SocialForceModelState_SetDesiredSpeed(modelState, newMaxSpeed);
    //                             }
    //                             break;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         JPS_AgentIdIterator_Free(agentsInArea);
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
    //                     const bool ok = JPS_Simulation_SwitchGeometry(myJPSSimulation, myJPSGeometryWithTrainsAndRamps, nullptr, &message);
    //                     if (!ok) {
    //                         WRITE_WARNINGF(TL("While switching to train geometry: %"), JPS_ErrorMessage_GetMessage(message));
    //                     }
    //                     removePolygonFromDrawing(PEDESTRIAN_NETWORK_ID);
    //                     preparePolygonForDrawing(pedestrianNetworkWithTrainsAndRampsLargestComponent, PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_ID, PEDESTRIAN_NETWORK_CARRIAGES_AND_RAMPS_COLOR);
    //                     GEOSGeom_destroy(pedestrianNetworkWithTrainsAndRamps);
    //                 }
    //                 GEOSGeom_destroy(rampsCollection);
    //                 GEOSGeom_destroy(carriagesCollection);
    //             }
    //         } else {
    //             if (myJPSGeometryWithTrainsAndRamps != nullptr) {
    //                 const bool ok = JPS_Simulation_SwitchGeometry(myJPSSimulation, myJPSGeometry, nullptr, &message);
    //                 if (!ok) {
    //                     WRITE_WARNINGF(TL("While switching to default geometry: %"), JPS_ErrorMessage_GetMessage(message));
    //                 }
    //                 preparePolygonForDrawing(myGEOSPedestrianNetworkLargestComponent, PEDESTRIAN_NETWORK_ID, PEDESTRIAN_NETWORK_COLOR);
    //                 myJPSGeometryWithTrainsAndRamps = nullptr;
    //             }
    //         }
    //         myAllStoppedTrainIDs = allStoppedTrainIDs;
    //     }
    //
    //     JPS_ErrorMessage_Free(message);
    //
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, time: SUMOTime) -> SUMOTime {
        // shortcut: needs the JuPedSim C API (JPS_Simulation_Iterate, JPS_Agent_* model states) and GEOS for train geometry, upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::execute needs the JuPedSim C API (jupedsim.h) and GEOS (geos crate)")
    }

    /// C++: `bool MSPModel_JuPedSim_legacy::usingShortcuts()` [virtual]
    // bool usingShortcuts() override {
    //         return myHaveAdditionalWalkableAreas;
    //     }
    #[allow(unused_variables)]
    pub fn using_shortcuts(&mut self) -> bool {
        self.myHaveAdditionalWalkableAreas
    }

    /// C++: `bool MSPModel_JuPedSim_legacy::usingDoors()` [virtual]
    /// precise lateral positioning when leaving trains is supported by JuPedSim
    // bool usingDoors() override {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn using_doors(&mut self) -> bool {
        true
    }

    /// C++: `void MSPModel_JuPedSim_legacy::registerArrived(const int)`
    // void MSPModel_JuPedSim_legacy::registerArrived(const JPS_AgentId agentID) {
    //     myNumActivePedestrians--;
    //     JPS_Simulation_MarkAgentForRemoval(myJPSSimulation, agentID, nullptr);
    // }
    #[allow(unused_variables)]
    pub fn register_arrived(&mut self, agentID: i32) {
        // shortcut: needs the JuPedSim C API (JPS_Simulation_MarkAgentForRemoval), upgrade once the C API is bound via bindgen
        panic!("MSPModel_JuPedSim_legacy::registerArrived needs the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::clearState()` [virtual]
    // void MSPModel_JuPedSim_legacy::clearState() {
    //     myPedestrianStates.clear();
    //     myNumActivePedestrians = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myPedestrianStates.clear();
        self.myNumActivePedestrians = 0;
    }

    /// C++: `void MSPModel_JuPedSim_legacy::polygonChanged(const SUMOPolygon *const, const bool, const bool)` [virtual]
    // void
    // MSPModel_JuPedSim_legacy::polygonChanged(const SUMOPolygon* const poly, const bool added, const bool removed) {
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
    //             std::vector<JPS_Point> areaBoundary;
    //             for (const Position& p : poly->getShape()) {
    //                 areaBoundary.push_back({p.x(), p.y()});
    //             }
    //             // Make sure the shape is not repeating the first point.
    //             if (areaBoundary.back().x == areaBoundary.front().x && areaBoundary.back().y == areaBoundary.front().y) {
    //                 areaBoundary.pop_back();
    //             }
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
                let mut areaBoundary: Vec<(f64, f64)> = poly.get_shape().iter().map(|p| (p.x(), p.y())).collect();
                if !areaBoundary.is_empty() && areaBoundary.last() == areaBoundary.first() {
                    areaBoundary.pop();
                }
                let type_ = StringTokenizer::with_token(poly.get_shape_type(), ".", false).get_vector()[1].clone();
                self.myAreas.push(Box::new(MSPModel_JuPedSim_legacy_AreaData {
                    id: poly.get_id().to_string(),
                    areaType: type_,
                    areaBoundary,
                    params: Ptr::new(poly.base2.get_parameters_map()),
                    lastRemovalTime,
                }));
            }
        }
    }

    /// C++: `void MSPModel_JuPedSim_legacy::initialize(const OptionsCont &)` [private]
    // void
    // MSPModel_JuPedSim_legacy::initialize(const OptionsCont& oc) {
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
    //     myJPSGeometryWithTrainsAndRamps = nullptr;
    //     preparePolygonForDrawing(myGEOSPedestrianNetworkLargestComponent, PEDESTRIAN_NETWORK_ID, PEDESTRIAN_NETWORK_COLOR);
    //     PROGRESS_DONE_MESSAGE();
    //     JPS_ErrorMessage message = nullptr;
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
    //
    //     switch (myJPSModel) {
    //         case JPS_Model::CollisionFreeSpeed: {
    //             JPS_CollisionFreeSpeedModelBuilder modelBuilder = JPS_CollisionFreeSpeedModelBuilder_Create(
    //                         oc.getFloat("pedestrian.jupedsim.strength-neighbor-repulsion"),
    //                         oc.getFloat("pedestrian.jupedsim.range-neighbor-repulsion"),
    //                         strengthGeometryRepulsion,
    //                         rangeGeometryRepulsion);
    //             myJPSOperationalModel = JPS_CollisionFreeSpeedModelBuilder_Build(modelBuilder, &message);
    //             JPS_CollisionFreeSpeedModelBuilder_Free(modelBuilder);
    //             break;
    //         }
    //         case JPS_Model::CollisionFreeSpeedV2: {
    //             JPS_CollisionFreeSpeedModelV2Builder modelBuilder = JPS_CollisionFreeSpeedModelV2Builder_Create();
    //             myJPSOperationalModel = JPS_CollisionFreeSpeedModelV2Builder_Build(modelBuilder, &message);
    //             JPS_CollisionFreeSpeedModelV2Builder_Free(modelBuilder);
    //             break;
    //         }
    //         case JPS_Model::GeneralizedCentrifugalForce: {
    //             JPS_GeneralizedCentrifugalForceModelBuilder modelBuilder = JPS_GeneralizedCentrifugalForceModelBuilder_Create(
    //                         oc.getFloat("pedestrian.jupedsim.strength-neighbor-repulsion"),
    //                         strengthGeometryRepulsion,
    //                         2.0,
    //                         2.0,
    //                         0.1,
    //                         0.1,
    //                         9.0,
    //                         3.0);
    //             myJPSOperationalModel = JPS_GeneralizedCentrifugalForceModelBuilder_Build(modelBuilder, &message);
    //             JPS_GeneralizedCentrifugalForceModelBuilder_Free(modelBuilder);
    //             break;
    //         }
    //         case JPS_Model::SocialForce: {
    //             JPS_SocialForceModelBuilder modelBuilder = JPS_SocialForceModelBuilder_Create(120000.0, 240000.0);
    //             myJPSOperationalModel = JPS_SocialForceModelBuilder_Build(modelBuilder, &message);
    //             JPS_SocialForceModelBuilder_Free(modelBuilder);
    //             break;
    //         }
    //     }
    //
    //     if (myJPSOperationalModel == nullptr) {
    //         const std::string error = TLF("Error creating the pedestrian model: %", JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         throw ProcessError(error);
    //     }
    //     myJPSSimulation = JPS_Simulation_Create(myJPSOperationalModel, myJPSGeometry, STEPS2TIME(myJPSDeltaT), &message);
    //     if (myJPSSimulation == nullptr) {
    //         const std::string error = TLF("Error creating the simulation: %", JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         throw ProcessError(error);
    //     }
    //     // Polygons that define vanishing areas aren't part of the regular JuPedSim geometry.
    //     for (const auto& polygonWithID : myNetwork->getShapeContainer().getPolygons()) {
    //         polygonChanged(polygonWithID.second, true, false);
    //     }
    //     if (OutputDevice::createDeviceByOption("pedestrian.jupedsim.py")) {
    //         myPythonScript = &OutputDevice::getDeviceByOption("pedestrian.jupedsim.py");
    //         myPythonScript->setPrecision(10);
    //         (*myPythonScript) << "import jupedsim as jps\nimport shapely\n\n"
    //                           "with open('" << filename << "') as f: geom = shapely.from_wkt(f.read())\n"
    //                           "simulation = jps.Simulation(dt=" << STEPS2TIME(myJPSDeltaT) << ", model=jps.CollisionFreeSpeedModel(), geometry=geom,\n"
    //                           "trajectory_writer=jps.SqliteTrajectoryWriter(output_file='out.sql'))\n";
    //     }
    //     myNetwork->getShapeContainer().addShapeListener(this);
    // }
    #[allow(unused_variables)]
    pub fn initialize(&mut self, oc: &OptionsCont) {
        // shortcut: needs GEOS (geos 11.3.1) for the pedestrian network and the JuPedSim C API (model builders, JPS_Simulation_Create), upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::initialize needs GEOS (geos crate) and the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::tryPedestrianInsertion(PState *, const Position &)` [private]
    // void
    // MSPModel_JuPedSim_legacy::tryPedestrianInsertion(PState* state, const Position& p) {
    //     const MSVehicleType& type = state->getPerson()->getVehicleType();
    //
    //     const double angle = state->getAngle(*state->getStage(), 0);
    //     JPS_Point orientation;
    //     if (fabs(angle - M_PI / 2) < NUMERICAL_EPS) {
    //         orientation = JPS_Point{0., 1.};
    //     } else if (fabs(angle + M_PI / 2) < NUMERICAL_EPS) {
    //         orientation = JPS_Point{0., -1.};
    //     } else {
    //         orientation = JPS_Point{1., tan(angle)};
    //     }
    //
    //     JPS_AgentId agentId = 0;
    //     JPS_ErrorMessage message = nullptr;
    //
    //     switch (myJPSModel) {
    //         case JPS_Model::CollisionFreeSpeed: {
    //             JPS_CollisionFreeSpeedModelAgentParameters agentParameters{};
    //             agentParameters.position = {p.x(), p.y()};
    //             agentParameters.journeyId = state->getJourneyId();
    //             agentParameters.stageId = state->getStageId();
    //             agentParameters.v0 = state->getPerson()->getMaxSpeed();
    //             agentParameters.radius = getRadius(type);
    //             agentId = JPS_Simulation_AddCollisionFreeSpeedModelAgent(myJPSSimulation, agentParameters, &message);
    //             break;
    //         }
    //         case JPS_Model::CollisionFreeSpeedV2: {
    //             JPS_CollisionFreeSpeedModelV2AgentParameters agentParameters{};
    //             agentParameters.position = {p.x(), p.y()};
    //             agentParameters.journeyId = state->getJourneyId();
    //             agentParameters.stageId = state->getStageId();
    //             agentParameters.v0 = state->getPerson()->getMaxSpeed();
    //             agentParameters.radius = getRadius(type);
    //             agentId = JPS_Simulation_AddCollisionFreeSpeedModelV2Agent(myJPSSimulation, agentParameters, &message);
    //             break;
    //         }
    //         case JPS_Model::GeneralizedCentrifugalForce: {
    //             JPS_GeneralizedCentrifugalForceModelAgentParameters agentParameters{};
    //             agentParameters.position = {p.x(), p.y()};
    //             agentParameters.orientation = orientation;
    //             agentParameters.journeyId = state->getJourneyId();
    //             agentParameters.stageId = state->getStageId();
    //             agentParameters.v0 = state->getPerson()->getMaxSpeed();
    //             agentId = JPS_Simulation_AddGeneralizedCentrifugalForceModelAgent(myJPSSimulation, agentParameters, &message);
    //             break;
    //         }
    //         case JPS_Model::SocialForce: {
    //             JPS_SocialForceModelAgentParameters agentParameters{};
    //             agentParameters.position = {p.x(), p.y()};
    //             agentParameters.orientation = orientation;
    //             agentParameters.journeyId = state->getJourneyId();
    //             agentParameters.stageId = state->getStageId();
    //             agentParameters.desiredSpeed = state->getPerson()->getMaxSpeed();
    //             agentParameters.radius = getRadius(type);
    //             agentId = JPS_Simulation_AddSocialForceModelAgent(myJPSSimulation, agentParameters, &message);
    //             break;
    //         }
    //     }
    //     if (message != nullptr) {
    //         WRITE_WARNINGF(TL("Error while adding person '%' as JuPedSim agent: %"), state->getPerson()->getID(), JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //     } else {
    //         state->setAgentId(agentId);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn try_pedestrian_insertion(&mut self, state: Ptr<MSPModel_Striping_PState>, p: &Position) {
        // shortcut: needs the JuPedSim C API (JPS_Simulation_Add*ModelAgent), upgrade once the C API is bound via bindgen
        panic!("MSPModel_JuPedSim_legacy::tryPedestrianInsertion needs the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `bool MSPModel_JuPedSim_legacy::addStage(int, int &, const std::string &, const int)` [private]
    // bool
    // MSPModel_JuPedSim_legacy::addStage(JPS_JourneyDescription journey, JPS_StageId& predecessor, const std::string& agentID, const JPS_StageId stage) {
    //     JPS_ErrorMessage message = nullptr;
    //     if (predecessor != 0) {
    //         const JPS_Transition transition = JPS_Transition_CreateFixedTransition(stage, &message);
    //         if (message != nullptr) {
    //             WRITE_WARNINGF(TL("Error while creating fixed transition for person '%': %"), agentID, JPS_ErrorMessage_GetMessage(message));
    //             JPS_ErrorMessage_Free(message);
    //             return false;
    //         }
    //         JPS_JourneyDescription_SetTransitionForStage(journey, predecessor, transition, &message);
    //         if (message != nullptr) {
    //             WRITE_WARNINGF(TL("Error while setting transition for person '%': %"), agentID, JPS_ErrorMessage_GetMessage(message));
    //             JPS_ErrorMessage_Free(message);
    //             return false;
    //         }
    //         JPS_Transition_Free(transition);
    //     }
    //     JPS_JourneyDescription_AddStage(journey, stage);
    //     predecessor = stage;
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_stage(&mut self, journey: i32, predecessor: &mut i32, agentID: &str, stage: i32) -> bool {
        // shortcut: needs the JuPedSim C API (JPS_JourneyDescription, JPS_Transition), upgrade once the C API is bound via bindgen
        panic!("MSPModel_JuPedSim_legacy::addStage needs the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `bool MSPModel_JuPedSim_legacy::addWaypoint(int, int &, const std::string &, const WaypointDesc &)` [private]
    // bool
    // MSPModel_JuPedSim_legacy::addWaypoint(JPS_JourneyDescription journey, JPS_StageId& predecessor, const std::string& agentID, const WaypointDesc& waypoint) {
    //     JPS_ErrorMessage message = nullptr;
    //     const Position& coords = std::get<1>(waypoint);
    //     const JPS_StageId waypointId = JPS_Simulation_AddStageWaypoint(myJPSSimulation, {coords.x(), coords.y()},
    //                                    std::get<2>(waypoint), &message);
    //     if (message != nullptr) {
    //         WRITE_WARNINGF(TL("Error while adding waypoint for person '%': %"), agentID, JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         return false;
    //     }
    //     const JPS_StageId waiting = std::get<0>(waypoint);
    //     if (waiting != 0) {
    //         if (myPythonScript != nullptr) {
    //             (*myPythonScript) << "journey.add(" << waiting << ")\n";
    //             if (predecessor != 0) {
    //                 (*myPythonScript) << "journey.set_transition_for_stage(" << predecessor << ", jps.Transition.create_fixed_transition(" << waiting << "))\n";
    //             }
    //         }
    //         if (!addStage(journey, predecessor, agentID, waiting)) {
    //             return false;
    //         }
    //     }
    //     if (myPythonScript != nullptr) {
    //         (*myPythonScript) << "ws = simulation.add_waypoint_stage((" << coords.x() << "," << coords.y() << "), " << std::get<2>(waypoint) << ")\n"
    //                           "journey.add(ws)\n";
    //         if (predecessor != 0) {
    //             (*myPythonScript) << "journey.set_transition_for_stage(" << predecessor << ", jps.Transition.create_fixed_transition(ws))\n";
    //         }
    //     }
    //     if (!addStage(journey, predecessor, agentID, waypointId)) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_waypoint(&mut self, journey: i32, predecessor: &mut i32, agentID: &str, waypoint: &MSPModel_JuPedSim_legacy_WaypointDesc) -> bool {
        // shortcut: needs the JuPedSim C API (JPS_Simulation_AddStageWaypoint), upgrade once the C API is bound via bindgen
        panic!("MSPModel_JuPedSim_legacy::addWaypoint needs the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `int * MSPModel_JuPedSim_legacy::createGeometryFromCenterLine(PositionVector, double, int)` [static, private]
    // GEOSGeometry*
    // MSPModel_JuPedSim_legacy::createGeometryFromCenterLine(PositionVector centerLine, double width, int capStyle) {
    //     GEOSCoordSequence* coordinateSequence = convertToGEOSPoints(centerLine);
    //     GEOSGeometry* lineString = GEOSGeom_createLineString(coordinateSequence);
    //     GEOSGeometry* dilatedLineString = GEOSBufferWithStyle(lineString, width, GEOS_QUADRANT_SEGMENTS, capStyle, GEOSBUF_JOIN_ROUND, GEOS_MITRE_LIMIT);
    //     GEOSGeom_destroy(lineString);
    //     return dilatedLineString;
    // }
    #[allow(unused_variables)]
    pub fn create_geometry_from_center_line(centerLine: PositionVector, width: f64, capStyle: i32) -> Ptr<i32> {
        // shortcut: needs GEOS (geos 11.3.1, GEOSBufferWithStyle), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::createGeometryFromCenterLine needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim_legacy::createGeometryFromShape(PositionVector, std::string, std::string, bool)` [static, private]
    // GEOSGeometry*
    // MSPModel_JuPedSim_legacy::createGeometryFromShape(PositionVector shape, std::string junctionID, std::string shapeID, bool isInternalShape) {
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
        panic!("MSPModel_JuPedSim_legacy::createGeometryFromShape needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim_legacy::buildPedestrianNetwork(MSNet *)` [private]
    // GEOSGeometry*
    // MSPModel_JuPedSim_legacy::buildPedestrianNetwork(MSNet* network) {
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
        panic!("MSPModel_JuPedSim_legacy::buildPedestrianNetwork needs GEOS (geos crate)")
    }

    /// C++: `int * MSPModel_JuPedSim_legacy::convertToGEOSPoints(PositionVector)` [static, private]
    // GEOSCoordSequence*
    // MSPModel_JuPedSim_legacy::convertToGEOSPoints(PositionVector shape) {
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
        panic!("MSPModel_JuPedSim_legacy::convertToGEOSPoints needs GEOS (geos crate)")
    }

    /// C++: `int MSPModel_JuPedSim_legacy::convertToJPSPoints(const int *)` [static, private]
    // std::vector<JPS_Point>
    // MSPModel_JuPedSim_legacy::convertToJPSPoints(const GEOSGeometry* geometry) {
    //     std::vector<JPS_Point> pointVector;
    //     const GEOSCoordSequence* coordinateSequence = GEOSGeom_getCoordSeq(geometry);
    //     unsigned int coordinateSequenceSize;
    //     GEOSCoordSeq_getSize(coordinateSequence, &coordinateSequenceSize);
    //     double x;
    //     double y;
    //     // Remove the last point so that CGAL doesn't complain about the simplicity of the polygon downstream.
    //     for (unsigned int i = 0; i < coordinateSequenceSize - 1; i++) {
    //         GEOSCoordSeq_getX(coordinateSequence, i, &x);
    //         GEOSCoordSeq_getY(coordinateSequence, i, &y);
    //         pointVector.push_back({x, y});
    //     }
    //     return pointVector;
    // }
    #[allow(unused_variables)]
    pub fn convert_to_jps_points(geometry: Ptr<i32>) -> i32 {
        // shortcut: needs GEOS (geos 11.3.1, reads coordinates) and returns a JPS_Point vector for the JuPedSim C API, upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::convertToJPSPoints needs GEOS (geos crate)")
    }

    /// C++: `PositionVector MSPModel_JuPedSim_legacy::convertToSUMOPoints(const int *)` [static, private]
    // PositionVector
    // MSPModel_JuPedSim_legacy::convertToSUMOPoints(const GEOSGeometry* geometry) {
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
        panic!("MSPModel_JuPedSim_legacy::convertToSUMOPoints needs GEOS (geos crate)")
    }

    /// C++: `double MSPModel_JuPedSim_legacy::getLinearRingArea(const int *)` [static, private]
    // double
    // MSPModel_JuPedSim_legacy::getLinearRingArea(const GEOSGeometry* linearRing) {
    //     double area;
    //     GEOSGeometry* linearRingAsPolygon = GEOSGeom_createPolygon(GEOSGeom_clone(linearRing), nullptr, 0);
    //     GEOSArea(linearRingAsPolygon, &area);
    //     GEOSGeom_destroy(linearRingAsPolygon);
    //     return area;
    // }
    #[allow(unused_variables)]
    pub fn get_linear_ring_area(linearRing: Ptr<i32>) -> f64 {
        // shortcut: needs GEOS (geos 11.3.1, GEOSArea), upgrade once GEOS is linked (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::getLinearRingArea needs GEOS (geos crate)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::removePolygonFromDrawing(const std::string &)` [private]
    // void
    // MSPModel_JuPedSim_legacy::removePolygonFromDrawing(const std::string& polygonId) {
    //     myShapeContainer.removePolygon(polygonId);
    // }
    #[allow(unused_variables)]
    pub fn remove_polygon_from_drawing(&mut self, polygonId: &str) {
        self.myShapeContainer.m().remove_polygon(polygonId, true);
    }

    /// C++: `void MSPModel_JuPedSim_legacy::preparePolygonForDrawing(const int *, const std::string &, const RGBColor &)` [private]
    // void
    // MSPModel_JuPedSim_legacy::preparePolygonForDrawing(const GEOSGeometry* polygon, const std::string& polygonId, const RGBColor& color) {
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
        panic!("MSPModel_JuPedSim_legacy::preparePolygonForDrawing needs GEOS (geos crate)")
    }

    /// C++: `const int * MSPModel_JuPedSim_legacy::getLargestComponent(const int *, int &, double &, double &)` [static, private]
    // const GEOSGeometry*
    // MSPModel_JuPedSim_legacy::getLargestComponent(const GEOSGeometry* polygon, int& nbrComponents, double& maxArea, double& totalArea) {
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
        panic!("MSPModel_JuPedSim_legacy::getLargestComponent needs GEOS (geos crate)")
    }

    /// C++: `int MSPModel_JuPedSim_legacy::buildJPSGeometryFromGEOSGeometry(const int *)` [static, private]
    // JPS_Geometry
    // MSPModel_JuPedSim_legacy::buildJPSGeometryFromGEOSGeometry(const GEOSGeometry* polygon) {
    //     JPS_GeometryBuilder geometryBuilder = JPS_GeometryBuilder_Create();
    //
    //     // Handle the exterior polygon.
    //     const GEOSGeometry* exterior =  GEOSGetExteriorRing(polygon);
    //     std::vector<JPS_Point> exteriorCoordinates = convertToJPSPoints(exterior);
    //     JPS_GeometryBuilder_AddAccessibleArea(geometryBuilder, exteriorCoordinates.data(), exteriorCoordinates.size());
    //
    //     // Handle the interior polygons (holes).
    //     int nbrInteriorRings = GEOSGetNumInteriorRings(polygon);
    //     if (nbrInteriorRings != -1) {
    //         for (int k = 0; k < nbrInteriorRings; k++) {
    //             const GEOSGeometry* linearRing = GEOSGetInteriorRingN(polygon, k);
    //             double area = getLinearRingArea(linearRing);
    //             if (area > GEOS_MIN_AREA) {
    //                 std::vector<JPS_Point> holeCoordinates = convertToJPSPoints(linearRing);
    //                 JPS_GeometryBuilder_ExcludeFromAccessibleArea(geometryBuilder, holeCoordinates.data(), holeCoordinates.size());
    //             }
    //         }
    //     }
    //
    //     JPS_ErrorMessage message = nullptr;
    //     JPS_Geometry geometry = JPS_GeometryBuilder_Build(geometryBuilder, &message);
    //     if (geometry == nullptr) {
    //         const std::string error = TLF("Error while generating geometry: %", JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         throw ProcessError(error);
    //     }
    //     JPS_GeometryBuilder_Free(geometryBuilder);
    //     return geometry;
    // }
    #[allow(unused_variables)]
    pub fn build_jps_geometry_from_geos_geometry(polygon: Ptr<i32>) -> i32 {
        // shortcut: needs GEOS (geos 11.3.1, reads rings) and the JuPedSim C API (JPS_GeometryBuilder), upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::buildJPSGeometryFromGEOSGeometry needs GEOS (geos crate) and the JuPedSim C API (jupedsim.h)")
    }

    /// C++: `void MSPModel_JuPedSim_legacy::dumpGeometry(const int *, const std::string &, bool)` [static, private]
    // void
    // MSPModel_JuPedSim_legacy::dumpGeometry(const GEOSGeometry* polygon, const std::string& filename, bool useGeoCoordinates) {
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
        panic!("MSPModel_JuPedSim_legacy::dumpGeometry needs GEOS (geos crate)")
    }

    /// C++: `double MSPModel_JuPedSim_legacy::getRadius(const MSVehicleType &)` [static, private]
    // double
    // MSPModel_JuPedSim_legacy::getRadius(const MSVehicleType& vehType) {
    //     return 0.5 * MAX2(vehType.getLength(), vehType.getWidth());
    // }
    #[allow(unused_variables)]
    pub fn get_radius(vehType: &MSVehicleType) -> f64 {
        let length = vehType.get_length();
        let width = vehType.get_width();
        0.5 * (if length > width { length } else { width })
    }

    /// C++: `int MSPModel_JuPedSim_legacy::addWaitingSet(const MSLane *const, const bool)` [private]
    // JPS_StageId
    // MSPModel_JuPedSim_legacy::addWaitingSet(const MSLane* const crossing, const bool entry) {
    //     JPS_ErrorMessage message = nullptr;
    //     JPS_StageId waitingStage = 0;
    //     PositionVector shape = crossing->getShape();
    //     const double radius = getRadius(*MSNet::getInstance()->getVehicleControl().getVType(DEFAULT_PEDTYPE_ID, nullptr, true));
    //     shape.extrapolate2D((shape.length() + radius) / shape.length());
    //     const double offset = 2 * radius + POSITION_EPS;
    //     const double lonOffset = entry ? NUMERICAL_EPS : shape.length() - NUMERICAL_EPS;
    //     PositionVector pv;
    //     pv.push_back(shape.positionAtOffset(lonOffset));
    //     for (double latOff = offset; latOff < crossing->getWidth() / 2. - offset; latOff += offset) {
    //         PositionVector moved(shape);
    //         moved.move2side(latOff);
    //         pv.push_back(moved.positionAtOffset(lonOffset));
    //         moved.move2side(-2. * latOff);
    //         pv.push_back(moved.positionAtOffset(lonOffset));
    //     }
    //     if (entry && crossing->getIncomingLanes().size() == 1 && crossing->getIncomingLanes().front().lane->isWalkingArea()) {
    //         pv.push_back(crossing->getIncomingLanes().front().lane->getShape().getCentroid());
    //     }
    //     if (!entry && crossing->getLinkCont().size() == 1 && crossing->getLinkCont().front()->getLane()->isWalkingArea()) {
    //         pv.push_back(crossing->getLinkCont().front()->getLane()->getShape().getCentroid());
    //     }
    //     std::vector<JPS_Point> points;
    //     for (const Position& p : pv) {
    //         GEOSCoordSequence* seq = GEOSCoordSeq_create(1, 2); // 1 point, 2 dimensions
    //         GEOSCoordSeq_setX(seq, 0, p.x());
    //         GEOSCoordSeq_setY(seq, 0, p.y());
    //         GEOSGeometry* point = GEOSGeom_createPoint(seq);
    //         if (GEOSContains(myGEOSPedestrianNetworkLargestComponent, point)) {
    //             points.push_back({p.x(), p.y()});
    //         } else {
    //             WRITE_WARNINGF("Waiting point %,% is not in the geometry for % on '%'.", p.x(), p.y(), entry ? "entry" : "exit", crossing->getID())
    //         }
    //         GEOSGeom_destroy(point);
    //     }
    //     waitingStage = JPS_Simulation_AddStageWaitingSet(myJPSSimulation, points.data(), points.size(), &message);
    //     if (message != nullptr) {
    //         const std::string error = TLF("Error while adding waiting set for % on '%': %",
    //                                       entry ? "entry" : "exit", crossing->getID(), JPS_ErrorMessage_GetMessage(message));
    //         JPS_ErrorMessage_Free(message);
    //         throw ProcessError(error);
    //     }
    //     const auto proxy = JPS_Simulation_GetWaitingSetProxy(myJPSSimulation, waitingStage, &message);
    //     JPS_WaitingSetProxy_SetWaitingSetState(proxy, JPS_WaitingSet_Inactive);
    //     myCrossings[waitingStage] = crossing;
    //     if (myPythonScript != nullptr) {
    //         (*myPythonScript) << "ws = simulation.add_waiting_set_stage([";
    //         for (const JPS_Point& p : points) {
    //             (*myPythonScript) << "(" << p.x << "," << p.y << "),";
    //         }
    //         (*myPythonScript) << "])\nsimulation.get_stage(ws).state = jps.stages.WaitingSetState.INACTIVE\n";
    //     }
    //     return waitingStage;
    // }
    #[allow(unused_variables)]
    pub fn add_waiting_set(&mut self, crossing: Ptr<MSLane>, entry: bool) -> i32 {
        // shortcut: needs GEOS (geos 11.3.1, GEOSContains on the pedestrian network) and the JuPedSim C API (JPS_Simulation_AddStageWaitingSet), upgrade once both are bound (tools/oracle/GEOS_DECISION.md)
        panic!("MSPModel_JuPedSim_legacy::addWaitingSet needs GEOS (geos crate) and the JuPedSim C API (jupedsim.h)")
    }

}

/// Holds pedestrian state and performs updates
/// C++ class `MSPModel_JuPedSim_legacy::PState` (microsim/transportables/MSPModel_JuPedSim_legacy.h)
#[repr(C)]
pub struct MSPModel_JuPedSim_legacy_PState {
    /// C++ base class `MSPModel_InteractingState`
    pub base: MSPModel_InteractingState,
    /// id of the journey, needed for modifying it
    pub myJourneyId: i64,
    pub myStageId: i64,
    pub myWaypoints: Vec<MSPModel_JuPedSim_legacy_WaypointDesc>,
    pub myAgentId: i64,
}

impl std::ops::Deref for MSPModel_JuPedSim_legacy_PState {
    type Target = MSPModel_InteractingState;
    fn deref(&self) -> &MSPModel_InteractingState {
        &self.base
    }
}
impl std::ops::DerefMut for MSPModel_JuPedSim_legacy_PState {
    fn deref_mut(&mut self) -> &mut MSPModel_InteractingState {
        &mut self.base
    }
}

impl MSPModel_JuPedSim_legacy_PState {
    /// C++: `void PState::PState(MSPerson *, MSStageMoving *, int, int, const int &)`
    // MSPModel_JuPedSim_legacy::PState::PState(MSPerson* person, MSStageMoving* stage,
    //         JPS_JourneyId journeyId, JPS_StageId stageId,
    //         const std::vector<WaypointDesc>& waypoints) :
    //     MSPModel_InteractingState(person, stage, nullptr),
    //     myJourneyId(journeyId), myStageId(stageId), myWaypoints(waypoints), myAgentId(0) {
    //     myDir = FORWARD;
    // }
    #[allow(unused_variables)]
    pub fn new(person: Ptr<MSPerson>, stage: Ptr<MSStageMoving>, journeyId: i64, stageId: i64, waypoints: &[MSPModel_JuPedSim_legacy_WaypointDesc]) -> Self {
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
    // MSPModel_JuPedSim_legacy::PState::~PState() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {

    }

    /// C++: `void PState::reinit(MSStageMoving *, int, int, const int &)`
    // void
    // MSPModel_JuPedSim_legacy::PState::reinit(MSStageMoving* stage, JPS_JourneyId journeyId, JPS_StageId stageId,
    //         const std::vector<WaypointDesc>& waypoints) {
    //     if (myStage != nullptr) {
    //         myStage->setPState(nullptr);  // we need to remove the old state reference to avoid double deletion
    //     }
    //     myStage = stage;
    //     myJourneyId = journeyId;
    //     myStageId = stageId;
    //     myWaypoints = waypoints;
    // }
    #[allow(unused_variables)]
    pub fn reinit(&mut self, stage: Ptr<MSStageMoving>, journeyId: i64, stageId: i64, waypoints: &[MSPModel_JuPedSim_legacy_WaypointDesc]) {
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
    // void MSPModel_JuPedSim_legacy::PState::setPosition(const double x, const double y, const double z) {
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
    // const MSEdge* MSPModel_JuPedSim_legacy::PState::getNextEdge(const MSStageMoving& stage) const {
    //     return stage.getNextRouteEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self, stage: &MSStageMoving) -> Ptr<MSEdge> {
        stage.get_next_route_edge()
    }

    /// C++: `const MSPModel_JuPedSim_legacy::WaypointDesc * PState::getNextWaypoint(const int) const` [const]
    // const MSPModel_JuPedSim_legacy::WaypointDesc*
    // MSPModel_JuPedSim_legacy::PState::getNextWaypoint(const int offset) const {
    //     return offset < (int)myWaypoints.size() ? &myWaypoints[offset] : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_waypoint(&self, offset: i32) -> Ptr<MSPModel_JuPedSim_legacy_WaypointDesc> {
        if (offset as usize) < self.myWaypoints.len() {
            Ptr::new(&self.myWaypoints[offset as usize])
        } else {
            Ptr::null()
        }
    }

    /// C++: `int PState::getAgentId() const` [const]
    // inline JPS_AgentId getAgentId() const {
    //             return myAgentId;
    //         }
    #[allow(unused_variables)]
    pub fn get_agent_id(&self) -> i64 {
        self.myAgentId
    }

    /// C++: `void PState::setAgentId(int)`
    // void setAgentId(JPS_AgentId id) {
    //             myAgentId = id;
    //             myWaitingToEnter = false;
    //         }
    #[allow(unused_variables)]
    pub fn set_agent_id(&mut self, id: i64) {
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

    /// C++: `int PState::getJourneyId() const` [const]
    // JPS_JourneyId getJourneyId() const {
    //             return myJourneyId;
    //         }
    #[allow(unused_variables)]
    pub fn get_journey_id(&self) -> i64 {
        self.myJourneyId
    }

    /// C++: `int PState::getStageId() const` [const]
    /// first stage of the journey
    // JPS_StageId getStageId() const {
    //             return myStageId;
    //         }
    #[allow(unused_variables)]
    pub fn get_stage_id(&self) -> i64 {
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

