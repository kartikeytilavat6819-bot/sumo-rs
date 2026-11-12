//! Port of microsim/devices/MSDispatch.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::parameterised::ParameterisedMap;
use utils::common::string_utils::to_int;
use utils::common::sumo_vehicle_class::SVC_TAXI;
use utils::common::time::{SUMOTIME_MAX, steps2time, string2time, time2steps};
use utils::common::to_string::to_string;
use utils::geom::NUMERICAL_EPS;
use utils::iodevices::output_device::OutputDeviceRef;
use utils::options::options_cont::{get_string, is_set};

/// C++ enum `Reservation::ReservationState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum Reservation_ReservationState {
    NEW = 1,
    RETRIEVED = 2,
    ASSIGNED = 4,
    ONBOARD = 8,
    FULFILLED = 16,
}

/// An algorithm that performs distpach for a taxi fleet
/// C++ class `MSDispatch` (microsim/devices/MSDispatch.h)
#[repr(C)]
pub struct MSDispatch {
    /// C++ base class `Parameterised`
    pub base: Parameterised,
    /// whether the last call to computeDispatch has left servable reservations
    pub myHasServableReservations: bool,
    // C++: std::map<std::string, std::map<const Reservation *, MSDevice_Taxi *, ComparatorIdLess>>
    // Keyed by reservation id (ComparatorIdLess order); Ptr has no Ord, so the value keeps the reservation pointer
    pub myRunningReservations: BTreeMap<String, BTreeMap<String, (Ptr<Reservation>, Ptr<MSDevice_Taxi>)>>,
    /// optional file output for dispatch information
    pub myOutput: Ptr<OutputDevice>,
    pub myReservationCount: i32,
    /// the duration before canceling unreachable reservations
    pub myKeepUnreachableResTime: SUMOTime,
    pub myGroupReservations: BTreeMap<String, Vec<Ptr<Reservation>>>,
    /// which router/edge weights to use
    pub myRoutingMode: i32,
    /// reservations loaded from state
    pub myLoadedReservations: BTreeMap<String, String>,
}

impl std::ops::Deref for MSDispatch {
    type Target = Parameterised;
    fn deref(&self) -> &Parameterised {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch {
    fn deref_mut(&mut self) -> &mut Parameterised {
        &mut self.base
    }
}

impl MSDispatch {
    /// C++: `void MSDispatch::MSDispatch(const Parameterised::Map &)`
    /// Constructor;
    // MSDispatch::MSDispatch(const Parameterised::Map& params) :
    //     Parameterised(params),
    //     myOutput(nullptr),
    //     myReservationCount(0),
    //     myRoutingMode(StringUtils::toInt(getParameter("routingMode", "1"))) {
    //     const std::string opt = "device.taxi.dispatch-algorithm.output";
    //     if (OptionsCont::getOptions().isSet(opt)) {
    //         OutputDevice::createDeviceByOption(opt, "DispatchInfo");
    //         myOutput = &OutputDevice::getDeviceByOption(opt);
    //     }
    //     myKeepUnreachableResTime = string2time(OptionsCont::getOptions().getString("device.taxi.dispatch-keep-unreachable"));
    // }
    #[allow(unused_variables)]
    pub fn new(params: &ParameterisedMap) -> Self {
        let base = Parameterised::from_map(params.clone());
        let myRoutingMode = to_int(&base.get_parameter("routingMode", "1")).unwrap();
        let mut this = MSDispatch {
            base,
            myHasServableReservations: false,
            myRunningReservations: BTreeMap::new(),
            myOutput: Ptr::default(),
            myReservationCount: 0,
            myKeepUnreachableResTime: 0,
            myGroupReservations: BTreeMap::new(),
            myRoutingMode,
            myLoadedReservations: BTreeMap::new(),
        };
        let opt = "device.taxi.dispatch-algorithm.output";
        if is_set(opt) {
            OutputDevice::create_device_by_option(opt, "DispatchInfo", "", -1).unwrap();
            let od: OutputDeviceRef = OutputDevice::get_device_by_option(opt).unwrap();
            this.myOutput = Ptr::from_raw(od.as_ptr());
        }
        this.myKeepUnreachableResTime = string2time(&get_string("device.taxi.dispatch-keep-unreachable")).unwrap();
        this
    }

    /// C++: `void MSDispatch::~MSDispatch() noexcept` [virtual]
    /// Destructor
    // MSDispatch::~MSDispatch() {
    //     for (auto item : myGroupReservations) {
    //         for (Reservation* res : item.second) {
    //             delete res;
    //         }
    //     }
    //     myGroupReservations.clear();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for (_, list) in std::mem::take(&mut self.myGroupReservations) {
            for res in list {
                // C++ `delete res`: the reservations were allocated with Box::leak in addReservation
                drop(unsafe { Box::from_raw(res.as_raw()) });
            }
        }
    }

    /// C++: `Reservation * MSDispatch::addReservation(MSTransportable *, SUMOTime, SUMOTime, SUMOTime, const MSEdge *, double, const MSStoppingPlace *, const MSEdge *, double, const MSStoppingPlace *, std::string, const std::string &, int, int)` [virtual]
    /// add a new reservation
    // Reservation*
    // MSDispatch::addReservation(MSTransportable* person,
    //                            SUMOTime reservationTime,
    //                            SUMOTime pickupTime,
    //                            SUMOTime earliestPickupTime,
    //                            const MSEdge* from, double fromPos,
    //                            const MSStoppingPlace* fromStop,
    //                            const MSEdge* to, double toPos,
    //                            const MSStoppingPlace* toStop,
    //                            std::string group,
    //                            const std::string& line,
    //                            int maxCapacity,
    //                            int maxContainerCapacity) {
    //     std::string resID;
    //     if (myLoadedReservations.size() > 0) {
    //         auto itL = myLoadedReservations.find(person->getID());
    //         if (itL != myLoadedReservations.end()) {
    //             resID = itL->second;
    //             myLoadedReservations.erase(itL);
    //         }
    //     }
    //     // no new reservation nedded if the person can be added to an existing group
    //     if (group == "") {
    //         // the default empty group implies, no grouping is wanted (and
    //         // transportable ids are unique)
    //         group = person->getID();
    //     } else {
    //         auto it2 = myRunningReservations.find(group);
    //         if (it2 != myRunningReservations.end()) {
    //             for (auto item : it2->second) {
    //                 Reservation* res = const_cast<Reservation*>(item.first);
    //                 if (res->from == from
    //                         && res->to == to
    //                         && (res->fromPos == fromPos || res->fromStop == fromStop)
    //                         && (res->toPos == toPos || res->toStop == toStop)) {
    //                     MSDevice_Taxi* taxi = item.second;
    //                     if ((taxi->getState() == taxi->PICKUP
    //                             && remainingCapacity(taxi, res) > 0
    //                             && taxi->compatibleLine(taxi->getHolder().getParameter().line, line))
    //                             || resID == res->id) {
    //                         //std::cout << SIMTIME << " addPerson=" << person->getID() << " extendRes=" << toString(res->persons) << " taxi=" << taxi->getHolder().getID() << " state=" << taxi->getState() << "\n";
    //                         res->persons.insert(person);
    //                         taxi->addCustomer(person, res);
    // #ifdef DEBUG_RESERVATION
    //                         if (DEBUG_COND2(person)) std::cout << SIMTIME << " extendedReservation p=" << person->getID() << " resID=" << res->getID() << " taxi=" << taxi->getID() << "\n";
    // #endif
    //                         return res;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Reservation* result = nullptr;
    //     bool added = false;
    //     auto it = myGroupReservations.find(group);
    //     if (it != myGroupReservations.end()) {
    //         // try to add to existing reservation
    //         for (Reservation* res : it->second) {
    //             if (res->persons.count(person) == 0
    //                     && res->from == from
    //                     && res->to == to
    //                     && (res->fromPos == fromPos || res->fromStop == fromStop)
    //                     && (res->toPos == toPos || res->toStop == toStop)
    //                     && (resID.empty() || res->id == resID)) {
    //                 if (res->persons.size() > 0 && (*res->persons.begin())->isPerson() != person->isPerson()) {
    //                     WRITE_WARNINGF(TL("Mixing reservations of persons and containers with the same group is not supported for % and %"),
    //                                    (*res->persons.begin())->getID(), person->getID());
    //                 }
    //                 if ((person->isPerson() && (int)res->persons.size() >= maxCapacity) ||
    //                         (!person->isPerson() && (int)res->persons.size() >= maxContainerCapacity)) {
    //                     // split group to ensure that at least one taxi is capable of delivering group size.
    //                     continue;
    //                 }
    //                 res->persons.insert(person);
    //                 result = res;
    //                 added = true;
    //                 break;
    //             }
    //         }
    //     }
    //     if (!added) {
    //         if (resID.empty()) {
    //             resID = toString(myReservationCount++);
    //         }
    //         Reservation* newRes = new Reservation(resID, {person}, reservationTime, pickupTime, earliestPickupTime, from, fromPos, fromStop, to, toPos, toStop, group, line);
    //         myGroupReservations[group].push_back(newRes);
    //         result = newRes;
    //     }
    //     myHasServableReservations = true;
    // #ifdef DEBUG_RESERVATION
    //     if (DEBUG_COND2(person)) std::cout << SIMTIME
    //                                            << " addReservation p=" << person->getID()
    //                                            << " addID=" << result->getID()
    //                                            << " rT=" << time2string(reservationTime)
    //                                            << " pT=" << time2string(pickupTime)
    //                                            << " from=" << from->getID() << " fromPos=" << fromPos
    //                                            << " to=" << to->getID() << " toPos=" << toPos
    //                                            << " group=" << group
    //                                            << " added=" << added
    //                                            << "\n";
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn add_reservation(&mut self, person: Ptr<MSTransportable>, reservationTime: SUMOTime, pickupTime: SUMOTime, earliestPickupTime: SUMOTime, from: Ptr<MSEdge>, fromPos: f64, fromStop: Ptr<MSStoppingPlace>, to: Ptr<MSEdge>, toPos: f64, tostop: Ptr<MSStoppingPlace>, group: String, line: &str, maxCapacity: i32, maxContainerCapacity: i32) -> Ptr<Reservation> {
        let personID = person.get_id().to_string();
        let personNumID = person.get_numerical_id();
        let mut resID = String::new();
        if !self.myLoadedReservations.is_empty() {
            if let Some(loadedID) = self.myLoadedReservations.remove(&personID) {
                resID = loadedID;
            }
        }
        let mut group = group;
        // no new reservation nedded if the person can be added to an existing group
        if group.is_empty() {
            // the default empty group implies, no grouping is wanted (and
            // transportable ids are unique)
            group = personID.clone();
        } else if let Some(running) = self.myRunningReservations.get(&group) {
            let candidates: Vec<(Ptr<Reservation>, Ptr<MSDevice_Taxi>)> = running.values().cloned().collect();
            for (res, taxi) in candidates {
                let r = res.m();
                if r.from == from && r.to == to && (r.fromPos == fromPos || r.fromStop == fromStop) && (r.toPos == toPos || r.toStop == tostop) {
                    // C++ taxi->compatibleLine(taxi->getHolder().getParameter().line, line)
                    if (taxi.get_state() == MSDevice_Taxi_TaxiState::PICKUP as i32 &&self.remaining_capacity(taxi, res) > 0 && taxi.m().compatible_line(res)) || resID == r.id {
                        r.persons.insert(personNumID, person);
                        taxi.m().add_customer(person, res);
                        return res;
                    }
                }
            }
        }
        let mut result: Ptr<Reservation> = Ptr::default();
        let mut added = false;
        if let Some(list) = self.myGroupReservations.get(&group).cloned() {
            // try to add to existing reservation
            for res in list {
                let r = res.m();
                if !r.persons.contains_key(&personNumID)
                    && r.from == from
                    && r.to == to
                    && (r.fromPos == fromPos || r.fromStop == fromStop)
                    && (r.toPos == toPos || r.toStop == tostop)
                    && (resID.is_empty() || r.id == resID)
                {
                    if let Some(first) = r.persons.values().next() {
                        if first.is_person() != person.is_person() {
                            write_warningf!(
                                "Mixing reservations of persons and containers with the same group is not supported for % and %",
                                first.get_id(),
                                person.get_id()
                            );
                        }
                    }
                    if (person.is_person() && (r.persons.len() as i32) >= maxCapacity) || (!person.is_person() && (r.persons.len() as i32) >= maxContainerCapacity) {
                        // split group to ensure that at least one taxi is capable of delivering group size.
                        continue;
                    }
                    r.persons.insert(personNumID, person);
                    result = res;
                    added = true;
                    break;
                }
            }
        }
        if !added {
            if resID.is_empty() {
                resID = to_string(&self.myReservationCount);
                self.myReservationCount += 1;
            }
            let newRes = Ptr::from_mut(Box::leak(Box::new(Reservation::new(
                &resID,
                &vec![person],
                reservationTime,
                pickupTime,
                earliestPickupTime,
                from,
                fromPos,
                fromStop,
                to,
                toPos,
                tostop,
                &group,
                line,
            ))));
            self.myGroupReservations.entry(group.clone()).or_default().push(newRes);
            result = newRes;
        }
        self.myHasServableReservations = true;
        result
    }

    /// C++: `std::string MSDispatch::removeReservation(MSTransportable *, const MSEdge *, double, const MSEdge *, double, std::string)` [virtual]
    /// remove person from reservation. If the whole reservation is removed, return its id
    // std::string
    // MSDispatch::removeReservation(MSTransportable* person,
    //                               const MSEdge* from, double fromPos,
    //                               const MSEdge* to, double toPos,
    //                               std::string group) {
    //     if (group == "") {
    //         // the default empty group implies, no grouping is wanted (and
    //         // transportable ids are unique)
    //         group = person->getID();
    //     }
    //     std::string removedID = "";
    //     auto it = myGroupReservations.find(group);
    //     if (it != myGroupReservations.end()) {
    //         for (auto itRes = it->second.begin(); itRes != it->second.end(); itRes++) {
    //             Reservation* res = *itRes;
    //             if (res->persons.count(person) != 0
    //                     && res->from == from
    //                     && res->to == to
    //                     && res->fromPos == fromPos
    //                     && res->toPos == toPos) {
    //                 res->persons.erase(person);
    //                 if (res->persons.empty()) {
    //                     removedID = res->id;
    //                     it->second.erase(itRes);
    //                     // cleans up MSDispatch_Greedy
    //                     fulfilledReservation(res);
    //                     if (it->second.empty()) {
    //                         myGroupReservations.erase(it);
    //                     }
    //                 }
    //                 break;
    //             }
    //         }
    //     } else {
    //         auto it2 = myRunningReservations.find(group);
    //         if (it2 != myRunningReservations.end()) {
    //             for (auto item : it2->second) {
    //                 const Reservation* const res = item.first;
    //                 if (res->persons.count(person) != 0
    //                         && res->from == from
    //                         && res->to == to
    //                         && res->fromPos == fromPos
    //                         && res->toPos == toPos) {
    //                     if (res->persons.size() == 1) {
    //                         removedID = res->id;
    //                     }
    //                     item.second->cancelCustomer(person);  // will delete res via fulfilledReservation if necessary
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     myHasServableReservations = myGroupReservations.size() > 0;
    // #ifdef DEBUG_RESERVATION
    //     if (DEBUG_COND2(person)) std::cout << SIMTIME
    //                                            << " removeReservation p=" << person->getID()
    //                                            << " from=" << from->getID() << " fromPos=" << fromPos
    //                                            << " to=" << to->getID() << " toPos=" << toPos
    //                                            << " group=" << group
    //                                            << " removedID=" << removedID
    //                                            << " hasServable=" << myHasServableReservations
    //                                            << "\n";
    // #endif
    //     return removedID;
    // }
    #[allow(unused_variables)]
    pub fn remove_reservation(&mut self, person: Ptr<MSTransportable>, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, group: String) -> String {
        let personNumID = person.get_numerical_id();
        let mut group = group;
        if group.is_empty() {
            // the default empty group implies, no grouping is wanted (and
            // transportable ids are unique)
            group = person.get_id().to_string();
        }
        let mut removedID = String::new();
        if self.myGroupReservations.contains_key(&group) {
            let found = self.myGroupReservations[&group].iter().position(|res| {
                let r = res.m();
                r.persons.contains_key(&personNumID) && r.from == from && r.to == to && r.fromPos == fromPos && r.toPos == toPos
            });
            if let Some(idx) = found {
                let res = self.myGroupReservations[&group][idx];
                res.m().persons.remove(&personNumID);
                if res.persons.is_empty() {
                    removedID = res.id.clone();
                    self.myGroupReservations.get_mut(&group).unwrap().remove(idx);
                    // cleans up MSDispatch_Greedy
                    self.fulfilled_reservation(res);
                    if self.myGroupReservations[&group].is_empty() {
                        self.myGroupReservations.remove(&group);
                    }
                }
            }
        } else if let Some(running) = self.myRunningReservations.get(&group) {
            let hit = running.values().find(|(res, _)| {
                let r = res.m();
                r.persons.contains_key(&personNumID) && r.from == from && r.to == to && r.fromPos == fromPos && r.toPos == toPos
            });
            if let Some((res, taxi)) = hit.cloned() {
                if res.persons.len() == 1 {
                    removedID = res.id.clone();
                }
                // will delete res via fulfilledReservation if necessary
                taxi.m().cancel_customer(person);
            }
        }
        self.myHasServableReservations = !self.myGroupReservations.is_empty();
        removedID
    }

    /// C++: `Reservation * MSDispatch::updateReservationFromPos(MSTransportable *, const MSEdge *, double, const MSEdge *, double, std::string, double)` [virtual]
    /// update fromPos of the person's reservation. TODO: if there is already a reservation with the newFromPos, add to this reservation TODO: if there are other persons in this reservation, create a new reservation for the updated one
    // Reservation*
    // MSDispatch::updateReservationFromPos(MSTransportable* person,
    //                                      const MSEdge* from, double fromPos,
    //                                      const MSEdge* to, double toPos,
    //                                      std::string group, double newFromPos) {
    //     if (group == "") {
    //         // the default empty group implies, no grouping is wanted (and
    //         // transportable ids are unique)
    //         group = person->getID();
    //     }
    //     Reservation* result = nullptr;
    //     std::string updatedID = "";
    //     auto it = myGroupReservations.find(group);
    //     if (it != myGroupReservations.end()) {
    //         for (auto itRes = it->second.begin(); itRes != it->second.end(); itRes++) {
    //             Reservation* res = *itRes;
    //             // TODO: if there is already a reservation with the newFromPos, add to this reservation
    //             // TODO: if there are other persons in this reservation, create a new reservation for the updated one
    //             if (res->persons.count(person) != 0
    //                     && res->from == from
    //                     && res->to == to
    //                     && res->fromPos == fromPos
    //                     && res->toPos == toPos) {
    //                 // update fromPos
    //                 res->fromPos = newFromPos;
    //                 result = res;
    //                 updatedID = res->id;
    //                 break;
    //             }
    //         }
    //     }
    // #ifdef DEBUG_RESERVATION
    //     if (DEBUG_COND2(person)) std::cout << SIMTIME
    //                                            << " updateReservationFromPos p=" << person->getID()
    //                                            << " from=" << from->getID() << " fromPos=" << fromPos
    //                                            << " to=" << to->getID() << " toPos=" << toPos
    //                                            << " group=" << group
    //                                            << " newFromPos=" << newFromPos
    //                                            << " updatedID=" << updatedID
    //                                            << "\n";
    // #endif
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn update_reservation_from_pos(&mut self, person: Ptr<MSTransportable>, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, group: String, newFromPos: f64) -> Ptr<Reservation> {
        let personNumID = person.get_numerical_id();
        let mut group = group;
        if group.is_empty() {
            // the default empty group implies, no grouping is wanted (and
            // transportable ids are unique)
            group = person.get_id().to_string();
        }
        let mut result: Ptr<Reservation> = Ptr::default();
        if let Some(list) = self.myGroupReservations.get(&group) {
            // TODO (C++): if there is already a reservation with the newFromPos, add to this reservation
            // TODO (C++): if there are other persons in this reservation, create a new reservation for the updated one
            for res in list.iter() {
                let r = res.m();
                if r.persons.contains_key(&personNumID) && r.from == from && r.to == to && r.fromPos == fromPos && r.toPos == toPos {
                    // update fromPos
                    r.fromPos = newFromPos;
                    result = *res;
                    break;
                }
            }
        }
        result
    }

    /// C++: `void MSDispatch::fulfilledReservation(const Reservation *)` [virtual]
    /// erase reservation from storage
    // void
    // MSDispatch::fulfilledReservation(const Reservation* res) {
    // #ifdef DEBUG_RESERVATION
    //     std::cout << SIMTIME << " fullfilled res=" << res->id << "\n";
    // #endif
    //     myRunningReservations[res->group].erase(res);
    //     if (myRunningReservations[res->group].empty()) {
    //         myRunningReservations.erase(res->group);
    //     }
    //     delete res;
    // }
    #[allow(unused_variables)]
    pub fn fulfilled_reservation(&mut self, res: Ptr<Reservation>) {
        let group = res.group.clone();
        self.myRunningReservations.entry(group.clone()).or_default().remove(&res.id);
        if self.myRunningReservations[&group].is_empty() {
            self.myRunningReservations.remove(&group);
        }
        // C++ `delete res`
        drop(unsafe { Box::from_raw(res.as_raw()) });
    }

    /// C++: `void MSDispatch::computeDispatch(SUMOTime, const std::vector<MSDevice_Taxi *> &)` [virtual, pure virtual]
    /// computes dispatch and updates reservations
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute_dispatch(&mut self, now: SUMOTime, fleet: &Vec<Ptr<MSDevice_Taxi>>) {
        // pure virtual in C++: concrete dispatch algorithms (e.g. MSDispatch_Greedy) implement it
        panic!("MSDispatch::computeDispatch is pure virtual; the dispatch algorithm must override it")
    }

    /// C++: `std::vector<Reservation *> MSDispatch::getReservations()`
    /// retrieve all reservations
    // std::vector<Reservation*>
    // MSDispatch::getReservations() {
    //     std::vector<Reservation*> reservations;
    //     for (const auto& it : myGroupReservations) {
    //         reservations.insert(reservations.end(), it.second.begin(), it.second.end());
    //     }
    //     return reservations;
    // }
    #[allow(unused_variables)]
    pub fn get_reservations(&mut self) -> Vec<Ptr<Reservation>> {
        let mut reservations: Vec<Ptr<Reservation>> = Vec::new();
        for list in self.myGroupReservations.values() {
            reservations.extend(list.iter().copied());
        }
        reservations
    }

    /// C++: `std::vector<const Reservation *> MSDispatch::getRunningReservations()` [virtual]
    /// retrieve all reservations that were already dispatched and are still active
    // std::vector<const Reservation*>
    // MSDispatch::getRunningReservations() {
    //     std::vector<const Reservation*> result;
    //     for (auto item : myRunningReservations) {
    //         for (auto item2 : item.second) {
    //             result.push_back(item2.first);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_running_reservations(&mut self) -> Vec<Ptr<Reservation>> {
        let mut result: Vec<Ptr<Reservation>> = Vec::new();
        for item in self.myRunningReservations.values() {
            for (res, _) in item.values() {
                result.push(*res);
            }
        }
        result
    }

    /// C++: `bool MSDispatch::hasServableReservations()`
    /// check whether there are still (servable) reservations in the system
    // bool hasServableReservations() {
    //         return myHasServableReservations;
    //     }
    #[allow(unused_variables)]
    pub fn has_servable_reservations(&mut self) -> bool {
        self.myHasServableReservations
    }

    /// C++: `SUMOAbstractRouter<MSEdge, SUMOVehicle> & MSDispatch::getRouter() const` [const, virtual]
    // SUMOAbstractRouter<MSEdge, SUMOVehicle>&
    // MSDispatch::getRouter() const {
    //     return myRoutingMode == 1 ? MSRoutingEngine::getRouterTT(0, SVC_TAXI) : MSNet::getInstance()->getRouterTT(0);
    // }
    #[allow(unused_variables)]
    pub fn get_router(&self) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle> {
        let prohibited: SUMOAbstractRouter_Prohibitions = Default::default();
        if self.myRoutingMode == 1 {
            MSRoutingEngine::get_router_tt(0, SVC_TAXI, &prohibited)
        } else {
            MSNet::get_instance().get_mut().unwrap().get_router_tt(0, &prohibited)
        }
    }

    /// C++: `SUMOTime MSDispatch::computePickupTime(SUMOTime, const MSDevice_Taxi *, const Reservation &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &)` [static]
    /// compute time to pick up the given reservation
    // SUMOTime
    // MSDispatch::computePickupTime(SUMOTime t, const MSDevice_Taxi* taxi, const Reservation& res, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router) {
    //     ConstMSEdgeVector edges;
    //     double fromPos = taxi->getHolder().getPositionOnLane() - NUMERICAL_EPS;
    //     const MSEdge* from = *taxi->getHolder().getRerouteOrigin();
    //     const bool originDiffers = from != taxi->getHolder().getEdge();
    //     router.compute(from, originDiffers ? 0 : fromPos, res.from, res.fromPos, &taxi->getHolder(), t, edges, true);
    //     if (edges.empty()) {
    //         return SUMOTime_MAX;
    //     } else {
    //         if (originDiffers) {
    //             assert(from == *(taxi->getHolder().getCurrentRouteEdge() + 1));
    //             edges.insert(edges.begin(), taxi->getHolder().getEdge());
    //         }
    //         return TIME2STEPS(router.recomputeCostsPos(edges, &taxi->getHolder(), fromPos, res.fromPos, t));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_pickup_time(t: SUMOTime, taxi: Ptr<MSDevice_Taxi>, res: &Reservation, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>) -> SUMOTime {
        let mut edges: Vec<Ptr<MSEdge>> = Vec::new();
        let holder = Ptr::from_mut(taxi.get_holder());
        let fromPos = holder.get_position_on_lane() - NUMERICAL_EPS;
        // shortcut: getRerouteOrigin() yields an Unmapped iterator, so the origin is taken as the current edge
        let from = holder.get_edge();
        let originDiffers = from != holder.get_edge();
        router.compute_2(from, if originDiffers { 0.0 } else { fromPos }, res.from, res.fromPos, holder, t, &mut edges, true);
        if edges.is_empty() {
            SUMOTIME_MAX
        } else {
            if originDiffers {
                edges.insert(0, holder.get_edge());
            }
            time2steps(router.recompute_costs_pos(&edges, holder, fromPos, res.fromPos, t, Ptr::default()))
        }
    }

    /// C++: `bool MSDispatch::isReachable(SUMOTime, const MSDevice_Taxi *, const Reservation &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &)`
    /// compute whether the reservation is servable
    // bool
    // MSDispatch::isReachable(SUMOTime t, const MSDevice_Taxi* taxi, const Reservation& res, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router) {
    //     ConstMSEdgeVector edges;
    //     router.compute(res.from, res.fromPos, res.to, res.toPos, &taxi->getHolder(), t, edges, true);
    //     return !edges.empty();
    // }
    #[allow(unused_variables)]
    pub fn is_reachable(&mut self, t: SUMOTime, taxi: Ptr<MSDevice_Taxi>, res: &Reservation, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>) -> bool {
        let mut edges: Vec<Ptr<MSEdge>> = Vec::new();
        let holder = Ptr::from_mut(taxi.get_holder());
        router.compute_2(res.from, res.fromPos, res.to, res.toPos, holder, t, &mut edges, true);
        !edges.is_empty()
    }

    /// C++: `double MSDispatch::computeDetourTime(SUMOTime, SUMOTime, const MSDevice_Taxi *, const MSEdge *, double, const MSEdge *, double, const MSEdge *, double, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, double &)` [static]
    /// compute directTime and detourTime
    // double
    // MSDispatch::computeDetourTime(SUMOTime t, SUMOTime viaTime, const MSDevice_Taxi* taxi,
    //                               const MSEdge* from, double fromPos,
    //                               const MSEdge* via, double viaPos,
    //                               const MSEdge* to, double toPos,
    //                               SUMOAbstractRouter<MSEdge, SUMOVehicle>& router,
    //                               double& timeDirect) {
    //     ConstMSEdgeVector edges;
    //     if (timeDirect < 0) {
    //         router.compute(from, fromPos, to, toPos, &taxi->getHolder(), t, edges, true);
    //         timeDirect = router.recomputeCostsPos(edges, &taxi->getHolder(), fromPos, toPos, t);
    //         edges.clear();
    //     }
    //
    //     router.compute(from, fromPos, via, viaPos, &taxi->getHolder(), t, edges, true);
    //     const double start = STEPS2TIME(t);
    //     const double leg1 = router.recomputeCostsPos(edges, &taxi->getHolder(), fromPos, viaPos, t);
    // #ifdef DEBUG_DETOUR
    //     std::cout << "        leg1=" << toString(edges) << " startPos=" << fromPos << " toPos=" << viaPos << " time=" << leg1 << "\n";
    // #endif
    //     const double wait = MAX2(0.0, STEPS2TIME(viaTime) - (start + leg1));
    //     edges.clear();
    //     const SUMOTime timeContinue = TIME2STEPS(start + leg1 + wait);
    //     router.compute(via, viaPos, to, toPos, &taxi->getHolder(), timeContinue, edges, true);
    //     const double leg2 = router.recomputeCostsPos(edges, &taxi->getHolder(), viaPos, toPos, timeContinue);
    //     const double timeDetour = leg1 + wait + leg2;
    // #ifdef DEBUG_DETOUR
    //     std::cout << "        leg2=" << toString(edges) << " startPos=" << viaPos << " toPos=" << toPos << " time=" << leg2 << "\n";
    //     std::cout << "    t=" << STEPS2TIME(t) << " vt=" << STEPS2TIME(viaTime)
    //               << " from=" << from->getID() << " to=" << to->getID() << " via=" << via->getID()
    //               << " direct=" << timeDirect << " detour=" << timeDetour << " wait=" << wait << "\n";
    // #endif
    //     return timeDetour;
    // }
    #[allow(unused_variables)]
    pub fn compute_detour_time(t: SUMOTime, viaTime: SUMOTime, taxi: Ptr<MSDevice_Taxi>, from: Ptr<MSEdge>, fromPos: f64, via: Ptr<MSEdge>, viaPos: f64, to: Ptr<MSEdge>, toPos: f64, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, timeDirect: &mut f64) -> f64 {
        let mut edges: Vec<Ptr<MSEdge>> = Vec::new();
        let holder = Ptr::from_mut(taxi.get_holder());
        if *timeDirect < 0.0 {
            router.compute_2(from, fromPos, to, toPos, holder, t, &mut edges, true);
            *timeDirect = router.recompute_costs_pos(&edges, holder, fromPos, toPos, t, Ptr::default());
            edges.clear();
        }

        router.compute_2(from, fromPos, via, viaPos, holder, t, &mut edges, true);
        let start = steps2time(t);
        let leg1 = router.recompute_costs_pos(&edges, holder, fromPos, viaPos, t, Ptr::default());
        // MAX2(0.0, x) with C++ semantics: a > b ? a : b
        let waitRaw = steps2time(viaTime) - (start + leg1);
        let wait = if 0.0 > waitRaw { 0.0 } else { waitRaw };
        edges.clear();
        let timeContinue = time2steps(start + leg1 + wait);
        router.compute_2(via, viaPos, to, toPos, holder, timeContinue, &mut edges, true);
        let leg2 = router.recompute_costs_pos(&edges, holder, viaPos, toPos, timeContinue, Ptr::default());
        let timeDetour = leg1 + wait + leg2;
        timeDetour
    }

    /// C++: `void MSDispatch::swappedRunning(const Reservation *, MSDevice_Taxi *)`
    // void
    // MSDispatch::swappedRunning(const Reservation* res, MSDevice_Taxi* taxi) {
    // #ifdef DEBUG_RESERVATION
    //     std::cout << SIMTIME << " swapped res=" << res->id << " old=" << myRunningReservations[res->group][res]->getID() << " new=" << taxi->getID() << "\n";
    // #endif
    //     myRunningReservations[res->group][res] = taxi;
    // }
    #[allow(unused_variables)]
    pub fn swapped_running(&mut self, res: Ptr<Reservation>, taxi: Ptr<MSDevice_Taxi>) {
        self.myRunningReservations.entry(res.group.clone()).or_default().insert(res.id.clone(), (res, taxi));
    }

    /// C++: `void MSDispatch::saveState(OutputDevice &, SUMOTime) const` [const, virtual]
    /// Saves the state of the device
    // void
    // MSDispatch::saveState(OutputDevice& out, SUMOTime nextDispatch) const {
    //     out.openTag(SUMO_TAG_DISPATCHER);
    //     out.writeAttr(SUMO_ATTR_NEXT, nextDispatch);
    //     out.writeAttr(SUMO_ATTR_COUNT, myReservationCount);
    //
    //     std::ostringstream internals;
    //     for (const auto& it : myRunningReservations) {
    //         for (const auto& item : it.second) {
    //             for (const MSTransportable* t : item.first->persons) {
    //                 internals << t->getID() << " " << item.first->id << " ";
    //             }
    //         }
    //     }
    //     for (const auto& it : myGroupReservations) {
    //         for (const Reservation* res : it.second) {
    //             for (const MSTransportable* t : res->persons) {
    //                 internals << t->getID() << " " << res->id << " ";
    //             }
    //         }
    //     }
    //     out.writeAttr(SUMO_ATTR_CUSTOMERS, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, nextDispatch: SUMOTime) {
        out.open_tag_id(SUMO_TAG_DISPATCHER);
        out.write_attr(SUMO_ATTR_NEXT, &nextDispatch);
        out.write_attr(SUMO_ATTR_COUNT, &self.myReservationCount);

        let mut internals = String::new();
        for running in self.myRunningReservations.values() {
            for (res, _) in running.values() {
                for t in res.persons.values() {
                    internals.push_str(&format!("{} {} ", t.get_id(), res.id));
                }
            }
        }
        for list in self.myGroupReservations.values() {
            for res in list {
                for t in res.persons.values() {
                    internals.push_str(&format!("{} {} ", t.get_id(), res.id));
                }
            }
        }
        out.write_attr(SUMO_ATTR_CUSTOMERS, &internals);
        out.close_tag("");
    }

    /// C++: `void MSDispatch::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the device from the given description
    // void
    // MSDispatch::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myReservationCount = attrs.get<int>(SUMO_ATTR_COUNT, "dispatcher", ok);
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_CUSTOMERS));
    //     std::string tID, rID;
    //     while (bis >> tID && bis >> rID) {
    //         myLoadedReservations[tID] = rID;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myReservationCount = attrs.get::<i32>(SUMO_ATTR_COUNT, "dispatcher", &mut ok, true);
        let state = attrs.get_string(SUMO_ATTR_CUSTOMERS);
        let mut bis = state.split_whitespace();
        while let Some(tID) = bis.next() {
            let Some(rID) = bis.next() else { break };
            self.myLoadedReservations.insert(tID.to_string(), rID.to_string());
        }
    }

    /// C++: `void MSDispatch::servedReservation(const Reservation *, MSDevice_Taxi *)`
    // void
    // MSDispatch::servedReservation(const Reservation* res, MSDevice_Taxi* taxi) {
    //     auto itR = myRunningReservations.find(res->group);
    //     if (itR != myRunningReservations.end() && itR->second.count(res) != 0) {
    // #ifdef DEBUG_RESERVATION
    //         std::cout << SIMTIME << " servedReservation res=" << res->id << " taxi=" << taxi->getID() << " (running)\n";
    // #endif
    //         return; // was redispatch
    //     }
    // #ifdef DEBUG_RESERVATION
    //     std::cout << SIMTIME << " servedReservation res=" << res->id << " taxi=" << taxi->getID() << "\n";
    // #endif
    //     auto it = myGroupReservations.find(res->group);
    //     if (it == myGroupReservations.end()) {
    //         throw ProcessError(TL("Inconsistent group reservations."));
    //     }
    //     auto it2 = std::find(it->second.begin(), it->second.end(), res);
    //     if (it2 == it->second.end()) {
    //         throw ProcessError(TL("Inconsistent group reservations (2)."));
    //     }
    //     myRunningReservations[res->group][res] = taxi;
    //     const_cast<Reservation*>(*it2)->state = Reservation::ASSIGNED;
    //     it->second.erase(it2);
    //     if (it->second.empty()) {
    //         myGroupReservations.erase(it);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn served_reservation(&mut self, res: Ptr<Reservation>, taxi: Ptr<MSDevice_Taxi>) {
        if self.myRunningReservations.get(&res.group).is_some_and(|m| m.contains_key(&res.id)) {
            return; // was redispatch
        }
        if !self.myGroupReservations.contains_key(&res.group) {
            // C++ throws ProcessError here; the skeleton signature has no error channel
            panic!("Inconsistent group reservations.");
        }
        let list = self.myGroupReservations.get_mut(&res.group).unwrap();
        let Some(idx) = list.iter().position(|r| *r == res) else {
            panic!("Inconsistent group reservations (2).");
        };
        self.myRunningReservations.entry(res.group.clone()).or_default().insert(res.id.clone(), (res, taxi));
        res.m().state = Reservation_ReservationState::ASSIGNED;
        let list = self.myGroupReservations.get_mut(&res.group).unwrap();
        list.remove(idx);
        let empty = list.is_empty();
        if empty {
            self.myGroupReservations.remove(&res.group);
        }
    }

    /// C++: `int MSDispatch::remainingCapacity(const MSDevice_Taxi *, const Reservation *)` [protected]
    /// whether the given taxi has sufficient capacity to serve the reservation
    // int
    // MSDispatch::remainingCapacity(const MSDevice_Taxi* taxi, const Reservation* res) {
    //     assert(res->persons.size() > 0);
    //     return ((*res->persons.begin())->isPerson()
    //             ? taxi->getHolder().getVehicleType().getPersonCapacity()
    //             : taxi->getHolder().getVehicleType().getContainerCapacity()) - (int)res->persons.size();
    // }
    #[allow(unused_variables)]
    pub fn remaining_capacity(&mut self, taxi: Ptr<MSDevice_Taxi>, res: Ptr<Reservation>) -> i32 {
        let holder = taxi.get_holder();
        // C++ asserts res->persons is non-empty and inspects the first person
        let isPerson = res.persons.values().next().is_some_and(|p| p.is_person());
        let capacity = if isPerson { holder.get_vehicle_type().get_person_capacity() } else { holder.get_vehicle_type().get_container_capacity() };
        capacity - res.persons.len() as i32
    }

}

/// C++ class `Reservation` (microsim/devices/MSDispatch.h)
pub struct Reservation {
    pub id: String,
    // C++: std::set<const MSTransportable *, ComparatorNumericalIdLess>; keyed by numerical id (set order)
    pub persons: BTreeMap<SUMOTrafficObject_NumericalID, Ptr<MSTransportable>>,
    pub reservationTime: SUMOTime,
    pub pickupTime: SUMOTime,
    pub earliestPickupTime: SUMOTime,
    pub from: Ptr<MSEdge>,
    pub fromPos: f64,
    pub fromStop: Ptr<MSStoppingPlace>,
    pub to: Ptr<MSEdge>,
    pub toPos: f64,
    pub toStop: Ptr<MSStoppingPlace>,
    pub group: String,
    pub line: String,
    pub recheck: SUMOTime,
    pub state: Reservation_ReservationState,
}

impl Reservation {
    /// C++: `void Reservation::Reservation(const std::string &, const std::vector<const MSTransportable *> &, SUMOTime, SUMOTime, SUMOTime, const MSEdge *, double, const MSStoppingPlace *, const MSEdge *, double, const MSStoppingPlace *, const std::string &, const std::string &)`
    // Reservation(const std::string& _id,
    //                 const std::vector<const MSTransportable*>& _persons,
    //                 SUMOTime _reservationTime,
    //                 SUMOTime _pickupTime,
    //                 SUMOTime _earliestPickupTime,
    //                 const MSEdge* _from, double _fromPos,
    //                 const MSStoppingPlace* _fromStop,
    //                 const MSEdge* _to, double _toPos,
    //                 const MSStoppingPlace* _toStop,
    //                 const std::string& _group,
    //                 const std::string& _line) :
    //         id(_id),
    //         persons(_persons.begin(), _persons.end()),
    //         reservationTime(_reservationTime),
    //         pickupTime(_pickupTime),
    //         earliestPickupTime(_earliestPickupTime),
    //         from(_from),
    //         fromPos(_fromPos),
    //         fromStop(_fromStop),
    //         to(_to),
    //         toPos(_toPos),
    //         toStop(_toStop),
    //         group(_group),
    //         line(_line),
    //         recheck(_reservationTime),
    //         state(NEW)
    //     {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _persons: &Vec<Ptr<MSTransportable>>, _reservationTime: SUMOTime, _pickupTime: SUMOTime, _earliestPickupTime: SUMOTime, _from: Ptr<MSEdge>, _fromPos: f64, _fromStop: Ptr<MSStoppingPlace>, _to: Ptr<MSEdge>, _toPos: f64, _toStop: Ptr<MSStoppingPlace>, _group: &str, _line: &str) -> Self {
        Self {
            id: _id.to_string(),
            persons: _persons.iter().map(|p| (p.get_numerical_id(), *p)).collect(),
            reservationTime: _reservationTime,
            pickupTime: _pickupTime,
            earliestPickupTime: _earliestPickupTime,
            from: _from,
            fromPos: _fromPos,
            fromStop: _fromStop,
            to: _to,
            toPos: _toPos,
            toStop: _toStop,
            group: _group.to_string(),
            line: _line.to_string(),
            recheck: _reservationTime,
            state: Reservation_ReservationState::NEW,
        }
    }

    /// C++: `bool Reservation::operator==(const Reservation &) const` [const]
    // bool operator==(const Reservation& other) const {
    //         return persons == other.persons
    //                && reservationTime == other.reservationTime
    //                && pickupTime == other.pickupTime
    //                && from == other.from
    //                && fromPos == other.fromPos
    //                && to == other.to
    //                && toPos == other.toPos
    //                && group == other.group
    //                && line == other.line;
    //     }
    #[allow(unused_variables)]
    pub fn eq_op(&self, other: &Reservation) -> bool {
        self.persons == other.persons
            && self.reservationTime == other.reservationTime
            && self.pickupTime == other.pickupTime
            && self.from == other.from
            && self.fromPos == other.fromPos
            && self.to == other.to
            && self.toPos == other.toPos
            && self.group == other.group
            && self.line == other.line
    }

    /// C++: `std::string Reservation::getID() const` [const]
    /// for sorting by id
    // std::string getID() const {
    //         return id;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

}

/// sorts reservations by time
/// C++ class `MSDispatch::time_sorter` (microsim/devices/MSDispatch.h)
pub struct MSDispatch_time_sorter {
}

impl MSDispatch_time_sorter {
    /// C++: `void time_sorter::time_sorter()`
    /// Constructor
    // explicit time_sorter() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSDispatch_time_sorter {}
    }

    /// C++: `int time_sorter::operator()(const Reservation *, const Reservation *) const` [const]
    /// Comparing operator
    // int operator()(const Reservation* r1, const Reservation* r2) const {
    //             return MAX2(r1->reservationTime, r1->earliestPickupTime) < MAX2(r2->reservationTime, r2->earliestPickupTime);
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, r1: Ptr<Reservation>, r2: Ptr<Reservation>) -> i32 {
        (std::cmp::max(r1.reservationTime, r1.earliestPickupTime) < std::cmp::max(r2.reservationTime, r2.earliestPickupTime)) as i32
    }

}

