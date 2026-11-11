//! Port of foreign/PHEMlight/cpp/CEP.{h,cpp}

use std::collections::BTreeMap;

use super::constants::*;
use super::helpers::Helpers;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum NormalizingType {
    #[default]
    RatedPower,
    DrivingPower,
}

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

/// Port of `CEP::FindLowerUpperInPattern`. Returns `(lowerIndex, upperIndex)`.
fn find_lower_upper_in_pattern(pattern: &[f64], value: f64) -> (usize, usize) {
    if value <= pattern[0] {
        return (0, 0);
    }

    let last = pattern.len() - 1;
    if value >= pattern[last] {
        return (last, last);
    }

    // bisection search to find correct position in power pattern
    let mut middle_index = last / 2;
    let mut upper_index = last;
    let mut lower_index = 0;

    while upper_index - lower_index > 1 {
        if pattern[middle_index] == value {
            return (middle_index, middle_index);
        } else if pattern[middle_index] < value {
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
    heavy_vehicle: bool,
    fuel_type: String,
    normalizing_type: NormalizingType,
    rated_power: f64,
    normalizing_power: f64,
    driving_power: f64,

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
    power_pattern_fc: Vec<f64>,
    normalized_power_pattern_fc: Vec<f64>,
    // C++ member name `_normailzedPowerPatternPollutants` (typo kept out of the Rust name)
    normalized_power_pattern_pollutants: Vec<f64>,
    power_pattern_pollutants: Vec<f64>,

    cep_curve_fc: Vec<f64>,
    normed_cep_curve_fc: Vec<f64>,
    gear_transmission_curve: Vec<f64>,
    speed_curve_rotational: Vec<f64>,
    cep_curve_pollutants: BTreeMap<String, Vec<f64>>,
    cep_normalized_curve_pollutants: BTreeMap<String, Vec<f64>>,
    idling_value_fc: f64,
    idling_values_pollutants: BTreeMap<String, f64>,

    n_norm_table: Vec<f64>,
    drag_norm_table: Vec<f64>,
}

impl CEP {
    /// Port of the `CEP` constructor. Malformed matrices abort the fill early, exactly like the C++
    /// `return` statements, leaving the remaining fields at their default values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        heavy_vehicle: bool,
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
        transmission_gear_ratios: &[f64],
        aux_power: f64,
        rated_power: f64,
        engine_idling_speed: f64,
        engine_rated_speed: f64,
        effective_wheel_diameter: f64,
        p_norm_v0: f64,
        p_norm_p0: f64,
        p_norm_v1: f64,
        p_norm_p1: f64,
        vehicle_fuel_type: &str,
        matrix_fc: &[Vec<f64>],
        header_line_pollutants: &[String],
        matrix_pollutants: &[Vec<f64>],
        matrix_speed_rotational: &[Vec<f64>],
        normed_drag_table: &[Vec<f64>],
        idling_fc: f64,
        idling_pollutants: &[f64],
    ) -> Self {
        // transmission_gear_ratios is unused in the C++ constructor as well.
        let _ = transmission_gear_ratios;
        let mut cep = CEP {
            resistance_f0: f0,
            resistance_f1: f1,
            resistance_f2: f2,
            resistance_f3: f3,
            resistance_f4: f4,
            cw_value,
            cross_sectional_area: cross_area,
            mass_vehicle: vehicle_mass,
            vehicle_loading,
            vehicle_mass_rot,
            rated_power,
            engine_idling_speed,
            engine_rated_speed,
            effective_wheel_diameter,
            heavy_vehicle,
            fuel_type: vehicle_fuel_type.to_string(),
            axle_ratio,
            aux_power,
            p_norm_v0: p_norm_v0 / 3.6,
            p_norm_p0,
            p_norm_v1: p_norm_v1 / 3.6,
            p_norm_p1,
            ..Default::default()
        };

        // looping through matrix and assigning values for speed rotational table
        for row in matrix_speed_rotational {
            if row.len() != 3 {
                return cep;
            }
            cep.speed_pattern_rotational.push(row[0] / 3.6);
            cep.gear_transmission_curve.push(row[1]);
            cep.speed_curve_rotational.push(row[2]);
        }

        // looping through matrix and assigning values for drag table
        for row in normed_drag_table {
            if row.len() != 2 {
                return cep;
            }
            cep.n_norm_table.push(row[0]);
            cep.drag_norm_table.push(row[1]);
        }

        // looping through matrix and assigning values for Fuel consumption
        for row in matrix_fc {
            if row.len() != 2 {
                return cep;
            }
            cep.power_pattern_fc.push(row[0] * cep.rated_power);
            cep.normalized_power_pattern_fc.push(row[0]);
            cep.cep_curve_fc.push(row[1] * cep.rated_power);
            cep.normed_cep_curve_fc.push(row[1]);
        }

        let mut pollutant_multiplyer = 1.0;

        cep.driving_power = cep.calc_power(NORMALIZING_SPEED, NORMALIZING_ACCELARATION, 0.0);
        cep.normalizing_power = cep.driving_power;

        // looping through matrix and assigning values for pollutants
        if heavy_vehicle {
            cep.normalizing_power = cep.rated_power;
            cep.normalizing_type = NormalizingType::RatedPower;
            pollutant_multiplyer = cep.rated_power;
        } else {
            cep.normalizing_power = cep.driving_power;
            cep.normalizing_type = NormalizingType::DrivingPower;
        }

        let header_count = header_line_pollutants.len();
        let mut pollutant_measures: Vec<Vec<f64>> = vec![Vec::new(); header_count];
        let mut normalized_pollutant_measures: Vec<Vec<f64>> = vec![Vec::new(); header_count];
        for row in matrix_pollutants {
            for (j, &value) in row.iter().enumerate() {
                if row.len() != header_count + 1 {
                    return cep;
                }
                if j == 0 {
                    cep.normalized_power_pattern_pollutants.push(value);
                    cep.power_pattern_pollutants.push(value * cep.normalizing_power);
                } else {
                    pollutant_measures[j - 1].push(value * pollutant_multiplyer);
                    normalized_pollutant_measures[j - 1].push(value);
                }
            }
        }

        for (i, identifier) in header_line_pollutants.iter().enumerate() {
            insert_first(&mut cep.cep_curve_pollutants, identifier, pollutant_measures[i].clone());
            insert_first(&mut cep.cep_normalized_curve_pollutants, identifier, normalized_pollutant_measures[i].clone());
            insert_first(&mut cep.idling_values_pollutants, identifier, idling_pollutants[i] * pollutant_multiplyer);
        }

        cep.idling_value_fc = idling_fc * cep.rated_power;
        cep
    }

    pub fn get_heavy_vehicle(&self) -> bool {
        self.heavy_vehicle
    }

    pub fn get_fuel_type(&self) -> &str {
        &self.fuel_type
    }

    pub fn get_normalizing_type_x(&self) -> NormalizingType {
        self.normalizing_type
    }

    pub fn get_rated_power(&self) -> f64 {
        self.rated_power
    }

    pub fn set_rated_power(&mut self, value: f64) {
        self.rated_power = value;
    }

    pub fn get_normalizing_power(&self) -> f64 {
        self.normalizing_power
    }

    pub fn get_driving_power(&self) -> f64 {
        self.driving_power
    }

    pub fn set_driving_power(&mut self, value: f64) {
        self.driving_power = value;
    }

    pub fn calc_power(&self, speed: f64, acc: f64, gradient: f64) -> f64 {
        // Declaration
        let mut power = 0.0;
        let rot_factor = self.get_rotational_coeffecient(speed);
        let power_aux = self.aux_power * self.rated_power;

        // Calculate the power
        power += (self.mass_vehicle + self.vehicle_loading) * GRAVITY_CONST * (self.resistance_f0 + self.resistance_f1 * speed + self.resistance_f4 * speed.powf(4.0)) * speed;
        power += (self.cross_sectional_area * self.cw_value * AIR_DENSITY_CONST / 2.0) * speed.powf(3.0);
        power += (self.mass_vehicle * rot_factor + self.vehicle_mass_rot + self.vehicle_loading) * acc * speed;
        power += (self.mass_vehicle + self.vehicle_loading) * GRAVITY_CONST * gradient * 0.01 * speed;
        power /= 1000.0;
        power /= get_drive_train_efficiency();
        power += power_aux;

        // Return result
        power
    }

    pub fn calc_eng_power(&self, power: f64) -> f64 {
        let front = self.power_pattern_fc[0];
        if power < front {
            return front;
        }
        let back = self.power_pattern_fc[self.power_pattern_fc.len() - 1];
        if power > back {
            return back;
        }
        power
    }

    pub fn get_emission(&self, pollutant: &str, power: f64, speed: f64, vehicle_class: &mut Helpers) -> f64 {
        // bisection search to find correct position in power pattern
        if self.fuel_type != STR_BEV && speed.abs() <= ZERO_SPEED_ACCURACY {
            if pollutant == "FC" {
                return self.idling_value_fc;
            }
            if !self.cep_curve_pollutants.contains_key(pollutant) {
                vehicle_class.set_err_msg(&format!("Emission pollutant {} not found!", pollutant));
                return 0.0;
            }
            return self.idling_values_pollutants.get(pollutant).copied().unwrap_or(0.0);
        }

        let (emission_curve, power_pattern): (&[f64], &[f64]) = if pollutant == "FC" {
            (&self.cep_curve_fc, &self.power_pattern_fc)
        } else {
            match self.cep_curve_pollutants.get(pollutant) {
                Some(curve) => (curve, &self.power_pattern_pollutants),
                None => {
                    vehicle_class.set_err_msg(&format!("Emission pollutant {} not found!", pollutant));
                    return 0.0;
                }
            }
        };

        if emission_curve.is_empty() {
            vehicle_class.set_err_msg(&format!("Empty emission curve for {} found!", pollutant));
            return 0.0;
        }
        if emission_curve.len() == 1 {
            return emission_curve[0];
        }

        // in case that the demanded power is smaller than the first entry (smallest) in the power pattern the first is returned (should never happen)
        if power <= power_pattern[0] {
            return emission_curve[0];
        }

        // if power bigger than all entries in power pattern return the last (should never happen)
        if power >= power_pattern[power_pattern.len() - 1] {
            return emission_curve[emission_curve.len() - 1];
        }

        let (lower_index, upper_index) = find_lower_upper_in_pattern(power_pattern, power);
        interpolate(power, power_pattern[lower_index], power_pattern[upper_index], emission_curve[lower_index], emission_curve[upper_index])
    }

    pub fn get_co2_emission(&self, fc: f64, co: f64, hc: f64, vehicle_class: &mut Helpers) -> f64 {
        let f_cbr;
        let mut f_chc = 0.866;
        let f_cco = 0.429;
        let f_cco2 = 0.273;

        if self.fuel_type == STR_GASOLINE {
            f_cbr = 0.865;
        } else if self.fuel_type == STR_DIESEL {
            f_cbr = 0.863;
        } else if self.fuel_type == STR_CNG {
            f_cbr = 0.693;
            f_chc = 0.803;
        } else if self.fuel_type == STR_LPG {
            f_cbr = 0.825;
            f_chc = 0.825;
        } else {
            vehicle_class.set_err_msg(&format!("The propolsion type is not known! ({})", self.fuel_type));
            return 0.0;
        }

        (fc * f_cbr - co * f_cco - hc * f_chc) / f_cco2
    }

    pub fn get_decel_coast(&self, speed: f64, acc: f64, gradient: f64) -> f64 {
        if speed < SPEED_DCEL_MIN {
            return speed / SPEED_DCEL_MIN * self.get_decel_coast(SPEED_DCEL_MIN, acc, gradient);
        }

        let rot_coeff = self.get_rotational_coeffecient(speed);
        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.speed_pattern_rotational, speed);
        let i_gear = interpolate(
            speed,
            self.speed_pattern_rotational[lower_index],
            self.speed_pattern_rotational[upper_index],
            self.gear_transmission_curve[lower_index],
            self.gear_transmission_curve[upper_index],
        );

        let i_tot = i_gear * self.axle_ratio;

        let n = (30.0 * speed * i_tot) / ((self.effective_wheel_diameter / 2.0) * std::f64::consts::PI);
        let n_norm = (n - self.engine_idling_speed) / (self.engine_rated_speed - self.engine_idling_speed);

        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.n_norm_table, n_norm);

        let mut f_mot = 0.0;

        if speed >= 10e-2 {
            f_mot = (-interpolate(
                n_norm,
                self.n_norm_table[lower_index],
                self.n_norm_table[upper_index],
                self.drag_norm_table[lower_index],
                self.drag_norm_table[upper_index],
            ) * self.rated_power
                * 1000.0
                / speed)
                / 0.9;
        }

        let f_roll = (self.resistance_f0
            + self.resistance_f1 * speed
            + (self.resistance_f2 * speed).powf(2.0)
            + (self.resistance_f3 * speed).powf(3.0)
            + (self.resistance_f4 * speed).powf(4.0))
            * (self.mass_vehicle + self.vehicle_loading)
            * GRAVITY_CONST;

        let f_air = self.cw_value * self.cross_sectional_area * 1.2 * 0.5 * speed.powf(2.0);

        let f_grad = (self.mass_vehicle + self.vehicle_loading) * GRAVITY_CONST * gradient / 100.0;

        -(f_mot + f_roll + f_air + f_grad) / ((self.mass_vehicle + self.vehicle_loading) * rot_coeff)
    }

    pub fn get_rotational_coeffecient(&self, speed: f64) -> f64 {
        let (lower_index, upper_index) = find_lower_upper_in_pattern(&self.speed_pattern_rotational, speed);
        interpolate(
            speed,
            self.speed_pattern_rotational[lower_index],
            self.speed_pattern_rotational[upper_index],
            self.speed_curve_rotational[lower_index],
            self.speed_curve_rotational[upper_index],
        )
    }

    pub fn get_max_accel(&self, speed: f64, gradient: f64) -> f64 {
        let rot_factor = self.get_rotational_coeffecient(speed);
        let p_max_for_acc = self.get_p_max_norm(speed) * self.rated_power - self.calc_power(speed, 0.0, gradient);

        (p_max_for_acc * 1000.0) / ((self.mass_vehicle * rot_factor + self.vehicle_mass_rot + self.vehicle_loading) * speed)
    }

    fn get_p_max_norm(&self, speed: f64) -> f64 {
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
    fn interpolate_matches_cpp_formula() {
        // e1 + (px - p1) / (p2 - p1) * (e2 - e1)
        assert_eq!(interpolate(5.0, 0.0, 10.0, 1.0, 3.0), 2.0);
        // equal pattern points return the first emission value
        assert_eq!(interpolate(5.0, 2.0, 2.0, 7.0, 9.0), 7.0);
    }

    #[test]
    fn bisection_returns_exact_and_bracketing_indices() {
        let pattern = [0.0, 1.0, 2.0, 3.0, 4.0];
        assert_eq!(find_lower_upper_in_pattern(&pattern, -1.0), (0, 0));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 9.0), (4, 4));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 2.0), (2, 2));
        assert_eq!(find_lower_upper_in_pattern(&pattern, 2.5), (2, 3));
    }

    #[test]
    fn co2_uses_fuel_factors() {
        let mut h = Helpers::default();
        let cep = CEP { fuel_type: STR_GASOLINE.to_string(), ..Default::default() };
        // (FC*0.865 - CO*0.429 - HC*0.866) / 0.273 with FC=1, CO=0, HC=0
        assert_eq!(cep.get_co2_emission(1.0, 0.0, 0.0, &mut h), 0.865 / 0.273);
        let bad = CEP { fuel_type: "X".to_string(), ..Default::default() };
        assert_eq!(bad.get_co2_emission(1.0, 0.0, 0.0, &mut h), 0.0);
        assert_eq!(h.get_err_msg(), "The propolsion type is not known! (X)");
    }
}
