//! Port of microsim/traffic_lights/MSSOTLPolicy5DStimulus.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the stimulus of a MSSOTLPolicy when used in combination with a high level policy. The stimulus function is calculated as follows: stimulus = cox * exp(-pow(pheroIn - offsetIn, 2)/divisor -pow(pheroOut - offsetOut, 2)/divisor)
/// C++ class `MSSOTLPolicy5DStimulus` (microsim/traffic_lights/MSSOTLPolicy5DStimulus.h)
#[repr(C)]
pub struct MSSOTLPolicy5DStimulus {
    /// C++ base class `MSSOTLPolicyDesirability`
    pub base: MSSOTLPolicyDesirability,
    pub stimCoxDVal: f64,
    pub stimOffsetInDVal: f64,
    pub stimOffsetOutDVal: f64,
    pub stimOffsetDispersionInDVal: f64,
    pub stimOffsetDispersionOutDVal: f64,
    pub stimDivInDVal: f64,
    pub stimDivOutDVal: f64,
    pub stimDivDispersionInDVal: f64,
    pub stimDivDispersionOutDVal: f64,
    pub stimCoxExpInDVal: f64,
    pub stimCoxExpOutDVal: f64,
    pub stimCoxExpDispersionInDVal: f64,
    pub stimCoxExpDispersionOutDVal: f64,
}

impl std::ops::Deref for MSSOTLPolicy5DStimulus {
    type Target = MSSOTLPolicyDesirability;
    fn deref(&self) -> &MSSOTLPolicyDesirability {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicy5DStimulus {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicyDesirability {
        &mut self.base
    }
}

impl MSSOTLPolicy5DStimulus {
    /// C++: `void MSSOTLPolicy5DStimulus::MSSOTLPolicy5DStimulus(std::string, const Parameterised::Map &)`
    // MSSOTLPolicy5DStimulus::MSSOTLPolicy5DStimulus(std::string keyPrefix,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLPolicyDesirability(keyPrefix, parameters) {
    //
    //     stimCoxDVal = 1;
    //     stimOffsetInDVal = 1;
    //     stimOffsetOutDVal = 1;
    //     stimOffsetDispersionInDVal = 1;
    //     stimOffsetDispersionOutDVal = 1;
    //     stimDivInDVal = 1;
    //     stimDivOutDVal = 1;
    //     stimDivDispersionInDVal = 1;
    //     stimDivDispersionOutDVal = 1;
    //     stimCoxExpInDVal = 0;
    //     stimCoxExpOutDVal = 0;
    //     stimCoxExpDispersionInDVal = 0;
    //     stimCoxExpDispersionOutDVal = 0;
    // }
    #[allow(unused_variables)]
    pub fn new(keyPrefix: String, parameters: &ParameterisedMap) -> Self {
        Self {
            base: MSSOTLPolicyDesirability::new(keyPrefix, parameters),
            stimCoxDVal: 1.0,
            stimOffsetInDVal: 1.0,
            stimOffsetOutDVal: 1.0,
            stimOffsetDispersionInDVal: 1.0,
            stimOffsetDispersionOutDVal: 1.0,
            stimDivInDVal: 1.0,
            stimDivOutDVal: 1.0,
            stimDivDispersionInDVal: 1.0,
            stimDivDispersionOutDVal: 1.0,
            stimCoxExpInDVal: 0.0,
            stimCoxExpOutDVal: 0.0,
            stimCoxExpDispersionInDVal: 0.0,
            stimCoxExpDispersionOutDVal: 0.0,
        }
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimCox()`
    // double getStimCox() {
    //         std::string key = getKeyPrefix() + "_STIM_COX";
    //         return getDouble(key, stimCoxDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_cox(&mut self) -> f64 {
        let key = format!("{}_STIM_COX", self.get_key_prefix());
        self.get_double(&key, self.stimCoxDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimCoxDefVal(double)`
    // void setStimCoxDefVal(double defVal) {
    //         stimCoxDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_cox_def_val(&mut self, defVal: f64) {
        self.stimCoxDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimOffsetIn()`
    // double getStimOffsetIn() {
    //         std::string key = getKeyPrefix() + "_STIM_OFFSET_IN";
    //         return getDouble(key, stimOffsetInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_offset_in(&mut self) -> f64 {
        let key = format!("{}_STIM_OFFSET_IN", self.get_key_prefix());
        self.get_double(&key, self.stimOffsetInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimOffsetInDefVal(double)`
    // void setStimOffsetInDefVal(double defVal) {
    //         stimOffsetInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_offset_in_def_val(&mut self, defVal: f64) {
        self.stimOffsetInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimOffsetOut()`
    // double getStimOffsetOut() {
    //         std::string key = getKeyPrefix() + "_STIM_OFFSET_OUT";
    //         return getDouble(key, stimOffsetOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_offset_out(&mut self) -> f64 {
        let key = format!("{}_STIM_OFFSET_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimOffsetOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimOffsetOutDefVal(double)`
    // void setStimOffsetOutDefVal(double defVal) {
    //         stimOffsetOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_offset_out_def_val(&mut self, defVal: f64) {
        self.stimOffsetOutDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimOffsetDispersionIn()`
    // double getStimOffsetDispersionIn() {
    //         std::string key = getKeyPrefix() + "_STIM_OFFSET_DISPERSION_IN";
    //         return getDouble(key, stimOffsetDispersionInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_offset_dispersion_in(&mut self) -> f64 {
        let key = format!("{}_STIM_OFFSET_DISPERSION_IN", self.get_key_prefix());
        self.get_double(&key, self.stimOffsetDispersionInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimOffsetDispersionInDefVal(double)`
    // void setStimOffsetDispersionInDefVal(double defVal) {
    //         stimOffsetDispersionInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_offset_dispersion_in_def_val(&mut self, defVal: f64) {
        self.stimOffsetDispersionInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimOffsetDispersionOut()`
    // double getStimOffsetDispersionOut() {
    //         std::string key = getKeyPrefix() + "_STIM_OFFSET_DISPERSION_OUT";
    //         return getDouble(key, stimOffsetDispersionOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_offset_dispersion_out(&mut self) -> f64 {
        let key = format!("{}_STIM_OFFSET_DISPERSION_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimOffsetDispersionOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimOffsetDispersionOutDefVal(double)`
    // void setStimOffsetDispersionOutDefVal(double defVal) {
    //         stimOffsetDispersionOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_offset_dispersion_out_def_val(&mut self, defVal: f64) {
        self.stimOffsetDispersionOutDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimDivisorIn()`
    // double getStimDivisorIn() {
    //         std::string key = getKeyPrefix() + "_STIM_DIVISOR_IN";
    //         return getDouble(key, stimDivInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_divisor_in(&mut self) -> f64 {
        let key = format!("{}_STIM_DIVISOR_IN", self.get_key_prefix());
        self.get_double(&key, self.stimDivInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimDivisorInDefVal(double)`
    // void setStimDivisorInDefVal(double defVal) {
    //         stimDivInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_divisor_in_def_val(&mut self, defVal: f64) {
        self.stimDivInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimDivisorOut()`
    // double getStimDivisorOut() {
    //         std::string key = getKeyPrefix() + "_STIM_DIVISOR_OUT";
    //         return getDouble(key, stimDivOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_divisor_out(&mut self) -> f64 {
        let key = format!("{}_STIM_DIVISOR_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimDivOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimDivisorOutDefVal(double)`
    // void setStimDivisorOutDefVal(double defVal) {
    //         stimDivOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_divisor_out_def_val(&mut self, defVal: f64) {
        self.stimDivOutDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimDivisorDispersionIn()`
    // double getStimDivisorDispersionIn() {
    //         std::string key = getKeyPrefix() + "_STIM_DIVISOR_DISPERSION_IN";
    //         return getDouble(key, stimDivDispersionInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_divisor_dispersion_in(&mut self) -> f64 {
        let key = format!("{}_STIM_DIVISOR_DISPERSION_IN", self.get_key_prefix());
        self.get_double(&key, self.stimDivDispersionInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimDivisorDispersionInDefVal(double)`
    // void setStimDivisorDispersionInDefVal(double defVal) {
    //         stimDivDispersionInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_divisor_dispersion_in_def_val(&mut self, defVal: f64) {
        self.stimDivDispersionInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimDivisorDispersionOut()`
    // double getStimDivisorDispersionOut() {
    //         std::string key = getKeyPrefix() + "_STIM_DIVISOR_DISPERSION_OUT";
    //         return getDouble(key, stimDivDispersionOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_divisor_dispersion_out(&mut self) -> f64 {
        let key = format!("{}_STIM_DIVISOR_DISPERSION_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimDivDispersionOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimDivisorDispersionOutDefVal(double)`
    // void setStimDivisorDispersionOutDefVal(double defVal) {
    //         stimDivDispersionOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_divisor_dispersion_out_def_val(&mut self, defVal: f64) {
        self.stimDivDispersionOutDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimCoxExpIn()`
    // double getStimCoxExpIn() {
    //         std::string key = getKeyPrefix() + "_STIM_COX_EXP_IN";
    //         return getDouble(key, stimCoxExpInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_cox_exp_in(&mut self) -> f64 {
        let key = format!("{}_STIM_COX_EXP_IN", self.get_key_prefix());
        self.get_double(&key, self.stimCoxExpInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimCoxExpInDefVal(double)`
    // void setStimCoxExpInDefVal(double defVal) {
    //         stimCoxExpInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_cox_exp_in_def_val(&mut self, defVal: f64) {
        self.stimCoxExpInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimCoxExpOut()`
    // double getStimCoxExpOut() {
    //         std::string key = getKeyPrefix() + "_STIM_COX_EXP_OUT";
    //         return getDouble(key, stimCoxExpOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_cox_exp_out(&mut self) -> f64 {
        let key = format!("{}_STIM_COX_EXP_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimCoxExpOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimCoxExpOutDefVal(double)`
    // void setStimCoxExpOutDefVal(double defVal) {
    //         stimCoxExpOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_cox_exp_out_def_val(&mut self, defVal: f64) {
        self.stimCoxExpOutDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimCoxExpDispersionIn()`
    // double getStimCoxExpDispersionIn() {
    //         std::string key = getKeyPrefix() + "_STIM_COX_EXP_DISPERSION_IN";
    //         return getDouble(key, stimCoxExpDispersionInDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_cox_exp_dispersion_in(&mut self) -> f64 {
        let key = format!("{}_STIM_COX_EXP_DISPERSION_IN", self.get_key_prefix());
        self.get_double(&key, self.stimCoxExpDispersionInDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimCoxExpDispersionInDefVal(double)`
    // void setStimCoxExpDispersionInDefVal(double defVal) {
    //         stimCoxExpDispersionInDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_cox_exp_dispersion_in_def_val(&mut self, defVal: f64) {
        self.stimCoxExpDispersionInDVal = defVal;
    }

    /// C++: `double MSSOTLPolicy5DStimulus::getStimCoxExpDispersionOut()`
    // double getStimCoxExpDispersionOut() {
    //         std::string key = getKeyPrefix() + "_STIM_COX_EXP_DISPERSION_OUT";
    //         return getDouble(key, stimCoxExpDispersionOutDVal);
    //     }
    #[allow(unused_variables)]
    pub fn get_stim_cox_exp_dispersion_out(&mut self) -> f64 {
        let key = format!("{}_STIM_COX_EXP_DISPERSION_OUT", self.get_key_prefix());
        self.get_double(&key, self.stimCoxExpDispersionOutDVal)
    }

    /// C++: `void MSSOTLPolicy5DStimulus::setStimCoxExpDispersionOutDefVal(double)`
    // void setStimCoxExpDispersionOutDefVal(double defVal) {
    //         stimCoxExpDispersionOutDVal = defVal;
    //     }
    #[allow(unused_variables)]
    pub fn set_stim_cox_exp_dispersion_out_def_val(&mut self, defVal: f64) {
        self.stimCoxExpDispersionOutDVal = defVal;
    }

    /// C++: `std::string MSSOTLPolicy5DStimulus::getMessage()` [virtual]
    // std::string  MSSOTLPolicy5DStimulus::getMessage() {
    //     std::ostringstream _str;
    //     _str << " stimCox " << getStimCox()
    //          << " StimOffsetIn " << getStimOffsetIn()
    //          << " StimOffsetOut " << getStimOffsetOut()
    //          << " StimDivisorIn " << getStimDivisorIn()
    //          << " StimDivisorOut " << getStimDivisorOut()
    //          << " StimCoxExpIn " << getStimCoxExpIn()
    //          << " StimCoxExpOut " << getStimCoxExpOut()
    //          << " .";
    //     return _str.str();
    // }
    #[allow(unused_variables)]
    pub fn get_message(&mut self) -> String {
        let stimCox = utils::common::to_string::f64_to_string_default(self.get_stim_cox());
        let offsetIn = utils::common::to_string::f64_to_string_default(self.get_stim_offset_in());
        let offsetOut = utils::common::to_string::f64_to_string_default(self.get_stim_offset_out());
        let divisorIn = utils::common::to_string::f64_to_string_default(self.get_stim_divisor_in());
        let divisorOut = utils::common::to_string::f64_to_string_default(self.get_stim_divisor_out());
        let coxExpIn = utils::common::to_string::f64_to_string_default(self.get_stim_cox_exp_in());
        let coxExpOut = utils::common::to_string::f64_to_string_default(self.get_stim_cox_exp_out());
        format!(" stimCox {} StimOffsetIn {} StimOffsetOut {} StimDivisorIn {} StimDivisorOut {} StimCoxExpIn {} StimCoxExpOut {} .", stimCox, offsetIn, offsetOut, divisorIn, divisorOut, coxExpIn, coxExpOut)
    }

    /// C++: `double MSSOTLPolicy5DStimulus::computeDesirability(double, double)` [virtual]
    /// Computes stimulus function stimulus = cox * exp(-pow(pheroIn - offsetIn, 2)/divisor -pow(pheroOut - offsetOut, 2)/divisor);
    // double MSSOTLPolicy5DStimulus::computeDesirability(double vehInMeasure, double vehOutMeasure) {
    //
    //     return computeDesirability(vehInMeasure, vehOutMeasure, 0, 0);
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability(&mut self, vehInMeasure: f64, vehOutMeasure: f64) -> f64 {
        self.compute_desirability_2(vehInMeasure, vehOutMeasure, 0.0, 0.0)
    }

    /// C++: `double MSSOTLPolicy5DStimulus::computeDesirability(double, double, double, double)` [virtual]
    // double MSSOTLPolicy5DStimulus::computeDesirability(double vehInMeasure, double vehOutMeasure, double vehInDispersionMeasure, double vehOutDispersionMeasure) {
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "cox=" << getStimCox() << ", cox_exp_in=" << getStimCoxExpIn() << ", cox_exp_out=" << getStimCoxExpOut()
    //         << ", off_in=" << getStimOffsetIn() << ", off_out=" << getStimOffsetOut() << ", div_in=" << getStimDivisorIn() << ", div_out=" << getStimDivisorOut();
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     //		it seems to be not enough, a strange segmentation fault appears...
    //     //	 if((getStimCoxExpIn()!=0.0 && getStimDivisorIn()==0.0)||(getStimCoxExpOut()!=0.0 && getStimDivisorOut()==0.0)){
    //     if (getStimDivisorIn() == 0 || getStimDivisorOut() == 0) {
    //         std::ostringstream errorMessage;
    //         errorMessage << "INCORRECT VALUES" << "\nStimCoxExpIn="
    //                      << getStimCoxExpIn() << ", StimDivisorIn=" << getStimDivisorIn()
    //                      << ", StimCoxExpOut=" << getStimCoxExpOut()
    //                      << ", StimDivisorOut=" << getStimDivisorOut();
    //         WRITE_ERROR(errorMessage.str());
    //         assert(-1);
    //         return -1;
    //     } else {
    //         double stimulus = getStimCox()
    //                           * exp(
    //                               -getStimCoxExpIn()
    //                               * pow(vehInMeasure - getStimOffsetIn(), 2)
    //                               / getStimDivisorIn()
    //                               - getStimCoxExpOut()
    //                               * pow(vehOutMeasure - getStimOffsetOut(), 2)
    //                               / getStimDivisorOut()
    //                               - getStimCoxExpDispersionIn()
    //                               * pow(vehInDispersionMeasure - getStimOffsetDispersionIn(), 2)
    //                               / getStimDivisorDispersionIn()
    //                               - getStimCoxExpDispersionOut()
    //                               * pow(vehOutDispersionMeasure - getStimOffsetDispersionOut(), 2)
    //                               / getStimDivisorDispersionOut()
    //
    //                           );
    //         return stimulus;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability_2(&mut self, vehInMeasure: f64, vehOutMeasure: f64, vehInDispersionMeasure: f64, vehOutDispersionMeasure: f64) -> f64 {
        //it seems to be not enough, a strange segmentation fault appears...
        if self.get_stim_divisor_in() == 0.0 || self.get_stim_divisor_out() == 0.0 {
            let errorMessage = format!("INCORRECT VALUES\nStimCoxExpIn={}, StimDivisorIn={}, StimCoxExpOut={}, StimDivisorOut={}", utils::common::to_string::f64_to_string_default(self.get_stim_cox_exp_in()), utils::common::to_string::f64_to_string_default(self.get_stim_divisor_in()), utils::common::to_string::f64_to_string_default(self.get_stim_cox_exp_out()), utils::common::to_string::f64_to_string_default(self.get_stim_divisor_out()));
            utils::common::msg_handler::write_error(&errorMessage);
            //C++ assert(-1) is always true, so it has no effect
            return -1.0;
        } else {
            let stimulus = self.get_stim_cox()
                * (-self.get_stim_cox_exp_in()
                    * (vehInMeasure - self.get_stim_offset_in()).powf(2.0)
                    / self.get_stim_divisor_in()
                    - self.get_stim_cox_exp_out()
                        * (vehOutMeasure - self.get_stim_offset_out()).powf(2.0)
                        / self.get_stim_divisor_out()
                    - self.get_stim_cox_exp_dispersion_in()
                        * (vehInDispersionMeasure - self.get_stim_offset_dispersion_in()).powf(2.0)
                        / self.get_stim_divisor_dispersion_in()
                    - self.get_stim_cox_exp_dispersion_out()
                        * (vehOutDispersionMeasure - self.get_stim_offset_dispersion_out()).powf(2.0)
                        / self.get_stim_divisor_dispersion_out())
                    .exp();
            return stimulus;
        }
    }

}

