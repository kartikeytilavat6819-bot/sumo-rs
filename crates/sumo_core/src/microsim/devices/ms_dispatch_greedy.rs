//! Port of microsim/devices/MSDispatch_Greedy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A dispatch algorithm that services customers in reservation order and always sends the closest available taxi
/// C++ class `MSDispatch_Greedy` (microsim/devices/MSDispatch_Greedy.h)
#[repr(C)]
pub struct MSDispatch_Greedy {
    /// C++ base class `MSDispatch`
    pub base: MSDispatch,
    /// maximum time to arrive earlier at customer
    pub myMaximumWaitingTime: SUMOTime,
    /// recheck interval for early reservations
    pub myRecheckTime: SUMOTime,
    pub myRecheckSafety: SUMOTime,
}

impl std::ops::Deref for MSDispatch_Greedy {
    type Target = MSDispatch;
    fn deref(&self) -> &MSDispatch {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch_Greedy {
    fn deref_mut(&mut self) -> &mut MSDispatch {
        &mut self.base
    }
}

impl MSDispatch_Greedy {
    /// C++: `void MSDispatch_Greedy::MSDispatch_Greedy(const Parameterised::Map &)`
    // MSDispatch_Greedy(const Parameterised::Map& params) :
    //         MSDispatch(params),
    //         myMaximumWaitingTime(TIME2STEPS(StringUtils::toInt(getParameter("maxWaitingTime", "300")))),
    //         myRecheckTime(TIME2STEPS(StringUtils::toInt(getParameter("recheckTime", "120")))),
    //         myRecheckSafety(TIME2STEPS(StringUtils::toInt(getParameter("recheckSafety", "3600"))))
    //     { }
    #[allow(unused_variables)]
    pub fn new(params: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSDispatch_Greedy::MSDispatch_Greedy")
    }

    /// C++: `void MSDispatch_Greedy::computeDispatch(SUMOTime, const std::vector<MSDevice_Taxi *> &)` [virtual]
    // void
    // MSDispatch_Greedy::computeDispatch(SUMOTime now, const std::vector<MSDevice_Taxi*>& fleet) {
    //     int numDispatched = 0;
    //     int numPostponed = 0;
    //     // find available vehicles
    //     std::set<MSDevice_Taxi*, MSVehicleDevice::ComparatorNumericalVehicleIdLess> available;
    //     for (auto* taxi : fleet) {
    //         if (taxi->isEmpty()) {
    //             available.insert(taxi);
    //         }
    //     }
    //     // greedy assign closest vehicle in reservation order
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = getRouter();
    //     std::vector<Reservation*> reservations = getReservations();
    //     std::sort(reservations.begin(), reservations.end(), time_sorter());
    // #ifdef DEBUG_DISPATCH
    //     std::cout << SIMTIME << " computeDispatch fleet=" << fleet.size() << " available=" << available.size() << " reservations=" << toString(reservations) << "\n";
    // #endif
    //     for (auto it = reservations.begin(); it != reservations.end();) {
    //         if (available.size() == 0) {
    //             break;
    //         }
    //         Reservation* res = *it;
    //         if (res->recheck > now) {
    //             it++;
    //             numPostponed++;
    //             continue;
    //         }
    //         //Position pos = res.from->getLanes().front()->geometryPositionAtOffset(res.fromPos);
    //         MSDevice_Taxi* closest = nullptr;
    //         SUMOTime closestTime = SUMOTime_MAX;
    //         bool tooEarly = false;
    //         bool unreachable = false;
    //         bool unreachableDest = false;
    //         for (auto* taxi : available) {
    //             if (remainingCapacity(taxi, res) < 0 || !taxi->compatibleLine(res)) {
    //                 continue;
    //             }
    //             SUMOTime travelTime = computePickupTime(now, taxi, *res, router);
    //             const bool destConnected = isReachable(now, taxi, *res, router);
    // #ifdef DEBUG_TRAVELTIME
    //             if (DEBUG_COND2(res)) {
    //                 std::cout << SIMTIME << " taxi=" << taxi->getHolder().getID() << " person=" << toString(res->persons) << " traveltime=" << time2string(travelTime) << " reachable=" << destConnected << "\n";
    //             }
    // #endif
    //             if (travelTime < closestTime && destConnected) {
    //                 closestTime = travelTime;
    //                 closest = taxi;
    //                 SUMOTime taxiWait = res->pickupTime - (now + closestTime);
    //                 if (taxiWait > myMaximumWaitingTime) {
    //                     // no need to service this customer now
    //                     tooEarly = true;
    //                     res->recheck += MAX2(now + myRecheckTime, res->pickupTime - closestTime - myRecheckSafety);
    //                     break;
    //                 }
    //             } else if (travelTime == SUMOTime_MAX) {
    //                 unreachable = true;
    //             } else if (!destConnected) {
    //                 unreachableDest = true;
    //             }
    //         }
    //         if (tooEarly || closest == nullptr) {
    //             // too early or all taxis are too small
    //             SUMOTime resWait = now - res->pickupTime;
    //             if ((unreachable || unreachableDest) && resWait > myKeepUnreachableResTime) {
    //                 it = reservations.erase(it);
    //                 if (unreachable) {
    //                     WRITE_WARNINGF("Aborting reservation for customers '%' from '%' after waiting time % because no taxi can reach the pickup location, time=%.",
    //                                    toString(res->persons), res->from->getID(), time2string(resWait), time2string(SIMSTEP));
    //                 } else {
    //                     WRITE_WARNINGF("Aborting reservation for customers '%' to '%' after waiting time % because no taxi can reach the dropoff location, time=%.",
    //                                    toString(res->persons), res->from->getID(), time2string(resWait), time2string(SIMSTEP));
    //                 }
    //                 std::set<const MSTransportable*, ComparatorNumericalIdLess> persons = res->persons;
    //                 for (const MSTransportable* p : persons) {
    //                     removeReservation(const_cast<MSTransportable*>(p), res->from, res->fromPos, res->to, res->toPos, res->group);
    //                 }
    //             } else {
    //                 it++;
    //                 numPostponed++;
    //             }
    //         } else {
    //             numDispatched += dispatch(closest, it, router, reservations);
    //             available.erase(closest);
    //         }
    //     }
    //     // check if any taxis are able to service the remaining requests
    //     myHasServableReservations = reservations.size() > 0 && (available.size() < fleet.size() || numPostponed > 0 || numDispatched > 0);
    // #ifdef DEBUG_SERVABLE
    //     std::cout << SIMTIME << " reservations=" << reservations.size() << " avail=" << available.size()
    //               << " fleet=" << fleet.size() << " postponed=" << numPostponed << " dispatched=" << numDispatched << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_dispatch(&mut self, now: SUMOTime, fleet: &Vec<Ptr<MSDevice_Taxi>>) {
        todo!("port MSDispatch_Greedy::computeDispatch")
    }

    /// C++: `int MSDispatch_Greedy::dispatch(MSDevice_Taxi *, std::vector<Reservation *>::iterator &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, std::vector<Reservation *> &)` [virtual, protected]
    /// trigger taxi dispatch.
    // int
    // MSDispatch_Greedy::dispatch(MSDevice_Taxi* taxi, std::vector<Reservation*>::iterator& resIt, SUMOAbstractRouter<MSEdge, SUMOVehicle>& /*router*/, std::vector<Reservation*>& reservations) {
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND2(*resIt)) {
    //         std::cout << SIMTIME << " dispatch taxi=" << taxi->getHolder().getID() << " person=" << toString((*resIt)->persons) << "\n";
    //     }
    // #endif
    //     taxi->dispatch(**resIt);
    //     servedReservation(*resIt, taxi); // deleting res
    //     resIt = reservations.erase(resIt);
    //     return 1;
    // }
    #[allow(unused_variables)]
    pub fn dispatch(&mut self, taxi: Ptr<MSDevice_Taxi>, resIt: &mut Unmapped /* std::vector<Reservation *>::iterator */, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, reservations: &mut Vec<Ptr<Reservation>>) -> i32 {
        todo!("port MSDispatch_Greedy::dispatch")
    }

    /// C++: `MSDispatch_Greedy & MSDispatch_Greedy::operator=(const MSDispatch_Greedy &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDispatch_Greedy) -> &mut MSDispatch_Greedy {
        todo!("port MSDispatch_Greedy::operator=")
    }

}

/// A dispatch algorithm that services the reservations with the shortest traveltime-to-pickup first
/// C++ class `MSDispatch_GreedyClosest` (microsim/devices/MSDispatch_Greedy.h)
#[repr(C)]
pub struct MSDispatch_GreedyClosest {
    /// C++ base class `MSDispatch_Greedy`
    pub base: MSDispatch_Greedy,
}

impl std::ops::Deref for MSDispatch_GreedyClosest {
    type Target = MSDispatch_Greedy;
    fn deref(&self) -> &MSDispatch_Greedy {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch_GreedyClosest {
    fn deref_mut(&mut self) -> &mut MSDispatch_Greedy {
        &mut self.base
    }
}

impl MSDispatch_GreedyClosest {
    /// C++: `void MSDispatch_GreedyClosest::MSDispatch_GreedyClosest(const Parameterised::Map &)`
    // MSDispatch_GreedyClosest(const Parameterised::Map& params) :
    //         MSDispatch_Greedy(params)
    //     {}
    #[allow(unused_variables)]
    pub fn new(params: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSDispatch_GreedyClosest::MSDispatch_GreedyClosest")
    }

    /// C++: `void MSDispatch_GreedyClosest::computeDispatch(SUMOTime, const std::vector<MSDevice_Taxi *> &)` [virtual]
    // void
    // MSDispatch_GreedyClosest::computeDispatch(SUMOTime now, const std::vector<MSDevice_Taxi*>& fleet) {
    //     bool havePostponed = false;
    //     int numDispatched = 0;
    //     // find available vehicles
    //     std::set<MSDevice_Taxi*, MSVehicleDevice::ComparatorNumericalVehicleIdLess> available;
    //     for (auto* taxi : fleet) {
    //         if (taxi->isEmpty()) {
    //             available.insert(taxi);
    //         }
    //     }
    // #ifdef DEBUG_DISPATCH
    //     std::cout << SIMTIME << " computeDispatch fleet=" << fleet.size() << " available=" << available.size() << "\n";
    // #endif
    //     // greedy assign closest vehicle
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = myRoutingMode == 1 ? MSRoutingEngine::getRouterTT(0, SVC_TAXI) : MSNet::getInstance()->getRouterTT(0);
    //     std::vector<Reservation*> activeReservations;
    //     for (Reservation* res : getReservations()) {
    //         if (res->recheck <= now) {
    //             activeReservations.push_back(res);
    //         }
    //     }
    //     while (available.size() > 0 && activeReservations.size() > 0) {
    //         Reservation* closest = nullptr;
    //         MSDevice_Taxi* closestTaxi = nullptr;
    //         SUMOTime closestTime = SUMOTime_MAX;
    //         for (Reservation* res : activeReservations) {
    //             SUMOTime recheck = SUMOTime_MAX;
    //             for (auto* taxi : available) {
    //                 if (remainingCapacity(taxi, res) < 0 || !taxi->compatibleLine(res)) {
    //                     continue;
    //                 }
    //                 SUMOTime travelTime = computePickupTime(now, taxi, *res, router);
    //                 SUMOTime taxiWait = res->pickupTime - (now + travelTime);
    // #ifdef DEBUG_TRAVELTIME
    //                 if (DEBUG_COND2(res)) std::cout << SIMTIME << " taxi=" << taxi->getHolder().getID() << " person=" << toString(res->persons)
    //                                                        << " traveltime=" << time2string(travelTime)
    //                                                        << " pickupTime=" << time2string(res->pickupTime)
    //                                                        << " taxiWait=" << time2string(taxiWait) << "\n";
    // #endif
    //                 if (travelTime < closestTime) {
    //                     if (taxiWait < myMaximumWaitingTime) {
    //                         closestTime = travelTime;
    //                         closest = res;
    //                         closestTaxi = taxi;
    // #ifdef DEBUG_DISPATCH
    //                         if (DEBUG_COND2(res)) std::cout << SIMTIME << " bestTaxi=" << taxi->getHolder().getID() << " bestRes=" << toString(res->persons)
    //                                                                << " taxiPos=" << taxi->getHolder().getPositionOnLane() << " resFromPos=" << res->fromPos << " traveltime=" << time2string(travelTime) << " taxiWait=" << time2string(taxiWait) << "\n";
    // #endif
    //                     } else {
    //                         recheck = MIN2(recheck,
    //                                        MAX2(now + myRecheckTime, res->pickupTime - closestTime - myRecheckSafety));
    //                     }
    //                 }
    //             }
    //             /*
    //             if (closestTaxi == nullptr) {
    //                 // all taxis would arrive to early. postpone recheck
    //                 res.recheck = recheck;
    //             }
    //             */
    //         }
    //         if (closestTaxi != nullptr) {
    //             auto closeIt = std::find(activeReservations.begin(), activeReservations.end(), closest);
    //             numDispatched += dispatch(closestTaxi, closeIt, router, activeReservations);
    //             available.erase(closestTaxi);
    //         } else {
    //             // all current reservations are too early or too big
    //             havePostponed = true;
    //             break;
    //         }
    //     }
    //     // check if any taxis are able to service the remaining requests
    //     myHasServableReservations = getReservations().size() > 0 && (available.size() < fleet.size() || havePostponed || numDispatched > 0);
    // #ifdef DEBUG_SERVABLE
    //     std::cout << SIMTIME << " reservations=" << getReservations().size() << " avail=" << available.size()
    //               << " fleet=" << fleet.size() << " postponed=" << havePostponed << " dispatched=" << numDispatched << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn compute_dispatch(&mut self, now: SUMOTime, fleet: &Vec<Ptr<MSDevice_Taxi>>) {
        todo!("port MSDispatch_GreedyClosest::computeDispatch")
    }

}

