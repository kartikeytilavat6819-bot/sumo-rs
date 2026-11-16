//! Port of traci-server/TraCIServerAPI_Simulation.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ `throw libsumo::TraCIException(what)`
// shortcut: panics with the message (C++ throws libsumo::TraCIException); TraCIException::new is still todo!, upgrade when it is ported and TraCIServer::process_get catches exceptions instead of unwinding
fn throw_traci_exception(what: &str) -> ! {
    panic!("{}", what)
}

/// APIs for getting/setting simulation values via TraCI
/// C++ class `TraCIServerAPI_Simulation` (traci-server/TraCIServerAPI_Simulation.h)
pub struct TraCIServerAPI_Simulation {
}

impl TraCIServerAPI_Simulation {
    /// C++: `bool TraCIServerAPI_Simulation::processGet(TraCIServer &, tcpip::Storage &, const std::string &, const int)` [static]
    /// Processes a get value command (Command 0xab: Get Simulation Variable)
    // bool
    // TraCIServerAPI_Simulation::processGet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                       const std::string& objID, const int variable) {
    //     // unlike the other domains we cannot check here first whether libsumo::Simulation can handle it because the implementations for the state variables differ
    //     switch (variable) {
    //         case libsumo::VAR_LOADED_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::BUILT);
    //             break;
    //         case libsumo::VAR_LOADED_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::BUILT);
    //             break;
    //         case libsumo::VAR_DEPARTED_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::DEPARTED);
    //             break;
    //         case libsumo::VAR_DEPARTED_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::DEPARTED);
    //             break;
    //         case libsumo::VAR_TELEPORT_STARTING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_TELEPORT);
    //             break;
    //         case libsumo::VAR_TELEPORT_STARTING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_TELEPORT);
    //             break;
    //         case libsumo::VAR_TELEPORT_ENDING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_TELEPORT);
    //             break;
    //         case libsumo::VAR_TELEPORT_ENDING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_TELEPORT);
    //             break;
    //         case libsumo::VAR_ARRIVED_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::ARRIVED);
    //             break;
    //         case libsumo::VAR_ARRIVED_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::ARRIVED);
    //             break;
    //         case libsumo::VAR_DEPARTED_PERSONS_NUMBER:
    //             writeTransportableStateNumber(server, server.getWrapperStorage(), MSNet::TransportableState::PERSON_DEPARTED);
    //             break;
    //         case libsumo::VAR_DEPARTED_PERSONS_IDS:
    //             writeTransportableStateIDs(server, server.getWrapperStorage(), MSNet::TransportableState::PERSON_DEPARTED);
    //             break;
    //         case libsumo::VAR_ARRIVED_PERSONS_NUMBER:
    //             writeTransportableStateNumber(server, server.getWrapperStorage(), MSNet::TransportableState::PERSON_ARRIVED);
    //             break;
    //         case libsumo::VAR_ARRIVED_PERSONS_IDS:
    //             writeTransportableStateIDs(server, server.getWrapperStorage(), MSNet::TransportableState::PERSON_ARRIVED);
    //             break;
    //         case libsumo::VAR_PARKING_STARTING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_PARKING);
    //             break;
    //         case libsumo::VAR_PARKING_STARTING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_PARKING);
    //             break;
    //         case libsumo::VAR_PARKING_MANEUVERING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::MANEUVERING);
    //             break;
    //         case libsumo::VAR_PARKING_MANEUVERING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::MANEUVERING);
    //             break;
    //         case libsumo::VAR_PARKING_ENDING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_PARKING);
    //             break;
    //         case libsumo::VAR_PARKING_ENDING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_PARKING);
    //             break;
    //         case libsumo::VAR_STOP_STARTING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_STOP);
    //             break;
    //         case libsumo::VAR_STOP_STARTING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::STARTING_STOP);
    //             break;
    //         case libsumo::VAR_STOP_ENDING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_STOP);
    //             break;
    //         case libsumo::VAR_STOP_ENDING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::ENDING_STOP);
    //             break;
    //         case libsumo::VAR_COLLIDING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::COLLISION);
    //             break;
    //         case libsumo::VAR_COLLIDING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::COLLISION);
    //             break;
    //         case libsumo::VAR_EMERGENCYSTOPPING_VEHICLES_NUMBER:
    //             writeVehicleStateNumber(server, server.getWrapperStorage(), MSNet::VehicleState::EMERGENCYSTOP);
    //             break;
    //         case libsumo::VAR_EMERGENCYSTOPPING_VEHICLES_IDS:
    //             writeVehicleStateIDs(server, server.getWrapperStorage(), MSNet::VehicleState::EMERGENCYSTOP);
    //             break;
    //         case libsumo::VAR_COLLISIONS: {
    //             std::vector<libsumo::TraCICollision> collisions = libsumo::Simulation::getCollisions();
    //             server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_COMPOUND);
    //             const int cnt = 1 + (int)collisions.size() * 4;
    //             server.getWrapperStorage().writeInt(cnt);
    //             server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_INTEGER);
    //             server.getWrapperStorage().writeInt((int)collisions.size());
    //             for (const auto& c : collisions) {
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.collider);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.victim);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.colliderType);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.victimType);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_DOUBLE);
    //                 server.getWrapperStorage().writeDouble(c.colliderSpeed);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_DOUBLE);
    //                 server.getWrapperStorage().writeDouble(c.victimSpeed);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.type);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_STRING);
    //                 server.getWrapperStorage().writeString(c.lane);
    //                 server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_DOUBLE);
    //                 server.getWrapperStorage().writeDouble(c.pos);
    //             }
    //             break;
    //         }
    //         case libsumo::VAR_NET_BOUNDING_BOX: {
    //             server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_POLYGON);
    //             libsumo::TraCIPositionVector tb = libsumo::Simulation::getNetBoundary();
    //             server.getWrapperStorage().writeByte(2);
    //             server.getWrapperStorage().writeDouble(tb.value[0].x);
    //             server.getWrapperStorage().writeDouble(tb.value[0].y);
    //             server.getWrapperStorage().writeDouble(tb.value[1].x);
    //             server.getWrapperStorage().writeDouble(tb.value[1].y);
    //             break;
    //         }
    //         case libsumo::POSITION_CONVERSION: {
    //             if (inputStorage.readUnsignedByte() != libsumo::TYPE_COMPOUND) {
    //                 throw libsumo::TraCIException("Position conversion requires a compound object.");
    //             }
    //             const int compoundSize = inputStorage.readInt();
    //             if (compoundSize < 2 || compoundSize > 3) {
    //                 throw libsumo::TraCIException("Position conversion requires a source position and a position type as parameter.");
    //             }
    //             commandPositionConversion(inputStorage, compoundSize, server.getWrapperStorage());
    //             break;
    //         }
    //         case libsumo::DISTANCE_REQUEST:
    //             if (inputStorage.readUnsignedByte() != libsumo::TYPE_COMPOUND) {
    //                 throw libsumo::TraCIException("Retrieval of distance requires a compound object.");
    //             }
    //             if (inputStorage.readInt() != 3) {
    //                 throw libsumo::TraCIException("Retrieval of distance requires two positions and a distance type as parameter.");
    //             }
    //             commandDistanceRequest(inputStorage, server.getWrapperStorage());
    //             break;
    //         case libsumo::FIND_ROUTE: {
    //             const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Retrieval of a route requires a compound object.");
    //             if (parameterCount < 5 || parameterCount > 7) {
    //                 throw libsumo::TraCIException("Retrieval of a route requires between five to seven parameters.");
    //             }
    //             const std::string from = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as first parameter.");
    //             const std::string to = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as second parameter.");
    //             const std::string vtype = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as third parameter.");
    //             const double depart = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as fourth parameter.");
    //             const int routingMode = StoHelp::readTypedInt(inputStorage, "Retrieval of a route requires an integer as fifth parameter.");
    //             double departPos = 0.;
    //             if (parameterCount > 5) {
    //                 departPos = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as sixth parameter.");
    //             }
    //             double arrivalPos = libsumo::INVALID_DOUBLE_VALUE;
    //             if (parameterCount > 6) {
    //                 arrivalPos = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as seventh parameter.");
    //             }
    //             StoHelp::writeStage(server.getWrapperStorage(), libsumo::Simulation::findRoute(from, to, vtype, depart, routingMode, departPos, arrivalPos));
    //             break;
    //         }
    //         case libsumo::FIND_INTERMODAL_ROUTE: {
    //             StoHelp::readCompound(inputStorage, 13, "Retrieval of an intermodal route requires thirteen parameters.");
    //             const std::string from = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as first parameter.");
    //             const std::string to = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as second parameter.");
    //             const std::string modes = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as third parameter.");
    //             const double depart = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as fourth parameter.");
    //             const int routingMode = StoHelp::readTypedInt(inputStorage, "Retrieval of a route requires an integer as fifth parameter.");
    //             const double speed = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as sixth parameter.");
    //             const double walkFactor = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as seventh parameter.");
    //             const double departPos = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as eighth parameter.");
    //             const double arrivalPos = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as ninth parameter.");
    //             const double departPosLat = StoHelp::readTypedDouble(inputStorage, "Retrieval of a route requires a double as tenth parameter.");
    //             const std::string ptype = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as eleventh parameter.");
    //             const std::string vtype = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as twelfth parameter.");
    //             const std::string destStop = StoHelp::readTypedString(inputStorage, "Retrieval of a route requires a string as thirteenth parameter.");
    //             const std::vector<libsumo::TraCIStage>& result = libsumo::Simulation::findIntermodalRoute(from, to, modes, depart, routingMode, speed, walkFactor, departPos, arrivalPos, departPosLat, ptype, vtype, destStop);
    //             server.getWrapperStorage().writeUnsignedByte(libsumo::TYPE_COMPOUND);
    //             server.getWrapperStorage().writeInt((int)result.size());
    //             for (const libsumo::TraCIStage& s : result) {
    //                 StoHelp::writeStage(server.getWrapperStorage(), s);
    //             }
    //             break;
    //         }
    //         default:
    //             if (!libsumo::Simulation::handleVariable(objID, variable, &server, &inputStorage)) {
    //                 throw libsumo::TraCIException("Get Simulation Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //             }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_get(server: &mut TraCIServer, inputStorage: &mut Storage, objID: &str, variable: i32) -> bool {
        match variable {
            VAR_LOADED_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::BUILT);
            }
            VAR_LOADED_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::BUILT);
            }
            VAR_DEPARTED_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::DEPARTED);
            }
            VAR_DEPARTED_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::DEPARTED);
            }
            VAR_TELEPORT_STARTING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::STARTING_TELEPORT);
            }
            VAR_TELEPORT_STARTING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::STARTING_TELEPORT);
            }
            VAR_TELEPORT_ENDING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::ENDING_TELEPORT);
            }
            VAR_TELEPORT_ENDING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::ENDING_TELEPORT);
            }
            VAR_ARRIVED_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::ARRIVED);
            }
            VAR_ARRIVED_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::ARRIVED);
            }
            VAR_PARKING_STARTING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::STARTING_PARKING);
            }
            VAR_PARKING_STARTING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::STARTING_PARKING);
            }
            VAR_PARKING_MANEUVERING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::MANEUVERING);
            }
            VAR_PARKING_MANEUVERING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::MANEUVERING);
            }
            VAR_PARKING_ENDING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::ENDING_PARKING);
            }
            VAR_PARKING_ENDING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::ENDING_PARKING);
            }
            VAR_STOP_STARTING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::STARTING_STOP);
            }
            VAR_STOP_STARTING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::STARTING_STOP);
            }
            VAR_STOP_ENDING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::ENDING_STOP);
            }
            VAR_STOP_ENDING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::ENDING_STOP);
            }
            VAR_COLLIDING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::COLLISION);
            }
            VAR_COLLIDING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::COLLISION);
            }
            VAR_EMERGENCYSTOPPING_VEHICLES_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_number(server, ws.m(), MSNet_VehicleState::EMERGENCYSTOP);
            }
            VAR_EMERGENCYSTOPPING_VEHICLES_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_vehicle_state_i_ds(server, ws.m(), MSNet_VehicleState::EMERGENCYSTOP);
            }
            VAR_DEPARTED_PERSONS_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_transportable_state_number(server, ws.m(), MSNet_TransportableState::PERSON_DEPARTED);
            }
            VAR_DEPARTED_PERSONS_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_transportable_state_i_ds(server, ws.m(), MSNet_TransportableState::PERSON_DEPARTED);
            }
            VAR_ARRIVED_PERSONS_NUMBER => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_transportable_state_number(server, ws.m(), MSNet_TransportableState::PERSON_ARRIVED);
            }
            VAR_ARRIVED_PERSONS_IDS => {
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::write_transportable_state_i_ds(server, ws.m(), MSNet_TransportableState::PERSON_ARRIVED);
            }
            VAR_COLLISIONS => {
                let collisions = Simulation::get_collisions();
                let ws = server.get_wrapper_storage();
                ws.write_unsigned_byte(TYPE_COMPOUND).unwrap();
                let cnt = 1 + (collisions.len() as i32) * 4;
                ws.write_int(cnt);
                ws.write_unsigned_byte(TYPE_INTEGER).unwrap();
                ws.write_int(collisions.len() as i32);
                for c in &collisions {
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.collider);
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.victim);
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.colliderType);
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.victimType);
                    ws.write_unsigned_byte(TYPE_DOUBLE).unwrap();
                    ws.write_double(c.colliderSpeed);
                    ws.write_unsigned_byte(TYPE_DOUBLE).unwrap();
                    ws.write_double(c.victimSpeed);
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.type_);
                    ws.write_unsigned_byte(TYPE_STRING).unwrap();
                    ws.write_string(&c.lane);
                    ws.write_unsigned_byte(TYPE_DOUBLE).unwrap();
                    ws.write_double(c.pos);
                }
            }
            VAR_NET_BOUNDING_BOX => {
                let tb = Simulation::get_net_boundary();
                let ws = server.get_wrapper_storage();
                ws.write_unsigned_byte(TYPE_POLYGON).unwrap();
                ws.write_byte(2).unwrap();
                ws.write_double(tb.value[0].x);
                ws.write_double(tb.value[0].y);
                ws.write_double(tb.value[1].x);
                ws.write_double(tb.value[1].y);
            }
            POSITION_CONVERSION => {
                if inputStorage.read_unsigned_byte().unwrap() != TYPE_COMPOUND {
                    throw_traci_exception("Position conversion requires a compound object.");
                }
                let compoundSize = inputStorage.read_int().unwrap();
                if compoundSize < 2 || compoundSize > 3 {
                    throw_traci_exception("Position conversion requires a source position and a position type as parameter.");
                }
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::command_position_conversion(inputStorage, compoundSize, ws.m());
            }
            DISTANCE_REQUEST => {
                if inputStorage.read_unsigned_byte().unwrap() != TYPE_COMPOUND {
                    throw_traci_exception("Retrieval of distance requires a compound object.");
                }
                if inputStorage.read_int().unwrap() != 3 {
                    throw_traci_exception("Retrieval of distance requires two positions and a distance type as parameter.");
                }
                let ws = Ptr::from_mut(server.get_wrapper_storage());
                Self::command_distance_request(inputStorage, ws.m());
            }
            FIND_ROUTE => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "Retrieval of a route requires a compound object.");
                if parameterCount < 5 || parameterCount > 7 {
                    throw_traci_exception("Retrieval of a route requires between five to seven parameters.");
                }
                let from = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as first parameter.");
                let to = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as second parameter.");
                let vtype = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as third parameter.");
                let depart = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as fourth parameter.");
                let routingMode = StoHelp::read_typed_int(inputStorage, "Retrieval of a route requires an integer as fifth parameter.");
                let mut departPos = 0.;
                if parameterCount > 5 {
                    departPos = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as sixth parameter.");
                }
                let mut arrivalPos = INVALID_DOUBLE_VALUE;
                if parameterCount > 6 {
                    arrivalPos = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as seventh parameter.");
                }
                let stage = Simulation::find_route(&from, &to, &vtype, depart, routingMode, departPos, arrivalPos);
                StoHelp::write_stage(server.get_wrapper_storage(), &stage);
            }
            FIND_INTERMODAL_ROUTE => {
                StoHelp::read_compound(inputStorage, 13, "Retrieval of an intermodal route requires thirteen parameters.");
                let from = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as first parameter.");
                let to = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as second parameter.");
                let modes = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as third parameter.");
                let depart = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as fourth parameter.");
                let routingMode = StoHelp::read_typed_int(inputStorage, "Retrieval of a route requires an integer as fifth parameter.");
                let speed = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as sixth parameter.");
                let walkFactor = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as seventh parameter.");
                let departPos = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as eighth parameter.");
                let arrivalPos = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as ninth parameter.");
                let departPosLat = StoHelp::read_typed_double(inputStorage, "Retrieval of a route requires a double as tenth parameter.");
                let ptype = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as eleventh parameter.");
                let vtype = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as twelfth parameter.");
                let destStop = StoHelp::read_typed_string(inputStorage, "Retrieval of a route requires a string as thirteenth parameter.");
                let result = Simulation::find_intermodal_route(&from, &to, &modes, depart, routingMode, speed, walkFactor, departPos, arrivalPos, departPosLat, &ptype, &vtype, &destStop);
                let ws = server.get_wrapper_storage();
                ws.write_unsigned_byte(TYPE_COMPOUND).unwrap();
                ws.write_int(result.len() as i32);
                for s in &result {
                    StoHelp::write_stage(ws, s);
                }
            }
            _ => {
                if !Simulation::handle_variable(objID, variable, Ptr::from_mut(server), Ptr::from_mut(inputStorage)) {
                    throw_traci_exception(&format!("Get Simulation Variable: unsupported variable {} specified", to_hex(variable as u64, 2, 0)));
                }
            }
        }
        true
    }

    /// C++: `bool TraCIServerAPI_Simulation::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xcb: Set Simulation Variable)
    // bool
    // TraCIServerAPI_Simulation::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                       tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::CMD_CLEAR_PENDING_VEHICLES
    //             && variable != libsumo::CMD_SAVE_SIMSTATE
    //             && variable != libsumo::CMD_LOAD_SIMSTATE
    //             && variable != libsumo::VAR_PARAMETER
    //             && variable != libsumo::VAR_SCALE
    //             && variable != libsumo::CMD_MESSAGE
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_SIM_VARIABLE, "Set Simulation Variable: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     // process
    //     try {
    //         switch (variable) {
    //             case libsumo::VAR_SCALE: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "A double is needed for setting traffic scale.");
    //                 if (value < 0.0) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_SIM_VARIABLE, "Traffic scale may not be negative.", outputStorage);
    //                 }
    //                 libsumo::Simulation::setScale(value);
    //             }
    //             break;
    //             case libsumo::CMD_CLEAR_PENDING_VEHICLES:
    //                 //clear any pending vehicle insertions
    //                 libsumo::Simulation::clearPending(StoHelp::readTypedString(inputStorage, "A string is needed for clearing pending vehicles."));
    //                 break;
    //             case libsumo::CMD_SAVE_SIMSTATE:
    //                 //save current simulation state
    //                 libsumo::Simulation::saveState(StoHelp::readTypedString(inputStorage, "A string is needed for saving simulation state."));
    //                 break;
    //             case libsumo::CMD_LOAD_SIMSTATE: {
    //                 //quick-load simulation state
    //                 const double time = libsumo::Simulation::loadState(StoHelp::readTypedString(inputStorage, "A string is needed for loading simulation state."));
    //                 TraCIServer::getInstance()->stateLoaded(TIME2STEPS(time));
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Simulation::setParameter(id, name, value);
    //                 break;
    //             }
    //             case libsumo::CMD_MESSAGE:
    //                 libsumo::Simulation::writeMessage(StoHelp::readTypedString(inputStorage, "A string is needed for adding a log message."));
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_SIM_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_SIM_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = ""; // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().unwrap();
        if variable != CMD_CLEAR_PENDING_VEHICLES
            && variable != CMD_SAVE_SIMSTATE
            && variable != CMD_LOAD_SIMSTATE
            && variable != VAR_PARAMETER
            && variable != VAR_SCALE
            && variable != CMD_MESSAGE
        {
            return server.write_error_status_cmd(CMD_SET_SIM_VARIABLE, &format!("Set Simulation Variable: unsupported variable {} specified", to_hex(variable as u64, 2, 0)), outputStorage);
        }
        // id
        let id = inputStorage.read_string().unwrap();
        // process
        // C++ try/catch of libsumo::TraCIException: a thrown exception unwinds to catch_unwind below
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<bool> {
            match variable {
                VAR_SCALE => {
                    let value = StoHelp::read_typed_double(inputStorage, "A double is needed for setting traffic scale.");
                    if value < 0.0 {
                        return Some(server.write_error_status_cmd(CMD_SET_SIM_VARIABLE, "Traffic scale may not be negative.", outputStorage));
                    }
                    Simulation::set_scale(value);
                }
                CMD_CLEAR_PENDING_VEHICLES => {
                    //clear any pending vehicle insertions
                    Simulation::clear_pending(&StoHelp::read_typed_string(inputStorage, "A string is needed for clearing pending vehicles."));
                }
                CMD_SAVE_SIMSTATE => {
                    //save current simulation state
                    Simulation::save_state(&StoHelp::read_typed_string(inputStorage, "A string is needed for saving simulation state."));
                }
                CMD_LOAD_SIMSTATE => {
                    //quick-load simulation state
                    let time = Simulation::load_state(&StoHelp::read_typed_string(inputStorage, "A string is needed for loading simulation state."));
                    TraCIServer::get_instance().m().state_loaded(time2steps(time));
                }
                VAR_PARAMETER => {
                    StoHelp::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                    let name = StoHelp::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                    let value = StoHelp::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                    Simulation::set_parameter(&id, &name, &value);
                }
                CMD_MESSAGE => {
                    Simulation::write_message(&StoHelp::read_typed_string(inputStorage, "A string is needed for adding a log message."));
                }
                _ => {}
            }
            None
        }));
        match outcome {
            Ok(Some(ret)) => return ret,
            Ok(None) => {}
            Err(payload) => {
                // the TraCIException message is the panic message
                let what = match payload.downcast::<String>() {
                    Ok(s) => *s,
                    Err(payload) => match payload.downcast::<&str>() {
                        Ok(s) => s.to_string(),
                        Err(payload) => std::panic::resume_unwind(payload),
                    },
                };
                return server.write_error_status_cmd(CMD_SET_SIM_VARIABLE, &what, outputStorage);
            }
        }
        server.write_status_cmd(CMD_SET_SIM_VARIABLE, RTYPE_OK, warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Simulation::commandPositionConversion(tcpip::Storage &, const int, tcpip::Storage &)` [static, private]
    // void
    // TraCIServerAPI_Simulation::commandPositionConversion(tcpip::Storage& inputStorage,
    //         const int compoundSize, tcpip::Storage& outputStorage) {
    //     std::pair<MSLane*, double> roadPos;
    //     Position cartesianPos;
    //     Position geoPos;
    //     double z = 0;
    //
    //     // actual position type that will be converted
    //     int srcPosType = inputStorage.readUnsignedByte();
    //
    //     switch (srcPosType) {
    //         case libsumo::POSITION_2D:
    //         case libsumo::POSITION_3D:
    //         case libsumo::POSITION_LON_LAT:
    //         case libsumo::POSITION_LON_LAT_ALT: {
    //             const double x = inputStorage.readDouble();
    //             const double y = inputStorage.readDouble();
    //             if (srcPosType != libsumo::POSITION_2D && srcPosType != libsumo::POSITION_LON_LAT) {
    //                 z = inputStorage.readDouble();
    //             }
    //             geoPos.set(x, y);
    //             cartesianPos.set(x, y);
    //             if (srcPosType == libsumo::POSITION_LON_LAT || srcPosType == libsumo::POSITION_LON_LAT_ALT) {
    //                 GeoConvHelper::getFinal().x2cartesian_const(cartesianPos);
    //             } else {
    //                 GeoConvHelper::getFinal().cartesian2geo(geoPos);
    //             }
    //         }
    //         break;
    //         case libsumo::POSITION_ROADMAP: {
    //             const std::string roadID = inputStorage.readString();
    //             const double pos = inputStorage.readDouble();
    //             const int laneIdx = inputStorage.readUnsignedByte();
    //             // convert edge,offset,laneIdx to cartesian position
    //             cartesianPos = geoPos = libsumo::Helper::getLaneChecking(roadID, laneIdx, pos)->geometryPositionAtOffset(pos);
    //             z = cartesianPos.z();
    //             GeoConvHelper::getFinal().cartesian2geo(geoPos);
    //         }
    //         break;
    //         default:
    //             throw libsumo::TraCIException("Source position type not supported");
    //     }
    //
    //     const int destPosType = StoHelp::readTypedUnsignedByte(inputStorage, "Destination position type must be of type ubyte.");
    //     SUMOVehicleClass vClass = SVC_IGNORING;
    //     if (compoundSize == 3) {
    //         inputStorage.readUnsignedByte();
    //         const std::string& vClassString = inputStorage.readString();
    //         if (!SumoVehicleClassStrings.hasString(vClassString)) {
    //             throw libsumo::TraCIException("Unknown vehicle class '" + vClassString + "'.");
    //         }
    //         vClass = SumoVehicleClassStrings.get(vClassString);
    //     }
    //
    //     switch (destPosType) {
    //         case libsumo::POSITION_ROADMAP: {
    //             // convert cartesion position to edge,offset,lane_index
    //             roadPos = libsumo::Helper::convertCartesianToRoadMap(cartesianPos, vClass);
    //             if (roadPos.first == nullptr) {
    //                 throw libsumo::TraCIException("No matching lane found.");
    //             }
    //             // write result that is added to response msg
    //             outputStorage.writeUnsignedByte(libsumo::POSITION_ROADMAP);
    //             outputStorage.writeString(roadPos.first->getEdge().getID());
    //             outputStorage.writeDouble(roadPos.second);
    //             outputStorage.writeUnsignedByte(roadPos.first->getIndex());
    //         }
    //         break;
    //         case libsumo::POSITION_2D:
    //         case libsumo::POSITION_3D:
    //         case libsumo::POSITION_LON_LAT:
    //         case libsumo::POSITION_LON_LAT_ALT:
    //             outputStorage.writeUnsignedByte(destPosType);
    //             if (destPosType == libsumo::POSITION_LON_LAT || destPosType == libsumo::POSITION_LON_LAT_ALT) {
    //                 outputStorage.writeDouble(geoPos.x());
    //                 outputStorage.writeDouble(geoPos.y());
    //             } else {
    //                 outputStorage.writeDouble(cartesianPos.x());
    //                 outputStorage.writeDouble(cartesianPos.y());
    //             }
    //             if (destPosType != libsumo::POSITION_2D && destPosType != libsumo::POSITION_LON_LAT) {
    //                 outputStorage.writeDouble(z);
    //             }
    //             break;
    //         default:
    //             throw libsumo::TraCIException("Destination position type not supported");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn command_position_conversion(inputStorage: &mut Storage, compoundSize: i32, outputStorage: &mut Storage) {
        // actual position type that will be converted
        let mut cartesianPos = Position::default();
        let mut geoPos = Position::default();
        let mut z: f64 = 0.;

        let srcPosType = inputStorage.read_unsigned_byte().unwrap();

        match srcPosType {
            POSITION_2D | POSITION_3D | POSITION_LON_LAT | POSITION_LON_LAT_ALT => {
                let x = inputStorage.read_double().unwrap();
                let y = inputStorage.read_double().unwrap();
                if srcPosType != POSITION_2D && srcPosType != POSITION_LON_LAT {
                    z = inputStorage.read_double().unwrap();
                }
                geoPos.set(x, y);
                cartesianPos.set(x, y);
                if srcPosType == POSITION_LON_LAT || srcPosType == POSITION_LON_LAT_ALT {
                    GeoConvHelper::with_final(|g| g.x2cartesian_const(&mut cartesianPos));
                } else {
                    GeoConvHelper::with_final(|g| g.cartesian2geo(&mut geoPos));
                }
            }
            POSITION_ROADMAP => {
                let roadID = inputStorage.read_string().unwrap();
                let pos = inputStorage.read_double().unwrap();
                let laneIdx = inputStorage.read_unsigned_byte().unwrap();
                // convert edge,offset,laneIdx to cartesian position
                cartesianPos = Helper::get_lane_checking(&roadID, laneIdx, pos).m().geometry_position_at_offset(pos, 0.0);
                geoPos = cartesianPos;
                z = cartesianPos.z();
                GeoConvHelper::with_final(|g| g.cartesian2geo(&mut geoPos));
            }
            _ => throw_traci_exception("Source position type not supported"),
        }

        let destPosType = StoHelp::read_typed_unsigned_byte(inputStorage, "Destination position type must be of type ubyte.");
        let mut vClass: SUMOVehicleClass = SVC_IGNORING;
        if compoundSize == 3 {
            inputStorage.read_unsigned_byte().unwrap();
            let vClassString = inputStorage.read_string().unwrap();
            if !SUMO_VEHICLE_CLASS_STRINGS.has_string(&vClassString) {
                throw_traci_exception(&format!("Unknown vehicle class '{}'.", vClassString));
            }
            vClass = SUMO_VEHICLE_CLASS_STRINGS.get(&vClassString).unwrap();
        }

        match destPosType {
            POSITION_ROADMAP => {
                // convert cartesion position to edge,offset,lane_index
                let roadPos = Helper::convert_cartesian_to_road_map(&cartesianPos, vClass);
                if roadPos.0.is_null() {
                    throw_traci_exception("No matching lane found.");
                }
                // write result that is added to response msg
                outputStorage.write_unsigned_byte(POSITION_ROADMAP).unwrap();
                let edgeID = roadPos.0.m().get_edge().get_id();
                outputStorage.write_string(&edgeID);
                outputStorage.write_double(roadPos.1);
                outputStorage.write_unsigned_byte(roadPos.0.m().get_index()).unwrap();
            }
            POSITION_2D | POSITION_3D | POSITION_LON_LAT | POSITION_LON_LAT_ALT => {
                outputStorage.write_unsigned_byte(destPosType).unwrap();
                if destPosType == POSITION_LON_LAT || destPosType == POSITION_LON_LAT_ALT {
                    outputStorage.write_double(geoPos.x());
                    outputStorage.write_double(geoPos.y());
                } else {
                    outputStorage.write_double(cartesianPos.x());
                    outputStorage.write_double(cartesianPos.y());
                }
                if destPosType != POSITION_2D && destPosType != POSITION_LON_LAT {
                    outputStorage.write_double(z);
                }
            }
            _ => throw_traci_exception("Destination position type not supported"),
        }
    }

    /// C++: `void TraCIServerAPI_Simulation::commandDistanceRequest(tcpip::Storage &, tcpip::Storage &)` [static, private]
    // void
    // TraCIServerAPI_Simulation::commandDistanceRequest(tcpip::Storage& inputStorage, tcpip::Storage& outputStorage) {
    //     Position pos1;
    //     Position pos2;
    //     std::pair<const MSLane*, double> roadPos1;
    //     std::pair<const MSLane*, double> roadPos2;
    //
    //     // read position 1
    //     int posType = inputStorage.readUnsignedByte();
    //     switch (posType) {
    //         case libsumo::POSITION_ROADMAP: {
    //             std::string roadID = inputStorage.readString();
    //             roadPos1.second = inputStorage.readDouble();
    //             roadPos1.first = libsumo::Helper::getLaneChecking(roadID, inputStorage.readUnsignedByte(), roadPos1.second);
    //             pos1 = roadPos1.first->geometryPositionAtOffset(roadPos1.second);
    //             break;
    //         }
    //         case libsumo::POSITION_2D:
    //         case libsumo::POSITION_3D: {
    //             double p1x = inputStorage.readDouble();
    //             double p1y = inputStorage.readDouble();
    //             pos1.set(p1x, p1y);
    //         }
    //         if (posType == libsumo::POSITION_3D) {
    //             inputStorage.readDouble();// z value is ignored
    //         }
    //         roadPos1 = libsumo::Helper::convertCartesianToRoadMap(pos1, SVC_IGNORING);
    //         break;
    //         case libsumo::POSITION_LON_LAT:
    //         case libsumo::POSITION_LON_LAT_ALT: {
    //             double p1x = inputStorage.readDouble();
    //             double p1y = inputStorage.readDouble();
    //             pos1.set(p1x, p1y);
    //             GeoConvHelper::getFinal().x2cartesian_const(pos1);
    //         }
    //         if (posType == libsumo::POSITION_LON_LAT_ALT) {
    //             inputStorage.readDouble();// altitude value is ignored
    //         }
    //         roadPos1 = libsumo::Helper::convertCartesianToRoadMap(pos1, SVC_IGNORING);
    //         break;
    //         default:
    //             throw libsumo::TraCIException("Unknown position format used for distance request");
    //     }
    //
    //     // read position 2
    //     posType = inputStorage.readUnsignedByte();
    //     switch (posType) {
    //         case libsumo::POSITION_ROADMAP: {
    //             std::string roadID = inputStorage.readString();
    //             roadPos2.second = inputStorage.readDouble();
    //             roadPos2.first = libsumo::Helper::getLaneChecking(roadID, inputStorage.readUnsignedByte(), roadPos2.second);
    //             pos2 = roadPos2.first->geometryPositionAtOffset(roadPos2.second);
    //             break;
    //         }
    //         case libsumo::POSITION_2D:
    //         case libsumo::POSITION_3D: {
    //             double p2x = inputStorage.readDouble();
    //             double p2y = inputStorage.readDouble();
    //             pos2.set(p2x, p2y);
    //         }
    //         if (posType == libsumo::POSITION_3D) {
    //             inputStorage.readDouble();// z value is ignored
    //         }
    //         roadPos2 = libsumo::Helper::convertCartesianToRoadMap(pos2, SVC_IGNORING);
    //         break;
    //         case libsumo::POSITION_LON_LAT:
    //         case libsumo::POSITION_LON_LAT_ALT: {
    //             double p2x = inputStorage.readDouble();
    //             double p2y = inputStorage.readDouble();
    //             pos2.set(p2x, p2y);
    //             GeoConvHelper::getFinal().x2cartesian_const(pos2);
    //         }
    //         if (posType == libsumo::POSITION_LON_LAT_ALT) {
    //             inputStorage.readDouble();// altitude value is ignored
    //         }
    //         roadPos2 = libsumo::Helper::convertCartesianToRoadMap(pos2, SVC_IGNORING);
    //         break;
    //         default:
    //             throw libsumo::TraCIException("Unknown position format used for distance request");
    //     }
    //
    //     // read distance type
    //     const int distType = inputStorage.readUnsignedByte();
    //
    //     double distance = 0.0;
    //     if (distType == libsumo::REQUEST_DRIVINGDIST) {
    //         distance = libsumo::Helper::getDrivingDistance(roadPos1, roadPos2);
    //     } else {
    //         // compute air distance (default)
    //         distance = pos1.distanceTo(pos2);
    //     }
    //     // write response command
    //     StoHelp::writeTypedDouble(outputStorage, distance);
    // }
    #[allow(unused_variables)]
    pub fn command_distance_request(inputStorage: &mut Storage, outputStorage: &mut Storage) {
        let mut pos1 = Position::default();
        let mut pos2 = Position::default();
        let mut roadPos1: (Ptr<MSLane>, f64) = (Ptr::null(), 0.0);
        let mut roadPos2: (Ptr<MSLane>, f64) = (Ptr::null(), 0.0);

        // read position 1
        let mut posType = inputStorage.read_unsigned_byte().unwrap();
        match posType {
            POSITION_ROADMAP => {
                let roadID = inputStorage.read_string().unwrap();
                roadPos1.1 = inputStorage.read_double().unwrap();
                let laneIdx = inputStorage.read_unsigned_byte().unwrap();
                roadPos1.0 = Helper::get_lane_checking(&roadID, laneIdx, roadPos1.1);
                pos1 = roadPos1.0.m().geometry_position_at_offset(roadPos1.1, 0.0);
            }
            POSITION_2D | POSITION_3D => {
                let p1x = inputStorage.read_double().unwrap();
                let p1y = inputStorage.read_double().unwrap();
                pos1.set(p1x, p1y);
                if posType == POSITION_3D {
                    inputStorage.read_double().unwrap(); // z value is ignored
                }
                roadPos1 = Helper::convert_cartesian_to_road_map(&pos1, SVC_IGNORING);
            }
            POSITION_LON_LAT | POSITION_LON_LAT_ALT => {
                let p1x = inputStorage.read_double().unwrap();
                let p1y = inputStorage.read_double().unwrap();
                pos1.set(p1x, p1y);
                GeoConvHelper::with_final(|g| g.x2cartesian_const(&mut pos1));
                if posType == POSITION_LON_LAT_ALT {
                    inputStorage.read_double().unwrap(); // altitude value is ignored
                }
                roadPos1 = Helper::convert_cartesian_to_road_map(&pos1, SVC_IGNORING);
            }
            _ => throw_traci_exception("Unknown position format used for distance request"),
        }

        // read position 2
        posType = inputStorage.read_unsigned_byte().unwrap();
        match posType {
            POSITION_ROADMAP => {
                let roadID = inputStorage.read_string().unwrap();
                roadPos2.1 = inputStorage.read_double().unwrap();
                let laneIdx = inputStorage.read_unsigned_byte().unwrap();
                roadPos2.0 = Helper::get_lane_checking(&roadID, laneIdx, roadPos2.1);
                pos2 = roadPos2.0.m().geometry_position_at_offset(roadPos2.1, 0.0);
            }
            POSITION_2D | POSITION_3D => {
                let p2x = inputStorage.read_double().unwrap();
                let p2y = inputStorage.read_double().unwrap();
                pos2.set(p2x, p2y);
                if posType == POSITION_3D {
                    inputStorage.read_double().unwrap(); // z value is ignored
                }
                roadPos2 = Helper::convert_cartesian_to_road_map(&pos2, SVC_IGNORING);
            }
            POSITION_LON_LAT | POSITION_LON_LAT_ALT => {
                let p2x = inputStorage.read_double().unwrap();
                let p2y = inputStorage.read_double().unwrap();
                pos2.set(p2x, p2y);
                GeoConvHelper::with_final(|g| g.x2cartesian_const(&mut pos2));
                if posType == POSITION_LON_LAT_ALT {
                    inputStorage.read_double().unwrap(); // altitude value is ignored
                }
                roadPos2 = Helper::convert_cartesian_to_road_map(&pos2, SVC_IGNORING);
            }
            _ => throw_traci_exception("Unknown position format used for distance request"),
        }

        // read distance type
        let distType = inputStorage.read_unsigned_byte().unwrap();

        let distance = if distType == REQUEST_DRIVINGDIST {
            Helper::get_driving_distance(&mut roadPos1, &mut roadPos2)
        } else {
            // compute air distance (default)
            pos1.distance_to(&pos2)
        };
        // write response command
        StoHelp::write_typed_double(outputStorage, distance);
    }

    /// C++: `void TraCIServerAPI_Simulation::writeVehicleStateNumber(TraCIServer &, tcpip::Storage &, MSNet::VehicleState)` [static, private]
    // void
    // TraCIServerAPI_Simulation::writeVehicleStateNumber(TraCIServer& server, tcpip::Storage& outputStorage, MSNet::VehicleState state) {
    //     StoHelp::writeTypedInt(outputStorage, (int)server.getVehicleStateChanges().find(state)->second.size());
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle_state_number(server: &mut TraCIServer, outputStorage: &mut Storage, state: MSNet_VehicleState) {
        StoHelp::write_typed_int(outputStorage, server.get_vehicle_state_changes()[&state].len() as i32);
    }

    /// C++: `void TraCIServerAPI_Simulation::writeVehicleStateIDs(TraCIServer &, tcpip::Storage &, MSNet::VehicleState)` [static, private]
    // void
    // TraCIServerAPI_Simulation::writeVehicleStateIDs(TraCIServer& server, tcpip::Storage& outputStorage, MSNet::VehicleState state) {
    //     StoHelp::writeTypedStringList(outputStorage, server.getVehicleStateChanges().find(state)->second);
    // }
    #[allow(unused_variables)]
    pub fn write_vehicle_state_i_ds(server: &mut TraCIServer, outputStorage: &mut Storage, state: MSNet_VehicleState) {
        StoHelp::write_typed_string_list(outputStorage, &server.get_vehicle_state_changes()[&state]);
    }

    /// C++: `void TraCIServerAPI_Simulation::writeTransportableStateNumber(TraCIServer &, tcpip::Storage &, MSNet::TransportableState)` [static, private]
    // void
    // TraCIServerAPI_Simulation::writeTransportableStateNumber(TraCIServer& server, tcpip::Storage& outputStorage, MSNet::TransportableState state) {
    //     StoHelp::writeTypedInt(outputStorage, (int)server.getTransportableStateChanges().find(state)->second.size());
    // }
    #[allow(unused_variables)]
    pub fn write_transportable_state_number(server: &mut TraCIServer, outputStorage: &mut Storage, state: MSNet_TransportableState) {
        StoHelp::write_typed_int(outputStorage, server.get_transportable_state_changes()[&state].len() as i32);
    }

    /// C++: `void TraCIServerAPI_Simulation::writeTransportableStateIDs(TraCIServer &, tcpip::Storage &, MSNet::TransportableState)` [static, private]
    // void
    // TraCIServerAPI_Simulation::writeTransportableStateIDs(TraCIServer& server, tcpip::Storage& outputStorage, MSNet::TransportableState state) {
    //     StoHelp::writeTypedStringList(outputStorage, server.getTransportableStateChanges().find(state)->second);
    // }
    #[allow(unused_variables)]
    pub fn write_transportable_state_i_ds(server: &mut TraCIServer, outputStorage: &mut Storage, state: MSNet_TransportableState) {
        StoHelp::write_typed_string_list(outputStorage, &server.get_transportable_state_changes()[&state]);
    }

    /// C++: `void TraCIServerAPI_Simulation::TraCIServerAPI_Simulation(const TraCIServerAPI_Simulation &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Simulation) -> Self {
        panic!("TraCIServerAPI_Simulation copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Simulation & TraCIServerAPI_Simulation::operator=(const TraCIServerAPI_Simulation &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Simulation) -> &mut TraCIServerAPI_Simulation {
        panic!("TraCIServerAPI_Simulation assignment operator is private and undefined in C++; it cannot be called")
    }

}

