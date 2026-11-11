//! Port of libsumo/StorageHelper.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `libsumo::StorageHelper StoHelp`
pub type StoHelp = StorageHelper;

/// C++ class `libsumo::StorageHelper` (libsumo/StorageHelper.h)
pub struct StorageHelper {
}

impl StorageHelper {
    /// C++: `std::shared_ptr<tcpip::Storage> StorageHelper::toStorage(const TraCIResult &)` [static]
    // static inline std::shared_ptr<tcpip::Storage> toStorage(const TraCIResult& v) {
    //         std::shared_ptr<tcpip::Storage> result = std::make_shared<tcpip::Storage>();
    //         if (v.getType() == POSITION_ROADMAP || v.getType() == POSITION_2D || v.getType() == POSITION_3D) {
    //             writeCompound(*result, 2);
    //         }
    //         if (v.getType() != -1) {
    //             result->writeUnsignedByte(v.getType());
    //         }
    //         switch (v.getType()) {
    //             case TYPE_STRING:
    //                 result->writeString(v.getString());
    //                 break;
    //             case TYPE_DOUBLE:
    //                 result->writeDouble(((const TraCIDouble&)v).value);
    //                 break;
    //             case TYPE_INTEGER:
    //                 result->writeInt(((const TraCIInt&)v).value);
    //                 break;
    //             case TYPE_BYTE:
    //                 result->writeByte(((const TraCIInt&)v).value);
    //                 break;
    //             case TYPE_UBYTE:
    //                 result->writeUnsignedByte(((const TraCIInt&)v).value);
    //                 break;
    //             case POSITION_ROADMAP:
    //                 result->writeString(((const TraCIRoadPosition&)v).edgeID);
    //                 result->writeDouble(((const TraCIRoadPosition&)v).pos);
    //                 result->writeUnsignedByte(((const TraCIRoadPosition&)v).laneIndex);
    //                 break;
    //             case POSITION_2D:
    //                 result->writeDouble(((const TraCIPosition&)v).x);
    //                 result->writeDouble(((const TraCIPosition&)v).y);
    //                 break;
    //             case POSITION_3D:
    //                 result->writeDouble(((const TraCIPosition&)v).x);
    //                 result->writeDouble(((const TraCIPosition&)v).y);
    //                 result->writeDouble(((const TraCIPosition&)v).z);
    //                 break;
    //             case -1: {
    //                 // a hack for transfering multiple values
    //                 const auto& pl = ((const TraCIStringDoublePairList&)v).value;
    //                 const bool tisb = pl.size() == 2 && pl[0].first != "";
    //                 writeCompound(*result, pl.size() == 2 && !tisb ? 2 : (int)pl.size() + 1);
    //                 if (pl.size() == 1) {
    //                     writeTypedDouble(*result, pl.front().second);
    //                     writeTypedString(*result, pl.front().first);
    //                 } else if (pl.size() == 2) {
    //                     if (tisb) {
    //                         writeTypedInt(*result, (int)(pl.front().second + 0.5));
    //                         writeTypedString(*result, pl.front().first);
    //                         writeTypedByte(*result, (int)(pl.back().second + 0.5));
    //                     } else {
    //                         writeTypedDouble(*result, pl.front().second);
    //                         writeTypedDouble(*result, pl.back().second);
    //                     }
    //                 } else if (pl.size() == 3) {
    //                     writeTypedDouble(*result, pl[0].second);
    //                     writeTypedDouble(*result, pl[1].second);
    //                     writeTypedDouble(*result, pl[2].second);
    //                     writeTypedString(*result, pl[2].first);
    //                 } else if (pl.size() == 4) {
    //                     writeTypedDouble(*result, pl[0].second);
    //                     writeTypedDouble(*result, pl[1].second);
    //                     writeTypedDouble(*result, pl[2].second);
    //                     writeTypedDouble(*result, pl[3].second);
    //                     writeTypedString(*result, pl[3].first);
    //                 }
    //                 break;
    //             }
    //             default:
    //                 throw TraCIException("Unknown type " + toHex(v.getType()));
    //         }
    //         if (v.getType() == POSITION_ROADMAP || v.getType() == POSITION_2D || v.getType() == POSITION_3D) {
    //             result->writeUnsignedByte(REQUEST_DRIVINGDIST);
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn to_storage(v: &TraCIResult) -> Rc<Storage> {
        todo!("port StorageHelper::toStorage")
    }

    /// C++: `int StorageHelper::readTypedInt(tcpip::Storage &, const std::string &)` [static]
    // static inline int readTypedInt(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_INTEGER && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readInt();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_int(ret: &mut Storage, error: &str) -> i32 {
        todo!("port StorageHelper::readTypedInt")
    }

    /// C++: `int StorageHelper::readTypedByte(tcpip::Storage &, const std::string &)` [static]
    // static inline int readTypedByte(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_BYTE && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readByte();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_byte(ret: &mut Storage, error: &str) -> i32 {
        todo!("port StorageHelper::readTypedByte")
    }

    /// C++: `int StorageHelper::readTypedUnsignedByte(tcpip::Storage &, const std::string &)` [static]
    // static inline int readTypedUnsignedByte(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_UBYTE && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readUnsignedByte();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_unsigned_byte(ret: &mut Storage, error: &str) -> i32 {
        todo!("port StorageHelper::readTypedUnsignedByte")
    }

    /// C++: `double StorageHelper::readTypedDouble(tcpip::Storage &, const std::string &)` [static]
    // static inline double readTypedDouble(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_DOUBLE && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readDouble();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_double(ret: &mut Storage, error: &str) -> f64 {
        todo!("port StorageHelper::readTypedDouble")
    }

    /// C++: `std::vector<double> StorageHelper::readTypedDoubleList(tcpip::Storage &, const std::string &)` [static]
    // static inline std::vector<double> readTypedDoubleList(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_DOUBLELIST && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readDoubleList();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_double_list(ret: &mut Storage, error: &str) -> Vec<f64> {
        todo!("port StorageHelper::readTypedDoubleList")
    }

    /// C++: `std::string StorageHelper::readTypedString(tcpip::Storage &, const std::string &)` [static]
    // static inline std::string readTypedString(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_STRING && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readString();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_string(ret: &mut Storage, error: &str) -> String {
        todo!("port StorageHelper::readTypedString")
    }

    /// C++: `std::vector<std::string> StorageHelper::readTypedStringList(tcpip::Storage &, const std::string &)` [static]
    // static inline std::vector<std::string> readTypedStringList(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_STRINGLIST && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readStringList();
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_string_list(ret: &mut Storage, error: &str) -> Vec<String> {
        todo!("port StorageHelper::readTypedStringList")
    }

    /// C++: `const libsumo::TraCIColor StorageHelper::readTypedColor(tcpip::Storage &, const std::string &)` [static]
    // static inline const libsumo::TraCIColor readTypedColor(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_COLOR && error != "") {
    //             throw TraCIException(error);
    //         }
    //         libsumo::TraCIColor into;
    //         into.r = static_cast<unsigned char>(ret.readUnsignedByte());
    //         into.g = static_cast<unsigned char>(ret.readUnsignedByte());
    //         into.b = static_cast<unsigned char>(ret.readUnsignedByte());
    //         into.a = static_cast<unsigned char>(ret.readUnsignedByte());
    //         return into;
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_color(ret: &mut Storage, error: &str) -> TraCIColor {
        todo!("port StorageHelper::readTypedColor")
    }

    /// C++: `const libsumo::TraCIPosition StorageHelper::readTypedPosition2D(tcpip::Storage &, const std::string &)` [static]
    // static inline const libsumo::TraCIPosition readTypedPosition2D(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::POSITION_2D && error != "") {
    //             throw TraCIException(error);
    //         }
    //         libsumo::TraCIPosition p;
    //         p.x = ret.readDouble();
    //         p.y = ret.readDouble();
    //         return p;
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_position2_d(ret: &mut Storage, error: &str) -> TraCIPosition {
        todo!("port StorageHelper::readTypedPosition2D")
    }

    /// C++: `const libsumo::TraCIPositionVector StorageHelper::readTypedPolygon(tcpip::Storage &, const std::string &)` [static]
    // static inline const libsumo::TraCIPositionVector readTypedPolygon(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_POLYGON && error != "") {
    //             throw TraCIException(error);
    //         }
    //         libsumo::TraCIPositionVector poly;
    //         readPolygon(ret, poly);
    //         return poly;
    //     }
    #[allow(unused_variables)]
    pub fn read_typed_polygon(ret: &mut Storage, error: &str) -> TraCIPositionVector {
        todo!("port StorageHelper::readTypedPolygon")
    }

    /// C++: `int StorageHelper::readCompound(tcpip::Storage &, int, const std::string &)` [static]
    // static inline int readCompound(tcpip::Storage& ret, int expectedSize = -1, const std::string& error = "") {
    //         const int type = ret.readUnsignedByte();
    //         const int size = ret.readInt();
    //         if (error != "") {
    //             if (type != libsumo::TYPE_COMPOUND || (expectedSize != -1 && size != expectedSize)) {
    //                 throw TraCIException(error);
    //             }
    //         }
    //         return size;
    //     }
    #[allow(unused_variables)]
    pub fn read_compound(ret: &mut Storage, expectedSize: i32, error: &str) -> i32 {
        todo!("port StorageHelper::readCompound")
    }

    /// C++: `void StorageHelper::readPolygon(tcpip::Storage &, libsumo::TraCIPositionVector &)` [static]
    // static inline void readPolygon(tcpip::Storage& ret, libsumo::TraCIPositionVector& poly) {
    //         int size = ret.readUnsignedByte();
    //         if (size == 0) {
    //             size = ret.readInt();
    //         }
    //         for (int i = 0; i < size; ++i) {
    //             libsumo::TraCIPosition p;
    //             p.x = ret.readDouble();
    //             p.y = ret.readDouble();
    //             p.z = 0.;
    //             poly.value.emplace_back(p);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_polygon(ret: &mut Storage, poly: &mut TraCIPositionVector) {
        todo!("port StorageHelper::readPolygon")
    }

    /// C++: `bool StorageHelper::readBool(tcpip::Storage &, const std::string &)` [static]
    // static inline bool readBool(tcpip::Storage& ret, const std::string& error = "") {
    //         if (ret.readUnsignedByte() != libsumo::TYPE_UBYTE && error != "") {
    //             throw TraCIException(error);
    //         }
    //         return ret.readUnsignedByte() != 0;
    //     }
    #[allow(unused_variables)]
    pub fn read_bool(ret: &mut Storage, error: &str) -> bool {
        todo!("port StorageHelper::readBool")
    }

    /// C++: `void StorageHelper::readStage(tcpip::Storage &, libsumo::TraCIStage &, const std::string &)` [static]
    // static inline void readStage(tcpip::Storage& inputStorage, libsumo::TraCIStage& stage, const std::string& error = "") {
    //         stage.type = readTypedInt(inputStorage, error);
    //         stage.vType = readTypedString(inputStorage, error);
    //         stage.line = readTypedString(inputStorage, error);
    //         stage.destStop = readTypedString(inputStorage, error);
    //         stage.edges = readTypedStringList(inputStorage, error);
    //         stage.travelTime = readTypedDouble(inputStorage, error);
    //         stage.cost = readTypedDouble(inputStorage, error);
    //         stage.length = readTypedDouble(inputStorage, error);
    //         stage.intended = readTypedString(inputStorage, error);
    //         stage.depart = readTypedDouble(inputStorage, error);
    //         stage.departPos = readTypedDouble(inputStorage, error);
    //         stage.arrivalPos = readTypedDouble(inputStorage, error);
    //         stage.description = readTypedString(inputStorage, error);
    //     }
    #[allow(unused_variables)]
    pub fn read_stage(inputStorage: &mut Storage, stage: &mut TraCIStage, error: &str) {
        todo!("port StorageHelper::readStage")
    }

    /// C++: `void StorageHelper::readConnection(tcpip::Storage &, libsumo::TraCIConnection &, const std::string &)` [static]
    // static inline void readConnection(tcpip::Storage& inputStorage, libsumo::TraCIConnection& connection, const std::string& error = "") {
    //         connection.approachedLane = readTypedString(inputStorage, error);
    //         connection.approachedInternal = readTypedString(inputStorage, error);
    //         connection.hasPrio = readBool(inputStorage, error);
    //         connection.isOpen = readBool(inputStorage, error);
    //         connection.hasFoe = readBool(inputStorage, error);
    //         connection.state = readTypedString(inputStorage, error);
    //         connection.direction = readTypedString(inputStorage, error);
    //         connection.length = readTypedDouble(inputStorage, error);
    //     }
    #[allow(unused_variables)]
    pub fn read_connection(inputStorage: &mut Storage, connection: &mut TraCIConnection, error: &str) {
        todo!("port StorageHelper::readConnection")
    }

    /// C++: `void StorageHelper::readVehicleDataVector(tcpip::Storage &, std::vector<libsumo::TraCIVehicleData> &, const std::string &)` [static]
    // static inline void readVehicleDataVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCIVehicleData>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCIVehicleData vd;
    //             vd.id = readTypedString(inputStorage, error);
    //             vd.length = readTypedDouble(inputStorage, error);
    //             vd.entryTime = readTypedDouble(inputStorage, error);
    //             vd.leaveTime = readTypedDouble(inputStorage, error);
    //             vd.typeID = readTypedString(inputStorage, error);
    //             result.emplace_back(vd);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_vehicle_data_vector(inputStorage: &mut Storage, result: &mut Vec<TraCIVehicleData>, error: &str) {
        todo!("port StorageHelper::readVehicleDataVector")
    }

    /// C++: `void StorageHelper::readReservation(tcpip::Storage &, libsumo::TraCIReservation &, const std::string &)` [static]
    // static inline void readReservation(tcpip::Storage& inputStorage, libsumo::TraCIReservation& result, const std::string& error = "") {
    //         readCompound(inputStorage, 10, error);
    //         result.id = readTypedString(inputStorage, error);
    //         result.persons = readTypedStringList(inputStorage, error);
    //         result.group = readTypedString(inputStorage, error);
    //         result.fromEdge = readTypedString(inputStorage, error);
    //         result.toEdge = readTypedString(inputStorage, error);
    //         result.departPos = readTypedDouble(inputStorage, error);
    //         result.arrivalPos = readTypedDouble(inputStorage, error);
    //         result.depart = readTypedDouble(inputStorage, error);
    //         result.reservationTime = readTypedDouble(inputStorage, error);
    //         result.state = readTypedInt(inputStorage, error);
    //     }
    #[allow(unused_variables)]
    pub fn read_reservation(inputStorage: &mut Storage, result: &mut TraCIReservation, error: &str) {
        todo!("port StorageHelper::readReservation")
    }

    /// C++: `void StorageHelper::readLogic(tcpip::Storage &, libsumo::TraCILogic &, const std::string &)` [static]
    // static inline void readLogic(tcpip::Storage& inputStorage, libsumo::TraCILogic& logic, const std::string& error = "") {
    //         readCompound(inputStorage, 5, error);
    //         logic.programID = readTypedString(inputStorage);
    //         logic.type = readTypedInt(inputStorage);
    //         logic.currentPhaseIndex = readTypedInt(inputStorage);
    //         int numPhases = readCompound(inputStorage);
    //         while (numPhases-- > 0) {
    //             readCompound(inputStorage, 6);
    //             libsumo::TraCIPhase* phase = new libsumo::TraCIPhase();
    //             phase->duration = readTypedDouble(inputStorage);
    //             phase->state = readTypedString(inputStorage);
    //             phase->minDur = readTypedDouble(inputStorage);
    //             phase->maxDur = readTypedDouble(inputStorage);
    //             int numNext = readCompound(inputStorage);
    //             while (numNext-- > 0) {
    //                 phase->next.push_back(readTypedInt(inputStorage));
    //             }
    //             phase->name = readTypedString(inputStorage);
    //             logic.phases.emplace_back(phase);
    //         }
    //         int numParams = readCompound(inputStorage);
    //         while (numParams-- > 0) {
    //             const std::vector<std::string> key_value = readTypedStringList(inputStorage);
    //             logic.subParameter[key_value[0]] = key_value[1];
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_logic(inputStorage: &mut Storage, logic: &mut TraCILogic, error: &str) {
        todo!("port StorageHelper::readLogic")
    }

    /// C++: `void StorageHelper::readConstraintVector(tcpip::Storage &, std::vector<libsumo::TraCISignalConstraint> &, const std::string &)` [static]
    // static inline void readConstraintVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCISignalConstraint>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCISignalConstraint c;
    //             c.signalId = readTypedString(inputStorage);
    //             c.tripId = readTypedString(inputStorage);
    //             c.foeId = readTypedString(inputStorage);
    //             c.foeSignal = readTypedString(inputStorage);
    //             c.limit = readTypedInt(inputStorage);
    //             c.type = readTypedInt(inputStorage);
    //             c.mustWait = readTypedByte(inputStorage) != 0;
    //             c.active = readTypedByte(inputStorage) != 0;
    //             const std::vector<std::string> paramItems = readTypedStringList(inputStorage);
    //             for (int j = 0; j < (int)paramItems.size(); j += 2) {
    //                 c.param[paramItems[j]] = paramItems[j + 1];
    //             }
    //             result.emplace_back(c);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_constraint_vector(inputStorage: &mut Storage, result: &mut Vec<TraCISignalConstraint>, error: &str) {
        todo!("port StorageHelper::readConstraintVector")
    }

    /// C++: `void StorageHelper::readLinkVectorVector(tcpip::Storage &, std::vector<std::vector<libsumo::TraCILink>> &, const std::string &)` [static]
    // static inline void readLinkVectorVector(tcpip::Storage& inputStorage, std::vector< std::vector<libsumo::TraCILink> >& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             std::vector<libsumo::TraCILink> controlledLinks;
    //             int numLinks = readTypedInt(inputStorage);
    //             while (numLinks-- > 0) {
    //                 std::vector<std::string> link = readTypedStringList(inputStorage);
    //                 controlledLinks.emplace_back(link[0], link[2], link[1]);
    //             }
    //             result.emplace_back(controlledLinks);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_link_vector_vector(inputStorage: &mut Storage, result: &mut Vec<Vec<TraCILink>>, error: &str) {
        todo!("port StorageHelper::readLinkVectorVector")
    }

    /// C++: `void StorageHelper::readBestLanesVector(tcpip::Storage &, std::vector<libsumo::TraCIBestLanesData> &, const std::string &)` [static]
    // static inline void readBestLanesVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCIBestLanesData>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCIBestLanesData info;
    //             info.laneID = readTypedString(inputStorage);
    //             info.length = readTypedDouble(inputStorage);
    //             info.occupation = readTypedDouble(inputStorage);
    //             info.bestLaneOffset = readTypedUnsignedByte(inputStorage);
    //             info.allowsContinuation = readBool(inputStorage);
    //             info.continuationLanes = readTypedStringList(inputStorage, error);
    //             result.emplace_back(info);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_best_lanes_vector(inputStorage: &mut Storage, result: &mut Vec<TraCIBestLanesData>, error: &str) {
        todo!("port StorageHelper::readBestLanesVector")
    }

    /// C++: `void StorageHelper::readCollisionVector(tcpip::Storage &, std::vector<libsumo::TraCICollision> &, const std::string &)` [static]
    // static inline void readCollisionVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCICollision>& result, const std::string& error = "") {
    //         int numCollisions = readTypedInt(inputStorage, error);
    //         while (numCollisions-- > 0) {
    //             libsumo::TraCICollision c;
    //             c.collider = readTypedString(inputStorage);
    //             c.victim = readTypedString(inputStorage);
    //             c.colliderType = readTypedString(inputStorage);
    //             c.victimType = readTypedString(inputStorage);
    //             c.colliderSpeed = readTypedDouble(inputStorage);
    //             c.victimSpeed = readTypedDouble(inputStorage);
    //             c.type = readTypedString(inputStorage);
    //             c.lane = readTypedString(inputStorage);
    //             c.pos = readTypedDouble(inputStorage);
    //             result.emplace_back(c);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_collision_vector(inputStorage: &mut Storage, result: &mut Vec<TraCICollision>, error: &str) {
        todo!("port StorageHelper::readCollisionVector")
    }

    /// C++: `void StorageHelper::readJunctionFoeVector(tcpip::Storage &, std::vector<libsumo::TraCIJunctionFoe> &, const std::string &)` [static]
    // static inline void readJunctionFoeVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCIJunctionFoe>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCIJunctionFoe info;
    //             info.foeId = readTypedString(inputStorage);
    //             info.egoDist = readTypedDouble(inputStorage);
    //             info.foeDist = readTypedDouble(inputStorage);
    //             info.egoExitDist = readTypedDouble(inputStorage);
    //             info.foeExitDist = readTypedDouble(inputStorage);
    //             info.egoLane = readTypedString(inputStorage);
    //             info.foeLane = readTypedString(inputStorage);
    //             info.egoResponse = readBool(inputStorage);
    //             info.foeResponse = readBool(inputStorage);
    //             result.emplace_back(info);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_junction_foe_vector(inputStorage: &mut Storage, result: &mut Vec<TraCIJunctionFoe>, error: &str) {
        todo!("port StorageHelper::readJunctionFoeVector")
    }

    /// C++: `void StorageHelper::readStopVector(tcpip::Storage &, std::vector<libsumo::TraCINextStopData> &, const std::string &)` [static]
    // static inline void readStopVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCINextStopData>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCINextStopData s;
    //             s.lane = readTypedString(inputStorage);
    //             s.endPos = readTypedDouble(inputStorage);
    //             s.stoppingPlaceID = readTypedString(inputStorage);
    //             s.stopFlags = readTypedInt(inputStorage);
    //             s.duration = readTypedDouble(inputStorage);
    //             s.until = readTypedDouble(inputStorage);
    //             s.startPos = readTypedDouble(inputStorage);
    //             s.intendedArrival = readTypedDouble(inputStorage);
    //             s.arrival = readTypedDouble(inputStorage);
    //             s.depart = readTypedDouble(inputStorage);
    //             s.split = readTypedString(inputStorage);
    //             s.join = readTypedString(inputStorage);
    //             s.actType = readTypedString(inputStorage);
    //             s.tripId = readTypedString(inputStorage);
    //             s.line = readTypedString(inputStorage);
    //             s.speed = readTypedDouble(inputStorage);
    //             result.emplace_back(s);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_stop_vector(inputStorage: &mut Storage, result: &mut Vec<TraCINextStopData>, error: &str) {
        todo!("port StorageHelper::readStopVector")
    }

    /// C++: `void StorageHelper::readTLSDataVector(tcpip::Storage &, std::vector<libsumo::TraCINextTLSData> &, const std::string &)` [static]
    // static inline void readTLSDataVector(tcpip::Storage& inputStorage, std::vector<libsumo::TraCINextTLSData>& result, const std::string& error = "") {
    //         const int n = readTypedInt(inputStorage, error);
    //         for (int i = 0; i < n; ++i) {
    //             libsumo::TraCINextTLSData tls;
    //             tls.id = readTypedString(inputStorage);
    //             tls.tlIndex = readTypedInt(inputStorage);
    //             tls.dist = readTypedDouble(inputStorage);
    //             tls.state = (char)readTypedByte(inputStorage);
    //             result.emplace_back(tls);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn read_tls_data_vector(inputStorage: &mut Storage, result: &mut Vec<TraCINextTLSData>, error: &str) {
        todo!("port StorageHelper::readTLSDataVector")
    }

    /// C++: `void StorageHelper::writeTypedByte(tcpip::Storage &, int)` [static]
    // static inline void writeTypedByte(tcpip::Storage& content, int value) {
    //         content.writeUnsignedByte(libsumo::TYPE_BYTE);
    //         content.writeByte(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_byte(content: &mut Storage, value: i32) {
        todo!("port StorageHelper::writeTypedByte")
    }

    /// C++: `void StorageHelper::writeTypedUnsignedByte(tcpip::Storage &, int)` [static]
    // static inline void writeTypedUnsignedByte(tcpip::Storage& content, int value) {
    //         content.writeUnsignedByte(libsumo::TYPE_UBYTE);
    //         content.writeUnsignedByte(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_unsigned_byte(content: &mut Storage, value: i32) {
        todo!("port StorageHelper::writeTypedUnsignedByte")
    }

    /// C++: `void StorageHelper::writeTypedInt(tcpip::Storage &, int)` [static]
    // static inline void writeTypedInt(tcpip::Storage& content, int value) {
    //         content.writeUnsignedByte(libsumo::TYPE_INTEGER);
    //         content.writeInt(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_int(content: &mut Storage, value: i32) {
        todo!("port StorageHelper::writeTypedInt")
    }

    /// C++: `void StorageHelper::writeTypedDouble(tcpip::Storage &, double)` [static]
    // static inline void writeTypedDouble(tcpip::Storage& content, double value) {
    //         content.writeUnsignedByte(libsumo::TYPE_DOUBLE);
    //         content.writeDouble(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_double(content: &mut Storage, value: f64) {
        todo!("port StorageHelper::writeTypedDouble")
    }

    /// C++: `void StorageHelper::writeTypedString(tcpip::Storage &, const std::string &)` [static]
    // static inline void writeTypedString(tcpip::Storage& content, const std::string& value) {
    //         content.writeUnsignedByte(libsumo::TYPE_STRING);
    //         content.writeString(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_string(content: &mut Storage, value: &str) {
        todo!("port StorageHelper::writeTypedString")
    }

    /// C++: `void StorageHelper::writeTypedStringList(tcpip::Storage &, const std::vector<std::string> &)` [static]
    // static inline void writeTypedStringList(tcpip::Storage& content, const std::vector<std::string>& value) {
    //         content.writeUnsignedByte(libsumo::TYPE_STRINGLIST);
    //         content.writeStringList(value);
    //     }
    #[allow(unused_variables)]
    pub fn write_typed_string_list(content: &mut Storage, value: &Vec<String>) {
        todo!("port StorageHelper::writeTypedStringList")
    }

    /// C++: `void StorageHelper::writeCompound(tcpip::Storage &, int)` [static]
    // static inline void writeCompound(tcpip::Storage& content, int size) {
    //         content.writeUnsignedByte(libsumo::TYPE_COMPOUND);
    //         content.writeInt(size);
    //     }
    #[allow(unused_variables)]
    pub fn write_compound(content: &mut Storage, size: i32) {
        todo!("port StorageHelper::writeCompound")
    }

    /// C++: `void StorageHelper::writePolygon(tcpip::Storage &, const libsumo::TraCIPositionVector &)` [static]
    // static inline void writePolygon(tcpip::Storage& content, const libsumo::TraCIPositionVector& shape) {
    //         content.writeUnsignedByte(libsumo::TYPE_POLYGON);
    //         if (shape.value.size() <= 255) {
    //             content.writeUnsignedByte((int)shape.value.size());
    //         } else {
    //             content.writeUnsignedByte(0);
    //             content.writeInt((int)shape.value.size());
    //         }
    //         for (const libsumo::TraCIPosition& pos : shape.value) {
    //             content.writeDouble(pos.x);
    //             content.writeDouble(pos.y);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn write_polygon(content: &mut Storage, shape: &TraCIPositionVector) {
        todo!("port StorageHelper::writePolygon")
    }

    /// C++: `void StorageHelper::writeStage(tcpip::Storage &, const libsumo::TraCIStage &)` [static]
    // static inline void writeStage(tcpip::Storage& content, const libsumo::TraCIStage& stage) {
    //         writeCompound(content, 13);
    //         content.writeUnsignedByte(libsumo::TYPE_INTEGER);
    //         content.writeInt(stage.type);
    //         writeTypedString(content, stage.vType);
    //         writeTypedString(content, stage.line);
    //         writeTypedString(content, stage.destStop);
    //         writeTypedStringList(content, stage.edges);
    //         writeTypedDouble(content, stage.travelTime);
    //         writeTypedDouble(content, stage.cost);
    //         writeTypedDouble(content, stage.length);
    //         writeTypedString(content, stage.intended);
    //         writeTypedDouble(content, stage.depart);
    //         writeTypedDouble(content, stage.departPos);
    //         writeTypedDouble(content, stage.arrivalPos);
    //         writeTypedString(content, stage.description);
    //     }
    #[allow(unused_variables)]
    pub fn write_stage(content: &mut Storage, stage: &TraCIStage) {
        todo!("port StorageHelper::writeStage")
    }

    /// C++: `void StorageHelper::writeConstraint(tcpip::Storage &, const libsumo::TraCISignalConstraint &)` [static]
    // static inline void writeConstraint(tcpip::Storage& content, const libsumo::TraCISignalConstraint& c) {
    //         writeTypedString(content, c.signalId);
    //         writeTypedString(content, c.tripId);
    //         writeTypedString(content, c.foeId);
    //         writeTypedString(content, c.foeSignal);
    //         writeTypedInt(content, c.limit);
    //         writeTypedInt(content, c.type);
    //         writeTypedByte(content, c.mustWait);
    //         writeTypedByte(content, c.active);
    //         std::vector<std::string> paramItems;
    //         for (const auto& item : c.param) {
    //             paramItems.push_back(item.first);
    //             paramItems.push_back(item.second);
    //         }
    //         writeTypedStringList(content, paramItems);
    //     }
    #[allow(unused_variables)]
    pub fn write_constraint(content: &mut Storage, c: &TraCISignalConstraint) {
        todo!("port StorageHelper::writeConstraint")
    }

}

