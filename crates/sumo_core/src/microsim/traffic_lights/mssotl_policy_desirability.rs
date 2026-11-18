//! Port of microsim/traffic_lights/MSSOTLPolicyDesirability.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the desirability algorithm of a MSSOTLPolicy when used in combination with a high level policy
/// C++ class `MSSOTLPolicyDesirability` (microsim/traffic_lights/MSSOTLPolicyDesirability.h)
#[repr(C)]
pub struct MSSOTLPolicyDesirability {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    pub myKeyPrefix: String,
    /// virtual dispatch tag: true for MSSOTLPolicy5DFamilyStimulus, false for MSSOTLPolicy5DStimulus (and its MSSOTLPolicy3DStimulus subclass)
    pub myIsFamily: bool,
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
    pub fn new(keyPrefix: String, parameters: &ParameterisedMap) -> Self {
        MSSOTLPolicyDesirability {
            base: Parameterised::from_map(parameters.clone()),
            myKeyPrefix: keyPrefix,
            myIsFamily: false,
        }
    }

    /// C++: `void MSSOTLPolicyDesirability::~MSSOTLPolicyDesirability() noexcept` [virtual]
    // MSSOTLPolicyDesirability::~MSSOTLPolicyDesirability() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `double MSSOTLPolicyDesirability::computeDesirability(double, double)` [virtual, pure virtual]
    /// Calculates the desirability of the policy
    // (pure virtual: no C++ body; subclasses implement it)
    // dispatched to the most derived override (the only concrete subclasses are the 5D stimulus and the 5D family)
    #[allow(unused_variables)]
    pub fn compute_desirability(&mut self, vehInMeasure: f64, vehOutMeasure: f64) -> f64 {
        let me = Ptr::from_mut(self);
        if self.myIsFamily {
            me.cast::<MSSOTLPolicy5DFamilyStimulus>().m().compute_desirability(vehInMeasure, vehOutMeasure)
        } else {
            me.cast::<MSSOTLPolicy5DStimulus>().m().compute_desirability(vehInMeasure, vehOutMeasure)
        }
    }

    /// C++: `double MSSOTLPolicyDesirability::computeDesirability(double, double, double, double)` [virtual, pure virtual]
    /// Calculates the desirability of the policy
    // (pure virtual: no C++ body; subclasses implement it)
    // dispatched to the most derived override
    #[allow(unused_variables)]
    pub fn compute_desirability_2(&mut self, vehInMeasure: f64, vehOutMeasure: f64, vehInDispersionMeasure: f64, vehOutDispersionMeasure: f64) -> f64 {
        let me = Ptr::from_mut(self);
        if self.myIsFamily {
            me.cast::<MSSOTLPolicy5DFamilyStimulus>().m().compute_desirability_2(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure)
        } else {
            me.cast::<MSSOTLPolicy5DStimulus>().m().compute_desirability_2(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure)
        }
    }

    /// C++: `std::string MSSOTLPolicyDesirability::getMessage()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    // dispatched to the most derived override
    #[allow(unused_variables)]
    pub fn get_message(&mut self) -> String {
        let me = Ptr::from_mut(self);
        if self.myIsFamily {
            me.cast::<MSSOTLPolicy5DFamilyStimulus>().m().get_message()
        } else {
            me.cast::<MSSOTLPolicy5DStimulus>().m().get_message()
        }
    }

    /// C++: `void MSSOTLPolicyDesirability::setKeyPrefix(std::string)`
    // void setKeyPrefix(std::string val) {
    //         myKeyPrefix = val;
    //     }
    #[allow(unused_variables)]
    pub fn set_key_prefix(&mut self, val: String) {
        self.myKeyPrefix = val;
    }

    /// C++: `std::string MSSOTLPolicyDesirability::getKeyPrefix()`
    // std::string getKeyPrefix() {
    //         return myKeyPrefix;
    //     }
    #[allow(unused_variables)]
    pub fn get_key_prefix(&mut self) -> String {
        self.myKeyPrefix.clone()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    // C++ calls computeDesirability/getMessage through MSSOTLPolicyDesirability*: the base must reach the subclass override
    #[test]
    fn base_pointer_dispatches_to_stimulus_and_family() {
        let params = ParameterisedMap::new();
        let mut stim = MSSOTLPolicy5DStimulus::new("PLATOON".to_string(), &params);
        let direct = stim.compute_desirability_2(3., 2., 0.5, 0.25);
        let base = Ptr::from_mut(&mut stim).cast::<MSSOTLPolicyDesirability>();
        assert!(!base.myIsFamily);
        assert_eq!(base.m().compute_desirability_2(3., 2., 0.5, 0.25), direct);
        assert_eq!(base.m().get_message(), stim.get_message());

        let mut fam = MSSOTLPolicy5DFamilyStimulus::new("PLATOON".to_string(), &params);
        let direct = fam.compute_desirability(3., 2.);
        let base = Ptr::from_mut(&mut fam).cast::<MSSOTLPolicyDesirability>();
        assert!(base.myIsFamily);
        assert_eq!(base.m().compute_desirability(3., 2.), direct);
        assert_eq!(base.m().get_message(), fam.get_message());
    }
}
