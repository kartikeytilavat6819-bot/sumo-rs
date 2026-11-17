//! Port of microsim/transportables/MSTransportableControl.{h,cpp}
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::microsim::ms_globals::stop_tolerance;
use utils::common::msg_handler::write_warning;
use utils::common::time::{delta_t, string2time};
use utils::options::options_cont::{get_bool, get_int, get_string, is_set, with_options};

/// Heap-allocates a transportable or model and hands out a raw pointer, like C++ `new`.
fn leak_ptr<T>(v: T) -> Ptr<T> {
    Ptr::from_raw(Box::into_raw(Box::new(v)))
}

/// C++ `MSTransportableControl::TransportableVector`: `std::vector<MSTransportable*>`
pub type MSTransportableControl_TransportableVector = Vec<Ptr<MSTransportable>>;

/// Key of myWaiting4Vehicle: C++ orders edges by `ComparatorNumericalIdLess`, so the key is
/// (numerical id, edge); ids are unique, so iteration matches the C++ map.
// determinism: ordering by numerical id as in C++ ComparatorNumericalIdLess
fn edge_key(edge: Ptr<MSEdge>) -> (i32, Ptr<MSEdge>) {
    (edge.get_numerical_id(), edge)
}

/// C++ `SIMSTEP`
fn simstep() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}

/// C++ typedef `std::map<std::string, MSTransportable*>::const_iterator MSTransportableControl::constVehIt`
pub type MSTransportableControl_constVehIt<'a> = std::collections::btree_map::Iter<'a, String, Ptr<MSTransportable>>;

/// The class is used to handle transportables (persons and containers) who are not using a transportation system but are walking or waiting. This includes waiting for the arrival or departure time / the time the waiting is over.
/// C++ class `MSTransportableControl` (microsim/transportables/MSTransportableControl.h)
pub struct MSTransportableControl {
    /// all currently created transportables by id
    pub myTransportables: BTreeMap<String, Ptr<MSTransportable>>,
    /// Transportables waiting for departure
    pub myWaiting4Departure: BTreeMap<SUMOTime, MSTransportableControl_TransportableVector>,
    /// the lists of walking / stopping transportables
    pub myWaitingUntil: BTreeMap<SUMOTime, MSTransportableControl_TransportableVector>,
    /// the lists of waiting transportables
    pub myWaiting4Vehicle: BTreeMap<(i32, Ptr<MSEdge>), MSTransportableControl_TransportableVector>,
    /// The number of build transportables
    pub myLoadedNumber: i32,
    /// The number of discarded transportables
    pub myDiscardedNumber: i32,
    /// The number of transportables within the network (build and inserted but not removed)
    pub myRunningNumber: i32,
    /// The number of jammed transportables
    pub myJammedNumber: i32,
    /// The number of transportables waiting for departure
    pub myWaitingForDepartureNumber: i32,
    /// The number of transportables waiting for vehicles
    pub myWaitingForVehicleNumber: i32,
    /// The number of transportables waiting for a specified time
    pub myWaitingUntilNumber: i32,
    /// The number of transportables currently in an access stage
    pub myAccessNumber: i32,
    /// The number of transportables that exited the simulation
    pub myEndedNumber: i32,
    /// The number of transportables that arrived at their destination
    pub myArrivedNumber: i32,
    /// The number of teleports due to long waits for a ride
    pub myTeleportsAbortWait: i32,
    /// The number of teleports due to wrong destination
    pub myTeleportsWrongDest: i32,
    /// whether a new transportable waiting for a vehicle has been added in the last step
    pub myHaveNewWaiting: bool,
    /// maximum transportable count
    pub myMaxTransportableNumber: i32,
    pub myMovementModel: Ptr<MSPModel>,
    pub myNonInteractingModel: Ptr<MSPModel>,
    /// Information needed to sort transportable output by departure time
    pub myRouteInfos: MSDevice_Vehroutes_SortedRouteInfo,
    /// The time until waiting for a ride is aborted
    pub myAbortWaitingTimeout: SUMOTime,
}

impl MSTransportableControl {
    /// C++: `void MSTransportableControl::MSTransportableControl(const bool)`
    /// Constructor
    // MSTransportableControl::MSTransportableControl(const bool isPerson):
    //     myLoadedNumber(0),
    //     myDiscardedNumber(0),
    //     myRunningNumber(0),
    //     myJammedNumber(0),
    //     myWaitingForDepartureNumber(0),
    //     myWaitingForVehicleNumber(0),
    //     myWaitingUntilNumber(0),
    //     myAccessNumber(0),
    //     myEndedNumber(0),
    //     myArrivedNumber(0),
    //     myTeleportsAbortWait(0),
    //     myTeleportsWrongDest(0),
    //     myHaveNewWaiting(false) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     MSNet* const net = MSNet::getInstance();
    //     myMovementModel = myNonInteractingModel = new MSPModel_NonInteracting(oc, net);
    //     if (isPerson) {
    //         const std::string& model = oc.getString("pedestrian.model");
    //         if (model == "striping") {
    //             myMovementModel = new MSPModel_Striping(oc, net);
    // #ifdef HAVE_JUPEDSIM_GRPC
    //         } else if (model == "jupedsim") {
    //             myMovementModel = new MSPModel_JuPedSim(oc, net);
    // #endif
    // #ifdef HAVE_JUPEDSIM_LEGACY
    //         } else if (model == "jupedsim_legacy") {
    //             myMovementModel = new MSPModel_JuPedSim_legacy(oc, net);
    // #endif
    //         } else if (model != "nonInteracting") {
    //             delete myNonInteractingModel;
    //             throw ProcessError(TLF("Unknown pedestrian model '%'", model));
    //         }
    //     }
    //     if (oc.isSet("vehroute-output")) {
    //         myRouteInfos.routeOut = &OutputDevice::getDeviceByOption("vehroute-output");
    //     }
    //     if (oc.isSet("personroute-output")) {
    //         OutputDevice::createDeviceByOption("personroute-output", "routes", "routes_file.xsd");
    //         myRouteInfos.routeOut = &OutputDevice::getDeviceByOption("personroute-output");
    //     }
    //     if (oc.isSet("personinfo-output")) {
    //         OutputDevice::createDeviceByOption("personinfo-output", "tripinfos", "tripinfo_file.xsd");
    //     }
    //     myAbortWaitingTimeout = string2time(oc.getString("time-to-teleport.ride"));
    //     myMaxTransportableNumber = isPerson ? oc.getInt("max-num-persons") : -1;
    // }
    #[allow(unused_variables)]
    pub fn new(isPerson: bool) -> Self {
        let net = MSNet::get_instance();
        let nonInteracting: Ptr<MSPModel> = leak_ptr(with_options(|oc| MSPModel_NonInteracting::new(oc, net))).cast::<MSPModel>();
        let mut movementModel = nonInteracting;
        if isPerson {
            let model = get_string("pedestrian.model");
            // shortcut: C++ compiles the jupedsim branches only with HAVE_JUPEDSIM_GRPC / HAVE_JUPEDSIM_LEGACY; the port has no such build flags, so they are always on, upgrade when build flags are ported
            if model == "striping" {
                movementModel = leak_ptr(with_options(|oc| MSPModel_Striping::new(oc, net))).cast::<MSPModel>();
            } else if model == "jupedsim" {
                movementModel = leak_ptr(with_options(|oc| MSPModel_JuPedSim::new(oc, net))).cast::<MSPModel>();
            } else if model == "jupedsim_legacy" {
                movementModel = leak_ptr(with_options(|oc| MSPModel_JuPedSim_legacy::new(oc, net))).cast::<MSPModel>();
            } else if model != "nonInteracting" {
                // C++ deletes myNonInteractingModel here before throwing; the port does not own models yet
                panic!("{}", ProcessError::new(utils::tlf!("Unknown pedestrian model '%'", model)));
            }
        }
        let mut s = MSTransportableControl {
            myTransportables: BTreeMap::new(),
            myWaiting4Departure: BTreeMap::new(),
            myWaitingUntil: BTreeMap::new(),
            myWaiting4Vehicle: BTreeMap::new(),
            myLoadedNumber: 0,
            myDiscardedNumber: 0,
            myRunningNumber: 0,
            myJammedNumber: 0,
            myWaitingForDepartureNumber: 0,
            myWaitingForVehicleNumber: 0,
            myWaitingUntilNumber: 0,
            myAccessNumber: 0,
            myEndedNumber: 0,
            myArrivedNumber: 0,
            myTeleportsAbortWait: 0,
            myTeleportsWrongDest: 0,
            myHaveNewWaiting: false,
            myMaxTransportableNumber: 0,
            myMovementModel: movementModel,
            myNonInteractingModel: nonInteracting,
            myRouteInfos: MSDevice_Vehroutes_SortedRouteInfo::new(),
            myAbortWaitingTimeout: 0,
        };
        if is_set("vehroute-output") {
            let od = OutputDevice::get_device_by_option("vehroute-output").unwrap();
            s.myRouteInfos.routeOut = Ptr::from_raw(od.as_ptr());
        }
        if is_set("personroute-output") {
            OutputDevice::create_device_by_option("personroute-output", "routes", "routes_file.xsd", -1).unwrap();
            let od = OutputDevice::get_device_by_option("personroute-output").unwrap();
            s.myRouteInfos.routeOut = Ptr::from_raw(od.as_ptr());
        }
        if is_set("personinfo-output") {
            OutputDevice::create_device_by_option("personinfo-output", "tripinfos", "tripinfo_file.xsd", -1).unwrap();
        }
        s.myAbortWaitingTimeout = string2time(&get_string("time-to-teleport.ride")).unwrap_or_else(|e| panic!("{}", e));
        s.myMaxTransportableNumber = if isPerson { get_int("max-num-persons") } else { -1 };
        s
    }

    /// C++: `void MSTransportableControl::~MSTransportableControl()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_state();
        // shortcut: C++ deletes myMovementModel and myNonInteractingModel here; the port does not own model objects yet, so they are leaked, upgrade with model ownership
    }

    /// C++: `bool MSTransportableControl::add(MSTransportable *)`
    /// Adds a single transportable, returns false if an id clash occurred
    // bool
    // MSTransportableControl::add(MSTransportable* transportable) {
    //     const SUMOVehicleParameter& param = transportable->getParameter();
    //     if (myTransportables.find(param.id) == myTransportables.end()) {
    //         myTransportables[param.id] = transportable;
    //         const SUMOTime step = param.depart % DELTA_T == 0 ? param.depart : (param.depart / DELTA_T + 1) * DELTA_T;
    //         myWaiting4Departure[step].push_back(transportable);
    //         myLoadedNumber++;
    //         myWaitingForDepartureNumber++;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, transportable: Ptr<MSTransportable>) -> bool {
        let (id, depart) = {
            let param = transportable.get_parameter();
            (param.id.clone(), param.depart)
        };
        if !self.myTransportables.contains_key(&id) {
            self.myTransportables.insert(id, transportable);
            let step = if depart % delta_t() == 0 { depart } else { (depart / delta_t() + 1) * delta_t() };
            self.myWaiting4Departure.entry(step).or_default().push(transportable);
            self.myLoadedNumber += 1;
            self.myWaitingForDepartureNumber += 1;
            true
        } else {
            false
        }
    }

    /// C++: `MSTransportable * MSTransportableControl::get(const std::string &) const` [const]
    /// Returns the named transportable, if existing
    // MSTransportable*
    // MSTransportableControl::get(const std::string& id) const {
    //     std::map<std::string, MSTransportable*>::const_iterator i = myTransportables.find(id);
    //     if (i == myTransportables.end()) {
    //         return nullptr;
    //     }
    //     return (*i).second;
    // }
    #[allow(unused_variables)]
    pub fn get(&self, id: &str) -> Ptr<MSTransportable> {
        self.myTransportables.get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `void MSTransportableControl::erase(MSTransportable *)` [virtual]
    /// removes a single transportable
    // void
    // MSTransportableControl::erase(MSTransportable* transportable) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (oc.isSet("personinfo-output")) {
    //         transportable->tripInfoOutput(OutputDevice::getDeviceByOption("personinfo-output"));
    //     } else if (oc.isSet("tripinfo-output")) {
    //         transportable->tripInfoOutput(OutputDevice::getDeviceByOption("tripinfo-output"));
    //     } else if (oc.getBool("duration-log.statistics")) {
    //         // collecting statistics is a sideffect
    //         OutputDevice_String dev;
    //         transportable->tripInfoOutput(dev);
    //     }
    //     if (oc.isSet("vehroute-output") || oc.isSet("personroute-output")) {
    //         if (transportable->hasArrived() || oc.getBool("vehroute-output.write-unfinished")) {
    //             // @todo: adapt this if we ever introduce person-device.vehroute
    //             if (transportable->getBoolParam("has.vehroute.person-device", false, true)) {
    //                 if (oc.getBool("vehroute-output.sorted")) {
    //                     const SUMOTime departure = oc.getBool("vehroute-output.intended-depart") ? transportable->getParameter().depart : transportable->getDeparture();
    //                     OutputDevice_String od(1);
    //                     transportable->routeOutput(od, oc.getBool("vehroute-output.route-length"));
    //                     MSDevice_Vehroutes::writeSortedOutput(&myRouteInfos,
    //                                                           departure, transportable->getNumericalID(), od.getString());
    //                 } else {
    //                     transportable->routeOutput(*myRouteInfos.routeOut, oc.getBool("vehroute-output.route-length"));
    //                 }
    //             }
    //         }
    //     }
    //     const std::map<std::string, MSTransportable*>::iterator i = myTransportables.find(transportable->getID());
    //     if (i != myTransportables.end()) {
    //         if (i->second->hasDeparted()) {
    //             myRunningNumber--;
    //         }
    //         myEndedNumber++;
    //         MSNet::getInstance()->informTransportableStateListener(transportable,
    //                 transportable->isPerson() ? MSNet::TransportableState::PERSON_ARRIVED : MSNet::TransportableState::CONTAINER_ARRIVED);
    //         delete i->second;
    //         myTransportables.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn erase(&mut self, transportable: Ptr<MSTransportable>) {
        if is_set("personinfo-output") {
            let dev = OutputDevice::get_device_by_option("personinfo-output").unwrap();
            transportable.trip_info_output(&mut dev.borrow_mut());
        } else if is_set("tripinfo-output") {
            let dev = OutputDevice::get_device_by_option("tripinfo-output").unwrap();
            transportable.trip_info_output(&mut dev.borrow_mut());
        } else if get_bool("duration-log.statistics") {
            // collecting statistics is a sideffect
            let mut dev = OutputDevice::new_string(0);
            transportable.trip_info_output(&mut dev);
        }
        if is_set("vehroute-output") || is_set("personroute-output") {
            if transportable.has_arrived() || get_bool("vehroute-output.write-unfinished") {
                // @todo: adapt this if we ever introduce person-device.vehroute
                if transportable.get_bool_param("has.vehroute.person-device", false, true) {
                    if get_bool("vehroute-output.sorted") {
                        let departure = if get_bool("vehroute-output.intended-depart") { transportable.get_parameter().depart } else { transportable.get_departure() };
                        let mut od = OutputDevice::new_string(1);
                        transportable.route_output(&mut od, get_bool("vehroute-output.route-length"));
                        MSDevice_Vehroutes::write_sorted_output(Ptr::from_mut(&mut self.myRouteInfos), departure, transportable.get_numerical_id(), &od.get_string());
                    } else {
                        transportable.route_output(self.myRouteInfos.routeOut.m(), get_bool("vehroute-output.route-length"));
                    }
                }
            }
        }
        let id = transportable.get_id().to_string();
        if let Some(stored) = self.myTransportables.get(&id).copied() {
            if stored.has_departed() {
                self.myRunningNumber -= 1;
            }
            self.myEndedNumber += 1;
            let state = if transportable.is_person() { MSNet_TransportableState::PERSON_ARRIVED } else { MSNet_TransportableState::CONTAINER_ARRIVED };
            MSNet::get_instance().m().inform_transportable_state_listener(transportable, state, "");
            self.myTransportables.remove(&id);
            // shortcut: C++ `delete i->second` is skipped; MSTransportable subclasses have no virtual drop in the port, so the object is leaked, upgrade with transportable ownership
        }
    }

    /// C++: `void MSTransportableControl::eraseAll()`
    /// all transportables
    // void
    // MSTransportableControl::eraseAll() {
    //     // avoid taxi device having dangling references to erased customers
    //     MSDevice_Taxi::allCustomersErased();
    //     while (loadedBegin() != loadedEnd()) {
    //         erase(loadedBegin()->second);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn erase_all(&mut self) {
        // avoid taxi device having dangling references to erased customers
        MSDevice_Taxi::all_customers_erased();
        while let Some(t) = self.myTransportables.values().next().copied() {
            self.erase(t);
        }
    }

    /// C++: `void MSTransportableControl::setWaitEnd(SUMOTime, MSTransportable *)`
    /// sets the arrival time for a waiting transportable
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_wait_end(&mut self, time: SUMOTime, transportable: Ptr<MSTransportable>) {
        let step = if time % delta_t() == 0 { time } else { (time / delta_t() + 1) * delta_t() };
        // avoid double registration
        let transportables = self.myWaitingUntil.entry(step).or_default();
        if !transportables.contains(&transportable) {
            transportables.push(transportable);
            self.myWaitingUntilNumber += 1;
        }
    }

    /// C++: `void MSTransportableControl::checkWaiting(MSNet *, const SUMOTime)`
    /// checks whether any transportables waiting time is over
    // void
    // MSTransportableControl::checkWaiting(MSNet* net, const SUMOTime time) {
    //     myHaveNewWaiting = false;
    //     while (myWaiting4Departure.find(time) != myWaiting4Departure.end()) {
    //         TransportableVector& transportables = myWaiting4Departure[time];
    //         // we cannot use an iterator here because there might be additions to the vector while proceeding
    //         for (auto it = transportables.begin(); it != transportables.end();) {
    //             MSTransportable* t = *it;
    //             if (myMaxTransportableNumber > 0 && myRunningNumber >= myMaxTransportableNumber) {
    //                 TransportableVector& nextStep = myWaiting4Departure[time + DELTA_T];
    //                 nextStep.insert(nextStep.begin(), transportables.begin(), transportables.end());
    //                 transportables.clear();
    //                 break;
    //             } else if (myMaxTransportableNumber == 0) {
    //                 erase(t);
    //                 it = transportables.erase(it);
    //                 continue;
    //             }
    //             it = transportables.erase(it);
    //             myWaitingForDepartureNumber--;
    //             const bool isPerson = t->isPerson();
    //             t->setDeparted(time);
    //             if (t->proceed(net, time)) {
    //                 myRunningNumber++;
    //                 MSNet::getInstance()->informTransportableStateListener(t,
    //                         isPerson ? MSNet::TransportableState::PERSON_DEPARTED : MSNet::TransportableState::CONTAINER_DEPARTED);
    //                 const OptionsCont& oc = OptionsCont::getOptions();
    //                 if (oc.getBool("vehroute-output.sorted")) {
    //                     const SUMOTime departure = oc.getBool("vehroute-output.intended-depart") ? t->getParameter().depart : time;
    //                     if (oc.isSet("personroute-output")) {
    //                         myRouteInfos.departureCounts[departure]++;
    //                     } else {
    //                         MSDevice_Vehroutes::registerTransportableDepart(departure);
    //                     }
    //                 }
    //             } else {
    //                 erase(t);
    //             }
    //         }
    //         myWaiting4Departure.erase(time);
    //     }
    //     while (myWaitingUntil.find(time) != myWaitingUntil.end()) {
    //         // make a copy because 0-duration stops might modify the vector
    //         const TransportableVector transportables = myWaitingUntil[time];
    //         myWaitingUntil.erase(time);
    //         for (MSTransportable* t : transportables) {
    //             myWaitingUntilNumber--;
    //             if (!t->proceed(net, time)) {
    //                 erase(t);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_waiting(&mut self, net: Ptr<MSNet>, time: SUMOTime) {
        self.myHaveNewWaiting = false;
        while self.myWaiting4Departure.contains_key(&time) {
            // we cannot use an iterator here because there might be additions to the vector while proceeding; index i mirrors the C++ iterator
            let mut i: usize = 0;
            loop {
                let t = match self.myWaiting4Departure.get(&time).and_then(|v| v.get(i)) {
                    Some(t) => *t,
                    None => break,
                };
                if self.myMaxTransportableNumber > 0 && self.myRunningNumber >= self.myMaxTransportableNumber {
                    let rest: Vec<Ptr<MSTransportable>> = self.myWaiting4Departure.get_mut(&time).unwrap().drain(i..).collect();
                    self.myWaiting4Departure.entry(time + delta_t()).or_default().splice(0..0, rest);
                    break;
                } else if self.myMaxTransportableNumber == 0 {
                    self.erase(t);
                    self.myWaiting4Departure.get_mut(&time).unwrap().remove(i);
                    continue;
                }
                self.myWaiting4Departure.get_mut(&time).unwrap().remove(i);
                self.myWaitingForDepartureNumber -= 1;
                let isPerson = t.is_person();
                t.m().set_departed(time);
                if t.m().proceed(net, time, false) {
                    self.myRunningNumber += 1;
                    net.m().inform_transportable_state_listener(
                        t,
                        if isPerson { MSNet_TransportableState::PERSON_DEPARTED } else { MSNet_TransportableState::CONTAINER_DEPARTED },
                        "",
                    );
                    if get_bool("vehroute-output.sorted") {
                        let departure = if get_bool("vehroute-output.intended-depart") { t.get_parameter().depart } else { time };
                        if is_set("personroute-output") {
                            *self.myRouteInfos.departureCounts.entry(departure).or_insert(0) += 1;
                        } else {
                            MSDevice_Vehroutes::register_transportable_depart(departure);
                        }
                    }
                } else {
                    self.erase(t);
                }
            }
            self.myWaiting4Departure.remove(&time);
        }
        while self.myWaitingUntil.contains_key(&time) {
            // make a copy because 0-duration stops might modify the vector
            let transportables = self.myWaitingUntil.remove(&time).unwrap();
            for t in transportables {
                self.myWaitingUntilNumber -= 1;
                if !t.m().proceed(net, time, false) {
                    self.erase(t);
                }
            }
        }
    }

    /// C++: `void MSTransportableControl::addWaiting(const MSEdge *, MSTransportable *)`
    /// adds a transportable to the list of transportables waiting for a vehicle on the specified edge
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn add_waiting(&mut self, edge: Ptr<MSEdge>, person: Ptr<MSTransportable>) {
        self.myWaiting4Vehicle.entry(edge_key(edge)).or_default().push(person);
        self.myWaitingForVehicleNumber += 1;
        self.myHaveNewWaiting = true;
        if self.myAbortWaitingTimeout >= 0 {
            person.m().set_abort_waiting(self.myAbortWaitingTimeout);
        }
    }

    /// C++: `void MSTransportableControl::forceDeparture()`
    /// register forced (traci) departure
    // void
    // MSTransportableControl::forceDeparture() {
    //     myRunningNumber++;
    // }
    #[allow(unused_variables)]
    pub fn force_departure(&mut self) {
        self.myRunningNumber += 1;
    }

    /// C++: `bool MSTransportableControl::hasAnyWaiting(const MSEdge *, SUMOVehicle *) const` [const]
    /// check whether any transportables are waiting for the given vehicle
    // bool
    // MSTransportableControl::hasAnyWaiting(const MSEdge* edge, SUMOVehicle* vehicle) const {
    //     const auto wait = myWaiting4Vehicle.find(edge);
    //     if (wait != myWaiting4Vehicle.end()) {
    //         for (const MSTransportable* t : wait->second) {
    //             if (t->isWaitingFor(vehicle)
    //                     && vehicle->allowsBoarding(t)
    //                     && vehicle->isStoppedInRange(t->getEdgePos(), MSGlobals::gStopTolerance, true)) {
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_any_waiting(&self, edge: Ptr<MSEdge>, vehicle: Ptr<SUMOVehicle>) -> bool {
        if let Some(waiting) = self.myWaiting4Vehicle.get(&edge_key(edge)) {
            for t in waiting {
                if t.is_waiting_for(vehicle)
                    && vehicle.allows_boarding(*t)
                    && vehicle.is_stopped_in_range(t.get_edge_pos(), crate::microsim::ms_globals::stop_tolerance(), true)
                {
                    return true;
                }
            }
        }
        false
    }

    /// C++: `bool MSTransportableControl::loadAnyWaiting(const MSEdge *, SUMOVehicle *, SUMOTime &, SUMOTime &, MSTransportable *const)`
    /// load any applicable transportables Loads any person / container that is waiting on that edge for the given vehicle and removes them from myWaiting
    // bool
    // MSTransportableControl::loadAnyWaiting(const MSEdge* edge, SUMOVehicle* vehicle, SUMOTime& timeToLoadNext, SUMOTime& stopDuration, MSTransportable* const force) {
    //     bool ret = false;
    //     const auto wait = myWaiting4Vehicle.find(edge);
    //     if (wait != myWaiting4Vehicle.end()) {
    //         const SUMOTime currentTime = SIMSTEP;
    //         TransportableVector& transportables = wait->second;
    //         for (TransportableVector::iterator i = transportables.begin(); i != transportables.end();) {
    //             MSTransportable* const t = *i;
    //             if (t->isWaitingFor(vehicle) && (t == force ||
    //                                              (vehicle->allowsBoarding(t)
    //                                               && timeToLoadNext - DELTA_T <= currentTime
    //                                               && vehicle->isStoppedInRange(t->getEdgePos(), MSGlobals::gStopTolerance)))) {
    //                 edge->removeTransportable(t);
    //                 vehicle->addTransportable(t);
    //                 if (myAbortWaitingTimeout >= 0) {
    //                     t->setAbortWaiting(-1);
    //                 }
    //                 if (timeToLoadNext >= 0) { // meso does not have loading times
    //                     const SUMOTime loadingDuration = (SUMOTime)((double)vehicle->getVehicleType().getLoadingDuration(t->isPerson()) * t->getVehicleType().getBoardingFactor());
    //                     //update the time point at which the next transportable can be loaded on the vehicle
    //                     if (timeToLoadNext > currentTime - DELTA_T) {
    //                         timeToLoadNext += loadingDuration;
    //                     } else {
    //                         timeToLoadNext = currentTime + loadingDuration;
    //                     }
    //                 }
    //
    //                 static_cast<MSStageDriving*>(t->getCurrentStage())->setVehicle(vehicle);
    //                 if (t->getCurrentStage()->getOriginStop() != nullptr) {
    //                     t->getCurrentStage()->getOriginStop()->removeTransportable(*i);
    //                 }
    //                 i = transportables.erase(i);
    //                 myWaitingForVehicleNumber--;
    //                 ret = true;
    //             } else {
    //                 ++i;
    //             }
    //         }
    //         if (transportables.empty()) {
    //             myWaiting4Vehicle.erase(wait);
    //         }
    //         if (ret && timeToLoadNext >= 0) {
    //             //if the time a transportable needs to get loaded on the vehicle extends the duration of the stop of the vehicle extend
    //             //the duration by setting it to the loading duration of the transportable
    //             stopDuration = MAX2(stopDuration, timeToLoadNext - currentTime);
    //         }
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn load_any_waiting(&mut self, edge: Ptr<MSEdge>, vehicle: Ptr<SUMOVehicle>, timeToLoadNext: &mut SUMOTime, stopDuration: &mut SUMOTime, force: Ptr<MSTransportable>) -> bool {
        let mut ret = false;
        let key = edge_key(edge);
        if let Some(transportables) = self.myWaiting4Vehicle.get_mut(&key) {
            let currentTime = simstep();
            let mut i: usize = 0;
            while i < transportables.len() {
                let t = transportables[i];
                if t.is_waiting_for(vehicle)
                    && (t == force
                        || (vehicle.allows_boarding(t)
                            && *timeToLoadNext - delta_t() <= currentTime
                            && vehicle.is_stopped_in_range(t.get_edge_pos(), crate::microsim::ms_globals::stop_tolerance(), false)))
                {
                    edge.remove_transportable(t);
                    vehicle.m().add_transportable(t);
                    if self.myAbortWaitingTimeout >= 0 {
                        t.m().set_abort_waiting(-1);
                    }
                    if *timeToLoadNext >= 0 {
                        // meso does not have loading times
                        let loadingDuration: SUMOTime = ((vehicle.get_vehicle_type().get_loading_duration(t.is_person()) as f64) * t.get_vehicle_type().get_boarding_factor()) as SUMOTime;
                        // update the time point at which the next transportable can be loaded on the vehicle
                        if *timeToLoadNext > currentTime - delta_t() {
                            *timeToLoadNext += loadingDuration;
                        } else {
                            *timeToLoadNext = currentTime + loadingDuration;
                        }
                    }
                    t.get_current_stage().cast::<MSStageDriving>().m().set_vehicle(vehicle);
                    let origin = t.get_current_stage().get_origin_stop();
                    if !origin.is_null() {
                        origin.m().remove_transportable(t);
                    }
                    transportables.remove(i);
                    self.myWaitingForVehicleNumber -= 1;
                    ret = true;
                } else {
                    i += 1;
                }
            }
            let empty = transportables.is_empty();
            if empty {
                self.myWaiting4Vehicle.remove(&key);
            }
            // if the time a transportable needs to get loaded on the vehicle extends the duration of the stop of the vehicle extend
            // the duration by setting it to the loading duration of the transportable
            if ret && *timeToLoadNext >= 0 {
                *stopDuration = std::cmp::max(*stopDuration, *timeToLoadNext - currentTime);
            }
        }
        ret
    }

    /// C++: `bool MSTransportableControl::hasTransportables() const` [const]
    /// checks whether any transportable waits to finish her plan
    // bool
    // MSTransportableControl::hasTransportables() const {
    //     return !myTransportables.empty();
    // }
    #[allow(unused_variables)]
    pub fn has_transportables(&self) -> bool {
        !self.myTransportables.is_empty()
    }

    /// C++: `bool MSTransportableControl::hasNonWaiting() const` [const]
    /// checks whether any transportable is still engaged in walking / stopping
    // bool
    // MSTransportableControl::hasNonWaiting() const {
    //     return !myWaiting4Departure.empty() || getMovingNumber() > 0 || myWaitingUntilNumber > 0 || myHaveNewWaiting;
    // }
    #[allow(unused_variables)]
    pub fn has_non_waiting(&self) -> bool {
        !self.myWaiting4Departure.is_empty() || self.get_moving_number() > 0 || self.myWaitingUntilNumber > 0 || self.myHaveNewWaiting
    }

    /// C++: `int MSTransportableControl::getActiveCount()`
    /// return the number of active transportable objects
    // int
    // MSTransportableControl::getActiveCount() {
    //     return (int)myWaiting4Departure.size() + myRunningNumber - myWaitingForVehicleNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_active_count(&mut self) -> i32 {
        self.myWaiting4Departure.len() as i32 + self.myRunningNumber - self.myWaitingForVehicleNumber
    }

    /// C++: `void MSTransportableControl::abortAnyWaitingForVehicle()`
    /// aborts the plan for any transportable that is still waiting for a ride
    // void
    // MSTransportableControl::abortAnyWaitingForVehicle() {
    //     for (const auto& it : myWaiting4Vehicle) {
    //         const MSEdge* edge = it.first;
    //         for (MSTransportable* const p : it.second) {
    //             edge->removeTransportable(p);
    //             MSStageDriving* stage = dynamic_cast<MSStageDriving*>(p->getCurrentStage());
    //             const std::string waitDescription = stage == nullptr ? "waiting" : stage->getWaitingDescription();
    //             WRITE_WARNING(p->getObjectType() + " '" + p->getID() + "' aborted " + waitDescription + ".");
    //             if (myAbortWaitingTimeout >= 0) {
    //                 p->setAbortWaiting(-1);
    //             }
    //             erase(p);
    //         }
    //     }
    //     myWaiting4Vehicle.clear();
    //     myWaitingForVehicleNumber = 0;
    // }
    #[allow(unused_variables)]
    pub fn abort_any_waiting_for_vehicle(&mut self) {
        // erase() does not touch myWaiting4Vehicle, so the map is taken out up front, as the C++ loop only clears it at the end
        let waiting = std::mem::take(&mut self.myWaiting4Vehicle);
        for ((_, edge), list) in waiting {
            for p in list {
                edge.remove_transportable(p);
                let stage = p.get_current_stage();
                let waitDescription = if stage.is_null() || stage.get_stage_type() != crate::microsim::transportables::ms_stage::MSStageType::DRIVING { "waiting".to_string() } else { stage.cast::<MSStageDriving>().get_waiting_description() };
                write_warning(&format!("{} '{}' aborted {}.", p.m().get_object_type(), p.get_id(), waitDescription));
                if self.myAbortWaitingTimeout >= 0 {
                    p.m().set_abort_waiting(-1);
                }
                self.erase(p);
            }
        }
        self.myWaiting4Vehicle.clear();
        self.myWaitingForVehicleNumber = 0;
    }

    /// C++: `void MSTransportableControl::abortWaitingForVehicle(MSTransportable *)`
    /// let the given transportable abort waiting for a vehicle (when removing stage via TraCI)
    // void
    // MSTransportableControl::abortWaitingForVehicle(MSTransportable* t) {
    //     const MSEdge* edge = t->getEdge();
    //     auto it = myWaiting4Vehicle.find(edge);
    //     if (it != myWaiting4Vehicle.end()) {
    //         TransportableVector& waiting = it->second;
    //         auto it2 = std::find(waiting.begin(), waiting.end(), t);
    //         if (it2 != waiting.end()) {
    //             if (myAbortWaitingTimeout >= 0) {
    //                 (*it2)->setAbortWaiting(-1);
    //             }
    //             waiting.erase(it2);
    //             myWaitingForVehicleNumber--;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_waiting_for_vehicle(&mut self, t: Ptr<MSTransportable>) {
        let edge = t.get_edge();
        if let Some(waiting) = self.myWaiting4Vehicle.get_mut(&edge_key(edge)) {
            if let Some(pos) = waiting.iter().position(|x| *x == t) {
                if self.myAbortWaitingTimeout >= 0 {
                    waiting[pos].m().set_abort_waiting(-1);
                }
                waiting.remove(pos);
                self.myWaitingForVehicleNumber -= 1;
            }
        }
    }

    /// C++: `void MSTransportableControl::abortWaiting(MSTransportable *)`
    /// aborts waiting stage of transportable
    // void
    // MSTransportableControl::abortWaiting(MSTransportable* t) {
    //     for (std::map<SUMOTime, TransportableVector>::iterator it = myWaiting4Departure.begin(); it != myWaiting4Departure.end(); ++it) {
    //         TransportableVector& ts = it->second;
    //         TransportableVector::iterator it2 = std::find(ts.begin(), ts.end(), t);
    //         if (it2 != ts.end()) {
    //             ts.erase(it2);
    //         }
    //     }
    //     for (std::map<SUMOTime, TransportableVector>::iterator it = myWaitingUntil.begin(); it != myWaitingUntil.end(); ++it) {
    //         TransportableVector& ts = it->second;
    //         TransportableVector::iterator it2 = std::find(ts.begin(), ts.end(), t);
    //         if (it2 != ts.end()) {
    //             ts.erase(it2);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_waiting(&mut self, t: Ptr<MSTransportable>) {
        for ts in self.myWaiting4Departure.values_mut() {
            if let Some(pos) = ts.iter().position(|x| *x == t) {
                ts.remove(pos);
            }
        }
        for ts in self.myWaitingUntil.values_mut() {
            if let Some(pos) = ts.iter().position(|x| *x == t) {
                ts.remove(pos);
            }
        }
    }

    /// C++: `MSTransportable * MSTransportableControl::buildPerson(const SUMOVehicleParameter *, MSVehicleType *, MSTransportable::MSTransportablePlan *, SumoRNG *) const` [const, virtual]
    /// Builds a new person
    // MSTransportable*
    // MSTransportableControl::buildPerson(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportable::MSTransportablePlan* plan,
    //                                     SumoRNG* rng) const {
    //     const double speedFactor = vtype->computeChosenSpeedDeviation(pars->speedFactor, rng);
    //     return new MSPerson(pars, vtype, plan, speedFactor);
    // }
    #[allow(unused_variables)]
    pub fn build_person(&self, pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>, rng: Ptr<SumoRNG>) -> Ptr<MSTransportable> {
        let speedFactor = vtype.compute_chosen_speed_deviation(pars.speedFactor, rng, -1.0);
        // C++ builds the devices in the MSTransportable constructor; here after leaking, so the devices' holder pointers stay valid
        let person = leak_ptr(MSPerson::new(pars, vtype, plan, speedFactor)).cast::<MSTransportable>();
        let p = person.m();
        let mut devices = std::mem::take(&mut p.myDevices);
        MSDevice::build_transportable_devices(p, &mut devices);
        p.myDevices = devices;
        person
    }

    /// C++: `MSTransportable * MSTransportableControl::buildContainer(const SUMOVehicleParameter *, MSVehicleType *, MSTransportable::MSTransportablePlan *) const` [const, virtual]
    /// Builds a new container
    // MSTransportable*
    // MSTransportableControl::buildContainer(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportable::MSTransportablePlan* plan) const {
    //     return new MSTransportable(pars, vtype, plan, false);
    // }
    #[allow(unused_variables)]
    pub fn build_container(&self, pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>) -> Ptr<MSTransportable> {
        let container = leak_ptr(MSTransportable::new(pars, vtype, plan, false));
        let c = container.m();
        let mut devices = std::mem::take(&mut c.myDevices);
        MSDevice::build_transportable_devices(c, &mut devices);
        c.myDevices = devices;
        container
    }

    /// C++: `constVehIt MSTransportableControl::loadedBegin() const` [const]
    /// Returns the begin of the internal transportables map
    // constVehIt loadedBegin() const {
    //         return myTransportables.begin();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_begin(&self) -> MSTransportableControl_constVehIt<'_> {
        self.myTransportables.iter()
    }

    /// C++: `constVehIt MSTransportableControl::loadedEnd() const` [const]
    /// Returns the end of the internal transportables map
    // constVehIt loadedEnd() const {
    //         return myTransportables.end();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_end(&self) -> MSTransportableControl_constVehIt<'_> {
        // shortcut: Rust iterators have no end value, so end() is an exhausted iterator; no caller compares against it yet, upgrade with a sentinel-based iterator when one is needed
        let mut it = self.myTransportables.iter();
        it.by_ref().for_each(drop);
        it
    }

    /// C++: `int MSTransportableControl::size() const` [const]
    /// Returns the number of known transportables
    // int size() const {
    //         return (int)myTransportables.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myTransportables.len() as i32
    }

    /// C++: `void MSTransportableControl::registerJammed()`
    /// register a jammed transportable
    // void registerJammed() {
    //         myJammedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn register_jammed(&mut self) {
        self.myJammedNumber += 1;
    }

    /// C++: `void MSTransportableControl::registerTeleportAbortWait()`
    /// register a teleport after aborting a long wait
    // void registerTeleportAbortWait() {
    //         myTeleportsAbortWait++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_abort_wait(&mut self) {
        self.myTeleportsAbortWait += 1;
    }

    /// C++: `void MSTransportableControl::registerTeleportWrongDest()`
    /// register a teleport to the final destination
    // void registerTeleportWrongDest() {
    //         myTeleportsWrongDest++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_wrong_dest(&mut self) {
        self.myTeleportsWrongDest += 1;
    }

    /// C++: `void MSTransportableControl::fixLoadCount(const MSTransportable *)`
    /// decrement counter to avoid double counting transportables loaded from state
    // void
    // MSTransportableControl::fixLoadCount(const MSTransportable* transportable) {
    //     myLoadedNumber--;
    //     if (transportable->hasDeparted()) {
    //         const SUMOVehicleParameter& param = transportable->getParameter();
    //         const SUMOTime step = param.depart % DELTA_T == 0 ? param.depart : (param.depart / DELTA_T + 1) * DELTA_T;
    //         TransportableVector& waiting = myWaiting4Departure[step];
    //         auto it = std::find(waiting.begin(), waiting.end(), transportable);
    //         if (it != waiting.end()) {
    //             waiting.erase(it);
    //             if (waiting.size() == 0) {
    //                 myWaiting4Departure.erase(step);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_load_count(&mut self, transportable: Ptr<MSTransportable>) {
        self.myLoadedNumber -= 1;
        if transportable.has_departed() {
            let depart = transportable.get_parameter().depart;
            let step = if depart % delta_t() == 0 { depart } else { (depart / delta_t() + 1) * delta_t() };
            let waiting = self.myWaiting4Departure.entry(step).or_default();
            if let Some(pos) = waiting.iter().position(|x| *x == transportable) {
                waiting.remove(pos);
                if waiting.is_empty() {
                    self.myWaiting4Departure.remove(&step);
                }
            }
        }
    }

    /// C++: `int MSTransportableControl::getLoadedNumber() const` [const]
    /// Returns the number of build transportables
    // int getLoadedNumber() const {
    //         return myLoadedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_number(&self) -> i32 {
        self.myLoadedNumber
    }

    /// C++: `int MSTransportableControl::getDepartedNumber() const` [const]
    // int
    // MSTransportableControl::getDepartedNumber() const {
    //     return myLoadedNumber - myWaitingForDepartureNumber - myDiscardedNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_departed_number(&self) -> i32 {
        self.myLoadedNumber - self.myWaitingForDepartureNumber - self.myDiscardedNumber
    }

    /// C++: `int MSTransportableControl::getRunningNumber() const` [const]
    /// Returns the number of build and inserted, but not yet deleted transportables
    // int getRunningNumber() const {
    //         return myRunningNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_running_number(&self) -> i32 {
        self.myRunningNumber
    }

    /// C++: `int MSTransportableControl::getJammedNumber() const` [const]
    /// Returns the number of times a transportables was jammed
    // int getJammedNumber() const {
    //         return myJammedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_jammed_number(&self) -> i32 {
        self.myJammedNumber
    }

    /// C++: `int MSTransportableControl::getWaitingForVehicleNumber() const` [const]
    /// Returns the number of transportables waiting for a ride
    // int getWaitingForVehicleNumber() const {
    //         return myWaitingForVehicleNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_for_vehicle_number(&self) -> i32 {
        self.myWaitingForVehicleNumber
    }

    /// C++: `int MSTransportableControl::getWaitingUntilNumber() const` [const]
    /// Returns the number of transportables waiting for a specified amount of time
    // int getWaitingUntilNumber() const {
    //         return myWaitingUntilNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_until_number(&self) -> i32 {
        self.myWaitingUntilNumber
    }

    /// C++: `int MSTransportableControl::getMovingNumber() const` [const]
    /// Returns the number of transportables moving by themselvs (i.e. walking)
    // int
    // MSTransportableControl::getMovingNumber() const {
    //     return myMovementModel->getActiveNumber() + myAccessNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_moving_number(&self) -> i32 {
        self.myMovementModel.m().get_active_number() + self.myAccessNumber
    }

    /// C++: `int MSTransportableControl::getRidingNumber() const` [const]
    /// Returns the number of transportables riding a vehicle
    // int
    // MSTransportableControl::getRidingNumber() const {
    //     return myRunningNumber - myWaitingUntilNumber - myWaitingForVehicleNumber - getMovingNumber();
    // }
    #[allow(unused_variables)]
    pub fn get_riding_number(&self) -> i32 {
        self.myRunningNumber - self.myWaitingUntilNumber - self.myWaitingForVehicleNumber - self.get_moving_number()
    }

    /// C++: `int MSTransportableControl::getEndedNumber() const` [const]
    /// Returns the number of transportables that exited the simulation
    // int getEndedNumber() const {
    //         return myEndedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_ended_number(&self) -> i32 {
        self.myEndedNumber
    }

    /// C++: `int MSTransportableControl::getArrivedNumber() const` [const]
    /// Returns the number of transportables that arrived at their destination
    // int getArrivedNumber() const {
    //         return myArrivedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrived_number(&self) -> i32 {
        self.myArrivedNumber
    }

    /// C++: `int MSTransportableControl::getDiscardedNumber() const` [const]
    /// Returns the number of discarded transportables
    // int getDiscardedNumber() const {
    //         return myDiscardedNumber;
    //     }
    #[allow(unused_variables)]
    pub fn get_discarded_number(&self) -> i32 {
        self.myDiscardedNumber
    }

    /// C++: `int MSTransportableControl::getTeleportsAbortWait() const` [const]
    /// return the number of teleports due to excessive waiting for a ride
    // int getTeleportsAbortWait() const {
    //         return myTeleportsAbortWait;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_abort_wait(&self) -> i32 {
        self.myTeleportsAbortWait
    }

    /// C++: `int MSTransportableControl::getTeleportsWrongDest() const` [const]
    /// return the number of teleports of transportables riding to the wrong destination
    // int getTeleportsWrongDest() const {
    //         return myTeleportsWrongDest;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_wrong_dest(&self) -> i32 {
        self.myTeleportsWrongDest
    }

    /// C++: `int MSTransportableControl::getTeleportCount() const` [const]
    /// Returns the number of teleports transportables did
    // int getTeleportCount() const {
    //         return myTeleportsAbortWait + myTeleportsWrongDest;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleport_count(&self) -> i32 {
        self.myTeleportsAbortWait + self.myTeleportsWrongDest
    }

    /// C++: `MSPModel * MSTransportableControl::getMovementModel()`
    /// Returns the default movement model for this kind of transportables
    // inline MSPModel* getMovementModel() {
    //         return myMovementModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_movement_model(&mut self) -> Ptr<MSPModel> {
        self.myMovementModel
    }

    /// C++: `MSPModel * MSTransportableControl::getNonInteractingModel()`
    /// Returns the non interacting movement model (for tranship and "beaming")
    // inline MSPModel* getNonInteractingModel() {
    //         return myNonInteractingModel;
    //     }
    #[allow(unused_variables)]
    pub fn get_non_interacting_model(&mut self) -> Ptr<MSPModel> {
        self.myNonInteractingModel
    }

    /// C++: `void MSTransportableControl::addArrived()`
    // void addArrived() {
    //         myArrivedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn add_arrived(&mut self) {
        self.myArrivedNumber += 1;
    }

    /// C++: `void MSTransportableControl::addDiscarded()`
    // void addDiscarded() {
    //         myLoadedNumber++;
    //         myDiscardedNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn add_discarded(&mut self) {
        self.myLoadedNumber += 1;
        self.myDiscardedNumber += 1;
    }

    /// C++: `void MSTransportableControl::startedAccess()`
    // void startedAccess() {
    //         myAccessNumber++;
    //     }
    #[allow(unused_variables)]
    pub fn started_access(&mut self) {
        self.myAccessNumber += 1;
    }

    /// C++: `void MSTransportableControl::endedAccess()`
    // void endedAccess() {
    //         myAccessNumber--;
    //     }
    #[allow(unused_variables)]
    pub fn ended_access(&mut self) {
        self.myAccessNumber -= 1;
    }

    /// C++: `void MSTransportableControl::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSTransportableControl::saveState(OutputDevice& out) {
    //     std::ostringstream oss;
    //     oss << myRunningNumber << " " << myLoadedNumber << " " << myEndedNumber << " " << myWaitingForDepartureNumber << " " << myArrivedNumber << " " << myDiscardedNumber;
    //     oss << " " << myJammedNumber << " " << myWaitingForVehicleNumber << " " << myHaveNewWaiting;
    //     out.writeAttr(SUMO_ATTR_STATE, oss.str());
    //     for (const auto& it : myTransportables) {
    //         it.second->saveState(out);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        let mut oss = format!(
            "{} {} {} {} {} {}",
            self.myRunningNumber, self.myLoadedNumber, self.myEndedNumber, self.myWaitingForDepartureNumber, self.myArrivedNumber, self.myDiscardedNumber
        );
        oss.push_str(&format!(" {} {} {}", self.myJammedNumber, self.myWaitingForVehicleNumber, self.myHaveNewWaiting as i32));
        out.write_attr(SUMO_ATTR_STATE, oss.as_str());
        for t in self.myTransportables.values() {
            t.m().save_state(out);
        }
    }

    /// C++: `void MSTransportableControl::loadState(const std::string &)`
    /// Reconstruct the current state
    // void
    // MSTransportableControl::loadState(const std::string& state) {
    //     std::istringstream iss(state);
    //     iss >> myRunningNumber >> myLoadedNumber >> myEndedNumber >> myWaitingForDepartureNumber >> myArrivedNumber >> myDiscardedNumber;
    //     iss >> myJammedNumber >> myWaitingForVehicleNumber >> myHaveNewWaiting;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, state: &str) {
        let vals: Vec<&str> = state.split_whitespace().collect();
        // C++ stream extraction: stop at the first value that does not parse, leaving the rest unchanged
        let mut fields: [&mut i32; 8] = [
            &mut self.myRunningNumber,
            &mut self.myLoadedNumber,
            &mut self.myEndedNumber,
            &mut self.myWaitingForDepartureNumber,
            &mut self.myArrivedNumber,
            &mut self.myDiscardedNumber,
            &mut self.myJammedNumber,
            &mut self.myWaitingForVehicleNumber,
        ];
        for (idx, f) in fields.iter_mut().enumerate() {
            match vals.get(idx).and_then(|s| s.parse::<i32>().ok()) {
                Some(v) => **f = v,
                None => return,
            }
        }
        if let Some(v) = vals.get(8).and_then(|s| s.parse::<i32>().ok()) {
            self.myHaveNewWaiting = v != 0;
        }
    }

    /// C++: `void MSTransportableControl::clearState()`
    /// Resets transportables when quick-loading state
    // void
    // MSTransportableControl::clearState() {
    //     for (std::map<std::string, MSTransportable*>::iterator i = myTransportables.begin(); i != myTransportables.end(); ++i) {
    //         delete (*i).second;
    //     }
    //     myTransportables.clear();
    //     myWaiting4Vehicle.clear();
    //     myWaiting4Departure.clear();
    //     myWaitingUntil.clear();
    //     myLoadedNumber = 0;
    //     myDiscardedNumber = 0;
    //     myRunningNumber = 0;
    //     myJammedNumber = 0;
    //     myWaitingForDepartureNumber = 0;
    //     myWaitingForVehicleNumber = 0;
    //     myWaitingUntilNumber = 0;
    //     myEndedNumber = 0;
    //     myArrivedNumber = 0;
    //     myHaveNewWaiting = false;
    //     if (myMovementModel != myNonInteractingModel) {
    //         myMovementModel->clearState();
    //     }
    //     myNonInteractingModel->clearState();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        // shortcut: C++ deletes the transportables here; the port has no owning Box for them yet, so they are leaked, upgrade with transportable ownership
        self.myTransportables.clear();
        self.myWaiting4Vehicle.clear();
        self.myWaiting4Departure.clear();
        self.myWaitingUntil.clear();
        self.myLoadedNumber = 0;
        self.myDiscardedNumber = 0;
        self.myRunningNumber = 0;
        self.myJammedNumber = 0;
        self.myWaitingForDepartureNumber = 0;
        self.myWaitingForVehicleNumber = 0;
        self.myWaitingUntilNumber = 0;
        self.myEndedNumber = 0;
        self.myArrivedNumber = 0;
        self.myHaveNewWaiting = false;
        if self.myMovementModel != self.myNonInteractingModel {
            self.myMovementModel.m().clear_state();
        }
        self.myNonInteractingModel.m().clear_state();
    }

    /// C++: `const MSDevice_Vehroutes::SortedRouteInfo & MSTransportableControl::getRouteInfo()`
    // const MSDevice_Vehroutes::SortedRouteInfo& getRouteInfo() {
    //         return myRouteInfos;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_info(&mut self) -> &MSDevice_Vehroutes_SortedRouteInfo {
        &self.myRouteInfos
    }

    /// C++: `MSTransportableControl & MSTransportableControl::operator=(const MSTransportableControl &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &MSTransportableControl) -> &mut MSTransportableControl {
        panic!("MSTransportableControl copy assignment is invalidated in C++ (declared private and deleted)")
    }

}

