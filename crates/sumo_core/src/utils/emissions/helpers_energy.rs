//! Port of utils/emissions/HelpersEnergy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::f64::consts::PI;
use utils::common::msg_handler::write_error;
use utils::common::time::{accel2speed, speed2dist, ts};
use utils::geom::geom_helper::{deg2rad, GRAVITY};

/// Helper methods for energy-based electricity consumption computation based on the battery device
/// C++ class `HelpersEnergy` (utils/emissions/HelpersEnergy.h)
#[repr(C)]
pub struct HelpersEnergy {
    /// C++ base class `PollutantsInterface::Helper`
    pub base: PollutantsInterface_Helper,
}

impl std::ops::Deref for HelpersEnergy {
    type Target = PollutantsInterface_Helper;
    fn deref(&self) -> &PollutantsInterface_Helper {
        &self.base
    }
}
impl std::ops::DerefMut for HelpersEnergy {
    fn deref_mut(&mut self) -> &mut PollutantsInterface_Helper {
        &mut self.base
    }
}

// C++ static members of HelpersEnergy (port as module-level statics, see AGENTS.md):
//   static const int HelpersEnergy::ENERGY_BASE;
//   static const double HelpersEnergy::myDefaultMass;
//   static const double HelpersEnergy::myDefaultFrontSurfaceArea;
//   static const double HelpersEnergy::myDefaultAirDragCoefficient;
//   static const double HelpersEnergy::myDefaultRotatingMass;
//   static const double HelpersEnergy::myDefaultRadialDragCoefficient;
//   static const double HelpersEnergy::myDefaultRollDragCoefficient;
//   static const double HelpersEnergy::myDefaultConstantPowerIntake;
//   static const double HelpersEnergy::myDefaultPropulsionEfficiency;
//   static const double HelpersEnergy::myDefaultRecuperationEfficiency;
//   static const double HelpersEnergy::myDefaultRecuperationEfficiencyByDeceleration;
const ENERGY_BASE: i32 = 4 << 16;
const MY_DEFAULT_MASS: f64 = 1830.;
const MY_DEFAULT_FRONT_SURFACE_AREA: f64 = 2.6;
const MY_DEFAULT_AIR_DRAG_COEFFICIENT: f64 = 0.35;
const MY_DEFAULT_ROTATING_MASS: f64 = 40.;
const MY_DEFAULT_RADIAL_DRAG_COEFFICIENT: f64 = 0.1;
const MY_DEFAULT_ROLL_DRAG_COEFFICIENT: f64 = 0.01;
const MY_DEFAULT_CONSTANT_POWER_INTAKE: f64 = 100.;
const MY_DEFAULT_PROPULSION_EFFICIENCY: f64 = 0.98;
const MY_DEFAULT_RECUPERATION_EFFICIENCY: f64 = 0.96;
const MY_DEFAULT_RECUPERATION_EFFICIENCY_BY_DECELERATION: f64 = 0.0;

// C++ PolySolver (utils/common/PolySolver.cpp) is used only by HelpersEnergy::acceleration(), so its port lives here
/// C++ `PolySolver::quadraticSolve`: returns only real roots
fn quadratic_solve(a: f64, b: f64, c: f64) -> (i32, f64, f64) {
    // ax^2 + bx + c = 0
    if a == 0. {
        if b == 0. {
            if c == 0. {
                return (2, f64::INFINITY, -f64::INFINITY);
            } else {
                return (0, f64::NAN, f64::NAN);
            }
        } else {
            return (1, -c / b, f64::NAN);
        }
    }

    if c == 0. {
        return (2, 0., -b / a);
    }

    let disc = b * b - 4. * a * c;

    if disc > 0. {
        // two different real roots
        let x1_real = (-b + disc.sqrt()) / (2. * a);
        let x2_real = (-b - disc.sqrt()) / (2. * a);
        (2, x1_real, x2_real)
    } else if disc == 0. {
        // a real root with multiplicity 2
        let x1_real = (-b + disc.sqrt()) / (2. * a);
        (1, x1_real, f64::NAN)
    } else {
        // all roots are complex
        (0, f64::NAN, f64::NAN)
    }
}

/// C++ `PolySolver::cubicSolve`: returns only real roots
fn cubic_solve(a: f64, b: f64, c: f64, d: f64) -> (i32, f64, f64, f64) {
    // ax^3 + bx^2 + cx + d = 0
    if a == 0. {
        let (numX, x1, x2) = quadratic_solve(b, c, d);
        return (numX, x1, x2, f64::NAN);
    }
    if d == 0. {
        let (numX, x1, x2) = quadratic_solve(a, b, c);
        return (numX + 1, 0., x1, x2);
    }

    let b = b / a;
    let c = c / a;
    let d = d / a;

    let mut q = (3.0 * c - (b * b)) / 9.0;
    let mut r = -(27.0 * d) + b * (9.0 * c - 2.0 * (b * b));
    r /= 54.0;
    let disc = q * q * q + r * r;
    let term1 = b / 3.0;

    if disc > 0. {
        // One root real, two are complex
        let mut s = r + disc.sqrt();
        s = if s < 0. { -(-s).cbrt() } else { s.cbrt() };
        let mut t = r - disc.sqrt();
        t = if t < 0. { -(-t).cbrt() } else { t.cbrt() };
        let x1_real = -term1 + s + t;
        return (1, x1_real, f64::NAN, f64::NAN);
    }
    // The remaining options are all real
    else if disc == 0. {
        // All roots real, at least two are equal.
        let r13 = if r < 0. { -(-r).cbrt() } else { r.cbrt() };
        let x1_real = -term1 + 2.0 * r13;
        let x2_real = -(r13 + term1);
        return (2, x1_real, x2_real, f64::NAN);
    }
    // Only option left is that all roots are real and unequal (to get here, q < 0)
    else {
        q = -q;
        let mut dum1 = q * q * q;
        dum1 = (r / dum1.sqrt()).acos();
        let r13 = 2.0 * q.sqrt();
        let x1_real = -term1 + r13 * (dum1 / 3.0).cos();
        let x2_real = -term1 + r13 * ((dum1 + 2.0 * PI) / 3.0).cos();
        let x3_real = -term1 + r13 * ((dum1 + 4.0 * PI) / 3.0).cos();
        return (3, x1_real, x2_real, x3_real);
    }
}

impl HelpersEnergy {
    /// C++: `void HelpersEnergy::HelpersEnergy()`
    /// Constructor (initializes myEmissionClassStrings)
    // HelpersEnergy::HelpersEnergy():
    //     PollutantsInterface::Helper("Energy", ENERGY_BASE, ENERGY_BASE)
    // { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            base: PollutantsInterface_Helper::new("Energy".to_string(), ENERGY_BASE, ENERGY_BASE),
        }
    }

    /// C++: `std::string HelpersEnergy::getFuel(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the fuel type described by this emission class as described in the Amitran interface (Gasoline, Diesel, ...)
    // std::string getFuel(const SUMOEmissionClass /* c */) const {
    //         return "Electricity";
    //     }
    #[allow(unused_variables)]
    pub fn get_fuel(&self, arg0: SUMOEmissionClass) -> String {
        "Electricity".to_string()
    }

    /// C++: `double HelpersEnergy::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class This implementation returns the default mass for this model.
    // double getWeight(const SUMOEmissionClass /* c */) const {
    //         return myDefaultMass;
    //     }
    #[allow(unused_variables)]
    pub fn get_weight(&self, arg0: SUMOEmissionClass) -> f64 {
        MY_DEFAULT_MASS
    }

    /// C++: `double HelpersEnergy::compute(const SUMOEmissionClass, const PollutantsInterface::EmissionType, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Computes the emitted pollutant amount using the given speed and acceleration
    // double
    // HelpersEnergy::compute(const SUMOEmissionClass /* c */, const PollutantsInterface::EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     if (e != PollutantsInterface::ELEC) {
    //         return 0.;
    //     }
    //     if (param == nullptr) {
    //         param = EnergyParams::getDefault();
    //     }
    //     if (param->isOff()) {
    //         return 0.;
    //     }
    //     //@ToDo: All formulas below work with the logic of the euler update (refs #860).
    //     //       Approximation order could be improved. Refs. #2592.
    //
    //     const double lastV = v - ACCEL2SPEED(a);
    //     const double mass = param->getTotalMass(myDefaultMass, 0.);
    //
    //     // calculate power needed for potential energy difference
    //     double power = mass * GRAVITY * sin(DEG2RAD(slope)) * v;
    //
    //     // power needed for kinetic energy difference of vehicle
    //     power += 0.5 * mass * (v * v - lastV * lastV) / TS;
    //
    //     // add power needed for rotational energy diff of internal rotating elements
    //     power += 0.5 * param->getDoubleOptional(SUMO_ATTR_ROTATINGMASS, myDefaultRotatingMass) * (v * v - lastV * lastV) / TS;
    //
    //     // power needed for Energy loss through Air resistance [Ws]
    //     // Calculate energy losses:
    //     // EnergyLoss,Air = 1/2 * rho_air [kg/m^3] * myFrontSurfaceArea [m^2] * myAirDragCoefficient [-] * v_Veh^2 [m/s] * s [m]
    //     //                    ... with rho_air [kg/m^3] = 1,2041 kg/m^3 (at T = 20C)
    //     //                    ... with s [m] = v_Veh [m/s] * TS [s]
    //     // divided by TS to get power instead of energy
    //     power += 0.5 * 1.2041 * param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA, myDefaultFrontSurfaceArea) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT, myDefaultAirDragCoefficient) * v * v * v;
    //
    //     // power needed for Energy loss through Roll resistance [Ws]
    //     //                    ... (fabs(veh.getSpeed())>=0.01) = 0, if vehicle isn't moving
    //     // EnergyLoss,Tire = c_R [-] * F_N [N] * s [m]
    //     //                    ... with c_R = ~0.012    (car tire on asphalt)
    //     //                    ... with F_N [N] = myMass [kg] * g [m/s^2]
    //     // divided by TS to get power instead of energy
    //     power += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, myDefaultRollDragCoefficient) * GRAVITY * mass * v;
    //
    //     // Energy loss through friction by radial force [Ws]
    //     // If angle of vehicle was changed
    //     const double angleDiff = param->getAngleDiff();
    //     if (angleDiff != 0.) {
    //         // Compute new radio
    //         double radius = SPEED2DIST(v) / fabs(angleDiff);
    //
    //         // Check if radius is in the interval [0.0001 - 10000] (To avoid overflow and division by zero)
    //         if (radius < 0.0001) {
    //             radius = 0.0001;
    //         } else if (radius > 10000) {
    //             radius = 10000;
    //         }
    //         // EnergyLoss,internalFrictionRadialForce = c [m] * F_rad [N];
    //         // Energy loss through friction by radial force [Ws]
    //         // divided by TS to get power instead of energy
    //         power += param->getDoubleOptional(SUMO_ATTR_RADIALDRAGCOEFFICIENT, myDefaultRadialDragCoefficient) * mass * v * v / radius / TS;
    //     }
    //
    //     // E_Bat = E_kin_pot + EnergyLoss;
    //     if (power > 0) {
    //         // Assumption: Efficiency of myPropulsionEfficiency when accelerating
    //         power /= param->getDoubleOptional(SUMO_ATTR_PROPULSIONEFFICIENCY, myDefaultPropulsionEfficiency);
    //     } else {
    //         // Assumption: Efficiency of myRecuperationEfficiency when recuperating
    //         power *= param->getDoubleOptional(SUMO_ATTR_RECUPERATIONEFFICIENCY, myDefaultRecuperationEfficiency);
    //         if (a != 0) {
    //             // Fiori, Chiara & Ahn, Kyoungho & Rakha, Hesham. (2016).
    //             // Power-based electric vehicle energy consumption model: Model
    //             // development and validation. Applied Energy. 168. 257-268.
    //             // 10.1016/j.apenergy.2016.01.097.
    //             //
    //             // Insaf Sagaama, Amine Kchiche, Wassim Trojet, Farouk Kamoun
    //             // Improving The Accuracy of The Energy Consumption Model for
    //             // Electric Vehicle in SUMO Considering The Ambient Temperature
    //             // Effects
    //             power *= (1 / exp(param->getDoubleOptional(SUMO_ATTR_RECUPERATIONEFFICIENCY_BY_DECELERATION, myDefaultRecuperationEfficiencyByDeceleration) / fabs(a)));
    //         }
    //     }
    //
    //     // EnergyLoss,constantConsumers
    //     // Energy loss through constant loads (e.g. A/C) [W]
    //     power += param->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE, myDefaultConstantPowerIntake);
    //
    //     // convert from [W], to [Wh/s] (3600s / 1h):
    //     return power / 3600.;
    // }
    #[allow(unused_variables)]
    pub fn compute(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        if e != PollutantsInterface_EmissionType::ELEC {
            return 0.;
        }
        let mut param = param;
        if param.is_null() {
            param = EnergyParams::get_default();
        }
        if param.is_off() {
            return 0.;
        }
        //@ToDo: All formulas below work with the logic of the euler update (refs #860).
        //       Approximation order could be improved. Refs. #2592.

        let lastV = v - accel2speed(a);
        let mass = param.get_total_mass(MY_DEFAULT_MASS, 0.);

        // calculate power needed for potential energy difference
        let mut power = mass * GRAVITY * deg2rad(slope).sin() * v;

        // power needed for kinetic energy difference of vehicle
        power += 0.5 * mass * (v * v - lastV * lastV) / ts();

        // add power needed for rotational energy diff of internal rotating elements
        power += 0.5 * param.get_double_optional(SUMO_ATTR_ROTATINGMASS, MY_DEFAULT_ROTATING_MASS) * (v * v - lastV * lastV) / ts();

        // power needed for Energy loss through Air resistance [Ws]
        // Calculate energy losses:
        // EnergyLoss,Air = 1/2 * rho_air [kg/m^3] * myFrontSurfaceArea [m^2] * myAirDragCoefficient [-] * v_Veh^2 [m/s] * s [m]
        //                    ... with rho_air [kg/m^3] = 1,2041 kg/m^3 (at T = 20C)
        //                    ... with s [m] = v_Veh [m/s] * TS [s]
        // divided by TS to get power instead of energy
        power += 0.5 * 1.2041 * param.get_double_optional(SUMO_ATTR_FRONTSURFACEAREA, MY_DEFAULT_FRONT_SURFACE_AREA) * param.get_double_optional(SUMO_ATTR_AIRDRAGCOEFFICIENT, MY_DEFAULT_AIR_DRAG_COEFFICIENT) * v * v * v;

        // power needed for Energy loss through Roll resistance [Ws]
        //                    ... (fabs(veh.getSpeed())>=0.01) = 0, if vehicle isn't moving
        // EnergyLoss,Tire = c_R [-] * F_N [N] * s [m]
        //                    ... with c_R = ~0.012    (car tire on asphalt)
        //                    ... with F_N [N] = myMass [kg] * g [m/s^2]
        // divided by TS to get power instead of energy
        power += param.get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, MY_DEFAULT_ROLL_DRAG_COEFFICIENT) * GRAVITY * mass * v;

        // Energy loss through friction by radial force [Ws]
        // If angle of vehicle was changed
        let angleDiff = param.get_angle_diff();
        if angleDiff != 0. {
            // Compute new radio
            let mut radius = speed2dist(v) / angleDiff.abs();

            // Check if radius is in the interval [0.0001 - 10000] (To avoid overflow and division by zero)
            if radius < 0.0001 {
                radius = 0.0001;
            } else if radius > 10000. {
                radius = 10000.;
            }
            // EnergyLoss,internalFrictionRadialForce = c [m] * F_rad [N];
            // Energy loss through friction by radial force [Ws]
            // divided by TS to get power instead of energy
            power += param.get_double_optional(SUMO_ATTR_RADIALDRAGCOEFFICIENT, MY_DEFAULT_RADIAL_DRAG_COEFFICIENT) * mass * v * v / radius / ts();
        }

        // E_Bat = E_kin_pot + EnergyLoss;
        if power > 0. {
            // Assumption: Efficiency of myPropulsionEfficiency when accelerating
            power /= param.get_double_optional(SUMO_ATTR_PROPULSIONEFFICIENCY, MY_DEFAULT_PROPULSION_EFFICIENCY);
        } else {
            // Assumption: Efficiency of myRecuperationEfficiency when recuperating
            power *= param.get_double_optional(SUMO_ATTR_RECUPERATIONEFFICIENCY, MY_DEFAULT_RECUPERATION_EFFICIENCY);
            if a != 0. {
                // Fiori, Chiara & Ahn, Kyoungho & Rakha, Hesham. (2016).
                // Power-based electric vehicle energy consumption model: Model
                // development and validation. Applied Energy. 168. 257-268.
                // 10.1016/j.apenergy.2016.01.097.
                //
                // Insaf Sagaama, Amine Kchiche, Wassim Trojet, Farouk Kamoun
                // Improving The Accuracy of The Energy Consumption Model for
                // Electric Vehicle in SUMO Considering The Ambient Temperature
                // Effects
                power *= 1. / (param.get_double_optional(SUMO_ATTR_RECUPERATIONEFFICIENCY_BY_DECELERATION, MY_DEFAULT_RECUPERATION_EFFICIENCY_BY_DECELERATION) / a.abs()).exp();
            }
        }

        // EnergyLoss,constantConsumers
        // Energy loss through constant loads (e.g. A/C) [W]
        power += param.get_double_optional(SUMO_ATTR_CONSTANTPOWERINTAKE, MY_DEFAULT_CONSTANT_POWER_INTAKE);

        // convert from [W], to [Wh/s] (3600s / 1h):
        power / 3600.
    }

    /// C++: `double HelpersEnergy::acceleration(const SUMOEmissionClass, const PollutantsInterface::EmissionType, const double, const double, const double, const EnergyParams *) const` [const]
    /// Computes the achievable acceleration using the given speed and amount of consumed electric power
    // double
    // HelpersEnergy::acceleration(const SUMOEmissionClass /* c */, const PollutantsInterface::EmissionType e, const double v, const double P, const double slope, const EnergyParams* param) const {
    //     if (e != PollutantsInterface::ELEC) {
    //         return 0.;
    //     }
    //
    //     if (param == nullptr) {
    //         param = EnergyParams::getDefault();
    //     }
    //
    //     // Inverse formula for the function compute()
    //
    //     // Computes the achievable acceleration using the given speed and drive power in [Wh/s]
    //     // It does not consider friction losses by radial force and the acceleration-dependent
    //     // recuperation efficiency (eta_recuperation is considered constant)
    //     //
    //     // The power `Prest` used for acceleration is given by a cubic polynomial in acceleration,
    //     // i.e.the equation
    //     //
    //     //   Prest = const1*a + const2*a^2 + const3*a^3
    //     //
    //     // and we need to find `a` given `Prest`.
    //     //
    //     // The solutions of `a(P)` is stored in variables `x1`, `x2`, and `x3` returned by
    //     // the method `PolySolver::cubicSolve()`, see below.
    //     //
    //     // Power used for accelerating, `Prest`, is the total used power minus power wasted by running resistances.
    //
    //     const double mass = param->getTotalMass(myDefaultMass, 0.);
    //     const double rotMass = param->getDoubleOptional(SUMO_ATTR_ROTATINGMASS, myDefaultRotatingMass);
    //     double const1, const2, const3;
    //     double Prest;
    //     int numX;
    //     double x1, x2, x3;
    //
    //     // Power delivered by the drive
    //     if (P > 0) {
    //         // Assumption: Efficiency of myPropulsionEfficiency when accelerating
    //         Prest = P * 3600 * param->getDoubleOptional(SUMO_ATTR_PROPULSIONEFFICIENCY, myDefaultPropulsionEfficiency);
    //     } else {
    //         // Assumption: Efficiency of myRecuperationEfficiency when recuperating
    //         Prest = P * 3600 / param->getDoubleOptional(SUMO_ATTR_RECUPERATIONEFFICIENCY, myDefaultRecuperationEfficiency);
    //     }
    //
    //     // calculate power drop due to a potential energy difference
    //     // in inverse original formula:      power = mass * GRAVITY * sin(DEG2RAD(slope)) * v;
    //     // i.e. in terms of 'lastV' and 'a': power = mass * GRAVITY * sin(DEG2RAD(slope)) * (lastV +  TS * a);
    //     Prest -= mass * GRAVITY * sin(DEG2RAD(slope)) * (v);
    //     const1 = mass * GRAVITY * sin(DEG2RAD(slope)) * (TS);
    //
    //     // update coefficients of a(P) equation considering power loss through Roll resistance
    //     // in inverse original formula:      power += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT) * GRAVITY * mass * v;
    //     // i.e. in terms of 'lastV' and 'a': power += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT) * GRAVITY * mass * (lastV +  TS * a);
    //     Prest -= param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, myDefaultRollDragCoefficient) * GRAVITY * mass * v;
    //     const1 += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, myDefaultRollDragCoefficient) * GRAVITY * mass * (TS);
    //
    //     //Constant loads are omitted. We assume P as the max limit for the main traction drive. Constant loads are often covered by an auxiliary drive
    //     //Power loss through constant loads (e.g. A/C) [W]
    //     //Prest -= param->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE) / TS;
    //
    //     // update coefficients of a(P) equation considering kinetic energy difference of vehicle
    //     // in inverse original formula:      power += 0.5 * mass * (v * v - lastV * lastV) / TS;
    //     // i.e. in terms of 'lastV' and 'a': power += 0.5 * mass * (2 * lastV * a + TS * a * a);
    //     const1 += 0.5 * mass * (2 * v);
    //     const2 = 0.5 * mass * (TS);
    //
    //     // update coefficients of a(P) equation considering rotational energy diff of internal rotating elements
    //     // in inverse original formula:      power += 0.5 * rotMass * (v * v - lastV * lastV) / TS;
    //     // i.e. in terms of 'lastV' and 'a': power += 0.5 * rotMass * (2 * lastV * a + TS * a * a);
    //     const1 += 0.5 * rotMass * (2 * v);
    //     const2 += 0.5 * rotMass * (TS);
    //
    //     // update coefficients of a(P) equation considering energy loss through Air resistance
    //     // in inverse original formula:      power += 0.5 * 1.2041 * param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT) * v * v * v;
    //     // i.e. in terms of 'lastV' and 'a': power += 0.5 * rotMass * (lastV^3 + 3* lastV^2 * TS * a +  3* lastV * TS^2 *a^2 + TS^3 * a^3);
    //     const double drag = 0.5 * 1.2041 * param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA, myDefaultFrontSurfaceArea) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT, myDefaultAirDragCoefficient);
    //     Prest -= drag * (v * v * v);
    //     const1 += drag * (3 * v * v * TS);
    //     const2 += drag * (3 * v * TS * TS);
    //     const3 = drag * (TS * TS * TS);
    //
    //     // Prest = const1*a + const2*a^2 + const3*a^3
    //     // Solve cubic equation in `a` for real roots, and return the number of roots in `numX`
    //     // and the roots in `x1`, `x2`, and `x3`.
    //     std::tie(numX, x1, x2, x3) = PolySolver::cubicSolve(const3, const2, const1, -Prest);
    //
    //
    //     switch (numX) {
    //         case 1:
    //             return x1;
    //         case 2:
    //             return MAX2(x1, x2);
    //         case 3:
    //             return MAX3(x1, x2, x3);
    //         default:
    //             WRITE_ERROR(TL("An acceleration given by the power was not found."));
    //             return 0;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn acceleration(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, P: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        if e != PollutantsInterface_EmissionType::ELEC {
            return 0.;
        }

        let mut param = param;
        if param.is_null() {
            param = EnergyParams::get_default();
        }

        let mass = param.get_total_mass(MY_DEFAULT_MASS, 0.);
        let rotMass = param.get_double_optional(SUMO_ATTR_ROTATINGMASS, MY_DEFAULT_ROTATING_MASS);
        let mut const1: f64;
        let mut const2: f64;
        let const3: f64;
        let mut Prest: f64;

        // Power delivered by the drive
        if P > 0. {
            // Assumption: Efficiency of myPropulsionEfficiency when accelerating
            Prest = P * 3600. * param.get_double_optional(SUMO_ATTR_PROPULSIONEFFICIENCY, MY_DEFAULT_PROPULSION_EFFICIENCY);
        } else {
            // Assumption: Efficiency of myRecuperationEfficiency when recuperating
            Prest = P * 3600. / param.get_double_optional(SUMO_ATTR_RECUPERATIONEFFICIENCY, MY_DEFAULT_RECUPERATION_EFFICIENCY);
        }

        // calculate power drop due to a potential energy difference
        // in inverse original formula:      power = mass * GRAVITY * sin(DEG2RAD(slope)) * v;
        // i.e. in terms of 'lastV' and 'a': power = mass * GRAVITY * sin(DEG2RAD(slope)) * (lastV +  TS * a);
        Prest -= mass * GRAVITY * deg2rad(slope).sin() * (v);
        const1 = mass * GRAVITY * deg2rad(slope).sin() * (ts());

        // update coefficients of a(P) equation considering power loss through Roll resistance
        // in inverse original formula:      power += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT) * GRAVITY * mass * v;
        // i.e. in terms of 'lastV' and 'a': power += param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT) * GRAVITY * mass * (lastV +  TS * a);
        Prest -= param.get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, MY_DEFAULT_ROLL_DRAG_COEFFICIENT) * GRAVITY * mass * v;
        const1 += param.get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, MY_DEFAULT_ROLL_DRAG_COEFFICIENT) * GRAVITY * mass * (ts());

        //Constant loads are omitted. We assume P as the max limit for the main traction drive. Constant loads are often covered by an auxiliary drive
        //Power loss through constant loads (e.g. A/C) [W]
        //Prest -= param->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE) / TS;

        // update coefficients of a(P) equation considering kinetic energy difference of vehicle
        // in inverse original formula:      power += 0.5 * mass * (v * v - lastV * lastV) / TS;
        // i.e. in terms of 'lastV' and 'a': power += 0.5 * mass * (2 * lastV * a + TS * a * a);
        const1 += 0.5 * mass * (2. * v);
        const2 = 0.5 * mass * (ts());

        // update coefficients of a(P) equation considering rotational energy diff of internal rotating elements
        // in inverse original formula:      power += 0.5 * rotMass * (v * v - lastV * lastV) / TS;
        // i.e. in terms of 'lastV' and 'a': power += 0.5 * rotMass * (2 * lastV * a + TS * a * a);
        const1 += 0.5 * rotMass * (2. * v);
        const2 += 0.5 * rotMass * (ts());

        // update coefficients of a(P) equation considering energy loss through Air resistance
        // in inverse original formula:      power += 0.5 * 1.2041 * param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT) * v * v * v;
        // i.e. in terms of 'lastV' and 'a': power += 0.5 * rotMass * (lastV^3 + 3* lastV^2 * TS * a +  3* lastV * TS^2 *a^2 + TS^3 * a^3);
        let drag = 0.5 * 1.2041 * param.get_double_optional(SUMO_ATTR_FRONTSURFACEAREA, MY_DEFAULT_FRONT_SURFACE_AREA) * param.get_double_optional(SUMO_ATTR_AIRDRAGCOEFFICIENT, MY_DEFAULT_AIR_DRAG_COEFFICIENT);
        Prest -= drag * (v * v * v);
        const1 += drag * (3. * v * v * ts());
        const2 += drag * (3. * v * ts() * ts());
        const3 = drag * (ts() * ts() * ts());

        // Prest = const1*a + const2*a^2 + const3*a^3
        // Solve cubic equation in `a` for real roots, and return the number of roots in `numX`
        // and the roots in `x1`, `x2`, and `x3`.
        let (numX, x1, x2, x3) = cubic_solve(const3, const2, const1, -Prest);

        match numX {
            1 => x1,
            2 => {
                if x1 > x2 { x1 } else { x2 }
            }
            3 => {
                // MAX3(a, b, c) = MAX2(c, a > b ? a : b)
                let m = if x1 > x2 { x1 } else { x2 };
                if x3 > m { x3 } else { m }
            }
            _ => {
                write_error("An acceleration given by the power was not found.");
                0.
            }
        }
    }

}

