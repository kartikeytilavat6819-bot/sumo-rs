//! Port of microsim/traffic_lights/NEMAController.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::ms_simple_traffic_light_logic::MS_SIMPLE_TRAFFIC_LIGHT_LOGIC_VT;

// shortcut: the lookup is get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR)[id] downcast from MSDetectorFileOutput to MSE2Collector, but MSDetectorFileOutput is MSE2Collector's second base (MSMoveReminder is first), so a plain Ptr::cast is wrong; upgrade when a base-offset-aware downcast exists
fn lookup_e2_detector_by_id(id: &str) -> Ptr<MSE2Collector> {
    panic!("shortcut: E2 detector lookup for '{}' needs typed MSDetectorControl::getTypedDetectors", id)
}

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
        Self {
            det: _det,
            lastGreenTime: 0,
            servedPhase: vec![false; numPhases as usize],
        }
    }

}

/// A NEMA (adaptive) traffic light logic based on E2Detector
/// C++ class `NEMALogic` (microsim/traffic_lights/NEMAController.h)
#[repr(C)]
pub struct NEMALogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    /// constant for storing the priority order for light heads. Iterates left to right and stops when finds a match.
    pub lightHeadPriority: String,
    /// whether the controller is in coordinated mode or not
    pub coordinateMode: bool,
    /// a store of the coordinated phase objects. Only used meaningfully when the controller is in coordinated mode
    pub coordinatePhaseObjs: [NEMALogic_PhasePtr; 2],
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
    pub myActivePhaseObjs: [NEMALogic_PhasePtr; 2],
    /// a vector that stores a pointer to the instantiated NEMAPhase objects
    pub myPhaseObjs: Vec<NEMALogic_PhasePtr>,
    /// an array to store the phases located at a barrier for each ring
    pub defaultBarrierPhases: [[NEMALogic_PhasePtr; 2]; 2],
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

fn nema_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<NEMALogic>().m().init(nb)
}
fn nema_try_switch(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<NEMALogic>().m().try_switch()
}
fn nema_get_current_phase_def(l: &MSTrafficLightLogic) -> &MSPhaseDefinition {
    Ptr::new(l).cast::<NEMALogic>().get().unwrap().get_current_phase_def()
}
fn nema_reset_last_switch(l: &mut MSTrafficLightLogic, t: SUMOTime) {
    Ptr::from_mut(l).cast::<NEMALogic>().m().reset_last_switch(t)
}
fn nema_get_detector_states(l: &MSTrafficLightLogic) -> BTreeMap<String, f64> {
    Ptr::new(l).cast::<NEMALogic>().get().unwrap().get_detector_states()
}

fn nema_ext_get_parameter(l: &mut MSTrafficLightLogic, key: &str, defaultValue: &str) -> String {
    Ptr::from_mut(l).cast::<NEMALogic>().m().get_parameter(key, defaultValue.to_string())
}
fn nema_ext_set_parameter(l: &mut MSTrafficLightLogic, key: &str, value: &str) {
    Ptr::from_mut(l).cast::<NEMALogic>().m().set_parameter(key, value)
}
fn nema_ext_activate_program(l: &mut MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<NEMALogic>().m().activate_program()
}
fn nema_ext_deactivate_program(l: &mut MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<NEMALogic>().m().deactivate_program()
}

static NEMA_LOGIC_EXT_VT: super::ms_traffic_light_logic::MSTrafficLightLogicExtVt = super::ms_traffic_light_logic::MSTrafficLightLogicExtVt {
    get_parameter: nema_ext_get_parameter,
    set_parameter: nema_ext_set_parameter,
    activate_program: nema_ext_activate_program,
    deactivate_program: nema_ext_deactivate_program,
    ..super::ms_traffic_light_logic::DEFAULT_MS_TRAFFIC_LIGHT_LOGIC_EXT_VT
};

static NEMA_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    try_switch: nema_try_switch,
    init: nema_init,
    get_current_phase_def: nema_get_current_phase_def,
    reset_last_switch: nema_reset_last_switch,
    get_detector_states: nema_get_detector_states,
    ..MS_SIMPLE_TRAFFIC_LIGHT_LOGIC_VT
};

impl NEMALogic {
    /// C++: `void NEMALogic::NEMALogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const MSSimpleTrafficLightLogic::Phases &, int, SUMOTime, const std::map<std::string, std::string> &, const std::string &)`
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameter: &BTreeMap<String, String>, basePath: &str) -> Self {
        let mut this = Self {
            base: MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, offset, TrafficLightType::NEMA, phases, step, delay, parameter),
            lightHeadPriority: String::new(),
            coordinateMode: false,
            coordinatePhaseObjs: [Ptr::null(); 2],
            queuedTraciChanges: false,
            offset: 0,
            myNextOffset: 0,
            myCycleLength: 0,
            myNextCycleLength: 0,
            simTime: 0,
            myNumberRings: 2,
            myPhaseStrLen: -1,
            myActivePhaseObjs: [Ptr::null(); 2],
            myPhaseObjs: Vec::new(),
            defaultBarrierPhases: [[Ptr::null(); 2]; 2],
            myDetectorForPhase: Vec::new(),
            myDetectorInfoVector: Vec::new(),
            myLaneDetectorMap: BTreeMap::new(),
            myLanePhaseMap: BTreeMap::new(),
            myDetectorLaneMap: BTreeMap::new(),
            myDetectorLength: 0.,
            myDetectorLengthLeftTurnLane: 0.,
            myShowDetectors: false,
            myFile: String::new(),
            myFreq: 0,
            myVehicleTypes: String::new(),
            rings: Vec::new(),
            phase2ControllerLanesMap: BTreeMap::new(),
            fixForceOff: false,
            cycleRefPoint: 0,
            whetherOutputState: false,
            ignoreErrors: false,
            myControllerType: NEMALogic_controllerType::Type170,
            myPhase: MSPhaseDefinition::new(phases[0].duration, phases[0].get_state(), ""),
        };
        this.base.base.vt = &NEMA_LOGIC_VT;
        this.base.base.ext_vt = &NEMA_LOGIC_EXT_VT;
        this.myDetectorLength = utils::common::string_utils::to_double(&this.get_parameter("detector-length", "20".to_string())).unwrap();
        this.myDetectorLengthLeftTurnLane = utils::common::string_utils::to_double(&this.get_parameter("detector-length-leftTurnLane", "20".to_string())).unwrap();
        let cycle_default = this.get_parameter("cycleTime", "60".to_string());
        let cycle_default = this.get_parameter("cycle-length", cycle_default);
        let cycle_str = this.get_parameter("total-cycle-length", cycle_default);
        this.myCycleLength = utils::common::time::time2steps(utils::common::string_utils::to_double(&cycle_str).unwrap());
        this.myNextCycleLength = this.myCycleLength;
        this.myDefaultCycleTime = this.myCycleLength;
        let show_default = utils::options::options_cont::get_bool("tls.actuated.show-detectors").to_string();
        let show_str = this.get_parameter("show-detectors", show_default);
        this.myShowDetectors = utils::common::string_utils::to_bool(&show_str).unwrap();
        let file_str = this.get_parameter("file", "NUL".to_string());
        this.myFile = utils::common::file_helpers::check_for_relativity(&file_str, basePath);
        let freq_str = this.get_parameter("freq", "300".to_string());
        this.myFreq = utils::common::time::time2steps(utils::common::string_utils::to_double(&freq_str).unwrap());
        this.myVehicleTypes = this.get_parameter("vTypes", String::new());
        let controller_type_str = this.get_parameter("controllerType", "TS2".to_string());
        this.myControllerType = this.parse_controller_type(controller_type_str);
        let ignore_str = this.get_parameter("ignore-errors", "false".to_string());
        this.ignoreErrors = utils::common::string_utils::to_bool(&ignore_str).unwrap();
        // This should be extended in the future.
        this.myNumberRings = 2;
        this
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
        // TODO: Create a parameter for this
        self.cycleRefPoint = utils::common::time::time2steps(0.);

        let mut barriers = self.get_parameter("barrierPhases", String::new());
        let coordinates_default = self.get_parameter("barrier2Phases", String::new());
        let mut coordinates = self.get_parameter("coordinatePhases", coordinates_default);
        let mut ring1 = self.get_parameter("ring1", String::new());
        let mut ring2 = self.get_parameter("ring2", String::new());

        let fix_str = self.get_parameter("fixForceOff", "false".to_string());
        self.fixForceOff = utils::common::string_utils::to_bool(&fix_str).unwrap();
        self.offset = self.myOffset;
        self.myNextOffset = self.myOffset;
        let whether_str = self.get_parameter("whetherOutputState", "false".to_string());
        self.whetherOutputState = utils::common::string_utils::to_bool(&whether_str).unwrap();
        let coord_str = self.get_parameter("coordinate-mode", "false".to_string());
        self.coordinateMode = utils::common::string_utils::to_bool(&coord_str).unwrap();

        // set the queued traci changes to false
        self.queuedTraciChanges = false;

        //missing parameter error
        self.error_handle_not_set(ring1.clone(), "ring1".to_string());
        self.error_handle_not_set(ring2.clone(), "ring2".to_string());
        self.error_handle_not_set(barriers.clone(), "barrierPhases".to_string());
        self.error_handle_not_set(coordinates.clone(), "barrier2Phases or coordinatePhases".to_string());

        // DEBUG_NEMA print block omitted (compile-time debug output only)

        // Construct the NEMA specific timing data types and initial phases
        self.construct_timing_and_phase_defs(&mut barriers, &mut coordinates, &mut ring1, &mut ring2);

        //init the traffic light
        MSTrafficLightLogic::init(&mut self.base.base, nb);
        assert!(!self.myLanes.is_empty());
        //iterate through the lanes and build one E2 detector for each lane associated with the traffic light control junction
        let lanes_all = self.myLanes.clone();
        for lanes in lanes_all.iter() {
            for lane in lanes.iter() {
                let lane = *lane;
                //decide the detector length
                let detector_length = if self.is_left_turn_lane(lane) {
                    self.myDetectorLengthLeftTurnLane
                } else {
                    self.myDetectorLength
                };
                if utils::common::sumo_vehicle_class::no_vehicles(lane.get_permissions()) {
                    // do not build detectors on green verges or sidewalks
                    continue;
                }
                // Build detector and register them in the detector control
                if !self.myLaneDetectorMap.contains_key(&lane) {
                    let mut det: Ptr<MSE2Collector> = Ptr::null();
                    let customID = self.get_parameter(lane.get_id(), String::new());
                    if customID != "" {
                        det = lookup_e2_detector_by_id(&customID);
                        //set the detector to be visible in gui
                        det.m().set_visible(self.myShowDetectors);
                    } else {
                        let mut phaseNumber = 0;
                        if let Some(p) = self.myLanePhaseMap.get(lane.get_id()) {
                            phaseNumber = *p;
                        }
                        let mut index = lane.get_index();
                        let mut id = format!("{}_{}_D{}.{}", self.get_id(), self.myProgramID, phaseNumber, index);
                        while MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR).contains_key(&id) {
                            index += 1;
                            id = format!("{}_{}_D{}.{}", self.get_id(), self.myProgramID, phaseNumber, index);
                        }
                        //createE2Detector() method will lead to bad detector showing in sumo-gui
                        //so it is better to use build2Detector() rather than createE2Detector()
                        let _built = nb.build_e2_detector(
                            &id,                                  // detectorID
                            lane,                                 // lane to build this detector
                            -1000000., // INVALID_POSITION: set the detector location by end point and length
                            lane.get_length(),                    // set the end position of the detector at the end of the lane
                            detector_length,                      // detector length
                            &self.myFile,                         // detector information output file
                            self.myFreq,                          // detector reading interval
                            0,                                    // time-based threshold that describes how much time has to pass until a vehicle is considered as halting
                            0.,                                   // speed threshold as halting
                            0.,                                   // minimum dist to the next standing vehicle to make this vehicle count as a participant to the jam
                            String::new(),
                            &self.myVehicleTypes,                 // vehicle types to consider, if it is empty, meaning consider all types of vehicles
                            "",                                   // nextEdges (no filtering by vehicle route)
                            PersonMode::NONE as i32,              // detector vehicles, not persons
                            true,                                 // whether to give some slack on positioning
                            self.myShowDetectors,                 // whether to show detectors in sumo-gui
                            Ptr::null(),                          // traffic light that triggers aggregation when switching
                            Ptr::null(),                          // outgoing lane that associated with the traffic light
                        );

                        //get the detector to be used in the lane detector map loading
                        det = lookup_e2_detector_by_id(&id);
                    }

                    //map the detector to lane and lane to detector
                    self.myLaneDetectorMap.insert(lane, det);
                    self.myDetectorLaneMap.insert(det, lane);
                    let numPhases = self.myPhases.len() as i32;
                    self.myDetectorInfoVector.push(NEMALogic_DetectorInfo::new(det, numPhases));
                }
            }
        }

        let p2c = self.phase2ControllerLanesMap.clone();
        for (NEMAPhaseIndex, laneIDs) in p2c.iter() {
            let mut detectors: Vec<Ptr<MSE2Collector>> = Vec::new();
            let mut detector: Ptr<MSE2Collector> = Ptr::null();
            for laneID in laneIDs.iter() {
                let lane = MSLane::dictionary_2(laneID);
                detector = *self.myLaneDetectorMap.entry(lane).or_insert(Ptr::null());
                detectors.push(detector);
            }
            // have to try this on both rings, because of the case where both rings have the same phase
            // See Basic NEMA test
            for i in 0..2usize {
                let ring_i = self.rings[i].clone();
                if self.vector_contains_phase(ring_i, *NEMAPhaseIndex) {
                    self.get_phase_obj(*NEMAPhaseIndex, i as i32).m().set_detectors(detectors.clone());
                }
            }
        }

        //Do not delete. SUMO traffic logic check.
        //SUMO check begin
        let motorized: SVCPermissions = !(SVC_PEDESTRIAN | SVC_BICYCLE);
        let mut linkToDetectors: BTreeMap<i32, BTreeSet<Ptr<MSE2Collector>>> = BTreeMap::new();
        let mut actuatedLinks: BTreeSet<i32> = BTreeSet::new();

        let numLinks = self.myLinks.len() as i32;
        let mut neverMajor = vec![true; numLinks as usize];
        let phases = self.myPhases.clone();
        for phase in phases.iter() {
            let state = phase.get_state().to_string();
            for i in 0..numLinks {
                if state.as_bytes()[i as usize] as i32 == LINKSTATE_TL_GREEN_MAJOR as i32 {
                    neverMajor[i as usize] = false;
                }
            }
        }
        let mut oneLane = vec![false; numLinks as usize];
        for i in 0..numLinks {
            for lane in self.get_lanes_at(i).clone() {
                let mut numMotorized = 0;
                for l in lane.get_edge().get_lanes().clone() {
                    if (l.get_permissions() & motorized) != 0 {
                        numMotorized += 1;
                    }
                }
                if numMotorized == 1 {
                    oneLane[i as usize] = true;
                    break;
                }
            }
        }

        for phase in phases.iter() {
            let phaseIndex = self.myDetectorForPhase.len() as i32;
            // determinism: std::set<MSE2Collector*> orders by pointer address, as does BTreeSet on Ptr
            let mut detectors: BTreeSet<Ptr<MSE2Collector>> = BTreeSet::new();
            if phase.is_actuated() {
                let state = phase.get_state().to_string();
                let mut greenLinks: BTreeSet<i32> = BTreeSet::new();
                // determinism: std::map<MSE2Collector*, ...> orders by pointer address
                let mut detectorLinks: BTreeMap<Ptr<MSE2Collector>, BTreeSet<i32>> = BTreeMap::new();

                for i in 0..numLinks {
                    let c = state.as_bytes()[i as usize] as i32;
                    if c == LINKSTATE_TL_GREEN_MAJOR as i32
                        || (c == LINKSTATE_TL_GREEN_MINOR as i32
                            && ((neverMajor[i as usize] // check1a
                                && self.has_major(&state, self.get_lanes_at(i))) // check1b
                                || oneLane[i as usize])) // check1c
                    {
                        greenLinks.insert(i);
                        actuatedLinks.insert(i);
                    }

                    for lane in self.get_lanes_at(i).clone() {
                        if self.myLaneDetectorMap.contains_key(&lane) {
                            let d = self.myLaneDetectorMap[&lane];
                            detectorLinks.entry(d).or_default().insert(i);
                        }
                    }
                }
                // std::map iteration visits keys inserted during the loop; walk by key so the same holds here
                let mut cursor: Option<Ptr<MSE2Collector>> = None;
                loop {
                    let next_entry = match cursor {
                        None => detectorLinks.iter().next().map(|(d, l)| (*d, l.clone())),
                        Some(c) => detectorLinks
                            .range((std::ops::Bound::Excluded(c), std::ops::Bound::Unbounded))
                            .next()
                            .map(|(d, l)| (*d, l.clone())),
                    };
                    let (det, links) = match next_entry {
                        Some(e) => e,
                        None => break,
                    };
                    cursor = Some(det);
                    let detectorLane = *self.myDetectorLaneMap.entry(det).or_insert(Ptr::null());
                    let mut usable = true;
                    // check 1
                    for j in links.iter() {
                        if !greenLinks.contains(j) {
                            usable = false;
                        }
                    }

                    //check 2
                    if usable {
                        for link in detectorLane.get_link_cont().clone() {
                            let next = link.get_lane();
                            if self.myLaneDetectorMap.contains_key(&next) {
                                let nextDet = self.myLaneDetectorMap[&next];
                                let nextLinks = detectorLinks.entry(nextDet).or_default().clone();
                                for j in nextLinks.iter() {
                                    if !greenLinks.contains(j) {
                                        usable = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if usable {
                        detectors.insert(det);
                        for j in links.iter() {
                            linkToDetectors.entry(*j).or_default().insert(det);
                        }
                    }
                }
                if detectors.is_empty() {
                    utils::common::msg_handler::write_warning(&format!(
                        "At NEMA tlLogic '{}', actuated phase {} has no controlling detector",
                        self.get_id(),
                        phaseIndex
                    ));
                }
            }
            let detectorInfos: Vec<Ptr<NEMALogic_DetectorInfo>> = Vec::new();
            self.myDetectorForPhase.push(detectorInfos);
            for det in detectors.iter() {
                for detInfo in self.myDetectorInfoVector.iter_mut() {
                    if detInfo.det == *det {
                        self.myDetectorForPhase.last_mut().unwrap().push(Ptr::from_mut(detInfo));
                        detInfo.servedPhase[phaseIndex as usize] = true;
                    }
                }
            }
        }

        for i in actuatedLinks.iter() {
            let noDetector = linkToDetectors.entry(*i).or_default().is_empty();
            let idx = *i as usize;
            if noDetector
                && self.myLinks[idx].len() > 0
                && (self.myLinks[idx][0].get_lane_before().get_permissions() & motorized) != 0
            {
                utils::common::msg_handler::write_warning(&format!(
                    "At NEMA tlLogic '{}, linkIndex {} has no controlling detector",
                    self.get_id(),
                    i
                ));
            }
        }
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
        // delete the phase objects (destructor only; the memory is owned by the container that allocated it)
        for p in self.myPhaseObjs.clone() {
            p.m().drop_();
        }
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
        let mut nextPhases: [Ptr<PhaseTransitionLogic>; 2] = [Ptr::null(), Ptr::null()];

        // update the internal time. This is a must. Could have just used a reference to the time
        self.set_current_time();

        // Check the Detectors
        for p in self.myPhaseObjs.clone() {
            p.m().check_my_detectors();
        }

        // Update the timing parameters
        let controller = Ptr::from_mut(self);
        for p in self.myActivePhaseObjs.clone() {
            p.m().update(controller);
        }

        // Calculate the Next Phases, but only if atleast one of them is ready to transition
        if self.myActivePhaseObjs[0].readyToSwitch || self.myActivePhaseObjs[1].readyToSwitch {
            let mut transitions: NEMALogic_TransitionPairs = Vec::new();
            // set the next phases by reference
            self.get_next_phases(&mut transitions);

            // Sort the next phases by distance and select the closest.
            // determinism: std::sort is not stable; Rust sort_by is stable, so ties may order differently
            if transitions.len() > 1 {
                transitions.sort_by(|i, j| {
                    if i.distance < j.distance {
                        std::cmp::Ordering::Less
                    } else if j.distance < i.distance {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
            }

            // Set the Next Phases = to the transition with least combined distance
            nextPhases[0] = transitions[0].p1;
            nextPhases[1] = transitions[0].p2;

            // Try the exit logic. This doesn't necessarily mean that the phase will exit,
            // as it could go into green rest or green transfer, but this is considered an "exit"
            for p in self.myActivePhaseObjs.clone() {
                if p.readyToSwitch {
                    p.m().exit(controller, &mut nextPhases);
                }
            }

            // This is the only time when something might have happened, so we update the phase strings here
            let newState = self.compose_light_string();
            if newState.as_str() != self.myPhase.get_state() {
                self.myPhase.set_state(&newState);
                let name = format!("{}+{}", self.myActivePhaseObjs[0].phaseName, self.myActivePhaseObjs[1].phaseName);
                self.myPhase.set_name(&name);
                // ensure that SwitchCommand::execute notices a change
                self.myStep = 1 - self.myStep;
            }
        }

        // clear the phases' detectors
        for p in self.myPhaseObjs.clone() {
            p.m().clear_my_detectors();
        }

        // FUZZ_TESTING barrier assertion omitted (compile-time test hook only)

        // return the simulation timestep, as this controller must be checked every simulation step
        utils::common::time::delta_t()
    }

    /// C++: `const MSPhaseDefinition & NEMALogic::getCurrentPhaseDef() const` [const, virtual]
    /// Returns myPhase, which doesn't correspond to a NEMA phase, but rather the composite light string
    // const MSPhaseDefinition&
    // NEMALogic::getCurrentPhaseDef() const {
    //     return myPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        &self.myPhase
    }

    /// C++: `void NEMALogic::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // NEMALogic::resetLastSwitch(SUMOTime t) {
    //     myPhase.myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        self.myPhase.myLastSwitch = t;
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
        MSTrafficLightLogic::activate_program_base(&mut self.base.base);
        for item in self.myLaneDetectorMap.values() {
            item.m().set_visible(true);
        }
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
        MSTrafficLightLogic::deactivate_program_base(&mut self.base.base);
        for item in self.myLaneDetectorMap.values() {
            item.m().set_visible(false);
        }
    }

    /// C++: `bool NEMALogic::showDetectors() const` [const]
    // bool showDetectors() const {
    //         return myShowDetectors;
    //     }
    #[allow(unused_variables)]
    pub fn show_detectors(&self) -> bool {
        self.myShowDetectors
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
        self.myShowDetectors = show;
        for item in self.myLaneDetectorMap.values() {
            item.m().set_visible(self.myShowDetectors);
        }
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
        let mut result: BTreeMap<String, f64> = BTreeMap::new();
        for (det, _lane) in self.myDetectorLaneMap.iter() {
            result.insert(det.base2.base.get_id().to_string(), det.get_current_vehicle_number() as f64);
        }
        result
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
        let controller = Ptr::from_mut(self);
        let mut potentialPhases: Vec<Vec<Ptr<PhaseTransitionLogic>>> = Vec::new();

        // Get a vector of each phases' potential transitions
        for p in self.myActivePhaseObjs.clone() {
            potentialPhases.push(p.m().try_switch(controller));
        }

        // Loop through all combination of transitions, keeping only the valid ones and filling in the gaps where necessary
        for r1_t in potentialPhases[0].clone() {
            for r2_t in potentialPhases[1].clone() {
                // if both transitions go to the same barrier then we are good
                if r1_t.get_to_phase().barrierNum == r2_t.get_to_phase().barrierNum {
                    let distance = (r1_t.m().get_distance(r2_t) + r2_t.m().get_distance(r1_t)) as f32 / 2.0;
                    transitions.push(NEMALogic_transitionInfo { p1: r1_t, p2: r2_t, distance });
                } else {
                    // If the rings are different, add a choice where one of them is the default choice for whatever ring it is
                    // create two choices, one for each of the phase as they are on  different rings
                    if r1_t.get_from_phase().readyToSwitch {
                        // get the r2 default
                        let r2_t_temp = self.get_default_transition(r2_t, r1_t);
                        // only add it if it does not cross a barrier!
                        if r2_t_temp.get_to_phase().barrierNum == r1_t.get_to_phase().barrierNum {
                            let distance = (r2_t_temp.m().get_distance(r1_t) + r1_t.m().get_distance(r2_t_temp)) as f32 / 2.0;
                            transitions.push(NEMALogic_transitionInfo { p1: r1_t, p2: r2_t_temp, distance });
                        }
                    }
                    if r2_t.get_from_phase().readyToSwitch {
                        // R1 default
                        let r1_t_temp = self.get_default_transition(r1_t, r2_t);
                        // only add it if it does not cross a barrier!
                        if r1_t_temp.get_to_phase().barrierNum == r2_t.get_to_phase().barrierNum {
                            let distance = (r2_t.m().get_distance(r1_t_temp) + r1_t_temp.m().get_distance(r2_t)) as f32 / 2.0;
                            transitions.push(NEMALogic_transitionInfo { p1: r1_t_temp, p2: r2_t, distance });
                        }
                    }
                    // If the distances are <= 1, this means that this is the shortest transition possible
                    // and we can return early without considering any other options
                    if let Some(last) = transitions.last() {
                        if last.distance < 1.0 {
                            return;
                        }
                    }
                }
            }
        }
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
        let mut c = a - b;
        while c >= b {
            c = c - b;
        }
        while c < 0 {
            c += b;
        }
        c
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
        // C++ std::set `output` is built but never read, so it is not ported
        for i in 0..self.myLinks.len() {
            if self.myLinks[i].is_empty() {
                // unused index
                continue;
            }
            let ch = state.as_bytes()[i];
            // if the ch is 'G', it means that the phase is controlling this lane
            if ch == b'G' {
                stateIndex.push(i as i32);
                for link in self.myLinks[i].clone() {
                    let incoming = link.get_lane_before();
                    if incoming.is_normal() {
                        laneIDs.push(incoming.get_id().to_string());
                    }
                }
            }
        }
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
        for p in self.myPhaseObjs.clone() {
            let idx = (p.phaseName - 1) as usize;
            if newMaxGreens[idx] > 0. {
                // set the phase's nextMaxDuration. This will be implemented when implementTraciChanges is called
                p.m().nextMaxDuration = utils::common::time::time2steps(newMaxGreens[idx]);
            }
        }
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
        assert!(newSplits.len() == 8);
        for p in self.myPhaseObjs.clone() {
            let idx = (p.phaseName - 1) as usize;
            if newSplits[idx] > 0. {
                // set the phase's nextMaxDuration. This will be implemented when implementTraciChanges is called
                p.m().nextMaxDuration = utils::common::time::time2steps(newSplits[idx]) - p.yellow - p.red;
            }
        }
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
        // set the controller's next cycle length. This will be implemented when implementTraciChanges is called
        self.myNextCycleLength = utils::common::time::time2steps(newCycleLength);
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
        // set the controller's offset. This will be implemented when implementTraciChanges is called
        self.myNextOffset = utils::common::time::time2steps(newOffset);
    }

    /// C++: `SUMOTime NEMALogic::getCurrentCycleLength()`
    /// Get the current cycle length
    // SUMOTime getCurrentCycleLength() {
    //         return myCycleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_cycle_length(&mut self) -> SUMOTime {
        self.myCycleLength
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
        self.queuedTraciChanges = true;
        if utils::common::string_utils::starts_with(key, "NEMA.") {
            if key == "NEMA.splits" || key == "NEMA.maxGreens" {
                //splits="2.0 3.0 4.0 5.0 2.0 3.0 4.0 5.0"
                let tmp = utils::common::string_tokenizer::StringTokenizer::new(value).get_vector();
                if tmp.len() != 8 {
                    self.queuedTraciChanges = false;
                    panic!("{}", InvalidArgument(format!("Parameter '{}' for NEMA controller '{}' requires 8 space or comma separated values", key, self.get_id())).0);
                }
                let mut timing: Vec<f64> = Vec::new();
                for s in tmp.iter() {
                    timing.push(utils::common::string_utils::to_double(s).unwrap());
                }
                if key == "NEMA.maxGreens" {
                    self.set_new_max_greens(timing);
                } else {
                    self.set_new_splits(timing);
                }
            } else if key == "NEMA.cycleLength" {
                self.set_new_cycle_length(utils::common::string_utils::to_double(value).unwrap());
            } else if key == "NEMA.offset" {
                self.set_new_offset(utils::common::string_utils::to_double(value).unwrap());
            } else {
                self.queuedTraciChanges = false;
                panic!("{}", InvalidArgument(format!("Unsupported parameter '{}' for NEMA controller '{}'", key, self.get_id())).0);
            }
        }
        self.base2.set_parameter(key, value);
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
        if utils::common::string_utils::starts_with(key, "NEMA.") {
            if key == "NEMA.phaseCall" {
                let mut activeCalls = [0i32; 8];
                for p in self.myPhaseObjs.iter() {
                    // This handles the case when the controller has multiple of the same phase call
                    let idx = (p.phaseName - 1) as usize;
                    if activeCalls[idx] == 0 {
                        activeCalls[idx] = 1 * (p.lastDetectActive as i32);
                    }
                }
                let outStr: Vec<String> = activeCalls.iter().map(|c| c.to_string()).collect();
                return outStr.join(",");
            } else {
                panic!("{}", InvalidArgument(format!("Unsupported parameter '{}' for NEMA controller '{}'", key, self.get_id())).0);
            }
        } else {
            self.base2.get_parameter(key, &defaultValue)
        }
    }

    /// C++: `SUMOTime NEMALogic::getCurrentTime() const` [const]
    /// Wrapper Function to Simplify Accessing Time
    // inline SUMOTime getCurrentTime(void) const {
    //         return simTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_time(&self) -> SUMOTime {
        self.simTime
    }

    /// C++: `SUMOTime NEMALogic::getTimeInCycle() const` [const]
    /// override Function to Simplify Accessing Offset Cycle Time
    // inline SUMOTime getTimeInCycle() const {
    //         return (simTime - cycleRefPoint - offset) % myCycleLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_in_cycle(&self) -> SUMOTime {
        (self.simTime - self.cycleRefPoint - self.offset) % self.myCycleLength
    }

    /// C++: `void NEMALogic::setActivePhase(PhasePtr)`
    /// set the active phase
    // void
    // NEMALogic::setActivePhase(PhasePtr phase) {
    //     myActivePhaseObjs[phase->ringNum] = phase;
    // }
    #[allow(unused_variables)]
    pub fn set_active_phase(&mut self, phase: NEMALogic_PhasePtr) {
        self.myActivePhaseObjs[phase.ringNum as usize] = phase;
    }

    /// C++: `PhasePtr NEMALogic::getActivePhase(int)`
    /// Get the Active Phase object for a specified ring
    // inline PhasePtr getActivePhase(int ringNum) {
    //         return myActivePhaseObjs[ringNum];
    //     }
    #[allow(unused_variables)]
    pub fn get_active_phase(&mut self, ringNum: i32) -> NEMALogic_PhasePtr {
        self.myActivePhaseObjs[ringNum as usize]
    }

    /// C++: `std::vector<PhasePtr> NEMALogic::getPhasesByRing(int)`
    /// get all phases for a given ring
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases_by_ring(&mut self, ringNum: i32) -> Vec<NEMALogic_PhasePtr> {
        let mut phases: Vec<NEMALogic_PhasePtr> = Vec::new();
        for p in self.myPhaseObjs.iter() {
            if p.ringNum == ringNum {
                phases.push(*p);
            }
        }
        phases
    }

    /// C++: `PhasePtr NEMALogic::getPhaseObj(int, int)`
    /// get the phase object matching the phaseNum If ringNum is passed, it will only search for the phase in the given ring
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phase_obj(&mut self, phaseNum: i32, ringNum: i32) -> NEMALogic_PhasePtr {
        // This satisfies the case where there is a "duplicate" phase on both ring
        let iterRing = if ringNum >= 0 { self.get_phases_by_ring(ringNum) } else { self.myPhaseObjs.clone() };
        for p in iterRing {
            if p.phaseName == phaseNum {
                return p;
            }
        }
        // the phase must always be found
        panic!(
            "At traffic signal '{}' program '{}' phase '{}' not found in ring '{}'.",
            self.get_id(),
            self.myProgramID,
            phaseNum,
            ringNum
        );
    }

    /// C++: `std::vector<PhasePtr> NEMALogic::getPhaseObjs()`
    /// get a vector of all phase objects
    // inline std::vector<PhasePtr> getPhaseObjs(void) {
    //         return myPhaseObjs;
    //     }
    #[allow(unused_variables)]
    pub fn get_phase_objs(&mut self) -> Vec<NEMALogic_PhasePtr> {
        self.myPhaseObjs.clone()
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
        let length = self.rings[ringNum as usize].len() as i32;
        let mut d = 0;
        let mut found = false;
        // Loop around the ring and keep track of the distance from p1 to p2
        for i in 0..(length * 2) {
            let v = self.rings[ringNum as usize][(i % length) as usize];
            if v > 0 {
                if found {
                    d += 1;
                    if v == p2 {
                        break;
                    }
                } else if v == p1 {
                    found = true;
                }
            }
        }
        assert!(d > 0);
        d
    }

    /// C++: `bool NEMALogic::isType170() const` [const]
    /// checks if the controller is of type170
    // inline bool isType170(void) const {
    //         return myControllerType == Type170;
    //     }
    #[allow(unused_variables)]
    pub fn is_type170(&self) -> bool {
        self.myControllerType == NEMALogic_controllerType::Type170
    }

    /// C++: `PhasePtr NEMALogic::getOtherPhase(PhasePtr)`
    /// Get the opposite active phase
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_other_phase(&mut self, p: NEMALogic_PhasePtr) -> NEMALogic_PhasePtr {
        // return a pointer to the other active phase
        self.myActivePhaseObjs[if p.ringNum == 0 { 1 } else { 0 }]
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
        // Implement Traci Updates on the start of ring1 coordinated phase (rising edge of it turning green)
        if self.queuedTraciChanges {
            for p in self.myPhaseObjs.clone() {
                p.m().maxDuration = p.nextMaxDuration;
            }
            self.offset = self.myNextOffset;
            self.myCycleLength = self.myNextCycleLength;
            // now that we have set the cycle length, offset and max duration, we need to update force off times
            self.calculate_force_offs();
            self.queuedTraciChanges = false;
        }
    }

    /// C++: `void NEMALogic::setCurrentTime()` [protected]
    /// Set the simTime
    // inline void setCurrentTime(void) {
    //         simTime = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    #[allow(unused_variables)]
    pub fn set_current_time(&mut self) {
        self.simTime = MSNet::get_instance().get_current_time_step();
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
        // read in the barrier and coordinated phases from the XML
        let barrierPhases = self.read_para_from_string(barriers.clone());
        let coordinatePhases = self.read_para_from_string(coordinates.clone());

        // create a {{}, {}} vector of phases
        let ring1Phases = self.read_para_from_string(ring1.clone());
        self.rings.push(ring1Phases);
        let ring2Phases = self.read_para_from_string(ring2.clone());
        self.rings.push(ring2Phases);

        // load the recalls, if they exist
        let minRecallStr = self.get_parameter("minRecall", "1,2,3,4,5,6,7,8".to_string());
        let VecMinRecall = self.read_para_from_string(minRecallStr);
        let maxRecallStr = self.get_parameter("maxRecall", String::new());
        let VecMaxRecall = self.read_para_from_string(maxRecallStr);

        let controller = Ptr::from_mut(self);

        // loop through the rings and construct NEMAPhases.
        // This relies on the phase being in order in the rings parameter in the configuration file
        let mut ringNum: i32 = 0;
        let mut lastPhaseIter: i32 = 0;
        let mut phaseIter: i32 = 0;
        let rings = self.rings.clone();
        for r in rings.iter() {
            let mut ringIter: i32 = 0;
            lastPhaseIter = phaseIter;
            phaseIter = 0;
            for &p in r.iter() {
                if p != 0 {
                    // find the phase definition matching the phase integer
                    let mut tempPhase: Ptr<MSPhaseDefinition> = Ptr::null();
                    for pDef in self.myPhases.clone() {
                        if self.string2int(pDef.get_name().to_string()) == p {
                            tempPhase = pDef;
                            break;
                        }
                    }
                    // there must be a matching MSPhaseDefinition
                    if tempPhase.is_null() {
                        panic!("At traffic signal '{}' program '{}' no phase named '{}' was found", self.get_id(), self.myProgramID, p);
                    }

                    // create lane specific objects
                    let state = tempPhase.get_state().to_string();

                    // check that all phases have the same length. myPhaseStrLen is initially set to -1.
                    if self.myPhaseStrLen < 0 {
                        self.myPhaseStrLen = state.len() as i32;
                    } else if self.myPhaseStrLen != state.len() as i32 {
                        panic!("At NEMA tlLogic '{}', different sizes of NEMA phase states. Please check the NEMA XML", self.get_id());
                    }

                    // get the lane-based info
                    let mut laneIDs: StringVector = Vec::new();
                    let mut controlledStateIndexes: IntVector = Vec::new();
                    self.get_lane_info_from_nema_state(state.clone(), &mut laneIDs, &mut controlledStateIndexes);

                    let mut laneIDs_vector: Vec<String> = Vec::new();
                    for laneID in laneIDs.iter() {
                        laneIDs_vector.push(laneID.clone());
                        self.myLanePhaseMap.insert(laneID.clone(), p);
                    }
                    self.phase2ControllerLanesMap.insert(p, laneIDs_vector);

                    // Create the Phase Object
                    // find if it is at a barrier
                    let barrierPhase = self.vector_contains_phase(barrierPhases.clone(), p) || self.vector_contains_phase(coordinatePhases.clone(), p);
                    // is it a coordinate phase
                    let coordinatePhase = self.vector_contains_phase(coordinatePhases.clone(), p) && self.coordinateMode;
                    // is there a minimum or max recall
                    let minRecall = self.vector_contains_phase(VecMinRecall.clone(), p);
                    let maxRecall = self.vector_contains_phase(VecMaxRecall.clone(), p);
                    // A phase can "green rest" only if it has a recall and no other phases on that ring do OR if NO phases have a recall (unique case)
                    let mut phaseGreenRest = (VecMaxRecall.len() + VecMinRecall.len()) < 1;
                    if !phaseGreenRest {
                        let mut recallActive = minRecall || maxRecall;
                        if recallActive {
                            for &pO in r.iter() {
                                if pO != p {
                                    if self.vector_contains_phase(VecMinRecall.clone(), pO) || self.vector_contains_phase(VecMaxRecall.clone(), pO) {
                                        recallActive = false;
                                        break;
                                    }
                                }
                            }
                            // only set the green rest to true if I am the only phase on my ring with a recall
                            phaseGreenRest = recallActive;
                        }
                    }
                    // could add per-phase fixforceoff here
                    // barrierNum is either 0 or 1, depending on mainline side or sidestreet
                    let barrierNum = ringIter / 2;

                    // now ready to create the phase
                    // shortcut: NEMAPhase objects are leaked boxes owned for the whole simulation, like the C++ new
                    let phaseObj = NEMAPhase::new(
                        p,
                        barrierPhase,
                        phaseGreenRest,
                        coordinatePhase,
                        minRecall,
                        maxRecall,
                        self.fixForceOff,
                        barrierNum,
                        ringNum,
                        controlledStateIndexes,
                        tempPhase,
                    );
                    self.myPhaseObjs.push(Ptr::from_mut(Box::leak(Box::new(phaseObj))));

                    // Add a reference to the sequentially prior phase
                    if phaseIter > 0 {
                        let prior = self.myPhaseObjs[(lastPhaseIter + (phaseIter - 1)) as usize];
                        self.myPhaseObjs.last().unwrap().m().set_sequential_prior_phase(prior);
                    }
                    phaseIter += 1;
                }
                ringIter += 1;
            }
            if lastPhaseIter >= self.myPhaseObjs.len() as i32 {
                panic!("At traffic signal '{}', ring {} contains only '0'", self.get_id(), ringNum + 1);
            }
            // Set the first to point to the last, wrapping around the ring.
            let first = self.myPhaseObjs[lastPhaseIter as usize];
            let last = self.myPhaseObjs[(lastPhaseIter + phaseIter - 1) as usize];
            first.m().set_sequential_prior_phase(last);
            // index the ring counter
            ringNum += 1;
        }

        //TODO: set the default phases. This could also be set using dual entry in future
        for i in 0..2usize {
            // create the coordinate phase ptr
            let coordPhase = self.get_phase_obj(coordinatePhases[i], i as i32);
            self.coordinatePhaseObjs[i] = coordPhase;
            self.defaultBarrierPhases[i][coordPhase.barrierNum as usize] = coordPhase;
            // create the other barrier phase ptr
            let b = self.get_phase_obj(barrierPhases[i], i as i32);
            self.defaultBarrierPhases[i][b.barrierNum as usize] = b;
            // the barrier 1 and barrier 0 default phase must not have the same barrier number
            if b.barrierNum == self.coordinatePhaseObjs[i].barrierNum {
                panic!(
                    "At traffic signal {} the barrier and coordinated phases {}, {} are located on the same side of a barrier. Please check your configuration file",
                    self.get_id(),
                    b.phaseName,
                    self.coordinatePhaseObjs[i].barrierNum
                );
            }
        }

        // Create the PhaseDetectorInfo for each of the phases (needs knowledge of other phases to create)
        let latchingStr = self.get_parameter("latchingDetectors", String::new());
        let latchingDetectors = self.read_para_from_string(latchingStr);
        let mut cp: Vec<(i32, i32)> = Vec::new();
        for p in self.myPhaseObjs.clone() {
            let cps = format!("crossPhaseSwitching:{}", p.phaseName);
            let crossPhase = utils::common::string_utils::to_int(&self.get_parameter(&cps, "0".to_string())).unwrap();
            if crossPhase > 0 {
                cp.push((p.phaseName, crossPhase));
            }
        }

        // Knowing the cross phase info, we can add that to the phase
        for p in self.myPhaseObjs.clone() {
            let latching = self.vector_contains_phase(latchingDetectors.clone(), p.phaseName);
            let mut cpTarget = 0;
            let mut cpSource = 0;
            for cp_pair in cp.iter() {
                if cp_pair.0 == p.phaseName || cp_pair.1 == p.phaseName {
                    cpTarget = cp_pair.0;
                    cpSource = cp_pair.1;
                }
            }
            p.m().init(controller, cpTarget, cpSource, latching);
        }

        // Calculate Force offs Based on Timing
        self.calculate_force_offs();

        if self.coordinateMode {
            // Calculate the Initial Phases in coordinated operation only.
            // Otherwise they have already been calculated above
            self.calculate_initial_phases();
        } else {
            // Fall back being the barrier 0 default phases
            let defaultP0 = self.get_phases_by_ring(0)[0];
            let defaultP1 = self.get_phases_by_ring(1)[0];
            defaultP0.m().force_enter(controller);
            defaultP1.m().force_enter(controller);
        }

        // Set the initial light state
        let composed = self.compose_light_string();
        self.myPhase.set_state(&composed);
        let name = format!("{}+{}", self.myActivePhaseObjs[0].phaseName, self.myActivePhaseObjs[1].phaseName);
        self.myPhase.set_name(&name);
        let simstep = MSNet::get_instance().get_current_time_step();
        self.set_traffic_light_signals(simstep);
        self.myStep = 0;

        //validating timing
        self.validate_timing();
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
        // FIX with plan to support #10742
        let mut out: Vec<u8> = vec![b'r'; self.myPhaseStrLen as usize];
        for i in 0..self.myPhaseStrLen {
            let mut controlled = false;
            let mut phaseChars: Vec<i8> = Vec::new();
            for p in self.myActivePhaseObjs.clone() {
                phaseChars.push(p.m().get_nema_char(i));
                if p.m().controlled_index(i) {
                    out[i as usize] = p.m().get_nema_char(i) as u8;
                    controlled = true;
                }
            }
            // if the index wasn't a controlled one, the prior priority order still stands
            if !controlled {
                for priority_char in self.lightHeadPriority.bytes() {
                    if phaseChars.contains(&(priority_char as i8)) {
                        out[i as usize] = priority_char;
                        break;
                    }
                }
            }
        }
        String::from_utf8_lossy(&out).into_owned()
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
        v.contains(&phaseNum)
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
    pub fn has_major(&self, state: &str, lanes: &MSTrafficLightLogic_LaneVector) -> bool {
        for i in 0..state.len() {
            if state.as_bytes()[i] as i32 == LINKSTATE_TL_GREEN_MAJOR as i32 {
                for cand in self.get_lanes_at(i as i32).iter() {
                    for lane in lanes.iter() {
                        if *lane == *cand {
                            return true;
                        }
                    }
                }
            }
        }
        false
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
        let mut output: Vec<i32> = Vec::new();
        for c in s.chars() {
            if c >= '0' && c <= '9' {
                let temp = c as i32 - '0' as i32;
                output.push(temp);
            }
        }
        output
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
        let links = lane.get_link_cont().clone();
        if links.len() == 1 && links[0].get_direction() == LinkDirection::LEFT {
            return true;
        }
        false
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
        // emulates `std::stringstream ss(s) >> ret` with ret = 0 on failure
        let bytes = s.trim_start().as_bytes().to_vec();
        let mut i = 0usize;
        let mut neg = false;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            neg = bytes[i] == b'-';
            i += 1;
        }
        let start = i;
        let mut v: i64 = 0;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            v = (v * 10 + (bytes[i] - b'0') as i64).min(i32::MAX as i64 + 1);
            i += 1;
        }
        if i == start {
            0
        } else if neg {
            (-v).max(i32::MIN as i64) as i32
        } else {
            v.min(i32::MAX as i64) as i32
        }
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
        let p = t.get_from_phase();
        // if the current phase is not ready to switch or a barrier cross is desired by the other transition
        // and t fromPhase is not ready to switch, the default transition is back to myself
        if !p.readyToSwitch || (p.barrierNum == ot.get_to_phase().barrierNum && p.get_current_state() >= LightState::Green) {
            return p.m().get_transition(p.phaseName);
        }
        // otherwise the default transition is to the default phase on whatever barrier ot wants to transition to
        let target = self.defaultBarrierPhases[p.ringNum as usize][ot.get_to_phase().barrierNum as usize].phaseName;
        p.m().get_transition(target)
    }

    /// C++: `controllerType NEMALogic::parseControllerType(std::string)` [protected]
    /// parse the controllerType from the tllogic description
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn parse_controller_type(&mut self, inputType: String) -> NEMALogic_controllerType {
        let mut cleanString = String::new();
        for c in inputType.chars() {
            if c.is_ascii_alphabetic() || c.is_ascii_digit() {
                cleanString.push(c.to_ascii_lowercase());
            }
        }
        if cleanString == "type170" {
            NEMALogic_controllerType::Type170
        } else if cleanString == "ts2" {
            NEMALogic_controllerType::TS2
        } else {
            panic!("{}", InvalidArgument(format!("Please set controllerType for NEMA tlLogic {} to either Type170 or TS2", self.get_id())).0);
        }
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
        if param_variable == "" {
            panic!("{}", InvalidArgument(format!("Please set {} for NEMA tlLogic '{}'", param_name, self.get_id())).0);
        }
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
        // check that the cycle length for each ring adds up to the specified cycle length
        for ringIndex in 0..2usize {
            let mut cycleLengthCalculated: SUMOTime = 0;
            for p in self.get_phases_by_ring(ringIndex as i32) {
                cycleLengthCalculated += p.maxDuration + p.yellow + p.red;
            }
            if self.coordinateMode && (cycleLengthCalculated != self.myCycleLength) {
                let ringNumber = ringIndex + 1;
                let error = format!("At NEMA tlLogic '{}', Ring {} does not add to cycle length.", self.get_id(), ringNumber);
                if self.ignoreErrors {
                    utils::common::msg_handler::write_warning(&error);
                } else {
                    panic!("{}", error);
                }
            }
        }
        // check that the barriers sum together
        let mut cycleLengths: [[SUMOTime; 2]; 2] = [[0, 0], [0, 0]];
        for ringIndex in 0..2usize {
            // TS2 Force Offs don't go in order, so using a different method to check cycle time
            for p in self.get_phases_by_ring(ringIndex as i32) {
                cycleLengths[ringIndex][p.barrierNum as usize] += p.maxDuration + p.yellow + p.red;
            }
        }
        // Write warnings if the barriers do not sum
        for barrierNum in 0..2usize {
            if cycleLengths[0][barrierNum] != cycleLengths[1][barrierNum] {
                let error = format!(
                    "At NEMA tlLogic '{}', the phases before barrier {} from both rings do not add up. (ring1={}, ring2={})",
                    self.get_id(),
                    barrierNum + 1,
                    utils::common::to_string::to_string(&utils::common::time::steps2time(cycleLengths[0][barrierNum])),
                    utils::common::to_string::to_string(&utils::common::time::steps2time(cycleLengths[1][barrierNum]))
                );
                if self.coordinateMode && !self.ignoreErrors {
                    panic!("{}", error);
                } else {
                    utils::common::msg_handler::write_warning(&error);
                }
            }
        }

        // no offset for non coordinated
        if !self.coordinateMode && self.offset != 0 {
            utils::common::msg_handler::write_warning(&format!("NEMA tlLogic '{}' is not coordinated but an offset was set.", self.get_id()));
        }
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
        // TS2 "0" cycle time is the start of the "first" coordinated phases.
        // We can find this "0" point by first constructing the forceOffs in sequential order via the 170 method
        self.calculate_force_offs170();

        // Switch the Force Off Times to align with TS2 Cycle, which is the *start* of the earliest coordinated phase
        // The coordinate phases will always be the defaultBarrierPhases[i][0]
        let c0 = self.coordinatePhaseObjs[0];
        let c1 = self.coordinatePhaseObjs[1];
        let minCoordTime = std::cmp::min(c0.forceOffTime - c0.maxDuration, c1.forceOffTime - c1.maxDuration);

        // loop through all the phases and subtract this minCoordTime to move the 0 point to the start of the first coordinated phase
        for p in self.myPhaseObjs.clone() {
            if (p.forceOffTime - minCoordTime) >= 0 {
                p.m().forceOffTime -= minCoordTime;
            } else {
                p.m().forceOffTime = self.myCycleLength + (p.forceOffTime - minCoordTime);
            }
            let gs = self.mode_cycle(p.greatestStartTime - minCoordTime, self.myCycleLength);
            p.m().greatestStartTime = gs;
        }
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
        let mut zeroTime: [SUMOTime; 2] = [utils::common::time::time2steps(0.), utils::common::time::time2steps(0.)];
        for i in 0..2usize {
            let mut runningTime: SUMOTime = 0;
            // loop through the phases for ring 0 and then 1
            for p in self.get_phases_by_ring(i as i32) {
                runningTime += p.maxDuration + p.m().get_transition_time_stateless();
                // in 170, the cycle "starts" when the coordinated phase goes to yellow.
                // See https://ops.fhwa.dot.gov/publications/fhwahop08024/chapter6.html
                if p.coordinatePhase {
                    zeroTime[i] = runningTime;
                }
                let forceOff = runningTime - p.m().get_transition_time_stateless();
                p.m().forceOffTime = forceOff;
                p.m().greatestStartTime = forceOff - p.minDuration;
            }
        }
        // find the minimum offset time and then subtract from everything, modecycling where negative
        // This sets the 0 cycle time as start of yellow on earliest ending coordinated phase
        let minCoordYellow = std::cmp::min(zeroTime[0], zeroTime[1]);
        for p in self.myPhaseObjs.clone() {
            let forceOff = self.mode_cycle(p.forceOffTime - minCoordYellow, self.myCycleLength);
            p.m().forceOffTime = forceOff;
            let greatestStart = self.mode_cycle(p.greatestStartTime - minCoordYellow, self.myCycleLength);
            p.m().greatestStartTime = greatestStart;
        }
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
        match self.myControllerType {
            NEMALogic_controllerType::Type170 => self.calculate_force_offs170(),
            NEMALogic_controllerType::TS2 => self.calculate_force_offs_ts2(),
        }
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
        // Modifications where made to 170 algorithm so that it works with both.
        self.calculate_initial_phases170();
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
        let controller = Ptr::from_mut(self);
        // get the time in the cycle
        let cycleTime = self.mode_cycle(self.get_time_in_cycle(), self.myCycleLength);
        let mut activePhases: [Ptr<NEMAPhase>; 2] = [Ptr::null(), Ptr::null()];
        for i in 0..2usize {
            let mut ringCopy = self.get_phases_by_ring(i as i32);
            // sort by the minimum start time in the cycle.
            // determinism: C++ std::sort with the non-strict `<=` comparator is unspecified on ties; a stable sort on greatestStartTime is used
            ringCopy.sort_by(|a, b| a.greatestStartTime.cmp(&b.greatestStartTime));
            let mut found = false;
            // loop through the sorted phases by time and try to find the phase that should be active given the time in the cycle
            for p in ringCopy.iter() {
                // This handles the wrap around. Checks if the prior phases start time should have already happened.
                // If it should have happened and it's not to the start time of me yet, start in my phase ( will have to be in my phase longer than max time likely )
                let priorPhase = p.m().get_sequential_prior_phase();
                let syntheticPriorStart = if priorPhase.greatestStartTime < p.greatestStartTime {
                    priorPhase.greatestStartTime
                } else {
                    priorPhase.greatestStartTime - self.myCycleLength
                };
                if cycleTime <= self.mode_cycle(p.greatestStartTime, self.myCycleLength) && cycleTime > self.mode_cycle(syntheticPriorStart, self.myCycleLength) {
                    found = true;
                    activePhases[i] = *p;
                    break;
                }
            }
            if !found {
                // DEBUG_NEMA warning omitted
                activePhases[0] = self.defaultBarrierPhases[0][0];
                activePhases[1] = self.defaultBarrierPhases[1][0];
            }
        }

        // ensure that the two found phases are on the same side of the barrier. If they aren't, just override with the default barrier phases
        if activePhases[0].barrierNum != activePhases[1].barrierNum {
            // give preference to whatever is on the coordinate side of the barrier, one must be if they aren't equal to each other
            activePhases[0] = if activePhases[0].barrierNum == 0 { activePhases[0] } else { self.defaultBarrierPhases[0][0] };
            activePhases[1] = if activePhases[1].barrierNum == 0 { activePhases[1] } else { self.defaultBarrierPhases[1][0] };
        }

        // force enter the phases and update their expected duration to last until the forceOff
        activePhases[0].m().force_enter(controller);
        activePhases[1].m().force_enter(controller);
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
        match self.myControllerType {
            NEMALogic_controllerType::Type170 => self.calculate_initial_phases170(),
            NEMALogic_controllerType::TS2 => self.calculate_initial_phases_ts2(),
        }
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
        let mut this = Self {
            phaseName,
            isAtBarrier: isBarrier,
            isGreenRest,
            barrierNum,
            coordinatePhase: isCoordinated,
            minRecall,
            maxRecall,
            fixForceOff,
            ringNum,
            lastDetectActive: false,
            greenRestTimer: 0,
            greatestStartTime: 0,
            forceOffTime: 0,
            readyToSwitch: false,
            yellow: 0,
            red: 0,
            minDuration: 0,
            maxDuration: 0,
            nextMaxDuration: 0,
            vehExt: 0,
            myCorePhase: phase,
            // C++ sets myInstance = this, but nothing reads it and the address would change on return; left null
            myInstance: Ptr::null(),
            myLastPhaseInstance: Ptr::null(),
            sequentialPriorPhase: Ptr::null(),
            myLightState: LightState::Red,
            myDetectorInfo: NEMAPhase_PhaseDetectorInfo::new(),
            maxGreenDynamic: 0,
            myStartTime: 0,
            myExpectedDuration: 0,
            myLastEnd: 0,
            myPhaseStringInds: phaseStringInds,
            myGreenString: String::new(),
            myYellowString: String::new(),
            myRedString: String::new(),
            myTransitions: Vec::new(),
            transitionActive: false,
            lastTransitionDecision: Ptr::null(),
        };

        // Timing Parameters
        this.maxGreenDynamic = this.myCorePhase.maxDuration;
        this.myStartTime = utils::common::time::time2steps(0.);
        this.myExpectedDuration = this.myCorePhase.minDuration;
        this.myLastEnd = utils::common::time::time2steps(0.);

        // set the phase colors
        this.set_my_nema_states();
        this
    }

    /// C++: `void NEMAPhase::~NEMAPhase()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // Delete the transitions from their alloc
        for t in self.myTransitions.clone() {
            t.m().drop_();
        }
    }

    /// C++: `LightState NEMAPhase::getCurrentState() const` [const]
    /// gets the current light state
    // inline LightState getCurrentState() const {
    //         return myLightState;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_state(&self) -> LightState {
        self.myLightState
    }

    /// C++: `std::vector<MSE2Collector *> NEMAPhase::getDetectors() const` [const]
    /// returns a vector of the phases detectors
    // inline std::vector<MSE2Collector*> getDetectors() const {
    //         return myDetectorInfo.detectors;
    //     }
    #[allow(unused_variables)]
    pub fn get_detectors(&self) -> Vec<Ptr<MSE2Collector>> {
        self.myDetectorInfo.detectors.clone()
    }

    /// C++: `void NEMAPhase::setDetectors(std::vector<MSE2Collector *>)`
    /// sets the detectors for the phase
    // inline void setDetectors(std::vector<MSE2Collector*> detectors) {
    //         myDetectorInfo.detectors = detectors;
    //     }
    #[allow(unused_variables)]
    pub fn set_detectors(&mut self, detectors: Vec<Ptr<MSE2Collector>>) {
        self.myDetectorInfo.detectors = detectors;
    }

    /// C++: `bool NEMAPhase::isTransitionActive() const` [const]
    /// check if a transition is active
    // inline bool isTransitionActive() const {
    //         return myLightState < LightState::Green;
    //     }
    #[allow(unused_variables)]
    pub fn is_transition_active(&self) -> bool {
        self.transitionActive
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
        // switch the durations from steps2time
        self.recalculate_timing();

        let this = Ptr::from_mut(self);
        for p in controller.m().get_phases_by_ring(self.ringNum) {
            // construct transitions for all potential movements, including back to myself
            let transition = Ptr::from_mut(Box::leak(Box::new(PhaseTransitionLogic::new(this, p))));
            let distance = controller.m().measure_ring_distance(self.phaseName, p.phaseName, self.ringNum);
            transition.m().set_distance(distance);
            self.myTransitions.push(transition);
        }

        // sort the transitions by distance for speed later. Using plain distance here
        // determinism: std::sort is unstable; Rust sort_by is stable, so equal distances may order differently
        self.myTransitions.sort_by(|i, j| i.distance.cmp(&j.distance));

        // create the phase detector info
        let cpdSource = if crossPhaseSource > 0 { controller.m().get_phase_obj(crossPhaseSource, -1) } else { Ptr::null() };
        let cpdTarget = if crossPhaseTarget > 0 { controller.m().get_phase_obj(crossPhaseTarget, -1) } else { Ptr::null() };
        self.myDetectorInfo = NEMAPhase_PhaseDetectorInfo::new_2(latching, cpdSource, cpdTarget);
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
        let this = Ptr::from_mut(self);
        let dt = utils::common::time::delta_t();
        // If I am in a transition, the rest of the update logic does not matter
        if self.myLightState < LightState::Green {
            // return early
            self.readyToSwitch = true;
            return;
        }

        // Continuation Logic
        let duration = controller.m().get_current_time() - self.myStartTime;
        // Check the vehicle extension timer as long as not in green transfer and not a coordinated phase
        if self.myLightState != LightState::GreenXfer && !self.coordinatePhase {
            self.myExpectedDuration = self.calc_vehicle_extension(duration);
        }
        // Special Logic for Green Rest, which behaves uniquely
        if self.myLightState == LightState::GreenRest {
            // check all other detectors and see if anything else is active. If so,
            // start the green rest timer countdown, which is == to the max duration of the phase
            let mut vehicleActive = false;
            for p in controller.m().get_phase_objs() {
                if p.phaseName != self.phaseName && p.phaseName != controller.m().get_other_phase(this).phaseName && p.m().call_active() {
                    self.greenRestTimer -= dt;
                    vehicleActive = true;
                    break;
                }
            }
            // catch the rising edge of the sidestreet detection and calculate the maximum timer
            if vehicleActive && (self.greenRestTimer + dt >= self.maxDuration) {
                self.maxGreenDynamic = self.minDuration + self.maxDuration;
            }

            // if there are no other vehicles slide the startTime along
            if !vehicleActive {
                self.greenRestTimer = self.maxDuration;
                if duration >= self.minDuration {
                    self.myStartTime = controller.m().get_current_time() - self.minDuration;
                    self.maxGreenDynamic = self.minDuration + self.maxDuration;
                    self.myExpectedDuration = self.minDuration + std::cmp::max(0, self.myExpectedDuration - duration);
                }
            }

            // if the green rest timer is exhausted, set ready to switch
            if self.greenRestTimer < dt {
                self.readyToSwitch = true;
                // force the counterpart to be ready to switch too. This needs to be latching....
                let otherPhase = controller.m().get_other_phase(this);
                if otherPhase.get_current_state() > LightState::Green {
                    otherPhase.m().readyToSwitch = true;
                }
            }

            // Special Behavior when the Green Rest Circles all the way around in coordinated mode
            if self.coordinatePhase {
                // This means that we have green rested until I should "start" again. Just call my entry function again.
                if controller.m().get_time_in_cycle() <= ((self.forceOffTime - self.maxDuration) + dt / 2) {
                    self.enter(controller, this);
                }
            }
        }
        // Check to see if a switch is desired
        if duration >= self.myExpectedDuration {
            self.readyToSwitch = true;
        }
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
    pub fn exit(&mut self, controller: Ptr<NEMALogic>, nextPhases: &mut [Ptr<PhaseTransitionLogic>; 2]) {
        let this = Ptr::from_mut(self);
        let r = self.ringNum as usize;
        if nextPhases[r].get_to_phase() != this {
            // if the next phase is not me, then I need to go into a transition
            self.lastTransitionDecision = nextPhases[r];
            if self.myLightState >= LightState::Green {
                // if I am in green, then I need to enter yellow
                self.enter_yellow(controller);
                return;
            }

            let sinceEnd = controller.m().get_current_time() - self.myLastEnd;
            if sinceEnd < (self.yellow + self.red) {
                if sinceEnd >= self.yellow {
                    // if I am in yellow, then I need to enter red
                    self.myLightState = LightState::Red;
                }
                // I am currently in the Red state but haven't reached max
                return;
            }

            self.handle_red_xfer_or_next_phase(controller, nextPhases);
            return;
        }

        self.handle_green_rest_or_transfer(controller, nextPhases);
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
    pub fn handle_green_rest_or_transfer(&mut self, controller: Ptr<NEMALogic>, nextPhases: &mut [Ptr<PhaseTransitionLogic>; 2]) {
        let this = Ptr::from_mut(self);
        let otherPhase = controller.m().get_other_phase(this);
        self.readyToSwitch = false;
        // !ringNum in C++ is 1 for ring 0 and 0 for ring 1
        let notRing = if self.ringNum == 0 { 1 } else { 0 };
        let isOtherPhaseReady = nextPhases[notRing].get_to_phase() == otherPhase && otherPhase.readyToSwitch;
        let isOtherPhaseInGreenRest = otherPhase.greenRestTimer >= otherPhase.maxDuration && otherPhase.get_current_state() == LightState::GreenRest;

        if isOtherPhaseReady || isOtherPhaseInGreenRest {
            self.myLightState = LightState::GreenRest;
            self.myStartTime = controller.m().get_current_time() - self.minDuration;
            self.myExpectedDuration = self.minDuration;
            self.greenRestTimer = self.maxDuration * (self.isGreenRest as SUMOTime);
        } else {
            self.myLightState = LightState::GreenXfer;
            if self.isAtBarrier {
                self.myExpectedDuration = (otherPhase.myExpectedDuration + otherPhase.myStartTime) - self.myStartTime;
            }
        }
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
        self.myLastEnd = controller.m().get_current_time();
        self.myLightState = LightState::Yellow;
        self.transitionActive = true;
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
    pub fn handle_red_xfer_or_next_phase(&mut self, controller: Ptr<NEMALogic>, nextPhases: &mut [Ptr<PhaseTransitionLogic>; 2]) {
        let this = Ptr::from_mut(self);
        let otherPhase = controller.m().get_other_phase(this);
        let r = self.ringNum as usize;
        let o = otherPhase.ringNum as usize;
        let barrierCross = nextPhases[r].get_to_phase().barrierNum != self.barrierNum;
        let barrierCrossButOkay = barrierCross
            && (nextPhases[r].get_to_phase().barrierNum == nextPhases[o].get_to_phase().barrierNum)
            && otherPhase.m().okay2_force_switch(controller);

        if !barrierCross {
            nextPhases[r].get_to_phase().m().enter(controller, this);
            return;
        }

        if barrierCrossButOkay {
            // if the barrier is crossed, but the other phase is going to the same barrier, then I can enter red transfer
            // enter the next phase
            nextPhases[r].get_to_phase().m().enter(controller, this);
            // trigger the other phase to enter red transfer
            nextPhases[o].get_to_phase().m().enter(controller, this);
            return;
        }

        self.myLightState = LightState::RedXfer;
        self.readyToSwitch = true;
        self.transitionActive = false;
    }

    /// C++: `bool NEMAPhase::hasRecall()`
    /// simple method to check if there is a recall on the phase.
    // inline bool hasRecall(void) {
    //         return minRecall || maxRecall;
    //     }
    #[allow(unused_variables)]
    pub fn has_recall(&mut self) -> bool {
        self.minRecall || self.maxRecall
    }

    /// C++: `bool NEMAPhase::callActive()`
    /// simple method to check if there is either a recall or an active detector
    // inline bool callActive(void) {
    //         return minRecall || maxRecall || myDetectorInfo.detectActive;
    //     }
    #[allow(unused_variables)]
    pub fn call_active(&mut self) -> bool {
        self.minRecall || self.maxRecall || self.myDetectorInfo.detectActive
    }

    /// C++: `bool NEMAPhase::detectActive()`
    /// simple method to check if a detector is active
    // inline bool detectActive(void) {
    //         return myDetectorInfo.detectActive;
    //     }
    #[allow(unused_variables)]
    pub fn detect_active(&mut self) -> bool {
        self.myDetectorInfo.detectActive
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
        // Check my Detectors, only necessary if it isn't currently marked as on
        if !self.myDetectorInfo.detectActive {
            // If I have a cross phase target and it is active and I am not, save my detector as not active
            if !self.myDetectorInfo.cpdTarget.is_null() {
                if self.myDetectorInfo.cpdTarget.get_current_state() >= LightState::Green {
                    if self.myLightState < LightState::Green {
                        self.myDetectorInfo.detectActive = false;
                        return;
                    }
                }
            }
            // If we make it to this point, check my detector like normal.
            for d in self.myDetectorInfo.detectors.clone() {
                if d.get_current_vehicle_number() > 0 {
                    self.myDetectorInfo.detectActive = true;
                    return;
                }
            }
            // If my detector is not active, check my cross phase
            if !self.myDetectorInfo.cpdSource.is_null() && (self.myLightState >= LightState::Green) {
                if self.myDetectorInfo.cpdSource.get_current_state() < LightState::Green {
                    for d in self.myDetectorInfo.cpdSource.m().get_detectors() {
                        if d.get_current_vehicle_number() > 0 {
                            self.myDetectorInfo.detectActive = true;
                            return;
                        }
                    }
                }
            }
        }
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
        self.lastDetectActive = self.myDetectorInfo.detectActive;
        // remove the active flag on the detector if the detector is not latching or if it is green
        if (!self.myDetectorInfo.latching) || (self.myLightState >= LightState::Green) {
            self.myDetectorInfo.detectActive = false;
        }
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
        if self.myLightState == LightState::RedXfer {
            // if in red xfer, I am ready to switch whenevery
            return utils::common::time::time2steps(0.);
        }
        if !self.transitionActive {
            // if a transition is not active, the transition is just yellow + red time
            return self.get_transition_time_stateless();
        }
        // if a transition is active, then return the time left in the transition
        std::cmp::max(utils::common::time::time2steps(0.), (self.yellow + self.red) - (controller.m().get_current_time() - self.myLastEnd))
    }

    /// C++: `SUMOTime NEMAPhase::getTransitionTimeStateless()`
    /// Get the Transition time given
    // inline SUMOTime getTransitionTimeStateless(void) {
    //         return yellow + red;
    //     }
    #[allow(unused_variables)]
    pub fn get_transition_time_stateless(&mut self) -> SUMOTime {
        self.yellow + self.red
    }

    /// C++: `PhasePtr NEMAPhase::getSequentialPriorPhase()`
    /// get the prior phase
    // inline PhasePtr getSequentialPriorPhase(void) {
    //         return sequentialPriorPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_sequential_prior_phase(&mut self) -> NEMALogic_PhasePtr {
        self.sequentialPriorPhase
    }

    /// C++: `void NEMAPhase::setSequentialPriorPhase(PhasePtr)`
    /// set the prior phase
    // inline void setSequentialPriorPhase(PhasePtr priorPhase) {
    //         sequentialPriorPhase = priorPhase;
    //     }
    #[allow(unused_variables)]
    pub fn set_sequential_prior_phase(&mut self, priorPhase: NEMALogic_PhasePtr) {
        self.sequentialPriorPhase = priorPhase;
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
        // this function returns the preferred valid transition for the phase
        let mut nextTransitions: Vec<Ptr<PhaseTransitionLogic>> = Vec::new();
        if self.readyToSwitch {
            // only try to switch if I am ready to switch
            for t in self.myTransitions.clone() {
                // for the transitions check if it is okay
                if t.m().okay(controller) {
                    // if there was already a transition decision, it can be overriden but only if the new transition
                    // is on the same side of a barrier
                    if !self.lastTransitionDecision.is_null() {
                        if t.get_to_phase().barrierNum == self.lastTransitionDecision.get_to_phase().barrierNum {
                            nextTransitions.push(t);
                            break;
                        }
                    } else {
                        nextTransitions.push(t);
                        // break once there is a valid option (they have already been sorted)
                        break;
                    }
                }
            }
        }
        // Add in the last transition decision if it hasn't been added in yet
        if !self.lastTransitionDecision.is_null() {
            let mut found = false;
            let mut sameBarrier = false;
            for t in nextTransitions.iter() {
                if *t == self.lastTransitionDecision {
                    found = true;
                    break;
                }
                if t.get_to_phase().barrierNum == self.lastTransitionDecision.get_to_phase().barrierNum {
                    sameBarrier = true;
                    break;
                }
            }
            // but only need to add it if it is not in the list AND if nothing in the list is the same barrier as it was.
            if !found && !sameBarrier {
                nextTransitions.push(self.lastTransitionDecision);
            }
        }
        // Add the transition back to myself, but only in the case when no others have been added
        if nextTransitions.is_empty() {
            nextTransitions.push(*self.myTransitions.last().unwrap());
        }
        nextTransitions
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
        for t in self.myTransitions.clone() {
            if t.get_to_phase().phaseName == toPhase {
                return t;
            }
        }
        // This point should never be reached (assert(0) in C++ is a no-op in release builds, so fall back to the front)
        self.myTransitions[0]
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
        // TODO: this can be computed once.
        if self.myLightState >= LightState::Green {
            self.myGreenString.as_bytes()[i as usize] as i8
        } else if self.myLightState <= LightState::Red {
            self.myRedString.as_bytes()[i as usize] as i8
        } else {
            self.myYellowString.as_bytes()[i as usize] as i8
        }
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
        // This could be extended in the future to allow for traci manipulation
        self.yellow = self.myCorePhase.yellow;
        self.red = self.myCorePhase.red;
        self.minDuration = self.myCorePhase.minDuration;
        self.maxDuration = self.myCorePhase.maxDuration;
        self.nextMaxDuration = self.myCorePhase.maxDuration;
        self.maxGreenDynamic = self.myCorePhase.maxDuration;
        self.vehExt = self.myCorePhase.vehext;
    }

    /// C++: `void NEMAPhase::forceEnter(NEMALogic *)`
    /// Force Enter. This Should only be called at initialization time
    // inline void forceEnter(NEMALogic* controller) {
    //         enter(controller, sequentialPriorPhase);
    //     }
    #[allow(unused_variables)]
    pub fn force_enter(&mut self, controller: Ptr<NEMALogic>) {
        let seq = self.sequentialPriorPhase;
        self.enter(controller, seq);
    }

    /// C++: `bool NEMAPhase::controlledIndex(int)`
    /// Return whether or not the phase index is controlled by me
    // inline bool controlledIndex(int i) {
    //         return std::count(myPhaseStringInds.begin(), myPhaseStringInds.end(), i) > 0;
    //     }
    #[allow(unused_variables)]
    pub fn controlled_index(&mut self, i: i32) -> bool {
        self.myPhaseStringInds.contains(&i)
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
        self.transitionActive = false;
        self.readyToSwitch = false;
        self.myLightState = LightState::Red;
    }

    /// C++: `bool NEMAPhase::okay2ForceSwitch(NEMALogic *)`
    /// simple internal check to see if done okay to transition
    // inline bool okay2ForceSwitch(NEMALogic* controller) {
    //         return readyToSwitch && !transitionActive && (getTransitionTime(controller) <= TIME2STEPS(0));
    //     }
    #[allow(unused_variables)]
    pub fn okay2_force_switch(&mut self, controller: Ptr<NEMALogic>) -> bool {
        self.readyToSwitch && !self.transitionActive && (self.get_transition_time(controller) <= utils::common::time::time2steps(0.))
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
        if self.myExpectedDuration < self.maxGreenDynamic && self.myDetectorInfo.detectActive {
            // add the vehicle extension timer if the detector is active.
            // capped by the minimum and maximum duration
            return std::cmp::min(std::cmp::max(duration + self.vehExt, self.minDuration), self.maxGreenDynamic);
        }
        self.myExpectedDuration
    }

    /// C++: `void NEMAPhase::enter(NEMALogic *, PhasePtr)` [private]
    /// handles entry to the phase during simulation Sets the color to green and determines maximum duration
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn enter(&mut self, controller: Ptr<NEMALogic>, lastPhase: NEMALogic_PhasePtr) {
        let this = Ptr::from_mut(self);
        // set the last phase instance to inactive
        lastPhase.m().cleanup_exit();

        // Enter the phase
        self.myStartTime = controller.m().get_current_time();
        self.myLightState = LightState::Green;
        self.myLastPhaseInstance = lastPhase;
        self.readyToSwitch = false;

        // implement the new timing parameters on the first coordinated phase to appear
        let coordPhaseName = controller.m().coordinatePhaseObjs[self.ringNum as usize].phaseName;
        if self.phaseName == coordPhaseName {
            controller.m().implement_traci_changes();
        }

        // Handle Green Rest Peculiarities
        if !controller.coordinateMode && self.isGreenRest {
            // If the controller is in free mode and the phase is a green rest phase, then it should enter as "green rest"
            self.myLightState = LightState::GreenRest;
            // if the phase has "green rest" capabilities, set its timer to the dynamic maxGreen
            self.greenRestTimer = self.maxDuration * (self.isGreenRest as SUMOTime);
        }

        // clear the last transition decision
        self.lastTransitionDecision = Ptr::null();

        // Calculate the Max Green Time & Expected Duration here:
        if controller.coordinateMode {
            if self.coordinatePhase {
                let timeInCycle = controller.m().get_time_in_cycle();
                let cycleLength = controller.m().get_current_cycle_length();
                self.myExpectedDuration = controller.m().mode_cycle(self.forceOffTime - timeInCycle, cycleLength);
            } else {
                let timeInCycle = controller.m().get_time_in_cycle();
                let cycleLength = controller.m().get_current_cycle_length();
                self.maxGreenDynamic = controller.m().mode_cycle(self.forceOffTime - timeInCycle, cycleLength);
                if !self.fixForceOff {
                    self.maxGreenDynamic = std::cmp::min(self.maxDuration, self.maxGreenDynamic);
                }
                self.myExpectedDuration = self.minDuration;
            }
        } else {
            self.myExpectedDuration = self.minDuration;
        }
        // Implements the maxRecall functionality
        if self.maxRecall && !self.coordinatePhase {
            self.myExpectedDuration = self.maxGreenDynamic;
        }
        // Set the controller's active phase
        controller.m().set_active_phase(this);
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
        self.myGreenString = self.myCorePhase.get_state().to_string();
        self.myRedString = String::new();
        self.myYellowString = String::new();
        for ch in self.myGreenString.clone().chars() {
            self.myRedString.push('r');
            if ch == 'G' || ch == 'g' {
                self.myYellowString.push('y');
            } else {
                self.myYellowString.push(ch);
            }
        }
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
        Self { detectors: Vec::new(), cpdTarget: Ptr::null(), cpdSource: Ptr::null(), detectActive: false, latching: false }
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
        Self { detectors: Vec::new(), cpdTarget, cpdSource, detectActive: false, latching }
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
        Self { distance: 0, fromPhase, toPhase }
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
        // Picking the correct transition logic to use
        if self.fromPhase == self.toPhase {
            // for green rest or green transfer, it cannot return to itself if a transition is active
            self.fromPhase.get_current_state() >= LightState::Green
        } else if self.fromPhase.coordinatePhase {
            // if the from phase is a coordinated phase i.e. {2, 6} in a standard setup
            self.from_coord(controller)
        } else if self.fromPhase.isAtBarrier {
            // if the phase is at a barrier i.e. {2, 6, 4, 8} in a standard setup
            self.from_barrier(controller)
        } else if controller.coordinateMode {
            // typical coordinate mode transition,
            self.coord_base(controller)
        } else {
            // base transition logic
            self.free_base(controller)
        }
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
        // Returns the other transitions distance in green transfer situations
        if (self.toPhase == self.fromPhase) && (otherTrans.toPhase.barrierNum == self.toPhase.barrierNum) {
            if self.toPhase.get_current_state() == LightState::Green || self.toPhase.get_current_state() == LightState::GreenXfer {
                return otherTrans.distance;
            }
        }
        self.distance
    }

    /// C++: `void PhaseTransitionLogic::setDistance(int)`
    /// set the transition distance
    // inline void setDistance(int d) {
    //         distance = d;
    //     }
    #[allow(unused_variables)]
    pub fn set_distance(&mut self, d: i32) {
        self.distance = d;
    }

    /// C++: `void PhaseTransitionLogic::~PhaseTransitionLogic() noexcept`
    /// deconstructor
    // ~PhaseTransitionLogic() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the object is owned by the leaked allocation made in NEMAPhase::init, so nothing is released here
    }

    /// C++: `PhasePtr PhaseTransitionLogic::getToPhase() const` [const]
    /// get the to phase
    // inline PhasePtr getToPhase(void) const {
    //         return toPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_phase(&self) -> NEMALogic_PhasePtr {
        self.toPhase
    }

    /// C++: `PhasePtr PhaseTransitionLogic::getFromPhase() const` [const]
    /// get the from phase
    // inline PhasePtr getFromPhase(void) const {
    //         return fromPhase;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_phase(&self) -> NEMALogic_PhasePtr {
        self.fromPhase
    }

    /// C++: `void PhaseTransitionLogic::buildLogic()` [private]
    /// build the transition logic based on the from and to phase
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_logic(&mut self) {
        // C++ declares buildLogic() but never defines it (no body in NEMAController.cpp), so there is nothing to port
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
        if self.free_base(controller) {
            if self.fromPhase.barrierNum == self.toPhase.barrierNum {
                // same barrier side so we are good.
                // Check if green transfer is active. If so, we need to make sure that there are no calls on the other side of the barrier
                if self.fromPhase.get_current_state() >= LightState::Green {
                    let fromRing = self.fromPhase.ringNum;
                    let fromBarrier = self.fromPhase.barrierNum;
                    for p in controller.m().get_phases_by_ring(fromRing) {
                        if p.barrierNum != fromBarrier && p.m().call_active() {
                            return false;
                        }
                    }
                }
                return true;
            } else {
                // This is now a barrier cross and we need to make sure that the other phase is also ready to transition
                if self.fromPhase.readyToSwitch && controller.m().get_other_phase(self.fromPhase).readyToSwitch {
                    return true;
                }
            }
        }
        false
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
        if self.coord_base(controller) {
            // Determine if the other phase is also ready to switch
            if controller.m().get_other_phase(self.fromPhase).readyToSwitch {
                // Dr. Wang had the Type-170 code setup in a way that it could transition whenever - meaning that it didn't matter if the prior phase could fit or not
                if controller.m().is_type170() {
                    return true;
                }
                // If the transition is already active, then report that the movement is possible
                if self.fromPhase.m().is_transition_active() {
                    return true;
                }
                // now determine if there my prior phase can fit or not. We already know that I can fit.
                let priorPhase = self.toPhase.m().get_sequential_prior_phase();
                let timeInCycle = controller.m().get_time_in_cycle();
                let cycleLength = controller.m().get_current_cycle_length();
                let timeTillForceOff = controller.m().mode_cycle(priorPhase.forceOffTime - timeInCycle, cycleLength);
                let transitionTime = self.fromPhase.m().get_transition_time(controller);
                // if the time till the force off is less than the min duration ||
                // if it is greater than the cycle length minus the length of the coordinate phase (which the fromPhase automatically is)
                if (priorPhase.minDuration + transitionTime) > timeTillForceOff || timeTillForceOff > (controller.m().get_current_cycle_length() - self.fromPhase.minDuration) {
                    return true;
                }
            }
        }
        false
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
        // Simplest transition logic. Just check if a detector (or recall) is active on that phase and
        let mut okay = false;
        // is a call active on the toPhase?
        if self.toPhase.m().call_active() {
            // would the transition be a barrier cross?
            if self.fromPhase.barrierNum != self.toPhase.barrierNum {
                let otherPhase = controller.m().get_other_phase(self.fromPhase);
                // If it is a barrier cross, then the other phase must also be ready to switch
                // or have a transition time that is lower than mine currently. DELTA_T is critical here
                if otherPhase.readyToSwitch {
                    // #&& otherPhase->getTransitionTime(controller) <= fromPhase->getTransitionTime(controller)) {
                    okay = true;
                }
            } else {
                okay = true;
            }
        }
        okay
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
        if self.toPhase.coordinatePhase && (controller.m().get_other_phase(self.fromPhase).readyToSwitch || self.fromPhase.barrierNum == self.toPhase.barrierNum) {
            // transitions TO the coordinated phase may always happen, as long as the other phase is okay to switch too
            return true;
        }
        // first check if the free logic is upheld
        else if self.free_base(controller) {
            // Then check if the "to phase" can fit, which means that there is enough time to fit the current transition + the minimum time of the next phase
            let transitionTime = self.fromPhase.m().get_transition_time(controller);
            let timeInCycle = controller.m().get_time_in_cycle();
            let cycleLength = controller.m().get_current_cycle_length();
            let timeTillForceOff = controller.m().mode_cycle(self.toPhase.forceOffTime - timeInCycle, cycleLength);
            if self.toPhase.minDuration + transitionTime <= timeTillForceOff {
                return true;
            }
        }
        false
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

