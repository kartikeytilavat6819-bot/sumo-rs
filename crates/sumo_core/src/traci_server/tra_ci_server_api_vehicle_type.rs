//! Port of traci-server/TraCIServerAPI_VehicleType.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting vehicle type values via TraCI
/// C++ class `TraCIServerAPI_VehicleType` (traci-server/TraCIServerAPI_VehicleType.h)
pub struct TraCIServerAPI_VehicleType {
}

impl TraCIServerAPI_VehicleType {
    /// C++: `bool TraCIServerAPI_VehicleType::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc5: Change Vehicle Type State)
    // bool
    // TraCIServerAPI_VehicleType::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                        tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_LENGTH && variable != libsumo::VAR_MAXSPEED && variable != libsumo::VAR_VEHICLECLASS
    //             && variable != libsumo::VAR_SPEED_FACTOR && variable != libsumo::VAR_SPEED_DEVIATION && variable != libsumo::VAR_EMISSIONCLASS
    //             && variable != libsumo::VAR_WIDTH && variable != libsumo::VAR_MINGAP && variable != libsumo::VAR_SHAPECLASS
    //             && variable != libsumo::VAR_ACCEL && variable != libsumo::VAR_IMPERFECTION
    //             && variable != libsumo::VAR_DECEL && variable != libsumo::VAR_EMERGENCY_DECEL && variable != libsumo::VAR_APPARENT_DECEL
    //             && variable != libsumo::VAR_TAU && variable != libsumo::VAR_COLOR && variable != libsumo::VAR_ACTIONSTEPLENGTH
    //             && variable != libsumo::VAR_SCALE
    //             && variable != libsumo::VAR_HEIGHT
    //             && variable != libsumo::VAR_MASS
    //             && variable != libsumo::VAR_MINGAP_LAT
    //             && variable != libsumo::VAR_MAXSPEED_LAT
    //             && variable != libsumo::VAR_LATALIGNMENT
    //             && variable != libsumo::VAR_BOARDING_DURATION
    //             && variable != libsumo::VAR_IMPATIENCE
    //             && variable != libsumo::VAR_PARAMETER
    //             && variable != libsumo::COPY
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLETYPE_VARIABLE,
    //                                           "Change Vehicle Type State: unsupported variable " + toHex(variable, 2)
    //                                           + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    // //    MSVehicleType* v = libsumo::VehicleType::getVType(id);
    // //    if (v == 0) {
    // //        return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLETYPE_VARIABLE, "Vehicle type '" + id + "' is not known",
    // //                                          outputStorage);
    // //    }
    //     // process
    //     try {
    //         if (setVariable(libsumo::CMD_SET_VEHICLETYPE_VARIABLE, variable, id, server, inputStorage, outputStorage)) {
    //             server.writeStatusCmd(libsumo::CMD_SET_VEHICLETYPE_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //             return true;
    //         }
    //     } catch (ProcessError& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLETYPE_VARIABLE, e.what(), outputStorage);
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLETYPE_VARIABLE, e.what(), outputStorage);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_LENGTH
            && variable != VAR_MAXSPEED
            && variable != VAR_VEHICLECLASS
            && variable != VAR_SPEED_FACTOR
            && variable != VAR_SPEED_DEVIATION
            && variable != VAR_EMISSIONCLASS
            && variable != VAR_WIDTH
            && variable != VAR_MINGAP
            && variable != VAR_SHAPECLASS
            && variable != VAR_ACCEL
            && variable != VAR_IMPERFECTION
            && variable != VAR_DECEL
            && variable != VAR_EMERGENCY_DECEL
            && variable != VAR_APPARENT_DECEL
            && variable != VAR_TAU
            && variable != VAR_COLOR
            && variable != VAR_ACTIONSTEPLENGTH
            && variable != VAR_SCALE
            && variable != VAR_HEIGHT
            && variable != VAR_MASS
            && variable != VAR_MINGAP_LAT
            && variable != VAR_MAXSPEED_LAT
            && variable != VAR_LATALIGNMENT
            && variable != VAR_BOARDING_DURATION
            && variable != VAR_IMPATIENCE
            && variable != VAR_PARAMETER
            && variable != COPY
        {
            return server.write_error_status_cmd(
                CMD_SET_VEHICLETYPE_VARIABLE,
                &format!("Change Vehicle Type State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");
        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        if Self::set_variable(CMD_SET_VEHICLETYPE_VARIABLE, variable, &id, server, inputStorage, outputStorage) {
            server.write_status_cmd(CMD_SET_VEHICLETYPE_VARIABLE, RTYPE_OK, &warning, outputStorage);
            return true;
        }
        false
    }

    /// C++: `bool TraCIServerAPI_VehicleType::setVariable(const int, const int, const std::string &, TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value for the given type
    // bool
    // TraCIServerAPI_VehicleType::setVariable(const int cmd, const int variable,
    //                                         const std::string& id, TraCIServer& server,
    //                                         tcpip::Storage& inputStorage, tcpip::Storage& outputStorage) {
    //     switch (variable) {
    //         case libsumo::VAR_LENGTH: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting length requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid length.", outputStorage);
    //             }
    //             libsumo::VehicleType::setLength(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_HEIGHT: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting height requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid height.", outputStorage);
    //             }
    //             libsumo::VehicleType::setHeight(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_MASS: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting mass requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid mass.", outputStorage);
    //             }
    //             libsumo::VehicleType::setMass(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_MAXSPEED: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting maximum speed requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid maximum speed.", outputStorage);
    //             }
    //             libsumo::VehicleType::setMaxSpeed(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_VEHICLECLASS: {
    //             libsumo::VehicleType::setVehicleClass(id, StoHelp::readTypedString(inputStorage, "Setting vehicle class requires a string."));
    //             break;
    //         }
    //         case libsumo::VAR_SPEED_FACTOR: {
    //             double value = StoHelp::readTypedDouble(inputStorage, "Setting speed factor requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid speed factor.", outputStorage);
    //             }
    //             libsumo::VehicleType::setSpeedFactor(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_SPEED_DEVIATION: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting speed deviation requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid speed deviation.", outputStorage);
    //             }
    //             libsumo::VehicleType::setSpeedDeviation(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_EMISSIONCLASS: {
    //             libsumo::VehicleType::setEmissionClass(id, StoHelp::readTypedString(inputStorage, "Setting emission class requires a string."));
    //             break;
    //         }
    //         case libsumo::VAR_WIDTH: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting width requires a double.");
    //             if (value <= 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid width.", outputStorage);
    //             }
    //             libsumo::VehicleType::setWidth(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_MINGAP: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting minimum gap requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid minimum gap.", outputStorage);
    //             }
    //             libsumo::VehicleType::setMinGap(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_MINGAP_LAT: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting minimum lateral gap requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid minimum lateral gap.", outputStorage);
    //             }
    //             libsumo::VehicleType::setMinGapLat(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_MAXSPEED_LAT: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting maximum lateral speed requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid maximum lateral speed.", outputStorage);
    //             }
    //             libsumo::VehicleType::setMaxSpeedLat(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_LATALIGNMENT: {
    //             libsumo::VehicleType::setLateralAlignment(id, StoHelp::readTypedString(inputStorage, "Setting preferred lateral alignment requires a string."));
    //             break;
    //         }
    //         case libsumo::VAR_SHAPECLASS: {
    //             libsumo::VehicleType::setShapeClass(id, StoHelp::readTypedString(inputStorage, "Setting vehicle shape requires a string."));
    //             break;
    //         }
    //         case libsumo::VAR_ACCEL: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting acceleration requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid acceleration.", outputStorage);
    //             }
    //             libsumo::VehicleType::setAccel(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_DECEL: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting deceleration requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid deceleration.", outputStorage);
    //             }
    //             libsumo::VehicleType::setDecel(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_EMERGENCY_DECEL: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting emergency deceleration requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid emergency deceleration.", outputStorage);
    //             }
    //             libsumo::VehicleType::setEmergencyDecel(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_APPARENT_DECEL: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting apparent deceleration requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid apparent deceleration.", outputStorage);
    //             }
    //             libsumo::VehicleType::setApparentDecel(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_SCALE: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Traffic scale requires a double.");
    //             if (value < 0.0) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid traffic scale.", outputStorage);
    //             }
    //             libsumo::VehicleType::setScale(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_ACTIONSTEPLENGTH: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting action step length requires a double.");
    //             if (fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid action step length.", outputStorage);
    //             }
    //             bool resetActionOffset = value >= 0.0;
    //             libsumo::VehicleType::setActionStepLength(id, fabs(value), resetActionOffset);
    //             break;
    //         }
    //         case libsumo::VAR_IMPERFECTION: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting driver imperfection requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid driver imperfection.", outputStorage);
    //             }
    //             libsumo::VehicleType::setImperfection(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_TAU: {
    //             const double value = StoHelp::readTypedDouble(inputStorage, "Setting headway time requires a double.");
    //             if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                 return server.writeErrorStatusCmd(cmd, "Invalid headway time.", outputStorage);
    //             }
    //             libsumo::VehicleType::setTau(id, value);
    //             break;
    //         }
    //         case libsumo::VAR_IMPATIENCE: {
    //             libsumo::VehicleType::setImpatience(id, StoHelp::readTypedDouble(inputStorage, "Setting impatience requires a double."));
    //             break;
    //         }
    //         case libsumo::VAR_BOARDING_DURATION: {
    //             libsumo::VehicleType::setBoardingDuration(id, StoHelp::readTypedDouble(inputStorage, "Setting boarding duration requires a double."));
    //             break;
    //         }
    //         case libsumo::VAR_COLOR: {
    //             libsumo::VehicleType::setColor(id, StoHelp::readTypedColor(inputStorage, "The color must be given using the according type."));
    //             break;
    //         }
    //         case libsumo::COPY: {
    //             libsumo::VehicleType::copy(id, StoHelp::readTypedString(inputStorage, "Copying a vehicle type requires a string."));
    //             break;
    //         }
    //         case libsumo::VAR_PARAMETER: {
    //             StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //             const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //             const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //             libsumo::VehicleType::setParameter(id, name, value);
    //             break;
    //         }
    //         default:
    //             break;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn set_variable(cmd: i32, variable: i32, id: &str, server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        match variable {
            VAR_LENGTH => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting length requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid length.", outputStorage);
                }
                VehicleType::set_length(id, value);
            }
            VAR_HEIGHT => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting height requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid height.", outputStorage);
                }
                VehicleType::set_height(id, value);
            }
            VAR_MASS => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting mass requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid mass.", outputStorage);
                }
                VehicleType::set_mass(id, value);
            }
            VAR_MAXSPEED => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting maximum speed requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid maximum speed.", outputStorage);
                }
                VehicleType::set_max_speed(id, value);
            }
            VAR_VEHICLECLASS => {
                VehicleType::set_vehicle_class(id, &StorageHelper::read_typed_string(inputStorage, "Setting vehicle class requires a string."));
            }
            VAR_SPEED_FACTOR => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting speed factor requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid speed factor.", outputStorage);
                }
                VehicleType::set_speed_factor(id, value);
            }
            VAR_SPEED_DEVIATION => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting speed deviation requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid speed deviation.", outputStorage);
                }
                VehicleType::set_speed_deviation(id, value);
            }
            VAR_EMISSIONCLASS => {
                VehicleType::set_emission_class(id, &StorageHelper::read_typed_string(inputStorage, "Setting emission class requires a string."));
            }
            VAR_WIDTH => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting width requires a double.");
                if value <= 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid width.", outputStorage);
                }
                VehicleType::set_width(id, value);
            }
            VAR_MINGAP => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting minimum gap requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid minimum gap.", outputStorage);
                }
                VehicleType::set_min_gap(id, value);
            }
            VAR_MINGAP_LAT => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting minimum lateral gap requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid minimum lateral gap.", outputStorage);
                }
                VehicleType::set_min_gap_lat(id, value);
            }
            VAR_MAXSPEED_LAT => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting maximum lateral speed requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid maximum lateral speed.", outputStorage);
                }
                VehicleType::set_max_speed_lat(id, value);
            }
            VAR_LATALIGNMENT => {
                VehicleType::set_lateral_alignment(id, &StorageHelper::read_typed_string(inputStorage, "Setting preferred lateral alignment requires a string."));
            }
            VAR_SHAPECLASS => {
                VehicleType::set_shape_class(id, &StorageHelper::read_typed_string(inputStorage, "Setting vehicle shape requires a string."));
            }
            VAR_ACCEL => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting acceleration requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid acceleration.", outputStorage);
                }
                VehicleType::set_accel(id, value);
            }
            VAR_DECEL => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting deceleration requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid deceleration.", outputStorage);
                }
                VehicleType::set_decel(id, value);
            }
            VAR_EMERGENCY_DECEL => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting emergency deceleration requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid emergency deceleration.", outputStorage);
                }
                VehicleType::set_emergency_decel(id, value);
            }
            VAR_APPARENT_DECEL => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting apparent deceleration requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid apparent deceleration.", outputStorage);
                }
                VehicleType::set_apparent_decel(id, value);
            }
            VAR_SCALE => {
                let value = StorageHelper::read_typed_double(inputStorage, "Traffic scale requires a double.");
                if value < 0.0 {
                    return server.write_error_status_cmd(cmd, "Invalid traffic scale.", outputStorage);
                }
                VehicleType::set_scale(id, value);
            }
            VAR_ACTIONSTEPLENGTH => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting action step length requires a double.");
                if value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid action step length.", outputStorage);
                }
                let resetActionOffset = value >= 0.0;
                VehicleType::set_action_step_length(id, value.abs(), resetActionOffset);
            }
            VAR_IMPERFECTION => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting driver imperfection requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid driver imperfection.", outputStorage);
                }
                VehicleType::set_imperfection(id, value);
            }
            VAR_TAU => {
                let value = StorageHelper::read_typed_double(inputStorage, "Setting headway time requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(cmd, "Invalid headway time.", outputStorage);
                }
                VehicleType::set_tau(id, value);
            }
            VAR_IMPATIENCE => {
                VehicleType::set_impatience(id, StorageHelper::read_typed_double(inputStorage, "Setting impatience requires a double."));
            }
            VAR_BOARDING_DURATION => {
                VehicleType::set_boarding_duration(id, StorageHelper::read_typed_double(inputStorage, "Setting boarding duration requires a double."));
            }
            VAR_COLOR => {
                VehicleType::set_color(id, &StorageHelper::read_typed_color(inputStorage, "The color must be given using the according type."));
            }
            COPY => {
                VehicleType::copy(id, &StorageHelper::read_typed_string(inputStorage, "Copying a vehicle type requires a string."));
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                VehicleType::set_parameter(id, &name, &value);
            }
            _ => {}
        }
        true
    }

    /// C++: `void TraCIServerAPI_VehicleType::TraCIServerAPI_VehicleType(const TraCIServerAPI_VehicleType &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_VehicleType) -> Self {
        panic!("TraCIServerAPI_VehicleType copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_VehicleType & TraCIServerAPI_VehicleType::operator=(const TraCIServerAPI_VehicleType &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_VehicleType) -> &mut TraCIServerAPI_VehicleType {
        panic!("TraCIServerAPI_VehicleType assignment operator is private and undefined in C++; it cannot be called")
    }

}

