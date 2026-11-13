//! Port of microsim/traffic_lights/MSSOTLWaveTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ class `MSSOTLWaveTrafficLightLogic` (microsim/traffic_lights/MSSOTLWaveTrafficLightLogic.h)
#[repr(C)]
pub struct MSSOTLWaveTrafficLightLogic {
    /// C++ base class `MSSOTLTrafficLightLogic`
    pub base: MSSOTLTrafficLightLogic,
}

impl std::ops::Deref for MSSOTLWaveTrafficLightLogic {
    type Target = MSSOTLTrafficLightLogic;
    fn deref(&self) -> &MSSOTLTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLWaveTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLTrafficLightLogic {
        &mut self.base
    }
}

impl MSSOTLWaveTrafficLightLogic {
    /// C++: `void MSSOTLWaveTrafficLightLogic::MSSOTLWaveTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSOTLWaveTrafficLightLogic::MSSOTLWaveTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay,
    //     const Parameterised::Map& parameters) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::SOTL_WAVE, phases, step, delay,
    //                             parameters) {
    //     MsgHandler::getMessageInstance()->inform(
    //         "*** Intersection " + id
    //         + " will run using MSSOTLWaveTrafficLightLogic ***");
    //     //sets the lastDuration of every phase to the same value as the default duration of that phase
    //     for (int i = 0; i < getPhaseNumber(); i++) {
    //         (*myPhases[i]).lastDuration = (*myPhases[i]).duration;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        let base = MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, TrafficLightType::SOTL_WAVE, phases, step, delay, parameters);
        utils::common::msg_handler::write_message(&format!("*** Intersection {} will run using MSSOTLWaveTrafficLightLogic ***", id));
        let mut s = Self { base };
        for i in 0..s.get_phase_number() {
            let phase = s.myPhases[i as usize];
            phase.m().lastDuration = phase.duration;
        }
        s
    }

    /// C++: `void MSSOTLWaveTrafficLightLogic::MSSOTLWaveTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLWaveTrafficLightLogic::MSSOTLWaveTrafficLightLogic(
    //     MSTLLogicControl& tlcontrol, const std::string& id,
    //     const std::string& programID, const Phases& phases, int step,
    //     SUMOTime delay, const Parameterised::Map& parameters,
    //     MSSOTLSensors* sensors) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, TrafficLightType::SOTL_WAVE, phases, step, delay,
    //                             parameters, sensors) {
    //     //sets the lastDuration of every phase to the same value as the default duration of that phase
    //     for (int i = 0; i < getPhaseNumber(); i++) {
    //         (*myPhases[i]).lastDuration = (*myPhases[i]).duration;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, sensors: Ptr<MSSOTLSensors>) -> Self {
        let mut s = Self { base: MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, TrafficLightType::SOTL_WAVE, phases, step, delay, parameters, sensors) };
        for i in 0..s.get_phase_number() {
            let phase = s.myPhases[i as usize];
            phase.m().lastDuration = phase.duration;
        }
        s
    }

    /// C++: `const std::string MSSOTLWaveTrafficLightLogic::getLogicType() const` [const]
    /// Returns the type of the logic as a string
    // const std::string getLogicType() const {
    //         return "waveTrafficLightLogic";
    //     }
    #[allow(unused_variables)]
    pub fn get_logic_type(&self) -> String {
        "waveTrafficLightLogic".to_string()
    }

    /// C++: `bool MSSOTLWaveTrafficLightLogic::canRelease()` [virtual, protected]
    // bool MSSOTLWaveTrafficLightLogic::canRelease() {
    //
    //     //10% of lastDuration
    //     SUMOTime delta = 10 * getCurrentPhaseDef().lastDuration / 100;
    //
    //     //this allows a minimum variation of +-1s
    //     if (delta < 1000) {
    //         delta = 1000;
    //     }
    //     if (getCurrentPhaseElapsed() >= getCurrentPhaseDef().minDuration) {
    //         if (getCurrentPhaseElapsed()
    //                 >= getCurrentPhaseDef().lastDuration - delta) {
    //             if ((countVehicles() == 0) //no other vehicles approaching green lights
    //                     || (getCurrentPhaseElapsed()
    //                         >= getCurrentPhaseDef().lastDuration + delta) //maximum value of the window surrounding lastDuration
    //                     || (getCurrentPhaseElapsed()
    //                         >= getCurrentPhaseDef().maxDuration) //declared maximum duration has been reached
    //                ) {
    //
    //                 (*myPhases[getCurrentPhaseIndex()]).lastDuration =
    //                     getCurrentPhaseElapsed();
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self) -> bool {
        //10% of lastDuration
        let mut delta: SUMOTime = 10 * self.get_current_phase_def().lastDuration / 100;

        //this allows a minimum variation of +-1s
        if delta < 1000 {
            delta = 1000;
        }
        if self.get_current_phase_elapsed() >= self.get_current_phase_def().minDuration {
            if self.get_current_phase_elapsed() >= self.get_current_phase_def().lastDuration - delta {
                if (self.count_vehicles() == 0) //no other vehicles approaching green lights
                    || (self.get_current_phase_elapsed() >= self.get_current_phase_def().lastDuration + delta) //maximum value of the window surrounding lastDuration
                    || (self.get_current_phase_elapsed() >= self.get_current_phase_def().maxDuration)
                //declared maximum duration has been reached
                {
                    let elapsed = self.get_current_phase_elapsed();
                    let phase = self.myPhases[self.get_current_phase_index() as usize];
                    phase.m().lastDuration = elapsed;
                    return true;
                }
            }
        }
        false
    }

    /// C++: `int MSSOTLWaveTrafficLightLogic::countVehicles()` [private]
    // int MSSOTLWaveTrafficLightLogic::countVehicles() {
    //     std::string state = getCurrentPhaseDef().getState();
    //     int vehicles = 0;
    //     for (int i = 0; i < (int)getLaneVectors().size(); i++) {
    //         if (i > 0
    //                 && ((getLaneVectors()[i][0]->getID()).compare(
    //                         getLaneVectors()[i - 1][0]->getID()) == 0)) {
    //             continue;
    //         }
    //         if (state[i] != 'r') {
    //             vehicles += getSensors()->countVehicles(getLaneVectors()[i][0]);
    //         }
    //
    //     }
    //     return vehicles;
    // }
    #[allow(unused_variables)]
    pub fn count_vehicles(&mut self) -> i32 {
        let state: String = self.get_current_phase_def().get_state().to_string();
        let mut vehicles = 0;
        for i in 0..self.get_lane_vectors().len() {
            if i > 0 && (self.get_lane_vectors()[i][0].get_id() == self.get_lane_vectors()[i - 1][0].get_id()) {
                continue;
            }
            if state.as_bytes()[i] != b'r' {
                vehicles += self.get_sensors().m().count_vehicles(self.get_lane_vectors()[i][0]);
            }
        }
        vehicles
    }

}

