//! Port of microsim/traffic_lights/MSSOTLTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{with_instance, write_error, write_message};
use utils::common::rand_helper;
use super::ms_simple_traffic_light_logic::MS_SIMPLE_TRAFFIC_LIGHT_LOGIC_VT;

/// C++ typedef `std::map<const std::string, std::vector<MSPushButton *>> MSSOTLTrafficLightLogic::PhasePushButtons`
pub type MSSOTLTrafficLightLogic_PhasePushButtons = BTreeMap<String, Vec<Ptr<MSPushButton>>>;

fn sotl_try_switch(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSSOTLTrafficLightLogic>().m().try_switch()
}
fn sotl_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSSOTLTrafficLightLogic>().m().init(nb)
}
fn sotl_decide_next_phase(l: &mut MSTrafficLightLogic) -> i32 {
    Ptr::from_mut(l).cast::<MSSOTLTrafficLightLogic>().m().decide_next_phase()
}
fn sotl_can_release(l: &mut MSTrafficLightLogic) -> bool {
    Ptr::from_mut(l).cast::<MSSOTLTrafficLightLogic>().m().can_release()
}
fn sotl_compute_return_time(l: &mut MSTrafficLightLogic) -> SUMOTime {
    Ptr::from_mut(l).cast::<MSSOTLTrafficLightLogic>().m().compute_return_time()
}

/// Virtual dispatch for the SOTL family; subclasses override through their own tables built with `..MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT`.
pub(crate) static MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    try_switch: sotl_try_switch,
    init: sotl_init,
    decide_next_phase: sotl_decide_next_phase,
    can_release: sotl_can_release,
    compute_return_time: sotl_compute_return_time,
    ..MS_SIMPLE_TRAFFIC_LIGHT_LOGIC_VT
};

/// A self-organizing traffic light logic
/// C++ class `MSSOTLTrafficLightLogic` (microsim/traffic_lights/MSSOTLTrafficLightLogic.h)
#[repr(C)]
pub struct MSSOTLTrafficLightLogic {
    /// C++ base class `MSSimpleTrafficLightLogic`
    pub base: MSSimpleTrafficLightLogic,
    pub m_pushButtons: MSSOTLTrafficLightLogic_PhasePushButtons,
    pub mySensors: Ptr<MSSOTLSensors>,
    pub myCountSensors: Ptr<MSSOTLE2Sensors>,
    pub sensorsSelfBuilt: bool,
    pub targetPhasesCTS: BTreeMap<i32, SUMOTime>,
    pub lastCheckForTargetPhase: BTreeMap<i32, SUMOTime>,
    pub targetPhasesLastSelection: BTreeMap<i32, i32>,
    pub lastChain: i32,
    pub decayThreshold: f64,
}

impl std::ops::Deref for MSSOTLTrafficLightLogic {
    type Target = MSSimpleTrafficLightLogic;
    fn deref(&self) -> &MSSimpleTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSimpleTrafficLightLogic {
        &mut self.base
    }
}

impl MSSOTLTrafficLightLogic {
    /// C++: `void MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol,
    //     const std::string& id,
    //     const std::string& programID,
    //     const TrafficLightType logicType,
    //     const Phases& phases,
    //     int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters) :
    //     MSSimpleTrafficLightLogic(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters) {
    //     this->mySensors = nullptr;
    //     this->myCountSensors = nullptr;
    //     sensorsSelfBuilt = true;
    //     checkPhases();
    //     setupCTS();
    //     setToATargetPhase();
    // }
    #[allow(unused_variables)]
    /// Shared C++ member initialisation for both constructors (member order follows the header).
    fn with_base(mut base: MSSimpleTrafficLightLogic, mySensors: Ptr<MSSOTLSensors>, sensorsSelfBuilt: bool) -> Self {
        base.base.vt = &MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT;
        Self {
            base,
            m_pushButtons: BTreeMap::new(),
            mySensors,
            myCountSensors: Ptr::null(),
            sensorsSelfBuilt,
            targetPhasesCTS: BTreeMap::new(),
            lastCheckForTargetPhase: BTreeMap::new(),
            targetPhasesLastSelection: BTreeMap::new(),
            lastChain: 0, // shortcut: C++ leaves lastChain uninitialised, Rust starts it at 0
            decayThreshold: 0.0, // shortcut: C++ leaves decayThreshold uninitialised, Rust starts it at 0
        }
    }

    /// C++ `(MSSOTLE2Sensors*)mySensors`: mySensors is always built as an E2 sensor here (SENSORS_TYPE is E2).
    fn e2_sensors(&self) -> Ptr<MSSOTLE2Sensors> {
        Ptr::from_raw(self.mySensors.as_raw() as *mut MSSOTLE2Sensors)
    }

    /// C++ `trySwitch()` and `decideNextPhase()` call the virtual `decideNextPhase()` / `canRelease()` / `computeReturnTime()` through `this`
    fn dispatch_decide_next_phase(&mut self) -> i32 {
        let vt = self.base.base.vt;
        (vt.decide_next_phase)(&mut self.base.base)
    }
    fn dispatch_can_release(&mut self) -> bool {
        let vt = self.base.base.vt;
        (vt.can_release)(&mut self.base.base)
    }
    fn dispatch_compute_return_time(&mut self) -> SUMOTime {
        let vt = self.base.base.vt;
        (vt.compute_return_time)(&mut self.base.base)
    }

    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let base = MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters);
        let mut this = Self::with_base(base, Ptr::null(), true);
        this.check_phases();
        this.setup_cts();
        this.set_to_a_target_phase();
        this
    }

    /// C++: `void MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLTrafficLightLogic::MSSOTLTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol,
    //     const std::string& id,
    //     const std::string& programID,
    //     const TrafficLightType logicType,
    //     const Phases& phases,
    //     int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters,
    //     MSSOTLSensors* sensors) :
    //     MSSimpleTrafficLightLogic(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters) {
    //     this->mySensors = sensors;
    //     sensorsSelfBuilt = false;
    //     checkPhases();
    //     setupCTS();
    //     setToATargetPhase();
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, sensors: Ptr<MSSOTLSensors>) -> Self {
        let base = MSSimpleTrafficLightLogic::new(tlcontrol, id, programID, 0, logicType, phases, step, delay, parameters);
        let mut this = Self::with_base(base, sensors, false);
        this.check_phases();
        this.setup_cts();
        this.set_to_a_target_phase();
        this
    }

    /// C++: `void MSSOTLTrafficLightLogic::~MSSOTLTrafficLightLogic() noexcept` [virtual]
    /// Destructor
    // MSSOTLTrafficLightLogic::~MSSOTLTrafficLightLogic() {
    //     for (PhasePushButtons::iterator mapIt = m_pushButtons.begin(); mapIt != m_pushButtons.end(); ++mapIt)
    //         for (std::vector<MSPushButton*>::iterator vIt = mapIt->second.begin(); vIt != mapIt->second.end(); ++vIt) {
    //             delete *vIt;
    //         }
    //     m_pushButtons.clear();
    //     if (sensorsSelfBuilt) {
    //         delete mySensors;
    // //		delete myCountSensors;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for buttons in self.m_pushButtons.values() {
            for button in buttons {
                unsafe { drop(Box::from_raw(button.as_raw())) };
            }
        }
        self.m_pushButtons.clear();
        if self.sensorsSelfBuilt && !self.mySensors.is_null() {
            unsafe { drop(Box::from_raw(self.e2_sensors().as_raw())) };
        }
        // C++ `delete myCountSensors` is commented out upstream, so it is not freed here.
        self.base.drop_();
    }

    /// C++: `void MSSOTLTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with sensors on incoming and outgoing lanes Sensors are built in the simulation according to the type of sensor specified in the simulation parameter
    // void
    // MSSOTLTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //
    //     MSTrafficLightLogic::init(nb);
    //
    //     if (isDecayThresholdActivated()) {
    //         decayThreshold = 1;
    //     }
    //     if (sensorsSelfBuilt) {
    //         //Building SOTLSensors
    //         switch (SENSORS_TYPE) {
    //             case SENSORS_TYPE_E1:
    //                 throw ProcessError(TL("SOTL can only handle E2 sensors."));
    //             case SENSORS_TYPE_E2:
    //
    //                 //Adding Sensors to the ingoing Lanes
    //
    //                 LaneVectorVector lvv = getLaneVectors();
    //
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("Listing lanes for TLS " + getID());
    //
    //                 for (int i = 0; i < lvv.size(); i++) {
    //                     LaneVector lv = lvv[i];
    //
    //                     for (int j = 0; j < lv.size(); j++) {
    //                         MSLane* lane = lv[j];
    //                         WRITE_MESSAGE(lane ->getID());
    //                     }
    //                 }
    // #endif
    //
    //                 mySensors = new MSSOTLE2Sensors(myID, &(getPhases()));
    //                 ((MSSOTLE2Sensors*)mySensors)->buildSensors(myLanes, nb, getInputSensorsLength());
    //                 mySensors->stepChanged(getCurrentPhaseIndex());
    //                 if (getParameter("USE_VEHICLE_TYPES_WEIGHTS", "0") == "1") {
    //                     ((MSSOTLE2Sensors*) mySensors)->setVehicleWeigths(getParameter("VEHICLE_TYPES_WEIGHTS", ""));
    //                 }
    //
    //                 //threshold speed param for tuning with irace
    //                 ((MSSOTLE2Sensors*)mySensors)->setSpeedThresholdParam(getSpeedThreshold());
    //
    //                 myCountSensors = new MSSOTLE2Sensors(myID + "Count", &(getPhases()));
    //                 myCountSensors->buildCountSensors(myLanes, nb);
    //                 myCountSensors->stepChanged(getCurrentPhaseIndex());
    //
    //                 //Adding Sensors to the outgoing Lanes
    //
    //                 LinkVectorVector links = getLinks();
    //
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE(TL("Listing output lanes"));
    //                 for (int i = 0; i < links.size(); i++) {
    //                     LinkVector oneLink = getLinksAt(i);
    //                     for (int j = 0; j < oneLink.size(); j++) {
    //                         MSLane* lane  = oneLink[j]->getLane();
    //                         WRITE_MESSAGE(lane ->getID());
    //                     }
    //                 }
    // #endif
    //
    //                 LaneVectorVector myLaneVector;
    //                 LaneVector outLanes;
    //                 LinkVectorVector myoutLinks = getLinks();
    //
    //                 for (int i = 0; i < (int)links.size(); i++) {
    //                     LinkVector oneLink = getLinksAt(i);
    //                     for (int j = 0; j < (int)oneLink.size(); j++) {
    //                         MSLane* lane  = oneLink[j]->getLane();
    //                         outLanes.push_back(lane);
    //                     }
    //                 }
    //
    //                 if (outLanes.size() > 0) {
    //                     myLaneVector.push_back(outLanes);
    //                 }
    //                 if (myLaneVector.size() > 0) {
    //                     ((MSSOTLE2Sensors*)mySensors)->buildOutSensors(myLaneVector, nb, getOutputSensorsLength());
    //                     myCountSensors->buildCountOutSensors(myLaneVector, nb);
    //                 }
    //
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        // MSTrafficLightLogic::init(nb);
        self.base.base.init_base(nb);

        if self.is_decay_threshold_activated() {
            self.decayThreshold = 1.0;
        }
        if self.sensorsSelfBuilt {
            // SENSORS_TYPE is SENSORS_TYPE_E2, so the E1 branch (throws ProcessError) is unreachable and not ported.
            // Adding Sensors to the ingoing Lanes
            let sensor_phases = Ptr::new(self.get_phases());
            let e2 = MSSOTLE2Sensors::new(self.get_id().to_string(), sensor_phases);
            self.mySensors = Ptr::from_raw(Box::into_raw(Box::new(e2)) as *mut MSSOTLSensors);
            let myLanes = self.myLanes.clone();
            let inputLength = self.get_input_sensors_length();
            self.e2_sensors().m().build_sensors_2(myLanes, nb, inputLength);
            let idx = self.get_current_phase_index();
            self.mySensors.m().step_changed(idx);
            if self.get_parameter("USE_VEHICLE_TYPES_WEIGHTS", "0".to_string()) == "1" {
                let weights = self.get_parameter("VEHICLE_TYPES_WEIGHTS", "".to_string());
                self.e2_sensors().m().set_vehicle_weigths(&weights);
            }
            // threshold speed param for tuning with irace
            let speedThreshold = self.get_speed_threshold();
            self.e2_sensors().m().set_speed_threshold_param(speedThreshold);

            let count_phases = Ptr::new(self.get_phases());
            let count = MSSOTLE2Sensors::new(format!("{}Count", self.get_id()), count_phases);
            self.myCountSensors = Ptr::from_raw(Box::into_raw(Box::new(count)));
            let myLanes = self.myLanes.clone();
            self.myCountSensors.m().build_count_sensors(myLanes, nb);
            let idx = self.get_current_phase_index();
            self.myCountSensors.m().step_changed(idx);

            // Adding Sensors to the outgoing Lanes
            let mut myLaneVector: MSTrafficLightLogic_LaneVectorVector = Vec::new();
            let mut outLanes: MSTrafficLightLogic_LaneVector = Vec::new();
            for i in 0..self.get_links().len() {
                let oneLink = self.get_links_at(i as i32).clone();
                for j in 0..oneLink.len() {
                    let lane = oneLink[j].get_lane();
                    outLanes.push(lane);
                }
            }
            if !outLanes.is_empty() {
                myLaneVector.push(outLanes);
            }
            if !myLaneVector.is_empty() {
                let outLength = self.get_output_sensors_length();
                self.e2_sensors().m().build_out_sensors_2(myLaneVector.clone(), nb, outLength);
                self.myCountSensors.m().build_count_out_sensors(myLaneVector, nb);
            }
        }
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::trySwitch()` [virtual]
    // SUMOTime
    // MSSOTLTrafficLightLogic::trySwitch() {
    //     if (MSNet::getInstance()->getCurrentTimeStep() % 1000 == 0) {
    //         WRITE_MESSAGE("MSSOTLTrafficLightLogic::trySwitch()");
    //         // To check if decideNextPhase changes the step
    //         int previousStep = getCurrentPhaseIndex() ;
    // #ifdef ANALYSIS_DEBUG
    //         SUMOTime elapsed = getCurrentPhaseElapsed();
    // #endif
    //         // Update CTS according to sensors
    //         updateCTS();
    //
    //         // Invoking the function member, specialized for each SOTL logic
    //         setStep(decideNextPhase());
    //         MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //
    //         //At the end, check if new step started
    //         if (getCurrentPhaseIndex() != previousStep) {
    //             //Check if a new steps chain started
    //             if (currentPhase.isTarget())  {
    //                 //Reset CTS for the ending steps chain
    //                 resetCTS(lastChain);
    //                 //Update lastTargetPhase
    //                 lastChain = getCurrentPhaseIndex();
    //                 for (std::map<int, int>::iterator it = targetPhasesLastSelection.begin(); it != targetPhasesLastSelection.end(); ++ it) {
    //                     if (it->first == lastChain) {
    //                         if (it->second >= getTargetPhaseMaxLastSelection()) {
    //                             std::ostringstream oss;
    //                             oss << "Forced selection of the phase " << lastChain << " since its last selection was " << it->second << " changes ago";
    //                             WRITE_MESSAGE(oss.str())
    //                         }
    //                         it->second = 0;
    //                     } else if (it->first != previousStep) {
    //                         ++it->second;
    //                     }
    //                 }
    //                 if (isDecayThresholdActivated()) {
    //                     decayThreshold = 1;
    //                 }
    //             }
    //             //Inform the sensors logic
    //             mySensors->stepChanged(getCurrentPhaseIndex());
    //             //Store the time the new phase started
    //             currentPhase.myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //             if (isDecayThresholdActivated()) {
    //                 decayThreshold = 1;
    //             }
    // #ifdef ANALYSIS_DEBUG
    //             std::ostringstream oss;
    //             oss << getID() << " from " << getPhase(previousStep).getState() << " to " << currentPhase.getState() << " after " << time2string(elapsed);
    //             WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + "\tMSSOTLTrafficLightLogic::trySwitch " + oss.str());
    // #endif
    //         }
    //     }
    //     return computeReturnTime();
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        if MSNet::get_instance().get_current_time_step() % 1000 == 0 {
            write_message("MSSOTLTrafficLightLogic::trySwitch()");
            // To check if decideNextPhase changes the step
            let previousStep = self.get_current_phase_index();
            // Update CTS according to sensors
            self.update_cts();

            // Invoking the function member, specialized for each SOTL logic
            let next = self.dispatch_decide_next_phase();
            self.set_step(next);
            // C++ copies the current phase here and sets its myLastSwitch on that copy, which has no effect on the real phase, so no write is made.

            //At the end, check if new step started
            if self.get_current_phase_index() != previousStep {
                //Check if a new steps chain started
                if self.get_current_phase_def().is_target() {
                    //Reset CTS for the ending steps chain
                    let oldChain = self.lastChain;
                    self.reset_cts(oldChain);
                    //Update lastTargetPhase
                    self.lastChain = self.get_current_phase_index();
                    let lastChain = self.lastChain;
                    let maxLastSelection = self.get_target_phase_max_last_selection();
                    for (key, val) in self.targetPhasesLastSelection.iter_mut() {
                        if *key == lastChain {
                            if *val >= maxLastSelection {
                                write_message(&format!("Forced selection of the phase {} since its last selection was {} changes ago", lastChain, val));
                            }
                            *val = 0;
                        } else if *key != previousStep {
                            *val += 1;
                        }
                    }
                    if self.is_decay_threshold_activated() {
                        self.decayThreshold = 1.0;
                    }
                }
                //Inform the sensors logic
                let idx = self.get_current_phase_index();
                self.mySensors.m().step_changed(idx);
                if self.is_decay_threshold_activated() {
                    self.decayThreshold = 1.0;
                }
            }
        }
        self.dispatch_compute_return_time()
    }

    /// C++: `void MSSOTLTrafficLightLogic::logStatus()` [protected]
    // void MSSOTLTrafficLightLogic::logStatus() {
    //
    // }
    #[allow(unused_variables)]
    pub fn log_status(&mut self) {
        // C++ body is empty.
    }

    /// C++: `int MSSOTLTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int
    // MSSOTLTrafficLightLogic::decideNextPhase() {
    //     MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //     //If the junction was in a commit step
    //     //=> go to the target step that gives green to the set with the current highest CTS
    //     //   and return computeReturnTime()
    //     if (currentPhase.isCommit()) {
    //         // decide which chain to activate. Gotta work on this
    //         return getPhaseIndexWithMaxCTS();
    //     }
    //     if (currentPhase.isTransient()) {
    //         //If the junction was in a transient step
    //         //=> go to the next step and return computeReturnTime()
    //         return getCurrentPhaseIndex() + 1;
    //     }
    //
    //     if (currentPhase.isDecisional()) {
    //
    //         if (canRelease()) {
    //             return getCurrentPhaseIndex() + 1;
    //         }
    //     }
    //
    //     return getCurrentPhaseIndex();
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        let (commit, transient, decisional) = {
            let currentPhase = self.get_current_phase_def();
            (currentPhase.is_commit(), currentPhase.is_transient(), currentPhase.is_decisional())
        };
        //If the junction was in a commit step
        //=> go to the target step that gives green to the set with the current highest CTS
        if commit {
            return self.get_phase_index_with_max_cts();
        }
        //If the junction was in a transient step => go to the next step
        if transient {
            return self.get_current_phase_index() + 1;
        }
        if decisional {
            if self.dispatch_can_release() {
                return self.get_current_phase_index() + 1;
            }
        }
        self.get_current_phase_index()
    }

    /// C++: `bool MSSOTLTrafficLightLogic::canRelease()` [virtual, pure virtual, protected]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        panic!("MSSOTLTrafficLightLogic::canRelease is pure virtual; subclasses must override it")
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::getCurrentPhaseElapsed()` [protected]
    // SUMOTime
    // MSSOTLTrafficLightLogic::getCurrentPhaseElapsed() {
    //     MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //
    //     SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     SUMOTime elapsed = now - currentPhase.myLastSwitch;
    //
    //     return elapsed;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_elapsed(&mut self) -> SUMOTime {
        let now = MSNet::get_instance().get_current_time_step();
        let elapsed = now - self.get_current_phase_def().myLastSwitch;
        elapsed
    }

    /// C++: `int MSSOTLTrafficLightLogic::countVehicles(MSPhaseDefinition)` [protected]
    // int
    // MSSOTLTrafficLightLogic::countVehicles(MSPhaseDefinition phase) {
    //
    //     if (!phase.isTarget()) {
    //         return 0;
    //     }
    //
    //     int accumulator = 0;
    //     //Iterate over the target lanes for the current target phase to get the number of approaching vehicles
    //     for (const std::string& lane : phase.getTargetLaneSet()) {
    //         //SWITCH between 3 counting vehicles function
    //         switch (getMode()) {
    //             case (0):
    //                 accumulator += mySensors->countVehicles(lane); //SUMO
    //                 break;
    //             case (1):
    //                 accumulator += ((MSSOTLE2Sensors*)mySensors)->estimateVehicles(lane);  //COMPLEX
    //                 break;
    //             case (2):
    //                 accumulator = MAX2((int)((MSSOTLE2Sensors*)mySensors)->getEstimateQueueLength(lane), accumulator);  //QUEUE
    //                 break;
    //             default:
    //                 WRITE_ERROR(TL("Unrecognized traffic threshold calculation mode"));
    //         }
    //     }
    //     return accumulator;
    // }
    #[allow(unused_variables)]
    // Signature change: C++ takes MSPhaseDefinition by value (a copy); MSPhaseDefinition is not Clone, so a pointer is passed.
    pub fn count_vehicles(&mut self, phase: Ptr<MSPhaseDefinition>) -> i32 {
        if !phase.is_target() {
            return 0;
        }

        let mut accumulator: i32 = 0;
        //Iterate over the target lanes for the current target phase to get the number of approaching vehicles
        for lane in phase.get_target_lane_set() {
            //SWITCH between 3 counting vehicles function
            match self.get_mode() {
                0 => accumulator += self.e2_sensors().m().count_vehicles_2(lane.clone()), //SUMO
                1 => accumulator += self.e2_sensors().m().estimate_vehicles(lane.clone()), //COMPLEX
                2 => accumulator = std::cmp::max(self.e2_sensors().m().get_estimate_queue_length(lane.clone()) as i32, accumulator), //QUEUE
                _ => write_error("Unrecognized traffic threshold calculation mode"),
            }
        }
        accumulator
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isThresholdPassed()` [protected]
    // bool
    // MSSOTLTrafficLightLogic::isThresholdPassed() {
    // #ifdef SWARM_DEBUG
    //     //	WRITE_MESSAGEF(TL("\n% tlsid=%  //	WRITE_MESSAGEF(TL("\n% tlsid=" + getID()), ime2string(MSNet::getInstance()->getCurrentTimeStep()) +"\tMSSOTLTrafficLightLogic::isThresholdPassed()::  ", getID()), ime2string(MSNet::getInstance()->getCurrentTimeStep()) +"\tMSSOTLTrafficLightLogic::isThresholdPassed()::  ");
    //
    //     std::ostringstream threshold_str;
    //     //	threshold_str << "tlsid=" << getID() << " targetPhaseCTS size=" << targetPhasesCTS.size();
    // //			threshold_str << "\n";
    //     WRITE_MESSAGE(threshold_str.str());
    // #endif
    //     /*
    //      * if a dynamic threshold based on the exponential decrease, if passed we force the phase change
    //      */
    // //	double random = ((double) RandHelper::rand(RAND_MAX) / (RAND_MAX));
    //     double random = RandHelper::rand();
    // //	ANALYSIS_DBG(
    // #ifdef SWARM_DEBUG
    //     if (isDecayThresholdActivated()) {
    //         std::ostringstream str;
    //         str << time2string(MSNet::getInstance()->getCurrentTimeStep()) << "\tMSSOTLTrafficLightLogic::isThresholdPassed()::  "
    //             << " tlsid=" << getID() << " decayThreshold=" << decayThreshold << " random=" << random << ">" << (1 - decayThreshold)
    //             << (random > (1 - decayThreshold) ? " true" : " false");
    //
    //         WRITE_MESSAGE(str.str());
    //     }
    // #endif
    //     if (!isDecayThresholdActivated() || (isDecayThresholdActivated() && random > (1 - decayThreshold))) {
    //         for (std::map<int, SUMOTime>::const_iterator iterator =
    //                     targetPhasesCTS.begin(); iterator != targetPhasesCTS.end();
    //                 iterator++) {
    // #ifdef SWARM_DEBUG
    //             SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //             std::ostringstream threshold_str;
    //             //	threshold_str <<"\tTL " +getID()<<" time=" +time2string(step)<< "(getThreshold()= " << getThreshold()
    //             //		<< ", targetPhaseCTS= " << iterator->second << " )" << " phase="<<getPhase(iterator->first).getState();
    //             threshold_str << getCurrentPhaseDef().getState() << ";" << time2string(step) << ";" << getThreshold()
    //                           << ";" << iterator->second << ";" << getPhase(iterator->first).getState() << ";"
    //                           << iterator->first << "!=" << lastChain;
    //             WRITE_MESSAGE(threshold_str.str());
    // #endif
    //             //Note that the current chain is not eligible to be directly targeted again, it would be unfair
    //             if ((iterator->first != lastChain) && (getThreshold() <= iterator->second)) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     } else {
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn is_threshold_passed(&mut self) -> bool {
        /*
         * if a dynamic threshold based on the exponential decrease, if passed we force the phase change
         */
        let random = rand_helper::rand();
        if !self.is_decay_threshold_activated() || (self.is_decay_threshold_activated() && random > (1.0 - self.decayThreshold)) {
            // Snapshot the map: getThreshold() needs &mut self inside the loop.
            let entries: Vec<(i32, SUMOTime)> = self.targetPhasesCTS.iter().map(|(&k, &v)| (k, v)).collect();
            for (chain, cts) in entries {
                //Note that the current chain is not eligible to be directly targeted again, it would be unfair
                if chain != self.lastChain && (self.get_threshold() as SUMOTime) <= cts {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isPushButtonPressed()` [protected]
    /// Checks the if the pushbutton was activated for the current stage
    // bool MSSOTLTrafficLightLogic::isPushButtonPressed() {
    //     if (getParameter("USE_PUSH_BUTTON", "0") == "0") {
    //         return false;
    //     }
    //     const MSPhaseDefinition currentPhase = getCurrentPhaseDef();
    //     if (m_pushButtons.find(currentPhase.getState()) == m_pushButtons.end()) {
    //         m_pushButtons[currentPhase.getState()] = MSPedestrianPushButton::loadPushButtons(&currentPhase);
    //     }
    //     return MSPushButton::anyActive(m_pushButtons[currentPhase.getState()]);
    // }
    #[allow(unused_variables)]
    pub fn is_push_button_pressed(&mut self) -> bool {
        if self.get_parameter("USE_PUSH_BUTTON", "0".to_string()) == "0" {
            return false;
        }
        let state = self.get_current_phase_def().get_state().to_string();
        if !self.m_pushButtons.contains_key(&state) {
            let buttons = MSPedestrianPushButton::load_push_buttons(Ptr::new(self.get_current_phase_def()));
            self.m_pushButtons.insert(state.clone(), buttons);
        }
        MSPushButton::any_active(&self.m_pushButtons[&state])
    }

    /// C++: `int MSSOTLTrafficLightLogic::getThreshold()` [protected]
    // int getThreshold() {
    //         return StringUtils::toInt(getParameter("THRESHOLD", "10"));
    //     }
    #[allow(unused_variables)]
    pub fn get_threshold(&mut self) -> i32 {
        utils::common::string_utils::to_int(&self.get_parameter("THRESHOLD", "10".to_string())).unwrap()
    }

    /// C++: `double MSSOTLTrafficLightLogic::getSpeedThreshold()` [protected]
    // double getSpeedThreshold() {
    //         return StringUtils::toDouble(getParameter("THRESHOLDSPEED", "2"));
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_threshold(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("THRESHOLDSPEED", "2".to_string())).unwrap()
    }

    /// C++: `double MSSOTLTrafficLightLogic::getInputSensorsLength()` [protected]
    // double getInputSensorsLength() {
    //         return StringUtils::toDouble(getParameter("INSENSORSLENGTH", "100"));
    //     }
    #[allow(unused_variables)]
    pub fn get_input_sensors_length(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("INSENSORSLENGTH", "100".to_string())).unwrap()
    }

    /// C++: `double MSSOTLTrafficLightLogic::getOutputSensorsLength()` [protected]
    // double getOutputSensorsLength() {
    //         return StringUtils::toDouble(getParameter("OUTSENSORSLENGTH", "80"));
    //     }
    #[allow(unused_variables)]
    pub fn get_output_sensors_length(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("OUTSENSORSLENGTH", "80".to_string())).unwrap()
    }

    /// C++: `int MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS()` [protected]
    // int
    // MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS() {
    //     SUMOTime maxCTS = 0;
    //     int maxLastStep = getTargetPhaseMaxLastSelection();
    //     bool usedMaxCTS = false;
    //     std::vector<int> equalIndexes;
    //     for (std::map<int, int>::const_iterator it = targetPhasesLastSelection.begin();
    //             it != targetPhasesLastSelection.end(); ++it) {
    //         if (it->first != lastChain) {
    //             if (maxLastStep < it->second) {
    //                 maxLastStep = it->second;
    //                 equalIndexes.clear();
    //                 equalIndexes.push_back(it->first);
    //             } else if (maxLastStep == it->second) {
    //                 equalIndexes.push_back(it->first);
    //             }
    //         }
    //     }
    //     if (equalIndexes.size() == 0) {
    //         usedMaxCTS = true;
    //         for (std::map<int, SUMOTime>::const_iterator iterator = targetPhasesCTS.begin();
    //                 iterator != targetPhasesCTS.end(); ++iterator) {
    //             if (iterator->first != lastChain) {
    //                 if (maxCTS < iterator->second) {
    //                     maxCTS = iterator->second;
    //                     equalIndexes.clear();
    //                     equalIndexes.push_back(iterator->first);
    //                 } else if (maxCTS == iterator->second) {
    //                     equalIndexes.push_back(iterator->first);
    //                 }
    //             }
    //         }
    //     }
    //
    //     std::ostringstream oss;
    //     oss << "MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS-> TLC " << getID();
    //     if (usedMaxCTS) {
    //         oss << " maxCTS " << maxCTS;
    //     } else {
    //         oss << " forcing selection since not selected for " << maxLastStep;
    //     }
    //     if (equalIndexes.size() == 1) {
    //         oss << " phase " << equalIndexes[0];
    //         WRITE_MESSAGE(oss.str());
    //         return equalIndexes[0];
    //     } else {
    //         const int index = RandHelper::getRandomFrom(equalIndexes);
    //         oss << " phases [";
    //         for (std::vector<int>::const_iterator it = equalIndexes.begin(); it != equalIndexes.end(); ++it) {
    //             oss << *it << ", ";
    //         }
    //         oss << "]. Random select " << index;
    //         WRITE_MESSAGE(oss.str());
    //         return index;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_with_max_cts(&mut self) -> i32 {
        let mut maxCTS: SUMOTime = 0;
        let mut maxLastStep: i32 = self.get_target_phase_max_last_selection();
        let mut usedMaxCTS = false;
        let mut equalIndexes: Vec<i32> = Vec::new();
        for (&idx, &last) in self.targetPhasesLastSelection.iter() {
            if idx != self.lastChain {
                if maxLastStep < last {
                    maxLastStep = last;
                    equalIndexes.clear();
                    equalIndexes.push(idx);
                } else if maxLastStep == last {
                    equalIndexes.push(idx);
                }
            }
        }
        if equalIndexes.is_empty() {
            usedMaxCTS = true;
            for (&idx, &cts) in self.targetPhasesCTS.iter() {
                if idx != self.lastChain {
                    if maxCTS < cts {
                        maxCTS = cts;
                        equalIndexes.clear();
                        equalIndexes.push(idx);
                    } else if maxCTS == cts {
                        equalIndexes.push(idx);
                    }
                }
            }
        }

        let mut oss = format!("MSSOTLTrafficLightLogic::getPhaseIndexWithMaxCTS-> TLC {}", self.get_id());
        if usedMaxCTS {
            oss.push_str(&format!(" maxCTS {}", maxCTS));
        } else {
            oss.push_str(&format!(" forcing selection since not selected for {}", maxLastStep));
        }
        if equalIndexes.len() == 1 {
            oss.push_str(&format!(" phase {}", equalIndexes[0]));
            write_message(&oss);
            equalIndexes[0]
        } else {
            // determinism: RandHelper::getRandomFrom draws from the shared RNG, same as the C++ call.
            let index = rand_helper::with_default(|r| *r.get_random_from(&equalIndexes));
            oss.push_str(" phases [");
            for it in &equalIndexes {
                oss.push_str(&format!("{}, ", it));
            }
            oss.push_str(&format!("]. Random select {}", index));
            write_message(&oss);
            index
        }
    }

    /// C++: `MSSOTLSensors * MSSOTLTrafficLightLogic::getSensors()` [protected]
    // MSSOTLSensors* getSensors() {
    //         return mySensors;
    //     }
    #[allow(unused_variables)]
    pub fn get_sensors(&mut self) -> Ptr<MSSOTLSensors> {
        self.mySensors
    }

    /// C++: `MSSOTLE2Sensors * MSSOTLTrafficLightLogic::getCountSensors()` [protected]
    /// Return the sensors that count the passage of vehicles in and out of the tl.
    // MSSOTLE2Sensors* getCountSensors() {
    //         return myCountSensors;
    //     }
    #[allow(unused_variables)]
    pub fn get_count_sensors(&mut self) -> Ptr<MSSOTLE2Sensors> {
        self.myCountSensors
    }

    /// C++: `SUMOTime MSSOTLTrafficLightLogic::computeReturnTime()` [virtual, protected]
    // virtual SUMOTime computeReturnTime() {
    //         if (getCurrentPhaseDef().isTransient()) {
    //             return getCurrentPhaseDef().duration;
    //
    //         }
    //         return DELTA_T;
    //
    //     }
    #[allow(unused_variables)]
    pub fn compute_return_time(&mut self) -> SUMOTime {
        let (transient, duration) = {
            let currentPhase = self.get_current_phase_def();
            (currentPhase.is_transient(), currentPhase.duration)
        };
        if transient {
            return duration;
        }
        delta_t()
    }

    /// C++: `void MSSOTLTrafficLightLogic::setStep(int)` [protected]
    /// Forces a specific step
    // void MSSOTLTrafficLightLogic::setStep(int step) {
    //     step = step % myPhases.size();
    //     if (myStep != step) {
    //         myStep = step;
    //         myPhases[myStep]->myLastSwitch = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_step(&mut self, step: i32) {
        // C++ `step % myPhases.size()`: the int is converted to size_t, so a negative step wraps.
        let step = ((step as u64) % (self.myPhases.len() as u64)) as i32;
        if self.myStep != step {
            self.myStep = step;
            let now = MSNet::get_instance().get_current_time_step();
            self.myPhases[self.myStep as usize].m().myLastSwitch = now;
        }
    }

    /// C++: `int MSSOTLTrafficLightLogic::getTargetPhaseMaxLastSelection()` [private]
    // int getTargetPhaseMaxLastSelection() {
    //         //return 2 * targetPhasesCTS.size() - 1;
    //         return (int)targetPhasesCTS.size() - 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_phase_max_last_selection(&mut self) -> i32 {
        //return 2 * targetPhasesCTS.size() - 1;
        self.targetPhasesCTS.len() as i32 - 1
    }

    /// C++: `void MSSOTLTrafficLightLogic::checkPhases()` [private]
    // void
    // MSSOTLTrafficLightLogic::checkPhases() {
    //     for (int step = 0; step < (int)getPhases().size(); step++) {
    //         if (getPhase(step).isUndefined()) {
    //             MsgHandler::getErrorInstance()->inform("Step " + toString(step) + " of traffic light logic " + myID + " phases declaration has its type undeclared!");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_phases(&mut self) {
        for step in 0..self.get_phases().len() as i32 {
            if self.get_phase(step).is_undefined() {
                let msg = format!("Step {} of traffic light logic {} phases declaration has its type undeclared!", step, self.get_id());
                with_instance(MsgType::Error, |m| m.inform(&msg, true));
            }
        }
    }

    /// C++: `void MSSOTLTrafficLightLogic::setToATargetPhase()` [private]
    // void
    // MSSOTLTrafficLightLogic::setToATargetPhase() {
    //     for (int step = 0; step < (int)getPhases().size(); step++) {
    //         if (getPhase(step).isTarget()) {
    //             setStep(step);
    //             lastChain = step;
    //             return;
    //         }
    //     }
    //     MsgHandler::getErrorInstance()->inform("No phase of type target found for traffic light logic " + myID + " The logic could malfunction. Check phases declaration.");
    // }
    #[allow(unused_variables)]
    pub fn set_to_a_target_phase(&mut self) {
        for step in 0..self.get_phases().len() as i32 {
            if self.get_phase(step).is_target() {
                self.set_step(step);
                self.lastChain = step;
                return;
            }
        }
        let msg = format!("No phase of type target found for traffic light logic {} The logic could malfunction. Check phases declaration.", self.get_id());
        with_instance(MsgType::Error, |m| m.inform(&msg, true));
    }

    /// C++: `void MSSOTLTrafficLightLogic::setupCTS()` [private]
    // void
    // MSSOTLTrafficLightLogic::setupCTS() {
    //     for (int phaseStep = 0; phaseStep < (int)getPhases().size(); phaseStep++) {
    //         if (getPhase(phaseStep).isTarget()) {
    //             targetPhasesCTS[phaseStep] = 0;
    //             lastCheckForTargetPhase[phaseStep] = MSNet::getInstance()->getCurrentTimeStep();
    //             targetPhasesLastSelection[phaseStep] = 0;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn setup_cts(&mut self) {
        for phaseStep in 0..self.get_phases().len() as i32 {
            if self.get_phase(phaseStep).is_target() {
                self.targetPhasesCTS.insert(phaseStep, 0);
                let now = MSNet::get_instance().get_current_time_step();
                self.lastCheckForTargetPhase.insert(phaseStep, now);
                self.targetPhasesLastSelection.insert(phaseStep, 0);
            }
        }
    }

    /// C++: `void MSSOTLTrafficLightLogic::updateCTS()` [private]
    // void
    // MSSOTLTrafficLightLogic::updateCTS() {
    //     SUMOTime elapsedTimeSteps = 0;
    //     SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     //Iterate over the target phase map and update CTS value for every target phase except for the one belonging to the current steps chain
    //     for (std::map<int, SUMOTime>::iterator mapIterator = targetPhasesCTS.begin();
    //             mapIterator != targetPhasesCTS.end();
    //             mapIterator++) {
    //         int chain = mapIterator->first;
    //         SUMOTime oldVal = mapIterator->second;
    //         if (chain != lastChain) {
    //             //Get the number of timesteps since the last check for that phase
    //             elapsedTimeSteps = now - lastCheckForTargetPhase[chain];
    //             //Update the last check time
    //             lastCheckForTargetPhase[chain] = now;
    //             //Increment the CTS
    //             //SWITCH between 3 counting vehicles function
    //             switch (getMode()) {
    //                 case (0):
    //                     mapIterator->second += elapsedTimeSteps
    //                                            * countVehicles(getPhase(chain)); //SUMO
    //                     break;
    //                 case (1):
    //                     mapIterator->second += elapsedTimeSteps
    //                                            * countVehicles(getPhase(chain)); //COMPLEX
    //                     break;
    //                 case (2):
    //                     mapIterator->second = countVehicles(getPhase(chain)); //QUEUE
    //                     break;
    //                 default:
    //                     WRITE_ERROR(TL("Unrecognized traffic threshold calculation mode"));
    //             }
    //             std::ostringstream oss;
    //             oss << "MSSOTLTrafficLightLogic::updateCTS->TLC " << getID() << " chain " << chain << " oldVal " << oldVal << " newVal " << mapIterator->second;
    //             WRITE_MESSAGE(oss.str());
    //         }
    //         if (isDecayThresholdActivated()) {
    //             updateDecayThreshold();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_cts(&mut self) {
        let mut elapsedTimeSteps: SUMOTime = 0;
        let now = MSNet::get_instance().get_current_time_step();
        // Iterate over the target phase map and update CTS value for every target phase except for the one belonging to the current steps chain
        // (keys are snapshotted because getMode()/countVehicles() need &mut self inside the loop; only values change)
        let chains: Vec<i32> = self.targetPhasesCTS.keys().copied().collect();
        for chain in chains {
            let oldVal = self.targetPhasesCTS[&chain];
            if chain != self.lastChain {
                //Get the number of timesteps since the last check for that phase
                let lastCheck = self.lastCheckForTargetPhase.get(&chain).copied().unwrap_or(0);
                elapsedTimeSteps = now - lastCheck;
                //Update the last check time
                self.lastCheckForTargetPhase.insert(chain, now);
                //Increment the CTS
                //SWITCH between 3 counting vehicles function
                match self.get_mode() {
                    0 | 1 => {
                        // SUMO (0) and COMPLEX (1) both add elapsed steps times the vehicle count
                        let phase = Ptr::new(self.get_phase(chain));
                        let count = self.count_vehicles(phase);
                        *self.targetPhasesCTS.get_mut(&chain).unwrap() += elapsedTimeSteps * count as SUMOTime;
                    }
                    2 => {
                        // QUEUE
                        let phase = Ptr::new(self.get_phase(chain));
                        let count = self.count_vehicles(phase);
                        *self.targetPhasesCTS.get_mut(&chain).unwrap() = count as SUMOTime;
                    }
                    _ => write_error("Unrecognized traffic threshold calculation mode"),
                }
                let newVal = self.targetPhasesCTS[&chain];
                write_message(&format!("MSSOTLTrafficLightLogic::updateCTS->TLC {} chain {} oldVal {} newVal {}", self.get_id(), chain, oldVal, newVal));
            }
            if self.is_decay_threshold_activated() {
                self.update_decay_threshold();
            }
        }
    }

    /// C++: `void MSSOTLTrafficLightLogic::resetCTS(int)` [private]
    // void
    // MSSOTLTrafficLightLogic::resetCTS(int phaseStep) {
    //     std::map<int, SUMOTime>::iterator phaseIterator = targetPhasesCTS.find(phaseStep);
    //     if (phaseIterator != targetPhasesCTS.end()) {
    //         phaseIterator->second = 0;
    //         lastCheckForTargetPhase[phaseStep] = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_cts(&mut self, phaseStep: i32) {
        if self.targetPhasesCTS.contains_key(&phaseStep) {
            self.targetPhasesCTS.insert(phaseStep, 0);
            let now = MSNet::get_instance().get_current_time_step();
            self.lastCheckForTargetPhase.insert(phaseStep, now);
        }
    }

    /// C++: `void MSSOTLTrafficLightLogic::updateDecayThreshold()` [private]
    // void
    // MSSOTLTrafficLightLogic::updateDecayThreshold() {
    //     if (getCurrentPhaseDef().isGreenPhase()) {
    //         decayThreshold = decayThreshold * exp(getDecayConstant());
    //     }
    // #ifdef SWARM_DEBUG
    //     std::stringstream out;
    //     out << decayThreshold;
    //     WRITE_MESSAGE("\n" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + "\tMSSOTLTrafficLightLogic::updateDecayThreshold()::  " + out.str());
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn update_decay_threshold(&mut self) {
        if self.get_current_phase_def().is_green_phase() {
            let factor = self.get_decay_constant().exp();
            self.decayThreshold = self.decayThreshold * factor;
        }
    }

    /// C++: `int MSSOTLTrafficLightLogic::getMode()` [private]
    // int getMode() {
    //         return StringUtils::toInt(getParameter("MODE", "0"));
    //     }
    #[allow(unused_variables)]
    pub fn get_mode(&mut self) -> i32 {
        utils::common::string_utils::to_int(&self.get_parameter("MODE", "0".to_string())).unwrap()
    }

    /// C++: `bool MSSOTLTrafficLightLogic::isDecayThresholdActivated()` [private]
    // bool isDecayThresholdActivated() {
    //         return StringUtils::toBool(getParameter("DECAY_THRESHOLD", "0"));
    //     }
    #[allow(unused_variables)]
    pub fn is_decay_threshold_activated(&mut self) -> bool {
        utils::common::string_utils::to_bool(&self.get_parameter("DECAY_THRESHOLD", "0".to_string())).unwrap()
    }

    /// C++: `double MSSOTLTrafficLightLogic::getDecayConstant()` [private]
    // double getDecayConstant() {
    //         return StringUtils::toDouble(getParameter("DECAY_CONSTANT", "-0.001"));
    //     }
    #[allow(unused_variables)]
    pub fn get_decay_constant(&mut self) -> f64 {
        utils::common::string_utils::to_double(&self.get_parameter("DECAY_CONSTANT", "-0.001".to_string())).unwrap()
    }

}

