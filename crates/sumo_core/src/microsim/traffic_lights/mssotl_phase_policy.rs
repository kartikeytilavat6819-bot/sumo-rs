//! Port of microsim/traffic_lights/MSSOTLPhasePolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mssotl_policy::{MSSOTLPolicyVt, MSSOTL_POLICY_BASE_VT};

fn phase_policy_can_release(p: &mut MSSOTLPolicy, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
    Ptr::from_mut(p).cast::<MSSOTLPhasePolicy>().m().can_release(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount)
}

static MS_SOTL_PHASE_POLICY_VT: MSSOTLPolicyVt = MSSOTLPolicyVt {
    can_release: phase_policy_can_release,
    ..MSSOTL_POLICY_BASE_VT
};
use utils::common::msg_handler::write_message;

/// Class for low-level phase policy.
/// C++ class `MSSOTLPhasePolicy` (microsim/traffic_lights/MSSOTLPhasePolicy.h)
#[repr(C)]
pub struct MSSOTLPhasePolicy {
    /// C++ base class `MSSOTLPolicy`
    pub base: MSSOTLPolicy,
    /// C++ base class `PushButtonLogic`
    pub base2: PushButtonLogic,
    /// C++ base class `SigmoidLogic`
    pub base3: SigmoidLogic,
    pub m_useVehicleTypesWeights: bool,
}

impl std::ops::Deref for MSSOTLPhasePolicy {
    type Target = MSSOTLPolicy;
    fn deref(&self) -> &MSSOTLPolicy {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPhasePolicy {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy {
        &mut self.base
    }
}

impl MSSOTLPhasePolicy {
    /// C++: `void MSSOTLPhasePolicy::MSSOTLPhasePolicy(const Parameterised::Map &)`
    // MSSOTLPhasePolicy::MSSOTLPhasePolicy(const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Phase", parameters) {
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new(parameters: &ParameterisedMap) -> Self {
        let mut s = MSSOTLPhasePolicy {
            base: MSSOTLPolicy::new("Phase".to_string(), parameters),
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
            base3: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            m_useVehicleTypesWeights: false,
        };
        s.base.vt = &MS_SOTL_PHASE_POLICY_VT;
        s.init();
        s
    }

    /// C++: `void MSSOTLPhasePolicy::MSSOTLPhasePolicy(MSSOTLPolicyDesirability *)`
    // MSSOTLPhasePolicy::MSSOTLPhasePolicy(MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     MSSOTLPolicy("Phase", desirabilityAlgorithm) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("PHASE");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_2(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        let mut s = MSSOTLPhasePolicy {
            base: MSSOTLPolicy::new_2("Phase".to_string(), desirabilityAlgorithm),
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
            base3: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            m_useVehicleTypesWeights: false,
        };
        s.get_desirability_algorithm().m().set_key_prefix("PHASE".to_string());
        s.base.vt = &MS_SOTL_PHASE_POLICY_VT;
        s.init();
        s
    }

    /// C++: `void MSSOTLPhasePolicy::MSSOTLPhasePolicy(MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    // MSSOTLPhasePolicy::MSSOTLPhasePolicy(MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //                                      const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Phase", desirabilityAlgorithm, parameters) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("PHASE");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_3(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &ParameterisedMap) -> Self {
        let mut s = MSSOTLPhasePolicy {
            base: MSSOTLPolicy::new_3("Phase".to_string(), desirabilityAlgorithm, parameters),
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
            base3: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            m_useVehicleTypesWeights: false,
        };
        s.get_desirability_algorithm().m().set_key_prefix("PHASE".to_string());
        s.base.vt = &MS_SOTL_PHASE_POLICY_VT;
        s.init();
        s
    }

    /// C++: `bool MSSOTLPhasePolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual]
    // bool MSSOTLPhasePolicy::canRelease(SUMOTime elapsed, bool thresholdPassed, bool pushButtonPressed,
    //                                    const MSPhaseDefinition* stage, int vehicleCount) {
    // //  DBG(
    //     std::ostringstream str;
    //     str << "MSSOTLPhasePolicy::canRelease threshold " << thresholdPassed << " vehicle " << vehicleCount << " elapsed " << elapsed << " min " << stage->minDuration;
    //     WRITE_MESSAGE(str.str());
    // //          );
    //     if (elapsed >= stage->minDuration) {
    //         if (pushButtonLogic(elapsed, pushButtonPressed, stage)) {
    //             return true;
    //         }
    //         if (thresholdPassed) {
    //             return thresholdPassed;
    //         } else if (m_useVehicleTypesWeights) {
    //             if (sigmoidLogic(elapsed, stage, vehicleCount)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        // C++ streams the bool as 0/1 (no boolalpha)
        write_message(&format!(
            "MSSOTLPhasePolicy::canRelease threshold {} vehicle {} elapsed {} min {}",
            thresholdPassed as i32, vehicleCount, elapsed, stage.minDuration
        ));
        if elapsed >= stage.minDuration {
            if self.base2.push_button_logic(elapsed, pushButtonPressed, stage) {
                return true;
            }
            if thresholdPassed {
                return thresholdPassed;
            } else if self.m_useVehicleTypesWeights {
                if self.base3.sigmoid_logic(elapsed, stage, vehicleCount) {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void MSSOTLPhasePolicy::init()` [virtual, protected]
    // void MSSOTLPhasePolicy::init() {
    //     PushButtonLogic::init("MSSOTLPhasePolicy", this);
    //     SigmoidLogic::init("MSSOTLPhasePolicy", this);
    //     m_useVehicleTypesWeights = getParameter("USE_VEHICLE_TYPES_WEIGHTS", "0") == "1";
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        self.base2.init("MSSOTLPhasePolicy".to_string(), Ptr::from_mut(&mut self.base.base));
        self.base3.init("MSSOTLPhasePolicy".to_string(), Ptr::from_mut(&mut self.base.base));
        self.m_useVehicleTypesWeights = self.get_parameter("USE_VEHICLE_TYPES_WEIGHTS", "0") == "1";
    }

}

