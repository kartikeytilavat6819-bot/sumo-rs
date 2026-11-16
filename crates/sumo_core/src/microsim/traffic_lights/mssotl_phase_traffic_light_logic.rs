//! Port of microsim/traffic_lights/MSSOTLPhaseTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mssotl_traffic_light_logic::MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT;

fn phase_can_release(l: &mut MSTrafficLightLogic) -> bool {
    Ptr::from_mut(l).cast::<MSSOTLPhaseTrafficLightLogic>().m().can_release()
}

static MS_SOTL_PHASE_TRAFFIC_LIGHT_LOGIC_VT: MSTrafficLightLogicVt = MSTrafficLightLogicVt {
    can_release: phase_can_release,
    ..MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT
};

/// C++ class `MSSOTLPhaseTrafficLightLogic` (microsim/traffic_lights/MSSOTLPhaseTrafficLightLogic.h)
#[repr(C)]
pub struct MSSOTLPhaseTrafficLightLogic {
    /// C++ base class `MSSOTLTrafficLightLogic`
    pub base: MSSOTLTrafficLightLogic,
}

impl std::ops::Deref for MSSOTLPhaseTrafficLightLogic {
    type Target = MSSOTLTrafficLightLogic;
    fn deref(&self) -> &MSSOTLTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPhaseTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLTrafficLightLogic {
        &mut self.base
    }
}

impl MSSOTLPhaseTrafficLightLogic {
    /// C++: `void MSSOTLPhaseTrafficLightLogic::MSSOTLPhaseTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSOTLPhaseTrafficLightLogic::MSSOTLPhaseTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay,
    //                             parameters) {
    //     MsgHandler::getMessageInstance()->inform(
    //         "*** Intersection " + id
    //         + " will run using MSSOTLPhaseTrafficLightLogic ***");
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let mut base = MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay, parameters);
        base.base.base.vt = &MS_SOTL_PHASE_TRAFFIC_LIGHT_LOGIC_VT;
        utils::common::msg_handler::write_message(&format!("*** Intersection {} will run using MSSOTLPhaseTrafficLightLogic ***", id));
        Self { base }
    }

    /// C++: `void MSSOTLPhaseTrafficLightLogic::MSSOTLPhaseTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLPhaseTrafficLightLogic::MSSOTLPhaseTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters,
    //     MSSOTLSensors* sensors) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay,
    //                             parameters, sensors) {
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, sensors: Ptr<MSSOTLSensors>) -> Self {
        let mut base = MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, TrafficLightType::SOTL_PHASE, phases, step, delay, parameters, sensors);
        base.base.base.vt = &MS_SOTL_PHASE_TRAFFIC_LIGHT_LOGIC_VT;
        Self { base }
    }

    /// C++: `bool MSSOTLPhaseTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSSOTLPhaseTrafficLightLogic::canRelease() {
    //     if (getCurrentPhaseElapsed() >= getCurrentPhaseDef().minDuration) {
    //         return isThresholdPassed();
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        if self.get_current_phase_elapsed() >= self.get_current_phase_def().minDuration {
            return self.is_threshold_passed();
        }
        false
    }

}

