//! Port of microsim/traffic_lights/MSDeterministicHiLevelTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mssotl_traffic_light_logic::MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT;

fn det_hi_init(l: &mut MSTrafficLightLogic, nb: &mut NLDetectorBuilder) {
    Ptr::from_mut(l).cast::<MSDeterministicHiLevelTrafficLightLogic>().m().init(nb)
}
fn det_hi_decide_next_phase(l: &mut MSTrafficLightLogic) -> i32 {
    Ptr::from_mut(l).cast::<MSDeterministicHiLevelTrafficLightLogic>().m().decide_next_phase()
}
fn det_hi_can_release(l: &mut MSTrafficLightLogic) -> bool {
    Ptr::from_mut(l).cast::<MSDeterministicHiLevelTrafficLightLogic>().m().can_release()
}

static MS_DETERMINISTIC_HI_LEVEL_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    init: det_hi_init,
    decide_next_phase: det_hi_decide_next_phase,
    can_release: det_hi_can_release,
    ..MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT
};

/// Leaks a heap object for the whole simulation, like the C++ `new` the constructor uses.
fn leak<T>(value: T) -> &'static mut T {
    Box::leak(Box::new(value))
}

/// Owned copy of a phase (C++ passes MSPhaseDefinition by value to countVehicles).
fn phase_copy(p: &MSPhaseDefinition) -> MSPhaseDefinition {
    p.clone()
}

/// C++ class `MSDeterministicHiLevelTrafficLightLogic` (microsim/traffic_lights/MSDeterministicHiLevelTrafficLightLogic.h)
#[repr(C)]
pub struct MSDeterministicHiLevelTrafficLightLogic {
    /// C++ base class `MSSOTLHiLevelTrafficLightLogic`
    pub base: MSSOTLHiLevelTrafficLightLogic,
    /// This pheronome is an indicator of congestion on input lanes. Its levels refer to the average speed of vehicles passing the input lane: the lower the speed the higher the pheromone. These levels are updated on every input lane, independently on lights state.
    pub inputLanes: MSLaneID_set,
    /// This pheromone is an indicator of congestion on output lanes. Its levels refer to the average speed of vehicles passing the output lane: the lower the speed the higher the pheromone. These levels are updated on every output lane, independently on lights state.
    pub outputLanes: MSLaneID_set,
}

impl std::ops::Deref for MSDeterministicHiLevelTrafficLightLogic {
    type Target = MSSOTLHiLevelTrafficLightLogic;
    fn deref(&self) -> &MSSOTLHiLevelTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSDeterministicHiLevelTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLHiLevelTrafficLightLogic {
        &mut self.base
    }
}

impl MSDeterministicHiLevelTrafficLightLogic {
    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::MSDeterministicHiLevelTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSDeterministicHiLevelTrafficLightLogic::MSDeterministicHiLevelTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters) :
    //     MSSOTLHiLevelTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::HILVL_DETERMINISTIC, phases, step,
    //                                    delay, parameters) {
    //
    //     addPolicy(new MSSOTLPlatoonPolicy(new MSSOTLPolicy3DStimulus("PLATOON", parameters), parameters));
    //     addPolicy(new MSSOTLPhasePolicy(new MSSOTLPolicy3DStimulus("PHASE", parameters), parameters));
    //     addPolicy(new MSSOTLMarchingPolicy(new MSSOTLPolicy3DStimulus("MARCHING", parameters), parameters));
    //     addPolicy(new MSSOTLCongestionPolicy(new MSSOTLPolicy3DStimulus("CONGESTION", parameters), parameters));
    //
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let mut hi = MSSOTLHiLevelTrafficLightLogic::new(tlcontrol, id, programID, TrafficLightType::HILVL_DETERMINISTIC, phases, step, delay, parameters);
        hi.base.base.base.vt = &MS_DETERMINISTIC_HI_LEVEL_TRAFFIC_LIGHT_LOGIC_VT;
        let mut this = Self {
            base: hi,
            inputLanes: BTreeSet::new(),
            outputLanes: BTreeSet::new(),
        };
        // the policies are owned for the whole simulation, as the C++ `new` calls leak them
        let platoonStimulus = leak(MSSOTLPolicy3DStimulus::new("PLATOON".to_string(), parameters));
        let platoon = leak(MSSOTLPlatoonPolicy::new_3(Ptr::from_mut(&mut platoonStimulus.base.base), parameters));
        this.add_policy(Ptr::from_mut(&mut platoon.base));
        let phaseStimulus = leak(MSSOTLPolicy3DStimulus::new("PHASE".to_string(), parameters));
        let phase = leak(MSSOTLPhasePolicy::new_3(Ptr::from_mut(&mut phaseStimulus.base.base), parameters));
        this.add_policy(Ptr::from_mut(&mut phase.base));
        let marchingStimulus = leak(MSSOTLPolicy3DStimulus::new("MARCHING".to_string(), parameters));
        let marching = leak(MSSOTLMarchingPolicy::new_3(Ptr::from_mut(&mut marchingStimulus.base.base), parameters));
        this.add_policy(Ptr::from_mut(&mut marching.base));
        let congestionStimulus = leak(MSSOTLPolicy3DStimulus::new("CONGESTION".to_string(), parameters));
        let congestion = leak(MSSOTLCongestionPolicy::new_3(Ptr::from_mut(&mut congestionStimulus.base.base), parameters));
        this.add_policy(Ptr::from_mut(&mut congestion.base));
        this
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::~MSDeterministicHiLevelTrafficLightLogic() noexcept` [virtual]
    // MSDeterministicHiLevelTrafficLightLogic::~MSDeterministicHiLevelTrafficLightLogic() {
    //
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls with sensors on incoming and outgoing lanes Sensors are built in the simulation according to the type of sensor specified in the simulation parameter
    // void MSDeterministicHiLevelTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSSOTLHiLevelTrafficLightLogic::init(nb);
    //     //Setting the startup policy
    //     choosePolicy(0, 0);
    //     WRITE_MESSAGE(
    //         "*** Intersection " + getID()
    //         + " will run using MSDeterministicHiLevelTrafficLightLogic ***");
    //
    //     MSLane* currentLane = nullptr;
    //     for (MSTrafficLightLogic::LaneVectorVector::const_iterator laneVector =
    //                 myLanes.begin(); laneVector != myLanes.end(); laneVector++) {
    //         for (MSTrafficLightLogic::LaneVector::const_iterator lane =
    //                     laneVector->begin(); lane != laneVector->end(); lane++) {
    //             currentLane = (*lane);
    //             if (inputLanes.find(currentLane->getID()) == inputLanes.end()) {
    //                 inputLanes.insert(currentLane->getID());
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("*** Intersection " + getID() + " inputLanes adding " + currentLane->getID());
    // #endif
    //             }
    //         }
    //     }
    //
    //     for (const LinkVector& oneLink : getLinks()) {
    //         for (int j = 0; j < (int)oneLink.size(); j++) {
    //             currentLane = oneLink[j]->getLane();
    //             if (outputLanes.find(currentLane->getID()) == outputLanes.end()) {
    //                 outputLanes.insert(currentLane->getID());
    // #ifdef SWARM_DEBUG
    //                 WRITE_MESSAGE("*** Intersection " + getID() + " outputLanes adding " + currentLane->getID());
    // #endif
    //             }
    //         }
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        self.base.init(nb);
        // Setting the startup policy
        self.choose_policy(0.0, 0.0);
        utils::common::msg_handler::write_message(&format!("*** Intersection {} will run using MSDeterministicHiLevelTrafficLightLogic ***", self.get_id()));

        let myLanes = self.myLanes.clone();
        for laneVector in myLanes.iter() {
            for &currentLane in laneVector.iter() {
                if !self.inputLanes.contains(currentLane.get_id()) {
                    self.inputLanes.insert(currentLane.get_id().to_string());
                }
            }
        }

        let links = self.get_links().clone();
        for oneLink in links.iter() {
            for j in 0..oneLink.len() {
                let currentLane = oneLink[j].get_lane();
                if !self.outputLanes.contains(currentLane.get_id()) {
                    self.outputLanes.insert(currentLane.get_id().to_string());
                }
            }
        }
    }

    /// C++: `const std::string MSDeterministicHiLevelTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "DeterministicHighLevelTrafficLightLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        "DeterministicHighLevelTrafficLightLogic".to_string()
    }

    /// C++: `int MSDeterministicHiLevelTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int MSDeterministicHiLevelTrafficLightLogic::decideNextPhase() {
    // #ifdef SWARM_DEBUG
    //     MsgHandler::getMessageInstance()->inform("\n" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic decideNextPhase()");
    //     std::ostringstream dnp;
    //     dnp << (MSNet::getInstance()->getCurrentTimeStep()) << " MSDeterministicHiLevelTrafficLightLogic::decideNextPhase:: " << "tlsid=" << getID() << " getCurrentPhaseDef().getState()=" << getCurrentPhaseDef().getState() << " is commit?" << getCurrentPhaseDef().isCommit();
    //     MsgHandler::getMessageInstance()->inform(dnp.str());
    // #endif
    //
    //     //Decide the current policy according to pheromone levels. this should be done only at the end of a chain, before selecting the new one
    //     if (getCurrentPhaseDef().isCommit()) {
    //         decidePolicy();
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "tlsID=" << getID() << " currentPolicyname=" + getCurrentPolicy()->getName();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //
    //     //Execute current policy. congestion "policy" must maintain the commit phase, and that must be an all-red one
    //     return myCurrentPolicy->decideNextPhase(getCurrentPhaseElapsed(),
    //                                             &getCurrentPhaseDef(), getCurrentPhaseIndex(),
    //                                             getPhaseIndexWithMaxCTS(), isThresholdPassed(), isPushButtonPressed(),
    //                                             countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        // Decide the current policy according to pheromone levels. this should be done only at the end of a chain, before selecting the new one
        if self.get_current_phase_def().is_commit() {
            self.decide_policy();
        }
        // Execute current policy. congestion "policy" must maintain the commit phase, and that must be an all-red one
        let policy = self.myCurrentPolicy;
        let curPhase: Ptr<MSPhaseDefinition> = Ptr::new(self.get_current_phase_def());
        let elapsed = self.get_current_phase_elapsed();
        let currentPhaseIndex = self.get_current_phase_index();
        let phaseMaxCTS = self.get_phase_index_with_max_cts();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let vehicleCount = self.count_vehicles(curPhase);
        policy.m().decide_next_phase(elapsed, curPhase, currentPhaseIndex, phaseMaxCTS, thresholdPassed, pushButtonPressed, vehicleCount)
    }

    /// C++: `bool MSDeterministicHiLevelTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSDeterministicHiLevelTrafficLightLogic::canRelease() {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream phero_str;
    //     phero_str << "getCurrentPhaseElapsed()=" << time2string(getCurrentPhaseElapsed()) << " isThresholdPassed()=" << isThresholdPassed() << " currentPhase=" << (&getCurrentPhaseDef())->getState() << " countVehicles()=" << countVehicles(getCurrentPhaseDef());
    //     WRITE_MESSAGE("\nMSDeterministicHiLevelTrafficLightLogic::canRelease(): " + phero_str.str());
    // #endif
    //     return myCurrentPolicy->canRelease(getCurrentPhaseElapsed(),
    //                                        isThresholdPassed(), isPushButtonPressed(), &getCurrentPhaseDef(),
    //                                        countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        let policy = self.myCurrentPolicy;
        let elapsed = self.get_current_phase_elapsed();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let curPhase: Ptr<MSPhaseDefinition> = Ptr::new(self.get_current_phase_def());
        let vehicleCount = self.count_vehicles(curPhase);
        policy.m().can_release(elapsed, thresholdPassed, pushButtonPressed, curPhase, vehicleCount)
    }

    /// C++: `double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes()` [protected]
    // double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes() {
    //     if (inputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double vSpeedInTot = 0;
    //     for (MSLaneID_set::iterator laneIterator = inputLanes.begin();
    //             laneIterator != inputLanes.end(); laneIterator++) {
    //         std::string laneId = *laneIterator;
    //         double maxSpeed = getSensors()->meanVehiclesSpeed(laneId);
    //         if (maxSpeed > -1) {
    //             vSpeedInTot += (13.89 - maxSpeed) * 10. / 13.89;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream i_str;
    //         i_str << " meanVehiclesSpeed " << maxSpeed << " inputLane " << laneId << " ID " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForInputLanes:: in" + i_str.str());
    // #endif
    //     }
    //     return vSpeedInTot / (double)inputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_for_input_lanes(&mut self) -> f64 {
        if self.inputLanes.len() == 0 {
            return 0.0;
        }
        let mut vSpeedInTot: f64 = 0.0;
        let inputLanes = self.inputLanes.clone();
        for laneId in inputLanes.iter() {
            let maxSpeed = self.get_sensors().m().mean_vehicles_speed_2(laneId.clone());
            if maxSpeed > -1.0 {
                vSpeedInTot += (13.89 - maxSpeed) * 10.0 / 13.89;
            }
        }
        vSpeedInTot / (self.inputLanes.len() as f64)
    }

    /// C++: `double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes()` [protected]
    // double MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes() {
    //     if (outputLanes.size() == 0) {
    //         return 0;
    //     }
    //     double vSpeedOutTot = 0;
    //     for (MSLaneID_set::iterator laneIterator = outputLanes.begin();
    //             laneIterator != outputLanes.end(); laneIterator++) {
    //         std::string laneId = *laneIterator;
    //         double maxSpeed = getSensors()->meanVehiclesSpeed(laneId);
    //         if (maxSpeed > -1) {
    //             vSpeedOutTot += (13.89 - maxSpeed) * 10. / 13.89;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream i_str;
    //         i_str << " meanVehiclesSpeed " << maxSpeed << " outputLane " << laneId << " ID " << getID() << " .";
    //         WRITE_MESSAGE(time2string(MSNet::getInstance()->getCurrentTimeStep()) + " MSDeterministicHiLevelTrafficLightLogic::getMeanSpeedForOutputLanes:: out" + i_str.str());
    // #endif
    //     }
    //     return vSpeedOutTot / (double)outputLanes.size();
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed_for_output_lanes(&mut self) -> f64 {
        if self.outputLanes.len() == 0 {
            return 0.0;
        }
        let mut vSpeedOutTot: f64 = 0.0;
        let outputLanes = self.outputLanes.clone();
        for laneId in outputLanes.iter() {
            let maxSpeed = self.get_sensors().m().mean_vehicles_speed_2(laneId.clone());
            if maxSpeed > -1.0 {
                vSpeedOutTot += (13.89 - maxSpeed) * 10.0 / 13.89;
            }
        }
        vSpeedOutTot / (self.outputLanes.len() as f64)
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::decidePolicy()` [virtual, protected]
    /// Decide the current policy according to pheromone levels The decision reflects on currentPolicy value
    // void MSDeterministicHiLevelTrafficLightLogic::decidePolicy() {
    //     // Decide if it is the case to check for another plan
    //     double mean_vSpeed_in = getMeanSpeedForInputLanes();
    //     double mean_vSpeed_out = getMeanSpeedForOutputLanes();
    // #ifdef SWARM_DEBUG
    //     MSSOTLPolicy* oldPolicy = getCurrentPolicy();
    // #endif
    //     choosePolicy(mean_vSpeed_in, mean_vSpeed_out);
    //
    // #ifdef SWARM_DEBUG
    //     MSSOTLPolicy* newPolicy = getCurrentPolicy();
    //     if (newPolicy != oldPolicy) {
    //         SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //         std::ostringstream phero_str;
    //         phero_str << " (mean_vSpeed_in= " << mean_vSpeed_in << " ,mean_vSpeed_out= " << mean_vSpeed_out << " )";
    //         WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: " + newPolicy->getName() + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    //     } else { //debug purpose only
    //         std::ostringstream phero_str;
    //         phero_str << " (mean_vSpeed_in= " << mean_vSpeed_in << " ,mean_vSpeed_out= " << mean_vSpeed_out << " )";
    //         SUMOTime step = MSNet::getInstance()->getCurrentTimeStep();
    //         WRITE_MESSAGE("TL " + getID() + " time=" + time2string(step) + " Policy: Nochanges" + phero_str.str() + " OldPolicy: " + oldPolicy->getName() + " id " + getID() + " .");
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn decide_policy(&mut self) {
        // Decide if it is the case to check for another plan
        let mean_vSpeed_in = self.get_mean_speed_for_input_lanes();
        let mean_vSpeed_out = self.get_mean_speed_for_output_lanes();
        self.choose_policy(mean_vSpeed_in, mean_vSpeed_out);
    }

    /// C++: `void MSDeterministicHiLevelTrafficLightLogic::choosePolicy(double, double)` [protected]
    // void MSDeterministicHiLevelTrafficLightLogic::choosePolicy(
    //     double mean_vSpeed_in, double mean_vSpeed_out) {
    //
    //     int index_maxStimulus = 0;
    //     double maxStimulus = -1;
    //     // Compute simulus for each policy
    //     for (int i = 0; i < (int)myPolicies.size(); i++) {
    //         double stimulus = myPolicies[i]->computeDesirability(mean_vSpeed_in,
    //                           mean_vSpeed_out);
    //         if (stimulus > maxStimulus) {
    //             maxStimulus = stimulus;
    //             index_maxStimulus = i;
    //         }
    // #ifdef SWARM_DEBUG
    //         std::ostringstream so_str;
    //         so_str << " policy " << getPolicies()[i]->getName() << " stimulus " << stimulus;
    //         WRITE_MESSAGE("MSDeterministicHiLevelTrafficLightLogic::choosePolicy::" + so_str.str());
    // #endif
    //     }
    //     activate(myPolicies[index_maxStimulus]);
    //
    // }
    #[allow(unused_variables)]
    pub fn choose_policy(&mut self, mean_vSpeed_in: f64, mean_vSpeed_out: f64) {
        let mut index_maxStimulus: usize = 0;
        let mut maxStimulus: f64 = -1.0;
        // Compute simulus for each policy
        for i in 0..self.myPolicies.len() {
            let policy = self.myPolicies[i];
            let stimulus = policy.m().compute_desirability_2(mean_vSpeed_in, mean_vSpeed_out);
            if stimulus > maxStimulus {
                maxStimulus = stimulus;
                index_maxStimulus = i;
            }
        }
        let chosen = self.myPolicies[index_maxStimulus];
        self.activate(chosen);
    }

}

