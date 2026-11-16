//! Port of netload/NLJunctionControlBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::SUMO_MAX_CONNECTIONS;

/// C++ typedef `std::vector<MSLane *> NLJunctionControlBuilder::LaneVector`
pub type NLJunctionControlBuilder_LaneVector = Vec<Ptr<MSLane>>;

/// C++ typedef `Parameterised::Map NLJunctionControlBuilder::StringParameterMap`
pub type NLJunctionControlBuilder_StringParameterMap = ParameterisedMap;

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
    pub myActiveLogic: Vec<[bool; SUMO_MAX_CONNECTIONS]>,
    /// The description about which lanes disallow other passing the junction simultaneously
    pub myActiveFoes: Vec<[bool; SUMO_MAX_CONNECTIONS]>,
    /// The description about which lanes have an internal follower
    pub myActiveConts: [bool; SUMO_MAX_CONNECTIONS],
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
    pub myActiveIncomingLanes: NLJunctionControlBuilder_LaneVector,
    /// The list of the internal lanes of the currently chosen junction
    pub myActiveInternalLanes: NLJunctionControlBuilder_LaneVector,
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
const NO_REQUEST_SIZE: i32 = -1;

/// C++ `throw InvalidArgument(msg)`: the void signatures in this file unwind with the ProcessError (as other ported callers expect).
fn throw_invalid_argument(msg: String) -> ! {
    std::panic::panic_any(ProcessError::InvalidArgument(msg))
}

/// C++ `new T(...)` for objects the simulation owns for its whole lifetime (the C++ code never deletes them).
fn leak_box<T>(v: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(v)))
}

/// C++ `std::bitset<SUMO_MAX_CONNECTIONS>(str)`: the rightmost character is bit 0, missing bits are 0.
fn bitset_from_str(s: &str) -> [bool; SUMO_MAX_CONNECTIONS] {
    let mut bits = [false; SUMO_MAX_CONNECTIONS];
    for (i, c) in s.chars().rev().take(SUMO_MAX_CONNECTIONS).enumerate() {
        match c {
            '0' => {}
            '1' => bits[i] = true,
            // std::invalid_argument in C++
            _ => panic!("bitset_from_str: invalid character in a junction logic string"),
        }
    }
    bits
}

/// C++ `new MSBitsetLogic(nLinks, logic, foes, conts)` as an MSJunctionLogic pointer.
fn new_bitset_logic(nLinks: i32, logic: &Vec<[bool; SUMO_MAX_CONNECTIONS]>, foes: &Vec<[bool; SUMO_MAX_CONNECTIONS]>, conts: [bool; SUMO_MAX_CONNECTIONS]) -> Ptr<MSJunctionLogic> {
    leak_box(MSBitSetLogic::<SUMO_MAX_CONNECTIONS>::new(nLinks, logic, foes, conts)).cast::<MSJunctionLogic>()
}

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
        let mut b = Self {
            myNet: Ptr::from_mut(net),
            myDetectorBuilder: Ptr::from_mut(db),
            myOffset: 0,
            // C++ leaves the remaining members uninitialised until the first junction is opened
            myLogicType: TrafficLightType::INVALID,
            myActiveLogic: Vec::new(),
            myActiveFoes: Vec::new(),
            myActiveConts: [false; SUMO_MAX_CONNECTIONS],
            myActivePhases: Vec::new(),
            myActiveConditions: ParameterisedMap::new(),
            myActiveAssignments: Vec::new(),
            myActiveFunctions: BTreeMap::new(),
            myActiveFunction: MSActuatedTrafficLightLogic_Function { id: String::new(), nArgs: 0, assignments: Vec::new() },
            myRequestSize: NO_REQUEST_SIZE,
            myRequestItemNumber: 0,
            myJunctions: leak_box(MSJunctionControl::new()),
            myActiveIncomingLanes: Vec::new(),
            myActiveInternalLanes: Vec::new(),
            myActiveID: String::new(),
            myActiveKey: String::new(),
            myActiveProgram: String::new(),
            myType: SumoXMLNodeType::UNKNOWN,
            myPosition: Position::default(),
            myAbsDuration: 0,
            myShape: PositionVector::default(),
            myActiveName: String::new(),
            myNetworkLogics: Vec::new(),
            myAdditionalLogics: Vec::new(),
            myRailSignals: Vec::new(),
            myLogicParams: BTreeMap::new(),
            myLogicControl: Ptr::null(),
            myAdditionalParameter: ParameterisedMap::new(),
            myCurrentHasError: false,
            myNetIsLoaded: false,
        };
        b.myLogicControl = leak_box(MSTLLogicControl::new());
        b
    }

    /// C++: `void NLJunctionControlBuilder::~NLJunctionControlBuilder() noexcept` [virtual]
    /// Destructor
    // NLJunctionControlBuilder::~NLJunctionControlBuilder() {
    //     delete myLogicControl;
    //     delete myJunctions;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // delete myLogicControl
        if !self.myLogicControl.is_null() {
            let lc = self.myLogicControl;
            lc.m().drop_();
            drop(unsafe { Box::from_raw(lc.as_raw()) });
        }
        // delete myJunctions
        if !self.myJunctions.is_null() {
            let jc = self.myJunctions;
            jc.m().drop_();
            drop(unsafe { Box::from_raw(jc.as_raw()) });
        }
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
        self.myActiveInternalLanes = internalLanes.clone();
        self.myActiveIncomingLanes = incomingLanes.clone();
        self.myActiveID = id.to_string();
        self.myActiveKey = key.to_string();
        self.myType = type_;
        self.myPosition = pos;
        self.myShape = shape.clone();
        self.myActiveName = name.to_string();
        self.myAdditionalParameter.clear();
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
        if self.myCurrentHasError {
            // had an error before...
            return;
        }
        if self.myRequestSize != NO_REQUEST_SIZE && self.myRequestItemNumber != self.myRequestSize {
            throw_invalid_argument(format!("The description for the junction logic '{}' is malicious.", self.myActiveKey));
        }
        if self.myJunctions.is_null() {
            std::panic::panic_any(ProcessError::new("Information about the number of nodes was missing."));
        }
        let mut junction: Ptr<MSJunction> = Ptr::null();
        match self.myType {
            SumoXMLNodeType::NOJUNCTION | SumoXMLNodeType::DEAD_END | SumoXMLNodeType::DEAD_END_DEPRECATED | SumoXMLNodeType::DISTRICT | SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION => {
                if !self.myActiveLogic.is_empty() {
                    utils::common::msg_handler::write_warning(&format!("Ignoring junction logic for junction '{}'.", self.myActiveID));
                }
                junction = self.build_no_logic_junction();
            }
            SumoXMLNodeType::TRAFFIC_LIGHT
            | SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED
            | SumoXMLNodeType::RIGHT_BEFORE_LEFT
            | SumoXMLNodeType::LEFT_BEFORE_RIGHT
            | SumoXMLNodeType::PRIORITY
            | SumoXMLNodeType::PRIORITY_STOP
            | SumoXMLNodeType::ALLWAY_STOP
            | SumoXMLNodeType::ZIPPER => {
                let logic = new_bitset_logic(self.myRequestSize, &self.myActiveLogic, &self.myActiveFoes, self.myActiveConts);
                junction = self.build_logic_junction(logic);
            }
            SumoXMLNodeType::INTERNAL => {
                if crate::microsim::ms_globals::using_internal_lanes() {
                    if !self.myActiveLogic.is_empty() {
                        utils::common::msg_handler::write_warning(&format!("Ignoring junction logic for junction '{}'.", self.myActiveID));
                    }
                    junction = self.build_internal_junction();
                }
            }
            SumoXMLNodeType::RAIL_SIGNAL | SumoXMLNodeType::RAIL_CROSSING => {
                self.myOffset = 0;
                self.myActiveKey = self.myActiveID.clone();
                self.myActiveProgram = "0".to_string();
                self.myLogicType = if matches!(self.myType, SumoXMLNodeType::RAIL_SIGNAL) { TrafficLightType::RAIL_SIGNAL } else { TrafficLightType::RAIL_CROSSING };
                self.close_traffic_light_logic(basePath);
                let logic = new_bitset_logic(self.myRequestSize, &self.myActiveLogic, &self.myActiveFoes, self.myActiveConts);
                junction = self.build_logic_junction(logic);
            }
            _ => throw_invalid_argument("False junction logic type.".to_string()),
        }
        if !junction.is_null() {
            let added = {
                let jc = self.myJunctions.m();
                if jc.contains_key(&self.myActiveID) {
                    false
                } else {
                    jc.insert(self.myActiveID.clone(), junction);
                    true
                }
            };
            if !added {
                // shortcut: C++ `delete junction` on a duplicate id is skipped, the concrete junction type is not known here to free it
                throw_invalid_argument(format!("Another junction with the id '{}' exists.", self.myActiveID));
            }
            junction.m().base2.update_parameters(&self.myAdditionalParameter);
        }
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
    pub fn build(&mut self) -> Ptr<MSJunctionControl> {
        // C++ is const but clears myJunctions, which needs &mut self here
        let js = self.myJunctions;
        self.myJunctions = Ptr::null();
        js
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
        self.myActiveKey = id.to_string();
        self.myActiveProgram = String::new();
        self.myActiveLogic.clear();
        self.myActiveFoes.clear();
        self.myActiveConts = [false; SUMO_MAX_CONNECTIONS];
        self.myRequestSize = NO_REQUEST_SIZE; // seems not to be used
        self.myRequestItemNumber = 0;
        self.myCurrentHasError = false;
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
        if self.myCurrentHasError {
            // had an error
            return;
        }
        if request >= SUMO_MAX_CONNECTIONS as i32 {
            // bad request
            self.myCurrentHasError = true;
            throw_invalid_argument(format!("Junction logic '{}' is larger than allowed; recheck the network.", self.myActiveKey));
        }
        if self.myRequestSize == NO_REQUEST_SIZE {
            // initialize
            self.myRequestSize = response.len() as i32;
        }
        if response.len() as i32 != self.myRequestSize {
            self.myCurrentHasError = true;
            throw_invalid_argument(format!("Invalid response size {} in Junction logic '{}' (expected  {})", response.len(), self.myActiveKey, self.myRequestSize));
        }
        if foes.len() as i32 != self.myRequestSize {
            self.myCurrentHasError = true;
            throw_invalid_argument(format!("Invalid foes size {} in Junction logic '{}' (expected  {})", foes.len(), self.myActiveKey, self.myRequestSize));
        }
        // assert that the logicitems come ordered by their request index
        debug_assert_eq!(self.myActiveLogic.len() as i32, request);
        debug_assert_eq!(self.myActiveFoes.len() as i32, request);
        // add the read response for the given request index
        self.myActiveLogic.push(bitset_from_str(response));
        // add the read junction-internal foes for the given request index
        self.myActiveFoes.push(bitset_from_str(foes));
        // add whether the vehicle may drive a little bit further
        self.myActiveConts[request as usize] = cont;
        // increse number of set information
        self.myRequestItemNumber += 1;
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
        self.myActiveKey = id.to_string();
        self.myActiveProgram = programID.to_string();
        self.myActivePhases.clear();
        self.myActiveConditions.clear();
        self.myActiveAssignments.clear();
        self.myActiveFunctions.clear();
        self.myAbsDuration = 0;
        self.myRequestSize = NO_REQUEST_SIZE;
        self.myLogicType = type_;
        self.myOffset = offset;
        self.myAdditionalParameter.clear();
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
        // build and add the phase definition to the list
        self.myActivePhases.push(phase);
        // add phase duration to the absolute duration
        self.myAbsDuration += phase.duration;
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
        if !self.myActiveConditions.contains_key(id) {
            self.myActiveConditions.insert(id.to_string(), value.to_string());
            true
        } else {
            false
        }
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
        if self.myActiveFunction.id == "" {
            self.myActiveAssignments.push((id.to_string(), check.to_string(), value.to_string()));
        } else {
            self.myActiveFunction.assignments.push((id.to_string(), check.to_string(), value.to_string()));
        }
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
        self.myActiveFunction.id = id.to_string();
        self.myActiveFunction.nArgs = nArgs;
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
        self.myActiveFunctions.insert(self.myActiveFunction.id.clone(), self.myActiveFunction.clone());
        self.myActiveFunction.id = String::new();
        self.myActiveFunction.assignments.clear();
    }

    /// C++: `MSTLLogicControl::TLSLogicVariants & NLJunctionControlBuilder::getTLLogic(const std::string &) const` [const]
    /// Returns a previously build tls logic
    // MSTLLogicControl::TLSLogicVariants&
    // NLJunctionControlBuilder::getTLLogic(const std::string& id) const {
    //     return getTLLogicControlToUse().get(id);
    // }
    #[allow(unused_variables)]
    pub fn get_tl_logic(&self, id: &str) -> &mut MSTLLogicControl_TLSLogicVariants {
        self.get_tl_logic_control_to_use().get(id)
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
        if !self.myLogicControl.m().close_network_reading() {
            std::panic::panic_any(ProcessError::new("Traffic lights could not be built."));
        }
        for logic in self.myRailSignals.iter() {
            logic.m().init(self.myDetectorBuilder.m());
        }
        let ret = self.myLogicControl;
        self.myNetIsLoaded = true;
        self.myLogicControl = Ptr::null();
        ret
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
        if self.myActiveProgram == "off" {
            if self.myAbsDuration > 0 {
                throw_invalid_argument(format!("The off program for TLS '{}' has phases.", self.myActiveKey));
            }
            let offLogic = leak_box(MSOffTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey));
            offLogic.m().base.schedule_switch_command(self.get_tl_logic_control_to_use(), 0);
            let off = offLogic.cast::<MSTrafficLightLogic>();
            if !self.get_tl_logic_control_to_use().add(&self.myActiveKey, &self.myActiveProgram, off, false) {
                throw_invalid_argument(format!("Another logic with id '{}' and programID '{}' exists.", self.myActiveKey, self.myActiveProgram));
            }
            return;
        }
        let mut firstEventOffset: SUMOTime = 0;
        let mut step: i32 = 0;
        // C++ iterator `i` over myActivePhases; it moves to the existing logic's phases below
        let mut phaseSrc: Ptr<MSTrafficLightLogic_Phases> = Ptr::from_mut(&mut self.myActivePhases);
        let mut i: usize = 0;
        let existing: Ptr<MSTrafficLightLogic> = self.get_tl_logic_control_to_use().get_2(&self.myActiveKey, &self.myActiveProgram);
        let now = self.myNet.get_current_time_step();
        if !existing.is_null() && matches!(existing.get_logic_type(), TrafficLightType::RAIL_SIGNAL | TrafficLightType::RAIL_CROSSING) {
            existing.m().base2.update_parameters(&self.myAdditionalParameter);
            return;
        }
        if !matches!(self.myLogicType, TrafficLightType::RAIL_SIGNAL | TrafficLightType::RAIL_CROSSING) {
            if self.myAbsDuration == 0 {
                if existing.is_null() {
                    throw_invalid_argument(format!("TLS program '{}' for TLS '{}' has a duration of 0.", self.myActiveProgram, self.myActiveKey));
                } else {
                    // only modify the offset of an existing logic
                    self.myAbsDuration = existing.get_default_cycle_time();
                    phaseSrc = Ptr::new(existing.get_phases());
                    i = 0;
                }
            } else if !existing.is_null() {
                throw_invalid_argument(format!("Another logic with id '{}' and programID '{}' exists.", self.myActiveKey, self.myActiveProgram));
            }
            // compute the initial step and first switch time of the tls-logic
            // a positive offset delays all phases by x (advance by absDuration - x) while a negative offset advances all phases by x seconds
            // @note The implementation of % for negative values is implementation defined in ISO1998
            let mut offset: SUMOTime; // the time to run the traffic light in advance
            if self.myOffset >= 0 {
                offset = (now + self.myAbsDuration - (self.myOffset % self.myAbsDuration)) % self.myAbsDuration;
            } else {
                offset = (now + ((-self.myOffset) % self.myAbsDuration)) % self.myAbsDuration;
            }
            let phases = phaseSrc.m();
            while offset >= phases[i].duration {
                step += 1;
                offset -= phases[i].duration;
                i += 1;
            }
            firstEventOffset = phases[i].duration - offset + now;
            if !existing.is_null() {
                existing.m().change_step_and_duration(self.get_tl_logic_control_to_use(), now, step, phases[i].duration - offset);
                // parameters that are used when initializing a logic will not take
                // effect but parameters that are checked at runtime can be used
                // here (i.e. device.glosa.range)
                self.myLogicParams.insert(existing, self.myAdditionalParameter.clone());
                // MESegment::OVERRIDE_TLS_PENALTIES
                const OVERRIDE_TLS_PENALTIES: &str = "meso.tls.control";
                if self.myAdditionalParameter.contains_key(OVERRIDE_TLS_PENALTIES) {
                    // value must be available when calling setMesoTypes and before setting the rest in postLoadInitialization
                    let v = self.myAdditionalParameter[OVERRIDE_TLS_PENALTIES].clone();
                    existing.m().base2.set_parameter(OVERRIDE_TLS_PENALTIES, &v);
                }
                return;
            }
        }

        if self.myActiveProgram == "" {
            self.myActiveProgram = "default".to_string();
        }
        let mut tlLogic: Ptr<MSTrafficLightLogic> = Ptr::null();
        // build the tls-logic in dependence to its type
        match self.myLogicType {
            TrafficLightType::SWARM_BASED => {
                firstEventOffset = utils::common::time::delta_t(); // this is needed because swarm needs to update the pheromone on the lanes at every step
                let swarm = leak_box(MSSwarmTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter));
                swarm.m().base.base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = swarm.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::HILVL_DETERMINISTIC => {
                let deterministic = leak_box(MSDeterministicHiLevelTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter));
                deterministic.m().base.base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = deterministic.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::SOTL_REQUEST => {
                let policy = leak_box(MSSOTLRequestPolicy::new(&self.myAdditionalParameter)).cast::<MSSOTLPolicy>();
                let policyLogic = leak_box(MSSOTLPolicyBasedTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, self.myLogicType, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter, policy));
                policyLogic.m().base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = policyLogic.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::SOTL_PLATOON => {
                let policy = leak_box(MSSOTLPlatoonPolicy::new(&self.myAdditionalParameter)).cast::<MSSOTLPolicy>();
                let policyLogic = leak_box(MSSOTLPolicyBasedTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, self.myLogicType, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter, policy));
                policyLogic.m().base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = policyLogic.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::SOTL_WAVE => {
                let wave = leak_box(MSSOTLWaveTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter));
                wave.m().base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = wave.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::SOTL_PHASE => {
                let policy = leak_box(MSSOTLPhasePolicy::new(&self.myAdditionalParameter)).cast::<MSSOTLPolicy>();
                let policyLogic = leak_box(MSSOTLPolicyBasedTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, self.myLogicType, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter, policy));
                policyLogic.m().base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = policyLogic.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::SOTL_MARCHING => {
                let policy = leak_box(MSSOTLMarchingPolicy::new(&self.myAdditionalParameter)).cast::<MSSOTLPolicy>();
                let policyLogic = leak_box(MSSOTLPolicyBasedTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, self.myLogicType, &self.myActivePhases, step, firstEventOffset, &self.myAdditionalParameter, policy));
                policyLogic.m().base.base.schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = policyLogic.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::ACTUATED => {
                // @note it is unclear how to apply the given offset in the context
                // of variable-length phases
                let actuated = leak_box(MSActuatedTrafficLightLogic::new(
                    self.get_tl_logic_control_to_use(),
                    &self.myActiveKey,
                    &self.myActiveProgram,
                    self.myOffset,
                    &self.myActivePhases,
                    step,
                    phaseSrc.m()[i].minDuration + now,
                    &self.myAdditionalParameter,
                    basePath,
                    &self.myActiveConditions,
                    &self.myActiveAssignments,
                    &self.myActiveFunctions,
                ));
                actuated.m().base.schedule_initial_switch(self.get_tl_logic_control_to_use(), phaseSrc.m()[i].minDuration + now);
                tlLogic = actuated.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::NEMA => {
                let nema = leak_box(NEMALogic::new(
                    self.get_tl_logic_control_to_use(),
                    &self.myActiveKey,
                    &self.myActiveProgram,
                    self.myOffset,
                    &self.myActivePhases,
                    step,
                    phaseSrc.m()[i].minDuration + now,
                    &self.myAdditionalParameter,
                    basePath,
                ));
                nema.m().base.schedule_initial_switch(self.get_tl_logic_control_to_use(), phaseSrc.m()[i].minDuration + now);
                tlLogic = nema.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::DELAYBASED => {
                let delayBased = leak_box(MSDelayBasedTrafficLightLogic::new(
                    self.get_tl_logic_control_to_use(),
                    &self.myActiveKey,
                    &self.myActiveProgram,
                    self.myOffset,
                    &self.myActivePhases,
                    step,
                    phaseSrc.m()[i].minDuration + now,
                    &self.myAdditionalParameter,
                    basePath,
                ));
                delayBased.m().base.schedule_initial_switch(self.get_tl_logic_control_to_use(), phaseSrc.m()[i].minDuration + now);
                tlLogic = delayBased.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::STATIC => {
                let simple = leak_box(MSSimpleTrafficLightLogic::new(
                    self.get_tl_logic_control_to_use(),
                    &self.myActiveKey,
                    &self.myActiveProgram,
                    self.myOffset,
                    TrafficLightType::STATIC,
                    &self.myActivePhases,
                    step,
                    firstEventOffset,
                    &self.myAdditionalParameter,
                ));
                // the C++ constructor schedules the first SwitchCommand itself; without it the signal never leaves phase 0
                simple.m().schedule_initial_switch(self.get_tl_logic_control_to_use(), firstEventOffset);
                tlLogic = simple.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::RAIL_SIGNAL => {
                // no initial switch: C++ MSRailSignal deschedules the base command it creates
                tlLogic = leak_box(MSRailSignal::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, now, &self.myAdditionalParameter)).cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::RAIL_CROSSING => {
                let crossing = leak_box(MSRailCrossing::new(self.get_tl_logic_control_to_use(), &self.myActiveKey, &self.myActiveProgram, now, &self.myAdditionalParameter));
                // base-level only: the C++ Simple constructor sees no phases yet, so no earliest-end adjustment
                crossing.m().base.base.schedule_switch_command(self.get_tl_logic_control_to_use(), now);
                tlLogic = crossing.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::OFF => {
                let off = leak_box(MSOffTrafficLightLogic::new(self.get_tl_logic_control_to_use(), &self.myActiveKey));
                off.m().base.schedule_switch_command(self.get_tl_logic_control_to_use(), 0);
                tlLogic = off.cast::<MSTrafficLightLogic>();
            }
            TrafficLightType::INVALID => {
                std::panic::panic_any(ProcessError::new(format!("Invalid traffic light type '{}'", TrafficLightTypes.get_string(self.myLogicType))));
            }
        }
        self.myActivePhases.clear();
        if !tlLogic.is_null() {
            if self.get_tl_logic_control_to_use().add(&self.myActiveKey, &self.myActiveProgram, tlLogic, false) {
                if self.myNetIsLoaded {
                    self.myAdditionalLogics.push(tlLogic);
                } else if matches!(self.myLogicType, TrafficLightType::RAIL_SIGNAL) {
                    // special case: intialize earlier because signals are already used when
                    // loading train routes in additional files
                    self.myRailSignals.push(tlLogic);
                } else {
                    self.myNetworkLogics.push(tlLogic);
                }
            } else {
                utils::common::msg_handler::write_error(&format!("Another logic with id '{}' and programID '{}' exists.", self.myActiveKey, self.myActiveProgram));
            }
        }
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
        self.myAdditionalParameter.insert(key.to_string(), value.to_string());
    }

    /// C++: `const std::string & NLJunctionControlBuilder::getActiveKey() const` [const]
    /// Returns the active key
    // const std::string&
    // NLJunctionControlBuilder::getActiveKey() const {
    //     return myActiveKey;
    // }
    #[allow(unused_variables)]
    pub fn get_active_key(&self) -> &str {
        &self.myActiveKey
    }

    /// C++: `const std::string & NLJunctionControlBuilder::getActiveSubKey() const` [const]
    /// Returns the active sub key
    // const std::string&
    // NLJunctionControlBuilder::getActiveSubKey() const {
    //     return myActiveProgram;
    // }
    #[allow(unused_variables)]
    pub fn get_active_sub_key(&self) -> &str {
        &self.myActiveProgram
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
        if !self.myLogicControl.is_null() {
            return self.myLogicControl.m();
        }
        self.myNet.m().get_tls_control()
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
        for logic in self.myNetworkLogics.iter() {
            logic.m().init(self.myDetectorBuilder.m());
        }
        for logic in self.myAdditionalLogics.iter() {
            logic.m().init(self.myDetectorBuilder.m());
        }
        // delay parameter loading until initialization
        for (logic, params) in self.myLogicParams.iter() {
            logic.m().base2.update_parameters(params);
        }
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
        if !self.myJunctions.is_null() {
            // MSJunctionControl::get: the null pointer when the id is unknown
            self.myJunctions.m().get(&id).copied().unwrap_or(Ptr::null())
        } else {
            Ptr::null()
        }
    }

    /// C++: `const MSSimpleTrafficLightLogic::Phases & NLJunctionControlBuilder::getLoadedPhases() const` [const]
    /// return the phases loaded so far (for error reporting and cleanup)
    // const MSSimpleTrafficLightLogic::Phases& getLoadedPhases() const {
    //         return myActivePhases;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_phases(&self) -> &MSTrafficLightLogic_Phases {
        &self.myActivePhases
    }

    /// C++: `void NLJunctionControlBuilder::netIsLoaded()`
    // void netIsLoaded() {
    //         myNetIsLoaded = true;
    //     }
    #[allow(unused_variables)]
    pub fn net_is_loaded(&mut self) {
        self.myNetIsLoaded = true;
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
        leak_box(MSNoLogicJunction::new(&self.myActiveID, self.myType, &self.myPosition, &self.myShape, &self.myActiveName, self.myActiveIncomingLanes.clone(), self.myActiveInternalLanes.clone())).cast::<MSJunction>()
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
        leak_box(MSRightOfWayJunction::new(&self.myActiveID, self.myType, &self.myPosition, &self.myShape, &self.myActiveName, self.myActiveIncomingLanes.clone(), self.myActiveInternalLanes.clone(), logic)).cast::<MSJunction>()
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
        // build the junction
        leak_box(MSInternalJunction::new(&self.myActiveID, self.myType, &self.myPosition, &self.myShape, self.myActiveIncomingLanes.clone(), self.myActiveInternalLanes.clone())).cast::<MSJunction>()
    }

    /// C++: `void NLJunctionControlBuilder::NLJunctionControlBuilder(const NLJunctionControlBuilder &)` [private]
    /// invalidated copy operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLJunctionControlBuilder) -> Self {
        panic!("NLJunctionControlBuilder copy constructor is private and never defined in C++")
    }

    /// C++: `NLJunctionControlBuilder & NLJunctionControlBuilder::operator=(const NLJunctionControlBuilder &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLJunctionControlBuilder) -> &mut NLJunctionControlBuilder {
        panic!("NLJunctionControlBuilder::operator= is private and never defined in C++")
    }

}

