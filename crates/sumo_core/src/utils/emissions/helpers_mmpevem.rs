//! Port of utils/emissions/HelpersMMPEVEM.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::geom_helper::deg2rad;

/// C++ `HelpersMMPEVEM::MMPEVEM_BASE` (5 << 16)
pub const MMPEVEM_BASE: i32 = 5 << 16;

/// This helper class allows the PollutantsInterface to load and use different MMPEVEMs.
/// C++ class `HelpersMMPEVEM` (utils/emissions/HelpersMMPEVEM.h)
pub struct HelpersMMPEVEM {
    /// C++ base class `PollutantsInterface::Helper`
    pub base: PollutantsInterface_Helper,
}

impl std::ops::Deref for HelpersMMPEVEM {
    type Target = PollutantsInterface_Helper;
    fn deref(&self) -> &PollutantsInterface_Helper {
        &self.base
    }
}
impl std::ops::DerefMut for HelpersMMPEVEM {
    fn deref_mut(&mut self) -> &mut PollutantsInterface_Helper {
        &mut self.base
    }
}

// C++ static members of HelpersMMPEVEM (port as module-level statics, see AGENTS.md):
//   static const int HelpersMMPEVEM::MMPEVEM_BASE;

impl HelpersMMPEVEM {
    /// C++: `void HelpersMMPEVEM::HelpersMMPEVEM()`
    /// Constructor
    // HelpersMMPEVEM::HelpersMMPEVEM()
    //     : PollutantsInterface::Helper("MMPEVEM", MMPEVEM_BASE, MMPEVEM_BASE + 1)
    // { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { base: PollutantsInterface_Helper::new("MMPEVEM".to_string(), MMPEVEM_BASE, MMPEVEM_BASE + 1) }
    }

    /// C++: `std::string HelpersMMPEVEM::getFuel(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the fuel type described by this emission class as described in the Amitran interface (Gasoline, Diesel, ...)
    // std::string getFuel(const SUMOEmissionClass /* c */) const {
    //         return "Electricity";
    //     }
    #[allow(unused_variables)]
    pub fn get_fuel(&self, arg0: SUMOEmissionClass) -> String {
        "Electricity".to_string()
    }

    /// C++: `double HelpersMMPEVEM::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class This implementation returns the default mass for this model.
    // double getWeight(const SUMOEmissionClass /* c */) const {
    //         return 1794.;
    //     }
    #[allow(unused_variables)]
    pub fn get_weight(&self, arg0: SUMOEmissionClass) -> f64 {
        1794.
    }

    /// C++: `double HelpersMMPEVEM::compute(const SUMOEmissionClass, const PollutantsInterface::EmissionType, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Compute the amount of emitted pollutants for an emission class in a given state.
    // double HelpersMMPEVEM::compute(const SUMOEmissionClass c,
    //                                const PollutantsInterface::EmissionType e, const double v, const double a,
    //                                const double slope, const EnergyParams* ptr_energyParams) const {
    //     if (e != PollutantsInterface::ELEC) {
    //         return 0.0;
    //     }
    //
    //     // Extract all required parameters, default values taken from the VW_ID3
    //     // Vehicle mass [kg]
    //     const double m = ptr_energyParams->getTotalMass(getWeight(c), 0.);
    //     // Wheel radius [m]
    //     const double r_wheel = ptr_energyParams->getDoubleOptional(SUMO_ATTR_WHEELRADIUS, 0.3588);
    //     // Internal moment of inertia [kgm^2]
    //     const double Theta = ptr_energyParams->getDoubleOptional(SUMO_ATTR_INTERNALMOMENTOFINERTIA, 12.5);
    //     // Rolling resistance coefficient
    //     const double c_rr = ptr_energyParams->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, 0.007);
    //     // Air drag coefficient
    //     const double c_d = ptr_energyParams->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT, 0.26);
    //     // Cross-sectional area of the front of the car [m^2]
    //     const double A_front = ptr_energyParams->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA, 2.36);
    //     // Gear ratio [1]
    //     const double i_gear = ptr_energyParams->getDoubleOptional(SUMO_ATTR_GEARRATIO, 10.);
    //     // Gearbox efficiency [1]
    //     const double eta_gear = ptr_energyParams->getDoubleOptional(SUMO_ATTR_GEAREFFICIENCY, 0.96);
    //     // Maximum torque [Nm]
    //     const double M_max = ptr_energyParams->getDoubleOptional(SUMO_ATTR_MAXIMUMTORQUE, 310.);
    //     // Maximum power [W]
    //     const double P_max = ptr_energyParams->getDoubleOptional(SUMO_ATTR_MAXIMUMPOWER, 107000.);
    //     // Maximum recuperation torque [Nm]
    //     const double M_recup_max = ptr_energyParams->getDoubleOptional(SUMO_ATTR_MAXIMUMRECUPERATIONTORQUE, 95.5);
    //     // Maximum recuperation power [W]
    //     const double P_recup_max = ptr_energyParams->getDoubleOptional(SUMO_ATTR_MAXIMUMRECUPERATIONPOWER, 42800.);
    //     // Internal battery resistance [Ohm]
    //     const double R_battery = ptr_energyParams->getDoubleOptional(SUMO_ATTR_INTERNALBATTERYRESISTANCE, 0.1142);
    //     // Nominal battery voltage [V]
    //     const double U_battery_0 = ptr_energyParams->getDoubleOptional(SUMO_ATTR_NOMINALBATTERYVOLTAGE, 396.);
    //     // Constant power consumption [W]
    //     const double P_const = ptr_energyParams->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE, 360.);
    //     // Power loss map
    //     const CharacteristicMap& ref_powerLossMap = ptr_energyParams->getCharacteristicMap(SUMO_ATTR_POWERLOSSMAP);
    //
    //     double P = 0.0;  // [W]
    //     bool b_stateValid = calcPowerConsumption(m, r_wheel, Theta, c_rr, c_d,
    //                         A_front, i_gear, eta_gear, M_max, P_max, M_recup_max, P_recup_max,
    //                         R_battery, U_battery_0, P_const, ref_powerLossMap, TS, v, a, slope, P);
    //     P /= 3600.0;  // [Wh/s]
    //
    //     if (b_stateValid) {
    //         return P;
    //     } else {
    //         return std::nan("");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, ptr_energyParams: Ptr<EnergyParams>) -> f64 {
        if e != PollutantsInterface_EmissionType::ELEC {
            return 0.0;
        }
        // Extract all required parameters, default values taken from the VW_ID3
        let p = ptr_energyParams.m();
        // Vehicle mass [kg]
        let m = p.get_total_mass(self.get_weight(c), 0.);
        // Wheel radius [m]
        let r_wheel = p.get_double_optional(SUMO_ATTR_WHEELRADIUS, 0.3588);
        // Internal moment of inertia [kgm^2]
        let Theta = p.get_double_optional(SUMO_ATTR_INTERNALMOMENTOFINERTIA, 12.5);
        // Rolling resistance coefficient
        let c_rr = p.get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, 0.007);
        // Air drag coefficient
        let c_d = p.get_double_optional(SUMO_ATTR_AIRDRAGCOEFFICIENT, 0.26);
        // Cross-sectional area of the front of the car [m^2]
        let A_front = p.get_double_optional(SUMO_ATTR_FRONTSURFACEAREA, 2.36);
        // Gear ratio [1]
        let i_gear = p.get_double_optional(SUMO_ATTR_GEARRATIO, 10.);
        // Gearbox efficiency [1]
        let eta_gear = p.get_double_optional(SUMO_ATTR_GEAREFFICIENCY, 0.96);
        // Maximum torque [Nm]
        let M_max = p.get_double_optional(SUMO_ATTR_MAXIMUMTORQUE, 310.);
        // Maximum power [W]
        let P_max = p.get_double_optional(SUMO_ATTR_MAXIMUMPOWER, 107000.);
        // Maximum recuperation torque [Nm]
        let M_recup_max = p.get_double_optional(SUMO_ATTR_MAXIMUMRECUPERATIONTORQUE, 95.5);
        // Maximum recuperation power [W]
        let P_recup_max = p.get_double_optional(SUMO_ATTR_MAXIMUMRECUPERATIONPOWER, 42800.);
        // Internal battery resistance [Ohm]
        let R_battery = p.get_double_optional(SUMO_ATTR_INTERNALBATTERYRESISTANCE, 0.1142);
        // Nominal battery voltage [V]
        let U_battery_0 = p.get_double_optional(SUMO_ATTR_NOMINALBATTERYVOLTAGE, 396.);
        // Constant power consumption [W]
        let P_const = p.get_double_optional(SUMO_ATTR_CONSTANTPOWERINTAKE, 360.);
        // Power loss map
        let ref_powerLossMap = p.get_characteristic_map(SUMO_ATTR_POWERLOSSMAP);

        let mut P = 0.0; // [W]
        let b_stateValid = calc_power_consumption(m, r_wheel, Theta, c_rr, c_d,
                            A_front, i_gear, eta_gear, M_max, P_max, M_recup_max, P_recup_max,
                            R_battery, U_battery_0, P_const, ref_powerLossMap, utils::common::time::ts(), v, a, slope, &mut P);
        P /= 3600.0; // [Wh/s]

        if b_stateValid {
            P
        } else {
            f64::NAN
        }
    }

}

/// C++: `bool (double, double, double, double, double, double, double, double, double, double, double, double, double, double, double, const CharacteristicMap &, double, double, double, double, double &) calcPowerConsumption`
// bool calcPowerConsumption(double m, double r_wheel, double Theta, double c_rr,
//                           double c_d, double A_front, double i_gear, double eta_gear, double M_max,
//                           double P_max, double M_recup_max, double P_recup_max, double R_battery,
//                           double U_battery_0, double P_const,
//                           const CharacteristicMap& ref_powerLossMap, double dt, double v, double a,
//                           double alpha, double& ref_powerConsumption) {
//     const double EPS = 1e-6;
//     const double RHO_AIR = 1.204;  // Air density [kg/m^3]
//     bool b_stateValid = true;
//
//     // Mass factor [1]
//     const double e_i = 1.0 + Theta / (m * r_wheel * r_wheel);
//     // Average speed during the previous timestep
//     const double v_mean = v - 0.5 * a * dt;
//
//     // Force required for the desired acceleration [N]
//     double F_a = m * a * e_i;
//     // Grade resistance [N]
//     double F_gr = m * GRAVITY * std::sin(DEG2RAD(alpha));
//     // Rolling resistance [N]
//     double F_rr = m * GRAVITY * std::cos(DEG2RAD(alpha)) * c_rr;
//     if (std::abs(v_mean) <= EPS) {
//         F_rr = 0;
//     }
//     // Drag [N]
//     double F_d = 0.5 * c_d * A_front * RHO_AIR * v_mean * v_mean;
//     // Tractive force [N]
//     const double F_tractive = F_a + F_gr + F_rr + F_d;
//
//     // Speed of the motor [rpm]
//     const double n_motor = v_mean / (2 * M_PI * r_wheel) * 60 * i_gear;
//     // Angular velocity of the motor [1/s]
//     double omega_motor = 2 * M_PI * n_motor / 60;
//     if (omega_motor == 0) {
//         omega_motor = EPS;
//     }
//
//     // Torque at the motor [Nm]. Please note that this model, like most real EVs,
//     // utilizes the EM to hold the vehicle on an incline rather than the brakes.
//     double M_motor = F_tractive * r_wheel / i_gear;
//     if (F_tractive < 0) {
//         M_motor *= eta_gear;
//     } else {
//         M_motor /= eta_gear;
//     }
//     // Engine power [W]
//     double P_motor = M_motor * omega_motor;
//
//     // Cap torque or power if necessary
//     // Accelerating
//     if (M_motor >= 0) {
//         if (M_motor > M_max) {
//             M_motor = M_max;
//             P_motor = M_motor * omega_motor;
//             b_stateValid = false;
//         }
//         if (P_motor > P_max) {
//             P_motor = P_max;
//             M_motor = P_motor / omega_motor;
//             b_stateValid = false;
//         }
//     }
//     // Recuperating
//     else {
//         // Even when capping, the state is still valid here because the extra energy
//         // is assumed to go to the brakes
//         if (M_motor < -M_recup_max) {
//             M_motor = -M_recup_max;
//             P_motor = M_motor * omega_motor;
//         }
//         if (P_motor < -P_recup_max) {
//             P_motor = -P_recup_max;
//             M_motor = P_motor / omega_motor;
//         }
//     }
//
//     // Power loss in the electric motor + inverter [W]
//     double P_loss_motor =
//         ref_powerLossMap.eval(std::vector<double> {n_motor, M_motor})[0];
//     if (std::isnan(P_loss_motor)) {
//         P_loss_motor = 0.0;
//         b_stateValid = false;
//     }
//
//     // Power demand at the battery [W]
//     double P_battery = P_motor + P_loss_motor + P_const;
//     // Total power demand (including the power loss in the battery) [W]
//     double P_total = (U_battery_0 * U_battery_0) / (2.0 * R_battery)
//                      - U_battery_0 * std::sqrt((U_battery_0 * U_battery_0
//                              - 4.0 * R_battery * P_battery) / (4.0 * R_battery * R_battery));
//
//     ref_powerConsumption = P_total;
//     return b_stateValid;
// }
#[allow(unused_variables)]
pub fn calc_power_consumption(m: f64, r_wheel: f64, Theta: f64, c_rr: f64, c_d: f64, A_front: f64, i_gear: f64, eta_gear: f64, M_max: f64, P_max: f64, M_recup_max: f64, P_recup_max: f64, R_battery: f64, U_battery_0: f64, P_const: f64, ref_powerLossMap: &CharacteristicMap, dt: f64, v: f64, a: f64, alpha: f64, ref_powerConsumption: &mut f64) -> bool {
    let EPS = 1e-6;
    let RHO_AIR = 1.204; // Air density [kg/m^3]
    let mut b_stateValid = true;

    // Mass factor [1]
    let e_i = 1.0 + Theta / (m * r_wheel * r_wheel);
    // Average speed during the previous timestep
    let v_mean = v - 0.5 * a * dt;

    // Force required for the desired acceleration [N]
    let mut F_a = m * a * e_i;
    // Grade resistance [N]
    let F_gr = m * utils::geom::geom_helper::GRAVITY * deg2rad(alpha).sin();
    // Rolling resistance [N]
    let mut F_rr = m * utils::geom::geom_helper::GRAVITY * deg2rad(alpha).cos() * c_rr;
    if v_mean.abs() <= EPS {
        F_rr = 0.;
    }
    // Drag [N]
    let F_d = 0.5 * c_d * A_front * RHO_AIR * v_mean * v_mean;
    // Tractive force [N]
    let F_tractive = F_a + F_gr + F_rr + F_d;

    // Speed of the motor [rpm]
    let n_motor = v_mean / (2. * utils::geom::geom_helper::M_PI * r_wheel) * 60. * i_gear;
    // Angular velocity of the motor [1/s]
    let mut omega_motor = 2. * utils::geom::geom_helper::M_PI * n_motor / 60.;
    if omega_motor == 0. {
        omega_motor = EPS;
    }

    // Torque at the motor [Nm]. Please note that this model, like most real EVs,
    // utilizes the EM to hold the vehicle on an incline rather than the brakes.
    let mut M_motor = F_tractive * r_wheel / i_gear;
    if F_tractive < 0. {
        M_motor *= eta_gear;
    } else {
        M_motor /= eta_gear;
    }
    // Engine power [W]
    let mut P_motor = M_motor * omega_motor;

    // Cap torque or power if necessary
    // Accelerating
    if M_motor >= 0. {
        if M_motor > M_max {
            M_motor = M_max;
            P_motor = M_motor * omega_motor;
            b_stateValid = false;
        }
        if P_motor > P_max {
            P_motor = P_max;
            M_motor = P_motor / omega_motor;
            b_stateValid = false;
        }
    }
    // Recuperating
    else {
        // Even when capping, the state is still valid here because the extra energy
        // is assumed to go to the brakes
        if M_motor < -M_recup_max {
            M_motor = -M_recup_max;
            P_motor = M_motor * omega_motor;
        }
        if P_motor < -P_recup_max {
            P_motor = -P_recup_max;
            M_motor = P_motor / omega_motor;
        }
    }

    // Power loss in the electric motor + inverter [W]
    let mut P_loss_motor = ref_powerLossMap.eval(&vec![n_motor, M_motor], 1e-6)[0];
    if P_loss_motor.is_nan() {
        P_loss_motor = 0.0;
        b_stateValid = false;
    }

    // Power demand at the battery [W]
    let P_battery = P_motor + P_loss_motor + P_const;
    // Total power demand (including the power loss in the battery) [W]
    let P_total = (U_battery_0 * U_battery_0) / (2.0 * R_battery)
        - U_battery_0 * ((U_battery_0 * U_battery_0 - 4.0 * R_battery * P_battery) / (4.0 * R_battery * R_battery)).sqrt();

    *ref_powerConsumption = P_total;
    b_stateValid
}

