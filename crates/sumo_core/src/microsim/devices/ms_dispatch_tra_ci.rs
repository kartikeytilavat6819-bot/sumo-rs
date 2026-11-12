//! Port of microsim/devices/MSDispatch_TraCI.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A dispatch algorithm that services customers in reservation order and always sends the closest available taxi
/// C++ class `MSDispatch_TraCI` (microsim/devices/MSDispatch_TraCI.h)
#[repr(C)]
pub struct MSDispatch_TraCI {
    /// C++ base class `MSDispatch`
    pub base: MSDispatch,
    pub myReservationLookup: StringBijection<Ptr<Reservation>>,
}

impl std::ops::Deref for MSDispatch_TraCI {
    type Target = MSDispatch;
    fn deref(&self) -> &MSDispatch {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch_TraCI {
    fn deref_mut(&mut self) -> &mut MSDispatch {
        &mut self.base
    }
}

impl MSDispatch_TraCI {
    /// C++: `void MSDispatch_TraCI::MSDispatch_TraCI(const Parameterised::Map &)`
    // MSDispatch_TraCI(const Parameterised::Map& params) :
    //         MSDispatch(params)
    //     { }
    #[allow(unused_variables)]
    pub fn new(params: &utils::common::parameterised::ParameterisedMap) -> Self {
        Self {
            base: MSDispatch::new(params),
            myReservationLookup: StringBijection::default(),
        }
    }

    /// C++: `Reservation * MSDispatch_TraCI::addReservation(MSTransportable *, SUMOTime, SUMOTime, SUMOTime, const MSEdge *, double, const MSStoppingPlace *, const MSEdge *, double, const MSStoppingPlace *, std::string, const std::string &, int, int)` [virtual]
    /// add a new reservation
    // Reservation*
    // MSDispatch_TraCI::addReservation(MSTransportable* person,
    //                                  SUMOTime reservationTime,
    //                                  SUMOTime pickupTime,
    //                                  SUMOTime earliestPickupTime,
    //                                  const MSEdge* from, double fromPos,
    //                                  const MSStoppingPlace* fromStop,
    //                                  const MSEdge* to, double toPos,
    //                                  const MSStoppingPlace* toStop,
    //                                  std::string group,
    //                                  const std::string& line,
    //                                  int maxCapacity,
    //                                  int maxContainerCapacity) {
    //     Reservation* res = MSDispatch::addReservation(person, reservationTime, pickupTime, earliestPickupTime, from, fromPos, fromStop, to, toPos, toStop, group, line, maxCapacity, maxContainerCapacity);
    //     if (!myReservationLookup.has(res)) {
    //         myReservationLookup.insert(res->id, res);
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn add_reservation(&mut self, person: Ptr<MSTransportable>, reservationTime: SUMOTime, pickupTime: SUMOTime, earliestPickupTime: SUMOTime, from: Ptr<MSEdge>, fromPos: f64, fromStop: Ptr<MSStoppingPlace>, to: Ptr<MSEdge>, toPos: f64, toStop: Ptr<MSStoppingPlace>, group: String, line: &str, maxCapacity: i32, maxContainerCapacity: i32) -> Ptr<Reservation> {
        let res = self.base.add_reservation(person, reservationTime, pickupTime, earliestPickupTime, from, fromPos, fromStop, to, toPos, toStop, group, line, maxCapacity, maxContainerCapacity);
        if !self.myReservationLookup.has(res) {
            self.myReservationLookup.insert(&res.id, res, true).unwrap_or_else(|e| std::panic::panic_any(e));
        }
        res
    }

    /// C++: `std::string MSDispatch_TraCI::removeReservation(MSTransportable *, const MSEdge *, double, const MSEdge *, double, std::string)` [virtual]
    /// remove person from reservation. If the whole reservation is removed, return its id
    // std::string
    // MSDispatch_TraCI::removeReservation(MSTransportable* person,
    //                                     const MSEdge* from, double fromPos,
    //                                     const MSEdge* to, double toPos,
    //                                     std::string group) {
    //     const std::string removedID = MSDispatch::removeReservation(person, from, fromPos, to, toPos, group);
    //     if (myReservationLookup.hasString(removedID)) {
    //         // warning! res is already deleted
    //         const Reservation* res = myReservationLookup.get(removedID);
    //         myReservationLookup.remove(removedID, res);
    //     }
    //     return removedID;
    // }
    #[allow(unused_variables)]
    pub fn remove_reservation(&mut self, person: Ptr<MSTransportable>, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, group: String) -> String {
        let removedID = self.base.remove_reservation(person, from, fromPos, to, toPos, group);
        if self.myReservationLookup.has_string(&removedID) {
            // warning! res is already deleted
            let res = self.myReservationLookup.get(&removedID).unwrap_or_else(|e| std::panic::panic_any(e));
            self.myReservationLookup.remove(&removedID, res);
        }
        removedID
    }

    /// C++: `void MSDispatch_TraCI::computeDispatch(SUMOTime, const std::vector<MSDevice_Taxi *> &)` [virtual]
    /// do nothing (dispatch happens via TraCI calls)
    // void computeDispatch(SUMOTime /*now*/, const std::vector<MSDevice_Taxi*>& /*fleet*/) override {}
    #[allow(unused_variables)]
    pub fn compute_dispatch(&mut self, arg0: SUMOTime, arg1: &Vec<Ptr<MSDevice_Taxi>>) {
        // do nothing (dispatch happens via TraCI calls)
    }

    /// C++: `void MSDispatch_TraCI::interpretDispatch(MSDevice_Taxi *, const std::vector<std::string> &)`
    /// trigger taxi dispatch.
    // void
    // MSDispatch_TraCI::interpretDispatch(MSDevice_Taxi* taxi, const std::vector<std::string>& reservationsIDs) {
    //     std::vector<const Reservation*> reservations;
    //     for (std::string resID : reservationsIDs) {
    //         if (myReservationLookup.hasString(resID)) {
    //             reservations.push_back(myReservationLookup.get(resID));
    //         } else {
    //             throw InvalidArgument("Reservation id '" + resID + "' is not known");
    //         }
    //     }
    //     try {
    //         if (reservations.size() == 1) {
    //             taxi->dispatch(*reservations.front());
    //         } else {
    //             taxi->dispatchShared(reservations);
    //         }
    //     } catch (ProcessError& e) {
    //         throw InvalidArgument(e.what());
    //     }
    //     // in case of ride sharing the same reservation may occur multiple times
    //     std::set<const Reservation*> unique(reservations.begin(), reservations.end());
    //     for (const Reservation* res : unique) {
    //         servedReservation(res, taxi);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn interpret_dispatch(&mut self, taxi: Ptr<MSDevice_Taxi>, reservationsIDs: &Vec<String>) {
        let mut reservations: Vec<Ptr<Reservation>> = Vec::new();
        for resID in reservationsIDs.iter() {
            if self.myReservationLookup.has_string(resID) {
                reservations.push(self.myReservationLookup.get(resID).unwrap_or_else(|e| std::panic::panic_any(e)));
            } else {
                std::panic::panic_any(ProcessError::InvalidArgument(format!("Reservation id '{}' is not known", resID)));
            }
        }
        // shortcut: C++ catches ProcessError from dispatch and rethrows InvalidArgument; the Rust dispatch reports its errors directly
        if reservations.len() == 1 {
            taxi.m().dispatch(&*reservations[0]);
        } else {
            taxi.m().dispatch_shared(reservations.clone());
        }
        // in case of ride sharing the same reservation may occur multiple times
        // determinism: std::set<const Reservation*> orders by pointer, BTreeSet<Ptr<_>> orders by address the same way
        let unique: BTreeSet<Ptr<Reservation>> = reservations.iter().cloned().collect();
        for res in unique.iter() {
            self.served_reservation(*res, taxi);
        }
    }

    /// C++: `std::string MSDispatch_TraCI::splitReservation(std::string, std::vector<std::string>)`
    /// split existing reservations and return the new reservation id
    // std::string
    // MSDispatch_TraCI::splitReservation(std::string resID, std::vector<std::string> personIDs) {
    //     if (myReservationLookup.hasString(resID)) {
    //         Reservation* res = const_cast<Reservation*>(myReservationLookup.get(resID));
    //         if (myRunningReservations.count(res->group) != 0 && myRunningReservations[res->group].count(res) != 0) {
    //             throw InvalidArgument("Cannot split reservation '" + resID + "' after dispatch");
    //         }
    //         std::set<std::string> allPersons;
    //         for (const MSTransportable* t : res->persons) {
    //             allPersons.insert(t->getID());
    //         }
    //         for (std::string p : personIDs) {
    //             if (allPersons.count(p) == 0) {
    //                 throw InvalidArgument("Person '" + p + "' is not part of reservation '" + resID + "'");
    //             }
    //         }
    //         if (personIDs.size() == allPersons.size()) {
    //             throw InvalidArgument("Cannot remove all person from reservation '" + resID + "'");
    //         }
    //         std::vector<const MSTransportable*> split;
    //         for (const std::string& p : personIDs) {
    //             for (const MSTransportable* const t : res->persons) {
    //                 if (t->getID() == p) {
    //                     res->persons.erase(t);
    //                     split.push_back(t);
    //                     break;
    //                 }
    //             }
    //         }
    //         Reservation* newRes = new Reservation(toString(myReservationCount++), split,
    //                                               res->reservationTime, res->pickupTime,
    //                                               res->earliestPickupTime,
    //                                               res->from, res->fromPos, res->fromStop,
    //                                               res->to, res->toPos, res->toStop,
    //                                               res->group, res->line);
    //         myGroupReservations[res->group].push_back(newRes);
    //         myReservationLookup.insert(newRes->id, newRes);
    //         return newRes->id;
    //     } else {
    //         throw InvalidArgument("Reservation id '" + resID + "' is not known");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn split_reservation(&mut self, resID: String, personIDs: Vec<String>) -> String {
        if self.myReservationLookup.has_string(&resID) {
            let res: Ptr<Reservation> = self.myReservationLookup.get(&resID).unwrap_or_else(|e| std::panic::panic_any(e));
            if self.base.myRunningReservations.get(&res.group).map_or(false, |m| m.contains_key(&res.id)) {
                std::panic::panic_any(ProcessError::InvalidArgument(format!("Cannot split reservation '{}' after dispatch", resID)));
            }
            let mut allPersons: BTreeSet<String> = BTreeSet::new();
            for t in res.persons.values() {
                allPersons.insert(t.get_id().to_string());
            }
            for p in personIDs.iter() {
                if !allPersons.contains(p) {
                    std::panic::panic_any(ProcessError::InvalidArgument(format!("Person '{}' is not part of reservation '{}'", p, resID)));
                }
            }
            if personIDs.len() == allPersons.len() {
                std::panic::panic_any(ProcessError::InvalidArgument(format!("Cannot remove all person from reservation '{}'", resID)));
            }
            let mut split: Vec<Ptr<MSTransportable>> = Vec::new();
            for p in personIDs.iter() {
                let key = res.persons.iter().find(|(_, t)| t.get_id() == p.as_str()).map(|(k, _)| *k);
                if let Some(k) = key {
                    if let Some(t) = res.m().persons.remove(&k) {
                        split.push(t);
                    }
                }
            }
            let newId = self.myReservationCount.to_string();
            self.myReservationCount += 1;
            let newRes: Ptr<Reservation> = Ptr::from_mut(Box::leak(Box::new(Reservation::new(
                &newId,
                &split,
                res.reservationTime,
                res.pickupTime,
                res.earliestPickupTime,
                res.from,
                res.fromPos,
                res.fromStop,
                res.to,
                res.toPos,
                res.toStop,
                &res.group,
                &res.line,
            ))));
            self.base.myGroupReservations.entry(res.group.clone()).or_default().push(newRes);
            self.myReservationLookup.insert(&newRes.id, newRes, true).unwrap_or_else(|e| std::panic::panic_any(e));
            newRes.id.clone()
        } else {
            std::panic::panic_any(ProcessError::InvalidArgument(format!("Reservation id '{}' is not known", resID)));
        }
    }

    /// C++: `void MSDispatch_TraCI::fulfilledReservation(const Reservation *)` [virtual]
    /// erase reservation from storage
    // void
    // MSDispatch_TraCI::fulfilledReservation(const Reservation* res) {
    //     myReservationLookup.remove(res->id, res);
    //     MSDispatch::fulfilledReservation(res);
    //     myHasServableReservations = myReservationLookup.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn fulfilled_reservation(&mut self, res: Ptr<Reservation>) {
        self.myReservationLookup.remove(&res.id, res);
        self.base.fulfilled_reservation(res);
        self.base.myHasServableReservations = self.myReservationLookup.size() > 0;
    }

    /// C++: `MSDispatch_TraCI & MSDispatch_TraCI::operator=(const MSDispatch_TraCI &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDispatch_TraCI) -> &mut MSDispatch_TraCI {
        panic!("MSDispatch_TraCI assignment operator is private and undefined in C++")
    }

}

