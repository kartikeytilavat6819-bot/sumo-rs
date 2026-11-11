//! Port of microsim/devices/MSDevice_Taxi.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSDevice_Taxi::TaxiState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSDevice_Taxi_TaxiState {
    EMPTY = 0,
    PICKUP = 1,
    OCCUPIED = 2,
}

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Taxi` (microsim/devices/MSDevice_Taxi.h)
pub struct MSDevice_Taxi {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    pub myState: i32,
    /// number of customers that were served
    pub myCustomersServed: i32,
    /// distance driven with customers
    pub myOccupiedDistance: f64,
    /// time spent driving with customers
    pub myOccupiedTime: SUMOTime,
    /// the time at which the taxi service ends (end the vehicle may leave the simulation)
    pub myServiceEnd: SUMOTime,
    /// whether the vehicle is currently stopped
    pub myIsStopped: bool,
    /// the customer of the current reservation
    pub myCustomers: BTreeSet<Ptr<MSTransportable>>,
    /// algorithm for controlling idle behavior
    pub myIdleAlgorithm: Ptr<MSIdling>,
    /// whether the taxi has reached its schedule service end
    pub myReachedServiceEnd: bool,
    /// reservations currently being served
    pub myCurrentReservations: BTreeSet<Ptr<Reservation>>,
    /// routing device (if the vehicle has one)
    pub myRoutingDevice: Ptr<MSDevice_Routing>,
    /// the last dispatch order
    pub myLastDispatch: Vec<Ptr<Reservation>>,
}

impl std::ops::Deref for MSDevice_Taxi {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Taxi {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

// C++ static members of MSDevice_Taxi (port as module-level statics, see AGENTS.md):
//   static SUMOTime MSDevice_Taxi::myDispatchPeriod;
//   static MSDispatch * MSDevice_Taxi::myDispatcher;
//   static Command * MSDevice_Taxi::myDispatchCommand;
//   static std::vector<MSDevice_Taxi *> MSDevice_Taxi::myFleet;
//   static int MSDevice_Taxi::myMaxCapacity;
//   static int MSDevice_Taxi::myMaxContainerCapacity;
//   static std::map<SUMOVehicleClass, std::string> MSDevice_Taxi::myTaxiTypes;
//   static SUMOTime MSDevice_Taxi::myNextDispatchTime;
//   static std::map<std::string, MSDevice_Taxi *> MSDevice_Taxi::myStateLoadedCustomers;
//   static std::map<std::string, MSDevice_Taxi *> MSDevice_Taxi::myStateLoadedReservations;

impl MSDevice_Taxi {
    /// C++: `void MSDevice_Taxi::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Taxi-options
    // void
    // MSDevice_Taxi::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Taxi Device");
    //     insertDefaultAssignmentOptions("taxi", "Taxi Device", oc);
    //
    //     oc.doRegister("device.taxi.dispatch-algorithm", new Option_String("greedy"));
    //     oc.addDescription("device.taxi.dispatch-algorithm", "Taxi Device", TL("The dispatch algorithm [greedy|greedyClosest|greedyShared|routeExtension|traci]"));
    //
    //     oc.doRegister("device.taxi.dispatch-algorithm.output", new Option_FileName());
    //     oc.addDescription("device.taxi.dispatch-algorithm.output", "Taxi Device", TL("Write information from the dispatch algorithm to FILE"));
    //
    //     oc.doRegister("device.taxi.dispatch-algorithm.params", new Option_String(""));
    //     oc.addDescription("device.taxi.dispatch-algorithm.params", "Taxi Device", TL("Load dispatch algorithm parameters in format KEY1:VALUE1[,KEY2:VALUE]"));
    //
    //     oc.doRegister("device.taxi.dispatch-period", new Option_String("60", "TIME"));
    //     oc.addDescription("device.taxi.dispatch-period", "Taxi Device", TL("The period between successive calls to the dispatcher"));
    //
    //     oc.doRegister("device.taxi.dispatch-keep-unreachable", new Option_String("3600", "TIME"));
    //     oc.addDescription("device.taxi.dispatch-keep-unreachable", "Taxi Device", TL("The time before aborting unreachable reservations"));
    //
    //     oc.doRegister("device.taxi.idle-algorithm", new Option_String("stop"));
    //     oc.addDescription("device.taxi.idle-algorithm", "Taxi Device", TL("The behavior of idle taxis [stop|randomCircling|taxistand]"));
    //
    //     oc.doRegister("device.taxi.idle-algorithm.output", new Option_FileName());
    //     oc.addDescription("device.taxi.idle-algorithm.output", "Taxi Device", TL("Write information from the idling algorithm to FILE"));
    //
    //     oc.doRegister("device.taxi.vclasses", new Option_StringVector({"taxi"}));
    //     oc.addSynonyme("device.taxi.vclasses", "taxi.vclasses");
    //     oc.addDescription("device.taxi.vclasses", "Taxi Device", TL("Network permissions that can be accessed by taxis"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSDevice_Taxi::insertOptions")
    }

    /// C++: `void MSDevice_Taxi::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Taxi::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "taxi", v, false)) {
    //         // build the device
    //         MSDevice_Taxi* device = new MSDevice_Taxi(v, "taxi_" + v.getID());
    //         into.push_back(device);
    //         myFleet.push_back(device);
    //         if (v.getParameter().line == "") {
    //             // automatically set the line so that persons are willing to enter
    //             // (see MSStageDriving::isWaitingFor)
    //             const_cast<SUMOVehicleParameter&>(v.getParameter()).line = TAXI_SERVICE;
    //         }
    //         const int personCapacity = v.getVehicleType().getPersonCapacity();
    //         const int containerCapacity = v.getVehicleType().getContainerCapacity();
    //         myMaxCapacity = MAX2(myMaxCapacity, personCapacity);
    //         myMaxContainerCapacity = MAX2(myMaxContainerCapacity, containerCapacity);
    //         if (myTaxiTypes[v.getVClass()] == "") {
    //             myTaxiTypes[v.getVClass()] = v.getVehicleType().getID();
    //         }
    //         if ((gTaxiClasses & v.getVClass()) == 0) {
    //             gTaxiClasses |= v.getVClass();
    //             MSNet::getInstance()->resetIntermodalRouter();
    //         }
    //         if (personCapacity < 1 && containerCapacity < 1) {
    //             WRITE_WARNINGF(TL("Vehicle '%' with personCapacity % and containerCapacity % is not usable as taxi."), v.getID(), toString(personCapacity), toString(containerCapacity));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Taxi::buildVehicleDevices")
    }

    /// C++: `bool MSDevice_Taxi::isReservation(const std::set<std::string> &)` [static]
    /// whether the given lines description is a taxi call
    // bool
    // MSDevice_Taxi::isReservation(const std::set<std::string>& lines) {
    //     return lines.size() == 1 && (
    //                *lines.begin() == TAXI_SERVICE
    //                || StringUtils::startsWith(*lines.begin(), TAXI_SERVICE_PREFIX));
    // }
    #[allow(unused_variables)]
    pub fn is_reservation(lines: &BTreeSet<String>) -> bool {
        todo!("port MSDevice_Taxi::isReservation")
    }

    /// C++: `void MSDevice_Taxi::addReservation(MSTransportable *, const std::set<std::string> &, SUMOTime, SUMOTime, SUMOTime, const MSEdge *, double, const MSStoppingPlace *, const MSEdge *, double, const MSStoppingPlace *, const std::string &)` [static]
    /// add new reservation
    // void
    // MSDevice_Taxi::addReservation(MSTransportable* person,
    //                               const std::set<std::string>& lines,
    //                               SUMOTime reservationTime,
    //                               SUMOTime pickupTime,
    //                               SUMOTime earliestPickupTime,
    //                               const MSEdge* from, double fromPos,
    //                               const MSStoppingPlace* fromStop,
    //                               const MSEdge* to, double toPos,
    //                               const MSStoppingPlace* toStop,
    //                               const std::string& group) {
    //     if (!isReservation(lines)) {
    //         return;
    //     }
    //     if ((to->getPermissions() & gTaxiClasses) == 0) {
    //         throw ProcessError("Cannot add taxi reservation for " + std::string(person->isPerson() ? "person" : "container")
    //                            + " '" + person->getID() + "' because destination edge '" + to->getID() + "'"
    //                            + " does not permit taxi access");
    //     }
    //     if ((from->getPermissions() & gTaxiClasses) == 0) {
    //         throw ProcessError("Cannot add taxi reservation for " + std::string(person->isPerson() ? "person" : "container")
    //                            + " '" + person->getID() + "' because origin edge '" + from->getID() + "'"
    //                            + " does not permit taxi access");
    //     }
    //     if (myDispatchCommand == nullptr) {
    //         initDispatch();
    //     }
    //     if (fromStop != nullptr && &fromStop->getLane().getEdge() == from) {
    //         // pickup position should be at the stop-endPos
    //         fromPos = fromStop->getEndLanePosition();
    //     }
    //     Reservation* res = myDispatcher->addReservation(person, reservationTime, pickupTime, earliestPickupTime, from, fromPos, fromStop, to, toPos, toStop, group, *lines.begin(), myMaxCapacity, myMaxContainerCapacity);
    //     if (myStateLoadedCustomers.size() > 0) {
    //         auto it = myStateLoadedCustomers.find(person->getID());
    //         if (it != myStateLoadedCustomers.end()) {
    //             //std::cout << SIMTIME << " loadedServed p=" << person->getID() << " res=" << res->getID() << " taxi=" << it->second->getID() << "\n";
    //             myDispatcher->servedReservation(res, it->second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_reservation(person: Ptr<MSTransportable>, lines: &BTreeSet<String>, reservationTime: SUMOTime, pickupTime: SUMOTime, earliestPickupTime: SUMOTime, from: Ptr<MSEdge>, fromPos: f64, fromStop: Ptr<MSStoppingPlace>, to: Ptr<MSEdge>, toPos: f64, toStop: Ptr<MSStoppingPlace>, group: &str) {
        todo!("port MSDevice_Taxi::addReservation")
    }

    /// C++: `void MSDevice_Taxi::removeReservation(MSTransportable *, const std::set<std::string> &, const MSEdge *, double, const MSEdge *, double, const std::string &)` [static]
    /// retract reservation
    // void
    // MSDevice_Taxi::removeReservation(MSTransportable* person,
    //                                  const std::set<std::string>& lines,
    //                                  const MSEdge* from, double fromPos,
    //                                  const MSEdge* to, double toPos,
    //                                  const std::string& group) {
    //     if (myDispatcher != nullptr && lines.size() == 1 && *lines.begin() == TAXI_SERVICE) {
    //         myDispatcher->removeReservation(person, from, fromPos, to, toPos, group);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_reservation(person: Ptr<MSTransportable>, lines: &BTreeSet<String>, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, group: &str) {
        todo!("port MSDevice_Taxi::removeReservation")
    }

    /// C++: `void MSDevice_Taxi::updateReservationFromPos(MSTransportable *, const std::set<std::string> &, const MSEdge *, double, const MSEdge *, double, const std::string &, double)` [static]
    /// update reservation's fromPos due to pre-booking
    // void
    // MSDevice_Taxi::updateReservationFromPos(MSTransportable* person,
    //                                         const std::set<std::string>& lines,
    //                                         const MSEdge* from, double fromPos,
    //                                         const MSEdge* to, double toPos,
    //                                         const std::string& group, double newFromPos) {
    //     if (myDispatcher != nullptr && lines.size() == 1 && *lines.begin() == TAXI_SERVICE) {
    //         myDispatcher->updateReservationFromPos(person, from, fromPos, to, toPos, group, newFromPos);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_reservation_from_pos(person: Ptr<MSTransportable>, lines: &BTreeSet<String>, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, group: &str, newFromPos: f64) {
        todo!("port MSDevice_Taxi::updateReservationFromPos")
    }

    /// C++: `SUMOTime MSDevice_Taxi::triggerDispatch(SUMOTime)` [static]
    /// period command to trigger the dispatch algorithm
    // SUMOTime
    // MSDevice_Taxi::triggerDispatch(SUMOTime currentTime) {
    //     std::vector<MSDevice_Taxi*> active;
    //     for (MSDevice_Taxi* taxi : myFleet) {
    //         if (taxi->getHolder().hasDeparted()) {
    //             active.push_back(taxi);
    //         }
    //     }
    //     myDispatcher->computeDispatch(currentTime, active);
    //     myNextDispatchTime = currentTime + myDispatchPeriod;
    //     return myDispatchPeriod;
    // }
    #[allow(unused_variables)]
    pub fn trigger_dispatch(currentTime: SUMOTime) -> SUMOTime {
        todo!("port MSDevice_Taxi::triggerDispatch")
    }

    /// C++: `bool MSDevice_Taxi::hasServableReservations()` [static]
    /// check whether there are still (servable) reservations in the system
    // bool
    // MSDevice_Taxi::hasServableReservations() {
    //     return myDispatcher != nullptr && myDispatcher->hasServableReservations();
    // }
    #[allow(unused_variables)]
    pub fn has_servable_reservations() -> bool {
        todo!("port MSDevice_Taxi::hasServableReservations")
    }

    /// C++: `void MSDevice_Taxi::cleanup()` [static]
    /// resets counters
    // void
    // MSDevice_Taxi::cleanup() {
    //     if (myDispatcher != nullptr) {
    //         delete myDispatcher;
    //         myDispatcher = nullptr;
    //     }
    //     myDispatchCommand = nullptr;
    //     myTaxiTypes.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSDevice_Taxi::cleanup")
    }

    /// C++: `MSDispatch * MSDevice_Taxi::getDispatchAlgorithm()` [static]
    // static MSDispatch* getDispatchAlgorithm() {
    //         return myDispatcher;
    //     }
    #[allow(unused_variables)]
    pub fn get_dispatch_algorithm() -> Ptr<MSDispatch> {
        todo!("port MSDevice_Taxi::getDispatchAlgorithm")
    }

    /// C++: `const std::vector<MSDevice_Taxi *> & MSDevice_Taxi::getFleet()` [static]
    // static const std::vector<MSDevice_Taxi*>& getFleet() {
    //         return myFleet;
    //     }
    #[allow(unused_variables)]
    pub fn get_fleet() -> &'static Vec<Ptr<MSDevice_Taxi>> {
        todo!("port MSDevice_Taxi::getFleet")
    }

    /// C++: `void MSDevice_Taxi::~MSDevice_Taxi() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Taxi::~MSDevice_Taxi() {
    //     myFleet.erase(std::find(myFleet.begin(), myFleet.end(), this));
    //     // recompute myMaxCapacity
    //     myMaxCapacity = 0;
    //     myMaxContainerCapacity = 0;
    //     for (MSDevice_Taxi* taxi : myFleet) {
    //         myMaxCapacity = MAX2(myMaxCapacity, taxi->getHolder().getVehicleType().getPersonCapacity());
    //         myMaxContainerCapacity = MAX2(myMaxContainerCapacity, taxi->getHolder().getVehicleType().getContainerCapacity());
    //     }
    //     delete myIdleAlgorithm;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_Taxi::~MSDevice_Taxi")
    }

    /// C++: `bool MSDevice_Taxi::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_Taxi::notifyMove(SUMOTrafficObject& /*tObject*/, double oldPos,
    //                           double newPos, double /*newSpeed*/) {
    //     updateMove(DELTA_T, newPos - oldPos);
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_Taxi::notifyMove")
    }

    /// C++: `bool MSDevice_Taxi::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves departure info on insertion
    // bool
    // MSDevice_Taxi::notifyEnter(SUMOTrafficObject& /*veh*/, MSMoveReminder::Notification /*reason*/, const MSLane* /* enteredLane */) {
    //     if (isEmpty() && MSNet::getInstance()->getCurrentTimeStep() < myServiceEnd) {
    //         myIdleAlgorithm->idle(this);
    //     }
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSDevice_Taxi::notifyEnter")
    }

    /// C++: `const std::string MSDevice_Taxi::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "taxi";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSDevice_Taxi::deviceName")
    }

    /// C++: `bool MSDevice_Taxi::isEmpty()`
    /// whether the taxi is empty
    // bool
    // MSDevice_Taxi::isEmpty() {
    //     return myState == EMPTY;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&mut self) -> bool {
        todo!("port MSDevice_Taxi::isEmpty")
    }

    /// C++: `int MSDevice_Taxi::getState() const` [const]
    // int getState() const {
    //         return myState;
    //     }
    #[allow(unused_variables)]
    pub fn get_state(&self) -> i32 {
        todo!("port MSDevice_Taxi::getState")
    }

    /// C++: `bool MSDevice_Taxi::hasFleet()` [static]
    /// returns whether taxis have been loaded
    // bool
    // MSDevice_Taxi::hasFleet() {
    //     return myFleet.size() > 0;;
    // }
    #[allow(unused_variables)]
    pub fn has_fleet() -> bool {
        todo!("port MSDevice_Taxi::hasFleet")
    }

    /// C++: `void MSDevice_Taxi::dispatch(const Reservation &)`
    /// service the given reservation
    // void
    // MSDevice_Taxi::dispatch(const Reservation& res) {
    //     dispatchShared({&res, &res});
    // }
    #[allow(unused_variables)]
    pub fn dispatch(&mut self, res: &Reservation) {
        todo!("port MSDevice_Taxi::dispatch")
    }

    /// C++: `void MSDevice_Taxi::dispatchShared(std::vector<const Reservation *>)`
    /// service the given reservations
    // void
    // MSDevice_Taxi::dispatchShared(std::vector<const Reservation*> reservations) {
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " taxi=" << myHolder.getID() << " dispatch:\n";
    //         for (const Reservation* res : reservations) {
    //             std::cout << "   res=" << res->getID();
    //             std::cout << " persons=" << toString(res->persons) << "\n";
    //         }
    //     }
    // #endif
    //     myLastDispatch = reservations;
    //     ConstMSEdgeVector tmpEdges;
    //     StopParVector stops;
    //     double lastPos = myHolder.getPositionOnLane();
    //     const MSEdge* rerouteOrigin = *myHolder.getRerouteOrigin();
    //     if (isEmpty()) {
    //         // start fresh from the current edge
    //         if (myHolder.isStoppedParking()) {
    //             // parking stop must be ended normally
    //             MSStop& stop = myHolder.getNextStopMutable();
    //             stop.duration = 0;
    //             lastPos = stop.pars.endPos;
    //             if (myHolder.isStoppedTriggered()) {
    //                 stop.triggered = false;
    //                 stop.containerTriggered = false;
    //                 stop.joinTriggered = false;
    //                 myHolder.unregisterWaiting();
    //             }
    //             // prevent unauthorized/premature entry
    //             const_cast<SUMOVehicleParameter::Stop&>(stop.pars).permitted.insert("");
    //             while (myHolder.getStops().size() > 1) {
    //                 myHolder.abortNextStop(1);
    //             }
    //         } else {
    //             while (myHolder.hasStops()) {
    //                 // in meso there might be more than 1 stop at this point
    //                 myHolder.abortNextStop();
    //             }
    //             assert(!myHolder.hasStops());
    //         }
    //         tmpEdges.push_back(myHolder.getEdge());
    //         if (myHolder.getEdge() != rerouteOrigin) {
    //             tmpEdges.push_back(rerouteOrigin);
    //         }
    //     } else {
    //         assert(myHolder.hasStops());
    //         // check how often existing customers appear in the new reservations
    //         std::map<const MSTransportable*, int> nOccur;
    //         for (const Reservation* res : reservations) {
    //             for (const MSTransportable* person : res->persons) {
    //                 if (myCustomers.count(person) != 0) {
    //                     nOccur[person] += 1;
    //                     if (myCurrentReservations.count(res) == 0) {
    //                         throw ProcessError(TLF("Invalid Re-dispatch for existing customer '%' with a new reservation", person->getID()));
    //                     }
    //                 }
    //             }
    //         }
    // #ifdef DEBUG_DISPATCH
    //         if (DEBUG_COND) {
    //             for (auto item : nOccur) {
    //                 std::cout << "   previousCustomer=" << item.first->getID() << " occurs=" << item.second << "\n";
    //             }
    //         }
    // #endif
    //         if (nOccur.size() == 0) {
    //             // no overlap with existing customers - extend route
    //             tmpEdges = myHolder.getRoute().getEdges();
    //             lastPos = myHolder.getStops().back().pars.endPos;
    // #ifdef DEBUG_DISPATCH
    //             if (DEBUG_COND) {
    //                 std::cout << " re-dispatch with route-extension\n";
    //             }
    // #endif
    //         } else if (nOccur.size() == myCustomers.size()) {
    //             // redefine route (verify correct number of mentions)
    //             std::set<const MSTransportable*> onBoard;
    //             const std::vector<MSTransportable*>& onBoardP = myHolder.getPersons();
    //             const std::vector<MSTransportable*>& onBoardC = myHolder.getContainers();
    //             onBoard.insert(onBoardP.begin(), onBoardP.end());
    //             onBoard.insert(onBoardC.begin(), onBoardC.end());
    //             std::set<const MSTransportable*> redundantPickup;
    //             for (auto item : nOccur) {
    //                 if (item.second == 1) {
    //                     // customers must already be on board
    //                     if (onBoard.count(item.first) == 0) {
    //                         throw ProcessError(TLF("Re-dispatch did not mention pickup for existing customer '%'", item.first->getID()));
    //                     }
    //                 } else if (item.second == 2) {
    //                     if (onBoard.count(item.first) == 0) {
    //                         // treat like a new customer
    //                         // TODO: need to be checked
    //                         myCustomers.erase(item.first);
    //                     } else {
    //                         redundantPickup.insert(item.first);
    //                     }
    //                 } else {
    //                     throw ProcessError("Re-dispatch mentions existing customer '" + item.first->getID() + "' " + toString(item.second) + " times");
    //                 }
    //             }
    //             // remove redundancy
    //             if (!redundantPickup.empty()) {
    //                 for (auto it = reservations.begin(); it != reservations.end();) {
    //                     bool isRedundant = false;
    //                     for (const MSTransportable* person : (*it)->persons) {
    //                         if (redundantPickup.count(person) != 0) {
    //                             isRedundant = true;
    //                             break;
    //                         }
    //                     }
    //                     if (isRedundant) {
    //                         for (const MSTransportable* person : (*it)->persons) {
    //                             redundantPickup.erase(person);
    //                         }
    //                         it = reservations.erase(it);
    //                     } else {
    //                         it++;
    //                     }
    //                 }
    //             }
    //             while (myHolder.hasStops()) {
    //                 myHolder.abortNextStop();
    //             }
    //             tmpEdges.push_back(myHolder.getEdge());
    //             if (myHolder.getEdge() != rerouteOrigin) {
    //                 tmpEdges.push_back(rerouteOrigin);
    //             }
    // #ifdef DEBUG_DISPATCH
    //             if (DEBUG_COND) {
    //                 std::cout << " re-dispatch from scratch\n";
    //             }
    // #endif
    //         } else {
    //             // inconsistent re-dispatch
    //             std::vector<std::string> missing;
    //             for (const MSTransportable* c : myCustomers) {
    //                 if (nOccur.count(c) == 0) {
    //                     missing.push_back(c->getID());
    //                 }
    //             }
    //             throw ProcessError("Re-dispatch did mention some customers but failed to mention " + joinToStringSorting(missing, " "));
    //         }
    //     }
    //
    //     bool hasPickup = false;
    //     for (const Reservation* res : reservations) {
    //         myCurrentReservations.insert(res);
    //         bool isPickup = false;
    //         for (const MSTransportable* person : res->persons) {
    //             if (myCustomers.count(person) == 0) {
    //                 myCustomers.insert(person);
    //                 isPickup = true;
    //                 hasPickup = true;
    //             }
    //         }
    //         if (isPickup) {
    //             prepareStop(tmpEdges, stops, lastPos, res->from, res->fromPos, res->fromStop, "pickup " + toString(res->persons) + " (" + res->id + ")", res, isPickup);
    //             for (const MSTransportable* const transportable : res->persons) {
    //                 if (transportable->isPerson()) {
    //                     stops.back().triggered = true;
    //                 } else {
    //                     stops.back().containerTriggered = true;
    //                 }
    //                 stops.back().permitted.insert(transportable->getID());
    //                 stops.back().parametersSet |= STOP_PERMITTED_SET | STOP_TRIGGER_SET;
    //             }
    //             for (const MSTransportable* t : res->persons) {
    //                 stops.back().awaitedPersons.insert(t->getID());
    //             }
    //
    //             if (stops.back().duration == -1) {
    //                 // keep dropOffDuration if the stop is dropOff and pickUp
    //                 stops.back().duration = TIME2STEPS(myHolder.getFloatParam("device.taxi.pickUpDuration", false, 0));
    //             }
    //         } else {
    //             prepareStop(tmpEdges, stops, lastPos, res->to, res->toPos, res->toStop, "dropOff " + toString(res->persons) + " (" + res->id + ")", res, isPickup);
    //             stops.back().duration = TIME2STEPS(myHolder.getFloatParam("device.taxi.dropOffDuration", false, 60)); // pay and collect bags
    //         }
    //         stops.back().parametersSet |= STOP_DURATION_SET | STOP_PARKING_SET;
    //     }
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND) {
    //         std::cout << "   tmpEdges=" << toString(tmpEdges) << "\n";
    //     }
    // #endif
    //     const SUMOTime t = SIMSTEP;
    //     if (!myHolder.replaceRouteEdges(tmpEdges, -1, 0, "taxi:prepare_dispatch", false, false, false)) {
    //         throw ProcessError("Route replacement for taxi dispatch failed for vehicle '" + myHolder.getID()
    //                            + "' at time=" + time2string(t) + ".");
    //     }
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND) std::cout << "   replacedRoute=" << toString(tmpEdges)
    //                                   << "\n     actualRoute=" << toString(myHolder.getRoute().getEdges()) << "\n";
    // #endif
    //     for (SUMOVehicleParameter::Stop& stop : stops) {
    //         std::string error;
    //         if (!myHolder.addStop(stop, error)) {
    //             WRITE_WARNINGF(TL("Could not add taxi stop for %, desc=% time=% error=%"), myHolder.getID(), stop.actType, time2string(t), error)
    //         }
    //     }
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //     // SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = myHolder.getInfluencer().getRouterTT(veh->getRNGIndex())
    //     myHolder.reroute(t, "taxi:dispatch", router, false);
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND) {
    //         std::cout << "\n      finalRoute=" << toString(myHolder.getRoute().getEdges()) << " routeIndex=" << myHolder.getRoutePosition() << "\n";
    //     }
    // #endif
    //     if (hasPickup) {
    //         myState |= PICKUP;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn dispatch_shared(&mut self, reservations: Vec<Ptr<Reservation>>) {
        todo!("port MSDevice_Taxi::dispatchShared")
    }

    /// C++: `void MSDevice_Taxi::cancelCurrentCustomers()`
    /// remove the persons the taxi is currently waiting for from reservations
    // void
    // MSDevice_Taxi::cancelCurrentCustomers() {
    //     // check if taxi has stopped
    //     if (myHolder.getNextStopParameter() == nullptr) {
    //         return;
    //     }
    //     // find customers of the current stop
    //     std::set<const MSTransportable*> customersToBeRemoved;
    //     std::set<const MSTransportable*> onBoard;
    //     onBoard.insert(myHolder.getPersons().begin(), myHolder.getPersons().end());
    //     onBoard.insert(myHolder.getContainers().begin(), myHolder.getContainers().end());
    //     for (std::string tID : myHolder.getNextStopParameter()->permitted) {
    //         for (auto t : myCustomers) {
    //             if (t->getID() == tID && onBoard.count(t) == 0) {
    //                 customersToBeRemoved.insert(t);
    //             }
    //         }
    //     }
    //     if (!customersToBeRemoved.empty()) {
    //         WRITE_WARNINGF(TL("Taxi '%' aborts waiting for customers: % at time=%."),
    //                        myHolder.getID(), toString(customersToBeRemoved), time2string(SIMSTEP));
    //     }
    //     for (auto t : customersToBeRemoved) {
    //         cancelCustomer(t);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cancel_current_customers(&mut self) {
        todo!("port MSDevice_Taxi::cancelCurrentCustomers")
    }

    /// C++: `bool MSDevice_Taxi::cancelCustomer(const MSTransportable *)`
    /// remove person from reservations
    // bool
    // MSDevice_Taxi::cancelCustomer(const MSTransportable* t) {
    // #ifdef DEBUG_CANCEL
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " taxi=" << myHolder.getID() << " cancelCustomer " << t->getID() << "\n";
    //     }
    // #endif
    //
    //     // is the given transportable a customer of the reservations?
    //     if (myCustomers.count(t) == 0) {
    //         return false;
    //     }
    //     myCustomers.erase(t);
    //     // check whether a single reservation has been fulfilled or another customer is part of the reservation
    //     for (auto resIt = myCurrentReservations.begin(); resIt != myCurrentReservations.end();) {
    //         bool fulfilled = false;
    //         if ((*resIt)->persons.size() == 1 && (*resIt)->persons.count(t) != 0) {
    //             // the reservation contains only the customer
    //             fulfilled = true;
    //         }
    //         if (fulfilled) {
    //             const Reservation* res = *resIt;
    //             // remove reservation from the current dispatch
    //             for (auto it = myLastDispatch.begin(); it != myLastDispatch.end();) {
    //                 if (*it == res) {
    //                     it = myLastDispatch.erase(it);
    //                 } else {
    //                     ++it;
    //                 }
    //             }
    //             // remove reservation from the served reservations
    //             resIt = myCurrentReservations.erase(resIt);
    //             // delete the reservation
    //             myDispatcher->fulfilledReservation(res);
    //         } else {
    //             ++resIt;
    //         }
    //     }
    //     myState &= ~PICKUP;  // remove state PICKUP
    //     for (const Reservation* res : myCurrentReservations) {
    //         // if there is another pickup in the dispatch left, add the state PICKUP
    //         if (std::count(myLastDispatch.begin(), myLastDispatch.end(), res) == 2) {
    //             myState |= PICKUP;  // add state PICKUP
    //         }
    //     }
    //     // we also have to clean reservations from myLastDispatch where the customers arrived in the meantime
    //     for (auto it = myLastDispatch.begin(); it != myLastDispatch.end();) {
    //         if (myCurrentReservations.count(*it) == 0) {
    //             it = myLastDispatch.erase(it);
    //         } else {
    //             ++it;
    //         }
    //     }
    //     // if there are reservations left, go on with the dispatch
    //     // in meso, wait for the next dispatch cycle to avoid updating stops in this stage
    //     if (!MSGlobals::gUseMesoSim) {
    //         dispatchShared(myLastDispatch);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn cancel_customer(&mut self, t: Ptr<MSTransportable>) -> bool {
        todo!("port MSDevice_Taxi::cancelCustomer")
    }

    /// C++: `void MSDevice_Taxi::addCustomer(const MSTransportable *, const Reservation *)`
    /// add person after extending reservation
    // void
    // MSDevice_Taxi::addCustomer(const MSTransportable* t, const Reservation* res) {
    //     myCustomers.insert(t);
    //     MSBaseVehicle& veh = dynamic_cast<MSBaseVehicle&>(myHolder);
    //     for (const MSStop& stop : veh.getStops()) {
    //         SUMOVehicleParameter::Stop& pars = const_cast<SUMOVehicleParameter::Stop&>(stop.pars);
    //         //std::cout << " sE=" << (*stop.edge)->getID() << " sStart=" << pars.startPos << " sEnd=" << pars.endPos << " rFrom=" <<
    //         //    res->from->getID() << " rTo=" << res->to->getID() << " rFromPos=" << res->fromPos << " resToPos=" << res->toPos << "\n";
    //         if (*stop.edge == res->from
    //                 && pars.startPos <= res->fromPos
    //                 && pars.endPos >= res->fromPos) {
    //             pars.awaitedPersons.insert(t->getID());
    //             pars.permitted.insert(t->getID());
    //             pars.actType += " +" + t->getID();
    //         } else if (*stop.edge == res->to
    //                 && pars.startPos <= res->toPos
    //                 && pars.endPos >= res->toPos) {
    //             pars.actType += " +" + t->getID();
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_customer(&mut self, t: Ptr<MSTransportable>, res: Ptr<Reservation>) {
        todo!("port MSDevice_Taxi::addCustomer")
    }

    /// C++: `bool MSDevice_Taxi::allowsBoarding(const MSTransportable *) const` [const]
    /// whether the given person is allowed to board this taxi
    // bool
    // MSDevice_Taxi::allowsBoarding(const MSTransportable* t) const {
    //     return myCustomers.count(t) != 0;
    // }
    #[allow(unused_variables)]
    pub fn allows_boarding(&self, t: Ptr<MSTransportable>) -> bool {
        todo!("port MSDevice_Taxi::allowsBoarding")
    }

    /// C++: `void MSDevice_Taxi::customerEntered(const MSTransportable *)`
    /// called by MSDevice_Transportable upon loading a person
    // void
    // MSDevice_Taxi::customerEntered(const MSTransportable* t) {
    //     myState |= OCCUPIED;
    //     if (!hasFuturePickup()) {
    //         myState &= ~PICKUP;
    //     }
    //     for (const Reservation* res : myCurrentReservations) {
    //         for (const MSTransportable* cand : res->persons) {
    //             if (cand == t) {
    //                 const_cast<Reservation*>(res)->state = Reservation::ONBOARD;
    //                 break;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn customer_entered(&mut self, t: Ptr<MSTransportable>) {
        todo!("port MSDevice_Taxi::customerEntered")
    }

    /// C++: `void MSDevice_Taxi::customerArrived(const MSTransportable *)`
    /// called by MSDevice_Transportable upon unloading a person
    // void
    // MSDevice_Taxi::customerArrived(const MSTransportable* person) {
    //     myCustomersServed++;
    //     myCustomers.erase(person);
    //     if (myHolder.getPersonNumber() == 0 && myHolder.getContainerNumber() == 0) {
    //         myState &= ~OCCUPIED;
    //         if (myHolder.getStops().size() > 1 && (myState & PICKUP) == 0) {
    //             WRITE_WARNINGF(TL("All customers left vehicle '%' at time=% but there are % remaining stops"),
    //                            myHolder.getID(), time2string(SIMSTEP), myHolder.getStops().size() - 1);
    //             while (myHolder.getStops().size() > 1) {
    //                 myHolder.abortNextStop(1);
    //             }
    //         }
    //     }
    //     if (isEmpty()) {
    //         // cleanup
    //         for (const Reservation* res : myCurrentReservations) {
    //             myDispatcher->fulfilledReservation(res);
    //         }
    //         myCurrentReservations.clear();
    //         checkTaskSwap();
    //         if (isEmpty()) {
    //             if (MSGlobals::gUseMesoSim && MSNet::getInstance()->getCurrentTimeStep() < myServiceEnd) {
    //                 myIdleAlgorithm->idle(this);
    //             }
    //         }
    //     } else {
    //         // check whether a single reservation has been fulfilled
    //         for (auto resIt = myCurrentReservations.begin(); resIt != myCurrentReservations.end();) {
    //             bool fulfilled = true;
    //             for (const MSTransportable* t : (*resIt)->persons) {
    //                 if (myCustomers.count(t) != 0) {
    //                     fulfilled = false;
    //                     break;
    //                 }
    //             }
    //             if (fulfilled) {
    //                 myDispatcher->fulfilledReservation(*resIt);
    //                 resIt = myCurrentReservations.erase(resIt);
    //             } else {
    //                 ++resIt;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn customer_arrived(&mut self, person: Ptr<MSTransportable>) {
        todo!("port MSDevice_Taxi::customerArrived")
    }

    /// C++: `void MSDevice_Taxi::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDevice_Taxi::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_DEVICE);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     std::ostringstream internals;
    //     internals << myState
    //         << " " << myCustomersServed
    //         << " " << myOccupiedDistance
    //         << " " << myOccupiedTime;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     if (myCustomers.size() > 0) {
    //         out.writeAttr(SUMO_ATTR_CUSTOMERS, joinNamedToStringSorting(myCustomers, " "));
    //     }
    //     if (myCurrentReservations.size() > 0) {
    //         out.writeAttr(SUMO_ATTR_RESERVATIONS, joinNamedToStringSorting(myCurrentReservations, " "));
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        todo!("port MSDevice_Taxi::saveState")
    }

    /// C++: `void MSDevice_Taxi::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDevice_Taxi::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myState;
    //     bis >> myCustomersServed;
    //     bis >> myOccupiedDistance;
    //     bis >> myOccupiedTime;
    //     if (attrs.hasAttribute(SUMO_ATTR_CUSTOMERS)) {
    //         for (const std::string& id : attrs.get<std::vector<std::string> >(SUMO_ATTR_CUSTOMERS, getID().c_str(), ok)) {
    //             myStateLoadedCustomers[id] = this;
    //         }
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_RESERVATIONS)) {
    //         for (const std::string& id : attrs.get<std::vector<std::string> >(SUMO_ATTR_RESERVATIONS, getID().c_str(), ok)) {
    //             myStateLoadedReservations[id] = this;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port MSDevice_Taxi::loadState")
    }

    /// C++: `void MSDevice_Taxi::finalizeLoadState()` [static]
    /// call during state loading after all transportables are loaded
    // void
    // MSDevice_Taxi::finalizeLoadState() {
    //     if (myStateLoadedCustomers.size() > 0) {
    //         MSNet* net = MSNet::getInstance();
    //         MSTransportableControl* pc = net->hasPersons() ? &net->getPersonControl() : nullptr;
    //         MSTransportableControl* cc = net->hasContainers() ? &net->getContainerControl() : nullptr;
    //         for (auto item : myStateLoadedCustomers) {
    //             MSTransportable* t = nullptr;
    //             if (pc != nullptr) {
    //                 t = pc->get(item.first);
    //             }
    //             if (t == nullptr && cc != nullptr) {
    //                 t = cc->get(item.first);
    //             }
    //             if (t == nullptr) {
    //                 WRITE_ERRORF("Could not find taxi customer '%'. Ensure state contains transportables", item.first);
    //             } else {
    //                 item.second->myCustomers.insert(t);
    //             }
    //         }
    //     }
    //     if (myStateLoadedReservations.size() > 0) {
    //         assert(myDispatcher != nullptr);
    //         std::map<std::string, const Reservation*> resLookup;
    //         for (const Reservation* res : myDispatcher->getReservations()) {
    //             resLookup[res->getID()] = res;
    //         }
    //         for (const Reservation* res : myDispatcher->getRunningReservations()) {
    //             resLookup[res->getID()] = res;
    //         }
    //         for (auto item : myStateLoadedReservations) {
    //             auto it = resLookup.find(item.first);
    //             if (it == resLookup.end()) {
    //                 WRITE_ERRORF("Could not find taxi reservation '%'.", item.first);
    //             } else {
    //                 item.second->myCurrentReservations.insert(it->second);
    //             }
    //         }
    //     }
    //     myStateLoadedCustomers.clear();
    //     myStateLoadedReservations.clear();
    // }
    #[allow(unused_variables)]
    pub fn finalize_load_state() {
        todo!("port MSDevice_Taxi::finalizeLoadState")
    }

    /// C++: `std::string MSDevice_Taxi::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Taxi::getParameter(const std::string& key) const {
    //     if (key == "customers") {
    //         return toString(myCustomersServed);
    //     } else if (key == "occupiedDistance") {
    //         return toString(myOccupiedDistance);
    //     } else if (key == "occupiedTime") {
    //         return toString(STEPS2TIME(myOccupiedTime));
    //     } else if (key == "state") {
    //         return toString(myState);
    //     } else if (key == "currentCustomers") {
    //         return joinNamedToStringSorting(myCustomers, " ");
    //     } else if (key == "pickUpDuration") {
    //         return myHolder.getStringParam("device.taxi.pickUpDuration", false, "0");
    //     } else if (key == "dropOffDuration") {
    //         return myHolder.getStringParam("device.taxi.dropOffDuration", false, "60");
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        todo!("port MSDevice_Taxi::getParameter")
    }

    /// C++: `void MSDevice_Taxi::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Taxi::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == "pickUpDuration" || key == "dropOffDuration") {
    //         // store as generic vehicle parameters
    //         ((SUMOVehicleParameter&)myHolder.getParameter()).setParameter("device.taxi." + key, value);
    //     } else {
    //         UNUSED_PARAMETER(doubleValue);
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSDevice_Taxi::setParameter")
    }

    /// C++: `void MSDevice_Taxi::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_Taxi::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->openTag("taxi");
    //         tripinfoOut->writeAttr("customers", toString(myCustomersServed));
    //         tripinfoOut->writeAttr("occupiedDistance", toString(myOccupiedDistance));
    //         tripinfoOut->writeAttr("occupiedTime", time2string(myOccupiedTime));
    //         tripinfoOut->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        todo!("port MSDevice_Taxi::generateOutput")
    }

    /// C++: `bool MSDevice_Taxi::compatibleLine(const Reservation *)`
    /// whether the given reservation is compatible with the taxi line
    // bool
    // MSDevice_Taxi::compatibleLine(const Reservation* res) {
    //     return compatibleLine(myHolder.getParameter().line, res->line);
    // }
    #[allow(unused_variables)]
    pub fn compatible_line(&mut self, res: Ptr<Reservation>) -> bool {
        todo!("port MSDevice_Taxi::compatibleLine")
    }

    /// C++: `bool MSDevice_Taxi::compatibleLine(const std::string &, const std::string &)` [static]
    // bool
    // MSDevice_Taxi::compatibleLine(const std::string& taxiLine, const std::string& rideLine) {
    //     return ((taxiLine == rideLine && StringUtils::startsWith(rideLine, "taxi") && StringUtils::startsWith(taxiLine, "taxi"))
    //             || (taxiLine == TAXI_SERVICE && StringUtils::startsWith(rideLine, "taxi:"))
    //             || (rideLine == TAXI_SERVICE && StringUtils::startsWith(taxiLine, "taxi:")));
    // }
    #[allow(unused_variables)]
    pub fn compatible_line_2(taxiLine: &str, rideLine: &str) -> bool {
        todo!("port MSDevice_Taxi::compatibleLine")
    }

    /// C++: `void MSDevice_Taxi::allCustomersErased()` [static]
    /// signal the end of the simulation and the removal of all customers
    // void
    // MSDevice_Taxi::allCustomersErased() {
    //     for (MSDevice_Taxi* taxi : myFleet) {
    //         // disable taskSwap
    //         taxi->myState = EMPTY;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn all_customers_erased() {
        todo!("port MSDevice_Taxi::allCustomersErased")
    }

    /// C++: `const std::map<SUMOVehicleClass, std::string> & MSDevice_Taxi::getTaxiTypes()` [static]
    /// return all types that are known to carry a taxi device (or the default type if no devices are initialized)
    // const std::map<SUMOVehicleClass, std::string>&
    // MSDevice_Taxi::getTaxiTypes() {
    //     if (myTaxiTypes.size() < std::bitset<64>(gTaxiClasses).count()) {
    //         for (const std::string& vClassName : OptionsCont::getOptions().getStringVector("device.taxi.vclasses")) {
    //             SUMOVehicleClass svc = (SUMOVehicleClass)parseVehicleClasses(vClassName);
    //             if (myTaxiTypes[svc] == "") {
    //                 switch(svc) {
    //                     // @see MSVehicleControl::initDefaultTypes()
    //                     case SVC_TAXI:
    //                         myTaxiTypes[svc] = DEFAULT_TAXITYPE_ID;
    //                         break;
    //                     case SVC_RAIL:
    //                         myTaxiTypes[svc] = DEFAULT_RAILTYPE_ID;
    //                         break;
    //                     case SVC_BICYCLE:
    //                         myTaxiTypes[svc] = DEFAULT_BIKETYPE_ID;
    //                         break;
    //                     case SVC_PASSENGER:
    //                         myTaxiTypes[svc] = DEFAULT_VTYPE_ID;
    //                         break;
    //                     default: {
    //                         const std::string typeID = "DEFAULT_" + StringUtils::to_upper_case(vClassName) + "TYPE";
    //                         MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //                         if (!vc.hasVType(typeID)) {
    //                             SUMOVTypeParameter tp(typeID, svc);
    //                             MSVehicleType* t = MSVehicleType::build(tp);
    //                             vc.addVType(t);
    //                         }
    //                         myTaxiTypes[svc] = typeID;;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return myTaxiTypes;
    // }
    #[allow(unused_variables)]
    pub fn get_taxi_types() -> &'static BTreeMap<SUMOVehicleClass, String> {
        todo!("port MSDevice_Taxi::getTaxiTypes")
    }

    /// C++: `SUMOTime MSDevice_Taxi::getNextDispatchTime()` [static]
    // SUMOTime
    // MSDevice_Taxi::getNextDispatchTime() {
    //     return myNextDispatchTime;
    // }
    #[allow(unused_variables)]
    pub fn get_next_dispatch_time() -> SUMOTime {
        todo!("port MSDevice_Taxi::getNextDispatchTime")
    }

    /// C++: `void MSDevice_Taxi::initDispatch(SUMOTime)` [static]
    /// initialize the dispatch algorithm
    // void
    // MSDevice_Taxi::initDispatch(SUMOTime next) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     myDispatchPeriod = string2time(oc.getString("device.taxi.dispatch-period"));
    //     // init dispatch algorithm
    //     std::string algo = oc.getString("device.taxi.dispatch-algorithm");
    //     Parameterised params;
    //     params.setParametersStr(OptionsCont::getOptions().getString("device.taxi.dispatch-algorithm.params"), ":", ",");
    //     if (algo == "greedy") {
    //         myDispatcher = new MSDispatch_Greedy(params.getParametersMap());
    //     } else if (algo == "greedyClosest") {
    //         myDispatcher = new MSDispatch_GreedyClosest(params.getParametersMap());
    //     } else if (algo == "greedyShared") {
    //         myDispatcher = new MSDispatch_GreedyShared(params.getParametersMap());
    //     } else if (algo == "routeExtension") {
    //         myDispatcher = new MSDispatch_RouteExtension(params.getParametersMap());
    //     } else if (algo == "traci") {
    //         myDispatcher = new MSDispatch_TraCI(params.getParametersMap());
    //     } else {
    //         throw ProcessError(TLF("Dispatch algorithm '%' is not known", algo));
    //     }
    //     myDispatchCommand = new StaticCommand<MSDevice_Taxi>(&MSDevice_Taxi::triggerDispatch);
    //     // round to next multiple of myDispatchPeriod
    //     if (next < 0) {
    //         const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //         const SUMOTime begin = string2time(oc.getString("begin"));
    //         const SUMOTime delay = (myDispatchPeriod - ((now - begin) % myDispatchPeriod)) % myDispatchPeriod;
    //         next = now + delay;
    //     }
    //     myNextDispatchTime = next;
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myDispatchCommand, next);
    // }
    #[allow(unused_variables)]
    pub fn init_dispatch(next: SUMOTime) {
        todo!("port MSDevice_Taxi::initDispatch")
    }

    /// C++: `void MSDevice_Taxi::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual, protected]
    /// Internal notification about the vehicle moves, see MSMoveReminder::notifyMoveInternal()
    // void
    // MSDevice_Taxi::notifyMoveInternal(const SUMOTrafficObject& /* veh */,
    //                                   const double /* frontOnLane */,
    //                                   const double timeOnLane,
    //                                   const double /* meanSpeedFrontOnLane */,
    //                                   const double /* meanSpeedVehicleOnLane */,
    //                                   const double travelledDistanceFrontOnLane,
    //                                   const double /* travelledDistanceVehicleOnLane */,
    //                                   const double /* meanLengthOnLane */) {
    //     updateMove(TIME2STEPS(timeOnLane), travelledDistanceFrontOnLane);
    // }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        todo!("port MSDevice_Taxi::notifyMoveInternal")
    }

    /// C++: `void MSDevice_Taxi::MSDevice_Taxi(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSDevice_Taxi::MSDevice_Taxi(SUMOVehicle& holder, const std::string& id) :
    //     MSVehicleDevice(holder, id) {
    //     std::string defaultServiceEnd = toString(1e15);
    //     const std::string algo = holder.getStringParam("device.taxi.idle-algorithm");
    //     if (algo == "stop") {
    //         myIdleAlgorithm = new MSIdling_Stop();
    //     } else if (algo == "randomCircling") {
    //         myIdleAlgorithm = new MSIdling_RandomCircling();
    //         // make sure simulation terminates
    //         defaultServiceEnd = toString(STEPS2TIME(
    //                                          myHolder.getParameter().departProcedure == DepartDefinition::GIVEN
    //                                          ? myHolder.getParameter().depart
    //                                          : MSNet::getInstance()->getCurrentTimeStep()) + (3600 * 8));
    //     } else if (algo == "taxistand") {
    //         const std::string rerouterID = holder.getStringParam("device.taxi.stands-rerouter");
    //         if (rerouterID.empty()) {
    //             throw ProcessError("Idle algorithm '" + algo + "' requires a rerouter id to be defined using device param 'stands-rerouter' for vehicle '" + myHolder.getID() + "'");
    //         }
    //         if (MSTriggeredRerouter::getInstances().count(rerouterID) == 0) {
    //             throw ProcessError("Unknown rerouter '" + rerouterID + "' when loading taxi stands for vehicle '" + myHolder.getID() + "'");
    //         }
    //         MSTriggeredRerouter* rerouter = MSTriggeredRerouter::getInstances().find(rerouterID)->second;
    //         myIdleAlgorithm = new MSIdling_TaxiStand(rerouter);
    //     } else {
    //         throw ProcessError("Idle algorithm '" + algo + "' is not known for vehicle '" + myHolder.getID() + "'");
    //     }
    //     myServiceEnd = string2time(holder.getStringParam("device.taxi.end", false, defaultServiceEnd));
    //     myRoutingDevice = static_cast<MSDevice_Routing*>(myHolder.getDevice(typeid(MSDevice_Routing)));
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSDevice_Taxi::MSDevice_Taxi")
    }

    /// C++: `void MSDevice_Taxi::updateMove(const SUMOTime, const double)` [private]
    // void
    // MSDevice_Taxi::updateMove(const SUMOTime traveltime, const double travelledDist) {
    //     if (myHolder.getPersonNumber() > 0 || myHolder.getContainerNumber() > 0) {
    //         myOccupiedDistance += travelledDist;
    //         myOccupiedTime += traveltime;
    //     }
    //     if (isEmpty()) {
    //         if (MSNet::getInstance()->getCurrentTimeStep() < myServiceEnd) {
    //             myIdleAlgorithm->idle(this);
    //             if (myRoutingDevice != nullptr) {
    //                 // prevent rerouting during idling (#11079)
    //                 myRoutingDevice->setActive(false);
    //             }
    //         } else if (!myReachedServiceEnd) {
    //             WRITE_WARNINGF(TL("Taxi '%' reaches scheduled end of service at time=%."), myHolder.getID(), time2string(SIMSTEP));
    //             myReachedServiceEnd = true;
    //         }
    //     } else if (myRoutingDevice != nullptr) {
    //         myRoutingDevice->setActive(true);
    //     }
    //     if (myHolder.isStopped() && (isEmpty() || MSGlobals::gUseMesoSim) && myHolder.getNextStop().endBoarding > myServiceEnd) {
    //         // limit duration of stop (but only for idling-related stops)
    //         myHolder.getNextStopMutable().endBoarding = myServiceEnd;
    //     }
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND && myIsStopped != myHolder.isStopped()) {
    //         std::cout << SIMTIME << " updateMove veh=" << myHolder.getID() << " myIsStopped=" << myIsStopped << " myHolderStopped=" << myHolder.isStopped() << " myState=" << myState << "\n";
    //     }
    // #endif
    //     myIsStopped = myHolder.isStopped();
    // }
    #[allow(unused_variables)]
    pub fn update_move(&mut self, traveltime: SUMOTime, travelledDist: f64) {
        todo!("port MSDevice_Taxi::updateMove")
    }

    /// C++: `void MSDevice_Taxi::prepareStop(ConstMSEdgeVector &, StopParVector &, double &, const MSEdge *, double, const MSStoppingPlace *, const std::string &, const Reservation *, const bool)` [private]
    /// prepare stop for the given action
    // void
    // MSDevice_Taxi::prepareStop(ConstMSEdgeVector& edges,
    //                            StopParVector& stops,
    //                            double& lastPos, const MSEdge* stopEdge, double stopPos,
    //                            const MSStoppingPlace* stopPlace,
    //                            const std::string& action, const Reservation* res, const bool isPickup) {
    //     assert(!edges.empty());
    //     if (stopPlace != nullptr && &stopPlace->getLane().getEdge() == stopEdge) {
    //         stopPos = stopPlace->getEndLanePosition();
    //     }
    //     if (stopPos < lastPos && stopPos + NUMERICAL_EPS >= lastPos) {
    //         stopPos = lastPos;
    //     }
    //     bool addedEdge = false;
    //
    //     if (stops.empty()) {
    //         // check brakeGap
    //         double distToStop = stopPos - lastPos;
    //         const double brakeGap = myHolder.getBrakeGap();
    //         if (myHolder.getLane() != nullptr && myHolder.getLane()->isInternal()) {
    //             distToStop += myHolder.getLane()->getLength();
    //         }
    //         if (stopEdge != edges.back()) {
    //             distToStop += edges.back()->getLength();
    //             if (distToStop < brakeGap) {
    //                 // the distance between current edge and stop edge may be small
    //                 SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = MSRoutingEngine::getRouterTT(myHolder.getRNGIndex(), myHolder.getVClass());
    //                 ConstMSEdgeVector toFirstStop;
    //                 router.compute(edges.back(), stopEdge, &myHolder, SIMSTEP, toFirstStop, true);
    //                 for (int i = 1; i < (int)toFirstStop.size() - 1; i++) {
    //                     distToStop += toFirstStop[i]->getLength();
    //                 }
    //             }
    //         }
    //         if (distToStop < brakeGap) {
    //             // circle back to stopEdge
    //             //std::cout << SIMTIME << " taxi=" << getID() << " brakeGap=" << brakeGap << " distToStop=" << distToStop << "\n";
    //             edges.push_back(stopEdge);
    //             addedEdge = true;
    //         }
    //     }
    //
    //     if (stopEdge == edges.back() && !stops.empty()) {
    //         if (stopPos >= lastPos && stopPos <= stops.back().endPos) {
    //             // no new stop and no adaption needed
    //             stops.back().actType += "," + action;
    //             return;
    //         }
    //         if (stopPos >= lastPos && stopPos <= lastPos + myHolder.getVehicleType().getLength()) {
    //             // stop length adaption needed
    //             stops.back().endPos = MIN2(lastPos + myHolder.getVehicleType().getLength(), stopEdge->getLength());
    //             stops.back().actType += "," + action;
    //             return;
    //         }
    //     }
    //     if (!addedEdge && (stopEdge != edges.back() || stopPos < lastPos)) {
    //         //std::cout << SIMTIME << " stopPos=" << stopPos << " lastPos=" << lastPos << "\n";
    //         edges.push_back(stopEdge);
    //     }
    //     lastPos = stopPos;
    //     SUMOVehicleParameter::Stop stop;
    //     stop.lane = getStopLane(stopEdge, action)->getID();
    //     if (stopPlace != nullptr && &stopPlace->getLane().getEdge() == stopEdge) {
    //         stop.startPos = stopPlace->getBeginLanePosition();
    //         stop.endPos = stopPlace->getEndLanePosition();
    //         const SumoXMLTag tag = stopPlace->getElement();
    //         if (tag == SUMO_TAG_BUS_STOP || tag == SUMO_TAG_TRAIN_STOP) {
    //             stop.busstop = stopPlace->getID();
    //         } else if (tag == SUMO_TAG_PARKING_AREA) {
    //             stop.parkingarea = stopPlace->getID();
    //         } else if (tag == SUMO_TAG_CONTAINER_STOP) {
    //             stop.containerstop = stopPlace->getID();
    //         }
    //     } else {
    //         stop.startPos = stopPos;
    //         stop.endPos = MAX2(stopPos, MIN2(myHolder.getVehicleType().getLength(), stopEdge->getLength()));
    //         stop.parametersSet |= STOP_START_SET | STOP_END_SET;
    //     }
    //     stop.parking = SUMOVehicleParameter::parseParkingType(myHolder.getStringParam("device.taxi.parking", false, "true"));
    //     stop.actType = action;
    //     stop.index = STOP_INDEX_END;
    //     // In case of prebooking if person is not there/ comes to late for pickup set maximum waiting time:
    //     SUMOTime earliestPickupTime = res->earliestPickupTime;
    //     if (isPickup && earliestPickupTime >= 0) {
    //         stop.waitUntil = earliestPickupTime;
    //         // TODO: replace hard coded extension with parameter
    //         stop.extension = static_cast<SUMOTime>(3 * 60 * 1000);  // 3mins
    //     }
    //     stops.push_back(stop);
    // }
    #[allow(unused_variables)]
    pub fn prepare_stop(&mut self, edges: &mut ConstMSEdgeVector, stops: &mut StopParVector, lastPos: &mut f64, stopEdge: Ptr<MSEdge>, stopPos: f64, stopPlace: Ptr<MSStoppingPlace>, action: &str, res: Ptr<Reservation>, isPickup: bool) {
        todo!("port MSDevice_Taxi::prepareStop")
    }

    /// C++: `MSLane * MSDevice_Taxi::getStopLane(const MSEdge *, const std::string &)` [private]
    /// determine stopping lane for taxi
    // MSLane*
    // MSDevice_Taxi::getStopLane(const MSEdge* edge, const std::string& action) {
    //     const std::vector<MSLane*>* allowedLanes = edge->allowedLanes(myHolder.getVClass());
    //     if (allowedLanes == nullptr) {
    //         throw ProcessError("Taxi vehicle '" + myHolder.getID() + "' cannot stop on edge '" + edge->getID() + "' (" + action + ")");
    //     }
    //     return allowedLanes->front();
    // }
    #[allow(unused_variables)]
    pub fn get_stop_lane(&mut self, edge: Ptr<MSEdge>, action: &str) -> Ptr<MSLane> {
        todo!("port MSDevice_Taxi::getStopLane")
    }

    /// C++: `bool MSDevice_Taxi::hasFuturePickup()` [private]
    /// whether the taxi has another pickup scheduled
    // bool
    // MSDevice_Taxi::hasFuturePickup() {
    //     for (const auto& stop : myHolder.getStops()) {
    //         if (stop.reached) {
    //             continue;
    //         }
    //         if (stop.pars.permitted.size() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_future_pickup(&mut self) -> bool {
        todo!("port MSDevice_Taxi::hasFuturePickup")
    }

    /// C++: `void MSDevice_Taxi::checkTaskSwap()` [private]
    /// optionally swap tasks when a taxi becomes idle
    // void
    // MSDevice_Taxi::checkTaskSwap() {
    //     const std::string swapGroup = myHolder.getStringParam("device.taxi.swapGroup", false, "");
    //     if (swapGroup != "") {
    //         SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = myDispatcher->getRouter();
    //         const double stopTime = myHolder.isStopped() ? MAX2(0.0, STEPS2TIME(myHolder.getNextStop().duration)) : 0;
    //         double maxSaving = 0;
    //         MSDevice_Taxi* bestSwap = nullptr;
    //         for (MSDevice_Taxi* taxi : myFleet) {
    //             if (taxi->getHolder().hasDeparted() && taxi->getState() == PICKUP
    //                     && taxi->getHolder().getStringParam("device.taxi.swapGroup", false, "") == swapGroup) {
    //                 SUMOVehicle& veh = taxi->getHolder();
    //                 const MSStop& stop = veh.getNextStop();
    //                 ConstMSEdgeVector toPickup(veh.getCurrentRouteEdge(), stop.edge + 1);
    //                 const double cost = router.recomputeCostsPos(toPickup, &veh, veh.getPositionOnLane(), stop.pars.endPos, SIMSTEP);
    //                 ConstMSEdgeVector toPickup2;
    //                 router.compute(myHolder.getEdge(), myHolder.getPositionOnLane(), *stop.edge, stop.pars.endPos, &myHolder, SIMSTEP, toPickup2, true);
    //                 if (!toPickup2.empty()) {
    //                     const double cost2 = router.recomputeCostsPos(toPickup2, &myHolder, myHolder.getPositionOnLane(), stop.pars.endPos, SIMSTEP);
    //                     const double saving = cost - cost2 - stopTime;
    //                     //std::cout << SIMTIME << " taxi=" << getID() << " other=" << veh.getID() << " cost=" << cost << " cost2=" << cost2 << " stopTime=" << stopTime << " saving=" << saving << " route1=" << toString(toPickup) << " route2=" << toString(toPickup2) << "\n";
    //                     if (saving > maxSaving) {
    //                         maxSaving = saving;
    //                         bestSwap = taxi;
    //                     }
    //                 }
    //             }
    //         }
    //         if (maxSaving > SWAP_THRESHOLD) {
    // #ifdef DEBUG_DISPATCH
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " taxi=" << myHolder.getID() << " swapWith=" << bestSwap->getHolder().getID() << " saving=" << maxSaving << " lastDispatch=";
    //                 for (const Reservation* res : bestSwap->myLastDispatch) {
    //                     std::cout << toString(res->persons) << "; ";
    //                 }
    //                 std::cout << "\n";
    //             }
    // #endif
    //             dispatchShared(bestSwap->myLastDispatch);
    //             bestSwap->myCurrentReservations.clear();
    //             bestSwap->myCustomers.clear();
    //             bestSwap->myState = EMPTY;
    //             while (bestSwap->getHolder().hasStops()) {
    //                 bestSwap->getHolder().abortNextStop();
    //             }
    //             for (const Reservation* res : myCurrentReservations) {
    //                 myDispatcher->swappedRunning(res, this);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_task_swap(&mut self) {
        todo!("port MSDevice_Taxi::checkTaskSwap")
    }

    /// C++: `void MSDevice_Taxi::MSDevice_Taxi(const MSDevice_Taxi &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Taxi) -> Self {
        todo!("port MSDevice_Taxi::MSDevice_Taxi")
    }

    /// C++: `MSDevice_Taxi & MSDevice_Taxi::operator=(const MSDevice_Taxi &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Taxi) -> &mut MSDevice_Taxi {
        todo!("port MSDevice_Taxi::operator=")
    }

}

