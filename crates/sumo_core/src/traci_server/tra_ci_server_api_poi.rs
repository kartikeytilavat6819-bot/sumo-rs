//! Port of traci-server/TraCIServerAPI_POI.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// APIs for getting/setting POI values via TraCI
/// C++ class `TraCIServerAPI_POI` (traci-server/TraCIServerAPI_POI.h)
pub struct TraCIServerAPI_POI {
}

impl TraCIServerAPI_POI {
    /// C++: `bool TraCIServerAPI_POI::processSet(TraCIServer &, tcpip::Storage &, tcpip::Storage &)` [static]
    /// Processes a set value command (Command 0xc7: Change PoI State)
    // bool
    // TraCIServerAPI_POI::processSet(TraCIServer& server, tcpip::Storage& inputStorage,
    //                                tcpip::Storage& outputStorage) {
    //     std::string warning = ""; // additional description for response
    //     // variable & id
    //     int variable = inputStorage.readUnsignedByte();
    //     std::string id = inputStorage.readString();
    //     // check variable
    //     if (variable != libsumo::VAR_TYPE &&
    //             variable != libsumo::VAR_COLOR &&
    //             variable != libsumo::VAR_POSITION &&
    //             variable != libsumo::VAR_WIDTH &&
    //             variable != libsumo::VAR_HEIGHT &&
    //             variable != libsumo::VAR_ANGLE &&
    //             variable != libsumo::VAR_IMAGEFILE &&
    //             variable != libsumo::VAR_HIGHLIGHT &&
    //             variable != libsumo::ADD &&
    //             variable != libsumo::REMOVE &&
    //             variable != libsumo::VAR_PARAMETER) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, "Change PoI State: unsupported variable " + toHex(variable, 2) + " specified", outputStorage);
    //     }
    //     // process
    //     try {
    //         switch (variable) {
    //             case libsumo::VAR_TYPE: {
    //                 libsumo::POI::setType(id, StoHelp::readTypedString(inputStorage, "The type must be given as a string."));
    //             }
    //             break;
    //             case libsumo::VAR_COLOR: {
    //                 libsumo::POI::setColor(id, StoHelp::readTypedColor(inputStorage, "The color must be given using the according type."));
    //             }
    //             break;
    //             case libsumo::VAR_POSITION: {
    //                 const libsumo::TraCIPosition pos = StoHelp::readTypedPosition2D(inputStorage, "The position must be given using an according type.");
    //                 libsumo::POI::setPosition(id, pos.x, pos.y);
    //             }
    //             break;
    //             case libsumo::VAR_WIDTH: {
    //                 libsumo::POI::setWidth(id, StoHelp::readTypedDouble(inputStorage, "The width must be given as a double."));
    //             }
    //             break;
    //             case libsumo::VAR_HEIGHT: {
    //                 libsumo::POI::setHeight(id, StoHelp::readTypedDouble(inputStorage, "The height must be given as a double."));
    //             }
    //             break;
    //             case libsumo::VAR_ANGLE: {
    //                 libsumo::POI::setAngle(id, StoHelp::readTypedDouble(inputStorage, "The angle must be given as a double."));
    //             }
    //             break;
    //             case libsumo::VAR_IMAGEFILE: {
    //                 libsumo::POI::setImageFile(id, StoHelp::readTypedString(inputStorage, "The image file must be given as a string."));
    //             }
    //             break;
    //             case libsumo::VAR_HIGHLIGHT: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for highlighting an object.");
    //                 if (parameterCount > 5) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, "Highlighting an object needs zero to five parameters.", outputStorage);
    //                 }
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
    //                 libsumo::POI::highlight(id, col, size, alphaMax, duration, type);
    //             }
    //             break;
    //             case libsumo::ADD: {
    //                 const int parameterCount = StoHelp::readCompound(inputStorage, -1, "A compound object is needed for adding a new PoI.");
    //                 const std::string type = StoHelp::readTypedString(inputStorage, "The first PoI parameter must be the type encoded as a string.");
    //                 libsumo::TraCIColor col = StoHelp::readTypedColor(inputStorage, "The second PoI parameter must be the color.");
    //                 const int layer = StoHelp::readTypedInt(inputStorage, "The third PoI parameter must be the layer encoded as int.");
    //                 const libsumo::TraCIPosition pos = StoHelp::readTypedPosition2D(inputStorage, "The fourth PoI parameter must be the position.");
    //                 if (parameterCount == 4) {
    //                     if (!libsumo::POI::add(id, pos.x, pos.y, col, type, layer)) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, "Could not add PoI.", outputStorage);
    //                     }
    //                 } else if (parameterCount >= 8) {
    //                     const std::string imgFile = StoHelp::readTypedString(inputStorage, "The fifth PoI parameter must be the imgFile encoded as a string.");
    //                     const double width = StoHelp::readTypedDouble(inputStorage, "The sixth PoI parameter must be the width encoded as a double.");
    //                     const double height = StoHelp::readTypedDouble(inputStorage, "The seventh PoI parameter must be the height encoded as a double.");
    //                     const double angle = StoHelp::readTypedDouble(inputStorage, "The eighth PoI parameter must be the angle encoded as a double.");
    //                     std::string icon;
    //                     if (parameterCount == 9) {
    //                         icon = StoHelp::readTypedString(inputStorage, "The ninth PoI parameter must be the icon encoded as a string.");
    //                     }
    //                     if (!libsumo::POI::add(id, pos.x, pos.y, col, type, layer, imgFile, width, height, angle, icon)) {
    //                         return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, "Could not add PoI.", outputStorage);
    //                     }
    //                 } else {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE,
    //                                                       "Adding a PoI requires either only type, color, layer and position parameters or these and icon, imageFile, width, height and angle parameters.",
    //                                                       outputStorage);
    //                 }
    //             }
    //             break;
    //             case libsumo::REMOVE: {
    //                 const int layer = StoHelp::readTypedInt(inputStorage, "The layer must be given using an int.");
    //                 if (!libsumo::POI::remove(id, layer)) {
    //                     return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, "Could not remove PoI '" + id + "'", outputStorage);
    //                 }
    //             }
    //             break;
    //             case libsumo::VAR_PARAMETER: {
    //                 StoHelp::readCompound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
    //                 const std::string name = StoHelp::readTypedString(inputStorage, "The name of the parameter must be given as a string.");
    //                 const std::string value = StoHelp::readTypedString(inputStorage, "The value of the parameter must be given as a string.");
    //                 libsumo::POI::setParameter(id, name, value);
    //             }
    //             break;
    //             default:
    //                 break;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return server.writeErrorStatusCmd(libsumo::CMD_SET_POI_VARIABLE, e.what(), outputStorage);
    //     }
    //     server.writeStatusCmd(libsumo::CMD_SET_POI_VARIABLE, libsumo::RTYPE_OK, warning, outputStorage);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_set(server: &mut TraCIServer, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        let warning = String::new(); // additional description for response
        // variable & id
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        let id = inputStorage.read_string().expect("readString");
        // check variable
        if variable != VAR_TYPE && variable != VAR_COLOR && variable != VAR_POSITION && variable != VAR_WIDTH
            && variable != VAR_HEIGHT && variable != VAR_ANGLE && variable != VAR_IMAGEFILE && variable != VAR_HIGHLIGHT
            && variable != ADD && variable != REMOVE && variable != VAR_PARAMETER {
            return server.write_error_status_cmd(
                CMD_SET_POI_VARIABLE,
                &format!("Change PoI State: unsupported variable {} specified", to_hex(variable as u64, 2, 0)),
                outputStorage,
            );
        }
        // shortcut: libsumo has no TraCIException mapping yet, so the C++ catch (which writes an error status) is not reproduced; upgrade when libsumo gets an error convention
        // process
        match variable {
            VAR_TYPE => {
                let poiType = StorageHelper::read_typed_string(inputStorage, "The type must be given as a string.");
                POI::set_type(&id, &poiType);
            }
            VAR_COLOR => {
                let color = StorageHelper::read_typed_color(inputStorage, "The color must be given using the according type.");
                POI::set_color(&id, &color);
            }
            VAR_POSITION => {
                let pos = StorageHelper::read_typed_position2_d(inputStorage, "The position must be given using an according type.");
                POI::set_position(&id, pos.x, pos.y);
            }
            VAR_WIDTH => {
                let width = StorageHelper::read_typed_double(inputStorage, "The width must be given as a double.");
                POI::set_width(&id, width);
            }
            VAR_HEIGHT => {
                let height = StorageHelper::read_typed_double(inputStorage, "The height must be given as a double.");
                POI::set_height(&id, height);
            }
            VAR_ANGLE => {
                let angle = StorageHelper::read_typed_double(inputStorage, "The angle must be given as a double.");
                POI::set_angle(&id, angle);
            }
            VAR_IMAGEFILE => {
                let imageFile = StorageHelper::read_typed_string(inputStorage, "The image file must be given as a string.");
                POI::set_image_file(&id, &imageFile);
            }
            VAR_HIGHLIGHT => {
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for highlighting an object.");
                if parameterCount > 5 {
                    return server.write_error_status_cmd(CMD_SET_POI_VARIABLE, "Highlighting an object needs zero to five parameters.", outputStorage);
                }
                let mut col = TraCIColor { base: TraCIResult::new(), r: 255, g: 0, b: 0, a: 255 };
                if parameterCount > 0 {
                    col = StorageHelper::read_typed_color(inputStorage, "The first parameter for highlighting must be the highlight color.");
                }
                let mut size: f64 = -1.0;
                if parameterCount > 1 {
                    size = StorageHelper::read_typed_double(inputStorage, "The second parameter for highlighting must be the highlight size.");
                }
                let mut alphaMax: i32 = -1;
                if parameterCount > 2 {
                    alphaMax = StorageHelper::read_typed_unsigned_byte(inputStorage, "The third parameter for highlighting must be maximal alpha.");
                }
                let mut duration: f64 = -1.0;
                if parameterCount > 3 {
                    duration = StorageHelper::read_typed_double(inputStorage, "The fourth parameter for highlighting must be the highlight duration.");
                }
                let mut type_: i32 = 0;
                if parameterCount > 4 {
                    type_ = StorageHelper::read_typed_unsigned_byte(inputStorage, "The fifth parameter for highlighting must be the highlight type id as ubyte.");
                }
                POI::highlight(&id, &col, size, alphaMax, duration, type_);
            }
            ADD => {
                let parameterCount = StorageHelper::read_compound(inputStorage, -1, "A compound object is needed for adding a new PoI.");
                let poiType = StorageHelper::read_typed_string(inputStorage, "The first PoI parameter must be the type encoded as a string.");
                let col = StorageHelper::read_typed_color(inputStorage, "The second PoI parameter must be the color.");
                let layer = StorageHelper::read_typed_int(inputStorage, "The third PoI parameter must be the layer encoded as int.");
                let pos = StorageHelper::read_typed_position2_d(inputStorage, "The fourth PoI parameter must be the position.");
                if parameterCount == 4 {
                    if !POI::add(&id, pos.x, pos.y, &col, &poiType, layer, "", 1.0, 1.0, 0.0, "") {
                        return server.write_error_status_cmd(CMD_SET_POI_VARIABLE, "Could not add PoI.", outputStorage);
                    }
                } else if parameterCount >= 8 {
                    let imgFile = StorageHelper::read_typed_string(inputStorage, "The fifth PoI parameter must be the imgFile encoded as a string.");
                    let width = StorageHelper::read_typed_double(inputStorage, "The sixth PoI parameter must be the width encoded as a double.");
                    let height = StorageHelper::read_typed_double(inputStorage, "The seventh PoI parameter must be the height encoded as a double.");
                    let angle = StorageHelper::read_typed_double(inputStorage, "The eighth PoI parameter must be the angle encoded as a double.");
                    let mut icon = String::new();
                    if parameterCount == 9 {
                        icon = StorageHelper::read_typed_string(inputStorage, "The ninth PoI parameter must be the icon encoded as a string.");
                    }
                    if !POI::add(&id, pos.x, pos.y, &col, &poiType, layer, &imgFile, width, height, angle, &icon) {
                        return server.write_error_status_cmd(CMD_SET_POI_VARIABLE, "Could not add PoI.", outputStorage);
                    }
                } else {
                    return server.write_error_status_cmd(CMD_SET_POI_VARIABLE,
                        "Adding a PoI requires either only type, color, layer and position parameters or these and icon, imageFile, width, height and angle parameters.",
                        outputStorage);
                }
            }
            REMOVE => {
                let layer = StorageHelper::read_typed_int(inputStorage, "The layer must be given using an int.");
                if !POI::remove(&id, layer) {
                    return server.write_error_status_cmd(CMD_SET_POI_VARIABLE, &format!("Could not remove PoI '{}'", id), outputStorage);
                }
            }
            VAR_PARAMETER => {
                StorageHelper::read_compound(inputStorage, 2, "A compound object of size 2 is needed for setting a parameter.");
                let name = StorageHelper::read_typed_string(inputStorage, "The name of the parameter must be given as a string.");
                let value = StorageHelper::read_typed_string(inputStorage, "The value of the parameter must be given as a string.");
                POI::set_parameter(&id, &name, &value);
            }
            _ => {}
        }
        server.write_status_cmd(CMD_SET_POI_VARIABLE, RTYPE_OK, &warning, outputStorage);
        true
    }

    /// C++: `void TraCIServerAPI_POI::TraCIServerAPI_POI(const TraCIServerAPI_POI &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(s: &TraCIServerAPI_POI) -> Self {
        panic!("TraCIServerAPI_POI copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `TraCIServerAPI_POI & TraCIServerAPI_POI::operator=(const TraCIServerAPI_POI &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServerAPI_POI) -> &mut TraCIServerAPI_POI {
        panic!("TraCIServerAPI_POI assignment operator is private and undefined in C++; it cannot be called")
    }

}

