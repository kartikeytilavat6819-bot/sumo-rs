//! Port of traci-server/TraCIServerAPI_Route.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting route values via TraCI
/// C++ class `TraCIServerAPI_Route` (traci-server/TraCIServerAPI_Route.h)
pub struct TraCIServerAPI_Route {
}

impl TraCIServerAPI_Route {
    /// C++: `bool TraCIServerAPI_Route::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change Route State)
    // bool
    // TraCIServerAPI_Route::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                  tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::ADD && variable != libsumo::REMOVE && variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_ROUTE_VARIABLE, "Change Route State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::ADD:
    //                 libsumo::Route::add(id, StoHelp::readTypedStringList(inputStorage, "A string list is needed for adding a new route."));
    //                 break;
    //             case libsumo::REMOVE: {
    //                 libsumo::Route::remove(id);
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Route::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_ROUTE_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_ROUTE_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != ADD && variable != REMOVE && variable != VAR_PARAMETER {
            return server.write_error_status_cmd(
                CMD_SET_ROUTE_VARIABLE,
                &format!("Change Route State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");

        // C++ catch (writeErrorStatusCmd) is reproduced by TraCIServer::dispatch_command, which turns the TraCIException panic into an error status
        // process
        match variable {
            ADD => {
                let edges = StorageHelper::read_typed_string_list(inputStorage, "A string list is needed for adding a new route.");
                Route::add(&id, &edges);
            }
            REMOVE => {
                Route::remove(&id);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Route::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_ROUTE_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Route::TraCIServerAPI_Route(const TraCIServerAPI_Route &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Route) -> Self {
        panic!("TraCIServerAPI_Route copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Route & TraCIServerAPI_Route::operator=(const TraCIServerAPI_Route &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Route) -> &mut TraCIServerAPI_Route {
        panic!("TraCIServerAPI_Route assignment operator is private and undefined in C++; it cannot be called")
    }

}

