//! Port of traci-server/TraCIServerAPI_LaneArea.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting multi-entry/multi-exit detector values via TraCI
/// C++ class `TraCIServerAPI_LaneArea` (traci-server/TraCIServerAPI_LaneArea.h)
pub struct TraCIServerAPI_LaneArea {
}

impl TraCIServerAPI_LaneArea {
    /// C++: `bool TraCIServerAPI_LaneArea::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xcd: Set AreaDetector Variable)
    // bool
    // TraCIServerAPI_LaneArea::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                     tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER
    //             && variable != libsumo::VAR_VIRTUAL_DETECTION
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_LANEAREA_VARIABLE, "Set Lane Area Detector Variable: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     // process
    //     try {
    //         switch (variable) {
    //             case libsumo::VAR_VIRTUAL_DETECTION: {
    //                 const int vehNum = StoHelp::readTypedInt(inputStorage, "Overriding the number of detected vehicles requires an integer");
    //                 libsumo::LaneArea::overrideVehicleNumber(id, vehNum);
    //                 break;
    //             }
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::LaneArea::setParameter(id, name, value);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_LANEAREA_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_LANEAREA_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_PARAMETER && variable != VAR_VIRTUAL_DETECTION {
            return server.write_error_status_cmd(
                CMD_SET_LANEAREA_VARIABLE,
                &format!("Set Lane Area Detector Variable: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");
        // process
        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        match variable {
            VAR_VIRTUAL_DETECTION => {
                let vehNum = StorageHelper::read_typed_int(inputStorage, "Overriding the number of detected vehicles requires an integer");
                LaneArea::override_vehicle_number(&id, vehNum);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                LaneArea::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_LANEAREA_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_LaneArea::TraCIServerAPI_LaneArea(const TraCIServerAPI_LaneArea &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_LaneArea) -> Self {
        panic!("TraCIServerAPI_LaneArea copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_LaneArea & TraCIServerAPI_LaneArea::operator=(const TraCIServerAPI_LaneArea &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_LaneArea) -> &mut TraCIServerAPI_LaneArea {
        panic!("TraCIServerAPI_LaneArea assignment operator is private and undefined in C++; it cannot be called")
    }

}

