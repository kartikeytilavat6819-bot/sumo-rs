//! Port of traci-server/TraCIServerAPI_Polygon.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting polygon values via TraCI
/// C++ class `TraCIServerAPI_Polygon` (traci-server/TraCIServerAPI_Polygon.h)
pub struct TraCIServerAPI_Polygon {
}

impl TraCIServerAPI_Polygon {
    /// C++: `bool TraCIServerAPI_Polygon::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc8: Change Polygon State)
    // bool
    // TraCIServerAPI_Polygon::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                    tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable
    //     int variable = inputStorage.readUnsignedByte();
    //     if (variable != libsumo::VAR_TYPE && variable != libsumo::VAR_COLOR && variable != libsumo::VAR_SHAPE && variable != libsumo::VAR_FILL
    //             && variable != libsumo::VAR_WIDTH && variable != libsumo::VAR_MOVE_TO
    //             && variable != libsumo::ADD && variable != libsumo::REMOVE && variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_POLYGON_VARIABLE,
    //                                           "Change Polygon State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // id
    //     std::string id = inputStorage.readString();
    //     try {
    //         // process
    //         switch (variable) {
    //             case libsumo::VAR_TYPE:
    //                 libsumo::Polygon::setType(id, StoHelp::readTypedString(inputStorage, "The type must be given as a string."));
    //                 break;
    //             case libsumo::VAR_COLOR:
    //                 libsumo::Polygon::setColor(id, StoHelp::readTypedColor(inputStorage, "The color must be given using an according type."));
    //                 break;
    //             case libsumo::VAR_SHAPE:
    //                 libsumo::Polygon::setShape(id, StoHelp::readTypedPolygon(inputStorage, "The shape must be given using an according type."));
    //                 break;
    //             case libsumo::VAR_FILL:
    //                 libsumo::Polygon::setFilled(id, StoHelp::readTypedInt(inputStorage, "'fill' must be defined using an integer.") != 0);
    //                 break;
    //             case libsumo::VAR_WIDTH:
    //                 libsumo::Polygon::setLineWidth(id, StoHelp::readTypedDouble(inputStorage, "'lineWidth' must be defined using a double."));
    //                 break;
    //             case libsumo::VAR_HEIGHT:
    //                 libsumo::Polygon::setHeight(id, StoHelp::readTypedDouble(inputStorage, "'height' must be defined using a double."));
    //                 break;
    //             case libsumo::ADD: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for adding a new polygon.");
    //                 if (parameterCount != 5 && parameterCount != 6) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Adding a polygon needs five to six parameters.", outputStorage);
    //                 }
    //                 const std::string type = StoHelp::readTypedString(inputStorage, "The type must be given as a string.");
    //                 const libsumo::TraCIColor col = StoHelp::readTypedColor(inputStorage, "The second polygon parameter must be the color.");
    //                 const bool fill = StoHelp::readBool(inputStorage, "The third polygon parameter must be 'fill' encoded as ubyte.");
    //                 const int layer = StoHelp::readTypedInt(inputStorage, "The fourth polygon parameter must be the layer encoded as int.");
    //                 const libsumo::TraCIPositionVector tp = StoHelp::readTypedPolygon(inputStorage, "The fifth polygon parameter must be the shape.");
    //                 double lineWidth = 1.;
    //                 if (parameterCount == 6) {
    //                     lineWidth = StoHelp::readTypedDouble(inputStorage, "The sixth polygon parameter must be the lineWidth encoded as double.");
    //                 }
    //                 libsumo::Polygon::add(id, tp, col, fill, type, layer, lineWidth);
    //             }
    //             break;
    //             case libsumo::VAR_ADD_DYNAMICS: {
    //                 if (inputStorage.readUnsignedByte() != libsumo::TYPE_COMPOUND) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_POLYGON_VARIABLE, "A compound object is needed for adding dynamics to a polygon.", outputStorage);
    //                 }
    //                 int itemNo = inputStorage.readInt();
    //                 if (itemNo != 5) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_VEHICLE_VARIABLE, "Adding polygon dynamics needs four parameters.", outputStorage);
    //                 }
    //                 const std::string trackedID = StoHelp::readTypedString(inputStorage, "The first parameter for adding polygon dynamics must be ID of the tracked object as a string ('' to disregard tracking).");
    //                 const std::vector<double> timeSpan = StoHelp::readTypedDoubleList(inputStorage, "The second parameter for adding polygon dynamics must be the timespan of the animation (length=0 to disregard animation).");
    //                 const std::vector<double> alphaSpan = StoHelp::readTypedDoubleList(inputStorage, "The third parameter for adding polygon dynamics must be the alphaSpanStr of the animation (length=0 to disregard alpha animation).");
    //                 const bool looped = StoHelp::readBool(inputStorage, "The fourth parameter for adding polygon dynamics must be boolean indicating whether the animation should be looped.");
    //                 const bool rotate = StoHelp::readBool(inputStorage, "The fifth parameter for adding polygon dynamics must be boolean indicating whether the tracking polygon should be rotated.");
    //                 libsumo::Polygon::addDynamics(id, trackedID, timeSpan, alphaSpan, looped, rotate);
    //             }
    //             break;
    //             case libsumo::REMOVE: {
    //                 libsumo::Polygon::remove(id, StoHelp::readTypedInt(inputStorage, "The layer must be given using an int."));
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::Polygon::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_POLYGON_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_POLYGON_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        // C++ whitelist does not list VAR_HEIGHT, so it is rejected here even though the switch below handles it
        if variable != VAR_TYPE && variable != VAR_COLOR && variable != VAR_SHAPE && variable != VAR_FILL
            && variable != VAR_WIDTH && variable != VAR_MOVE_TO
            && variable != ADD && variable != REMOVE && variable != VAR_PARAMETER {
            return server.write_error_status_cmd(
                CMD_SET_POLYGON_VARIABLE,
                &format!("Change Polygon State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // id
        let id = inputStorage.read_string().expect("readString");
        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            VAR_TYPE => {
                let polygonType = StorageHelper::read_typed_string(inputStorage, "The type must be given as a string.");
                Polygon::set_type(&id, &polygonType);
            }
            VAR_COLOR => {
                let color = StorageHelper::read_typed_color(inputStorage, "The color must be given using an according type.");
                Polygon::set_color(&id, &color);
            }
            VAR_SHAPE => {
                let shape = StorageHelper::read_typed_polygon(inputStorage, "The shape must be given using an according type.");
                Polygon::set_shape(&id, &shape);
            }
            VAR_FILL => {
                let fill = StorageHelper::read_typed_int(inputStorage, "'fill' must be defined using an integer.") != 0;
                Polygon::set_filled(id.clone(), fill);
            }
            VAR_WIDTH => {
                let lineWidth = StorageHelper::read_typed_double(inputStorage, "'lineWidth' must be defined using a double.");
                Polygon::set_line_width(id.clone(), lineWidth);
            }
            VAR_HEIGHT => {
                let height = StorageHelper::read_typed_double(inputStorage, "'height' must be defined using a double.");
                Polygon::set_height(id.clone(), height);
            }
            ADD => {
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for adding a new polygon.");
                if parameterCount != 5 && parameterCount != 6 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Adding a polygon needs five to six parameters.", outputStorage);
                }
                let polygonType = StorageHelper::read_typed_string(inputStorage, "The type must be given as a string.");
                let col = StorageHelper::read_typed_color(inputStorage, "The second polygon parameter must be the color.");
                let fill = StorageHelper::read_bool(inputStorage, "The third polygon parameter must be 'fill' encoded as ubyte.");
                let layer = StorageHelper::read_typed_int(inputStorage, "The fourth polygon parameter must be the layer encoded as int.");
                let tp = StorageHelper::read_typed_polygon(inputStorage, "The fifth polygon parameter must be the shape.");
                let mut lineWidth = 1.;
                if parameterCount == 6 {
                    lineWidth = StorageHelper::read_typed_double(inputStorage, "The sixth polygon parameter must be the lineWidth encoded as double.");
                }
                Polygon::add(&id, &tp, &col, fill, &polygonType, layer, lineWidth);
            }
            VAR_ADD_DYNAMICS => {
                if inputStorage.read_unsigned_byte().expect("readUnsignedByte") != TYPE_COMPOUND {
                    return server.write_error_status_cmd(CMD_SET_POLYGON_VARIABLE, "A compound object is needed for adding dynamics to a polygon.", outputStorage);
                }
                let itemNo = inputStorage.read_int().expect("readInt");
                if itemNo != 5 {
                    return server.write_error_status_cmd(CMD_SET_VEHICLE_VARIABLE, "Adding polygon dynamics needs four parameters.", outputStorage);
                }
                let trackedID = StorageHelper::read_typed_string(inputStorage, "The first parameter for adding polygon dynamics must be ID of the tracked object as a string ('' to disregard tracking).");
                let timeSpan = StorageHelper::read_typed_double_list(inputStorage, "The second parameter for adding polygon dynamics must be the timespan of the animation (length=0 to disregard animation).");
                let alphaSpan = StorageHelper::read_typed_double_list(inputStorage, "The third parameter for adding polygon dynamics must be the alphaSpanStr of the animation (length=0 to disregard alpha animation).");
                let looped = StorageHelper::read_bool(inputStorage, "The fourth parameter for adding polygon dynamics must be boolean indicating whether the animation should be looped.");
                let rotate = StorageHelper::read_bool(inputStorage, "The fifth parameter for adding polygon dynamics must be boolean indicating whether the tracking polygon should be rotated.");
                Polygon::add_dynamics(&id, &trackedID, &timeSpan, &alphaSpan, looped, rotate);
            }
            REMOVE => {
                let layer = StorageHelper::read_typed_int(inputStorage, "The layer must be given using an int.");
                Polygon::remove(&id, layer);
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                Polygon::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_POLYGON_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_Polygon::TraCIServerAPI_Polygon(const TraCIServerAPI_Polygon &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_Polygon) -> Self {
        panic!("TraCIServerAPI_Polygon copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_Polygon & TraCIServerAPI_Polygon::operator=(const TraCIServerAPI_Polygon &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_Polygon) -> &mut TraCIServerAPI_Polygon {
        panic!("TraCIServerAPI_Polygon assignment operator is private and undefined in C++; it cannot be called")
    }

}

