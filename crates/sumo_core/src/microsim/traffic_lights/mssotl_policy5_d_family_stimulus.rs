//! Port of microsim/traffic_lights/MSSOTLPolicy5DFamilyStimulus.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This class determines the stimulus of a MSSOTLPolicy when used in combination with a high level policy. The stimulus function is calculated as follows: stimulus = cox * exp(-pow(pheroIn - offsetIn, 2)/divisor -pow(pheroOut - offsetOut, 2)/divisor)
/// C++ class `MSSOTLPolicy5DFamilyStimulus` (microsim/traffic_lights/MSSOTLPolicy5DFamilyStimulus.h)
#[repr(C)]
pub struct MSSOTLPolicy5DFamilyStimulus {
    /// C++ base class `MSSOTLPolicyDesirability`
    pub base: MSSOTLPolicyDesirability,
    pub default_values: ParameterisedMap,
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
    pub fn new(keyPrefix: String, parameters: &ParameterisedMap) -> Self {
        let mut this = Self {
            base: MSSOTLPolicyDesirability::new(keyPrefix.clone(), parameters),
            default_values: ParameterisedMap::new(),
            params_names: Vec::new(),
            family: Vec::new(),
        };

        this.default_values.insert("_STIM_COX".to_string(), "1".to_string());
        this.default_values.insert("_STIM_OFFSET_IN".to_string(), "1".to_string());
        this.default_values.insert("_STIM_OFFSET_OUT".to_string(), "1".to_string());
        this.default_values.insert("_STIM_OFFSET_DISPERSION_IN".to_string(), "1".to_string());
        this.default_values.insert("_STIM_OFFSET_DISPERSION_OUT".to_string(), "1".to_string());
        this.default_values.insert("_STIM_DIVISOR_IN".to_string(), "1".to_string());
        this.default_values.insert("_STIM_DIVISOR_OUT".to_string(), "1".to_string());
        this.default_values.insert("_STIM_DIVISOR_DISPERSION_IN".to_string(), "1".to_string());
        this.default_values.insert("_STIM_DIVISOR_DISPERSION_OUT".to_string(), "1".to_string());
        this.default_values.insert("_STIM_COX_EXP_IN".to_string(), "0".to_string());
        this.default_values.insert("_STIM_COX_EXP_OUT".to_string(), "0".to_string());
        this.default_values.insert("_STIM_COX_EXP_DISPERSION_IN".to_string(), "0".to_string());
        this.default_values.insert("_STIM_COX_EXP_DISPERSION_OUT".to_string(), "0".to_string());

        this.params_names.push("_STIM_COX".to_string());
        this.params_names.push("_STIM_OFFSET_IN".to_string());
        this.params_names.push("_STIM_OFFSET_OUT".to_string());
        this.params_names.push("_STIM_OFFSET_DISPERSION_IN".to_string());
        this.params_names.push("_STIM_OFFSET_DISPERSION_OUT".to_string());
        this.params_names.push("_STIM_DIVISOR_IN".to_string());
        this.params_names.push("_STIM_DIVISOR_OUT".to_string());
        this.params_names.push("_STIM_DIVISOR_DISPERSION_IN".to_string());
        this.params_names.push("_STIM_DIVISOR_DISPERSION_OUT".to_string());
        this.params_names.push("_STIM_COX_EXP_IN".to_string());
        this.params_names.push("_STIM_COX_EXP_OUT".to_string());
        this.params_names.push("_STIM_COX_EXP_DISPERSION_IN".to_string());
        this.params_names.push("_STIM_COX_EXP_DISPERSION_OUT".to_string());

        let size_family = this.get_double(&format!("{}_SIZE_FAMILY", keyPrefix), 1.) as i32;

        let mut sliced_maps: Vec<ParameterisedMap> = Vec::new();
        for _ in 0..size_family {
            sliced_maps.push(ParameterisedMap::new());
        }

        // For each param list, slice values
        for i in 0..this.params_names.len() {
            let key = format!("{}{}", keyPrefix, this.params_names[i]);
            let default_value = this.default_values.get(&this.params_names[i]).cloned().unwrap_or_default();
            let param_list = this.get_parameter(&key, &default_value);
            let tokens = StringTokenizer::with_token(&param_list, ";", false).get_vector();

            for token_counter in 0..size_family {
                let tc = token_counter as usize;
                if tc >= tokens.len() {
                    utils::common::msg_handler::write_error(&format!("Error in {}: not enough tokens.", key));
                    // assert(-1) in C++ never fires
                }
                // shortcut: C++ reads tokens[token_counter] past the end here (undefined behaviour), the missing value is stored as an empty string, upgrade when the C++ behaviour is defined
                let value = tokens.get(tc).cloned().unwrap_or_default();
                sliced_maps[tc].insert(key.clone(), value);
            }
        }

        for i in 0..size_family {
            let ref_map = sliced_maps[i as usize].clone();
            // shortcut: C++ `new MSSOTLPolicy5DStimulus` is never deleted, the Box is leaked to match, upgrade when the family owner is typed
            let stimulus: &'static mut MSSOTLPolicy5DStimulus = Box::leak(Box::new(MSSOTLPolicy5DStimulus::new(keyPrefix.clone(), &ref_map)));
            this.family.push(Ptr::from_mut(stimulus));
        }

        this
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

