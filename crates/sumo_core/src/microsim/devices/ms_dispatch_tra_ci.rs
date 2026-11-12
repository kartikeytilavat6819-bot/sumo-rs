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
    pub fn new(params: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSDispatch_TraCI::MSDispatch_TraCI")
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
        todo!("port MSDispatch_TraCI::addReservation")
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
        todo!("port MSDispatch_TraCI::removeReservation")
    }

    /// C++: `void MSDispatch_TraCI::computeDispatch(SUMOTime, const std::vector<MSDevice_Taxi *> &)` [virtual]
    /// do nothing (dispatch happens via TraCI calls)
    // void computeDispatch(SUMOTime /*now*/, const std::vector<MSDevice_Taxi*>& /*fleet*/) override {}
    #[allow(unused_variables)]
    pub fn compute_dispatch(&mut self, arg0: SUMOTime, arg1: &Vec<Ptr<MSDevice_Taxi>>) {
        todo!("port MSDispatch_TraCI::computeDispatch")
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
        todo!("port MSDispatch_TraCI::interpretDispatch")
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
        todo!("port MSDispatch_TraCI::splitReservation")
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
        todo!("port MSDispatch_TraCI::fulfilledReservation")
    }

    /// C++: `MSDispatch_TraCI & MSDispatch_TraCI::operator=(const MSDispatch_TraCI &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDispatch_TraCI) -> &mut MSDispatch_TraCI {
        todo!("port MSDispatch_TraCI::operator=")
    }

}

