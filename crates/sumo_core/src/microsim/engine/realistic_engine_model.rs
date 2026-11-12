//! Port of microsim/engine/RealisticEngineModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::xml::xml_sub_sys::run_parser;

/// This class implements a detailed engine and braking model which takes into account engine characteristics, gear ratios, external forces due to air drag, etc. This class uses a particular notation for variables to take into account both name and unit, to avoid confusion and mistakes. We write the name in camel case, then an underscore, then the unit of measure. For example, engine force in newton (kg * m / s^2) we use
/// C++ class `RealisticEngineModel` (microsim/engine/RealisticEngineModel.h)
pub struct RealisticEngineModel {
    /// C++ base class `GenericEngineModel`
    pub base: GenericEngineModel,
    pub ep: EngineParameters,
    pub currentGear: i32,
    pub minSpeed_mps: f64,
    pub dt_s: f64,
    pub xmlFile: String,
    pub vehicleType: String,
}

impl std::ops::Deref for RealisticEngineModel {
    type Target = GenericEngineModel;
    fn deref(&self) -> &GenericEngineModel {
        &self.base
    }
}
impl std::ops::DerefMut for RealisticEngineModel {
    fn deref_mut(&mut self) -> &mut GenericEngineModel {
        &mut self.base
    }
}

const GRAVITY_MPS2: f64 = 9.81; // EngineParameters.h #define
const HP_TO_W: f64 = 745.699872; // EngineParameters.h #define
const TAU_MAX: f64 = 0.5; // EngineParameters.h #define
const ENGINE_PAR_VEHICLE: &str = "vehicle"; // CC_Const.h
const ENGINE_PAR_XMLFILE: &str = "xmlFile"; // CC_Const.h
const ENGINE_PAR_DT: &str = "dt_s"; // CC_Const.h

/// C++ `ep.gearRatios[i]` (gearRatios is a raw double array)
fn gear_ratio(ep: &EngineParameters, i: i32) -> f64 {
    // SAFETY: gearRatios points at nGears values, as in C++
    unsafe { *ep.gearRatios.as_raw().add(i as usize) }
}

/// C++ std::min(a, b): returns (b < a) ? b : a
fn cpp_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}

/// C++ std::max(a, b): returns (a < b) ? b : a
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

/// shortcut: EngineParameters_PolynomialEngineModelRpmToHp.x is Unmapped (double[9]) in engine_parameters.rs, read m.x[i] once that field is typed as [f64; 9]
fn poly_x(m: &EngineParameters_PolynomialEngineModelRpmToHp, i: i32) -> f64 {
    let _ = (m, i);
    panic!("shortcut: engine polynomial coefficients are not ported yet (EngineParameters x is unmapped)")
}

impl RealisticEngineModel {
    /// C++: `double RealisticEngineModel::rpmToSpeed_mps(double, double, double, double)` [protected]
    /// Given engine rotation in rpm, tracting wheels diameter, differential ratio, and gear ratio, computes vehicle speed in mps
    // double RealisticEngineModel::rpmToSpeed_mps(double rpm, double wheelDiameter_m = 0.94,
    //         double differentialRatio = 4.6, double gearRatio = 4.5) {
    //     return rpm * wheelDiameter_m * M_PI / (differentialRatio * gearRatio * 60);
    // }
    #[allow(unused_variables)]
    pub fn rpm_to_speed_mps(&mut self, rpm: f64, wheelDiameter_m: f64, differentialRatio: f64, gearRatio: f64) -> f64 {
        rpm * wheelDiameter_m * utils::geom::geom_helper::M_PI / (differentialRatio * gearRatio * 60.0)
    }

    /// C++: `double RealisticEngineModel::rpmToSpeed_mps(double)` [protected]
    /// Computes vehicle speed in mps using stored values
    // double RealisticEngineModel::rpmToSpeed_mps(double rpm) {
    //     return ep.__rpmToSpeedCoefficient * rpm / ep.gearRatios[currentGear];
    // }
    #[allow(unused_variables)]
    pub fn rpm_to_speed_mps_2(&mut self, rpm: f64) -> f64 {
        self.ep.__rpmToSpeedCoefficient * rpm / gear_ratio(&self.ep, self.currentGear)
    }

    /// C++: `double RealisticEngineModel::speed_mpsToRpm(double, double, double, double)` [protected]
    /// Given vehicle speed in mps, tracting wheels diameter, differential ratio, and gear ratio, computes engine rotation speed in rpm
    // double RealisticEngineModel::speed_mpsToRpm(double speed_mps, double wheelDiameter_m,
    //         double differentialRatio, double gearRatio) {
    //     return speed_mps * differentialRatio * gearRatio * 60 / (wheelDiameter_m * M_PI);
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_rpm(&mut self, speed_mps: f64, wheelDiameter_m: f64, differentialRatio: f64, gearRatio: f64) -> f64 {
        speed_mps * differentialRatio * gearRatio * 60.0 / (wheelDiameter_m * utils::geom::geom_helper::M_PI)
    }

    /// C++: `double RealisticEngineModel::speed_mpsToRpm(double)` [protected]
    /// Computes engine rotation speed in rpm using stored values
    // double RealisticEngineModel::speed_mpsToRpm(double speed_mps) {
    //     return ep.__speedToRpmCoefficient * speed_mps * ep.gearRatios[currentGear];
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_rpm_2(&mut self, speed_mps: f64) -> f64 {
        self.ep.__speedToRpmCoefficient * speed_mps * gear_ratio(&self.ep, self.currentGear)
    }

    /// C++: `double RealisticEngineModel::speed_mpsToRpm(double, double)` [protected]
    /// Computes engine rotation speed in rpm using stored values and given gear ratio
    // double RealisticEngineModel::speed_mpsToRpm(double speed_mps, double gearRatio) {
    //     return ep.__speedToRpmCoefficient * speed_mps * gearRatio;
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_rpm_3(&mut self, speed_mps: f64, gearRatio: f64) -> f64 {
        self.ep.__speedToRpmCoefficient * speed_mps * gearRatio
    }

    /// C++: `double RealisticEngineModel::rpmToPower_hp(double, const struct EngineParameters::PolynomialEngineModelRpmToHp *)` [protected]
    /// Given an engine mapping computes the available power (hp) given current engine speed in rpm
    // double RealisticEngineModel::rpmToPower_hp(double rpm, const struct EngineParameters::PolynomialEngineModelRpmToHp* engineMapping) {
    //     double sum = engineMapping->x[0];
    //     for (int i = 1; i < engineMapping->degree; i++) {
    //         sum += engineMapping->x[i] + pow(rpm, i);
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn rpm_to_power_hp(&mut self, rpm: f64, engineMapping: Ptr<EngineParameters_PolynomialEngineModelRpmToHp>) -> f64 {
        let mut sum = poly_x(&engineMapping, 0);
        for i in 1..engineMapping.degree {
            sum += poly_x(&engineMapping, i) + rpm.powf(i as f64);
        }
        sum
    }

    /// C++: `double RealisticEngineModel::rpmToPower_hp(double)` [protected]
    /// Computes the available power (hp) using stored values
    // double RealisticEngineModel::rpmToPower_hp(double rpm) {
    //     if (rpm >= ep.maxRpm) {
    //         rpm = ep.maxRpm;
    //     }
    //     double sum = ep.engineMapping.x[0];
    //     for (int i = 1; i < ep.engineMapping.degree; i++) {
    //         sum += ep.engineMapping.x[i] * pow(rpm, i);
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn rpm_to_power_hp_2(&mut self, rpm: f64) -> f64 {
        let mut rpm = rpm;
        if rpm >= self.ep.maxRpm {
            rpm = self.ep.maxRpm;
        }
        let mut sum = poly_x(&self.ep.engineMapping, 0);
        for i in 1..self.ep.engineMapping.degree {
            sum += poly_x(&self.ep.engineMapping, i) * rpm.powf(i as f64);
        }
        sum
    }

    /// C++: `double RealisticEngineModel::speed_mpsToPower_hp(double, const struct EngineParameters::PolynomialEngineModelRpmToHp *, double, double, double)` [protected]
    /// Given vehicle speed in mps, and the rpm to hp mapping, computes the available power at the current speed
    // double RealisticEngineModel::speed_mpsToPower_hp(double speed_mps,
    //         const struct EngineParameters::PolynomialEngineModelRpmToHp* engineMapping,
    //         double wheelDiameter_m, double differentialRatio,
    //         double gearRatio) {
    //     double rpm = speed_mpsToRpm(speed_mps, wheelDiameter_m, differentialRatio, gearRatio);
    //     return rpmToPower_hp(rpm, engineMapping);
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_power_hp(&mut self, speed_mps: f64, engineMapping: Ptr<EngineParameters_PolynomialEngineModelRpmToHp>, wheelDiameter_m: f64, differentialRatio: f64, gearRatio: f64) -> f64 {
        let rpm = self.speed_mps_to_rpm(speed_mps, wheelDiameter_m, differentialRatio, gearRatio);
        self.rpm_to_power_hp(rpm, engineMapping)
    }

    /// C++: `double RealisticEngineModel::speed_mpsToPower_hp(double)` [protected]
    /// Computes the available power at the current speed using stored values
    // double RealisticEngineModel::speed_mpsToPower_hp(double speed_mps) {
    //     return rpmToPower_hp(speed_mpsToRpm(speed_mps));
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_power_hp_2(&mut self, speed_mps: f64) -> f64 {
        let rpm = self.speed_mps_to_rpm_2(speed_mps);
        self.rpm_to_power_hp_2(rpm)
    }

    /// C++: `double RealisticEngineModel::speed_mpsToThrust_N(double, const struct EngineParameters::PolynomialEngineModelRpmToHp *, double, double, double, double)` [protected]
    /// Given vehicle speed in mps, and the rpm to hp mapping, computes the available maximum thrust at current speed
    // double RealisticEngineModel::speed_mpsToThrust_N(double speed_mps,
    //         const struct EngineParameters::PolynomialEngineModelRpmToHp* engineMapping,
    //         double wheelDiameter_m, double differentialRatio,
    //         double gearRatio, double engineEfficiency) {
    //     double power_hp = speed_mpsToPower_hp(speed_mps, engineMapping, wheelDiameter_m, differentialRatio, gearRatio);
    //     return engineEfficiency * power_hp * HP_TO_W / speed_mps;
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_thrust_n(&mut self, speed_mps: f64, engineMapping: Ptr<EngineParameters_PolynomialEngineModelRpmToHp>, wheelDiameter_m: f64, differentialRatio: f64, gearRatio: f64, engineEfficiency: f64) -> f64 {
        let power_hp = self.speed_mps_to_power_hp(speed_mps, engineMapping, wheelDiameter_m, differentialRatio, gearRatio);
        engineEfficiency * power_hp * HP_TO_W / speed_mps
    }

    /// C++: `double RealisticEngineModel::speed_mpsToThrust_N(double)` [protected]
    /// Computes the available maximum thrust at current speed using stored values
    // double RealisticEngineModel::speed_mpsToThrust_N(double speed_mps) {
    //     double power_hp = speed_mpsToPower_hp(speed_mps);
    //     return ep.__speedToThrustCoefficient * power_hp / speed_mps;
    // }
    #[allow(unused_variables)]
    pub fn speed_mps_to_thrust_n_2(&mut self, speed_mps: f64) -> f64 {
        let power_hp = self.speed_mps_to_power_hp_2(speed_mps);
        self.ep.__speedToThrustCoefficient * power_hp / speed_mps
    }

    /// C++: `double RealisticEngineModel::airDrag_N(double, double, double, double)` [protected]
    /// Computes air drag resistance
    // double RealisticEngineModel::airDrag_N(double speed_mps, double cAir, double a_m2, double rho_kgpm3) {
    //     return 0.5 * cAir * a_m2 * rho_kgpm3 * speed_mps * speed_mps;
    // }
    #[allow(unused_variables)]
    pub fn air_drag_n(&mut self, speed_mps: f64, cAir: f64, a_m2: f64, rho_kgpm3: f64) -> f64 {
        0.5 * cAir * a_m2 * rho_kgpm3 * speed_mps * speed_mps
    }

    /// C++: `double RealisticEngineModel::airDrag_N(double)` [protected]
    /// Computes air drag resistance using stored values
    // double RealisticEngineModel::airDrag_N(double speed_mps) {
    //     return ep.__airFrictionCoefficient * speed_mps * speed_mps;
    // }
    #[allow(unused_variables)]
    pub fn air_drag_n_2(&mut self, speed_mps: f64) -> f64 {
        self.ep.__airFrictionCoefficient * speed_mps * speed_mps
    }

    /// C++: `double RealisticEngineModel::rollingResistance_N(double, double, double, double)` [protected]
    /// Computes rolling resistance
    // double RealisticEngineModel::rollingResistance_N(double speed_mps, double mass_kg, double cr1, double cr2) {
    //     return mass_kg * GRAVITY_MPS2 * (cr1 + cr2 * speed_mps * speed_mps);
    // }
    #[allow(unused_variables)]
    pub fn rolling_resistance_n(&mut self, speed_mps: f64, mass_kg: f64, cr1: f64, cr2: f64) -> f64 {
        mass_kg * GRAVITY_MPS2 * (cr1 + cr2 * speed_mps * speed_mps)
    }

    /// C++: `double RealisticEngineModel::rollingResistance_N(double)` [protected]
    /// Computes rolling resistance using stored values
    // double RealisticEngineModel::rollingResistance_N(double speed_mps) {
    //     return ep.__cr1 + ep.__cr2 * speed_mps * speed_mps;
    // }
    #[allow(unused_variables)]
    pub fn rolling_resistance_n_2(&mut self, speed_mps: f64) -> f64 {
        self.ep.__cr1 + self.ep.__cr2 * speed_mps * speed_mps
    }

    /// C++: `double RealisticEngineModel::gravityForce_N(double, double)` [protected]
    /// Computes gravitational force due to slope
    // double RealisticEngineModel::gravityForce_N(double mass_kg, double slope = 0) {
    //     return mass_kg * GRAVITY_MPS2 * sin(slope / 180 * M_PI);
    // }
    #[allow(unused_variables)]
    pub fn gravity_force_n(&mut self, mass_kg: f64, slope: f64) -> f64 {
        mass_kg * GRAVITY_MPS2 * (slope / 180.0 * utils::geom::geom_helper::M_PI).sin()
    }

    /// C++: `double RealisticEngineModel::gravityForce_N()` [protected]
    /// Computes gravitational force using stored values
    // double RealisticEngineModel::gravityForce_N() {
    //     return ep.__gravity;
    // }
    #[allow(unused_variables)]
    pub fn gravity_force_n_2(&mut self) -> f64 {
        self.ep.__gravity
    }

    /// C++: `double RealisticEngineModel::opposingForce_N(double, double, double, double, double, double, double, double)` [protected]
    /// Computes total opposing force, i.e., air drag, rolling resistance, and gravity
    // double RealisticEngineModel::opposingForce_N(double speed_mps, double mass_kg, double slope,
    //         double cAir, double a_m2, double rho_kgpm3,
    //         double cr1, double cr2) {
    //     return airDrag_N(speed_mps, cAir, a_m2, rho_kgpm3) +
    //            rollingResistance_N(speed_mps, mass_kg, cr1, cr2) +
    //            gravityForce_N(mass_kg, slope);
    // }
    #[allow(unused_variables)]
    pub fn opposing_force_n(&mut self, speed_mps: f64, mass_kg: f64, slope: f64, cAir: f64, a_m2: f64, rho_kgpm3: f64, cr1: f64, cr2: f64) -> f64 {
        self.air_drag_n(speed_mps, cAir, a_m2, rho_kgpm3) + self.rolling_resistance_n(speed_mps, mass_kg, cr1, cr2) + self.gravity_force_n(mass_kg, slope)
    }

    /// C++: `double RealisticEngineModel::opposingForce_N(double)` [protected]
    /// Computes total opposing force, using stored values
    // double RealisticEngineModel::opposingForce_N(double speed_mps) {
    //     return airDrag_N(speed_mps) + rollingResistance_N(speed_mps) + gravityForce_N();
    // }
    #[allow(unused_variables)]
    pub fn opposing_force_n_2(&mut self, speed_mps: f64) -> f64 {
        self.air_drag_n_2(speed_mps) + self.rolling_resistance_n_2(speed_mps) + self.gravity_force_n_2()
    }

    /// C++: `double RealisticEngineModel::maxNoSlipAcceleration_mps2(double, double)` [protected]
    /// Computes maximum acceleration without slipping tires
    // double RealisticEngineModel::maxNoSlipAcceleration_mps2(double slope, double frictionCoefficient) {
    //     return frictionCoefficient * GRAVITY_MPS2 * cos(slope / 180 * M_PI);
    // }
    #[allow(unused_variables)]
    pub fn max_no_slip_acceleration_mps2(&mut self, slope: f64, frictionCoefficient: f64) -> f64 {
        frictionCoefficient * GRAVITY_MPS2 * (slope / 180.0 * utils::geom::geom_helper::M_PI).cos()
    }

    /// C++: `double RealisticEngineModel::maxNoSlipAcceleration_mps2()` [protected]
    /// Computes maximum acceleration without slipping tires using stored values
    // double RealisticEngineModel::maxNoSlipAcceleration_mps2() {
    //     return ep.__maxNoSlipAcceleration;
    // }
    #[allow(unused_variables)]
    pub fn max_no_slip_acceleration_mps2_2(&mut self) -> f64 {
        self.ep.__maxNoSlipAcceleration
    }

    /// C++: `double RealisticEngineModel::thrust_NToAcceleration_mps2(double)` [protected]
    /// Computes the acceleration for a certain thrust, using stored values of mass
    // double RealisticEngineModel::thrust_NToAcceleration_mps2(double thrust_N) {
    //     return thrust_N / ep.__maxAccelerationCoefficient;
    // }
    #[allow(unused_variables)]
    pub fn thrust_n_to_acceleration_mps2(&mut self, thrust_N: f64) -> f64 {
        thrust_N / self.ep.__maxAccelerationCoefficient
    }

    /// C++: `int RealisticEngineModel::performGearShifting(double, double)` [protected]
    /// Computes current gear according to gear shifting rules
    // int RealisticEngineModel::performGearShifting(double speed_mps, double acceleration_mps2) {
    //     int newGear = 0;
    //     const double delta = acceleration_mps2 >= 0 ? ep.shiftingRule.deltaRpm : -ep.shiftingRule.deltaRpm;
    //     for (newGear = 0; newGear < ep.nGears - 1; newGear++) {
    //         const double rpm = speed_mpsToRpm(speed_mps, ep.gearRatios[newGear]);
    //         if (rpm >= ep.shiftingRule.rpm + delta) {
    //             continue;
    //         } else {
    //             break;
    //         }
    //     }
    //     currentGear = newGear;
    //     return currentGear;
    // }
    #[allow(unused_variables)]
    pub fn perform_gear_shifting(&mut self, speed_mps: f64, acceleration_mps2: f64) -> i32 {
        let mut newGear: i32 = 0;
        let delta = if acceleration_mps2 >= 0.0 { self.ep.shiftingRule.deltaRpm } else { -self.ep.shiftingRule.deltaRpm };
        while newGear < self.ep.nGears - 1 {
            let rpm = self.speed_mps_to_rpm_3(speed_mps, gear_ratio(&self.ep, newGear));
            if rpm >= self.ep.shiftingRule.rpm + delta {
                newGear += 1;
            } else {
                break;
            }
        }
        self.currentGear = newGear;
        self.currentGear
    }

    /// C++: `double RealisticEngineModel::getEngineTimeConstant_s(double)` [protected]
    /// Computes the engine actuation time constant as function of the rpm
    // double RealisticEngineModel::getEngineTimeConstant_s(double rpm) {
    //     if (rpm <= 0) {
    //         return TAU_MAX;
    //     } else {
    //         if (ep.fixedTauBurn)
    //             //in this case, tau_burn is fixed and is within __engineTauDe_s
    //         {
    //             return std::min(TAU_MAX, ep.__engineTau2 / rpm + ep.__engineTauDe_s);
    //         } else
    //             //in this case, tau_burn is dynamic and is within __engineTau1
    //         {
    //             return std::min(TAU_MAX, ep.__engineTau1 / rpm + ep.tauEx_s);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_engine_time_constant_s(&mut self, rpm: f64) -> f64 {
        if rpm <= 0.0 {
            TAU_MAX
        } else if self.ep.fixedTauBurn {
            //in this case, tau_burn is fixed and is within __engineTauDe_s
            cpp_min(TAU_MAX, self.ep.__engineTau2 / rpm + self.ep.__engineTauDe_s)
        } else {
            //in this case, tau_burn is dynamic and is within __engineTau1
            cpp_min(TAU_MAX, self.ep.__engineTau1 / rpm + self.ep.tauEx_s)
        }
    }

    /// C++: `double RealisticEngineModel::maxEngineAcceleration_mps2(double)` [protected]
    /// Computes maximum vehicle acceleration. This function computes the maximum acceleration at the wheel, considering tires friction but without considering air friction, rolling resistance, and road slope.
    // double RealisticEngineModel::maxEngineAcceleration_mps2(double speed_mps) {
    //     return speed_mpsToThrust_N(speed_mps) / ep.__maxAccelerationCoefficient;
    // }
    #[allow(unused_variables)]
    pub fn max_engine_acceleration_mps2(&mut self, speed_mps: f64) -> f64 {
        let thrust = self.speed_mps_to_thrust_n_2(speed_mps);
        thrust / self.ep.__maxAccelerationCoefficient
    }

    /// C++: `double RealisticEngineModel::getRealBrakingAcceleration(double, double, double, SUMOTime)` [protected]
    /// Computes real vehicle deceleration
    // double RealisticEngineModel::getRealBrakingAcceleration(double speed_mps, double accel_mps2, double reqAccel_mps2, SUMOTime t) {
    //     UNUSED_PARAMETER(t);
    //     //compute which part of the deceleration is currently done by frictions
    //     double frictionDeceleration = thrust_NToAcceleration_mps2(opposingForce_N(speed_mps));
    //     //the maximum deceleration is what the vehicle can do given the tyres plus the friction forces (friction can make the vehicle brake stronger)
    //     double maxDeceleration = -maxNoSlipAcceleration_mps2() - frictionDeceleration;
    //     //compute the new brakes deceleration
    //     double newBrakesAccel_mps2 = ep.__brakesAlpha * reqAccel_mps2 + ep.__brakesOneMinusAlpha * accel_mps2;
    //     //keep actual deceleration within physical limits (considering that wind force can help us slowing down)
    //     return std::max(maxDeceleration, newBrakesAccel_mps2);
    //
    // }
    #[allow(unused_variables)]
    pub fn get_real_braking_acceleration(&mut self, speed_mps: f64, accel_mps2: f64, reqAccel_mps2: f64, t: SUMOTime) -> f64 {
        let _ = t; // UNUSED_PARAMETER(t)
        //compute which part of the deceleration is currently done by frictions
        let opposing = self.opposing_force_n_2(speed_mps);
        let frictionDeceleration = self.thrust_n_to_acceleration_mps2(opposing);
        //the maximum deceleration is what the vehicle can do given the tyres plus the friction forces (friction can make the vehicle brake stronger)
        let maxDeceleration = -self.max_no_slip_acceleration_mps2_2() - frictionDeceleration;
        //compute the new brakes deceleration
        let newBrakesAccel_mps2 = self.ep.__brakesAlpha * reqAccel_mps2 + self.ep.__brakesOneMinusAlpha * accel_mps2;
        //keep actual deceleration within physical limits (considering that wind force can help us slowing down)
        cpp_max(maxDeceleration, newBrakesAccel_mps2)
    }

    /// C++: `void RealisticEngineModel::RealisticEngineModel()`
    // RealisticEngineModel::RealisticEngineModel() {
    //     className = "RealisticEngineModel";
    //     dt_s = 0.01;
    //     xmlFile = "vehicles.xml";
    //     minSpeed_mps = rpmToSpeed_mps(ep.minRpm, ep.wheelDiameter_m, ep.differentialRatio, ep.gearRatios[0]);
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut m = RealisticEngineModel {
            base: GenericEngineModel::new(),
            ep: EngineParameters::new(),
            currentGear: 0, // C++ leaves this uninitialised
            minSpeed_mps: 0.0,
            dt_s: 0.0,
            xmlFile: String::new(),
            vehicleType: String::new(),
        };
        m.base.className = "RealisticEngineModel".to_string();
        m.dt_s = 0.01;
        m.xmlFile = "vehicles.xml".to_string();
        m.minSpeed_mps = m.rpm_to_speed_mps(m.ep.minRpm, m.ep.wheelDiameter_m, m.ep.differentialRatio, gear_ratio(&m.ep, 0));
        m
    }

    /// C++: `void RealisticEngineModel::~RealisticEngineModel() noexcept` [virtual]
    // RealisticEngineModel::~RealisticEngineModel() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty: nothing to release.
    }

    /// C++: `double RealisticEngineModel::getRealAcceleration(double, double, double, SUMOTime)` [virtual]
    /// Computes real vehicle acceleration given current speed, current acceleration, and requested acceleration. If the requested acceleration is negative, then the braking model is invoked
    // double RealisticEngineModel::getRealAcceleration(double speed_mps, double accel_mps2, double reqAccel_mps2, SUMOTime timeStep) {
    //     double realAccel_mps2;
    //     //perform gear shifting, if needed
    //     performGearShifting(speed_mps, accel_mps2);
    //     //since we consider no clutch (clutch always engaged), 0 speed would mean 0 rpm, and thus
    //     //0 available power. thus, the car could never start from a complete stop. so we assume
    //     //a minimum speed of 1 m/s to compute engine power
    //     double correctedSpeed = std::max(speed_mps, minSpeed_mps);
    //     if (reqAccel_mps2 >= 0) {
    //         //the system wants to accelerate
    //         //compute what is the maximum acceleration that can be delivered by the engine
    //         double engineAccel = maxEngineAcceleration_mps2(correctedSpeed);
    //         //the maximum acceleration is given by the maximum acceleration the engine can provide minus friction forces
    //         double maxAccel = engineAccel - thrust_NToAcceleration_mps2(opposingForce_N(speed_mps));
    //         //now we need to computed delayed acceleration due to actuation lag
    //         double tau = getEngineTimeConstant_s(speed_mpsToRpm(correctedSpeed));
    //         double alpha = ep.dt / (tau + ep.dt);
    //         //use standard first order lag with time constant depending on engine rpm
    //         realAccel_mps2 = alpha * reqAccel_mps2 + (1 - alpha) * accel_mps2;
    //         //limit the actual acceleration by considering friction forces and the maximum acceleration considering tyres
    //         realAccel_mps2 = std::min(std::min(realAccel_mps2, maxAccel), maxNoSlipAcceleration_mps2());
    //     } else {
    //         realAccel_mps2 = getRealBrakingAcceleration(speed_mps, accel_mps2, reqAccel_mps2, timeStep);
    //     }
    //     return realAccel_mps2;
    // }
    #[allow(unused_variables)]
    pub fn get_real_acceleration(&mut self, speed_mps: f64, accel_mps2: f64, reqAccel_mps2: f64, timeStep: SUMOTime) -> f64 {
        //perform gear shifting, if needed
        self.perform_gear_shifting(speed_mps, accel_mps2);
        //since we consider no clutch (clutch always engaged), 0 speed would mean 0 rpm, and thus
        //0 available power. thus, the car could never start from a complete stop. so we assume
        //a minimum speed of 1 m/s to compute engine power
        let correctedSpeed = cpp_max(speed_mps, self.minSpeed_mps);
        let realAccel_mps2: f64 = if reqAccel_mps2 >= 0.0 {
            //the system wants to accelerate
            //compute what is the maximum acceleration that can be delivered by the engine
            let engineAccel = self.max_engine_acceleration_mps2(correctedSpeed);
            //the maximum acceleration is given by the maximum acceleration the engine can provide minus friction forces
            let opposing = self.opposing_force_n_2(speed_mps);
            let maxAccel = engineAccel - self.thrust_n_to_acceleration_mps2(opposing);
            //now we need to computed delayed acceleration due to actuation lag
            let rpm = self.speed_mps_to_rpm_2(correctedSpeed);
            let tau = self.get_engine_time_constant_s(rpm);
            let alpha = self.ep.dt / (tau + self.ep.dt);
            //use standard first order lag with time constant depending on engine rpm
            let mut accel = alpha * reqAccel_mps2 + (1.0 - alpha) * accel_mps2;
            //limit the actual acceleration by considering friction forces and the maximum acceleration considering tyres
            accel = cpp_min(cpp_min(accel, maxAccel), self.max_no_slip_acceleration_mps2_2());
            accel
        } else {
            self.get_real_braking_acceleration(speed_mps, accel_mps2, reqAccel_mps2, timeStep)
        };
        realAccel_mps2
    }

    /// C++: `void RealisticEngineModel::loadParameters()`
    /// Load parameters from xml file
    // void RealisticEngineModel::loadParameters() {
    //     VehicleEngineHandler engineHandler(vehicleType);
    //     if (!XMLSubSys::runParser(engineHandler, xmlFile)) {
    //         throw ProcessError();
    //     }
    //     //copy loaded parameters into our engine parameters
    //     ep = engineHandler.getEngineParameters();
    //     ep.dt = dt_s;
    //     ep.computeCoefficients();
    //     //compute "minimum speed" to be used when computing maximum acceleration at speeds close to 0
    //     minSpeed_mps = rpmToSpeed_mps(ep.minRpm, ep.wheelDiameter_m, ep.differentialRatio, ep.gearRatios[0]);
    // }
    #[allow(unused_variables)]
    pub fn load_parameters(&mut self) -> Result<(), ProcessError> {
        let mut engineHandler = VehicleEngineHandler::new(&self.vehicleType);
        if !run_parser(&mut engineHandler, &self.xmlFile, false, false, false, true)? {
            return Err(ProcessError::default_msg());
        }
        //copy loaded parameters into our engine parameters
        self.ep.assign_op(engineHandler.get_engine_parameters());
        self.ep.dt = self.dt_s;
        self.ep.compute_coefficients();
        //compute "minimum speed" to be used when computing maximum acceleration at speeds close to 0
        self.minSpeed_mps = self.rpm_to_speed_mps(self.ep.minRpm, self.ep.wheelDiameter_m, self.ep.differentialRatio, gear_ratio(&self.ep, 0));
        Ok(())
    }

    /// C++: `void RealisticEngineModel::setParameter(const std::string, const std::string &)` [virtual]
    /// Sets a single parameter value
    // void RealisticEngineModel::setParameter(const std::string parameter, const std::string& value) {
    //     if (parameter == ENGINE_PAR_XMLFILE) {
    //         xmlFile = value;
    //     }
    //     if (parameter == ENGINE_PAR_VEHICLE) {
    //         vehicleType = value;
    //         if (xmlFile != "") {
    //             loadParameters();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, parameter: String, value: &str) -> Result<(), ProcessError> {
        if parameter == ENGINE_PAR_XMLFILE {
            self.xmlFile = value.to_string();
        }
        if parameter == ENGINE_PAR_VEHICLE {
            self.vehicleType = value.to_string();
            if self.xmlFile != "" {
                self.load_parameters()?;
            }
        }
        Ok(())
    }

    /// C++: `void RealisticEngineModel::setParameter(const std::string, double)` [virtual]
    // void RealisticEngineModel::setParameter(const std::string parameter, double value) {
    //     if (parameter == ENGINE_PAR_DT) {
    //         dt_s = value;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter_2(&mut self, parameter: String, value: f64) {
        if parameter == ENGINE_PAR_DT {
            self.dt_s = value;
        }
    }

    /// C++: `void RealisticEngineModel::setParameter(const std::string, int)` [virtual]
    // void RealisticEngineModel::setParameter(const std::string parameter, int value) {
    //     UNUSED_PARAMETER(parameter);
    //     UNUSED_PARAMETER(value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter_3(&mut self, parameter: String, value: i32) {
        // UNUSED_PARAMETER(parameter); UNUSED_PARAMETER(value); (C++ body is empty)
    }

    /// C++: `void RealisticEngineModel::getEngineData(double, int &, double &)`
    /// Returns current engine RPM and gear
    // void RealisticEngineModel::getEngineData(double speed_mps, int& gear, double& rpm) {
    //     gear = currentGear;
    //     rpm = speed_mpsToRpm(speed_mps);
    // }
    #[allow(unused_variables)]
    pub fn get_engine_data(&mut self, speed_mps: f64, gear: &mut i32, rpm: &mut f64) {
        *gear = self.currentGear;
        *rpm = self.speed_mps_to_rpm_2(speed_mps);
    }

}

