//! Port of microsim/MSNet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSNet::SimulationState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSNet_SimulationState {
    SIMSTATE_LOADING = 0,
    SIMSTATE_RUNNING = 1,
    SIMSTATE_END_STEP_REACHED = 2,
    SIMSTATE_NO_FURTHER_VEHICLES = 3,
    SIMSTATE_CONNECTION_CLOSED = 4,
    SIMSTATE_ERROR_IN_SIM = 5,
    SIMSTATE_INTERRUPTED = 6,
    SIMSTATE_TOO_MANY_TELEPORTS = 7,
}

/// C++ enum `MSNet::TransportableState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSNet_TransportableState {
    PERSON_DEPARTED = 0,
    PERSON_ARRIVED = 1,
    CONTAINER_DEPARTED = 2,
    CONTAINER_ARRIVED = 3,
}

/// C++ enum `MSNet::VehicleState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSNet_VehicleState {
    BUILT = 0,
    DEPARTED = 1,
    STARTING_TELEPORT = 2,
    ENDING_TELEPORT = 3,
    ARRIVED = 4,
    NEWROUTE = 5,
    STARTING_PARKING = 6,
    ENDING_PARKING = 7,
    STARTING_STOP = 8,
    ENDING_STOP = 9,
    COLLISION = 10,
    EMERGENCYSTOP = 11,
    MANEUVERING = 12,
}

/// C++ typedef `std::map<std::string, std::vector<Collision>> MSNet::CollisionMap`
pub type MSNet_CollisionMap = BTreeMap<String, Vec<MSNet_Collision>>;

/// C++ typedef `std::map<const MSEdge *, RouterProhibition> MSNet::Prohibitions`
pub type MSNet_Prohibitions = BTreeMap<Ptr<MSEdge>, RouterProhibition>;

/// collision tracking
/// C++ class `MSNet::Collision` (microsim/MSNet.h)
pub struct MSNet_Collision {
    pub victim: String,
    pub colliderType: String,
    pub victimType: String,
    pub colliderSpeed: f64,
    pub victimSpeed: f64,
    pub colliderFront: Position,
    pub victimFront: Position,
    pub colliderBack: Position,
    pub victimBack: Position,
    pub type_: String,
    pub lane: Ptr<MSLane>,
    pub pos: f64,
    pub time: SUMOTime,
    pub continuationTime: SUMOTime,
}

impl MSNet_Collision {
}

/// The simulated network and simulation perfomer
/// C++ class `MSNet` (microsim/MSNet.h)
pub struct MSNet {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// Route loader for dynamic loading of routes
    pub myRouteLoaders: Ptr<SUMORouteLoaderControl>,
    /// Current time step
    pub myStep: SUMOTime,
    pub myStateLoaderTime: SUMOTime,
    /// whether libsumo triggered a partial step (executeMove)
    pub myStepCompletionMissing: bool,
    /// Maximum number of teleports.
    pub myMaxTeleports: i32,
    /// whether an interrupt occurred
    pub myAmInterrupted: bool,
    /// Controls vehicle building and deletion;
    pub myVehicleControl: Ptr<MSVehicleControl>,
    /// Controls person building and deletion;
    pub myPersonControl: Ptr<MSTransportableControl>,
    /// Controls container building and deletion;
    pub myContainerControl: Ptr<MSTransportableControl>,
    /// Controls edges, performs vehicle movement;
    pub myEdges: Ptr<MSEdgeControl>,
    /// Controls junctions, realizes right-of-way rules;
    pub myJunctions: Ptr<MSJunctionControl>,
    /// Controls tls logics, realizes waiting on tls rules;
    pub myLogics: Ptr<MSTLLogicControl>,
    /// Controls vehicle insertion;
    pub myInserter: Ptr<MSInsertionControl>,
    /// Controls detectors;
    pub myDetectorControl: Ptr<MSDetectorControl>,
    /// Controls events executed at the begin of a time step;
    pub myBeginOfTimestepEvents: Ptr<MSEventControl>,
    /// Controls events executed at the end of a time step;
    pub myEndOfTimestepEvents: Ptr<MSEventControl>,
    /// Controls insertion events;
    pub myInsertionEvents: Ptr<MSEventControl>,
    /// A container for geometrical shapes;
    pub myShapeContainer: Ptr<ShapeContainer>,
    /// The net's knowledge about edge efforts/travel times;
    pub myEdgeWeights: Ptr<MSEdgeWeightsStorage>,
    /// Information whether the simulation duration shall be logged
    pub myLogExecutionTime: bool,
    /// Information whether the number of the simulation step shall be logged
    pub myLogStepNumber: bool,
    /// Period between successive step-log outputs
    pub myLogStepPeriod: i32,
    /// The last simulation step duration
    pub myTraCIStepDuration: i64,
    /// The last simulation step duration
    pub mySimStepDuration: i64,
    /// The overall simulation duration
    pub mySimBeginMillis: i64,
    /// The overall time spent waiting for traci operations including
    pub myTraCIMillis: i64,
    /// The overall number of vehicle movements
    pub myVehiclesMoved: i64,
    pub myPersonsMoved: i64,
    /// Times at which a state shall be written
    pub myStateDumpTimes: Vec<SUMOTime>,
    /// The names for the state files
    pub myStateDumpFiles: Vec<String>,
    /// The names of the last K periodic state files (only only K shall be kept)
    pub myPeriodicStateFiles: Vec<String>,
    /// The period for writing state
    pub myStateDumpPeriod: SUMOTime,
    /// name components for periodic state
    pub myStateDumpPrefix: String,
    pub myStateDumpSuffix: String,
    /// Whether the network contains edges which not all vehicles may pass
    pub myHavePermissions: bool,
    /// The vehicle class specific speed restrictions
    pub myRestrictions: BTreeMap<String, BTreeMap<SUMOVehicleClass, f64>>, // C++: std::map<std::string, std::map<SUMOVehicleClass, double>>
    /// Preferences for routing
    pub myVClassPreferences: BTreeMap<SUMOVehicleClass, BTreeMap<String, f64>>, // C++: std::map<SUMOVehicleClass, std::map<std::string, double>>
    pub myVTypePreferences: BTreeMap<String, BTreeMap<String, f64>>, // C++: std::map<std::string, std::map<std::string, double>>
    /// The edge type specific meso parameters
    pub myMesoEdgeTypes: BTreeMap<String, MESegment_MesoEdgeType>,
    /// Whether the network contains internal links/lanes/edges
    pub myHasInternalLinks: bool,
    /// Whether the network was built with higher speed on junctions
    pub myJunctionHigherSpeeds: bool,
    /// Whether the network contains elevation data
    pub myHasElevation: bool,
    /// Whether the network contains pedestrian network elements
    pub myHasPedestrianNetwork: bool,
    /// Whether the network contains bidirectional rail edges
    pub myHasBidiEdges: bool,
    /// the network version
    pub myVersion: Unmapped /* MMVersion */,
    /// end of loaded edgeData
    pub myEdgeDataEndTime: SUMOTime,
    /// Dictionary of bus / container stops
    pub myStoppingPlaces: BTreeMap<SumoXMLTag, Unmapped /* NamedObjectCont<MSStoppingPlace *> */>,
    /// dictionary of named stopping places
    pub myNamedStoppingPlaces: BTreeMap<SumoXMLTag, BTreeMap<String, Vec<Ptr<MSStoppingPlace>>>>, // C++: std::map<SumoXMLTag, std::map<std::string, std::vector<MSStoppingPlace *>>>
    /// Dictionary of traction substations
    pub myTractionSubstations: Vec<Ptr<MSTractionSubstation>>,
    /// Container for vehicle state listener
    pub myVehicleStateListeners: Vec<Ptr<MSNet_VehicleStateListener>>,
    /// Container for transportable state listener
    pub myTransportableStateListeners: Vec<Ptr<MSNet_TransportableStateListener>>,
    /// collisions in the current time step
    pub myCollisions: MSNet_CollisionMap,
    /// container to record warnings that shall only be issued once
    pub myWarnedOnce: BTreeMap<String, bool>,
    pub myRouterTT: BTreeMap<i32, Ptr<MSVehicleRouter>>,
    pub myRouterEffort: BTreeMap<i32, Ptr<MSVehicleRouter>>,
    pub myPedestrianRouter: BTreeMap<i32, Ptr<MSPedestrianRouter>>,
    pub myIntermodalRouter: BTreeMap<i32, Ptr<MSTransportableRouter>>,
    /// An RTree structure holding lane IDs
    pub myLanesRTree: (bool, Unmapped /* NamedRTree */),
    /// Updater for dynamic shapes that are tracking traffic objects (ensures removal of shape dynamics when the objects are removed)
    pub myDynamicShapeUpdater: Box<MSDynamicShapeUpdater>,
}

impl std::ops::Deref for MSNet {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSNet {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

// C++ static members of MSNet (port as module-level statics, see AGENTS.md):
//   static const std::string MSNet::STAGE_EVENTS;
//   static const std::string MSNet::STAGE_MOVEMENTS;
//   static const std::string MSNet::STAGE_LANECHANGE;
//   static const std::string MSNet::STAGE_INSERTIONS;
//   static const std::string MSNet::STAGE_REMOTECONTROL;
//   static MSNet * MSNet::myInstance;
//   static const NamedObjectCont<MSStoppingPlace *> MSNet::myEmptyStoppingPlaceCont;
//   static const std::vector<MSStoppingPlace *> MSNet::myEmptyStoppingPlaceVector;

impl MSNet {
    /// C++: `MSNet * MSNet::getInstance()` [static]
    /// Returns the pointer to the unique instance of MSNet (singleton).
    // MSNet*
    // MSNet::getInstance(void) {
    //     if (myInstance != nullptr) {
    //         return myInstance;
    //     }
    //     throw ProcessError(TL("A network was not yet constructed."));
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<MSNet> {
        todo!("port MSNet::getInstance")
    }

    /// C++: `bool MSNet::isGUINet() const` [const, virtual]
    /// Returns whether this is a GUI Net
    // virtual bool isGUINet() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_gui_net(&self) -> bool {
        todo!("port MSNet::isGUINet")
    }

    /// C++: `void MSNet::initStatic()` [static]
    /// Place for static initializations of simulation components (called after successful net build)
    // void
    // MSNet::initStatic() {
    //     gRoutingPreferences = false;
    //     MSDriveWay::init();
    // }
    #[allow(unused_variables)]
    pub fn init_static() {
        todo!("port MSNet::initStatic")
    }

    /// C++: `void MSNet::cleanupStatic()` [static]
    /// Place for static initializations of simulation components (called after successful net build)
    // void
    // MSNet::cleanupStatic() {
    //     if (!MSGlobals::gUseMesoSim) {
    //         MSVehicle::Influencer::cleanup();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup_static() {
        todo!("port MSNet::cleanupStatic")
    }

    /// C++: `bool MSNet::hasInstance()` [static]
    /// Returns whether the network was already constructed
    // static bool hasInstance() {
    //         return myInstance != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_instance() -> bool {
        todo!("port MSNet::hasInstance")
    }

    /// C++: `void MSNet::MSNet(MSVehicleControl *, MSEventControl *, MSEventControl *, MSEventControl *, ShapeContainer *)`
    /// Constructor
    // MSNet::MSNet(MSVehicleControl* vc, MSEventControl* beginOfTimestepEvents,
    //              MSEventControl* endOfTimestepEvents,
    //              MSEventControl* insertionEvents,
    //              ShapeContainer* shapeCont):
    //     myAmInterrupted(false),
    //     myVehiclesMoved(0),
    //     myPersonsMoved(0),
    //     myHavePermissions(false),
    //     myHasInternalLinks(false),
    //     myJunctionHigherSpeeds(false),
    //     myHasElevation(false),
    //     myHasPedestrianNetwork(false),
    //     myHasBidiEdges(false),
    //     myEdgeDataEndTime(-1),
    //     myDynamicShapeUpdater(nullptr) {
    //     if (myInstance != nullptr) {
    //         throw ProcessError(TL("A network was already constructed."));
    //     }
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     myStep = string2time(oc.getString("begin"));
    //     myStateLoaderTime = myStep - 1,
    //     myMaxTeleports = oc.getInt("max-num-teleports");
    //     myLogExecutionTime = !oc.getBool("no-duration-log");
    //     myLogStepNumber = !oc.getBool("no-step-log");
    //     myLogStepPeriod = oc.getInt("step-log.period");
    //     myInserter = new MSInsertionControl(*vc, string2time(oc.getString("max-depart-delay")), oc.getBool("eager-insert"), oc.getInt("max-num-vehicles"),
    //                                         string2time(oc.getString("random-depart-offset")));
    //     myVehicleControl = vc;
    //     myDetectorControl = new MSDetectorControl();
    //     myEdges = nullptr;
    //     myJunctions = nullptr;
    //     myRouteLoaders = nullptr;
    //     myLogics = nullptr;
    //     myPersonControl = nullptr;
    //     myContainerControl = nullptr;
    //     myEdgeWeights = nullptr;
    //     myShapeContainer = shapeCont == nullptr ? new ShapeContainer() : shapeCont;
    //
    //     myBeginOfTimestepEvents = beginOfTimestepEvents;
    //     myEndOfTimestepEvents = endOfTimestepEvents;
    //     myInsertionEvents = insertionEvents;
    //     myLanesRTree.first = false;
    //
    //     if (MSGlobals::gUseMesoSim) {
    //         MSGlobals::gMesoNet = new MELoop(string2time(oc.getString("meso-recheck")));
    //     }
    //     myInstance = this;
    //     initStatic();
    // }
    #[allow(unused_variables)]
    pub fn new(vc: Ptr<MSVehicleControl>, beginOfTimestepEvents: Ptr<MSEventControl>, endOfTimestepEvents: Ptr<MSEventControl>, insertionEvents: Ptr<MSEventControl>, shapeCont: Ptr<ShapeContainer>) -> Self {
        todo!("port MSNet::MSNet")
    }

    /// C++: `void MSNet::~MSNet() noexcept` [virtual]
    /// Destructor
    // MSNet::~MSNet() {
    //     cleanupStatic();
    //     // delete controls
    //     delete myJunctions;
    //     delete myDetectorControl;
    //     // delete mean data
    //     delete myEdges;
    //     delete myInserter;
    //     myInserter = nullptr;
    //     delete myLogics;
    //     delete myRouteLoaders;
    //     if (myPersonControl != nullptr) {
    //         delete myPersonControl;
    //         myPersonControl = nullptr; // just to have that clear for later cleanups
    //     }
    //     if (myContainerControl != nullptr) {
    //         delete myContainerControl;
    //         myContainerControl = nullptr; // just to have that clear for later cleanups
    //     }
    //     // the CCH metric families (myCCHLive/myCCHFreeflow) own ref vehicles
    //     // that hold a raw MSVehicleType*; tear them down before the vehicle
    //     // types go away below (MSDevice::cleanupAll() -> MSRoutingEngine::
    //     // cleanup() runs this again later via clearAll(), harmlessly, since
    //     // cleanupCCH() nulls out what it deletes)
    //     MSRoutingEngine::cleanupCCH();
    //     delete myVehicleControl; // must happen after deleting transportables
    //     // ShapeContainer registers polygon-update commands with the event controls.
    //     // It must be torn down before the event controls so that ~ShapeContainer
    //     // can still deschedule() its (live) commands; the event controls then
    //     // delete the commands themselves.
    //     delete myShapeContainer;
    //     myShapeContainer = nullptr;
    //     // delete events late so that vehicles can get rid of references first
    //     delete myBeginOfTimestepEvents;
    //     myBeginOfTimestepEvents = nullptr;
    //     delete myEndOfTimestepEvents;
    //     myEndOfTimestepEvents = nullptr;
    //     delete myInsertionEvents;
    //     myInsertionEvents = nullptr;
    //     delete myEdgeWeights;
    //     for (auto& router : myRouterTT) {
    //         delete router.second;
    //     }
    //     myRouterTT.clear();
    //     for (auto& router : myRouterEffort) {
    //         delete router.second;
    //     }
    //     myRouterEffort.clear();
    //     for (auto& router : myPedestrianRouter) {
    //         delete router.second;
    //     }
    //     myPedestrianRouter.clear();
    //     resetIntermodalRouter();
    //     myLanesRTree.second.RemoveAll();
    //     for (MSTractionSubstation* sub : myTractionSubstations) {
    //         delete sub;
    //     }
    //     myTractionSubstations.clear();
    //     clearAll();
    //     if (MSGlobals::gUseMesoSim) {
    //         delete MSGlobals::gMesoNet;
    //     }
    //     myInstance = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSNet::~MSNet")
    }

    /// C++: `void MSNet::closeBuilding(const OptionsCont &, MSEdgeControl *, MSJunctionControl *, SUMORouteLoaderControl *, MSTLLogicControl *, std::vector<SUMOTime>, std::vector<std::string>, bool, bool, const MMVersion &)`
    /// Closes the network's building process
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn close_building(&mut self, oc: &OptionsCont, edges: Ptr<MSEdgeControl>, junctions: Ptr<MSJunctionControl>, routeLoaders: Ptr<SUMORouteLoaderControl>, tlc: Ptr<MSTLLogicControl>, stateDumpTimes: Vec<SUMOTime>, stateDumpFiles: Vec<String>, hasInternalLinks: bool, junctionHigherSpeeds: bool, version: &Unmapped /* MMVersion */) {
        todo!("port MSNet::closeBuilding")
    }

    /// C++: `bool MSNet::hasPermissions() const` [const]
    /// Returns whether the network has specific vehicle class permissions
    // bool hasPermissions() const {
    //         return myHavePermissions;
    //     }
    #[allow(unused_variables)]
    pub fn has_permissions(&self) -> bool {
        todo!("port MSNet::hasPermissions")
    }

    /// C++: `void MSNet::setPermissionsFound()`
    /// Labels the network to contain vehicle class permissions
    // void setPermissionsFound() {
    //         myHavePermissions = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_permissions_found(&mut self) {
        todo!("port MSNet::setPermissionsFound")
    }

    /// C++: `void MSNet::addRestriction(const std::string &, const SUMOVehicleClass, const double)`
    /// Adds a restriction for an edge type
    // void
    // MSNet::addRestriction(const std::string& id, const SUMOVehicleClass svc, const double speed) {
    //     myRestrictions[id][svc] = speed;
    // }
    #[allow(unused_variables)]
    pub fn add_restriction(&mut self, id: &str, svc: SUMOVehicleClass, speed: f64) {
        todo!("port MSNet::addRestriction")
    }

    /// C++: `const std::map<SUMOVehicleClass, double> * MSNet::getRestrictions(const std::string &) const` [const]
    /// Returns the restrictions for an edge type If no restrictions are present, 0 is returned.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_restrictions(&self, id: &str) -> Ptr<BTreeMap<SUMOVehicleClass, f64>> {
        todo!("port MSNet::getRestrictions")
    }

    /// C++: `double MSNet::getPreference(const std::string &, const SUMOVTypeParameter &) const` [const]
    /// retriefe edge type specific routing preference
    // double
    // MSNet::getPreference(const std::string& routingType, const SUMOVTypeParameter& pars) const {
    //     if (gRoutingPreferences) {
    //         auto it = myVTypePreferences.find(pars.id);
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //         auto it3 = myVClassPreferences.find(pars.vehicleClass);
    //         if (it3 != myVClassPreferences.end()) {
    //             auto it4 = it3->second.find(routingType);
    //             if (it4 != it3->second.end()) {
    //                 return it4->second;
    //             }
    //         }
    //         // fallback to generel preferences
    //         it = myVTypePreferences.find("");
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //     }
    //     return 1;
    // }
    #[allow(unused_variables)]
    pub fn get_preference(&self, routingType: &str, pars: &SUMOVTypeParameter) -> f64 {
        todo!("port MSNet::getPreference")
    }

    /// C++: `void MSNet::addPreference(const std::string &, SUMOVehicleClass, double)`
    /// add edge type specific routing preference
    // void
    // MSNet::addPreference(const std::string& routingType, SUMOVehicleClass svc, double prio) {
    //     myVClassPreferences[svc][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference(&mut self, routingType: &str, svc: SUMOVehicleClass, prio: f64) {
        todo!("port MSNet::addPreference")
    }

    /// C++: `void MSNet::addPreference(const std::string &, std::string, double)`
    /// add edge type specific routing preference
    // void
    // MSNet::addPreference(const std::string& routingType, std::string vType, double prio) {
    //     myVTypePreferences[vType][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference_2(&mut self, routingType: &str, vType: String, prio: f64) {
        todo!("port MSNet::addPreference")
    }

    /// C++: `void MSNet::addMesoType(const std::string &, const MESegment::MesoEdgeType &)`
    /// Adds edge type specific meso parameters
    // void
    // MSNet::addMesoType(const std::string& typeID, const MESegment::MesoEdgeType& edgeType) {
    //     myMesoEdgeTypes[typeID] = edgeType;
    // }
    #[allow(unused_variables)]
    pub fn add_meso_type(&mut self, typeID: &str, edgeType: &MESegment_MesoEdgeType) {
        todo!("port MSNet::addMesoType")
    }

    /// C++: `const MESegment::MesoEdgeType & MSNet::getMesoType(const std::string &)`
    /// Returns edge type specific meso parameters if no type specific parameters have been loaded, default values are returned
    // const MESegment::MesoEdgeType&
    // MSNet::getMesoType(const std::string& typeID) {
    //     if (myMesoEdgeTypes.count(typeID) == 0) {
    //         // init defaults
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         MESegment::MesoEdgeType edgeType;
    //         edgeType.tauff = string2time(oc.getString("meso-tauff"));
    //         edgeType.taufj = string2time(oc.getString("meso-taufj"));
    //         edgeType.taujf = string2time(oc.getString("meso-taujf"));
    //         edgeType.taujj = string2time(oc.getString("meso-taujj"));
    //         edgeType.jamThreshold = oc.getFloat("meso-jam-threshold");
    //         edgeType.junctionControl = oc.getBool("meso-junction-control");
    //         edgeType.tlsPenalty = oc.getFloat("meso-tls-penalty");
    //         edgeType.tlsFlowPenalty = oc.getFloat("meso-tls-flow-penalty");
    //         edgeType.minorPenalty = string2time(oc.getString("meso-minor-penalty"));
    //         edgeType.overtaking = oc.getBool("meso-overtaking");
    //         edgeType.edgeLength = oc.getFloat("meso-edgelength");
    //         edgeType.lookahead = oc.getFloat("meso-lookahead");
    //         myMesoEdgeTypes[typeID] = edgeType;
    //     }
    //     return myMesoEdgeTypes[typeID];
    // }
    #[allow(unused_variables)]
    pub fn get_meso_type(&mut self, typeID: &str) -> &MESegment_MesoEdgeType {
        todo!("port MSNet::getMesoType")
    }

    /// C++: `void MSNet::clearAll()` [static]
    /// Clears all dictionaries
    // void
    // MSNet::clearAll() {
    //     // clear container
    //     MSEdge::clear();
    //     MSLane::clear();
    //     MSRoute::clear();
    //     delete MSVehicleTransfer::getInstance();
    //     MSDevice::cleanupAll();
    //     MSCalibrator::cleanup();
    //     while (!MSLaneSpeedTrigger::getInstances().empty()) {
    //         delete MSLaneSpeedTrigger::getInstances().begin()->second;
    //     }
    //     while (!MSTriggeredRerouter::getInstances().empty()) {
    //         delete MSTriggeredRerouter::getInstances().begin()->second;
    //     }
    //     MSDevice_BTsender::cleanup();
    //     MSDevice_SSM::cleanup();
    //     MSDevice_ToC::cleanup();
    //     MSStopOut::cleanup();
    //     MSRailSignalConstraint::cleanup();
    //     MSRailSignalControl::cleanup();
    //     MSDriveWay::cleanup();
    //     TraCIServer* t = TraCIServer::getInstance();
    //     if (t != nullptr) {
    //         t->cleanup();
    //     }
    //     libsumo::Helper::cleanup();
    //     OutputDevice::closeAll(true);
    // }
    #[allow(unused_variables)]
    pub fn clear_all() {
        todo!("port MSNet::clearAll")
    }

    /// C++: `bool MSNet::hasFlow(const std::string &) const` [const]
    /// return whether the given flow is known
    // bool
    // MSNet::hasFlow(const std::string& id) const {
    //     // inserter is deleted at the end of the simulation
    //     return myInserter != nullptr && myInserter->hasFlow(id);
    // }
    #[allow(unused_variables)]
    pub fn has_flow(&self, id: &str) -> bool {
        todo!("port MSNet::hasFlow")
    }

    /// C++: `SimulationState MSNet::simulate(SUMOTime, SUMOTime)`
    /// Simulates from timestep start to stop
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn simulate(&mut self, start: SUMOTime, stop: SUMOTime) -> MSNet_SimulationState {
        todo!("port MSNet::simulate")
    }

    /// C++: `void MSNet::simulationStep(const bool)`
    /// Performs a single simulation step
    // void
    // MSNet::simulationStep(const bool onlyMove) {
    //     if (myStepCompletionMissing) {
    //         postMoveStep();
    //         myStepCompletionMissing = false;
    //         return;
    //     }
    // #ifdef DEBUG_SIMSTEP
    //     std::cout << SIMTIME << ": MSNet::simulationStep() called"
    //               << ", myStep = " << myStep
    //               << std::endl;
    // #endif
    //     TraCIServer* t = TraCIServer::getInstance();
    //     int lastTraCICmd = 0;
    //     if (t != nullptr) {
    //         if (myLogExecutionTime) {
    //             myTraCIStepDuration = SysUtils::getCurrentMillis();
    //         }
    //         lastTraCICmd = t->processCommands(myStep);
    // #ifdef DEBUG_SIMSTEP
    //         bool loadRequested = !TraCI::getLoadArgs().empty();
    //         assert(t->getTargetTime() >= myStep || loadRequested || TraCIServer::wasClosed());
    // #endif
    //         if (myLogExecutionTime) {
    //             myTraCIStepDuration = SysUtils::getCurrentMillis() - myTraCIStepDuration;
    //         }
    //         if (TraCIServer::wasClosed() || !t->getLoadArgs().empty()) {
    //             return;
    //         }
    //     }
    // #ifdef DEBUG_SIMSTEP
    //     std::cout << SIMTIME << ": TraCI target time: " << t->getTargetTime() << std::endl;
    // #endif
    //     // execute beginOfTimestepEvents
    //     if (myLogExecutionTime) {
    //         mySimStepDuration = SysUtils::getCurrentMillis();
    //     }
    //     // simulation state output
    //     std::vector<SUMOTime>::iterator timeIt = std::find(myStateDumpTimes.begin(), myStateDumpTimes.end(), myStep);
    //     if (timeIt != myStateDumpTimes.end()) {
    //         const int dist = (int)distance(myStateDumpTimes.begin(), timeIt);
    //         MSStateHandler::saveState(myStateDumpFiles[dist], myStep);
    //     }
    //     if (myStateDumpPeriod > 0 && myStep % myStateDumpPeriod == 0) {
    //         std::string timeStamp = time2string(myStep);
    //         std::replace(timeStamp.begin(), timeStamp.end(), ':', '-');
    //         const std::string filename = myStateDumpPrefix + "_" + timeStamp + myStateDumpSuffix;
    //         MSStateHandler::saveState(filename, myStep);
    //         myPeriodicStateFiles.push_back(filename);
    //         int keep = OptionsCont::getOptions().getInt("save-state.period.keep");
    //         if (keep > 0 && (int)myPeriodicStateFiles.size() > keep) {
    //             std::remove(myPeriodicStateFiles.front().c_str());
    //             myPeriodicStateFiles.erase(myPeriodicStateFiles.begin());
    //         }
    //     }
    //     myBeginOfTimestepEvents->execute(myStep);
    //     if (MSRailSignalControl::hasInstance()) {
    //         MSRailSignalControl::getInstance().updateSignals(myStep);
    //     }
    // #ifdef HAVE_FOX
    //     MSRoutingEngine::waitForAll();
    // #endif
    //     if (MSGlobals::gCheck4Accidents && !MSGlobals::gUseMesoSim) {
    //         myEdges->detectCollisions(myStep, STAGE_EVENTS);
    //     }
    //     // check whether the tls programs need to be switched
    //     myLogics->check2Switch(myStep);
    //
    //     if (MSGlobals::gUseMesoSim) {
    //         MSGlobals::gMesoNet->simulate(myStep);
    //     } else {
    //         // assure all lanes with vehicles are 'active'
    //         myEdges->patchActiveLanes();
    //
    //         // compute safe velocities for all vehicles for the next few lanes
    //         // also register ApproachingVehicleInformation for all links
    //         myEdges->planMovements(myStep);
    //
    //         // register junction approaches based on planned velocities as basis for right-of-way decision
    //         myEdges->setJunctionApproaches();
    //
    //         // decide right-of-way and execute movements
    //         myEdges->executeMovements(myStep);
    //         if (MSGlobals::gCheck4Accidents) {
    //             myEdges->detectCollisions(myStep, STAGE_MOVEMENTS);
    //         }
    //
    //         // vehicles may change lanes
    //         myEdges->changeLanes(myStep);
    //
    //         if (MSGlobals::gCheck4Accidents) {
    //             myEdges->detectCollisions(myStep, STAGE_LANECHANGE);
    //         }
    //     }
    //     // flush arrived meso vehicles and micro vehicles that were removed due to collision
    //     myVehicleControl->removePending();
    //     loadRoutes();
    //
    //     // persons
    //     if (myPersonControl != nullptr && myPersonControl->hasTransportables()) {
    //         myPersonControl->checkWaiting(this, myStep);
    //     }
    //     // containers
    //     if (myContainerControl != nullptr && myContainerControl->hasTransportables()) {
    //         myContainerControl->checkWaiting(this, myStep);
    //     }
    // #ifdef HAVE_FOX
    //     MSRoutingEngine::waitForAll();
    //     if (myPersonControl != nullptr) {
    //         myPersonControl->processPendingRouting(this, myStep);
    //     }
    //     if (myContainerControl != nullptr) {
    //         myContainerControl->processPendingRouting(this, myStep);
    //     }
    // #endif
    //     if (MSRailSignalControl::hasInstance()) {
    //         MSRailSignalControl::getInstance().resetWaitRelations();
    //         // preserve waitRelation from insertion for the next step
    //     }
    //     // insert vehicles
    //     myInserter->determineCandidates(myStep);
    //     myInsertionEvents->execute(myStep);
    // #ifdef HAVE_FOX
    //     MSRoutingEngine::waitForAll();
    // #endif
    //     myInserter->emitVehicles(myStep);
    //     if (MSGlobals::gCheck4Accidents && !MSGlobals::gUseMesoSim) {
    //         //myEdges->patchActiveLanes(); // @note required to detect collisions on lanes that were empty before insertion. wasteful?
    //         myEdges->detectCollisions(myStep, STAGE_INSERTIONS);
    //     }
    //     MSVehicleTransfer::getInstance()->checkInsertions(myStep);
    //
    //     // execute endOfTimestepEvents
    //     myEndOfTimestepEvents->execute(myStep);
    //
    //     if (myLogExecutionTime) {
    //         myTraCIStepDuration -= SysUtils::getCurrentMillis();
    //     }
    //     if (onlyMove) {
    //         myStepCompletionMissing = true;
    //         return;
    //     }
    //     if (t != nullptr && lastTraCICmd == libsumo::CMD_EXECUTEMOVE) {
    //         t->processCommands(myStep, true);
    //     }
    //     postMoveStep();
    // }
    #[allow(unused_variables)]
    pub fn simulation_step(&mut self, onlyMove: bool) {
        todo!("port MSNet::simulationStep")
    }

    /// C++: `void MSNet::loadRoutes()`
    /// loads routes for the next few steps
    // void
    // MSNet::loadRoutes() {
    //     myRouteLoaders->loadNext(myStep);
    // }
    #[allow(unused_variables)]
    pub fn load_routes(&mut self) {
        todo!("port MSNet::loadRoutes")
    }

    /// C++: `const std::string MSNet::generateStatistics(const SUMOTime, const long)`
    /// Writes performance output and running vehicle stats
    // const std::string
    // MSNet::generateStatistics(const SUMOTime start, const long now) {
    //     std::ostringstream msg;
    //     if (myLogExecutionTime) {
    //         const long duration = now - mySimBeginMillis;
    //         // print performance notice
    //         msg << "Performance:\n" << " Duration: " << elapsedMs2string(duration) << "\n";
    //         if (duration != 0) {
    //             if (TraCIServer::getInstance() != nullptr) {
    //                 msg << " TraCI-Duration: " << elapsedMs2string(myTraCIMillis) << "\n";
    //             }
    //             msg << " Real time factor: " << (STEPS2TIME(myStep - start) * 1000. / (double)duration) << "\n";
    //             msg.setf(std::ios::fixed, std::ios::floatfield);     // use decimal format
    //             msg.setf(std::ios::showpoint);    // print decimal point
    //             msg << " UPS: " << ((double)myVehiclesMoved / ((double)duration / 1000)) << "\n";
    //             if (myPersonsMoved > 0) {
    //                 msg << " UPS-Persons: " << ((double)myPersonsMoved / ((double)duration / 1000)) << "\n";
    //             }
    //         }
    //         // print vehicle statistics
    //         const std::string vehDiscardNotice = ((myVehicleControl->getLoadedVehicleNo() != myVehicleControl->getDepartedVehicleNo()) ?
    //                                               " (Loaded: " + toString(myVehicleControl->getLoadedVehicleNo()) + ")" : "");
    //         msg << "Vehicles:\n"
    //             << " Inserted: " << myVehicleControl->getDepartedVehicleNo() << vehDiscardNotice << "\n"
    //             << " Running: " << myVehicleControl->getRunningVehicleNo() << "\n"
    //             << " Waiting: " << myInserter->getWaitingVehicleNo() << "\n";
    //
    //         if (myVehicleControl->getTeleportCount() > 0 || myVehicleControl->getCollisionCount() > 0) {
    //             // print optional teleport statistics
    //             std::vector<std::string> reasons;
    //             if (myVehicleControl->getCollisionCount() > 0) {
    //                 reasons.push_back("Collisions: " + toString(myVehicleControl->getCollisionCount()));
    //             }
    //             if (myVehicleControl->getTeleportsJam() > 0) {
    //                 reasons.push_back("Jam: " + toString(myVehicleControl->getTeleportsJam()));
    //             }
    //             if (myVehicleControl->getTeleportsYield() > 0) {
    //                 reasons.push_back("Yield: " + toString(myVehicleControl->getTeleportsYield()));
    //             }
    //             if (myVehicleControl->getTeleportsWrongLane() > 0) {
    //                 reasons.push_back("Wrong Lane: " + toString(myVehicleControl->getTeleportsWrongLane()));
    //             }
    //             msg << " Teleports: " << myVehicleControl->getTeleportCount() << " (" << joinToString(reasons, ", ") << ")\n";
    //         }
    //         if (myVehicleControl->getEmergencyStops() > 0) {
    //             msg << " Emergency Stops: " << myVehicleControl->getEmergencyStops() << "\n";
    //         }
    //         if (myVehicleControl->getEmergencyBrakingCount() > 0) {
    //             msg << " Emergency Braking: " << myVehicleControl->getEmergencyBrakingCount() << "\n";
    //         }
    //         if (myPersonControl != nullptr && myPersonControl->getLoadedNumber() > 0) {
    //             const std::string discardNotice = ((myPersonControl->getLoadedNumber() != myPersonControl->getDepartedNumber()) ?
    //                                                " (Loaded: " + toString(myPersonControl->getLoadedNumber()) + ")" : "");
    //             msg << "Persons:\n"
    //                 << " Inserted: " << myPersonControl->getDepartedNumber() << discardNotice << "\n"
    //                 << " Running: " << myPersonControl->getRunningNumber() << "\n";
    //             if (myPersonControl->getJammedNumber() > 0) {
    //                 msg << " Jammed: " << myPersonControl->getJammedNumber() << "\n";
    //             }
    //             if (myPersonControl->getTeleportCount() > 0) {
    //                 std::vector<std::string> reasons;
    //                 if (myPersonControl->getTeleportsAbortWait() > 0) {
    //                     reasons.push_back("Abort Wait: " + toString(myPersonControl->getTeleportsAbortWait()));
    //                 }
    //                 if (myPersonControl->getTeleportsWrongDest() > 0) {
    //                     reasons.push_back("Wrong Dest: " + toString(myPersonControl->getTeleportsWrongDest()));
    //                 }
    //                 msg << " Teleports: " << myPersonControl->getTeleportCount() << " (" << joinToString(reasons, ", ") << ")\n";
    //             }
    //         }
    //         if (myContainerControl != nullptr && myContainerControl->getLoadedNumber() > 0) {
    //             const std::string discardNotice = ((myContainerControl->getLoadedNumber() != myContainerControl->getDepartedNumber()) ?
    //                                                " (Loaded: " + toString(myContainerControl->getLoadedNumber()) + ")" : "");
    //             msg << "Containers:\n"
    //                 << " Inserted: " << myContainerControl->getDepartedNumber() << "\n"
    //                 << " Running: " << myContainerControl->getRunningNumber() << "\n";
    //             if (myContainerControl->getJammedNumber() > 0) {
    //                 msg << " Jammed: " << myContainerControl->getJammedNumber() << "\n";
    //             }
    //             if (myContainerControl->getTeleportCount() > 0) {
    //                 std::vector<std::string> reasons;
    //                 if (myContainerControl->getTeleportsAbortWait() > 0) {
    //                     reasons.push_back("Abort Wait: " + toString(myContainerControl->getTeleportsAbortWait()));
    //                 }
    //                 if (myContainerControl->getTeleportsWrongDest() > 0) {
    //                     reasons.push_back("Wrong Dest: " + toString(myContainerControl->getTeleportsWrongDest()));
    //                 }
    //                 msg << " Teleports: " << myContainerControl->getTeleportCount() << " (" << joinToString(reasons, ", ") << ")\n";
    //             }
    //         }
    //     }
    //     if (OptionsCont::getOptions().getBool("duration-log.statistics")) {
    //         msg << MSDevice_Tripinfo::printStatistics();
    //     }
    //     std::string result = msg.str();
    //     result.erase(result.end() - 1);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn generate_statistics(&mut self, start: SUMOTime, now: i64) -> String {
        todo!("port MSNet::generateStatistics")
    }

    /// C++: `void MSNet::writeCollisions() const` [const]
    /// write collision output to (xml) file
    // void
    // MSNet::writeCollisions() const {
    //     OutputDevice& od = OutputDevice::getDeviceByOption("collision-output");
    //     for (const auto& item : myCollisions) {
    //         for (const auto& c : item.second) {
    //             if (c.time != SIMSTEP) {
    //                 continue;
    //             }
    //             od.openTag("collision");
    //             od.writeAttr("time", time2string(getCurrentTimeStep()));
    //             od.writeAttr("type", c.type);
    //             od.writeAttr("lane", c.lane->getID());
    //             od.writeAttr("pos", c.pos);
    //             od.writeAttr("collider", item.first);
    //             od.writeAttr("victim", c.victim);
    //             od.writeAttr("colliderType", c.colliderType);
    //             od.writeAttr("victimType", c.victimType);
    //             od.writeAttr("colliderSpeed", c.colliderSpeed);
    //             od.writeAttr("victimSpeed", c.victimSpeed);
    //             od.writeAttr("colliderFront", c.colliderFront);
    //             od.writeAttr("colliderBack", c.colliderBack);
    //             od.writeAttr("victimFront", c.victimFront);
    //             od.writeAttr("victimBack", c.victimBack);
    //             od.closeTag();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_collisions(&self) {
        todo!("port MSNet::writeCollisions")
    }

    /// C++: `void MSNet::writeStatistics(const SUMOTime, const long) const` [const]
    /// write statistic output to (xml) file
    // void
    // MSNet::writeStatistics(const SUMOTime start, const long now) const {
    //     const long duration = now - mySimBeginMillis;
    //     OutputDevice& od = OutputDevice::getDeviceByOption("statistic-output");
    //     od.openTag("performance");
    //     od.writeAttr("clockBegin", time2string(mySimBeginMillis));
    //     od.writeAttr("clockEnd", time2string(now));
    //     od.writeAttr("clockDuration", time2string(duration));
    //     od.writeAttr("traciDuration", time2string(myTraCIMillis));
    //     od.writeAttr("realTimeFactor", duration != 0 ? (double)(myStep - start) / (double)duration : -1);
    //     od.writeAttr("vehicleUpdatesPerSecond", duration != 0 ? (double)myVehiclesMoved / ((double)duration / 1000) : -1);
    //     od.writeAttr("personUpdatesPerSecond", duration != 0 ? (double)myPersonsMoved / ((double)duration / 1000) : -1);
    //     od.writeAttr("begin", time2string(start));
    //     od.writeAttr("end", time2string(myStep));
    //     od.writeAttr("duration", time2string(myStep - start));
    //     od.closeTag();
    //     od.openTag("vehicles");
    //     od.writeAttr("loaded", myVehicleControl->getLoadedVehicleNo());
    //     od.writeAttr("inserted", myVehicleControl->getDepartedVehicleNo());
    //     od.writeAttr("running", myVehicleControl->getRunningVehicleNo());
    //     od.writeAttr("waiting", myInserter->getWaitingVehicleNo());
    //     od.closeTag();
    //     od.openTag("teleports");
    //     od.writeAttr("total", myVehicleControl->getTeleportCount());
    //     od.writeAttr("jam", myVehicleControl->getTeleportsJam());
    //     od.writeAttr("yield", myVehicleControl->getTeleportsYield());
    //     od.writeAttr("wrongLane", myVehicleControl->getTeleportsWrongLane());
    //     od.closeTag();
    //     od.openTag("safety");
    //     od.writeAttr("collisions", myVehicleControl->getCollisionCount());
    //     od.writeAttr("emergencyStops", myVehicleControl->getEmergencyStops());
    //     od.writeAttr("emergencyBraking", myVehicleControl->getEmergencyBrakingCount());
    //     od.closeTag();
    //     od.openTag("persons");
    //     od.writeAttr("loaded", myPersonControl != nullptr ? myPersonControl->getLoadedNumber() : 0);
    //     od.writeAttr("running", myPersonControl != nullptr ? myPersonControl->getRunningNumber() : 0);
    //     od.writeAttr("jammed", myPersonControl != nullptr ? myPersonControl->getJammedNumber() : 0);
    //     od.closeTag();
    //     od.openTag("personTeleports");
    //     od.writeAttr("total", myPersonControl != nullptr ? myPersonControl->getTeleportCount() : 0);
    //     od.writeAttr("abortWait", myPersonControl != nullptr ? myPersonControl->getTeleportsAbortWait() : 0);
    //     od.writeAttr("wrongDest", myPersonControl != nullptr ? myPersonControl->getTeleportsWrongDest() : 0);
    //     od.closeTag();
    //     if (OptionsCont::getOptions().isSet("tripinfo-output") || OptionsCont::getOptions().getBool("duration-log.statistics")) {
    //         MSDevice_Tripinfo::writeStatistics(od);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_statistics(&self, start: SUMOTime, now: i64) {
        todo!("port MSNet::writeStatistics")
    }

    /// C++: `void MSNet::writeSummaryOutput(bool)`
    /// write summary-output to (xml) file
    // void
    // MSNet::writeSummaryOutput(bool finalStep) {
    //     // summary output
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const bool hasOutput = oc.isSet("summary-output");
    //     const bool hasPersonOutput = oc.isSet("person-summary-output");
    //     if (hasOutput || hasPersonOutput) {
    //         const SUMOTime period = string2time(oc.getString("summary-output.period"));
    //         const SUMOTime begin = string2time(oc.getString("begin"));
    //         if ((period > 0 && (myStep - begin) % period != 0 && !finalStep)
    //                 // it's the final step but we already wrote output
    //                 || (finalStep && (period <= 0 || (myStep - begin) % period == 0))) {
    //             return;
    //         }
    //     }
    //     if (hasOutput) {
    //         OutputDevice& od = OutputDevice::getDeviceByOption("summary-output");
    //         int departedVehiclesNumber = myVehicleControl->getDepartedVehicleNo();
    //         const double meanWaitingTime = departedVehiclesNumber != 0 ? myVehicleControl->getTotalDepartureDelay() / (double) departedVehiclesNumber : -1.;
    //         int endedVehicleNumber = myVehicleControl->getEndedVehicleNo();
    //         const double meanTravelTime = endedVehicleNumber != 0 ? myVehicleControl->getTotalTravelTime() / (double) endedVehicleNumber : -1.;
    //         od.openTag("step");
    //         od.writeAttr("time", time2string(myStep));
    //         od.writeAttr("loaded", myVehicleControl->getLoadedVehicleNo());
    //         od.writeAttr("inserted", myVehicleControl->getDepartedVehicleNo());
    //         od.writeAttr("running", myVehicleControl->getRunningVehicleNo());
    //         od.writeAttr("waiting", myInserter->getWaitingVehicleNo());
    //         od.writeAttr("ended", myVehicleControl->getEndedVehicleNo());
    //         od.writeAttr("arrived", myVehicleControl->getArrivedVehicleNo());
    //         od.writeAttr("collisions", myVehicleControl->getCollisionCount());
    //         od.writeAttr("teleports", myVehicleControl->getTeleportCount());
    //         od.writeAttr("halting", myVehicleControl->getHaltingVehicleNo());
    //         od.writeAttr("stopped", myVehicleControl->getStoppedVehiclesCount());
    //         od.writeAttr("meanWaitingTime", meanWaitingTime);
    //         od.writeAttr("meanTravelTime", meanTravelTime);
    //         std::pair<double, double> meanSpeed = myVehicleControl->getVehicleMeanSpeeds();
    //         od.writeAttr("meanSpeed", meanSpeed.first);
    //         od.writeAttr("meanSpeedRelative", meanSpeed.second);
    //         od.writeAttr("discarded", myVehicleControl->getDiscardedVehicleNo());
    //         if (myLogExecutionTime) {
    //             od.writeAttr("duration", mySimStepDuration);
    //         }
    //         od.closeTag();
    //     }
    //     if (hasPersonOutput) {
    //         OutputDevice& od = OutputDevice::getDeviceByOption("person-summary-output");
    //         MSTransportableControl& pc = getPersonControl();
    //         od.openTag("step");
    //         od.writeAttr("time", time2string(myStep));
    //         od.writeAttr("loaded", pc.getLoadedNumber());
    //         od.writeAttr("inserted", pc.getDepartedNumber());
    //         od.writeAttr("walking", pc.getMovingNumber());
    //         od.writeAttr("waitingForRide", pc.getWaitingForVehicleNumber());
    //         od.writeAttr("riding", pc.getRidingNumber());
    //         od.writeAttr("stopping", pc.getWaitingUntilNumber());
    //         od.writeAttr("jammed", pc.getJammedNumber());
    //         od.writeAttr("ended", pc.getEndedNumber());
    //         od.writeAttr("arrived", pc.getArrivedNumber());
    //         od.writeAttr("teleports", pc.getTeleportCount());
    //         od.writeAttr("discarded", pc.getDiscardedNumber());
    //         if (myLogExecutionTime) {
    //             od.writeAttr("duration", mySimStepDuration);
    //         }
    //         od.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_summary_output(&mut self, finalStep: bool) {
        todo!("port MSNet::writeSummaryOutput")
    }

    /// C++: `void MSNet::closeSimulation(SUMOTime, const std::string &)`
    /// Closes the simulation (all files, connections, etc.)
    // void
    // MSNet::closeSimulation(SUMOTime start, const std::string& reason) {
    //     // report the end when wished
    //     WRITE_MESSAGE(TLF("Simulation ended at time: %.", time2string(getCurrentTimeStep())));
    //     if (reason != "") {
    //         WRITE_MESSAGE(TL("Reason: ") + reason);
    //     }
    //     myDetectorControl->close(myStep);
    //     if (OptionsCont::getOptions().isSet("queue-output")) {
    //         MSQueueExport::finish(OutputDevice::getDeviceByOption("queue-output"), myStep);
    //     }
    //     if (MSStopOut::active() && OptionsCont::getOptions().getBool("stop-output.write-unfinished")) {
    //         MSStopOut::getInstance()->generateOutputForUnfinished();
    //     }
    //     MSDevice_Vehroutes::writePendingOutput(OptionsCont::getOptions().getBool("vehroute-output.write-unfinished"));
    //     if (OptionsCont::getOptions().getBool("tripinfo-output.write-unfinished")) {
    //         MSDevice_Tripinfo::generateOutputForUnfinished();
    //     }
    //     if (OptionsCont::getOptions().isSet("chargingstations-output")) {
    //         if (!OptionsCont::getOptions().getBool("chargingstations-output.aggregated")) {
    //             writeChargingStationOutput();
    //         } else if (OptionsCont::getOptions().getBool("chargingstations-output.aggregated.write-unfinished")) {
    //             MSChargingStationExport::write(OutputDevice::getDeviceByOption("chargingstations-output"), true);
    //         }
    //     }
    //     if (OptionsCont::getOptions().isSet("overheadwiresegments-output")) {
    //         writeOverheadWireSegmentOutput();
    //     }
    //     if (OptionsCont::getOptions().isSet("substations-output")) {
    //         writeSubstationOutput();
    //     }
    //     writeRailSignalBlocks();
    //     const long now = SysUtils::getCurrentMillis();
    //     if (myLogExecutionTime || OptionsCont::getOptions().getBool("duration-log.statistics")) {
    //         WRITE_MESSAGE(generateStatistics(start, now));
    //     }
    //     if (OptionsCont::getOptions().isSet("statistic-output")) {
    //         writeStatistics(start, now);
    //     }
    //     // maybe write a final line of output if reporting is periodic
    //     writeSummaryOutput(true);
    // }
    #[allow(unused_variables)]
    pub fn close_simulation(&mut self, start: SUMOTime, reason: &str) {
        todo!("port MSNet::closeSimulation")
    }

    /// C++: `SimulationState MSNet::simulationState(SUMOTime) const` [const]
    /// This method returns the current simulation state. It should not modify status.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn simulation_state(&self, stopTime: SUMOTime) -> MSNet_SimulationState {
        todo!("port MSNet::simulationState")
    }

    /// C++: `SimulationState MSNet::adaptToState(const SimulationState, const bool) const` [const]
    /// Called after a simulation step, this method adapts the current simulation state if necessary
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn adapt_to_state(&self, state: MSNet_SimulationState, isLibsumo: bool) -> MSNet_SimulationState {
        todo!("port MSNet::adaptToState")
    }

    /// C++: `std::string MSNet::getStateMessage(SimulationState)` [static]
    /// Returns the message to show if a certain state occurs
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_state_message(state: MSNet_SimulationState) -> String {
        todo!("port MSNet::getStateMessage")
    }

    /// C++: `SUMOTime MSNet::getCurrentTimeStep() const` [const]
    /// Returns the current simulation step
    // inline SUMOTime getCurrentTimeStep() const {
    //         return myStep;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_time_step(&self) -> SUMOTime {
        todo!("port MSNet::getCurrentTimeStep")
    }

    /// C++: `void MSNet::setCurrentTimeStep(const SUMOTime)`
    /// Sets the current simulation step (used by state loading)
    // inline void setCurrentTimeStep(const SUMOTime step) {
    //         myStep = step;
    //     }
    #[allow(unused_variables)]
    pub fn set_current_time_step(&mut self, step: SUMOTime) {
        todo!("port MSNet::setCurrentTimeStep")
    }

    /// C++: `void MSNet::clearState(const SUMOTime, bool)`
    /// Resets events when quick-loading state
    // void
    // MSNet::clearState(const SUMOTime step, bool quickReload) {
    //     MSGlobals::gClearState = true;
    //     if (MSGlobals::gUseMesoSim) {
    //         MSGlobals::gMesoNet->clearState();
    //         for (MSEdge* const edge : MSEdge::getAllEdges()) {
    //             for (MESegment* s = MSGlobals::gMesoNet->getSegmentForEdge(*edge); s != nullptr; s = s->getNextSegment()) {
    //                 s->clearState();
    //             }
    //             for (MSLane* const lane : edge->getLanes()) {
    //                 for (MSLink* link : lane->getLinkCont()) {
    //                     link->clearState();
    //                 }
    //             }
    //         }
    //     } else {
    //         for (MSEdge* const edge : MSEdge::getAllEdges()) {
    //             for (MSLane* const lane : edge->getLanes()) {
    //                 lane->getVehiclesSecure();
    //                 lane->clearState();
    //                 lane->releaseVehicles();
    //             }
    //             edge->clearState();
    //         }
    //     }
    //     myInserter->clearState();
    //     // detectors may still reference persons/vehicles
    //     myDetectorControl->updateDetectors(myStep);
    //     myDetectorControl->writeOutput(myStep, true);
    //     myDetectorControl->clearState(step);
    //
    //     if (myPersonControl != nullptr) {
    //         myPersonControl->clearState();
    //     }
    //     if (myContainerControl != nullptr) {
    //         myContainerControl->clearState();
    //     }
    //     // delete vtypes after transportables have removed their types
    //     myVehicleControl->clearState(true);
    //     MSVehicleTransfer::getInstance()->clearState();
    //     myLogics->clearState(step, quickReload);
    //     // delete all routes after vehicles and detector output is done
    //     MSRoute::dict_clearState();
    //     for (auto& item : myStoppingPlaces) {
    //         for (auto& item2 : item.second) {
    //             item2.second->clearState();
    //         }
    //     }
    //     myShapeContainer->clearState();
    //     myBeginOfTimestepEvents->clearState(myStep, step);
    //     myEndOfTimestepEvents->clearState(myStep, step);
    //     myInsertionEvents->clearState(myStep, step);
    //     MSRailSignalControl::clearState();
    //     MSDriveWay::clearState();
    //     myStep = step;
    //     MSGlobals::gClearState = false;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, step: SUMOTime, quickReload: bool) {
        todo!("port MSNet::clearState")
    }

    /// C++: `SUMOTime MSNet::getLoaderTime() const` [const]
    // SUMOTime
    // MSNet::getLoaderTime() const {
    //     return myRouteLoaders->getCurrentLoadTime();
    // }
    #[allow(unused_variables)]
    pub fn get_loader_time(&self) -> SUMOTime {
        todo!("port MSNet::getLoaderTime")
    }

    /// C++: `void MSNet::setLoaderTime(SUMOTime)`
    // void
    // MSNet::setLoaderTime(SUMOTime time) {
    //     myRouteLoaders->setCurrentLoadTime(time);
    //     myStateLoaderTime = MAX2(myStateLoaderTime, time);
    // }
    #[allow(unused_variables)]
    pub fn set_loader_time(&mut self, time: SUMOTime) {
        todo!("port MSNet::setLoaderTime")
    }

    /// C++: `SUMOTime MSNet::getStateLoaderTime() const` [const]
    /// return the next time for route loading the time of state loading
    // SUMOTime getStateLoaderTime() const {
    //         return myStateLoaderTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_state_loader_time(&self) -> SUMOTime {
        todo!("port MSNet::getStateLoaderTime")
    }

    /// C++: `void MSNet::writeOutput()`
    /// Write netstate, summary and detector output
    // void
    // MSNet::writeOutput() {
    //     // update detector values
    //     myDetectorControl->updateDetectors(myStep);
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //
    //     // check state dumps
    //     if (oc.isSet("netstate-dump")) {
    //         MSXMLRawOut::write(OutputDevice::getDeviceByOption("netstate-dump"), *myEdges, myStep,
    //                            oc.getInt("netstate-dump.precision"));
    //     }
    //
    //     // check fcd dumps
    //     if (OptionsCont::getOptions().isSet("fcd-output")) {
    //         if (OptionsCont::getOptions().isSet("person-fcd-output")) {
    //             MSFCDExport::write(OutputDevice::getDeviceByOption("fcd-output"), myStep, SUMO_TAG_VEHICLE);
    //             MSFCDExport::write(OutputDevice::getDeviceByOption("person-fcd-output"), myStep, SUMO_TAG_PERSON);
    //         } else {
    //             MSFCDExport::write(OutputDevice::getDeviceByOption("fcd-output"), myStep);
    //         }
    //     }
    //
    //     // check emission dumps
    //     if (OptionsCont::getOptions().isSet("emission-output")) {
    //         MSEmissionExport::write(OutputDevice::getDeviceByOption("emission-output"), myStep);
    //     }
    //
    //     // battery dumps
    //     if (OptionsCont::getOptions().isSet("battery-output")) {
    //         MSBatteryExport::write(OutputDevice::getDeviceByOption("battery-output"), myStep,
    //                                oc.getInt("battery-output.precision"));
    //     }
    //
    //     // charging station aggregated dumps
    //     if (OptionsCont::getOptions().isSet("chargingstations-output") && OptionsCont::getOptions().getBool("chargingstations-output.aggregated")) {
    //         MSChargingStationExport::write(OutputDevice::getDeviceByOption("chargingstations-output"));
    //     }
    //
    //     // elecHybrid dumps
    //     if (OptionsCont::getOptions().isSet("elechybrid-output")) {
    //         std::string output = OptionsCont::getOptions().getString("elechybrid-output");
    //
    //         if (oc.getBool("elechybrid-output.aggregated")) {
    //             // build a xml file with aggregated device.elechybrid output
    //             MSElecHybridExport::writeAggregated(OutputDevice::getDeviceByOption("elechybrid-output"), myStep,
    //                                                 oc.getInt("elechybrid-output.precision"));
    //         } else {
    //             // build a separate xml file for each vehicle equipped with device.elechybrid
    //             // RICE_TODO: Does this have to be placed here in MSNet.cpp ?
    //             MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //             for (MSVehicleControl::constVehIt it = vc.loadedVehBegin(); it != vc.loadedVehEnd(); ++it) {
    //                 const SUMOVehicle* veh = it->second;
    //                 if (!veh->isOnRoad()) {
    //                     continue;
    //                 }
    //                 if (static_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid))) != nullptr) {
    //                     std::string vehID = veh->getID();
    //                     std::string filename2 = output + "_" + vehID + ".xml";
    //                     OutputDevice& dev = OutputDevice::getDevice(filename2);
    //                     std::map<SumoXMLAttr, std::string> attrs;
    //                     attrs[SUMO_ATTR_VEHICLE] = vehID;
    //                     attrs[SUMO_ATTR_MAXIMUMBATTERYCAPACITY] = toString(dynamic_cast<MSDevice_ElecHybrid*>(veh->getDevice(typeid(MSDevice_ElecHybrid)))->getMaximumBatteryCapacity());
    //                     attrs[SUMO_ATTR_RECUPERATIONENABLE] = toString(MSGlobals::gOverheadWireRecuperation);
    //                     dev.writeXMLHeader("elecHybrid-export", "", attrs);
    //                     MSElecHybridExport::write(OutputDevice::getDevice(filename2), veh, myStep, oc.getInt("elechybrid-output.precision"));
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     // check full dumps
    //     if (OptionsCont::getOptions().isSet("full-output")) {
    //         MSGlobals::gHaveEmissions = true;
    //         MSFullExport::write(OutputDevice::getDeviceByOption("full-output"), myStep);
    //     }
    //
    //     // check queue dumps
    //     if (OptionsCont::getOptions().isSet("queue-output")) {
    //         MSQueueExport::write(OutputDevice::getDeviceByOption("queue-output"), myStep);
    //     }
    //
    //     // check amitran dumps
    //     if (OptionsCont::getOptions().isSet("amitran-output")) {
    //         MSAmitranTrajectories::write(OutputDevice::getDeviceByOption("amitran-output"), myStep);
    //     }
    //
    //     // check vtk dumps
    //     if (OptionsCont::getOptions().isSet("vtk-output")) {
    //
    //         if (MSNet::getInstance()->getVehicleControl().getRunningVehicleNo() > 0) {
    //             std::string timestep = time2string(myStep);
    //             if (TS >= 1.0) {
    //                 timestep = timestep.substr(0, timestep.length() - 3);
    //             } else if (DELTA_T % 100 == 0) {
    //                 timestep = timestep.substr(0, timestep.length() - 1);
    //             }
    //             std::string output = OptionsCont::getOptions().getString("vtk-output");
    //             std::string filename = output + "_" + timestep + ".vtp";
    //
    //             OutputDevice_File dev(filename);
    //
    //             //build a huge mass of xml files
    //             MSVTKExport::write(dev, myStep);
    //
    //         }
    //
    //     }
    //
    //     writeSummaryOutput();
    //
    //     // write detector values
    //     myDetectorControl->writeOutput(myStep + DELTA_T, false);
    //
    //     // write link states
    //     if (OptionsCont::getOptions().isSet("link-output")) {
    //         OutputDevice& od = OutputDevice::getDeviceByOption("link-output");
    //         od.openTag("timestep");
    //         od.writeAttr(SUMO_ATTR_ID, STEPS2TIME(myStep));
    //         for (const MSEdge* const edge : myEdges->getEdges()) {
    //             for (const MSLane* const lane : edge->getLanes()) {
    //                 for (const MSLink* const link : lane->getLinkCont()) {
    //                     link->writeApproaching(od, lane->getID());
    //                 }
    //             }
    //         }
    //         od.closeTag();
    //     }
    //
    //     // write SSM output
    //     for (MSDevice_SSM* dev : MSDevice_SSM::getInstances()) {
    //         dev->updateAndWriteOutput();
    //     }
    //
    //     // write ToC output
    //     for (MSDevice_ToC* dev : MSDevice_ToC::getInstances()) {
    //         if (dev->generatesOutput()) {
    //             dev->writeOutput();
    //         }
    //     }
    //
    //     if (OptionsCont::getOptions().isSet("collision-output")) {
    //         writeCollisions();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_output(&mut self) {
        todo!("port MSNet::writeOutput")
    }

    /// C++: `bool MSNet::logSimulationDuration() const` [const]
    /// Returns whether duration shall be logged
    // bool
    // MSNet::logSimulationDuration() const {
    //     return myLogExecutionTime;
    // }
    #[allow(unused_variables)]
    pub fn log_simulation_duration(&self) -> bool {
        todo!("port MSNet::logSimulationDuration")
    }

    /// C++: `void MSNet::preSimStepOutput() const` [const]
    /// Prints the current step number
    // void
    // MSNet::preSimStepOutput() const {
    //     std::cout << "Step #" << time2string(myStep);
    // }
    #[allow(unused_variables)]
    pub fn pre_sim_step_output(&self) {
        todo!("port MSNet::preSimStepOutput")
    }

    /// C++: `void MSNet::postSimStepOutput() const` [const]
    /// Prints the statistics of the step at its end
    // void
    // MSNet::postSimStepOutput() const {
    //     if (myLogExecutionTime) {
    //         std::ostringstream oss;
    //         oss.setf(std::ios::fixed, std::ios::floatfield);     // use decimal format
    //         oss.setf(std::ios::showpoint);    // print decimal point
    //         oss << std::setprecision(gPrecision);
    //         if (mySimStepDuration != 0) {
    //             const double durationSec = (double)mySimStepDuration / 1000.;
    //             oss << " (" << mySimStepDuration << "ms ~= "
    //                 << (TS / durationSec) << "*RT, ~"
    //                 << ((double) myVehicleControl->getRunningVehicleNo() / durationSec);
    //         } else {
    //             oss << " (0ms ?*RT. ?";
    //         }
    //         oss << "UPS, ";
    //         if (TraCIServer::getInstance() != nullptr) {
    //             oss << "TraCI: " << myTraCIStepDuration << "ms, ";
    //         }
    //         oss << "vehicles TOT " << myVehicleControl->getDepartedVehicleNo()
    //             << " ACT " << myVehicleControl->getRunningVehicleNo()
    //             << " BUF " << myInserter->getWaitingVehicleNo()
    //             << ")                                              ";
    //         std::string prev = "Step #" + time2string(myStep - DELTA_T);
    //         std::cout << oss.str().substr(0, 90 - prev.length());
    //     }
    //     std::cout << '\r';
    // }
    #[allow(unused_variables)]
    pub fn post_sim_step_output(&self) {
        todo!("port MSNet::postSimStepOutput")
    }

    /// C++: `MSVehicleControl & MSNet::getVehicleControl()`
    /// Returns the vehicle control
    // MSVehicleControl& getVehicleControl() {
    //         return *myVehicleControl;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_control(&mut self) -> &mut MSVehicleControl {
        todo!("port MSNet::getVehicleControl")
    }

    /// C++: `MSTransportableControl & MSNet::getPersonControl()` [virtual]
    /// Returns the person control
    // MSTransportableControl&
    // MSNet::getPersonControl() {
    //     if (myPersonControl == nullptr) {
    //         myPersonControl = new MSTransportableControl(true);
    //     }
    //     return *myPersonControl;
    // }
    #[allow(unused_variables)]
    pub fn get_person_control(&mut self) -> &mut MSTransportableControl {
        todo!("port MSNet::getPersonControl")
    }

    /// C++: `bool MSNet::hasPersons() const` [const]
    /// Returns whether persons are simulated
    // bool hasPersons() const {
    //         return myPersonControl != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_persons(&self) -> bool {
        todo!("port MSNet::hasPersons")
    }

    /// C++: `MSTransportableControl & MSNet::getContainerControl()` [virtual]
    /// Returns the container control
    // MSTransportableControl&
    // MSNet::getContainerControl() {
    //     if (myContainerControl == nullptr) {
    //         myContainerControl = new MSTransportableControl(false);
    //     }
    //     return *myContainerControl;
    // }
    #[allow(unused_variables)]
    pub fn get_container_control(&mut self) -> &mut MSTransportableControl {
        todo!("port MSNet::getContainerControl")
    }

    /// C++: `bool MSNet::hasContainers() const` [const]
    /// Returns whether containers are simulated
    // bool hasContainers() const {
    //         return myContainerControl != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_containers(&self) -> bool {
        todo!("port MSNet::hasContainers")
    }

    /// C++: `MSEdgeControl & MSNet::getEdgeControl()`
    /// Returns the edge control
    // MSEdgeControl& getEdgeControl() {
    //         return *myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_control(&mut self) -> &mut MSEdgeControl {
        todo!("port MSNet::getEdgeControl")
    }

    /// C++: `MSInsertionControl & MSNet::getInsertionControl()`
    /// Returns the insertion control
    // MSInsertionControl& getInsertionControl() {
    //         return *myInserter;
    //     }
    #[allow(unused_variables)]
    pub fn get_insertion_control(&mut self) -> &mut MSInsertionControl {
        todo!("port MSNet::getInsertionControl")
    }

    /// C++: `MSDetectorControl & MSNet::getDetectorControl()`
    /// Returns the detector control
    // MSDetectorControl& getDetectorControl() {
    //         return *myDetectorControl;
    //     }
    #[allow(unused_variables)]
    pub fn get_detector_control(&mut self) -> &mut MSDetectorControl {
        todo!("port MSNet::getDetectorControl")
    }

    /// C++: `MSTLLogicControl & MSNet::getTLSControl()`
    /// Returns the tls logics control
    // MSTLLogicControl& getTLSControl() {
    //         return *myLogics;
    //     }
    #[allow(unused_variables)]
    pub fn get_tls_control(&mut self) -> &mut MSTLLogicControl {
        todo!("port MSNet::getTLSControl")
    }

    /// C++: `MSJunctionControl & MSNet::getJunctionControl()`
    /// Returns the junctions control
    // MSJunctionControl& getJunctionControl() {
    //         return *myJunctions;
    //     }
    #[allow(unused_variables)]
    pub fn get_junction_control(&mut self) -> &mut MSJunctionControl {
        todo!("port MSNet::getJunctionControl")
    }

    /// C++: `MSEventControl * MSNet::getBeginOfTimestepEvents()`
    /// Returns the event control for events executed at the begin of a time step
    // MSEventControl* getBeginOfTimestepEvents() {
    //         return myBeginOfTimestepEvents;
    //     }
    #[allow(unused_variables)]
    pub fn get_begin_of_timestep_events(&mut self) -> Ptr<MSEventControl> {
        todo!("port MSNet::getBeginOfTimestepEvents")
    }

    /// C++: `MSEventControl * MSNet::getEndOfTimestepEvents()`
    /// Returns the event control for events executed at the end of a time step
    // MSEventControl* getEndOfTimestepEvents() {
    //         return myEndOfTimestepEvents;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_of_timestep_events(&mut self) -> Ptr<MSEventControl> {
        todo!("port MSNet::getEndOfTimestepEvents")
    }

    /// C++: `MSEventControl * MSNet::getInsertionEvents()`
    /// Returns the event control for insertion events
    // MSEventControl* getInsertionEvents() {
    //         return myInsertionEvents;
    //     }
    #[allow(unused_variables)]
    pub fn get_insertion_events(&mut self) -> Ptr<MSEventControl> {
        todo!("port MSNet::getInsertionEvents")
    }

    /// C++: `ShapeContainer & MSNet::getShapeContainer()`
    /// Returns the shapes container
    // ShapeContainer& getShapeContainer() {
    //         return *myShapeContainer;
    //     }
    #[allow(unused_variables)]
    pub fn get_shape_container(&mut self) -> &mut ShapeContainer {
        todo!("port MSNet::getShapeContainer")
    }

    /// C++: `MSDynamicShapeUpdater * MSNet::getDynamicShapeUpdater()`
    /// Returns the dynamic shapes updater
    // MSDynamicShapeUpdater* getDynamicShapeUpdater() {
    //         return myDynamicShapeUpdater.get();
    //     }
    #[allow(unused_variables)]
    pub fn get_dynamic_shape_updater(&mut self) -> Ptr<MSDynamicShapeUpdater> {
        todo!("port MSNet::getDynamicShapeUpdater")
    }

    /// C++: `MSDynamicShapeUpdater * MSNet::makeDynamicShapeUpdater()`
    /// Creates and returns a dynamic shapes updater
    // MSDynamicShapeUpdater*
    // MSNet::makeDynamicShapeUpdater() {
    //     myDynamicShapeUpdater = std::unique_ptr<MSDynamicShapeUpdater> (new MSDynamicShapeUpdater(*myShapeContainer));
    //     return myDynamicShapeUpdater.get();
    // }
    #[allow(unused_variables)]
    pub fn make_dynamic_shape_updater(&mut self) -> Ptr<MSDynamicShapeUpdater> {
        todo!("port MSNet::makeDynamicShapeUpdater")
    }

    /// C++: `MSEdgeWeightsStorage & MSNet::getWeightsStorage()`
    /// Returns the net's internal edge travel times/efforts container
    // MSEdgeWeightsStorage&
    // MSNet::getWeightsStorage() {
    //     if (myEdgeWeights == nullptr) {
    //         myEdgeWeights = new MSEdgeWeightsStorage();
    //     }
    //     return *myEdgeWeights;
    // }
    #[allow(unused_variables)]
    pub fn get_weights_storage(&mut self) -> &mut MSEdgeWeightsStorage {
        todo!("port MSNet::getWeightsStorage")
    }

    /// C++: `bool MSNet::addStoppingPlace(SumoXMLTag, MSStoppingPlace *)`
    /// Adds a stopping place
    // bool
    // MSNet::addStoppingPlace(SumoXMLTag category, MSStoppingPlace* stop) {
    //     if (category == SUMO_TAG_TRAIN_STOP) {
    //         category = SUMO_TAG_BUS_STOP;
    //     }
    //     const bool isNew = myStoppingPlaces[category].add(stop->getID(), stop);
    //     if (isNew && stop->getMyName() != "") {
    //         myNamedStoppingPlaces[category][stop->getMyName()].push_back(stop);
    //     }
    //     return isNew;
    // }
    #[allow(unused_variables)]
    pub fn add_stopping_place(&mut self, category: SumoXMLTag, stop: Ptr<MSStoppingPlace>) -> bool {
        todo!("port MSNet::addStoppingPlace")
    }

    /// C++: `bool MSNet::addTractionSubstation(MSTractionSubstation *)`
    /// Adds a traction substation
    // bool
    // MSNet::addTractionSubstation(MSTractionSubstation* substation) {
    //     if (find(myTractionSubstations.begin(), myTractionSubstations.end(), substation) == myTractionSubstations.end()) {
    //         myTractionSubstations.push_back(substation);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_traction_substation(&mut self, substation: Ptr<MSTractionSubstation>) -> bool {
        todo!("port MSNet::addTractionSubstation")
    }

    /// C++: `MSStoppingPlace * MSNet::getStoppingPlace(const std::string &, const SumoXMLTag) const` [const]
    /// Returns the named stopping place of the given category
    // MSStoppingPlace*
    // MSNet::getStoppingPlace(const std::string& id, const SumoXMLTag category) const {
    //     if (myStoppingPlaces.count(category) > 0) {
    //         return myStoppingPlaces.find(category)->second.get(id);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place(&self, id: &str, category: SumoXMLTag) -> Ptr<MSStoppingPlace> {
        todo!("port MSNet::getStoppingPlace")
    }

    /// C++: `MSStoppingPlace * MSNet::getStoppingPlace(const std::string &) const` [const]
    /// Returns the named stopping place by looking through all categories
    // MSStoppingPlace*
    // MSNet::getStoppingPlace(const std::string& id) const {
    //     for (SumoXMLTag category : std::vector<SumoXMLTag>({SUMO_TAG_BUS_STOP, SUMO_TAG_PARKING_AREA, SUMO_TAG_CONTAINER_STOP, SUMO_TAG_CHARGING_STATION, SUMO_TAG_OVERHEAD_WIRE_SEGMENT})) {
    //         MSStoppingPlace* result = getStoppingPlace(id, category);
    //         if (result != nullptr) {
    //             return result;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_2(&self, id: &str) -> Ptr<MSStoppingPlace> {
        todo!("port MSNet::getStoppingPlace")
    }

    /// C++: `std::string MSNet::getStoppingPlaceID(const MSLane *, const double, const SumoXMLTag) const` [const]
    /// Returns the stop of the given category close to the given position
    // std::string
    // MSNet::getStoppingPlaceID(const MSLane* lane, const double pos, const SumoXMLTag category) const {
    //     if (myStoppingPlaces.count(category) > 0) {
    //         for (const auto& it : myStoppingPlaces.find(category)->second) {
    //             MSStoppingPlace* stop = it.second;
    //             if (&stop->getLane() == lane && stop->getBeginLanePosition() - POSITION_EPS <= pos && stop->getEndLanePosition() + POSITION_EPS >= pos) {
    //                 return stop->getID();
    //             }
    //         }
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_id(&self, lane: Ptr<MSLane>, pos: f64, category: SumoXMLTag) -> String {
        todo!("port MSNet::getStoppingPlaceID")
    }

    /// C++: `const std::vector<MSStoppingPlace *> & MSNet::getStoppingPlaceAlternatives(const std::string &, SumoXMLTag) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stopping_place_alternatives(&self, name: &str, category: SumoXMLTag) -> &Vec<Ptr<MSStoppingPlace>> {
        todo!("port MSNet::getStoppingPlaceAlternatives")
    }

    /// C++: `const NamedObjectCont<MSStoppingPlace *> & MSNet::getStoppingPlaces(SumoXMLTag) const` [const]
    /// @}
    // const NamedObjectCont<MSStoppingPlace*>&
    // MSNet::getStoppingPlaces(SumoXMLTag category) const {
    //     auto it = myStoppingPlaces.find(category);
    //     if (it != myStoppingPlaces.end()) {
    //         return it->second;
    //     } else {
    //         return myEmptyStoppingPlaceCont;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_places(&self, category: SumoXMLTag) -> &Unmapped /* NamedObjectCont<MSStoppingPlace *> */ {
        todo!("port MSNet::getStoppingPlaces")
    }

    /// C++: `void MSNet::writeChargingStationOutput() const` [const]
    /// write charging station output
    // void
    // MSNet::writeChargingStationOutput() const {
    //     if (myStoppingPlaces.count(SUMO_TAG_CHARGING_STATION) > 0) {
    //         OutputDevice& output = OutputDevice::getDeviceByOption("chargingstations-output");
    //         for (const auto& it : myStoppingPlaces.find(SUMO_TAG_CHARGING_STATION)->second) {
    //             static_cast<MSChargingStation*>(it.second)->writeChargingStationOutput(output);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_charging_station_output(&self) {
        todo!("port MSNet::writeChargingStationOutput")
    }

    /// C++: `void MSNet::writeRailSignalBlocks() const` [const]
    /// write rail signal block output
    // void
    // MSNet::writeRailSignalBlocks() const {
    //     if (OptionsCont::getOptions().isSet("railsignal-block-output")) {
    //         OutputDevice& output = OutputDevice::getDeviceByOption("railsignal-block-output");
    //         for (auto tls : myLogics->getAllLogics()) {
    //             MSRailSignal* rs = dynamic_cast<MSRailSignal*>(tls);
    //             if (rs != nullptr) {
    //                 rs->writeBlocks(output, false);
    //             }
    //         }
    //         MSDriveWay::writeDepatureBlocks(output, false);
    //     }
    //     if (OptionsCont::getOptions().isSet("railsignal-vehicle-output")) {
    //         OutputDevice& output = OutputDevice::getDeviceByOption("railsignal-vehicle-output");
    //         for (auto tls : myLogics->getAllLogics()) {
    //             MSRailSignal* rs = dynamic_cast<MSRailSignal*>(tls);
    //             if (rs != nullptr) {
    //                 rs->writeBlocks(output, true);
    //             }
    //         }
    //         MSDriveWay::writeDepatureBlocks(output, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_rail_signal_blocks(&self) {
        todo!("port MSNet::writeRailSignalBlocks")
    }

    /// C++: `void MSNet::createTLWrapper(MSTrafficLightLogic *)` [virtual]
    /// creates a wrapper for the given logic (see GUINet)
    // virtual void createTLWrapper(MSTrafficLightLogic*) {}
    #[allow(unused_variables)]
    pub fn create_tl_wrapper(&mut self, arg0: Ptr<MSTrafficLightLogic>) {
        todo!("port MSNet::createTLWrapper")
    }

    /// C++: `void MSNet::writeOverheadWireSegmentOutput() const` [const]
    /// write the output generated by an overhead wire segment
    // void
    // MSNet::writeOverheadWireSegmentOutput() const {
    //     if (myStoppingPlaces.count(SUMO_TAG_OVERHEAD_WIRE_SEGMENT) > 0) {
    //         OutputDevice& output = OutputDevice::getDeviceByOption("overheadwiresegments-output");
    //         for (const auto& it : myStoppingPlaces.find(SUMO_TAG_OVERHEAD_WIRE_SEGMENT)->second) {
    //             static_cast<MSOverheadWire*>(it.second)->writeOverheadWireSegmentOutput(output);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_overhead_wire_segment_output(&self) {
        todo!("port MSNet::writeOverheadWireSegmentOutput")
    }

    /// C++: `void MSNet::writeSubstationOutput() const` [const]
    /// write electrical substation output
    // void
    // MSNet::writeSubstationOutput() const {
    //     if (myTractionSubstations.size() > 0) {
    //         OutputDevice& output = OutputDevice::getDeviceByOption("substations-output");
    //         output.setPrecision(OptionsCont::getOptions().getInt("substations-output.precision"));
    //         for (auto& it : myTractionSubstations) {
    //             it->writeTractionSubstationOutput(output);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_substation_output(&self) {
        todo!("port MSNet::writeSubstationOutput")
    }

    /// C++: `bool MSNet::isSelected(const MSTrafficLightLogic *) const` [const, virtual]
    /// return wheter the given logic (or rather its wrapper) is selected in the GUI
    // virtual bool isSelected(const MSTrafficLightLogic*) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self, arg0: Ptr<MSTrafficLightLogic>) -> bool {
        todo!("port MSNet::isSelected")
    }

    /// C++: `void MSNet::updateGUI() const` [const, virtual]
    /// update view after simulation.loadState
    // virtual void updateGUI() const { }
    #[allow(unused_variables)]
    pub fn update_gui(&self) {
        todo!("port MSNet::updateGUI")
    }

    /// C++: `SUMOTime MSNet::loadState(const std::string &, const bool)`
    /// load state from file and return new time
    // SUMOTime
    // MSNet::loadState(const std::string& fileName, const bool catchExceptions) {
    //     // load time only
    //     const SUMOTime newTime = MSStateHandler::MSStateTimeHandler::getTime(fileName);
    //     // clean up state
    //     clearState(newTime);
    //     // load state
    //     MSStateHandler h(fileName, 0);
    //     XMLSubSys::runParser(h, fileName, false, false, false, catchExceptions);
    //     if (MsgHandler::getErrorInstance()->wasInformed()) {
    //         throw ProcessError(TLF("Loading state from '%' failed.", fileName));
    //     }
    //     // reset route loaders
    //     delete myRouteLoaders;
    //     myRouteLoaders = NLBuilder::buildRouteLoaderControl(OptionsCont::getOptions());
    //     // prevent loading errors on rewound route file
    //     MSGlobals::gStateLoaded = true;
    //
    //     updateGUI();
    //     return newTime;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, fileName: &str, catchExceptions: bool) -> SUMOTime {
        todo!("port MSNet::loadState")
    }

    /// C++: `void MSNet::quickReload()`
    /// reset state to the beginning without reloading the network
    // void
    // MSNet::quickReload() {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     clearState(string2time(oc.getString("begin")), true);
    //     NLBuilder::initRandomness();
    //     // load traffic from additional files
    //     if (!oc.getBool("ignore.additionalelements")) {
    //         for (std::string file : oc.getStringVector("additional-files")) {
    //             // ignore failure on parsing calibrator flow
    //             MSRouteHandler rh(file, true);
    //             const long before = PROGRESS_BEGIN_TIME_MESSAGE("Loading traffic from '" + file + "'");
    //             if (!XMLSubSys::runParser(rh, file, false)) {
    //                 throw ProcessError(TLF("Loading of % failed.", file));
    //             }
    //             PROGRESS_TIME_MESSAGE(before);
    //         }
    //     }
    //     delete myRouteLoaders;
    //     myRouteLoaders = NLBuilder::buildRouteLoaderControl(OptionsCont::getOptions());
    //     updateGUI();
    // }
    #[allow(unused_variables)]
    pub fn quick_reload(&mut self) {
        todo!("port MSNet::quickReload")
    }

    /// C++: `void MSNet::addVehicleStateListener(VehicleStateListener *)`
    /// Adds a vehicle states listener
    // void
    // MSNet::addVehicleStateListener(VehicleStateListener* listener) {
    //     if (find(myVehicleStateListeners.begin(), myVehicleStateListeners.end(), listener) == myVehicleStateListeners.end()) {
    //         myVehicleStateListeners.push_back(listener);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_state_listener(&mut self, listener: Ptr<MSNet_VehicleStateListener>) {
        todo!("port MSNet::addVehicleStateListener")
    }

    /// C++: `void MSNet::removeVehicleStateListener(VehicleStateListener *)`
    /// Removes a vehicle states listener
    // void
    // MSNet::removeVehicleStateListener(VehicleStateListener* listener) {
    //     std::vector<VehicleStateListener*>::iterator i = std::find(myVehicleStateListeners.begin(), myVehicleStateListeners.end(), listener);
    //     if (i != myVehicleStateListeners.end()) {
    //         myVehicleStateListeners.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_vehicle_state_listener(&mut self, listener: Ptr<MSNet_VehicleStateListener>) {
        todo!("port MSNet::removeVehicleStateListener")
    }

    /// C++: `void MSNet::informVehicleStateListener(const SUMOVehicle *const, VehicleState, const std::string &)`
    /// Informs all added listeners about a vehicle's state change
    // void
    // MSNet::informVehicleStateListener(const SUMOVehicle* const vehicle, VehicleState to, const std::string& info) {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myVehicleStateListenerMutex, MSGlobals::gNumThreads > 1);
    // #endif
    //     for (VehicleStateListener* const listener : myVehicleStateListeners) {
    //         listener->vehicleStateChanged(vehicle, to, info);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_vehicle_state_listener(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        todo!("port MSNet::informVehicleStateListener")
    }

    /// C++: `void MSNet::addTransportableStateListener(TransportableStateListener *)`
    /// Adds a transportable states listener
    // void
    // MSNet::addTransportableStateListener(TransportableStateListener* listener) {
    //     if (find(myTransportableStateListeners.begin(), myTransportableStateListeners.end(), listener) == myTransportableStateListeners.end()) {
    //         myTransportableStateListeners.push_back(listener);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_transportable_state_listener(&mut self, listener: Ptr<MSNet_TransportableStateListener>) {
        todo!("port MSNet::addTransportableStateListener")
    }

    /// C++: `void MSNet::removeTransportableStateListener(TransportableStateListener *)`
    /// Removes a transportable states listener
    // void
    // MSNet::removeTransportableStateListener(TransportableStateListener* listener) {
    //     std::vector<TransportableStateListener*>::iterator i = std::find(myTransportableStateListeners.begin(), myTransportableStateListeners.end(), listener);
    //     if (i != myTransportableStateListeners.end()) {
    //         myTransportableStateListeners.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable_state_listener(&mut self, listener: Ptr<MSNet_TransportableStateListener>) {
        todo!("port MSNet::removeTransportableStateListener")
    }

    /// C++: `void MSNet::informTransportableStateListener(const MSTransportable *const, TransportableState, const std::string &)`
    /// Informs all added listeners about a transportable's state change
    // void
    // MSNet::informTransportableStateListener(const MSTransportable* const transportable, TransportableState to, const std::string& info) {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myTransportableStateListenerMutex, MSGlobals::gNumThreads > 1);
    // #endif
    //     for (TransportableStateListener* const listener : myTransportableStateListeners) {
    //         listener->transportableStateChanged(transportable, to, info);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_transportable_state_listener(&mut self, transportable: Ptr<MSTransportable>, to: MSNet_TransportableState, info: &str) {
        todo!("port MSNet::informTransportableStateListener")
    }

    /// C++: `bool MSNet::registerCollision(const SUMOTrafficObject *, const SUMOTrafficObject *, const std::string &, const MSLane *, double)`
    /// register collision and return whether it was the first one involving these vehicles
    // bool
    // MSNet::registerCollision(const SUMOTrafficObject* collider, const SUMOTrafficObject* victim, const std::string& collisionType, const MSLane* lane, double pos) {
    //     auto it = myCollisions.find(collider->getID());
    //     if (it != myCollisions.end()) {
    //         for (Collision& old : it->second) {
    //             if (old.victim == victim->getID()) {
    //                 // collision from previous step continues
    //                 old.continuationTime = myStep;
    //                 return false;
    //             }
    //         }
    //     } else {
    //         // maybe the roles have been reversed
    //         auto it2 = myCollisions.find(victim->getID());
    //         if (it2 != myCollisions.end()) {
    //             for (Collision& old : it2->second) {
    //                 if (old.victim == collider->getID()) {
    //                     // collision from previous step continues (keep the old roles)
    //                     old.continuationTime = myStep;
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     Collision c;
    //     c.victim = victim->getID();
    //     c.colliderType = collider->getVehicleType().getID();
    //     c.victimType = victim->getVehicleType().getID();
    //     c.colliderSpeed = collider->getSpeed();
    //     c.victimSpeed = victim->getSpeed();
    //     c.colliderFront = collider->getPosition();
    //     c.victimFront = victim->getPosition();
    //     c.colliderBack = collider->getPosition(-collider->getVehicleType().getLength());
    //     c.victimBack = victim->getPosition(-victim->getVehicleType().getLength());
    //     c.type = collisionType;
    //     c.lane = lane;
    //     c.pos = pos;
    //     c.time = myStep;
    //     c.continuationTime = myStep;
    //     myCollisions[collider->getID()].push_back(c);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn register_collision(&mut self, collider: Ptr<SUMOTrafficObject>, victim: Ptr<SUMOTrafficObject>, collisionType: &str, lane: Ptr<MSLane>, pos: f64) -> bool {
        todo!("port MSNet::registerCollision")
    }

    /// C++: `const CollisionMap & MSNet::getCollisions() const` [const]
    // const CollisionMap& getCollisions() const {
    //         return myCollisions;
    //     }
    #[allow(unused_variables)]
    pub fn get_collisions(&self) -> &MSNet_CollisionMap {
        todo!("port MSNet::getCollisions")
    }

    /// C++: `double MSNet::getTravelTime(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    /// Returns the travel time to pass an edge
    // double
    // MSNet::getTravelTime(const MSEdge* const e, const SUMOVehicle* const v, double t) {
    //     double value;
    //     const MSVehicle* const veh = dynamic_cast<const MSVehicle* const>(v);
    //     if (veh != nullptr && veh->getWeightsStorage().retrieveExistingTravelTime(e, t, value)) {
    //         return value;
    //     }
    //     if (getInstance()->getWeightsStorage().retrieveExistingTravelTime(e, t, value)) {
    //         return value;
    //     }
    //     if (veh != nullptr) {
    //         if ((veh->getRoutingMode() & libsumo::ROUTING_MODE_AGGREGATED_CUSTOM) != 0) {
    //             return MSRoutingEngine::getEffortExtra(e, v, t);
    //         } else if ((veh->getRoutingMode() & libsumo::ROUTING_MODE_AGGREGATED) != 0) {
    //             if (MSRoutingEngine::hasBikeSpeeds() && v->getVClass() == SVC_BICYCLE) {
    //                 return MSRoutingEngine::getEffortBike(e, v, t);
    //             } else {
    //                 return MSRoutingEngine::getEffort(e, v, t);
    //             }
    //         } else if (MSRoutingEngine::haveExtras()) {
    //             double tt = e->getMinimumTravelTime(v);
    //             MSRoutingEngine::applyExtras(e, v, SIMSTEP, tt);
    //             return tt;
    //         }
    //     }
    //     return e->getMinimumTravelTime(v);
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
        todo!("port MSNet::getTravelTime")
    }

    /// C++: `double MSNet::getEffort(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    /// Returns the effort to pass an edge
    // double
    // MSNet::getEffort(const MSEdge* const e, const SUMOVehicle* const v, double t) {
    //     double value;
    //     const MSVehicle* const veh = dynamic_cast<const MSVehicle* const>(v);
    //     if (veh != nullptr && veh->getWeightsStorage().retrieveExistingEffort(e, t, value)) {
    //         return value;
    //     }
    //     if (getInstance()->getWeightsStorage().retrieveExistingEffort(e, t, value)) {
    //         return value;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_effort(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
        todo!("port MSNet::getEffort")
    }

    /// C++: `MSVehicleRouter & MSNet::getRouterTT(int, const Prohibitions &) const` [const]
    // MSVehicleRouter&
    // MSNet::getRouterTT(int rngIndex, const Prohibitions& prohibited) const {
    //     if (MSGlobals::gNumSimThreads == 1) {
    //         rngIndex = 0;
    //     }
    //     if (myRouterTT.count(rngIndex) == 0) {
    //         const std::string routingAlgorithm = OptionsCont::getOptions().getString("routing-algorithm");
    //         if (routingAlgorithm == "dijkstra") {
    //             myRouterTT[rngIndex] = new DijkstraRouter<MSEdge, SUMOVehicle>(MSEdge::getAllEdges(), true, &MSNet::getTravelTime, nullptr, false, nullptr, true);
    //         } else if (routingAlgorithm == "CCH") {
    //             // free-flow CCH metrics per vehicle type; every query a shared
    //             // metric cannot serve (TraCI edge weights, non-default routing
    //             // modes, prohibitions beyond live closures) routes on the exact
    //             // embedded fallback -- see MSRoutingEngine::getFreeflowCCHMetric
    //             SUMOAbstractRouter<MSEdge, SUMOVehicle>* fallback =
    //                 new AStarRouter<MSEdge, SUMOVehicle, MSMapMatcher>(MSEdge::getAllEdges(), true, &MSNet::getTravelTime, nullptr, true);
    //             myRouterTT[rngIndex] = new CCHRouter<MSEdge, SUMOVehicle, MSCCHGraph>(
    //                 MSRoutingEngine::ensureCCHGraph(), &MSRoutingEngine::getFreeflowCCHMetric,
    //                 &MSNet::getTravelTime, true, fallback);
    //         } else {
    //             if (routingAlgorithm != "astar") {
    //                 WRITE_WARNINGF(TL("TraCI and Triggers cannot use routing algorithm '%'. using 'astar' instead."), routingAlgorithm);
    //             }
    //             myRouterTT[rngIndex] = new AStarRouter<MSEdge, SUMOVehicle, MSMapMatcher>(MSEdge::getAllEdges(), true, &MSNet::getTravelTime, nullptr, true);
    //         }
    //     }
    //     myRouterTT[rngIndex]->prohibit(prohibited);
    //     return *myRouterTT[rngIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_router_tt(&self, rngIndex: i32, prohibited: &SUMOAbstractRouter_Prohibitions) -> &mut MSVehicleRouter {
        todo!("port MSNet::getRouterTT")
    }

    /// C++: `MSVehicleRouter & MSNet::getRouterEffort(int, const Prohibitions &) const` [const]
    // MSVehicleRouter&
    // MSNet::getRouterEffort(int rngIndex, const Prohibitions& prohibited) const {
    //     if (MSGlobals::gNumSimThreads == 1) {
    //         rngIndex = 0;
    //     }
    //     if (myRouterEffort.count(rngIndex) == 0) {
    //         myRouterEffort[rngIndex] = new DijkstraRouter<MSEdge, SUMOVehicle>(MSEdge::getAllEdges(), true, &MSNet::getEffort, &MSNet::getTravelTime, false, nullptr, true);
    //     }
    //     myRouterEffort[rngIndex]->prohibit(prohibited);
    //     return *myRouterEffort[rngIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_router_effort(&self, rngIndex: i32, prohibited: &SUMOAbstractRouter_Prohibitions) -> &mut MSVehicleRouter {
        todo!("port MSNet::getRouterEffort")
    }

    /// C++: `MSPedestrianRouter & MSNet::getPedestrianRouter(int, const Prohibitions &) const` [const]
    // MSPedestrianRouter&
    // MSNet::getPedestrianRouter(int rngIndex, const Prohibitions& prohibited) const {
    //     if (MSGlobals::gNumSimThreads == 1) {
    //         rngIndex = 0;
    //     }
    //     if (myPedestrianRouter.count(rngIndex) == 0) {
    //         myPedestrianRouter[rngIndex] = new MSPedestrianRouter();
    //     }
    //     myPedestrianRouter[rngIndex]->prohibit(prohibited);
    //     return *myPedestrianRouter[rngIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_pedestrian_router(&self, rngIndex: i32, prohibited: &SUMOAbstractRouter_Prohibitions) -> &mut MSPedestrianRouter {
        todo!("port MSNet::getPedestrianRouter")
    }

    /// C++: `MSTransportableRouter & MSNet::getIntermodalRouter(int, const int, const Prohibitions &) const` [const]
    // MSTransportableRouter&
    // MSNet::getIntermodalRouter(int rngIndex, const int routingMode, const Prohibitions& prohibited) const {
    //     if (MSGlobals::gNumSimThreads == 1) {
    //         rngIndex = 0;
    //     }
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const int key = rngIndex * oc.getInt("thread-rngs") + routingMode;
    //     if (myIntermodalRouter.count(key) == 0) {
    //         const int carWalk = SUMOVehicleParserHelper::parseCarWalkTransfer(oc, MSDevice_Taxi::hasFleet() || myInserter->hasTaxiFlow());
    //         const std::string routingAlgorithm = OptionsCont::getOptions().getString("routing-algorithm");
    //         const double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //         if (routingMode == libsumo::ROUTING_MODE_COMBINED) {
    //             myIntermodalRouter[key] = new MSTransportableRouter(MSNet::adaptIntermodalRouter, carWalk, taxiWait, routingAlgorithm, routingMode, new FareModul());
    //         } else {
    //             myIntermodalRouter[key] = new MSTransportableRouter(MSNet::adaptIntermodalRouter, carWalk, taxiWait, routingAlgorithm, routingMode);
    //         }
    //     }
    //     myIntermodalRouter[key]->prohibit(prohibited);
    //     return *myIntermodalRouter[key];
    // }
    #[allow(unused_variables)]
    pub fn get_intermodal_router(&self, rngIndex: i32, routingMode: i32, prohibited: &SUMOAbstractRouter_Prohibitions) -> &mut MSTransportableRouter {
        todo!("port MSNet::getIntermodalRouter")
    }

    /// C++: `void MSNet::resetIntermodalRouter() const` [const]
    /// force reconstruction of intermodal network
    // void
    // MSNet::resetIntermodalRouter() const {
    //     for (auto& router : myIntermodalRouter) {
    //         delete router.second;
    //     }
    //     myIntermodalRouter.clear();
    // }
    #[allow(unused_variables)]
    pub fn reset_intermodal_router(&self) {
        todo!("port MSNet::resetIntermodalRouter")
    }

    /// C++: `void MSNet::adaptIntermodalRouter(MSTransportableRouter &)` [static]
    // void
    // MSNet::adaptIntermodalRouter(MSTransportableRouter& router) {
    //     double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //     // add access to all parking areas
    //     EffortCalculator* const external = router.getExternalEffort();
    //     for (const auto& stopType : myInstance->myStoppingPlaces) {
    //         // add access to all stopping places
    //         const SumoXMLTag element = stopType.first;
    //         for (const auto& i : stopType.second) {
    //             const MSEdge* const edge = &i.second->getLane().getEdge();
    //             router.getNetwork()->addAccess(i.first, edge, i.second->getBeginLanePosition(), i.second->getEndLanePosition(),
    //                                            0., element, false, taxiWait);
    //             if (element == SUMO_TAG_BUS_STOP) {
    //                 // add access to all public transport stops
    //                 for (const auto& a : i.second->getAllAccessPos()) {
    //                     router.getNetwork()->addAccess(i.first, &a.lane->getEdge(), a.startPos, a.endPos, a.length, element, true, taxiWait);
    //                 }
    //                 if (external != nullptr) {
    //                     external->addStop(router.getNetwork()->getStopEdge(i.first)->getNumericalID(), *i.second);
    //                 }
    //             }
    //         }
    //     }
    //     myInstance->getInsertionControl().adaptIntermodalRouter(router);
    //     myInstance->getVehicleControl().adaptIntermodalRouter(router);
    //     // add access to transfer from walking to taxi-use
    //     if ((router.getCarWalkTransfer() & ModeChangeOptions::TAXI_PICKUP_ANYWHERE) != 0) {
    //         for (MSEdge* edge : myInstance->getEdgeControl().getEdges()) {
    //             if ((edge->getPermissions() & SVC_PEDESTRIAN) != 0 && (edge->getPermissions() & SVC_TAXI) != 0) {
    //                 router.getNetwork()->addCarAccess(edge, SVC_TAXI, taxiWait);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_intermodal_router(router: &mut MSTransportableRouter) {
        todo!("port MSNet::adaptIntermodalRouter")
    }

    /// C++: `bool MSNet::hasInternalLinks() const` [const]
    /// return whether the network contains internal links
    // bool hasInternalLinks() const {
    //         return myHasInternalLinks;
    //     }
    #[allow(unused_variables)]
    pub fn has_internal_links(&self) -> bool {
        todo!("port MSNet::hasInternalLinks")
    }

    /// C++: `bool MSNet::hasJunctionHigherSpeeds() const` [const]
    /// return whether the network was built with higher junction speeds
    // bool hasJunctionHigherSpeeds() const {
    //         return myJunctionHigherSpeeds;
    //     }
    #[allow(unused_variables)]
    pub fn has_junction_higher_speeds(&self) -> bool {
        todo!("port MSNet::hasJunctionHigherSpeeds")
    }

    /// C++: `bool MSNet::hasElevation() const` [const]
    /// return whether the network contains elevation data
    // bool hasElevation() const {
    //         return myHasElevation;
    //     }
    #[allow(unused_variables)]
    pub fn has_elevation(&self) -> bool {
        todo!("port MSNet::hasElevation")
    }

    /// C++: `bool MSNet::hasPedestrianNetwork() const` [const]
    /// return whether the network contains walkingareas and crossings
    // bool hasPedestrianNetwork() const {
    //         return myHasPedestrianNetwork;
    //
    //     }
    #[allow(unused_variables)]
    pub fn has_pedestrian_network(&self) -> bool {
        todo!("port MSNet::hasPedestrianNetwork")
    }

    /// C++: `bool MSNet::hasBidiEdges() const` [const]
    /// return whether the network contains bidirectional rail edges
    // bool hasBidiEdges() const {
    //         return myHasBidiEdges;
    //     }
    #[allow(unused_variables)]
    pub fn has_bidi_edges(&self) -> bool {
        todo!("port MSNet::hasBidiEdges")
    }

    /// C++: `MMVersion MSNet::getNetworkVersion() const` [const]
    /// return the network version
    // MMVersion getNetworkVersion() const {
    //         return myVersion;
    //     }
    #[allow(unused_variables)]
    pub fn get_network_version(&self) -> Unmapped /* MMVersion */ {
        todo!("port MSNet::getNetworkVersion")
    }

    /// C++: `bool MSNet::warnOnce(const std::string &)`
    /// return whether a warning regarding the given object shall be issued
    // bool
    // MSNet::warnOnce(const std::string& typeAndID) {
    //     if (myWarnedOnce.find(typeAndID) == myWarnedOnce.end()) {
    //         myWarnedOnce[typeAndID] = true;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn warn_once(&mut self, typeAndID: &str) -> bool {
        todo!("port MSNet::warnOnce")
    }

    /// C++: `void MSNet::interrupt()`
    // void interrupt() {
    //         myAmInterrupted = true;
    //     }
    #[allow(unused_variables)]
    pub fn interrupt(&mut self) {
        todo!("port MSNet::interrupt")
    }

    /// C++: `bool MSNet::isInterrupted() const` [const]
    // bool isInterrupted() const {
    //         return myAmInterrupted;
    //     }
    #[allow(unused_variables)]
    pub fn is_interrupted(&self) -> bool {
        todo!("port MSNet::isInterrupted")
    }

    /// C++: `bool MSNet::skipFinalReset() const` [const, virtual]
    /// gui may prevent final meanData reset to keep live data visible
    // virtual bool skipFinalReset() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn skip_final_reset(&self) -> bool {
        todo!("port MSNet::skipFinalReset")
    }

    /// C++: `void MSNet::lockPendingEmits() const` [const, virtual]
    /// grant exclusive access to pending emits
    // virtual void lockPendingEmits() const {}
    #[allow(unused_variables)]
    pub fn lock_pending_emits(&self) {
        todo!("port MSNet::lockPendingEmits")
    }

    /// C++: `void MSNet::unlockPendingEmits() const` [const, virtual]
    /// release exclusive access to pending emits
    // virtual void unlockPendingEmits() const {}
    #[allow(unused_variables)]
    pub fn unlock_pending_emits(&self) {
        todo!("port MSNet::unlockPendingEmits")
    }

    /// C++: `MSMapMatcher * MSNet::getMapMatcher() const` [const]
    // MSMapMatcher*
    // MSNet::getMapMatcher() const {
    //     auto loader = myRouteLoaders->getFirstLoader();
    //     if (loader != nullptr) {
    //         return dynamic_cast<MSMapMatcher*>(loader->getRouteHandler());
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_map_matcher(&self) -> Ptr<MSMapMatcher> {
        todo!("port MSNet::getMapMatcher")
    }

    /// C++: `MSTractionSubstation * MSNet::findTractionSubstation(const std::string &)`
    /// find electrical substation by its id
    // MSTractionSubstation*
    // MSNet::findTractionSubstation(const std::string& substationId) {
    //     for (std::vector<MSTractionSubstation*>::iterator it = myTractionSubstations.begin(); it != myTractionSubstations.end(); ++it) {
    //         if ((*it)->getID() == substationId) {
    //             return *it;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_traction_substation(&mut self, substationId: &str) -> Ptr<MSTractionSubstation> {
        todo!("port MSNet::findTractionSubstation")
    }

    /// C++: `bool MSNet::checkElevation()` [protected]
    /// check all lanes for elevation data
    // bool
    // MSNet::checkElevation() {
    //     const MSEdgeVector& edges = myEdges->getEdges();
    //     for (MSEdgeVector::const_iterator e = edges.begin(); e != edges.end(); ++e) {
    //         for (std::vector<MSLane*>::const_iterator i = (*e)->getLanes().begin(); i != (*e)->getLanes().end(); ++i) {
    //             if ((*i)->getShape().hasElevation()) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_elevation(&mut self) -> bool {
        todo!("port MSNet::checkElevation")
    }

    /// C++: `bool MSNet::checkWalkingarea()` [protected]
    /// check all lanes for type walkingArea
    // bool
    // MSNet::checkWalkingarea() {
    //     for (const MSEdge* e : myEdges->getEdges()) {
    //         if (e->getFunction() == SumoXMLEdgeFunc::WALKINGAREA) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_walkingarea(&mut self) -> bool {
        todo!("port MSNet::checkWalkingarea")
    }

    /// C++: `bool MSNet::checkBidiEdges()` [protected]
    /// check whether bidirectional edges occur in the network
    // bool
    // MSNet::checkBidiEdges() {
    //     for (const MSEdge* e : myEdges->getEdges()) {
    //         if (e->getBidiEdge() != nullptr) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_bidi_edges(&mut self) -> bool {
        todo!("port MSNet::checkBidiEdges")
    }

    /// C++: `void MSNet::removeOutdatedCollisions()` [protected]
    /// remove collisions from the previous simulation step
    // void
    // MSNet::removeOutdatedCollisions() {
    //     for (auto it = myCollisions.begin(); it != myCollisions.end();) {
    //         for (auto it2 = it->second.begin(); it2 != it->second.end();) {
    //             if (it2->continuationTime != myStep) {
    //                 it2 = it->second.erase(it2);
    //             } else {
    //                 it2++;
    //             }
    //         }
    //         if (it->second.size() == 0) {
    //             it = myCollisions.erase(it);
    //         } else {
    //             it++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_outdated_collisions(&mut self) {
        todo!("port MSNet::removeOutdatedCollisions")
    }

    /// C++: `void MSNet::postMoveStep()` [protected]
    /// Performs the parts of the simulation step which happen after the move
    // void
    // MSNet::postMoveStep() {
    //     const int numControlled = libsumo::Helper::postProcessRemoteControl();
    //     if (numControlled > 0 && MSGlobals::gCheck4Accidents) {
    //         myEdges->detectCollisions(myStep, STAGE_REMOTECONTROL);
    //     }
    //     if (myLogExecutionTime) {
    //         myTraCIStepDuration += SysUtils::getCurrentMillis();
    //         myTraCIMillis += myTraCIStepDuration;
    //     }
    //     if (MSGlobals::gCheck4Accidents && !MSGlobals::gUseMesoSim) {
    //         // collisions from the previous step were kept to avoid duplicate
    //         // warnings. we must remove them now to ensure correct output.
    //         removeOutdatedCollisions();
    //     }
    //     // update and write (if needed) detector values
    //     mySimStepDuration = SysUtils::getCurrentMillis() - mySimStepDuration;
    //     writeOutput();
    //
    //     if (myLogExecutionTime) {
    //         myVehiclesMoved += myVehicleControl->getRunningVehicleNo();
    //         if (myPersonControl != nullptr) {
    //             myPersonsMoved += myPersonControl->getRunningNumber();
    //         }
    //     }
    //     myStep += DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn post_move_step(&mut self) {
        todo!("port MSNet::postMoveStep")
    }

    /// C++: `void MSNet::MSNet(const MSNet &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSNet) -> Self {
        todo!("port MSNet::MSNet")
    }

    /// C++: `MSNet & MSNet::operator=(const MSNet &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSNet) -> &mut MSNet {
        todo!("port MSNet::operator=")
    }

}

/// Interface for objects listening to transportable state changes
/// C++ class `MSNet::TransportableStateListener` (microsim/MSNet.h)
pub struct MSNet_TransportableStateListener {
}

impl MSNet_TransportableStateListener {
    /// C++: `void TransportableStateListener::TransportableStateListener()`
    /// Constructor
    // TransportableStateListener() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port TransportableStateListener::TransportableStateListener")
    }

    /// C++: `void TransportableStateListener::~TransportableStateListener() noexcept` [virtual]
    /// Destructor
    // virtual ~TransportableStateListener() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port TransportableStateListener::~TransportableStateListener")
    }

    /// C++: `void TransportableStateListener::transportableStateChanged(const MSTransportable *const, TransportableState, const std::string &)` [virtual, pure virtual]
    /// Called if a transportable changes its state
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn transportable_state_changed(&mut self, transportable: Ptr<MSTransportable>, to: MSNet_TransportableState, info: &str) {
        todo!("port TransportableStateListener::transportableStateChanged")
    }

}

/// Interface for objects listening to vehicle state changes
/// C++ class `MSNet::VehicleStateListener` (microsim/MSNet.h)
pub struct MSNet_VehicleStateListener {
}

impl MSNet_VehicleStateListener {
    /// C++: `void VehicleStateListener::VehicleStateListener()`
    /// Constructor
    // VehicleStateListener() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port VehicleStateListener::VehicleStateListener")
    }

    /// C++: `void VehicleStateListener::~VehicleStateListener() noexcept` [virtual]
    /// Destructor
    // virtual ~VehicleStateListener() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port VehicleStateListener::~VehicleStateListener")
    }

    /// C++: `void VehicleStateListener::vehicleStateChanged(const SUMOVehicle *const, VehicleState, const std::string &)` [virtual, pure virtual]
    /// Called if a vehicle changes its state
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        todo!("port VehicleStateListener::vehicleStateChanged")
    }

}

