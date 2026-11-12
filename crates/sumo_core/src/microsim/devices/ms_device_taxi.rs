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
#[repr(C)]
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
thread_local! {
    static MY_DISPATCH_PERIOD: std::cell::RefCell<SUMOTime> = std::cell::RefCell::new(0);
    static MY_DISPATCHER: std::cell::RefCell<Ptr<MSDispatch>> = std::cell::RefCell::new(Ptr::null());
    static MY_DISPATCH_COMMAND: std::cell::RefCell<Ptr<Unmapped>> = std::cell::RefCell::new(Ptr::null());
    static MY_FLEET: std::cell::RefCell<Vec<Ptr<MSDevice_Taxi>>> = std::cell::RefCell::new(Vec::new());
    static MY_MAX_CAPACITY: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
    static MY_MAX_CONTAINER_CAPACITY: std::cell::RefCell<i32> = std::cell::RefCell::new(0);
    static MY_TAXI_TYPES: std::cell::RefCell<BTreeMap<SUMOVehicleClass, String>> = std::cell::RefCell::new(BTreeMap::new());
    static MY_NEXT_DISPATCH_TIME: std::cell::RefCell<SUMOTime> = std::cell::RefCell::new(0);
    static MY_STATE_LOADED_CUSTOMERS: std::cell::RefCell<BTreeMap<String, Ptr<MSDevice_Taxi>>> = std::cell::RefCell::new(BTreeMap::new());
    static MY_STATE_LOADED_RESERVATIONS: std::cell::RefCell<BTreeMap<String, Ptr<MSDevice_Taxi>>> = std::cell::RefCell::new(BTreeMap::new());
}

/// C++ `#define TAXI_SERVICE "taxi"`
pub const TAXI_SERVICE: &str = "taxi";
/// C++ `#define TAXI_SERVICE_PREFIX "taxi:"`
pub const TAXI_SERVICE_PREFIX: &str = "taxi:";

// C++ SUMOVehicleParameter.h stop-parameter flags (`2 << n`, as in C++)
const STOP_DURATION_SET: i32 = 2 << 1;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;
const STOP_PERMITTED_SET: i32 = 2 << 15;

// C++ utils/vehicle/SUMOVehicleParameter.h stop flags and MSDevice_Taxi.cpp constants
const STOP_START_SET: i32 = 1;
const STOP_END_SET: i32 = 2;
const STOP_INDEX_END: i32 = -1;
const NUMERICAL_EPS: f64 = 0.001;

/// C++ `toString(std::set<const MSTransportable *>)`: ids joined by a space (Named::getIDSecure)
fn ids_to_string<'a>(v: impl Iterator<Item = &'a Ptr<MSTransportable>>) -> String {
    v.map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ")
}

/// C++ `joinNamedToStringSorting(set, " ")`: the ids sorted, joined by a space
fn sorted_join(mut ids: Vec<String>) -> String {
    ids.sort();
    ids.join(" ")
}

/// C++ `string2time(s)`, which throws on malformed input
fn string2time_or_die(s: &str) -> SUMOTime {
    utils::common::time::string2time(s).unwrap_or_else(|e| panic!("{}", e))
}

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
        oc.add_option_sub_topic("Taxi Device");
        MSDevice::insert_default_assignment_options("taxi", "Taxi Device", oc, false);

        oc.do_register("device.taxi.dispatch-algorithm", SumoOption::string_with("greedy"));
        oc.add_description("device.taxi.dispatch-algorithm", "Taxi Device", "The dispatch algorithm [greedy|greedyClosest|greedyShared|routeExtension|traci]");

        oc.do_register("device.taxi.dispatch-algorithm.output", SumoOption::file_name());
        oc.add_description("device.taxi.dispatch-algorithm.output", "Taxi Device", "Write information from the dispatch algorithm to FILE");

        oc.do_register("device.taxi.dispatch-algorithm.params", SumoOption::string_with(""));
        oc.add_description("device.taxi.dispatch-algorithm.params", "Taxi Device", "Load dispatch algorithm parameters in format KEY1:VALUE1[,KEY2:VALUE]");

        oc.do_register("device.taxi.dispatch-period", SumoOption::string_typed("60", "TIME"));
        oc.add_description("device.taxi.dispatch-period", "Taxi Device", "The period between successive calls to the dispatcher");

        oc.do_register("device.taxi.dispatch-keep-unreachable", SumoOption::string_typed("3600", "TIME"));
        oc.add_description("device.taxi.dispatch-keep-unreachable", "Taxi Device", "The time before aborting unreachable reservations");

        oc.do_register("device.taxi.idle-algorithm", SumoOption::string_with("stop"));
        oc.add_description("device.taxi.idle-algorithm", "Taxi Device", "The behavior of idle taxis [stop|randomCircling|taxistand]");

        oc.do_register("device.taxi.idle-algorithm.output", SumoOption::file_name());
        oc.add_description("device.taxi.idle-algorithm.output", "Taxi Device", "Write information from the idling algorithm to FILE");

        oc.do_register("device.taxi.vclasses", SumoOption::string_vector_with(vec!["taxi".to_string()]));
        oc.add_synonyme("device.taxi.vclasses", "taxi.vclasses", false);
        oc.add_description("device.taxi.vclasses", "Taxi Device", "Network permissions that can be accessed by taxis");
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
        if with_options(|oc| MSDevice::equipped_by_default_assignment_options(oc, "taxi", v, false, false)) {
            // build the device
            let device: Ptr<MSDevice_Taxi> = Ptr::from_mut(Box::leak(Box::new(MSDevice_Taxi::new(v, &format!("taxi_{}", v.get_id())))));
            into.push(device.cast::<MSVehicleDevice>());
            MY_FLEET.with(|f| f.borrow_mut().push(device));
            if v.get_parameter().line.is_empty() {
                // automatically set the line so that persons are willing to enter
                // (see MSStageDriving::isWaitingFor)
                // const_cast on the vehicle parameter, as in C++
                let p = v.get_parameter() as *const SUMOVehicleParameter as *mut SUMOVehicleParameter;
                unsafe { (*p).line = TAXI_SERVICE.to_string(); }
            }
            let personCapacity = v.get_vehicle_type().get_person_capacity();
            let containerCapacity = v.get_vehicle_type().get_container_capacity();
            MY_MAX_CAPACITY.with(|c| *c.borrow_mut() = std::cmp::max(*c.borrow(), personCapacity));
            MY_MAX_CONTAINER_CAPACITY.with(|c| *c.borrow_mut() = std::cmp::max(*c.borrow(), containerCapacity));
            let vclass = v.get_v_class();
            let needType = MY_TAXI_TYPES.with(|t| t.borrow().get(&vclass).map_or(true, |s| s.is_empty()));
            if needType {
                let tid = v.get_vehicle_type().get_id().to_string();
                MY_TAXI_TYPES.with(|t| t.borrow_mut().insert(vclass, tid));
            }
            if (g_taxi_classes() as SVCPermissions & vclass) == 0 {
                set_g_taxi_classes((g_taxi_classes() as SVCPermissions | vclass) as i32);
                MSNet::get_instance().reset_intermodal_router();
            }
            if personCapacity < 1 && containerCapacity < 1 {
                write_warningf!("Vehicle '%' with personCapacity % and containerCapacity % is not usable as taxi.", v.get_id(), personCapacity, containerCapacity);
            }
        }
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
        lines.len() == 1 && {
            let first = lines.iter().next().unwrap().as_str();
            first == TAXI_SERVICE || first.starts_with(TAXI_SERVICE_PREFIX)
        }
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
        if !Self::is_reservation(lines) {
            return;
        }
        let kind = if person.is_person() { "person" } else { "container" };
        let pid = person.get_id().to_string();
        if (to.get_permissions() & g_taxi_classes() as SVCPermissions) == 0 {
            panic!("{}", ProcessError::new(format!("Cannot add taxi reservation for {} '{}' because destination edge '{}' does not permit taxi access", kind, pid, to.get_id())));
        }
        if (from.get_permissions() & g_taxi_classes() as SVCPermissions) == 0 {
            panic!("{}", ProcessError::new(format!("Cannot add taxi reservation for {} '{}' because origin edge '{}' does not permit taxi access", kind, pid, from.get_id())));
        }
        if MY_DISPATCH_COMMAND.with(|c| c.borrow().is_null()) {
            Self::init_dispatch(0);
        }
        // pickup position should be at the stop-endPos
        let fromPos = if !fromStop.is_null() && Ptr::new(fromStop.get_lane().get_edge()) == from {
            fromStop.get_end_lane_position()
        } else {
            fromPos
        };
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        let res = dispatcher.m().add_reservation(person, reservationTime, pickupTime, earliestPickupTime, from, fromPos, fromStop, to, toPos, toStop, group.to_string(), lines.iter().next().unwrap().as_str(), MY_MAX_CAPACITY.with(|c| *c.borrow()), MY_MAX_CONTAINER_CAPACITY.with(|c| *c.borrow()));
        if MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow().len() > 0) {
            let taxi = MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow().get(&pid).copied());
            if let Some(taxi) = taxi {
                //std::cout << SIMTIME << " loadedServed p=" << person->getID() << " res=" << res->getID() << " taxi=" << it->second->getID() << "\n";
                dispatcher.m().served_reservation(res, taxi);
            }
        }
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
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        if !dispatcher.is_null() && lines.len() == 1 && lines.iter().next().unwrap().as_str() == TAXI_SERVICE {
            dispatcher.m().remove_reservation(person, from, fromPos, to, toPos, group.to_string());
        }
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
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        if !dispatcher.is_null() && lines.len() == 1 && lines.iter().next().unwrap().as_str() == TAXI_SERVICE {
            dispatcher.m().update_reservation_from_pos(person, from, fromPos, to, toPos, group.to_string(), newFromPos);
        }
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
        let mut active: Vec<Ptr<MSDevice_Taxi>> = Vec::new();
        for taxi in MY_FLEET.with(|f| f.borrow().clone()) {
            if taxi.get_holder().has_departed() {
                active.push(taxi);
            }
        }
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        dispatcher.m().compute_dispatch(currentTime, &active);
        let period = MY_DISPATCH_PERIOD.with(|p| *p.borrow());
        MY_NEXT_DISPATCH_TIME.with(|t| *t.borrow_mut() = currentTime + period);
        period
    }

    /// C++: `bool MSDevice_Taxi::hasServableReservations()` [static]
    /// check whether there are still (servable) reservations in the system
    // bool
    // MSDevice_Taxi::hasServableReservations() {
    //     return myDispatcher != nullptr && myDispatcher->hasServableReservations();
    // }
    #[allow(unused_variables)]
    pub fn has_servable_reservations() -> bool {
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        !dispatcher.is_null() && dispatcher.m().has_servable_reservations()
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
        let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
        if !dispatcher.is_null() {
            // delete myDispatcher
            drop(unsafe { Box::from_raw(dispatcher.as_raw()) });
            MY_DISPATCHER.with(|d| *d.borrow_mut() = Ptr::null());
        }
        MY_DISPATCH_COMMAND.with(|c| *c.borrow_mut() = Ptr::null());
        MY_TAXI_TYPES.with(|t| t.borrow_mut().clear());
    }

    /// C++: `MSDispatch * MSDevice_Taxi::getDispatchAlgorithm()` [static]
    // static MSDispatch* getDispatchAlgorithm() {
    //         return myDispatcher;
    //     }
    #[allow(unused_variables)]
    pub fn get_dispatch_algorithm() -> Ptr<MSDispatch> {
        MY_DISPATCHER.with(|d| *d.borrow())
    }

    /// C++: `const std::vector<MSDevice_Taxi *> & MSDevice_Taxi::getFleet()` [static]
    // static const std::vector<MSDevice_Taxi*>& getFleet() {
    //         return myFleet;
    //     }
    #[allow(unused_variables)]
    pub fn get_fleet() -> &'static Vec<Ptr<MSDevice_Taxi>> {
        // SAFETY: thread-local storage lives for the thread; C++ returns a reference to the global vector
        MY_FLEET.with(|f| unsafe { &*f.as_ptr() })
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
        let me = Ptr::from_mut(self);
        MY_FLEET.with(|f| {
            let mut fleet = f.borrow_mut();
            if let Some(pos) = fleet.iter().position(|t| *t == me) {
                fleet.remove(pos);
            }
            // recompute myMaxCapacity
            let mut maxCap = 0;
            let mut maxContainerCap = 0;
            for taxi in fleet.iter() {
                maxCap = std::cmp::max(maxCap, taxi.get_holder().get_vehicle_type().get_person_capacity());
                maxContainerCap = std::cmp::max(maxContainerCap, taxi.get_holder().get_vehicle_type().get_container_capacity());
            }
            MY_MAX_CAPACITY.with(|c| *c.borrow_mut() = maxCap);
            MY_MAX_CONTAINER_CAPACITY.with(|c| *c.borrow_mut() = maxContainerCap);
        });
        // delete myIdleAlgorithm
        drop(unsafe { Box::from_raw(self.myIdleAlgorithm.as_raw()) });
        // base class destructor runs after the derived one in C++
        self.base.drop_();
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
        self.update_move(delta_t(), newPos - oldPos);
        true // keep the device
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
        if self.is_empty() && MSNet::get_instance().get_current_time_step() < self.myServiceEnd {
            { let idle = self.myIdleAlgorithm; idle.m().idle(Ptr::from_mut(self)); }
        }
        true // keep the device
    }

    /// C++: `const std::string MSDevice_Taxi::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "taxi";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "taxi".to_string()
    }

    /// C++: `bool MSDevice_Taxi::isEmpty()`
    /// whether the taxi is empty
    // bool
    // MSDevice_Taxi::isEmpty() {
    //     return myState == EMPTY;
    // }
    #[allow(unused_variables)]
    pub fn is_empty(&mut self) -> bool {
        self.myState == MSDevice_Taxi_TaxiState::EMPTY as i32
    }

    /// C++: `int MSDevice_Taxi::getState() const` [const]
    // int getState() const {
    //         return myState;
    //     }
    #[allow(unused_variables)]
    pub fn get_state(&self) -> i32 {
        self.myState
    }

    /// C++: `bool MSDevice_Taxi::hasFleet()` [static]
    /// returns whether taxis have been loaded
    // bool
    // MSDevice_Taxi::hasFleet() {
    //     return myFleet.size() > 0;;
    // }
    #[allow(unused_variables)]
    pub fn has_fleet() -> bool {
        MY_FLEET.with(|f| f.borrow().len() > 0)
    }

    /// C++: `void MSDevice_Taxi::dispatch(const Reservation &)`
    /// service the given reservation
    // void
    // MSDevice_Taxi::dispatch(const Reservation& res) {
    //     dispatchShared({&res, &res});
    // }
    #[allow(unused_variables)]
    pub fn dispatch(&mut self, res: &Reservation) {
        self.dispatch_shared(vec![Ptr::new(res), Ptr::new(res)]);
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
        // #ifdef DEBUG_DISPATCH output omitted (debug only)
        let mut reservations = reservations;
        self.myLastDispatch = reservations.clone();
        let mut tmpEdges: ConstMSEdgeVector = Vec::new();
        let mut stops: StopParVector = Vec::new();
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        let mut lastPos: f64 = holder.get_position_on_lane();
        // shortcut: C++ `*getRerouteOrigin()` is an iterator into the route; MSBaseVehicle::getRerouteOrigin is not ported yet and returns its current edge, upgrade when it returns Ptr<MSEdge>
        let rerouteOrigin: Ptr<MSEdge> = holder.get_edge();
        if self.is_empty() {
            // start fresh from the current edge
            if holder.is_stopped_parking() {
                // parking stop must be ended normally
                let stoppedTriggered = holder.is_stopped_triggered();
                {
                    let stop = holder.get_next_stop_mutable();
                    stop.duration = 0;
                    lastPos = stop.pars.endPos;
                    if stoppedTriggered {
                        stop.triggered = false;
                        stop.containerTriggered = false;
                        stop.joinTriggered = false;
                    }
                }
                if stoppedTriggered {
                    holder.unregister_waiting();
                }
                // prevent unauthorized/premature entry
                holder.get_next_stop_mutable().pars.permitted.insert(String::new());
                while holder.get_stops().len() > 1 {
                    holder.abort_next_stop(1);
                }
            } else {
                while holder.has_stops() {
                    // in meso there might be more than 1 stop at this point
                    holder.abort_next_stop(0);
                }
                assert!(!holder.has_stops());
            }
            tmpEdges.push(holder.get_edge());
            if holder.get_edge() != rerouteOrigin {
                tmpEdges.push(rerouteOrigin);
            }
        } else {
            assert!(holder.has_stops());
            // check how often existing customers appear in the new reservations
            let mut nOccur: BTreeMap<Ptr<MSTransportable>, i32> = BTreeMap::new();
            for &res in reservations.iter() {
                for &person in res.persons.iter() {
                    if self.myCustomers.contains(&person) {
                        *nOccur.entry(person).or_insert(0) += 1;
                        if !self.myCurrentReservations.contains(&res) {
                            panic!("{}", ProcessError::new(format!("Invalid Re-dispatch for existing customer '{}' with a new reservation", person.get_id())));
                        }
                    }
                }
            }
            if nOccur.is_empty() {
                // no overlap with existing customers - extend route
                tmpEdges = holder.get_route().get_edges().clone();
                lastPos = holder.get_stops().back().unwrap().pars.endPos;
            } else if nOccur.len() == self.myCustomers.len() {
                // redefine route (verify correct number of mentions)
                let mut onBoard: BTreeSet<Ptr<MSTransportable>> = BTreeSet::new();
                onBoard.extend(holder.get_persons().iter().copied());
                onBoard.extend(holder.get_containers().iter().copied());
                let mut redundantPickup: BTreeSet<Ptr<MSTransportable>> = BTreeSet::new();
                for (&person, &count) in nOccur.iter() {
                    if count == 1 {
                        // customers must already be on board
                        if !onBoard.contains(&person) {
                            panic!("{}", ProcessError::new(format!("Re-dispatch did not mention pickup for existing customer '{}'", person.get_id())));
                        }
                    } else if count == 2 {
                        if !onBoard.contains(&person) {
                            // treat like a new customer
                            // TODO: need to be checked
                            self.myCustomers.remove(&person);
                        } else {
                            redundantPickup.insert(person);
                        }
                    } else {
                        panic!("{}", ProcessError::new(format!("Re-dispatch mentions existing customer '{}' {} times", person.get_id(), count)));
                    }
                }
                // remove redundancy
                if !redundantPickup.is_empty() {
                    let mut i = 0;
                    while i < reservations.len() {
                        let isRedundant = reservations[i].persons.iter().any(|p| redundantPickup.contains(p));
                        if isRedundant {
                            for p in reservations[i].persons.iter() {
                                redundantPickup.remove(p);
                            }
                            reservations.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }
                while holder.has_stops() {
                    holder.abort_next_stop(0);
                }
                tmpEdges.push(holder.get_edge());
                if holder.get_edge() != rerouteOrigin {
                    tmpEdges.push(rerouteOrigin);
                }
            } else {
                // inconsistent re-dispatch
                let mut missing: Vec<String> = Vec::new();
                for c in self.myCustomers.iter() {
                    if !nOccur.contains_key(c) {
                        missing.push(c.get_id().to_string());
                    }
                }
                panic!("{}", ProcessError::new(format!("Re-dispatch did mention some customers but failed to mention {}", join_to_string_sorting(&missing, " "))));
            }
        }

        let mut hasPickup = false;
        for &res in reservations.iter() {
            self.myCurrentReservations.insert(res);
            let mut isPickup = false;
            for &person in res.persons.iter() {
                if !self.myCustomers.contains(&person) {
                    self.myCustomers.insert(person);
                    isPickup = true;
                    hasPickup = true;
                }
            }
            if isPickup {
                let action = format!("pickup {} ({})", ids_to_string(res.persons.iter()), res.id);
                self.prepare_stop(&mut tmpEdges, &mut stops, &mut lastPos, res.from, res.fromPos, res.fromStop, &action, res, isPickup);
                for &transportable in res.persons.iter() {
                    if transportable.is_person() {
                        stops.last_mut().unwrap().triggered = true;
                    } else {
                        stops.last_mut().unwrap().containerTriggered = true;
                    }
                    stops.last_mut().unwrap().permitted.insert(transportable.get_id().to_string());
                    stops.last_mut().unwrap().parametersSet |= STOP_PERMITTED_SET | STOP_TRIGGER_SET;
                }
                for &t in res.persons.iter() {
                    stops.last_mut().unwrap().awaitedPersons.insert(t.get_id().to_string());
                }

                if stops.last().unwrap().duration == -1 {
                    // keep dropOffDuration if the stop is dropOff and pickUp
                    let d = holder.get_float_param("device.taxi.pickUpDuration", false, 0.0, false);
                    stops.last_mut().unwrap().duration = time2steps(d);
                }
            } else {
                let action = format!("dropOff {} ({})", ids_to_string(res.persons.iter()), res.id);
                self.prepare_stop(&mut tmpEdges, &mut stops, &mut lastPos, res.to, res.toPos, res.toStop, &action, res, isPickup);
                // pay and collect bags
                let d = holder.get_float_param("device.taxi.dropOffDuration", false, 60.0, false);
                stops.last_mut().unwrap().duration = time2steps(d);
            }
            stops.last_mut().unwrap().parametersSet |= STOP_DURATION_SET | STOP_PARKING_SET;
        }
        // #ifdef DEBUG_DISPATCH output omitted (debug only)

        let t: SUMOTime = MSNet::get_instance().get_current_time_step();
        if !holder.replace_route_edges(&mut tmpEdges, -1.0, 0.0, "taxi:prepare_dispatch", false, false, false, Ptr::null()) {
            panic!("{}", ProcessError::new(format!("Route replacement for taxi dispatch failed for vehicle '{}' at time={}.", holder.get_id(), time2string(t))));
        }
        for stop in stops.iter() {
            let mut error = String::new();
            if !holder.add_stop(stop, &mut error, 0, Ptr::null()) {
                write_warningf!("Could not add taxi stop for %, desc=% time=% error=%", holder.get_id(), stop.actType, time2string(t), error);
            }
        }
        let router = MSRoutingEngine::get_router_tt(holder.get_rng_index(), holder.get_v_class(), &Default::default());
        holder.reroute(t, "taxi:dispatch", router, false, false, false, Ptr::null());
        if hasPickup {
            self.myState |= MSDevice_Taxi_TaxiState::PICKUP as i32;
        }
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        // check if taxi has stopped
        if holder.get_next_stop_parameter().is_null() {
            return;
        }
        // find customers of the current stop
        let mut customersToBeRemoved: BTreeSet<Ptr<MSTransportable>> = BTreeSet::new();
        let mut onBoard: BTreeSet<Ptr<MSTransportable>> = BTreeSet::new();
        onBoard.extend(holder.get_persons().iter().copied());
        onBoard.extend(holder.get_containers().iter().copied());
        let permitted: Vec<String> = holder.get_next_stop_parameter().permitted.iter().cloned().collect();
        for tID in permitted.iter() {
            for &t in self.myCustomers.iter() {
                if t.get_id().to_string() == *tID && !onBoard.contains(&t) {
                    customersToBeRemoved.insert(t);
                }
            }
        }
        if !customersToBeRemoved.is_empty() {
            write_warningf!("Taxi '%' aborts waiting for customers: % at time=%.",
                holder.get_id(), ids_to_string(customersToBeRemoved.iter()), time2string(MSNet::get_instance().get_current_time_step()));
        }
        for t in customersToBeRemoved {
            self.cancel_customer(t);
        }
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
        if !self.myCustomers.contains(&t) {
            return false;
        }
        self.myCustomers.remove(&t);
        // check whether a single reservation has been fulfilled or another customer is part of the reservation
        let current: Vec<Ptr<Reservation>> = self.myCurrentReservations.iter().copied().collect();
        for res in current {
            let fulfilled = res.persons.len() == 1 && res.persons.contains(&t);
            if fulfilled {
                // remove reservation from the current dispatch
                self.myLastDispatch.retain(|x| *x != res);
                // remove reservation from the served reservations
                self.myCurrentReservations.remove(&res);
                // delete the reservation
                MY_DISPATCHER.with(|d| *d.borrow()).m().fulfilled_reservation(res);
            }
        }
        self.myState &= !(MSDevice_Taxi_TaxiState::PICKUP as i32); // remove state PICKUP
        for &res in self.myCurrentReservations.iter() {
            // if there is another pickup in the dispatch left, add the state PICKUP
            if self.myLastDispatch.iter().filter(|x| **x == res).count() == 2 {
                self.myState |= MSDevice_Taxi_TaxiState::PICKUP as i32; // add state PICKUP
            }
        }
        // we also have to clean reservations from myLastDispatch where the customers arrived in the meantime
        let current_reservations = &self.myCurrentReservations;
        self.myLastDispatch.retain(|x| current_reservations.contains(x));
        // if there are reservations left, go on with the dispatch
        // in meso, wait for the next dispatch cycle to avoid updating stops in this stage
        if !MSGlobals::g_use_meso_sim() {
            let last = self.myLastDispatch.clone();
            self.dispatch_shared(last);
        }
        true
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
        self.myCustomers.insert(t);
        let holderPtr = self.myHolder;
        let veh: &mut MSBaseVehicle = holderPtr.cast::<MSBaseVehicle>().m();
        let tid = t.get_id().to_string();
        for stop in veh.get_stops().iter() {
            // const_cast on the stop parameters, as in C++
            let pars: &mut SUMOVehicleParameter_Stop = unsafe { &mut *(&stop.pars as *const SUMOVehicleParameter_Stop as *mut SUMOVehicleParameter_Stop) };
            if *stop.edge == res.from && pars.startPos <= res.fromPos && pars.endPos >= res.fromPos {
                pars.awaitedPersons.insert(tid.clone());
                pars.permitted.insert(tid.clone());
                pars.actType.push_str(&format!(" +{}", tid));
            } else if *stop.edge == res.to && pars.startPos <= res.toPos && pars.endPos >= res.toPos {
                pars.actType.push_str(&format!(" +{}", tid));
                break;
            }
        }
    }

    /// C++: `bool MSDevice_Taxi::allowsBoarding(const MSTransportable *) const` [const]
    /// whether the given person is allowed to board this taxi
    // bool
    // MSDevice_Taxi::allowsBoarding(const MSTransportable* t) const {
    //     return myCustomers.count(t) != 0;
    // }
    #[allow(unused_variables)]
    pub fn allows_boarding(&self, t: Ptr<MSTransportable>) -> bool {
        self.myCustomers.contains(&t)
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
        self.myState |= MSDevice_Taxi_TaxiState::OCCUPIED as i32;
        if !self.has_future_pickup() {
            self.myState &= !(MSDevice_Taxi_TaxiState::PICKUP as i32);
        }
        for res in self.myCurrentReservations.iter() {
            for cand in res.persons.iter() {
                if *cand == t {
                    res.m().state = Reservation_ReservationState::ONBOARD;
                    break;
                }
            }
        }
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
        self.myCustomersServed += 1;
        self.myCustomers.remove(&person);
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        if holder.get_person_number() == 0 && holder.get_container_number() == 0 {
            self.myState &= !(MSDevice_Taxi_TaxiState::OCCUPIED as i32);
            if holder.get_stops().len() > 1 && (self.myState & (MSDevice_Taxi_TaxiState::PICKUP as i32)) == 0 {
                write_warningf!("All customers left vehicle '%' at time=% but there are % remaining stops",
                    holder.get_id(), time2string(MSNet::get_instance().get_current_time_step()), holder.get_stops().len() - 1);
                while holder.get_stops().len() > 1 {
                    holder.abort_next_stop(1);
                }
            }
        }
        if self.is_empty() {
            // cleanup
            let current: Vec<Ptr<Reservation>> = self.myCurrentReservations.iter().copied().collect();
            for res in current {
                MY_DISPATCHER.with(|d| *d.borrow()).m().fulfilled_reservation(res);
            }
            self.myCurrentReservations.clear();
            self.check_task_swap();
            if self.is_empty() {
                if MSGlobals::g_use_meso_sim() && MSNet::get_instance().get_current_time_step() < self.myServiceEnd {
                    { let idle = self.myIdleAlgorithm; idle.m().idle(Ptr::from_mut(self)); }
                }
            }
        } else {
            // check whether a single reservation has been fulfilled
            let current: Vec<Ptr<Reservation>> = self.myCurrentReservations.iter().copied().collect();
            for res in current {
                let fulfilled = res.persons.iter().all(|t| !self.myCustomers.contains(t));
                if fulfilled {
                    MY_DISPATCHER.with(|d| *d.borrow()).m().fulfilled_reservation(res);
                    self.myCurrentReservations.remove(&res);
                }
            }
        }
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
        out.open_tag(SUMO_TAG_DEVICE);
        out.write_attr(SUMO_ATTR_ID, self.get_id());
        let internals = format!("{} {} {} {}",
            self.myState,
            self.myCustomersServed,
            f64_to_string_prec(self.myOccupiedDistance, 6),
            f64_to_string_prec(self.myOccupiedTime as f64, 6));
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        if self.myCustomers.len() > 0 {
            out.write_attr(SUMO_ATTR_CUSTOMERS, sorted_join(self.myCustomers.iter().map(|c| c.get_id().to_string()).collect()).as_str());
        }
        if self.myCurrentReservations.len() > 0 {
            out.write_attr(SUMO_ATTR_RESERVATIONS, sorted_join(self.myCurrentReservations.iter().map(|r| r.id.clone()).collect()).as_str());
        }
        out.close_tag("");
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
        let mut ok = true;
        let id = self.get_id().to_string();
        let state = attrs.get_string(SUMO_ATTR_STATE).to_string();
        let mut bis = state.split_whitespace();
        self.myState = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        self.myCustomersServed = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        self.myOccupiedDistance = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        self.myOccupiedTime = bis.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        if attrs.has_attribute(SUMO_ATTR_CUSTOMERS) {
            let customers: Vec<String> = attrs.get(SUMO_ATTR_CUSTOMERS, &id, &mut ok, true);
            for cid in customers {
                MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow_mut().insert(cid, Ptr::from_mut(self)));
            }
        }
        if attrs.has_attribute(SUMO_ATTR_RESERVATIONS) {
            let reservations: Vec<String> = attrs.get(SUMO_ATTR_RESERVATIONS, &id, &mut ok, true);
            for rid in reservations {
                MY_STATE_LOADED_RESERVATIONS.with(|m| m.borrow_mut().insert(rid, Ptr::from_mut(self)));
            }
        }
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
        if MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow().len() > 0) {
            let net = MSNet::get_instance();
            let pc = if net.has_persons() { Some(net.get_person_control()) } else { None };
            let cc = if net.has_containers() { Some(net.get_container_control()) } else { None };
            let items: Vec<(String, Ptr<MSDevice_Taxi>)> = MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow().iter().map(|(k, v)| (k.clone(), *v)).collect());
            for (id, taxi) in items {
                let mut t: Ptr<MSTransportable> = Ptr::null();
                if let Some(pc) = pc {
                    t = pc.get(&id);
                }
                if t.is_null() {
                    if let Some(cc) = cc {
                        t = cc.get(&id);
                    }
                }
                if t.is_null() {
                    write_errorf!("Could not find taxi customer '%'. Ensure state contains transportables", id);
                } else {
                    taxi.m().myCustomers.insert(t);
                }
            }
        }
        if MY_STATE_LOADED_RESERVATIONS.with(|m| m.borrow().len() > 0) {
            let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
            assert!(!dispatcher.is_null());
            let mut resLookup: BTreeMap<String, Ptr<Reservation>> = BTreeMap::new();
            for res in dispatcher.m().get_reservations() {
                resLookup.insert(res.id.clone(), res);
            }
            for res in dispatcher.m().get_running_reservations() {
                resLookup.insert(res.id.clone(), res);
            }
            let items: Vec<(String, Ptr<MSDevice_Taxi>)> = MY_STATE_LOADED_RESERVATIONS.with(|m| m.borrow().iter().map(|(k, v)| (k.clone(), *v)).collect());
            for (id, taxi) in items {
                match resLookup.get(&id) {
                    None => {
                        write_errorf!("Could not find taxi reservation '%'.", id);
                    }
                    Some(res) => {
                        taxi.m().myCurrentReservations.insert(*res);
                    }
                }
            }
        }
        MY_STATE_LOADED_CUSTOMERS.with(|m| m.borrow_mut().clear());
        MY_STATE_LOADED_RESERVATIONS.with(|m| m.borrow_mut().clear());
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
        if key == "customers" {
            self.myCustomersServed.to_string()
        } else if key == "occupiedDistance" {
            utils::common::to_string::to_string(&self.myOccupiedDistance)
        } else if key == "occupiedTime" {
            utils::common::to_string::to_string(&steps2time(self.myOccupiedTime))
        } else if key == "state" {
            self.myState.to_string()
        } else if key == "currentCustomers" {
            sorted_join(self.myCustomers.iter().map(|c| c.get_id().to_string()).collect())
        } else if key == "pickUpDuration" {
            self.myHolder.get_string_param("device.taxi.pickUpDuration", false, "0")
        } else if key == "dropOffDuration" {
            self.myHolder.get_string_param("device.taxi.dropOffDuration", false, "60")
        } else {
            panic!("{}", ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
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
        if value.parse::<f64>().is_err() {
            panic!("{}", ProcessError::InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name())));
        }
        if key == "pickUpDuration" || key == "dropOffDuration" {
            // store as generic vehicle parameters
            let holderPtr = self.myHolder;
            let par: &mut SUMOVehicleParameter = unsafe { &mut *(holderPtr.m().get_parameter() as *const SUMOVehicleParameter as *mut SUMOVehicleParameter) };
            par.set_parameter(&format!("device.taxi.{}", key), value);
        } else {
            panic!("{}", ProcessError::InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())));
        }
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
        if !tripinfoOut.is_null() {
            let out = tripinfoOut.m();
            out.open_tag("taxi");
            out.write_attr("customers", self.myCustomersServed.to_string().as_str());
            out.write_attr("occupiedDistance", utils::common::to_string::to_string(&self.myOccupiedDistance).as_str());
            out.write_attr("occupiedTime", time2string(self.myOccupiedTime).as_str());
            out.close_tag("");
        }
    }

    /// C++: `bool MSDevice_Taxi::compatibleLine(const Reservation *)`
    /// whether the given reservation is compatible with the taxi line
    // bool
    // MSDevice_Taxi::compatibleLine(const Reservation* res) {
    //     return compatibleLine(myHolder.getParameter().line, res->line);
    // }
    #[allow(unused_variables)]
    pub fn compatible_line(&mut self, res: Ptr<Reservation>) -> bool {
        let taxiLine = self.myHolder.get_parameter().line.clone();
        Self::compatible_line_2(&taxiLine, &res.line)
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
        (taxiLine == rideLine && rideLine.starts_with("taxi") && taxiLine.starts_with("taxi"))
            || (taxiLine == TAXI_SERVICE && rideLine.starts_with("taxi:"))
            || (rideLine == TAXI_SERVICE && taxiLine.starts_with("taxi:"))
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
        for taxi in MY_FLEET.with(|f| f.borrow().clone()) {
            // disable taskSwap
            taxi.m().myState = MSDevice_Taxi_TaxiState::EMPTY as i32;
        }
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
        if MY_TAXI_TYPES.with(|t| t.borrow().len()) < (g_taxi_classes() as i64 as u64).count_ones() as usize {
            for vClassName in utils::options::options_cont::get_string_vector("device.taxi.vclasses") {
                let svc: SUMOVehicleClass = utils::common::sumo_vehicle_class::parse_vehicle_classes(&vClassName);
                let unset = MY_TAXI_TYPES.with(|t| t.borrow().get(&svc).map_or(true, |s| s.is_empty()));
                if unset {
                    let typeID: String = if svc == SVC_TAXI {
                        DEFAULT_TAXITYPE_ID.to_string()
                    } else if svc == SVC_RAIL {
                        DEFAULT_RAILTYPE_ID.to_string()
                    } else if svc == SVC_BICYCLE {
                        DEFAULT_BIKETYPE_ID.to_string()
                    } else if svc == SVC_PASSENGER {
                        DEFAULT_VTYPE_ID.to_string()
                    } else {
                        let typeID = format!("DEFAULT_{}TYPE", vClassName.to_uppercase());
                        let netPtr = MSNet::get_instance();
    let vc = netPtr.m().get_vehicle_control();
                        if !vc.has_v_type(&typeID) {
                            let mut tp = SUMOVTypeParameter::new(&typeID, svc);
                            let t = MSVehicleType::build(&mut tp, "");
                            vc.add_v_type(t);
                        }
                        typeID
                    };
                    MY_TAXI_TYPES.with(|t| t.borrow_mut().insert(svc, typeID));
                }
            }
        }
        // SAFETY: thread-local storage lives for the thread; C++ returns a reference to the global map
        unsafe { &*MY_TAXI_TYPES.with(|t| t.as_ptr()) }
    }

    /// C++: `SUMOTime MSDevice_Taxi::getNextDispatchTime()` [static]
    // SUMOTime
    // MSDevice_Taxi::getNextDispatchTime() {
    //     return myNextDispatchTime;
    // }
    #[allow(unused_variables)]
    pub fn get_next_dispatch_time() -> SUMOTime {
        MY_NEXT_DISPATCH_TIME.with(|t| *t.borrow())
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
        let mut next = next;
        MY_DISPATCH_PERIOD.with(|p| *p.borrow_mut() = string2time_or_die(&utils::options::options_cont::get_string("device.taxi.dispatch-period")));
        // init dispatch algorithm
        let algo = utils::options::options_cont::get_string("device.taxi.dispatch-algorithm");
        let mut params = Parameterised::new();
        params.set_parameters_str(&utils::options::options_cont::get_string("device.taxi.dispatch-algorithm.params"), ":", ",");
        let pm = params.get_parameters_map();
        let dispatcher: Ptr<MSDispatch> = if algo == "greedy" {
            Ptr::from_mut(Box::leak(Box::new(MSDispatch_Greedy::new(pm)))).cast::<MSDispatch>()
        } else if algo == "greedyClosest" {
            Ptr::from_mut(Box::leak(Box::new(MSDispatch_GreedyClosest::new(pm)))).cast::<MSDispatch>()
        } else if algo == "greedyShared" {
            Ptr::from_mut(Box::leak(Box::new(MSDispatch_GreedyShared::new(pm)))).cast::<MSDispatch>()
        } else if algo == "routeExtension" {
            Ptr::from_mut(Box::leak(Box::new(MSDispatch_RouteExtension::new(pm)))).cast::<MSDispatch>()
        } else if algo == "traci" {
            Ptr::from_mut(Box::leak(Box::new(MSDispatch_TraCI::new(pm)))).cast::<MSDispatch>()
        } else {
            panic!("{}", ProcessError::new(format!("Dispatch algorithm '{}' is not known", algo)));
        };
        MY_DISPATCHER.with(|d| *d.borrow_mut() = dispatcher);
        // shortcut: C++ `new StaticCommand<MSDevice_Taxi>(&MSDevice_Taxi::triggerDispatch)` is not ported (no Command type); a non-null marker keeps the "already initialised" check working, upgrade when StaticCommand is ported and scheduled via addEvent
        let command: Ptr<Unmapped> = Ptr::from_mut(Box::leak(Box::new(())));
        MY_DISPATCH_COMMAND.with(|c| *c.borrow_mut() = command);
        // round to next multiple of myDispatchPeriod
        if next < 0 {
            let now = MSNet::get_instance().get_current_time_step();
            let begin = string2time_or_die(&utils::options::options_cont::get_string("begin"));
            let period = MY_DISPATCH_PERIOD.with(|p| *p.borrow());
            let delay = (period - ((now - begin) % period)) % period;
            next = now + delay;
        }
        MY_NEXT_DISPATCH_TIME.with(|t| *t.borrow_mut() = next);
        MSNet::get_instance().get_end_of_timestep_events().add_event(command, next);
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
        self.update_move(time2steps(timeOnLane), travelledDistanceFrontOnLane);
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
        let mut defaultServiceEnd = utils::common::to_string::to_string(&1e15_f64);
        let algo = holder.get_string_param("device.taxi.idle-algorithm", false, "");
        let idle: Ptr<MSIdling>;
        if algo == "stop" {
            idle = Ptr::from_mut(Box::leak(Box::new(MSIdling_Stop::new()))).cast::<MSIdling>();
        } else if algo == "randomCircling" {
            idle = Ptr::from_mut(Box::leak(Box::new(MSIdling_RandomCircling::new()))).cast::<MSIdling>();
            // make sure simulation terminates
            let depart = if holder.get_parameter().departProcedure == DepartDefinition::GIVEN {
                holder.get_parameter().depart
            } else {
                MSNet::get_instance().get_current_time_step()
            };
            defaultServiceEnd = utils::common::to_string::to_string(&(steps2time(depart) + (3600 * 8) as f64));
        } else if algo == "taxistand" {
            let rerouterID = holder.get_string_param("device.taxi.stands-rerouter", false, "");
            if rerouterID.is_empty() {
                panic!("{}", ProcessError::new(format!("Idle algorithm '{}' requires a rerouter id to be defined using device param 'stands-rerouter' for vehicle '{}'", algo, holder.get_id())));
            }
            if !MSTriggeredRerouter::get_instances().contains_key(&rerouterID) {
                panic!("{}", ProcessError::new(format!("Unknown rerouter '{}' when loading taxi stands for vehicle '{}'", rerouterID, holder.get_id())));
            }
            let rerouter = *MSTriggeredRerouter::get_instances().get(&rerouterID).unwrap();
            idle = Ptr::from_mut(Box::leak(Box::new(MSIdling_TaxiStand::new(rerouter)))).cast::<MSIdling>();
        } else {
            panic!("{}", ProcessError::new(format!("Idle algorithm '{}' is not known for vehicle '{}'", algo, holder.get_id())));
        }
        let serviceEnd = string2time_or_die(&holder.get_string_param("device.taxi.end", false, &defaultServiceEnd));
        let routing = holder.get_device(&()).cast::<MSDevice_Routing>();
        MSDevice_Taxi {
            base: MSVehicleDevice::new(holder, id),
            myState: MSDevice_Taxi_TaxiState::EMPTY as i32,
            myCustomersServed: 0,
            myOccupiedDistance: 0.0,
            myOccupiedTime: 0,
            myServiceEnd: serviceEnd,
            myIsStopped: false,
            myCustomers: BTreeSet::new(),
            myIdleAlgorithm: idle,
            myReachedServiceEnd: false,
            myCurrentReservations: BTreeSet::new(),
            myRoutingDevice: routing,
            myLastDispatch: Vec::new(),
        }
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        if holder.get_person_number() > 0 || holder.get_container_number() > 0 {
            self.myOccupiedDistance += travelledDist;
            self.myOccupiedTime += traveltime;
        }
        if self.is_empty() {
            if MSNet::get_instance().get_current_time_step() < self.myServiceEnd {
                { let idle = self.myIdleAlgorithm; idle.m().idle(Ptr::from_mut(self)); }
                if !self.myRoutingDevice.is_null() {
                    // prevent rerouting during idling (#11079)
                    self.myRoutingDevice.m().set_active(false);
                }
            } else if !self.myReachedServiceEnd {
                write_warningf!("Taxi '%' reaches scheduled end of service at time=%.", holder.get_id(), time2string(MSNet::get_instance().get_current_time_step()));
                self.myReachedServiceEnd = true;
            }
        } else if !self.myRoutingDevice.is_null() {
            self.myRoutingDevice.m().set_active(true);
        }
        if holder.is_stopped() && (self.is_empty() || MSGlobals::g_use_meso_sim()) && holder.get_next_stop().endBoarding > self.myServiceEnd {
            // limit duration of stop (but only for idling-related stops)
            holder.get_next_stop_mutable().endBoarding = self.myServiceEnd;
        }
        self.myIsStopped = holder.is_stopped();
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
        assert!(!edges.is_empty());
        let mut stopPos = stopPos;
        let stopPlaceEdge = if stopPlace.is_null() { Ptr::null() } else { Ptr::new(stopPlace.get_lane().get_edge()) };
        if !stopPlace.is_null() && stopPlaceEdge == stopEdge {
            stopPos = stopPlace.get_end_lane_position();
        }
        if stopPos < *lastPos && stopPos + NUMERICAL_EPS >= *lastPos {
            stopPos = *lastPos;
        }
        let mut addedEdge = false;

        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        if stops.is_empty() {
            // check brakeGap
            let mut distToStop = stopPos - *lastPos;
            let brakeGap = holder.get_brake_gap(false);
            let lane = holder.get_lane();
            if !lane.is_null() && lane.is_internal() {
                distToStop += lane.get_length();
            }
            let lastEdge = *edges.last().unwrap();
            if stopEdge != lastEdge {
                distToStop += lastEdge.get_length();
                if distToStop < brakeGap {
                    // the distance between current edge and stop edge may be small
                    let router = MSRoutingEngine::get_router_tt(holder.get_rng_index(), holder.get_v_class(), &Default::default());
                    let mut toFirstStop: ConstMSEdgeVector = Vec::new();
                    router.compute(lastEdge, stopEdge, Ptr::from_mut(holder), MSNet::get_instance().get_current_time_step(), &mut toFirstStop, true);
                    let n = toFirstStop.len() as i64 - 1;
                    let mut i: i64 = 1;
                    while i < n {
                        distToStop += toFirstStop[i as usize].get_length();
                        i += 1;
                    }
                }
            }
            if distToStop < brakeGap {
                // circle back to stopEdge
                edges.push(stopEdge);
                addedEdge = true;
            }
        }

        let lastEdge = *edges.last().unwrap();
        if stopEdge == lastEdge && !stops.is_empty() {
            if stopPos >= *lastPos && stopPos <= stops.last().unwrap().endPos {
                // no new stop and no adaption needed
                stops.last_mut().unwrap().actType += &format!(",{}", action);
                return;
            }
            let vehLength = holder.get_vehicle_type().get_length();
            if stopPos >= *lastPos && stopPos <= *lastPos + vehLength {
                // stop length adaption needed
                let newEnd = *lastPos + vehLength;
                let edgeLength = stopEdge.get_length();
                stops.last_mut().unwrap().endPos = if newEnd < edgeLength { newEnd } else { edgeLength };
                stops.last_mut().unwrap().actType += &format!(",{}", action);
                return;
            }
        }
        if !addedEdge && (stopEdge != lastEdge || stopPos < *lastPos) {
            edges.push(stopEdge);
        }
        *lastPos = stopPos;
        let mut stop = SUMOVehicleParameter_Stop::default();
        stop.lane = self.get_stop_lane(stopEdge, action).get_id().to_string();
        if !stopPlace.is_null() && stopPlaceEdge == stopEdge {
            stop.startPos = stopPlace.get_begin_lane_position();
            stop.endPos = stopPlace.get_end_lane_position();
            let tag = stopPlace.get_element();
            if tag == SUMO_TAG_BUS_STOP || tag == SUMO_TAG_TRAIN_STOP {
                stop.busstop = stopPlace.get_id().to_string();
            } else if tag == SUMO_TAG_PARKING_AREA {
                stop.parkingarea = stopPlace.get_id().to_string();
            } else if tag == SUMO_TAG_CONTAINER_STOP {
                stop.containerstop = stopPlace.get_id().to_string();
            }
        } else {
            stop.startPos = stopPos;
            let vehLength = holder.get_vehicle_type().get_length();
            let edgeLength = stopEdge.get_length();
            let capped = if vehLength < edgeLength { vehLength } else { edgeLength };
            stop.endPos = if stopPos > capped { stopPos } else { capped };
            stop.parametersSet |= STOP_START_SET | STOP_END_SET;
        }
        stop.parking = SUMOVehicleParameter::parse_parking_type(&holder.get_string_param("device.taxi.parking", false, "true"));
        stop.actType = action.to_string();
        stop.index = STOP_INDEX_END;
        // In case of prebooking if person is not there/ comes to late for pickup set maximum waiting time:
        let earliestPickupTime = res.earliestPickupTime;
        if isPickup && earliestPickupTime >= 0 {
            stop.waitUntil = earliestPickupTime;
            // TODO: replace hard coded extension with parameter
            stop.extension = 3 * 60 * 1000; // 3mins
        }
        stops.push(stop);
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        let allowedLanes = edge.allowed_lanes_2(holder.get_v_class());
        if allowedLanes.is_null() {
            panic!("{}", ProcessError::new(format!("Taxi vehicle '{}' cannot stop on edge '{}' ({})", holder.get_id(), edge.get_id(), action)));
        }
        let lanes: &Vec<Ptr<MSLane>> = allowedLanes.get().unwrap();
        lanes[0]
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        for stop in holder.get_stops().iter() {
            if stop.reached {
                continue;
            }
            if stop.pars.permitted.len() > 0 {
                return true;
            }
        }
        false
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
        let holderPtr = self.myHolder;
        let holder = holderPtr.m();
        let swapGroup = holder.get_string_param("device.taxi.swapGroup", false, "");
        if swapGroup != "" {
            let stopTime: f64 = if holder.is_stopped() { f64::max(0.0, steps2time(holder.get_next_stop().duration)) } else { 0.0 };
            let mut maxSaving: f64 = 0.0;
            let mut bestSwap: Ptr<MSDevice_Taxi> = Ptr::null();
            for taxi in MY_FLEET.with(|f| f.borrow().clone()) {
                if taxi.get_holder().has_departed() && taxi.get_state() == MSDevice_Taxi_TaxiState::PICKUP as i32
                    && taxi.get_holder().get_string_param("device.taxi.swapGroup", false, "") == swapGroup {
                    // shortcut: C++ builds toPickup from the route iterator range (getCurrentRouteEdge() .. stop.edge + 1) and costs it with the router; MSRouteIterator is not mapped yet (Unmapped), so the candidate evaluation is skipped and no swap happens, upgrade when MSRouteIterator is ported
                }
            }
            let _ = stopTime;
            if maxSaving > 5.0 {
                self.dispatch_shared(bestSwap.m().myLastDispatch.clone());
                bestSwap.m().myCurrentReservations.clear();
                bestSwap.m().myCustomers.clear();
                bestSwap.m().myState = MSDevice_Taxi_TaxiState::EMPTY as i32;
                while bestSwap.m().get_holder().has_stops() {
                    bestSwap.m().get_holder().abort_next_stop(0);
                }
                let dispatcher = MY_DISPATCHER.with(|d| *d.borrow());
                for res in self.myCurrentReservations.iter() {
                    dispatcher.m().swapped_running(*res, Ptr::from_mut(self));
                }
            }
        }
    }

    /// C++: `void MSDevice_Taxi::MSDevice_Taxi(const MSDevice_Taxi &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Taxi) -> Self {
        panic!("MSDevice_Taxi copy constructor is private and undefined in C++")
    }

    /// C++: `MSDevice_Taxi & MSDevice_Taxi::operator=(const MSDevice_Taxi &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Taxi) -> &mut MSDevice_Taxi {
        panic!("MSDevice_Taxi assignment operator is private and undefined in C++")
    }

}

