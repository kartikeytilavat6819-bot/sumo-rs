//! Port of traci-server/TraCIServerAPI_Vehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::{delta_t, steps2time, SUMOTIME_MAX};
use utils::common::to_string::to_string;

/// APIs for getting/setting vehicle values via TraCI
/// C++ class `TraCIServerAPI_Vehicle` (traci-server/TraCIServerAPI_Vehicle.h)
pub struct TraCIServerAPI_Vehicle {
}

impl TraCIServerAPI_Vehicle {
    /// C++: `bool TraCIServerAPI_Vehicle::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc4: Change Vehicle State)
    // bool
    // TraCIServerAPI_Vehicle::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                    tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::CMD_STOP && variable != libsumo::CMD_CHANGELANE
    //             && variable != libsumo::CMD_REROUTE_TO_PARKING
    //             && variable != libsumo::CMD_CHANGESUBLANE && variable != libsumo::CMD_OPENGAP
    //             && variable != libsumo::CMD_REPLACE_STOP
    //             && variable != libsumo::CMD_INSERT_STOP
    //             && variable != libsumo::VAR_STOP_PARAMETER
    //             && variable != libsumo::CMD_SLOWDOWN && variable != libsumo::CMD_CHANGETARGET && variable != libsumo::CMD_RESUME
    //             && variable != libsumo::VAR_TYPE && variable != libsumo::VAR_ROUTE_ID && variable != libsumo::VAR_ROUTE
    //             && variable != libsumo::VAR_LANEPOSITION_LAT
    //             && variable != libsumo::VAR_UPDATE_BESTLANES
    //             && variable != libsumo::VAR_EDGE_TRAVELTIME && variable != libsumo::VAR_EDGE_EFFORT
    //             && variable != libsumo::CMD_REROUTE_TRAVELTIME && variable != libsumo::CMD_REROUTE_EFFORT
    //             && variable != libsumo::VAR_SIGNALS && variable != libsumo::VAR_MOVE_TO
    //             && variable != libsumo::VAR_LENGTH && variable != libsumo::VAR_MAXSPEED && variable != libsumo::VAR_VEHICLECLASS
    //             && variable != libsumo::VAR_SPEED_FACTOR && variable != libsumo::VAR_EMISSIONCLASS
    //             && variable != libsumo::VAR_WIDTH && variable != libsumo::VAR_MINGAP && variable != libsumo::VAR_SHAPECLASS
    //             && variable != libsumo::VAR_ACCEL && variable != libsumo::VAR_DECEL && variable != libsumo::VAR_IMPERFECTION
    //             && variable != libsumo::VAR_APPARENT_DECEL && variable != libsumo::VAR_EMERGENCY_DECEL
    //             && variable != libsumo::VAR_ACTIONSTEPLENGTH
    //             && variable != libsumo::VAR_TAU && variable != libsumo::VAR_LANECHANGE_MODE
    //             && variable != libsumo::VAR_SPEED && variable != libsumo::VAR_ACCELERATION && variable != libsumo::VAR_PREV_SPEED && variable != libsumo::VAR_SPEEDSETMODE && variable != libsumo::VAR_COLOR
    //             && variable != libsumo::ADD && variable != libsumo::ADD_FULL && variable != libsumo::REMOVE
    //             && variable != libsumo::VAR_HEIGHT
    //             && variable != libsumo::VAR_MASS
    //             && variable != libsumo::VAR_ROUTING_MODE
    //             && variable != libsumo::VAR_LATALIGNMENT
    //             && variable != libsumo::VAR_MAXSPEED_LAT
    //             && variable != libsumo::VAR_MINGAP_LAT
    //             && variable != libsumo::VAR_LINE
    //             && variable != libsumo::VAR_VIA
    //             && variable != libsumo::VAR_IMPATIENCE
    //             && variable != libsumo::VAR_BOARDING_DURATION
    //             && variable != libsumo::VAR_HIGHLIGHT
    //             && variable != libsumo::CMD_TAXI_DISPATCH
    //             && variable != libsumo::MOVE_TO_XY && variable != libsumo::VAR_PARAMETER/* && variable != libsumo::VAR_SPEED_TIME_LINE && variable != libsumo::VAR_LANE_TIME_LINE*/
    //        ) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Change Vehicle State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    // #ifdef DEBUG_MOVEXY
    //     std::cout << SIMTIME << " processSet veh=" << id << "\n";
    // #endif
    //     const bool shouldExist = variable != libsumo::ADD && variable != libsumo::ADD_FULL;
    //     SUMOVehicle* sumoVehicle = MSNet::getInstance()->getVehicleControl().getVehicle(id);
    //     if (sumoVehicle == nullptr) {
    //         if (shouldExist) {
    //             return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Vehicle '" + id + "' is not known", outputStorage);
    //         }
    //     }
    //     MSBaseVehicle* v = dynamic_cast<MSBaseVehicle*>(sumoVehicle);
    //     if (v == nullptr && shouldExist) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_GET_VEHICLE_VARIABLE, "Vehicle '" + id + "' is not a proper vehicle", outputStorage);
    //     }
    //     try {
    //         switch (variable) {
    //             case libsumo::CMD_STOP: {
    //                 const int compoundSize = StoHelp::readCompound(inputStorage, -1, "Stop needs a compound object description.");
    //                 if (compoundSize < 4 || compoundSize > 7) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Stop needs a compound object description of four to seven items.", outputStorage);
    //                 }
    //                 // read road map position
    //                 const std::string edgeID = StoHelp::readTypedString(inputStorage, "The first stop parameter must be the edge id given as a string.");
    //                 const double pos = StoHelp::readTypedDouble(inputStorage, "The second stop parameter must be the end position along the edge given as a double.");
    //                 const int laneIndex = StoHelp::readTypedByte(inputStorage, "The third stop parameter must be the lane index given as a byte.");
    //                 const double duration = StoHelp::readTypedDouble(inputStorage, "The fourth stop parameter must be the stopping duration given as a double.");
    //                 int stopFlags = 0;
    //                 if (compoundSize >= 5) {
    //                     stopFlags = StoHelp::readTypedByte(inputStorage, "The fifth stop parameter must be a byte indicating its parking/triggered status.");
    //                 }
    //                 double startPos = libsumo::INVALID_DOUBLE_VALUE;
    //                 if (compoundSize >= 6) {
    //                     startPos = StoHelp::readTypedDouble(inputStorage, "The sixth stop parameter must be the start position along the edge given as a double.");
    //                 }
    //                 double until = libsumo::INVALID_DOUBLE_VALUE;
    //                 if (compoundSize >= 7) {
    //                     until = StoHelp::readTypedDouble(inputStorage, "The seventh stop parameter must be the minimum departure time given as a double.");
    //                 }
    //                 libsumo::Vehicle::setStop(id, edgeID, pos, laneIndex, duration, stopFlags, startPos, until);
    //             }
    //             break;
    //             case libsumo::CMD_REPLACE_STOP:
    //                 if (!insertReplaceStop(server, inputStorage, outputStorage, id, true)) {
    //                     return false;
    //                 }
    //                 break;
    //             case libsumo::CMD_INSERT_STOP:
    //                 if (!insertReplaceStop(server, inputStorage, outputStorage, id, false)) {
    //                     return false;
    //                 }
    //                 break;
    //             case libsumo::VAR_STOP_PARAMETER: {
    //                 const int compoundSize = StoHelp::readCompound(inputStorage, -1, "Setting stop parameter needs a compound object description.");
    //                 if (compoundSize != 3 && compoundSize != 4) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting a stop parameter needs a compound object description of 3 or 4 items.", outputStorage);
    //                 }
    //                 const int nextStopIndex = StoHelp::readTypedInt(inputStorage, "The first setStopParameter parameter must be the nextStopIndex given as an integer.");
    //                 const std::string param = StoHelp::readTypedString(inputStorage, "The second setStopParameter parameter must be the param given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The third setStopParameter parameter must be the value given as a string.");
    //                 int customParam = 0;
    //                 if (compoundSize == 4) {
    //                     customParam = StoHelp::readTypedByte(inputStorage, "The fourth setStopParameter parameter must be the customParam flag given as a byte.");
    //                 }
    //                 libsumo::Vehicle::setStopParameter(id, nextStopIndex, param, value, customParam != 0);
    //             }
    //             break;
    //             case libsumo::CMD_REROUTE_TO_PARKING: {
    //                 StoHelp::readCompound(inputStorage, 1, "Reroute to stop needs a compound object description of 1 item.");
    //                 libsumo::Vehicle::rerouteParkingArea(id, StoHelp::readTypedString(inputStorage, "The first reroute to stop parameter must be the parking area id given as a string."));
    //             }
    //             break;
    //             case libsumo::CMD_RESUME: {
    //                 StoHelp::readCompound(inputStorage, 0, "Resuming requires an empty compound object.");
    //                 libsumo::Vehicle::resume(id);
    //             }
    //             break;
    //             case libsumo::CMD_CHANGELANE: {
    //                 const int compoundSize = StoHelp::readCompound(inputStorage, -1, "Lane change needs a compound object description.");
    //                 if (compoundSize != 3 && compoundSize != 2) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Lane change needs a compound object description of two or three items.", outputStorage);
    //                 }
    //                 const int laneIndex = StoHelp::readTypedByte(inputStorage, "The first lane change parameter must be the lane index given as a byte.");
    //                 const double duration = StoHelp::readTypedDouble(inputStorage, "The second lane change parameter must be the duration given as a double.");
    //                 // relative lane change
    //                 int relative = 0;
    //                 if (compoundSize == 3) {
    //                     relative = StoHelp::readTypedByte(inputStorage, "The third lane change parameter must be a Byte for defining whether a relative lane change should be applied.");
    //                 }
    //
    //                 if ((laneIndex < 0 || laneIndex >= (int)v->getEdge()->getLanes().size()) && relative < 1) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "No lane with index '" + toString(laneIndex) + "' on road '" + v->getEdge()->getID() + "'.", outputStorage);
    //                 }
    //
    //                 if (relative < 1) {
    //                     libsumo::Vehicle::changeLane(id, laneIndex, duration);
    //                 } else {
    //                     libsumo::Vehicle::changeLaneRelative(id, laneIndex, duration);
    //                 }
    //             }
    //             break;
    //             case libsumo::CMD_CHANGESUBLANE: {
    //                 libsumo::Vehicle::changeSublane(id, StoHelp::readTypedDouble(inputStorage, "Sublane-changing requires a double."));
    //             }
    //             break;
    //             case libsumo::CMD_SLOWDOWN: {
    //                 StoHelp::readCompound(inputStorage, 2, "Slow down needs a compound object description of two items.");
    //                 const double newSpeed = StoHelp::readTypedDouble(inputStorage, "The first slow down parameter must be the speed given as a double.");
    //                 if (newSpeed < 0.) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Speed must not be negative", outputStorage);
    //                 }
    //                 const double duration = StoHelp::readTypedDouble(inputStorage, "The second slow down parameter must be the duration given as a double.");
    //                 if (duration < 0. || SIMTIME + duration > STEPS2TIME(SUMOTime_MAX - DELTA_T)) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid time interval", outputStorage);
    //                 }
    //                 libsumo::Vehicle::slowDown(id, newSpeed, duration);
    //             }
    //             break;
    //             case libsumo::CMD_CHANGETARGET: {
    //                 libsumo::Vehicle::changeTarget(id, StoHelp::readTypedString(inputStorage, "Change target requires a string containing the id of the new destination edge as parameter."));
    //             }
    //             break;
    //             case libsumo::CMD_OPENGAP: {
    //                 const int compoundSize = StoHelp::readCompound(inputStorage, -1, "Create gap needs a compound object description.");
    //                 if (compoundSize != 5 && compoundSize != 6) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Create gap needs a compound object description of five or six items.", outputStorage);
    //                 }
    //                 const double newTimeHeadway = StoHelp::readTypedDouble(inputStorage, "The first create gap parameter must be the new desired time headway (tau) given as a double.");
    //                 double newSpaceHeadway = StoHelp::readTypedDouble(inputStorage, "The second create gap parameter must be the new desired space headway given as a double.");
    //                 const double duration = StoHelp::readTypedDouble(inputStorage, "The third create gap parameter must be the duration given as a double.");
    //                 const double changeRate = StoHelp::readTypedDouble(inputStorage, "The fourth create gap parameter must be the change rate given as a double.");
    //                 const double maxDecel = StoHelp::readTypedDouble(inputStorage, "The fifth create gap parameter must be the maximal braking rate given as a double.");
    //
    //                 if (newTimeHeadway == -1 && newSpaceHeadway == -1 && duration == -1 && changeRate == -1 && maxDecel == -1) {
    //                     libsumo::Vehicle::deactivateGapControl(id);
    //                 } else {
    //                     if (newTimeHeadway <= 0) {
    //                         if (newTimeHeadway != -1) {
    //                             return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "The value for the new desired time headway (tau) must be positive for create gap", outputStorage);
    //                         } // else if == -1: keep vehicles current headway, see libsumo::Vehicle::openGap
    //                     }
    //                     if (newSpaceHeadway < 0) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "The value for the new desired space headway must be non-negative for create gap", outputStorage);
    //                     }
    //                     if ((duration < 0 && duration != -1)  || SIMTIME + duration > STEPS2TIME(SUMOTime_MAX - DELTA_T)) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid time interval for create gap", outputStorage);
    //                     }
    //                     if (changeRate <= 0) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "The value for the change rate must be positive for the openGap command", outputStorage);
    //                     }
    //                     if (maxDecel <= 0 && maxDecel != -1 && maxDecel != libsumo::INVALID_DOUBLE_VALUE) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "The value for the maximal braking rate must be positive for the openGap command", outputStorage);
    //                     } // else if <= 0: don't limit cf model's suggested brake rate, see libsumo::Vehicle::openGap
    //                     std::string refVehID = "";
    //                     if (compoundSize == 6) {
    //                         refVehID = StoHelp::readTypedString(inputStorage, "The sixth create gap parameter must be a reference vehicle's ID given as a string.");
    //                     }
    //                     libsumo::Vehicle::openGap(id, newTimeHeadway, newSpaceHeadway, duration, changeRate, maxDecel, refVehID);
    //                 }
    //             }
    //             break;
    //             case libsumo::VAR_TYPE: {
    //                 libsumo::Vehicle::setType(id, StoHelp::readTypedString(inputStorage, "The vehicle type id must be given as a string."));
    //             }
    //             break;
    //             case libsumo::VAR_ROUTE_ID: {
    //                 libsumo::Vehicle::setRouteID(id, StoHelp::readTypedString(inputStorage, "The route id must be given as a string."));
    //             }
    //             break;
    //             case libsumo::VAR_ROUTE: {
    //                 libsumo::Vehicle::setRoute(id, StoHelp::readTypedStringList(inputStorage, "A route must be defined as a list of edge ids."));
    //             }
    //             break;
    //             case libsumo::VAR_EDGE_TRAVELTIME: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Setting travel time requires a compound object.");
    //                 std::string edgeID;
    //                 double begTime = 0.;
    //                 double endTime = std::numeric_limits<double>::max();
    //                 double value = libsumo::INVALID_DOUBLE_VALUE;
    //                 if (parameterCount == 4) {
    //                     begTime = StoHelp::readTypedDouble(inputStorage, "Setting travel time using 4 parameters requires the begin time as first parameter.");
    //                     endTime = StoHelp::readTypedDouble(inputStorage, "Setting travel time using 4 parameters requires the end time as second parameter.");
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting travel time using 4 parameters requires the referenced edge as third parameter.");
    //                     value = StoHelp::readTypedDouble(inputStorage, "Setting travel time using 4 parameters requires the travel time as double as fourth parameter.");
    //                 } else if (parameterCount == 2) {
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting travel time using 2 parameters requires the referenced edge as first parameter.");
    //                     value = StoHelp::readTypedDouble(inputStorage, "Setting travel time using 2 parameters requires the travel time as double as second parameter.");
    //                 } else if (parameterCount == 1) {
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting travel time using 1 parameter requires the referenced edge as first parameter.");
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting travel time requires 1, 2, or 4 parameters.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setAdaptedTraveltime(id, edgeID, value, begTime, endTime);
    //             }
    //             break;
    //             case libsumo::VAR_EDGE_EFFORT: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Setting travel time requires a compound object.");
    //                 std::string edgeID;
    //                 double begTime = 0.;
    //                 double endTime = std::numeric_limits<double>::max();
    //                 double value = libsumo::INVALID_DOUBLE_VALUE;
    //                 if (parameterCount == 4) {
    //                     begTime = StoHelp::readTypedDouble(inputStorage, "Setting effort using 4 parameters requires the begin time as first parameter.");
    //                     endTime = StoHelp::readTypedDouble(inputStorage, "Setting effort using 4 parameters requires the end time as second parameter.");
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting effort using 4 parameters requires the referenced edge as third parameter.");
    //                     value = StoHelp::readTypedDouble(inputStorage, "Setting effort using 4 parameters requires the effort as double as fourth parameter.");
    //                 } else if (parameterCount == 2) {
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting effort using 2 parameters requires the referenced edge as first parameter.");
    //                     value = StoHelp::readTypedDouble(inputStorage, "Setting effort using 2 parameters requires the effort as double as second parameter.");
    //                 } else if (parameterCount == 1) {
    //                     edgeID = StoHelp::readTypedString(inputStorage, "Setting effort using 1 parameter requires the referenced edge as first parameter.");
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting effort requires 1, 2, or 4 parameters.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setEffort(id, edgeID, value, begTime, endTime);
    //             }
    //             break;
    //             case libsumo::CMD_REROUTE_TRAVELTIME: {
    //                 StoHelp::readCompound(inputStorage, 0, "Rerouting by travel time requires an empty compound object.");
    //                 libsumo::Vehicle::rerouteTraveltime(id, false);
    //             }
    //             break;
    //             case libsumo::CMD_REROUTE_EFFORT: {
    //                 StoHelp::readCompound(inputStorage, 0, "Rerouting by effort requires an empty compound object.");
    //                 libsumo::Vehicle::rerouteEffort(id);
    //             }
    //             break;
    //             case libsumo::VAR_SIGNALS:
    //                 libsumo::Vehicle::setSignals(id, StoHelp::readTypedInt(inputStorage, "Setting signals requires an integer."));
    //                 break;
    //             case libsumo::VAR_MOVE_TO: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "Setting position requires a compound object.");
    //                 if (parameterCount < 2 || parameterCount > 3) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting position should obtain the lane id and the position and optionally the reason.", outputStorage);
    //                 }
    //                 const std::string laneID = StoHelp::readTypedString(inputStorage, "The first parameter for setting a position must be the lane ID given as a string.");
    //                 const double position = StoHelp::readTypedDouble(inputStorage, "The second parameter for setting a position must be the position given as a double.");
    //                 int reason = libsumo::MOVE_AUTOMATIC;
    //                 if (parameterCount == 3) {
    //                     reason = StoHelp::readTypedInt(inputStorage, "The third parameter for setting a position must be the reason given as an int.");
    //                 }
    //                 // process
    //                 libsumo::Vehicle::moveTo(id, laneID, position, reason);
    //             }
    //             break;
    //             case libsumo::VAR_IMPATIENCE: {
    //                 libsumo::Vehicle::setImpatience(id, StoHelp::readTypedDouble(inputStorage, "Setting impatience requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_SPEED: {
    //                 libsumo::Vehicle::setSpeed(id, StoHelp::readTypedDouble(inputStorage, "Setting speed requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_ACCELERATION: {
    //                 StoHelp::readCompound(inputStorage, 2, "Setting acceleration requires 2 parameters.");
    //                 const double accel = StoHelp::readTypedDouble(inputStorage, "Setting acceleration requires the acceleration as first parameter given as a double.");
    //                 const double duration = StoHelp::readTypedDouble(inputStorage, "Setting acceleration requires the duration as second parameter given as a double.");
    //                 if (duration < 0) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Duration must not be negative.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setAcceleration(id, accel, duration);
    //             }
    //             break;
    //             case libsumo::VAR_PREV_SPEED: {
    //                 double prevSpeed = 0;
    //                 double prevAcceleration = libsumo::INVALID_DOUBLE_VALUE;
    //                 int inputtype = inputStorage.readUnsignedByte();
    //                 if (inputtype == libsumo::TYPE_COMPOUND) {
    //                     // Setting previous speed with 2 parameters, uses a compound object description
    //                     const int parameterCount = inputStorage.readInt();
    //                     if (parameterCount == 2) {
    //                         prevSpeed = StoHelp::readTypedDouble(inputStorage, "Setting previous speed using 2 parameters requires the previous speed as first parameter given as a double.");
    //                         prevAcceleration = StoHelp::readTypedDouble(inputStorage, "Setting previous speed using 2 parameters requires the previous acceleration as second parameter given as a double.");
    //                     } else if (parameterCount == 1) {
    //                         prevSpeed = StoHelp::readTypedDouble(inputStorage, "Setting previous speed using 1 parameter requires the previous speed as first parameter given as a double.");
    //                     } else {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting previous speed requires 1 or 2 parameters.", outputStorage);
    //                     }
    //                 } else if (inputtype == libsumo::TYPE_DOUBLE) {
    //                     // Setting previous speed with 1 parameter (double), no compound object description
    //                     prevSpeed = inputStorage.readDouble();
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Setting previous speed requires 1 parameter given as a double or 2 parameters as compound object description.", outputStorage);
    //                 }
    //                 if (prevSpeed < 0) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Previous speed must not be negative.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setPreviousSpeed(id, prevSpeed, prevAcceleration);
    //             }
    //             break;
    //             case libsumo::VAR_SPEEDSETMODE:
    //                 libsumo::Vehicle::setSpeedMode(id, StoHelp::readTypedInt(inputStorage, "Setting speed mode requires an integer."));
    //                 break;
    //             case libsumo::VAR_LANECHANGE_MODE:
    //                 libsumo::Vehicle::setLaneChangeMode(id, StoHelp::readTypedInt(inputStorage, "Setting lane change mode requires an integer."));
    //                 break;
    //             case libsumo::VAR_ROUTING_MODE:
    //                 libsumo::Vehicle::setRoutingMode(id, StoHelp::readTypedInt(inputStorage, "Setting routing mode requires an integer."));
    //                 break;
    //             case libsumo::VAR_COLOR: {
    //                 libsumo::Vehicle::setColor(id, StoHelp::readTypedColor(inputStorage, "The color must be given using the according type."));
    //                 break;
    //             }
    //             case libsumo::ADD: {
    //                 StoHelp::readCompound(inputStorage, 6, "Adding a vehicle needs six parameters.");
    //                 const std::string vTypeID = StoHelp::readTypedString(inputStorage, "First parameter (type) requires a string.");
    //                 const std::string routeID = StoHelp::readTypedString(inputStorage, "Second parameter (route) requires a string.");
    //                 const int departCode = StoHelp::readTypedInt(inputStorage, "Third parameter (depart) requires an integer.");
    //                 std::string depart = toString(STEPS2TIME(departCode));
    //                 if (-departCode == static_cast<int>(DepartDefinition::TRIGGERED)) {
    //                     depart = "triggered";
    //                 } else if (-departCode == static_cast<int>(DepartDefinition::CONTAINER_TRIGGERED)) {
    //                     depart = "containerTriggered";
    //                 } else if (-departCode == static_cast<int>(DepartDefinition::NOW)) {
    //                     depart = "now";
    //                 } else if (-departCode == static_cast<int>(DepartDefinition::SPLIT)) {
    //                     depart = "split";
    //                 } else if (-departCode == static_cast<int>(DepartDefinition::BEGIN)) {
    //                     depart = "begin";
    //                 }
    //
    //                 const double departPosCode = StoHelp::readTypedDouble(inputStorage, "Fourth parameter (position) requires a double.");
    //                 std::string departPos = toString(departPosCode);
    //                 if (-departPosCode == (int)DepartPosDefinition::RANDOM) {
    //                     departPos = "random";
    //                 } else if (-departPosCode == (int)DepartPosDefinition::RANDOM_FREE) {
    //                     departPos = "random_free";
    //                 } else if (-departPosCode == (int)DepartPosDefinition::FREE) {
    //                     departPos = "free";
    //                 } else if (-departPosCode == (int)DepartPosDefinition::BASE) {
    //                     departPos = "base";
    //                 } else if (-departPosCode == (int)DepartPosDefinition::LAST) {
    //                     departPos = "last";
    //                 } else if (-departPosCode == (int)DepartPosDefinition::GIVEN) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid departure position.", outputStorage);
    //                 }
    //
    //                 const double departSpeedCode = StoHelp::readTypedDouble(inputStorage, "Fifth parameter (speed) requires a double.");
    //                 std::string departSpeed = toString(departSpeedCode);
    //                 if (-departSpeedCode == (int)DepartSpeedDefinition::RANDOM) {
    //                     departSpeed = "random";
    //                 } else if (-departSpeedCode == (int)DepartSpeedDefinition::MAX) {
    //                     departSpeed = "max";
    //                 } else if (-departSpeedCode == (int)DepartSpeedDefinition::DESIRED) {
    //                     departSpeed = "desired";
    //                 } else if (-departSpeedCode == (int)DepartSpeedDefinition::LIMIT) {
    //                     departSpeed = "speedLimit";
    //                 } else if (-departSpeedCode == (int)DepartSpeedDefinition::LAST) {
    //                     departSpeed = "last";
    //                 } else if (-departSpeedCode == (int)DepartSpeedDefinition::AVG) {
    //                     departSpeed = "avg";
    //                 }
    //
    //                 const int departLaneCode = StoHelp::readTypedByte(inputStorage, "Sixth parameter (lane) requires a byte.");
    //                 std::string departLane = toString(departLaneCode);
    //                 if (-departLaneCode == (int)DepartLaneDefinition::RANDOM) {
    //                     departLane = "random";
    //                 } else if (-departLaneCode == (int)DepartLaneDefinition::FREE) {
    //                     departLane = "free";
    //                 } else if (-departLaneCode == (int)DepartLaneDefinition::ALLOWED_FREE) {
    //                     departLane = "allowed";
    //                 } else if (-departLaneCode == (int)DepartLaneDefinition::BEST_FREE) {
    //                     departLane = "best";
    //                 } else if (-departLaneCode == (int)DepartLaneDefinition::FIRST_ALLOWED) {
    //                     departLane = "first";
    //                 }
    //                 libsumo::Vehicle::add(id, routeID, vTypeID, depart, departLane, departPos, departSpeed);
    //             }
    //             break;
    //             case libsumo::ADD_FULL: {
    //                 const int numElements = StoHelp::readCompound(inputStorage, -1, "Adding a fully specified vehicle needs fourteen to eighteen parameters.");
    //                 if (numElements < 14 || numElements > 18) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Adding a fully specified vehicle needs fourteen to eighteen parameters.", outputStorage);
    //                 }
    //                 const std::string routeID = StoHelp::readTypedString(inputStorage, "First parameter (route) requires a string.");
    //                 const std::string vTypeID = StoHelp::readTypedString(inputStorage, "Second parameter (type) requires a string.");
    //                 const std::string depart = StoHelp::readTypedString(inputStorage, "Third parameter (depart) requires an string.");
    //                 const std::string departLane = StoHelp::readTypedString(inputStorage, "Fourth parameter (depart lane) requires a string.");
    //                 const std::string departPos = StoHelp::readTypedString(inputStorage, "Fifth parameter (depart position) requires a string.");
    //                 const std::string departSpeed = StoHelp::readTypedString(inputStorage, "Sixth parameter (depart speed) requires a string.");
    //                 const std::string arrivalLane = StoHelp::readTypedString(inputStorage, "Seventh parameter (arrival lane) requires a string.");
    //                 const std::string arrivalPos = StoHelp::readTypedString(inputStorage, "Eighth parameter (arrival position) requires a string.");
    //                 const std::string arrivalSpeed = StoHelp::readTypedString(inputStorage, "Ninth parameter (arrival speed) requires a string.");
    //                 const std::string fromTaz = StoHelp::readTypedString(inputStorage, "Tenth parameter (from taz) requires a string.");
    //                 const std::string toTaz = StoHelp::readTypedString(inputStorage, "Eleventh parameter (to taz) requires a string.");
    //                 const std::string line = StoHelp::readTypedString(inputStorage, "Twelth parameter (line) requires a string.");
    //                 const int personCapacity = StoHelp::readTypedInt(inputStorage, "13th parameter (person capacity) requires an int.");
    //                 const int personNumber = StoHelp::readTypedInt(inputStorage, "14th parameter (person number) requires an int.");
    //                 std::string departPosLat;
    //                 std::string arrivalPosLat;
    //                 double speedFactor = -1.;
    //                 std::string insertionChecks;
    //                 if (numElements >= 15) {
    //                     departPosLat = StoHelp::readTypedString(inputStorage, "15th parameter (depart position lat) requires a string.");
    //                 }
    //                 if (numElements >= 16) {
    //                     arrivalPosLat = StoHelp::readTypedString(inputStorage, "16th parameter (arrival position lat) requires a string.");
    //                 }
    //                 if (numElements >= 17) {
    //                     speedFactor = StoHelp::readTypedDouble(inputStorage, "17th parameter (speed factor) requires a double.");
    //                 }
    //                 if (numElements >= 18) {
    //                     insertionChecks = StoHelp::readTypedString(inputStorage, "18th parameter (insertion checks) requires a string.");
    //                 }
    //                 libsumo::Vehicle::add(id, routeID, vTypeID, depart, departLane, departPos, departSpeed, arrivalLane, arrivalPos, arrivalSpeed,
    //                                       fromTaz, toTaz, line, personCapacity, personNumber,
    //                                       departPosLat, arrivalPosLat, speedFactor, insertionChecks);
    //             }
    //             break;
    //             case libsumo::REMOVE: {
    //                 libsumo::Vehicle::remove(id, (char)StoHelp::readTypedByte(inputStorage, "Removing a vehicle requires a byte."));
    //             }
    //             break;
    //             case libsumo::MOVE_TO_XY: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "MoveToXY vehicle requires a compound object.");
    //                 if (parameterCount < 5 || parameterCount > 7) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "MoveToXY vehicle should obtain: edgeID, lane, x, y, angle and optionally keepRouteFlag and matchThreshold.", outputStorage);
    //                 }
    //                 const std::string edgeID = StoHelp::readTypedString(inputStorage, "The first parameter for moveToXY must be the edge ID given as a string.");
    //                 const int laneIndex = StoHelp::readTypedInt(inputStorage, "The second parameter for moveToXY must be lane given as an int.");
    //                 const double x = StoHelp::readTypedDouble(inputStorage, "The third parameter for moveToXY must be the x-position given as a double.");
    //                 const double y = StoHelp::readTypedDouble(inputStorage, "The fourth parameter for moveToXY must be the y-position given as a double.");
    //                 const double angle = StoHelp::readTypedDouble(inputStorage, "The fifth parameter for moveToXY must be the angle given as a double.");
    //                 int keepRouteFlag = 1;
    //                 if (parameterCount >= 6) {
    //                     keepRouteFlag = StoHelp::readTypedByte(inputStorage, "The sixth parameter for moveToXY must be the keepRouteFlag given as a byte.");
    //                 }
    //                 double matchThreshold = 100.;
    //                 if (parameterCount == 7) {
    //                     matchThreshold = StoHelp::readTypedDouble(inputStorage, "The seventh parameter for moveToXY must be the matchThreshold given as a double.");
    //                 }
    //                 libsumo::Vehicle::moveToXY(id, edgeID, laneIndex, x, y, angle, keepRouteFlag, matchThreshold);
    //             }
    //             break;
    //             case libsumo::VAR_SPEED_FACTOR: {
    //                 libsumo::Vehicle::setSpeedFactor(id, StoHelp::readTypedDouble(inputStorage, "Setting speed factor requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_LINE: {
    //                 libsumo::Vehicle::setLine(id, StoHelp::readTypedString(inputStorage, "The line must be given as a string."));
    //             }
    //             break;
    //             case libsumo::VAR_VIA: {
    //                 libsumo::Vehicle::setVia(id, StoHelp::readTypedStringList(inputStorage, "Vias must be defined as a list of edge ids."));
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Vehicle::setParameter(id, name, value);
    //             }
    //             break;
    //             case libsumo::VAR_HIGHLIGHT: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for highlighting an object.");
    //                 if (parameterCount > 5) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Highlighting an object needs zero to five parameters.", outputStorage);
    //                 }
    //
    //                 libsumo::TraCIColor col = libsumo::TraCIColor(255, 0, 0);
    //                 if (parameterCount > 0) {
    //                     col = StoHelp::readTypedColor(inputStorage, "The first parameter for highlighting must be the highlight color.");
    //                 }
    //                 double size = -1;
    //                 if (parameterCount > 1) {
    //                     size = StoHelp::readTypedDouble(inputStorage, "The second parameter for highlighting must be the highlight size.");
    //                 }
    //                 int alphaMax = -1;
    //                 if (parameterCount > 2) {
    //                     alphaMax = StoHelp::readTypedUnsignedByte(inputStorage, "The third parameter for highlighting must be maximal alpha.");
    //                 }
    //                 double duration = -1;
    //                 if (parameterCount > 3) {
    //                     duration = StoHelp::readTypedDouble(inputStorage, "The fourth parameter for highlighting must be the highlight duration.");
    //                 }
    //                 int type = 0;
    //                 if (parameterCount > 4) {
    //                     type = StoHelp::readTypedUnsignedByte(inputStorage, "The fifth parameter for highlighting must be the highlight type id as ubyte.");
    //                 }
    //                 libsumo::Vehicle::highlight(id, col, size, alphaMax, duration, type);
    //             }
    //             break;
    //             case libsumo::CMD_TAXI_DISPATCH: {
    //                 libsumo::Vehicle::dispatchTaxi(id, StoHelp::readTypedStringList(inputStorage, "A dispatch command  must be defined as a list of reservation ids."));
    //             }
    //             break;
    //             case libsumo::VAR_ACTIONSTEPLENGTH: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "Setting action step length requires a double.");
    //                 if (fabs(value) == std::numeric_limits<double>::infinity()) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid action step length.", outputStorage);
    //                 }
    //                 bool resetActionOffset = value >= 0.0;
    //                 libsumo::Vehicle::setActionStepLength(id, fabs(value), resetActionOffset);
    //             }
    //             break;
    //             case libsumo::VAR_LANEPOSITION_LAT: {
    //                 libsumo::Vehicle::setLateralLanePosition(id, StoHelp::readTypedDouble(inputStorage, "Setting lateral lane position requires a double."));
    //             }
    //             break;
    //             case libsumo::VAR_UPDATE_BESTLANES: {
    //                 libsumo::Vehicle::updateBestLanes(id);
    //             }
    //             break;
    //             case libsumo::VAR_MINGAP: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "Setting minimum gap requires a double.");
    //                 if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid minimum gap.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setMinGap(id, value);
    //             }
    //             break;
    //             case libsumo::VAR_MINGAP_LAT: {
    //                 const double value = StoHelp::readTypedDouble(inputStorage, "Setting minimum lateral gap requires a double.");
    //                 if (value < 0.0 || fabs(value) == std::numeric_limits<double>::infinity()) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Invalid minimum lateral gap.", outputStorage);
    //                 }
    //                 libsumo::Vehicle::setMinGapLat(id, value);
    //             }
    //             break;
    //             default: {
    //                 try {
    //                     if (!TraCIServerAPI_VehicleType::setVariable(libsumo::CMD_SET_VEHICLE_VARIABLE, variable, v->getSingularType().getID(), server, inputStorage, outputStorage)) {
    //                         return false;
    //                     }
    //                 } catch (ProcessError& e) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, e.what(), outputStorage);
    //                 } catch (libsumo::TraCIException& e) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, e.what(), outputStorage);
    //                 }
    //             }
    //             break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        // C++ outer catch (TraCIException) is reproduced by dispatch_command; a ProcessError escapes in C++ too and is re-raised there
        let warning = String::new();
        let variable = inputStorage.read_unsigned_byte().unwrap();
        let supported = variable == CMD_STOP
            || variable == CMD_CHANGELANE
            || variable == CMD_REROUTE_TO_PARKING
            || variable == CMD_CHANGESUBLANE
            || variable == CMD_OPENGAP
            || variable == CMD_REPLACE_STOP
            || variable == CMD_INSERT_STOP
            || variable == VAR_STOP_PARAMETER
            || variable == CMD_SLOWDOWN
            || variable == CMD_CHANGETARGET
            || variable == CMD_RESUME
            || variable == VAR_TYPE
            || variable == VAR_ROUTE_ID
            || variable == VAR_ROUTE
            || variable == VAR_LANEPOSITION_LAT
            || variable == VAR_UPDATE_BESTLANES
            || variable == VAR_EDGE_TRAVELTIME
            || variable == VAR_EDGE_EFFORT
            || variable == CMD_REROUTE_TRAVELTIME
            || variable == CMD_REROUTE_EFFORT
            || variable == VAR_SIGNALS
            || variable == VAR_MOVE_TO
            || variable == VAR_LENGTH
            || variable == VAR_MAXSPEED
            || variable == VAR_VEHICLECLASS
            || variable == VAR_SPEED_FACTOR
            || variable == VAR_EMISSIONCLASS
            || variable == VAR_WIDTH
            || variable == VAR_MINGAP
            || variable == VAR_SHAPECLASS
            || variable == VAR_ACCEL
            || variable == VAR_DECEL
            || variable == VAR_IMPERFECTION
            || variable == VAR_APPARENT_DECEL
            || variable == VAR_EMERGENCY_DECEL
            || variable == VAR_ACTIONSTEPLENGTH
            || variable == VAR_TAU
            || variable == VAR_LANECHANGE_MODE
            || variable == VAR_SPEED
            || variable == VAR_ACCELERATION
            || variable == VAR_PREV_SPEED
            || variable == VAR_SPEEDSETMODE
            || variable == VAR_COLOR
            || variable == ADD
            || variable == ADD_FULL
            || variable == REMOVE
            || variable == VAR_HEIGHT
            || variable == VAR_MASS
            || variable == VAR_ROUTING_MODE
            || variable == VAR_LATALIGNMENT
            || variable == VAR_MAXSPEED_LAT
            || variable == VAR_MINGAP_LAT
            || variable == VAR_LINE
            || variable == VAR_VIA
            || variable == VAR_IMPATIENCE
            || variable == VAR_BOARDING_DURATION
            || variable == VAR_HIGHLIGHT
            || variable == CMD_TAXI_DISPATCH
            || variable == MOVE_TO_XY
            || variable == VAR_PARAMETER;
        if !supported {
            return server.write_error_status_cmd(
                CMD_SET_VEHICLE_VARIABLE,
                &format!("Change Vehicle State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        let id = inputStorage.read_string().unwrap();
        let shouldExist = variable != ADD && variable != ADD_FULL;
        let sumoVehicle = MSNet::get_instance().m().get_vehicle_control().get_vehicle(&id);
        if sumoVehicle.is_null() {
            if shouldExist {
                return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, &format!("Vehicle '{}' is not known", id), outputStorage);
            }
        }
        // shortcut: dynamic_cast<MSBaseVehicle*> is not checked at runtime, the vehicle control only holds base vehicles here
        let v: Ptr<MSBaseVehicle> = sumoVehicle.cast::<MSBaseVehicle>();
        if v.is_null() && shouldExist {
            return server.write_error_status_cmd(CMD_GET_VEHICLE_VARIABLE, &format!("Vehicle '{}' is not a proper vehicle", id), outputStorage);
        }
        match variable {
            CMD_STOP => {
                let compoundSize = StoHelp::read_compound(inputStorage, -1, "Stop needs a compound object description.");
                if compoundSize < 4 || compoundSize > 7 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Stop needs a compound object description of four to seven items.", outputStorage);
                }
                let edgeID = StoHelp::read_typed_string(inputStorage, "The first stop parameter must be the edge id given as a string.");
                let pos = StoHelp::read_typed_double(inputStorage, "The second stop parameter must be the end position along the edge given as a double.");
                let laneIndex = StoHelp::read_typed_byte(inputStorage, "The third stop parameter must be the lane index given as a byte.");
                let duration = StoHelp::read_typed_double(inputStorage, "The fourth stop parameter must be the stopping duration given as a double.");
                let mut stopFlags: i32 = 0;
                if compoundSize >= 5 {
                    stopFlags = StoHelp::read_typed_byte(inputStorage, "The fifth stop parameter must be a byte indicating its parking/triggered status.");
                }
                let mut startPos: f64 = INVALID_DOUBLE_VALUE;
                if compoundSize >= 6 {
                    startPos = StoHelp::read_typed_double(inputStorage, "The sixth stop parameter must be the start position along the edge given as a double.");
                }
                let mut until: f64 = INVALID_DOUBLE_VALUE;
                if compoundSize >= 7 {
                    until = StoHelp::read_typed_double(inputStorage, "The seventh stop parameter must be the minimum departure time given as a double.");
                }
                Vehicle::set_stop(&id, &edgeID, pos, laneIndex, duration, stopFlags, startPos, until);
            }
            CMD_REPLACE_STOP => {
                if !Self::insert_replace_stop(server, inputStorage, outputStorage, &id, true) {
                    return false;
                }
            }
            CMD_INSERT_STOP => {
                if !Self::insert_replace_stop(server, inputStorage, outputStorage, &id, false) {
                    return false;
                }
            }
            VAR_STOP_PARAMETER => {
                let compoundSize = StoHelp::read_compound(inputStorage, -1, "Setting stop parameter needs a compound object description.");
                if compoundSize != 3 && compoundSize != 4 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting a stop parameter needs a compound object description of 3 or 4 items.", outputStorage);
                }
                let nextStopIndex = StoHelp::read_typed_int(inputStorage, "The first setStopParameter parameter must be the nextStopIndex given as an integer.");
                let param = StoHelp::read_typed_string(inputStorage, "The second setStopParameter parameter must be the param given as a string.");
                let value = StoHelp::read_typed_string(inputStorage, "The third setStopParameter parameter must be the value given as a string.");
                let mut customParam: i32 = 0;
                if compoundSize == 4 {
                    customParam = StoHelp::read_typed_byte(inputStorage, "The fourth setStopParameter parameter must be the customParam flag given as a byte.");
                }
                Vehicle::set_stop_parameter(&id, nextStopIndex, &param, &value, customParam != 0);
            }
            CMD_REROUTE_TO_PARKING => {
                StoHelp::read_compound(inputStorage, 1, "Reroute to stop needs a compound object description of 1 item.");
                Vehicle::reroute_parking_area(&id, &StoHelp::read_typed_string(inputStorage, "The first reroute to stop parameter must be the parking area id given as a string."));
            }
            CMD_RESUME => {
                StoHelp::read_compound(inputStorage, 0, "Resuming requires an empty compound object.");
                Vehicle::resume(&id);
            }
            CMD_CHANGELANE => {
                let compoundSize = StoHelp::read_compound(inputStorage, -1, "Lane change needs a compound object description.");
                if compoundSize != 3 && compoundSize != 2 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Lane change needs a compound object description of two or three items.", outputStorage);
                }
                let laneIndex = StoHelp::read_typed_byte(inputStorage, "The first lane change parameter must be the lane index given as a byte.");
                let duration = StoHelp::read_typed_double(inputStorage, "The second lane change parameter must be the duration given as a double.");
                let mut relative: i32 = 0;
                if compoundSize == 3 {
                    relative = StoHelp::read_typed_byte(inputStorage, "The third lane change parameter must be a Byte for defining whether a relative lane change should be applied.");
                }

                if (laneIndex < 0 || laneIndex >= v.get_edge().get_lanes().len() as i32) && relative < 1 {
                    return server.write_error_status_cmd(
                        CMD_SET_VEHICLE_VARIABLE,
                        &format!("No lane with index '{}' on road '{}'.", laneIndex, v.get_edge().get_id()),
                        outputStorage,
                    );
                }

                if relative < 1 {
                    Vehicle::change_lane(&id, laneIndex, duration);
                } else {
                    Vehicle::change_lane_relative(&id, laneIndex, duration);
                }
            }
            CMD_CHANGESUBLANE => {
                Vehicle::change_sublane(&id, StoHelp::read_typed_double(inputStorage, "Sublane-changing requires a double."));
            }
            CMD_SLOWDOWN => {
                StoHelp::read_compound(inputStorage, 2, "Slow down needs a compound object description of two items.");
                let newSpeed = StoHelp::read_typed_double(inputStorage, "The first slow down parameter must be the speed given as a double.");
                if newSpeed < 0.0 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Speed must not be negative", outputStorage);
                }
                let duration = StoHelp::read_typed_double(inputStorage, "The second slow down parameter must be the duration given as a double.");
                if duration < 0.0 || steps2time(MSNet::get_instance().get_current_time_step()) + duration > steps2time(SUMOTIME_MAX - delta_t()) {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid time interval", outputStorage);
                }
                Vehicle::slow_down(&id, newSpeed, duration);
            }
            CMD_CHANGETARGET => {
                Vehicle::change_target(&id, &StoHelp::read_typed_string(inputStorage, "Change target requires a string containing the id of the new destination edge as parameter."));
            }
            CMD_OPENGAP => {
                let compoundSize = StoHelp::read_compound(inputStorage, -1, "Create gap needs a compound object description.");
                if compoundSize != 5 && compoundSize != 6 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Create gap needs a compound object description of five or six items.", outputStorage);
                }
                let newTimeHeadway = StoHelp::read_typed_double(inputStorage, "The first create gap parameter must be the new desired time headway (tau) given as a double.");
                let newSpaceHeadway = StoHelp::read_typed_double(inputStorage, "The second create gap parameter must be the new desired space headway given as a double.");
                let duration = StoHelp::read_typed_double(inputStorage, "The third create gap parameter must be the duration given as a double.");
                let changeRate = StoHelp::read_typed_double(inputStorage, "The fourth create gap parameter must be the change rate given as a double.");
                let maxDecel = StoHelp::read_typed_double(inputStorage, "The fifth create gap parameter must be the maximal braking rate given as a double.");

                if newTimeHeadway == -1.0 && newSpaceHeadway == -1.0 && duration == -1.0 && changeRate == -1.0 && maxDecel == -1.0 {
                    Vehicle::deactivate_gap_control(&id);
                } else {
                    if newTimeHeadway <= 0.0 {
                        if newTimeHeadway != -1.0 {
                            return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "The value for the new desired time headway (tau) must be positive for create gap", outputStorage);
                        }
                    }
                    if newSpaceHeadway < 0.0 {
                        return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "The value for the new desired space headway must be non-negative for create gap", outputStorage);
                    }
                    if (duration < 0.0 && duration != -1.0) || steps2time(MSNet::get_instance().get_current_time_step()) + duration > steps2time(SUMOTIME_MAX - delta_t()) {
                        return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid time interval for create gap", outputStorage);
                    }
                    if changeRate <= 0.0 {
                        return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "The value for the change rate must be positive for the openGap command", outputStorage);
                    }
                    if maxDecel <= 0.0 && maxDecel != -1.0 && maxDecel != INVALID_DOUBLE_VALUE {
                        return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "The value for the maximal braking rate must be positive for the openGap command", outputStorage);
                    }
                    let mut refVehID = String::new();
                    if compoundSize == 6 {
                        refVehID = StoHelp::read_typed_string(inputStorage, "The sixth create gap parameter must be a reference vehicle's ID given as a string.");
                    }
                    Vehicle::open_gap(&id, newTimeHeadway, newSpaceHeadway, duration, changeRate, maxDecel, &refVehID);
                }
            }
            VAR_TYPE => {
                Vehicle::set_type(&id, &StoHelp::read_typed_string(inputStorage, "The vehicle type id must be given as a string."));
            }
            VAR_ROUTE_ID => {
                Vehicle::set_route_id(&id, &StoHelp::read_typed_string(inputStorage, "The route id must be given as a string."));
            }
            VAR_ROUTE => {
                Vehicle::set_route_2(&id, &StoHelp::read_typed_string_list(inputStorage, "A route must be defined as a list of edge ids."));
            }
            VAR_EDGE_TRAVELTIME => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "Setting travel time requires a compound object.");
                let mut edgeID = String::new();
                let mut begTime: f64 = 0.0;
                let mut endTime: f64 = f64::MAX;
                let mut value: f64 = INVALID_DOUBLE_VALUE;
                if parameterCount == 4 {
                    begTime = StoHelp::read_typed_double(inputStorage, "Setting travel time using 4 parameters requires the begin time as first parameter.");
                    endTime = StoHelp::read_typed_double(inputStorage, "Setting travel time using 4 parameters requires the end time as second parameter.");
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting travel time using 4 parameters requires the referenced edge as third parameter.");
                    value = StoHelp::read_typed_double(inputStorage, "Setting travel time using 4 parameters requires the travel time as double as fourth parameter.");
                } else if parameterCount == 2 {
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting travel time using 2 parameters requires the referenced edge as first parameter.");
                    value = StoHelp::read_typed_double(inputStorage, "Setting travel time using 2 parameters requires the travel time as double as second parameter.");
                } else if parameterCount == 1 {
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting travel time using 1 parameter requires the referenced edge as first parameter.");
                } else {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting travel time requires 1, 2, or 4 parameters.", outputStorage);
                }
                Vehicle::set_adapted_traveltime(&id, &edgeID, value, begTime, endTime);
            }
            VAR_EDGE_EFFORT => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "Setting travel time requires a compound object.");
                let mut edgeID = String::new();
                let mut begTime: f64 = 0.0;
                let mut endTime: f64 = f64::MAX;
                let mut value: f64 = INVALID_DOUBLE_VALUE;
                if parameterCount == 4 {
                    begTime = StoHelp::read_typed_double(inputStorage, "Setting effort using 4 parameters requires the begin time as first parameter.");
                    endTime = StoHelp::read_typed_double(inputStorage, "Setting effort using 4 parameters requires the end time as second parameter.");
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting effort using 4 parameters requires the referenced edge as third parameter.");
                    value = StoHelp::read_typed_double(inputStorage, "Setting effort using 4 parameters requires the effort as double as fourth parameter.");
                } else if parameterCount == 2 {
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting effort using 2 parameters requires the referenced edge as first parameter.");
                    value = StoHelp::read_typed_double(inputStorage, "Setting effort using 2 parameters requires the effort as double as second parameter.");
                } else if parameterCount == 1 {
                    edgeID = StoHelp::read_typed_string(inputStorage, "Setting effort using 1 parameter requires the referenced edge as first parameter.");
                } else {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting effort requires 1, 2, or 4 parameters.", outputStorage);
                }
                Vehicle::set_effort(&id, &edgeID, value, begTime, endTime);
            }
            CMD_REROUTE_TRAVELTIME => {
                StoHelp::read_compound(inputStorage, 0, "Rerouting by travel time requires an empty compound object.");
                Vehicle::reroute_traveltime(&id, false);
            }
            CMD_REROUTE_EFFORT => {
                StoHelp::read_compound(inputStorage, 0, "Rerouting by effort requires an empty compound object.");
                Vehicle::reroute_effort(&id);
            }
            VAR_SIGNALS => {
                Vehicle::set_signals(&id, StoHelp::read_typed_int(inputStorage, "Setting signals requires an integer."));
            }
            VAR_MOVE_TO => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "Setting position requires a compound object.");
                if parameterCount < 2 || parameterCount > 3 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting position should obtain the lane id and the position and optionally the reason.", outputStorage);
                }
                let laneID = StoHelp::read_typed_string(inputStorage, "The first parameter for setting a position must be the lane ID given as a string.");
                let position = StoHelp::read_typed_double(inputStorage, "The second parameter for setting a position must be the position given as a double.");
                let mut reason: i32 = MOVE_AUTOMATIC;
                if parameterCount == 3 {
                    reason = StoHelp::read_typed_int(inputStorage, "The third parameter for setting a position must be the reason given as an int.");
                }
                Vehicle::move_to(&id, &laneID, position, reason);
            }
            VAR_IMPATIENCE => {
                Vehicle::set_impatience(&id, StoHelp::read_typed_double(inputStorage, "Setting impatience requires a double."));
            }
            VAR_SPEED => {
                Vehicle::set_speed(&id, StoHelp::read_typed_double(inputStorage, "Setting speed requires a double."));
            }
            VAR_ACCELERATION => {
                StoHelp::read_compound(inputStorage, 2, "Setting acceleration requires 2 parameters.");
                let accel = StoHelp::read_typed_double(inputStorage, "Setting acceleration requires the acceleration as first parameter given as a double.");
                let duration = StoHelp::read_typed_double(inputStorage, "Setting acceleration requires the duration as second parameter given as a double.");
                if duration < 0.0 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Duration must not be negative.", outputStorage);
                }
                Vehicle::set_acceleration(&id, accel, duration);
            }
            VAR_PREV_SPEED => {
                let mut prevSpeed: f64 = 0.0;
                let mut prevAcceleration: f64 = INVALID_DOUBLE_VALUE;
                let inputtype = inputStorage.read_unsigned_byte().unwrap();
                if inputtype == TYPE_COMPOUND {
                    let parameterCount = inputStorage.read_int().unwrap();
                    if parameterCount == 2 {
                        prevSpeed = StoHelp::read_typed_double(inputStorage, "Setting previous speed using 2 parameters requires the previous speed as first parameter given as a double.");
                        prevAcceleration = StoHelp::read_typed_double(inputStorage, "Setting previous speed using 2 parameters requires the previous acceleration as second parameter given as a double.");
                    } else if parameterCount == 1 {
                        prevSpeed = StoHelp::read_typed_double(inputStorage, "Setting previous speed using 1 parameter requires the previous speed as first parameter given as a double.");
                    } else {
                        return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting previous speed requires 1 or 2 parameters.", outputStorage);
                    }
                } else if inputtype == TYPE_DOUBLE {
                    prevSpeed = inputStorage.read_double().unwrap();
                } else {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Setting previous speed requires 1 parameter given as a double or 2 parameters as compound object description.", outputStorage);
                }
                if prevSpeed < 0.0 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Previous speed must not be negative.", outputStorage);
                }
                Vehicle::set_previous_speed(&id, prevSpeed, prevAcceleration);
            }
            VAR_SPEEDSETMODE => {
                Vehicle::set_speed_mode(&id, StoHelp::read_typed_int(inputStorage, "Setting speed mode requires an integer."));
            }
            VAR_LANECHANGE_MODE => {
                Vehicle::set_lane_change_mode(&id, StoHelp::read_typed_int(inputStorage, "Setting lane change mode requires an integer."));
            }
            VAR_ROUTING_MODE => {
                Vehicle::set_routing_mode(&id, StoHelp::read_typed_int(inputStorage, "Setting routing mode requires an integer."));
            }
            VAR_COLOR => {
                Vehicle::set_color(&id, &StoHelp::read_typed_color(inputStorage, "The color must be given using the according type."));
            }
            ADD => {
                StoHelp::read_compound(inputStorage, 6, "Adding a vehicle needs six parameters.");
                let vTypeID = StoHelp::read_typed_string(inputStorage, "First parameter (type) requires a string.");
                let routeID = StoHelp::read_typed_string(inputStorage, "Second parameter (route) requires a string.");
                let departCode = StoHelp::read_typed_int(inputStorage, "Third parameter (depart) requires an integer.");
                let mut depart = to_string(&steps2time(departCode as i64));
                if -departCode == DepartDefinition::TRIGGERED as i32 {
                    depart = "triggered".to_string();
                } else if -departCode == DepartDefinition::CONTAINER_TRIGGERED as i32 {
                    depart = "containerTriggered".to_string();
                } else if -departCode == DepartDefinition::NOW as i32 {
                    depart = "now".to_string();
                } else if -departCode == DepartDefinition::SPLIT as i32 {
                    depart = "split".to_string();
                } else if -departCode == DepartDefinition::BEGIN as i32 {
                    depart = "begin".to_string();
                }

                let departPosCode = StoHelp::read_typed_double(inputStorage, "Fourth parameter (position) requires a double.");
                let mut departPos = to_string(&departPosCode);
                if -departPosCode == DepartPosDefinition::RANDOM as i64 as f64 {
                    departPos = "random".to_string();
                } else if -departPosCode == DepartPosDefinition::RANDOM_FREE as i64 as f64 {
                    departPos = "random_free".to_string();
                } else if -departPosCode == DepartPosDefinition::FREE as i64 as f64 {
                    departPos = "free".to_string();
                } else if -departPosCode == DepartPosDefinition::BASE as i64 as f64 {
                    departPos = "base".to_string();
                } else if -departPosCode == DepartPosDefinition::LAST as i64 as f64 {
                    departPos = "last".to_string();
                } else if -departPosCode == DepartPosDefinition::GIVEN as i64 as f64 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid departure position.", outputStorage);
                }

                let departSpeedCode = StoHelp::read_typed_double(inputStorage, "Fifth parameter (speed) requires a double.");
                let mut departSpeed = to_string(&departSpeedCode);
                if -departSpeedCode == DepartSpeedDefinition::RANDOM as i64 as f64 {
                    departSpeed = "random".to_string();
                } else if -departSpeedCode == DepartSpeedDefinition::MAX as i64 as f64 {
                    departSpeed = "max".to_string();
                } else if -departSpeedCode == DepartSpeedDefinition::DESIRED as i64 as f64 {
                    departSpeed = "desired".to_string();
                } else if -departSpeedCode == DepartSpeedDefinition::LIMIT as i64 as f64 {
                    departSpeed = "speedLimit".to_string();
                } else if -departSpeedCode == DepartSpeedDefinition::LAST as i64 as f64 {
                    departSpeed = "last".to_string();
                } else if -departSpeedCode == DepartSpeedDefinition::AVG as i64 as f64 {
                    departSpeed = "avg".to_string();
                }

                let departLaneCode = StoHelp::read_typed_byte(inputStorage, "Sixth parameter (lane) requires a byte.");
                let mut departLane = departLaneCode.to_string();
                if -departLaneCode == DepartLaneDefinition::RANDOM as i32 {
                    departLane = "random".to_string();
                } else if -departLaneCode == DepartLaneDefinition::FREE as i32 {
                    departLane = "free".to_string();
                } else if -departLaneCode == DepartLaneDefinition::ALLOWED_FREE as i32 {
                    departLane = "allowed".to_string();
                } else if -departLaneCode == DepartLaneDefinition::BEST_FREE as i32 {
                    departLane = "best".to_string();
                } else if -departLaneCode == DepartLaneDefinition::FIRST_ALLOWED as i32 {
                    departLane = "first".to_string();
                }
                Vehicle::add(&id, &routeID, &vTypeID, &depart, &departLane, &departPos, &departSpeed, "current", "max", "current", "", "", "", 4, 0, "", "", -1.0, "");
            }
            ADD_FULL => {
                // Release 1.28.0 requires exactly fourteen parameters. The
                // fourteen-to-eighteen extension is snapshot-only.
                let _numElements = StoHelp::read_compound(inputStorage, 14, "Adding a fully specified vehicle needs fourteen parameters.");
                let routeID = StoHelp::read_typed_string(inputStorage, "First parameter (route) requires a string.");
                let vTypeID = StoHelp::read_typed_string(inputStorage, "Second parameter (type) requires a string.");
                let depart = StoHelp::read_typed_string(inputStorage, "Third parameter (depart) requires an string.");
                let departLane = StoHelp::read_typed_string(inputStorage, "Fourth parameter (depart lane) requires a string.");
                let departPos = StoHelp::read_typed_string(inputStorage, "Fifth parameter (depart position) requires a string.");
                let departSpeed = StoHelp::read_typed_string(inputStorage, "Sixth parameter (depart speed) requires a string.");
                let arrivalLane = StoHelp::read_typed_string(inputStorage, "Seventh parameter (arrival lane) requires a string.");
                let arrivalPos = StoHelp::read_typed_string(inputStorage, "Eighth parameter (arrival position) requires a string.");
                let arrivalSpeed = StoHelp::read_typed_string(inputStorage, "Ninth parameter (arrival speed) requires a string.");
                let fromTaz = StoHelp::read_typed_string(inputStorage, "Tenth parameter (from taz) requires a string.");
                let toTaz = StoHelp::read_typed_string(inputStorage, "Eleventh parameter (to taz) requires a string.");
                let line = StoHelp::read_typed_string(inputStorage, "Twelth parameter (line) requires a string.");
                let personCapacity = StoHelp::read_typed_int(inputStorage, "13th parameter (person capacity) requires an int.");
                let personNumber = StoHelp::read_typed_int(inputStorage, "14th parameter (person number) requires an int.");
                Vehicle::add(
                    &id,
                    &routeID,
                    &vTypeID,
                    &depart,
                    &departLane,
                    &departPos,
                    &departSpeed,
                    &arrivalLane,
                    &arrivalPos,
                    &arrivalSpeed,
                    &fromTaz,
                    &toTaz,
                    &line,
                    personCapacity,
                    personNumber,
                    "",
                    "",
                    -1.0,
                    "",
                );
            }
            REMOVE => {
                Vehicle::remove(&id, StoHelp::read_typed_byte(inputStorage, "Removing a vehicle requires a byte.") as i8 as i32);
            }
            MOVE_TO_XY => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "MoveToXY vehicle requires a compound object.");
                if parameterCount < 5 || parameterCount > 7 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "MoveToXY vehicle should obtain: edgeID, lane, x, y, angle and optionally keepRouteFlag and matchThreshold.", outputStorage);
                }
                let edgeID = StoHelp::read_typed_string(inputStorage, "The first parameter for moveToXY must be the edge ID given as a string.");
                let laneIndex = StoHelp::read_typed_int(inputStorage, "The second parameter for moveToXY must be lane given as an int.");
                let x = StoHelp::read_typed_double(inputStorage, "The third parameter for moveToXY must be the x-position given as a double.");
                let y = StoHelp::read_typed_double(inputStorage, "The fourth parameter for moveToXY must be the y-position given as a double.");
                let angle = StoHelp::read_typed_double(inputStorage, "The fifth parameter for moveToXY must be the angle given as a double.");
                let mut keepRouteFlag: i32 = 1;
                if parameterCount >= 6 {
                    keepRouteFlag = StoHelp::read_typed_byte(inputStorage, "The sixth parameter for moveToXY must be the keepRouteFlag given as a byte.");
                }
                let mut matchThreshold: f64 = 100.0;
                if parameterCount == 7 {
                    matchThreshold = StoHelp::read_typed_double(inputStorage, "The seventh parameter for moveToXY must be the matchThreshold given as a double.");
                }
                Vehicle::move_to_xy(&id, &edgeID, laneIndex, x, y, angle, keepRouteFlag, matchThreshold);
            }
            VAR_SPEED_FACTOR => {
                Vehicle::set_speed_factor(&id, StoHelp::read_typed_double(inputStorage, "Setting speed factor requires a double."));
            }
            VAR_LINE => {
                Vehicle::set_line(&id, &StoHelp::read_typed_string(inputStorage, "The line must be given as a string."));
            }
            VAR_VIA => {
                Vehicle::set_via(&id, &StoHelp::read_typed_string_list(inputStorage, "Vias must be defined as a list of edge ids."));
            }
            VAR_PARAMETER => {
                StoHelp::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StoHelp::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StoHelp::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Vehicle::set_parameter(&id, &name, &value);
            }
            VAR_HIGHLIGHT => {
                let parameterCount = StoHelp::read_compound(inputStorage, -1, "A compound object is needed for highlighting an object.");
                if parameterCount > 5 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Highlighting an object needs zero to five parameters.", outputStorage);
                }

                let mut col = TraCIColor::new_2(255, 0, 0, 255);
                if parameterCount > 0 {
                    col = StoHelp::read_typed_color(inputStorage, "The first parameter for highlighting must be the highlight color.");
                }
                let mut size: f64 = -1.0;
                if parameterCount > 1 {
                    size = StoHelp::read_typed_double(inputStorage, "The second parameter for highlighting must be the highlight size.");
                }
                let mut alphaMax: i32 = -1;
                if parameterCount > 2 {
                    alphaMax = StoHelp::read_typed_unsigned_byte(inputStorage, "The third parameter for highlighting must be maximal alpha.");
                }
                let mut duration: f64 = -1.0;
                if parameterCount > 3 {
                    duration = StoHelp::read_typed_double(inputStorage, "The fourth parameter for highlighting must be the highlight duration.");
                }
                let mut type_: i32 = 0;
                if parameterCount > 4 {
                    type_ = StoHelp::read_typed_unsigned_byte(inputStorage, "The fifth parameter for highlighting must be the highlight type id as ubyte.");
                }
                Vehicle::highlight(&id, &col, size, alphaMax, duration, type_);
            }
            CMD_TAXI_DISPATCH => {
                Vehicle::dispatch_taxi(&id, &StoHelp::read_typed_string_list(inputStorage, "A dispatch command  must be defined as a list of reservation ids."));
            }
            VAR_ACTIONSTEPLENGTH => {
                let value = StoHelp::read_typed_double(inputStorage, "Setting action step length requires a double.");
                if value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid action step length.", outputStorage);
                }
                let resetActionOffset = value >= 0.0;
                Vehicle::set_action_step_length(&id, value.abs(), resetActionOffset);
            }
            VAR_LANEPOSITION_LAT => {
                Vehicle::set_lateral_lane_position(&id, StoHelp::read_typed_double(inputStorage, "Setting lateral lane position requires a double."));
            }
            VAR_UPDATE_BESTLANES => {
                Vehicle::update_best_lanes(&id);
            }
            VAR_MINGAP => {
                let value = StoHelp::read_typed_double(inputStorage, "Setting minimum gap requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid minimum gap.", outputStorage);
                }
                Vehicle::set_min_gap(&id, value);
            }
            VAR_MINGAP_LAT => {
                let value = StoHelp::read_typed_double(inputStorage, "Setting minimum lateral gap requires a double.");
                if value < 0.0 || value.abs() == f64::INFINITY {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Invalid minimum lateral gap.", outputStorage);
                }
                Vehicle::set_min_gap_lat(&id, value);
            }
            _ => {
                // C++ inner `catch (ProcessError&)` and `catch (libsumo::TraCIException&)` from the vehicle type setter -> writeErrorStatusCmd
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| TraCIServerAPI_VehicleType::set_variable(CMD_SET_VEHICLE_VARIABLE, variable, v.m().get_singular_type().get_id(), server, inputStorage, outputStorage))) {
                    Ok(true) => {}
                    Ok(false) => return false,
                    Err(e) => return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, &crate::traci_server::tra_ci_server::panic_text(e), outputStorage),
                }
            }
        }
        server.write_status_cmd(CMD_SET_VEHICLE_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `bool TraCIServerAPI_Vehicle::insertReplaceStop(TraCIServer &, tcpip::Storage &, tcpip::Storage &, const std::string &, bool)` [static, private]
    // bool
    // TraCIServerAPI_Vehicle::insertReplaceStop(TraCIServer& server, tcpip::Storage& inputStorage, tcpip::Storage& outputStorage, const std::string& id, bool replace) {
    //     const std::string m1 = replace ? "Replacing" : "Inserting";
    //     const std::string m2 = replace ? "replacement" : "insertion";
    //     const int parameterCount = StoHelp::readCompound(inputStorage, -1, m1 + " stop needs a compound object description.");
    //     if (parameterCount != 8 && parameterCount != 9) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, m1 + " stop needs a compound object description of eight or nine items.", outputStorage);
    //     }
    //     // read road map position
    //     const std::string edgeID = StoHelp::readTypedString(inputStorage, "The first stop " + m2 + " parameter must be the edge id given as a string.");
    //     const double pos = StoHelp::readTypedDouble(inputStorage, "The second stop " + m2 + " parameter must be the end position along the edge given as a double.");
    //     const int laneIndex = StoHelp::readTypedByte(inputStorage, "The third stop " + m2 + " parameter must be the lane index given as a byte.");
    //     const double duration = StoHelp::readTypedDouble(inputStorage, "The fourth stop " + m2 + " parameter must be the stopping duration given as a double.");
    //     const int stopFlags = StoHelp::readTypedInt(inputStorage, "The fifth stop " + m2 + " parameter must be an int indicating its parking/triggered status.");
    //     const double startPos = StoHelp::readTypedDouble(inputStorage, "The sixth stop " + m2 + " parameter must be the start position along the edge given as a double.");
    //     const double until = StoHelp::readTypedDouble(inputStorage, "The seventh stop " + m2 + " parameter must be the minimum departure time given as a double.");
    //     const int nextStopIndex = StoHelp::readTypedInt(inputStorage, "The eighth stop " + m2 + " parameter must be the replacement index given as an int.");
    //     int teleport = 0;
    //     if (parameterCount == 9) {
    //         teleport = StoHelp::readTypedByte(inputStorage, "The ninth stop " + m2 + " parameter must be the teleport flag given as a byte.");
    //     }
    //     if (replace) {
    //         libsumo::Vehicle::replaceStop(id, nextStopIndex, edgeID, pos, laneIndex, duration, stopFlags, startPos, until, teleport);
    //     } else {
    //         libsumo::Vehicle::insertStop(id, nextStopIndex, edgeID, pos, laneIndex, duration, stopFlags, startPos, until, teleport);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert_replace_stop(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage, id: &str, replace: bool) -> bool {
        let m1 = if replace { "Replacing" } else { "Inserting" };
        let m2 = if replace { "replacement" } else { "insertion" };
        let parameterCount = StoHelp::read_compound(inputStorage, -1, &format!("{} stop needs a compound object description.", m1));
        if parameterCount != 8 && parameterCount != 9 {
            return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, &format!("{} stop needs a compound object description of eight or nine items.", m1), outputStorage);
        }
        // read road map position
        let edgeID = StoHelp::read_typed_string(inputStorage, &format!("The first stop {} parameter must be the edge id given as a string.", m2));
        let pos = StoHelp::read_typed_double(inputStorage, &format!("The second stop {} parameter must be the end position along the edge given as a double.", m2));
        let laneIndex = StoHelp::read_typed_byte(inputStorage, &format!("The third stop {} parameter must be the lane index given as a byte.", m2));
        let duration = StoHelp::read_typed_double(inputStorage, &format!("The fourth stop {} parameter must be the stopping duration given as a double.", m2));
        let stopFlags = StoHelp::read_typed_int(inputStorage, &format!("The fifth stop {} parameter must be an int indicating its parking/triggered status.", m2));
        let startPos = StoHelp::read_typed_double(inputStorage, &format!("The sixth stop {} parameter must be the start position along the edge given as a double.", m2));
        let until = StoHelp::read_typed_double(inputStorage, &format!("The seventh stop {} parameter must be the minimum departure time given as a double.", m2));
        let nextStopIndex = StoHelp::read_typed_int(inputStorage, &format!("The eighth stop {} parameter must be the replacement index given as an int.", m2));
        let mut teleport: i32 = 0;
        if parameterCount == 9 {
            teleport = StoHelp::read_typed_byte(inputStorage, &format!("The ninth stop {} parameter must be the teleport flag given as a byte.", m2));
        }
        if replace {
            Vehicle::replace_stop(id, nextStopIndex, &edgeID, pos, laneIndex, duration, stopFlags, startPos, until, teleport);
        } else {
            Vehicle::insert_stop(id, nextStopIndex, &edgeID, pos, laneIndex, duration, stopFlags, startPos, until, teleport);
        }
        true
    }

    /// C++: `void TraCIServerAPI_Vehicle::TraCIServerAPI_Vehicle(const TraCIServerAPI_Vehicle &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Vehicle) -> Self {
        panic!("TraCIServerAPI_Vehicle copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Vehicle & TraCIServerAPI_Vehicle::operator=(const TraCIServerAPI_Vehicle &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Vehicle) -> &mut TraCIServerAPI_Vehicle {
        panic!("TraCIServerAPI_Vehicle assignment operator is private and undefined in C++; it cannot be called")
    }

}

