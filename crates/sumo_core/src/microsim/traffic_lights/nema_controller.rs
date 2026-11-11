//! Port of microsim/traffic_lights/NEMAController.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `LightState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum LightState {
    RedXfer = 0,
    Red = 1,
    Yellow = 2,
    Green = 3,
    GreenXfer = 4,
    GreenRest = 5,
}

/// C++ enum `NEMALogic::controllerType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum NEMALogic_controllerType {
    Type170 = 0,
    TS2 = 1,
}

/// C++ typedef `std::map<MSE2Collector *, MSLane *, ComparatorIdLess> NEMALogic::DetectorLaneMap`
pub type NEMALogic_DetectorLaneMap = BTreeMap<Ptr<MSE2Collector>, Ptr<MSLane>>;

/// C++ typedef `std::map<MSLane *, MSE2Collector *> NEMALogic::LaneDetectorMap`
pub type NEMALogic_LaneDetectorMap = BTreeMap<Ptr<MSLane>, Ptr<MSE2Collector>>;

/// C++ typedef `NEMAPhase * NEMALogic::PhasePtr`
pub type NEMALogic_PhasePtr = Ptr<NEMAPhase>;

/// C++ typedef `NEMAPhase * NEMAPhase::PhasePtr`
pub type NEMAPhase_PhasePtr = Ptr<NEMAPhase>;

/// C++ typedef `NEMAPhase * PhaseTransitionLogic::PhasePtr`
pub type PhaseTransitionLogic_PhasePtr = Ptr<NEMAPhase>;

/// C++ typedef `std::vector<transitionInfo> NEMALogic::TransitionPairs`
pub type NEMALogic_TransitionPairs = Vec<NEMALogic_transitionInfo>;

/// C++ typedef `std::vector<std::vector<DetectorInfo *>> NEMALogic::detectorMap`
pub type NEMALogic_detectorMap = Vec<Vec<Ptr<NEMALogic_DetectorInfo>>>;

/// C++ class `NEMALogic::DetectorInfo` (microsim/traffic_lights/NEMAController.h)
pub struct NEMALogic_DetectorInfo {
    pub det: Ptr<MSE2Collector>,
    pub lastGreenTime: SUMOTime,
    pub servedPhase: Vec<bool>,
}

impl NEMALogic_DetectorInfo {
    /// C++: `void DetectorInfo::DetectorInfo(MSE2Collector *, int)`
    // DetectorInfo(MSE2Collector* _det, int numPhases) :
    //             det(_det),
    //             servedPhase(numPhases, false)
    //         {}
    #[allow(unused_variables)]
    pub fn new(_det: Ptr<MSE2Collector>, numPhases: i32) -> Self {
        todo!("port DetectorInfo::DetectorInfo")
    }

}

/// A NEMA (adaptive) traffic light logic based on E2Detector
/// C++ class `NEMALogic` (microsim/traffic_lights/NEMAController.h)
pub struct NEMALogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    /// constant for storing the priority order for light heads. Iterates left to right and stops when finds a match.
    pub lightHeadPriority: String,
    /// whether the controller is in coordinated mode or not
    pub coordinateMode: bool,
    /// a store of the coordinated phase objects. Only used meaningfully when the controller is in coordinated mode
    pub coordinatePhaseObjs: Unmapped /* PhasePtr[2] */,
    /// flag to keep track of whether a timing change has been requested via traci
    pub queuedTraciChanges: bool,
    /// the controller's offset
    pub offset: SUMOTime,
    /// the next offset to implement
    pub myNextOffset: SUMOTime,
    /// the coordinated cycle length
    pub myCycleLength: SUMOTime,
    /// the next cycle length (set by traci)
    pub myNextCycleLength: SUMOTime,
    /// stores the simulation time to make it easily accessible
    pub simTime: SUMOTime,
    /// stores controllers # of rings
    pub myNumberRings: i32,
    /// stores the length of phase string for the controller "GGrrrrs" = 6. Must be the same length for all phases
    pub myPhaseStrLen: i32,
    /// variable to store the active phases
    pub myActivePhaseObjs: Unmapped /* PhasePtr[2] */,
    /// a vector that stores a pointer to the instantiated NEMAPhase objects
    pub myPhaseObjs: Vec<NEMALogic_PhasePtr>,
    /// an array to store the phases located at a barrier for each ring
    pub defaultBarrierPhases: Unmapped /* PhasePtr[2][2] */,
    pub myDetectorForPhase: NEMALogic_detectorMap,
    /// storing the detector info in a vector
    pub myDetectorInfoVector: Vec<NEMALogic_DetectorInfo>,
    /// A map from lanes to detectors
    pub myLaneDetectorMap: NEMALogic_LaneDetectorMap,
    /// A map from lanes names to phases
    pub myLanePhaseMap: BTreeMap<String, i32>,
    /// A map from detectors to lanes
    pub myDetectorLaneMap: NEMALogic_DetectorLaneMap,
    /// store the generated detector length
    pub myDetectorLength: f64,
    /// store the left turn lane detestor length
    pub myDetectorLengthLeftTurnLane: f64,
    /// Whether the detectors shall be shown in the GUI
    pub myShowDetectors: bool,
    /// The output file for generated detectors
    pub myFile: String,
    /// The frequency for aggregating detector output
    pub myFreq: SUMOTime,
    /// Whether detector output separates by vType
    pub myVehicleTypes: String,
    pub rings: Vec<Vec<i32>>,
    pub phase2ControllerLanesMap: BTreeMap<i32, Vec<String>>,
    pub fixForceOff: bool,
    pub cycleRefPoint: SUMOTime,
    pub whetherOutputState: bool,
    pub ignoreErrors: bool,
    pub myControllerType: NEMALogic_controllerType,
    /// virtual phase that holds the current state
    pub myPhase: MSPhaseDefinition,
}

impl std::ops::Deref for NEMALogic {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for NEMALogic {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

impl NEMALogic {
    /// C++: `void NEMALogic::NEMALogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const MSSimpleTrafficLightLogic::Phases &, int, SUMOTime, const std::map<std::string, std::string> &, const std::string &)`
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameter: &BTreeMap<String, String>, basePath: &str) -> Self {
        todo!("port NEMALogic::NEMALogic")
    }

    /// C++: `void NEMALogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with information about incoming lanes
    // void
    // NEMALogic::init(NLDetectorBuilder& nb) {
    //
    //     // TODO: Create a parameter for this
    //     cycleRefPoint = TIME2STEPS(0);
    //
    //     std::string barriers = getParameter("barrierPhases", "");
    //     std::string coordinates = getParameter("coordinatePhases", getParameter("barrier2Phases", ""));
    //     std::string ring1 = getParameter("ring1", "");
    //     std::string ring2 = getParameter("ring2", "");
    //
    //     fixForceOff = StringUtils::toBool(getParameter("fixForceOff", "false"));
    //     offset = myOffset;
    //     myNextOffset = myOffset;
    //     whetherOutputState = StringUtils::toBool(getParameter("whetherOutputState", "false"));
    //     coordinateMode = StringUtils::toBool(getParameter("coordinate-mode", "false"));
    //
    //     // set the queued traci changes to false
    //     queuedTraciChanges = false;
    //
    //     //missing parameter error
    //     error_handle_not_set(ring1, "ring1");
    //     error_handle_not_set(ring2, "ring2");
    //     error_handle_not_set(barriers, "barrierPhases");
    //     error_handle_not_set(coordinates, "barrier2Phases or coordinatePhases");
    //
    //     //print to check
    // #ifdef DEBUG_NEMA
    //     std::cout << "JunctionID = " << myID << std::endl;
    //     std::cout << "All parameters after calling constructor are: " << std::endl;
    //     std::cout << "myDetectorLength = " << myDetectorLength << std::endl;
    //     std::cout << "cycleLength = " << STEPS2TIME(myCycleLength) << std::endl;
    //     std::cout << "ring1 = " << ring1 << std::endl;
    //     std::cout << "ring2 = " << ring2 << std::endl;
    //     std::cout << "barriers = " << barriers << std::endl;
    //     std::cout << "coordinates = " << coordinates << std::endl;
    //     std::cout << "offset = " << offset << std::endl;
    //     std::cout << "cycleSecond = " << getTimeInCycle() << std::endl;
    //     std::cout << "whetherOutputState = " << whetherOutputState << std::endl;
    //     std::cout << "myShowDetectors = " << myShowDetectors << std::endl;
    //     std::cout << "coordinateMode = " << coordinateMode << std::endl;
    //     std::cout << "fixForceOff = " << fixForceOff << std::endl;
    //     std::cout << "You reach the end of constructor" << std::endl;
    //     std::cout << "****************************************\n";
    // #endif
    //     // Construct the NEMA specific timing data types and initial phases
    //     constructTimingAndPhaseDefs(barriers, coordinates, ring1, ring2);
    //
    //     //init the traffic light
    //     MSTrafficLightLogic::init(nb);
    //     assert(myLanes.size() > 0);
    //     //iterate through the lanes and build one E2 detector for each lane associated with the traffic light control junction
    //     for (const LaneVector& lanes : myLanes) {
    //         for (MSLane* const lane : lanes) {
    //             //decide the detector length
    //             double detector_length = 0;
    //             if (isLeftTurnLane(lane)) {
    //                 detector_length = myDetectorLengthLeftTurnLane;
    //             } else {
    //                 detector_length = myDetectorLength;
    //             }
    //             if (noVehicles(lane->getPermissions())) {
    //                 // do not build detectors on green verges or sidewalks
    //                 continue;
    //             }
    //             // Build detector and register them in the detector control
    //             if (myLaneDetectorMap.find(lane) == myLaneDetectorMap.end()) {
    //                 MSE2Collector* det = nullptr;
    //                 const std::string customID = getParameter(lane->getID());
    //                 if (customID != "") {
    //                     det = dynamic_cast<MSE2Collector*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).get(customID));
    //                     if (det == nullptr) {
    //                         throw ProcessError("Unknown laneAreaDetector '" + customID + "' given as custom detector for NEMA tlLogic '" + getID() + "', program '" + getProgramID() + ".");
    //                     }
    //                     //set the detector to be visible in gui
    //                     det->setVisible(myShowDetectors);
    //                 } else {
    //                     int phaseNumber = 0;
    //                     if (myLanePhaseMap.find(lane->getID()) != myLanePhaseMap.end()) {
    //                         phaseNumber = myLanePhaseMap.find(lane->getID())->second;
    //                     }
    //                     int index = lane->getIndex();
    //                     std::string id = myID + "_" + myProgramID + "_D" + toString(phaseNumber) + "." + toString(index);
    //                     while (MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).get(id) != nullptr) {
    //                         index++;
    //                         id = myID + "_" + myProgramID + "_D" + toString(phaseNumber) + "." + toString(index);
    //                     }
    //                     //createE2Detector() method will lead to bad detector showing in sumo-gui
    //                     //so it is better to use build2Detector() rather than createE2Detector()
    //                     nb.buildE2Detector(id, //detectorID
    //                                        lane, //lane to build this detector
    //                                        INVALID_POSITION, // set the detector location by end point and length, so this one is set to invalue value so this parameter can be passed
    //                                        lane->getLength(), // set the end position of the detector at the end of the lane, which is right at the position of stop bar of a junction
    //                                        detector_length, //detector length
    //                                        myFile, // detector information output file
    //                                        myFreq, // detector reading interval
    //                                        0, // time-based threshold that describes how much time has to pass until a vehicle is considered as halting
    //                                        0, // speed threshold as halting
    //                                        0, // minimum dist to the next standing vehicle to make this vehicle count as a participant to the jam
    //                                        "",
    //                                        myVehicleTypes, //vehicle types to consider, if it is empty, meaning consider all types of vehicles
    //                                        "", // nextEdges (no filtering by vehicle route)
    //                                        (int)PersonMode::NONE, // detector vehicles, not persons
    //                                        true, // whether to give some slack on positioning
    //                                        myShowDetectors, // whether to show detectors in sumo-gui
    //                                        nullptr, //traffic light that triggers aggregation when switching
    //                                        nullptr); // outgoing lane that associated with the traffic light
    //
    //                     //get the detector to be used in the lane detector map loading
    //                     det = dynamic_cast<MSE2Collector*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_LANE_AREA_DETECTOR).get(id));
    //                 }
    //
    //                 //map the detector to lane and lane to detector
    //                 myLaneDetectorMap[lane] = det;
    //                 myDetectorLaneMap[det] = lane;
    //                 myDetectorInfoVector.push_back(DetectorInfo(det, (int)myPhases.size()));
    //
    //             }
    //         }
    //     }
    //     for (auto item : phase2ControllerLanesMap) {
    //         int NEMAPhaseIndex = item.first;
    //         std::vector<std::string> laneIDs = item.second;
    //         std::vector<MSE2Collector*> detectors;
    //         MSE2Collector* detector = nullptr;
    //         for (std::string laneID : laneIDs) {
    //             MSLane* lane = MSLane::dictionary(laneID);
    //             detector = myLaneDetectorMap[lane];
    //             detectors.push_back(detector);
    //         }
    //         // have to try this on both rings, because of the case where both rings have the same phase
    //         // See Basic NEMA test
    //         for (int i = 0; i < 2; i++) {
    //             if (vectorContainsPhase(rings[i], NEMAPhaseIndex)) {
    //                 getPhaseObj(NEMAPhaseIndex, i)->setDetectors(detectors);
    //             }
    //         }
    //     }
    //
    //     //Do not delete. SUMO traffic logic check.
    //     //SUMO check begin
    //     const SVCPermissions motorized = ~(SVC_PEDESTRIAN | SVC_BICYCLE);
    //     std::map<int, std::set<MSE2Collector*>> linkToDetectors;
    //     std::set<int> actuatedLinks;
    //
    //     const int numLinks = (int)myLinks.size();
    //     std::vector<bool> neverMajor(numLinks, true);
    //     for (const MSPhaseDefinition* phase : myPhases) {
    //         const std::string& state = phase->getState();
    //         for (int i = 0; i < numLinks; i++) {
    //             if (state[i] == LINKSTATE_TL_GREEN_MAJOR) {
    //                 neverMajor[i] = false;
    //             }
    //         }
    //     }
    //     std::vector<bool> oneLane(numLinks, false);
    //     for (int i = 0; i < numLinks; i++) {
    //         for (MSLane* lane : getLanesAt(i)) {
    //             int numMotorized = 0;
    //             for (MSLane* l : lane->getEdge().getLanes()) {
    //                 if ((l->getPermissions() & motorized) != 0) {
    //                     numMotorized++;
    //                 }
    //             }
    //             if (numMotorized == 1) {
    //                 oneLane[i] = true;
    //                 break;
    //             }
    //         }
    //     }
    //
    //     for (const MSPhaseDefinition* phase : myPhases) {
    //         const int phaseIndex = (int)myDetectorForPhase.size();
    //         std::set<MSE2Collector*> detectors;
    //         if (phase->isActuated()) {
    //             const std::string& state = phase->getState();
    //             std::set<int> greenLinks;
    //             std::map<MSE2Collector*, std::set<int>> detectorLinks;
    //
    //             for (int i = 0; i < numLinks; i++) {
    //                 if (state[i] == LINKSTATE_TL_GREEN_MAJOR
    //                         || (state[i] == LINKSTATE_TL_GREEN_MINOR
    //                             && ((neverMajor[i]  // check1a
    //                                  && hasMajor(state, getLanesAt(i))) // check1b
    //                                 || oneLane[i])) // check1c
    //                    ) {
    //                     greenLinks.insert(i);
    //                     actuatedLinks.insert(i);
    //                 }
    //
    //                 for (MSLane* lane : getLanesAt(i)) {
    //                     if (myLaneDetectorMap.count(lane) != 0) {
    //                         detectorLinks[myLaneDetectorMap[lane]].insert(i);
    //                     }
    //                 }
    //             }
    //             for (auto& item : detectorLinks) {
    //                 MSE2Collector* det = item.first;
    //                 MSLane* detectorLane = myDetectorLaneMap[det];
    //                 bool usable = true;
    //                 // check 1
    //                 for (int j : item.second) {
    //                     if (greenLinks.count(j) == 0) {
    //                         usable = false;
    //                     }
    //                 }
    //
    //                 //check 2
    //                 if (usable) {
    //                     for (MSLink* link : detectorLane->getLinkCont()) {
    //                         MSLane* next = link->getLane();
    //                         if (myLaneDetectorMap.count(next) != 0) {
    //                             MSE2Collector* nextDet = myLaneDetectorMap[next];
    //                             for (int j : detectorLinks[nextDet]) {
    //                                 if (greenLinks.count(j) == 0) {
    //                                     usable = false;
    //                                     break;
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //
    //                 if (usable) {
    //                     detectors.insert(item.first);
    //                     for (int j : item.second) {
    //                         linkToDetectors[j].insert(item.first);
    //                     }
    //                 }
    //             }
    //             if (detectors.size() == 0) {
    //                 WRITE_WARNINGF(TL("At NEMA tlLogic '%', actuated phase % has no controlling detector"), getID(), toString(phaseIndex));
    //             }
    //         }
    //         std::vector<DetectorInfo*> detectorInfos;
    //         myDetectorForPhase.push_back(detectorInfos);
    //         for (MSE2Collector* det : detectors) {
    //             for (DetectorInfo& detInfo : myDetectorInfoVector) {
    //                 if (detInfo.det == det) {
    //                     myDetectorForPhase.back().push_back(&detInfo);
    //                     detInfo.servedPhase[phaseIndex] = true;
    //                 }
    //             }
    //         }
    //     }
    //
    //     for (int i : actuatedLinks) {
    //         if (linkToDetectors[i].size() == 0 && myLinks[i].size() > 0
    //                 && (myLinks[i].front()->getLaneBefore()->getPermissions() & motorized) != 0) {
    //             WRITE_WARNINGF(TL("At NEMA tlLogic '%, linkIndex % has no controlling detector"), getID(), toString(i));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        todo!("port NEMALogic::init")
    }

    /// C++: `void NEMALogic::~NEMALogic() noexcept` [virtual]
    /// Destructor
    // NEMALogic::~NEMALogic() {
    //     // delete the phase objects
    //     for (auto p : myPhaseObjs) {
    //         delete p;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NEMALogic::~NEMALogic")
    }

    /// C++: `SUMOTime NEMALogic::trySwitch()` [virtual]
    /// overrides the MSSimpleTrafficLightLogic trySwitch method
    // SUMOTime
    // NEMALogic::trySwitch() {
    // #ifdef DEBUG_NEMA_SWITCH
    //     std::cout << SIMTIME << " trySwitch tls=" << getID() << "\n";
    // #endif
    //     PhaseTransitionLogic* nextPhases[2] = { nullptr, nullptr };
    //
    //     // update the internal time. This is a must. Could have just used a reference to the time
    //     setCurrentTime();
    //
    //     // Check the Detectors
    //     for (auto& p : myPhaseObjs) {
    //         p->checkMyDetectors();
    //     }
    //
    //     // Update the timing parameters
    //     for (const auto& p : myActivePhaseObjs) {
    //         p->update(this);
    //     }
    //
    //     // Calculate the Next Phases, but only if atleast one of them is ready to transition
    //     if (myActivePhaseObjs[0]->readyToSwitch || myActivePhaseObjs[1]->readyToSwitch) {
    //         TransitionPairs transitions;
    //         // set the next phases by reference
    //         getNextPhases(transitions);
    //
    //         // Sort the next phases by distance and select the closest.
    //         // TODO: Is there a way to avoid this sort? The transitions are already sorted by distance prior
    //         // to picking the valid ones
    //         if (transitions.size() > 1) {
    //             std::sort(transitions.begin(), transitions.end(),
    //             [](const transitionInfo & i, const transitionInfo & j) {
    //                 return i.distance < j.distance;
    //             });
    //         }
    //
    //         // Set the Next Phases = to the transition with least combined distance
    //         nextPhases[0] = transitions.front().p1;
    //         nextPhases[1] = transitions.front().p2;
    //
    //         // Try the exit logic. This doesn't necessarily mean that the phase will exit,
    //         // as it could go into green rest or green transfer, but this is considered an "exit"
    //         for (const auto& p : myActivePhaseObjs) {
    //             if (p->readyToSwitch) {
    //                 p->exit(this, nextPhases);
    //             }
    //         }
    //
    //         // This is the only time when something might have happened, so we update the phase strings here
    //         std::string newState = composeLightString();
    //         if (newState != myPhase.getState()) {
    //             myPhase.setState(newState);
    //             myPhase.setName(toString(myActivePhaseObjs[0]->phaseName) + "+" + toString(myActivePhaseObjs[1]->phaseName));
    //             // ensure that SwitchCommand::execute notices a change
    //             myStep = 1 - myStep;
    //
    //         }
    //     }
    //
    //     // clear the phases' detectors
    //     for (auto& p : myPhaseObjs) {
    //         p->clearMyDetectors();
    //     }
    //
    //
    // #ifdef FUZZ_TESTING
    //     // Basic Assertion to ensure that the Barrier is not crossed
    //     assert(myActivePhaseObjs[0]->barrierNum == myActivePhaseObjs[1]->barrierNum);
    // #endif
    //
    //     // return the simulation timestep, as this controller must be checked every simulation step
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        todo!("port NEMALogic::trySwitch")
    }

    /// C++: `const MSPhaseDefinition & NEMALogic::getCurrentPhaseDef() const` [const, virtual]
    /// Returns myPhase, which doesn't correspond to a NEMA phase, but rather the composite light string
    // const MSPhaseDefinition&
    // NEMALogic::getCurrentPhaseDef() const {
    //     return myPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        todo!("port NEMALogic::getCurrentPhaseDef")
    }

    /// C++: `void NEMALogic::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // NEMALogic::resetLastSwitch(SUMOTime t) {
    //     myPhase.myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        todo!("port NEMALogic::resetLastSwitch")
    }

    /// C++: `void NEMALogic::activateProgram()` [virtual]
    // void
    // NEMALogic::activateProgram() {
    //     MSTrafficLightLogic::activateProgram();
    //     for (auto& item : myLaneDetectorMap) {
    //         item.second->setVisible(true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_program(&mut self) {
        todo!("port NEMALogic::activateProgram")
    }

    /// C++: `void NEMALogic::deactivateProgram()` [virtual]
    // void
    // NEMALogic::deactivateProgram() {
    //     MSTrafficLightLogic::deactivateProgram();
    //     for (auto& item : myLaneDetectorMap) {
    //         item.second->setVisible(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_program(&mut self) {
        todo!("port NEMALogic::deactivateProgram")
    }

    /// C++: `bool NEMALogic::showDetectors() const` [const]
    // bool showDetectors() const {
    //         return myShowDetectors;
    //     }
    #[allow(unused_variables)]
    pub fn show_detectors(&self) -> bool {
        todo!("port NEMALogic::showDetectors")
    }

    /// C++: `void NEMALogic::setShowDetectors(bool)`
    // void
    // NEMALogic::setShowDetectors(bool show) {
    //     myShowDetectors = show;
    //     for (auto& item : myLaneDetectorMap) {
    //         item.second->setVisible(myShowDetectors);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_show_detectors(&mut self, show: bool) {
        todo!("port NEMALogic::setShowDetectors")
    }

    /// C++: `std::map<std::string, double> NEMALogic::getDetectorStates() const` [const, virtual]
    /// retrieve all detectors used by this program
    // std::map<std::string, double>
    // NEMALogic::getDetectorStates() const {
    //     std::map<std::string, double> result;
    //     for (auto item : myDetectorLaneMap) {
    //         result[item.first->getID()] = item.first->getCurrentVehicleNumber();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_states(&self) -> BTreeMap<String, f64> {
        todo!("port NEMALogic::getDetectorStates")
    }

    /// C++: `void NEMALogic::getNextPhases(TransitionPairs &)`
    /// extends the transitions vector with valid Transitions given the current traffic light state
    // void
    // NEMALogic::getNextPhases(TransitionPairs& transitions) {
    //     std::vector<std::vector<PhaseTransitionLogic* >> potentialPhases;
    //
    //     // Get a vector of each phases' potential transitions
    //     for (const auto& p : myActivePhaseObjs) {
    //         potentialPhases.push_back(p->trySwitch(this));
    //     }
    //
    //     // Loop through all combination of transitions, keeping only the valid ones and filling in the gaps where necessary
    //     for (const auto& r1_t : potentialPhases[0]) {
    //         for (const auto& r2_t : potentialPhases[1]) {
    //             // if both transitions go to the same barrier then we are good
    //             if (r1_t->getToPhase()->barrierNum == r2_t->getToPhase()->barrierNum) {
    //                 transitions.push_back({ r1_t, r2_t, (float)(r1_t->getDistance(r2_t) + r2_t->getDistance(r1_t)) / 2 });
    //             } else {
    //                 // If the rings are different, add a choice where one of them is the default choice for whatever ring it is
    //                 // create two choices, one for each of the phase as they are on  different rings
    //                 if (r1_t->getFromPhase()->readyToSwitch) {
    //                     // get the r2 default
    //                     PhaseTransitionLogic* r2_t_temp = getDefaultTransition(r2_t, r1_t);
    //                     // only add it if it does not cross a barrier!
    //                     if (r2_t_temp->getToPhase()->barrierNum == r1_t->getToPhase()->barrierNum) {
    //                         transitions.push_back({ r1_t, r2_t_temp, (float)(r2_t_temp->getDistance(r1_t) + r1_t->getDistance(r2_t_temp)) / 2 });
    //                     }
    //                 }
    //                 if (r2_t->getFromPhase()->readyToSwitch) {
    //                     // R1 default
    //                     PhaseTransitionLogic* r1_t_temp = getDefaultTransition(r1_t, r2_t);
    //                     // only add it if it does not cross a barrier!
    //                     if (r1_t_temp->getToPhase()->barrierNum == r2_t->getToPhase()->barrierNum) {
    //                         transitions.push_back({ r1_t_temp, r2_t, (float)(r2_t->getDistance(r1_t_temp) + r1_t_temp->getDistance(r2_t)) / 2 });
    //                     }
    //                 }
    //                 // If the distances are <= 1, this means that this is the shortest transition possible
    //                 // and we can return early without considering any other options
    //                 if (!transitions.empty()) {
    //                     if (transitions.back().distance < 1) {
    //                         return;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_next_phases(&mut self, transitions: &mut NEMALogic_TransitionPairs) {
        todo!("port NEMALogic::getNextPhases")
    }

    /// C++: `SUMOTime NEMALogic::ModeCycle(SUMOTime, SUMOTime)`
    /// Calculates the modulus a / b, normally used to calculate the cycle time between two times. Usage example: ModeCycle(t1 - t2, cycleLength)
    // SUMOTime
    // NEMALogic::ModeCycle(SUMOTime a, SUMOTime b) {
    //     SUMOTime c = a - b;
    //     while (c >= b) {
    //         c = c - b;
    //     }
    //     while (c < 0) {
    //         c += b;
    //     }
    //     return c;
    // }
    #[allow(unused_variables)]
    pub fn mode_cycle(&mut self, a: SUMOTime, b: SUMOTime) -> SUMOTime {
        todo!("port NEMALogic::ModeCycle")
    }

    /// C++: `void NEMALogic::getLaneInfoFromNEMAState(std::string, StringVector &, IntVector &)`
    /// returns the IDs of the phase's controlled lanes. Found by looking for the "G" in the light state string
    // void
    // NEMALogic::getLaneInfoFromNEMAState(std::string state, StringVector& laneIDs, IntVector& stateIndex) {
    //     std::set<std::string> output;
    //     for (int i = 0; i < (int)myLinks.size(); i++) {
    //         if (myLinks[i].empty()) {
    //             // unused index
    //             continue;
    //         }
    //         char ch = state[i];
    //         // if the ch is 'G', it means that the phase is controlling this lane
    //         if (ch == 'G') {
    //             stateIndex.push_back(i);
    //             for (auto link : myLinks[i]) {
    //                 const MSLane* incoming = link->getLaneBefore();
    //                 if (incoming->isNormal()) {
    //                     laneIDs.push_back(incoming->getID());
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_lane_info_from_nema_state(&mut self, state: String, laneIDs: &mut StringVector, stateIndex: &mut IntVector) {
        todo!("port NEMALogic::getLaneInfoFromNEMAState")
    }

    /// C++: `void NEMALogic::setNewMaxGreens(std::vector<double>)`
    /// Set the max green of all phases.
    // void
    // NEMALogic::setNewMaxGreens(std::vector<double> newMaxGreens) {
    //     for (auto& p : myPhaseObjs) {
    //         if (newMaxGreens[p->phaseName - 1] > 0) {
    //             // set the phase's nextMaxDuration. This will be implemented when implementTraciChanges is called
    //             p->nextMaxDuration = TIME2STEPS(newMaxGreens[p->phaseName - 1]);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_new_max_greens(&mut self, newMaxGreens: Vec<f64>) {
        todo!("port NEMALogic::setNewMaxGreens")
    }

    /// C++: `void NEMALogic::setNewSplits(std::vector<double>)`
    /// Set the new splits of all phases
    // void
    // NEMALogic::setNewSplits(std::vector<double> newSplits) {
    //     assert(newSplits.size() == 8);
    //     for (auto& p : myPhaseObjs) {
    //         if (newSplits[p->phaseName - 1] > 0) {
    //             // set the phase's nextMaxDuration. This will be implemented when implementTraciChanges is called
    //             p->nextMaxDuration = TIME2STEPS(newSplits[p->phaseName - 1]) - p->yellow - p->red;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_new_splits(&mut self, newSplits: Vec<f64>) {
        todo!("port NEMALogic::setNewSplits")
    }

    /// C++: `void NEMALogic::setNewCycleLength(double)`
    /// set the new cycle length for the controller
    // void
    // NEMALogic::setNewCycleLength(double newCycleLength) {
    //     // set the controller's next cycle length. This will be implemented when implementTraciChanges is called
    //     myNextCycleLength = TIME2STEPS(newCycleLength);
    // }
    #[allow(unused_variables)]
    pub fn set_new_cycle_length(&mut self, newCycleLength: f64) {
        todo!("port NEMALogic::setNewCycleLength")
    }

    /// C++: `void NEMALogic::setNewOffset(double)`
    /// Set the new offset for the controller
    // void
    // NEMALogic::setNewOffset(double newOffset) {
    //     // set the controller's offset. This will be implemented when implementTraciChanges is called
    //     myNextOffset = TIME2STEPS(newOffset);
    // }
    #[allow(unused_variables)]
    pub fn set_new_offset(&mut self, newOffset: f64) {
        todo!("port NEMALogic::setNewOffset")
    }

    /// C++: `SUMOTime NEMALogic::getCurrentCycleLength()`
    /// Get the current cycle length
    // SUMOTime getCurrentCycleLength() {
    //         return myCycleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_cycle_length(&mut self) -> SUMOTime {
        todo!("port NEMALogic::getCurrentCycleLength")
    }

    /// C++: `void NEMALogic::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter. Parameters prefixed with 'NEMA.' control functionality
    // void
    // NEMALogic::setParameter(const std::string& key, const std::string& value) {
    //     queuedTraciChanges = true;
    //     if (StringUtils::startsWith(key, "NEMA.")) {
    //         if (key == "NEMA.splits" || key == "NEMA.maxGreens") {
    //             //splits="2.0 3.0 4.0 5.0 2.0 3.0 4.0 5.0"
    //             const std::vector<std::string>& tmp = StringTokenizer(value).getVector();
    //             if (tmp.size() != 8) {
    //                 queuedTraciChanges = false;
    //                 throw InvalidArgument("Parameter '" + key + "' for NEMA controller '" + getID() + "' requires 8 space or comma separated values");
    //             }
    //             std::vector<double> timing;
    //             for (const std::string& s : tmp) {
    //                 timing.push_back(StringUtils::toDouble(s));
    //             }
    //             if (key == "NEMA.maxGreens") {
    //                 setNewMaxGreens(timing);
    //             } else {
    //                 setNewSplits(timing);
    //             }
    //         } else if (key == "NEMA.cycleLength") {
    //             setNewCycleLength(StringUtils::toDouble(value));
    //         } else if (key == "NEMA.offset") {
    //             setNewOffset(StringUtils::toDouble(value));
    //         } else {
    //             queuedTraciChanges = false;
    //             throw InvalidArgument("Unsupported parameter '" + key + "' for NEMA controller '" + getID() + "'");
    //         }
    //     }
    //     Parameterised::setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port NEMALogic::setParameter")
    }

    /// C++: `const std::string NEMALogic::getParameter(const std::string &, const std::string) const` [const, virtual]
    /// try to get the value of the given parameter. Parameters prefixed with 'NEMA.' control functionality
    // const std::string
    // NEMALogic::getParameter(const std::string& key, const std::string defaultValue) const {
    //     if (StringUtils::startsWith(key, "NEMA.")) {
    //         if (key == "NEMA.phaseCall") {
    //             int activeCalls[8] = { 0 };
    //             for (const auto p : myPhaseObjs) {
    //                 // This handles the case when the controller has multiple of the same phase call
    //                 if (!activeCalls[p->phaseName - 1]) {
    //                     activeCalls[p->phaseName - 1] = 1 * p->lastDetectActive;
    //                 }
    //             }
    //             std::string outStr = "";
    //             for (int i = 0; i < 8; i++) {
    //                 outStr += std::to_string(activeCalls[i]);
    //                 if (i < 7) {
    //                     outStr += ",";
    //                 }
    //             }
    //             return outStr;
    //         } else {
    //             throw InvalidArgument("Unsupported parameter '" + key + "' for NEMA controller '" + getID() + "'");
    //         }
    //     } else {
    //         return Parameterised::getParameter(key, defaultValue);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str, defaultValue: String) -> String {
        todo!("port NEMALogic::getParameter")
    }

    /// C++: `SUMOTime NEMALogic::getCurrentTime() const` [const]
    /// Wrapper Function to Simplify Accessing Time
    // inline SUMOTime getCurrentTime(void) const {
    //         return simTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_time(&self) -> SUMOTime {
        todo!("port NEMALogic::getCurrentTime")
    }

    /// C++: `SUMOTime NEMALogic::getTimeInCycle() const` [const]
    /// override Function to Simplify Accessing Offset Cycle Time
    // inline SUMOTime getTimeInCycle() const {
    //         return (simTime - cycleRefPoint - offset) % myCycleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_in_cycle(&self) -> SUMOTime {
        todo!("port NEMALogic::getTimeInCycle")
    }

    /// C++: `void NEMALogic::setActivePhase(PhasePtr)`
    /// set the active phase
    // void
    // NEMALogic::setActivePhase(PhasePtr phase) {
    //     myActivePhaseObjs[phase->ringNum] = phase;
    // }
    #[allow(unused_variables)]
    pub fn set_active_phase(&mut self, phase: NEMALogic_PhasePtr) {
        todo!("port NEMALogic::setActivePhase")
    }

    /// C++: `PhasePtr NEMALogic::getActivePhase(int)`
    /// Get the Active Phase object for a specified ring
    // inline PhasePtr getActivePhase(int ringNum) {
    //         return myActivePhaseObjs[ringNum];
    //     }
    #[allow(unused_variables)]
    pub fn get_active_phase(&mut self, ringNum: i32) -> NEMALogic_PhasePtr {
        todo!("port NEMALogic::getActivePhase")
    }

    /// C++: `std::vector<PhasePtr> NEMALogic::getPhasesByRing(int)`
    /// get all phases for a given ring
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases_by_ring(&mut self, ringNum: i32) -> Vec<NEMALogic_PhasePtr> {
        todo!("port NEMALogic::getPhasesByRing")
    }

    /// C++: `PhasePtr NEMALogic::getPhaseObj(int, int)`
    /// get the phase object matching the phaseNum If ringNum is passed, it will only search for the phase in the given ring
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phase_obj(&mut self, phaseNum: i32, ringNum: i32) -> NEMALogic_PhasePtr {
        todo!("port NEMALogic::getPhaseObj")
    }

    /// C++: `std::vector<PhasePtr> NEMALogic::getPhaseObjs()`
    /// get a vector of all phase objects
    // inline std::vector<PhasePtr> getPhaseObjs(void) {
    //         return myPhaseObjs;
    //     }
    #[allow(unused_variables)]
    pub fn get_phase_objs(&mut self) -> Vec<NEMALogic_PhasePtr> {
        todo!("port NEMALogic::getPhaseObjs")
    }

    /// C++: `int NEMALogic::measureRingDistance(int, int, int)`
    /// return the ring distance between two phases
    // int NEMALogic::measureRingDistance(int p1, int p2, int ringNum) {
    //     int length = (int)rings[ringNum].size();
    //     int d = 0;
    //     bool found = false;
    //     // Loop around the ring and keep track of the distance from p1 to p2
    //     for (int i = 0; i < (length * 2); i++) {
    //         if (rings[ringNum][i % length] > 0) {
    //             if (found) {
    //                 d++;
    //                 if (rings[ringNum][i % length] == p2) {
    //                     break;
    //                 }
    //             } else if (rings[ringNum][i % length] == p1) {
    //                 found = true;
    //             }
    //         }
    //     }
    //     assert(d > 0);
    //     return d;
    // }
    #[allow(unused_variables)]
    pub fn measure_ring_distance(&mut self, p1: i32, p2: i32, ringNum: i32) -> i32 {
        todo!("port NEMALogic::measureRingDistance")
    }

    /// C++: `bool NEMALogic::isType170() const` [const]
    /// checks if the controller is of type170
    // inline bool isType170(void) const {
    //         return myControllerType == Type170;
    //     }
    #[allow(unused_variables)]
    pub fn is_type170(&self) -> bool {
        todo!("port NEMALogic::isType170")
    }

    /// C++: `PhasePtr NEMALogic::getOtherPhase(PhasePtr)`
    /// Get the opposite active phase
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_other_phase(&mut self, p: NEMALogic_PhasePtr) -> NEMALogic_PhasePtr {
        todo!("port NEMALogic::getOtherPhase")
    }

    /// C++: `void NEMALogic::implementTraciChanges()`
    /// implement any pending traci changes This function is called once per cycle
    // void
    // NEMALogic::implementTraciChanges(void) {
    //     // Implement Traci Updates on the start of ring1 coordinated phase (rising edge of it turning green)
    //     if (queuedTraciChanges) {
    //         for (auto& p : myPhaseObjs) {
    //             p->maxDuration = p->nextMaxDuration;
    //         }
    //         offset = myNextOffset;
    //         myCycleLength = myNextCycleLength;
    //         // now that we have set the cycle length, offset and max duration, we need to update force off times
    //         calculateForceOffs();
    //         queuedTraciChanges = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn implement_traci_changes(&mut self) {
        todo!("port NEMALogic::implementTraciChanges")
    }

    /// C++: `void NEMALogic::setCurrentTime()` [protected]
    /// Set the simTime
    // inline void setCurrentTime(void) {
    //         simTime = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    #[allow(unused_variables)]
    pub fn set_current_time(&mut self) {
        todo!("port NEMALogic::setCurrentTime")
    }

    /// C++: `void NEMALogic::constructTimingAndPhaseDefs(std::string &, std::string &, std::string &, std::string &)` [protected]
    /// constructs phase using the configuration file
    // void
    // NEMALogic::constructTimingAndPhaseDefs(std::string& barriers, std::string& coordinates, std::string& ring1, std::string& ring2) {
    //
    //     // read in the barrier and coordinated phases from the XML
    //     std::vector<int> barrierPhases = readParaFromString(barriers);
    //     std::vector<int> coordinatePhases = readParaFromString(coordinates);
    //
    //     // create a {{}, {}} vector of phases
    //     rings.push_back(readParaFromString(ring1));
    //     rings.push_back(readParaFromString(ring2));
    //
    // #ifdef DEBUG_NEMA
    //     //print to check
    //     for (int i = 0; i < (int)rings.size(); i++) {
    //         int count = 0;
    //         std::cout << "Ring" << i + 1 << " includes phases: \t";
    //         for (auto j : rings[i]) {
    //             count++;
    //             std::cout << j << " ";
    //             if (count == 2 || count == 4) {
    //                 std::cout << " | ";
    //             }
    //         }
    //         std::cout << std::endl;
    //     }
    // #endif
    //
    //     // load the recalls, if they exist
    //     std::vector<int> VecMinRecall = readParaFromString(getParameter("minRecall", "1,2,3,4,5,6,7,8"));
    //     std::vector<int> VecMaxRecall = readParaFromString(getParameter("maxRecall", ""));
    //
    // #ifdef DEBUG_NEMA
    //     std::cout << "minRecall: ";
    //     for (int i = 0; i < 8; i++) {
    //         std::cout << vectorContainsPhase(VecMinRecall, i + 1) << '\t';
    //     }
    //     std::cout << std::endl;
    //
    //     std::cout << "maxRecall: ";
    //     for (int i = 0; i < 8; i++) {
    //         std::cout << vectorContainsPhase(VecMaxRecall, i + 1) << '\t';
    //     }
    //     std::cout << std::endl;
    // #endif
    //
    //     // loop through the rings and construct NEMAPhases.
    //     // This relies on the phase being in order in the rings parameter in the configuration file
    //     int ringNum = 0;
    //     int lastPhaseIter = 0;
    //     int phaseIter = 0;
    //     for (const auto& r : rings) {
    //         int ringIter = 0;
    //         lastPhaseIter = phaseIter;
    //         phaseIter = 0;
    //         for (const auto& p : r) {
    //             if (p != 0) {
    //                 // find the phase definition matching the phase integer
    //                 MSPhaseDefinition* tempPhase = nullptr;
    //                 for (const auto& pDef : myPhases) {
    //                     if (string2int(pDef->getName()) == p) {
    //                         tempPhase = pDef;
    //                         break;
    //                     }
    //                 }
    //                 // there must be a matching MSPhaseDefinition
    //                 if (tempPhase == nullptr) {
    //                     throw ProcessError("At traffic signal '" + myID + "' program '" + myProgramID + "' no phase named '" + toString(p) + "' was found");
    //                 }
    //
    //                 // create lane specific objects
    //                 std::string state = tempPhase->getState();
    //
    //                 // check that all phases have the same length. myPhaseStrLen is initially set to -1.
    //                 if (myPhaseStrLen < 0) {
    //                     myPhaseStrLen = (int)state.size();
    //                 } else if (myPhaseStrLen != (int)state.size()) {
    //                     throw ProcessError(TLF("At NEMA tlLogic '%', different sizes of NEMA phase states. Please check the NEMA XML", getID()));
    //                 }
    //
    //                 // get the lane-based info
    //                 StringVector laneIDs;
    //                 IntVector controlledStateIndexes;
    //                 getLaneInfoFromNEMAState(state, laneIDs, controlledStateIndexes);
    //
    //                 std::vector<std::string> laneIDs_vector;
    //                 for (std::string laneID : laneIDs) {
    //                     laneIDs_vector.push_back(laneID);
    //                     myLanePhaseMap[laneID] = p;
    //                 }
    //                 phase2ControllerLanesMap[p] = laneIDs_vector;
    //
    //                 // Create the Phase Object
    //                 // find if it is at a barrier
    //                 bool barrierPhase = vectorContainsPhase(barrierPhases, p) || vectorContainsPhase(coordinatePhases, p);
    //                 // is it a coordinate phase
    //                 bool coordinatePhase = vectorContainsPhase(coordinatePhases, p) && coordinateMode;
    //                 // is there a minimum or max recall
    //                 bool minRecall = vectorContainsPhase(VecMinRecall, p);
    //                 bool maxRecall = vectorContainsPhase(VecMaxRecall, p);
    //                 // A phase can "green rest" only if it has a recall and no other phases on that ring do OR if NO phases have a recall (unique case)
    //                 bool phaseGreenRest = ((VecMaxRecall.size() + VecMinRecall.size()) < 1);
    //                 if (!phaseGreenRest) {
    //                     bool recallActive = minRecall || maxRecall;
    //                     if (recallActive) {
    //                         for (const auto& pO : r) {
    //                             if (pO != p) {
    //                                 if (vectorContainsPhase(VecMinRecall, pO)
    //                                         || vectorContainsPhase(VecMaxRecall, pO)) {
    //                                     recallActive = false;
    //                                     break;
    //                                 }
    //                             }
    //                         }
    //                         // only set the green rest to true if I am the only phase on my ring with a recall
    //                         phaseGreenRest = recallActive;
    //                     }
    //                 }
    //                 // could add per-phase fixforceoff here
    //                 // barrierNum is either 0 or 1, depending on mainline side or sidestreet
    //                 int barrierNum = ringIter / 2;
    //
    //                 // now ready to create the phase
    //                 myPhaseObjs.push_back(
    //                     new NEMAPhase(p, barrierPhase, phaseGreenRest, coordinatePhase, minRecall, maxRecall, fixForceOff, barrierNum, ringNum, controlledStateIndexes, tempPhase)
    //                 );
    //
    //                 // Add a reference to the sequentially prior phase
    //                 if (phaseIter > 0) {
    //                     myPhaseObjs.back()->setSequentialPriorPhase(myPhaseObjs[lastPhaseIter + (phaseIter - 1)]);
    //                 }
    //                 phaseIter++;
    //             }
    //             ringIter++;
    //         }
    //         if (lastPhaseIter >= (int)myPhaseObjs.size()) {
    //             throw ProcessError("At traffic signal '" + myID + "', ring " + toString(ringNum + 1) + " contains only '0'");
    //         }
    //         // Set the first to point to the last, wrapping around the ring.
    //         myPhaseObjs[lastPhaseIter]->setSequentialPriorPhase(myPhaseObjs[lastPhaseIter + phaseIter - 1]);
    //         // index the ring counter
    //         ringNum++;
    //     }
    //
    //     //TODO: set the default phases. This could also be set using dual entry in future
    //     for (int i = 0; i < 2; i++) {
    //         // create the coordinate phase ptr
    //         coordinatePhaseObjs[i] = getPhaseObj(coordinatePhases[i], i);
    //         defaultBarrierPhases[i][coordinatePhaseObjs[i]->barrierNum] = coordinatePhaseObjs[i];
    //         // create the other barrier phase ptr
    //         PhasePtr b = getPhaseObj(barrierPhases[i], i);
    //         defaultBarrierPhases[i][b->barrierNum] = b;
    //         // the barrier 1 and barrier 0 default phase must not have the same barrier number
    //         if (b->barrierNum == coordinatePhaseObjs[i]->barrierNum) {
    //             throw ProcessError("At traffic signal " + myID + " the barrier and coordinated phases " +
    //                                std::to_string(b->phaseName) + ", " + std::to_string(coordinatePhaseObjs[i]->barrierNum) +
    //                                " are located on the same side of a barrier." +
    //                                " Please check your configuration file");
    //         }
    //     }
    //
    //     // Create the PhaseDetectorInfo for each of the phases (needs knowledge of other phases to create)
    //     IntVector latchingDetectors = readParaFromString(getParameter("latchingDetectors", ""));
    //     std::vector<std::pair<int, int>> cp;
    //     for (auto& p : myPhaseObjs) {
    //         std::string cps = "crossPhaseSwitching:";
    //         int crossPhase = StringUtils::toInt(getParameter(cps.append(std::to_string(p->phaseName)), "0"));
    //         if (crossPhase > 0) {
    //             cp.push_back({ p->phaseName, crossPhase });
    //         }
    //     }
    //
    //     // Knowing the cross phase info, we can add that to the phase
    //     for (auto& p : myPhaseObjs) {
    //         bool latching = vectorContainsPhase(latchingDetectors, p->phaseName);
    //         int cpTarget = 0;
    //         int cpSource = 0;
    //         for (auto& cp_pair : cp) {
    //             if (cp_pair.first == p->phaseName || cp_pair.second == p->phaseName) {
    //                 cpTarget = cp_pair.first;
    //                 cpSource = cp_pair.second;
    //             }
    //         }
    //         p->init(this, cpTarget, cpSource, latching);
    //     }
    //
    //     // Calculate Force offs Based on Timing
    //     calculateForceOffs();
    //
    //     if (coordinateMode) {
    //         // Calculate the Initial Phases in coordinated operation only.
    //         // Otherwise they have already been calculated above
    //         calculateInitialPhases();
    //     } else {
    //         // Fall back being the barrier 0 default phases
    //         // NEMAPhase* defaultP[2] = {defaultBarrierPhases[0][0], defaultBarrierPhases[1][0]};
    //         NEMAPhase* defaultP[2] = { getPhasesByRing(0).front(), getPhasesByRing(1).front() };
    //         defaultP[0]->forceEnter(this);
    //         defaultP[1]->forceEnter(this);
    //     }
    //
    //
    // #ifdef DEBUG_NEMA
    //     //print to check the rings and barriers active phase
    //     std::cout << "After init, active ring1 phase is " << myActivePhaseObjs[0]->phaseName << std::endl;
    //     std::cout << "After init, active ring2 phase is " << myActivePhaseObjs[1]->phaseName << std::endl;
    //
    //
    //     //print to check the phase definition is correct
    //     std::cout << "Print to check NEMA phase definitions\n";
    //     for (auto& p : myPhaseObjs) {
    //         std::cout << "index = " << p->phaseName << "; ";
    //         std::cout << "minDur = " << std::to_string(p->minDuration) << "; ";
    //         std::cout << "maxDur = " << std::to_string(p->maxDuration) << "; ";
    //         std::cout << "vehext = " << std::to_string(p->vehExt) << "; ";
    //         std::cout << "yellow = " << std::to_string(p->yellow) << "; ";
    //         std::cout << "red = " << std::to_string(p->red) << "; ";
    //         std::cout << "state = " << std::to_string((int)p->getCurrentState()) << std::endl;
    //     }
    // #endif
    //
    //
    // #ifdef DEBUG_NEMA
    //     std::cout << "After init, r1/r2 barrier phase = " << defaultBarrierPhases[0][1]->phaseName << " and " << defaultBarrierPhases[1][1]->phaseName << std::endl;
    //     std::cout << "After init, r1/r2 coordinate phase = " << defaultBarrierPhases[0][0]->phaseName << " and " << defaultBarrierPhases[1][0]->phaseName << std::endl;
    // #endif
    //
    //
    // #ifdef DEBUG_NEMA
    //     std::cout << "R1State = " << myActivePhaseObjs[0]->phaseName << " and its state = " << std::to_string((int)myActivePhaseObjs[0]->getCurrentState()) << std::endl;
    //     std::cout << "R2State = " << myActivePhaseObjs[1]->phaseName << " and its state = " << std::to_string((int)myActivePhaseObjs[0]->getCurrentState()) << std::endl;
    // #endif
    //
    //     // Set the initial light state
    //     myPhase.setState(composeLightString());
    //     myPhase.setName(toString(myActivePhaseObjs[0]->phaseName) + "+" + toString(myActivePhaseObjs[1]->phaseName));
    //     setTrafficLightSignals(SIMSTEP);
    //     myStep = 0;
    //
    //     //validating timing
    //     validate_timing();
    // }
    #[allow(unused_variables)]
    pub fn construct_timing_and_phase_defs(&mut self, barriers: &mut String, coordinates: &mut String, ring1: &mut String, ring2: &mut String) {
        todo!("port NEMALogic::constructTimingAndPhaseDefs")
    }

    /// C++: `std::string NEMALogic::composeLightString()` [protected]
    /// iterates over the two active phases (myActivePhaseObjs) and merges the two active phases
    // std::string
    // NEMALogic::composeLightString() {
    //     // FIX with plan to support #10742
    //     std::string out(myPhaseStrLen, 'r');
    //     for (int i = 0; i < myPhaseStrLen; i++) {
    //         bool controlled = false;
    //         std::string phaseChars = "";
    //         for (auto& p : myActivePhaseObjs) {
    //             phaseChars += p->getNEMAChar(i);
    //             if (p->controlledIndex(i)) {
    //                 out[i] = p->getNEMAChar(i);
    //                 controlled = true;
    //             }
    //         }
    //         // if the index wasn't a controlled one, the prior priority order still stands
    //         if (!controlled) {
    //             for (auto priority_char : lightHeadPriority) {
    //                 if (std::count(phaseChars.begin(), phaseChars.end(), priority_char)) {
    //                     out[i] = priority_char;
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     return out;
    // }
    #[allow(unused_variables)]
    pub fn compose_light_string(&mut self) -> String {
        todo!("port NEMALogic::composeLightString")
    }

    /// C++: `bool NEMALogic::vectorContainsPhase(std::vector<int>, int)` [protected]
    /// check if a vector contains an element
    // bool
    // NEMALogic::vectorContainsPhase(std::vector<int> v, int phaseNum) {
    //     if (std::find(v.begin(), v.end(), phaseNum) != v.end()) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn vector_contains_phase(&mut self, v: Vec<i32>, phaseNum: i32) -> bool {
        todo!("port NEMALogic::vectorContainsPhase")
    }

    /// C++: `bool NEMALogic::hasMajor(const std::string &, const LaneVector &) const` [const, protected]
    /// return whether there is a major link from the given lane in the given phase
    // bool
    // NEMALogic::hasMajor(const std::string& state, const LaneVector& lanes) const {
    //     for (int i = 0; i < (int)state.size(); i++) {
    //         if (state[i] == LINKSTATE_TL_GREEN_MAJOR) {
    //             for (MSLane* cand : getLanesAt(i)) {
    //                 for (MSLane* lane : lanes) {
    //                     if (lane == cand) {
    //                         return true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_major(&self, state: &str, lanes: &LaneVector) -> bool {
        todo!("port NEMALogic::hasMajor")
    }

    /// C++: `std::vector<int> NEMALogic::readParaFromString(std::string)` [protected]
    /// converts a comma separated string into a integer vector "1,2,3,4" -> {1,2,3,4}
    // std::vector<int> NEMALogic::readParaFromString(std::string s) {
    //     std::vector<int> output;
    //     for (char c : s) {
    //         if (c >= '0' && c <= '9') {
    //             int temp = c - '0';
    //             output.push_back(temp);
    //         }
    //     }
    //     return output;
    // }
    #[allow(unused_variables)]
    pub fn read_para_from_string(&mut self, s: String) -> Vec<i32> {
        todo!("port NEMALogic::readParaFromString")
    }

    /// C++: `bool NEMALogic::isLeftTurnLane(const MSLane *const) const` [const, protected]
    /// decide whether the detector is for left turn lane if it is, use the detector length for left turn lane
    // bool NEMALogic::isLeftTurnLane(const MSLane* const lane) const {
    //     const std::vector<MSLink*> links = lane->getLinkCont();
    //     if (links.size() == 1 && links.front()->getDirection() == LinkDirection::LEFT) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_left_turn_lane(&self, lane: Ptr<MSLane>) -> bool {
        todo!("port NEMALogic::isLeftTurnLane")
    }

    /// C++: `int NEMALogic::string2int(std::string)` [protected]
    /// convert a string to an integer
    // int NEMALogic::string2int(std::string s) {
    //     std::stringstream ss(s);
    //     int ret = 0;
    //     ss >> ret;
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn string2int(&mut self, s: String) -> i32 {
        todo!("port NEMALogic::string2int")
    }

    /// C++: `PhaseTransitionLogic * NEMALogic::getDefaultTransition(PhaseTransitionLogic *, PhaseTransitionLogic *)` [protected]
    /// return the default transition for t give its and the ot's state
    // PhaseTransitionLogic*
    // NEMALogic::getDefaultTransition(PhaseTransitionLogic* t, PhaseTransitionLogic* ot) {
    //     NEMAPhase* p = t->getFromPhase();
    //     // if the current phase is not ready to switch or a barrier cross is desired by the other transition
    //     // and t fromPhase is not ready to switch, the default transition is back to myself
    //     if (!p->readyToSwitch ||
    //             (p->barrierNum == ot->getToPhase()->barrierNum && p->getCurrentState() >= LightState::Green)) {
    //         return p->getTransition(p->phaseName);
    //     }
    //     // otherwise the default transition is to the default phase on whatever barrier ot wants to transition to
    //     else {
    //         return p->getTransition(defaultBarrierPhases[p->ringNum][ot->getToPhase()->barrierNum]->phaseName);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_default_transition(&mut self, t: Ptr<PhaseTransitionLogic>, ot: Ptr<PhaseTransitionLogic>) -> Ptr<PhaseTransitionLogic> {
        todo!("port NEMALogic::getDefaultTransition")
    }

    /// C++: `controllerType NEMALogic::parseControllerType(std::string)` [protected]
    /// parse the controllerType from the tllogic description
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn parse_controller_type(&mut self, inputType: String) -> NEMALogic_controllerType {
        todo!("port NEMALogic::parseControllerType")
    }

    /// C++: `void NEMALogic::error_handle_not_set(std::string, std::string)` [protected]
    /// throw an InvalidArgument error if the param_name is not set
    // void
    // NEMALogic::error_handle_not_set(std::string param_variable, std::string param_name) {
    //     if (param_variable == "") {
    //         throw InvalidArgument("Please set " + param_name + " for NEMA tlLogic '" + getID() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn error_handle_not_set(&mut self, param_variable: String, param_name: String) {
        todo!("port NEMALogic::error_handle_not_set")
    }

    /// C++: `void NEMALogic::validate_timing()` [protected]
    /// validates the NEMA timing. Writes warnings if ignoreError set to true else throws ProcessError
    // void
    // NEMALogic::validate_timing() {
    //     // check that the cycle length for each ring adds up to the specified cycle length
    //     for (int ringIndex = 0; ringIndex < 2; ringIndex++) {
    //         SUMOTime cycleLengthCalculated = 0;
    //         for (auto& p : getPhasesByRing(ringIndex)) {
    //             cycleLengthCalculated += (p->maxDuration + p->yellow + p->red);
    //         }
    //         if (coordinateMode && (cycleLengthCalculated != myCycleLength)) {
    //             int ringNumber = ringIndex + 1;
    //             const std::string error = "At NEMA tlLogic '" + getID() + "', Ring " + toString(ringNumber) + " does not add to cycle length.";
    //             if (ignoreErrors) {
    //                 WRITE_WARNING(error);
    //             } else {
    //                 throw  ProcessError(error);
    //             }
    //         }
    //     }
    //     // check that the barriers sum together
    //     SUMOTime cycleLengths[2][2] = { {0, 0}, {0, 0} };
    //     for (int ringIndex = 0; ringIndex < 2; ringIndex++) {
    //         // TS2 Force Offs don't go in order, so using a different method to check cycle time
    //         for (const auto p : getPhasesByRing(ringIndex)) {
    //             cycleLengths[ringIndex][p->barrierNum] += p->maxDuration + p->yellow + p->red;
    //         }
    //     }
    //     // Write warnings if the barriers do not sum
    //     for (int barrierNum = 0; barrierNum < 2; barrierNum++) {
    //         if (cycleLengths[0][barrierNum] != cycleLengths[1][barrierNum]) {
    //             const std::string error = "At NEMA tlLogic '" + getID() + "', the phases before barrier " + toString(barrierNum + 1) + " from both rings do not add up. (ring1="
    //                                       + toString(STEPS2TIME(cycleLengths[0][barrierNum])) + ", ring2=" + toString(STEPS2TIME(cycleLengths[1][barrierNum])) + ")";
    //             if (coordinateMode && !ignoreErrors) {
    //                 throw  ProcessError(error);
    //             } else {
    //                 WRITE_WARNING(error);
    //             }
    //         }
    //     }
    //
    //     // no offset for non coordinated
    //     if (!coordinateMode && offset != 0) {
    //         WRITE_WARNINGF(TL("NEMA tlLogic '%' is not coordinated but an offset was set."), getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn validate_timing(&mut self) {
        todo!("port NEMALogic::validate_timing")
    }

    /// C++: `void NEMALogic::calculateForceOffsTS2()` [protected]
    /// calculate the forceOffs for a TS2 style offset From https://ops.fhwa.dot.gov/publications/fhwahop08024/chapter6.htm#6.3
    // void
    // NEMALogic::calculateForceOffsTS2() {
    //     // TS2 "0" cycle time is the start of the "first" coordinated phases.
    //     // We can find this "0" point by first constructing the forceOffs in sequential order via the 170 method
    //     calculateForceOffs170();
    //
    //     // Switch the Force Off Times to align with TS2 Cycle, which is the *start* of the earliest coordinated phase
    //     // The coordinate phases will always be the defaultBarrierPhases[i][0]
    //     SUMOTime minCoordTime = MIN2(coordinatePhaseObjs[0]->forceOffTime - coordinatePhaseObjs[0]->maxDuration,
    //                                  coordinatePhaseObjs[1]->forceOffTime - coordinatePhaseObjs[1]->maxDuration);
    //
    //     // loop through all the phases and subtract this minCoordTime to move the 0 point to the start of the first coordinated phase
    //     for (auto& p : myPhaseObjs) {
    //         if ((p->forceOffTime - minCoordTime) >= 0) {
    //             p->forceOffTime -= (minCoordTime);
    //         } else {
    //             p->forceOffTime = (myCycleLength + (p->forceOffTime - (minCoordTime)));
    //         }
    //         p->greatestStartTime = ModeCycle(p->greatestStartTime - minCoordTime, myCycleLength);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn calculate_force_offs_ts2(&mut self) {
        todo!("port NEMALogic::calculateForceOffsTS2")
    }

    /// C++: `void NEMALogic::calculateForceOffs170()` [protected]
    /// calculate the forceOffs for a Type 170 style offset From https://ops.fhwa.dot.gov/publications/fhwahop08024/chapter6.htm#6.3
    // void
    // NEMALogic::calculateForceOffs170() {
    //     SUMOTime zeroTime[2] = { TIME2STEPS(0), TIME2STEPS(0) };
    //     for (int i = 0; i < 2; i++) {
    //         SUMOTime runningTime = 0;
    //         // loop through the phases for ring 0 and then 1
    //         for (auto& p : getPhasesByRing(i)) {
    //             runningTime += p->maxDuration + p->getTransitionTimeStateless();
    //             // in 170, the cycle "starts" when the coordinated phase goes to yellow.
    //             // See https://ops.fhwa.dot.gov/publications/fhwahop08024/chapter6.html
    //             if (p->coordinatePhase) {
    //                 zeroTime[i] = runningTime;
    //             }
    //             p->forceOffTime = runningTime - p->getTransitionTimeStateless();
    //             p->greatestStartTime = p->forceOffTime - p->minDuration;
    //         }
    //     }
    //     // find the minimum offset time and then subtract from everything, modecycling where negative
    //     // This sets the 0 cycle time as start of yellow on earliest ending coordinated phase
    //     SUMOTime minCoordYellow = MIN2(zeroTime[0], zeroTime[1]);
    //     for (auto& p : myPhaseObjs) {
    //         p->forceOffTime = ModeCycle(p->forceOffTime - minCoordYellow, myCycleLength);
    //         p->greatestStartTime = ModeCycle(p->greatestStartTime - minCoordYellow, myCycleLength);
    //     }
    //
    // #ifdef DEBUG_NEMA
    //     std::ios_base::fmtflags oldflags = std::cout.flags();
    //     std::streamsize oldprecision = std::cout.precision();
    //     for (int i = 0; i < 2; i++) {
    //         std::cout << "Ring" << i + 1 << " force offs: \t";
    //         for (auto& p : rings[i]) {
    //             if (p > 0) {
    //                 PhasePtr pObj = getPhaseObj(p, i);
    //                 std::cout << std::fixed << std::setprecision(2) << STEPS2TIME(pObj->forceOffTime) << "\t";
    //             } else {
    //                 std::cout << std::to_string(0) << "\t";
    //             }
    //         }
    //         std::cout << std::endl;
    //     }
    //     std::cout.flags(oldflags);
    //     std::cout.precision(oldprecision);
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn calculate_force_offs170(&mut self) {
        todo!("port NEMALogic::calculateForceOffs170")
    }

    /// C++: `void NEMALogic::calculateForceOffs()` [protected]
    /// directs the code to the correct force off function accorifing to its cabinet type
    // void calculateForceOffs() {
    //         switch (myControllerType) {
    //             case Type170:
    //                 return calculateForceOffs170();
    //             case TS2:
    //                 return calculateForceOffsTS2();
    //             default:
    //                 return calculateForceOffs170();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn calculate_force_offs(&mut self) {
        todo!("port NEMALogic::calculateForceOffs")
    }

    /// C++: `void NEMALogic::calculateInitialPhasesTS2()` [protected]
    /// calculate the initial phases for the TS2 style controller to start in
    // void
    // NEMALogic::calculateInitialPhasesTS2() {
    //     // Modifications where made to 170 algorithm so that it works with both.
    //     calculateInitialPhases170();
    // }
    #[allow(unused_variables)]
    pub fn calculate_initial_phases_ts2(&mut self) {
        todo!("port NEMALogic::calculateInitialPhasesTS2")
    }

    /// C++: `void NEMALogic::calculateInitialPhases170()` [protected]
    /// calculate the initial phases for Type 170
    // void
    // NEMALogic::calculateInitialPhases170() {
    //     // get the time in the cycle
    //     SUMOTime cycleTime = ModeCycle(getTimeInCycle(), myCycleLength);
    //     NEMAPhase* activePhases[2];
    //     for (int i = 0; i < 2; i++) {
    //         std::vector<NEMAPhase*> ringCopy = getPhasesByRing(i);
    //         // sort by the minimum start time in the cycle.
    //         std::sort(ringCopy.begin(), ringCopy.end(),
    //         [](NEMAPhase * p, NEMAPhase * p1) {
    //             return p->greatestStartTime <= p1->greatestStartTime;
    //         }
    //                  );
    //         bool found = false;
    //         // loop through the sorted phases by time and try to find the phase that should be active given the time in the cycle
    //         for (auto& p : ringCopy) {
    //             // This handles the wrap around. Checks if the prior phases start time should have already happened.
    //             // If it should have happened and it's not to the start time of me yet, start in my phase ( will have to be in my phase longer than max time likely )
    //             SUMOTime syntheticPriorStart = p->getSequentialPriorPhase()->greatestStartTime < p->greatestStartTime ?
    //                                            p->getSequentialPriorPhase()->greatestStartTime : p->getSequentialPriorPhase()->greatestStartTime - myCycleLength;
    //             if (cycleTime <= ModeCycle(p->greatestStartTime, myCycleLength) && cycleTime > ModeCycle(syntheticPriorStart, myCycleLength)) {
    //                 found = true;
    //                 activePhases[i] = p;
    //                 break;
    //             }
    //         }
    //         if (!found) {
    // #ifdef DEBUG_NEMA
    //             const std::string error = "I can't find the correct phase for NEMA tlLogic '" + getID() + "' Ring " + toString(i) + " to start in.";
    //             WRITE_WARNING(error);
    //             WRITE_WARNING(TL("I am starting in the coordinated phases"));
    // #endif
    //             activePhases[0] = defaultBarrierPhases[0][0];
    //             activePhases[1] = defaultBarrierPhases[1][0];
    //         }
    //     }
    //
    //     // ensure that the two found phases are on the same side of the barrier. If they aren't, just override with the default barrier phases
    //     if (activePhases[0]->barrierNum != activePhases[1]->barrierNum) {
    //         // give preference to whatever is on the coordinate side of the barrier, one must be if they aren't equal to each other
    //         activePhases[0] = activePhases[0]->barrierNum == 0 ? activePhases[0] : defaultBarrierPhases[0][0];
    //         activePhases[1] = activePhases[1]->barrierNum == 0 ? activePhases[1] : defaultBarrierPhases[1][0];
    //     }
    //
    //     // force enter the phases and update their expected duration to last until the forceOff
    //     activePhases[0]->forceEnter(this);
    //     activePhases[1]->forceEnter(this);
    // }
    #[allow(unused_variables)]
    pub fn calculate_initial_phases170(&mut self) {
        todo!("port NEMALogic::calculateInitialPhases170")
    }

    /// C++: `void NEMALogic::calculateInitialPhases()` [protected]
    /// directs the controller to the correct calculate phases function
    // void calculateInitialPhases() {
    //         switch (myControllerType) {
    //             case Type170:
    //                 return calculateInitialPhases170();
    //             case TS2:
    //                 return calculateInitialPhasesTS2();
    //             default:
    //                 // Default to Type170
    //                 return calculateInitialPhases170();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn calculate_initial_phases(&mut self) {
        todo!("port NEMALogic::calculateInitialPhases")
    }

}

/// One phase in the NEMAController
/// C++ class `NEMAPhase` (microsim/traffic_lights/NEMAController.h)
pub struct NEMAPhase {
    pub phaseName: i32,
    pub isAtBarrier: bool,
    pub isGreenRest: bool,
    pub barrierNum: i32,
    pub coordinatePhase: bool,
    pub minRecall: bool,
    pub maxRecall: bool,
    pub fixForceOff: bool,
    pub ringNum: i32,
    /// store the last detect check for traci purposes
    pub lastDetectActive: bool,
    /// a count down timer to track green rest transition time
    pub greenRestTimer: SUMOTime,
    pub greatestStartTime: SUMOTime,
    /// stores the force off time in coordinated mode
    pub forceOffTime: SUMOTime,
    /// flag to for the supervisory controller to denote whether phase is ready to switch or not.
    pub readyToSwitch: bool,
    /// @{
    pub yellow: SUMOTime,
    pub red: SUMOTime,
    pub minDuration: SUMOTime,
    pub maxDuration: SUMOTime,
    pub nextMaxDuration: SUMOTime,
    pub vehExt: SUMOTime,
    /// A reference to the core phase of which NEMAPhase wraps
    pub myCorePhase: Ptr<MSPhaseDefinition>,
    /// @{
    pub myInstance: NEMALogic_PhasePtr,
    pub myLastPhaseInstance: NEMALogic_PhasePtr,
    pub sequentialPriorPhase: NEMALogic_PhasePtr,
    /// @}
    pub myLightState: LightState,
    pub myDetectorInfo: NEMAPhase_PhaseDetectorInfo,
    /// @{
    pub maxGreenDynamic: SUMOTime,
    pub myStartTime: SUMOTime,
    pub myExpectedDuration: SUMOTime,
    pub myLastEnd: SUMOTime,
    /// @{
    pub myPhaseStringInds: IntVector,
    pub myGreenString: String,
    pub myYellowString: String,
    pub myRedString: String,
    /// stores a sorted list of potential transitions
    pub myTransitions: Vec<Ptr<PhaseTransitionLogic>>,
    /// variable to store whether a transition is active or not
    pub transitionActive: bool,
    /// pointer to save the last transition
    pub lastTransitionDecision: Ptr<PhaseTransitionLogic>,
}

impl NEMAPhase {
    /// C++: `void NEMAPhase::NEMAPhase(int, bool, bool, bool, bool, bool, bool, int, int, IntVector, MSPhaseDefinition *)`
    /// Construct a new NEMAPhase object
    // NEMAPhase::NEMAPhase(int phaseName, bool isBarrier, bool isGreenRest, bool isCoordinated,
    //                      bool minRecall, bool maxRecall, bool fixForceOff, int barrierNum, int ringNum,
    //                      IntVector phaseStringInds,
    //                      MSPhaseDefinition* phase) :
    //     phaseName(phaseName),
    //     isAtBarrier(isBarrier),
    //     isGreenRest(isGreenRest),
    //     barrierNum(barrierNum),
    //     coordinatePhase(isCoordinated),
    //     minRecall(minRecall),
    //     maxRecall(maxRecall),
    //     fixForceOff(fixForceOff),
    //     ringNum(ringNum),
    //     myCorePhase(phase),
    //     myPhaseStringInds(phaseStringInds) {
    //     // Public
    //     readyToSwitch = false;
    //     greenRestTimer = 0;
    //     forceOffTime = 0;
    //     lastDetectActive = false;
    //
    //     // Private
    //     myInstance = this;
    //     myLastPhaseInstance = nullptr;
    //     sequentialPriorPhase = nullptr;
    //     myLightState = LightState::Red;
    //     transitionActive = false;
    //
    //     // Timing Parameters
    //     maxGreenDynamic = myCorePhase->maxDuration;
    //     myStartTime = TIME2STEPS(0.);
    //     myExpectedDuration = myCorePhase->minDuration;
    //     myLastEnd = TIME2STEPS(0.);
    //
    //     // set the phase colors
    //     setMyNEMAStates();
    // }
    #[allow(unused_variables)]
    pub fn new(phaseName: i32, isBarrier: bool, isGreenRest: bool, isCoordinated: bool, minRecall: bool, maxRecall: bool, fixForceOff: bool, barrierNum: i32, ringNum: i32, phaseStringInds: IntVector, phase: Ptr<MSPhaseDefinition>) -> Self {
        todo!("port NEMAPhase::NEMAPhase")
    }

    /// C++: `void NEMAPhase::~NEMAPhase()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port NEMAPhase::~NEMAPhase")
    }

    /// C++: `LightState NEMAPhase::getCurrentState() const` [const]
    /// gets the current light state
    // inline LightState getCurrentState() const {
    //         return myLightState;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_state(&self) -> LightState {
        todo!("port NEMAPhase::getCurrentState")
    }

    /// C++: `std::vector<MSE2Collector *> NEMAPhase::getDetectors() const` [const]
    /// returns a vector of the phases detectors
    // inline std::vector<MSE2Collector*> getDetectors() const {
    //         return myDetectorInfo.detectors;
    //     }
    #[allow(unused_variables)]
    pub fn get_detectors(&self) -> Vec<Ptr<MSE2Collector>> {
        todo!("port NEMAPhase::getDetectors")
    }

    /// C++: `void NEMAPhase::setDetectors(std::vector<MSE2Collector *>)`
    /// sets the detectors for the phase
    // inline void setDetectors(std::vector<MSE2Collector*> detectors) {
    //         myDetectorInfo.detectors = detectors;
    //     }
    #[allow(unused_variables)]
    pub fn set_detectors(&mut self, detectors: Vec<Ptr<MSE2Collector>>) {
        todo!("port NEMAPhase::setDetectors")
    }

    /// C++: `bool NEMAPhase::isTransitionActive() const` [const]
    /// check if a transition is active
    // inline bool isTransitionActive() const {
    //         return myLightState < LightState::Green;
    //     }
    #[allow(unused_variables)]
    pub fn is_transition_active(&self) -> bool {
        todo!("port NEMAPhase::isTransitionActive")
    }

    /// C++: `void NEMAPhase::init(NEMALogic *, int, int, bool)`
    /// initializes the object
    // void
    // NEMAPhase::init(NEMALogic* controller, int crossPhaseTarget, int crossPhaseSource, bool latching) {
    //     // switch the durations from steps2time
    //     recalculateTiming();
    //
    //     for (auto p : controller->getPhasesByRing(ringNum)) {
    //         // construct transitions for all potential movements, including back to myself
    //         myTransitions.push_back(new PhaseTransitionLogic(this, p));
    //         myTransitions.back()->setDistance(controller->measureRingDistance(phaseName, p->phaseName, ringNum));
    //     }
    //
    //     // sort the transitions by distance for speed later. Using plain distance here
    //     std::sort(myTransitions.begin(), myTransitions.end(), [&](const PhaseTransitionLogic * i, const PhaseTransitionLogic * j) {
    //         return i->distance < j->distance;
    //     });
    //
    //     // create the phase detector info
    //     myDetectorInfo = PhaseDetectorInfo(latching,
    //                                        crossPhaseSource > 0 ? controller->getPhaseObj(crossPhaseSource) : nullptr,
    //                                        crossPhaseTarget > 0 ? controller->getPhaseObj(crossPhaseTarget) : nullptr
    //                                       );
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, controller: Ptr<NEMALogic>, crossPhaseTarget: i32, crossPhaseSource: i32, latching: bool) {
        todo!("port NEMAPhase::init")
    }

    /// C++: `void NEMAPhase::update(NEMALogic *)`
    /// update is called on the active phases by the NEMAController at every time step
    // void
    // NEMAPhase::update(NEMALogic* controller) {
    //     // If I am in a transition, the rest of the update logic does not matter
    //     if (myLightState < LightState::Green) {
    //         // return early
    //         readyToSwitch = true;
    //         return;
    //     }
    //
    //     // Continuation Logic
    //     SUMOTime duration = controller->getCurrentTime() - myStartTime;
    //     // Check the vehicle extension timer as long as not in green transfer and not a coordinated phase
    //     if (myLightState != LightState::GreenXfer && !coordinatePhase) {
    //         myExpectedDuration = calcVehicleExtension(duration);
    //     }
    //     // Special Logic for Green Rest, which behaves uniquely
    //     if (myLightState == LightState::GreenRest) {
    //         // check all other detectors and see if anything else is active. If so,
    //         // start the green rest timer countdown, which is == to the max duration of the phase
    //         bool vehicleActive = false;
    //         for (auto& p : controller->getPhaseObjs()) {
    //             if ((p->phaseName != phaseName)
    //                     && (p->phaseName != controller->getOtherPhase(this)->phaseName)
    //                     && p->callActive()) {
    //                 greenRestTimer -= DELTA_T;
    //                 vehicleActive = true;
    //                 break;
    //             }
    //         }
    //         // catch the rising edge of the sidestreet detection and calculate the maximum timer
    //         if (vehicleActive && (greenRestTimer + DELTA_T >= maxDuration)) {
    //             maxGreenDynamic = minDuration + maxDuration;
    //         }
    //
    //         // if there are no other vehicles slide the startTime along
    //         if (!vehicleActive) {
    //             greenRestTimer = maxDuration;
    //             if (duration >= minDuration) {
    //                 myStartTime = controller->getCurrentTime() - minDuration;
    //                 maxGreenDynamic = minDuration + maxDuration;
    //                 myExpectedDuration = minDuration + MAX2(TIME2STEPS(0), myExpectedDuration - duration);
    //             }
    //         }
    //
    //         // if the green rest timer is exhausted, set ready to switch
    //         if (greenRestTimer < DELTA_T) {
    //             readyToSwitch = true;
    //             // force the counterpart to be ready to switch too. This needs to be latching....
    //             NEMAPhase* otherPhase = controller->getOtherPhase(this);
    //             if (otherPhase->getCurrentState() > LightState::Green) {
    //                 otherPhase->readyToSwitch = true;
    //             }
    //         }
    //
    //         // Special Behavior when the Green Rest Circles all the way around in coordinated mode
    //         if (coordinatePhase) {
    //             // This means that we have green rested until I should "start" again. Just call my entry function again.
    //             if (controller->getTimeInCycle() <= ((forceOffTime - maxDuration) + DELTA_T / 2)) {
    //                 enter(controller, this);
    //             }
    //         }
    //     }
    //     // Check to see if a switch is desired
    //     if (duration >= myExpectedDuration) {
    //         readyToSwitch = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update(&mut self, controller: Ptr<NEMALogic>) {
        todo!("port NEMAPhase::update")
    }

    /// C++: `void NEMAPhase::exit(NEMALogic *, PhaseTransitionLogic **)`
    /// handles the transition out of a phase into the next (puts the phase through (G -> Y -> R) transition
    // void NEMAPhase::exit(NEMALogic* controller, PhaseTransitionLogic* nextPhases[2]) {
    //     if (nextPhases[ringNum]->getToPhase() != this) {
    //         // if the next phase is not me, then I need to go into a transition
    //         lastTransitionDecision = nextPhases[ringNum];
    //         if (myLightState >= LightState::Green) {
    //             // if I am in green, then I need to enter yellow
    //             enterYellow(controller);
    //             return;
    //         }
    //
    //         if (controller->getCurrentTime() - myLastEnd < (yellow + red)) {
    //             if (controller->getCurrentTime() - myLastEnd >= yellow) {
    //                 // if I am in yellow, then I need to enter red
    //                 myLightState = LightState::Red;
    //             }
    //             // I am currently in the Red state but haven't reached max
    //             return;
    //         }
    //
    //         handleRedXferOrNextPhase(controller, nextPhases);
    //         return;
    //     }
    //
    //     handleGreenRestOrTransfer(controller, nextPhases);
    // }
    #[allow(unused_variables)]
    pub fn exit(&mut self, controller: Ptr<NEMALogic>, nextPhases: Unmapped /* PhaseTransitionLogic *[2] */) {
        todo!("port NEMAPhase::exit")
    }

    /// C++: `void NEMAPhase::handleGreenRestOrTransfer(NEMALogic *, PhaseTransitionLogic **)`
    /// handles the transition into a green rest state
    // void NEMAPhase::handleGreenRestOrTransfer(NEMALogic* controller, PhaseTransitionLogic* nextPhases[2]) {
    //     NEMAPhase* otherPhase = controller->getOtherPhase(this);
    //     readyToSwitch = false;
    //     bool isOtherPhaseReady = nextPhases[!ringNum]->getToPhase() == otherPhase && otherPhase->readyToSwitch;
    //     bool isOtherPhaseInGreenRest = otherPhase->greenRestTimer >= otherPhase->maxDuration && otherPhase->getCurrentState() == LightState::GreenRest;
    //
    //     if (isOtherPhaseReady || isOtherPhaseInGreenRest) {
    //         myLightState = LightState::GreenRest;
    //         myStartTime = controller->getCurrentTime() - minDuration;
    //         myExpectedDuration = minDuration;
    //         greenRestTimer = maxDuration * isGreenRest;
    //     } else {
    //         myLightState = LightState::GreenXfer;
    //         if (isAtBarrier) {
    //             myExpectedDuration = (otherPhase->myExpectedDuration + otherPhase->myStartTime) - myStartTime;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_green_rest_or_transfer(&mut self, controller: Ptr<NEMALogic>, nextPhases: Unmapped /* PhaseTransitionLogic *[2] */) {
        todo!("port NEMAPhase::handleGreenRestOrTransfer")
    }

    /// C++: `void NEMAPhase::enterYellow(NEMALogic *)`
    /// handles the transition into yellow
    // void NEMAPhase::enterYellow(NEMALogic* controller) {
    //     myLastEnd = controller->getCurrentTime();
    //     myLightState = LightState::Yellow;
    //     transitionActive = true;
    // }
    #[allow(unused_variables)]
    pub fn enter_yellow(&mut self, controller: Ptr<NEMALogic>) {
        todo!("port NEMAPhase::enterYellow")
    }

    /// C++: `void NEMAPhase::handleRedXferOrNextPhase(NEMALogic *, PhaseTransitionLogic **)`
    /// handles the transition into a red xfer state, which is roughly the same as green rest
    // void NEMAPhase::handleRedXferOrNextPhase(NEMALogic* controller, PhaseTransitionLogic* nextPhases[2]) {
    //     PhasePtr otherPhase = controller->getOtherPhase(this);
    //     bool barrierCross = nextPhases[ringNum]->getToPhase()->barrierNum != barrierNum;
    //     bool barrierCrossButOkay = barrierCross && (
    //                                    nextPhases[ringNum]->getToPhase()->barrierNum == nextPhases[otherPhase->ringNum]->getToPhase()->barrierNum
    //                                ) && otherPhase->okay2ForceSwitch(controller);
    //
    //     if (!barrierCross) {
    //         nextPhases[ringNum]->getToPhase()->enter(controller, this);
    //         return;
    //     }
    //
    //     if (barrierCrossButOkay) {
    //         // if the barrier is crossed, but the other phase is going to the same barrier, then I can enter red transfer
    //         // enter the next phase
    //         nextPhases[ringNum]->getToPhase()->enter(controller, this);
    //         // trigger the other phase to enter red transfer
    //         nextPhases[otherPhase->ringNum]->getToPhase()->enter(controller, this);
    //         return;
    //     }
    //
    //     myLightState = LightState::RedXfer;
    //     readyToSwitch = true;
    //     transitionActive = false;
    // }
    #[allow(unused_variables)]
    pub fn handle_red_xfer_or_next_phase(&mut self, controller: Ptr<NEMALogic>, nextPhases: Unmapped /* PhaseTransitionLogic *[2] */) {
        todo!("port NEMAPhase::handleRedXferOrNextPhase")
    }

    /// C++: `bool NEMAPhase::hasRecall()`
    /// simple method to check if there is a recall on the phase.
    // inline bool hasRecall(void) {
    //         return minRecall || maxRecall;
    //     }
    #[allow(unused_variables)]
    pub fn has_recall(&mut self) -> bool {
        todo!("port NEMAPhase::hasRecall")
    }

    /// C++: `bool NEMAPhase::callActive()`
    /// simple method to check if there is either a recall or an active detector
    // inline bool callActive(void) {
    //         return minRecall || maxRecall || myDetectorInfo.detectActive;
    //     }
    #[allow(unused_variables)]
    pub fn call_active(&mut self) -> bool {
        todo!("port NEMAPhase::callActive")
    }

    /// C++: `bool NEMAPhase::detectActive()`
    /// simple method to check if a detector is active
    // inline bool detectActive(void) {
    //         return myDetectorInfo.detectActive;
    //     }
    #[allow(unused_variables)]
    pub fn detect_active(&mut self) -> bool {
        todo!("port NEMAPhase::detectActive")
    }

    /// C++: `void NEMAPhase::checkMyDetectors()`
    /// Check Detectors. Called on all phases at every step
    // void
    // NEMAPhase::checkMyDetectors() {
    //     // Check my Detectors, only necessary if it isn't currently marked as on
    //     if (!myDetectorInfo.detectActive) {
    //         // If I have a cross phase target and it is active and I am not, save my detector as not active
    //         if (myDetectorInfo.cpdTarget != nullptr) {
    //             if (myDetectorInfo.cpdTarget->getCurrentState() >= LightState::Green) {
    //                 if (myLightState < LightState::Green) {
    //                     myDetectorInfo.detectActive = false;
    //                     return;
    //                 }
    //             }
    //         }
    //         // If we make it to this point, check my detector like normal.
    //         for (auto& d : myDetectorInfo.detectors) {
    //             if (d->getCurrentVehicleNumber() > 0) {
    //                 myDetectorInfo.detectActive = true;
    //                 return;
    //             }
    //         }
    //         // If my detector is not active, check my cross phase
    //         if ((myDetectorInfo.cpdSource != nullptr) && (myLightState >= LightState::Green)) {
    //             if (myDetectorInfo.cpdSource->getCurrentState() < LightState::Green) {
    //                 for (auto& d : myDetectorInfo.cpdSource->getDetectors()) {
    //                     if (d->getCurrentVehicleNumber() > 0) {
    //                         myDetectorInfo.detectActive = true;
    //                         return;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_my_detectors(&mut self) {
        todo!("port NEMAPhase::checkMyDetectors")
    }

    /// C++: `void NEMAPhase::clearMyDetectors()`
    /// Clear My Detectors. Called on all phases at every step
    // void
    // NEMAPhase::clearMyDetectors() {
    //     lastDetectActive = myDetectorInfo.detectActive;
    //     // remove the active flag on the detector if the detector is not latching or if it is green
    //     if ((!myDetectorInfo.latching) || (myLightState >= LightState::Green)) {
    //         myDetectorInfo.detectActive = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_my_detectors(&mut self) {
        todo!("port NEMAPhase::clearMyDetectors")
    }

    /// C++: `SUMOTime NEMAPhase::getTransitionTime(NEMALogic *)`
    /// Get the Transition Time
    // SUMOTime
    // NEMAPhase::getTransitionTime(NEMALogic* controller) {
    //     if (myLightState == LightState::RedXfer) {
    //         // if in red xfer, I am ready to switch whenevery
    //         return TIME2STEPS(0);
    //     }
    //     if (!transitionActive) {
    //         // if a transition is not active, the transition is just yellow + red time
    //         return getTransitionTimeStateless();
    //     }
    //     // if a transition is active, then return the time left in the transition
    //     return MAX2(TIME2STEPS(0), ((yellow + red) - (controller->getCurrentTime() - myLastEnd)));
    // }
    #[allow(unused_variables)]
    pub fn get_transition_time(&mut self, controller: Ptr<NEMALogic>) -> SUMOTime {
        todo!("port NEMAPhase::getTransitionTime")
    }

    /// C++: `SUMOTime NEMAPhase::getTransitionTimeStateless()`
    /// Get the Transition time given
    // inline SUMOTime getTransitionTimeStateless(void) {
    //         return yellow + red;
    //     }
    #[allow(unused_variables)]
    pub fn get_transition_time_stateless(&mut self) -> SUMOTime {
        todo!("port NEMAPhase::getTransitionTimeStateless")
    }

    /// C++: `PhasePtr NEMAPhase::getSequentialPriorPhase()`
    /// get the prior phase
    // inline PhasePtr getSequentialPriorPhase(void) {
    //         return sequentialPriorPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_sequential_prior_phase(&mut self) -> NEMALogic_PhasePtr {
        todo!("port NEMAPhase::getSequentialPriorPhase")
    }

    /// C++: `void NEMAPhase::setSequentialPriorPhase(PhasePtr)`
    /// set the prior phase
    // inline void setSequentialPriorPhase(PhasePtr priorPhase) {
    //         sequentialPriorPhase = priorPhase;
    //     }
    #[allow(unused_variables)]
    pub fn set_sequential_prior_phase(&mut self, priorPhase: NEMALogic_PhasePtr) {
        todo!("port NEMAPhase::setSequentialPriorPhase")
    }

    /// C++: `std::vector<PhaseTransitionLogic *> NEMAPhase::trySwitch(NEMALogic *)`
    /// calculate a vector of potention next phases
    // std::vector<PhaseTransitionLogic*>
    // NEMAPhase::trySwitch(NEMALogic* controller) {
    //     // this function returns the preferred valid transition for the phase
    //     std::vector<PhaseTransitionLogic*> nextTransitions;
    //     if (readyToSwitch) {
    //         // only try to switch if I am ready to switch
    //         for (auto& t : myTransitions) {
    //             // for the transitions check if it is okay
    //             if (t->okay(controller)) {
    //                 // if there was already a transition decision, it can be overriden but only if the new transition
    //                 // is on the same side of a barrier
    //                 if (lastTransitionDecision != nullptr) {
    //                     if (t->getToPhase()->barrierNum == lastTransitionDecision->getToPhase()->barrierNum) {
    //                         nextTransitions.push_back(t);
    //                         break;
    //                     }
    //                 } else {
    //                     nextTransitions.push_back(t);
    //                     // break once there is a valid option (they have already been sorted)
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     // Add in the last transition decision if it hasn't been added in yet
    //     if (lastTransitionDecision != nullptr) {
    //         bool found = false;
    //         bool sameBarrier = false;
    //         for (auto& t : nextTransitions) {
    //             if (t == lastTransitionDecision) {
    //                 found = true;
    //                 break;
    //             }
    //             if (t->getToPhase()->barrierNum == lastTransitionDecision->getToPhase()->barrierNum) {
    //                 sameBarrier = true;
    //                 break;
    //             }
    //         }
    //         // but only need to add it if it is not in the list AND if nothing in the list is the same barrier as it was.
    //         if (!found && !sameBarrier) {
    //             nextTransitions.push_back(lastTransitionDecision);
    //         }
    //     }
    //     // Add the transition back to myself, but only in the case when no others have been added
    //     if (nextTransitions.size() < 1) {
    //         nextTransitions.push_back(myTransitions.back());
    //     }
    //
    //     return nextTransitions;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self, controller: Ptr<NEMALogic>) -> Vec<Ptr<PhaseTransitionLogic>> {
        todo!("port NEMAPhase::trySwitch")
    }

    /// C++: `PhaseTransitionLogic * NEMAPhase::getTransition(int)`
    /// return the PhaseTransitionLogic matching the toPhase
    // PhaseTransitionLogic*
    // NEMAPhase::getTransition(int toPhase) {
    //     for (auto t : myTransitions) {
    //         if (t->getToPhase()->phaseName == toPhase) {
    //             return t;
    //         }
    //     }
    //     // This point should never be reached
    //     assert(0);
    //     // To satisfy the compiler and return value from all control paths
    //     return myTransitions.front();
    // }
    #[allow(unused_variables)]
    pub fn get_transition(&mut self, toPhase: i32) -> Ptr<PhaseTransitionLogic> {
        todo!("port NEMAPhase::getTransition")
    }

    /// C++: `char NEMAPhase::getNEMAChar(int)`
    /// Return the ryg light string for the phase
    // char
    // NEMAPhase::getNEMAChar(int i) {
    //     if (myLightState >= LightState::Green) {
    //         return myGreenString[i];
    //     } else if (myLightState <= LightState::Red) {
    //         return myRedString[i];
    //     } else {
    //         return myYellowString[i];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_nema_char(&mut self, i: i32) -> i8 {
        todo!("port NEMAPhase::getNEMAChar")
    }

    /// C++: `void NEMAPhase::recalculateTiming()`
    /// accessory function to recalculate timing
    // void
    // NEMAPhase::recalculateTiming(void) {
    //     // This could be extended in the future to allow for traci manipulation
    //     yellow = myCorePhase->yellow;
    //     red = myCorePhase->red;
    //     minDuration = myCorePhase->minDuration;
    //     maxDuration = myCorePhase->maxDuration;
    //     nextMaxDuration = myCorePhase->maxDuration;
    //     maxGreenDynamic = myCorePhase->maxDuration;
    //     vehExt = myCorePhase->vehext;
    // }
    #[allow(unused_variables)]
    pub fn recalculate_timing(&mut self) {
        todo!("port NEMAPhase::recalculateTiming")
    }

    /// C++: `void NEMAPhase::forceEnter(NEMALogic *)`
    /// Force Enter. This Should only be called at initialization time
    // inline void forceEnter(NEMALogic* controller) {
    //         enter(controller, sequentialPriorPhase);
    //     }
    #[allow(unused_variables)]
    pub fn force_enter(&mut self, controller: Ptr<NEMALogic>) {
        todo!("port NEMAPhase::forceEnter")
    }

    /// C++: `bool NEMAPhase::controlledIndex(int)`
    /// Return whether or not the phase index is controlled by me
    // inline bool controlledIndex(int i) {
    //         return std::count(myPhaseStringInds.begin(), myPhaseStringInds.end(), i) > 0;
    //     }
    #[allow(unused_variables)]
    pub fn controlled_index(&mut self, i: i32) -> bool {
        todo!("port NEMAPhase::controlledIndex")
    }

    /// C++: `void NEMAPhase::cleanupExit()`
    /// public method to set whether phase is active or not
    // inline void cleanupExit(void) {
    //         transitionActive = false;
    //         readyToSwitch = false;
    //         myLightState = LightState::Red;
    //     }
    #[allow(unused_variables)]
    pub fn cleanup_exit(&mut self) {
        todo!("port NEMAPhase::cleanupExit")
    }

    /// C++: `bool NEMAPhase::okay2ForceSwitch(NEMALogic *)`
    /// simple internal check to see if done okay to transition
    // inline bool okay2ForceSwitch(NEMALogic* controller) {
    //         return readyToSwitch && !transitionActive && (getTransitionTime(controller) <= TIME2STEPS(0));
    //     }
    #[allow(unused_variables)]
    pub fn okay2_force_switch(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port NEMAPhase::okay2ForceSwitch")
    }

    /// C++: `SUMOTime NEMAPhase::calcVehicleExtension(SUMOTime)` [private]
    /// Applies the vehicle extension timer if appropriate
    // SUMOTime
    // NEMAPhase::calcVehicleExtension(SUMOTime duration) {
    //     if (myExpectedDuration < maxGreenDynamic && myDetectorInfo.detectActive) {
    //         // add the vehicle extension timer if the detector is active.
    //         // capped by the minimum and maximum duration
    //         return MIN2(MAX2(duration + vehExt, minDuration), maxGreenDynamic);
    //     }
    //     return myExpectedDuration;
    // }
    #[allow(unused_variables)]
    pub fn calc_vehicle_extension(&mut self, duration: SUMOTime) -> SUMOTime {
        todo!("port NEMAPhase::calcVehicleExtension")
    }

    /// C++: `void NEMAPhase::enter(NEMALogic *, PhasePtr)` [private]
    /// handles entry to the phase during simulation Sets the color to green and determines maximum duration
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn enter(&mut self, controller: Ptr<NEMALogic>, lastPhase: NEMALogic_PhasePtr) {
        todo!("port NEMAPhase::enter")
    }

    /// C++: `void NEMAPhase::setMyNEMAStates()` [private]
    /// this function replaces getNEMAStates calculation at every call It sets my myGreenString, myYellowString, and myRedString on class creation
    // void
    // NEMAPhase::setMyNEMAStates() {
    //     myGreenString = myCorePhase->getState();
    //     myRedString = "";
    //     myYellowString = "";
    //     for (char ch : myGreenString) {
    //         myRedString += 'r';
    //         if (ch == 'G' || ch == 'g') {
    //             myYellowString += 'y';
    //         } else {
    //             myYellowString += ch;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_my_nema_states(&mut self) {
        todo!("port NEMAPhase::setMyNEMAStates")
    }

}

/// stores information about the phase's detector(s)
/// C++ class `NEMAPhase::PhaseDetectorInfo` (microsim/traffic_lights/NEMAController.h)
pub struct NEMAPhase_PhaseDetectorInfo {
    /// a vector of pointers to the phase's detectors
    pub detectors: Vec<Ptr<MSE2Collector>>,
    /// the cross-phase switching target for myself (6 if 6 should check 1 if 6 is green and I am phase 1)
    pub cpdTarget: NEMALogic_PhasePtr,
    /// the cross-phase switching source for myself (1 if 6 should check 1 if 6 is green and I am phase 6)
    pub cpdSource: NEMALogic_PhasePtr,
    /// where any of my detectors are active or not
    pub detectActive: bool,
    /// whether the detectors are latching or not
    pub latching: bool,
}

impl NEMAPhase_PhaseDetectorInfo {
    /// C++: `void PhaseDetectorInfo::PhaseDetectorInfo()`
    // PhaseDetectorInfo() :
    //             detectors(),
    //             cpdTarget(),
    //             cpdSource(),
    //             detectActive(),
    //             latching()
    //         {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port PhaseDetectorInfo::PhaseDetectorInfo")
    }

    /// C++: `void PhaseDetectorInfo::PhaseDetectorInfo(bool, PhasePtr, PhasePtr)`
    // PhaseDetectorInfo(bool latching, PhasePtr cpdSource, PhasePtr cpdTarget) :
    //             cpdTarget(cpdTarget),
    //             cpdSource(cpdSource),
    //             detectActive(false),
    //             latching(latching)
    //         {}
    #[allow(unused_variables)]
    pub fn new_2(latching: bool, cpdSource: NEMALogic_PhasePtr, cpdTarget: NEMALogic_PhasePtr) -> Self {
        todo!("port PhaseDetectorInfo::PhaseDetectorInfo")
    }

}

/// This class handles the transition logic between two phases
/// C++ class `PhaseTransitionLogic` (microsim/traffic_lights/NEMAController.h)
pub struct PhaseTransitionLogic {
    pub distance: i32,
    pub fromPhase: NEMALogic_PhasePtr,
    pub toPhase: NEMALogic_PhasePtr,
}

impl PhaseTransitionLogic {
    /// C++: `void PhaseTransitionLogic::PhaseTransitionLogic(PhasePtr, PhasePtr)`
    /// Construct a new Phase Transition Logic object
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(fromPhase: NEMALogic_PhasePtr, toPhase: NEMALogic_PhasePtr) -> Self {
        todo!("port PhaseTransitionLogic::PhaseTransitionLogic")
    }

    /// C++: `bool PhaseTransitionLogic::okay(NEMALogic *)`
    /// This function is the main PhaseTransitionLogic function It is called by the fromPhase to check if a transition to the toPhase is acceptable
    // bool
    // PhaseTransitionLogic::okay(NEMALogic* controller) {
    //     // Picking the correct transition logic to use
    //     // #TODO this could be a case of using function as variable and setting it at PhaseTransitionLogic
    //     // creation time
    //     if (fromPhase == toPhase) {
    //         // for green rest or green transfer, it cannot return to itself if a transition is active
    //         return fromPhase->getCurrentState() >= LightState::Green;
    //     } else if (fromPhase->coordinatePhase) {
    //         // if the from phase is a coordinated phase i.e. {2, 6} in a standard setup
    //         return fromCoord(controller);
    //     } else if (fromPhase->isAtBarrier) {
    //         // if the phase is at a barrier i.e. {2, 6, 4, 8} in a standard setup
    //         return fromBarrier(controller);
    //     } else if (controller->coordinateMode) {
    //         // typical coordinate mode transition,
    //         return coordBase(controller);
    //     } else {
    //         // base transition logic
    //         return freeBase(controller);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn okay(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port PhaseTransitionLogic::okay")
    }

    /// C++: `int PhaseTransitionLogic::getDistance(PhaseTransitionLogic *)`
    /// return the ring distance that this transition represents
    // int
    // PhaseTransitionLogic::getDistance(PhaseTransitionLogic* otherTrans) {
    //     // Returns the other transitions distance in green transfer situations
    //     if ((toPhase == fromPhase) && (otherTrans->toPhase->barrierNum == toPhase->barrierNum)) {
    //         if (toPhase->getCurrentState() == LightState::Green || toPhase->getCurrentState() == LightState::GreenXfer) {
    //             return otherTrans->distance;
    //         }
    //     }
    //     return distance;
    // }
    #[allow(unused_variables)]
    pub fn get_distance(&mut self, otherTrans: Ptr<PhaseTransitionLogic>) -> i32 {
        todo!("port PhaseTransitionLogic::getDistance")
    }

    /// C++: `void PhaseTransitionLogic::setDistance(int)`
    /// set the transition distance
    // inline void setDistance(int d) {
    //         distance = d;
    //     }
    #[allow(unused_variables)]
    pub fn set_distance(&mut self, d: i32) {
        todo!("port PhaseTransitionLogic::setDistance")
    }

    /// C++: `void PhaseTransitionLogic::~PhaseTransitionLogic() noexcept`
    /// deconstructor
    // ~PhaseTransitionLogic() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port PhaseTransitionLogic::~PhaseTransitionLogic")
    }

    /// C++: `PhasePtr PhaseTransitionLogic::getToPhase() const` [const]
    /// get the to phase
    // inline PhasePtr getToPhase(void) const {
    //         return toPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_phase(&self) -> NEMALogic_PhasePtr {
        todo!("port PhaseTransitionLogic::getToPhase")
    }

    /// C++: `PhasePtr PhaseTransitionLogic::getFromPhase() const` [const]
    /// get the from phase
    // inline PhasePtr getFromPhase(void) const {
    //         return fromPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_phase(&self) -> NEMALogic_PhasePtr {
        todo!("port PhaseTransitionLogic::getFromPhase")
    }

    /// C++: `void PhaseTransitionLogic::buildLogic()` [private]
    /// build the transition logic based on the from and to phase
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_logic(&mut self) {
        todo!("port PhaseTransitionLogic::buildLogic")
    }

    /// C++: `bool PhaseTransitionLogic::fromBarrier(NEMALogic *)` [private]
    /// If the fromPhase is at a barrier, then this function will be called to check whether the transition is valid
    // bool
    // PhaseTransitionLogic::fromBarrier(NEMALogic* controller) {
    //     if (freeBase(controller)) {
    //         if (fromPhase->barrierNum == toPhase->barrierNum) {
    //             // same barrier side so we are good.
    //             // Check if green transfer is active. If so, we need to make sure that there are no calls on the other side of the barrier
    //             if (fromPhase->getCurrentState() >= LightState::Green) {
    //                 for (auto& p : controller->getPhasesByRing(fromPhase->ringNum)) {
    //                     if (p->barrierNum != fromPhase->barrierNum && p->callActive()) {
    //                         return false;
    //                     }
    //                 }
    //             }
    //             return true;
    //         } else {
    //             // This is now a barrier cross and we need to make sure that the other phase is also ready to transition
    //             if (fromPhase->readyToSwitch && controller->getOtherPhase(fromPhase)->readyToSwitch) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn from_barrier(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port PhaseTransitionLogic::fromBarrier")
    }

    /// C++: `bool PhaseTransitionLogic::fromCoord(NEMALogic *)` [private]
    /// if the fromPhase is a coordinated phase, then this logic will be checked
    // bool
    // PhaseTransitionLogic::fromCoord(NEMALogic* controller) {
    //     if (coordBase(controller)) {
    //         // Determine if the other phase is also ready to switch
    //         if (controller->getOtherPhase(fromPhase)->readyToSwitch) {
    //             // Dr. Wang had the Type-170 code setup in a way that it could transition whenever - meaning that it didn't matter if the prior phase could fit or not
    //             if (controller->isType170()) {
    //                 return true;
    //             }
    //             // If the transition is already active, then report that the movement is possible
    //             if (fromPhase->isTransitionActive()) {
    //                 return true;
    //             }
    //             // now determine if there my prior phase can fit or not. We already know that I can fit.
    //             NEMAPhase* priorPhase = toPhase->getSequentialPriorPhase();
    //             SUMOTime timeTillForceOff = controller->ModeCycle(priorPhase->forceOffTime - controller->getTimeInCycle(), controller->getCurrentCycleLength());
    //             SUMOTime transitionTime = fromPhase->getTransitionTime(controller);
    //             // if the time till the force off is less than the min duration ||
    //             // if it is greater than the cycle length minus the length of the coordinate phase (which the fromPhase automatically is)
    //             if ((priorPhase->minDuration + transitionTime) > timeTillForceOff || timeTillForceOff > (controller->getCurrentCycleLength() - fromPhase->minDuration)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn from_coord(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port PhaseTransitionLogic::fromCoord")
    }

    /// C++: `bool PhaseTransitionLogic::freeBase(NEMALogic *)` [private]
    /// this represents the bare minimum logic, that the toPhase has an active detector and that the fromPhase is ready to switch
    // bool
    // PhaseTransitionLogic::freeBase(NEMALogic* controller) {
    //     // Simplest transition logic. Just check if a detector (or recall) is active on that phase and
    //     bool okay = false;
    //     // is a call active on the toPhase?
    //     if (toPhase->callActive()) {
    //         // would the transition be a barrier cross?
    //         if (fromPhase->barrierNum != toPhase->barrierNum) {
    //             PhasePtr otherPhase = controller->getOtherPhase(fromPhase);
    //             // If it is a barrier cross, then the other phase must also be ready to switch
    //             // or have a transition time that is lower than mine currently. DELTA_T is critical here
    //             if (otherPhase->readyToSwitch) {
    //                 // #&& otherPhase->getTransitionTime(controller) <= fromPhase->getTransitionTime(controller)) {
    //                 okay = true;
    //             }
    //         } else {
    //             okay = true;
    //         }
    //     }
    //     return okay;
    // }
    #[allow(unused_variables)]
    pub fn free_base(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port PhaseTransitionLogic::freeBase")
    }

    /// C++: `bool PhaseTransitionLogic::coordBase(NEMALogic *)` [private]
    /// represents the bare minimum coordinate mode logic. Requires that the toPhase can fit its minimum green time before the force off
    // bool
    // PhaseTransitionLogic::coordBase(NEMALogic* controller) {
    //     if (toPhase->coordinatePhase &&
    //             (controller->getOtherPhase(fromPhase)->readyToSwitch || fromPhase->barrierNum == toPhase->barrierNum)) {
    //         // transitions TO the coordinated phase may always happen, as long as the other phase is okay to switch too
    //         return true;
    //     }
    //     // first check if the free logic is upheld
    //     else if (freeBase(controller)) {
    //         // Then check if the "to phase" can fit, which means that there is enough time to fit the current transition + the minimum time of the next phase
    //         SUMOTime transitionTime = fromPhase->getTransitionTime(controller);
    //         SUMOTime timeTillForceOff = controller->ModeCycle(toPhase->forceOffTime - controller->getTimeInCycle(), controller->getCurrentCycleLength());
    //         if (toPhase->minDuration + transitionTime <= timeTillForceOff) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn coord_base(&mut self, controller: Ptr<NEMALogic>) -> bool {
        todo!("port PhaseTransitionLogic::coordBase")
    }

}

/// C++ class `NEMALogic::transitionInfo` (microsim/traffic_lights/NEMAController.h)
pub struct NEMALogic_transitionInfo {
    pub p1: Ptr<PhaseTransitionLogic>,
    pub p2: Ptr<PhaseTransitionLogic>,
    pub distance: f32,
}

impl NEMALogic_transitionInfo {
}

