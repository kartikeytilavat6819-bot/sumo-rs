//! Port of traci-server/TraCIServerAPI_Edge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting edge values via TraCI
/// C++ class `TraCIServerAPI_Edge` (traci-server/TraCIServerAPI_Edge.h)
pub struct TraCIServerAPI_Edge {
}

impl TraCIServerAPI_Edge {
    /// C++: `bool TraCIServerAPI_Edge::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xca: Change Edge State)
    // bool
    // TraCIServerAPI_Edge::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                 tcpip::Storage& outputStorage) {
    //     std::string warning; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_EDGE_TRAVELTIME
    //             && variable != libsumo::VAR_EDGE_EFFORT
    //             && variable != libsumo::VAR_MAXSPEED
    //             && variable != libsumo::LANE_ALLOWED
    //             && variable != libsumo::LANE_DISALLOWED
    //             && variable != libsumo::VAR_FRICTION
    //             && variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_EDGE_VARIABLE,
    //                                           "Change Edge State: unsupported variable " + toHex(variable, 2)
    //                                           + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::LANE_ALLOWED: {
    //                 // read and set allowed vehicle classes
    //                 const std::vector<std::string> classes = StoHelp::readTypedStringList(inputStorage, "Allowed vehicle classes must be given as a list of strings.");
    //                 libsumo::Edge::setAllowed(id, classes);
    //                 break;
    //             }
    //             case libsumo::LANE_DISALLOWED: {
    //                 // read and set disallowed vehicle classes
    //                 const std::vector<std::string> classes = StoHelp::readTypedStringList(inputStorage, "Not allowed vehicle classes must be given as a list of strings.");
    //                 libsumo::Edge::setDisallowed(id, classes);
    //                 break;
    //             }
    //             case libsumo::VAR_EDGE_TRAVELTIME: {
    //                 // read and set travel time
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Setting travel time requires a compound object.");
    //                 if (parameterCount == 3) {
    //                     // bound by time
    //                     const double begTime = StoHelp::readTypedDouble(inputStorage, "The first variable must be the begin time given as double.");
    //                     const double endTime = StoHelp::readTypedDouble(inputStorage, "The second variable must be the end time given as double.");
    //                     const double value = StoHelp::readTypedDouble(inputStorage, "The third variable must be the value given as double.");
    //                     libsumo::Edge::adaptTraveltime(id, value, begTime, endTime);
    //                 } else if (parameterCount == 1) {
    //                     // unbound
    //                     const double value = StoHelp::readTypedDouble(inputStorage, "The variable must be the value given as double.");
    //                     libsumo::Edge::adaptTraveltime(id, value, 0., std::numeric_limits<double>::max());
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_EDGE_VARIABLE,
    //                                                       "Setting travel time requires either begin time, end time, and value, or only value as parameter.",
    //                                                       outputStorage);
    //                 }
    //                 break;
    //             }
    //             case libsumo::VAR_EDGE_EFFORT: {
    //                 // read and set effort
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Setting effort requires a compound object.");
    //                 if (parameterCount == 3) {
    //                     // bound by time
    //                     const double begTime = StoHelp::readTypedDouble(inputStorage, "The first variable must be the begin time given as double.");
    //                     const double endTime = StoHelp::readTypedDouble(inputStorage, "The second variable must be the end time given as double.");
    //                     const double value = StoHelp::readTypedDouble(inputStorage, "The third variable must be the value given as double.");
    //                     libsumo::Edge::setEffort(id, value, begTime, endTime);
    //                 } else if (parameterCount == 1) {
    //                     // unbound
    //                     const double value = StoHelp::readTypedDouble(inputStorage, "The variable must be the value given as double.");
    //                     libsumo::Edge::setEffort(id, value, 0., std::numeric_limits<double>::max());
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_EDGE_VARIABLE,
    //                                                       "Setting effort requires either begin time, end time, and value, or only value as parameter.",
    //                                                       outputStorage);
    //                 }
    //                 break;
    //             }
    //             case libsumo::VAR_MAXSPEED: {
    //                 // read and set max. speed
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "The speed must be given as a double.");
    //                 libsumo::Edge::setMaxSpeed(id, value);
    //                 break;
    //             }
    //             case libsumo::VAR_FRICTION: {
    //                 // read and set friction for entire edge
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "The friction must be given as a double.");
    //                 libsumo::Edge::setFriction(id, value);
    //                 break;
    //             }
    //             case libsumo::VAR_PARAMETER: {
    //                 // read and check item number
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Edge::setParameter(id, name, value);
    //                 break;
    //             }
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_EDGE_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_EDGE_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let mut warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != VAR_EDGE_TRAVELTIME
            && variable != VAR_EDGE_EFFORT
            && variable != VAR_MAXSPEED
            && variable != LANE_ALLOWED
            && variable != LANE_DISALLOWED
            && variable != VAR_FRICTION
            && variable != VAR_PARAMETER
        {
            return server.write_error_status_cmd(
                CMD_SET_EDGE_VARIABLE,
                &format!("Change Edge State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");
        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            LANE_ALLOWED => {
                // read and set allowed vehicle classes
                let classes = StorageHelper::read_typed_string_list(inputStorage, "Allowed vehicle classes must be given as a list of strings.");
                Edge::set_allowed_2(&id, classes);
            }
            LANE_DISALLOWED => {
                // read and set disallowed vehicle classes
                let classes = StorageHelper::read_typed_string_list(inputStorage, "Not allowed vehicle classes must be given as a list of strings.");
                Edge::set_disallowed_2(&id, classes);
            }
            VAR_EDGE_TRAVELTIME => {
                // read and set travel time
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "Setting travel time requires a compound object.");
                if parameterCount == 3 {
                    // bound by time
                    let begTime = StorageHelper::read_typed_double(inputStorage, "The first variable must be the begin time given as double.");
                    let endTime = StorageHelper::read_typed_double(inputStorage, "The second variable must be the end time given as double.");
                    let value = StorageHelper::read_typed_double(inputStorage, "The third variable must be the value given as double.");
                    Edge::adapt_traveltime(&id, value, begTime, endTime);
                } else if parameterCount == 1 {
                    // unbound
                    let value = StorageHelper::read_typed_double(inputStorage, "The variable must be the value given as double.");
                    Edge::adapt_traveltime(&id, value, 0., f64::MAX);
                } else {
                    return server.write_error_status_cmd(
                        CMD_SET_EDGE_VARIABLE,
                        "Setting travel time requires either begin time, end time, and value, or only value as parameter.",
                        outputStorage,
                    );
                }
            }
            VAR_EDGE_EFFORT => {
                // read and set effort
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "Setting effort requires a compound object.");
                if parameterCount == 3 {
                    // bound by time
                    let begTime = StorageHelper::read_typed_double(inputStorage, "The first variable must be the begin time given as double.");
                    let endTime = StorageHelper::read_typed_double(inputStorage, "The second variable must be the end time given as double.");
                    let value = StorageHelper::read_typed_double(inputStorage, "The third variable must be the value given as double.");
                    Edge::set_effort(&id, value, begTime, endTime);
                } else if parameterCount == 1 {
                    // unbound
                    let value = StorageHelper::read_typed_double(inputStorage, "The variable must be the value given as double.");
                    Edge::set_effort(&id, value, 0., f64::MAX);
                } else {
                    return server.write_error_status_cmd(
                        CMD_SET_EDGE_VARIABLE,
                        "Setting effort requires either begin time, end time, and value, or only value as parameter.",
                        outputStorage,
                    );
                }
            }
            VAR_MAXSPEED => {
                // read and set max. speed
                let value = StorageHelper::read_typed_double(inputStorage, "The speed must be given as a double.");
                Edge::set_max_speed(&id, value);
            }
            VAR_FRICTION => {
                // read and set friction for entire edge
                let value = StorageHelper::read_typed_double(inputStorage, "The friction must be given as a double.");
                Edge::set_friction(&id, value);
            }
            VAR_PARAMETER => {
                // read and check item number
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Edge::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_EDGE_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Edge::TraCIServerAPI_Edge(const TraCIServerAPI_Edge &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Edge) -> Self {
        panic!("TraCIServerAPI_Edge copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Edge & TraCIServerAPI_Edge::operator=(const TraCIServerAPI_Edge &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Edge) -> &mut TraCIServerAPI_Edge {
        panic!("TraCIServerAPI_Edge assignment operator is private and undefined in C++; it cannot be called")
    }

}

