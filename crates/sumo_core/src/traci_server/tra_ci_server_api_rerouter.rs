//! Port of traci-server/TraCIServerAPI_Rerouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting busstop values via TraCI
/// C++ class `TraCIServerAPI_Rerouter` (traci-server/TraCIServerAPI_Rerouter.h)
pub struct TraCIServerAPI_Rerouter {
}

impl TraCIServerAPI_Rerouter {
    /// C++: `bool TraCIServerAPI_Rerouter::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change Rerouter State)
    // bool
    // TraCIServerAPI_Rerouter::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                     tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_REROUTER_VARIABLE, "Change Rerouter State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Rerouter::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_REROUTER_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_REROUTER_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_PARAMETER {
            return server.write_error_status_cmd(
                CMD_SET_REROUTER_VARIABLE,
                &format!("Change Rerouter State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");

        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Rerouter::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_REROUTER_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Rerouter::TraCIServerAPI_Rerouter(const TraCIServerAPI_Rerouter &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Rerouter) -> Self {
        panic!("TraCIServerAPI_Rerouter copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Rerouter & TraCIServerAPI_Rerouter::operator=(const TraCIServerAPI_Rerouter &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Rerouter) -> &mut TraCIServerAPI_Rerouter {
        panic!("TraCIServerAPI_Rerouter assignment operator is private and undefined in C++; it cannot be called")
    }

}

