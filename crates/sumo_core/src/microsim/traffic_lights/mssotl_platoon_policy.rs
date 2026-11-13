//! Port of microsim/traffic_lights/MSSOTLPlatoonPolicy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Class for low-level platoon policy.
/// C++ class `MSSOTLPlatoonPolicy` (microsim/traffic_lights/MSSOTLPlatoonPolicy.h)
#[repr(C)]
pub struct MSSOTLPlatoonPolicy {
    /// C++ base class `MSSOTLPolicy`
    pub base: MSSOTLPolicy,
    /// C++ base class `SigmoidLogic`
    pub base2: SigmoidLogic,
    /// C++ base class `PushButtonLogic`
    pub base3: PushButtonLogic,
}

impl std::ops::Deref for MSSOTLPlatoonPolicy {
    type Target = MSSOTLPolicy;
    fn deref(&self) -> &MSSOTLPolicy {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPlatoonPolicy {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy {
        &mut self.base
    }
}

impl MSSOTLPlatoonPolicy {
    /// C++: `void MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(const Parameterised::Map &)`
    // MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Platoon", parameters) {
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new(parameters: &ParameterisedMap) -> Self {
        let mut this = MSSOTLPlatoonPolicy {
            base: MSSOTLPolicy::new("Platoon".to_string(), parameters),
            base2: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            base3: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        this.init();
        this
    }

    /// C++: `void MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(MSSOTLPolicyDesirability *)`
    // MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(MSSOTLPolicyDesirability* desirabilityAlgorithm) :
    //     MSSOTLPolicy("Platoon", desirabilityAlgorithm) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("PLATOON");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_2(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>) -> Self {
        let mut this = MSSOTLPlatoonPolicy {
            base: MSSOTLPolicy::new_2("Platoon".to_string(), desirabilityAlgorithm),
            base2: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            base3: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        let mut desirability = this.get_desirability_algorithm();
        desirability.m().set_key_prefix("PLATOON".to_string());
        this.init();
        this
    }

    /// C++: `void MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(MSSOTLPolicyDesirability *, const Parameterised::Map &)`
    // MSSOTLPlatoonPolicy::MSSOTLPlatoonPolicy(MSSOTLPolicyDesirability* desirabilityAlgorithm,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLPolicy("Platoon", desirabilityAlgorithm, parameters) {
    //     getDesirabilityAlgorithm()->setKeyPrefix("PLATOON");
    //     init();
    // }
    #[allow(unused_variables)]
    pub fn new_3(desirabilityAlgorithm: Ptr<MSSOTLPolicyDesirability>, parameters: &ParameterisedMap) -> Self {
        let mut this = MSSOTLPlatoonPolicy {
            base: MSSOTLPolicy::new_3("Platoon".to_string(), desirabilityAlgorithm, parameters),
            base2: SigmoidLogic { m_useSigmoid: false, m_k: 0.0, m_prefix: String::new() },
            base3: PushButtonLogic { m_pushButtonScaleFactor: 0.0, m_prefix: String::new() },
        };
        let mut desirability = this.get_desirability_algorithm();
        desirability.m().set_key_prefix("PLATOON".to_string());
        this.init();
        this
    }

    /// C++: `bool MSSOTLPlatoonPolicy::canRelease(SUMOTime, bool, bool, const MSPhaseDefinition *, int)` [virtual]
    // bool MSSOTLPlatoonPolicy::canRelease(SUMOTime elapsed, bool thresholdPassed, bool pushButtonPressed,
    //                                      const MSPhaseDefinition* stage, int vehicleCount) {
    // //  DBG(std::ostringstream str; str << "invoked MSTLPlatoonPolicy::canRelease()"; WRITE_MESSAGE(str.str()););
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "MSSOTLPlatoonPolicy::canRelease elapsed " << elapsed << " threshold " << thresholdPassed << " pushbutton " << pushButtonPressed << " vcount " << vehicleCount
    //         << " minD " << stage->minDuration << " maxD " << stage->maxDuration;
    //     str << " will return " << ((thresholdPassed && ((vehicleCount == 0) || (elapsed >= stage->maxDuration))) ? "true" : "false");
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     if (elapsed >= stage->minDuration) {
    //         if (pushButtonLogic(elapsed, pushButtonPressed, stage)) {
    //             return true;
    //         }
    //         if (thresholdPassed) {
    //             //If there are no other vehicles approaching green lights
    //             //or the declared maximum duration has been reached
    //             return ((vehicleCount == 0) || (elapsed >= stage->maxDuration));
    //         } else {
    //             if (sigmoidLogic(elapsed, stage, vehicleCount)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_release(&mut self, elapsed: SUMOTime, thresholdPassed: bool, pushButtonPressed: bool, stage: Ptr<MSPhaseDefinition>, vehicleCount: i32) -> bool {
        if elapsed >= stage.minDuration {
            if self.base3.push_button_logic(elapsed, pushButtonPressed, stage) {
                return true;
            }
            if thresholdPassed {
                // If there are no other vehicles approaching green lights
                // or the declared maximum duration has been reached
                return (vehicleCount == 0) || (elapsed >= stage.maxDuration);
            } else {
                if self.base2.sigmoid_logic(elapsed, stage, vehicleCount) {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `void MSSOTLPlatoonPolicy::init()` [virtual, protected]
    // void MSSOTLPlatoonPolicy::init() {
    //     SigmoidLogic::init("MSSOTLPlatoonPolicy", this);
    //     PushButtonLogic::init("MSSOTLPlatoonPolicy", this);
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        self.base2.init("MSSOTLPlatoonPolicy".to_string(), Ptr::from_mut(&mut self.base.base));
        self.base3.init("MSSOTLPlatoonPolicy".to_string(), Ptr::from_mut(&mut self.base.base));
    }

}

