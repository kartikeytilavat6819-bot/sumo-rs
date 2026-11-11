//! Port of foreign/PHEMlight/V5/cpp/Correction.{h,cpp}

use std::collections::BTreeMap;
use std::io::Read;

use serde_json::Value;

use super::constants::*;
use super::helpers::Helpers;
use super::json;
use crate::phemlight::cep_handler::open_first;
use utils::common::string_utils::replace;

/// Port of `Correction::Interpolate`.
fn interpolate(px: f64, p1: f64, p2: f64, e1: f64, e2: f64) -> f64 {
    if p2 == p1 {
        return e1;
    }
    e1 + (px - p1) / (p2 - p1) * (e2 - e1)
}

/// Reads a JSON file from the first existing `<dir><file_name>`; on failure sets `err_msg` as C++ does.
fn read_json_file(data_path: &[String], file_name: &str, err_msg: &mut String) -> Option<Value> {
    let Some(mut reader) = open_first(data_path, file_name) else {
        *err_msg = format!("File does not exist! ({})", file_name);
        return None;
    };
    let mut text = String::new();
    let parsed = match reader.read_to_string(&mut text) {
        Ok(_) => serde_json::from_str::<Value>(&text).ok(),
        Err(_) => None,
    };
    if parsed.is_none() {
        *err_msg = format!("Error during file read! ({})", file_name);
    }
    parsed
}

/// `EUclass.substr(0, 6)` for the PC/LNF special class check.
fn short_eu_class(eu_class: &str) -> String {
    String::from_utf8_lossy(&eu_class.as_bytes()[..6]).into_owned()
}

#[derive(Debug, Default)]
pub struct Correction {
    private_use_det: bool,
    private_data_path: Vec<String>,
    private_det_file_path: String,
    private_vma_file_path: String,
    private_year: i32,
    private_veh_mileage: f64,
    private_use_tnox: bool,
    private_tnox_file_path: String,
    private_amb_temp: f64,
    private_tnox_factor: f64,
    pub det_factors: BTreeMap<String, f64>,
    det_data: Value,
    vma_data: Value,
    tnox_data: Value,
}

impl Correction {
    pub fn new(data_path: &[String]) -> Self {
        let mut c = Correction {
            private_data_path: data_path.to_vec(),
            ..Default::default()
        };
        c.set_use_det(false);
        c.set_use_tnox(false);
        c.set_amb_temp(20.0);
        c.set_year(2022);
        c.set_veh_mileage(-1.0);
        c.set_det_file_path("Deterioration.det");
        c.set_vma_file_path("Mileage.vma");
        c.set_tnox_file_path("NOxCor.tno");
        c
    }

    pub fn get_use_det(&self) -> bool {
        self.private_use_det
    }

    pub fn set_use_det(&mut self, value: bool) {
        self.private_use_det = value;
    }

    pub fn get_det_file_path(&self) -> &str {
        &self.private_det_file_path
    }

    pub fn set_det_file_path(&mut self, value: &str) {
        self.private_det_file_path = value.to_string();
    }

    pub fn get_vma_file_path(&self) -> &str {
        &self.private_vma_file_path
    }

    pub fn set_vma_file_path(&mut self, value: &str) {
        self.private_vma_file_path = value.to_string();
    }

    pub fn get_year(&self) -> i32 {
        self.private_year
    }

    pub fn set_year(&mut self, value: i32) {
        self.private_year = value;
    }

    pub fn get_veh_mileage(&self) -> f64 {
        self.private_veh_mileage
    }

    pub fn set_veh_mileage(&mut self, value: f64) {
        self.private_veh_mileage = value;
    }

    /// Read the file
    pub fn read_det(&mut self, err_msg: &mut String) -> bool {
        // Read Detoriation file
        if !self.read_det_file(err_msg) {
            return false;
        }
        if !self.read_vma_file(err_msg) {
            return false;
        }
        // Return value
        true
    }

    fn read_det_file(&mut self, err_msg: &mut String) -> bool {
        match read_json_file(&self.private_data_path, &self.private_det_file_path, err_msg) {
            Some(data) => {
                self.det_data = data;
                true
            }
            None => false,
        }
    }

    fn read_vma_file(&mut self, err_msg: &mut String) -> bool {
        match read_json_file(&self.private_data_path, &self.private_vma_file_path, err_msg) {
            Some(data) => {
                self.vma_data = data;
                true
            }
            None => false,
        }
    }

    /// Initialise the Detoriation Factor for the vehicle
    pub fn ini_det_factor(&mut self, helper: &Helpers) -> Result<bool, String> {
        // Initialise
        self.det_factors = BTreeMap::new();
        let det_data = self.det_data.clone();
        let vehicles = json::at(&det_data, "Vehicle")?;
        if json::contains(vehicles, helper.get_v_class()) {
            let propulsion = json::at(json::at(vehicles, helper.get_v_class())?, "PropulsionClass")?;
            if json::contains(propulsion, helper.get_p_class()) {
                let emissions = json::at(json::at(propulsion, helper.get_p_class())?, "Emission")?;
                for (name, it_value) in json::items(emissions)? {
                    let mut eu_class = replace(helper.get_e_class(), "EU", "EURO ");
                    // PC special classes check (ab, c, d, d-Temp). If available use otherwise use "EURO 6" if available
                    if (helper.get_v_class() == STR_PKW || helper.get_v_class() == STR_LNF) && eu_class.len() > 6 {
                        let eu_class_short = short_eu_class(&eu_class);
                        let eu_classes = json::at(it_value, "EUClass")?;
                        if !json::contains(eu_classes, &eu_class) && json::contains(eu_classes, &eu_class_short) {
                            eu_class = eu_class_short;
                        }
                    }

                    // Store in upper case because retrieval will be upper case as well
                    let key = name.to_ascii_uppercase();

                    // Get the factor
                    if json::contains(json::at(it_value, "EUClass")?, &eu_class) {
                        let mileage = json::get_f64_vec(json::at(it_value, "Mileage")?)?;
                        let factor = json::get_f64_vec(json::at(json::at(it_value, "EUClass")?, &eu_class)?)?;
                        if self.get_veh_mileage() < 0.0 {
                            let mileage_calc = self.get_mileage(helper)?;
                            self.set_veh_mileage(mileage_calc);
                        }
                        let veh_mileage = self.get_veh_mileage();
                        for i in 1..mileage.len() {
                            if i == 1 && mileage[i] > veh_mileage {
                                self.det_factors.entry(key).or_insert(factor[0]);
                                break;
                            } else if i == mileage.len() - 1 && veh_mileage > mileage[i] {
                                self.det_factors.entry(key).or_insert(factor[i]);
                                break;
                            } else if veh_mileage < mileage[i] {
                                self.det_factors.entry(key).or_insert(interpolate(veh_mileage, mileage[i - 1], mileage[i], factor[i - 1], factor[i]));
                                break;
                            }
                        }
                    } else {
                        self.det_factors.entry(key).or_insert(1.0);
                    }
                }
            }
        }
        // Return value
        Ok(true)
    }

    /// Get the milage of the vehicle
    fn get_mileage(&self, helper: &Helpers) -> Result<f64, String> {
        // Initialise
        let mut mileage = 0.0;
        let vehicles = json::at(&self.vma_data, "Vehicle")?;
        if json::contains(vehicles, helper.get_v_class()) {
            let propulsion = json::at(json::at(vehicles, helper.get_v_class())?, "PropulsionClass")?;
            if json::contains(propulsion, helper.get_p_class()) {
                let s_class = match helper.get_s_class() {
                    "I" => "1",
                    "II" => "2",
                    "III" => "3",
                    _ => "0",
                };
                let size_classes = json::at(json::at(propulsion, helper.get_p_class())?, "SizeClass")?;
                if json::contains(size_classes, s_class) {
                    let sclass_json = json::at(size_classes, s_class)?;
                    let mut eu_class = replace(helper.get_e_class(), "EU", "EURO ");
                    // PC special classes check (ab, c, d, d-Temp). If available use otherwise use "EURO 6" if available
                    if (helper.get_v_class() == STR_PKW || helper.get_v_class() == STR_LNF) && eu_class.len() > 6 {
                        let eu_class_short = short_eu_class(&eu_class);
                        let eu_classes = json::at(sclass_json, "EUClass")?;
                        if !json::contains(eu_classes, &eu_class) && json::contains(eu_classes, &eu_class_short) {
                            eu_class = eu_class_short;
                        }
                    }
                    if json::contains(sclass_json, &eu_class) {
                        // Calculate Mileage
                        let factor = json::get_f64_vec(json::at(sclass_json, &eu_class)?)?;
                        let anz_year = self.get_year() - 2020 + 1;
                        let anz = anz_year as f64;
                        mileage = factor[0] * anz.powf(3.0) + factor[1] * anz.powf(2.0) + factor[2] * anz + factor[3];
                        // Check calculated mileage
                        if mileage < 0.0 {
                            mileage = 0.0;
                        }
                    }
                }
            }
        }
        // Return value
        Ok(mileage)
    }

    pub fn get_use_tnox(&self) -> bool {
        self.private_use_tnox
    }

    pub fn set_use_tnox(&mut self, value: bool) {
        self.private_use_tnox = value;
    }

    pub fn get_tnox_file_path(&self) -> &str {
        &self.private_tnox_file_path
    }

    pub fn set_tnox_file_path(&mut self, value: &str) {
        self.private_tnox_file_path = value.to_string();
    }

    pub fn get_amb_temp(&self) -> f64 {
        self.private_amb_temp
    }

    pub fn set_amb_temp(&mut self, value: f64) {
        self.private_amb_temp = value;
    }

    pub fn get_tnox_factor(&self) -> f64 {
        self.private_tnox_factor
    }

    pub fn set_tnox_factor(&mut self, value: f64) {
        self.private_tnox_factor = value;
    }

    /// Read the file
    pub fn read_tnox(&mut self, err_msg: &mut String) -> bool {
        match read_json_file(&self.private_data_path, &self.private_tnox_file_path, err_msg) {
            Some(data) => {
                self.tnox_data = data;
                true
            }
            None => false,
        }
    }

    pub fn ini_tnox_factor(&mut self, helper: &Helpers) -> Result<bool, String> {
        // Initialise
        self.set_tnox_factor(1.0);

        // Calculation only for diesel vehicles
        if helper.get_p_class() != STR_DIESEL {
            return Ok(true);
        }

        let tnox_data = self.tnox_data.clone();
        let vehicles = json::at(&tnox_data, "Vehicle")?;
        if json::contains(vehicles, helper.get_v_class()) {
            let mut eu_class = replace(helper.get_e_class(), "EU", "EURO ");
            // PC special classes check (ab, c, d, d-Temp). If available use otherwise use "EURO 6" if available
            if (helper.get_v_class() == STR_PKW || helper.get_v_class() == STR_LNF) && eu_class.len() > 6 {
                let eu_class_short = short_eu_class(&eu_class);
                let eu_classes = json::at(json::at(vehicles, helper.get_v_class())?, "EUClass")?;
                if !json::contains(eu_classes, &eu_class) && json::contains(eu_classes, &eu_class_short) {
                    eu_class = eu_class_short;
                }
            }

            let eu_classes = json::at(json::at(vehicles, helper.get_v_class())?, "EUClass")?;
            if json::contains(eu_classes, &eu_class) {
                // Check/set temperature borders, because calculation is a straight function
                let eu_class_json = json::at(eu_classes, &eu_class)?;
                let m = json::get_f64(json::at(eu_class_json, "m")?)?;
                let c = json::get_f64(json::at(eu_class_json, "c")?)?;
                let tb0 = json::get_f64(json::index(json::at(eu_class_json, "TB")?, 0)?)?;
                if self.get_amb_temp() < tb0 {
                    self.set_tnox_factor(m + c * tb0);
                } else if self.get_amb_temp() > json::get_f64(json::index(json::at(eu_class_json, "TB")?, 1)?)? {
                    self.set_tnox_factor(1.0);
                } else {
                    self.set_tnox_factor(m + c * self.get_amb_temp());
                }
            }
        }

        // Return value
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn helper(v: &str, p: &str, e: &str) -> Helpers {
        let mut h = Helpers::default();
        h.set_v_class(v);
        h.set_p_class(p);
        h.set_s_class("");
        h.set_e_class(e);
        h
    }

    #[test]
    fn mileage_uses_size_class_polynomial_and_pc_class_shortcut() {
        let mut cor = Correction::new(&[]);
        // C++ reads the factors directly under the size class and uses "EUClass" for the PC shortcut check
        cor.vma_data = json!({"Vehicle": {"PC": {"PropulsionClass": {"D": {"SizeClass": {"0": {
            "EUClass": {"EURO 6": 1},
            "EURO 6": [0.0, 0.0, 0.0, 100.0]
        }}}}}}});
        // year 2022 -> AnzYear = 3; factors [0,0,0,100] give 100
        assert_eq!(cor.get_mileage(&helper("PC", "D", "EU6")).unwrap(), 100.0);
        // "EURO 6c" is longer than 6 chars and falls back to "EURO 6" for PC
        assert_eq!(cor.get_mileage(&helper("PC", "D", "EU6c")).unwrap(), 100.0);
        // unknown euro class: no mileage
        assert_eq!(cor.get_mileage(&helper("PC", "D", "EU5")).unwrap(), 0.0);
    }

    #[test]
    fn missing_vehicle_key_is_an_error_like_nlohmann_at() {
        let mut cor = Correction::new(&[]);
        cor.vma_data = json!({"Other": {}});
        assert!(cor.get_mileage(&helper("PC", "D", "EU6")).is_err());
    }
}
