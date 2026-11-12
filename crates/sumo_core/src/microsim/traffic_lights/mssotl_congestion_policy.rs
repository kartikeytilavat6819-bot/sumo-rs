//! Port of microsim/traffic_lights/MSSOTLCongestionPolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Class for low-level congestion policy.
/// C++ class `MSSOTLCongestionPolicy` (microsim/traffic_lights/MSSOTLCongestionPolicy.h)
#[repr(C)]
pub struct MSSOTLCongestionPolicy {
    /// C++ base class `MSSOTLPolicy`
    pub base: MSSOTLPolicy,
}

impl std::ops::Deref for MSSOTLCongestionPolicy {
    type Target = MSSOTLPolicy;
    fn deref(&self) -> &MSSOTLPolicy {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLCongestionPolicy {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy {
        &mut self.base
    }
}

impl MSSOTLCongestionPolicy {
    /// C++: `void MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(const Parameterised::Map &)`
    // MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Congestion", parameters) {
    // }
    #[allow(unused_variables)]
    pub fn new(parameters: &Unmapped /* Parameterised::Map */) -> Self {
        Self { base: MSSOTLPolicy::new("Congestion".to_string(), parameters) }
    }

    /// C++: `void MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(MSSOTLPolicyDesirability *)`
    // MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     MSSOTLPolicy("Congestion", desirabilityAlgorithm) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("CONGESTION");
    // }
    #[allow(unused_variables)]
    pub fn new_2(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        let mut base = MSSOTLPolicy::new_2("Congestion".to_string(), desirabilityAlgorithm);
        base.get_desirability_algorithm().m().set_key_prefix("CONGESTION".to_string());
        Self { base }
    }

    /// C++: `void MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    // MSSOTLCongestionPolicy::MSSOTLCongestionPolicy(
    //     MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //     const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Congestion", desirabilityAlgorithm, parameters) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("CONGESTION");
    //
    // }
    #[allow(unused_variables)]
    pub fn new_3(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        let mut base = MSSOTLPolicy::new_3("Congestion".to_string(), desirabilityAlgorithm, parameters);
        base.get_desirability_algorithm().m().set_key_prefix("CONGESTION".to_string());
        Self { base }
    }

    /// C++: `int MSSOTLCongestionPolicy::decideNextPhase(SUMOTime, const MSPhaseDefinition *, int, int, bool, bool, int)` [virtual]
    // int MSSOTLCongestionPolicy::decideNextPhase(SUMOTime elapsed,
    //         const MSPhaseDefinition* stage, int currentPhaseIndex,
    //         int /* phaseMaxCTS */, bool thresholdPassed, bool pushButtonPressed, int vehicleCount) {
    //     if (stage->isCommit()) {
    //         // decide which chain to activate. Gotta work on this
    //         return currentPhaseIndex;
    //     }
    //     if (stage->isTransient()) {
    //         //If the junction was in a transient step
    //         //=> go to the next step and return computeReturnTime()
    //         return currentPhaseIndex + 1;
    //     }
    //
    //     if (stage->isDecisional()) {
    //         if (canRelease(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount)) {
    //             return currentPhaseIndex + 1;
    //         }
    //     }
    //
    //     return currentPhaseIndex;
    // }
    #[allow(unused_variables)]
    pub fn decide_next_phase(&mut self, elapsed: SUMOTime, stage: Ptr<MSPhaseDefinition>, currentPhaseIndex: i32, phaseMaxCTS: i32, thresholdPassed: bool, pushButtonPressed: bool, vehicleCount: i32) -> i32 {
        if stage.is_commit() {
            // decide which chain to activate. Gotta work on this
            return currentPhaseIndex;
        }
        if stage.is_transient() {
            // If the junction was in a transient step
            // => go to the next step and return computeReturnTime()
            return currentPhaseIndex + 1;
        }
        if stage.is_decisional() {
            if self.can_release(elapsed, thresholdPassed, pushButtonPressed, stage, vehicleCount) {
                return currentPhaseIndex + 1;
            }
        }
        currentPhaseIndex
    }

    /// C++: `bool MSSOTLCongestionPolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual]
    // bool MSSOTLCongestionPolicy::canRelease(SUMOTime elapsed, bool /* thresholdPassed */, bool /* pushButtonPressed */,
    //                                         const MSPhaseDefinition* stage, int /* vehicleCount */) {
    //     return (elapsed >= stage->minDuration);
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        elapsed >= stage.minDuration
    }

}

