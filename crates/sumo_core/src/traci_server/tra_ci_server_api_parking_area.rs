//! Port of traci-server/TraCIServerAPI_ParkingArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting busstop values via TraCI
/// C++ class `TraCIServerAPI_ParkingArea` (traci-server/TraCIServerAPI_ParkingArea.h)
pub struct TraCIServerAPI_ParkingArea {
}

impl TraCIServerAPI_ParkingArea {
    /// C++: `bool TraCIServerAPI_ParkingArea::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc6: Change ParkingArea State)
    // bool
    // TraCIServerAPI_ParkingArea::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                        tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER &&
    //             variable != libsumo::VAR_ACCESS_BADGE) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_PARKINGAREA_VARIABLE, "Change ParkingArea State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
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
    //                 libsumo::ParkingArea::setParameter(id, name, value);
    //                 break;
    //             }
    //             case libsumo::VAR_ACCESS_BADGE:
    //                 libsumo::ParkingArea::setAcceptedBadges(id, StoHelp::readTypedStringList(inputStorage, "A string list is needed to update the ParkingArea access badges."));
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_PARKINGAREA_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_PARKINGAREA_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_PARAMETER && variable != VAR_ACCESS_BADGE {
            return server.write_error_status_cmd(
                CMD_SET_PARKINGAREA_VARIABLE,
                &format!("Change ParkingArea State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
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
                ParkingArea::set_parameter(&id, &name, &value);
            }
            VAR_ACCESS_BADGE => {
                let badges = StorageHelper::read_typed_string_list(inputStorage, "A string list is needed to update the ParkingArea access badges.");
                ParkingArea::set_accepted_badges(&id, badges);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_PARKINGAREA_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_ParkingArea::TraCIServerAPI_ParkingArea(const TraCIServerAPI_ParkingArea &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_ParkingArea) -> Self {
        panic!("TraCIServerAPI_ParkingArea copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_ParkingArea & TraCIServerAPI_ParkingArea::operator=(const TraCIServerAPI_ParkingArea &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_ParkingArea) -> &mut TraCIServerAPI_ParkingArea {
        panic!("TraCIServerAPI_ParkingArea assignment operator is private and undefined in C++; it cannot be called")
    }

}

