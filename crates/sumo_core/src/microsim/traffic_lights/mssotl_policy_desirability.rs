//! Port of microsim/traffic_lights/MSSOTLPolicyDesirability.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the desirability algorithm of a MSSOTLPolicy when used in combination with a high level policy
/// C++ class `MSSOTLPolicyDesirability` (microsim/traffic_lights/MSSOTLPolicyDesirability.h)
pub struct MSSOTLPolicyDesirability {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub myKeyPrefix: String,
}

impl std::ops::Deref for MSSOTLPolicyDesirability {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicyDesirability {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl MSSOTLPolicyDesirability {
    /// C++: `void MSSOTLPolicyDesirability::MSSOTLPolicyDesirability(std::string, const Parameterised::Map &)`
    // MSSOTLPolicyDesirability::MSSOTLPolicyDesirability(
    //     std::string keyPrefix,
    //     const Parameterised::Map& parameters) :
    //     Parameterised(parameters), myKeyPrefix(keyPrefix) {
    // }
    #[allow(unused_variables)]
    pub fn new(keyPrefix: String, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSSOTLPolicyDesirability::MSSOTLPolicyDesirability")
    }

    /// C++: `void MSSOTLPolicyDesirability::~MSSOTLPolicyDesirability() noexcept` [virtual]
    // MSSOTLPolicyDesirability::~MSSOTLPolicyDesirability() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSSOTLPolicyDesirability::~MSSOTLPolicyDesirability")
    }

    /// C++: `double MSSOTLPolicyDesirability::computeDesirability(double, double)` [virtual, pure virtual]
    /// Calculates the desirability of the policy
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute_desirability(&mut self, vehInMeasure: f64, vehOutMeasure: f64) -> f64 {
        todo!("port MSSOTLPolicyDesirability::computeDesirability")
    }

    /// C++: `double MSSOTLPolicyDesirability::computeDesirability(double, double, double, double)` [virtual, pure virtual]
    /// Calculates the desirability of the policy
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute_desirability_2(&mut self, vehInMeasure: f64, vehOutMeasure: f64, vehInDispersionMeasure: f64, vehOutDispersionMeasure: f64) -> f64 {
        todo!("port MSSOTLPolicyDesirability::computeDesirability")
    }

    /// C++: `std::string MSSOTLPolicyDesirability::getMessage()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_message(&mut self) -> String {
        todo!("port MSSOTLPolicyDesirability::getMessage")
    }

    /// C++: `void MSSOTLPolicyDesirability::setKeyPrefix(std::string)`
    // void setKeyPrefix(std::string val) {
    //         myKeyPrefix = val;
    //     }
    #[allow(unused_variables)]
    pub fn set_key_prefix(&mut self, val: String) {
        todo!("port MSSOTLPolicyDesirability::setKeyPrefix")
    }

    /// C++: `std::string MSSOTLPolicyDesirability::getKeyPrefix()`
    // std::string getKeyPrefix() {
    //         return myKeyPrefix;
    //     }
    #[allow(unused_variables)]
    pub fn get_key_prefix(&mut self) -> String {
        todo!("port MSSOTLPolicyDesirability::getKeyPrefix")
    }

}

