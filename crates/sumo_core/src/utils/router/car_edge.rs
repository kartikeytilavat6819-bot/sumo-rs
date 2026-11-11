//! Port of utils/router/CarEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `IntermodalEdge<E, L, N, V> CarEdge::_IntermodalEdge`
pub type CarEdge__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// the car edge type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `CarEdge` (utils/router/CarEdge.h)
pub struct CarEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    /// the starting position for split edges
    pub myStartPos: f64,
    /// The successors available for a given vClass
    pub myClassesSuccessorMap: BTreeMap<SUMOVehicleClass, Vec<Ptr<AccessEdge__IntermodalEdge>>>,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, Vec<(Ptr<AccessEdge__IntermodalEdge>, Ptr<AccessEdge__IntermodalEdge>)>>, // C++: std::map<SUMOVehicleClass, std::vector<std::pair<const _IntermodalEdge *, const _IntermodalEdge *>>>
    pub myOrigClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, Vec<(Ptr<AccessEdge__IntermodalEdge>, Ptr<AccessEdge__IntermodalEdge>)>>, // C++: std::map<SUMOVehicleClass, std::vector<std::pair<const _IntermodalEdge *, const _IntermodalEdge *>>>
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

impl<E, L, N, V> CarEdge<E, L, N, V> {
    /// C++: `void CarEdge::CarEdge<E, L, N, V>(int, const E *, const double)`
    // CarEdge(int numericalID, const E* edge, const double pos = -1.) :
    //         _IntermodalEdge(edge->getID() + "_car" + toString(pos), numericalID, edge, "!car"),
    //         myStartPos(pos >= 0 ? pos : 0.) { }
    #[allow(unused_variables)]
    pub fn new(numericalID: i32, edge: Ptr<E>, pos: f64) -> Self {
        todo!("port CarEdge::CarEdge<E, L, N, V>")
    }

    /// C++: `bool CarEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool /* allEdges */) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        todo!("port CarEdge::includeInRoute")
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
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &Vec<Ptr<AccessEdge__IntermodalEdge>> {
        todo!("port CarEdge::getSuccessors")
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
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &Vec<(Ptr<AccessEdge__IntermodalEdge>, Ptr<AccessEdge__IntermodalEdge>)> {
        todo!("port CarEdge::getViaSuccessors")
    }

    /// C++: `bool CarEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         return trip->vehicle == 0 || this->getEdge()->prohibits(trip->vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        todo!("port CarEdge::prohibits")
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
        todo!("port CarEdge::getPartialLength")
    }

    /// C++: `double CarEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTime(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         assert(E::getTravelTimeStatic(this->getEdge(), trip->vehicle, time) >= 0.);
    //         return getPartialTravelTime(E::getTravelTimeStatic(this->getEdge(), trip->vehicle, time), trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port CarEdge::getTravelTime")
    }

    /// C++: `double CarEdge::getTravelTimeAggregated(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTimeAggregated(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         assert(E::getTravelTimeAggregated(this->getEdge(), trip->vehicle, time) >= 0.);
    //         return getPartialTravelTime(E::getTravelTimeAggregated(this->getEdge(), trip->vehicle, time), trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port CarEdge::getTravelTimeAggregated")
    }

    /// C++: `double CarEdge::getStartPos() const` [const]
    // double getStartPos() const {
    //         return myStartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        todo!("port CarEdge::getStartPos")
    }

    /// C++: `double CarEdge::getEndPos() const` [const]
    // double getEndPos() const {
    //         return myStartPos + this->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        todo!("port CarEdge::getEndPos")
    }

    /// C++: `double CarEdge::getPartialTravelTime(double, const IntermodalTrip<E, N, V> *const) const` [const, private]
    // inline double getPartialTravelTime(double fullTravelTime, const IntermodalTrip<E, N, V>* const trip) const {
    //         const double distTravelled = getPartialLength(trip);
    //         assert(fullTravelTime * distTravelled / this->getEdge()->getLength() >= 0.);
    //         return fullTravelTime * distTravelled / this->getEdge()->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_travel_time(&self, fullTravelTime: f64, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        todo!("port CarEdge::getPartialTravelTime")
    }

}

