//! Port of traci-server/TraCIServerAPI_Person.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting person values via TraCI
/// C++ class `TraCIServerAPI_Person` (traci-server/TraCIServerAPI_Person.h)
pub struct TraCIServerAPI_Person {
}

impl TraCIServerAPI_Person {
    /// C++: `bool TraCIServerAPI_Person::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xce: Change Person State)
    // bool
    // TraCIServerAPI_Person::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                   tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_PARAMETER
    //             && variable != libsumo::ADD
    //             && variable != libsumo::REMOVE
    //             && variable != libsumo::APPEND_STAGE
    //             && variable != libsumo::REPLACE_STAGE
    //             && variable != libsumo::REMOVE_STAGE
    //             && variable != libsumo::CMD_REROUTE_TRAVELTIME
    //             && variable != libsumo::VAR_MOVE_TO
    //             && variable != libsumo::MOVE_TO_XY
    //             && variable != libsumo::VAR_SPEED
    //             && variable != libsumo::VAR_TYPE
    //             && variable != libsumo::VAR_SPEED_FACTOR
    //             && variable != libsumo::VAR_LENGTH
    //             && variable != libsumo::VAR_WIDTH
    //             && variable != libsumo::VAR_HEIGHT
    //             && variable != libsumo::VAR_MINGAP
    //             && variable != libsumo::VAR_COLOR
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Change Person State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //
    //     try {
    //         // TODO: remove declaration of c after completion
    //         MSTransportableControl& c = MSNet::getInstance()->getPersonControl();
    //         // id
    //         std::string id = inputStorage.readString();
    //         // TODO: remove declaration of p after completion
    //         const bool shouldExist = variable != libsumo::ADD;
    //         MSTransportable* p = c.get(id);
    //         if (p == nullptr && shouldExist) {
    //             return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Person '" + id + "' is not known", outputStorage);
    //         }
    //         // process
    //         switch (variable) {
    //             case libsumo::VAR_SPEED: {
    //                 // set the speed for all present and future (walking) stages and modify the vType so that stages added later are also affected
    //                 libsumo::Person::setSpeed(id, StoHelp::readTypedDouble(inputStorage, "Setting speed requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_TYPE: {
    //                 libsumo::Person::setType(id, StoHelp::readTypedString(inputStorage, "The vehicle type id must be given as a string."));
    //                 break;
    //             }
    //             case libsumo::VAR_SPEED_FACTOR: {
    //                 libsumo::Person::setSpeedFactor(id, StoHelp::readTypedDouble(inputStorage, "Setting SpeedFactor requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_COLOR: {
    //                 libsumo::Person::setColor(id, StoHelp::readTypedColor(inputStorage, "The color must be given using the according type."));
    //                 break;
    //             }
    //             case libsumo::ADD: {
    //                 StoHelp::readCompound(inputStorage, 4, "Adding a person needs four parameters.");
    //                 const std::string vTypeID = StoHelp::readTypedString(inputStorage, "First parameter (type) requires a string.");
    //                 const std::string edgeID = StoHelp::readTypedString(inputStorage, "Second parameter (edge) requires a string.");
    //                 const double depart = StoHelp::readTypedDouble(inputStorage, "Third parameter (depart) requires a double.");
    //                 const double pos = StoHelp::readTypedDouble(inputStorage, "Fourth parameter (position) requires a double.");
    //                 libsumo::Person::add(id, edgeID, pos, depart, vTypeID);
    //             }
    //             break;
    //             case libsumo::REMOVE: {
    //                 libsumo::Person::remove(id, (char)StoHelp::readTypedByte(inputStorage, "Removing a person requires a byte."));
    //             }
    //             break;
    //             case libsumo::APPEND_STAGE: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Adding a person stage requires a compound object.");
    //                 if (parameterCount == 13) {
    //                     libsumo::TraCIStage stage;
    //                     StoHelp::readStage(inputStorage, stage);
    //                     libsumo::Person::appendStage(id, stage);
    //                 } else {
    //                     const int stageType = StoHelp::readTypedInt(inputStorage, "The first parameter for adding a stage must be the stage type given as int.");
    //                     if (stageType == libsumo::STAGE_DRIVING) {
    //                         // append driving stage
    //                         if (parameterCount != 4) {
    //                             return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Adding a driving stage needs four parameters.", outputStorage);
    //                         }
    //                         const std::string edgeID = StoHelp::readTypedString(inputStorage, "Second parameter (edge) requires a string.");
    //                         const std::string lines = StoHelp::readTypedString(inputStorage, "Third parameter (lines) requires a string.");
    //                         const std::string stopID = StoHelp::readTypedString(inputStorage, "Fourth parameter (stopID) requires a string.");
    //                         libsumo::Person::appendDrivingStage(id, edgeID, lines, stopID);
    //                     } else if (stageType == libsumo::STAGE_WAITING) {
    //                         // append waiting stage
    //                         if (parameterCount != 4) {
    //                             return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Adding a waiting stage needs four parameters.", outputStorage);
    //                         }
    //                         const double duration = StoHelp::readTypedDouble(inputStorage, "Second parameter (duration) requires a double.");
    //                         const std::string description = StoHelp::readTypedString(inputStorage, "Third parameter (description) requires a string.");
    //                         const std::string stopID = StoHelp::readTypedString(inputStorage, "Fourth parameter (stopID) requires a string.");
    //                         libsumo::Person::appendWaitingStage(id, duration, description, stopID);
    //                     } else if (stageType == libsumo::STAGE_WALKING) {
    //                         // append walking stage
    //                         if (parameterCount != 6) {
    //                             return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Adding a walking stage needs six parameters.", outputStorage);
    //                         }
    //                         const std::vector<std::string> edgeIDs = StoHelp::readTypedStringList(inputStorage, "Second parameter (edges) route must be defined as a list of edge ids.");
    //                         const double arrivalPos = StoHelp::readTypedDouble(inputStorage, "Third parameter (arrivalPos) requires a double.");
    //                         const double duration = StoHelp::readTypedDouble(inputStorage, "Fourth parameter (duration) requires a double.");
    //                         const double speed = StoHelp::readTypedDouble(inputStorage, "Fifth parameter (speed) requires a double.");
    //                         const std::string stopID = StoHelp::readTypedString(inputStorage, "Sixth parameter (stopID) requires a string.");
    //                         libsumo::Person::appendWalkingStage(id, edgeIDs, arrivalPos, duration, speed, stopID);
    //                     } else {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "Invalid stage type for person '" + id + "'", outputStorage);
    //                     }
    //                 }
    //             }
    //             break;
    //             case libsumo::REPLACE_STAGE: {
    //                 StoHelp::readCompound(inputStorage, 2, "Replacing a person stage requires a compound object of size 2.");
    //                 const int nextStageIndex = StoHelp::readTypedInt(inputStorage, "First parameter of replace stage should be an integer");
    //                 StoHelp::readCompound(inputStorage, 13, "Second parameter of replace stage should be a compound object of size 13");
    //                 libsumo::TraCIStage stage;
    //                 StoHelp::readStage(inputStorage, stage);
    //                 libsumo::Person::replaceStage(id, nextStageIndex, stage);
    //             }
    //             break;
    //             case libsumo::REMOVE_STAGE: {
    //                 libsumo::Person::removeStage(id, StoHelp::readTypedInt(inputStorage, "The message must contain the stage index."));
    //             }
    //             break;
    //             case libsumo::CMD_REROUTE_TRAVELTIME: {
    //                 StoHelp::readCompound(inputStorage, 0, "Resuming requires an empty compound object.");
    //                 libsumo::Person::rerouteTraveltime(id);
    //             }
    //             break;
    //             case libsumo::VAR_MOVE_TO: {
    //                 StoHelp::readCompound(inputStorage, 3, "Setting position should obtain the edge id, the position and the lateral position.");
    //                 const std::string laneID = StoHelp::readTypedString(inputStorage, "The first parameter for setting a position must be the laneID given as a string.");
    //                 const double position = StoHelp::readTypedDouble(inputStorage, "The second parameter for setting a position must be the position given as a double.");
    //                 const double posLat = StoHelp::readTypedDouble(inputStorage, "The third parameter for setting a position must be the lateral position given as a double.");
    //                 libsumo::Person::moveTo(id, laneID, position, posLat);
    //             }
    //             break;
    //             case libsumo::MOVE_TO_XY: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "MoveToXY person requires a compound object.");
    //                 if (parameterCount != 5 && parameterCount != 6) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, "MoveToXY person should obtain: edgeID, x, y, angle, keepRouteFlag and optionally matchThreshold.", outputStorage);
    //                 }
    //                 const std::string edgeID = StoHelp::readTypedString(inputStorage, "The first parameter for moveToXY must be the edge ID given as a string.");
    //                 const double x = StoHelp::readTypedDouble(inputStorage, "The second parameter for moveToXY must be the x-position given as a double.");
    //                 const double y = StoHelp::readTypedDouble(inputStorage, "The third parameter for moveToXY must be the y-position given as a double.");
    //                 const double angle = StoHelp::readTypedDouble(inputStorage, "The fourth parameter for moveToXY must be the angle given as a double.");
    //                 const int keepRouteFlag = StoHelp::readTypedByte(inputStorage, "The fifth parameter for moveToXY must be the keepRouteFlag given as a byte.");
    //                 double matchThreshold = 100.;
    //                 if (parameterCount == 6) {
    //                     matchThreshold = StoHelp::readTypedDouble(inputStorage, "The sixth parameter for moveToXY must be the matchThreshold given as a double.");
    //                 }
    //                 libsumo::Person::moveToXY(id, edgeID, x, y, angle, keepRouteFlag, matchThreshold);
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Person::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 try {
    //                     if (!TraCIServerAPI_VehicleType::setVariable(libsumo::CMD_SET_PERSON_VARIABLE, variable, p->getSingularType().getID(), server, inputStorage, outputStorage)) {
    //                         return false;
    //                     }
    //                 } catch (ProcessError& e) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, e.what(), outputStorage);
    //                 }
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_PERSON_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        use crate::libsumo::person::Person;
        use crate::libsumo::storage_helper::StorageHelper;
        use crate::libsumo::tra_ci_defs::TraCIStage;
        // C++ outer catch (TraCIException) is reproduced by dispatch_command; a ProcessError escapes in C++ too and is re-raised there (the default branch has its own catch below)
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().unwrap();
        if variable != VAR_PARAMETER
            && variable != ADD
            && variable != REMOVE
            && variable != APPEND_STAGE
            && variable != REPLACE_STAGE
            && variable != REMOVE_STAGE
            && variable != CMD_REROUTE_TRAVELTIME
            && variable != VAR_MOVE_TO
            && variable != MOVE_TO_XY
            && variable != VAR_SPEED
            && variable != VAR_TYPE
            && variable != VAR_SPEED_FACTOR
            && variable != VAR_LENGTH
            && variable != VAR_WIDTH
            && variable != VAR_HEIGHT
            && variable != VAR_MINGAP
            && variable != VAR_COLOR
        {
            return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, &format!("Change Person State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)), outputStorage);
        }
        // id
        let id = inputStorage.read_string().unwrap();
        let shouldExist = variable != ADD;
        let p: Ptr<MSTransportable> = MSNet::get_instance().m().get_person_control().get(&id);
        if p.is_null() && shouldExist {
            return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, &format!("Person '{}' is not known", id), outputStorage);
        }
        // process
        match variable {
            VAR_SPEED => {
                // set the speed for all present and future (walking) stages and modify the vType so that stages added later are also affected
                Person::set_speed(&id, StorageHelper::read_typed_double(inputStorage, "Setting speed requires a double."));
            }
            VAR_TYPE => {
                Person::set_type(&id, &StorageHelper::read_typed_string(inputStorage, "The vehicle type id must be given as a string."));
            }
            VAR_SPEED_FACTOR => {
                Person::set_speed_factor(&id, StorageHelper::read_typed_double(inputStorage, "Setting SpeedFactor requires a double."));
            }
            VAR_COLOR => {
                Person::set_color(&id, &StorageHelper::read_typed_color(inputStorage, "The color must be given using the according type."));
            }
            ADD => {
                StorageHelper::read_compound(inputStorage, 4, "Adding a person needs four parameters.");
                let vTypeID = StorageHelper::read_typed_string(inputStorage, "First parameter (type) requires a string.");
                let edgeID = StorageHelper::read_typed_string(inputStorage, "Second parameter (edge) requires a string.");
                let depart = StorageHelper::read_typed_double(inputStorage, "Third parameter (depart) requires a double.");
                let pos = StorageHelper::read_typed_double(inputStorage, "Fourth parameter (position) requires a double.");
                Person::add(&id, &edgeID, pos, depart, vTypeID);
            }
            REMOVE => {
                Person::remove(&id, StorageHelper::read_typed_byte(inputStorage, "Removing a person requires a byte.") as i8);
            }
            APPEND_STAGE => {
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "Adding a person stage requires a compound object.");
                if parameterCount == 13 {
                    let mut stage = TraCIStage::new(INVALID_INT_VALUE, "", "", "", &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, "", INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, "");
                    StorageHelper::read_stage(inputStorage, &mut stage, "");
                    Person::append_stage(&id, &stage);
                } else {
                    let stageType = StorageHelper::read_typed_int(inputStorage, "The first parameter for adding a stage must be the stage type given as int.");
                    if stageType == STAGE_DRIVING {
                        // append driving stage
                        if parameterCount != 4 {
                            return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, "Adding a driving stage needs four parameters.", outputStorage);
                        }
                        let edgeID = StorageHelper::read_typed_string(inputStorage, "Second parameter (edge) requires a string.");
                        let lines = StorageHelper::read_typed_string(inputStorage, "Third parameter (lines) requires a string.");
                        let stopID = StorageHelper::read_typed_string(inputStorage, "Fourth parameter (stopID) requires a string.");
                        Person::append_driving_stage(&id, &edgeID, &lines, &stopID);
                    } else if stageType == STAGE_WAITING {
                        // append waiting stage
                        if parameterCount != 4 {
                            return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, "Adding a waiting stage needs four parameters.", outputStorage);
                        }
                        let duration = StorageHelper::read_typed_double(inputStorage, "Second parameter (duration) requires a double.");
                        let description = StorageHelper::read_typed_string(inputStorage, "Third parameter (description) requires a string.");
                        let stopID = StorageHelper::read_typed_string(inputStorage, "Fourth parameter (stopID) requires a string.");
                        Person::append_waiting_stage(&id, duration, &description, &stopID);
                    } else if stageType == STAGE_WALKING {
                        // append walking stage
                        if parameterCount != 6 {
                            return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, "Adding a walking stage needs six parameters.", outputStorage);
                        }
                        let edgeIDs = StorageHelper::read_typed_string_list(inputStorage, "Second parameter (edges) route must be defined as a list of edge ids.");
                        let arrivalPos = StorageHelper::read_typed_double(inputStorage, "Third parameter (arrivalPos) requires a double.");
                        let duration = StorageHelper::read_typed_double(inputStorage, "Fourth parameter (duration) requires a double.");
                        let speed = StorageHelper::read_typed_double(inputStorage, "Fifth parameter (speed) requires a double.");
                        let stopID = StorageHelper::read_typed_string(inputStorage, "Sixth parameter (stopID) requires a string.");
                        Person::append_walking_stage(&id, &edgeIDs, arrivalPos, duration, speed, &stopID);
                    }
                }
            }
            REPLACE_STAGE => {
                StorageHelper::read_compound(inputStorage, 2, "Replacing a person stage requires a compound object of size 2.");
                let nextStageIndex = StorageHelper::read_typed_int(inputStorage, "First parameter of replace stage should be an integer");
                StorageHelper::read_compound(inputStorage, 13, "Second parameter of replace stage should be a compound object of size 13");
                let mut stage = TraCIStage::new(INVALID_INT_VALUE, "", "", "", &Vec::new(), INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, "", INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, INVALID_DOUBLE_VALUE, "");
                StorageHelper::read_stage(inputStorage, &mut stage, "");
                Person::replace_stage(&id, nextStageIndex, &stage);
            }
            REMOVE_STAGE => {
                Person::remove_stage(&id, StorageHelper::read_typed_int(inputStorage, "The message must contain the stage index."));
            }
            CMD_REROUTE_TRAVELTIME => {
                StorageHelper::read_compound(inputStorage, 0, "Resuming requires an empty compound object.");
                Person::reroute_traveltime(&id);
            }
            VAR_MOVE_TO => {
                StorageHelper::read_compound(inputStorage, 3, "Setting position should obtain the edge id, the position and the lateral position.");
                let laneID = StorageHelper::read_typed_string(inputStorage, "The first parameter for setting a position must be the laneID given as a string.");
                let position = StorageHelper::read_typed_double(inputStorage, "The second parameter for setting a position must be the position given as a double.");
                let posLat = StorageHelper::read_typed_double(inputStorage, "The third parameter for setting a position must be the lateral position given as a double.");
                Person::move_to(&id, &laneID, position, posLat);
            }
            MOVE_TO_XY => {
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "MoveToXY person requires a compound object.");
                if parameterCount != 5 && parameterCount != 6 {
                    return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, "MoveToXY person should obtain: edgeID, x, y, angle, keepRouteFlag and optionally matchThreshold.", outputStorage);
                }
                let edgeID = StorageHelper::read_typed_string(inputStorage, "The first parameter for moveToXY must be the edge ID given as a string.");
                let x = StorageHelper::read_typed_double(inputStorage, "The second parameter for moveToXY must be the x-position given as a double.");
                let y = StorageHelper::read_typed_double(inputStorage, "The third parameter for moveToXY must be the y-position given as a double.");
                let angle = StorageHelper::read_typed_double(inputStorage, "The fourth parameter for moveToXY must be the angle given as a double.");
                let keepRouteFlag = StorageHelper::read_typed_byte(inputStorage, "The fifth parameter for moveToXY must be the keepRouteFlag given as a byte.");
                let mut matchThreshold: f64 = 100.;
                if parameterCount == 6 {
                    matchThreshold = StorageHelper::read_typed_double(inputStorage, "The sixth parameter for moveToXY must be the matchThreshold given as a double.");
                }
                Person::move_to_xy(&id, &edgeID, x, y, angle, keepRouteFlag, matchThreshold);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Person::set_parameter(&id, &name, &value);
            }
            _ => {
                // C++ inner `catch (ProcessError&)` and outer `catch (TraCIException&)` both answer with writeErrorStatusCmd
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| TraCIServerAPI_VehicleType::set_variable(CMD_SET_PERSON_VARIABLE, variable, p.m().get_singular_type().get_id(), server, inputStorage, outputStorage))) {
                    Ok(true) => {}
                    Ok(false) => return false,
                    Err(e) => return server.write_error_status_cmd(CMD_SET_PERSON_VARIABLE, &crate::traci_server::tra_ci_server::panic_text(e), outputStorage),
                }
            }
        }
        server.write_status_cmd(CMD_SET_PERSON_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Person::TraCIServerAPI_Person(const TraCIServerAPI_Person &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Person) -> Self {
        panic!("TraCIServerAPI_Person copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Person & TraCIServerAPI_Person::operator=(const TraCIServerAPI_Person &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Person) -> &mut TraCIServerAPI_Person {
        panic!("TraCIServerAPI_Person assignment operator is private and undefined in C++; it cannot be called")
    }

}

