//! Port of microsim/devices/MSDispatch_RouteExtension.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::pair<const MSEdge *, double>> MSDispatch_RouteExtension::EdgePosVector`
pub type MSDispatch_RouteExtension_EdgePosVector = Vec<(Ptr<MSEdge>, f64)>;

/// A dispatch algorithm that services customers in reservation order but also tries to do ride sharing by extending or reusing the existing route
/// C++ class `MSDispatch_RouteExtension` (microsim/devices/MSDispatch_RouteExtension.h)
#[repr(C)]
pub struct MSDispatch_RouteExtension {
    /// C++ base class `MSDispatch_Greedy`
    pub base: MSDispatch_Greedy,
}

impl std::ops::Deref for MSDispatch_RouteExtension {
    type Target = MSDispatch_Greedy;
    fn deref(&self) -> &MSDispatch_Greedy {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch_RouteExtension {
    fn deref_mut(&mut self) -> &mut MSDispatch_Greedy {
        &mut self.base
    }
}

impl MSDispatch_RouteExtension {
    /// C++: `void MSDispatch_RouteExtension::MSDispatch_RouteExtension(const Parameterised::Map &)`
    // MSDispatch_RouteExtension(const Parameterised::Map& params) :
    //         MSDispatch_Greedy(params)
    //     {}
    #[allow(unused_variables)]
    pub fn new(params: &utils::common::parameterised::ParameterisedMap) -> Self {
        MSDispatch_RouteExtension {
            base: MSDispatch_Greedy::new(params),
        }
    }

    /// C++: `int MSDispatch_RouteExtension::dispatch(MSDevice_Taxi *, std::vector<Reservation *>::iterator &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, std::vector<Reservation *> &)` [virtual, protected]
    /// trigger taxi dispatch.
    // int
    // MSDispatch_RouteExtension::dispatch(MSDevice_Taxi* taxi, std::vector<Reservation*>::iterator& resIt, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router, std::vector<Reservation*>& reservations) {
    //     const Reservation* const res = *resIt;
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND2(person)) {
    //         std::cout << SIMTIME << " dispatch taxi=" << taxi->getHolder().getID() << " person=" << toString(res->persons) << "\n";
    //     }
    // #endif
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     const bool isPerson = (*res->persons.begin())->isPerson();
    //     int capacityLeft = remainingCapacity(taxi, res);
    //     std::vector<const Reservation*> sequence{ res, res };
    //     std::vector<const Reservation*> toRemove{ res };
    //     EdgePosVector posSequence({ std::make_pair(res->from, res->fromPos), std::make_pair(res->to, res->toPos) });
    //     const Reservation* first = sequence.front();
    //     const Reservation* last = sequence.back();
    //     ConstMSEdgeVector route;
    //     router.compute(first->from, first->fromPos, last->to, last->toPos, &taxi->getHolder(), MAX2(now, first->pickupTime), route);
    //     // check whether the ride can be shared
    //     for (auto it2 = resIt + 1; it2 != reservations.end();) {
    //         Reservation* const res2 = *it2;
    //         const bool isPerson2 = (*res2->persons.begin())->isPerson();
    //         if (capacityLeft < (int)res2->persons.size() || isPerson != isPerson2 || !taxi->compatibleLine(res2)) {
    //             it2++;
    //             continue;
    //         }
    //         // check whether res2 picks up or gets picked up on the way
    //         ConstMSEdgeVector route2;
    //         // TODO It may be more efficient to check first whether from and to are already in the route
    //         router.compute(res2->from, res2->fromPos, res2->to, res2->toPos, &taxi->getHolder(), MAX2(now, res2->pickupTime), route2);
    //         const bool pickup = std::find(route.begin(), route.end(), res2->from) != route.end();
    //         const bool dropoff = std::find(route.begin(), route.end(), res2->to) != route.end();
    //         const bool pickup2 = std::find(route2.begin(), route2.end(), first->from) != route2.end();
    //         const bool dropoff2 = std::find(route2.begin(), route2.end(), last->to) != route2.end();
    // #ifdef DEBUG_DISPATCH
    //         if (DEBUG_COND2(person)) std::cout << "  consider sharing ride with " << toString(res2->persons)
    //                                                << " from=" << res2->from->getID() << ":" << res2->fromPos << " to=" << res2->to->getID() << ":" << res2->toPos
    //                                                << " pickup=" << pickup << " startFirst=" << pickup2
    //                                                << " dropoff=" << dropoff << " endLast=" << dropoff2
    //                                                << "\n";
    // #endif
    //         if ((pickup || pickup2) && (dropoff || dropoff2)) {
    //             std::vector<const Reservation*>::iterator resSeqIt = sequence.begin();
    //             EdgePosVector::iterator edgeIt = posSequence.begin();
    //             if (pickup) {
    //                 // new reservation gets picked up
    //                 findInsertionPoint(resSeqIt, edgeIt, posSequence.end(), route, res2->from, res2->fromPos);
    //             }
    //             resSeqIt = sequence.insert(resSeqIt, res2) + 1;
    //             edgeIt = posSequence.insert(edgeIt, std::make_pair(res2->from, res2->fromPos)) + 1;
    //             if (dropoff) {
    //                 // new reservation drops off and route continues
    //                 findInsertionPoint(resSeqIt, edgeIt, posSequence.end(), route, res2->to, res2->toPos);
    //                 sequence.insert(resSeqIt, res2);
    //                 posSequence.insert(edgeIt, std::make_pair(res2->to, res2->toPos));
    //             } else {
    //                 // new reservation ends last
    //                 sequence.push_back(res2);
    //                 posSequence.push_back(std::make_pair(res2->to, res2->toPos));
    //             }
    //             toRemove.push_back(res2);
    //             it2 = reservations.erase(it2); // (resIt before it2) stays valid
    //             // TODO we have more capacity if some pickup is after an earlier dropoff
    //             capacityLeft -= (int)res2->persons.size();
    //             if (capacityLeft == 0) {
    //                 break;
    //             }
    //             route.clear();
    //             first = sequence.front();
    //             last = sequence.back();
    //             // TODO this is wrong for non linear networks! should be reusing the route snippets from above
    //             router.compute(first->from, first->fromPos, last->to, last->toPos, &taxi->getHolder(), MAX2(now, first->pickupTime), route);
    //         } else {
    //             it2++;
    //         }
    //     }
    //     if (sequence.size() > 2) {
    //         taxi->dispatchShared(sequence);
    //         if (myOutput != nullptr) {
    //             std::vector<const MSTransportable*> sharing;
    //             for (const Reservation* s : sequence) {
    //                 sharing.insert(sharing.end(), s->persons.begin(), s->persons.end());
    //             }
    //             myOutput->writeXMLHeader("DispatchInfo_RouteExtension", "");
    //             myOutput->openTag("dispatchShared");
    //             myOutput->writeAttr("time", time2string(now));
    //             myOutput->writeAttr("id", taxi->getHolder().getID());
    //             myOutput->writeAttr("persons", toString(res->persons));
    //             myOutput->writeAttr("sharingPersons", toString(sharing));
    //             myOutput->writeAttr("type", "routeExtension");
    //             myOutput->closeTag();
    //         }
    // #ifdef DEBUG_DISPATCH
    //         if (DEBUG_COND2(person)) std::cout << "  sharing ride with " << toString(sequence)
    //                                                << "\n";
    // #endif
    //     } else {
    //         taxi->dispatch(*res);
    //     }
    //     for (const Reservation* r : toRemove) {
    //         servedReservation(r, taxi); // deleting r
    //     }
    //     resIt = reservations.erase(resIt);
    //     return (int)toRemove.size();
    // }
    #[allow(unused_variables)]
    pub fn dispatch(&mut self, taxi: Ptr<MSDevice_Taxi>, resIt: &mut usize /* std::vector<Reservation *>::iterator, as an index */, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, reservations: &mut Vec<Ptr<Reservation>>) -> i32 {
        let res: Ptr<Reservation> = reservations[*resIt];
        let now: SUMOTime = MSNet::get_instance().get_current_time_step();
        let isPerson = res.persons.values().next().unwrap().is_person();
        let mut capacityLeft: i32 = self.remaining_capacity(taxi, res);
        let mut sequence: Vec<Ptr<Reservation>> = vec![res, res];
        let mut toRemove: Vec<Ptr<Reservation>> = vec![res];
        let mut posSequence: MSDispatch_RouteExtension_EdgePosVector = vec![(res.from, res.fromPos), (res.to, res.toPos)];
        let mut first: Ptr<Reservation> = *sequence.first().unwrap();
        let mut last: Ptr<Reservation> = *sequence.last().unwrap();
        let mut route: ConstMSEdgeVector = Vec::new();
        router.compute_2(first.from, first.fromPos, last.to, last.toPos, taxi.myHolder, std::cmp::max(now, first.pickupTime), &mut route, false);
        // check whether the ride can be shared
        let mut it2: usize = *resIt + 1;
        while it2 < reservations.len() {
            let res2: Ptr<Reservation> = reservations[it2];
            let isPerson2 = res2.persons.values().next().unwrap().is_person();
            if capacityLeft < res2.persons.len() as i32 || isPerson != isPerson2 || !taxi.m().compatible_line(res2) {
                it2 += 1;
                continue;
            }
            // check whether res2 picks up or gets picked up on the way
            let mut route2: ConstMSEdgeVector = Vec::new();
            // TODO It may be more efficient to check first whether from and to are already in the route
            router.compute_2(res2.from, res2.fromPos, res2.to, res2.toPos, taxi.myHolder, std::cmp::max(now, res2.pickupTime), &mut route2, false);
            let pickup = route.iter().any(|e| *e == res2.from);
            let dropoff = route.iter().any(|e| *e == res2.to);
            let pickup2 = route2.iter().any(|e| *e == first.from);
            let dropoff2 = route2.iter().any(|e| *e == last.to);

            if (pickup || pickup2) && (dropoff || dropoff2) {
                let mut resSeqIt: usize = 0;
                let mut edgeIt: usize = 0;
                if pickup {
                    // new reservation gets picked up
                    let edgeEnd: usize = posSequence.len();
                    self.find_insertion_point(&mut resSeqIt, &mut edgeIt, &edgeEnd, &posSequence, &mut route, res2.from, res2.fromPos);
                }
                sequence.insert(resSeqIt, res2);
                resSeqIt += 1;
                posSequence.insert(edgeIt, (res2.from, res2.fromPos));
                edgeIt += 1;
                if dropoff {
                    // new reservation drops off and route continues
                    let edgeEnd: usize = posSequence.len();
                    self.find_insertion_point(&mut resSeqIt, &mut edgeIt, &edgeEnd, &posSequence, &mut route, res2.to, res2.toPos);
                    sequence.insert(resSeqIt, res2);
                    posSequence.insert(edgeIt, (res2.to, res2.toPos));
                } else {
                    // new reservation ends last
                    sequence.push(res2);
                    posSequence.push((res2.to, res2.toPos));
                }
                toRemove.push(res2);
                reservations.remove(it2); // it2 = reservations.erase(it2); (resIt before it2 stays valid)
                // TODO we have more capacity if some pickup is after an earlier dropoff
                capacityLeft -= res2.persons.len() as i32;
                if capacityLeft == 0 {
                    break;
                }
                route.clear();
                first = *sequence.first().unwrap();
                last = *sequence.last().unwrap();
                // TODO this is wrong for non linear networks! should be reusing the route snippets from above
                router.compute_2(first.from, first.fromPos, last.to, last.toPos, taxi.myHolder, std::cmp::max(now, first.pickupTime), &mut route, false);
            } else {
                it2 += 1;
            }
        }
        if sequence.len() > 2 {
            taxi.m().dispatch_shared(sequence.clone());
            if !self.myOutput.is_null() {
                let mut sharing: Vec<Ptr<MSTransportable>> = Vec::new();
                for s in sequence.iter() {
                    sharing.extend(s.persons.values().copied());
                }
                let out = self.myOutput.m();
                out.write_xml_header("DispatchInfo_RouteExtension", "", BTreeMap::new(), true);
                out.open_tag("dispatchShared");
                out.write_attr("time", utils::common::time::time2string(now).as_str());
                out.write_attr("id", taxi.myHolder.get_id());
                let persons = res.persons.values().map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ");
                out.write_attr("persons", persons.as_str());
                let sharingPersons = sharing.iter().map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ");
                out.write_attr("sharingPersons", sharingPersons.as_str());
                out.write_attr("type", "routeExtension");
                out.close_tag("");
            }
        } else {
            taxi.m().dispatch(&*res);
        }
        for r in toRemove.iter() {
            self.served_reservation(*r, taxi); // deleting r
        }
        reservations.remove(*resIt); // resIt = reservations.erase(resIt)
        toRemove.len() as i32
    }

    /// C++: `void MSDispatch_RouteExtension::findInsertionPoint(std::vector<const Reservation *>::iterator &, EdgePosVector::iterator &, const EdgePosVector::iterator &, ConstMSEdgeVector &, const MSEdge *, const double) const` [const, private]
    // void
    // MSDispatch_RouteExtension::findInsertionPoint(std::vector<const Reservation*>::iterator& resIt, EdgePosVector::iterator& edgeIt,
    //         const EdgePosVector::iterator& edgeEnd, ConstMSEdgeVector& route,
    //         const MSEdge* newEdge, const double newPos) const {
    //     for (const MSEdge* edge : route) {
    //         while (edgeIt != edgeEnd && edge == edgeIt->first) {
    //             if (edge == newEdge && edgeIt->second > newPos) {
    //                 break;
    //             }
    //             resIt++;
    //             edgeIt++;
    //         }
    //         if (edge == newEdge) {
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_insertion_point(&self, resIt: &mut usize /* std::vector<const Reservation *>::iterator, as an index */, edgeIt: &mut usize /* EdgePosVector::iterator, as an index */, edgeEnd: &usize /* EdgePosVector::iterator, as an index */, posSequence: &MSDispatch_RouteExtension_EdgePosVector /* added: the C++ iterators point into it */, route: &mut ConstMSEdgeVector, newEdge: Ptr<MSEdge>, newPos: f64) {
        for edge in route.iter() {
            while *edgeIt != *edgeEnd && *edge == posSequence[*edgeIt].0 {
                if *edge == newEdge && posSequence[*edgeIt].1 > newPos {
                    break;
                }
                *resIt += 1;
                *edgeIt += 1;
            }
            if *edge == newEdge {
                break;
            }
        }
    }

}

