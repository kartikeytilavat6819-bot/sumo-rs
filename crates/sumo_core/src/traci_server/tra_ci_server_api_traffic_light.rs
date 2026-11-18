//! Port of traci-server/TraCIServerAPI_TrafficLight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting traffic light values via TraCI
/// C++ class `TraCIServerAPI_TrafficLight` (traci-server/TraCIServerAPI_TrafficLight.h)
pub struct TraCIServerAPI_TrafficLight {
}

impl TraCIServerAPI_TrafficLight {
    /// C++: `bool TraCIServerAPI_TrafficLight::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc2: Change Traffic Lights State)
    // bool
    // TraCIServerAPI_TrafficLight::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                         tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     const int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::TL_PHASE_INDEX && variable != libsumo::TL_PROGRAM && variable != libsumo::TL_PHASE_DURATION
    //             && variable != libsumo::TL_RED_YELLOW_GREEN_STATE && variable != libsumo::TL_COMPLETE_PROGRAM_RYG
    //             && variable != libsumo::VAR_NAME
    //             && variable != libsumo::TL_CONSTRAINT_REMOVE
    //             && variable != libsumo::TL_CONSTRAINT_UPDATE
    //             && variable != libsumo::TL_CONSTRAINT_ADD
    //             && variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_TL_VARIABLE, "Change TLS State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     const std::string id = inputStorage.readString();
    //     try {
    //         switch (variable) {
    //             case libsumo::TL_PHASE_INDEX: {
    //                 libsumo::TrafficLight::setPhase(id, StoHelp::readTypedInt(inputStorage, "The phase index must be given as an integer."));
    //             }
    //             break;
    //             case libsumo::VAR_NAME:
    //                 libsumo::TrafficLight::setPhaseName(id, StoHelp::readTypedString(inputStorage, "The phase name must be given as a string."));
    //                 break;
    //             case libsumo::TL_PROGRAM:
    //                 libsumo::TrafficLight::setProgram(id, StoHelp::readTypedString(inputStorage, "The program must be given as a string."));
    //                 break;
    //             case libsumo::TL_PHASE_DURATION:
    //                 libsumo::TrafficLight::setPhaseDuration(id, StoHelp::readTypedDouble(inputStorage, "The phase duration must be given as a double."));
    //                 break;
    //             case libsumo::TL_RED_YELLOW_GREEN_STATE:
    //                 libsumo::TrafficLight::setRedYellowGreenState(id, StoHelp::readTypedString(inputStorage, "The phase must be given as a string."));
    //                 break;
    //             case libsumo::TL_COMPLETE_PROGRAM_RYG: {
    //                 StoHelp::readCompound(inputStorage, -1, "A compound object is needed for setting a new program.");
    //                 libsumo::TraCILogic logic;
    //                 logic.programID = StoHelp::readTypedString(inputStorage, "set program: 1. parameter (programID) must be a string.");
    //                 logic.type = StoHelp::readTypedInt(inputStorage, "set program: 2. parameter (type) must be an int.");
    //                 logic.currentPhaseIndex = StoHelp::readTypedInt(inputStorage, "set program: 3. parameter (index) must be an int.");
    //
    //                 const int numPhases = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for the phases.");
    //                 for (int j = 0; j < numPhases; ++j) {
    //                     const int items = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for every phase.");
    //                     if (items != 6 && items != 5) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_TL_VARIABLE, "A phase compound object requires 5 or 6 items.", outputStorage);
    //                     }
    //                     const double duration = StoHelp::readTypedDouble(inputStorage, "set program: 4.1. parameter (duration) must be a double.");
    //                     const std::string state = StoHelp::readTypedString(inputStorage, "set program: 4.2. parameter (phase) must be a string.");
    //                     const double minDuration = StoHelp::readTypedDouble(inputStorage, "set program: 4.3. parameter (min duration) must be a double.");
    //                     const double maxDuration = StoHelp::readTypedDouble(inputStorage, "set program: 4.4. parameter (max duration) must be a double.");
    //                     const int numNext = StoHelp::readCompound(inputStorage, -1, "set program 4.5 parameter (next) must be a compound (list of ints).");
    //                     std::vector<int> next;
    //                     for (int k = 0; k < numNext; ++k) {
    //                         next.push_back(StoHelp::readTypedInt(inputStorage, "set program: 4.5. parameter (next) must be a list of int."));
    //                     }
    //                     std::string name;
    //                     if (items == 6) {
    //                         name = StoHelp::readTypedString(inputStorage, "set program: 4.6. parameter (name) must be a string.");
    //                     }
    //                     logic.phases.emplace_back(new libsumo::TraCIPhase(duration, state, minDuration, maxDuration, next, name));
    //                 }
    //                 const int numParams = StoHelp::readCompound(inputStorage, -1, "set program: 5. parameter (subparams) must be a compound object.");
    //                 for (int j = 0; j < numParams; ++j) {
    //                     const std::vector<std::string> par = StoHelp::readTypedStringList(inputStorage);
    //                     logic.subParameter[par[0]] = par[1];
    //                 }
    //                 libsumo::TrafficLight::setCompleteRedYellowGreenDefinition(id, logic);
    //             }
    //             break;
    //             case libsumo::TL_CONSTRAINT_REMOVE: {
    //                 StoHelp::readCompound(inputStorage, 3, "A compound object of size 3 is needed for removing constraints.");
    //                 const std::string tripId = StoHelp::readTypedString(inputStorage, "The tripId must be given as a string.");
    //                 const std::string foeSignal = StoHelp::readTypedString(inputStorage, "The foeSignal id must be given as a string.");
    //                 const std::string foeId = StoHelp::readTypedString(inputStorage, "The foe tripId must be given as a string.");
    //                 libsumo::TrafficLight::removeConstraints(id, tripId, foeSignal, foeId);
    //             }
    //             break;
    //             case libsumo::TL_CONSTRAINT_UPDATE:
    //                 libsumo::TrafficLight::updateConstraints(id, StoHelp::readTypedString(inputStorage, "The tripId index must be given as a string."));
    //                 break;
    //             case libsumo::TL_CONSTRAINT_ADD: {
    //                 StoHelp::readCompound(inputStorage, 5, "A compound object of size 5 is needed for adding constraints.");
    //                 const std::string tripId = StoHelp::readTypedString(inputStorage, "The tripId must be given as a string.");
    //                 const std::string foeSignal = StoHelp::readTypedString(inputStorage, "The foe signal must be given as a string.");
    //                 const std::string foeId = StoHelp::readTypedString(inputStorage, "The foe tripId must be given as a string.");
    //                 const int type = StoHelp::readTypedInt(inputStorage, "The type must be an int.");
    //                 const int limit = StoHelp::readTypedInt(inputStorage, "The limit must be an int.");
    //                 libsumo::TrafficLight::addConstraint(id, tripId, foeSignal, foeId, type, limit);
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::TrafficLight::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_TL_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_TL_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if variable != TL_PHASE_INDEX
            && variable != TL_PROGRAM
            && variable != TL_PHASE_DURATION
            && variable != TL_RED_YELLOW_GREEN_STATE
            && variable != TL_COMPLETE_PROGRAM_RYG
            && variable != VAR_NAME
            && variable != TL_CONSTRAINT_REMOVE
            && variable != TL_CONSTRAINT_UPDATE
            && variable != TL_CONSTRAINT_ADD
            && variable != VAR_PARAMETER
        {
            return server.write_error_status_cmd(
                CMD_SET_TL_VARIABLE,
                &format!("Change TLS State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");

        // C++ catch (writeErrorStatusCmd) is reproduced by TraCIServer::dispatch_command, which turns the TraCIException panic into an error status
        match variable {
            TL_PHASE_INDEX => {
                TrafficLight::set_phase(&id, StorageHelper::read_typed_int(inputStorage, "The phase index must be given as an integer."));
            }
            VAR_NAME => {
                TrafficLight::set_phase_name(&id, &StorageHelper::read_typed_string(inputStorage, "The phase name must be given as a string."));
            }
            TL_PROGRAM => {
                TrafficLight::set_program(&id, &StorageHelper::read_typed_string(inputStorage, "The program must be given as a string."));
            }
            TL_PHASE_DURATION => {
                TrafficLight::set_phase_duration(&id, StorageHelper::read_typed_double(inputStorage, "The phase duration must be given as a double."));
            }
            TL_RED_YELLOW_GREEN_STATE => {
                TrafficLight::set_red_yellow_green_state(&id, &StorageHelper::read_typed_string(inputStorage, "The phase must be given as a string."));
            }
            TL_COMPLETE_PROGRAM_RYG => {
                StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for setting a new program.");
                let mut logic = TraCILogic::new();
                logic.programID = StorageHelper::read_typed_string(inputStorage, "set program: 1. parameter (programID) must be a string.");
                logic.type_ = StorageHelper::read_typed_int(inputStorage, "set program: 2. parameter (type) must be an int.");
                logic.currentPhaseIndex = StorageHelper::read_typed_int(inputStorage, "set program: 3. parameter (index) must be an int.");

                let numPhases = StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for the phases.");
                for _j in 0..numPhases {
                    let items = StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for every phase.");
                    if items != 6 && items != 5 {
                        return server.write_error_status_cmd(CMD_SET_TL_VARIABLE, "A phase compound object requires 5 or 6 items.", outputStorage);
                    }
                    let duration = StorageHelper::read_typed_double(inputStorage, "set program: 4.1. parameter (duration) must be a double.");
                    let state = StorageHelper::read_typed_string(inputStorage, "set program: 4.2. parameter (phase) must be a string.");
                    let minDuration = StorageHelper::read_typed_double(inputStorage, "set program: 4.3. parameter (min duration) must be a double.");
                    let maxDuration = StorageHelper::read_typed_double(inputStorage, "set program: 4.4. parameter (max duration) must be a double.");
                    let numNext = StorageHelper::read_compound(inputStorage, -1, "set program 4.5 parameter (next) must be a compound (list of ints).");
                    let mut next: Vec<i32> = Vec::new();
                    for _k in 0..numNext {
                        next.push(StorageHelper::read_typed_int(inputStorage, "set program: 4.5. parameter (next) must be a list of int."));
                    }
                    let mut name = String::new();
                    if items == 6 {
                        name = StorageHelper::read_typed_string(inputStorage, "set program: 4.6. parameter (name) must be a string.");
                    }
                    logic.phases.push(Rc::new(TraCIPhase::new_2(duration, &state, minDuration, maxDuration, &next, &name, "")));
                }
                let numParams = StorageHelper::read_compound(inputStorage, -1, "set program: 5. parameter (subparams) must be a compound object.");
                for _j in 0..numParams {
                    let par = StorageHelper::read_typed_string_list(inputStorage, "");
                    logic.subParameter.insert(par[0].clone(), par[1].clone());
                }
                TrafficLight::set_complete_red_yellow_green_definition(&id, &logic);
            }
            TL_CONSTRAINT_REMOVE => {
                StorageHelper::read_compound(inputStorage, 3, "A compound object of size 3 is needed for removing constraints.");
                let tripId = StorageHelper::read_typed_string(inputStorage, "The tripId must be given as a string.");
                let foeSignal = StorageHelper::read_typed_string(inputStorage, "The foeSignal id must be given as a string.");
                let foeId = StorageHelper::read_typed_string(inputStorage, "The foe tripId must be given as a string.");
                TrafficLight::remove_constraints(&id, &tripId, &foeSignal, &foeId);
            }
            TL_CONSTRAINT_UPDATE => {
                TrafficLight::update_constraints(&id, StorageHelper::read_typed_string(inputStorage, "The tripId index must be given as a string."));
            }
            TL_CONSTRAINT_ADD => {
                StorageHelper::read_compound(inputStorage, 5, "A compound object of size 5 is needed for adding constraints.");
                let tripId = StorageHelper::read_typed_string(inputStorage, "The tripId must be given as a string.");
                let foeSignal = StorageHelper::read_typed_string(inputStorage, "The foe signal must be given as a string.");
                let foeId = StorageHelper::read_typed_string(inputStorage, "The foe tripId must be given as a string.");
                let type_ = StorageHelper::read_typed_int(inputStorage, "The type must be an int.");
                let limit = StorageHelper::read_typed_int(inputStorage, "The limit must be an int.");
                TrafficLight::add_constraint(&id, &tripId, &foeSignal, &foeId, type_, limit);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                TrafficLight::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_TL_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_TrafficLight::writeConstraint(TraCIServer &, const libsumo::TraCISignalConstraint &)` [static, private]
    // void
    // TraCIServerAPI_TrafficLight::writeConstraint(TraCIServer& server, const libsumo::TraCISignalConstraint& c) {
    //     StoHelp::writeTypedString(server.getWrapperStorage(), c.signalId);
    //     StoHelp::writeTypedString(server.getWrapperStorage(), c.tripId);
    //     StoHelp::writeTypedString(server.getWrapperStorage(), c.foeId);
    //     StoHelp::writeTypedString(server.getWrapperStorage(), c.foeSignal);
    //     StoHelp::writeTypedInt(server.getWrapperStorage(), c.limit);
    //     StoHelp::writeTypedInt(server.getWrapperStorage(), c.type);
    //     StoHelp::writeTypedByte(server.getWrapperStorage(), c.mustWait);
    //     StoHelp::writeTypedByte(server.getWrapperStorage(), c.active);
    //     std::vector<std::string> paramItems;
    //     for (auto item : c.param) {
    //         paramItems.push_back(item.first);
    //         paramItems.push_back(item.second);
    //     }
    //     StoHelp::writeTypedStringList(server.getWrapperStorage(), paramItems);
    // }
    #[allow(unused_variables)]
    pub fn write_constraint(server: &mut TraCIServer, c: &TraCISignalConstraint) {
        StorageHelper::write_typed_string(server.get_wrapper_storage(), &c.signalId);
        StorageHelper::write_typed_string(server.get_wrapper_storage(), &c.tripId);
        StorageHelper::write_typed_string(server.get_wrapper_storage(), &c.foeId);
        StorageHelper::write_typed_string(server.get_wrapper_storage(), &c.foeSignal);
        StorageHelper::write_typed_int(server.get_wrapper_storage(), c.limit);
        StorageHelper::write_typed_int(server.get_wrapper_storage(), c.type_);
        StorageHelper::write_typed_byte(server.get_wrapper_storage(), c.mustWait as i32);
        StorageHelper::write_typed_byte(server.get_wrapper_storage(), c.active as i32);
        let mut paramItems: Vec<String> = Vec::new();
        for (k, v) in c.param.iter() {
            paramItems.push(k.clone());
            paramItems.push(v.clone());
        }
        StorageHelper::write_typed_string_list(server.get_wrapper_storage(), &paramItems);
    }

    /// C++: `void TraCIServerAPI_TrafficLight::TraCIServerAPI_TrafficLight(const TraCIServerAPI_TrafficLight &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_TrafficLight) -> Self {
        panic!("TraCIServerAPI_TrafficLight copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_TrafficLight & TraCIServerAPI_TrafficLight::operator=(const TraCIServerAPI_TrafficLight &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_TrafficLight) -> &mut TraCIServerAPI_TrafficLight {
        panic!("TraCIServerAPI_TrafficLight assignment operator is private and undefined in C++; it cannot be called")
    }

}

