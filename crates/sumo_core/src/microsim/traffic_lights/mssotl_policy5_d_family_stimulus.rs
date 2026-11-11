//! Port of microsim/traffic_lights/MSSOTLPolicy5DFamilyStimulus.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the stimulus of a MSSOTLPolicy when used in combination with a high level policy. The stimulus function is calculated as follows: stimulus = cox * exp(-pow(pheroIn - offsetIn, 2)/divisor -pow(pheroOut - offsetOut, 2)/divisor)
/// C++ class `MSSOTLPolicy5DFamilyStimulus` (microsim/traffic_lights/MSSOTLPolicy5DFamilyStimulus.h)
pub struct MSSOTLPolicy5DFamilyStimulus {
    /// C++ base class `MSSOTLPolicyDesirability`
    pub base: MSSOTLPolicyDesirability,
    pub default_values: Unmapped /* Parameterised::Map */,
    pub params_names: Vec<String>,
    pub family: Vec<Ptr<MSSOTLPolicy5DStimulus>>,
}

impl std::ops::Deref for MSSOTLPolicy5DFamilyStimulus {
    type Target = MSSOTLPolicyDesirability;
    fn deref(&self) -> &MSSOTLPolicyDesirability {
        &self.base
    }
}
impl std::ops::DerefMut for MSSOTLPolicy5DFamilyStimulus {
    fn deref_mut(&mut self) -> &mut MSSOTLPolicyDesirability {
        &mut self.base
    }
}

impl MSSOTLPolicy5DFamilyStimulus {
    /// C++: `void MSSOTLPolicy5DFamilyStimulus::MSSOTLPolicy5DFamilyStimulus(std::string, const Parameterised::Map &)`
    // MSSOTLPolicy5DFamilyStimulus::MSSOTLPolicy5DFamilyStimulus(std::string keyPrefix,
    //         const Parameterised::Map& parameters) :
    //     MSSOTLPolicyDesirability(keyPrefix, parameters) {
    //
    //     default_values["_STIM_COX"] = "1";
    //     default_values["_STIM_OFFSET_IN"] = "1";
    //     default_values["_STIM_OFFSET_OUT"] = "1";
    //     default_values["_STIM_OFFSET_DISPERSION_IN"] = "1";
    //     default_values["_STIM_OFFSET_DISPERSION_OUT"] = "1";
    //     default_values["_STIM_DIVISOR_IN"] = "1";
    //     default_values["_STIM_DIVISOR_OUT"] = "1";
    //     default_values["_STIM_DIVISOR_DISPERSION_IN"] = "1";
    //     default_values["_STIM_DIVISOR_DISPERSION_OUT"] = "1";
    //     default_values["_STIM_COX_EXP_IN"] = "0";
    //     default_values["_STIM_COX_EXP_OUT"] = "0";
    //     default_values["_STIM_COX_EXP_DISPERSION_IN"] = "0";
    //     default_values["_STIM_COX_EXP_DISPERSION_OUT"] = "0";
    //
    //     params_names.push_back("_STIM_COX");
    //     params_names.push_back("_STIM_OFFSET_IN");
    //     params_names.push_back("_STIM_OFFSET_OUT");
    //     params_names.push_back("_STIM_OFFSET_DISPERSION_IN");
    //     params_names.push_back("_STIM_OFFSET_DISPERSION_OUT");
    //     params_names.push_back("_STIM_DIVISOR_IN");
    //     params_names.push_back("_STIM_DIVISOR_OUT");
    //     params_names.push_back("_STIM_DIVISOR_DISPERSION_IN");
    //     params_names.push_back("_STIM_DIVISOR_DISPERSION_OUT");
    //     params_names.push_back("_STIM_COX_EXP_IN");
    //     params_names.push_back("_STIM_COX_EXP_OUT");
    //     params_names.push_back("_STIM_COX_EXP_DISPERSION_IN");
    //     params_names.push_back("_STIM_COX_EXP_DISPERSION_OUT");
    //
    //
    //     int size_family = int(getDouble(keyPrefix + "_SIZE_FAMILY", 1));
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << keyPrefix << "\n" << "size fam" << size_family;
    //     WRITE_MESSAGE(str.str());
    // #endif
    //
    //     std::vector< std::map <std::string, std::string > > sliced_maps;
    //
    //     for (int i = 0; i < size_family; i++) {
    //         sliced_maps.push_back(Parameterised::Map());
    //     }
    //
    //     //For each param list, slice values
    //     for (int i = 0; i < (int)params_names.size(); i ++) {
    //         std::string key = keyPrefix + params_names[i];
    //         std::string param_list = getParameter(key, default_values[params_names[i]]);
    //         std::vector<std::string> tokens = StringTokenizer(param_list, ";").getVector();
    //
    //         for (int token_counter = 0; token_counter < size_family; ++token_counter) {
    //             if (token_counter >= (int)tokens.size()) {
    //                 std::ostringstream errorMessage;
    //                 errorMessage << "Error in " << key << ": not enough tokens.";
    //                 WRITE_ERROR(errorMessage.str());
    //                 assert(-1);
    //             }
    // #ifdef SWARM_DEBUG
    //             std::ostringstream str;
    //             str << "found token " << tokens[token_counter] << " position " << token_counter;
    //             WRITE_MESSAGE(str.str());
    // #endif
    //             sliced_maps[token_counter][key] = tokens[token_counter];
    //         }
    //     }
    //
    //     for (int i = 0; i < size_family; i++) {
    //         Parameterised::Map& ref_map = sliced_maps[i];
    //         family.push_back(new MSSOTLPolicy5DStimulus(keyPrefix, ref_map));
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn new(keyPrefix: String, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSSOTLPolicy5DFamilyStimulus::MSSOTLPolicy5DFamilyStimulus")
    }

    /// C++: `std::string MSSOTLPolicy5DFamilyStimulus::getMessage()` [virtual]
    // std::string MSSOTLPolicy5DFamilyStimulus::getMessage() {
    //     std::ostringstream ot;
    //     for (int i = 0; i < (int)family.size(); i++) {
    //         ot << " gaussian " << i << ":" << family[i]->getMessage();
    //     }
    //     return ot.str();
    // }
    #[allow(unused_variables)]
    pub fn get_message(&mut self) -> String {
        let mut ot = String::new();
        for i in 0..self.family.len() {
            ot += &format!(" gaussian {}:{}", i, self.family[i].m().get_message());
        }
        ot
    }

    /// C++: `std::vector<MSSOTLPolicy5DStimulus *> MSSOTLPolicy5DFamilyStimulus::getFamilies()`
    // std::vector<MSSOTLPolicy5DStimulus*> getFamilies() {
    //         return family;
    //     }
    #[allow(unused_variables)]
    pub fn get_families(&mut self) -> Vec<Ptr<MSSOTLPolicy5DStimulus>> {
        self.family.clone()
    }

    /// C++: `double MSSOTLPolicy5DFamilyStimulus::computeDesirability(double, double)` [virtual]
    // double MSSOTLPolicy5DFamilyStimulus::computeDesirability(double vehInMeasure, double vehOutMeasure) {
    //
    //     return computeDesirability(vehInMeasure, vehOutMeasure, 0, 0);
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability(&mut self, vehInMeasure: f64, vehOutMeasure: f64) -> f64 {
        self.compute_desirability_2(vehInMeasure, vehOutMeasure, 0., 0.)
    }

    /// C++: `double MSSOTLPolicy5DFamilyStimulus::computeDesirability(double, double, double, double)` [virtual]
    // double MSSOTLPolicy5DFamilyStimulus::computeDesirability(double vehInMeasure, double vehOutMeasure, double vehInDispersionMeasure, double vehOutDispersionMeasure) {
    //     /*DBG(
    //     		std::ostringstream str;
    //     		str << "cox=" << getStimCox() << ", cox_exp_in=" << getStimCoxExpIn() << ", cox_exp_out=" << getStimCoxExpOut()
    //     				<< ", off_in=" << getStimOffsetIn() << ", off_out=" << getStimOffsetOut() << ", div_in=" << getStimDivisorIn() << ", div_out=" << getStimDivisorOut(); WRITE_MESSAGE(str.str());)
    //      */
    //     //		it seems to be not enough, a strange segmentation fault appears...
    //     //	 if((getStimCoxExpIn()!=0.0 && getStimDivisorIn()==0.0)||(getStimCoxExpOut()!=0.0 && getStimDivisorOut()==0.0)){
    //
    //     double best_stimulus = -1;
    //     for (std::vector<MSSOTLPolicy5DStimulus*>::const_iterator it  = family.begin(); it != family.end(); it++) {
    //         double temp_stimulus = (*it)->computeDesirability(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure);
    // #ifdef SWARM_DEBUG
    //         std::ostringstream str;
    //         str << "STIMULUS: " << temp_stimulus;
    //         WRITE_MESSAGE(str.str());
    // #endif
    //         if (temp_stimulus > best_stimulus) {
    //             best_stimulus = temp_stimulus;
    //         }
    //     }
    //
    // #ifdef SWARM_DEBUG
    //     std::ostringstream str;
    //     str << "BEST STIMULUS: " << best_stimulus;
    //     WRITE_MESSAGE(str.str());
    // #endif
    //     return best_stimulus;
    // }
    #[allow(unused_variables)]
    pub fn compute_desirability_2(&mut self, vehInMeasure: f64, vehOutMeasure: f64, vehInDispersionMeasure: f64, vehOutDispersionMeasure: f64) -> f64 {
        let mut best_stimulus = -1.;
        for it in self.family.iter() {
            let temp_stimulus = it.m().compute_desirability_2(vehInMeasure, vehOutMeasure, vehInDispersionMeasure, vehOutDispersionMeasure);
            if temp_stimulus > best_stimulus {
                best_stimulus = temp_stimulus;
            }
        }
        best_stimulus
    }

}

