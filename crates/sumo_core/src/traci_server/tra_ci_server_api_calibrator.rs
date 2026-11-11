//! Port of traci-server/TraCIServerAPI_Calibrator.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting Calibrator values via TraCI
/// C++ class `TraCIServerAPI_Calibrator` (traci-server/TraCIServerAPI_Calibrator.h)
pub struct TraCIServerAPI_Calibrator {
}

impl TraCIServerAPI_Calibrator {
    /// C++: `bool TraCIServerAPI_Calibrator::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change Calibrator State)
    // bool
    // TraCIServerAPI_Calibrator::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                       tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::CMD_SET_FLOW && variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_CALIBRATOR_VARIABLE, "Change Calibrator State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::CMD_SET_FLOW: {
    //                 StoHelp::readCompound(inputStorage, 8, "A compound object of size 8 is needed for setting calibrator flow.");
    //                 const double begin = StoHelp::readTypedDouble(inputStorage, "Setting flow requires the begin time as the first (double) value.");
    //                 const double end = StoHelp::readTypedDouble(inputStorage, "Setting flow requires the end time as the second (double) value.");
    //                 const double vehsPerHour = StoHelp::readTypedDouble(inputStorage, "Setting flow requires the number of vehicles per hour as the third (double) value.");
    //                 const double speed = StoHelp::readTypedDouble(inputStorage, "Setting flow requires the speed as the fourth (double) value.");
    //                 const std::string typeID = StoHelp::readTypedString(inputStorage, "Setting flow requires the type id as the fifth (string) value.");
    //                 const std::string routeID = StoHelp::readTypedString(inputStorage, "Setting flow requires the route id as the sixth (string) value.");
    //                 const std::string departLane = StoHelp::readTypedString(inputStorage, "Setting flow requires the departLane as the seventh (string) value.");
    //                 const std::string departSpeed = StoHelp::readTypedString(inputStorage, "Setting flow requires the departSpeed as the eigth (string) value.");
    //                 libsumo::Calibrator::setFlow(id, begin, end, vehsPerHour, speed, typeID, routeID, departLane, departSpeed);
    //                 break;
    //             }
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Calibrator::setParameter(id, name, value);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_CALIBRATOR_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_CALIBRATOR_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let mut warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != CMD_SET_FLOW && variable != VAR_PARAMETER {
            return server.write_error_status_cmd(
                CMD_SET_CALIBRATOR_VARIABLE,
                &format!("Change Calibrator State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");

        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            CMD_SET_FLOW => {
                // shortcut: see above
                StorageHelper::read_compound(inputStorage, 8, "A compound object of size 8 is needed for setting calibrator flow.");
                let begin = StorageHelper::read_typed_double(inputStorage, "Setting flow requires the begin time as the first (double) value.");
                let end = StorageHelper::read_typed_double(inputStorage, "Setting flow requires the end time as the second (double) value.");
                let vehsPerHour = StorageHelper::read_typed_double(inputStorage, "Setting flow requires the number of vehicles per hour as the third (double) value.");
                let speed = StorageHelper::read_typed_double(inputStorage, "Setting flow requires the speed as the fourth (double) value.");
                let typeID = StorageHelper::read_typed_string(inputStorage, "Setting flow requires the type id as the fifth (string) value.");
                let routeID = StorageHelper::read_typed_string(inputStorage, "Setting flow requires the route id as the sixth (string) value.");
                let departLane = StorageHelper::read_typed_string(inputStorage, "Setting flow requires the departLane as the seventh (string) value.");
                let departSpeed = StorageHelper::read_typed_string(inputStorage, "Setting flow requires the departSpeed as the eigth (string) value.");
                Calibrator::set_flow(&id, begin, end, vehsPerHour, speed, &typeID, &routeID, &departLane, &departSpeed);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Calibrator::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_CALIBRATOR_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Calibrator::TraCIServerAPI_Calibrator(const TraCIServerAPI_Calibrator &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Calibrator) -> Self {
        panic!("TraCIServerAPI_Calibrator copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Calibrator & TraCIServerAPI_Calibrator::operator=(const TraCIServerAPI_Calibrator &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Calibrator) -> &mut TraCIServerAPI_Calibrator {
        panic!("TraCIServerAPI_Calibrator assignment operator is private and undefined in C++; it cannot be called")
    }

}

