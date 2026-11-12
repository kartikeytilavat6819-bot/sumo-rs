//! Port of microsim/traffic_lights/MSSOTLPolicy3DStimulus.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the stimulus of a MSSOTLPolicy when used in combination with a high level policy. The stimulus function is calculated as follows: stimulus = cox * exp(-pow(pheroIn - offsetIn, 2)/divisor -pow(pheroOut - offsetOut, 2)/divisor)
/// C++ class `MSSOTLPolicy3DStimulus` (microsim/traffic_lights/MSSOTLPolicy3DStimulus.h)
#[repr(C)]
pub struct MSSOTLPolicy3DStimulus {
    /// C++ base class `MSSOTLPolicy5DStimulus`
    pub base: MSSOTLPolicy5DStimulus,
}

impl std::ops::Deref for MSSOTLPolicy3DStimulus {
    type Target = MSSOTLPolicy5DStimulus;
    fn deref(&self) -> &MSSOTLPolicy5DStimulus {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicy3DStimulus {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicy5DStimulus {
        &mut self.base
    }
}

impl MSSOTLPolicy3DStimulus {
    /// C++: `void MSSOTLPolicy3DStimulus::MSSOTLPolicy3DStimulus(std::string, const Parameterised::Map &)`
    // MSSOTLPolicy3DStimulus::MSSOTLPolicy3DStimulus(std::string keyPrefix,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLPolicy5DStimulus(keyPrefix, parameters) {
    //
    //     setStimCoxExpDispersionInDefVal(0);
    //     setStimCoxExpDispersionOutDefVal(0);
    // }
    #[allow(unused_variables)]
    pub fn new(keyPrefix: String, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        let mut base = MSSOTLPolicy5DStimulus::new(keyPrefix, parameters);
        base.set_stim_cox_exp_dispersion_in_def_val(0.);
        base.set_stim_cox_exp_dispersion_out_def_val(0.);
        Self { base }
    }

}

