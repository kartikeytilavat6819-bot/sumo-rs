//! Port of foreign/PHEMlight/cpp/CEPHandler.{h,cpp}

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::cep::CEP;
use super::constants::*;
use super::helpers::Helpers;

/// `ReadLine`: `std::getline` followed by stripping trailing " \n\r\t".
/// A line consisting only of those characters is kept unchanged (C++ `find_last_not_of` returns npos).
pub(crate) fn read_line<R: BufRead>(reader: &mut R) -> String {
    let mut buf = Vec::new();
    let _ = reader.read_until(b'\n', &mut buf);
    if buf.last() == Some(&b'\n') {
        buf.pop();
    }
    let trimmed_len = buf.iter().rposition(|c| !matches!(c, b' ' | b'\n' | b'\r' | b'\t')).map(|p| p + 1);
    if let Some(len) = trimmed_len {
        buf.truncate(len);
    }
    String::from_utf8_lossy(&buf).into_owned()
}

/// `split`: emulates repeated `std::getline(ss, item, delim)` on a stringstream.
/// Empty input yields no items; a trailing delimiter does not produce an extra empty item.
pub(crate) fn split(s: &str, delim: char) -> Vec<String> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut elems: Vec<String> = s.split(delim).map(str::to_string).collect();
    if s.ends_with(delim) {
        elems.pop();
    }
    elems
}

/// `todouble`: emulates `std::stringstream(s) >> double`. Leading whitespace is skipped, the longest
/// valid number prefix is taken, and an unparsable or incomplete number yields 0.
pub(crate) fn todouble(s: &str) -> f64 {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    let start = i;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut digits = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
        digits += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0.0;
    }
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        i += 1;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let exp_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        // an exponent marker without digits makes the conversion fail
        if i == exp_start {
            return 0.0;
        }
    }
    match s[start..i].parse::<f64>() {
        Ok(v) if v.is_infinite() => {
            // stream extraction clamps overflow to +/- numeric_limits<double>::max()
            if v > 0.0 { f64::MAX } else { -f64::MAX }
        }
        Ok(v) => v,
        Err(_) => 0.0,
    }
}

/// `todoubleList`
pub(crate) fn todouble_list(s: &[String]) -> Vec<f64> {
    s.iter().map(|item| todouble(item)).collect()
}

/// `line.substr(0, 1) == Helper->getCommentPrefix()`
fn is_comment_line(line: &str, comment_prefix: &str) -> bool {
    let n = line.len().min(1);
    line.as_bytes()[..n] == *comment_prefix.as_bytes()
}

/// Open the first existing `<dir><file_name>` in `data_path`.
pub(crate) fn open_first(data_path: &[String], file_name: &str) -> Option<BufReader<File>> {
    data_path.iter().find_map(|dir| File::open(format!("{}{}", dir, file_name)).ok()).map(BufReader::new)
}

/// Values produced by `CEPHandler::ReadVehicleFile`.
#[derive(Debug, Default)]
struct VehicleFile {
    vehicle_mass: f64,
    vehicle_loading: f64,
    vehicle_mass_rot: f64,
    cross_area: f64,
    cw_value: f64,
    f0: f64,
    f1: f64,
    f2: f64,
    f3: f64,
    f4: f64,
    axle_ratio: f64,
    aux_power: f64,
    rated_power: f64,
    engine_idling_speed: f64,
    engine_rated_speed: f64,
    effective_wheel_diameter: f64,
    transmission_gear_ratios: Vec<f64>,
    vehicle_mass_type: String,
    vehicle_fuel_type: String,
    p_norm_v0: f64,
    p_norm_p0: f64,
    p_norm_v1: f64,
    p_norm_p1: f64,
    matrix_speed_inertia_table: Vec<Vec<f64>>,
    normed_drag_table: Vec<Vec<f64>>,
}

/// Values produced by `CEPHandler::ReadEmissionData`.
#[derive(Debug, Default)]
struct EmissionData {
    header: Vec<String>,
    matrix: Vec<Vec<f64>>,
    idling_values: Vec<f64>,
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

    pub fn get_cep(&mut self, data_path: &[String], helper: &mut Helpers) -> bool {
        if !self.get_ceps().contains_key(helper.get_g_class()) && !self.load(data_path, helper) {
            return false;
        }
        true
    }

    fn load(&mut self, data_path: &[String], helper: &mut Helpers) -> bool {
        // get string identifier for PHEM emission class
        let emission_rep = helper.get_g_class().to_string();

        let Some(vehicle) = read_vehicle_file(data_path, &emission_rep, helper) else {
            return false;
        };
        let Some(fc) = read_emission_data(true, data_path, &emission_rep, helper) else {
            return false;
        };
        let Some(pollutants) = read_emission_data(false, data_path, &emission_rep, helper) else {
            return false;
        };

        // C++ calls front() on the FC idling values (undefined on an empty file); 0.0 is used here.
        let idling_fc = fc.idling_values.first().copied().unwrap_or(0.0);

        let cep = CEP::new(
            vehicle.vehicle_mass_type == HEAVY_VEHICLE,
            vehicle.vehicle_mass,
            vehicle.vehicle_loading,
            vehicle.vehicle_mass_rot,
            vehicle.cross_area,
            vehicle.cw_value,
            vehicle.f0,
            vehicle.f1,
            vehicle.f2,
            vehicle.f3,
            vehicle.f4,
            vehicle.axle_ratio,
            &vehicle.transmission_gear_ratios,
            vehicle.aux_power,
            vehicle.rated_power,
            vehicle.engine_idling_speed,
            vehicle.engine_rated_speed,
            vehicle.effective_wheel_diameter,
            vehicle.p_norm_v0,
            vehicle.p_norm_p0,
            vehicle.p_norm_v1,
            vehicle.p_norm_p1,
            &vehicle.vehicle_fuel_type,
            &fc.matrix,
            &pollutants.header,
            &pollutants.matrix,
            &vehicle.matrix_speed_inertia_table,
            &vehicle.normed_drag_table,
            idling_fc,
            &pollutants.idling_values,
        );
        // std::map::insert: an existing entry is kept
        self.ceps.entry(emission_rep).or_insert(cep);

        true
    }
}

/// `CEPHandler::ReadVehicleFile`. Returns `None` (with the error message set) when the file is missing.
fn read_vehicle_file(data_path: &[String], emission_class: &str, helper: &mut Helpers) -> Option<VehicleFile> {
    let file_name = format!("{}.PHEMLight.veh", emission_class);
    let Some(mut reader) = open_first(data_path, &file_name) else {
        helper.set_err_msg(&format!("File does not exist! ({})", file_name));
        return None;
    };

    let mut veh = VehicleFile::default();

    // skip header
    read_line(&mut reader);

    let mut data_count = 0;
    loop {
        let line = read_line(&mut reader);
        if line.is_empty() || data_count > 49 {
            break;
        }
        if is_comment_line(&line, helper.get_comment_prefix()) {
            continue;
        }
        data_count += 1;

        let cell = split(&line, ',').into_iter().next().unwrap_or_default();
        match data_count {
            1 => veh.vehicle_mass = todouble(&cell),
            2 => veh.vehicle_loading = todouble(&cell),
            3 => veh.cw_value = todouble(&cell),
            4 => veh.cross_area = todouble(&cell),
            7 => veh.vehicle_mass_rot = todouble(&cell),
            9 => veh.aux_power = todouble(&cell),
            10 => veh.rated_power = todouble(&cell),
            11 => veh.engine_rated_speed = todouble(&cell),
            12 => veh.engine_idling_speed = todouble(&cell),
            14 => veh.f0 = todouble(&cell),
            15 => veh.f1 = todouble(&cell),
            16 => veh.f2 = todouble(&cell),
            17 => veh.f3 = todouble(&cell),
            18 => veh.f4 = todouble(&cell),
            21 => veh.axle_ratio = todouble(&cell),
            22 => veh.effective_wheel_diameter = todouble(&cell),
            23..=40 => veh.transmission_gear_ratios.push(todouble(&cell)),
            45 => veh.vehicle_mass_type = cell,
            46 => veh.vehicle_fuel_type = cell,
            47 => veh.p_norm_v0 = todouble(&cell),
            48 => veh.p_norm_p0 = todouble(&cell),
            49 => veh.p_norm_v1 = todouble(&cell),
            50 => veh.p_norm_p1 = todouble(&cell),
            _ => {}
        }
    }

    loop {
        let line = read_line(&mut reader);
        if line.is_empty() || is_comment_line(&line, helper.get_comment_prefix()) {
            break;
        }
        veh.matrix_speed_inertia_table.push(todouble_list(&split(&line, ',')));
    }

    loop {
        let line = read_line(&mut reader);
        if line.is_empty() {
            break;
        }
        if is_comment_line(&line, helper.get_comment_prefix()) {
            continue;
        }
        veh.normed_drag_table.push(todouble_list(&split(&line, ',')));
    }

    Some(veh)
}

/// `CEPHandler::ReadEmissionData`. Returns `None` (with the error message set) when the file is missing.
fn read_emission_data(read_fc: bool, data_path: &[String], emission_class: &str, helper: &mut Helpers) -> Option<EmissionData> {
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
    Some(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_matches_getline_semantics() {
        assert_eq!(split("", ','), Vec::<String>::new());
        assert_eq!(split(",", ','), vec![""]);
        assert_eq!(split("a,b,", ','), vec!["a", "b"]);
        assert_eq!(split("a,,b", ','), vec!["a", "", "b"]);
    }

    #[test]
    fn todouble_matches_stream_extraction() {
        assert_eq!(todouble("  1.5abc"), 1.5);
        assert_eq!(todouble("-2e3"), -2000.0);
        assert_eq!(todouble("5."), 5.0);
        assert_eq!(todouble("1e"), 0.0);
        assert_eq!(todouble("nan"), 0.0);
        assert_eq!(todouble("0x1A"), 0.0);
        assert_eq!(todouble(""), 0.0);
        assert_eq!(todouble("1e400"), f64::MAX);
    }

    #[test]
    fn read_line_strips_trailing_whitespace_but_keeps_blank_lines() {
        let mut r: &[u8] = b"abc \t\r\n   \nlast";
        assert_eq!(read_line(&mut r), "abc");
        assert_eq!(read_line(&mut r), "   ");
        assert_eq!(read_line(&mut r), "last");
        assert_eq!(read_line(&mut r), "");
    }

    #[test]
    fn comment_prefix_compares_first_character_only() {
        assert!(is_comment_line("#x", "#"));
        assert!(!is_comment_line("#x", "##"));
        assert!(!is_comment_line("x", ""));
    }
}
