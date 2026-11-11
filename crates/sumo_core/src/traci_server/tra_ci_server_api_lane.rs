//! Port of traci-server/TraCIServerAPI_Lane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting lane values via TraCI
/// C++ class `TraCIServerAPI_Lane` (traci-server/TraCIServerAPI_Lane.h)
pub struct TraCIServerAPI_Lane {
}

impl TraCIServerAPI_Lane {
    /// C++: `bool TraCIServerAPI_Lane::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc3: Change Lane State)
    // bool
    // TraCIServerAPI_Lane::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                 tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_MAXSPEED && variable != libsumo::VAR_LENGTH && variable != libsumo::LANE_ALLOWED && variable != libsumo::LANE_DISALLOWED
    //             && variable != libsumo::VAR_PARAMETER && variable != libsumo::LANE_CHANGES) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_LANE_VARIABLE, "Change Lane State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     MSLane* l = MSLane::dictionary(id);
    //     if (l == nullptr) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_LANE_VARIABLE, "Lane '" + id + "' is not known", outputStorage);
    //     }
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::VAR_MAXSPEED: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "The speed must be given as a double.");
    //                 libsumo::Lane::setMaxSpeed(id, value);
    //                 break;
    //             }
    //             case libsumo::VAR_FRICTION: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "The friction must be given as a double.");
    //                 libsumo::Lane::setFriction(id, value);
    //                 break;
    //             }
    //             case libsumo::VAR_LENGTH: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "The length must be given as a double.");
    //                 libsumo::Lane::setLength(id, value);
    //                 break;
    //             }
    //             case libsumo::LANE_ALLOWED: {
    //                 const std::vector<std::string> classes = StoHelp::readTypedStringList(inputStorage, "Allowed vehicle classes must be given as a list of strings.");
    //                 libsumo::Lane::setAllowed(id, classes);
    //                 break;
    //             }
    //             case libsumo::LANE_DISALLOWED: {
    //                 const std::vector<std::string> classes = StoHelp::readTypedStringList(inputStorage, "Not allowed vehicle classes must be given as a list of strings.");
    //                 libsumo::Lane::setDisallowed(id, classes);
    //                 break;
    //             }
    //             case libsumo::LANE_CHANGES: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting lane change permissions.");
    //                 const std::vector<std::string> classes = StoHelp::readTypedStringList(inputStorage, "Vehicle classes allowed to change lane must be given as a list of strings.");
    //                 const int direction = StoHelp::readTypedByte(inputStorage, "The lane change direction must be given as an integer.");
    //                 libsumo::Lane::setChangePermissions(id, classes, direction);
    //                 break;
    //             }
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Lane::setParameter(id, name, value);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_LANE_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_LANE_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        todo!("port TraCIServerAPI_Lane::processSet")
    }

    /// C++: `void TraCIServerAPI_Lane::TraCIServerAPI_Lane(const TraCIServerAPI_Lane &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Lane) -> Self {
        todo!("port TraCIServerAPI_Lane::TraCIServerAPI_Lane")
    }

    /// C++: `TraCIServerAPI_Lane & TraCIServerAPI_Lane::operator=(const TraCIServerAPI_Lane &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Lane) -> &mut TraCIServerAPI_Lane {
        todo!("port TraCIServerAPI_Lane::operator=")
    }

}

