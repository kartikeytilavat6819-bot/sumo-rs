//! Port of microsim/devices/MSDispatch_GreedyShared.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::time2string;

/// A dispatch algorithm that services customers in reservation order but also tries to do ride sharing
/// C++ class `MSDispatch_GreedyShared` (microsim/devices/MSDispatch_GreedyShared.h)
#[repr(C)]
pub struct MSDispatch_GreedyShared {
    /// C++ base class `MSDispatch_Greedy`
    pub base: MSDispatch_Greedy,
    /// absolute time threshold for declining shared ride in s
    pub myAbsoluteLossThreshold: f64,
    /// relative time threshold for declining shared ride
    pub myRelativeLossThreshold: f64,
}

impl std::ops::Deref for MSDispatch_GreedyShared {
    type Target = MSDispatch_Greedy;
    fn deref(&self) -> &MSDispatch_Greedy {
        &self.base
    }
}
impl std::ops::DerefMut for MSDispatch_GreedyShared {
    fn deref_mut(&mut self) -> &mut MSDispatch_Greedy {
        &mut self.base
    }
}

/// C++ virtual `dispatch()` override — installs the ride-sharing body for the base `computeDispatch`.
fn greedy_shared_dispatch(g: &mut MSDispatch_Greedy, taxi: Ptr<MSDevice_Taxi>, resIt: &mut usize, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, reservations: &mut Vec<Ptr<Reservation>>) -> i32 {
    // SAFETY: installed only by MSDispatch_GreedyShared::new; g is the MSDispatch_Greedy base of an MSDispatch_GreedyShared (repr(C), base first)
    let shared = unsafe { &mut *(g as *mut MSDispatch_Greedy as *mut MSDispatch_GreedyShared) };
    shared.dispatch(taxi, resIt, router, reservations)
}

impl MSDispatch_GreedyShared {
    /// C++: `void MSDispatch_GreedyShared::MSDispatch_GreedyShared(const Parameterised::Map &)`
    // MSDispatch_GreedyShared(const Parameterised::Map& params) :
    //         MSDispatch_Greedy(params),
    //         myAbsoluteLossThreshold(StringUtils::toDouble(getParameter("absLossThreshold", "300"))),
    //         myRelativeLossThreshold(StringUtils::toDouble(getParameter("relLossThreshold", "0.2")))
    //     {}
    #[allow(unused_variables)]
    pub fn new(params: &utils::common::parameterised::ParameterisedMap) -> Self {
        let mut base = MSDispatch_Greedy::new(params);
        base.dispatch_vt = greedy_shared_dispatch;
        let myAbsoluteLossThreshold = utils::common::string_utils::to_double(&base.base.base.get_parameter("absLossThreshold", "300")).unwrap();
        let myRelativeLossThreshold = utils::common::string_utils::to_double(&base.base.base.get_parameter("relLossThreshold", "0.2")).unwrap();
        MSDispatch_GreedyShared {
            base,
            myAbsoluteLossThreshold,
            myRelativeLossThreshold,
        }
    }

    /// C++: `int MSDispatch_GreedyShared::dispatch(MSDevice_Taxi *, std::vector<Reservation *>::iterator &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, std::vector<Reservation *> &)` [virtual, protected]
    /// trigger taxi dispatch.
    // int
    // MSDispatch_GreedyShared::dispatch(MSDevice_Taxi* taxi, std::vector<Reservation*>::iterator& resIt, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router, std::vector<Reservation*>& reservations) {
    //     const Reservation* const res = *resIt;
    // #ifdef DEBUG_DISPATCH
    //     if (DEBUG_COND2(person)) {
    //         std::cout << SIMTIME << " dispatch taxi=" << taxi->getHolder().getID() << " person=" << toString(res->persons) << "\n";
    //     }
    // #endif
    //     const bool isPerson = (*res->persons.begin())->isPerson();
    //     const int capacityLeft = remainingCapacity(taxi, res);
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     // check whether the ride can be shared
    //     int shareCase = 0;
    //     Reservation* res2 = nullptr;
    //     double absLoss = 0;
    //     double relLoss = 0;
    //     double absLoss2 = 0;
    //     double relLoss2 = 0;
    //     double directTime = -1; // only computed once for res
    //     double directTime2 = -1;
    //     for (auto it2 = resIt + 1; it2 != reservations.end(); it2++) {
    //         res2 = *it2;
    //         const bool isPerson2 = (*res2->persons.begin())->isPerson();
    //
    //         if (capacityLeft < (int)res2->persons.size() || isPerson != isPerson2 || !taxi->compatibleLine(res2)) {
    //             // do not try to mix person and container dispatch
    //             continue;
    //         }
    //         // res picks up res2 on the way
    //         directTime2 = -1; // reset for new candidate
    //         const SUMOTime startPickup = MAX2(now, res->pickupTime);
    //         const double detourTime = computeDetourTime(startPickup, res2->pickupTime, taxi,
    //                                   res->from, res->fromPos, res2->from, res2->fromPos, res->to, res->toPos, router, directTime);
    //         const double absLossPickup = detourTime - directTime;
    //         const double relLossPickup = absLossPickup / directTime;
    //
    // #ifdef DEBUG_DISPATCH
    //         if (DEBUG_COND2(person)) std::cout << "  consider sharing ride with " << toString(res2->persons)
    //                                                << " absLossPickup=" << absLossPickup
    //                                                << " relLossPickup=" << relLossPickup
    //                                                << "\n";
    // #endif
    //         if (absLossPickup < myAbsoluteLossThreshold && relLossPickup < myRelativeLossThreshold) {
    //             const SUMOTime startDropOff = MAX2(now, res2->pickupTime);
    //             double directTimeTmp = -1; // direct time from picking up res2 to dropping of res
    //             // case 1: res2 is dropped off before res (more detour for res)
    //             double detourTime2 = computeDetourTime(startDropOff, startDropOff, taxi,
    //                                                    res2->from, res2->fromPos, res2->to, res2->toPos, res->to, res->toPos, router, directTimeTmp);
    //             const double absLoss_c1 = absLossPickup + (detourTime2 - directTimeTmp);
    //             const double relLoss_c1 = absLoss_c1 / directTime;
    //
    //             // case 2: res2 is dropped off after res (detour for res2)
    //             double detourTime3 = computeDetourTime(startDropOff, startDropOff, taxi,
    //                                                    res2->from, res2->fromPos, res->to, res->toPos, res2->to, res2->toPos, router, directTime2);
    //             const double absLoss_c2 = detourTime3 - directTime2;
    //             const double relLoss_c2 = absLoss_c2 / directTime2;
    //
    //             if (absLoss_c2 <= absLoss_c1 && absLoss_c2 < myAbsoluteLossThreshold && relLoss_c2 < myRelativeLossThreshold) {
    //                 shareCase = 2;
    //                 taxi->dispatchShared({res, res2, res, res2});
    //                 absLoss = absLossPickup;
    //                 relLoss = relLossPickup;
    //                 absLoss2 = absLoss_c2;
    //                 relLoss2 = relLoss_c2;
    //             } else if (absLoss_c1 < myAbsoluteLossThreshold && relLoss_c1 < myRelativeLossThreshold) {
    //                 shareCase = 1;
    //                 taxi->dispatchShared({res, res2, res2, res});
    //                 absLoss = absLoss_c1;
    //                 relLoss = relLoss_c1;
    //                 absLoss2 = 0;
    //                 relLoss2 = 0;
    //             }
    //             if (shareCase != 0) {
    //                 reservations.erase(it2); // (it before it2) stays valid
    //                 break;
    //             } else {
    // #ifdef DEBUG_DISPATCH
    //                 if (DEBUG_COND2(person)) std::cout << "     rejected:"
    //                                                        << " absLoss_c1=" << absLoss_c1
    //                                                        << " relLoss_c1=" << relLoss_c1
    //                                                        << " absLoss_c2=" << absLoss_c2
    //                                                        << " relLoss_c2=" << relLoss_c2
    //                                                        << "\n";
    // #endif
    //             }
    //         }
    //     }
    //     if (shareCase != 0) {
    //         if (myOutput != nullptr) {
    //             myOutput->openTag("dispatchShared");
    //             myOutput->writeAttr("time", time2string(now));
    //             myOutput->writeAttr("id", taxi->getHolder().getID());
    //             myOutput->writeAttr("persons", toString(res->persons));
    //             myOutput->writeAttr("sharingPersons", toString(res2->persons));
    //             myOutput->writeAttr("type", shareCase);
    //             myOutput->writeAttr("absLoss", absLoss);
    //             myOutput->writeAttr("relLoss", relLoss);
    //             myOutput->writeAttr("absLoss2", absLoss2);
    //             myOutput->writeAttr("relLoss2", relLoss2);
    //             myOutput->closeTag();
    //         }
    // #ifdef DEBUG_DISPATCH
    //         if (DEBUG_COND2(person)) std::cout << "  sharing ride with " << toString(res2->persons)
    //                                                << " type=" << shareCase
    //                                                << " absLoss=" << absLoss << " relLoss=" << relLoss
    //                                                << " absLoss2=" << absLoss2 << " relLoss2=" << relLoss2
    //                                                << "\n";
    // #endif
    //         servedReservation(res2, taxi); // deleting res2
    //     } else {
    //         taxi->dispatch(*res);
    //     }
    //     servedReservation(res, taxi); // deleting res
    //     resIt = reservations.erase(resIt);
    //     return shareCase == 0 ? 1 : 2;
    // }
    #[allow(unused_variables)]
    pub fn dispatch(&mut self, taxi: Ptr<MSDevice_Taxi>, resIt: &mut usize /* std::vector<Reservation *>::iterator, as an index */, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, reservations: &mut Vec<Ptr<Reservation>>) -> i32 {
        let res: Ptr<Reservation> = reservations[*resIt];
        let isPerson = res.persons.values().next().unwrap().is_person();
        let capacityLeft = self.remaining_capacity(taxi, res);
        let now: SUMOTime = MSNet::get_instance().get_current_time_step();
        // check whether the ride can be shared
        let mut shareCase = 0;
        let mut res2: Ptr<Reservation> = Ptr::default();
        let mut absLoss: f64 = 0.;
        let mut relLoss: f64 = 0.;
        let mut absLoss2: f64 = 0.;
        let mut relLoss2: f64 = 0.;
        let mut directTime: f64 = -1.; // only computed once for res
        let mut directTime2: f64 = -1.;
        for it2 in (*resIt + 1)..reservations.len() {
            res2 = reservations[it2];
            let isPerson2 = res2.persons.values().next().unwrap().is_person();

            if capacityLeft < res2.persons.len() as i32 || isPerson != isPerson2 || !taxi.m().compatible_line(res2) {
                // do not try to mix person and container dispatch
                continue;
            }
            // res picks up res2 on the way
            directTime2 = -1.; // reset for new candidate
            let startPickup: SUMOTime = std::cmp::max(now, res.pickupTime);
            let detourTime = MSDispatch::compute_detour_time(startPickup, res2.pickupTime, taxi, res.from, res.fromPos, res2.from, res2.fromPos, res.to, res.toPos, router, &mut directTime);
            let absLossPickup = detourTime - directTime;
            let relLossPickup = absLossPickup / directTime;

            if absLossPickup < self.myAbsoluteLossThreshold && relLossPickup < self.myRelativeLossThreshold {
                let startDropOff: SUMOTime = std::cmp::max(now, res2.pickupTime);
                let mut directTimeTmp: f64 = -1.; // direct time from picking up res2 to dropping of res
                // case 1: res2 is dropped off before res (more detour for res)
                let detourTime2 = MSDispatch::compute_detour_time(startDropOff, startDropOff, taxi, res2.from, res2.fromPos, res2.to, res2.toPos, res.to, res.toPos, router, &mut directTimeTmp);
                let absLoss_c1 = absLossPickup + (detourTime2 - directTimeTmp);
                let relLoss_c1 = absLoss_c1 / directTime;

                // case 2: res2 is dropped off after res (detour for res2)
                let detourTime3 = MSDispatch::compute_detour_time(startDropOff, startDropOff, taxi, res2.from, res2.fromPos, res.to, res.toPos, res2.to, res2.toPos, router, &mut directTime2);
                let absLoss_c2 = detourTime3 - directTime2;
                let relLoss_c2 = absLoss_c2 / directTime2;

                if absLoss_c2 <= absLoss_c1 && absLoss_c2 < self.myAbsoluteLossThreshold && relLoss_c2 < self.myRelativeLossThreshold {
                    shareCase = 2;
                    taxi.m().dispatch_shared(vec![res, res2, res, res2]);
                    absLoss = absLossPickup;
                    relLoss = relLossPickup;
                    absLoss2 = absLoss_c2;
                    relLoss2 = relLoss_c2;
                } else if absLoss_c1 < self.myAbsoluteLossThreshold && relLoss_c1 < self.myRelativeLossThreshold {
                    shareCase = 1;
                    taxi.m().dispatch_shared(vec![res, res2, res2, res]);
                    absLoss = absLoss_c1;
                    relLoss = relLoss_c1;
                    absLoss2 = 0.;
                    relLoss2 = 0.;
                }
                if shareCase != 0 {
                    reservations.remove(it2); // (it before it2) stays valid
                    break;
                }
            }
        }
        if shareCase != 0 {
            if !self.myOutput.is_null() {
                let out = self.myOutput.m();
                out.open_tag("dispatchShared");
                out.write_attr("time", time2string(now).as_str());
                out.write_attr("id", taxi.myHolder.get_id());
                let persons = res.persons.values().map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ");
                out.write_attr("persons", persons.as_str());
                let sharingPersons = res2.persons.values().map(|p| p.get_id().to_string()).collect::<Vec<_>>().join(" ");
                out.write_attr("sharingPersons", sharingPersons.as_str());
                out.write_attr("type", &shareCase);
                out.write_attr("absLoss", &absLoss);
                out.write_attr("relLoss", &relLoss);
                out.write_attr("absLoss2", &absLoss2);
                out.write_attr("relLoss2", &relLoss2);
                out.close_tag("");
            }
            self.served_reservation(res2, taxi); // deleting res2
        } else {
            taxi.m().dispatch(&*res);
        }
        self.served_reservation(res, taxi); // deleting res
        reservations.remove(*resIt); // resIt = reservations.erase(resIt)
        if shareCase == 0 { 1 } else { 2 }
    }

}

