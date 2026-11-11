//! Port of netload/NLJunctionControlBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<MSLane *> NLJunctionControlBuilder::LaneVector`
pub type NLJunctionControlBuilder_LaneVector = Vec<Ptr<MSLane>>;

/// C++ typedef `Parameterised::Map NLJunctionControlBuilder::StringParameterMap`
pub type NLJunctionControlBuilder_StringParameterMap = Unmapped /* Parameterised::Map */;

/// Builder of microsim-junctions and tls
/// C++ class `NLJunctionControlBuilder` (netload/NLJunctionControlBuilder.h)
pub struct NLJunctionControlBuilder {
    /// The net to use
    pub myNet: Ptr<MSNet>,
    /// The detector builder to use
    pub myDetectorBuilder: Ptr<NLDetectorBuilder>,
    /// The switch offset within the tls
    pub myOffset: SUMOTime,
    /// The current logic type
    pub myLogicType: TrafficLightType,
    /// The right-of-way-logic of the currently chosen bitset-logic
    pub myActiveLogic: MSBitSetLogic_Logic,
    /// The description about which lanes disallow other passing the junction simultaneously
    pub myActiveFoes: MSBitSetLogic_Foes,
    /// The description about which lanes have an internal follower
    pub myActiveConts: Unmapped /* std::bitset<256> */,
    /// The current phase definitions for a simple traffic light
    pub myActivePhases: MSTrafficLightLogic_Phases,
    /// The current switching conditions for an actuated traffic light
    pub myActiveConditions: MSActuatedTrafficLightLogic_ConditionMap,
    /// The current assignments for an actuated traffic light
    pub myActiveAssignments: MSActuatedTrafficLightLogic_AssignmentMap,
    /// The current functions for an actuated traffic light
    pub myActiveFunctions: MSActuatedTrafficLightLogic_FunctionMap,
    /// The current function for an actuated traffic light
    pub myActiveFunction: MSActuatedTrafficLightLogic_Function,
    /// The size of the request
    pub myRequestSize: i32,
    /// Counter for the inserted items
    pub myRequestItemNumber: i32,
    /// The junctions controls
    pub myJunctions: Ptr<MSJunctionControl>,
    /// The list of the incoming lanes of the currently chosen junction
    pub myActiveIncomingLanes: LaneVector,
    /// The list of the internal lanes of the currently chosen junction
    pub myActiveInternalLanes: LaneVector,
    /// The id of the currently chosen junction
    pub myActiveID: String,
    /// The key of the currently chosen junction
    pub myActiveKey: String,
    /// The key of the currently chosen junction
    pub myActiveProgram: String,
    /// The type of the currently chosen junction
    pub myType: SumoXMLNodeType,
    /// The position of the junction
    pub myPosition: Position,
    /// The absolute duration of a tls-control loop
    pub myAbsDuration: SUMOTime,
    /// The shape of the current junction
    pub myShape: PositionVector,
    /// the name of the current junction
    pub myActiveName: String,
    /// The container for information which junctions shall be initialised using which values
    pub myNetworkLogics: Vec<Ptr<MSTrafficLightLogic>>,
    pub myAdditionalLogics: Vec<Ptr<MSTrafficLightLogic>>,
    pub myRailSignals: Vec<Ptr<MSTrafficLightLogic>>,
    pub myLogicParams: BTreeMap<Ptr<MSTrafficLightLogic>, NLJunctionControlBuilder_StringParameterMap>,
    /// The tls control to use (0 if net's tls control shall be used)
    pub myLogicControl: Ptr<MSTLLogicControl>,
    /// Parameter map (key->value)
    pub myAdditionalParameter: NLJunctionControlBuilder_StringParameterMap,
    /// Information whether the current logic had an error
    pub myCurrentHasError: bool,
    /// whether the network has been loaded
    pub myNetIsLoaded: bool,
}

// C++ static members of NLJunctionControlBuilder (port as module-level statics, see AGENTS.md):
//   static const int NLJunctionControlBuilder::NO_REQUEST_SIZE;

impl NLJunctionControlBuilder {
    /// C++: `void NLJunctionControlBuilder::NLJunctionControlBuilder(MSNet &, NLDetectorBuilder &)`
    /// Constructor
    // NLJunctionControlBuilder::NLJunctionControlBuilder(MSNet& net, NLDetectorBuilder& db) :
    //     myNet(net),
    //     myDetectorBuilder(db),
    //     myOffset(0),
    //     myJunctions(new MSJunctionControl()),
    //     myNetIsLoaded(false) {
    //     myLogicControl = new MSTLLogicControl();
    // }
    #[allow(unused_variables)]
    pub fn new(net: &mut MSNet, db: &mut NLDetectorBuilder) -> Self {
        todo!("port NLJunctionControlBuilder::NLJunctionControlBuilder")
    }

    /// C++: `void NLJunctionControlBuilder::~NLJunctionControlBuilder() noexcept` [virtual]
    /// Destructor
    // NLJunctionControlBuilder::~NLJunctionControlBuilder() {
    //     delete myLogicControl;
    //     delete myJunctions;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NLJunctionControlBuilder::~NLJunctionControlBuilder")
    }

    /// C++: `void NLJunctionControlBuilder::openJunction(const std::string &, const std::string &, const SumoXMLNodeType, const Position, const PositionVector &, const std::vector<MSLane *> &, const std::vector<MSLane *> &, const std::string &)`
    /// Begins the processing of the named junction
    // void
    // NLJunctionControlBuilder::openJunction(const std::string& id,
    //                                        const std::string& key,
    //                                        const SumoXMLNodeType type,
    //                                        const Position pos,
    //                                        const PositionVector& shape,
    //                                        const std::vector<MSLane*>& incomingLanes,
    //                                        const std::vector<MSLane*>& internalLanes,
    //                                        const std::string& name) {
    //     myActiveInternalLanes = internalLanes;
    //     myActiveIncomingLanes = incomingLanes;
    //     myActiveID = id;
    //     myActiveKey = key;
    //     myType = type;
    //     myPosition.set(pos);
    //     myShape = shape;
    //     myActiveName = name;
    //     myAdditionalParameter.clear();
    // }
    #[allow(unused_variables)]
    pub fn open_junction(&mut self, id: &str, key: &str, type_: SumoXMLNodeType, pos: Position, shape: &PositionVector, incomingLanes: &Vec<Ptr<MSLane>>, internalLanes: &Vec<Ptr<MSLane>>, name: &str) {
        todo!("port NLJunctionControlBuilder::openJunction")
    }

    /// C++: `void NLJunctionControlBuilder::closeJunction(const std::string &)`
    /// Closes (ends) the processing of the current junction
    // void
    // NLJunctionControlBuilder::closeJunction(const std::string& basePath) {
    //     if (myCurrentHasError) {
    //         // had an error before...
    //         return;
    //     }
    //     if (myRequestSize != NO_REQUEST_SIZE && myRequestItemNumber != myRequestSize) {
    //         throw InvalidArgument("The description for the junction logic '" + myActiveKey + "' is malicious.");
    //     }
    //     if (myJunctions == nullptr) {
    //         throw ProcessError(TL("Information about the number of nodes was missing."));
    //     }
    //     MSJunction* junction = nullptr;
    //     switch (myType) {
    //         case SumoXMLNodeType::NOJUNCTION:
    //         case SumoXMLNodeType::DEAD_END:
    //         case SumoXMLNodeType::DEAD_END_DEPRECATED:
    //         case SumoXMLNodeType::DISTRICT:
    //         case SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION:
    //             if (!myActiveLogic.empty()) {
    //                 WRITE_WARNINGF(TL("Ignoring junction logic for junction '%'."), myActiveID)
    //             }
    //             junction = buildNoLogicJunction();
    //             break;
    //         case SumoXMLNodeType::TRAFFIC_LIGHT:
    //         case SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED:
    //         case SumoXMLNodeType::RIGHT_BEFORE_LEFT:
    //         case SumoXMLNodeType::LEFT_BEFORE_RIGHT:
    //         case SumoXMLNodeType::PRIORITY:
    //         case SumoXMLNodeType::PRIORITY_STOP:
    //         case SumoXMLNodeType::ALLWAY_STOP:
    //         case SumoXMLNodeType::ZIPPER:
    //             junction = buildLogicJunction(new MSBitsetLogic(myRequestSize, myActiveLogic, myActiveFoes, myActiveConts));
    //             break;
    //         case SumoXMLNodeType::INTERNAL:
    //             if (MSGlobals::gUsingInternalLanes) {
    //                 if (!myActiveLogic.empty()) {
    //                     WRITE_WARNINGF(TL("Ignoring junction logic for junction '%'."), myActiveID)
    //                 }
    //                 junction = buildInternalJunction();
    //             }
    //             break;
    //         case SumoXMLNodeType::RAIL_SIGNAL:
    //         case SumoXMLNodeType::RAIL_CROSSING:
    //             myOffset = 0;
    //             myActiveKey = myActiveID;
    //             myActiveProgram = "0";
    //             myLogicType = myType == SumoXMLNodeType::RAIL_SIGNAL ? TrafficLightType::RAIL_SIGNAL : TrafficLightType::RAIL_CROSSING;
    //             closeTrafficLightLogic(basePath);
    //             junction = buildLogicJunction(new MSBitsetLogic(myRequestSize, myActiveLogic, myActiveFoes, myActiveConts));
    //             break;
    //         default:
    //             throw InvalidArgument("False junction logic type.");
    //     }
    //     if (junction != nullptr) {
    //         if (!myJunctions->add(myActiveID, junction)) {
    //             delete junction;
    //             throw InvalidArgument("Another junction with the id '" + myActiveID + "' exists.");
    //         }
    //         junction->updateParameters(myAdditionalParameter);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_junction(&mut self, basePath: &str) {
        todo!("port NLJunctionControlBuilder::closeJunction")
    }

    /// C++: `MSJunctionControl * NLJunctionControlBuilder::build() const` [const]
    /// Builds the MSJunctionControl which holds all of the simulations junctions
    // MSJunctionControl*
    // NLJunctionControlBuilder::build() const {
    //     MSJunctionControl* js = myJunctions;
    //     myJunctions = nullptr;
    //     return js;
    // }
    #[allow(unused_variables)]
    pub fn build(&self) -> Ptr<MSJunctionControl> {
        todo!("port NLJunctionControlBuilder::build")
    }

    /// C++: `void NLJunctionControlBuilder::initJunctionLogic(const std::string &)`
    /// Initialises a junction logic
    // void
    // NLJunctionControlBuilder::initJunctionLogic(const std::string& id) {
    //     myActiveKey = id;
    //     myActiveProgram = "";
    //     myActiveLogic.clear();
    //     myActiveFoes.clear();
    //     myActiveConts.reset();
    //     myRequestSize = NO_REQUEST_SIZE; // seems not to be used
    //     myRequestItemNumber = 0;
    //     myCurrentHasError = false;
    // }
    #[allow(unused_variables)]
    pub fn init_junction_logic(&mut self, id: &str) {
        todo!("port NLJunctionControlBuilder::initJunctionLogic")
    }

    /// C++: `void NLJunctionControlBuilder::addLogicItem(int, const std::string &, const std::string &, bool)`
    /// Adds a logic item
    // void
    // NLJunctionControlBuilder::addLogicItem(int request,
    //                                        const std::string& response,
    //                                        const std::string& foes,
    //                                        bool cont) {
    //     if (myCurrentHasError) {
    //         // had an error
    //         return;
    //     }
    //     if (request >= SUMO_MAX_CONNECTIONS) {
    //         // bad request
    //         myCurrentHasError = true;
    //         throw InvalidArgument("Junction logic '" + myActiveKey + "' is larger than allowed; recheck the network.");
    //     }
    //     if (myRequestSize == NO_REQUEST_SIZE) {
    //         // initialize
    //         myRequestSize = (int)response.size();
    //     }
    //     if (static_cast<int>(response.size()) != myRequestSize) {
    //         myCurrentHasError = true;
    //         throw InvalidArgument("Invalid response size " + toString(response.size()) +
    //                               " in Junction logic '" + myActiveKey + "' (expected  " + toString(myRequestSize) + ")");
    //     }
    //     if (static_cast<int>(foes.size()) != myRequestSize) {
    //         myCurrentHasError = true;
    //         throw InvalidArgument("Invalid foes size " + toString(foes.size()) +
    //                               " in Junction logic '" + myActiveKey + "' (expected  " + toString(myRequestSize) + ")");
    //     }
    //     // assert that the logicitems come ordered by their request index
    //     assert((int)myActiveLogic.size() == request);
    //     assert((int)myActiveFoes.size() == request);
    //     // add the read response for the given request index
    //     myActiveLogic.push_back(std::bitset<SUMO_MAX_CONNECTIONS>(response));
    //     // add the read junction-internal foes for the given request index
    //     myActiveFoes.push_back(std::bitset<SUMO_MAX_CONNECTIONS>(foes));
    //     // add whether the vehicle may drive a little bit further
    //     myActiveConts.set(request, cont);
    //     // increse number of set information
    //     myRequestItemNumber++;
    // }
    #[allow(unused_variables)]
    pub fn add_logic_item(&mut self, request: i32, response: &str, foes: &str, cont: bool) {
        todo!("port NLJunctionControlBuilder::addLogicItem")
    }

    /// C++: `void NLJunctionControlBuilder::initTrafficLightLogic(const std::string &, const std::string &, TrafficLightType, SUMOTime)`
    /// Begins the reading of a traffic lights logic
    // void
    // NLJunctionControlBuilder::initTrafficLightLogic(const std::string& id, const std::string& programID,
    //         TrafficLightType type, SUMOTime offset) {
    //     myActiveKey = id;
    //     myActiveProgram = programID;
    //     myActivePhases.clear();
    //     myActiveConditions.clear();
    //     myActiveAssignments.clear();
    //     myActiveFunctions.clear();
    //     myAbsDuration = 0;
    //     myRequestSize = NO_REQUEST_SIZE;
    //     myLogicType = type;
    //     myOffset = offset;
    //     myAdditionalParameter.clear();
    // }
    #[allow(unused_variables)]
    pub fn init_traffic_light_logic(&mut self, id: &str, programID: &str, type_: TrafficLightType, offset: SUMOTime) {
        todo!("port NLJunctionControlBuilder::initTrafficLightLogic")
    }

    /// C++: `void NLJunctionControlBuilder::addPhase(MSPhaseDefinition *)`
    /// Adds a phase to the currently built traffic lights logic
    // void
    // NLJunctionControlBuilder::addPhase(MSPhaseDefinition* phase) {
    //     // build and add the phase definition to the list
    //     myActivePhases.push_back(phase);
    //     // add phase duration to the absolute duration
    //     myAbsDuration += phase->duration;
    // }
    #[allow(unused_variables)]
    pub fn add_phase(&mut self, phase: Ptr<MSPhaseDefinition>) {
        todo!("port NLJunctionControlBuilder::addPhase")
    }

    /// C++: `bool NLJunctionControlBuilder::addCondition(const std::string &, const std::string &)`
    /// Adds a condition to the currently built traffic lights logic
    // bool
    // NLJunctionControlBuilder::addCondition(const std::string& id, const std::string& value) {
    //     if (myActiveConditions.count(id) == 0) {
    //         myActiveConditions[id] = value;
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_condition(&mut self, id: &str, value: &str) -> bool {
        todo!("port NLJunctionControlBuilder::addCondition")
    }

    /// C++: `void NLJunctionControlBuilder::addAssignment(const std::string &, const std::string &, const std::string &)`
    /// Adds an assignment to the currently built traffic lights logic
    // void
    // NLJunctionControlBuilder::addAssignment(const std::string& id, const std::string& check, const std::string& value) {
    //     if (myActiveFunction.id == "") {
    //         myActiveAssignments.push_back(std::make_tuple(id, check, value));
    //     } else {
    //         myActiveFunction.assignments.push_back(std::make_tuple(id, check, value));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_assignment(&mut self, id: &str, check: &str, value: &str) {
        todo!("port NLJunctionControlBuilder::addAssignment")
    }

    /// C++: `void NLJunctionControlBuilder::addFunction(const std::string &, int)`
    /// adds a switching condition function to the traffic lights logic currently build
    // void
    // NLJunctionControlBuilder::addFunction(const std::string& id, int nArgs) {
    //     myActiveFunction.id = id;
    //     myActiveFunction.nArgs = nArgs;
    // }
    #[allow(unused_variables)]
    pub fn add_function(&mut self, id: &str, nArgs: i32) {
        todo!("port NLJunctionControlBuilder::addFunction")
    }

    /// C++: `void NLJunctionControlBuilder::closeFunction()`
    /// closes a switching condition function to the traffic lights logic currently build
    // void
    // NLJunctionControlBuilder::closeFunction() {
    //     myActiveFunctions[myActiveFunction.id] = myActiveFunction;
    //     myActiveFunction.id = "";
    //     myActiveFunction.assignments.clear();
    // }
    #[allow(unused_variables)]
    pub fn close_function(&mut self) {
        todo!("port NLJunctionControlBuilder::closeFunction")
    }

    /// C++: `MSTLLogicControl::TLSLogicVariants & NLJunctionControlBuilder::getTLLogic(const std::string &) const` [const]
    /// Returns a previously build tls logic
    // MSTLLogicControl::TLSLogicVariants&
    // NLJunctionControlBuilder::getTLLogic(const std::string& id) const {
    //     return getTLLogicControlToUse().get(id);
    // }
    #[allow(unused_variables)]
    pub fn get_tl_logic(&self, id: &str) -> &mut MSTLLogicControl_TLSLogicVariants {
        todo!("port NLJunctionControlBuilder::getTLLogic")
    }

    /// C++: `MSTLLogicControl * NLJunctionControlBuilder::buildTLLogics()`
    /// Returns the built tls-logic control
    // MSTLLogicControl*
    // NLJunctionControlBuilder::buildTLLogics() {
    //     if (!myLogicControl->closeNetworkReading()) {
    //         throw ProcessError(TL("Traffic lights could not be built."));
    //     }
    //     for (MSTrafficLightLogic* const logic : myRailSignals) {
    //         logic->init(myDetectorBuilder);
    //     }
    //     MSTLLogicControl* ret = myLogicControl;
    //     myNetIsLoaded = true;
    //     myLogicControl = nullptr;
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn build_tl_logics(&mut self) -> Ptr<MSTLLogicControl> {
        todo!("port NLJunctionControlBuilder::buildTLLogics")
    }

    /// C++: `void NLJunctionControlBuilder::closeTrafficLightLogic(const std::string &)` [virtual]
    /// Ends the building of a traffic lights logic
    // void
    // NLJunctionControlBuilder::closeTrafficLightLogic(const std::string& basePath) {
    //     if (myActiveProgram == "off") {
    //         if (myAbsDuration > 0) {
    //             throw InvalidArgument("The off program for TLS '" + myActiveKey + "' has phases.");
    //         }
    //         MSOffTrafficLightLogic* off = new MSOffTrafficLightLogic(getTLLogicControlToUse(), myActiveKey);
    //         if (!getTLLogicControlToUse().add(myActiveKey, myActiveProgram, off)) {
    //             throw InvalidArgument("Another logic with id '" + myActiveKey + "' and programID '" + myActiveProgram + "' exists.");
    //         }
    //         return;
    //     }
    //     SUMOTime firstEventOffset = 0;
    //     int step = 0;
    //     MSSimpleTrafficLightLogic::Phases::const_iterator i = myActivePhases.begin();
    //     MSTrafficLightLogic* existing = getTLLogicControlToUse().get(myActiveKey, myActiveProgram);
    //     if (existing != nullptr && (existing->getLogicType() == TrafficLightType::RAIL_SIGNAL || existing->getLogicType() == TrafficLightType::RAIL_CROSSING)) {
    //         existing->updateParameters(myAdditionalParameter);
    //         return;
    //     } else {
    //         if (myLogicType != TrafficLightType::RAIL_SIGNAL && myLogicType != TrafficLightType::RAIL_CROSSING) {
    //             if (myAbsDuration == 0) {
    //                 if (existing == nullptr) {
    //                     throw InvalidArgument("TLS program '" + myActiveProgram + "' for TLS '" + myActiveKey + "' has a duration of 0.");
    //                 } else {
    //                     // only modify the offset of an existing logic
    //                     myAbsDuration = existing->getDefaultCycleTime();
    //                     i = existing->getPhases().begin();
    //                 }
    //             } else if (existing != nullptr) {
    //                 throw InvalidArgument("Another logic with id '" + myActiveKey + "' and programID '" + myActiveProgram + "' exists.");
    //             }
    //             // compute the initial step and first switch time of the tls-logic
    //             // a positive offset delays all phases by x (advance by absDuration - x) while a negative offset advances all phases by x seconds
    //             // @note The implementation of % for negative values is implementation defined in ISO1998
    //             SUMOTime offset; // the time to run the traffic light in advance
    //             if (myOffset >= 0) {
    //                 offset = (myNet.getCurrentTimeStep() + myAbsDuration - (myOffset % myAbsDuration)) % myAbsDuration;
    //             } else {
    //                 offset = (myNet.getCurrentTimeStep() + ((-myOffset) % myAbsDuration)) % myAbsDuration;
    //             }
    //             while (offset >= (*i)->duration) {
    //                 step++;
    //                 offset -= (*i)->duration;
    //                 ++i;
    //             }
    //             firstEventOffset = (*i)->duration - offset + myNet.getCurrentTimeStep();
    //             if (existing != nullptr) {
    //                 existing->changeStepAndDuration(getTLLogicControlToUse(),
    //                                                 myNet.getCurrentTimeStep(), step, (*i)->duration - offset);
    //                 // parameters that are used when initializing a logic will not take
    //                 // effect but parameters that are checked at runtime can be used
    //                 // here (i.e. device.glosa.range)
    //                 myLogicParams[existing] = myAdditionalParameter;
    //                 if (myAdditionalParameter.count(MESegment::OVERRIDE_TLS_PENALTIES) != 0) {
    //                     // value must be available when calling setMesoTypes and before setting the rest in postLoadInitialization
    //                     existing->setParameter(MESegment::OVERRIDE_TLS_PENALTIES, myAdditionalParameter[MESegment::OVERRIDE_TLS_PENALTIES]);
    //                 }
    //                 return;
    //             }
    //         }
    //     }
    //
    //     if (myActiveProgram == "") {
    //         myActiveProgram = "default";
    //     }
    //     MSTrafficLightLogic* tlLogic = nullptr;
    //     // build the tls-logic in dependence to its type
    //     switch (myLogicType) {
    //         case TrafficLightType::SWARM_BASED:
    //             firstEventOffset = DELTA_T; //this is needed because swarm needs to update the pheromone on the lanes at every step
    //             tlLogic = new MSSwarmTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myActivePhases, step, firstEventOffset, myAdditionalParameter);
    //             break;
    //         case TrafficLightType::HILVL_DETERMINISTIC:
    //             tlLogic = new MSDeterministicHiLevelTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myActivePhases, step, firstEventOffset, myAdditionalParameter);
    //             break;
    //         case TrafficLightType::SOTL_REQUEST:
    //             tlLogic = new MSSOTLPolicyBasedTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myLogicType, myActivePhases, step, firstEventOffset, myAdditionalParameter, new MSSOTLRequestPolicy(myAdditionalParameter));
    //             break;
    //         case TrafficLightType::SOTL_PLATOON:
    //             tlLogic = new MSSOTLPolicyBasedTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myLogicType, myActivePhases, step, firstEventOffset, myAdditionalParameter, new MSSOTLPlatoonPolicy(myAdditionalParameter));
    //             break;
    //         case TrafficLightType::SOTL_WAVE:
    //             tlLogic = new MSSOTLWaveTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myActivePhases, step, firstEventOffset, myAdditionalParameter);
    //             break;
    //         case TrafficLightType::SOTL_PHASE:
    //             tlLogic = new MSSOTLPolicyBasedTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myLogicType, myActivePhases, step, firstEventOffset, myAdditionalParameter, new MSSOTLPhasePolicy(myAdditionalParameter));
    //             break;
    //         case TrafficLightType::SOTL_MARCHING:
    //             tlLogic = new MSSOTLPolicyBasedTrafficLightLogic(getTLLogicControlToUse(), myActiveKey, myActiveProgram, myLogicType, myActivePhases, step, firstEventOffset, myAdditionalParameter, new MSSOTLMarchingPolicy(myAdditionalParameter));
    //             break;
    //         case TrafficLightType::ACTUATED:
    //             // @note it is unclear how to apply the given offset in the context
    //             // of variable-length phases
    //             tlLogic = new MSActuatedTrafficLightLogic(getTLLogicControlToUse(),
    //                     myActiveKey, myActiveProgram, myOffset,
    //                     myActivePhases, step, (*i)->minDuration + myNet.getCurrentTimeStep(),
    //                     myAdditionalParameter, basePath, myActiveConditions, myActiveAssignments, myActiveFunctions);
    //             break;
    //         case TrafficLightType::NEMA:
    //             tlLogic = new NEMALogic(getTLLogicControlToUse(),
    //                                     myActiveKey, myActiveProgram, myOffset,
    //                                     myActivePhases, step, (*i)->minDuration + myNet.getCurrentTimeStep(),
    //                                     myAdditionalParameter, basePath);
    //             break;
    //         case TrafficLightType::DELAYBASED:
    //             tlLogic = new MSDelayBasedTrafficLightLogic(getTLLogicControlToUse(),
    //                     myActiveKey, myActiveProgram, myOffset,
    //                     myActivePhases, step, (*i)->minDuration + myNet.getCurrentTimeStep(),
    //                     myAdditionalParameter, basePath);
    //             break;
    //         case TrafficLightType::STATIC:
    //             tlLogic = new MSSimpleTrafficLightLogic(getTLLogicControlToUse(),
    //                                                     myActiveKey, myActiveProgram, myOffset,
    //                                                     TrafficLightType::STATIC,
    //                                                     myActivePhases, step, firstEventOffset,
    //                                                     myAdditionalParameter);
    //             break;
    //         case TrafficLightType::RAIL_SIGNAL:
    //             tlLogic = new MSRailSignal(getTLLogicControlToUse(),
    //                                        myActiveKey, myActiveProgram, myNet.getCurrentTimeStep(),
    //                                        myAdditionalParameter);
    //             break;
    //         case TrafficLightType::RAIL_CROSSING:
    //             tlLogic = new MSRailCrossing(getTLLogicControlToUse(),
    //                                          myActiveKey, myActiveProgram, myNet.getCurrentTimeStep(),
    //                                          myAdditionalParameter);
    //             break;
    //         case TrafficLightType::OFF:
    //             tlLogic = new MSOffTrafficLightLogic(getTLLogicControlToUse(), myActiveKey);
    //             break;
    //         case TrafficLightType::INVALID:
    //             throw ProcessError(TLF("Invalid traffic light type '%'", toString(myLogicType)));
    //     }
    //     myActivePhases.clear();
    //     if (tlLogic != nullptr) {
    //         if (getTLLogicControlToUse().add(myActiveKey, myActiveProgram, tlLogic)) {
    //             if (myNetIsLoaded) {
    //                 myAdditionalLogics.push_back(tlLogic);
    //             } else if (myLogicType == TrafficLightType::RAIL_SIGNAL) {
    //                 // special case: intialize earlier because signals are already used when
    //                 // loading train routes in additional files
    //                 myRailSignals.push_back(tlLogic);
    //             } else {
    //                 myNetworkLogics.push_back(tlLogic);
    //             }
    //         } else {
    //             WRITE_ERRORF(TL("Another logic with id '%' and programID '%' exists."), myActiveKey, myActiveProgram);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_traffic_light_logic(&mut self, basePath: &str) {
        todo!("port NLJunctionControlBuilder::closeTrafficLightLogic")
    }

    /// C++: `void NLJunctionControlBuilder::addParam(const std::string &, const std::string &)`
    /// Adds a parameter
    // void
    // NLJunctionControlBuilder::addParam(const std::string& key,
    //                                    const std::string& value) {
    //     myAdditionalParameter[key] = value;
    // }
    #[allow(unused_variables)]
    pub fn add_param(&mut self, key: &str, value: &str) {
        todo!("port NLJunctionControlBuilder::addParam")
    }

    /// C++: `const std::string & NLJunctionControlBuilder::getActiveKey() const` [const]
    /// Returns the active key
    // const std::string&
    // NLJunctionControlBuilder::getActiveKey() const {
    //     return myActiveKey;
    // }
    #[allow(unused_variables)]
    pub fn get_active_key(&self) -> &str {
        todo!("port NLJunctionControlBuilder::getActiveKey")
    }

    /// C++: `const std::string & NLJunctionControlBuilder::getActiveSubKey() const` [const]
    /// Returns the active sub key
    // const std::string&
    // NLJunctionControlBuilder::getActiveSubKey() const {
    //     return myActiveProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_active_sub_key(&self) -> &str {
        todo!("port NLJunctionControlBuilder::getActiveSubKey")
    }

    /// C++: `MSTLLogicControl & NLJunctionControlBuilder::getTLLogicControlToUse() const` [const]
    /// Returns the used tls control
    // MSTLLogicControl&
    // NLJunctionControlBuilder::getTLLogicControlToUse() const {
    //     if (myLogicControl != nullptr) {
    //         return *myLogicControl;
    //     }
    //     return myNet.getTLSControl();
    // }
    #[allow(unused_variables)]
    pub fn get_tl_logic_control_to_use(&self) -> &mut MSTLLogicControl {
        todo!("port NLJunctionControlBuilder::getTLLogicControlToUse")
    }

    /// C++: `void NLJunctionControlBuilder::postLoadInitialization()`
    /// initialize junctions after all connections have been loaded
    // void
    // NLJunctionControlBuilder::postLoadInitialization() {
    //     for (MSTrafficLightLogic* const logic : myNetworkLogics) {
    //         logic->init(myDetectorBuilder);
    //     }
    //     for (MSTrafficLightLogic* const logic : myAdditionalLogics) {
    //         logic->init(myDetectorBuilder);
    //     }
    //     // delay parameter loading until initialization
    //     for (auto item : myLogicParams) {
    //         item.first->updateParameters(item.second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn post_load_initialization(&mut self) {
        todo!("port NLJunctionControlBuilder::postLoadInitialization")
    }

    /// C++: `MSJunction * NLJunctionControlBuilder::retrieve(const std::string)`
    /// try to retrieve junction by id
    // MSJunction*
    // NLJunctionControlBuilder::retrieve(const std::string id) {
    //     if (myJunctions != nullptr) {
    //         return myJunctions->get(id);
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn retrieve(&mut self, id: String) -> Ptr<MSJunction> {
        todo!("port NLJunctionControlBuilder::retrieve")
    }

    /// C++: `const MSSimpleTrafficLightLogic::Phases & NLJunctionControlBuilder::getLoadedPhases() const` [const]
    /// return the phases loaded so far (for error reporting and cleanup)
    // const MSSimpleTrafficLightLogic::Phases& getLoadedPhases() const {
    //         return myActivePhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_phases(&self) -> &MSTrafficLightLogic_Phases {
        todo!("port NLJunctionControlBuilder::getLoadedPhases")
    }

    /// C++: `void NLJunctionControlBuilder::netIsLoaded()`
    // void netIsLoaded() {
    //         myNetIsLoaded = true;
    //     }
    #[allow(unused_variables)]
    pub fn net_is_loaded(&mut self) {
        todo!("port NLJunctionControlBuilder::netIsLoaded")
    }

    /// C++: `MSJunction * NLJunctionControlBuilder::buildNoLogicJunction()` [virtual, protected]
    /// Builds a junction that does not use a logic
    // MSJunction*
    // NLJunctionControlBuilder::buildNoLogicJunction() {
    //     return new MSNoLogicJunction(myActiveID, myType, myPosition, myShape, myActiveName,
    //                                  myActiveIncomingLanes, myActiveInternalLanes);
    // }
    #[allow(unused_variables)]
    pub fn build_no_logic_junction(&mut self) -> Ptr<MSJunction> {
        todo!("port NLJunctionControlBuilder::buildNoLogicJunction")
    }

    /// C++: `MSJunction * NLJunctionControlBuilder::buildLogicJunction(MSJunctionLogic *const)` [virtual, protected]
    /// Builds a junction with a logic
    // MSJunction*
    // NLJunctionControlBuilder::buildLogicJunction(MSJunctionLogic* const logic) {
    //     return new MSRightOfWayJunction(myActiveID, myType, myPosition, myShape, myActiveName,
    //                                     myActiveIncomingLanes, myActiveInternalLanes, logic);
    // }
    #[allow(unused_variables)]
    pub fn build_logic_junction(&mut self, logic: Ptr<MSJunctionLogic>) -> Ptr<MSJunction> {
        todo!("port NLJunctionControlBuilder::buildLogicJunction")
    }

    /// C++: `MSJunction * NLJunctionControlBuilder::buildInternalJunction()` [virtual, protected]
    /// Builds an internal junction
    // MSJunction*
    // NLJunctionControlBuilder::buildInternalJunction() {
    //     // build the junction
    //     return new MSInternalJunction(myActiveID, myType, myPosition, myShape, myActiveIncomingLanes,
    //                                   myActiveInternalLanes);
    // }
    #[allow(unused_variables)]
    pub fn build_internal_junction(&mut self) -> Ptr<MSJunction> {
        todo!("port NLJunctionControlBuilder::buildInternalJunction")
    }

    /// C++: `void NLJunctionControlBuilder::NLJunctionControlBuilder(const NLJunctionControlBuilder &)` [private]
    /// invalidated copy operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLJunctionControlBuilder) -> Self {
        todo!("port NLJunctionControlBuilder::NLJunctionControlBuilder")
    }

    /// C++: `NLJunctionControlBuilder & NLJunctionControlBuilder::operator=(const NLJunctionControlBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLJunctionControlBuilder) -> &mut NLJunctionControlBuilder {
        todo!("port NLJunctionControlBuilder::operator=")
    }

}

