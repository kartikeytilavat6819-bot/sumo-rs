//! Port of microsim/devices/MSDispatch_Greedy.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::SUMOTIME_MAX;

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

fn greedy_compute_dispatch(d: &mut MSDispatch, now: SUMOTime, fleet: &Vec<Ptr<MSDevice_Taxi>>) {
    // SAFETY: installed only by MSDispatch_Greedy::new, so d is the base of an MSDispatch_Greedy (repr(C), base first)
    let g = unsafe { &mut *(d as *mut MSDispatch as *mut MSDispatch_Greedy) };
    g.compute_dispatch(now, fleet);
}

pub(crate) const MS_DISPATCH_GREEDY_VT: MSDispatchVt = MSDispatchVt {
    compute_dispatch: greedy_compute_dispatch,
};

fn greedy_closest_compute_dispatch(d: &mut MSDispatch, now: SUMOTime, fleet: &Vec<Ptr<MSDevice_Taxi>>) {
    // SAFETY: installed only by MSDispatch_GreedyClosest::new, so d is the base of an MSDispatch_GreedyClosest (repr(C), base first)
    let g = unsafe { &mut *(d as *mut MSDispatch as *mut MSDispatch_GreedyClosest) };
    g.compute_dispatch(now, fleet);
}

pub(crate) const MS_DISPATCH_GREEDY_CLOSEST_VT: MSDispatchVt = MSDispatchVt {
    compute_dispatch: greedy_closest_compute_dispatch,
};

impl MSDispatch_Greedy {
    /// C++: `void MSDispatch_Greedy::MSDispatch_Greedy(const Parameterised::Map &)`
    // MSDispatch_Greedy(const Parameterised::Map& params) :
    //         MSDispatch(params),
    //         myMaximumWaitingTime(TIME2STEPS(StringUtils::toInt(getParameter("maxWaitingTime", "300")))),
    //         myRecheckTime(TIME2STEPS(StringUtils::toInt(getParameter("recheckTime", "120")))),
    //         myRecheckSafety(TIME2STEPS(StringUtils::toInt(getParameter("recheckSafety", "3600"))))
    //     { }
    #[allow(unused_variables)]
    pub fn new(params: &utils::common::parameterised::ParameterisedMap) -> Self {
        let mut base = MSDispatch::new(params);
        base.vt = &MS_DISPATCH_GREEDY_VT;
        let myMaximumWaitingTime = utils::common::time::time2steps(utils::common::string_utils::to_int(&base.base.get_parameter("maxWaitingTime", "300")).unwrap() as f64);
        let myRecheckTime = utils::common::time::time2steps(utils::common::string_utils::to_int(&base.base.get_parameter("recheckTime", "120")).unwrap() as f64);
        let myRecheckSafety = utils::common::time::time2steps(utils::common::string_utils::to_int(&base.base.get_parameter("recheckSafety", "3600")).unwrap() as f64);
        MSDispatch_Greedy {
            base,
            myMaximumWaitingTime,
            myRecheckTime,
            myRecheckSafety,
        }
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
        let mut numDispatched: i32 = 0;
        let mut numPostponed: i32 = 0;
        // find available vehicles
        // determinism: C++ std::set with ComparatorNumericalVehicleIdLess; keyed by the holder's numerical id so iteration order matches
        let mut available: BTreeMap<SUMOTrafficObject_NumericalID, Ptr<MSDevice_Taxi>> = BTreeMap::new();
        for taxi in fleet.iter() {
            if taxi.m().is_empty() {
                available.entry(taxi.myHolder.get_numerical_id()).or_insert(*taxi);
            }
        }
        // greedy assign closest vehicle in reservation order
        let router = Ptr::from_mut(self.get_router());
        let mut reservations: Vec<Ptr<Reservation>> = self.get_reservations();
        // determinism: C++ std::sort is unstable; std_sort reproduces libstdc++ tie order
        let sorter = MSDispatch_time_sorter::new();
        utils::common::std_sort::std_sort(reservations.as_mut_slice(), |a, b| sorter.call_op(a, b) != 0);
        let mut it: usize = 0;
        while it < reservations.len() {
            if available.len() == 0 {
                break;
            }
            let res = reservations[it];
            if res.recheck > now {
                it += 1;
                numPostponed += 1;
                continue;
            }
            //Position pos = res.from->getLanes().front()->geometryPositionAtOffset(res.fromPos);
            let mut closest: Ptr<MSDevice_Taxi> = Ptr::default();
            let mut closestTime: SUMOTime = SUMOTIME_MAX;
            let mut tooEarly = false;
            let mut unreachable = false;
            let mut unreachableDest = false;
            for &taxi in available.values() {
                if self.remaining_capacity(taxi, res) < 0 || !taxi.m().compatible_line(res) {
                    continue;
                }
                let travelTime = MSDispatch::compute_pickup_time(now, taxi, &*res, router.m());
                let destConnected = self.is_reachable(now, taxi, &*res, router.m());
                if travelTime < closestTime && destConnected {
                    closestTime = travelTime;
                    closest = taxi;
                    let taxiWait = res.pickupTime - (now + closestTime);
                    if taxiWait > self.myMaximumWaitingTime {
                        // no need to service this customer now
                        tooEarly = true;
                        res.m().recheck += std::cmp::max(now + self.myRecheckTime, res.pickupTime - closestTime - self.myRecheckSafety);
                        break;
                    }
                } else if travelTime == SUMOTIME_MAX {
                    unreachable = true;
                } else if !destConnected {
                    unreachableDest = true;
                }
            }
            if tooEarly || closest.is_null() {
                // too early or all taxis are too small
                let resWait = now - res.pickupTime;
                if (unreachable || unreachableDest) && resWait > self.myKeepUnreachableResTime {
                    reservations.remove(it);
                    let persons = res.persons.values().map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ");
                    if unreachable {
                        write_warningf!(
                            "Aborting reservation for customers '%' from '%' after waiting time % because no taxi can reach the pickup location, time=%.",
                            persons,
                            res.from.get_id(),
                            utils::common::time::time2string(resWait),
                            utils::common::time::time2string(MSNet::get_instance().get_current_time_step())
                        );
                    } else {
                        write_warningf!(
                            "Aborting reservation for customers '%' to '%' after waiting time % because no taxi can reach the dropoff location, time=%.",
                            persons,
                            res.from.get_id(),
                            utils::common::time::time2string(resWait),
                            utils::common::time::time2string(MSNet::get_instance().get_current_time_step())
                        );
                    }
                    let persons: Vec<Ptr<MSTransportable>> = res.persons.values().copied().collect();
                    for p in persons {
                        let _ = self.remove_reservation(p, res.from, res.fromPos, res.to, res.toPos, res.group.clone());
                    }
                } else {
                    it += 1;
                    numPostponed += 1;
                }
            } else {
                numDispatched += self.dispatch(closest, &mut it, router.m(), &mut reservations);
                available.remove(&closest.myHolder.get_numerical_id());
            }
        }
        // check if any taxis are able to service the remaining requests
        self.myHasServableReservations = reservations.len() > 0 && (available.len() < fleet.len() || numPostponed > 0 || numDispatched > 0);
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
    pub fn dispatch(&mut self, taxi: Ptr<MSDevice_Taxi>, resIt: &mut usize /* std::vector<Reservation *>::iterator, as an index */, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, reservations: &mut Vec<Ptr<Reservation>>) -> i32 {
        let res = reservations[*resIt];
        taxi.m().dispatch(&*res);
        self.served_reservation(res, taxi);
        reservations.remove(*resIt); // resIt = reservations.erase(resIt)
        1
    }

    /// C++: `MSDispatch_Greedy & MSDispatch_Greedy::operator=(const MSDispatch_Greedy &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDispatch_Greedy) -> &mut MSDispatch_Greedy {
        panic!("MSDispatch_Greedy cannot be assigned: the assignment operator is invalidated in C++")
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
    pub fn new(params: &utils::common::parameterised::ParameterisedMap) -> Self {
        let mut base = MSDispatch_Greedy::new(params);
        base.base.vt = &MS_DISPATCH_GREEDY_CLOSEST_VT;
        MSDispatch_GreedyClosest { base }
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
        let mut havePostponed = false;
        let mut numDispatched: i32 = 0;
        // find available vehicles
        // determinism: C++ std::set with ComparatorNumericalVehicleIdLess; keyed by the holder's numerical id so iteration order matches
        let mut available: BTreeMap<SUMOTrafficObject_NumericalID, Ptr<MSDevice_Taxi>> = BTreeMap::new();
        for taxi in fleet.iter() {
            if taxi.m().is_empty() {
                available.entry(taxi.myHolder.get_numerical_id()).or_insert(*taxi);
            }
        }
        // greedy assign closest vehicle
        let router = Ptr::from_mut(self.get_router());
        let mut activeReservations: Vec<Ptr<Reservation>> = Vec::new();
        for res in self.get_reservations() {
            if res.recheck <= now {
                activeReservations.push(res);
            }
        }
        while available.len() > 0 && activeReservations.len() > 0 {
            let mut closest: Ptr<Reservation> = Ptr::default();
            let mut closestTaxi: Ptr<MSDevice_Taxi> = Ptr::default();
            let mut closestTime: SUMOTime = SUMOTIME_MAX;
            for &res in activeReservations.iter() {
                let mut recheck: SUMOTime = SUMOTIME_MAX;
                for &taxi in available.values() {
                    if self.remaining_capacity(taxi, res) < 0 || !taxi.m().compatible_line(res) {
                        continue;
                    }
                    let travelTime = MSDispatch::compute_pickup_time(now, taxi, &*res, router.m());
                    // determinism: unreachable taxis give travelTime == SUMOTime_MAX; C++ wraps here and the value is unused, so wrap too
                    let taxiWait = res.pickupTime.wrapping_sub(now.wrapping_add(travelTime));
                    if travelTime < closestTime {
                        if taxiWait < self.myMaximumWaitingTime {
                            closestTime = travelTime;
                            closest = res;
                            closestTaxi = taxi;
                        } else {
                            recheck = std::cmp::min(recheck, std::cmp::max(now + self.myRecheckTime, res.pickupTime - closestTime - self.myRecheckSafety));
                        }
                    }
                }
                /*
                if (closestTaxi == nullptr) {
                    // all taxis would arrive to early. postpone recheck
                    res.recheck = recheck;
                }
                */
            }
            if !closestTaxi.is_null() {
                let mut closeIt: usize = activeReservations.iter().position(|r| *r == closest).expect("closest reservation is in activeReservations");
                numDispatched += self.dispatch(closestTaxi, &mut closeIt, router.m(), &mut activeReservations);
                available.remove(&closestTaxi.myHolder.get_numerical_id());
            } else {
                // all current reservations are too early or too big
                havePostponed = true;
                break;
            }
        }
        // check if any taxis are able to service the remaining requests
        self.myHasServableReservations = self.get_reservations().len() > 0 && (available.len() < fleet.len() || havePostponed || numDispatched > 0);
    }

}

