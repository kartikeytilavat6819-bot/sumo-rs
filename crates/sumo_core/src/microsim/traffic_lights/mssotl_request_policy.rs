//! Port of microsim/traffic_lights/MSSOTLRequestPolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Class for low-level request policy.
/// C++ class `MSSOTLRequestPolicy` (microsim/traffic_lights/MSSOTLRequestPolicy.h)
pub struct MSSOTLRequestPolicy {
    /// C++ base class `MSSOTLPolicy`
    pub base: MSSOTLPolicy,
}

impl std::ops::Deref for MSSOTLRequestPolicy {
    type Target = MSSOTLPolicy;
    fn deref(&self) -> &MSSOTLPolicy {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLRequestPolicy {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy {
        &mut self.base
    }
}

impl MSSOTLRequestPolicy {
    /// C++: `void MSSOTLRequestPolicy::MSSOTLRequestPolicy(const Parameterised::Map &)`
    // MSSOTLRequestPolicy::MSSOTLRequestPolicy(
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Request", parameters) {
    // }
    #[allow(unused_variables)]
    pub fn new(parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSSOTLRequestPolicy::MSSOTLRequestPolicy")
    }

    /// C++: `void MSSOTLRequestPolicy::MSSOTLRequestPolicy(MSSOTLPolicyDesirability *)`
    // MSSOTLRequestPolicy::MSSOTLRequestPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     MSSOTLPolicy("Request", desirabilityAlgorithm) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("REQUEST");
    // }
    #[allow(unused_variables)]
    pub fn new_2(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        todo!("port MSSOTLRequestPolicy::MSSOTLRequestPolicy")
    }

    /// C++: `void MSSOTLRequestPolicy::MSSOTLRequestPolicy(MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    // MSSOTLRequestPolicy::MSSOTLRequestPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Request", desirabilityAlgorithm, parameters) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("REQUEST");
    //
    // }
    #[allow(unused_variables)]
    pub fn new_3(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSSOTLRequestPolicy::MSSOTLRequestPolicy")
    }

    /// C++: `bool MSSOTLRequestPolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual]
    // bool MSSOTLRequestPolicy::canRelease(SUMOTime elapsed, bool thresholdPassed,  bool /* pushButtonPressed */,
    //                                      const MSPhaseDefinition* /* stage */, int /* vehicleCount */) {
    //     if (elapsed >= getMinDecisionalPhaseDuration()) {
    //         return thresholdPassed;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        todo!("port MSSOTLRequestPolicy::canRelease")
    }

    /// C++: `int MSSOTLRequestPolicy::getMinDecisionalPhaseDuration()`
    // int getMinDecisionalPhaseDuration() {
    //         return StringUtils::toInt(getParameter("MIN_DECISIONAL_PHASE_DUR", "5000"));
    //     }
    #[allow(unused_variables)]
    pub fn get_min_decisional_phase_duration(&mut self) -> i32 {
        todo!("port MSSOTLRequestPolicy::getMinDecisionalPhaseDuration")
    }

}

