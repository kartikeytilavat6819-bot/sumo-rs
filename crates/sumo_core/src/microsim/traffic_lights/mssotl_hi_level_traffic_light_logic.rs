//! Port of microsim/traffic_lights/MSSOTLHiLevelTrafficLightLogic.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A self-organizing high-level traffic light logic
/// C++ class `MSSOTLHiLevelTrafficLightLogic` (microsim/traffic_lights/MSSOTLHiLevelTrafficLightLogic.h)
#[repr(C)]
pub struct MSSOTLHiLevelTrafficLightLogic {
    /// C++ base class `MSSOTLTrafficLightLogic`
    pub base: MSSOTLTrafficLightLogic,
    pub myPolicies: Vec<Ptr<MSSOTLPolicy>>,
    pub myCurrentPolicy: Ptr<MSSOTLPolicy>,
}

impl std::ops::Deref for MSSOTLHiLevelTrafficLightLogic {
    type Target = MSSOTLTrafficLightLogic;
    fn deref(&self) -> &MSSOTLTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLHiLevelTrafficLightLogic {
    fn deref_mut(&mut self) -> &mut MSSOTLTrafficLightLogic {
        &mut self.base
    }
}

impl MSSOTLHiLevelTrafficLightLogic {
    /// C++: `void MSSOTLHiLevelTrafficLightLogic::MSSOTLHiLevelTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &)`
    /// Constructor without sensors passed
    // MSSOTLHiLevelTrafficLightLogic::MSSOTLHiLevelTrafficLightLogic(MSTLLogicControl& tlcontrol,
    //         const std::string& id, const std::string& programID, const TrafficLightType logicType, const Phases& phases,
    //         int step, SUMOTime delay,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, logicType, phases, step, delay,
    //                             parameters) {
    //     // Setting default values
    //
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap) -> Self {
        Self {
            base: MSSOTLTrafficLightLogic::new(tlcontrol, id, programID, logicType, phases, step, delay, parameters),
            myPolicies: Vec::new(),
            myCurrentPolicy: Ptr::null(),
        }
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::MSSOTLHiLevelTrafficLightLogic(MSTLLogicControl &, const std::string &, const std::string &, const TrafficLightType, const Phases &, int, SUMOTime, const Parameterised::Map &, MSSOTLSensors *)`
    /// Constructor with sensors passed
    // MSSOTLHiLevelTrafficLightLogic::MSSOTLHiLevelTrafficLightLogic(MSTLLogicControl& tlcontrol,
    //         const std::string& id, const std::string& programID, const TrafficLightType logicType, const Phases& phases,
    //         int step, SUMOTime delay,
    //         const Parameterised::Map& parameters,
    //         MSSOTLSensors* sensors) :
    //     MSSOTLTrafficLightLogic(tlcontrol, id, programID, logicType, phases, step, delay,
    //                             parameters, sensors) {
    //     // Setting default values
    //
    // }
    #[allow(unused_variables)]
    pub fn new_2(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, logicType: TrafficLightType, phases: &MSTrafficLightLogic_Phases, step: i32, delay: SUMOTime, parameters: &ParameterisedMap, sensors: Ptr<MSSOTLSensors>) -> Self {
        Self {
            base: MSSOTLTrafficLightLogic::new_2(tlcontrol, id, programID, logicType, phases, step, delay, parameters, sensors),
            myPolicies: Vec::new(),
            myCurrentPolicy: Ptr::null(),
        }
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::~MSSOTLHiLevelTrafficLightLogic() noexcept` [virtual]
    // MSSOTLHiLevelTrafficLightLogic::~MSSOTLHiLevelTrafficLightLogic() {
    //     for (MSSOTLPolicy* const policy : myPolicies) {
    //         delete policy;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for policy in self.myPolicies.iter() {
            // C++ `delete policy`: policies are owned by this logic
            unsafe { drop(Box::from_raw((*policy).as_raw())) };
        }
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the tls
    // void MSSOTLHiLevelTrafficLightLogic::init(NLDetectorBuilder& nb) {
    //     MSSOTLTrafficLightLogic::init(nb);
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        self.base.init(nb);
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::addPolicy(MSSOTLPolicy *)`
    // void MSSOTLHiLevelTrafficLightLogic::addPolicy(MSSOTLPolicy* policy) {
    //     myPolicies.push_back(policy);
    // }
    #[allow(unused_variables)]
    pub fn add_policy(&mut self, policy: Ptr<MSSOTLPolicy>) {
        self.myPolicies.push(policy);
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::activate(MSSOTLPolicy *)`
    // void MSSOTLHiLevelTrafficLightLogic::activate(MSSOTLPolicy* policy) {
    //     myCurrentPolicy = policy;
    // }
    #[allow(unused_variables)]
    pub fn activate(&mut self, policy: Ptr<MSSOTLPolicy>) {
        self.myCurrentPolicy = policy;
    }

    /// C++: `void MSSOTLHiLevelTrafficLightLogic::decidePolicy()` [virtual, pure virtual, protected]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn decide_policy(&mut self) {
        // shortcut: pure virtual with no C++ body; a subclass must override it, so calling the base is a bug (as in C++)
        panic!("pure virtual method MSSOTLHiLevelTrafficLightLogic::decidePolicy called")
    }

}

