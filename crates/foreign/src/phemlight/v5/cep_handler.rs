//! Port of foreign/PHEMlight/V5/cpp/CEPHandler.{h,cpp}

use std::collections::BTreeMap;
use std::io::Read;

use serde_json::Value;

use super::cep::CEP;
use super::correction::Correction;
use super::helpers::Helpers;
use super::json;
use super::veh::{Veh, VehicleData};
use crate::phemlight::cep_handler::{open_first, read_line, split, todouble_list};

/// `json2double`: the value of `key` as a double, or 0 when the key is absent.
fn json2double(vd: &Value, key: &str) -> Result<f64, String> {
    if json::contains(vd, key) {
        return json::get_f64(json::at(vd, key)?);
    }
    Ok(0.0)
}

/// `json.find(key)` on the root object (end() for non-objects).
fn find<'a>(root: &'a Value, key: &str) -> Option<&'a Value> {
    root.as_object().and_then(|map| map.get(key))
}

/// Values produced by `CEPHandler::ReadEmissionData`.
#[derive(Debug, Default)]
struct EmissionData {
    header: Vec<String>,
    matrix: Vec<Vec<f64>>,
    idling_values: Vec<f64>,
}

/// Port of `CEPHandler::GetDetTempCor`: deterioration and TNOx correction factor for one pollutant.
fn get_det_temp_cor(data_cor: Option<&Correction>, emi: &str) -> f64 {
    // Initialisation
    let mut cor_f = 1.0;
    let emi = emi.to_ascii_uppercase();
    if let Some(cor) = data_cor {
        if cor.get_use_det() && cor.det_factors.contains_key(&emi) {
            cor_f += cor.det_factors[&emi] - 1.0;
        }
        if cor.get_use_tnox() && emi.contains("NOX") {
            cor_f += cor.get_tnox_factor() - 1.0;
        }
    }
    cor_f
}

/// Port of `CEPHandler::CorrectEmissionData`. The C++ function always returns true.
fn correct_emission_data(data_cor: Option<&Correction>, header: &[String], matrix: &mut [Vec<f64>], idling_values: &mut [f64]) {
    for i in 0..header.len() {
        let cor_f = get_det_temp_cor(data_cor, &header[i]);
        if cor_f != 1.0 {
            for row in matrix.iter_mut() {
                row[i + 1] *= cor_f;
            }
            idling_values[i] *= cor_f;
        }
    }
}

/// Port of `CEPHandler::CalcCorrection`.
fn calc_correction(cor: &mut Correction, helper: &Helpers, vehicle_data: &VehicleData) -> Result<bool, String> {
    if cor.get_use_det() {
        cor.set_veh_mileage(-1.0);
        if vehicle_data.mileage > 0.0 {
            cor.set_veh_mileage(vehicle_data.mileage);
        }
        if !cor.ini_det_factor(helper)? {
            return Ok(false);
        }
    }
    if cor.get_use_tnox() && !cor.ini_tnox_factor(helper)? {
        return Ok(false);
    }
    // Return value
    Ok(true)
}

/// Port of `CEPHandler::ReadVehicleFile`. `Ok(None)` mirrors `return false` with the error message set;
/// `Err` mirrors a nlohmann exception that propagates out of the C++ code.
fn read_vehicle_file(data_path: &[String], emission_class: &str, helper: &mut Helpers) -> Result<Option<Veh>, String> {
    let file_name = format!("{}.PHEMLight.veh", emission_class);
    let Some(mut reader) = open_first(data_path, &file_name) else {
        helper.set_err_msg(&format!("File does not exist! ({})", file_name));
        return Ok(None);
    };

    // **** VEH Datei einlesen ****
    let mut text = String::new();
    let root: Value = match reader.read_to_string(&mut text).ok().and_then(|_| serde_json::from_str(&text).ok()) {
        Some(root) => root,
        None => {
            helper.set_err_msg(&format!("Error during file read! ({})", file_name));
            return Ok(None);
        }
    };

    let mut vehicle = Veh::default();

    // *** Get the vehicle data
    let Some(vd) = find(&root, "VehicleData") else {
        helper.set_err_msg(&format!("No VehicleData in {}!", file_name));
        return Ok(None);
    };
    vehicle.vehicle_data.mass_type = if json::contains(vd, "MassType") { json::get_string(json::at(vd, "MassType")?)? } else { "LV".to_string() };
    vehicle.vehicle_data.fuel_type = if json::contains(vd, "FuelType") { json::get_string(json::at(vd, "FuelType")?)? } else { "D".to_string() };
    vehicle.vehicle_data.calc_type = if json::contains(vd, "CalcType") { json::get_string(json::at(vd, "CalcType")?)? } else { "Conv".to_string() };
    vehicle.vehicle_data.mass = json2double(vd, "Mass")?;
    vehicle.vehicle_data.loading = json2double(vd, "Loading")?;
    vehicle.vehicle_data.red_mass_wheel = json2double(vd, "RedMassWheel")?;
    vehicle.vehicle_data.wheel_diameter = json2double(vd, "WheelDiameter")?;
    vehicle.vehicle_data.cw = json2double(vd, "Cw")?;
    vehicle.vehicle_data.a = json2double(vd, "A")?;
    vehicle.vehicle_data.mileage = json2double(vd, "Mileage")?;

    // Auxiliaries
    vehicle.auxiliaries_data.pauxnorm = match find(&root, "AuxiliariesData") {
        Some(aux) if json::contains(aux, "Pauxnorm") => json::get_f64(json::at(aux, "Pauxnorm")?)?,
        _ => 0.0,
    };

    // Engine Data
    let Some(eng) = find(&root, "EngineData").filter(|e| json::contains(e, "ICEData") && json::contains(e, "EMData")) else {
        helper.set_err_msg(&format!("Incomplete EngineData in {}!", file_name));
        return Ok(None);
    };
    let iced = json::at(eng, "ICEData")?;
    let emd = json::at(eng, "EMData")?;
    vehicle.engine_data.ice.prated = json2double(iced, "Prated")?;
    vehicle.engine_data.ice.nrated = json2double(iced, "nrated")?;
    vehicle.engine_data.ice.idling = json2double(iced, "Idling")?;
    vehicle.engine_data.em.prated = json2double(emd, "Prated")?;
    vehicle.engine_data.em.nrated = json2double(emd, "nrated")?;

    // Rolling resistance
    let Some(rrd) = find(&root, "RollingResData") else {
        helper.set_err_msg(&format!("No RollingResData in {}!", file_name));
        return Ok(None);
    };
    vehicle.rolling_res_data.fr0 = json2double(rrd, "Fr0")?;
    vehicle.rolling_res_data.fr1 = json2double(rrd, "Fr1")?;
    vehicle.rolling_res_data.fr2 = json2double(rrd, "Fr2")?;
    vehicle.rolling_res_data.fr3 = json2double(rrd, "Fr3")?;
    vehicle.rolling_res_data.fr4 = json2double(rrd, "Fr4")?;

    // Transmission
    let Some(trd) = find(&root, "TransmissionData") else {
        helper.set_err_msg(&format!("No TransmissionData in {}!", file_name));
        return Ok(None);
    };
    vehicle.transmission_data.axel_ratio = json2double(trd, "AxelRatio")?;
    // C++ error texts end with the (empty) path
    let Some(transm) = find(trd, "Transm") else {
        helper.set_err_msg("Transmission ratios missing in vehicle file! Calculation stopped! ()");
        return Ok(None);
    };
    for (key, what) in [("Speed", "Speed"), ("GearRatio", "GearRatio"), ("RotMassF", "RotMassF")] {
        if !json::contains(transm, key) {
            helper.set_err_msg(&format!("No {} signal in transmission data given! Calculation stopped! ()", what));
            return Ok(None);
        }
    }
    vehicle.transmission_data.transm = json::get_f64_map(transm)?;

    // Full load and drag
    let Some(fld) = find(&root, "FLDData") else {
        helper.set_err_msg(&format!("No FLDData in {}!", file_name));
        return Ok(None);
    };
    vehicle.fld_data.p_n_max_v0 = json2double(fld, "P_n_max_v0")?;
    vehicle.fld_data.p_n_max_p0 = json2double(fld, "P_n_max_p0")?;
    vehicle.fld_data.p_n_max_v1 = json2double(fld, "P_n_max_v1")?;
    vehicle.fld_data.p_n_max_p1 = json2double(fld, "P_n_max_p1")?;
    let Some(drag) = find(fld, "DragCurve") else {
        helper.set_err_msg("Drag curve missing in vehicle file! Calculation stopped! ()");
        return Ok(None);
    };
    for key in ["n_norm", "pe_drag_norm"] {
        if !json::contains(drag, key) {
            helper.set_err_msg(&format!("No {} signal in drag curve data given! Calculation stopped! ()", key));
            return Ok(None);
        }
    }
    vehicle.fld_data.drag_curve = json::get_f64_map(drag)?;

    Ok(Some(vehicle))
}

/// Port of `CEPHandler::ReadEmissionData`, including the correction step at its end.
fn read_emission_data(read_fc: bool, data_path: &[String], emission_class: &str, helper: &mut Helpers, data_cor: Option<&Correction>) -> Option<EmissionData> {
    let pollutant_extension = if read_fc { "_FC" } else { "" };
    let file_name = format!("{}{}.csv", emission_class, pollutant_extension);
    let Some(mut reader) = open_first(data_path, &file_name) else {
        helper.set_err_msg(&format!("File does not exist! ({})", file_name));
        return None;
    };

    let mut data = EmissionData::default();

    // read header line for pollutant identifiers
    let line = read_line(&mut reader);
    if !line.is_empty() {
        // skip first entry "Pe"
        data.header = split(&line, ',').into_iter().skip(1).collect();
    }

    // skip units
    read_line(&mut reader);

    // skip comment
    read_line(&mut reader);

    // readIdlingValues
    let line = read_line(&mut reader);
    let mut string_idlings = split(&line, ',');
    // C++ erases begin() unconditionally (undefined on an empty line); here an empty list is left as is.
    if !string_idlings.is_empty() {
        string_idlings.remove(0);
    }
    data.idling_values = todouble_list(&string_idlings);

    loop {
        let line = read_line(&mut reader);
        if line.is_empty() {
            break;
        }
        data.matrix.push(todouble_list(&split(&line, ',')));
    }

    // Data correction (Det & TNOx)
    correct_emission_data(data_cor, &data.header, &mut data.matrix, &mut data.idling_values);
    Some(data)
}

#[derive(Debug, Default)]
pub struct CEPHandler {
    ceps: BTreeMap<String, CEP>,
}

impl CEPHandler {
    pub fn new() -> Self {
        CEPHandler { ceps: BTreeMap::new() }
    }

    pub fn get_ceps(&self) -> &BTreeMap<String, CEP> {
        &self.ceps
    }

    pub fn get_cep(&mut self, data_path: &[String], helper: &mut Helpers, data_cor: Option<&mut Correction>) -> Result<bool, String> {
        if !self.get_ceps().contains_key(helper.get_g_class()) && !self.load(data_path, helper, data_cor)? {
            return Ok(false);
        }
        Ok(true)
    }

    fn load(&mut self, data_path: &[String], helper: &mut Helpers, mut data_cor: Option<&mut Correction>) -> Result<bool, String> {
        // get string identifier for PHEM emission class
        let emission_rep = helper.get_g_class().to_string();

        let Some(vehicle) = read_vehicle_file(data_path, &emission_rep, helper)? else {
            return Ok(false);
        };

        if let Some(cor) = data_cor.as_deref_mut() {
            if !calc_correction(cor, helper, &vehicle.vehicle_data)? {
                return Ok(false);
            }
        }

        let Some(fc) = read_emission_data(true, data_path, &emission_rep, helper, data_cor.as_deref()) else {
            return Ok(false);
        };
        let Some(pollutants) = read_emission_data(false, data_path, &emission_rep, helper, data_cor.as_deref()) else {
            return Ok(false);
        };

        let cep = CEP::new(
            &vehicle,
            &fc.header,
            &fc.matrix,
            &pollutants.header,
            &pollutants.matrix,
            &fc.idling_values,
            &pollutants.idling_values,
        );
        // std::map::insert: an existing entry is kept
        self.ceps.entry(emission_rep).or_insert(cep);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn det_temp_cor_applies_det_and_tnox_factors_in_cpp_order() {
        let mut cor = Correction::new(&[]);
        cor.set_use_det(true);
        cor.det_factors.insert("NOX".to_string(), 1.2);
        cor.set_use_tnox(true);
        cor.set_tnox_factor(0.5);
        // CorF = 1; CorF += DET - 1; CorF += TNOx - 1 (same operation order as C++)
        let mut expected = 1.0;
        expected += 1.2 - 1.0;
        expected += 0.5 - 1.0;
        assert_eq!(get_det_temp_cor(Some(&cor), "nox"), expected);
        // no deterioration factor and no "NOX" in the name: no correction
        assert_eq!(get_det_temp_cor(Some(&cor), "CO2"), 1.0);
        assert_eq!(get_det_temp_cor(None, "NOX"), 1.0);
    }

    #[test]
    fn correct_emission_data_scales_only_matching_columns() {
        let mut cor = Correction::new(&[]);
        cor.set_use_det(true);
        cor.det_factors.insert("NOX".to_string(), 2.0);
        let header = vec!["NOX".to_string(), "CO".to_string()];
        let mut matrix = vec![vec![10.0, 1.0, 2.0]];
        let mut idling = vec![3.0, 4.0];
        correct_emission_data(Some(&cor), &header, &mut matrix, &mut idling);
        // NOX factor 1 + (2 - 1) = 2 scales column 1 and idling[0]; CO is untouched
        assert_eq!(matrix, vec![vec![10.0, 2.0, 2.0]]);
        assert_eq!(idling, vec![6.0, 4.0]);
    }
}
