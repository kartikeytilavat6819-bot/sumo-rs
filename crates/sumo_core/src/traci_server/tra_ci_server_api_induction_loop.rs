//! Port of traci-server/TraCIServerAPI_InductionLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting induction loop values via TraCI
/// C++ class `TraCIServerAPI_InductionLoop` (traci-server/TraCIServerAPI_InductionLoop.h)
pub struct TraCIServerAPI_InductionLoop {
}

impl TraCIServerAPI_InductionLoop {
    /// C++: `bool TraCIServerAPI_InductionLoop::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc0: Set Induction Loop Variable)
    // bool
    // TraCIServerAPI_InductionLoop::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //         tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER
    //             && variable != libsumo::VAR_VIRTUAL_DETECTION
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_INDUCTIONLOOP_VARIABLE, "Set Induction Variable: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     // process
    //     try {
    //         switch (variable) {
    //             case libsumo::VAR_VIRTUAL_DETECTION: {
    //                 libsumo::InductionLoop::overrideTimeSinceDetection(id, StoHelp::readTypedDouble(inputStorage, "Setting time since last detection requires a double."));
    //                 break;
    //             }
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::InductionLoop::setParameter(id, name, value);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_INDUCTIONLOOP_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_INDUCTIONLOOP_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_PARAMETER && variable != VAR_VIRTUAL_DETECTION {
            return server.write_error_status_cmd(
                CMD_SET_INDUCTIONLOOP_VARIABLE,
                &format!("Set Induction Variable: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");
        // process
        // C++ catch (writeErrorStatusCmd) is reproduced by TraCIServer::dispatch_command, which turns the TraCIException panic into an error status
        match variable {
            VAR_VIRTUAL_DETECTION => {
                InductionLoop::override_time_since_detection(&id, StorageHelper::read_typed_double(inputStorage, "Setting time since last detection requires a double."));
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                InductionLoop::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_INDUCTIONLOOP_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_InductionLoop::TraCIServerAPI_InductionLoop(const TraCIServerAPI_InductionLoop &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_InductionLoop) -> Self {
        panic!("TraCIServerAPI_InductionLoop copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_InductionLoop & TraCIServerAPI_InductionLoop::operator=(const TraCIServerAPI_InductionLoop &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_InductionLoop) -> &mut TraCIServerAPI_InductionLoop {
        panic!("TraCIServerAPI_InductionLoop assignment operator is private and undefined in C++; it cannot be called")
    }

}

