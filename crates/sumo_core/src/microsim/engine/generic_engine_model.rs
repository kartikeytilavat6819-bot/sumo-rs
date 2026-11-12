//! Port of microsim/engine/GenericEngineModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// This is an interface for plexe engine models. It provides two virtual methods that should be overridden by implementing classes: getRealAcceleration and loadParameters
/// C++ class `GenericEngineModel` (microsim/engine/GenericEngineModel.h)
pub struct GenericEngineModel {
    pub className: String,
    pub maxAcceleration_mpsps: f64,
    pub maxDeceleration_mpsps: f64,
}

impl GenericEngineModel {
    /// C++: `void GenericEngineModel::GenericEngineModel()`
    /// constructor
    // GenericEngineModel() : maxAcceleration_mpsps(1.5), maxDeceleration_mpsps(7) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            className: String::new(),
            maxAcceleration_mpsps: 1.5,
            maxDeceleration_mpsps: 7.0,
        }
    }

    /// C++: `void GenericEngineModel::~GenericEngineModel() noexcept` [virtual]
    /// destructor
    // virtual ~GenericEngineModel() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty: nothing to release.
    }

    /// C++: `double GenericEngineModel::getRealAcceleration(double, double, double, SUMOTime)` [virtual, pure virtual]
    /// Computes real vehicle acceleration given current speed, current acceleration, and requested acceleration. Acceleration can be negative as well. The model should handle decelerations as well
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_real_acceleration(&mut self, speed_mps: f64, accel_mps2: f64, reqAccel_mps2: f64, timeStep: SUMOTime) -> f64 {
        panic!("GenericEngineModel::getRealAcceleration is pure virtual; subclasses must override it")
    }

    /// C++: `void GenericEngineModel::setParameter(const std::string, const std::string &)` [virtual, pure virtual]
    /// Sets a single parameter value
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, parameter: String, value: &str) {
        panic!("GenericEngineModel::setParameter is pure virtual; subclasses must override it")
    }

    /// C++: `void GenericEngineModel::setParameter(const std::string, double)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_parameter_2(&mut self, parameter: String, value: f64) {
        panic!("GenericEngineModel::setParameter is pure virtual; subclasses must override it")
    }

    /// C++: `void GenericEngineModel::setParameter(const std::string, int)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_parameter_3(&mut self, parameter: String, value: i32) {
        panic!("GenericEngineModel::setParameter is pure virtual; subclasses must override it")
    }

    /// C++: `void GenericEngineModel::setMaximumAcceleration(double)`
    /// Sets maximum acceleration value
    // void GenericEngineModel::setMaximumAcceleration(double maxAcc) {
    //     maxAcceleration_mpsps = maxAcc;
    // }
    #[allow(unused_variables)]
    pub fn set_maximum_acceleration(&mut self, maxAcc: f64) {
        self.maxAcceleration_mpsps = maxAcc;
    }

    /// C++: `void GenericEngineModel::setMaximumDeceleration(double)`
    /// Sets maximum deceleration value
    // void GenericEngineModel::setMaximumDeceleration(double maxDec) {
    //     maxDeceleration_mpsps = maxDec;
    // }
    #[allow(unused_variables)]
    pub fn set_maximum_deceleration(&mut self, maxDec: f64) {
        self.maxDeceleration_mpsps = maxDec;
    }

    /// C++: `void GenericEngineModel::printParameterError(std::string, std::string)` [protected]
    /// Prints a parameter error
    // void GenericEngineModel::printParameterError(std::string parameter, std::string value) {
    //     std::cerr << className << ": invalid value " << value << " for parameter " << parameter << std::endl;
    // }
    #[allow(unused_variables)]
    pub fn print_parameter_error(&mut self, parameter: String, value: String) {
        eprintln!("{}: invalid value {} for parameter {}", self.className, value, parameter);
    }

    /// C++: `void GenericEngineModel::parseParameter(const Parameterised::Map &, std::string, double &)` [protected]
    /// Parses a value from the parameter map
    // void GenericEngineModel::parseParameter(const Parameterised::Map& parameters, std::string parameter, double& value) {
    //     Parameterised::Map::const_iterator par = parameters.find(parameter);
    //     if (par != parameters.end()) {
    //         try {
    //             value = StringUtils::toDouble(par->second);
    //         } catch (ProcessError&) {
    //             printParameterError(par->first, par->second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parameter(&mut self, parameters: &ParameterisedMap, parameter: String, value: &mut f64) {
        if let Some(par) = parameters.get(&parameter) {
            match utils::common::string_utils::to_double(par) {
                Ok(v) => *value = v,
                Err(_) => self.print_parameter_error(parameter.clone(), par.clone()),
            }
        }
    }

    /// C++: `void GenericEngineModel::parseParameter(const Parameterised::Map &, std::string, int &)` [protected]
    // void GenericEngineModel::parseParameter(const Parameterised::Map& parameters, std::string parameter, int& value) {
    //     Parameterised::Map::const_iterator par = parameters.find(parameter);
    //     if (par != parameters.end()) {
    //         try {
    //             value = StringUtils::toInt(par->second);
    //         } catch (ProcessError&) {
    //             printParameterError(par->first, par->second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parameter_2(&mut self, parameters: &ParameterisedMap, parameter: String, value: &mut i32) {
        if let Some(par) = parameters.get(&parameter) {
            match utils::common::string_utils::to_int(par) {
                Ok(v) => *value = v,
                Err(_) => self.print_parameter_error(parameter.clone(), par.clone()),
            }
        }
    }

    /// C++: `void GenericEngineModel::parseParameter(const Parameterised::Map &, std::string, std::string &)` [protected]
    // void GenericEngineModel::parseParameter(const Parameterised::Map& parameters, std::string parameter, std::string& value) {
    //     Parameterised::Map::const_iterator par = parameters.find(parameter);
    //     if (par != parameters.end()) {
    //         value = par->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_parameter_3(&mut self, parameters: &ParameterisedMap, parameter: String, value: &mut String) {
        if let Some(par) = parameters.get(&parameter) {
            *value = par.clone();
        }
    }

}

