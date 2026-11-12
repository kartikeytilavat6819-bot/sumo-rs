//! Port of microsim/engine/FirstOrderLagModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

// C++ #defines from microsim/cfmodels/CC_Const.h
const FOLM_PAR_TAU: &str = "tau_s";
const FOLM_PAR_DT: &str = "dt_s";

/// This model models actuation lags using a first order lag, i.e., a first order low pass filter with a time constant tau
/// C++ class `FirstOrderLagModel` (microsim/engine/FirstOrderLagModel.h)
#[repr(C)]
pub struct FirstOrderLagModel {
    /// C++ base class `GenericEngineModel`
    pub base: GenericEngineModel,
    pub tau_s: f64,
    pub dt_s: f64,
    pub alpha: f64,
    pub oneMinusAlpha: f64,
}

impl std::ops::Deref for FirstOrderLagModel {
    type Target = GenericEngineModel;
    fn deref(&self) -> &GenericEngineModel {
        &self.base
    }
}
impl std::ops::DerefMut for FirstOrderLagModel {
    fn deref_mut(&mut self) -> &mut GenericEngineModel {
        &mut self.base
    }
}

impl FirstOrderLagModel {
    /// C++: `void FirstOrderLagModel::computeParameters()` [protected]
    /// Recomputes helper variables after loading parameters
    // void FirstOrderLagModel::computeParameters() {
    //     alpha = dt_s / (tau_s + dt_s);
    //     oneMinusAlpha = 1 - alpha;
    // }
    #[allow(unused_variables)]
    pub fn compute_parameters(&mut self) {
        self.alpha = self.dt_s / (self.tau_s + self.dt_s);
        self.oneMinusAlpha = 1.0 - self.alpha;
    }

    /// C++: `void FirstOrderLagModel::FirstOrderLagModel()`
    // FirstOrderLagModel::FirstOrderLagModel() : GenericEngineModel(),
    //     tau_s(0.5), dt_s(0.01) {
    //     className = "FirstOrderLagModel";
    //     computeParameters();
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut this = FirstOrderLagModel {
            base: GenericEngineModel::new(),
            tau_s: 0.5,
            dt_s: 0.01,
            alpha: 0.0,
            oneMinusAlpha: 0.0,
        };
        this.className = "FirstOrderLagModel".to_string();
        this.compute_parameters();
        this
    }

    /// C++: `void FirstOrderLagModel::~FirstOrderLagModel() noexcept` [virtual]
    // FirstOrderLagModel::~FirstOrderLagModel() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `double FirstOrderLagModel::getRealAcceleration(double, double, double, SUMOTime)` [virtual]
    /// Computes real vehicle acceleration given current speed, current acceleration, and requested acceleration. Acceleration can be negative as well. The model should handle decelerations as well
    // double FirstOrderLagModel::getRealAcceleration(double speed_mps, double accel_mps2, double reqAccel_mps2, SUMOTime timeStep) {
    //     UNUSED_PARAMETER(speed_mps);
    //     UNUSED_PARAMETER(timeStep);
    //     return std::min(
    //                maxAcceleration_mpsps,
    //                std::max(
    //                    -maxDeceleration_mpsps,
    //                    alpha * reqAccel_mps2 + oneMinusAlpha * accel_mps2
    //                )
    //            );
    // }
    #[allow(unused_variables)]
    pub fn get_real_acceleration(&mut self, speed_mps: f64, accel_mps2: f64, reqAccel_mps2: f64, timeStep: SUMOTime) -> f64 {
        // std::min(maxAcceleration, std::max(-maxDeceleration, v)), with the same comparisons as the C++ std functions
        let v = self.alpha * reqAccel_mps2 + self.oneMinusAlpha * accel_mps2;
        let lo = -self.maxDeceleration_mpsps;
        let m = if lo < v { v } else { lo };
        let hi = self.maxAcceleration_mpsps;
        if m < hi { m } else { hi }
    }

    /// C++: `void FirstOrderLagModel::loadParameters(const Parameterised::Map &)` [virtual]
    /// Load model parameters. This method requires a map of strings to be as flexible as possible, independently from the actual model implementation
    // void FirstOrderLagModel::loadParameters(const Parameterised::Map& parameters) {
    //     parseParameter(parameters, std::string(FOLM_PAR_TAU), tau_s);
    //     parseParameter(parameters, std::string(FOLM_PAR_DT), dt_s);
    //     computeParameters();
    // }
    #[allow(unused_variables)]
    pub fn load_parameters(&mut self, parameters: &ParameterisedMap) {
        let mut tau = self.tau_s;
        self.parse_parameter(parameters, FOLM_PAR_TAU.to_string(), &mut tau);
        self.tau_s = tau;
        let mut dt = self.dt_s;
        self.parse_parameter(parameters, FOLM_PAR_DT.to_string(), &mut dt);
        self.dt_s = dt;
        self.compute_parameters();
    }

    /// C++: `void FirstOrderLagModel::setParameter(const std::string, const std::string &)` [virtual]
    /// Sets a single parameter value
    // void FirstOrderLagModel::setParameter(const std::string parameter, const std::string& value) {
    //     UNUSED_PARAMETER(parameter);
    //     UNUSED_PARAMETER(value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, parameter: String, value: &str) {
        // C++ body is empty (parameters are unused)
    }

    /// C++: `void FirstOrderLagModel::setParameter(const std::string, double)` [virtual]
    // void FirstOrderLagModel::setParameter(const std::string parameter, double value) {
    //     if (parameter == FOLM_PAR_TAU) {
    //         tau_s = value;
    //     }
    //     if (parameter == FOLM_PAR_DT) {
    //         dt_s = value;
    //     }
    //     computeParameters();
    // }
    #[allow(unused_variables)]
    pub fn set_parameter_2(&mut self, parameter: String, value: f64) {
        if parameter == FOLM_PAR_TAU {
            self.tau_s = value;
        }
        if parameter == FOLM_PAR_DT {
            self.dt_s = value;
        }
        self.compute_parameters();
    }

    /// C++: `void FirstOrderLagModel::setParameter(const std::string, int)` [virtual]
    // void FirstOrderLagModel::setParameter(const std::string parameter, int value) {
    //     UNUSED_PARAMETER(parameter);
    //     UNUSED_PARAMETER(value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter_3(&mut self, parameter: String, value: i32) {
        // C++ body is empty (parameters are unused)
    }

}

