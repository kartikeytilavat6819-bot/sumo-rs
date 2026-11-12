//! Port of microsim/traffic_lights/MSSOTLPolicyBasedTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_message;

/// A self-organizing traffic light logic based on a particular policy
/// C++ class `MSSOTLPolicyBasedTrafficLightLogic` (microsim/traffic_lights/MSSOTLPolicyBasedTrafficLightLogic.h)
#[repr(C)]
pub struct MSSOTLPolicyBasedTrafficLightLogic {
    /// C++ base class `MSSOTLTrafficLightLogic`
    pub base: MSSOTLTrafficLightLogic,
    pub myPolicy: Ptr<MSSOTLPolicy>,
}

impl std::ops::Deref for MSSOTLPolicyBasedTrafficLightLogic {
    type Target = MSSOTLTrafficLightLogic;
    fn deref(&self) -> &MSSOTLTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicyBasedTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLTrafficLightLogic {
        &mut self.base
    }
}

/// shortcut: MSPhaseDefinition has no Clone yet, so countVehicles' by-value copy is made field by field; upgrade to .clone() once ms_phase_definition.rs derives Clone.
fn copy_phase_definition(p: &MSPhaseDefinition) -> MSPhaseDefinition {
    MSPhaseDefinition {
        duration: p.duration,
        lastDuration: p.lastDuration,
        minDuration: p.minDuration,
        maxDuration: p.maxDuration,
        earliestEnd: p.earliestEnd,
        latestEnd: p.latestEnd,
        earlyTarget: p.earlyTarget.clone(),
        finalTarget: p.finalTarget.clone(),
        myLastSwitch: p.myLastSwitch,
        myLastEnd: p.myLastEnd,
        nextPhases: p.nextPhases.clone(),
        name: p.name.clone(),
        yellow: p.yellow,
        red: p.red,
        vehext: p.vehext,
        myTransientNotDecisional: p.myTransientNotDecisional,
        myCommit: p.myCommit,
        myUndefined: p.myUndefined,
        myTargetLaneSet: p.myTargetLaneSet.clone(),
        myState: p.myState.clone(),
    }
}

impl MSSOTLPolicyBasedTrafficLightLogic {
    /// C++: `void MSSOTLPolicyBasedTrafficLightLogic::MSSOTLPolicyBasedTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLPolicy *)`
    /// Constructor without sensors passed
    // MSSOTLPolicyBasedTrafficLightLogic::MSSOTLPolicyBasedTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const TrafficLightType logicType, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters,
    //     MSSOTLPolicy* policy) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, logicType, phases, step, delay,
    //                             parameters), myPolicy(policy) {
    //
    //     MsgHandler::getMessageInstance()->inform(
    //         "*** Intersection " + id + " will run using MSSOTL"
    //         + policy->getName() + "TrafficLightLogic ***");
    //
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, policy: Ptr<MSSOTLPolicy>) -> Self {
        // shortcut: MSSOTLTrafficLightLogic::new still takes Unmapped parameters, so the map is copied into base2 after construction; upgrade when mssotl_traffic_light_logic.rs maps Parameterised::Map.
        let mut base = MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, logicType, phases, step, delay, &());
        base.base.base.base2 = Parameterised::from_map(parameters.clone());
        let res = MSSOTLPolicyBasedTrafficLightLogic { base, myPolicy: policy };
        write_message(&format!("*** Intersection {} will run using MSSOTL{}TrafficLightLogic ***", id, policy.m().get_name()));
        res
    }

    /// C++: `void MSSOTLPolicyBasedTrafficLightLogic::MSSOTLPolicyBasedTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLPolicy *, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLPolicyBasedTrafficLightLogic::MSSOTLPolicyBasedTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const TrafficLightType logicType, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters,
    //     MSSOTLPolicy* policy, MSSOTLSensors* sensors) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, logicType, phases, step, delay,
    //                             parameters, sensors), myPolicy(policy) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, policy: Ptr<MSSOTLPolicy>, sensors: Ptr<MSSOTLSensors>) -> Self {
        // shortcut: same as the constructor above, the base still takes Unmapped parameters; upgrade when mssotl_traffic_light_logic.rs maps Parameterised::Map.
        let mut base = MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, logicType, phases, step, delay, &(), sensors);
        base.base.base.base2 = Parameterised::from_map(parameters.clone());
        MSSOTLPolicyBasedTrafficLightLogic { base, myPolicy: policy }
    }

    /// C++: `void MSSOTLPolicyBasedTrafficLightLogic::~MSSOTLPolicyBasedTrafficLightLogic() noexcept` [virtual]
    // MSSOTLPolicyBasedTrafficLightLogic::~MSSOTLPolicyBasedTrafficLightLogic(void) {
    //
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `MSSOTLPolicy * MSSOTLPolicyBasedTrafficLightLogic::getPolicy()`
    // MSSOTLPolicy* getPolicy() {
    //         return myPolicy;
    //     }
    #[allow(unused_variables)]
    pub fn get_policy(&mut self) -> Ptr<MSSOTLPolicy> {
        self.myPolicy
    }

    /// C++: `const std::string MSSOTLPolicyBasedTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "policyBasedTrafficLightLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        "policyBasedTrafficLightLogic".to_string()
    }

    /// C++: `bool MSSOTLPolicyBasedTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSSOTLPolicyBasedTrafficLightLogic::canRelease() {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "\n" << time2string(MSNet::getInstance()->getCurrentTimeStep()) << " " << getID() << "invoked MSSOTLPolicyBasedTrafficLightLogic::canRelease()";
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     return myPolicy->canRelease(getCurrentPhaseElapsed(), isThresholdPassed(), isPushButtonPressed(),
    //                                 &getCurrentPhaseDef(), countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        // determinism: C++ leaves argument evaluation order unspecified; arguments are evaluated left to right here.
        let elapsed = self.get_current_phase_elapsed();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let stage = Ptr::new(self.get_current_phase_def());
        let mut phaseCopy = copy_phase_definition(self.get_current_phase_def());
        let phase = Ptr::from_mut(&mut phaseCopy);
        let vehicleCount = self.count_vehicles(phase);
        let policy = self.myPolicy;
        policy.m().can_release(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount)
    }

    /// C++: `int MSSOTLPolicyBasedTrafficLightLogic::decideNextPhase()` [virtual, protected]
    // int MSSOTLPolicyBasedTrafficLightLogic::decideNextPhase() {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "\n" << time2string(MSNet::getInstance()->getCurrentTimeStep()) << " " << getID() << "invoked MSSOTLPolicyBasedTrafficLightLogic::decideNextPhase()";
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     return myPolicy->decideNextPhase(getCurrentPhaseElapsed(),
    //                                      &getCurrentPhaseDef(), getCurrentPhaseIndex(),
    //                                      getPhaseIndexWithMaxCTS(), isThresholdPassed(), isPushButtonPressed(),
    //                                      countVehicles(getCurrentPhaseDef()));
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self) -> i32 {
        // determinism: C++ leaves argument evaluation order unspecified; arguments are evaluated left to right here.
        let elapsed = self.get_current_phase_elapsed();
        let stage = Ptr::new(self.get_current_phase_def());
        let currentPhaseIndex = self.get_current_phase_index();
        let phaseMaxCTS = self.get_phase_index_with_max_cts();
        let thresholdPassed = self.is_threshold_passed();
        let pushButtonPressed = self.is_push_button_pressed();
        let mut phaseCopy = copy_phase_definition(self.get_current_phase_def());
        let phase = Ptr::from_mut(&mut phaseCopy);
        let vehicleCount = self.count_vehicles(phase);
        let policy = self.myPolicy;
        policy.m().decide_next_phase(elapsed, stage, currentPhaseIndex, phaseMaxCTS, thresholdPassed, pushButtonPressed, vehicleCount)
    }

}

