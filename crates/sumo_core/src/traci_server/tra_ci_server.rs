//! Port of traci-server/TraCIServer.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use utils::common::msg_handler::{write_error, write_message};
use utils::common::sumo_vehicle_class::parse_vehicle_classes_vec;
use utils::common::time::{SUMOTIME_MAX, delta_t, steps2time, string2time, time2steps};

/// C++ typedef `bool (*)(TraCIServer &, tcpip::Storage &, tcpip::Storage &) TraCIServer::CmdExecutor`
pub type TraCIServer_CmdExecutor = fn(&mut TraCIServer, &mut Storage, &mut Storage) -> bool;

// C++ static members of TraCIServer: `myInstance` and `myDoCloseConnection`
static MY_INSTANCE: AtomicPtr<TraCIServer> = AtomicPtr::new(std::ptr::null_mut());
static MY_DO_CLOSE_CONNECTION: AtomicBool = AtomicBool::new(false);

fn my_instance() -> Ptr<TraCIServer> {
    Ptr::from_raw(MY_INSTANCE.load(Ordering::Relaxed))
}

fn set_my_instance(p: Ptr<TraCIServer>) {
    MY_INSTANCE.store(p.as_raw(), Ordering::Relaxed);
}

/// vehicle states that get a (possibly empty) change list, in the C++ initialisation order
const VEHICLE_STATES: [MSNet_VehicleState; 13] = [
    MSNet_VehicleState::BUILT,
    MSNet_VehicleState::DEPARTED,
    MSNet_VehicleState::STARTING_TELEPORT,
    MSNet_VehicleState::ENDING_TELEPORT,
    MSNet_VehicleState::ARRIVED,
    MSNet_VehicleState::NEWROUTE,
    MSNet_VehicleState::STARTING_PARKING,
    MSNet_VehicleState::MANEUVERING,
    MSNet_VehicleState::ENDING_PARKING,
    MSNet_VehicleState::STARTING_STOP,
    MSNet_VehicleState::ENDING_STOP,
    MSNet_VehicleState::COLLISION,
    MSNet_VehicleState::EMERGENCYSTOP,
];

/// transportable states that get a (possibly empty) change list, in the C++ initialisation order
const TRANSPORTABLE_STATES: [MSNet_TransportableState; 4] = [
    MSNet_TransportableState::PERSON_DEPARTED,
    MSNet_TransportableState::PERSON_ARRIVED,
    MSNet_TransportableState::CONTAINER_DEPARTED,
    MSNet_TransportableState::CONTAINER_ARRIVED,
];

// shortcut: C++ VERSION_STRING is generated (version.h); same build stamp as crates/sumo/src/main.rs, upgrade when version info is ported
const TRACI_SERVER_VERSION_STRING: &str = "v1.28.0-0000000000";

impl crate::libsumo::simulation::SimVariableWrapper for TraCIServer {
    fn wrap_double(&mut self, objID: &str, variable: i32, value: f64) -> bool {
        TraCIServer::wrap_double(self, objID, variable, value)
    }
    fn wrap_int(&mut self, objID: &str, variable: i32, value: i32) -> bool {
        TraCIServer::wrap_int(self, objID, variable, value)
    }
    fn wrap_string(&mut self, objID: &str, variable: i32, value: &str) -> bool {
        TraCIServer::wrap_string(self, objID, variable, value)
    }
    fn wrap_string_list(&mut self, objID: &str, variable: i32, value: &Vec<String>) -> bool {
        TraCIServer::wrap_string_list(self, objID, variable, value)
    }
    fn wrap_string_pair(&mut self, objID: &str, variable: i32, value: &(String, String)) -> bool {
        TraCIServer::wrap_string_pair(self, objID, variable, value)
    }
}

/// C++ class `TraCIServer::SocketInfo` (traci-server/TraCIServer.h)
pub struct TraCIServer_SocketInfo {
    /// next point of action for the client
    pub targetTime: SUMOTime,
    /// whether a "half step" has been done, executing only the move
    pub executeMove: bool,
    /// Socket object for this client
    pub socket: Ptr<Socket>,
    /// container for vehicle state changes since last step taken by this client
    pub vehicleStateChanges: BTreeMap<MSNet_VehicleState, Vec<String>>,
    /// container for transportable state changes since last step taken by this client
    pub transportableStateChanges: BTreeMap<MSNet_TransportableState, Vec<String>>,
}

impl TraCIServer_SocketInfo {
    /// C++: `void SocketInfo::SocketInfo(tcpip::Socket *, SUMOTime)`
    /// constructor
    // SocketInfo(tcpip::Socket* socket, SUMOTime t)
    //             : targetTime(t), socket(socket) {}
    #[allow(unused_variables)]
    pub fn new(socket: Ptr<Socket>, t: SUMOTime) -> Self {
        TraCIServer_SocketInfo {
            targetTime: t,
            executeMove: false,
            socket,
            vehicleStateChanges: BTreeMap::new(),
            transportableStateChanges: BTreeMap::new(),
        }
    }

    /// C++: `void SocketInfo::~SocketInfo() noexcept`
    /// destructor
    // ~SocketInfo() {
    //             delete socket;
    //         }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.socket.is_null() {
            drop(unsafe { Box::from_raw(self.socket.as_raw()) });
        }
    }

    /// C++: `void SocketInfo::SocketInfo(const SocketInfo &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &TraCIServer_SocketInfo) -> Self {
        // C++ declares this copy constructor private and never defines it, so it cannot be used
        panic!("TraCIServer::SocketInfo copy constructor is private and undefined in C++")
    }

}

/// TraCI server used to control sumo by a remote TraCI client
/// C++ class `TraCIServer` (traci-server/TraCIServer.h)
#[repr(C)]
pub struct TraCIServer {
    /// C++ base class `MSNet::VehicleStateListener` (first field, so the struct can be cast to it)
    pub base: MSNet_VehicleStateListener,
    /// C++ base class `libsumo::VariableWrapper`
    pub base2: VariableWrapper,
    /// C++ base class `MSNet::TransportableStateListener`
    pub base3: MSNet_TransportableStateListener,
    /// The socket connections to the clients the first component (index) determines the client's order (lowest index's commands are processed first),
    pub mySockets: BTreeMap<i32, Ptr<TraCIServer_SocketInfo>>,
    /// This stores the setOrder(int) requests of the clients.
    pub mySocketReorderRequests: BTreeMap<i32, Ptr<TraCIServer_SocketInfo>>,
    /// The currently active client, as its key in mySockets (None = end())
    pub myCurrentSocket: Option<i32>,
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
    pub myExecutors: BTreeMap<i32, TraCIServer_CmdExecutor>,
    /// Set of variables which have parameters
    pub myParameterized: BTreeSet<(i32, i32)>,
    /// Arguments of a load command
    pub myLoadArgs: Vec<String>,
    /// The list of known, still valid subscriptions
    pub mySubscriptions: Vec<Subscription>,
    /// The last modified context subscription (the one to add a filter to, see (), currently only for vehicle to vehicle context)
    pub myLastContextSubscription: Ptr<Subscription>,
    /// Changes in the states of simulated vehicles
    pub myVehicleStateChanges: BTreeMap<MSNet_VehicleState, Vec<String>>,
    /// Changes in the states of simulated transportables
    pub myTransportableStateChanges: BTreeMap<MSNet_TransportableState, Vec<String>>,
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

fn panic_text(p: Box<dyn std::any::Any + Send>) -> String {
    p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()
}

impl TraCIServer {
    /// C++ `myCurrentSocket->second`; only valid while the iterator is not end()
    fn cur_socket(&self) -> &Ptr<TraCIServer_SocketInfo> {
        &self.mySockets[&self.myCurrentSocket.unwrap()]
    }

    /// C++ `mySockets.begin()` as a key (None when there are no clients)
    fn first_socket(&self) -> Option<i32> {
        self.mySockets.keys().next().copied()
    }

    /// C++ `++it` on an iterator into mySockets, as a key (None = end())
    fn socket_after(&self, key: i32) -> Option<i32> {
        self.mySockets.range((std::ops::Bound::Excluded(key), std::ops::Bound::Unbounded)).next().map(|(k, _)| *k)
    }

    /// C++: `SUMOTime TraCIServer::getTargetTime() const` [const]
    // SUMOTime getTargetTime() const {
    //         return myTargetTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_time(&self) -> SUMOTime {
        self.myTargetTime
    }

    /// C++: `TraCIServer * TraCIServer::getInstance()` [static]
    // static TraCIServer* getInstance() {
    //         return myInstance;
    //     }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<TraCIServer> {
        my_instance()
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
    pub fn open_socket(execs: &BTreeMap<i32, TraCIServer_CmdExecutor>) {
        if my_instance().is_null() && !MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) && utils::options::options_cont::get_int("remote-port") != 0 {
            let server = TraCIServer::new(
                string2time(&utils::options::options_cont::get_string("begin")).unwrap(),
                utils::options::options_cont::get_int("remote-port"),
                utils::options::options_cont::get_int("num-clients"),
            );
            let inst = Ptr::from_raw(Box::into_raw(Box::new(server)));
            set_my_instance(inst);
            for (k, v) in execs {
                inst.m().myExecutors.insert(k.clone(), v.clone());
            }
        }
        let inst = my_instance();
        if !inst.is_null() {
            // maybe net was deleted and built again
            let net = MSNet::get_instance();
            net.m().add_vehicle_state_listener(inst.cast::<MSNet_VehicleStateListener>());
            net.m().add_transportable_state_listener(inst.cast::<MSNet_TransportableStateListener>());
            inst.m().mySubscriptionCache.write_int(0);
        }
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
        let inst = my_instance();
        if inst.is_null() {
            return;
        }
        // C++ `delete myInstance`: run the destructor, then free the object
        inst.m().drop_();
        drop(unsafe { Box::from_raw(inst.as_raw()) });
        set_my_instance(Ptr::null());
        MY_DO_CLOSE_CONNECTION.store(true, Ordering::Relaxed);
    }

    /// C++: `bool TraCIServer::wasClosed()` [static]
    /// check whether close was requested
    // bool
    // TraCIServer::wasClosed() {
    //     return myDoCloseConnection;
    // }
    #[allow(unused_variables)]
    pub fn was_closed() -> bool {
        MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed)
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
        // shortcut: C++ catches std::invalid_argument, libsumo::TraCIException and tcpip::SocketException and rethrows ProcessError; this returns i32, so those errors panic, upgrade when the TraCI server returns Result
        let mut finalCmd = 0;
        let firstStep = self.myCurrentSocket.is_some();
        // update client order if requested
        self.process_reordering_requests();
        if !firstStep && !afterMove {
            // This is the entry point after performing a SUMO step (block is skipped before first SUMO step since then no simulation results have to be sent)
            // update subscription results
            self.post_process_simulation_step();
            // Send out subscription results to clients which will act in this SUMO step (i.e. with client target time <= current sumo timestep end)
            self.send_output_to_all();
            self.myOutputStorage.reset();
        }

        // determine minimal next target time among clients
        self.myTargetTime = self.next_target_time();

        if step < self.myTargetTime {
            return finalCmd;
        }

        // Simulation should run until
        // 1. end time reached or
        // 2. got libsumo::CMD_CLOSE or
        // 3. got libsumo::CMD_LOAD or
        // 4. Client closes socket connection
        while !MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) && self.myTargetTime <= MSNet::get_instance().get_current_time_step() && finalCmd != CMD_EXECUTEMOVE {
            // Iterate over clients and process communication for the ones with target time == myTargetTime
            self.myCurrentSocket = self.first_socket();
            while let Some(cur) = self.myCurrentSocket {
                if self.mySockets[&cur].m().targetTime > self.myTargetTime || (afterMove && !self.mySockets[&cur].m().executeMove) {
                    // this client must wait
                    self.myCurrentSocket = self.socket_after(cur);
                    continue;
                }
                finalCmd = 0;
                while finalCmd == 0 {
                    if !self.myInputStorage.valid_pos() {
                        // have read request completely, send response if adequate
                        if self.myOutputStorage.size() > 0 {
                            // send response to previous query
                            self.cur_socket().socket.m().send_exact(&self.myOutputStorage).unwrap_or_else(|e| panic!("{}", e.0));
                            self.myOutputStorage.reset();
                        }
                        // Read next request
                        self.myInputStorage.reset();
                        let sock = self.cur_socket().socket;
                        sock.m().receive_exact(&mut self.myInputStorage).unwrap_or_else(|e| panic!("{}", e.0));
                    }

                    while self.myInputStorage.valid_pos() && !MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) {
                        let cmd = self.dispatch_command();
                        if cmd == CMD_SIMSTEP || cmd == CMD_LOAD || cmd == CMD_EXECUTEMOVE || cmd == CMD_CLOSE {
                            finalCmd = cmd;
                        }
                    }
                }
            }
            if !self.myLoadArgs.is_empty() {
                // Breaking loop to load new simulation.
                break;
            } else if MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) {
                // Breaking loop because last client closed connection.
                break;
            }
            let nextT = self.next_target_time();
            // minimal target time among clients should have been increased during the last loop through mySockets
            self.myTargetTime = nextT;
        }
        // All clients are done with the current time step
        // Reset myVehicleStateChanges and myTransportableStateChanges
        for item in self.myVehicleStateChanges.values_mut() {
            item.clear();
        }
        for item in self.myTransportableStateChanges.values_mut() {
            item.clear();
        }
        finalCmd
    }

    /// The switch of C++ processGet. Ok(false) is the C++ `return false` of the default case,
    /// Err carries the message of a TraCIException that the C++ code throws inside the switch.
    fn process_get_variable(&mut self, commandID: i32, variable: i32, id: &str, inputStorage: &mut Storage) -> Result<bool, String> {
        // The VariableWrapper dispatch needs a sink that reaches the server's own `wrap*` bodies.
        // `self` is the server; the borrow of `base2` below must not alias it, so the raw pointer is
        // taken first (fetched from ls02's verified one-line change).
        let me: *mut dyn crate::libsumo::variable_wrapper::VariableWrapperSink = self;
        self.base2.set_sink(me);
        let wrapper = Ptr::from_mut(&mut self.base2);
        let params = Ptr::from_mut(inputStorage);
        let hex = to_hex(variable as u64, 2, 0);
        match commandID {
            CMD_GET_INDUCTIONLOOP_VARIABLE => {
                if !InductionLoop::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Induction Loop Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_MULTIENTRYEXIT_VARIABLE => {
                if !MultiEntryExit::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Multi Entry Exit Detector Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_TL_VARIABLE => {
                if !TrafficLight::handle_variable(id, variable, wrapper, params) {
                    match variable {
                        TL_CONSTRAINT_SWAP => {
                            StorageHelper::read_compound(params.m(), 3, "A compound object of size 3 is needed for swapping constraints.");
                            let tripId = StorageHelper::read_typed_string(params.m(), "The tripId must be given as a string.");
                            let foeSignal = StorageHelper::read_typed_string(params.m(), "The foeSignal id must be given as a string.");
                            let foeId = StorageHelper::read_typed_string(params.m(), "The foe tripId must be given as a string.");
                            self.wrap_signal_constraint_vector(id, variable, &TrafficLight::swap_constraints(id, &tripId, &foeSignal, &foeId));
                        }
                        _ => {
                            return Err(format!("Get TLS Variable: unsupported variable {} specified", hex));
                        }
                    }
                }
            }
            CMD_GET_LANE_VARIABLE => {
                if !Lane::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Lane Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_VEHICLE_VARIABLE => {
                if !Vehicle::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Vehicle Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_VEHICLETYPE_VARIABLE => {
                if !VehicleType::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Vehicle Type Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_ROUTE_VARIABLE => {
                if !Route::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Route Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_POI_VARIABLE => {
                if !POI::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get PoI Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_POLYGON_VARIABLE => {
                if !Polygon::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Polygon Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_JUNCTION_VARIABLE => {
                if !Junction::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Junction Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_EDGE_VARIABLE => {
                if !Edge::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Edge Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_SIM_VARIABLE => {
                if !TraCIServerAPI_Simulation::process_get(self, params.m(), id, variable) {
                    return Err(format!("Get Simulation Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_LANEAREA_VARIABLE => {
                if !LaneArea::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Lane Area Detector Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_PERSON_VARIABLE => {
                if !Person::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Person Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_BUSSTOP_VARIABLE => {
                if !BusStop::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get BusStop Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_PARKINGAREA_VARIABLE => {
                if !ParkingArea::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get ParkingArea Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_CHARGINGSTATION_VARIABLE => {
                if !ChargingStation::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get ChargingStation Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_ROUTEPROBE_VARIABLE => {
                if !RouteProbe::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get RouteProbe Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_CALIBRATOR_VARIABLE => {
                if !Calibrator::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Calibrator Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_REROUTER_VARIABLE => {
                if !Rerouter::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get Rerouter Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_VARIABLESPEEDSIGN_VARIABLE => {
                if !VariableSpeedSign::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get VariableSpeedSign Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_MEANDATA_VARIABLE => {
                if !MeanData::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get MeanData Variable: unsupported variable {} specified", hex));
                }
            }
            CMD_GET_OVERHEADWIRE_VARIABLE => {
                if !OverheadWire::handle_variable(id, variable, wrapper, params) {
                    return Err(format!("Get OverheadWire Variable: unsupported variable {} specified", hex));
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
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
        let variable = inputStorage.read_unsigned_byte().expect("readUnsignedByte");
        let id = inputStorage.read_string().expect("readString");
        self.init_wrapper(commandID + 0x10, variable, &id);
        // C++ catches libsumo::TraCIException here and answers with writeErrorStatusCmd
        match self.process_get_variable(commandID, variable, &id, inputStorage) {
            Ok(true) => {}
            Ok(false) => return false,
            Err(e) => return self.write_error_status_cmd(commandID, &e, outputStorage),
        }
        self.write_status_cmd(commandID, RTYPE_OK, "", outputStorage);
        let wrapper_storage = Ptr::from_mut(&mut self.myWrapperStorage);
        self.write_response_with_length(outputStorage, wrapper_storage.m());
        true
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
        self.mySubscriptions.clear();
        self.myTargetTime = string2time(&utils::options::options_cont::get_string("begin")).unwrap();
        self.myCurrentSocket = self.first_socket();
        while let Some(k) = self.myCurrentSocket {
            let s = self.mySockets[&k];
            s.m().targetTime = self.myTargetTime;
            s.m().executeMove = false;
            self.myCurrentSocket = self.socket_after(k);
        }
        self.myOutputStorage.reset();
        self.myInputStorage.reset();
        self.mySubscriptionCache.reset();
        for (_, v) in self.myVehicleStateChanges.iter_mut() {
            v.clear();
        }
        for (_, v) in self.myTransportableStateChanges.iter_mut() {
            v.clear();
        }
        self.myCurrentSocket = self.first_socket();
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
        if !MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) {
            let id = vehicle.get_id().to_string();
            self.myVehicleStateChanges.entry(to).or_default().push(id.clone());
            for (_, s) in self.mySockets.iter() {
                s.m().vehicleStateChanges.entry(to).or_default().push(id.clone());
            }
        }
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
        if !MY_DO_CLOSE_CONNECTION.load(Ordering::Relaxed) {
            let id = transportable.get_id().to_string();
            self.myTransportableStateChanges.entry(to).or_default().push(id.clone());
            for (_, s) in self.mySockets.iter() {
                s.m().transportableStateChanges.entry(to).or_default().push(id.clone());
            }
        }
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
        if status == RTYPE_ERR {
            write_error(&format!("Answered with error to command {}: {}", to_hex(commandId as u64, 2, 0), description));
        } else if status == RTYPE_NOTIMPLEMENTED {
            write_error(&format!("Requested command not implemented ({}): {}", to_hex(commandId as u64, 2, 0), description));
        }
        outputStorage.write_unsigned_byte(1 + 1 + 1 + 4 + description.len() as i32).unwrap(); // command length
        outputStorage.write_unsigned_byte(commandId).unwrap(); // command type
        outputStorage.write_unsigned_byte(status).unwrap(); // status
        outputStorage.write_string(description); // description
    }

    /// C++: `void TraCIServer::writeStatusCmd(int, int, const std::string &)`
    /// Writes a status command to myOutputStorage
    // void
    // TraCIServer::writeStatusCmd(int commandId, int status, const std::string& description) {
    //     writeStatusCmd(commandId, status, description, myOutputStorage);
    // }
    #[allow(unused_variables)]
    pub fn write_status_cmd_2(&mut self, commandId: i32, status: i32, description: &str) {
        // C++ passes the member myOutputStorage by reference; a raw pointer keeps that aliasing
        let out = Ptr::from_mut(&mut self.myOutputStorage);
        self.write_status_cmd(commandId, status, description, out.m());
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
        self.write_status_cmd(commandId, RTYPE_ERR, description, outputStorage);
        false
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
    pub fn get_vehicle_state_changes(&self) -> &BTreeMap<MSNet_VehicleState, Vec<String>> {
        if self.myCurrentSocket.is_none() {
            // Requested in context of a subscription update
            &self.myVehicleStateChanges
        } else {
            // Requested in the context of a custom query by active client
            &self.cur_socket().m().vehicleStateChanges
        }
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
    pub fn get_transportable_state_changes(&self) -> &BTreeMap<MSNet_TransportableState, Vec<String>> {
        if self.myCurrentSocket.is_none() {
            // Requested in context of a subscription update
            &self.myTransportableStateChanges
        } else {
            // Requested in the context of a custom query by active client
            &self.cur_socket().m().transportableStateChanges
        }
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
        if tempMsg.size() < 254 {
            outputStorage.write_unsigned_byte(1 + tempMsg.size() as i32).unwrap(); // command length -> short
        } else {
            outputStorage.write_unsigned_byte(0).unwrap(); // command length -> extended
            outputStorage.write_int(1 + 4 + tempMsg.size() as i32);
        }
        outputStorage.write_storage(tempMsg);
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
        self.myTargetTime = targetTime;
        for (_, s) in self.mySockets.iter() {
            let s = s.m();
            s.targetTime = targetTime;
            s.executeMove = false;
            for (_, v) in s.vehicleStateChanges.iter_mut() {
                v.clear();
            }
            for (_, v) in s.transportableStateChanges.iter_mut() {
                v.clear();
            }
        }
        self.mySubscriptions.clear();
        self.mySubscriptionCache.reset();
    }

    /// C++: `int & TraCIServer::getLoadArgs()`
    // std::vector<std::string>& getLoadArgs() {
    //         return myLoadArgs;
    //     }
    #[allow(unused_variables)]
    pub fn get_load_args(&mut self) -> &mut Vec<String> {
        &mut self.myLoadArgs
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
        self.myWrapperStorage.reset();
        self.myWrapperStorage.write_unsigned_byte(domainID).unwrap();
        self.myWrapperStorage.write_unsigned_byte(variable).unwrap();
        self.myWrapperStorage.write_string(objID);
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
    pub fn wrap_connection_vector(&mut self, objID: &str, variable: i32, value: &[TraCIConnection]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 8);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for c in value {
            StorageHelper::write_typed_string(w, &c.approachedLane);
            StorageHelper::write_typed_string(w, &c.approachedInternal);
            StorageHelper::write_typed_unsigned_byte(w, c.hasPrio as i32);
            StorageHelper::write_typed_unsigned_byte(w, c.isOpen as i32);
            StorageHelper::write_typed_unsigned_byte(w, c.hasFoe as i32);
            StorageHelper::write_typed_string(w, &c.state);
            StorageHelper::write_typed_string(w, &c.direction);
            StorageHelper::write_typed_double(w, c.length);
        }
        true
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
        self.myWrapperStorage.write_unsigned_byte(TYPE_DOUBLE).unwrap();
        self.myWrapperStorage.write_double(value);
        true
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
        self.myWrapperStorage.write_unsigned_byte(TYPE_INTEGER).unwrap();
        self.myWrapperStorage.write_int(value);
        true
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
        self.myWrapperStorage.write_unsigned_byte(TYPE_STRING).unwrap();
        self.myWrapperStorage.write_string(value);
        true
    }

    /// C++: `bool TraCIServer::wrapStringList(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapStringList(const std::string& /* objID */, const int /* variable */, const std::vector<std::string>& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_STRINGLIST);
    //     myWrapperStorage.writeStringList(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_string_list(&mut self, objID: &str, variable: i32, value: &[String]) -> bool {
        self.myWrapperStorage.write_unsigned_byte(TYPE_STRINGLIST).unwrap();
        self.myWrapperStorage.write_string_list(value);
        true
    }

    /// C++: `bool TraCIServer::wrapDoubleList(const std::string &, const int, const int &)`
    // bool
    // TraCIServer::wrapDoubleList(const std::string& /* objID */, const int /* variable */, const std::vector<double>& value) {
    //     myWrapperStorage.writeUnsignedByte(libsumo::TYPE_DOUBLELIST);
    //     myWrapperStorage.writeDoubleList(value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_double_list(&mut self, objID: &str, variable: i32, value: &[f64]) -> bool {
        self.myWrapperStorage.write_unsigned_byte(TYPE_DOUBLELIST).unwrap();
        self.myWrapperStorage.write_double_list(value);
        true
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
        let include_z = variable == VAR_POSITION3D;
        self.myWrapperStorage.write_unsigned_byte(if include_z { POSITION_3D } else { POSITION_2D }).unwrap();
        self.myWrapperStorage.write_double(value.x);
        self.myWrapperStorage.write_double(value.y);
        if include_z {
            self.myWrapperStorage.write_double(value.z);
        }
        true
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
        self.myWrapperStorage.write_unsigned_byte(TYPE_POLYGON).unwrap();
        if value.value.len() < 256 {
            self.myWrapperStorage.write_unsigned_byte(value.value.len() as i32).unwrap();
        } else {
            self.myWrapperStorage.write_unsigned_byte(0).unwrap();
            self.myWrapperStorage.write_int(value.value.len() as i32);
        }
        for pos in &value.value {
            self.myWrapperStorage.write_double(pos.x);
            self.myWrapperStorage.write_double(pos.y);
        }
        true
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
        self.myWrapperStorage.write_unsigned_byte(TYPE_COLOR).unwrap();
        self.myWrapperStorage.write_unsigned_byte(value.r).unwrap();
        self.myWrapperStorage.write_unsigned_byte(value.g).unwrap();
        self.myWrapperStorage.write_unsigned_byte(value.b).unwrap();
        self.myWrapperStorage.write_unsigned_byte(value.a).unwrap();
        true
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
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 2);
        StorageHelper::write_typed_string(w, &value.0);
        StorageHelper::write_typed_double(w, value.1);
        true
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
    pub fn wrap_string_double_pair_list(&mut self, objID: &str, variable: i32, value: &[(String, f64)]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, value.len() as i32);
        for p in value {
            w.write_string(&p.0);
            w.write_double(p.1);
        }
        true
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
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 2);
        StorageHelper::write_typed_string(w, &value.0);
        StorageHelper::write_typed_string(w, &value.1);
        true
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
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 2);
        StorageHelper::write_typed_int(w, value.0);
        StorageHelper::write_typed_int(w, value.1);
        true
    }

    /// C++: `bool TraCIServer::wrapStage(const std::string &, const int, const libsumo::TraCIStage &)` [virtual]
    // bool
    // TraCIServer::wrapStage(const std::string& /* objID */, const int /* variable */, const libsumo::TraCIStage& value) {
    //     StoHelp::writeStage(myWrapperStorage, value);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn wrap_stage(&mut self, objID: &str, variable: i32, value: &TraCIStage) -> bool {
        StorageHelper::write_stage(&mut self.myWrapperStorage, value);
        true
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
    pub fn wrap_reservation_vector(&mut self, objID: &str, variable: i32, value: &[TraCIReservation]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, value.len() as i32);
        for r in value {
            StorageHelper::write_compound(w, 10);
            StorageHelper::write_typed_string(w, &r.id);
            StorageHelper::write_typed_string_list(w, &r.persons);
            StorageHelper::write_typed_string(w, &r.group);
            StorageHelper::write_typed_string(w, &r.fromEdge);
            StorageHelper::write_typed_string(w, &r.toEdge);
            StorageHelper::write_typed_double(w, r.departPos);
            StorageHelper::write_typed_double(w, r.arrivalPos);
            StorageHelper::write_typed_double(w, r.depart);
            StorageHelper::write_typed_double(w, r.reservationTime);
            StorageHelper::write_typed_int(w, r.state);
        }
        true
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
    pub fn wrap_logic_vector(&mut self, objID: &str, variable: i32, value: &[TraCILogic]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, value.len() as i32);
        for logic in value {
            StorageHelper::write_compound(w, 5);
            StorageHelper::write_typed_string(w, &logic.programID);
            StorageHelper::write_typed_int(w, logic.type_);
            StorageHelper::write_typed_int(w, logic.currentPhaseIndex);
            StorageHelper::write_compound(w, logic.phases.len() as i32);
            for phase in &logic.phases {
                StorageHelper::write_compound(w, 6);
                StorageHelper::write_typed_double(w, phase.duration);
                StorageHelper::write_typed_string(w, &phase.state);
                StorageHelper::write_typed_double(w, phase.minDur);
                StorageHelper::write_typed_double(w, phase.maxDur);
                StorageHelper::write_compound(w, phase.next.len() as i32);
                for n in &phase.next {
                    StorageHelper::write_typed_int(w, *n);
                }
                StorageHelper::write_typed_string(w, &phase.name);
            }
            StorageHelper::write_compound(w, logic.subParameter.len() as i32);
            for (k, v) in &logic.subParameter {
                StorageHelper::write_typed_string_list(w, &vec![k.clone(), v.clone()]);
            }
        }
        true
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
    pub fn wrap_link_vector_vector(&mut self, objID: &str, variable: i32, value: &[Vec<TraCILink>]) -> bool {
        let w = &mut self.myWrapperStorage;
        let mut cnt = 1;
        for sublinks in value {
            cnt += sublinks.len() as i32 + 1;
        }
        StorageHelper::write_compound(w, cnt);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for sublinks in value {
            StorageHelper::write_typed_int(w, sublinks.len() as i32);
            for link in sublinks {
                StorageHelper::write_typed_string_list(w, &vec![link.fromLane.clone(), link.toLane.clone(), link.viaLane.clone()]);
            }
        }
        true
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
    pub fn wrap_signal_constraint_vector(&mut self, objID: &str, variable: i32, value: &[TraCISignalConstraint]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 5);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for c in value {
            StorageHelper::write_constraint(w, c);
        }
        true
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
    pub fn wrap_junction_foe_vector(&mut self, objID: &str, variable: i32, value: &[TraCIJunctionFoe]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 9);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for c in value {
            StorageHelper::write_typed_string(w, &c.foeId);
            StorageHelper::write_typed_double(w, c.egoDist);
            StorageHelper::write_typed_double(w, c.foeDist);
            StorageHelper::write_typed_double(w, c.egoExitDist);
            StorageHelper::write_typed_double(w, c.foeExitDist);
            StorageHelper::write_typed_string(w, &c.egoLane);
            StorageHelper::write_typed_string(w, &c.foeLane);
            StorageHelper::write_typed_unsigned_byte(w, c.egoResponse as i32);
            StorageHelper::write_typed_unsigned_byte(w, c.foeResponse as i32);
        }
        true
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
    pub fn wrap_next_stop_data_vector(&mut self, objID: &str, variable: i32, value: &[TraCINextStopData]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 4);
        StorageHelper::write_typed_int(w, value.len() as i32);
        let full = variable == VAR_NEXT_STOPS2;
        for s in value {
            let legacy_stop_flags = (s.stopFlags << 1) + if s.arrival >= 0.0 { 1 } else { 0 };
            StorageHelper::write_typed_string(w, &s.lane);
            StorageHelper::write_typed_double(w, s.endPos);
            StorageHelper::write_typed_string(w, &s.stoppingPlaceID);
            StorageHelper::write_typed_int(w, if full { s.stopFlags } else { legacy_stop_flags });
            StorageHelper::write_typed_double(w, s.duration);
            StorageHelper::write_typed_double(w, s.until);
            if full {
                StorageHelper::write_typed_double(w, s.startPos);
                StorageHelper::write_typed_double(w, s.intendedArrival);
                StorageHelper::write_typed_double(w, s.arrival);
                StorageHelper::write_typed_double(w, s.depart);
                StorageHelper::write_typed_string(w, &s.split);
                StorageHelper::write_typed_string(w, &s.join);
                StorageHelper::write_typed_string(w, &s.actType);
                StorageHelper::write_typed_string(w, &s.tripId);
                StorageHelper::write_typed_string(w, &s.line);
                StorageHelper::write_typed_double(w, s.speed);
            }
        }
        true
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
    pub fn wrap_vehicle_data_vector(&mut self, objID: &str, variable: i32, value: &[TraCIVehicleData]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 5);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for vd in value {
            StorageHelper::write_typed_string(w, &vd.id);
            StorageHelper::write_typed_double(w, vd.length);
            StorageHelper::write_typed_double(w, vd.entryTime);
            StorageHelper::write_typed_double(w, vd.leaveTime);
            StorageHelper::write_typed_string(w, &vd.typeID);
        }
        true
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
    pub fn wrap_best_lanes_data_vector(&mut self, objID: &str, variable: i32, value: &[TraCIBestLanesData]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 6);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for bld in value {
            StorageHelper::write_typed_string(w, &bld.laneID);
            StorageHelper::write_typed_double(w, bld.length);
            StorageHelper::write_typed_double(w, bld.occupation);
            StorageHelper::write_typed_byte(w, bld.bestLaneOffset);
            StorageHelper::write_typed_unsigned_byte(w, if bld.allowsContinuation { 1 } else { 0 });
            StorageHelper::write_typed_string_list(w, &bld.continuationLanes);
        }
        true
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
    pub fn wrap_next_tls_data_vector(&mut self, objID: &str, variable: i32, value: &[TraCINextTLSData]) -> bool {
        let w = &mut self.myWrapperStorage;
        StorageHelper::write_compound(w, 1 + value.len() as i32 * 4);
        StorageHelper::write_typed_int(w, value.len() as i32);
        for tlsd in value {
            StorageHelper::write_typed_string(w, &tlsd.id);
            StorageHelper::write_typed_int(w, tlsd.tlIndex);
            StorageHelper::write_typed_double(w, tlsd.dist);
            StorageHelper::write_typed_byte(w, tlsd.state as i32);
        }
        true
    }

    /// C++: `tcpip::Storage & TraCIServer::getWrapperStorage()`
    // tcpip::Storage&
    // TraCIServer::getWrapperStorage() {
    //     return myWrapperStorage;
    // }
    #[allow(unused_variables)]
    pub fn get_wrapper_storage(&mut self) -> &mut Storage {
        &mut self.myWrapperStorage
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
        let mut s = TraCIServer {
            base: MSNet_VehicleStateListener::new(),
            base2: VariableWrapper::new(()),
            base3: MSNet_TransportableStateListener::new(),
            mySockets: BTreeMap::new(),
            mySocketReorderRequests: BTreeMap::new(),
            myCurrentSocket: None,
            myTargetTime: begin,
            myInputStorage: Storage::new(),
            myOutputStorage: Storage::new(),
            myWrapperStorage: Storage::new(),
            mySubscriptionCache: Storage::new(),
            myExecutors: BTreeMap::new(),
            myParameterized: BTreeSet::new(),
            myLoadArgs: Vec::new(),
            mySubscriptions: Vec::new(),
            myLastContextSubscription: Ptr::null(),
            myVehicleStateChanges: BTreeMap::new(),
            myTransportableStateChanges: BTreeMap::new(),
        };
        for st in VEHICLE_STATES {
            s.myVehicleStateChanges.insert(st, Vec::new());
        }
        for st in TRANSPORTABLE_STATES {
            s.myTransportableStateChanges.insert(st, Vec::new());
        }

        s.myExecutors.insert(CMD_SET_INDUCTIONLOOP_VARIABLE, TraCIServerAPI_InductionLoop::process_set);
        s.myExecutors.insert(CMD_SET_LANEAREA_VARIABLE, TraCIServerAPI_LaneArea::process_set);
        s.myExecutors.insert(CMD_SET_MULTIENTRYEXIT_VARIABLE, TraCIServerAPI_MultiEntryExit::process_set);

        s.myExecutors.insert(CMD_SET_TL_VARIABLE, TraCIServerAPI_TrafficLight::process_set);
        s.myExecutors.insert(CMD_SET_LANE_VARIABLE, TraCIServerAPI_Lane::process_set);
        s.myExecutors.insert(CMD_SET_VEHICLE_VARIABLE, TraCIServerAPI_Vehicle::process_set);
        s.myExecutors.insert(CMD_SET_VEHICLETYPE_VARIABLE, TraCIServerAPI_VehicleType::process_set);
        s.myExecutors.insert(CMD_SET_ROUTE_VARIABLE, TraCIServerAPI_Route::process_set);
        s.myExecutors.insert(CMD_SET_POI_VARIABLE, TraCIServerAPI_POI::process_set);
        s.myExecutors.insert(CMD_SET_POLYGON_VARIABLE, TraCIServerAPI_Polygon::process_set);
        s.myExecutors.insert(CMD_SET_JUNCTION_VARIABLE, TraCIServerAPI_Junction::process_set);
        s.myExecutors.insert(CMD_SET_EDGE_VARIABLE, TraCIServerAPI_Edge::process_set);
        s.myExecutors.insert(CMD_SET_SIM_VARIABLE, TraCIServerAPI_Simulation::process_set);
        s.myExecutors.insert(CMD_SET_PERSON_VARIABLE, TraCIServerAPI_Person::process_set);
        s.myExecutors.insert(CMD_SET_CALIBRATOR_VARIABLE, TraCIServerAPI_Calibrator::process_set);
        s.myExecutors.insert(CMD_SET_BUSSTOP_VARIABLE, TraCIServerAPI_BusStop::process_set);
        s.myExecutors.insert(CMD_SET_PARKINGAREA_VARIABLE, TraCIServerAPI_ParkingArea::process_set);
        s.myExecutors.insert(CMD_SET_CHARGINGSTATION_VARIABLE, TraCIServerAPI_ChargingStation::process_set);
        s.myExecutors.insert(CMD_SET_ROUTEPROBE_VARIABLE, TraCIServerAPI_RouteProbe::process_set);
        s.myExecutors.insert(CMD_SET_REROUTER_VARIABLE, TraCIServerAPI_Rerouter::process_set);
        s.myExecutors.insert(CMD_SET_VARIABLESPEEDSIGN_VARIABLE, TraCIServerAPI_VariableSpeedSign::process_set);
        // myExecutors[libsumo::CMD_SET_MEANDATA_VARIABLE] is not registered in C++ either
        s.myExecutors.insert(CMD_SET_OVERHEADWIRE_VARIABLE, TraCIServerAPI_OverheadWire::process_set);

        s.myParameterized.insert((CMD_SUBSCRIBE_MEANDATA_VARIABLE, VAR_MEANDATA_LANE));
        s.myParameterized.insert((CMD_SUBSCRIBE_MEANDATA_VARIABLE, VAR_MEANDATA_VALUES));
        s.myParameterized.insert((CMD_SUBSCRIBE_EDGE_VARIABLE, VAR_EDGE_TRAVELTIME));
        s.myParameterized.insert((CMD_SUBSCRIBE_EDGE_VARIABLE, VAR_EDGE_EFFORT));
        s.myParameterized.insert((CMD_SUBSCRIBE_EDGE_VARIABLE, VAR_ANGLE));
        s.myParameterized.insert((CMD_SUBSCRIBE_LANE_VARIABLE, VAR_ANGLE));
        s.myParameterized.insert((CMD_SUBSCRIBE_LANE_VARIABLE, LANE_CHANGES));
        s.myParameterized.insert((CMD_SUBSCRIBE_LANE_VARIABLE, VAR_FOES));
        s.myParameterized.insert((CMD_SUBSCRIBE_PERSON_VARIABLE, DISTANCE_REQUEST));
        s.myParameterized.insert((CMD_SUBSCRIBE_PERSON_VARIABLE, VAR_EDGES));
        s.myParameterized.insert((CMD_SUBSCRIBE_PERSON_VARIABLE, VAR_STAGE));
        s.myParameterized.insert((CMD_SUBSCRIBE_PERSON_VARIABLE, VAR_TAXI_RESERVATIONS));
        s.myParameterized.insert((CMD_SUBSCRIBE_PERSON_VARIABLE, SPLIT_TAXI_RESERVATIONS));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, TL_BLOCKING_VEHICLES));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, TL_RIVAL_VEHICLES));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, TL_PRIORITY_VEHICLES));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, TL_CONSTRAINT));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, TL_CONSTRAINT_BYFOE));
        s.myParameterized.insert((CMD_SUBSCRIBE_TL_VARIABLE, VAR_PERSON_NUMBER));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, DISTANCE_REQUEST));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_EDGE_TRAVELTIME));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_EDGE_EFFORT));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_FOLLOW_SPEED));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_SECURE_GAP));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_STOP_SPEED));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_FOES));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, CMD_CHANGELANE));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_LEADER));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_FOLLOWER));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_NEIGHBORS));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_STOP_PARAMETER));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_NEXT_STOPS)); // this is just a dummy to trigger an error
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_NEXT_STOPS2));
        s.myParameterized.insert((CMD_SUBSCRIBE_VEHICLE_VARIABLE, VAR_TAXI_FLEET));
        s.myParameterized.insert((0, VAR_PARAMETER));
        s.myParameterized.insert((0, VAR_PARAMETER_WITH_KEY));

        MY_DO_CLOSE_CONNECTION.store(false, Ordering::Relaxed);

        // shortcut: C++ warns when internal lanes are off (MSGlobals::gUsingInternalLanes / gUseMesoSim), MSGlobals flags are not ported yet, so this warning is skipped
        // shortcut: C++ rethrows socket errors as ProcessError, this constructor returns Self so socket errors panic instead, upgrade when the constructor returns Result
        write_message(&format!("***Starting server on port {} ***", port));
        let mut server_socket = Socket::new(port);
        if numClients > 1 {
            write_message(&format!("  waiting for {} clients...", numClients));
        }
        while (s.mySockets.len() as i32) < numClients {
            let index = s.mySockets.len() as i32 + MAX_ORDER + 1;
            let client = server_socket.accept(true).unwrap_or_else(|e| panic!("{}", e.0)).expect("accept(true) returns a client socket");
            let info = Ptr::from_raw(Box::into_raw(Box::new(TraCIServer_SocketInfo::new(Ptr::from_raw(Box::into_raw(Box::new(client))), begin))));
            s.mySockets.insert(index, info);
            for st in VEHICLE_STATES {
                info.m().vehicleStateChanges.insert(st, Vec::new());
            }
            for st in TRANSPORTABLE_STATES {
                info.m().transportableStateChanges.insert(st, Vec::new());
            }
            if numClients > 1 {
                write_message("  client connected");
            }
        }
        // When got here, all clients have connected
        if numClients > 1 {
            s.check_client_ordering();
        }
        // set myCurrentSocket != mySockets.end() to indicate that this is the first step in processCommands()
        s.myCurrentSocket = s.first_socket();
        s
    }

    /// C++: `void TraCIServer::~TraCIServer()` [virtual, private]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for socket in self.mySockets.values() {
            let socket = *socket;
            socket.m().drop_();
            drop(unsafe { Box::from_raw(socket.as_raw()) });
        }
        // there is no point in calling cleanup() here, it does not free any pointers and will only modify members which get deleted anyway
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
        // Prepare response
        let mut answer_tmp = Storage::new();
        answer_tmp.write_int(TRACI_VERSION);
        answer_tmp.write_string(&format!("SUMO {}", TRACI_SERVER_VERSION_STRING));
        // When we get here, the response is stored in answerTmp -> put into myOutputStorage
        self.write_status_cmd_2(CMD_GETVERSION, RTYPE_OK, "");
        // command length
        self.myOutputStorage.write_unsigned_byte(1 + 1 + answer_tmp.size() as i32).unwrap();
        // command type
        self.myOutputStorage.write_unsigned_byte(CMD_GETVERSION).unwrap();
        // and the parameter dependant part
        self.myOutputStorage.write_storage(&answer_tmp);
        true
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
        let t = MSNet::get_instance().get_current_time_step();
        self.write_status_cmd_2(CMD_SIMSTEP, RTYPE_OK, "");
        let mut no_active = 0;
        let mut i = 0;
        while i < self.mySubscriptions.len() {
            let s = &self.mySubscriptions[i];
            let is_arrived_vehicle = (s.commandId == CMD_SUBSCRIBE_VEHICLE_VARIABLE || s.commandId == CMD_SUBSCRIBE_VEHICLE_CONTEXT)
                && self.myVehicleStateChanges.get(&MSNet_VehicleState::ARRIVED).map_or(false, |v| v.contains(&s.id));

            let is_arrived_person = (s.commandId == CMD_SUBSCRIBE_PERSON_VARIABLE || s.commandId == CMD_SUBSCRIBE_PERSON_CONTEXT)
                && MSNet::get_instance().m().get_person_control().get(&s.id).is_null();
            if s.endTime < t || is_arrived_vehicle || is_arrived_person {
                self.mySubscriptions.remove(i);
                continue;
            }
            i += 1;
            if s.beginTime > t {
                continue;
            }
            no_active += 1;
        }
        self.mySubscriptionCache.reset();
        self.mySubscriptionCache.write_int(no_active);
        let mut i = 0;
        while i < self.mySubscriptions.len() {
            if self.mySubscriptions[i].beginTime > t {
                i += 1;
                continue;
            }
            // the subscription is only borrowed through a raw pointer, as the C++ code does, since processSingleSubscription needs &mut self
            let s = Ptr::from_mut(&mut self.mySubscriptions[i]);
            let mut into = Storage::new();
            let mut errors = String::new();
            let ok = self.process_single_subscription(s.m(), &mut into, &mut errors);
            self.mySubscriptionCache.write_storage(&into);
            if ok {
                i += 1;
            } else {
                self.mySubscriptions.remove(i);
            }
        }
        self.myOutputStorage.write_storage(&self.mySubscriptionCache);
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
        *commandStart = self.myInputStorage.position() as i32;
        *commandLength = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if *commandLength == 0 {
            *commandLength = self.myInputStorage.read_int().expect("readInt");
        }
        self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte")
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
        let mut commandStart = 0;
        let mut commandLength = 0;
        let commandId = self.read_command_id(&mut commandStart, &mut commandLength);
        // C++ passes the members by reference; raw pointers keep the same aliasing (the executors also write to myOutputStorage)
        let inp = Ptr::from_mut(&mut self.myInputStorage);
        let outp = Ptr::from_mut(&mut self.myOutputStorage);
        let mut success = false;
        // dispatch commands
        // C++ `catch (libsumo::TraCIException&)` -> writeErrorStatusCmd; libsumo reports those as panics here
        if self.myExecutors.contains_key(&commandId) {
            let exec = self.myExecutors[&commandId];
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| exec(self, inp.m(), outp.m()))) {
                Ok(s) => success = s,
                Err(p) => success = self.write_error_status_cmd(commandId, &panic_text(p), outp.m()),
            }
        } else if (commandId >= CMD_GET_INDUCTIONLOOP_VARIABLE && commandId <= CMD_GET_BUSSTOP_VARIABLE)
            || (commandId >= CMD_GET_PARKINGAREA_VARIABLE && commandId <= CMD_GET_OVERHEADWIRE_VARIABLE)
        {
            if commandId == CMD_GET_GUI_VARIABLE {
                self.write_status_cmd_2(commandId, RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo");
            } else {
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.process_get(commandId, inp.m(), outp.m()))) {
                    Ok(s) => success = s,
                    Err(p) => success = self.write_error_status_cmd(commandId, &panic_text(p), outp.m()),
                }
            }
        } else {
            match commandId {
                CMD_GETVERSION => {
                    success = self.command_get_version();
                }
                CMD_LOAD => {
                    // shortcut: C++ wraps the argument read in try/catch and answers with writeErrorStatusCmd; libsumo has no TraCIException mapping yet, so a bad argument panics here
                    let args = StorageHelper::read_typed_string_list(inp.m(), "A load command needs a list of string arguments.");
                    self.myLoadArgs = args;
                    success = true;
                    self.write_status_cmd_2(CMD_LOAD, RTYPE_OK, "");
                    // XXX: This only cares for the client that issued the load command.
                    // Multiclient-load functionality is still to be implemented. Refs #3146.
                    self.cur_socket().socket.m().send_exact(outp.m()).unwrap_or_else(|e| panic!("{}", e.0));
                    self.myCurrentSocket = None;
                    outp.m().reset();
                }
                CMD_EXECUTEMOVE => {
                    self.cur_socket().m().executeMove = true;
                    self.myCurrentSocket = self.socket_after(self.myCurrentSocket.unwrap());
                    success = true;
                    self.write_status_cmd_2(CMD_EXECUTEMOVE, RTYPE_OK, "");
                }
                CMD_SIMSTEP => {
                    let nextT = inp.m().read_double().expect("readDouble");
                    if nextT == 0. {
                        self.cur_socket().m().targetTime += delta_t();
                    } else {
                        self.cur_socket().m().targetTime = time2steps(nextT);
                    }
                    self.cur_socket().m().executeMove = false;
                    if self.cur_socket().m().targetTime <= MSNet::get_instance().get_current_time_step() {
                        // This is not the last TraCI simstep in the current SUMO simstep -> send single simstep response.
                        // @note: In the other case the simstep results are sent to all after the SUMO step was performed, see entry point for processCommands()
                        self.send_single_sim_step_response();
                    }
                    // Clear vehicleStateChanges and transportableStateChanges for this client
                    // -> For subsequent TraCI stepping
                    // that is performed within this SUMO step, no updates on vehicle states
                    // belonging to the last SUMO simulation step will be received by this client.
                    for item in self.cur_socket().m().vehicleStateChanges.values_mut() {
                        item.clear();
                    }
                    for item in self.cur_socket().m().transportableStateChanges.values_mut() {
                        item.clear();
                    }
                    self.myCurrentSocket = self.socket_after(self.myCurrentSocket.unwrap());
                    return commandId;
                }
                CMD_CLOSE => {
                    self.write_status_cmd_2(CMD_CLOSE, RTYPE_OK, "");
                    self.cur_socket().socket.m().send_exact(outp.m()).unwrap_or_else(|e| panic!("{}", e.0));
                    outp.m().reset();
                    if self.mySockets.len() == 1 {
                        // Last client has closed connection
                        MY_DO_CLOSE_CONNECTION.store(true, Ordering::Relaxed);
                    }
                    // remove current socket and increment to next socket in ordering
                    self.myCurrentSocket = self.remove_current_socket();
                    success = true;
                }
                CMD_SETORDER => {
                    let order = inp.m().read_int().expect("readInt");
                    if order > MAX_ORDER {
                        self.write_error_status_cmd(CMD_SETORDER, &format!("A set order command needs an int argument below {}.", MAX_ORDER), outp.m());
                        return 0;
                    }
                    if self.mySockets.contains_key(&order) || self.mySocketReorderRequests.contains_key(&order) {
                        self.write_error_status_cmd(CMD_SETORDER, &format!("Order '{}' is already taken.", order), outp.m());
                        return 0;
                    }
                    // memorize reorder request (will only take effect in the next step)
                    let cur = *self.cur_socket();
                    self.mySocketReorderRequests.insert(order, cur);
                    success = true;
                    self.write_status_cmd_2(CMD_SETORDER, RTYPE_OK, "");
                }
                CMD_SUBSCRIBE_BUSSTOP_VARIABLE
                | CMD_SUBSCRIBE_CALIBRATOR_VARIABLE
                | CMD_SUBSCRIBE_CHARGINGSTATION_VARIABLE
                | CMD_SUBSCRIBE_EDGE_VARIABLE
                | CMD_SUBSCRIBE_GUI_VARIABLE
                | CMD_SUBSCRIBE_INDUCTIONLOOP_VARIABLE
                | CMD_SUBSCRIBE_JUNCTION_VARIABLE
                | CMD_SUBSCRIBE_LANE_VARIABLE
                | CMD_SUBSCRIBE_LANEAREA_VARIABLE
                | CMD_SUBSCRIBE_MEANDATA_VARIABLE
                | CMD_SUBSCRIBE_MULTIENTRYEXIT_VARIABLE
                | CMD_SUBSCRIBE_OVERHEADWIRE_VARIABLE
                | CMD_SUBSCRIBE_PARKINGAREA_VARIABLE
                | CMD_SUBSCRIBE_PERSON_VARIABLE
                | CMD_SUBSCRIBE_POI_VARIABLE
                | CMD_SUBSCRIBE_POLYGON_VARIABLE
                | CMD_SUBSCRIBE_REROUTER_VARIABLE
                | CMD_SUBSCRIBE_ROUTE_VARIABLE
                | CMD_SUBSCRIBE_ROUTEPROBE_VARIABLE
                | CMD_SUBSCRIBE_SIM_VARIABLE
                | CMD_SUBSCRIBE_TL_VARIABLE
                | CMD_SUBSCRIBE_VARIABLESPEEDSIGN_VARIABLE
                | CMD_SUBSCRIBE_VEHICLE_VARIABLE
                | CMD_SUBSCRIBE_VEHICLETYPE_VARIABLE => {
                    success = self.add_object_variable_subscription(commandId, false);
                }
                CMD_SUBSCRIBE_BUSSTOP_CONTEXT
                | CMD_SUBSCRIBE_CALIBRATOR_CONTEXT
                | CMD_SUBSCRIBE_CHARGINGSTATION_CONTEXT
                | CMD_SUBSCRIBE_EDGE_CONTEXT
                | CMD_SUBSCRIBE_GUI_CONTEXT
                | CMD_SUBSCRIBE_INDUCTIONLOOP_CONTEXT
                | CMD_SUBSCRIBE_JUNCTION_CONTEXT
                | CMD_SUBSCRIBE_LANE_CONTEXT
                | CMD_SUBSCRIBE_LANEAREA_CONTEXT
                | CMD_SUBSCRIBE_MEANDATA_CONTEXT
                | CMD_SUBSCRIBE_MULTIENTRYEXIT_CONTEXT
                | CMD_SUBSCRIBE_OVERHEADWIRE_CONTEXT
                | CMD_SUBSCRIBE_PARKINGAREA_CONTEXT
                | CMD_SUBSCRIBE_PERSON_CONTEXT
                | CMD_SUBSCRIBE_POI_CONTEXT
                | CMD_SUBSCRIBE_POLYGON_CONTEXT
                | CMD_SUBSCRIBE_REROUTER_CONTEXT
                | CMD_SUBSCRIBE_ROUTE_CONTEXT
                | CMD_SUBSCRIBE_ROUTEPROBE_CONTEXT
                | CMD_SUBSCRIBE_SIM_CONTEXT
                | CMD_SUBSCRIBE_TL_CONTEXT
                | CMD_SUBSCRIBE_VARIABLESPEEDSIGN_CONTEXT
                | CMD_SUBSCRIBE_VEHICLE_CONTEXT
                | CMD_SUBSCRIBE_VEHICLETYPE_CONTEXT => {
                    success = self.add_object_variable_subscription(commandId, true);
                }
                CMD_ADD_SUBSCRIPTION_FILTER => {
                    success = self.add_subscription_filter();
                }
                CMD_SET_GUI_VARIABLE => {
                    self.write_status_cmd_2(commandId, RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo");
                }
                _ => {
                    self.write_status_cmd_2(commandId, RTYPE_NOTIMPLEMENTED, "Command not implemented in sumo");
                }
            }
        }
        if !success {
            while inp.m().valid_pos() && (inp.m().position() as i32) < commandStart + commandLength {
                inp.m().read_char().expect("readChar");
            }
        }
        if (inp.m().position() as i32) != commandStart + commandLength {
            let msg = format!(
                "Wrong position in requestMessage after dispatching command {}. Expected command length was {} but {} Bytes were read.",
                commandId,
                commandLength,
                inp.m().position() as i32 - commandStart
            );
            self.write_status_cmd_2(commandId, RTYPE_ERR, &msg);
            MY_DO_CLOSE_CONNECTION.store(true, Ordering::Relaxed);
        }
        commandId
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
        // check for SET_ORDER commands queued by connected clients
        // In multiclient cas it is mandatory that SET_ORDER  is sent as the first command (or directly after GET_VERSION)
        self.myCurrentSocket = self.first_socket();
        while let Some(cur) = self.myCurrentSocket {
            loop {
                self.myInputStorage.reset();
                let sock = self.cur_socket().socket;
                sock.m().receive_exact(&mut self.myInputStorage).unwrap_or_else(|e| panic!("{}", e.0));
                let mut command_start = 0;
                let mut command_length = 0;
                let command_id = self.read_command_id(&mut command_start, &mut command_length);
                // Whether the received command is a permitted command for the initialization phase.
                // Currently, getVersion and setOrder are permitted.
                let init_command = command_id == CMD_SETORDER || command_id == CMD_GETVERSION;
                if init_command {
                    // reset input storage to initial state before reading the commandId
                    // (ugly, but we can't just reset the store's iter_ from here)
                    // Giving the commandId to dispatch command didn't work either
                    let mut tmp = Storage::new();
                    tmp.write_storage(&self.myInputStorage);
                    self.myInputStorage.reset();
                    // we don't know whether the command was set with extended
                    // length syntax or not so we hardcode the length here (#5037)
                    self.myInputStorage.write_unsigned_byte(if command_id == CMD_SETORDER { 6 } else { 2 }).unwrap();
                    self.myInputStorage.write_unsigned_byte(command_id).unwrap();
                    self.myInputStorage.write_storage(&tmp);

                    // Handle initialization command completely
                    self.dispatch_command();
                    self.cur_socket().socket.m().send_exact(&self.myOutputStorage).unwrap_or_else(|e| panic!("{}", e.0));
                    self.myOutputStorage.reset();
                } else {
                    // shortcut: C++ throws ProcessError here, this function returns nothing so the error panics, upgrade when the TraCI server returns Result
                    panic!("Execution order (libsumo::CMD_SETORDER) was not set for all TraCI clients in pre-execution phase.");
                }
                if command_id == CMD_SETORDER {
                    // This is what we have waited for.
                    break;
                }
            }
            self.myCurrentSocket = self.socket_after(cur);
        }
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
        // Process reordering requests
        if !self.mySocketReorderRequests.is_empty() {
            // process reordering requests
            let requests: Vec<(i32, Ptr<TraCIServer_SocketInfo>)> = self.mySocketReorderRequests.iter().map(|(k, v)| (*k, *v)).collect();
            for (order, request) in requests {
                let j = self
                    .mySockets
                    .iter()
                    .find(|(_, s)| s.m().socket == request.m().socket)
                    .map(|(k, _)| *k);
                assert!(j.is_some());
                self.mySockets.remove(&j.unwrap());
                self.mySockets.insert(order, request);
            }
            self.mySocketReorderRequests.clear();
        }
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
        let mut target_time = SUMOTime::MAX;
        for s in self.mySockets.values() {
            target_time = target_time.min(s.targetTime);
        }
        target_time
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
        for s in self.mySockets.values() {
            if s.targetTime <= MSNet::get_instance().get_current_time_step() {
                // this client will become active before the next SUMO step. Provide subscription results.
                s.socket.m().send_exact(&self.myOutputStorage).unwrap_or_else(|e| panic!("{}", e.0));
            }
        }
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
        self.write_status_cmd_2(CMD_SIMSTEP, RTYPE_OK, "");

        // NOTE: the commented code would send an empty response
        self.myOutputStorage.write_storage(&self.mySubscriptionCache);
        // send results to active client
        self.cur_socket().socket.m().send_exact(&self.myOutputStorage).unwrap_or_else(|e| panic!("{}", e.0));
        self.myOutputStorage.reset();
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
    pub fn remove_current_socket(&mut self) -> Option<i32> {
        let key = self.myCurrentSocket.unwrap();
        // C++ `delete myCurrentSocket->second`
        let s = self.mySockets[&key];
        s.m().drop_();
        drop(unsafe { Box::from_raw(s.as_raw()) });
        self.mySockets.remove(&key);
        // C++ `mySockets.erase(myCurrentSocket)` returns the successor
        self.myCurrentSocket = self.socket_after(key);
        self.myCurrentSocket
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
        let beginTime = self.myInputStorage.read_double().expect("readDouble");
        let endTime = self.myInputStorage.read_double().expect("readDouble");
        let begin: SUMOTime = if beginTime == INVALID_DOUBLE_VALUE { 0 } else { time2steps(beginTime) };
        let end: SUMOTime = if endTime == INVALID_DOUBLE_VALUE || endTime > steps2time(SUMOTIME_MAX) { SUMOTIME_MAX } else { time2steps(endTime) };
        let id = self.myInputStorage.read_string().expect("readString");
        let domain = if hasContext { self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte") } else { 0 };
        let mut range = if hasContext { self.myInputStorage.read_double().expect("readDouble") } else { 0. };
        if commandId == CMD_SUBSCRIBE_SIM_CONTEXT {
            range = f64::MAX;
        }
        let num = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
        let mut variables: Vec<i32> = Vec::new();
        let mut parameters: Vec<Rc<Storage>> = Vec::new();
        for _ in 0..num {
            let varID = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
            variables.push(varID);
            let mut param = Storage::new();
            if self.myParameterized.contains(&(0, varID)) || self.myParameterized.contains(&(commandId, varID)) {
                if !self.myInputStorage.valid_pos() {
                    self.write_status_cmd_2(commandId, RTYPE_ERR, &format!("Missing parameter for subscription {}", to_hex(commandId as u64, 2, 0)));
                    return false;
                }
                let mut count = 1;
                while count > 0 {
                    count -= 1;
                    let parType = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                    param.write_unsigned_byte(parType).unwrap();
                    if parType == TYPE_DOUBLE {
                        let v = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(v);
                    } else if parType == TYPE_INTEGER {
                        let v = self.myInputStorage.read_int().expect("readInt");
                        param.write_int(v);
                    } else if parType == TYPE_STRING {
                        let v = self.myInputStorage.read_string().expect("readString");
                        param.write_string(&v);
                    } else if parType == TYPE_BYTE {
                        let v = self.myInputStorage.read_byte().expect("readByte");
                        param.write_byte(v).unwrap();
                    } else if parType == TYPE_UBYTE {
                        let v = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                        param.write_unsigned_byte(v).unwrap();
                    } else if parType == POSITION_2D {
                        let x = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(x);
                        let y = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(y);
                        if varID == DISTANCE_REQUEST {
                            let v = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                            param.write_unsigned_byte(v).unwrap();
                            break;
                        }
                    } else if parType == POSITION_3D {
                        let x = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(x);
                        let y = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(y);
                        let z = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(z);
                        if varID == DISTANCE_REQUEST {
                            let v = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                            param.write_unsigned_byte(v).unwrap();
                            break;
                        }
                    } else if parType == POSITION_ROADMAP {
                        let s = self.myInputStorage.read_string().expect("readString");
                        param.write_string(&s);
                        let d = self.myInputStorage.read_double().expect("readDouble");
                        param.write_double(d);
                        let edge_pos = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                        param.write_unsigned_byte(edge_pos).unwrap();
                        if varID == DISTANCE_REQUEST {
                            let v = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
                            param.write_unsigned_byte(v).unwrap();
                            break;
                        }
                    } else if parType == TYPE_COMPOUND {
                        count = self.myInputStorage.read_int().expect("readInt");
                        param.write_int(count);
                    } else {
                        self.write_status_cmd_2(commandId, RTYPE_ERR, &format!("Invalid parameter for subscription {}", to_hex(commandId as u64, 2, 0)));
                        return false;
                    }
                }
            }
            parameters.push(Rc::new(param));
        }
        // check subscribe/unsubscribe
        if variables.is_empty() {
            self.remove_subscription(commandId, &id, domain);
            return true;
        }
        // process subscription
        let mut s = Subscription::new(commandId, &id, &variables, &parameters, begin, end, domain, range);
        self.initialise_subscription(&mut s);
        true
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
        let mut writeInto = Storage::new();
        let mut errors = String::new();
        let mut modifiedSubscription: Ptr<Subscription> = Ptr::null();
        // shortcut: C++ catches libsumo::TraCIException and answers with writeStatusCmd(RTYPE_ERR, e.what()); libsumo has no TraCIException mapping yet, so that catch is not reproduced
        if self.process_single_subscription(s, &mut writeInto, &mut errors) {
            if s.endTime < MSNet::get_instance().get_current_time_step() {
                self.write_status_cmd_2(s.commandId, RTYPE_ERR, "Subscription has ended.");
            } else {
                if Helper::need_new_subscription(s, &mut self.mySubscriptions, &mut modifiedSubscription) {
                    // Add new subscription to subscription cache (note: seems a bit inefficient)
                    if s.beginTime < MSNet::get_instance().get_current_time_step() {
                        // copy new subscription into cache
                        let cached = if self.mySubscriptionCache.size() > 0 { self.mySubscriptionCache.read_int().expect("readInt") } else { 0 };
                        let noActive = 1 + cached;
                        let mut tmp = Storage::new();
                        tmp.write_int(noActive);
                        while self.mySubscriptionCache.valid_pos() {
                            let b = self.mySubscriptionCache.read_byte().expect("readByte");
                            tmp.write_byte(b).unwrap();
                        }
                        tmp.write_storage(&writeInto);
                        self.mySubscriptionCache.reset();
                        self.mySubscriptionCache.write_storage(&tmp);
                    }
                }
                self.write_status_cmd_2(s.commandId, RTYPE_OK, "");
            }
            if !modifiedSubscription.is_null()
                && (modifiedSubscription.m().is_vehicle_to_vehicle_context_subscription() || modifiedSubscription.m().is_vehicle_to_person_context_subscription())
            {
                // Set last modified vehicle context subscription active for filter modifications
                self.myLastContextSubscription = modifiedSubscription;
            } else {
                // adding other subscriptions deactivates the activation for filter addition
                self.myLastContextSubscription = Ptr::null();
            }
        } else {
            self.write_status_cmd_2(s.commandId, RTYPE_ERR, &format!("Could not add subscription. {}", errors));
        }
        self.myOutputStorage.write_storage(&writeInto);
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
        let mut found = false;
        let mut j = 0;
        while j < self.mySubscriptions.len() {
            if self.mySubscriptions[j].id == identity && self.mySubscriptions[j].commandId == commandId && self.mySubscriptions[j].contextDomain == domain {
                self.mySubscriptions.remove(j);
                if j < self.mySubscriptions.len() && self.myLastContextSubscription == Ptr::from_mut(&mut self.mySubscriptions[j]) {
                    // Remove also reference for filter additions
                    self.myLastContextSubscription = Ptr::null();
                }
                found = true;
                continue;
            }
            j += 1;
        }
        // try unsubscribe
        if found {
            self.write_status_cmd_2(commandId, RTYPE_OK, "");
        } else {
            self.write_status_cmd_2(commandId, RTYPE_ERR, "The subscription to remove was not found.");
        }
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
        let mut ok = true;
        let mut outputStorage = Storage::new();
        let getCommandId = if s.contextDomain > 0 { s.contextDomain } else { s.commandId - 0x30 };
        let mut objIDs: BTreeSet<String> = BTreeSet::new();
        if s.contextDomain > 0 {
            if (s.activeFilters & SubscriptionFilterType::SUBS_FILTER_NO_RTREE as i32) == 0 {
                let mut shape = PositionVector(Vec::new());
                Helper::find_object_shape(s.commandId, &s.id, &mut shape);
                Helper::collect_object_i_ds_in_range(s.contextDomain, &shape, s.range, &mut objIDs);
            }
            Helper::apply_subscription_filters(s, &mut objIDs);
        } else {
            objIDs.insert(s.id.clone());
        }
        let numVars: i32 = if s.contextDomain > 0 && s.variables.len() == 1 && s.variables[0] == TRACI_ID_LIST {
            0
        } else {
            s.variables.len() as i32
        };
        let skipped = 0;
        for j in &objIDs {
            if s.contextDomain > 0 {
                outputStorage.write_string(j);
            }
            if numVars > 0 {
                for (i, k) in s.variables.iter().zip(s.parameters.iter()) {
                    let mut message = Storage::new();
                    message.write_unsigned_byte(*i).unwrap();
                    message.write_string(j);
                    for v in k.as_bytes() {
                        message.write_char(*v);
                    }
                    let mut tmpOutput = Storage::new();
                    // shortcut: C++ catches std::invalid_argument from the executor and answers "Unsupported command specified"; the executors here do not throw it, so that catch is not reproduced
                    // C++ `catch (libsumo::TraCIException&)` in handleVariable -> error status in tmpOutput, which the copy below reports
                    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if self.myExecutors.contains_key(&getCommandId) {
                            let exec = self.myExecutors[&getCommandId];
                            exec(self, &mut message, &mut tmpOutput)
                        } else {
                            let r = self.process_get(getCommandId, &mut message, &mut tmpOutput);
                            if !r && getCommandId == CMD_GET_GUI_VARIABLE {
                                self.write_status_cmd(s.commandId, RTYPE_NOTIMPLEMENTED, "GUI is not running, command not implemented in command line sumo", &mut tmpOutput);
                            }
                            r
                        }
                    }));
                    match got {
                        Ok(r) => ok &= r,
                        Err(p) => ok &= self.write_error_status_cmd(getCommandId, &panic_text(p), &mut tmpOutput),
                    }
                    if ok {
                        let mut length = tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        length -= 1;
                        while length > 0 {
                            tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                            length -= 1;
                        }
                        let mut lengthLength: i32 = 1;
                        length = tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        if length == 0 {
                            lengthLength = 5;
                            length = tmpOutput.read_int().expect("readInt");
                        }
                        tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        let variable = tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        let id = tmpOutput.read_string().expect("readString");
                        outputStorage.write_unsigned_byte(variable).unwrap();
                        outputStorage.write_unsigned_byte(RTYPE_OK).unwrap();
                        length -= lengthLength + 1 + 4 + id.len() as i32;
                        length -= 1;
                        while length > 0 {
                            let b = tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                            outputStorage.write_unsigned_byte(b).unwrap();
                            length -= 1;
                        }
                    } else {
                        tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        tmpOutput.read_unsigned_byte().expect("readUnsignedByte");
                        let msg = tmpOutput.read_string().expect("readString");
                        outputStorage.write_unsigned_byte(*i).unwrap();
                        outputStorage.write_unsigned_byte(RTYPE_ERR).unwrap();
                        outputStorage.write_unsigned_byte(TYPE_STRING).unwrap();
                        outputStorage.write_string(&msg);
                        errors.push_str(&msg);
                    }
                }
            }
        }
        let mut length = (1 + 4) + 1 + (4 + s.id.len() as i32) + 1 + outputStorage.size() as i32;
        if s.contextDomain > 0 {
            length += 1 + 4; // context domain and number of objects
        }
        writeInto.write_unsigned_byte(0).unwrap(); // command length -> extended
        writeInto.write_int(length);
        writeInto.write_unsigned_byte(s.commandId + 0x10).unwrap();
        writeInto.write_string(&s.id);
        if s.contextDomain > 0 {
            writeInto.write_unsigned_byte(s.contextDomain).unwrap();
        }
        writeInto.write_unsigned_byte(numVars).unwrap();
        if s.contextDomain > 0 {
            writeInto.write_int(objIDs.len() as i32 - skipped);
        }
        if s.contextDomain == 0 || !objIDs.is_empty() {
            writeInto.write_storage(&outputStorage);
        }
        ok
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
        let mut success = true;
        // Read filter type
        let filterType = self.myInputStorage.read_unsigned_byte().expect("readUnsignedByte");
        if self.myLastContextSubscription.is_null() {
            self.write_status_cmd_2(
                CMD_ADD_SUBSCRIPTION_FILTER,
                RTYPE_ERR,
                &format!("No previous vehicle context subscription exists to apply filter type {}", to_hex(filterType as u64, 2, 0)),
            );
            return false;
        }
        // dispatch according to filter type
        match filterType {
            FILTER_TYPE_NONE => {
                // Remove all filters
                self.remove_filters();
            }
            FILTER_TYPE_LANES => {
                // Read relative lanes to consider for context filter
                let nrLanes = self.myInputStorage.read_byte().expect("readByte");
                let mut lanes: Vec<i32> = Vec::new();
                for _ in 0..nrLanes {
                    lanes.push(self.myInputStorage.read_byte().expect("readByte"));
                }
                self.add_subscription_filter_lanes(lanes);
            }
            FILTER_TYPE_NOOPPOSITE => {
                self.add_subscription_filter_no_opposite();
            }
            FILTER_TYPE_DOWNSTREAM_DIST => {
                self.myInputStorage.read_byte().expect("readByte"); // read type double
                let dist = self.myInputStorage.read_double().expect("readDouble");
                self.add_subscription_filter_downstream_distance(dist);
            }
            FILTER_TYPE_UPSTREAM_DIST => {
                self.myInputStorage.read_byte().expect("readByte"); // read type double
                let dist = self.myInputStorage.read_double().expect("readDouble");
                self.add_subscription_filter_upstream_distance(dist);
            }
            FILTER_TYPE_LEAD_FOLLOW => {
                self.add_subscription_filter_lead_follow();
            }
            FILTER_TYPE_TURN => {
                self.myInputStorage.read_byte().expect("readByte"); // read type double
                let dist = self.myInputStorage.read_double().expect("readDouble");
                self.add_subscription_filter_turn(dist);
            }
            FILTER_TYPE_VCLASS => {
                self.myInputStorage.read_byte().expect("readByte"); // read type stringlist
                let classes = self.myInputStorage.read_string_list().expect("readStringList");
                // shortcut: the C++ overload can throw on unknown class names; the ported parser returns a Result, which is dropped here
                let vClasses = parse_vehicle_classes_vec(&classes).ok().expect("parseVehicleClasses");
                self.add_subscription_filter_v_class(vClasses);
            }
            FILTER_TYPE_VTYPE => {
                self.myInputStorage.read_byte().expect("readByte"); // read type stringlist
                let vTypesVector = self.myInputStorage.read_string_list().expect("readStringList");
                let vTypesSet: BTreeSet<String> = vTypesVector.into_iter().collect();
                self.add_subscription_filter_v_type(vTypesSet);
            }
            FILTER_TYPE_FIELD_OF_VISION => {
                self.myInputStorage.read_byte().expect("readByte"); // read type double
                let angle = self.myInputStorage.read_double().expect("readDouble");
                self.add_subscription_filter_field_of_vision(angle);
            }
            FILTER_TYPE_LATERAL_DIST => {
                self.myInputStorage.read_byte().expect("readByte"); // read type double
                let dist = self.myInputStorage.read_double().expect("readDouble");
                self.add_subscription_filter_lateral_distance(dist);
            }
            _ => {
                self.write_status_cmd_2(
                    CMD_ADD_SUBSCRIPTION_FILTER,
                    RTYPE_NOTIMPLEMENTED,
                    &format!("'{}' is no valid filter type code.", filterType),
                );
                success = false;
            }
        }
        if success {
            self.write_status_cmd_2(CMD_ADD_SUBSCRIPTION_FILTER, RTYPE_OK, "");
        }
        success
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
        self.myLastContextSubscription.m().activeFilters = SubscriptionFilterType::SUBS_FILTER_NONE as i32;
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
    pub fn add_subscription_filter_lanes(&mut self, lanes: Vec<i32>) {
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_LANES as i32;
        s.filterLanes = lanes;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_NOOPPOSITE as i32;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_DOWNSTREAM_DIST as i32;
        s.filterDownstreamDist = dist;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_UPSTREAM_DIST as i32;
        s.filterUpstreamDist = dist;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_LEAD_FOLLOW as i32;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_TURN as i32;
        s.filterFoeDistToJunction = dist;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_VCLASS as i32;
        s.filterVClasses = vClasses;
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
    pub fn add_subscription_filter_v_type(&mut self, vTypes: BTreeSet<String>) {
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_VTYPE as i32;
        s.filterVTypes = vTypes;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_FIELD_OF_VISION as i32;
        s.filterFieldOfVisionOpeningAngle = openingAngle;
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
        let s = self.myLastContextSubscription.m();
        s.activeFilters = s.activeFilters | SubscriptionFilterType::SUBS_FILTER_LATERAL_DIST as i32;
        s.filterLateralDist = dist;
    }

    /// C++: `bool TraCIServer::centralObject(const libsumo::Subscription &, const std::string &)` [private]
    /// check whether a found objID refers to the central object of a context subscription
    // bool
    // TraCIServer::centralObject(const libsumo::Subscription& s, const std::string& objID) {
    //     return (s.id == objID && s.commandId + 32 == s.contextDomain);
    // }
    #[allow(unused_variables)]
    pub fn central_object(&mut self, s: &Subscription, objID: &str) -> bool {
        s.id == objID && s.commandId + 32 == s.contextDomain
    }

    /// C++: `TraCIServer & TraCIServer::operator=(const TraCIServer &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &TraCIServer) -> &mut TraCIServer {
        panic!("TraCIServer assignment operator is private and undefined in C++; it cannot be called")
    }

}

