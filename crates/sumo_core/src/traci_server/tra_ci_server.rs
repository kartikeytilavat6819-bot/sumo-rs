//! Port of traci-server/TraCIServer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `bool (*)(TraCIServer &, tcpip::Storage &, tcpip::Storage &) TraCIServer::CmdExecutor`
pub type TraCIServer_CmdExecutor = Unmapped /* bool (*)(TraCIServer &, tcpip::Storage &, tcpip::Storage &) */;

/// C++ class `TraCIServer::SocketInfo` (traci-server/TraCIServer.h)
pub struct TraCIServer_SocketInfo {
    /// next point of action for the client
    pub targetTime: SUMOTime,
    /// whether a "half step" has been done, executing only the move
    pub executeMove: bool,
    /// Socket object for this client
    pub socket: Ptr<Socket>,
    /// container for vehicle state changes since last step taken by this client
    pub vehicleStateChanges: i32,
    /// container for transportable state changes since last step taken by this client
    pub transportableStateChanges: i32,
}

impl TraCIServer_SocketInfo {
    /// C++: `void SocketInfo::SocketInfo(tcpip::Socket *, SUMOTime)`
    /// constructor
    // SocketInfo(tcpip::Socket* socket, SUMOTime t)
    //             : targetTime(t), socket(socket) {}
    #[allow(unused_variables)]
    pub fn new(socket: Ptr<Socket>, t: SUMOTime) -> Self {
        todo!("port SocketInfo::SocketInfo")
    }

    /// C++: `void SocketInfo::~SocketInfo() noexcept`
    /// destructor
    // ~SocketInfo() {
    //             delete socket;
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SocketInfo::~SocketInfo")
    }

    /// C++: `void SocketInfo::SocketInfo(const SocketInfo &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &TraCIServer_SocketInfo) -> Self {
        todo!("port SocketInfo::SocketInfo")
    }

}

/// TraCI server used to control sumo by a remote TraCI client
/// C++ class `TraCIServer` (traci-server/TraCIServer.h)
pub struct TraCIServer {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// C++ base class `libsumo::VariableWrapper`
    pub base2: VariableWrapper,
    /// C++ base class `MSNet::TransportableStateListener`
    pub base3: MSNet_TransportableStateListener,
    /// The socket connections to the clients the first component (index) determines the client's order (lowest index's commands are processed first),
    pub mySockets: i32,
    /// This stores the setOrder(int) requests of the clients.
    pub mySocketReorderRequests: i32,
    /// The currently active client socket
    pub myCurrentSocket: i32,
    /// The time step to reach until processing the next commands
    pub myTargetTime: SUMOTime,
    /// The storage to read from
    pub myInputStorage: Storage,
    /// The storage to write to
    pub myOutputStorage: Storage,
    /// A temporary storage to let the wrapper write to
    pub myWrapperStorage: Storage,
    /// The last timestep's subscription results
    pub mySubscriptionCache: Storage,
    /// Map of commandIds -> their executors; applicable if the executor applies to the method footprint
    pub myExecutors: i32,
    /// Set of variables which have parameters
    pub myParameterized: i32,
    pub myLoadArgs: i32,
    /// The list of known, still valid subscriptions
    pub mySubscriptions: i32,
    /// The last modified context subscription (the one to add a filter to, see (), currently only for vehicle to vehicle context)
    pub myLastContextSubscription: Ptr<Subscription>,
    /// Changes in the states of simulated vehicles
    pub myVehicleStateChanges: i32,
    /// Changes in the states of simulated transportables
    pub myTransportableStateChanges: i32,
}

impl std::ops::Deref for TraCIServer {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for TraCIServer {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

// C++ static members of TraCIServer (port as module-level statics, see AGENTS.md):
//   static TraCIServer * TraCIServer::myInstance;
//   static bool TraCIServer::myDoCloseConnection;

impl TraCIServer {
    /// C++: `SUMOTime TraCIServer::getTargetTime() const` [const]
    // SUMOTime getTargetTime() const {
    //         return myTargetTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_time(&self) -> SUMOTime {
        todo!("port TraCIServer::getTargetTime")
    }

    /// C++: `TraCIServer * TraCIServer::getInstance()` [static]
    // static TraCIServer* getInstance() {
    //         return myInstance;
    //     }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<TraCIServer> {
        todo!("port TraCIServer::getInstance")
    }

    /// C++: `void TraCIServer::openSocket(const int &)` [static]
    /// Initialises the server
    // void
    // TraCIServer::openSocket(const std::map<int, CmdExecutor>& execs) {
    //     if (myInstance == nullptr && !myDoCloseConnection && (OptionsCont::getOptions().getInt("remote-port") != 0)) {
    //         myInstance = new TraCIServer(string2time(OptionsCont::getOptions().getString("begin")),
    //                                      OptionsCont::getOptions().getInt("remote-port"),
    //                                      OptionsCont::getOptions().getInt("num-clients"));
    //         for (std::map<int, CmdExecutor>::const_iterator i = execs.begin(); i != execs.end(); ++i) {
    //             myInstance->myExecutors[i->first] = i->second;
    //         }
    //     }
    //     if (myInstance != nullptr) {
    //         // maybe net was deleted and built again
    //         MSNet::getInstance()->addVehicleStateListener(myInstance);
    //         MSNet::getInstance()->addTransportableStateListener(myInstance);
    //         myInstance->mySubscriptionCache.writeInt(0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_socket(execs: &i32) {
        todo!("port TraCIServer::openSocket")
    }

    /// C++: `void TraCIServer::close()` [static]
    /// request termination of connection
    // void
    // TraCIServer::close() {
    //     if (myInstance == nullptr) {
    //         return;
    //     }
    //     delete myInstance;
    //     myInstance = nullptr;
    //     myDoCloseConnection = true;
    // }
    #[allow(unused_variables)]
    pub fn close() {
        todo!("port TraCIServer::close")
    }

    /// C++: `bool TraCIServer::wasClosed()` [static]
    /// check whether close was requested
    // bool
    // TraCIServer::wasClosed() {
    //     return myDoCloseConnection;
    // }
    #[allow(unused_variables)]
    pub fn was_closed() -> bool {
        todo!("port TraCIServer::wasClosed")
    }

    /// C++: `int TraCIServer::processCommands(const SUMOTime, const bool)`
    /// process all commands until the next SUMO simulation step. It is guaranteed that t->getTargetTime() >= myStep after call (except the case that a load or close command is received)s
    // int
    // TraCIServer::processCommands(const SUMOTime step, const bool afterMove) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << SIMTIME << " processCommands(step = " << step << "):\n" << std::endl;
    // #endif
    //     try {
    //         int finalCmd = 0;
    //         const bool firstStep = myCurrentSocket != mySockets.end();
    //         // update client order if requested
    //         processReorderingRequests();
    //         if (!firstStep && !afterMove) {
    //             // This is the entry point after performing a SUMO step (block is skipped before first SUMO step since then no simulation results have to be sent)
    //             // update subscription results
    //             postProcessSimulationStep();
    //             // Send out subscription results to clients which will act in this SUMO step (i.e. with client target time <= current sumo timestep end)
    //             sendOutputToAll();
    //             myOutputStorage.reset();
    //         }
    //
    //         // determine minimal next target time among clients
    //         myTargetTime = nextTargetTime();
    //
    //         if (step < myTargetTime) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //             std::cout << "    next target time is larger than next SUMO simstep (" << step << "). Returning from processCommands()." << std::endl;
    // #endif
    //             return finalCmd;
    //         }
    //
    //         // Simulation should run until
    //         // 1. end time reached or
    //         // 2. got libsumo::CMD_CLOSE or
    //         // 3. got libsumo::CMD_LOAD or
    //         // 4. Client closes socket connection
    //         while (!myDoCloseConnection && myTargetTime <= (MSNet::getInstance()->getCurrentTimeStep()) && finalCmd != libsumo::CMD_EXECUTEMOVE) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //             std::cout << "  Next target time: " << myTargetTime << std::endl;
    // #endif
    //             // Iterate over clients and process communication for the ones with target time == myTargetTime
    //             myCurrentSocket = mySockets.begin();
    //             while (myCurrentSocket != mySockets.end()) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "  current socket: " << myCurrentSocket->second->socket
    //                           << " with target time=" << myCurrentSocket->second->targetTime
    //                           << std::endl;
    // #endif
    //
    //                 if (myCurrentSocket->second->targetTime > myTargetTime || (afterMove && !myCurrentSocket->second->executeMove)) {
    //                     // this client must wait
    // #ifdef DEBUG_MULTI_CLIENTS
    //                     std::cout <<  "       skipping client " << myCurrentSocket->second->socket
    //                               << " with target time=" << myCurrentSocket->second->targetTime << std::endl;
    // #endif
    //                     myCurrentSocket++;
    //                     continue;
    //                 }
    //                 finalCmd = 0;
    //                 while (finalCmd == 0) {
    //                     if (!myInputStorage.valid_pos()) {
    //                         // have read request completely, send response if adequate
    //                         if (myOutputStorage.size() > 0) {
    //                             // send response to previous query
    //                             myCurrentSocket->second->socket->sendExact(myOutputStorage);
    //                             myOutputStorage.reset();
    //                         }
    // #ifdef DEBUG_MULTI_CLIENTS
    //                         std::cout << "    resetting input storage and reading next command..." << std::endl;
    // #endif
    //                         // Read next request
    //                         myInputStorage.reset();
    //                         myCurrentSocket->second->socket->receiveExact(myInputStorage);
    //                     }
    //
    //                     while (myInputStorage.valid_pos() && !myDoCloseConnection) {
    //                         const int cmd = dispatchCommand();
    //                         if (cmd == libsumo::CMD_SIMSTEP || cmd == libsumo::CMD_LOAD || cmd == libsumo::CMD_EXECUTEMOVE || cmd == libsumo::CMD_CLOSE) {
    //                             finalCmd = cmd;
    //                         }
    //                     }
    //                 }
    //             }
    //             if (!myLoadArgs.empty()) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "  Breaking loop to load new simulation." << std::endl;
    // #endif
    //                 break;
    //             } else if (myDoCloseConnection) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "  Breaking loop because last client closed connection." << std::endl;
    // #endif
    //                 break;
    //             }
    //             SUMOTime nextT = nextTargetTime();
    //             // minimal target time among clients should have been increased during the last loop through mySockets
    //             // XXX: The assert below is disabled since many tests do sth. like simulationStep(step). Such that for a first call step=0,
    //             //      leading to targetTime==1000 (increased by DELTA_T in dispatchCommand()),
    //             //      the next call is then usually simulationStep(step=1000) leading to no further increase
    //             //      and thus a failing assertion here.
    //             //assert(myTargetTime < nextT || myDoCloseConnection);
    //             myTargetTime = nextT;
    //         }
    //         // All clients are done with the current time step
    //         // Reset myVehicleStateChanges and myTransportableStateChanges
    //         for (auto& item : myVehicleStateChanges) {
    //             item.second.clear();
    //         }
    //         for (auto& item : myTransportableStateChanges) {
    //             item.second.clear();
    //         }
    //         return finalCmd;
    //     } catch (std::invalid_argument& e) {
    //         throw ProcessError(e.what());
    //     } catch (libsumo::TraCIException& e) {
    //         throw ProcessError(e.what());
    //     } catch (tcpip::SocketException& e) {
    //         throw ProcessError(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_commands(&mut self, step: SUMOTime, afterMove: bool) -> i32 {
        todo!("port TraCIServer::processCommands")
    }

    /// C++: `bool TraCIServer::processGet(const int, tcpip::Storage &, tcpip::Storage &)`
    /// Processes a get value command
    // bool
    // TraCIServer::processGet(const int commandID, tcpip::Storage& inputStorage, tcpip::Storage& outputStorage) {
    //     const int variable = inputStorage.readUnsignedByte();
    //     const std::string id = inputStorage.readString();
    //     initWrapper(commandID + 0x10, variable, id);
    //     try {
    //         switch (commandID) {
    //             case libsumo::CMD_GET_INDUCTIONLOOP_VARIABLE:
    //                 if (!libsumo::InductionLoop::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Induction Loop Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_MULTIENTRYEXIT_VARIABLE:
    //                 if (!libsumo::MultiEntryExit::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Multi Entry Exit Detector Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_TL_VARIABLE:
    //                 if (!libsumo::TrafficLight::handleVariable(id, variable, this, &inputStorage)) {
    //                     switch (variable) {
    //                         case libsumo::TL_CONSTRAINT_SWAP: {
    //                             StoHelp::readCompound(inputStorage, 3, "A compound object of size 3 is needed for swapping constraints.");
    //                             const std::string tripId = StoHelp::readTypedString(inputStorage, "The tripId must be given as a string.");
    //                             const std::string foeSignal = StoHelp::readTypedString(inputStorage, "The foeSignal id must be given as a string.");
    //                             const std::string foeId = StoHelp::readTypedString(inputStorage, "The foe tripId must be given as a string.");
    //                             wrapSignalConstraintVector(id, variable, libsumo::TrafficLight::swapConstraints(id, tripId, foeSignal, foeId));
    //                             break;
    //                         }
    //                         default:
    //                             throw libsumo::TraCIException("Get TLS Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                     }
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_LANE_VARIABLE:
    //                 if (!libsumo::Lane::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Lane Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_VEHICLE_VARIABLE:
    //                 if (!libsumo::Vehicle::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Vehicle Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_VEHICLETYPE_VARIABLE:
    //                 if (!libsumo::VehicleType::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Vehicle Type Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_ROUTE_VARIABLE:
    //                 if (!libsumo::Route::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Route Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_POI_VARIABLE:
    //                 if (!libsumo::POI::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get PoI Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_POLYGON_VARIABLE:
    //                 if (!libsumo::Polygon::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Polygon Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_JUNCTION_VARIABLE:
    //                 if (!libsumo::Junction::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Junction Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_EDGE_VARIABLE:
    //                 if (!libsumo::Edge::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Edge Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_SIM_VARIABLE:
    //                 if (!TraCIServerAPI_Simulation::processGet(*this, inputStorage, id, variable)) {
    //                     throw libsumo::TraCIException("Get Simulation Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_LANEAREA_VARIABLE:
    //                 if (!libsumo::LaneArea::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Lane Area Detector Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_PERSON_VARIABLE:
    //                 if (!libsumo::Person::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Person Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_BUSSTOP_VARIABLE:
    //                 if (!libsumo::BusStop::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get BusStop Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_PARKINGAREA_VARIABLE:
    //                 if (!libsumo::ParkingArea::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get ParkingArea Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_CHARGINGSTATION_VARIABLE:
    //                 if (!libsumo::ChargingStation::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get ChargingStation Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_ROUTEPROBE_VARIABLE:
    //                 if (!libsumo::RouteProbe::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get RouteProbe Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_CALIBRATOR_VARIABLE:
    //                 if (!libsumo::Calibrator::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Calibrator Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_REROUTER_VARIABLE:
    //                 if (!libsumo::Rerouter::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get Rerouter Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_VARIABLESPEEDSIGN_VARIABLE:
    //                 if (!libsumo::VariableSpeedSign::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get VariableSpeedSign Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_MEANDATA_VARIABLE:
    //                 if (!libsumo::MeanData::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get MeanData Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             case libsumo::CMD_GET_OVERHEADWIRE_VARIABLE:
    //                 if (!libsumo::OverheadWire::handleVariable(id, variable, this, &inputStorage)) {
    //                     throw libsumo::TraCIException("Get OverheadWire Variable: unsupported variable " + toHex(variable, 2) + " specified");
    //                 }
    //                 break;
    //             default:
    //                 return false;
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         return writeErrorStatusCmd(commandID, e.what(), outputStorage);
    //     }
    //     writeStatusCmd(commandID, libsumo::RTYPE_OK, "", outputStorage);
    //     writeResponseWithLength(outputStorage, getWrapperStorage());
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn process_get(&mut self, commandID: i32, inputStorage: &mut Storage, outputStorage: &mut Storage) -> bool {
        todo!("port TraCIServer::processGet")
    }

    /// C++: `void TraCIServer::cleanup()`
    /// clean up subscriptions
    // void
    // TraCIServer::cleanup() {
    //     mySubscriptions.clear();
    //     myTargetTime = string2time(OptionsCont::getOptions().getString("begin"));
    //     for (myCurrentSocket = mySockets.begin(); myCurrentSocket != mySockets.end(); ++myCurrentSocket) {
    //         myCurrentSocket->second->targetTime = myTargetTime;
    //         myCurrentSocket->second->executeMove = false;
    //     }
    //     myOutputStorage.reset();
    //     myInputStorage.reset();
    //     mySubscriptionCache.reset();
    //     for (auto& i : myVehicleStateChanges) {
    //         i.second.clear();
    //     }
    //     for (auto& i : myTransportableStateChanges) {
    //         i.second.clear();
    //     }
    //     myCurrentSocket = mySockets.begin();
    // }
    #[allow(unused_variables)]
    pub fn cleanup(&mut self) {
        todo!("port TraCIServer::cleanup")
    }

    /// C++: `void TraCIServer::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    // void
    // TraCIServer::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& /*info*/) {
    //     if (!myDoCloseConnection) {
    //         myVehicleStateChanges[to].push_back(vehicle->getID());
    //         for (std::map<int, SocketInfo*>::iterator i = mySockets.begin(); i != mySockets.end(); ++i) {
    //             i->second->vehicleStateChanges[to].push_back(vehicle->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        todo!("port TraCIServer::vehicleStateChanged")
    }

    /// C++: `void TraCIServer::transportableStateChanged(const MSTransportable *const, MSNet::TransportableState, const std::string &)` [virtual]
    // void
    // TraCIServer::transportableStateChanged(const MSTransportable* const transportable, MSNet::TransportableState to, const std::string& /*info*/) {
    //     if (!myDoCloseConnection) {
    //         myTransportableStateChanges[to].push_back(transportable->getID());
    //         for (std::map<int, SocketInfo*>::iterator i = mySockets.begin(); i != mySockets.end(); ++i) {
    //             i->second->transportableStateChanges[to].push_back(transportable->getID());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn transportable_state_changed(&mut self, transportable: Ptr<MSTransportable>, to: MSNet_TransportableState, info: &str) {
        todo!("port TraCIServer::transportableStateChanged")
    }

    /// C++: `void TraCIServer::writeStatusCmd(int, int, const std::string &, tcpip::Storage &)`
    /// Writes a status command to the given storage
    // void
    // TraCIServer::writeStatusCmd(int commandId, int status, const std::string& description, tcpip::Storage& outputStorage) {
    //     if (status == libsumo::RTYPE_ERR) {
    //         WRITE_ERROR("Answered with error to command " + toHex(commandId, 2) + ": " + description);
    //     } else if (status == libsumo::RTYPE_NOTIMPLEMENTED) {
    //         WRITE_ERROR("Requested command not implemented (" + toHex(commandId, 2) + "): " + description);
    //     }
    //     outputStorage.writeUnsignedByte(1 + 1 + 1 + 4 + static_cast<int>(description.length())); // command length
    //     outputStorage.writeUnsignedByte(commandId); // command type
    //     outputStorage.writeUnsignedByte(status); // status
    //     outputStorage.writeString(description); // description
    // }
    #[allow(unused_variables)]
    pub fn write_status_cmd(&mut self, commandId: i32, status: i32, description: &str, outputStorage: &mut Storage) {
        todo!("port TraCIServer::writeStatusCmd")
    }

    /// C++: `void TraCIServer::writeStatusCmd(int, int, const std::string &)`
    /// Writes a status command to myOutputStorage
    // void
    // TraCIServer::writeStatusCmd(int commandId, int status, const std::string& description) {
    //     writeStatusCmd(commandId, status, description, myOutputStorage);
    // }
    #[allow(unused_variables)]
    pub fn write_status_cmd_2(&mut self, commandId: i32, status: i32, description: &str) {
        todo!("port TraCIServer::writeStatusCmd")
    }

    /// C++: `bool TraCIServer::writeErrorStatusCmd(int, const std::string &, tcpip::Storage &)`
    /// Writes a status command to the given storage with status = RTYPE_ERR
    // bool
    // TraCIServer::writeErrorStatusCmd(int commandId, const std::string& description, tcpip::Storage& outputStorage) {
    //     writeStatusCmd(commandId, libsumo::RTYPE_ERR, description, outputStorage);
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn write_error_status_cmd(&mut self, commandId: i32, description: &str, outputStorage: &mut Storage) -> bool {
        todo!("port TraCIServer::writeErrorStatusCmd")
    }

    /// C++: `const int & TraCIServer::getVehicleStateChanges() const` [const]
    /// @}
    // const std::map<MSNet::VehicleState, std::vector<std::string> >& getVehicleStateChanges() const {
    //         if (myCurrentSocket == mySockets.end()) {
    //             // Requested in context of a subscription update
    //             return myVehicleStateChanges;
    //         } else {
    //             // Requested in the context of a custom query by active client
    //             return myCurrentSocket->second->vehicleStateChanges;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_state_changes(&self) -> &i32 {
        todo!("port TraCIServer::getVehicleStateChanges")
    }

    /// C++: `const int & TraCIServer::getTransportableStateChanges() const` [const]
    // const std::map<MSNet::TransportableState, std::vector<std::string> >& getTransportableStateChanges() const {
    //         if (myCurrentSocket == mySockets.end()) {
    //             // Requested in context of a subscription update
    //             return myTransportableStateChanges;
    //         } else {
    //             // Requested in the context of a custom query by active client
    //             return myCurrentSocket->second->transportableStateChanges;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_transportable_state_changes(&self) -> &i32 {
        todo!("port TraCIServer::getTransportableStateChanges")
    }

    /// C++: `void TraCIServer::writeResponseWithLength(tcpip::Storage &, tcpip::Storage &)`
    // void
    // TraCIServer::writeResponseWithLength(tcpip::Storage& outputStorage, tcpip::Storage& tempMsg) {
    //     if (tempMsg.size() < 254) {
    //         outputStorage.writeUnsignedByte(1 + (int)tempMsg.size()); // command length -> short
    //     } else {
    //         outputStorage.writeUnsignedByte(0); // command length -> extended
    //         outputStorage.writeInt(1 + 4 + (int)tempMsg.size());
    //     }
    //     outputStorage.writeStorage(tempMsg);
    // }
    #[allow(unused_variables)]
    pub fn write_response_with_length(&mut self, outputStorage: &mut Storage, tempMsg: &mut Storage) {
        todo!("port TraCIServer::writeResponseWithLength")
    }

    /// C++: `void TraCIServer::stateLoaded(SUMOTime)`
    /// updates myTargetTime and resets vehicle state changes after loading a simulation state
    // void
    // TraCIServer::stateLoaded(SUMOTime targetTime) {
    //     myTargetTime = targetTime;
    //     for (auto& s : mySockets) {
    //         s.second->targetTime = targetTime;
    //         s.second->executeMove = false;
    //         for (auto& stateChange : s.second->vehicleStateChanges) {
    //             stateChange.second.clear();
    //         }
    //         for (auto& stateChange : s.second->transportableStateChanges) {
    //             stateChange.second.clear();
    //         }
    //     }
    //     mySubscriptions.clear();
    //     mySubscriptionCache.reset();
    // }
    #[allow(unused_variables)]
    pub fn state_loaded(&mut self, targetTime: SUMOTime) {
        todo!("port TraCIServer::stateLoaded")
    }

    /// C++: `int & TraCIServer::getLoadArgs()`
    // std::vector<std::string>& getLoadArgs() {
    //         return myLoadArgs;
    //     }
    #[allow(unused_variables)]
    pub fn get_load_args(&mut self) -> &mut i32 {
        todo!("port TraCIServer::getLoadArgs")
    }

    /// C++: `void TraCIServer::initWrapper(const int, const int, const std::string &)`
    /// @{
    // void
    // TraCIServer::initWrapper(const int domainID, const int variable, const std::string& objID) {
    //     myWrapperStorage.reset();
    //     myWrapperStorage.writeUnsignedByte(domainID);
    //     myWrapperStorage.writeUnsignedByte(variable);
    //     myWrapperStorage.writeString(objID);
    // }
    #[allow(unused_variables)]
    pub fn init_wrapper(&mut self, domainID: i32, variable: i32, objID: &str) {
        todo!("port TraCIServer::initWrapper")
    }

    /// C++: `bool TraCIServer::wrapConnectionVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapConnectionVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCIConnection>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 8);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCIConnection& c : value) {
    //         StoHelp::writeTypedString(myWrapperStorage, c.approachedLane);
    //         StoHelp::writeTypedString(myWrapperStorage, c.approachedInternal);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, c.hasPrio);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, c.isOpen);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, c.hasFoe);
    //         StoHelp::writeTypedString(myWrapperStorage, c.state);
    //         StoHelp::writeTypedString(myWrapperStorage, c.direction);
    //         StoHelp::writeTypedDouble(myWrapperStorage, c.length);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_connection_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapConnectionVector")
    }

    /// C++: `bool TraCIServer::wrapDouble(const std::string &, const int, const double)` [virtual]
    // bool
    // TraCIServer::wrapDouble(const std::string& /* objID */, const int /* variable */, const double value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_DOUBLE);
    //     myWrapperStorage.writeDouble(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_double(&mut self, objID: &str, variable: i32, value: f64) -> bool {
        todo!("port TraCIServer::wrapDouble")
    }

    /// C++: `bool TraCIServer::wrapInt(const std::string &, const int, const int)` [virtual]
    // bool
    // TraCIServer::wrapInt(const std::string& /* objID */, const int /* variable */, const int value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_INTEGER);
    //     myWrapperStorage.writeInt(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_int(&mut self, objID: &str, variable: i32, value: i32) -> bool {
        todo!("port TraCIServer::wrapInt")
    }

    /// C++: `bool TraCIServer::wrapString(const std::string &, const int, const std::string &)` [virtual]
    // bool
    // TraCIServer::wrapString(const std::string& /* objID */, const int /* variable */, const std::string& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_STRING);
    //     myWrapperStorage.writeString(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string(&mut self, objID: &str, variable: i32, value: &str) -> bool {
        todo!("port TraCIServer::wrapString")
    }

    /// C++: `bool TraCIServer::wrapStringList(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapStringList(const std::string& /* objID */, const int /* variable */, const std::vector<std::string>& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_STRINGLIST);
    //     myWrapperStorage.writeStringList(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_list(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapStringList")
    }

    /// C++: `bool TraCIServer::wrapDoubleList(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapDoubleList(const std::string& /* objID */, const int /* variable */, const std::vector<double>& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_DOUBLELIST);
    //     myWrapperStorage.writeDoubleList(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_double_list(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapDoubleList")
    }

    /// C++: `bool TraCIServer::wrapPosition(const std::string &, const int, const libsumo::TraCIPosition &)` [virtual]
    // bool
    // TraCIServer::wrapPosition(const std::string& /* objID */, const int variable, const libsumo::TraCIPosition& value) {
    //     const bool includeZ = variable == libsumo::VAR_POSITION3D;
    //     myWrapperStorage.writeUnsignedByte(includeZ ? libsumo::POSITION_3D : libsumo::POSITION_2D);
    //     myWrapperStorage.writeDouble(value.x);
    //     myWrapperStorage.writeDouble(value.y);
    //     if (includeZ) {
    //         myWrapperStorage.writeDouble(value.z);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_position(&mut self, objID: &str, variable: i32, value: &TraCIPosition) -> bool {
        todo!("port TraCIServer::wrapPosition")
    }

    /// C++: `bool TraCIServer::wrapPositionVector(const std::string &, const int, const libsumo::TraCIPositionVector &)` [virtual]
    // bool
    // TraCIServer::wrapPositionVector(const std::string& /* objID */, const int /* variable */, const libsumo::TraCIPositionVector& shape) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_POLYGON);
    //     if (shape.value.size() < 256) {
    //         myWrapperStorage.writeUnsignedByte((int)shape.value.size());
    //     } else {
    //         myWrapperStorage.writeUnsignedByte(0);
    //         myWrapperStorage.writeInt((int)shape.value.size());
    //     }
    //     for (const libsumo::TraCIPosition& pos : shape.value) {
    //         myWrapperStorage.writeDouble(pos.x);
    //         myWrapperStorage.writeDouble(pos.y);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_position_vector(&mut self, objID: &str, variable: i32, value: &TraCIPositionVector) -> bool {
        todo!("port TraCIServer::wrapPositionVector")
    }

    /// C++: `bool TraCIServer::wrapColor(const std::string &, const int, const libsumo::TraCIColor &)` [virtual]
    // bool
    // TraCIServer::wrapColor(const std::string& /* objID */, const int /* variable */, const libsumo::TraCIColor& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_COLOR);
    //     myWrapperStorage.writeUnsignedByte(value.r);
    //     myWrapperStorage.writeUnsignedByte(value.g);
    //     myWrapperStorage.writeUnsignedByte(value.b);
    //     myWrapperStorage.writeUnsignedByte(value.a);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_color(&mut self, objID: &str, variable: i32, value: &TraCIColor) -> bool {
        todo!("port TraCIServer::wrapColor")
    }

    /// C++: `bool TraCIServer::wrapStringDoublePair(const std::string &, const int, const std::pair<std::string, double> &)` [virtual]
    // bool
    // TraCIServer::wrapStringDoublePair(const std::string& /* objID */, const int /* variable */, const std::pair<std::string, double>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 2);
    //     StoHelp::writeTypedString(myWrapperStorage, value.first);
    //     StoHelp::writeTypedDouble(myWrapperStorage, value.second);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair(&mut self, objID: &str, variable: i32, value: &(String, f64)) -> bool {
        todo!("port TraCIServer::wrapStringDoublePair")
    }

    /// C++: `bool TraCIServer::wrapStringDoublePairList(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapStringDoublePairList(const std::string& /* objID */, const int /* variable */, const std::vector<std::pair<std::string, double> >& value) {
    //     StoHelp::writeCompound(myWrapperStorage, (int)value.size());
    //     for (const auto& p : value) {
    //         myWrapperStorage.writeString(p.first);
    //         myWrapperStorage.writeDouble(p.second);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_double_pair_list(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapStringDoublePairList")
    }

    /// C++: `bool TraCIServer::wrapStringPair(const std::string &, const int, const std::pair<std::string, std::string> &)` [virtual]
    // bool
    // TraCIServer::wrapStringPair(const std::string& /* objID */, const int /* variable */, const std::pair<std::string, std::string>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 2);
    //     StoHelp::writeTypedString(myWrapperStorage, value.first);
    //     StoHelp::writeTypedString(myWrapperStorage, value.second);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_pair(&mut self, objID: &str, variable: i32, value: &(String, String)) -> bool {
        todo!("port TraCIServer::wrapStringPair")
    }

    /// C++: `bool TraCIServer::wrapIntPair(const std::string &, const int, const std::pair<int, int> &)` [virtual]
    // bool
    // TraCIServer::wrapIntPair(const std::string& /* objID */, const int /* variable */, const std::pair<int, int>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 2);
    //     StoHelp::writeTypedInt(myWrapperStorage, value.first);
    //     StoHelp::writeTypedInt(myWrapperStorage, value.second);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_int_pair(&mut self, objID: &str, variable: i32, value: &(i32, i32)) -> bool {
        todo!("port TraCIServer::wrapIntPair")
    }

    /// C++: `bool TraCIServer::wrapStage(const std::string &, const int, const libsumo::TraCIStage &)` [virtual]
    // bool
    // TraCIServer::wrapStage(const std::string& /* objID */, const int /* variable */, const libsumo::TraCIStage& value) {
    //     StoHelp::writeStage(myWrapperStorage, value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_stage(&mut self, objID: &str, variable: i32, value: &TraCIStage) -> bool {
        todo!("port TraCIServer::wrapStage")
    }

    /// C++: `bool TraCIServer::wrapReservationVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapReservationVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCIReservation>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCIReservation& r : value) {
    //         StoHelp::writeCompound(myWrapperStorage, 10);
    //         StoHelp::writeTypedString(myWrapperStorage, r.id);
    //         StoHelp::writeTypedStringList(myWrapperStorage, r.persons);
    //         StoHelp::writeTypedString(myWrapperStorage, r.group);
    //         StoHelp::writeTypedString(myWrapperStorage, r.fromEdge);
    //         StoHelp::writeTypedString(myWrapperStorage, r.toEdge);
    //         StoHelp::writeTypedDouble(myWrapperStorage, r.departPos);
    //         StoHelp::writeTypedDouble(myWrapperStorage, r.arrivalPos);
    //         StoHelp::writeTypedDouble(myWrapperStorage, r.depart);
    //         StoHelp::writeTypedDouble(myWrapperStorage, r.reservationTime);
    //         StoHelp::writeTypedInt(myWrapperStorage, r.state);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_reservation_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapReservationVector")
    }

    /// C++: `bool TraCIServer::wrapLogicVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapLogicVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCILogic>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCILogic& logic : value) {
    //         StoHelp::writeCompound(myWrapperStorage, 5);
    //         StoHelp::writeTypedString(myWrapperStorage, logic.programID);
    //         StoHelp::writeTypedInt(myWrapperStorage, logic.type);
    //         StoHelp::writeTypedInt(myWrapperStorage, logic.currentPhaseIndex);
    //         StoHelp::writeCompound(myWrapperStorage, (int)logic.phases.size());
    //         for (const std::shared_ptr<libsumo::TraCIPhase>& phase : logic.phases) {
    //             StoHelp::writeCompound(myWrapperStorage, 6);
    //             StoHelp::writeTypedDouble(myWrapperStorage, phase->duration);
    //             StoHelp::writeTypedString(myWrapperStorage, phase->state);
    //             StoHelp::writeTypedDouble(myWrapperStorage, phase->minDur);
    //             StoHelp::writeTypedDouble(myWrapperStorage, phase->maxDur);
    //             StoHelp::writeCompound(myWrapperStorage, (int)phase->next.size());
    //             for (int n : phase->next) {
    //                 StoHelp::writeTypedInt(myWrapperStorage, n);
    //             }
    //             StoHelp::writeTypedString(myWrapperStorage, phase->name);
    //         }
    //         StoHelp::writeCompound(myWrapperStorage, (int)logic.subParameter.size());
    //         for (const auto& item : logic.subParameter) {
    //             StoHelp::writeTypedStringList(myWrapperStorage, std::vector<std::string> {item.first, item.second});
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_logic_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapLogicVector")
    }

    /// C++: `bool TraCIServer::wrapLinkVectorVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapLinkVectorVector(const std::string& /* objID */, const int /* variable */, const std::vector<std::vector<libsumo::TraCILink> >& value) {
    //     int cnt = 1;
    //     for (const std::vector<libsumo::TraCILink>& sublinks : value) {
    //         cnt += (int)sublinks.size() + 1;
    //     }
    //     StoHelp::writeCompound(myWrapperStorage, cnt);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const std::vector<libsumo::TraCILink>& sublinks : value) {
    //         StoHelp::writeTypedInt(myWrapperStorage, (int)sublinks.size());
    //         for (const libsumo::TraCILink& link : sublinks) {
    //             StoHelp::writeTypedStringList(myWrapperStorage, std::vector<std::string>({ link.fromLane, link.toLane, link.viaLane }));
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_link_vector_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapLinkVectorVector")
    }

    /// C++: `bool TraCIServer::wrapSignalConstraintVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapSignalConstraintVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCISignalConstraint>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 5);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const auto& c : value) {
    //         StoHelp::writeConstraint(myWrapperStorage, c);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_signal_constraint_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapSignalConstraintVector")
    }

    /// C++: `bool TraCIServer::wrapJunctionFoeVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapJunctionFoeVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCIJunctionFoe>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 9);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const auto& c : value) {
    //         StoHelp::writeTypedString(myWrapperStorage, c.foeId);
    //         StoHelp::writeTypedDouble(myWrapperStorage, c.egoDist);
    //         StoHelp::writeTypedDouble(myWrapperStorage, c.foeDist);
    //         StoHelp::writeTypedDouble(myWrapperStorage, c.egoExitDist);
    //         StoHelp::writeTypedDouble(myWrapperStorage, c.foeExitDist);
    //         StoHelp::writeTypedString(myWrapperStorage, c.egoLane);
    //         StoHelp::writeTypedString(myWrapperStorage, c.foeLane);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, c.egoResponse);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, c.foeResponse);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_junction_foe_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapJunctionFoeVector")
    }

    /// C++: `bool TraCIServer::wrapNextStopDataVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapNextStopDataVector(const std::string& /* objID */, const int variable, const std::vector<libsumo::TraCINextStopData>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 4);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     const bool full = variable == libsumo::VAR_NEXT_STOPS2;
    //     for (const auto& s : value) {
    //         const int legacyStopFlags = (s.stopFlags << 1) + (s.arrival >= 0 ? 1 : 0);
    //         StoHelp::writeTypedString(myWrapperStorage, s.lane);
    //         StoHelp::writeTypedDouble(myWrapperStorage, s.endPos);
    //         StoHelp::writeTypedString(myWrapperStorage, s.stoppingPlaceID);
    //         StoHelp::writeTypedInt(myWrapperStorage, full ? s.stopFlags : legacyStopFlags);
    //         StoHelp::writeTypedDouble(myWrapperStorage, s.duration);
    //         StoHelp::writeTypedDouble(myWrapperStorage, s.until);
    //         if (full) {
    //             StoHelp::writeTypedDouble(myWrapperStorage, s.startPos);
    //             StoHelp::writeTypedDouble(myWrapperStorage, s.intendedArrival);
    //             StoHelp::writeTypedDouble(myWrapperStorage, s.arrival);
    //             StoHelp::writeTypedDouble(myWrapperStorage, s.depart);
    //             StoHelp::writeTypedString(myWrapperStorage, s.split);
    //             StoHelp::writeTypedString(myWrapperStorage, s.join);
    //             StoHelp::writeTypedString(myWrapperStorage, s.actType);
    //             StoHelp::writeTypedString(myWrapperStorage, s.tripId);
    //             StoHelp::writeTypedString(myWrapperStorage, s.line);
    //             StoHelp::writeTypedDouble(myWrapperStorage, s.speed);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_next_stop_data_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapNextStopDataVector")
    }

    /// C++: `bool TraCIServer::wrapVehicleDataVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapVehicleDataVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCIVehicleData>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 5);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCIVehicleData& vd : value) {
    //         StoHelp::writeTypedString(myWrapperStorage, vd.id);
    //         StoHelp::writeTypedDouble(myWrapperStorage, vd.length);
    //         StoHelp::writeTypedDouble(myWrapperStorage, vd.entryTime);
    //         StoHelp::writeTypedDouble(myWrapperStorage, vd.leaveTime);
    //         StoHelp::writeTypedString(myWrapperStorage, vd.typeID);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_vehicle_data_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapVehicleDataVector")
    }

    /// C++: `bool TraCIServer::wrapBestLanesDataVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapBestLanesDataVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCIBestLanesData>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 6);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCIBestLanesData& bld : value) {
    //         StoHelp::writeTypedString(myWrapperStorage, bld.laneID);
    //         StoHelp::writeTypedDouble(myWrapperStorage, bld.length);
    //         StoHelp::writeTypedDouble(myWrapperStorage, bld.occupation);
    //         StoHelp::writeTypedByte(myWrapperStorage, bld.bestLaneOffset);
    //         StoHelp::writeTypedUnsignedByte(myWrapperStorage, bld.allowsContinuation ? 1 : 0);
    //         StoHelp::writeTypedStringList(myWrapperStorage, bld.continuationLanes);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_best_lanes_data_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapBestLanesDataVector")
    }

    /// C++: `bool TraCIServer::wrapNextTLSDataVector(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapNextTLSDataVector(const std::string& /* objID */, const int /* variable */, const std::vector<libsumo::TraCINextTLSData>& value) {
    //     StoHelp::writeCompound(myWrapperStorage, 1 + (int)value.size() * 4);
    //     StoHelp::writeTypedInt(myWrapperStorage, (int)value.size());
    //     for (const libsumo::TraCINextTLSData& tlsd : value) {
    //         StoHelp::writeTypedString(myWrapperStorage, tlsd.id);
    //         StoHelp::writeTypedInt(myWrapperStorage, tlsd.tlIndex);
    //         StoHelp::writeTypedDouble(myWrapperStorage, tlsd.dist);
    //         StoHelp::writeTypedByte(myWrapperStorage, tlsd.state);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_next_tls_data_vector(&mut self, objID: &str, variable: i32, value: &i32) -> bool {
        todo!("port TraCIServer::wrapNextTLSDataVector")
    }

    /// C++: `tcpip::Storage & TraCIServer::getWrapperStorage()`
    // tcpip::Storage&
    // TraCIServer::getWrapperStorage() {
    //     return myWrapperStorage;
    // }
    #[allow(unused_variables)]
    pub fn get_wrapper_storage(&mut self) -> &mut Storage {
        todo!("port TraCIServer::getWrapperStorage")
    }

    /// C++: `void TraCIServer::TraCIServer(const SUMOTime, const int, const int)` [private]
    /// Constructor
    // TraCIServer::TraCIServer(const SUMOTime begin, const int port, const int numClients)
    //     : myTargetTime(begin), myLastContextSubscription(nullptr) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "Creating new TraCIServer for " << numClients << " clients on port " << port << "." << std::endl;
    // #endif
    //     myVehicleStateChanges[MSNet::VehicleState::BUILT] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::DEPARTED] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::STARTING_TELEPORT] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::ENDING_TELEPORT] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::ARRIVED] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::NEWROUTE] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::STARTING_PARKING] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::MANEUVERING] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::ENDING_PARKING] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::STARTING_STOP] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::ENDING_STOP] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::COLLISION] = std::vector<std::string>();
    //     myVehicleStateChanges[MSNet::VehicleState::EMERGENCYSTOP] = std::vector<std::string>();
    //
    //     myTransportableStateChanges[MSNet::TransportableState::PERSON_DEPARTED] = std::vector<std::string>();
    //     myTransportableStateChanges[MSNet::TransportableState::PERSON_ARRIVED] = std::vector<std::string>();
    //     myTransportableStateChanges[MSNet::TransportableState::CONTAINER_DEPARTED] = std::vector<std::string>();
    //     myTransportableStateChanges[MSNet::TransportableState::CONTAINER_ARRIVED] = std::vector<std::string>();
    //
    //     myExecutors[libsumo::CMD_SET_INDUCTIONLOOP_VARIABLE] = &TraCIServerAPI_InductionLoop::processSet;
    //     myExecutors[libsumo::CMD_SET_LANEAREA_VARIABLE] = &TraCIServerAPI_LaneArea::processSet;
    //     myExecutors[libsumo::CMD_SET_MULTIENTRYEXIT_VARIABLE] = &TraCIServerAPI_MultiEntryExit::processSet;
    //
    //     myExecutors[libsumo::CMD_SET_TL_VARIABLE] = &TraCIServerAPI_TrafficLight::processSet;
    //     myExecutors[libsumo::CMD_SET_LANE_VARIABLE] = &TraCIServerAPI_Lane::processSet;
    //     myExecutors[libsumo::CMD_SET_VEHICLE_VARIABLE] = &TraCIServerAPI_Vehicle::processSet;
    //     myExecutors[libsumo::CMD_SET_VEHICLETYPE_VARIABLE] = &TraCIServerAPI_VehicleType::processSet;
    //     myExecutors[libsumo::CMD_SET_ROUTE_VARIABLE] = &TraCIServerAPI_Route::processSet;
    //     myExecutors[libsumo::CMD_SET_POI_VARIABLE] = &TraCIServerAPI_POI::processSet;
    //     myExecutors[libsumo::CMD_SET_POLYGON_VARIABLE] = &TraCIServerAPI_Polygon::processSet;
    //     myExecutors[libsumo::CMD_SET_JUNCTION_VARIABLE] = &TraCIServerAPI_Junction::processSet;
    //     myExecutors[libsumo::CMD_SET_EDGE_VARIABLE] = &TraCIServerAPI_Edge::processSet;
    //     myExecutors[libsumo::CMD_SET_SIM_VARIABLE] = &TraCIServerAPI_Simulation::processSet;
    //     myExecutors[libsumo::CMD_SET_PERSON_VARIABLE] = &TraCIServerAPI_Person::processSet;
    //     myExecutors[libsumo::CMD_SET_CALIBRATOR_VARIABLE] = &TraCIServerAPI_Calibrator::processSet;
    //     myExecutors[libsumo::CMD_SET_BUSSTOP_VARIABLE] = &TraCIServerAPI_BusStop::processSet;
    //     myExecutors[libsumo::CMD_SET_PARKINGAREA_VARIABLE] = &TraCIServerAPI_ParkingArea::processSet;
    //     myExecutors[libsumo::CMD_SET_CHARGINGSTATION_VARIABLE] = &TraCIServerAPI_ChargingStation::processSet;
    //     myExecutors[libsumo::CMD_SET_ROUTEPROBE_VARIABLE] = &TraCIServerAPI_RouteProbe::processSet;
    //     myExecutors[libsumo::CMD_SET_REROUTER_VARIABLE] = &TraCIServerAPI_Rerouter::processSet;
    //     myExecutors[libsumo::CMD_SET_VARIABLESPEEDSIGN_VARIABLE] = &TraCIServerAPI_VariableSpeedSign::processSet;
    //     //myExecutors[libsumo::CMD_SET_MEANDATA_VARIABLE] = &TraCIServerAPI_MeanData::processSet;
    //     myExecutors[libsumo::CMD_SET_OVERHEADWIRE_VARIABLE] = &TraCIServerAPI_OverheadWire::processSet;
    //
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_MEANDATA_VARIABLE, libsumo::VAR_MEANDATA_LANE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_MEANDATA_VARIABLE, libsumo::VAR_MEANDATA_VALUES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_EDGE_VARIABLE, libsumo::VAR_EDGE_TRAVELTIME));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_EDGE_VARIABLE, libsumo::VAR_EDGE_EFFORT));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_EDGE_VARIABLE, libsumo::VAR_ANGLE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_LANE_VARIABLE, libsumo::VAR_ANGLE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_LANE_VARIABLE, libsumo::LANE_CHANGES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_LANE_VARIABLE, libsumo::VAR_FOES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE, libsumo::DISTANCE_REQUEST));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE, libsumo::VAR_EDGES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE, libsumo::VAR_STAGE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE, libsumo::VAR_TAXI_RESERVATIONS));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE, libsumo::SPLIT_TAXI_RESERVATIONS));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::TL_BLOCKING_VEHICLES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::TL_RIVAL_VEHICLES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::TL_PRIORITY_VEHICLES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::TL_CONSTRAINT));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::TL_CONSTRAINT_BYFOE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_TL_VARIABLE, libsumo::VAR_PERSON_NUMBER));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::DISTANCE_REQUEST));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_EDGE_TRAVELTIME));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_EDGE_EFFORT));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_FOLLOW_SPEED));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_SECURE_GAP));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_STOP_SPEED));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_FOES));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::CMD_CHANGELANE));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_LEADER));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_FOLLOWER));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_NEIGHBORS));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_STOP_PARAMETER));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_NEXT_STOPS));  // this is just a dummy to trigger an error
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_NEXT_STOPS2));
    //     myParameterized.insert(std::make_pair(libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE, libsumo::VAR_TAXI_FLEET));
    //     myParameterized.insert(std::make_pair(0, libsumo::VAR_PARAMETER));
    //     myParameterized.insert(std::make_pair(0, libsumo::VAR_PARAMETER_WITH_KEY));
    //
    //     myDoCloseConnection = false;
    //
    //     // display warning if internal lanes are not used
    //     // TODO this may be redundant to the warning in NLBuilder::build
    //     if (!MSGlobals::gUsingInternalLanes && !MSGlobals::gUseMesoSim) {
    //         WRITE_WARNING(TL("Starting TraCI without using internal lanes!"));
    //         MsgHandler::getWarningInstance()->inform("Vehicles will jump over junctions.", false);
    //         MsgHandler::getWarningInstance()->inform("Use without option --no-internal-links to avoid unexpected behavior", false);
    //     }
    //
    //     try {
    //         WRITE_MESSAGEF(TL("***Starting server on port % ***"), toString(port));
    //         tcpip::Socket serverSocket(port);
    //         if (numClients > 1) {
    //             WRITE_MESSAGEF(TL("  waiting for % clients..."), toString(numClients));
    //         }
    //         while ((int)mySockets.size() < numClients) {
    //             int index = (int)mySockets.size() + libsumo::MAX_ORDER + 1;
    //             mySockets[index] = new SocketInfo(serverSocket.accept(true), begin);
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::BUILT] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::DEPARTED] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::STARTING_TELEPORT] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::ENDING_TELEPORT] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::ARRIVED] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::NEWROUTE] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::STARTING_PARKING] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::MANEUVERING] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::ENDING_PARKING] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::STARTING_STOP] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::ENDING_STOP] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::COLLISION] = std::vector<std::string>();
    //             mySockets[index]->vehicleStateChanges[MSNet::VehicleState::EMERGENCYSTOP] = std::vector<std::string>();
    //
    //             mySockets[index]->transportableStateChanges[MSNet::TransportableState::PERSON_DEPARTED] = std::vector<std::string>();
    //             mySockets[index]->transportableStateChanges[MSNet::TransportableState::PERSON_ARRIVED] = std::vector<std::string>();
    //             mySockets[index]->transportableStateChanges[MSNet::TransportableState::CONTAINER_DEPARTED] = std::vector<std::string>();
    //             mySockets[index]->transportableStateChanges[MSNet::TransportableState::CONTAINER_ARRIVED] = std::vector<std::string>();
    //             if (numClients > 1) {
    //                 WRITE_MESSAGE(TL("  client connected"));
    //             }
    //         }
    //         // When got here, all clients have connected
    //         if (numClients > 1) {
    //             checkClientOrdering();
    //         }
    //         // set myCurrentSocket != mySockets.end() to indicate that this is the first step in processCommands()
    //         myCurrentSocket = mySockets.begin();
    //     } catch (tcpip::SocketException& e) {
    //         throw ProcessError(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(begin: SUMOTime, port: i32, numClients: i32) -> Self {
        todo!("port TraCIServer::TraCIServer")
    }

    /// C++: `void TraCIServer::~TraCIServer()` [virtual, private]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port TraCIServer::~TraCIServer")
    }

    /// C++: `bool TraCIServer::commandGetVersion()` [private]
    /// Returns the TraCI-version
    // bool
    // TraCIServer::commandGetVersion() {
    //     // Prepare response
    //     tcpip::Storage answerTmp;
    //     answerTmp.writeInt(libsumo::TRACI_VERSION);
    //     answerTmp.writeString("SUMO " VERSION_STRING);
    //     // When we get here, the response is stored in answerTmp -> put into myOutputStorage
    //     writeStatusCmd(libsumo::CMD_GETVERSION, libsumo::RTYPE_OK, "");
    //     // command length
    //     myOutputStorage.writeUnsignedByte(1 + 1 + static_cast<int>(answerTmp.size()));
    //     // command type
    //     myOutputStorage.writeUnsignedByte(libsumo::CMD_GETVERSION);
    //     // and the parameter dependant part
    //     myOutputStorage.writeStorage(answerTmp);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn command_get_version(&mut self) -> bool {
        todo!("port TraCIServer::commandGetVersion")
    }

    /// C++: `void TraCIServer::postProcessSimulationStep()` [private]
    /// Handles subscriptions to send after a simstep2 command
    // void
    // TraCIServer::postProcessSimulationStep() {
    //     SUMOTime t = MSNet::getInstance()->getCurrentTimeStep();
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "   postProcessSimulationStep() at time=" << t << std::endl;
    // #endif
    //     writeStatusCmd(libsumo::CMD_SIMSTEP, libsumo::RTYPE_OK, "");
    //     int noActive = 0;
    //     for (std::vector<libsumo::Subscription>::iterator i = mySubscriptions.begin(); i != mySubscriptions.end();) {
    //         const libsumo::Subscription& s = *i;
    //         bool isArrivedVehicle = (s.commandId == libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE || s.commandId == libsumo::CMD_SUBSCRIBE_VEHICLE_CONTEXT)
    //                                 && (find(myVehicleStateChanges[MSNet::VehicleState::ARRIVED].begin(), myVehicleStateChanges[MSNet::VehicleState::ARRIVED].end(), s.id) != myVehicleStateChanges[MSNet::VehicleState::ARRIVED].end());
    //
    //         bool isArrivedPerson = (s.commandId == libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE || s.commandId == libsumo::CMD_SUBSCRIBE_PERSON_CONTEXT) && MSNet::getInstance()->getPersonControl().get(s.id) == nullptr;
    //         if ((s.endTime < t) || isArrivedVehicle || isArrivedPerson) {
    //             i = mySubscriptions.erase(i);
    //             continue;
    //         }
    //         ++i;
    //         if (s.beginTime > t) {
    //             continue;
    //         }
    //         ++noActive;
    //     }
    //     mySubscriptionCache.reset();
    // #ifdef DEBUG_SUBSCRIPTIONS
    //     std::cout << "   Initial size of mySubscriptionCache is " << mySubscriptionCache.size()
    //               << "\n   Nr. of active subscriptions = " << noActive << std::endl;
    // #endif
    //     mySubscriptionCache.writeInt(noActive);
    // #ifdef DEBUG_SUBSCRIPTIONS
    //     std::cout << "   Size after writing an int is " << mySubscriptionCache.size() << std::endl;
    // #endif
    //     for (std::vector<libsumo::Subscription>::iterator i = mySubscriptions.begin(); i != mySubscriptions.end();) {
    //         const libsumo::Subscription& s = *i;
    //         if (s.beginTime > t) {
    //             ++i;
    //             continue;
    //         }
    //         tcpip::Storage into;
    //         std::string errors;
    //         bool ok = processSingleSubscription(s, into, errors);
    // #ifdef DEBUG_SUBSCRIPTIONS
    //         std::cout << "   Size of into-store for subscription " << s.id
    //                   << ": " << into.size() << std::endl;
    // #endif
    //         mySubscriptionCache.writeStorage(into);
    //         if (ok) {
    //             ++i;
    //         } else {
    //             i = mySubscriptions.erase(i);
    //         }
    //     }
    //     myOutputStorage.writeStorage(mySubscriptionCache);
    // #ifdef DEBUG_SUBSCRIPTIONS
    //     std::cout << "   Size after writing subscriptions is " << mySubscriptionCache.size() << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn post_process_simulation_step(&mut self) {
        todo!("port TraCIServer::postProcessSimulationStep")
    }

    /// C++: `int TraCIServer::readCommandID(int &, int &)` [private]
    /// Reads the next command ID from the input storage
    // int
    // TraCIServer::readCommandID(int& commandStart, int& commandLength) {
    //     commandStart = myInputStorage.position();
    //     commandLength = myInputStorage.readUnsignedByte();
    //     if (commandLength == 0) {
    //         commandLength = myInputStorage.readInt();
    //     }
    // #ifdef DEBUG_RAW_INPUT
    //     std::cout << " commandStart=" << commandStart << " commandLength=" << commandLength << " pos=" << myInputStorage.position() << " raw=";
    //     for (auto it = myInputStorage.begin(); it != myInputStorage.end(); ++it) {
    //         std::cout << (int)*it << " ";
    //     }
    //     std::cout << "\n";
    // #endif
    //     return myInputStorage.readUnsignedByte();
    // }
    #[allow(unused_variables)]
    pub fn read_command_id(&mut self, commandStart: &mut i32, commandLength: &mut i32) -> i32 {
        todo!("port TraCIServer::readCommandID")
    }

    /// C++: `int TraCIServer::dispatchCommand()` [private]
    /// Handles command, writes response to myOutputStorage
    // int
    // TraCIServer::dispatchCommand() {
    //     int commandStart, commandLength;
    //     int commandId = readCommandID(commandStart, commandLength);
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "       dispatchCommand() called for client " << myCurrentSocket->second->socket
    //               << ", commandId = " << commandId << std::endl;
    // #endif
    //     bool success = false;
    //     // dispatch commands
    //     if (myExecutors.find(commandId) != myExecutors.end()) {
    //         success = myExecutors[commandId](*this, myInputStorage, myOutputStorage);
    //     } else if ((commandId >= libsumo::CMD_GET_INDUCTIONLOOP_VARIABLE && commandId <= libsumo::CMD_GET_BUSSTOP_VARIABLE)
    //                || (commandId >= libsumo::CMD_GET_PARKINGAREA_VARIABLE && commandId <= libsumo::CMD_GET_OVERHEADWIRE_VARIABLE)) {
    //         if (commandId == libsumo::CMD_GET_GUI_VARIABLE) {
    //             writeStatusCmd(commandId, libsumo::RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo");
    //         } else {
    //             success = processGet(commandId, myInputStorage, myOutputStorage);
    //         }
    //     } else {
    //         switch (commandId) {
    //             case libsumo::CMD_GETVERSION:
    //                 success = commandGetVersion();
    //                 break;
    //             case libsumo::CMD_LOAD: {
    //                 try {
    //                     const std::vector<std::string> args = StoHelp::readTypedStringList(myInputStorage, "A load command needs a list of string arguments.");
    // #ifdef DEBUG_MULTI_CLIENTS
    //                     std::cout << "       commandId == libsumo::CMD_LOAD"
    //                               << ", args = " << toString(args) << std::endl;
    // #endif
    //                     myLoadArgs = args;
    //                     success = true;
    //                     writeStatusCmd(libsumo::CMD_LOAD, libsumo::RTYPE_OK, "");
    //                     // XXX: This only cares for the client that issued the load command.
    //                     // Multiclient-load functionality is still to be implemented. Refs #3146.
    //                     myCurrentSocket->second->socket->sendExact(myOutputStorage);
    //                     myCurrentSocket = mySockets.end();
    //                     myOutputStorage.reset();
    //                 } catch (libsumo::TraCIException& e) {
    //                     return writeErrorStatusCmd(libsumo::CMD_LOAD, e.what(), myOutputStorage);
    //                 }
    //                 break;
    //             }
    //             case libsumo::CMD_EXECUTEMOVE:
    //                 myCurrentSocket->second->executeMove = true;
    //                 myCurrentSocket++;
    //                 success = true;
    //                 writeStatusCmd(libsumo::CMD_EXECUTEMOVE, libsumo::RTYPE_OK, "");
    //                 break;
    //             case libsumo::CMD_SIMSTEP: {
    //                 const double nextT = myInputStorage.readDouble();
    //                 if (nextT == 0.) {
    //                     myCurrentSocket->second->targetTime += DELTA_T;
    //                 } else {
    //                     myCurrentSocket->second->targetTime = TIME2STEPS(nextT);
    //                 }
    //                 myCurrentSocket->second->executeMove = false;
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "       commandId == libsumo::CMD_SIMSTEP"
    //                           << ", next target time for client is " << myCurrentSocket->second->targetTime << std::endl;
    // #endif
    //                 if (myCurrentSocket->second->targetTime <= MSNet::getInstance()->getCurrentTimeStep()) {
    //                     // This is not the last TraCI simstep in the current SUMO simstep -> send single simstep response.
    //                     // @note: In the other case the simstep results are sent to all after the SUMO step was performed, see entry point for processCommands()
    //                     sendSingleSimStepResponse();
    //                 }
    //                 // Clear vehicleStateChanges and transportableStateChanges for this client
    //                 // -> For subsequent TraCI stepping
    //                 // that is performed within this SUMO step, no updates on vehicle states
    //                 // belonging to the last SUMO simulation step will be received by this client.
    //                 for (auto& item : myCurrentSocket->second->vehicleStateChanges) {
    //                     item.second.clear();
    //                 }
    //                 for (auto& item : myCurrentSocket->second->transportableStateChanges) {
    //                     item.second.clear();
    //                 }
    //                 myCurrentSocket++;
    //                 return commandId;
    //             }
    //             case libsumo::CMD_CLOSE:
    //                 writeStatusCmd(libsumo::CMD_CLOSE, libsumo::RTYPE_OK, "");
    //                 myCurrentSocket->second->socket->sendExact(myOutputStorage);
    //                 myOutputStorage.reset();
    //                 if (mySockets.size() == 1) {
    //                     // Last client has closed connection
    //                     myDoCloseConnection = true;
    //                 }
    //                 // remove current socket and increment to next socket in ordering
    //                 myCurrentSocket = removeCurrentSocket();
    //                 success = true;
    //                 break;
    //             case libsumo::CMD_SETORDER: {
    //                 const int order = myInputStorage.readInt();
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "       commandId == libsumo::CMD_SETORDER"
    //                           << ", order index is " << order << std::endl;
    // #endif
    //                 if (order > libsumo::MAX_ORDER) {
    //                     return writeErrorStatusCmd(libsumo::CMD_SETORDER, "A set order command needs an int argument below " + toString(libsumo::MAX_ORDER) + ".", myOutputStorage);
    //                 }
    //                 if (mySockets.count(order) > 0 || mySocketReorderRequests.count(order) > 0) {
    //                     return writeErrorStatusCmd(libsumo::CMD_SETORDER, "Order '" + toString(order) + "' is already taken.", myOutputStorage);
    //                 }
    //                 // memorize reorder request (will only take effect in the next step)
    //                 mySocketReorderRequests[order] = myCurrentSocket->second;
    //                 success = true;
    //                 writeStatusCmd(libsumo::CMD_SETORDER, libsumo::RTYPE_OK, "");
    //                 break;
    //             }
    //             case libsumo::CMD_SUBSCRIBE_BUSSTOP_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_CALIBRATOR_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_CHARGINGSTATION_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_EDGE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_GUI_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_JUNCTION_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_LANE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_LANEAREA_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_MEANDATA_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_OVERHEADWIRE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_PARKINGAREA_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_PERSON_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_POI_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_POLYGON_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_REROUTER_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_ROUTE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_SIM_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_TL_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_VARIABLESPEEDSIGN_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_VEHICLE_VARIABLE:
    //             case libsumo::CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE:
    //                 success = addObjectVariableSubscription(commandId, false);
    //                 break;
    //             case libsumo::CMD_SUBSCRIBE_BUSSTOP_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_CALIBRATOR_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_EDGE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_GUI_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_JUNCTION_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_LANE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_LANEAREA_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_MEANDATA_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_OVERHEADWIRE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_PARKINGAREA_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_PERSON_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_POI_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_POLYGON_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_REROUTER_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_ROUTE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_SIM_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_TL_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_VARIABLESPEEDSIGN_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_VEHICLE_CONTEXT:
    //             case libsumo::CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT:
    //                 success = addObjectVariableSubscription(commandId, true);
    //                 break;
    //             case libsumo::CMD_ADD_SUBSCRIPTION_FILTER:
    //                 success = addSubscriptionFilter();
    //                 break;
    //             case libsumo::CMD_SET_GUI_VARIABLE:
    //                 writeStatusCmd(commandId, libsumo::RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo");
    //                 break;
    //             default:
    //                 writeStatusCmd(commandId, libsumo::RTYPE_NOTIMPLEMENTED, "Command not implemented in sumo");
    //         }
    //     }
    //     if (!success) {
    //         while (myInputStorage.valid_pos() && (int)myInputStorage.position() < commandStart + commandLength) {
    //             myInputStorage.readChar();
    //         }
    //     }
    //     if ((int)myInputStorage.position() != commandStart + commandLength) {
    //         std::ostringstream msg;
    //         msg << "Wrong position in requestMessage after dispatching command " << commandId << ".";
    //         msg << " Expected command length was " << commandLength;
    //         msg << " but " << myInputStorage.position() - commandStart << " Bytes were read.";
    //         writeStatusCmd(commandId, libsumo::RTYPE_ERR, msg.str());
    //         myDoCloseConnection = true;
    //     }
    //     return commandId;
    // }
    #[allow(unused_variables)]
    pub fn dispatch_command(&mut self) -> i32 {
        todo!("port TraCIServer::dispatchCommand")
    }

    /// C++: `void TraCIServer::checkClientOrdering()` [private]
    /// Called once after connection of all clients for executing SET_ORDER (and possibly prior GET_VERSION) commands, that should be executed before simulation starts (in processCommandsUntilNextSimStep()).
    // void
    // TraCIServer::checkClientOrdering() {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "Checking client order requests." << std::endl;
    // #endif
    //     // check for SET_ORDER commands queued by connected clients
    //     // In multiclient cas it is mandatory that SET_ORDER  is sent as the first command (or directly after GET_VERSION)
    //     myCurrentSocket = mySockets.begin();
    //     while (myCurrentSocket != mySockets.end()) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //         std::cout << "  Socket " << myCurrentSocket->second->socket << ":" << std::endl;
    // #endif
    // //        bool clientUnordered = true;
    //         while (true) {
    //             myInputStorage.reset();
    //             myCurrentSocket->second->socket->receiveExact(myInputStorage);
    //             int commandStart, commandLength;
    //             int commandId = readCommandID(commandStart, commandLength);
    // #ifdef DEBUG_MULTI_CLIENTS
    //             std::cout << "    received command " << commandId << std::endl;
    // #endif
    //             // Whether the received command is a permitted command for the initialization phase.
    //             // Currently, getVersion and setOrder are permitted.
    //             bool initCommand = commandId == libsumo::CMD_SETORDER || commandId == libsumo::CMD_GETVERSION;
    //             if (initCommand) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "    Init command. Sending response." << std::endl;
    // #endif
    //                 // reset input storage to initial state before reading the commandId
    //                 // (ugly, but we can't just reset the store's iter_ from here)
    //                 // Giving the commandId to dispatch command didn't work either
    //                 tcpip::Storage tmp;
    //                 tmp.writeStorage(myInputStorage);
    //                 myInputStorage.reset();
    //                 // we don't know whether the command was set with extended
    //                 // length syntax or not so we hardcode the length here (#5037)
    //                 myInputStorage.writeUnsignedByte(commandId == libsumo::CMD_SETORDER ? 6 : 2);
    //                 myInputStorage.writeUnsignedByte(commandId);
    //                 myInputStorage.writeStorage(tmp);
    //
    //                 // Handle initialization command completely
    //                 dispatchCommand();
    //                 myCurrentSocket->second->socket->sendExact(myOutputStorage);
    //                 myOutputStorage.reset();
    //             } else {
    // #ifdef DEBUG_MULTI_CLIENTS
    //                 std::cout << "    Client " << myCurrentSocket->second->socket << " did not set order initially." << std::endl;
    // #endif
    //                 throw ProcessError(TL("Execution order (libsumo::CMD_SETORDER) was not set for all TraCI clients in pre-execution phase."));
    //             }
    //             if (commandId == libsumo::CMD_SETORDER) {
    //                 // This is what we have waited for.
    //                 break;
    //             }
    //         }
    //         ++myCurrentSocket;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_client_ordering(&mut self) {
        todo!("port TraCIServer::checkClientOrdering")
    }

    /// C++: `void TraCIServer::processReorderingRequests()` [private]
    /// checks for and processes reordering requests (relevant for multiple clients)
    // void
    // TraCIServer::processReorderingRequests() {
    //     // Process reordering requests
    //     if (mySocketReorderRequests.size() > 0) {
    //         // process reordering requests
    //         std::map<int, SocketInfo*>::const_iterator i = mySocketReorderRequests.begin();
    //         std::map<int, SocketInfo*>::iterator j;
    // #ifdef DEBUG_MULTI_CLIENTS
    //         std::cout << SIMTIME << " Current socket ordering:\n";
    //         for (j = mySockets.begin(); j != mySockets.end(); ++j) {
    //             std::cout << "      " << j->first << ": " << j->second->socket << "\n";
    //         }
    //         std::cout << "Reordering requests:\n";
    //         for (i = mySocketReorderRequests.begin(); i != mySocketReorderRequests.end(); ++i) {
    //             std::cout << "      Socket " << i->second->socket << " -> " << i->first << "\n";
    //         }
    //         i = mySocketReorderRequests.begin();
    // #endif
    //         while (i != mySocketReorderRequests.end()) {
    //             j = mySockets.begin();
    //             while (j != mySockets.end()) {
    //                 if (j->second->socket == i->second->socket) {
    //                     break;
    //                 } else {
    //                     j++;
    //                 }
    //             }
    //             assert(j != mySockets.end());
    //             mySockets.erase(j);
    //             mySockets[i->first] = i->second;
    //             ++i;
    //         }
    //         mySocketReorderRequests.clear();
    // #ifdef DEBUG_MULTI_CLIENTS
    //         std::cout << "New socket ordering:\n";
    //         for (j = mySockets.begin(); j != mySockets.end(); ++j) {
    //             std::cout << "      " << j->first << ": " << j->second->socket << "\n";
    //         }
    //         std::cout << std::endl;
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn process_reordering_requests(&mut self) {
        todo!("port TraCIServer::processReorderingRequests")
    }

    /// C++: `SUMOTime TraCIServer::nextTargetTime() const` [const, private]
    /// get the minimal next target time among all clients
    // SUMOTime
    // TraCIServer::nextTargetTime() const {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "\n    Determining new target time..." << std::endl;
    //     if (mySockets.size() == 0) {
    //         std::cout << "    All clients have disconnected." << std::endl;
    //     }
    // #endif
    //     std::map<int, SocketInfo*>::const_iterator i;
    //     SUMOTime targetTime = std::numeric_limits<SUMOTime>::max();
    //     for (i = mySockets.begin(); i != mySockets.end(); ++i) {
    // #ifdef DEBUG_MULTI_CLIENTS
    //         std::cout << "    target time for client " << i->second->socket << ": " << i->second->targetTime << "\n";
    // #endif
    //         targetTime = MIN2(targetTime, i->second->targetTime);
    //     }
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << std::endl;
    // #endif
    //     return targetTime;
    // }
    #[allow(unused_variables)]
    pub fn next_target_time(&self) -> SUMOTime {
        todo!("port TraCIServer::nextTargetTime")
    }

    /// C++: `void TraCIServer::sendOutputToAll() const` [const, private]
    /// send out subscription results (actually just the content of myOutputStorage) to clients which will act in this step (i.e. with client target time <= myTargetTime)
    // void
    // TraCIServer::sendOutputToAll() const {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "\n    Sending subscription results to clients:\n";
    // #endif
    //     std::map<int, SocketInfo*>::const_iterator i = mySockets.begin();
    //     while (i != mySockets.end()) {
    //         if (i->second->targetTime <= MSNet::getInstance()->getCurrentTimeStep()) {
    //             // this client will become active before the next SUMO step. Provide subscription results.
    //             i->second->socket->sendExact(myOutputStorage);
    // #ifdef DEBUG_MULTI_CLIENTS
    //             std::cout << i->second->socket << "\n";
    // #endif
    //         }
    //         ++i;
    //     }
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << std::endl;
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn send_output_to_all(&self) {
        todo!("port TraCIServer::sendOutputToAll")
    }

    /// C++: `void TraCIServer::sendSingleSimStepResponse()` [private]
    /// sends an empty response to a simstep command to the current client. (This applies to a situation where the TraCI step frequency is higher than the SUMO step frequency)
    // void
    // TraCIServer::sendSingleSimStepResponse() {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "       Sending cached simstep response to current client " << myCurrentSocket->second->socket
    //               << " (-> intermediate TraCI step)."
    //               << "\n       Size of mySubscriptionCache is " << mySubscriptionCache.size()
    //               << std::endl;
    // #endif
    //     writeStatusCmd(libsumo::CMD_SIMSTEP, libsumo::RTYPE_OK, "");
    //
    // // NOTE: the commented code would send an empty response
    // //    myOutputStorage.writeInt(0);
    // //    myCurrentSocket->second->socket->sendExact(myOutputStorage);
    // //    myOutputStorage.reset();
    //     myOutputStorage.writeStorage(mySubscriptionCache);
    //     // send results to active client
    //     myCurrentSocket->second->socket->sendExact(myOutputStorage);
    //     myOutputStorage.reset();
    // }
    #[allow(unused_variables)]
    pub fn send_single_sim_step_response(&mut self) {
        todo!("port TraCIServer::sendSingleSimStepResponse")
    }

    /// C++: `int TraCIServer::removeCurrentSocket()` [private]
    /// removes myCurrentSocket from mySockets and returns an iterator pointing to the next member according to the ordering
    // std::map<int, TraCIServer::SocketInfo*>::iterator
    // TraCIServer::removeCurrentSocket() {
    // #ifdef DEBUG_MULTI_CLIENTS
    //     std::cout << "       Removing socket " << myCurrentSocket->second->socket
    //               << " (order " << myCurrentSocket->first << ")" << std::endl;
    // #endif
    //     delete myCurrentSocket->second;
    //     myCurrentSocket = mySockets.erase(myCurrentSocket);
    //     return myCurrentSocket;
    // }
    #[allow(unused_variables)]
    pub fn remove_current_socket(&mut self) -> i32 {
        todo!("port TraCIServer::removeCurrentSocket")
    }

    /// C++: `bool TraCIServer::addObjectVariableSubscription(const int, const bool)` [private]
    // bool
    // TraCIServer::addObjectVariableSubscription(const int commandId, const bool hasContext) {
    //     const double beginTime = myInputStorage.readDouble();
    //     const double endTime = myInputStorage.readDouble();
    //     const SUMOTime begin = beginTime == libsumo::INVALID_DOUBLE_VALUE ? 0 : TIME2STEPS(beginTime);
    //     const SUMOTime end = endTime == libsumo::INVALID_DOUBLE_VALUE || endTime > STEPS2TIME(SUMOTime_MAX) ? SUMOTime_MAX : TIME2STEPS(endTime);
    //     const std::string id = myInputStorage.readString();
    //     const int domain = hasContext ? myInputStorage.readUnsignedByte() : 0;
    //     double range = hasContext ? myInputStorage.readDouble() : 0.;
    //     if (commandId == libsumo::CMD_SUBSCRIBE_SIM_CONTEXT) {
    //         range = std::numeric_limits<double>::max();
    //     }
    //     const int num = myInputStorage.readUnsignedByte();
    //     std::vector<int> variables;
    //     std::vector<std::shared_ptr<tcpip::Storage> > parameters;
    //     for (int i = 0; i < num; ++i) {
    //         const int varID = myInputStorage.readUnsignedByte();
    //         variables.push_back(varID);
    //         parameters.push_back(std::make_shared<tcpip::Storage>());
    //         if ((myParameterized.count(std::make_pair(0, varID)) > 0) || (myParameterized.count(std::make_pair(commandId, varID)) > 0)) {
    //             if (!myInputStorage.valid_pos()) {
    //                 writeStatusCmd(commandId, libsumo::RTYPE_ERR, "Missing parameter for subscription " + toHex(commandId, 2));
    //                 return false;
    //             }
    //             int count = 1;
    //             while (count-- > 0) {
    //                 const int parType = myInputStorage.readUnsignedByte();
    //                 parameters.back()->writeUnsignedByte(parType);
    //                 if (parType == libsumo::TYPE_DOUBLE) {
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                 } else if (parType == libsumo::TYPE_INTEGER) {
    //                     parameters.back()->writeInt(myInputStorage.readInt());
    //                 } else if (parType == libsumo::TYPE_STRING) {
    //                     parameters.back()->writeString(myInputStorage.readString());
    //                 } else if (parType == libsumo::TYPE_BYTE) {
    //                     parameters.back()->writeByte(myInputStorage.readByte());
    //                 } else if (parType == libsumo::TYPE_UBYTE) {
    //                     parameters.back()->writeUnsignedByte(myInputStorage.readUnsignedByte());
    //                 } else if (parType == libsumo::POSITION_2D) {
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     if (varID == libsumo::DISTANCE_REQUEST) {
    //                         parameters.back()->writeUnsignedByte(myInputStorage.readUnsignedByte());
    //                         break;
    //                     }
    //                 } else if (parType == libsumo::POSITION_3D) {
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     if (varID == libsumo::DISTANCE_REQUEST) {
    //                         parameters.back()->writeUnsignedByte(myInputStorage.readUnsignedByte());
    //                         break;
    //                     }
    //                 } else if (parType == libsumo::POSITION_ROADMAP) {
    //                     parameters.back()->writeString(myInputStorage.readString());
    //                     parameters.back()->writeDouble(myInputStorage.readDouble());
    //                     parameters.back()->writeUnsignedByte(myInputStorage.readUnsignedByte());
    //                     if (varID == libsumo::DISTANCE_REQUEST) {
    //                         parameters.back()->writeUnsignedByte(myInputStorage.readUnsignedByte());
    //                         break;
    //                     }
    //                 } else if (parType == libsumo::TYPE_COMPOUND) {
    //                     count = myInputStorage.readInt();
    //                     parameters.back()->writeInt(count);
    //                 } else {
    //                     writeStatusCmd(commandId, libsumo::RTYPE_ERR, "Invalid parameter for subscription " + toHex(commandId, 2));
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     // check subscribe/unsubscribe
    //     if (variables.empty()) {
    //         removeSubscription(commandId, id, domain);
    //         return true;
    //     }
    //     // process subscription
    //     libsumo::Subscription s(commandId, id, variables, parameters, begin, end, domain, range);
    //     initialiseSubscription(s);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_object_variable_subscription(&mut self, commandId: i32, hasContext: bool) -> bool {
        todo!("port TraCIServer::addObjectVariableSubscription")
    }

    /// C++: `void TraCIServer::initialiseSubscription(libsumo::Subscription &)` [private]
    // void
    // TraCIServer::initialiseSubscription(libsumo::Subscription& s) {
    //     tcpip::Storage writeInto;
    //     std::string errors;
    //     libsumo::Subscription* modifiedSubscription = nullptr;
    //     try {
    //         if (processSingleSubscription(s, writeInto, errors)) {
    //             if (s.endTime < MSNet::getInstance()->getCurrentTimeStep()) {
    //                 writeStatusCmd(s.commandId, libsumo::RTYPE_ERR, "Subscription has ended.");
    //             } else {
    //                 if (libsumo::Helper::needNewSubscription(s, mySubscriptions, modifiedSubscription)) {
    //                     // Add new subscription to subscription cache (note: seems a bit inefficient)
    //                     if (s.beginTime < MSNet::getInstance()->getCurrentTimeStep()) {
    //                         // copy new subscription into cache
    //                         int noActive = 1 + (mySubscriptionCache.size() > 0 ? mySubscriptionCache.readInt() : 0);
    //                         tcpip::Storage tmp;
    //                         tmp.writeInt(noActive);
    //                         while (mySubscriptionCache.valid_pos()) {
    //                             tmp.writeByte(mySubscriptionCache.readByte());
    //                         }
    //                         tmp.writeStorage(writeInto);
    //                         mySubscriptionCache.reset();
    //                         mySubscriptionCache.writeStorage(tmp);
    //                     }
    //                 }
    //                 writeStatusCmd(s.commandId, libsumo::RTYPE_OK, "");
    //             }
    //             if (modifiedSubscription != nullptr && (
    //                         modifiedSubscription->isVehicleToVehicleContextSubscription()
    //                         || modifiedSubscription->isVehicleToPersonContextSubscription())) {
    //                 // Set last modified vehicle context subscription active for filter modifications
    //                 myLastContextSubscription = modifiedSubscription;
    //             } else {
    //                 // adding other subscriptions deactivates the activation for filter addition
    //                 myLastContextSubscription = nullptr;
    //             }
    //         } else {
    //             writeStatusCmd(s.commandId, libsumo::RTYPE_ERR, "Could not add subscription. " + errors);
    //         }
    //     } catch (libsumo::TraCIException& e) {
    //         writeStatusCmd(s.commandId, libsumo::RTYPE_ERR, e.what());
    //     }
    //     myOutputStorage.writeStorage(writeInto);
    // }
    #[allow(unused_variables)]
    pub fn initialise_subscription(&mut self, s: &mut Subscription) {
        todo!("port TraCIServer::initialiseSubscription")
    }

    /// C++: `void TraCIServer::removeSubscription(int, const std::string &, int)` [private]
    // void
    // TraCIServer::removeSubscription(int commandId, const std::string& id, int domain) {
    //     bool found = false;
    //     std::vector<libsumo::Subscription>::iterator j;
    //     for (j = mySubscriptions.begin(); j != mySubscriptions.end();) {
    //         if (j->id == id && j->commandId == commandId && j->contextDomain == domain) {
    //             j = mySubscriptions.erase(j);
    //             if (j != mySubscriptions.end() && myLastContextSubscription == &(*j)) {
    //                 // Remove also reference for filter additions
    //                 myLastContextSubscription = nullptr;
    //             }
    //             found = true;
    //             continue;
    //         }
    //         ++j;
    //     }
    //     // try unsubscribe
    //     if (found) {
    //         writeStatusCmd(commandId, libsumo::RTYPE_OK, "");
    //     } else {
    //         writeStatusCmd(commandId, libsumo::RTYPE_ERR, "The subscription to remove was not found.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_subscription(&mut self, commandId: i32, identity: &str, domain: i32) {
        todo!("port TraCIServer::removeSubscription")
    }

    /// C++: `bool TraCIServer::processSingleSubscription(const libsumo::Subscription &, tcpip::Storage &, std::string &)` [private]
    // bool
    // TraCIServer::processSingleSubscription(const libsumo::Subscription& s, tcpip::Storage& writeInto,
    //                                        std::string& errors) {
    //     bool ok = true;
    //     tcpip::Storage outputStorage;
    //     const int getCommandId = s.contextDomain > 0 ? s.contextDomain : s.commandId - 0x30;
    //     std::set<std::string> objIDs;
    //     if (s.contextDomain > 0) {
    //         if ((s.activeFilters & libsumo::SUBS_FILTER_NO_RTREE) == 0) {
    //             PositionVector shape;
    //             libsumo::Helper::findObjectShape(s.commandId, s.id, shape);
    //             libsumo::Helper::collectObjectIDsInRange(s.contextDomain, shape, s.range, objIDs);
    //         }
    //         libsumo::Helper::applySubscriptionFilters(s, objIDs);
    //     } else {
    //         objIDs.insert(s.id);
    //     }
    //     const int numVars = s.contextDomain > 0 && s.variables.size() == 1 && s.variables[0] == libsumo::TRACI_ID_LIST ? 0 : (int)s.variables.size();
    //     int skipped = 0;
    //     for (std::set<std::string>::iterator j = objIDs.begin(); j != objIDs.end(); ++j) {
    //         if (s.contextDomain > 0) {
    //             //if (centralObject(s, *j)) {
    //             //    skipped++;
    //             //    continue;
    //             //}
    //             outputStorage.writeString(*j);
    //         }
    //         if (numVars > 0) {
    //             std::vector<std::shared_ptr<tcpip::Storage> >::const_iterator k = s.parameters.begin();
    //             for (std::vector<int>::const_iterator i = s.variables.begin(); i != s.variables.end(); ++i, ++k) {
    //                 tcpip::Storage message;
    //                 message.writeUnsignedByte(*i);
    //                 message.writeString(*j);
    //                 // TODO check why writeStorage fails here (probably some kind of invalid iterator)
    //                 for (const auto& v :** k) {
    //                     message.writeChar(v);
    //                 }
    //                 tcpip::Storage tmpOutput;
    //                 try {
    //                     if (myExecutors.find(getCommandId) != myExecutors.end()) {
    //                         ok &= myExecutors[getCommandId](*this, message, tmpOutput);
    //                     } else if (!processGet(getCommandId, message, tmpOutput)) {
    //                         if (getCommandId == libsumo::CMD_GET_GUI_VARIABLE) {
    //                             writeStatusCmd(s.commandId, libsumo::RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo", tmpOutput);
    //                         }
    //                         ok = false;
    //                     }
    //                 } catch (const std::invalid_argument&) {
    //                     writeStatusCmd(s.commandId, libsumo::RTYPE_NOTIMPLEMENTED, "Unsupported command specified", tmpOutput);
    //                     ok = false;
    //                 }
    //                 // copy response part
    //                 if (ok) {
    //                     int length = tmpOutput.readUnsignedByte();
    //                     while (--length > 0) {
    //                         tmpOutput.readUnsignedByte();
    //                     }
    //                     int lengthLength = 1;
    //                     length = tmpOutput.readUnsignedByte();
    //                     if (length == 0) {
    //                         lengthLength = 5;
    //                         length = tmpOutput.readInt();
    //                     }
    //                     //read responseType
    //                     tmpOutput.readUnsignedByte();
    //                     int variable = tmpOutput.readUnsignedByte();
    //                     std::string id = tmpOutput.readString();
    //                     outputStorage.writeUnsignedByte(variable);
    //                     outputStorage.writeUnsignedByte(libsumo::RTYPE_OK);
    //                     length -= (lengthLength + 1 + 4 + (int)id.length());
    //                     while (--length > 0) {
    //                         outputStorage.writeUnsignedByte(tmpOutput.readUnsignedByte());
    //                     }
    //                 } else {
    //                     //read length
    //                     tmpOutput.readUnsignedByte();
    //                     //read cmd
    //                     tmpOutput.readUnsignedByte();
    //                     //read status
    //                     tmpOutput.readUnsignedByte();
    //                     std::string msg = tmpOutput.readString();
    //                     outputStorage.writeUnsignedByte(*i);
    //                     outputStorage.writeUnsignedByte(libsumo::RTYPE_ERR);
    //                     outputStorage.writeUnsignedByte(libsumo::TYPE_STRING);
    //                     outputStorage.writeString(msg);
    //                     errors = errors + msg;
    //                 }
    //             }
    //         }
    //     }
    //     int length = (1 + 4) + 1 + (4 + (int)s.id.length()) + 1 + (int)outputStorage.size();
    //     if (s.contextDomain > 0) {
    //         length += 1 + 4;  // context domain and number of objects
    //     }
    //     // we always write extended command length here for backward compatibility
    //     writeInto.writeUnsignedByte(0); // command length -> extended
    //     writeInto.writeInt(length);
    //     writeInto.writeUnsignedByte(s.commandId + 0x10);
    //     writeInto.writeString(s.id);
    //     if (s.contextDomain > 0) {
    //         writeInto.writeUnsignedByte(s.contextDomain);
    //     }
    //     writeInto.writeUnsignedByte(numVars);
    //     if (s.contextDomain > 0) {
    //         writeInto.writeInt((int)objIDs.size() - skipped);
    //     }
    //     if (s.contextDomain == 0 || objIDs.size() != 0) {
    //         writeInto.writeStorage(outputStorage);
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn process_single_subscription(&mut self, s: &Subscription, writeInto: &mut Storage, errors: &mut String) -> bool {
        todo!("port TraCIServer::processSingleSubscription")
    }

    /// C++: `bool TraCIServer::addSubscriptionFilter()` [private]
    // bool
    // TraCIServer::addSubscriptionFilter() {
    //     bool success  = true;
    //     // Read filter type
    //     int filterType = myInputStorage.readUnsignedByte();
    //
    //     if (myLastContextSubscription == nullptr) {
    //         writeStatusCmd(libsumo::CMD_ADD_SUBSCRIPTION_FILTER, libsumo::RTYPE_ERR,
    //                        "No previous vehicle context subscription exists to apply filter type " + toHex(filterType, 2));
    //         return false;
    //     }
    //
    //     // dispatch according to filter type
    //     switch (filterType) {
    //         case libsumo::FILTER_TYPE_NONE:
    //             // Remove all filters
    //             removeFilters();
    //             break;
    //         case libsumo::FILTER_TYPE_LANES: {
    //             // Read relative lanes to consider for context filter
    //             int nrLanes = (int)myInputStorage.readByte();
    //             std::vector<int> lanes;
    //             for (int i = 0; i < nrLanes; ++i) {
    //                 lanes.push_back((int) myInputStorage.readByte());
    //             }
    //             addSubscriptionFilterLanes(lanes);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_NOOPPOSITE:
    //             // Add no-opposite filter
    //             addSubscriptionFilterNoOpposite();
    //             break;
    //         case libsumo::FILTER_TYPE_DOWNSTREAM_DIST: {
    //             myInputStorage.readByte(); // read type double
    //             double dist = myInputStorage.readDouble();
    //             addSubscriptionFilterDownstreamDistance(dist);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_UPSTREAM_DIST: {
    //             myInputStorage.readByte(); // read type double
    //             double dist = myInputStorage.readDouble();
    //             addSubscriptionFilterUpstreamDistance(dist);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_LEAD_FOLLOW: {
    //             // Read relative lanes to consider for context filter
    //             addSubscriptionFilterLeadFollow();
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_TURN: {
    //             myInputStorage.readByte(); // read type double
    //             double dist = myInputStorage.readDouble();
    //             addSubscriptionFilterTurn(dist);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_VCLASS: {
    //             myInputStorage.readByte(); // read type stringlist
    //             SVCPermissions vClasses = parseVehicleClasses(myInputStorage.readStringList());
    //             addSubscriptionFilterVClass(vClasses);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_VTYPE: {
    //             myInputStorage.readByte(); // read type stringlist
    //             std::vector<std::string> vTypesVector = myInputStorage.readStringList();
    //             std::set<std::string> vTypesSet;
    //             vTypesSet.insert(vTypesVector.begin(), vTypesVector.end());
    //             addSubscriptionFilterVType(vTypesSet);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_FIELD_OF_VISION: {
    //             myInputStorage.readByte();  // read type double
    //             double angle = myInputStorage.readDouble();
    //             addSubscriptionFilterFieldOfVision(angle);
    //         }
    //         break;
    //         case libsumo::FILTER_TYPE_LATERAL_DIST: {
    //             myInputStorage.readByte();  // read type double
    //             double dist = myInputStorage.readDouble();
    //             addSubscriptionFilterLateralDistance(dist);
    //         }
    //         break;
    //         default:
    //             writeStatusCmd(libsumo::CMD_ADD_SUBSCRIPTION_FILTER, libsumo::RTYPE_NOTIMPLEMENTED,
    //                            "'" + toString(filterType) + "' is no valid filter type code.");
    //             success  = false;
    //     }
    //
    //     if (success) {
    //         // acknowledge filter addition
    //         writeStatusCmd(libsumo::CMD_ADD_SUBSCRIPTION_FILTER, libsumo::RTYPE_OK, "");
    //     }
    //
    //     return success;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter(&mut self) -> bool {
        todo!("port TraCIServer::addSubscriptionFilter")
    }

    /// C++: `void TraCIServer::removeFilters()` [private]
    // void
    // TraCIServer::removeFilters() {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Removing filters" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = libsumo::SUBS_FILTER_NONE;
    // }
    #[allow(unused_variables)]
    pub fn remove_filters(&mut self) {
        todo!("port TraCIServer::removeFilters")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterLanes(int)` [private]
    // void
    // TraCIServer::addSubscriptionFilterLanes(std::vector<int> lanes) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding lane filter (lanes=" << toString(lanes) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_LANES;
    //     myLastContextSubscription->filterLanes = lanes;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lanes(&mut self, lanes: i32) {
        todo!("port TraCIServer::addSubscriptionFilterLanes")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterNoOpposite()` [private]
    // void
    // TraCIServer::addSubscriptionFilterNoOpposite() {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding no opposite filter" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_NOOPPOSITE;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_no_opposite(&mut self) {
        todo!("port TraCIServer::addSubscriptionFilterNoOpposite")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterDownstreamDistance(double)` [private]
    // void
    // TraCIServer::addSubscriptionFilterDownstreamDistance(double dist) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding downstream dist filter (dist=" << toString(dist) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_DOWNSTREAM_DIST;
    //     myLastContextSubscription->filterDownstreamDist = dist;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_downstream_distance(&mut self, dist: f64) {
        todo!("port TraCIServer::addSubscriptionFilterDownstreamDistance")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterUpstreamDistance(double)` [private]
    // void
    // TraCIServer::addSubscriptionFilterUpstreamDistance(double dist) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding upstream dist filter (dist=" << toString(dist) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_UPSTREAM_DIST;
    //     myLastContextSubscription->filterUpstreamDist = dist;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_upstream_distance(&mut self, dist: f64) {
        todo!("port TraCIServer::addSubscriptionFilterUpstreamDistance")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterLeadFollow()` [private]
    // void
    // TraCIServer::addSubscriptionFilterLeadFollow() {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding Lead/Follow-maneuver filter" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_LEAD_FOLLOW;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lead_follow(&mut self) {
        todo!("port TraCIServer::addSubscriptionFilterLeadFollow")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterTurn(double)` [private]
    // void
    // TraCIServer::addSubscriptionFilterTurn(double dist) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding turn-maneuver filter" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_TURN;
    //     myLastContextSubscription->filterFoeDistToJunction = dist;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_turn(&mut self, dist: f64) {
        todo!("port TraCIServer::addSubscriptionFilterTurn")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterVClass(SVCPermissions)` [private]
    // void
    // TraCIServer::addSubscriptionFilterVClass(SVCPermissions vClasses) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding vClass filter (vClasses=" << toString(vClasses) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_VCLASS;
    //     myLastContextSubscription->filterVClasses = vClasses;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_v_class(&mut self, vClasses: SVCPermissions) {
        todo!("port TraCIServer::addSubscriptionFilterVClass")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterVType(int)` [private]
    // void
    // TraCIServer::addSubscriptionFilterVType(std::set<std::string> vTypes) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding vType filter (vTypes=" << toString(vTypes) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_VTYPE;
    //     myLastContextSubscription->filterVTypes = vTypes;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_v_type(&mut self, vTypes: i32) {
        todo!("port TraCIServer::addSubscriptionFilterVType")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterFieldOfVision(double)` [private]
    /// Filter only vehicles within field of vision
    // void
    // TraCIServer::addSubscriptionFilterFieldOfVision(double openingAngle) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding FieldOfVision filter (openingAngle=" << toString(openingAngle) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_FIELD_OF_VISION;
    //     myLastContextSubscription->filterFieldOfVisionOpeningAngle = openingAngle;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_field_of_vision(&mut self, openingAngle: f64) {
        todo!("port TraCIServer::addSubscriptionFilterFieldOfVision")
    }

    /// C++: `void TraCIServer::addSubscriptionFilterLateralDistance(double)` [private]
    /// Filter only vehicles within the given lateral distance
    // void
    // TraCIServer::addSubscriptionFilterLateralDistance(double dist) {
    // #ifdef DEBUG_SUBSCRIPTION_FILTERS
    //     std::cout << "Adding lateral dist filter (dist=" << toString(dist) << ")" << std::endl;
    // #endif
    //     myLastContextSubscription->activeFilters = myLastContextSubscription->activeFilters | libsumo::SUBS_FILTER_LATERAL_DIST;
    //     myLastContextSubscription->filterLateralDist = dist;
    // }
    #[allow(unused_variables)]
    pub fn add_subscription_filter_lateral_distance(&mut self, dist: f64) {
        todo!("port TraCIServer::addSubscriptionFilterLateralDistance")
    }

    /// C++: `bool TraCIServer::centralObject(const libsumo::Subscription &, const std::string &)` [private]
    /// check whether a found objID refers to the central object of a context subscription
    // bool
    // TraCIServer::centralObject(const libsumo::Subscription& s, const std::string& objID) {
    //     return (s.id == objID && s.commandId + 32 == s.contextDomain);
    // }
    #[allow(unused_variables)]
    pub fn central_object(&mut self, s: &Subscription, objID: &str) -> bool {
        todo!("port TraCIServer::centralObject")
    }

    /// C++: `TraCIServer & TraCIServer::operator=(const TraCIServer &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServer) -> &mut TraCIServer {
        panic!("TraCIServer assignment operator is private and undefined in C++; it cannot be called")
    }

}

