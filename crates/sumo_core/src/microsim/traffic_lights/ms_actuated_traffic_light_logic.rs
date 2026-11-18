//! Port of microsim/traffic_lights/MSActuatedTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::tuple<std::string, std::string, std::string>> MSActuatedTrafficLightLogic::AssignmentMap`
pub type MSActuatedTrafficLightLogic_AssignmentMap = Vec<(String, String, String)>;

/// C++ typedef `Parameterised::Map MSActuatedTrafficLightLogic::ConditionMap`
pub type MSActuatedTrafficLightLogic_ConditionMap = utils::common::parameterised::ParameterisedMap;

// C++ #define constants of MSActuatedTrafficLightLogic.cpp
const DEFAULT_MAX_GAP: &str = "3.0";
const DEFAULT_PASSING_TIME: &str = "1.9";
const DEFAULT_DETECTOR_GAP: &str = "2.0";
const DEFAULT_INACTIVE_THRESHOLD: &str = "180";
const DEFAULT_CURRENT_PRIORITY: i32 = 10;
const DEFAULT_CROSSING_PRIORITY: i32 = 100;
const DEFAULT_LENGTH_WITH_GAP: f64 = 7.5;
const DEFAULT_STATIC_MINDUR: SUMOTime = 0;
const NO_DETECTOR: &str = "NO_DETECTOR";
const DEFAULT_CONDITION: &str = "DEFAULT";
// C++ MSActuatedTrafficLightLogic::OPERATOR_PRECEDENCE
const OPERATOR_PRECEDENCE: &[&str] = &["**", "^", "*", "/", "+", "-", "%", "=", "==", "!=", "<", ">", "<=", ">=", "and", "&&", "or", "||"];
use super::ms_phase_definition::OVERRIDE_DURATION;
use super::ms_simple_traffic_light_logic::{simple_get_current_phase_def, simple_get_current_phase_index, simple_get_index_from_offset, simple_get_offset_from_index, simple_get_phase, simple_get_phase_index_at_time, simple_get_phase_number, simple_get_phases, simple_load_state, simple_map_time_in_cycle, simple_reset_last_switch, simple_save_state};
// C++ RGBColor::ORANGE / RGBColor::GREEN (the loop keeps a pointer to them, so they need statics)
static RGB_ORANGE: utils::common::rgb_color::RGBColor = utils::common::rgb_color::RGBColor::ORANGE;
static RGB_GREEN: utils::common::rgb_color::RGBColor = utils::common::rgb_color::RGBColor::GREEN;
static RGB_RED: utils::common::rgb_color::RGBColor = utils::common::rgb_color::RGBColor::RED;

/// C++ DEFAULT_BIKE_LENGTH_WITH_GAP: (getDefaultVehicleLength(SVC_BICYCLE) + 0.5)
fn default_bike_length_with_gap() -> f64 {
    get_default_vehicle_length(SVC_BICYCLE) + 0.5
}

/// C++ typedef `std::map<std::string, Function> MSActuatedTrafficLightLogic::FunctionMap`
pub type MSActuatedTrafficLightLogic_FunctionMap = BTreeMap<String, MSActuatedTrafficLightLogic_Function>;

/// C++ typedef `std::vector<std::vector<InductLoopInfo *>> MSActuatedTrafficLightLogic::InductLoopMap`
pub type MSActuatedTrafficLightLogic_InductLoopMap = Vec<Vec<Ptr<MSActuatedTrafficLightLogic_InductLoopInfo>>>;

/// C++ class `MSActuatedTrafficLightLogic::Function` (microsim/traffic_lights/MSActuatedTrafficLightLogic.h)
#[derive(Clone)]
pub struct MSActuatedTrafficLightLogic_Function {
    pub id: String,
    pub nArgs: i32,
    pub assignments: MSActuatedTrafficLightLogic_AssignmentMap,
}

impl MSActuatedTrafficLightLogic_Function {
    /// C++: `void Function::Function(const std::string &, int)`
    // Function(const std::string& _id = "", int _nArgs = -1):
    //             id(_id), nArgs(_nArgs) {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _nArgs: i32) -> Self {
        MSActuatedTrafficLightLogic_Function {
            id: _id.to_string(),
            nArgs: _nArgs,
            assignments: Vec::new(),
        }
    }

}

/// C++ class `MSActuatedTrafficLightLogic::InductLoopInfo` (microsim/traffic_lights/MSActuatedTrafficLightLogic.h)
pub(crate) fn actuated_save_state(l: &MSTrafficLightLogic, out: &mut OutputDevice) {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().save_state(out)
}

pub struct MSActuatedTrafficLightLogic_InductLoopInfo {
    pub loop_: Ptr<MSInductLoop>,
    pub lane: Ptr<MSLane>,
    pub lastGreenTime: SUMOTime,
    pub servedPhase: Vec<bool>,
    pub maxGap: f64,
    pub jamThreshold: f64,
}

impl MSActuatedTrafficLightLogic_InductLoopInfo {
    /// C++: `void InductLoopInfo::InductLoopInfo(MSInductLoop *, const MSLane *, int, double, double)`
    // InductLoopInfo(MSInductLoop* _loop, const MSLane* _lane, int numPhases, double _maxGap, double _jamThreshold):
    //             loop(_loop),
    //             lane(_lane),
    //             servedPhase(numPhases, false),
    //             maxGap(_maxGap),
    //             jamThreshold(_jamThreshold)
    //         {}
    #[allow(unused_variables)]
    pub fn new(_loop: Ptr<MSInductLoop>, _lane: Ptr<MSLane>, numPhases: i32, _maxGap: f64, _jamThreshold: f64) -> Self {
        MSActuatedTrafficLightLogic_InductLoopInfo {
            loop_: _loop,
            lane: _lane,
            lastGreenTime: 0,
            servedPhase: vec![false; numPhases as usize],
            maxGap: _maxGap,
            jamThreshold: _jamThreshold,
        }
    }

    /// C++: `bool InductLoopInfo::isJammed() const` [const]
    // bool isJammed() const {
    //             return jamThreshold > 0 && loop->getOccupancyTime() >= jamThreshold;
    //         }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        self.jamThreshold > 0.0 && self.loop_.get_occupancy_time() >= self.jamThreshold
    }

}

/// An actuated (adaptive) traffic light logic
/// C++ class `MSActuatedTrafficLightLogic` (microsim/traffic_lights/MSActuatedTrafficLightLogic.h)
#[repr(C)]
pub struct MSActuatedTrafficLightLogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    /// A map from phase to induction loops to be used for gap control
    pub myInductLoopsForPhase: MSActuatedTrafficLightLogic_InductLoopMap,
    pub myCrossingsForPhase: Vec<Vec<Ptr<MSLink>>>,
    pub myInductLoops: Vec<MSActuatedTrafficLightLogic_InductLoopInfo>,
    /// extra loops for output/tracking
    pub myExtraLoops: Vec<Ptr<MSInductLoop>>,
    pub myExtraE2: Vec<Ptr<MSE2Collector>>,
    /// The maximum gap to check in seconds
    pub myMaxGap: f64,
    /// The minimum continuous occupancy time to mark a detector as jammed
    pub myJamThreshold: f64,
    /// The passing time used in seconds
    pub myPassingTime: f64,
    /// The detector distance in seconds
    pub myDetectorGap: f64,
    /// The time threshold to avoid starved phases
    pub myInactiveThreshold: SUMOTime,
    /// Whether the detectors shall be shown in the GUI
    pub myShowDetectors: bool,
    /// Whether all detectors shall be built
    pub myBuildAllDetectors: bool,
    /// Whether any of the phases has multiple targets
    pub myHasMultiTarget: bool,
    /// The output file for generated detectors
    pub myFile: String,
    /// The frequency for aggregating detector output
    pub myFreq: SUMOTime,
    /// Whether detector output separates by vType
    pub myVehicleTypes: String,
    /// last time trySwitch was called
    pub myLastTrySwitchTime: SUMOTime,
    /// consecutive time that the given link index has been green
    pub myLinkGreenTimes: Vec<SUMOTime>,
    pub myLinkRedTimes: Vec<SUMOTime>,
    /// maximum consecutive time that the given link may remain green
    pub myLinkMaxGreenTimes: Vec<SUMOTime>,
    /// minimum consecutive time that the given link must remain green
    pub myLinkMinGreenTimes: Vec<SUMOTime>,
    /// The custom switching conditions
    pub myConditions: MSActuatedTrafficLightLogic_ConditionMap,
    /// The condition assignments
    pub myAssignments: MSActuatedTrafficLightLogic_AssignmentMap,
    /// The loaded functions
    pub myFunctions: MSActuatedTrafficLightLogic_FunctionMap,
    /// The function call stack;
    // C++ `mutable` (written by const evaluation methods)
    pub myStack: std::cell::RefCell<Vec<BTreeMap<String, f64>>>,
    /// the conditions which shall be listed in GUITLLogicPhasesTrackerWindow
    pub myListedConditions: BTreeSet<String>,
    /// whether the next switch time was requested via TraCI
    pub myTraCISwitch: bool,
    pub mySwitchingRules: Vec<MSActuatedTrafficLightLogic_SwitchingRules>,
    pub myDetectorPrefix: String,
    pub myTargets: BTreeMap<i32, BTreeMap<i32, Vec<i32>>>, // C++: std::map<int, std::map<int, std::vector<int>>>
}

impl std::ops::Deref for MSActuatedTrafficLightLogic {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSActuatedTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

// C++ static members of MSActuatedTrafficLightLogic (port as module-level statics, see AGENTS.md):
//   static const std::vector<std::string> MSActuatedTrafficLightLogic::OPERATOR_PRECEDENCE;

fn actuated_try_switch(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().try_switch()
}
fn actuated_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().init(nb)
}
fn actuated_change_step_and_duration(l: &mut MSTrafficLightLogic, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().change_step_and_duration(tlcontrol, simStep, step, stepDuration)
}
fn actuated_get_detector_states(l: &MSTrafficLightLogic) -> BTreeMap<String, f64> {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_detector_states()
}
fn actuated_get_conditions(l: &MSTrafficLightLogic) -> BTreeMap<String, f64> {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_conditions()
}
fn actuated_load_state(l: &mut MSTrafficLightLogic, tlcontrol: &mut MSTLLogicControl, t: SUMOTime, step: i32, spentDuration: SUMOTime, nextSwitch: SUMOTime, timeInCycle: SUMOTime, active: bool) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().load_state(tlcontrol, t, step, spentDuration, nextSwitch, timeInCycle, active)
}

fn actuated_load_extra_state(l: &mut MSTrafficLightLogic, state: &str) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().load_extra_state(state)
}
fn actuated_ext_get_parameter(l: &mut MSTrafficLightLogic, key: &str, defaultValue: &str) -> String {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().get_parameter(key, defaultValue.to_string())
}
fn actuated_ext_set_parameter(l: &mut MSTrafficLightLogic, key: &str, value: &str) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().set_parameter(key, value)
}
fn actuated_ext_activate_program(l: &mut MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().activate_program()
}
fn actuated_ext_deactivate_program(l: &mut MSTrafficLightLogic) {
    Ptr::from_mut(l).cast::<MSActuatedTrafficLightLogic>().m().deactivate_program()
}
fn actuated_ext_get_detector_state(l: &MSTrafficLightLogic, laneID: String) -> f64 {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_detector_state(laneID)
}
fn actuated_get_min_dur(l: &MSTrafficLightLogic, step: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_min_dur(step)
}
fn actuated_get_max_dur(l: &MSTrafficLightLogic, step: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_max_dur(step)
}
fn actuated_get_earliest_end(l: &MSTrafficLightLogic, step: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_earliest_end(step)
}
fn actuated_get_latest_end(l: &MSTrafficLightLogic, step: i32) -> SUMOTime {
    Ptr::new(l).cast::<MSActuatedTrafficLightLogic>().get().unwrap().get_latest_end(step)
}

static MS_ACTUATED_TRAFFIC_LIGHT_LOGIC_EXT_VT: super::ms_traffic_light_logic::MSTrafficLightLogicExtVt = super::ms_traffic_light_logic::MSTrafficLightLogicExtVt {
    get_rival_vehicles: super::ms_traffic_light_logic::default_ext_get_rival_vehicles,
    get_priority_vehicles: super::ms_traffic_light_logic::default_ext_get_priority_vehicles,
    get_blocking_drive_ways: super::ms_traffic_light_logic::default_ext_get_blocking_drive_ways,
    get_requested_drive_way: super::ms_traffic_light_logic::default_ext_get_requested_drive_way,
    get_detector_state: actuated_ext_get_detector_state,
    get_tl_queue_length: super::ms_traffic_light_logic::default_ext_get_tl_queue_length,
    load_extra_state: actuated_load_extra_state,
    get_parameter: actuated_ext_get_parameter,
    set_parameter: actuated_ext_set_parameter,
    activate_program: actuated_ext_activate_program,
    deactivate_program: actuated_ext_deactivate_program,
};

static MS_ACTUATED_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    try_switch: actuated_try_switch,
    get_phase_number: simple_get_phase_number,
    get_phases: simple_get_phases,
    get_phase: simple_get_phase,
    get_current_phase_index: simple_get_current_phase_index,
    get_current_phase_def: simple_get_current_phase_def,
    reset_last_switch: simple_reset_last_switch,
    get_phase_index_at_time: simple_get_phase_index_at_time,
    get_offset_from_index: simple_get_offset_from_index,
    get_index_from_offset: simple_get_index_from_offset,
    change_step_and_duration: actuated_change_step_and_duration,
    add_link: MSTrafficLightLogic::add_link_base,
    init: actuated_init,
    adapt_link_information_from: MSTrafficLightLogic::adapt_link_information_from_base,
    gets_major_green: MSTrafficLightLogic::gets_major_green_base,
    get_detector_states: actuated_get_detector_states,
    get_conditions: actuated_get_conditions,
    get_blocking_vehicles: MSTrafficLightLogic::get_blocking_vehicles_base,
    save_state: actuated_save_state,
    load_state: actuated_load_state,
    map_time_in_cycle: simple_map_time_in_cycle,
    decide_next_phase: super::ms_traffic_light_logic::default_decide_next_phase,
    can_release: super::ms_traffic_light_logic::default_can_release,
    compute_return_time: super::ms_traffic_light_logic::default_compute_return_time,
    get_min_dur: actuated_get_min_dur,
    get_max_dur: actuated_get_max_dur,
    get_earliest_end: actuated_get_earliest_end,
    get_latest_end: actuated_get_latest_end,
};

impl MSActuatedTrafficLightLogic {
    /// C++: `void MSActuatedTrafficLightLogic::MSActuatedTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const SUMOTime, const MSSimpleTrafficLightLogic::Phases &, int, SUMOTime, const Parameterised::Map &, const std::string &, const ConditionMap &, const AssignmentMap &, const FunctionMap &)`
    /// Constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, offset: SUMOTime, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameter: &ParameterisedMap, basePath: &str, conditions: &MSActuatedTrafficLightLogic_ConditionMap, assignments: &MSActuatedTrafficLightLogic_AssignmentMap, functions: &MSActuatedTrafficLightLogic_FunctionMap) -> Self {
        let mut this = MSActuatedTrafficLightLogic {
            base: MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, offset, TrafficLightType::ACTUATED, phases, step, delay, parameter),
            myInductLoopsForPhase: Vec::new(),
            myCrossingsForPhase: Vec::new(),
            myInductLoops: Vec::new(),
            myExtraLoops: Vec::new(),
            myExtraE2: Vec::new(),
            myMaxGap: 0.0,
            myJamThreshold: 0.0,
            myPassingTime: 0.0,
            myDetectorGap: 0.0,
            myInactiveThreshold: 0,
            myShowDetectors: false,
            myBuildAllDetectors: false,
            myHasMultiTarget: false,
            myFile: String::new(),
            myFreq: 0,
            myVehicleTypes: String::new(),
            myLastTrySwitchTime: 0,
            myLinkGreenTimes: Vec::new(),
            myLinkRedTimes: Vec::new(),
            myLinkMaxGreenTimes: Vec::new(),
            myLinkMinGreenTimes: Vec::new(),
            myConditions: conditions.clone(),
            myAssignments: assignments.clone(),
            myFunctions: functions.clone(),
            myStack: std::cell::RefCell::new(Vec::new()),
            myListedConditions: BTreeSet::new(),
            myTraCISwitch: false,
            mySwitchingRules: Vec::new(),
            myDetectorPrefix: format!("{}_{}_", id, programID),
            myTargets: BTreeMap::new(),
        };
        this.base.base.vt = &MS_ACTUATED_TRAFFIC_LIGHT_LOGIC_VT;
        this.base.base.ext_vt = &MS_ACTUATED_TRAFFIC_LIGHT_LOGIC_EXT_VT;
        let to_d = |s: String| utils::common::string_utils::to_double(&s).unwrap_or_else(|e| panic!("{}", e));
        this.myMaxGap = to_d(this.get_parameter("max-gap", DEFAULT_MAX_GAP.to_string()));
        this.myJamThreshold = to_d(this.get_parameter("jam-threshold", utils::options::options_cont::get_value_string("tls.actuated.jam-threshold")));
        this.myPassingTime = to_d(this.get_parameter("passing-time", DEFAULT_PASSING_TIME.to_string()));
        this.myDetectorGap = to_d(this.get_parameter("detector-gap", DEFAULT_DETECTOR_GAP.to_string()));
        this.myInactiveThreshold = string2time(&this.get_parameter("inactive-threshold", DEFAULT_INACTIVE_THRESHOLD.to_string())).unwrap_or_else(|e| panic!("{}", e));
        let showDetectorsDefault = utils::common::to_string::to_string(&utils::options::options_cont::get_bool("tls.actuated.show-detectors"));
        this.myShowDetectors = utils::common::string_utils::to_bool(&this.get_parameter("show-detectors", showDetectorsDefault)).unwrap_or_else(|e| panic!("{}", e));
        this.myBuildAllDetectors = utils::common::string_utils::to_bool(&this.get_parameter("build-all-detectors", "false".to_string())).unwrap_or_else(|e| panic!("{}", e));
        this.myFile = utils::common::file_helpers::check_for_relativity(&this.get_parameter("file", "NUL".to_string()), basePath);
        this.myFreq = time2steps(to_d(this.get_parameter("freq", "300".to_string())));
        this.myVehicleTypes = this.get_parameter("vTypes", String::new());
        if this.base2.has_parameter("hide-conditions") {
            let hidden: Vec<String> = utils::common::string_tokenizer::StringTokenizer::new(&this.get_parameter("hide-conditions", String::new())).get_vector();
            let hiddenSet: BTreeSet<String> = hidden.into_iter().collect();
            let keys: Vec<String> = this.myConditions.keys().cloned().collect();
            for item in keys {
                if !hiddenSet.contains(&item) {
                    this.myListedConditions.insert(item);
                }
            }
        } else {
            let showAll = this.get_parameter("show-conditions", String::new()) == "";
            let shown: Vec<String> = utils::common::string_tokenizer::StringTokenizer::new(&this.get_parameter("show-conditions", String::new())).get_vector();
            let shownSet: BTreeSet<String> = shown.into_iter().collect();
            let keys: Vec<String> = this.myConditions.keys().cloned().collect();
            for item in keys {
                if showAll || shownSet.contains(&item) {
                    this.myListedConditions.insert(item);
                }
            }
        }
        if this.base2.has_parameter("extra-detectors") {
            let extraIDs = this.get_parameter("extra-detectors", String::new());
            for customID in utils::common::string_tokenizer::StringTokenizer::new(&extraIDs).get_vector() {
                match this.retrieve_det_expression_loop(&customID, true) {
                    Some(loop_) => this.myExtraLoops.push(loop_),
                    None => {
                        let e2 = this.retrieve_det_expression_e2(&customID, true).unwrap_or_else(|| panic!("{}", ProcessError::new(format!("Unknown detector '{}' in expression '{}'", customID, extraIDs))));
                        this.myExtraE2.push(e2);
                    }
                }
            }
        }
        this.myStack.get_mut().push(BTreeMap::new());
        this
    }

    /// C++: `void MSActuatedTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with information about incoming lanes
    // void
    // MSActuatedTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSTrafficLightLogic::init(nb);
    //     initAttributeOverride();
    //     initSwitchingRules();
    //     for (int i = 0; i < (int)myPhases.size(); i++) {
    //         initTargets(i);
    //     }
    //     if (myLanes.size() == 0) {
    //         // must be an older network
    //         WRITE_WARNINGF(TL("Traffic light '%' does not control any links"), getID());
    //     }
    //     bool warn = true; // warn only once
    //     const int numLinks = (int)myLinks.size();
    //
    //     // Detector position should be computed based on road speed. If the position
    //     // is quite far away and the minDur is short this may cause the following
    //     // problems:
    //     //
    //     // 1)  high flow failure:
    //     // In a standing queue, no vehicle touches the detector.
    //     // By the time the queue advances, the detector gap has been exceeded and the phase terminates prematurely
    //     //
    //     // 2) low flow failure
    //     // The standing queue is fully between stop line and detector and there are no further vehicles.
    //     // The minDur is too short to let all vehicles pass
    //     //
    //     // Problem 2) is not so critical because there is less potential for
    //     // jamming in a low-flow situation. In contrast, problem 1) should be
    //     // avoided as it has big jamming potential. We compute an upper bound for the
    //     // detector distance to avoid it
    //
    //
    //     std::set<int> multiNextTargets = getMultiNextTargets();
    //     // change values for setting the loops and lanestate-detectors, here
    //     //SUMOTime inductLoopInterval = 1; //
    //     // build the induct loops
    //     std::map<const MSLane*, MSInductLoop*> laneInductLoopMap;
    //     std::map<MSInductLoop*, int> inductLoopInfoMap; // retrieve junction entry lane in case loops are placed further upstream (and other properties)
    //     int detEdgeIndex = -1;
    //     int detLaneIndex = 0;
    //     const double detDefaultLength = StringUtils::toDouble(getParameter("detector-length",
    //                                     OptionsCont::getOptions().getValueString("tls.actuated.detector-length")));
    //     MSEdge* prevDetEdge = nullptr;
    //     for (LaneVector& lanes : myLanes) {
    //         for (MSLane* lane : lanes) {
    //             const std::string customID = getParameter(lane->getID());
    //             if (noVehicles(lane->getPermissions()) && customID == "") {
    //                 // do not build detectors on green verges or sidewalks
    //                 continue;
    //             }
    //             if (laneInductLoopMap.find(lane) != laneInductLoopMap.end()) {
    //                 // only build one detector per lane
    //                 continue;
    //             }
    //             const SUMOTime minDur = getMinimumMinDuration(lane, multiNextTargets);
    //             if (minDur == std::numeric_limits<SUMOTime>::max() && customID == "" && !myBuildAllDetectors) {
    //                 // only build detector if this lane is relevant for an actuated phase
    //                 continue;
    //             }
    //             double length = lane->getLength();
    //             double ilpos;
    //             double inductLoopPosition;
    //             MSInductLoop* loop = nullptr;
    //             if (&lane->getEdge() != prevDetEdge) {
    //                 detEdgeIndex++;
    //                 detLaneIndex = 0;
    //                 prevDetEdge = &lane->getEdge();
    //             } else {
    //                 detLaneIndex++;
    //             }
    //             const bool isBikeLane = (lane->getPermissions() & ~SVC_PEDESTRIAN) == SVC_BICYCLE;
    //             const double defaultLength = isBikeLane ? DEFAULT_BIKE_LENGTH_WITH_GAP : DEFAULT_LENGTH_WITH_GAP;
    //             if (customID == "") {
    //                 const double speed = isBikeLane ? DEFAULT_BICYCLE_SPEED : lane->getSpeedLimit();
    //                 inductLoopPosition = MIN2(
    //                                          myDetectorGap * speed,
    //                                          (STEPS2TIME(minDur) / myPassingTime + 0.5) * defaultLength);
    //
    //                 // check whether the lane is long enough
    //                 ilpos = length - inductLoopPosition;
    //                 MSLane* placementLane = lane;
    //                 while (ilpos < 0 && placementLane->getIncomingLanes().size() == 1
    //                         && placementLane->getIncomingLanes().front().viaLink->getCorrespondingEntryLink()->getTLLogic() == nullptr) {
    //                     placementLane = placementLane->getLogicalPredecessorLane();
    //                     ilpos += placementLane->getLength();
    //                 }
    //                 if (ilpos < 0) {
    //                     ilpos = 0;
    //                 }
    //                 // Build the induct loop and set it into the container
    //                 const double detLength = getDouble("detector-length:" + lane->getID(), detDefaultLength);
    //                 std::string id = myDetectorPrefix + "D" + toString(detEdgeIndex) + "." + toString(detLaneIndex);
    //                 loop = static_cast<MSInductLoop*>(nb.createInductLoop(id, placementLane, ilpos, detLength, "", myVehicleTypes, "", (int)PersonMode::NONE, myShowDetectors, true));
    //                 MSNet::getInstance()->getDetectorControl().add(SUMO_TAG_INDUCTION_LOOP, loop, myFile, myFreq);
    //             } else if (customID == NO_DETECTOR) {
    //                 continue;
    //             } else {
    //                 loop = dynamic_cast<MSInductLoop*>(MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).get(customID));
    //                 if (loop == nullptr) {
    //                     throw ProcessError(TLF("Unknown inductionLoop '%' given as custom detector for actuated tlLogic '%', program '%.", customID, getID(), getProgramID()));
    //                 }
    //                 ilpos = loop->getPosition();
    //                 inductLoopPosition = length - ilpos;
    //             }
    //             const double maxGap = getDouble("max-gap:" + lane->getID(), myMaxGap);
    //             const double jamThreshold = getDouble("jam-threshold:" + lane->getID(), myJamThreshold);
    //             laneInductLoopMap[lane] = loop;
    //             inductLoopInfoMap[loop] = (int)myInductLoops.size();
    //             myInductLoops.push_back(InductLoopInfo(loop, lane, (int)myPhases.size(), maxGap, jamThreshold));
    //
    //             if (warn && floor(floor(inductLoopPosition / defaultLength) * myPassingTime) > STEPS2TIME(minDur)) {
    //                 // warn if the minGap is insufficient to clear vehicles between stop line and detector
    //                 WRITE_WARNINGF(TL("At actuated tlLogic '%', minDur % is too short for a detector gap of %m."), getID(), time2string(minDur), toString(inductLoopPosition));
    //                 warn = false;
    //             }
    //         }
    //     }
    //     // assign loops to phase index (myInductLoopsForPhase)
    //     //  check1: loops may not be used for a phase if there are other connections from the same lane that may not drive in that phase
    //     //            greenMinor is ambiguous as vehicles may not be able to drive
    //     //            Under the following condition we allow actuation from minor link:
    //     //              check1a : the minor link is minor in all phases
    //     //              check1b : there is another major link from the same lane in the current phase
    //     //              check1e : the conflict is only with bikes/pedestrians (i.e. for a right turn, also left turn with no oncoming traffic)
    //     //              check1f : the conflict is only with a link from the same edge
    //     //            (Under these conditions we assume that the minor link is unimportant and traffic is mostly for the major link)
    //     //
    //     //              check1c: when the edge has only one lane, we treat greenMinor as green as there would be no actuation otherwise
    //     //              check1d: for turnarounds 1b is sufficient and we do not require 1a
    //     //
    //     //  check2: if there are two loops on subsequent lanes (joined tls) and the second one has a red link, the first loop may not be used
    //     //
    //     //  if a jamThreshold is specificed for the loop, all checks are ignored
    //
    //     // also assign loops to link index for validation:
    //     // check if all links from actuated phases (minDur != maxDur) have an inductionloop in at least one phase
    //     const SVCPermissions motorized = ~(SVC_PEDESTRIAN | SVC_BICYCLE);
    //     std::map<int, std::set<MSInductLoop*> > linkToLoops;
    //     std::set<int> actuatedLinks;
    //
    //     std::vector<bool> neverMajor(numLinks, true);
    //     for (const MSPhaseDefinition* phase : myPhases) {
    //         const std::string& state = phase->getState();
    //         for (int i = 0; i < numLinks; i++)  {
    //             if (state[i] == LINKSTATE_TL_GREEN_MAJOR) {
    //                 neverMajor[i] = false;
    //             }
    //         }
    //     }
    //     std::vector<bool> oneLane(numLinks, false);
    //     std::vector<bool> turnaround(numLinks, true);
    //     for (int i = 0; i < numLinks; i++)  {
    //         for (MSLane* lane : getLanesAt(i)) {
    //             // only count motorized vehicle lanes
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
    //         for (MSLink* link : getLinksAt(i)) {
    //             if (!link->isTurnaround()) {
    //                 turnaround[i] = false;
    //                 break;
    //             }
    //         }
    //     }
    //
    //     for (const MSPhaseDefinition* phase : myPhases) {
    //         const int phaseIndex = (int)myInductLoopsForPhase.size();
    //         std::set<MSInductLoop*> loops;
    //         if (phase->isActuated() || multiNextTargets.count(phaseIndex) != 0) {
    //             const std::string& state = phase->getState();
    //             // collect indices of all green links for the phase
    //             std::set<int> greenLinks;
    //             // green links that could jam
    //             std::set<int> greenLinksPermissive;
    //             // collect green links for each induction loops (in this phase)
    //             std::map<MSInductLoop*, std::set<int> > loopLinks;
    //
    //             for (int i = 0; i < numLinks; i++)  {
    //                 if (state[i] == LINKSTATE_TL_GREEN_MAJOR) {
    //                     greenLinks.insert(i);
    //                     if (phase->isActuated()) {
    //                         actuatedLinks.insert(i);
    //                     }
    //
    //                     for (MSLink* link : getLinksAt(i)) {
    //                         if (link->getLane()->isCrossing()) {
    //                             while (myCrossingsForPhase.size() < myPhases.size()) {
    //                                 myCrossingsForPhase.push_back(std::vector<const MSLink*>());
    //                             }
    //                             myCrossingsForPhase[phaseIndex].push_back(link);
    //                         }
    //                     }
    //
    //                 } else if (state[i] == LINKSTATE_TL_GREEN_MINOR) {
    //                     if (((neverMajor[i] || turnaround[i])  // check1a, 1d
    //                             && hasMajor(state, getLanesAt(i))) // check1b
    //                             || oneLane[i] // check1c
    //                             || weakConflict(i, state)) { // check1e, check1f
    //                         greenLinks.insert(i);
    //                         if (!turnaround[i]) {
    //                             if (phase->isActuated()) {
    //                                 actuatedLinks.insert(i);
    //                             }
    //                         }
    //                     } else {
    //                         greenLinksPermissive.insert(i);
    //                     }
    //                 }
    // #ifdef DEBUG_DETECTORS
    //                 if (DEBUG_COND) {
    //                     std::cout << " phase=" << phaseIndex << " i=" << i << " state=" << state[i] << " green=" << greenLinks.count(i) << " oneLane=" << oneLane[i]
    //                               << " turn=" << turnaround[i] << " loopLanes=";
    //                     for (MSLane* lane : getLanesAt(i)) {
    //                         if (laneInductLoopMap.count(lane) != 0) {
    //                             std::cout << lane->getID() << " ";
    //                         }
    //                     }
    //                     std::cout << "\n";
    //                 }
    // #endif
    //                 for (MSLane* lane : getLanesAt(i)) {
    //                     if (laneInductLoopMap.count(lane) != 0) {
    //                         loopLinks[laneInductLoopMap[lane]].insert(i);
    //                     }
    //                 }
    //             }
    //             for (auto& item : loopLinks) {
    //                 MSInductLoop* loop = item.first;
    //                 const InductLoopInfo& info = myInductLoops[inductLoopInfoMap[loop]];
    //                 const MSLane* loopLane = info.lane;
    //                 bool usable = true;
    //                 bool foundUsable = false;
    //                 // check1
    //                 for (int j : item.second) {
    //                     if (greenLinks.count(j) == 0 && (info.jamThreshold <= 0 || greenLinksPermissive.count(j) == 0)) {
    //                         usable = false;
    // #ifdef DEBUG_DETECTORS
    //                         if (DEBUG_COND) {
    //                             std::cout << " phase=" << phaseIndex << " check1: loopLane=" << loopLane->getID() << " notGreen=" << j << " oneLane[j]=" << oneLane[j] << "\n";
    //                         }
    // #endif
    //                     } else {
    //                         foundUsable = true;
    //                     }
    //                 }
    //                 if (!usable && foundUsable && info.jamThreshold > 0) {
    //                     // permit green even when the same lane has green and red links (if we have jamDetection)
    //                     usable = true;
    //                 }
    //                 // check2 (skip if we have jam detection)
    //                 if (usable && info.jamThreshold <= 0) {
    //                     for (MSLink* link : loopLane->getLinkCont()) {
    //                         if (link->isTurnaround()) {
    //                             continue;
    //                         }
    //                         const MSLane* next = link->getLane();
    //                         if (laneInductLoopMap.count(next) != 0) {
    //                             MSInductLoop* nextLoop = laneInductLoopMap[next];
    //                             for (int j : loopLinks[nextLoop]) {
    //                                 if (greenLinks.count(j) == 0) {
    //                                     usable = false;
    // #ifdef DEBUG_DETECTORS
    //                                     if (DEBUG_COND) std::cout << " phase=" << phaseIndex << " check2: loopLane=" << loopLane->getID()
    //                                                                   << " nextLane=" << next->getID() << " nextLink=" << j << " nextState=" << state[j] << "\n";
    // #endif
    //                                     break;
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //
    //                 if (usable) {
    //                     loops.insert(item.first);
    // #ifdef DEBUG_DETECTORS
    //                     if (DEBUG_COND) {
    //                         std::cout << " phase=" << phaseIndex << " usableLoops=" << item.first->getID() << " links=" << joinToString(item.second, " ") << "\n";
    //                     }
    // #endif
    //                     for (int j : item.second) {
    //                         linkToLoops[j].insert(item.first);
    //                     }
    //                 }
    //             }
    //             if (loops.size() == 0 && phase->isActuated()) {
    //                 WRITE_WARNINGF(TL("At actuated tlLogic '%', actuated phase % has no controlling detector."), getID(), toString(phaseIndex));
    //             }
    //         }
    // #ifdef DEBUG_DETECTORS
    //         if (DEBUG_COND) {
    //             std::cout << " phase=" << phaseIndex << " loops=" << joinNamedToString(loops, " ") << "\n";
    //         }
    //         if (DEBUG_COND) {
    //             std::cout << " linkToLoops:\n";
    //             for (auto item : linkToLoops) {
    //                 std::cout << "   link=" << item.first << " loops=" << joinNamedToString(item.second, " ") << "\n";
    //             }
    //         }
    // #endif
    //         std::vector<InductLoopInfo*> loopInfos;
    //         myInductLoopsForPhase.push_back(loopInfos);
    //         for (MSInductLoop* loop : loops) {
    //             for (InductLoopInfo& loopInfo : myInductLoops) {
    //                 if (loopInfo.loop == loop) {
    //                     myInductLoopsForPhase.back().push_back(&loopInfo);
    //                     loopInfo.servedPhase[phaseIndex] = true;
    //                 }
    //             }
    //         }
    //     }
    // #ifdef DEBUG_DETECTORS
    //     if (DEBUG_COND) {
    //         std::cout << "final linkToLoops:\n";
    //         for (auto item : linkToLoops) {
    //             std::cout << "   link=" << item.first << " loops=" << joinNamedToString(item.second, " ") << "\n";
    //         }
    //     }
    // #endif
    //     std::vector<int> warnLinks;
    //     for (int i : actuatedLinks) {
    //         if (linkToLoops[i].size() == 0 && myLinks[i].size() > 0
    //                 && (myLinks[i].front()->getLaneBefore()->getPermissions() & motorized) != 0) {
    //             if (getParameter(myLinks[i].front()->getLaneBefore()->getID()) != NO_DETECTOR) {
    //                 warnLinks.push_back(i);
    //             }
    //         }
    //     }
    //     if (warnLinks.size() > 0) {
    //         WRITE_WARNINGF(TL("At actuated tlLogic '%', linkIndex % has no controlling detector."), getID(), joinToString(warnLinks, ","));
    //     }
    //     // parse maximum green times for each link (optional)
    //     for (const auto& kv : getParametersMap()) {
    //         if (StringUtils::startsWith(kv.first, "linkMaxDur:")) {
    //             int link = StringUtils::toInt(kv.first.substr(11));
    //             if (link < 0 || link >= myNumLinks) {
    //                 WRITE_ERRORF(TL("Invalid link '%' given as linkMaxDur parameter for actuated tlLogic '%', program '%."), kv.first.substr(11), getID(), getProgramID());
    //                 continue;
    //             }
    //             if (myLinkMaxGreenTimes.empty()) {
    //                 myLinkMaxGreenTimes = std::vector<SUMOTime>(myNumLinks, std::numeric_limits<SUMOTime>::max());
    //             }
    //             myLinkMaxGreenTimes[link] = string2time(kv.second);
    //         } else if (StringUtils::startsWith(kv.first, "linkMinDur:")) {
    //             int link = StringUtils::toInt(kv.first.substr(11));
    //             if (link < 0 || link >= myNumLinks) {
    //                 WRITE_ERRORF(TL("Invalid link '%' given as linkMinDur parameter for actuated tlLogic '%', program '%."), kv.first.substr(11), getID(), getProgramID());
    //                 continue;
    //             }
    //             if (myLinkMinGreenTimes.empty()) {
    //                 myLinkMinGreenTimes = std::vector<SUMOTime>(myNumLinks, 0);
    //             }
    //             myLinkMinGreenTimes[link] = string2time(kv.second);
    //         }
    //     }
    //     bool haveSwitchingRules = myConditions.size() > 0 || myAssignments.size() > 0 || myFunctions.size() > 0;
    //     for (auto sr : mySwitchingRules) {
    //         if (sr.enabled) {
    //             haveSwitchingRules = true;
    //             break;
    //         }
    //     }
    //     if (myLinkMaxGreenTimes.size() > 0 || myLinkMinGreenTimes.size() > 0 || haveSwitchingRules) {
    //         myLinkGreenTimes = std::vector<SUMOTime>(myNumLinks, 0);
    //         myLinkRedTimes = std::vector<SUMOTime>(myNumLinks, 0);
    //     }
    //     //std::cout << SIMTIME << " linkMaxGreenTimes=" << toString(myLinkMaxGreenTimes) << "\n";
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        self.base.base.init_base(nb);
        self.init_attribute_override();
        self.init_switching_rules();
        for i in 0..self.myPhases.len() {
            self.init_targets(i as i32);
        }
        if self.myLanes.len() == 0 {
            // must be an older network
            utils::write_warningf!("Traffic light '%' does not control any links", self.get_id());
        }
        let mut warn = true; // warn only once
        let numLinks = self.myLinks.len();

        // detector placement: the detector position is bounded by the road speed and minDur, see C++ comments above
        let multiNextTargets: BTreeSet<i32> = self.get_multi_next_targets();
        let mut laneInductLoopMap: BTreeMap<Ptr<MSLane>, Ptr<MSInductLoop>> = BTreeMap::new();
        let mut inductLoopInfoMap: BTreeMap<Ptr<MSInductLoop>, i32> = BTreeMap::new();
        let mut detEdgeIndex: i32 = -1;
        let mut detLaneIndex: i32 = 0;
        let detDefaultLength: f64 = utils::common::string_utils::to_double(&self.get_parameter("detector-length", utils::options::options_cont::get_value_string("tls.actuated.detector-length"))).unwrap_or_else(|e| panic!("{}", e));
        let mut prevDetEdge: Ptr<MSEdge> = Ptr::null();
        for lanes in self.myLanes.clone() {
            for lane in lanes {
                let customID: String = self.get_parameter(lane.get_id(), String::new());
                if no_vehicles(lane.get_permissions()) && customID == "" {
                    // do not build detectors on green verges or sidewalks
                    continue;
                }
                if laneInductLoopMap.contains_key(&lane) {
                    // only build one detector per lane
                    continue;
                }
                let minDur: SUMOTime = self.get_minimum_min_duration(lane, &multiNextTargets);
                if minDur == i64::MAX && customID == "" && !self.myBuildAllDetectors {
                    // only build detector if this lane is relevant for an actuated phase
                    continue;
                }
                let length: f64 = lane.get_length();
                let mut ilpos: f64;
                let mut inductLoopPosition: f64;
                let mut loop_: Ptr<MSInductLoop> = Ptr::null();
                let laneEdge: Ptr<MSEdge> = Ptr::from_mut(lane.get_edge());
                if laneEdge != prevDetEdge {
                    detEdgeIndex += 1;
                    detLaneIndex = 0;
                    prevDetEdge = laneEdge;
                } else {
                    detLaneIndex += 1;
                }
                let isBikeLane = (lane.get_permissions() & !SVC_PEDESTRIAN) == SVC_BICYCLE;
                let defaultLength: f64 = if isBikeLane { default_bike_length_with_gap() } else { DEFAULT_LENGTH_WITH_GAP };
                if customID == "" {
                    let speed: f64 = if isBikeLane { DEFAULT_BICYCLE_SPEED } else { lane.get_speed_limit() };
                    inductLoopPosition = f64::min(self.myDetectorGap * speed, (steps2time(minDur) / self.myPassingTime + 0.5) * defaultLength);
                    // check whether the lane is long enough
                    ilpos = length - inductLoopPosition;
                    let mut placementLane: Ptr<MSLane> = lane;
                    while ilpos < 0.0 && placementLane.get_incoming_lanes().len() == 1
                        && placementLane.get_incoming_lanes()[0].viaLink.get_corresponding_entry_link().get_tl_logic().is_null() {
                        placementLane = placementLane.get_logical_predecessor_lane();
                        ilpos += placementLane.get_length();
                    }
                    if ilpos < 0.0 {
                        ilpos = 0.0;
                    }
                    // Build the induct loop and set it into the container
                    let detLength: f64 = self.base2.get_double(&format!("detector-length:{}", lane.get_id()), detDefaultLength);
                    let id = format!("{}D{}.{}", self.myDetectorPrefix, detEdgeIndex, detLaneIndex);
                    let det = nb.create_induct_loop(&id, placementLane, ilpos, detLength, String::new(), &self.myVehicleTypes, "", 0 /* PersonMode::NONE */, self.myShowDetectors, true);
                    loop_ = crate::microsim::output::ms_induct_loop::induct_loop_of(det);
                    MSNet::get_instance().m().get_detector_control().add(SUMO_TAG_INDUCTION_LOOP, det, &self.myFile, self.myFreq, 0);
                } else if customID == NO_DETECTOR {
                    continue;
                } else {
                    loop_ = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).get(&customID.to_string()).map(|d| crate::microsim::output::ms_induct_loop::induct_loop_of(*d)).unwrap_or(Ptr::null());
                    if loop_.is_null() {
                        panic!("{}", ProcessError::new(format!("Unknown inductionLoop '{}' given as custom detector for actuated tlLogic '{}', program '{}.", customID, self.get_id(), self.get_program_id())));
                    }
                    ilpos = loop_.get_position();
                    inductLoopPosition = length - ilpos;
                }
                let maxGap: f64 = self.base2.get_double(&format!("max-gap:{}", lane.get_id()), self.myMaxGap);
                let jamThreshold: f64 = self.base2.get_double(&format!("jam-threshold:{}", lane.get_id()), self.myJamThreshold);
                laneInductLoopMap.insert(lane, loop_);
                inductLoopInfoMap.insert(loop_, self.myInductLoops.len() as i32);
                let numPhases = self.myPhases.len() as i32;
                self.myInductLoops.push(MSActuatedTrafficLightLogic_InductLoopInfo::new(loop_, lane, numPhases, maxGap, jamThreshold));
                if warn && f64::floor(f64::floor(inductLoopPosition / defaultLength) * self.myPassingTime) > steps2time(minDur) {
                    // warn if the minGap is insufficient to clear vehicles between stop line and detector
                    utils::write_warningf!("At actuated tlLogic '%', minDur % is too short for a detector gap of %m.", self.get_id(), time2string(minDur), utils::common::to_string::to_string(&inductLoopPosition));
                    warn = false;
                }
            }
        }

        // assign loops to phase index (myInductLoopsForPhase), see the C++ checks check1 / check2 above
        // also assign loops to link index for validation
        let motorized: SVCPermissions = !(SVC_PEDESTRIAN | SVC_BICYCLE);
        let mut linkToLoops: BTreeMap<i32, BTreeSet<Ptr<MSInductLoop>>> = BTreeMap::new();
        let mut actuatedLinks: BTreeSet<i32> = BTreeSet::new();

        let mut neverMajor: Vec<bool> = vec![true; numLinks];
        for phase in self.myPhases.clone() {
            let state = phase.get_state().to_string();
            for i in 0..numLinks {
                if ((state.as_bytes()[i] as i8) as LinkState) == LINKSTATE_TL_GREEN_MAJOR {
                    neverMajor[i] = false;
                }
            }
        }
        let mut oneLane: Vec<bool> = vec![false; numLinks];
        let mut turnaround: Vec<bool> = vec![true; numLinks];
        for i in 0..numLinks {
            for lane in self.get_lanes_at(i as i32).clone() {
                // only count motorized vehicle lanes
                let mut numMotorized = 0;
                for l in lane.get_edge().get_lanes().clone() {
                    if (l.get_permissions() & motorized) != 0 {
                        numMotorized += 1;
                    }
                }
                if numMotorized == 1 {
                    oneLane[i] = true;
                    break;
                }
            }
            for link in self.get_links_at(i as i32).clone() {
                if !link.is_turnaround() {
                    turnaround[i] = false;
                    break;
                }
            }
        }

        let phases: MSTrafficLightLogic_Phases = self.myPhases.clone();
        for phase in phases.iter() {
            let phaseIndex: i32 = self.myInductLoopsForPhase.len() as i32;
            let mut loops: BTreeSet<Ptr<MSInductLoop>> = BTreeSet::new();
            if phase.is_actuated() || multiNextTargets.contains(&phaseIndex) {
                let state = phase.get_state().to_string();
                // collect indices of all green links for the phase
                let mut greenLinks: BTreeSet<i32> = BTreeSet::new();
                // green links that could jam
                let mut greenLinksPermissive: BTreeSet<i32> = BTreeSet::new();
                // collect green links for each induction loops (in this phase)
                // determinism: C++ std::map is keyed by MSInductLoop* (address order); BTreeMap over Ptr keeps the same key order
                let mut loopLinks: BTreeMap<Ptr<MSInductLoop>, BTreeSet<i32>> = BTreeMap::new();

                for i in 0..numLinks {
                    let s: LinkState = (state.as_bytes()[i] as i8) as LinkState;
                    if s == LINKSTATE_TL_GREEN_MAJOR {
                        greenLinks.insert(i as i32);
                        if phase.is_actuated() {
                            actuatedLinks.insert(i as i32);
                        }
                        for link in self.get_links_at(i as i32).clone() {
                            if link.get_lane().is_crossing() {
                                while self.myCrossingsForPhase.len() < self.myPhases.len() {
                                    self.myCrossingsForPhase.push(Vec::new());
                                }
                                self.myCrossingsForPhase[phaseIndex as usize].push(link);
                            }
                        }
                    } else if s == LINKSTATE_TL_GREEN_MINOR {
                        if ((neverMajor[i] || turnaround[i]) && self.has_major(&state, self.get_lanes_at(i as i32))) // check1a, 1d, check1b
                            || oneLane[i] // check1c
                            || self.weak_conflict(i as i32, &state) { // check1e, check1f
                            greenLinks.insert(i as i32);
                            if !turnaround[i] && phase.is_actuated() {
                                actuatedLinks.insert(i as i32);
                            }
                        } else {
                            greenLinksPermissive.insert(i as i32);
                        }
                    }
                    for lane in self.get_lanes_at(i as i32).clone() {
                        if laneInductLoopMap.contains_key(&lane) {
                            let loopPtr = laneInductLoopMap[&lane];
                            loopLinks.entry(loopPtr).or_default().insert(i as i32);
                        }
                    }
                }
                // iterate like std::map: entries inserted during the walk (loopLinks[nextLoop] below) are visited if they follow the cursor
                let mut cursor: Option<Ptr<MSInductLoop>> = None;
                loop {
                    let item: Option<(Ptr<MSInductLoop>, BTreeSet<i32>)> = match cursor {
                        None => loopLinks.iter().next().map(|(k, v)| (*k, v.clone())),
                        Some(c) => loopLinks
                            .range((std::ops::Bound::Excluded(c), std::ops::Bound::Unbounded))
                            .next()
                            .map(|(k, v)| (*k, v.clone())),
                    };
                    let Some((loopKey, links)) = item else {
                        break;
                    };
                    cursor = Some(loopKey);
                    let info_idx = inductLoopInfoMap[&loopKey] as usize;
                    let info_jam: f64 = self.myInductLoops[info_idx].jamThreshold;
                    let loopLane: Ptr<MSLane> = self.myInductLoops[info_idx].lane;
                    let mut usable = true;
                    let mut foundUsable = false;
                    // check1
                    for j in links.iter().copied() {
                        if !greenLinks.contains(&j) && (info_jam <= 0.0 || !greenLinksPermissive.contains(&j)) {
                            usable = false;
                        } else {
                            foundUsable = true;
                        }
                    }
                    if !usable && foundUsable && info_jam > 0.0 {
                        // permit green even when the same lane has green and red links (if we have jamDetection)
                        usable = true;
                    }
                    // check2 (skip if we have jam detection)
                    if usable && info_jam <= 0.0 {
                        for link in loopLane.get_link_cont().clone() {
                            if link.is_turnaround() {
                                continue;
                            }
                            let next: Ptr<MSLane> = link.get_lane();
                            if laneInductLoopMap.contains_key(&next) {
                                let nextLoop = laneInductLoopMap[&next];
                                // loopLinks[nextLoop] (operator[] inserts an empty set when missing, as in C++)
                                let nextLinks: BTreeSet<i32> = loopLinks.entry(nextLoop).or_default().clone();
                                for j in nextLinks {
                                    if !greenLinks.contains(&j) {
                                        usable = false;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if usable {
                        loops.insert(loopKey);
                        for j in links.iter().copied() {
                            linkToLoops.entry(j).or_default().insert(loopKey);
                        }
                    }
                }
                if loops.len() == 0 && phase.is_actuated() {
                    utils::write_warningf!("At actuated tlLogic '%', actuated phase % has no controlling detector.", self.get_id(), phaseIndex);
                }
            }
            let loopInfos: Vec<Ptr<MSActuatedTrafficLightLogic_InductLoopInfo>> = Vec::new();
            self.myInductLoopsForPhase.push(loopInfos);
            for loopKey in loops.iter() {
                for idx in 0..self.myInductLoops.len() {
                    if self.myInductLoops[idx].loop_ == *loopKey {
                        let info = Ptr::from_mut(&mut self.myInductLoops[idx]);
                        self.myInductLoopsForPhase.last_mut().unwrap().push(info);
                        self.myInductLoops[idx].servedPhase[phaseIndex as usize] = true;
                    }
                }
            }
        }

        let mut warnLinks: Vec<i32> = Vec::new();
        for i in actuatedLinks.iter().copied() {
            if linkToLoops.entry(i).or_default().is_empty() && self.myLinks[i as usize].len() > 0
                && (self.myLinks[i as usize][0].get_lane_before().get_permissions() & motorized) != 0 {
                let laneID: String = self.myLinks[i as usize][0].get_lane_before().get_id().to_string();
                if self.get_parameter(&laneID, String::new()) != NO_DETECTOR {
                    warnLinks.push(i);
                }
            }
        }
        if warnLinks.len() > 0 {
            utils::write_warningf!("At actuated tlLogic '%', linkIndex % has no controlling detector.", self.get_id(), utils::common::to_string::join_to_string(&warnLinks, ","));
        }

        // parse maximum green times for each link (optional)
        let parameterEntries: Vec<(String, String)> = self.base2.get_parameters_map().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        for (key, value) in parameterEntries {
            if utils::common::string_utils::starts_with(&key, "linkMaxDur:") {
                let link: i32 = utils::common::string_utils::to_int(&key[11..]).unwrap_or_else(|e| panic!("{}", e));
                if link < 0 || link >= self.myNumLinks {
                    utils::write_errorf!("Invalid link '%' given as linkMaxDur parameter for actuated tlLogic '%', program '%.", &key[11..], self.get_id(), self.get_program_id());
                    continue;
                }
                if self.myLinkMaxGreenTimes.is_empty() {
                    self.myLinkMaxGreenTimes = vec![i64::MAX; self.myNumLinks as usize];
                }
                self.myLinkMaxGreenTimes[link as usize] = string2time(&value).unwrap_or_else(|e| panic!("{}", e));
            } else if utils::common::string_utils::starts_with(&key, "linkMinDur:") {
                let link: i32 = utils::common::string_utils::to_int(&key[11..]).unwrap_or_else(|e| panic!("{}", e));
                if link < 0 || link >= self.myNumLinks {
                    utils::write_errorf!("Invalid link '%' given as linkMinDur parameter for actuated tlLogic '%', program '%.", &key[11..], self.get_id(), self.get_program_id());
                    continue;
                }
                if self.myLinkMinGreenTimes.is_empty() {
                    self.myLinkMinGreenTimes = vec![0; self.myNumLinks as usize];
                }
                self.myLinkMinGreenTimes[link as usize] = string2time(&value).unwrap_or_else(|e| panic!("{}", e));
            }
        }
        let mut haveSwitchingRules = self.myConditions.len() > 0 || self.myAssignments.len() > 0 || self.myFunctions.len() > 0;
        for sr in self.mySwitchingRules.iter() {
            if sr.enabled {
                haveSwitchingRules = true;
                break;
            }
        }
        if self.myLinkMaxGreenTimes.len() > 0 || self.myLinkMinGreenTimes.len() > 0 || haveSwitchingRules {
            self.myLinkGreenTimes = vec![0; numLinks];
            self.myLinkRedTimes = vec![0; numLinks];
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::~MSActuatedTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSActuatedTrafficLightLogic::~MSActuatedTrafficLightLogic() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime
    // MSActuatedTrafficLightLogic::trySwitch() {
    //     // checks if the actual phase should be continued
    //     // @note any vehicles which arrived during the previous phases which are now waiting between the detector and the stop line are not
    //     // considere here. RiLSA recommends to set minDuration in a way that lets all vehicles pass the detector
    //     SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     executeAssignments(myAssignments, myConditions);
    //
    //     if (myLinkGreenTimes.size() > 0) {
    //         // constraints exist, record green time durations for each link
    //         const std::string& state = getCurrentPhaseDef().getState();
    //         SUMOTime lastDuration = SIMSTEP - myLastTrySwitchTime;
    //         for (int i = 0; i < myNumLinks; i++) {
    //             if (state[i] == 'G' || state[i] == 'g') {
    //                 myLinkGreenTimes[i] += lastDuration;
    //             } else {
    //                 myLinkGreenTimes[i] = 0;
    //             }
    //             if (state[i] == 'r' || state[i] == 'u') {
    //                 myLinkRedTimes[i] += lastDuration;
    //             } else {
    //                 myLinkRedTimes[i] = 0;
    //             }
    //         }
    //     }
    //     myLastTrySwitchTime = now;
    //     // decide the next phase
    //     const bool multiTarget = myPhases[myStep]->nextPhases.size() > 1 && myPhases[myStep]->nextPhases.front() >= 0;
    //     const int origStep = myStep;
    //     int nextStep = myStep;
    //     SUMOTime actDuration = now - myPhases[myStep]->myLastSwitch;
    //
    //     if (mySwitchingRules[myStep].enabled) {
    //         const bool mustSwitch = MIN2(getMaxDur() - actDuration, getLatest()) <= 0;
    //         nextStep = decideNextPhaseCustom(mustSwitch);
    //     } else {
    //         // default algorithm
    //         const double detectionGap = gapControl();
    // #ifdef DEBUG_PHASE_SELECTION
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " p=" << myStep
    //                       << " trySwitch dGap=" << (detectionGap == std::numeric_limits<double>::max() ? "inf" : toString(detectionGap))
    //                       << " multi=" << multiTarget << "\n";
    //         }
    // #endif
    //         if (detectionGap < std::numeric_limits<double>::max() && !multiTarget && !myTraCISwitch) {
    //             return duration(detectionGap);
    //         }
    //         if (multiTarget) {
    //             nextStep = decideNextPhase();
    //         } else {
    //             if (myPhases[myStep]->nextPhases.size() == 1 && myPhases[myStep]->nextPhases.front() >= 0) {
    //                 nextStep = myPhases[myStep]->nextPhases.front();
    //             } else {
    //                 nextStep = (myStep + 1) % (int)myPhases.size();
    //             }
    //         }
    //     }
    //
    //     myTraCISwitch = false;
    //     if (myLinkMinGreenTimes.size() > 0) {
    //         SUMOTime linkMinDur = getLinkMinDuration(getTarget(nextStep).first);
    //         if (linkMinDur > 0) {
    //             // for multiTarget, the current phase must be extended but if another
    //             // targer is chosen, earlier switching than linkMinDur is possible
    //             return multiTarget ? TIME2STEPS(1) : linkMinDur;
    //         }
    //     }
    //     myStep = nextStep;
    //     assert(myStep <= (int)myPhases.size());
    //     assert(myStep >= 0);
    //     //stores the time the phase started
    //     const SUMOTime prevStart = myPhases[myStep]->myLastSwitch;
    //     if (myStep != origStep) {
    //         myPhases[origStep]->myLastEnd = now;
    //         myPhases[myStep]->myLastSwitch = now;
    //         actDuration = 0;
    //     }
    //     // activate coloring
    //     if ((myShowDetectors || myHasMultiTarget) && getCurrentPhaseDef().isGreenPhase()) {
    //         for (InductLoopInfo* loopInfo : myInductLoopsForPhase[myStep]) {
    //             //std::cout << SIMTIME << " p=" << myStep << " loopinfo=" << loopInfo->loop->getID() << " set lastGreen=" << STEPS2TIME(now) << "\n";
    //             if (loopInfo->isJammed()) {
    //                 loopInfo->loop->setSpecialColor(&RGBColor::ORANGE);
    //             } else {
    //                 loopInfo->loop->setSpecialColor(&RGBColor::GREEN);
    //             }
    //             loopInfo->lastGreenTime = now;
    //         }
    //     }
    //     // set the next event
    // #ifdef DEBUG_PHASE_SELECTION
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " tl=" << getID() << " p=" << myStep
    //                   << " nextTryMinDur=" << STEPS2TIME(getMinDur() - actDuration)
    //                   << " nextTryEarliest=" << STEPS2TIME(getEarliest(prevStart)) << "\n";
    //     }
    // #endif
    //     SUMOTime minRetry = myStep != origStep ? 0 : TIME2STEPS(1);
    //     return MAX3(minRetry, getMinDur() - actDuration, getEarliest(prevStart));
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        // checks if the actual phase should be continued
        // @note any vehicles which arrived during the previous phases which are now waiting between the detector and the stop line are not
        // considere here. RiLSA recommends to set minDuration in a way that lets all vehicles pass the detector
        let now = MSNet::get_instance().get_current_time_step();
        let assignments = self.myAssignments.clone();
        let mut conditions = self.myConditions.clone();
        self.execute_assignments(&assignments, &mut conditions, &BTreeMap::new());
        self.myConditions = conditions;
        if self.myLinkGreenTimes.len() > 0 {
            // constraints exist, record green time durations for each link
            let state = self.get_current_phase_def().get_state().to_string();
            let lastDuration: SUMOTime = now - self.myLastTrySwitchTime;
            for i in 0..self.myNumLinks as usize {
                let c = state.as_bytes()[i] as char;
                if c == 'G' || c == 'g' {
                    self.myLinkGreenTimes[i] += lastDuration;
                } else {
                    self.myLinkGreenTimes[i] = 0;
                }
                if c == 'r' || c == 'u' {
                    self.myLinkRedTimes[i] += lastDuration;
                } else {
                    self.myLinkRedTimes[i] = 0;
                }
            }
        }
        self.myLastTrySwitchTime = now;
        // decide the next phase
        let multiTarget = self.myPhases[self.myStep as usize].nextPhases.len() > 1 && self.myPhases[self.myStep as usize].nextPhases[0] >= 0;
        let origStep = self.myStep;
        let mut nextStep = self.myStep;
        let mut actDuration: SUMOTime = now - self.myPhases[self.myStep as usize].myLastSwitch;
        if self.mySwitchingRules[self.myStep as usize].enabled {
            let mustSwitch = std::cmp::min(self.get_max_dur(-1) - actDuration, self.get_latest()) <= 0;
            nextStep = self.decide_next_phase_custom(mustSwitch);
        } else {
            // default algorithm
            let detectionGap = self.gap_control();
            if detectionGap < f64::MAX && !multiTarget && !self.myTraCISwitch {
                return self.duration(detectionGap);
            }
            if multiTarget {
                nextStep = self.decide_next_phase();
            } else {
                if self.myPhases[self.myStep as usize].nextPhases.len() == 1 && self.myPhases[self.myStep as usize].nextPhases[0] >= 0 {
                    nextStep = self.myPhases[self.myStep as usize].nextPhases[0];
                } else {
                    nextStep = (self.myStep + 1) % (self.myPhases.len() as i32);
                }
            }
        }
        self.myTraCISwitch = false;
        if self.myLinkMinGreenTimes.len() > 0 {
            let linkMinDur = self.get_link_min_duration(self.get_target(nextStep).0);
            if linkMinDur > 0 {
                // for multiTarget, the current phase must be extended but if another
                // targer is chosen, earlier switching than linkMinDur is possible
                return if multiTarget { time2steps(1.0) } else { linkMinDur };
            }
        }
        self.myStep = nextStep;
        assert!(self.myStep <= self.myPhases.len() as i32);
        assert!(self.myStep >= 0);
        // stores the time the phase started
        let prevStart: SUMOTime = self.myPhases[self.myStep as usize].myLastSwitch;
        if self.myStep != origStep {
            self.myPhases[origStep as usize].m().myLastEnd = now;
            self.myPhases[self.myStep as usize].m().myLastSwitch = now;
            actDuration = 0;
        }
        // activate coloring
        if (self.myShowDetectors || self.myHasMultiTarget) && self.get_current_phase_def().is_green_phase() {
            let infos: Vec<Ptr<MSActuatedTrafficLightLogic_InductLoopInfo>> = self.myInductLoopsForPhase[self.myStep as usize].clone();
            for loopInfo in infos {
                if loopInfo.is_jammed() {
                    loopInfo.loop_.m().set_special_color(Ptr::new(&RGB_ORANGE));
                } else {
                    loopInfo.loop_.m().set_special_color(Ptr::new(&RGB_GREEN));
                }
                loopInfo.m().lastGreenTime = now;
            }
        }
        // set the next event
        let minRetry: SUMOTime = if self.myStep != origStep { 0 } else { time2steps(1.0) };
        std::cmp::max(minRetry, std::cmp::max(self.get_min_dur(-1) - actDuration, self.get_earliest(prevStart)))
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getMinDur(int) const` [const, virtual]
    /// @}
    // SUMOTime
    // MSActuatedTrafficLightLogic::getMinDur(int step) const {
    //     step = step < 0 ? myStep : step;
    //     const MSPhaseDefinition* p = myPhases[step];
    //     return p->minDuration != MSPhaseDefinition::OVERRIDE_DURATION
    //            ? p->minDuration
    //            : TIME2STEPS(evalExpression(myConditions.find("minDur:" + toString(step))->second));
    // }
    #[allow(unused_variables)]
    pub fn get_min_dur(&self, step: i32) -> SUMOTime {
        let step = if step < 0 { self.myStep } else { step };
        let p = self.myPhases[step as usize];
        if p.minDuration != OVERRIDE_DURATION {
            p.minDuration
        } else {
            time2steps(self.eval_expression(self.myConditions.get(&format!("minDur:{}", step)).unwrap_or_else(|| panic!("missing condition minDur:{}", step))))
        }
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getMaxDur(int) const` [const, virtual]
    // SUMOTime
    // MSActuatedTrafficLightLogic::getMaxDur(int step) const {
    //     step = step < 0 ? myStep : step;
    //     const MSPhaseDefinition* p = myPhases[step];
    //     return p->maxDuration != MSPhaseDefinition::OVERRIDE_DURATION
    //            ? p->maxDuration
    //            : TIME2STEPS(evalExpression(myConditions.find("maxDur:" + toString(step))->second));
    // }
    #[allow(unused_variables)]
    pub fn get_max_dur(&self, step: i32) -> SUMOTime {
        let step = if step < 0 { self.myStep } else { step };
        let p = self.myPhases[step as usize];
        if p.maxDuration != OVERRIDE_DURATION {
            p.maxDuration
        } else {
            time2steps(self.eval_expression(self.myConditions.get(&format!("maxDur:{}", step)).unwrap_or_else(|| panic!("missing condition maxDur:{}", step))))
        }
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getEarliestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSActuatedTrafficLightLogic::getEarliestEnd(int step) const {
    //     step = step < 0 ? myStep : step;
    //     const MSPhaseDefinition* p = myPhases[step];
    //     return p->earliestEnd != MSPhaseDefinition::OVERRIDE_DURATION
    //            ? p->earliestEnd
    //            : TIME2STEPS(evalExpression(myConditions.find("earliestEnd:" + toString(step))->second));
    // }
    #[allow(unused_variables)]
    pub fn get_earliest_end(&self, step: i32) -> SUMOTime {
        let step = if step < 0 { self.myStep } else { step };
        let p = self.myPhases[step as usize];
        if p.earliestEnd != OVERRIDE_DURATION {
            p.earliestEnd
        } else {
            time2steps(self.eval_expression(self.myConditions.get(&format!("earliestEnd:{}", step)).unwrap_or_else(|| panic!("missing condition earliestEnd:{}", step))))
        }
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getLatestEnd(int) const` [const, virtual]
    // SUMOTime
    // MSActuatedTrafficLightLogic::getLatestEnd(int step) const {
    //     step = step < 0 ? myStep : step;
    //     const MSPhaseDefinition* p = myPhases[step];
    //     return p->latestEnd != MSPhaseDefinition::OVERRIDE_DURATION
    //            ? p->latestEnd
    //            : TIME2STEPS(evalExpression(myConditions.find("latestEnd:" + toString(step))->second));
    // }
    #[allow(unused_variables)]
    pub fn get_latest_end(&self, step: i32) -> SUMOTime {
        let step = if step < 0 { self.myStep } else { step };
        let p = self.myPhases[step as usize];
        if p.latestEnd != OVERRIDE_DURATION {
            p.latestEnd
        } else {
            time2steps(self.eval_expression(self.myConditions.get(&format!("latestEnd:{}", step)).unwrap_or_else(|| panic!("missing condition latestEnd:{}", step))))
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void
    // MSActuatedTrafficLightLogic::changeStepAndDuration(MSTLLogicControl& tlcontrol,
    //         SUMOTime simStep, int step, SUMOTime stepDuration) {
    //     // do not change timing if the phase changes
    //     if (step >= 0 && step != myStep) {
    //         myStep = step;
    //         myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //         setTrafficLightSignals(simStep);
    //         tlcontrol.get(getID()).executeOnSwitchActions();
    //     } else if (step < 0) {
    //         // TraCI requested new timing
    //         mySwitchCommand->deschedule(this);
    //         mySwitchCommand = new SwitchCommand(tlcontrol, this, stepDuration + simStep);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //             mySwitchCommand, stepDuration + simStep);
    //         myTraCISwitch = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        // do not change timing if the phase changes
        if step >= 0 && step != self.myStep {
            self.myStep = step;
            let now = MSNet::get_instance().get_current_time_step();
            self.myPhases[self.myStep as usize].m().myLastSwitch = now;
            self.apply_phase_signals(simStep);
            let id = self.get_id().to_string();
            tlcontrol.get(&id).execute_on_switch_actions();
        } else if step < 0 {
            // TraCI requested new timing
            let this = Ptr::from_mut(&mut self.base.base);
            self.base.base.mySwitchCommand.deschedule(this);
            self.base.base.mySwitchCommand = Ptr::from_raw(Box::into_raw(Box::new(MSTrafficLightLogic_SwitchCommand::new(tlcontrol, this, stepDuration + simStep))));
            let events = MSNet::get_instance().m().get_begin_of_timestep_events();
            events.m().add_event(Ptr::from_raw(self.base.base.mySwitchCommand.as_raw() as *mut dyn crate::microsim::ms_event_control::Command), stepDuration + simStep);
            self.myTraCISwitch = true;
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::activateProgram()` [virtual]
    /// called when switching programs
    // void
    // MSActuatedTrafficLightLogic::activateProgram() {
    //     MSTrafficLightLogic::activateProgram();
    //     for (InductLoopInfo& loopInfo : myInductLoops) {
    //         loopInfo.loop->setVisible(myShowDetectors);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_program(&mut self) {
        self.base.base.activate_program_base();
        for loopInfo in self.myInductLoops.iter() {
            loopInfo.loop_.m().set_visible(self.myShowDetectors);
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::deactivateProgram()` [virtual]
    // void
    // MSActuatedTrafficLightLogic::deactivateProgram() {
    //     MSTrafficLightLogic::deactivateProgram();
    //     for (InductLoopInfo& loopInfo : myInductLoops) {
    //         loopInfo.loop->setVisible(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn deactivate_program(&mut self) {
        self.base.base.deactivate_program_base();
        for loopInfo in self.myInductLoops.iter() {
            loopInfo.loop_.m().set_visible(false);
        }
    }

    /// C++: `bool MSActuatedTrafficLightLogic::showDetectors() const` [const]
    // bool showDetectors() const {
    //         return myShowDetectors;
    //     }
    #[allow(unused_variables)]
    pub fn show_detectors(&self) -> bool {
        self.myShowDetectors
    }

    /// C++: `void MSActuatedTrafficLightLogic::setShowDetectors(bool)`
    // void
    // MSActuatedTrafficLightLogic::setShowDetectors(bool show) {
    //     myShowDetectors = show;
    //     for (InductLoopInfo& loopInfo : myInductLoops) {
    //         loopInfo.loop->setVisible(myShowDetectors);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_show_detectors(&mut self, show: bool) {
        self.myShowDetectors = show;
        for loopInfo in self.myInductLoops.iter() {
            loopInfo.loop_.m().set_visible(self.myShowDetectors);
        }
    }

    /// C++: `const std::string MSActuatedTrafficLightLogic::getParameter(const std::string &, const std::string) const` [const, virtual]
    /// try to get the value of the given parameter (including prefixed parameters)
    // const std::string
    // MSActuatedTrafficLightLogic::getParameter(const std::string& key, const std::string defaultValue) const {
    //     if (StringUtils::startsWith(key, "condition.")) {
    //         const std::string cond = key.substr(10);
    //         auto it = myConditions.find(cond);
    //         if (it != myConditions.end()) {
    //             return toString(evalExpression(it->second));
    //         } else {
    //             throw InvalidArgument(TLF("Unknown condition '%' for actuated traffic light '%'", cond, getID()));
    //         }
    //     } else {
    //         return MSSimpleTrafficLightLogic::getParameter(key, defaultValue);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str, defaultValue: String) -> String {
        if utils::common::string_utils::starts_with(key, "condition.") {
            let cond = &key[10..];
            match self.myConditions.get(cond) {
                Some(c) => utils::common::to_string::to_string(&self.eval_expression(c)),
                None => panic!("{}", ProcessError::InvalidArgument(format!("Unknown condition '{}' for actuated traffic light '{}'", cond, self.get_id()))),
            }
        } else {
            self.base.get_parameter(key, defaultValue)
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::setParameter(const std::string &, const std::string &)` [virtual]
    /// Sets a parameter and updates internal constants
    // void
    // MSActuatedTrafficLightLogic::setParameter(const std::string& key, const std::string& value) {
    //     // some pre-defined parameters can be updated at runtime
    //     if (key == "detector-gap" || key == "passing-time" || key == "file" || key == "freq" || key == "vTypes"
    //             || key == "build-all-detectors"
    //             || StringUtils::startsWith(key, "linkMaxDur")
    //             || StringUtils::startsWith(key, "linkMinDur")) {
    //         throw InvalidArgument(key + " cannot be changed dynamically for actuated traffic light '" + getID() + "'");
    //     } else if (key == "max-gap") {
    //         myMaxGap = StringUtils::toDouble(value);
    //         // overwrite custom values
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             loopInfo.maxGap = myMaxGap;
    //         }
    //         Parameterised::setParameter(key, value);
    //     } else if (StringUtils::startsWith(key, "max-gap:")) {
    //         const std::string laneID = key.substr(8);
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             if (loopInfo.lane->getID() == laneID) {
    //                 loopInfo.maxGap = StringUtils::toDouble(value);
    //                 Parameterised::setParameter(key, value);
    //                 return;
    //             }
    //         }
    //         throw InvalidArgument(TLF("Invalid lane '%' in key '%' for actuated traffic light '%'", laneID, key, getID()));
    //     } else if (key == "jam-threshold") {
    //         myJamThreshold = StringUtils::toDouble(value);
    //         // overwrite custom values
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             loopInfo.jamThreshold = myJamThreshold;
    //         }
    //         Parameterised::setParameter(key, value);
    //     } else if (StringUtils::startsWith(key, "jam-threshold:")) {
    //         const std::string laneID = key.substr(14);
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             if (loopInfo.lane->getID() == laneID) {
    //                 loopInfo.jamThreshold = StringUtils::toDouble(value);
    //                 Parameterised::setParameter(key, value);
    //                 return;
    //             }
    //         }
    //         throw InvalidArgument(TLF("Invalid lane '%' in key '%' for actuated traffic light '%'", laneID, key, getID()));
    //     } else if (key == "show-detectors") {
    //         myShowDetectors = StringUtils::toBool(value);
    //         Parameterised::setParameter(key, value);
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             loopInfo.loop->setVisible(myShowDetectors);
    //         }
    //     } else if (key == "inactive-threshold") {
    //         myInactiveThreshold = string2time(value);
    //         Parameterised::setParameter(key, value);
    //     } else {
    //         MSSimpleTrafficLightLogic::setParameter(key, value);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        // some pre-defined parameters can be updated at runtime
        if key == "detector-gap" || key == "passing-time" || key == "file" || key == "freq" || key == "vTypes"
            || key == "build-all-detectors"
            || utils::common::string_utils::starts_with(key, "linkMaxDur")
            || utils::common::string_utils::starts_with(key, "linkMinDur") {
            panic!("{}", ProcessError::InvalidArgument(format!("{} cannot be changed dynamically for actuated traffic light '{}'", key, self.get_id())));
        } else if key == "max-gap" {
            self.myMaxGap = utils::common::string_utils::to_double(value).unwrap_or_else(|e| panic!("{}", e));
            // overwrite custom values
            for loopInfo in self.myInductLoops.iter_mut() {
                loopInfo.maxGap = self.myMaxGap;
            }
            self.base2.set_parameter(key, value);
        } else if utils::common::string_utils::starts_with(key, "max-gap:") {
            let laneID = &key[8..];
            for idx in 0..self.myInductLoops.len() {
                if self.myInductLoops[idx].lane.get_id() == laneID {
                    self.myInductLoops[idx].maxGap = utils::common::string_utils::to_double(value).unwrap_or_else(|e| panic!("{}", e));
                    self.base2.set_parameter(key, value);
                    return;
                }
            }
            panic!("{}", ProcessError::InvalidArgument(format!("Invalid lane '{}' in key '{}' for actuated traffic light '{}'", laneID, key, self.get_id())));
        } else if key == "jam-threshold" {
            self.myJamThreshold = utils::common::string_utils::to_double(value).unwrap_or_else(|e| panic!("{}", e));
            // overwrite custom values
            for loopInfo in self.myInductLoops.iter_mut() {
                loopInfo.jamThreshold = self.myJamThreshold;
            }
            self.base2.set_parameter(key, value);
        } else if utils::common::string_utils::starts_with(key, "jam-threshold:") {
            let laneID = &key[14..];
            for idx in 0..self.myInductLoops.len() {
                if self.myInductLoops[idx].lane.get_id() == laneID {
                    self.myInductLoops[idx].jamThreshold = utils::common::string_utils::to_double(value).unwrap_or_else(|e| panic!("{}", e));
                    self.base2.set_parameter(key, value);
                    return;
                }
            }
            panic!("{}", ProcessError::InvalidArgument(format!("Invalid lane '{}' in key '{}' for actuated traffic light '{}'", laneID, key, self.get_id())));
        } else if key == "show-detectors" {
            self.myShowDetectors = utils::common::string_utils::to_bool(value).unwrap_or_else(|e| panic!("{}", e));
            self.base2.set_parameter(key, value);
            for loopInfo in self.myInductLoops.iter() {
                loopInfo.loop_.m().set_visible(self.myShowDetectors);
            }
        } else if key == "inactive-threshold" {
            self.myInactiveThreshold = string2time(value).unwrap_or_else(|e| panic!("{}", e));
            self.base2.set_parameter(key, value);
        } else {
            self.base.set_parameter(key, value);
        }
    }

    /// C++: `std::map<std::string, double> MSActuatedTrafficLightLogic::getDetectorStates() const` [const, virtual]
    /// retrieve all detectors used by this program
    // std::map<std::string, double>
    // MSActuatedTrafficLightLogic::getDetectorStates() const {
    //     std::map<std::string, double> result;
    //     for (auto li : myInductLoops) {
    //         result[li.loop->getID()] = li.loop->getOccupancy() > 0 ? 1 : 0;
    //     }
    //     for (auto loop : myExtraLoops) {
    //         result[loop->getID()] = loop->getOccupancy() > 0 ? 1 : 0;
    //     }
    //     for (auto loop : myExtraE2) {
    //         result[loop->getID()] = loop->getCurrentVehicleNumber();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_states(&self) -> BTreeMap<String, f64> {
        let mut result: BTreeMap<String, f64> = BTreeMap::new();
        for li in self.myInductLoops.iter() {
            result.insert(li.loop_.m().base2.get_id().to_string(), if li.loop_.get_occupancy() > 0.0 { 1.0 } else { 0.0 });
        }
        for loop_ in self.myExtraLoops.iter() {
            result.insert(loop_.m().base2.get_id().to_string(), if loop_.m().get_occupancy() > 0.0 { 1.0 } else { 0.0 });
        }
        for loop_ in self.myExtraE2.iter() {
            result.insert(loop_.m().base2.get_id().to_string(), loop_.m().get_current_vehicle_number() as f64);
        }
        result
    }

    /// C++: `double MSActuatedTrafficLightLogic::getDetectorState(const std::string) const` [const, virtual]
    /// retrieve a specific detector used by this program
    // double
    // MSActuatedTrafficLightLogic::getDetectorState(const std::string laneID) const {
    //     double result = 0.0;
    //     for (auto li : myInductLoops) {
    //         if (li.lane->getID() == laneID) {
    //             result = li.loop->getOccupancy() > 0 ? 1 : 0;
    //             break;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_state(&self, laneID: String) -> f64 {
        let mut result: f64 = 0.0;
        for li in self.myInductLoops.iter() {
            if li.lane.get_id() == laneID.as_str() {
                result = if li.loop_.get_occupancy() > 0.0 { 1.0 } else { 0.0 };
                break;
            }
        }
        result
    }

    /// C++: `std::map<std::string, double> MSActuatedTrafficLightLogic::getConditions() const` [const, virtual]
    /// return all named conditions defined for this traffic light
    // std::map<std::string, double>
    // MSActuatedTrafficLightLogic::getConditions() const {
    //     std::map<std::string, double> result;
    //     for (auto item : myConditions) {
    //         if (myListedConditions.count(item.first) != 0) {
    //             try {
    //                 result[item.first] = evalExpression(item.second);
    //             } catch (ProcessError& e) {
    //                 WRITE_ERRORF(TL("Error when retrieving conditions '%' for tlLogic '%' (%)"), item.first, getID(), e.what());
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_conditions(&self) -> BTreeMap<String, f64> {
        let mut result: BTreeMap<String, f64> = BTreeMap::new();
        for (key, value) in self.myConditions.iter() {
            if self.myListedConditions.contains(key) {
                // shortcut: C++ catches the ProcessError of evalExpression and logs it; the Rust evalExpression panics instead, upgrade when it returns Result
                result.insert(key.clone(), self.eval_expression(value));
            }
        }
        result
    }

    /// C++: `void MSActuatedTrafficLightLogic::loadExtraState(const std::string &)` [virtual]
    /// restore the time since the last detection of every induction loop
    // void
    // MSActuatedTrafficLightLogic::loadExtraState(const std::string& state) {
    //     std::vector<std::string> timeGaps = StringTokenizer(state).getVector();
    //     int i = 0;
    //     for (const InductLoopInfo& loopInfo : myInductLoops) {
    //         MSInductLoop* loop = loopInfo.loop;
    //         if (i < (int)timeGaps.size()) {
    //             loop->loadTimeSinceLastDetection(StringUtils::toDouble(timeGaps[i]));
    //         }
    //         i++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_extra_state(&mut self, state: &str) {
        let timeGaps: Vec<String> = StringTokenizer::new(state).get_vector();
        for (i, loopInfo) in self.myInductLoops.iter().enumerate() {
            if i < timeGaps.len() {
                loopInfo.loop_.m().load_time_since_last_detection(utils::common::string_utils::to_double(&timeGaps[i]).unwrap());
            }
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::saveState(OutputDevice &) const` [const, virtual]
    /// save the time since the last detection of every induction loop
    // void
    // MSActuatedTrafficLightLogic::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_TLLOGIC);
    //     MSSimpleTrafficLightLogic::saveStateAttrs(out);
    //
    //     std::vector<double> state;
    //     for (const InductLoopInfo& loopInfo : myInductLoops) {
    //         MSInductLoop* loop = loopInfo.loop;
    //         state.push_back(loop->getTimeSinceLastDetection());
    //     }
    //     out.writeAttr(SUMO_ATTR_STATE, state);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_TLLOGIC);
        self.base.save_state_attrs(out);
        let mut state: Vec<f64> = Vec::new();
        for loopInfo in self.myInductLoops.iter() {
            state.push(loopInfo.loop_.get().unwrap().get_time_since_last_detection());
        }
        out.write_attr(SUMO_ATTR_STATE, &state);
        out.close_tag("");
    }

    /// C++: `void MSActuatedTrafficLightLogic::loadState(MSTLLogicControl &, SUMOTime, int, SUMOTime, SUMOTime, SUMOTime, bool)` [virtual]
    // void
    // MSActuatedTrafficLightLogic::loadState(MSTLLogicControl& tlcontrol, SUMOTime t, int step, SUMOTime spentDuration, SUMOTime nextSwitch, SUMOTime /*timeInCycle*/, bool active) {
    //     myAmActive = active;
    //     const SUMOTime lastSwitch = t - spentDuration;
    //     myStep = step;
    //     myPhases[myStep]->myLastSwitch = lastSwitch;
    //     mySwitchCommand->deschedule(this);
    //     mySwitchCommand = new SwitchCommand(tlcontrol, this, nextSwitch);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(mySwitchCommand, nextSwitch);
    //     if (myAmActive) {
    //         setTrafficLightSignals(lastSwitch);
    //     }
    //     tlcontrol.get(getID()).executeOnSwitchActions();
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, tlcontrol: &mut MSTLLogicControl, t: SUMOTime, step: i32, spentDuration: SUMOTime, nextSwitch: SUMOTime, timeInCycle: SUMOTime, active: bool) {
        self.base.base.myAmActive = active;
        let lastSwitch: SUMOTime = t - spentDuration;
        self.myStep = step;
        self.myPhases[self.myStep as usize].m().myLastSwitch = lastSwitch;
        let this = Ptr::from_mut(&mut self.base.base);
        self.base.base.mySwitchCommand.deschedule(this);
        self.base.base.mySwitchCommand = Ptr::from_raw(Box::into_raw(Box::new(MSTrafficLightLogic_SwitchCommand::new(tlcontrol, this, nextSwitch))));
        let events = MSNet::get_instance().m().get_begin_of_timestep_events();
        events.m().add_event(Ptr::from_raw(self.base.base.mySwitchCommand.as_raw() as *mut dyn crate::microsim::ms_event_control::Command), nextSwitch);
        if self.base.base.myAmActive {
            self.apply_phase_signals(lastSwitch);
        }
        let id = self.get_id().to_string();
        tlcontrol.get(&id).execute_on_switch_actions();
    }

    /// C++: `void MSActuatedTrafficLightLogic::initAttributeOverride()` [protected]
    /// initialize custom switching rules
    // void
    // MSActuatedTrafficLightLogic::initAttributeOverride() {
    //     const SUMOTime ovrd = MSPhaseDefinition::OVERRIDE_DURATION;
    //     for (int i = 0; i < (int)myPhases.size(); i++) {
    //         MSPhaseDefinition* phase = myPhases[i];
    //         const std::string errorSuffix = "' for overriding attribute in phase " + toString(i) + " of tlLogic '" + getID() + "' in program '" + getProgramID() + "'.";
    //         if (phase->minDuration == ovrd) {
    //             const std::string cond = "minDur:" + toString(i);
    //             if (myConditions.count(cond) == 0) {
    //                 throw ProcessError("Missing condition '" + cond + errorSuffix);
    //             }
    //         }
    //         if (phase->maxDuration == ovrd) {
    //             const std::string cond = "maxDur:" + toString(i);
    //             if (myConditions.count(cond) == 0) {
    //                 throw ProcessError("Missing condition '" + cond + errorSuffix);
    //             }
    //         }
    //         if (phase->earliestEnd == ovrd) {
    //             const std::string cond = "earliestEnd:" + toString(i);
    //             if (myConditions.count(cond) == 0) {
    //                 throw ProcessError("Missing condition '" + cond + errorSuffix);
    //             }
    //         }
    //         if (phase->latestEnd == ovrd) {
    //             const std::string cond = "latestEnd:" + toString(i);
    //             if (myConditions.count(cond) == 0) {
    //                 throw ProcessError("Missing condition '" + cond + errorSuffix);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_attribute_override(&mut self) {
        let ovrd: SUMOTime = OVERRIDE_DURATION;
        for i in 0..self.myPhases.len() {
            let phase = self.myPhases[i];
            let errorSuffix = format!("' for overriding attribute in phase {} of tlLogic '{}' in program '{}'.", i, self.get_id(), self.get_program_id());
            if phase.minDuration == ovrd {
                let cond = format!("minDur:{}", i);
                if !self.myConditions.contains_key(&cond) {
                    panic!("{}", ProcessError::new(format!("Missing condition '{}{}", cond, errorSuffix)));
                }
            }
            if phase.maxDuration == ovrd {
                let cond = format!("maxDur:{}", i);
                if !self.myConditions.contains_key(&cond) {
                    panic!("{}", ProcessError::new(format!("Missing condition '{}{}", cond, errorSuffix)));
                }
            }
            if phase.earliestEnd == ovrd {
                let cond = format!("earliestEnd:{}", i);
                if !self.myConditions.contains_key(&cond) {
                    panic!("{}", ProcessError::new(format!("Missing condition '{}{}", cond, errorSuffix)));
                }
            }
            if phase.latestEnd == ovrd {
                let cond = format!("latestEnd:{}", i);
                if !self.myConditions.contains_key(&cond) {
                    panic!("{}", ProcessError::new(format!("Missing condition '{}{}", cond, errorSuffix)));
                }
            }
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::initSwitchingRules()` [protected]
    // void
    // MSActuatedTrafficLightLogic::initSwitchingRules() {
    //     for (int i = 0; i < (int)myPhases.size(); i++) {
    //         SwitchingRules sr;
    //         MSPhaseDefinition* phase = myPhases[i];
    //         std::vector<int> nextPhases = phase->nextPhases;
    //         if (nextPhases.size() == 0) {
    //             nextPhases.push_back((i + 1) % (int)myPhases.size());
    //         } else if (nextPhases.size() > 1) {
    //             myHasMultiTarget = true;
    //         }
    //         for (int next : nextPhases) {
    //             if (next >= 0 && next < (int)myPhases.size()) {
    //                 const MSPhaseDefinition* nextPhase = myPhases[next];
    //                 if (nextPhase->earlyTarget != "" || nextPhase->finalTarget != "") {
    //                     sr.enabled = true;
    //                 }
    //             }
    //         }
    //         // simplifies later code
    //         phase->nextPhases = nextPhases;
    //         mySwitchingRules.push_back(sr);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_switching_rules(&mut self) {
        for i in 0..self.myPhases.len() {
            let mut sr = MSActuatedTrafficLightLogic_SwitchingRules { enabled: false };
            let phase = self.myPhases[i];
            let mut nextPhases: Vec<i32> = phase.nextPhases.clone();
            if nextPhases.len() == 0 {
                nextPhases.push(((i + 1) % self.myPhases.len()) as i32);
            } else if nextPhases.len() > 1 {
                self.myHasMultiTarget = true;
            }
            for next in nextPhases.iter().copied() {
                if next >= 0 && next < self.myPhases.len() as i32 {
                    let nextPhase = self.myPhases[next as usize];
                    if nextPhase.earlyTarget != "" || nextPhase.finalTarget != "" {
                        sr.enabled = true;
                    }
                }
            }
            // simplifies later code
            self.myPhases[i].m().nextPhases = nextPhases;
            self.mySwitchingRules.push(sr);
        }
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::duration(const double) const` [const, protected]
    /// Returns the minimum duration of the current phase
    // SUMOTime
    // MSActuatedTrafficLightLogic::duration(const double detectionGap) const {
    //     assert(getCurrentPhaseDef().isGreenPhase());
    //     assert((int)myPhases.size() > myStep);
    //     const SUMOTime actDuration = MSNet::getInstance()->getCurrentTimeStep() - myPhases[myStep]->myLastSwitch;
    //     // ensure that minimum duration is kept
    //     SUMOTime newDuration = getMinDur() - actDuration;
    //     // try to let the last detected vehicle pass the intersection (duration must be positive)
    //     newDuration = MAX3(newDuration, TIME2STEPS(myDetectorGap - detectionGap), SUMOTime(1));
    //     // cut the decimal places to ensure that phases always have integer duration
    //     if (newDuration % 1000 != 0) {
    //         const SUMOTime totalDur = newDuration + actDuration;
    //         newDuration = (totalDur / 1000 + 1) * 1000 - actDuration;
    //     }
    //     // ensure that the maximum duration is not exceeded
    //     newDuration = MIN3(newDuration, getMaxDur() - actDuration, getLatest());
    //     return newDuration;
    // }
    #[allow(unused_variables)]
    pub fn duration(&self, detectionGap: f64) -> SUMOTime {
        debug_assert!(self.get_current_phase_def().is_green_phase());
        debug_assert!((self.myPhases.len() as i32) > self.myStep);
        let actDuration: SUMOTime = MSNet::get_instance().get_current_time_step() - self.myPhases[self.myStep as usize].myLastSwitch;
        // ensure that minimum duration is kept
        let mut newDuration: SUMOTime = self.get_min_dur(-1) - actDuration;
        // try to let the last detected vehicle pass the intersection (duration must be positive)
        newDuration = std::cmp::max(std::cmp::max(newDuration, time2steps(self.myDetectorGap - detectionGap)), 1);
        // cut the decimal places to ensure that phases always have integer duration
        if newDuration % 1000 != 0 {
            let totalDur: SUMOTime = newDuration + actDuration;
            newDuration = (totalDur / 1000 + 1) * 1000 - actDuration;
        }
        // ensure that the maximum duration is not exceeded
        newDuration = std::cmp::min(std::cmp::min(newDuration, self.get_max_dur(-1) - actDuration), self.get_latest());
        newDuration
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getMinimumMinDuration(MSLane *, const std::set<int> &) const` [const, protected]
    /// get the minimum min duration for all stretchable phases that affect the given lane
    // SUMOTime
    // MSActuatedTrafficLightLogic::getMinimumMinDuration(MSLane* lane, const std::set<int>& multiNextTargets) const {
    //     SUMOTime result = std::numeric_limits<SUMOTime>::max();
    //     for (int pI = 0; pI < (int)myPhases.size(); pI++) {
    //         const MSPhaseDefinition* phase = myPhases[pI];
    //         const std::string& state = phase->getState();
    //         for (int i = 0; i < (int)state.size(); i++)  {
    //             if (state[i] == LINKSTATE_TL_GREEN_MAJOR || state[i] == LINKSTATE_TL_GREEN_MINOR) {
    //                 for (MSLane* cand : getLanesAt(i)) {
    //                     if (lane == cand) {
    //                         if (phase->isActuated()) {
    //                             result = MIN2(result, getMinDur(pI));
    //                         } else if (multiNextTargets.count(pI) != 0) {
    //                             result = MIN2(result, DEFAULT_STATIC_MINDUR);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_minimum_min_duration(&self, lane: Ptr<MSLane>, multiNextTargets: &BTreeSet<i32>) -> SUMOTime {
        let mut result: SUMOTime = i64::MAX;
        for pI in 0..self.myPhases.len() {
            let phase = self.myPhases[pI];
            let state: String = phase.get_state().to_string();
            for i in 0..state.len() {
                let s: LinkState = (state.as_bytes()[i] as i8) as LinkState;
                if s == LINKSTATE_TL_GREEN_MAJOR || s == LINKSTATE_TL_GREEN_MINOR {
                    for cand in self.get_lanes_at(i as i32).clone() {
                        if lane == cand {
                            if phase.is_actuated() {
                                result = std::cmp::min(result, self.get_min_dur(pI as i32));
                            } else if multiNextTargets.contains(&(pI as i32)) {
                                result = std::cmp::min(result, DEFAULT_STATIC_MINDUR);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// C++: `double MSActuatedTrafficLightLogic::gapControl()` [protected]
    /// Return the minimum detection gap of all detectors if the current phase should be extended and double::max otherwise
    // double
    // MSActuatedTrafficLightLogic::gapControl() {
    //     //intergreen times should not be lengthend
    //     assert((int)myPhases.size() > myStep);
    //     double result = std::numeric_limits<double>::max();
    //     // switch off active colors
    //     if (myShowDetectors) {
    //         for (InductLoopInfo& loopInfo : myInductLoops) {
    //             if (loopInfo.lastGreenTime < loopInfo.loop->getLastDetectionTime()) {
    //                 loopInfo.loop->setSpecialColor(&RGBColor::RED);
    //             } else {
    //                 loopInfo.loop->setSpecialColor(nullptr);
    //             }
    //         }
    //     }
    //     if (!getCurrentPhaseDef().isGreenPhase()) {
    //         return result; // end current phase
    //     }
    //
    //     // Checks, if the maxDuration is kept. No phase should last longer than maxDuration.
    //     SUMOTime actDuration = MSNet::getInstance()->getCurrentTimeStep() - myPhases[myStep]->myLastSwitch;
    //     if (actDuration >= getCurrentPhaseDef().maxDuration || maxLinkDurationReached() || getLatest() == 0) {
    // #ifdef DEBUG_PHASE_SELECTION
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " actDuration=" << STEPS2TIME(actDuration) << " maxDur=" << STEPS2TIME(getCurrentPhaseDef().maxDuration)
    //                       << " maxLinkDurationReached=" << maxLinkDurationReached() << " latest=" << STEPS2TIME(getLatest()) << "\n";
    //         }
    // #endif
    //         return result; // end current phase
    //     }
    //
    //     // now the gapcontrol starts
    //     for (InductLoopInfo* loopInfo : myInductLoopsForPhase[myStep]) {
    //         MSInductLoop* loop = loopInfo->loop;
    //         if (loopInfo->isJammed()) {
    //             loopInfo->loop->setSpecialColor(&RGBColor::ORANGE);
    //         } else {
    //             loopInfo->loop->setSpecialColor(&RGBColor::GREEN);
    //         }
    //         const double actualGap = loop->getTimeSinceLastDetection();
    //         if (actualGap < loopInfo->maxGap && !loopInfo->isJammed()) {
    //             result = MIN2(result, actualGap);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn gap_control(&mut self) -> f64 {
        //intergreen times should not be lengthend
        debug_assert!((self.myPhases.len() as i32) > self.myStep);
        let mut result: f64 = f64::MAX;
        // switch off active colors
        if self.myShowDetectors {
            for loopInfo in self.myInductLoops.iter() {
                if loopInfo.lastGreenTime < loopInfo.loop_.get_last_detection_time() {
                    loopInfo.loop_.m().set_special_color(Ptr::new(&RGB_RED));
                } else {
                    loopInfo.loop_.m().set_special_color(Ptr::null());
                }
            }
        }
        if !self.get_current_phase_def().is_green_phase() {
            return result; // end current phase
        }
        // Checks, if the maxDuration is kept. No phase should last longer than maxDuration.
        let actDuration: SUMOTime = MSNet::get_instance().get_current_time_step() - self.myPhases[self.myStep as usize].myLastSwitch;
        if actDuration >= self.get_current_phase_def().maxDuration || self.max_link_duration_reached() || self.get_latest() == 0 {
            return result; // end current phase
        }
        // now the gapcontrol starts
        for loopInfo in self.myInductLoopsForPhase[self.myStep as usize].clone() {
            let loop_: Ptr<MSInductLoop> = loopInfo.loop_;
            if loopInfo.is_jammed() {
                loop_.m().set_special_color(Ptr::new(&RGB_ORANGE));
            } else {
                loop_.m().set_special_color(Ptr::new(&RGB_GREEN));
            }
            let actualGap: f64 = loop_.get_time_since_last_detection();
            if actualGap < loopInfo.maxGap && !loopInfo.is_jammed() {
                result = f64::min(result, actualGap);
            }
        }
        result
    }

    /// C++: `bool MSActuatedTrafficLightLogic::hasMajor(const std::string &, const LaneVector &) const` [const, protected]
    /// return whether there is a major link from the given lane in the given phase
    // bool
    // MSActuatedTrafficLightLogic::hasMajor(const std::string& state, const LaneVector& lanes) const {
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
            if ((state.as_bytes()[i] as i8) as LinkState) == LINKSTATE_TL_GREEN_MAJOR {
                for cand in self.get_lanes_at(i as i32).clone() {
                    for lane in lanes.iter() {
                        if *lane == cand {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// C++: `int MSActuatedTrafficLightLogic::decideNextPhase()` [protected]
    /// select among candidate phases based on detector states
    // int
    // MSActuatedTrafficLightLogic::decideNextPhase() {
    //     const auto& cands = myPhases[myStep]->nextPhases;
    //     // decide by priority
    //     // first target is the default when there is no traffic
    //     // @note: to keep the current phase, even when there is no traffic, it must be added to 'next' explicitly
    //     int result = cands.front();
    //     int maxPrio = 0;
    //     SUMOTime actDuration = MSNet::getInstance()->getCurrentTimeStep() - myPhases[myStep]->myLastSwitch;
    //     const bool canExtend = actDuration < getCurrentPhaseDef().maxDuration && !maxLinkDurationReached() && getLatest() > 0;
    //     if (canExtend) {
    //         // consider keeping the current phase until maxDur is reached
    //         // (only when there is still traffic in that phase)
    //         int currentPrio = getPhasePriority(myStep);
    // #ifdef DEBUG_PHASE_SELECTION
    //         std::cout << SIMTIME << " p=" << myStep << " loops=" << myInductLoopsForPhase[myStep].size() << " currentPrio=" << currentPrio << "\n";
    // #endif
    //         if (currentPrio > maxPrio) {
    //             result = myStep;
    //             maxPrio = currentPrio;
    //         }
    //     }
    //     for (int step : cands) {
    //         int prio = 0;
    //         for (int target : myTargets[myStep][step]) {
    //             prio += getPhasePriority(target);
    // #ifdef DEBUG_PHASE_SELECTION
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " p=" << myStep << " step=" << step << " target=" << target << " loops=" << myInductLoopsForPhase[target].size() << " prio=" << prio << "\n";
    //             }
    // #endif
    //         }
    //         if (prio > maxPrio && canExtendLinkGreen(getTarget(step).first)) {
    //             maxPrio = prio;
    //             result = step;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        let cands: Vec<i32> = self.myPhases[self.myStep as usize].nextPhases.clone();
        // decide by priority
        // first target is the default when there is no traffic
        // @note: to keep the current phase, even when there is no traffic, it must be added to 'next' explicitly
        let mut result: i32 = cands[0];
        let mut maxPrio: i32 = 0;
        let actDuration: SUMOTime = MSNet::get_instance().get_current_time_step() - self.myPhases[self.myStep as usize].myLastSwitch;
        let canExtend = actDuration < self.get_current_phase_def().maxDuration && !self.max_link_duration_reached() && self.get_latest() > 0;
        if canExtend {
            // consider keeping the current phase until maxDur is reached
            // (only when there is still traffic in that phase)
            let currentPrio = self.get_phase_priority(self.myStep);
            if currentPrio > maxPrio {
                result = self.myStep;
                maxPrio = currentPrio;
            }
        }
        for step in cands.iter().copied() {
            let mut prio: i32 = 0;
            let targets: Vec<i32> = self.myTargets.entry(self.myStep).or_default().entry(step).or_default().clone();
            for target in targets {
                prio += self.get_phase_priority(target);
            }
            if prio > maxPrio && self.can_extend_link_green(self.get_target(step).0) {
                maxPrio = prio;
                result = step;
            }
        }
        result
    }

    /// C++: `int MSActuatedTrafficLightLogic::decideNextPhaseCustom(bool)` [protected]
    /// select among candidate phases based on detector states and custom switching rules
    // int
    // MSActuatedTrafficLightLogic::decideNextPhaseCustom(bool mustSwitch) {
    //     for (int next : getCurrentPhaseDef().nextPhases) {
    //         const MSPhaseDefinition* phase = myPhases[next];
    //         const std::string& condition = mustSwitch ? phase->finalTarget : phase->earlyTarget;
    // #ifdef DEBUG_PHASE_SELECTION_CUSTOM
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " mustSwitch=" << mustSwitch << " cur=" << myStep << " next=" << next << " condition=" << condition
    //                       << " eval=" << (condition == "" ? NAN : evalExpression(condition)) << "\n";
    //         }
    // #endif
    //         if (condition != "") {
    //             // backward compatibility if a user redefined DEFAULT_CONDITION
    //             if (condition == DEFAULT_CONDITION && myConditions.count(DEFAULT_CONDITION) == 0) {
    //                 if (gapControl() == std::numeric_limits<double>::max()) {
    //                     return next;
    //                 }
    //             } else if (evalExpression(condition)) {
    //                 return next;
    //             }
    //         }
    //     }
    //     return mustSwitch ? getCurrentPhaseDef().nextPhases.back() : myStep;
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase_custom(&mut self, mustSwitch: bool) -> i32 {
        for next in self.get_current_phase_def().nextPhases.clone() {
            let phase = self.myPhases[next as usize];
            let condition: String = if mustSwitch { phase.finalTarget.clone() } else { phase.earlyTarget.clone() };
            if condition != "" {
                // backward compatibility if a user redefined DEFAULT_CONDITION
                if condition == DEFAULT_CONDITION && !self.myConditions.contains_key(DEFAULT_CONDITION) {
                    if self.gap_control() == f64::MAX {
                        return next;
                    }
                } else if self.eval_expression(&condition) != 0.0 {
                    return next;
                }
            }
        }
        if mustSwitch {
            *self.get_current_phase_def().nextPhases.last().unwrap()
        } else {
            self.myStep
        }
    }

    /// C++: `double MSActuatedTrafficLightLogic::evalExpression(const std::string &) const` [const, protected]
    /// evaluate custom switching condition
    // double
    // MSActuatedTrafficLightLogic::evalExpression(const std::string& condition) const {
    //     const size_t bracketOpen = condition.find('(');
    //     if (bracketOpen != std::string::npos) {
    //         // find matching closing bracket
    //         size_t bracketClose = std::string::npos;
    //         int open = 1;
    //         for (size_t i = bracketOpen + 1; i < condition.size(); i++) {
    //             if (condition[i] == '(') {
    //                 open++;
    //             } else if (condition[i] == ')') {
    //                 open--;
    //                 if (open == 0) {
    //                     bracketClose = i;
    //                     break;
    //                 }
    //             }
    //         }
    //         if (bracketClose == std::string::npos) {
    //             throw ProcessError(TLF("Unmatched parentheses in condition %'", condition));
    //         }
    //         std::string cond2 = condition;
    //         const std::string inBracket = condition.substr(bracketOpen + 1, bracketClose - bracketOpen - 1);
    //         double bracketVal = evalExpression(inBracket);
    //         cond2.replace(bracketOpen, bracketClose - bracketOpen + 1, toString(bracketVal));
    //         try {
    //             return evalExpression(cond2);
    //         } catch (ProcessError& e) {
    //             throw ProcessError(TLF("Error when evaluating expression '%':\n  %", condition, e.what()));
    //         }
    //     }
    //     std::vector<std::string> tokens = StringTokenizer(condition).getVector();
    //     //std::cout << SIMTIME << " tokens(" << tokens.size() << ")=" << toString(tokens) << "\n";
    //     if (tokens.size() == 0) {
    //         throw ProcessError(TLF("Invalid empty condition '%'", condition));
    //     } else if (tokens.size() == 1) {
    //         try {
    //             return evalAtomicExpression(tokens[0]);
    //         } catch (ProcessError& e) {
    //             throw ProcessError(TLF("Error when evaluating expression '%':\n  %", condition, e.what()));
    //         }
    //     } else if (tokens.size() == 2) {
    //         if (tokens[0] == "not") {
    //             try {
    //                 return evalAtomicExpression(tokens[1]) == 0. ? 1. : 0.;
    //             } catch (ProcessError& e) {
    //                 throw ProcessError(TLF("Error when evaluating expression '%':\n  %", condition, e.what()));
    //             }
    //         } else {
    //             throw ProcessError(TLF("Unsupported condition '%'", condition));
    //         }
    //     } else if (tokens.size() == 3) {
    //         // infix expression
    //         const double a = evalAtomicExpression(tokens[0]);
    //         const double b = evalAtomicExpression(tokens[2]);
    //         const std::string& o = tokens[1];
    //         //std::cout << SIMTIME << " o=" << o << " a=" << a << " b=" << b << "\n";
    //         try {
    //             return evalTernaryExpression(a, o, b, condition);
    //         } catch (ProcessError& e) {
    //             throw ProcessError(TLF("Error when evaluating expression '%':\n  %", condition, e.what()));
    //         }
    //     } else {
    //         const int iEnd = (int)tokens.size() - 1;
    //         for (const std::string& o : OPERATOR_PRECEDENCE) {
    //             for (int i = 1; i < iEnd; i++) {
    //                 if (tokens[i] == o) {
    //                     try {
    //                         const double val = evalTernaryExpression(
    //                                                evalAtomicExpression(tokens[i - 1]), o,
    //                                                evalAtomicExpression(tokens[i + 1]), condition);
    //                         std::vector<std::string> newTokens(tokens.begin(), tokens.begin() + (i - 1));
    //                         newTokens.push_back(toString(val));
    //                         newTokens.insert(newTokens.end(), tokens.begin() + (i + 2), tokens.end());
    //                         return evalExpression(toString(newTokens));
    //                     } catch (ProcessError& e) {
    //                         throw ProcessError(TLF("Error when evaluating expression '%':\n  %", condition, e.what()));
    //                     }
    //                 }
    //             }
    //         }
    //         throw ProcessError(TLF("Parsing expressions with % elements ('%') is not supported", toString(tokens.size()), condition));
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn eval_expression(&self, condition: &str) -> f64 {
        // find the matching closing bracket of the first opening one
        if let Some(bracketOpen) = condition.find('(') {
            let mut bracketClose: Option<usize> = None;
            let mut open: i32 = 1;
            for i in (bracketOpen + 1)..condition.len() {
                let c = condition.as_bytes()[i] as char;
                if c == '(' {
                    open += 1;
                } else if c == ')' {
                    open -= 1;
                    if open == 0 {
                        bracketClose = Some(i);
                        break;
                    }
                }
            }
            let Some(bracketClose) = bracketClose else {
                panic!("{}", ProcessError::new(format!("Unmatched parentheses in condition {}'", condition)));
            };
            let mut cond2: String = condition.to_string();
            let inBracket: &str = &condition[bracketOpen + 1..bracketClose];
            let bracketVal: f64 = self.eval_expression(inBracket);
            // shortcut: C++ wraps nested evaluation errors with the outer condition; errors here panic instead of throwing, so that wrapping is not reproduced
            cond2.replace_range(bracketOpen..=bracketClose, &utils::common::to_string::to_string(&bracketVal));
            return self.eval_expression(&cond2);
        }
        let tokens: Vec<String> = utils::common::string_tokenizer::StringTokenizer::new(condition).get_vector();
        if tokens.len() == 0 {
            panic!("{}", ProcessError::new(format!("Invalid empty condition '{}'", condition)));
        } else if tokens.len() == 1 {
            return self.eval_atomic_expression(&tokens[0]);
        } else if tokens.len() == 2 {
            if tokens[0] == "not" {
                return if self.eval_atomic_expression(&tokens[1]) == 0.0 { 1.0 } else { 0.0 };
            } else {
                panic!("{}", ProcessError::new(format!("Unsupported condition '{}'", condition)));
            }
        } else if tokens.len() == 3 {
            // infix expression
            let a: f64 = self.eval_atomic_expression(&tokens[0]);
            let b: f64 = self.eval_atomic_expression(&tokens[2]);
            let o: String = tokens[1].clone();
            return self.eval_ternary_expression(a, &o, b, condition);
        } else {
            let iEnd: usize = tokens.len() - 1;
            for o in OPERATOR_PRECEDENCE.iter() {
                for i in 1..iEnd {
                    if tokens[i] == *o {
                        let val: f64 = self.eval_ternary_expression(
                            self.eval_atomic_expression(&tokens[i - 1]), o,
                            self.eval_atomic_expression(&tokens[i + 1]), condition);
                        let mut newTokens: Vec<String> = tokens[..i - 1].to_vec();
                        newTokens.push(utils::common::to_string::to_string(&val));
                        newTokens.extend_from_slice(&tokens[i + 2..]);
                        return self.eval_expression(&utils::common::to_string::to_string(&newTokens));
                    }
                }
            }
            panic!("{}", ProcessError::new(format!("Parsing expressions with {} elements ('{}') is not supported", tokens.len(), condition)));
        }
    }

    /// C++: `double MSActuatedTrafficLightLogic::evalTernaryExpression(double, const std::string &, double, const std::string &) const` [const, protected]
    /// evaluate atomic expression
    // double
    // MSActuatedTrafficLightLogic::evalTernaryExpression(double a, const std::string& o, double b, const std::string& condition) const {
    //     if (o == "=" || o == "==") {
    //         return (double)(a == b);
    //     } else if (o == "<") {
    //         return (double)(a < b);
    //     } else if (o == ">") {
    //         return (double)(a > b);
    //     } else if (o == "<=") {
    //         return (double)(a <= b);
    //     } else if (o == ">=") {
    //         return (double)(a >= b);
    //     } else if (o == "!=") {
    //         return (double)(a != b);
    //     } else if (o == "or" || o == "||") {
    //         return (double)(a || b);
    //     } else if (o == "and" || o == "&&") {
    //         return (double)(a && b);
    //     } else if (o == "+") {
    //         return a + b;
    //     } else if (o == "-") {
    //         return a - b;
    //     } else if (o == "*") {
    //         return a * b;
    //     } else if (o == "/") {
    //         if (b == 0) {
    //             WRITE_ERRORF(TL("Division by 0 in condition '%'"), condition);
    //             return 0;
    //         }
    //         return a / b;
    //     } else if (o == "%") {
    //         return fmod(a, b);
    //     } else if (o == "**" || o == "^") {
    //         return pow(a, b);
    //     } else  {
    //         throw ProcessError(TLF("Unsupported operator '%' in condition '%'", o, condition));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn eval_ternary_expression(&self, a: f64, o: &str, b: f64, condition: &str) -> f64 {
        if o == "=" || o == "==" {
            (a == b) as i32 as f64
        } else if o == "<" {
            (a < b) as i32 as f64
        } else if o == ">" {
            (a > b) as i32 as f64
        } else if o == "<=" {
            (a <= b) as i32 as f64
        } else if o == ">=" {
            (a >= b) as i32 as f64
        } else if o == "!=" {
            (a != b) as i32 as f64
        } else if o == "or" || o == "||" {
            (a != 0.0 || b != 0.0) as i32 as f64
        } else if o == "and" || o == "&&" {
            (a != 0.0 && b != 0.0) as i32 as f64
        } else if o == "+" {
            a + b
        } else if o == "-" {
            a - b
        } else if o == "*" {
            a * b
        } else if o == "/" {
            if b == 0.0 {
                utils::write_errorf!("Division by 0 in condition '%'", condition);
                return 0.0;
            }
            a / b
        } else if o == "%" {
            a % b
        } else if o == "**" || o == "^" {
            a.powf(b)
        } else {
            panic!("{}", ProcessError::new(format!("Unsupported operator '{}' in condition '{}'", o, condition)));
        }
    }

    /// C++: `double MSActuatedTrafficLightLogic::evalAtomicExpression(const std::string &) const` [const, protected]
    /// evaluate atomic expression
    // double
    // MSActuatedTrafficLightLogic::evalAtomicExpression(const std::string& expr) const {
    //     if (expr.size() == 0) {
    //         throw ProcessError(TL("Invalid empty expression"));
    //     } else if (expr[0] == '!') {
    //         return evalAtomicExpression(expr.substr(1)) == 0. ? 1. : 0.;
    //     } else if (expr[0] == '-') {
    //         return -evalAtomicExpression(expr.substr(1));
    //     } else {
    //         // check for 'operator:'
    //         const size_t pos = expr.find(':');
    //         if (pos == std::string::npos) {
    //             auto it = myConditions.find(expr);
    //             if (it != myConditions.end()) {
    //                 // symbol lookup
    //                 return evalExpression(it->second);
    //             } else {
    //                 // look at stack
    //                 auto it2 = myStack.back().find(expr);
    //                 if (it2 != myStack.back().end()) {
    //                     return it2->second;
    //                 }
    //                 // must be a number
    //                 return StringUtils::toDouble(expr);
    //             }
    //         } else {
    //             const std::string fun = expr.substr(0, pos);
    //             const std::string arg = expr.substr(pos + 1);
    //             if (fun == "z") {
    //                 return retrieveDetExpression<MSInductLoop, SUMO_TAG_INDUCTION_LOOP>(arg, expr, true)->getTimeSinceLastDetection();
    //             } else if (fun == "a") {
    //                 try {
    //                     return retrieveDetExpression<MSInductLoop, SUMO_TAG_INDUCTION_LOOP>(arg, expr, true)->getTimeSinceLastDetection() == 0;
    //                 } catch (ProcessError&) {
    //                     return retrieveDetExpression<MSE2Collector, SUMO_TAG_LANE_AREA_DETECTOR>(arg, expr, true)->getCurrentVehicleNumber();
    //                 }
    //             } else if (fun == "w") {
    //                 try {
    //                     return retrieveDetExpression<MSInductLoop, SUMO_TAG_INDUCTION_LOOP>(arg, expr, true)->getOccupancyTime();
    //                 } catch (ProcessError&) {
    //                     return retrieveDetExpression<MSE2Collector, SUMO_TAG_LANE_AREA_DETECTOR>(arg, expr, true)->getCurrentJamDuration();
    //                 }
    //             } else if (fun == "d") {
    //                 try {
    //                     return retrieveDetExpression<MSInductLoop, SUMO_TAG_INDUCTION_LOOP>(arg, expr, true)->getArrivalDelay();
    //                 } catch (ProcessError&) {
    //                     return retrieveDetExpression<MSE2Collector, SUMO_TAG_LANE_AREA_DETECTOR>(arg, expr, true)->getArrivalDelay();
    //                 }
    //             } else if (fun == "g" || fun == "r") {
    //                 try {
    //                     int linkIndex = StringUtils::toInt(arg);
    //                     if (linkIndex >= 0 && linkIndex < myNumLinks) {
    //                         const std::vector<SUMOTime>& times = fun == "g" ? myLinkGreenTimes : myLinkRedTimes;
    //                         if (times.empty()) {
    //                             return 0;
    //                         }
    //                         if (myLastTrySwitchTime < SIMSTEP) {
    //                             // times are only updated at the start of a phase where
    //                             // switching is possible (i.e. not during minDur).
    //                             // If somebody is looking at those values in the tracker
    //                             // this would be confusing
    //                             const LinkState ls = getCurrentPhaseDef().getSignalState(linkIndex);
    //                             if ((fun == "g" && (ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR))
    //                                     || (fun == "r" && (ls == LINKSTATE_TL_RED || ls == LINKSTATE_TL_REDYELLOW))) {
    //                                 const SUMOTime currentGreen = SIMSTEP - myLastTrySwitchTime;
    //                                 return STEPS2TIME(times[linkIndex] + currentGreen);
    //                             } else {
    //                                 return 0;
    //                             }
    //                         } else {
    //                             return STEPS2TIME(times[linkIndex]);
    //                         }
    //                     }
    //                 } catch (NumberFormatException&) { }
    //                 throw ProcessError(TLF("Invalid link index '%' in expression '%'", arg, expr));
    //             } else if (fun == "p") {
    //                 try {
    //                     int linkIndex = StringUtils::toInt(arg);
    //                     if (linkIndex >= 0 && linkIndex < myNumLinks) {
    //                         double approachingPersons = 0;
    //                         for (const MSLink* link : getLinksAt(linkIndex)) {
    //                             auto* aPersons = link->getApproachingPersons();
    //                             if (aPersons != nullptr) {
    //                                 approachingPersons += (double)aPersons->size();
    //                             }
    //                         }
    //                         return approachingPersons;
    //                     }
    //                 } catch (NumberFormatException&) { }
    //                 throw ProcessError(TLF("Invalid link index '%' in expression '%'", arg, expr));
    //             } else if (fun == "c") {
    //                 return STEPS2TIME(getTimeInCycle());
    //             } else {
    //                 if (myFunctions.find(fun) == myFunctions.end()) {
    //                     throw ProcessError(TLF("Unsupported function '%' in expression '%'", fun, expr));
    //                 }
    //                 return evalCustomFunction(fun, arg);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn eval_atomic_expression(&self, expr: &str) -> f64 {
        if expr.len() == 0 {
            panic!("{}", ProcessError::new("Invalid empty expression"));
        } else if expr.as_bytes()[0] == b'!' {
            return if self.eval_atomic_expression(&expr[1..]) == 0.0 { 1.0 } else { 0.0 };
        } else if expr.as_bytes()[0] == b'-' {
            return -self.eval_atomic_expression(&expr[1..]);
        }
        // check for 'operator:'
        match expr.find(':') {
            None => {
                if let Some(cond) = self.myConditions.get(expr) {
                    // symbol lookup
                    return self.eval_expression(cond);
                }
                // look at stack
                if let Some(v) = self.myStack.borrow().last().and_then(|m| m.get(expr)).copied() {
                    return v;
                }
                // must be a number
                utils::common::string_utils::to_double(expr).unwrap_or_else(|e| panic!("{}", e))
            }
            Some(pos) => {
                let fun: &str = &expr[..pos];
                let arg: &str = &expr[pos + 1..];
                if fun == "z" {
                    return self.retrieve_det_expression_loop_or_throw(arg, expr).get_time_since_last_detection();
                } else if fun == "a" {
                    if let Some(loop_) = self.retrieve_det_expression_loop(arg, true) {
                        return if loop_.get_time_since_last_detection() == 0.0 { 1.0 } else { 0.0 };
                    }
                    return self.retrieve_det_expression_e2_or_throw(arg, expr).get_current_vehicle_number() as f64;
                } else if fun == "w" {
                    if let Some(loop_) = self.retrieve_det_expression_loop(arg, true) {
                        return loop_.get_occupancy_time();
                    }
                    return self.retrieve_det_expression_e2_or_throw(arg, expr).get_current_jam_duration();
                } else if fun == "d" {
                    if let Some(loop_) = self.retrieve_det_expression_loop(arg, true) {
                        return loop_.get_arrival_delay();
                    }
                    return self.retrieve_det_expression_e2_or_throw(arg, expr).get_arrival_delay();
                } else if fun == "g" || fun == "r" {
                    if let Ok(linkIndex) = utils::common::string_utils::to_int(arg) {
                        if linkIndex >= 0 && linkIndex < self.myNumLinks {
                            let times: &Vec<SUMOTime> = if fun == "g" { &self.myLinkGreenTimes } else { &self.myLinkRedTimes };
                            if times.is_empty() {
                                return 0.0;
                            }
                            let now = MSNet::get_instance().get_current_time_step();
                            if self.myLastTrySwitchTime < now {
                                // times are only updated at the start of a phase where
                                // switching is possible (i.e. not during minDur).
                                // If somebody is looking at those values in the tracker
                                // this would be confusing
                                let ls: LinkState = self.get_current_phase_def().get_signal_state(linkIndex);
                                if (fun == "g" && (ls == LINKSTATE_TL_GREEN_MAJOR || ls == LINKSTATE_TL_GREEN_MINOR))
                                    || (fun == "r" && (ls == LINKSTATE_TL_RED || ls == LINKSTATE_TL_REDYELLOW)) {
                                    let currentGreen: SUMOTime = now - self.myLastTrySwitchTime;
                                    return steps2time(times[linkIndex as usize] + currentGreen);
                                } else {
                                    return 0.0;
                                }
                            } else {
                                return steps2time(times[linkIndex as usize]);
                            }
                        }
                    }
                    panic!("{}", ProcessError::new(format!("Invalid link index '{}' in expression '{}'", arg, expr)));
                } else if fun == "p" {
                    if let Ok(linkIndex) = utils::common::string_utils::to_int(arg) {
                        if linkIndex >= 0 && linkIndex < self.myNumLinks {
                            let mut approachingPersons: f64 = 0.0;
                            for link in self.get_links_at(linkIndex).clone() {
                                let aPersons = link.get_approaching_persons();
                                if !aPersons.is_null() {
                                    approachingPersons += aPersons.len() as f64;
                                }
                            }
                            return approachingPersons;
                        }
                    }
                    panic!("{}", ProcessError::new(format!("Invalid link index '{}' in expression '{}'", arg, expr)));
                } else if fun == "c" {
                    let now = MSNet::get_instance().get_current_time_step();
                    return steps2time(self.map_time_in_cycle(now));
                } else {
                    if !self.myFunctions.contains_key(fun) {
                        panic!("{}", ProcessError::new(format!("Unsupported function '{}' in expression '{}'", fun, expr)));
                    }
                    return self.eval_custom_function(fun, arg);
                }
            }
        }
    }

    /// C++: `double MSActuatedTrafficLightLogic::evalCustomFunction(const std::string &, const std::string &) const` [const, protected]
    /// evaluate function expression
    // double
    // MSActuatedTrafficLightLogic::evalCustomFunction(const std::string& fun, const std::string& arg) const {
    //     std::vector<std::string> args = StringTokenizer(arg, ",").getVector();
    //     const Function& f = myFunctions.find(fun)->second;
    //     if ((int)args.size() != f.nArgs) {
    //         throw ProcessError(TLF("Function '%' requires % arguments but % were given", fun, toString(f.nArgs), toString(args.size())));
    //     }
    //     std::vector<double> args2;
    //     for (auto a : args) {
    //         args2.push_back(evalExpression(a));
    //     }
    //     myStack.push_back(myStack.back());
    //     myStack.back()["$0"] = 0;
    //     for (int i = 0; i < (int)args2.size(); i++) {
    //         myStack.back()["$" + toString(i + 1)] = args2[i];
    //     }
    //     try {
    //         ConditionMap empty;
    //         executeAssignments(f.assignments, empty, myConditions);
    //     } catch (ProcessError& e) {
    //         throw ProcessError(TLF("Error when evaluating function '%' with args '%' (%)", fun, joinToString(args2, ","), e.what()));
    //     }
    //     double result = myStack.back()["$0"];
    //     myStack.pop_back();
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn eval_custom_function(&self, fun: &str, arg: &str) -> f64 {
        let args: Vec<String> = arg.split(',').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
        let f = self.myFunctions.get(fun).unwrap_or_else(|| panic!("{}", ProcessError::new(format!("Unsupported function '{}'", fun))));
        if args.len() as i32 != f.nArgs {
            panic!("{}", ProcessError::new(format!("Function '{}' requires {} arguments but {} were given", fun, f.nArgs, args.len())));
        }
        let mut args2: Vec<f64> = Vec::new();
        for a in args.iter() {
            args2.push(self.eval_expression(a));
        }
        // new stack frame, initialised as a copy of the current top frame
        {
            let mut stack = self.myStack.borrow_mut();
            let top: BTreeMap<String, f64> = stack.last().cloned().unwrap_or_default();
            stack.push(top);
            let frame = stack.last_mut().unwrap();
            frame.insert("$0".to_string(), 0.0);
            for (i, v) in args2.iter().enumerate() {
                frame.insert(format!("${}", i + 1), *v);
            }
        }
        // shortcut: C++ wraps errors of the function body with the function name and arguments; errors here panic instead of throwing, so that wrapping is not reproduced
        let mut empty: MSActuatedTrafficLightLogic_ConditionMap = BTreeMap::new();
        self.execute_assignments(&f.assignments, &mut empty, &self.myConditions);
        let result: f64 = {
            let stack = self.myStack.borrow();
            stack.last().and_then(|m| m.get("$0")).copied().unwrap_or(0.0)
        };
        self.myStack.borrow_mut().pop();
        result
    }

    /// C++: `void MSActuatedTrafficLightLogic::executeAssignments(const AssignmentMap &, ConditionMap &, const ConditionMap &) const` [const, protected]
    /// execute assignemnts of the logic or a custom function
    // void
    // MSActuatedTrafficLightLogic::executeAssignments(const AssignmentMap& assignments, ConditionMap& conditions, const ConditionMap& forbidden) const {
    //     for (const auto& assignment : assignments) {
    //         if (evalExpression(std::get<1>(assignment))) {
    //             const std::string& id = std::get<0>(assignment);
    //             const double val = evalExpression(std::get<2>(assignment));
    //             ConditionMap::iterator it = conditions.find(id);
    //             if (it != conditions.end()) {
    //                 it->second = toString(val);
    //             } else if (forbidden.find(id) != forbidden.end()) {
    //                 throw ProcessError(TLF("Modifying global condition '%' is forbidden", id));
    //             } else {
    //                 myStack.back()[id] = val;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn execute_assignments(&self, assignments: &MSActuatedTrafficLightLogic_AssignmentMap, conditions: &mut MSActuatedTrafficLightLogic_ConditionMap, forbidden: &MSActuatedTrafficLightLogic_ConditionMap) {
        for assignment in assignments.iter() {
            if self.eval_expression(&assignment.1) != 0.0 {
                let id: String = assignment.0.clone();
                let val: f64 = self.eval_expression(&assignment.2);
                if let Some(c) = conditions.get_mut(&id) {
                    *c = utils::common::to_string::to_string(&val);
                } else if forbidden.contains_key(&id) {
                    panic!("{}", ProcessError::new(format!("Modifying global condition '{}' is forbidden", id)));
                } else {
                    self.myStack.borrow_mut().last_mut().unwrap().insert(id, val);
                }
            }
        }
    }

    /// C++: `int MSActuatedTrafficLightLogic::getDetectorPriority(const InductLoopInfo &) const` [const, protected]
    // int
    // MSActuatedTrafficLightLogic::getDetectorPriority(const InductLoopInfo& loopInfo) const {
    //     MSInductLoop* loop = loopInfo.loop;
    //     const double actualGap = loop->getTimeSinceLastDetection();
    //     if ((actualGap < loopInfo.maxGap && !loopInfo.isJammed())
    //             || loopInfo.lastGreenTime < loop->getLastDetectionTime()) {
    //         SUMOTime inactiveTime = MSNet::getInstance()->getCurrentTimeStep() - loopInfo.lastGreenTime;
    //         // @note. Inactive time could also be tracked regardless of current activity (to increase robustness in case of detection failure
    //         if (inactiveTime > myInactiveThreshold) {
    // #ifdef DEBUG_PHASE_SELECTION
    //             if (DEBUG_COND) {
    //                 std::cout << "    loop=" << loop->getID() << " gap=" << loop->getTimeSinceLastDetection() << " lastGreen=" << STEPS2TIME(loopInfo.lastGreenTime)
    //                           << " lastDetection=" << STEPS2TIME(loop->getLastDetectionTime()) << " inactive=" << STEPS2TIME(inactiveTime) << "\n";
    //             }
    // #endif
    //             return (int)STEPS2TIME(inactiveTime);
    //         } else {
    //             // give bonus to detectors that are currently served (if that phase can stil be extended)
    //             if (loopInfo.servedPhase[myStep]) {
    //                 SUMOTime actDuration = MSNet::getInstance()->getCurrentTimeStep() - myPhases[myStep]->myLastSwitch;
    //                 const bool canExtend = actDuration < getCurrentPhaseDef().maxDuration && getLatest() > 0;
    // #ifdef DEBUG_PHASE_SELECTION
    //                 if (DEBUG_COND) {
    //                     std::cout << "    loop=" << loop->getID()
    //                               << " actDuration=" << STEPS2TIME(actDuration)
    //                               << " maxDur=" << STEPS2TIME(getCurrentPhaseDef().maxDuration)
    //                               << " getLatest=" << STEPS2TIME(getLatest())
    //                               << " canExtend=" << canExtend
    //                               << "\n";
    //                 }
    // #endif
    //                 if (canExtend) {
    //                     return DEFAULT_CURRENT_PRIORITY;
    //                 } else {
    //                     return 0;
    //                 }
    //             }
    //             return 1;
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_detector_priority(&self, loopInfo: &MSActuatedTrafficLightLogic_InductLoopInfo) -> i32 {
        let loop_: Ptr<MSInductLoop> = loopInfo.loop_;
        let actualGap: f64 = loop_.get_time_since_last_detection();
        if (actualGap < loopInfo.maxGap && !loopInfo.is_jammed()) || loopInfo.lastGreenTime < loop_.get_last_detection_time() {
            let inactiveTime: SUMOTime = MSNet::get_instance().get_current_time_step() - loopInfo.lastGreenTime;
            // @note. Inactive time could also be tracked regardless of current activity (to increase robustness in case of detection failure
            if inactiveTime > self.myInactiveThreshold {
                return steps2time(inactiveTime) as i32;
            } else {
                // give bonus to detectors that are currently served (if that phase can stil be extended)
                if loopInfo.servedPhase[self.myStep as usize] {
                    let actDuration: SUMOTime = MSNet::get_instance().get_current_time_step() - self.myPhases[self.myStep as usize].myLastSwitch;
                    let canExtend = actDuration < self.get_current_phase_def().maxDuration && self.get_latest() > 0;
                    if canExtend {
                        return DEFAULT_CURRENT_PRIORITY;
                    } else {
                        return 0;
                    }
                }
                return 1;
            }
        }
        0
    }

    /// C++: `int MSActuatedTrafficLightLogic::getPhasePriority(int) const` [const, protected]
    /// count the number of active detectors for the given step
    // int
    // MSActuatedTrafficLightLogic::getPhasePriority(int step) const {
    //     int result = 0;
    //     for (const InductLoopInfo* loopInfo : myInductLoopsForPhase[step]) {
    //         result += getDetectorPriority(*loopInfo);
    //     }
    //     if (myCrossingsForPhase.size() > 0) {
    //         for (const MSLink* crossingEntry : myCrossingsForPhase[step]) {
    //             auto* aPersons = crossingEntry->getApproachingPersons();
    //             if (aPersons != nullptr && aPersons->size() > 0) {
    //                 result += DEFAULT_CROSSING_PRIORITY;
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_priority(&self, step: i32) -> i32 {
        let mut result: i32 = 0;
        for loopInfo in self.myInductLoopsForPhase[step as usize].clone() {
            result += self.get_detector_priority(&*loopInfo);
        }
        if self.myCrossingsForPhase.len() > 0 {
            for crossingEntry in self.myCrossingsForPhase[step as usize].clone() {
                let aPersons = crossingEntry.get_approaching_persons();
                if !aPersons.is_null() && aPersons.len() > 0 {
                    result += DEFAULT_CROSSING_PRIORITY;
                }
            }
        }
        result
    }

    /// C++: `std::pair<int, SUMOTime> MSActuatedTrafficLightLogic::getTarget(int) const` [const, protected]
    /// get the green phase following step and the transition time
    // std::pair<int, SUMOTime>
    // MSActuatedTrafficLightLogic::getTarget(int step) const {
    //     int seen = 0;
    //     int origStep = step;
    //     SUMOTime dur = 0;
    //
    //     // if step is a transition, find the upcoming green phase
    //     while (!myPhases[step]->isGreenPhase()) {
    //         seen += 1;
    //         dur += myPhases[step]->duration;
    //         if (myPhases[step]->nextPhases.size() > 0 && myPhases[step]->nextPhases.front() >= 0) {
    //             for (int next : myPhases[step]->nextPhases) {
    //                 if (next != step) {
    //                     step = next;
    //                     break;
    //                 }
    //             }
    //         } else {
    //             step = (step + 1) % (int)myPhases.size();
    //         }
    //         if (step == origStep || seen > (int)myPhases.size()) {
    //             WRITE_WARNING("At actuated tlLogic '" + getID() + "', infinite transition loop from phase " + toString(origStep));
    //             return std::make_pair(0, 0);
    //         }
    //     }
    //     return std::make_pair(step, dur);
    // }
    #[allow(unused_variables)]
    pub fn get_target(&self, step: i32) -> (i32, SUMOTime) {
        let mut seen: i32 = 0;
        let origStep: i32 = step;
        let mut step: i32 = step;
        let mut dur: SUMOTime = 0;
        // if step is a transition, find the upcoming green phase
        while !self.myPhases[step as usize].is_green_phase() {
            seen += 1;
            dur += self.myPhases[step as usize].duration;
            if self.myPhases[step as usize].nextPhases.len() > 0 && self.myPhases[step as usize].nextPhases[0] >= 0 {
                for next in self.myPhases[step as usize].nextPhases.clone() {
                    if next != step {
                        step = next;
                        break;
                    }
                }
            } else {
                step = (step + 1) % (self.myPhases.len() as i32);
            }
            if step == origStep || seen > self.myPhases.len() as i32 {
                utils::common::msg_handler::write_warning(&format!("At actuated tlLogic '{}', infinite transition loop from phase {}", self.get_id(), origStep));
                return (0, 0);
            }
        }
        (step, dur)
    }

    /// C++: `bool MSActuatedTrafficLightLogic::maxLinkDurationReached()` [protected]
    /// whether the current phase cannot be continued due to linkMaxDur constraints
    // bool
    // MSActuatedTrafficLightLogic::maxLinkDurationReached() {
    //     if (myLinkMaxGreenTimes.empty()) {
    //         return false;
    //     }
    //     for (int i = 0; i < myNumLinks; i++) {
    //         if (myLinkGreenTimes[i] >= myLinkMaxGreenTimes[i]) {
    //             //std::cout << SIMTIME << " maxLinkDurationReached i=" << i << "\n";
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn max_link_duration_reached(&mut self) -> bool {
        if self.myLinkMaxGreenTimes.is_empty() {
            return false;
        }
        for i in 0..self.myNumLinks as usize {
            if self.myLinkGreenTimes[i] >= self.myLinkMaxGreenTimes[i] {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSActuatedTrafficLightLogic::canExtendLinkGreen(int)` [protected]
    /// whether the target phase is acceptable in light of linkMaxDur constraints
    // bool
    // MSActuatedTrafficLightLogic::canExtendLinkGreen(int target) {
    //     if (myLinkMaxGreenTimes.empty()) {
    //         return true;
    //     }
    //     const std::string& targetState = myPhases[target]->getState();
    //     for (int i = 0; i < myNumLinks; i++) {
    //         if (myLinkGreenTimes[i] >= myLinkMaxGreenTimes[i] && (
    //                     targetState[i] == 'G' || targetState[i] == 'g')) {
    //             //std::cout << SIMTIME << " cannotExtendLinkGreen target=" << target << " i=" << i << "\n";
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn can_extend_link_green(&mut self, target: i32) -> bool {
        if self.myLinkMaxGreenTimes.is_empty() {
            return true;
        }
        let targetState: String = self.myPhases[target as usize].get_state().to_string();
        for i in 0..self.myNumLinks as usize {
            let c = targetState.as_bytes()[i] as char;
            if self.myLinkGreenTimes[i] >= self.myLinkMaxGreenTimes[i] && (c == 'G' || c == 'g') {
                return false;
            }
        }
        true
    }

    /// C++: `SUMOTime MSActuatedTrafficLightLogic::getLinkMinDuration(int) const` [const, protected]
    /// the minimum duratin for keeping the current phase due to linkMinDur constraints
    // SUMOTime
    // MSActuatedTrafficLightLogic::getLinkMinDuration(int target) const {
    //     SUMOTime result = 0;
    //     if (target != myStep && myLinkMinGreenTimes.size() > 0) {
    //         const std::string& state = myPhases[myStep]->getState();
    //         const std::string& targetState = myPhases[target]->getState();
    //         for (int i = 0; i < myNumLinks; i++) {
    //             if (myLinkGreenTimes[i] < myLinkMinGreenTimes[i]
    //                     && (state[i] == 'G' || state[i] == 'g')
    //                     && !(targetState[i] == 'G' || targetState[i] == 'g')) {
    //                 result = MAX2(result, myLinkMinGreenTimes[i] - myLinkGreenTimes[i]);
    //                 //std::cout << SIMTIME << " getLinkMinDuration myStep=" << myStep << " target=" << target << " i=" << i
    //                 //    << " greenTime=" << STEPS2TIME(myLinkGreenTimes[i]) << " min=" << STEPS2TIME(myLinkMinGreenTimes[i]) << " result=" << STEPS2TIME(result) << "\n";
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_link_min_duration(&self, target: i32) -> SUMOTime {
        let mut result: SUMOTime = 0;
        if target != self.myStep && self.myLinkMinGreenTimes.len() > 0 {
            let state: String = self.myPhases[self.myStep as usize].get_state().to_string();
            let targetState: String = self.myPhases[target as usize].get_state().to_string();
            for i in 0..self.myNumLinks as usize {
                let c = state.as_bytes()[i] as char;
                let tc = targetState.as_bytes()[i] as char;
                if self.myLinkGreenTimes[i] < self.myLinkMinGreenTimes[i]
                    && (c == 'G' || c == 'g')
                    && !(tc == 'G' || tc == 'g') {
                    result = std::cmp::max(result, self.myLinkMinGreenTimes[i] - self.myLinkGreenTimes[i]);
                }
            }
        }
        result
    }

    /// C++: `bool MSActuatedTrafficLightLogic::weakConflict(int, const std::string &) const` [const, protected]
    /// whether a given link has only weak mode foes that are green in the given state
    // bool
    // MSActuatedTrafficLightLogic::weakConflict(int tlIndex, const std::string& state) const {
    //     for (MSLink* link : getLinksAt(tlIndex)) {
    //         int linkIndex = link->getIndex();
    //         const MSJunction* junction = link->getJunction();
    //         for (int i = 0; i < (int)myLinks.size(); i++) {
    //             if (i == tlIndex) {
    //                 continue;
    //             }
    //             if (state[i] == LINKSTATE_TL_GREEN_MAJOR || state[i] == LINKSTATE_TL_GREEN_MINOR) {
    //                 for (MSLink* foe : getLinksAt(i)) {
    //                     // junction logic is based on junction link index rather than tl index
    //                     int foeIndex = foe->getIndex();
    //                     const MSJunction* junction2 = foe->getJunction();
    //                     if (junction == junction2) {
    //                         const MSJunctionLogic* logic = junction->getLogic();
    //                         //std::cout << " greenLink=" << i << " isFoe=" << logic->getFoesFor(linkIndex).test(foeIndex) << "\n";
    //                         if (logic->getFoesFor(linkIndex).test(foeIndex)
    //                                 && (foe->getPermissions() & ~SVC_VULNERABLE) != 0 // check1e
    //                                 && &foe->getLaneBefore()->getEdge() != &link->getLaneBefore()->getEdge()) { // check1f
    //                             //std::cout << " strongConflict " << tlIndex << " in phase " << state << " with link " << foe->getTLIndex() << "\n";
    //                             return false;
    //                         }
    //                     }
    //                 }
    //
    //             }
    //         }
    //     }
    //     //std::cout << " weakConflict " << tlIndex << " in phase " << state << "\n";
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn weak_conflict(&self, tlIndex: i32, state: &str) -> bool {
        for link in self.get_links_at(tlIndex).clone() {
            let linkIndex: i32 = link.get_index();
            let junction: Ptr<MSJunction> = link.get_junction();
            for i in 0..self.myLinks.len() {
                if i as i32 == tlIndex {
                    continue;
                }
                let s: LinkState = (state.as_bytes()[i] as i8) as LinkState;
                if s == LINKSTATE_TL_GREEN_MAJOR || s == LINKSTATE_TL_GREEN_MINOR {
                    for foe in self.get_links_at(i as i32).clone() {
                        // junction logic is based on junction link index rather than tl index
                        let foeIndex: i32 = foe.get_index();
                        let junction2: Ptr<MSJunction> = foe.get_junction();
                        if junction == junction2 {
                            if junction.get_logic().get_foes_for(link.get_index()).test(foeIndex)
                                && (foe.get_permissions() & !SVC_VULNERABLE) != 0 // check1e
                                && Ptr::from_mut(foe.get_lane_before().get_edge()) != Ptr::from_mut(link.get_lane_before().get_edge()) { // check1f
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }

    /// C++: `const T * MSActuatedTrafficLightLogic::retrieveDetExpression(const std::string &, const std::string &, bool) const` [const, protected]
    // template<typename T, SumoXMLTag Tag>
    //     const T* retrieveDetExpression(const std::string& arg, const std::string& expr, bool tryPrefix) const {
    //         const T* det = dynamic_cast<const T*>(
    //                            MSNet::getInstance()->getDetectorControl().getTypedDetectors(Tag).get(
    //                                (tryPrefix ? myDetectorPrefix : "") + arg));
    //         if (det == nullptr) {
    //             if (tryPrefix) {
    //                 // try again without prefix
    //                 return retrieveDetExpression<T, Tag>(arg, expr, false);
    //             } else {
    //                 throw ProcessError("Unknown detector '" + arg + "' in expression '" + expr + "'");
    //             }
    //         } else {
    //             return det;
    //         }
    //     }
    /// Returns None where the C++ template throws ProcessError (not found); the tag container only holds MSInductLoop, so the cast is exact.
    pub fn retrieve_det_expression_loop(&self, arg: &str, tryPrefix: bool) -> Option<Ptr<MSInductLoop>> {
        let key = if tryPrefix { format!("{}{}", self.myDetectorPrefix, arg) } else { arg.to_string() };
        let det = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_INDUCTION_LOOP).get(&key).map(|d| crate::microsim::output::ms_induct_loop::induct_loop_of(*d)).unwrap_or(Ptr::null());
        if !det.is_null() {
            Some(det)
        } else if tryPrefix {
            // try again without prefix
            self.retrieve_det_expression_loop(arg, false)
        } else {
            None
        }
    }

    /// C++: `retrieveDetExpression<MSE2Collector, SUMO_TAG_LANE_AREA_DETECTOR>`, see retrieve_det_expression_loop
    pub fn retrieve_det_expression_e2(&self, arg: &str, tryPrefix: bool) -> Option<Ptr<MSE2Collector>> {
        let key = if tryPrefix { format!("{}{}", self.myDetectorPrefix, arg) } else { arg.to_string() };
        let det = MSNet::get_instance().m().get_detector_control().get_typed_detectors(SUMO_TAG_LANE_AREA_DETECTOR).get(&key).map(|d| crate::microsim::output::mse2_collector::mse2_collector_of(*d)).unwrap_or(Ptr::null());
        if !det.is_null() {
            Some(det)
        } else if tryPrefix {
            self.retrieve_det_expression_e2(arg, false)
        } else {
            None
        }
    }

    /// C++: `retrieveDetExpression<MSInductLoop, SUMO_TAG_INDUCTION_LOOP>(arg, expr, true)` (throws when unknown)
    pub fn retrieve_det_expression_loop_or_throw(&self, arg: &str, expr: &str) -> Ptr<MSInductLoop> {
        self.retrieve_det_expression_loop(arg, true).unwrap_or_else(|| panic!("{}", ProcessError::new(format!("Unknown detector '{}' in expression '{}'", arg, expr))))
    }

    /// C++: `retrieveDetExpression<MSE2Collector, SUMO_TAG_LANE_AREA_DETECTOR>(arg, expr, true)` (throws when unknown)
    pub fn retrieve_det_expression_e2_or_throw(&self, arg: &str, expr: &str) -> Ptr<MSE2Collector> {
        self.retrieve_det_expression_e2(arg, true).unwrap_or_else(|| panic!("{}", ProcessError::new(format!("Unknown detector '{}' in expression '{}'", arg, expr))))
    }

    /// C++: `std::set<int> MSActuatedTrafficLightLogic::getMultiNextTargets() const` [const, protected]
    /// find green phases target by a next attribute
    // std::set<int>
    // MSActuatedTrafficLightLogic::getMultiNextTargets() const {
    //     std::set<int> result;
    //     if (myHasMultiTarget) {
    //         // find all phase that are the target green phase of a 'next' attribute
    //         for (const MSPhaseDefinition* p : myPhases) {
    //             for (int next : p->nextPhases) {
    //                 result.insert(getTarget(next).first);
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_multi_next_targets(&self) -> BTreeSet<i32> {
        let mut result: BTreeSet<i32> = BTreeSet::new();
        if self.myHasMultiTarget {
            // find all phase that are the target green phase of a 'next' attribute
            for p in self.myPhases.iter() {
                for next in p.nextPhases.clone() {
                    result.insert(self.get_target(next).0);
                }
            }
        }
        result
    }

    /// C++: `void MSActuatedTrafficLightLogic::initTargets(int)` [protected]
    // void
    // MSActuatedTrafficLightLogic::initTargets(int step) {
    //     // next -> target -> transitionTime starting from step
    //     std::map<int, std::map<int, SUMOTime> > reached;
    //     const std::vector<int>& next = myPhases[step]->nextPhases;
    //     for (int n : next) {
    //         findTargets(step, n, 0, reached[n]);
    //     }
    //     for (int target = 0; target < (int)myPhases.size(); target++) {
    //         int bestNext = next[0];
    //         SUMOTime bestTime = SUMOTime_MAX;
    //         for (auto item : reached) {
    //             auto it = item.second.find(target);
    //             if (it != item.second.end()) {
    //                 SUMOTime transitionTime = it->second;
    //                 if (transitionTime < bestTime) {
    //                     bestTime = transitionTime;
    //                     bestNext = item.first;
    //                 }
    //             }
    //         }
    //         if (bestTime != SUMOTime_MAX) {
    //             myTargets[step][bestNext].push_back(target);
    //             //std::cout << " myTargets step=" << step << " bestNext=" << bestNext << " target=" << target << "\n";
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_targets(&mut self, step: i32) {
        // next -> target -> transitionTime starting from step
        let mut reached: BTreeMap<i32, BTreeMap<i32, SUMOTime>> = BTreeMap::new();
        let next: Vec<i32> = self.myPhases[step as usize].nextPhases.clone();
        for n in next.iter().copied() {
            self.find_targets(step, n, 0, reached.entry(n).or_default());
        }
        for target in 0..self.myPhases.len() as i32 {
            let mut bestNext: i32 = next[0];
            let mut bestTime: SUMOTime = SUMOTIME_MAX;
            for (itemFirst, itemSecond) in reached.iter() {
                if let Some(transitionTime) = itemSecond.get(&target) {
                    if *transitionTime < bestTime {
                        bestTime = *transitionTime;
                        bestNext = *itemFirst;
                    }
                }
            }
            if bestTime != SUMOTIME_MAX {
                self.myTargets.entry(step).or_default().entry(bestNext).or_default().push(target);
            }
        }
    }

    /// C++: `void MSActuatedTrafficLightLogic::findTargets(int, int, SUMOTime, std::map<int, SUMOTime> &)` [protected]
    // void
    // MSActuatedTrafficLightLogic::findTargets(int origStep, int n, SUMOTime priorTransition, std::map<int, SUMOTime>& found) {
    //     std::pair<int, SUMOTime> tDur = getTarget(n);
    //     int target = tDur.first;
    //     SUMOTime transitionTime = tDur.second + priorTransition;
    //     //std::cout << "   findTargets origStep=" << origStep << " n=" << n << " ptt=" << priorTransition << " target=" << target << " tt=" << transitionTime << "\n";
    //     if (target == origStep) {
    //         // full circle
    //         //std::cout << "     foundCircle\n";
    //         return;
    //     }
    //     auto it = found.find(target);
    //     if (it != found.end()) {
    //         if (it->second <= transitionTime) {
    //             //std::cout << "     oldShorterTime=" << it->second << "\n";
    //             // found the same target again
    //             return;
    //         } else {
    //             //std::cout << "     newShorterTime=" << it->second << "\n";
    //         }
    //     } else {
    //         //std::cout << "     newTarget\n";
    //     }
    //     found[target] = transitionTime;
    //     //std::cout << "    targetNext=" << toString(myPhases[target]->nextPhases) << "\n";
    //     for (int n2 : myPhases[target]->nextPhases) {
    //         findTargets(origStep, n2, transitionTime, found);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_targets(&mut self, origStep: i32, n: i32, priorTransition: SUMOTime, found: &mut BTreeMap<i32, SUMOTime>) {
        let (target, dur) = self.get_target(n);
        let transitionTime: SUMOTime = dur + priorTransition;
        if target == origStep {
            // full circle
            return;
        }
        if let Some(existing) = found.get(&target) {
            if *existing <= transitionTime {
                // found the same target again
                return;
            }
        }
        found.insert(target, transitionTime);
        for n2 in self.myPhases[target as usize].nextPhases.clone() {
            self.find_targets(origStep, n2, transitionTime, found);
        }
    }

}

/// C++ class `MSActuatedTrafficLightLogic::SwitchingRules` (microsim/traffic_lights/MSActuatedTrafficLightLogic.h)
pub struct MSActuatedTrafficLightLogic_SwitchingRules {
    pub enabled: bool,
}

impl MSActuatedTrafficLightLogic_SwitchingRules {
}

