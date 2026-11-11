//! Port of foreign/PHEMlight/V5/cpp/CEP.{h,cpp}

use std::collections::BTreeMap;

use super::constants::*;
use super::helpers::Helpers;
use super::veh::Veh;

/// `std::map::insert` semantics: an existing key keeps its first value.
fn insert_first<V>(map: &mut BTreeMap<String, V>, key: &str, value: V) {
    map.entry(key.to_string()).or_insert(value);
}

/// Port of `CEP::Interpolate`.
fn interpolate(px: f64, p1: f64, p2: f64, e1: f64, e2: f64) -> f64 {
    if p2 == p1 {
        return e1;
    }
    e1 + (px - p1) / (p2 - p1) * (e2 - e1)
}

/// Port of `CEP::FindLowerUpperInPattern` (the pattern values are multiplied by `scale`).
/// Returns `(lowerIndex, upperIndex)`.
fn find_lower_upper_in_pattern(pattern: &[f64], value: f64, scale: f64) -> (usize, usize) {
    if value <= pattern[0] * scale {
        return (0, 0);
    }

    let last = pattern.len() - 1;
    if value >= pattern[last] * scale {
        return (last, last);
    }

    // bisection search to find correct position in power pattern
    let mut middle_index = last / 2;
    let mut upper_index = last;
    let mut lower_index = 0;

    while upper_index - lower_index > 1 {
        if pattern[middle_index] * scale == value {
            return (middle_index, middle_index);
        } else if pattern[middle_index] * scale < value {
            lower_index = middle_index;
            middle_index = (upper_index - lower_index) / 2 + lower_index;
        } else {
            upper_index = middle_index;
            middle_index = (upper_index - lower_index) / 2 + lower_index;
        }
    }

    (lower_index, upper_index)
}

#[derive(Debug, Default, Clone)]
pub struct CEP {
    private_heavy_vehicle: bool,
    private_fuel_type: String,
    private_calc_type: String,
    private_rated_power: f64,

    mass_vehicle: f64,
    vehicle_loading: f64,
    vehicle_mass_rot: f64,
    cross_sectional_area: f64,
    cw_value: f64,
    resistance_f0: f64,
    resistance_f1: f64,
    resistance_f2: f64,
    resistance_f3: f64,
    resistance_f4: f64,
    axle_ratio: f64,
    aux_power: f64,
    p_norm_v0: f64,
    p_norm_p0: f64,
    p_norm_v1: f64,
    p_norm_p1: f64,
    engine_rated_speed: f64,
    engine_idling_speed: f64,
    effective_wheel_diameter: f64,

    speed_pattern_rotational: Vec<f64>,
    normalized_power_pattern_fc_values: Vec<f64>,
    normalized_power_pattern_pollutants: Vec<f64>,
    normed_cep_curve_fc_values: BTreeMap<String, Vec<f64>>,
    gear_transmission_curve: Vec<f64>,
    speed_curve_rotational: Vec<f64>,
    cep_normalized_curve_pollutants: BTreeMap<String, Vec<f64>>,
    fleet_mix: BTreeMap<String, f64>,
    idling_value_fc_values: BTreeMap<String, f64>,
    idling_values_pollutants: BTreeMap<String, f64>,
    n_norm_table: Vec<f64>,
    drag_norm_table: Vec<f64>,
}

impl CEP {
    /// Port of the V5 `CEP` constructor. Malformed matrices end the fill early (C++ `return`),
    /// leaving the remaining fields at their default values.
    pub fn new(
        vehicle: &Veh,
        header_line_fc_values: &[String],
        matrix_fc_values: &[Vec<f64>],
        header_line_pollutants: &[String],
        matrix_pollutants: &[Vec<f64>],
        idling_fc_values: &[f64],
        idling_pollutants: &[f64],
    ) -> Self {
        let rolling = &vehicle.rolling_res_data;
        let vd = &vehicle.vehicle_data;
        let engine = &vehicle.engine_data;
        let mut cep = CEP {
            resistance_f0: rolling.fr0,
            resistance_f1: rolling.fr1,
            resistance_f2: rolling.fr2,
            resistance_f3: rolling.fr3,
            resistance_f4: rolling.fr4,
            cw_value: vd.cw,
            cross_sectional_area: vd.a,
            mass_vehicle: vd.mass,
            vehicle_loading: vd.loading,
            vehicle_mass_rot: vd.red_mass_wheel,
            ..Default::default()
        };
        cep.set_calc_type(&vd.calc_type);

        // the rated power and speeds depend on the calculation type
        if cep.get_calc_type() == "Conv" {
            cep.set_rated_power(engine.ice.prated);
            cep.engine_rated_speed = engine.ice.nrated;
            cep.engine_idling_speed = engine.ice.idling;
        } else if cep.get_calc_type() == "HEV" {
            // Power von beiden zusammen Rest bezogen auf ICE
            cep.set_rated_power(engine.ice.prated + engine.em.prated);
            cep.engine_rated_speed = engine.ice.nrated;
            cep.engine_idling_speed = engine.ice.idling;
        } else if cep.get_calc_type() == "BEV" {
            cep.set_rated_power(engine.em.prated);
            cep.engine_rated_speed = engine.em.nrated;
            cep.engine_idling_speed = 0.0;
        }
        cep.effective_wheel_diameter = vd.wheel_diameter;
        cep.private_heavy_vehicle = vd.mass_type == HEAVY_VEHICLE;
        cep.set_fuel_type(&vd.fuel_type);
        cep.axle_ratio = vehicle.transmission_data.axel_ratio;
        cep.aux_power = vehicle.auxiliaries_data.pauxnorm;
        cep.p_norm_v0 = vehicle.fld_data.p_n_max_v0 / 3.6;
        cep.p_norm_p0 = vehicle.fld_data.p_n_max_p0;
        cep.p_norm_v1 = vehicle.fld_data.p_n_max_v1 / 3.6;
        cep.p_norm_p1 = vehicle.fld_data.p_n_max_p1;

        // Init pollutant identifiers, unit and measures
        let fc_identifier: &[String] = header_line_fc_values;
        let mut normalized_fc_values_measures: Vec<Vec<f64>> = vec![Vec::new(); header_line_fc_values.len()];
        let pollutant_identifier: &[String] = header_line_pollutants;
        let mut normalized_pollutant_measures: Vec<Vec<f64>> = vec![Vec::new(); header_line_pollutants.len()];

        // Assigning values for speed rotational table
        let transm = &vehicle.transmission_data.transm;
        cep.speed_pattern_rotational = transm["Speed"].iter().map(|s| s / 3.6).collect();
        cep.gear_transmission_curve = transm["GearRatio"].clone();
        cep.speed_curve_rotational = transm["RotMassF"].clone();

        // Assigning values for drag table
        cep.n_norm_table = vehicle.fld_data.drag_curve["n_norm"].clone();
        cep.drag_norm_table = vehicle.fld_data.drag_curve["pe_drag_norm"].clone();

        // Looping through matrix and assigning values for FC values
        let header_fc_count = header_line_fc_values.len();
        for row in matrix_fc_values {
            for (j, &value) in row.iter().enumerate() {
                if row.len() != header_fc_count + 1 {
                    return cep;
                }
                if j == 0 {
                    cep.normalized_power_pattern_fc_values.push(value);
                } else {
                    normalized_fc_values_measures[j - 1].push(value);
                }
            }
        }

        for (i, identifier) in fc_identifier.iter().enumerate() {
            insert_first(&mut cep.normed_cep_curve_fc_values, identifier, normalized_fc_values_measures[i].clone());
            insert_first(&mut cep.idling_value_fc_values, identifier, idling_fc_values[i]);
        }

        // Looping through matrix and assigning values for pollutants
        let header_count = header_line_pollutants.len();
        for row in matrix_pollutants {
            for (j, &value) in row.iter().enumerate() {
                if row.len() != header_count + 1 {
                    return cep;
                }
                if j == 0 {
                    cep.normalized_power_pattern_pollutants.push(value);
                } else {
                    normalized_pollutant_measures[j - 1].push(value);
                }
            }
        }

        for (i, identifier) in pollutant_identifier.iter().enumerate() {
            insert_first(&mut cep.cep_normalized_curve_pollutants, identifier, normalized_pollutant_measures[i].clone());
            insert_first(&mut cep.idling_values_pollutants, identifier, idling_pollutants[i]);
        }

        cep.fleet_mix = BTreeMap::new();
        cep.fleet_mix.insert(STR_GASOLINE.to_string(), 0.0);
        cep.fleet_mix.insert(STR_DIESEL.to_string(), 0.0);
        cep.fleet_mix.insert(STR_CNG.to_string(), 0.0);
        cep.fleet_mix.insert(STR_LPG.to_string(), 0.0);
        cep
    }

    pub fn get_heavy_vehicle(&self) -> bool {
        self.private_heavy_vehicle
    }

    pub fn get_fuel_type(&self) -> &str {
        &self.private_fuel_type
    }

    pub fn set_fuel_type(&mut self, value: &str) {
        self.private_fuel_type = value.to_string();
    }

    pub fn get_calc_type(&self) -> &str {
        &self.private_calc_type
    }

    pub fn set_calc_type(&mut self, value: &str) {
        self.private_calc_type = value.to_string();
    }

    pub fn get_rated_power(&self) -> f64 {
        self.private_rated_power
    }

    pub fn set_rated_power(&mut self, value: f64) {
        self.private_rated_power = value;
    }

    pub fn get_aux_power(&self) -> f64 {
        self.aux_power * self.get_rated_power()
    }

    pub fn get_vehicle_mass(&self) -> f64 {
        self.mass_vehicle
    }

    pub fn get_vehicle_loading(&self) -> f64 {
        self.vehicle_loading
    }

    pub fn get_vehicle_mass_rot(&self) -> f64 {
        self.vehicle_mass_rot
    }

    pub fn get_cross_sectional_area(&self) -> f64 {
        self.cross_sectional_area
    }

    pub fn get_cw_value(&self) -> f64 {
        self.cw_value
    }

    pub fn get_wheel_radius(&self) -> f64 {
        self.effective_wheel_diameter / 2.0
    }

    pub fn get_resistance_f0(&self) -> f64 {
        self.resistance_f0
    }

    pub fn get_resistance(&self, speed: f64, f0: f64) -> f64 {
        f0 + self.resistance_f1 * speed + self.resistance_f2 * speed.powf(2.0) + self.resistance_f3 * speed.powf(3.0) + self.resistance_f4 * speed.powf(4.0)
    }

    pub fn calc_eng_power(&self, power: f64, rated_power: f64) -> f64 {
        let front = self.normalized_power_pattern_fc_values[0];
        if power < front * rated_power {
            return front * rated_power;
        }
        let back = self.normalized_power_pattern_fc_values[self.normalized_power_pattern_fc_values.len() - 1];
        if power > back * rated_power {
            return back * rated_power;
        }
        power
    }

    pub fn get_emission(&self, pollutant: &str, power: f64, speed: f64, vehicle_class: &mut Helpers, driving_power: f64, rated_power: f64) -> f64 {
        let heavy = self.get_heavy_vehicle();
        let mut emission_multiplier = if heavy { rated_power } else { 1.0 };

        let in_pollutants = self.cep_normalized_curve_pollutants.contains_key(pollutant);
        let in_fc = self.normed_cep_curve_fc_values.contains_key(pollutant);

        if speed.abs() <= ZERO_SPEED_ACCURACY {
            if !in_pollutants && !in_fc {
                vehicle_class.set_err_msg(&format!("Emission pollutant or fuel value {} not found!", pollutant));
                return 0.0;
            }
            if in_fc {
                return self.idling_value_fc_values.get(pollutant).copied().unwrap_or(0.0) * rated_power;
            } else if in_pollutants {
                return self.idling_values_pollutants.get(pollutant).copied().unwrap_or(0.0) * emission_multiplier;
            }
        }

        if !in_pollutants && !in_fc {
            vehicle_class.set_err_msg(&format!("Emission pollutant or fuel value {} not found!", pollutant));
            return 0.0;
        }

        let mut normalizing_power = rated_power;
        let mut emission_curve: Option<&Vec<f64>> = None;
        let mut power_pattern: &[f64] = &[];
        if in_fc {
            emission_curve = self.normed_cep_curve_fc_values.get(pollutant);
            power_pattern = &self.normalized_power_pattern_fc_values;
            emission_multiplier = rated_power;
        } else if in_pollutants {
            emission_curve = self.cep_normalized_curve_pollutants.get(pollutant);
            power_pattern = &self.normalized_power_pattern_pollutants;
            if !heavy {
                normalizing_power = driving_power;
            }
        }

        let emission_curve = match emission_curve {
            Some(curve) if !curve.is_empty() => curve,
            _ => {
                vehicle_class.set_err_msg(&format!("Empty emission curve for {} found!", pollutant));
                return 0.0;
            }
        };
        if emission_curve.len() == 1 {
            return emission_curve[0] * emission_multiplier;
        }

        // in case that the demanded power is smaller than the first entry (smallest) in the power pattern the first is returned (should never happen)
        if power <= power_pattern[0] * normalizing_power {
            return emission_curve[0] * emission_multiplier;
        }

        // if power bigger than all entries in power pattern return the last (should never happen)
        let last = power_pattern.len() - 1;
        if power >= power_pattern[last] * normalizing_power {
            return emission_curve[emission_curve.len() - 1] * emission_multiplier;
        }

        let (lower_index, upper_index) = find_lower_upper_in_pattern(power_pattern, power, normalizing_power);
        interpolate(
            power,
            power_pattern[lower_index] * normalizing_power,
            power_pattern[upper_index] * normalizing_power,
            emission_curve[lower_index],
            emission_curve[upper_index],
        ) * emission_multiplier
    }

    pub fn get_co2_emission(&self, fc: f64, co: f64, hc: f64, vehicle_class: &mut Helpers) -> f64 {
        let mut f_cbr = 0.0;
        let mut f_chc = 0.0;
        let mut f_cco = 0.0;
        let mut f_cco2 = 0.0;

        if self.get_fuel_type() != "Mix" {
            if !self.get_fc_vals(self.get_fuel_type(), &mut f_cbr, &mut f_chc, &mut f_cco, &mut f_cco2, vehicle_class) {
                return 0.0;
            }
        } else if !self.calc_cf_val_mix(&mut f_cbr, &mut f_chc, &mut f_cco, &mut f_cco2, vehicle_class) {
            return 0.0;
        }

        (fc * f_cbr - co * f_cco - hc * f_chc) / f_cco2
    }

    /// Calculate the weighted fuel factor values for Fleetmix
    fn calc_cf_val_mix(&self, f_cbr: &mut f64, f_chc: &mut f64, f_cco: &mut f64, f_cco2: &mut f64, vehicle_class: &mut Helpers) -> bool {
        let mut sum = 0.0;
        let mut sum_f_cbr = 0.0;
        let mut sum_f_chc = 0.0;
        let mut sum_f_cco = 0.0;
        let mut sum_f_cco2 = 0.0;

        // calculate the sum
        for mix in self.fleet_mix.values() {
            sum += mix;
        }

        // Calculate the weighted fuel factors
        if sum <= 0.0 {
            vehicle_class.set_err_msg("All propolsion types in the fleetshares file are not known!");
            return false;
        }
        for (fuel, mix) in &self.fleet_mix {
            if !self.get_fc_vals(fuel, f_cbr, f_chc, f_cco, f_cco2, vehicle_class) {
                return false;
            }
            sum_f_cbr += *f_cbr * mix / sum;
            sum_f_chc += *f_chc * mix / sum;
            sum_f_cco += *f_cco * mix / sum;
            sum_f_cco2 += *f_cco2 * mix / sum;
        }

        // Result values
        *f_cbr = sum_f_cbr;
        *f_chc = sum_f_chc;
        *f_cco = sum_f_cco;
        *f_cco2 = sum_f_cco2;
        true
    }

    /// Get the fuel factor values
    fn get_fc_vals(&self, fuel_type_x: &str, f_cbr: &mut f64, f_chc: &mut f64, f_cco: &mut f64, f_cco2: &mut f64, vehicle_class: &mut Helpers) -> bool {
        *f_chc = 0.866;
        *f_cco = 0.429;
        *f_cco2 = 0.273;

        if fuel_type_x == STR_GASOLINE {
            *f_cbr = 0.865;
        } else if fuel_type_x == STR_DIESEL {
            *f_cbr = 0.863;
        } else if fuel_type_x == STR_CNG {
            *f_cbr = 0.693;
            *f_chc = 0.803;
        } else if fuel_type_x == STR_LPG {
            *f_cbr = 0.825;
            *f_chc = 0.825;
        } else {
            vehicle_class.set_err_msg(&format!("The propulsion type is not known! ({})", self.get_fuel_type()));
            return false;
        }
        true
    }

    pub fn get_f_mot(&self, speed: f64, rated_power: f64, wheel_radius: f64) -> f64 {
        if speed < 10e-2 {
            return 0.0;
        }

        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.speed_pattern_rotational, speed, 1.0);
        let i_gear = interpolate(
            speed,
            self.speed_pattern_rotational[lower_index],
            self.speed_pattern_rotational[upper_index],
            self.gear_transmission_curve[lower_index],
            self.gear_transmission_curve[upper_index],
        );

        let i_tot = i_gear * self.axle_ratio;

        let n = (30.0 * speed * i_tot) / (wheel_radius * std::f64::consts::PI);
        let n_norm = (n - self.engine_idling_speed) / (self.engine_rated_speed - self.engine_idling_speed);

        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.n_norm_table, n_norm, 1.0);
        (-interpolate(
            n_norm,
            self.n_norm_table[lower_index],
            self.n_norm_table[upper_index],
            self.drag_norm_table[lower_index],
            self.drag_norm_table[upper_index],
        ) * rated_power
            * 1000.0
            / speed)
            / get_drive_train_efficiency()
    }

    pub fn get_rotational_coeffecient(&self, speed: f64) -> f64 {
        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.speed_pattern_rotational, speed, 1.0);
        interpolate(
            speed,
            self.speed_pattern_rotational[lower_index],
            self.speed_pattern_rotational[upper_index],
            self.speed_curve_rotational[lower_index],
            self.speed_curve_rotational[upper_index],
        )
    }

    pub fn get_p_max_norm(&self, speed: f64) -> f64 {
        // Linear function between v0 and v1, constant elsewhere
        if speed <= self.p_norm_v0 {
            self.p_norm_p0
        } else if speed >= self.p_norm_v1 {
            self.p_norm_p1
        } else {
            interpolate(speed, self.p_norm_v0, self.p_norm_v1, self.p_norm_p0, self.p_norm_p1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisection_with_scale_matches_scaled_pattern() {
        let pattern = [1.0, 2.0, 3.0, 4.0];
        // scale 2: the scaled pattern is [2, 4, 6, 8]
        assert_eq!(find_lower_upper_in_pattern(&pattern, 1.0, 2.0), (0, 0));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 4.0, 2.0), (1, 1));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 5.0, 2.0), (1, 2));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 9.0, 2.0), (3, 3));
    }
}
