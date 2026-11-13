//! Port of microsim/traffic_lights/MSSOTLMarchingPolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Class for low-level marching policy.
/// C++ class `MSSOTLMarchingPolicy` (microsim/traffic_lights/MSSOTLMarchingPolicy.h)
#[repr(C)]
pub struct MSSOTLMarchingPolicy {
    /// C++ base class `MSSOTLPolicy`
    pub base: MSSOTLPolicy,
    /// C++ base class `PushButtonLogic`
    pub base2: PushButtonLogic,
}

impl std::ops::Deref for MSSOTLMarchingPolicy {
    type Target = MSSOTLPolicy;
    fn deref(&self) -> &MSSOTLPolicy {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLMarchingPolicy {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy {
        &mut self.base
    }
}

impl MSSOTLMarchingPolicy {
    /// C++: `void MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(const Parameterised::Map &)`
    // MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Marching", parameters) {
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new(parameters: &ParameterisedMap) -> Self {
        let mut s = Self {
            base: MSSOTLPolicy::new("Marching".to_string(), parameters),
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        s.init();
        s
    }

    /// C++: `void MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(MSSOTLPolicyDesirability *)`
    // MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     MSSOTLPolicy("Marching", desirabilityAlgorithm) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("MARCHING");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_2(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        let mut base = MSSOTLPolicy::new_2("Marching".to_string(), desirabilityAlgorithm);
        base.get_desirability_algorithm().m().set_key_prefix("MARCHING".to_string());
        let mut s = Self {
            base,
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        s.init();
        s
    }

    /// C++: `void MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    // MSSOTLMarchingPolicy::MSSOTLMarchingPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Marching", desirabilityAlgorithm, parameters) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("MARCHING");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_3(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &ParameterisedMap) -> Self {
        let mut base = MSSOTLPolicy::new_3("Marching".to_string(), desirabilityAlgorithm, parameters);
        base.get_desirability_algorithm().m().set_key_prefix("MARCHING".to_string());
        let mut s = Self {
            base,
            base2: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        s.init();
        s
    }

    /// C++: `bool MSSOTLMarchingPolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual]
    // bool MSSOTLMarchingPolicy::canRelease(SUMOTime elapsed, bool /* thresholdPassed */, bool pushButtonPressed,
    //                                       const MSPhaseDefinition* stage, int /* vehicleCount */) {
    //     if (elapsed >= stage->minDuration && pushButtonLogic(elapsed, pushButtonPressed, stage)) {
    //         return true;
    //     }
    //     return (elapsed >= stage->duration);
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        if elapsed >= stage.minDuration && self.base2.push_button_logic(elapsed, pushButtonPressed, stage) {
            return true;
        }
        elapsed >= stage.duration
    }

    /// C++: `void MSSOTLMarchingPolicy::init()` [virtual, protected]
    // void MSSOTLMarchingPolicy::init() {
    //     PushButtonLogic::init("MSSOTLMarchingPolicy", this);
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        self.base2.init("MSSOTLMarchingPolicy".to_string(), Ptr::from_mut(&mut self.base.base));
    }

}

