//! Port of utils/router/CarEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::intermodal_edge::{IntermodalEdge, IntermodalEdgeKind};
use crate::utils::router::intermodal_trip::IntermodalTrip;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};
use utils::common::stable_cache::StableCache;
use utils::common::to_string::to_string;

/// C++ typedef `IntermodalEdge<E, L, N, V> CarEdge::_IntermodalEdge`
pub type CarEdge__IntermodalEdge<E, L, N, V> = IntermodalEdge<E, L, N, V>;

/// the car edge type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `CarEdge` (utils/router/CarEdge.h)
#[repr(C)]
pub struct CarEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    /// the starting position for split edges
    pub myStartPos: f64,
    /// The successors available for a given vClass
    pub myClassesSuccessorMap: StableCache<SUMOVehicleClass, Vec<Ptr<IntermodalEdge<E, L, N, V>>>>,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: StableCache<SUMOVehicleClass, Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)>>, // C++: std::map<SUMOVehicleClass, std::vector<std::pair<const _IntermodalEdge *, const _IntermodalEdge *>>>
    pub myOrigClassesViaSuccessorMap: StableCache<SUMOVehicleClass, Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)>>, // C++: std::map<SUMOVehicleClass, std::vector<std::pair<const _IntermodalEdge *, const _IntermodalEdge *>>>
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for CarEdge<E, L, N, V> {
    type Target = IntermodalEdge<E, L, N, V>;
    fn deref(&self) -> &IntermodalEdge<E, L, N, V> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for CarEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut IntermodalEdge<E, L, N, V> {
        &mut self.base
    }
}

impl<E, L, N, V> CarEdge<E, L, N, V>
where
    E: RouterEdge<V, Node = N, Lane = L> + 'static,
    L: 'static,
    N: 'static,
    V: RouterVehicle + 'static,
{
    /// C++: `void CarEdge::CarEdge<E, L, N, V>(int, const E *, const double)`
    // CarEdge(int numericalID, const E* edge, const double pos = -1.) :
    //         _IntermodalEdge(edge->getID() + "_car" + toString(pos), numericalID, edge, "!car"),
    //         myStartPos(pos >= 0 ? pos : 0.) { }
    #[allow(unused_variables)]
    pub fn new(numericalID: i32, edge: Ptr<E>, pos: f64) -> Self {
        let id = format!("{}_car{}", edge.id(), to_string(&pos));
        Self {
            base: IntermodalEdge::new(id, numericalID, edge, "!car", -1.).with_kind(IntermodalEdgeKind::Car),
            myStartPos: if pos >= 0. { pos } else { 0. },
            myClassesSuccessorMap: StableCache::new(),
            myClassesViaSuccessorMap: StableCache::new(),
            myOrigClassesViaSuccessorMap: StableCache::new(),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `bool CarEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool /* allEdges */) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        true
    }

    /// C++: `const std::vector<_IntermodalEdge *> & CarEdge::getSuccessors(SUMOVehicleClass) const` [const]
    // const std::vector<_IntermodalEdge*>& getSuccessors(SUMOVehicleClass vClass = SVC_IGNORING) const {
    //         if (vClass == SVC_IGNORING) {
    //             return this->myFollowingEdges;
    //         }
    // #ifdef HAVE_FOX
    //         FXMutexLock locker(myLock);
    // #endif
    //         typename std::map<SUMOVehicleClass, std::vector<_IntermodalEdge*> >::const_iterator i = myClassesSuccessorMap.find(vClass);
    //         if (i != myClassesSuccessorMap.end()) {
    //             // can use cached value
    //             return i->second;
    //         } else {
    //             // this vClass is requested for the first time. rebuild all successors
    //             const std::set<const E*> classedCarFollowers = std::set<const E*>(this->getEdge()->getSuccessors(vClass).begin(), this->getEdge()->getSuccessors(vClass).end());
    //             for (_IntermodalEdge* const e : this->myFollowingEdges) {
    //                 if (!e->includeInRoute(false) || e->getEdge() == this->getEdge() || classedCarFollowers.count(e->getEdge()) > 0) {
    //                     myClassesSuccessorMap[vClass].push_back(e);
    //                 }
    //             }
    //             return myClassesSuccessorMap[vClass];
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &Vec<Ptr<IntermodalEdge<E, L, N, V>>> {
        if vClass == SVC_IGNORING {
            return &self.myFollowingEdges;
        }
        // C++ `mutable` cache: this vClass is requested for the first time, so rebuild all successors
        self.myClassesSuccessorMap.get_or_insert_with(vClass, || {
            // determinism: BTreeSet is only queried for membership, never iterated
            let classedCarFollowers: BTreeSet<Ptr<E>> = self.get_edge().successors(vClass).iter().copied().collect();
            let mut list = Vec::new();
            for &e in self.myFollowingEdges.iter() {
                if !e.include_in_route(false) || e.get_edge() == self.get_edge() || classedCarFollowers.contains(&e.get_edge()) {
                    list.push(e);
                }
            }
            list
        })
    }

    /// C++: `const std::vector<std::pair<const _IntermodalEdge *, const _IntermodalEdge *>> & CarEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual]
    // virtual const std::vector<std::pair<const _IntermodalEdge*, const _IntermodalEdge*> >& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         if (vClass == SVC_IGNORING) {
    //             return this->myFollowingViaEdges;
    //         }
    // #ifdef HAVE_FOX
    //         FXMutexLock locker(myLock);
    // #endif
    //         auto& viaMap = ignoreTransientPermissions ? myOrigClassesViaSuccessorMap : myClassesViaSuccessorMap;
    //         typename std::map<SUMOVehicleClass, std::vector<std::pair<const _IntermodalEdge*, const _IntermodalEdge*> > >::const_iterator i = viaMap.find(vClass);
    //         if (i != viaMap.end()) {
    //             // can use cached value
    //             return i->second;
    //         } else {
    //             // this vClass is requested for the first time. rebuild all successors
    //             std::set<const E*> classedCarFollowers;
    //             for (const auto& pair : this->getEdge()->getViaSuccessors(vClass)) {
    //                 classedCarFollowers.insert(pair.first);
    //             }
    //             for (const std::pair<const _IntermodalEdge*, const _IntermodalEdge*>& e : this->myFollowingViaEdges) {
    //                 if (!e.first->includeInRoute(false) || e.first->getEdge() == this->getEdge() || classedCarFollowers.count(e.first->getEdge()) > 0) {
    //                     viaMap[vClass].push_back(e);
    //                 }
    //             }
    //             return viaMap[vClass];
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)> {
        if vClass == SVC_IGNORING {
            return &self.myFollowingViaEdges;
        }
        let viaMap = if ignoreTransientPermissions { &self.myOrigClassesViaSuccessorMap } else { &self.myClassesViaSuccessorMap };
        // C++ `mutable` cache: this vClass is requested for the first time, so rebuild all successors
        viaMap.get_or_insert_with(vClass, || {
            // determinism: BTreeSet is only queried for membership, never iterated
            let mut classedCarFollowers: BTreeSet<Ptr<E>> = BTreeSet::new();
            for pair in self.get_edge().via_successors(vClass, false) {
                classedCarFollowers.insert(pair.0);
            }
            let mut list = Vec::new();
            for &e in self.myFollowingViaEdges.iter() {
                if !e.0.include_in_route(false) || e.0.get_edge() == self.get_edge() || classedCarFollowers.contains(&e.0.get_edge()) {
                    list.push(e);
                }
            }
            list
        })
    }

    /// C++: `bool CarEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         return trip->vehicle == 0 || this->getEdge()->prohibits(trip->vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        if trip.vehicle.is_null() {
            return true;
        }
        self.get_edge().prohibits(trip.vehicle, true)
    }

    /// C++: `double CarEdge::getPartialLength(const IntermodalTrip<E, N, V> *const) const` [const]
    // double getPartialLength(const IntermodalTrip<E, N, V>* const trip) const {
    //         double length = this->getLength();
    //         // checking arrivalPos first to have it correct for identical depart and arrival edge
    //         if (this->getEdge() == trip->to && trip->arrivalPos >= myStartPos && trip->arrivalPos < myStartPos + this->getLength()) {
    //             length = trip->arrivalPos - myStartPos;
    //         }
    //         if (this->getEdge() == trip->from && trip->departPos >= myStartPos && trip->departPos < myStartPos + this->getLength()) {
    //             length -= trip->departPos - myStartPos;
    //         }
    //         return length;
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_length(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        let mut length = self.get_length();
        // checking arrivalPos first to have it correct for identical depart and arrival edge
        if self.get_edge() == trip.to && trip.arrivalPos >= self.myStartPos && trip.arrivalPos < self.myStartPos + self.get_length() {
            length = trip.arrivalPos - self.myStartPos;
        }
        if self.get_edge() == trip.from && trip.departPos >= self.myStartPos && trip.departPos < self.myStartPos + self.get_length() {
            length -= trip.departPos - self.myStartPos;
        }
        length
    }

    /// C++: `double CarEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTime(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         assert(E::getTravelTimeStatic(this->getEdge(), trip->vehicle, time) >= 0.);
    //         return getPartialTravelTime(E::getTravelTimeStatic(this->getEdge(), trip->vehicle, time), trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        let full = E::travel_time_static(self.get_edge(), trip.vehicle, time);
        assert!(full >= 0.);
        self.get_partial_travel_time(full, trip)
    }

    /// C++: `double CarEdge::getTravelTimeAggregated(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTimeAggregated(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         assert(E::getTravelTimeAggregated(this->getEdge(), trip->vehicle, time) >= 0.);
    //         return getPartialTravelTime(E::getTravelTimeAggregated(this->getEdge(), trip->vehicle, time), trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        let full = E::travel_time_aggregated(self.get_edge(), trip.vehicle, time);
        assert!(full >= 0.);
        self.get_partial_travel_time(full, trip)
    }

    /// C++: `double CarEdge::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        self.myStartPos
    }

    /// C++: `double CarEdge::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myStartPos + this->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        self.myStartPos + self.get_length()
    }

    /// C++: `double CarEdge::getPartialTravelTime(double, const IntermodalTrip<E, N, V> *const) const` [const, private]
    // inline double getPartialTravelTime(double fullTravelTime, const IntermodalTrip<E, N, V>* const trip) const {
    //         const double distTravelled = getPartialLength(trip);
    //         assert(fullTravelTime * distTravelled / this->getEdge()->getLength() >= 0.);
    //         return fullTravelTime * distTravelled / this->getEdge()->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_travel_time(&self, fullTravelTime: f64, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        let distTravelled = self.get_partial_length(trip);
        assert!(fullTravelTime * distTravelled / self.get_edge().length() >= 0.);
        fullTravelTime * distTravelled / self.get_edge().length()
    }

}

