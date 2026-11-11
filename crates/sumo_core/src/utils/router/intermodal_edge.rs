//! Port of utils/router/IntermodalEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// the base edge type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `IntermodalEdge` (utils/router/IntermodalEdge.h)
pub struct IntermodalEdge<E, L, N, V> {
    /// C++ base class `Named`
    pub base: Named,
    /// List of edges that may be approached from this edge
    pub myFollowingEdges: Vec<Ptr<IntermodalEdge<E, L, N, V>>>,
    /// List of edges that may be approached from this edge with optional internal vias
    pub myFollowingViaEdges: Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)>,
    /// the index in myEdges
    pub myNumericalID: i32,
    /// the original edge
    pub myEdge: Ptr<E>,
    /// public transport line or ped vs car
    pub myLine: String,
    /// adaptable length (for splitted edges)
    pub myLength: f64,
    /// Container for passing effort varying over time for the edge
    pub myEfforts: Ptr<Unmapped /* ValueTimeLine<double> */>,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for IntermodalEdge<E, L, N, V> {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for IntermodalEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl<E, L, N, V> IntermodalEdge<E, L, N, V> {
    /// C++: `void IntermodalEdge::IntermodalEdge<E, L, N, V>(const std::string, int, const E *, const std::string &, const double)`
    // IntermodalEdge(const std::string id, int numericalID, const E* edge, const std::string& line, const double length = -1) :
    //         Named(id),
    //         myNumericalID(numericalID),
    //         myEdge(edge),
    //         myLine(line),
    //         myLength(edge == nullptr || length >= 0. ? MAX2(0.0, length) : edge->getLength()),
    //         myEfforts(nullptr) { }
    #[allow(unused_variables)]
    pub fn new(id: String, numericalID: i32, edge: Ptr<E>, line: &str, length: f64) -> Self {
        todo!("port IntermodalEdge::IntermodalEdge<E, L, N, V>")
    }

    /// C++: `void IntermodalEdge::~IntermodalEdge<E, L, N, V>()` [virtual]
    // virtual ~IntermodalEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port IntermodalEdge::~IntermodalEdge<E, L, N, V>")
    }

    /// C++: `bool IntermodalEdge::includeInRoute(bool) const` [const, virtual]
    // virtual bool includeInRoute(bool /* allEdges */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        todo!("port IntermodalEdge::includeInRoute")
    }

    /// C++: `const std::string & IntermodalEdge::getLine() const` [const]
    // inline const std::string& getLine() const {
    //         return myLine;
    //     }
    #[allow(unused_variables)]
    pub fn get_line(&self) -> &str {
        todo!("port IntermodalEdge::getLine")
    }

    /// C++: `const E * IntermodalEdge::getEdge() const` [const]
    // inline const E* getEdge() const {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<E> {
        todo!("port IntermodalEdge::getEdge")
    }

    /// C++: `int IntermodalEdge::getNumericalID() const` [const]
    // int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port IntermodalEdge::getNumericalID")
    }

    /// C++: `void IntermodalEdge::addSuccessor(IntermodalEdge<E, L, N, V> *const, IntermodalEdge<E, L, N, V> *const)`
    // void addSuccessor(IntermodalEdge* const s, IntermodalEdge* const via = nullptr) {
    //         myFollowingEdges.push_back(s);
    //         myFollowingViaEdges.push_back(std::make_pair(s, via));
    //     }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<IntermodalEdge<E, L, N, V>>, via: Ptr<IntermodalEdge<E, L, N, V>>) {
        todo!("port IntermodalEdge::addSuccessor")
    }

    /// C++: `void IntermodalEdge::transferSuccessors(IntermodalEdge<E, L, N, V> *)`
    // void transferSuccessors(IntermodalEdge* to) {
    //         to->myFollowingEdges = myFollowingEdges;
    //         to->myFollowingViaEdges = myFollowingViaEdges;
    //         myFollowingEdges.clear();
    //         myFollowingViaEdges.clear();
    //     }
    #[allow(unused_variables)]
    pub fn transfer_successors(&mut self, to: Ptr<IntermodalEdge<E, L, N, V>>) {
        todo!("port IntermodalEdge::transferSuccessors")
    }

    /// C++: `bool IntermodalEdge::removeSuccessor(const IntermodalEdge<E, L, N, V> *const)`
    // bool removeSuccessor(const IntermodalEdge* const edge) {
    //         auto it = std::find(myFollowingEdges.begin(), myFollowingEdges.end(), edge);
    //         if (it != myFollowingEdges.end()) {
    //             myFollowingEdges.erase(it);
    //         } else {
    //             return false;
    //         }
    //         for (auto viaIt = myFollowingViaEdges.begin(); viaIt != myFollowingViaEdges.end();) {
    //             if (viaIt->first == edge) {
    //                 viaIt = myFollowingViaEdges.erase(viaIt);
    //             } else {
    //                 ++viaIt;
    //             }
    //         }
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn remove_successor(&mut self, edge: Ptr<IntermodalEdge<E, L, N, V>>) -> bool {
        todo!("port IntermodalEdge::removeSuccessor")
    }

    /// C++: `const std::vector<IntermodalEdge<E, L, N, V> *> & IntermodalEdge::getSuccessors(SUMOVehicleClass) const` [const, virtual]
    // virtual const std::vector<IntermodalEdge*>& getSuccessors(SUMOVehicleClass vClass = SVC_IGNORING) const {
    //         UNUSED_PARAMETER(vClass);
    //         // the network is already tailored. No need to check for permissions here
    //         return myFollowingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &Vec<Ptr<IntermodalEdge<E, L, N, V>>> {
        todo!("port IntermodalEdge::getSuccessors")
    }

    /// C++: `const std::vector<std::pair<const IntermodalEdge<E, L, N, V> *, const IntermodalEdge<E, L, N, V> *>> & IntermodalEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const, virtual]
    // virtual const std::vector<std::pair<const IntermodalEdge*, const IntermodalEdge*> >& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         UNUSED_PARAMETER(vClass);
    //         UNUSED_PARAMETER(ignoreTransientPermissions);
    //         // the network is already tailored. No need to check for permissions here
    //         return myFollowingViaEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &Vec<(Ptr<IntermodalEdge<E, L, N, V>>, Ptr<IntermodalEdge<E, L, N, V>>)> {
        todo!("port IntermodalEdge::getViaSuccessors")
    }

    /// C++: `bool IntermodalEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual bool prohibits(const IntermodalTrip<E, N, V>* const /* trip */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        todo!("port IntermodalEdge::prohibits")
    }

    /// C++: `bool IntermodalEdge::restricts(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual bool restricts(const IntermodalTrip<E, N, V>* const /* trip */) const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        todo!("port IntermodalEdge::restricts")
    }

    /// C++: `double IntermodalEdge::getPartialLength(const IntermodalTrip<E, N, V> *const) const` [const, virtual]
    // virtual inline double getPartialLength(const IntermodalTrip<E, N, V>* const /*trip*/) const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_partial_length(&self, arg0: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        todo!("port IntermodalEdge::getPartialLength")
    }

    /// C++: `double IntermodalEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual inline double getTravelTime(const IntermodalTrip<E, N, V>* const /* trip */, double /* time */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, arg0: Ptr<IntermodalTrip<E, N, V>>, arg1: f64) -> f64 {
        todo!("port IntermodalEdge::getTravelTime")
    }

    /// C++: `double IntermodalEdge::getTravelTimeAggregated(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual inline double getTravelTimeAggregated(const IntermodalTrip<E, N, V>* const trip, double time) const {
    //         return getTravelTime(trip, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(&self, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port IntermodalEdge::getTravelTimeAggregated")
    }

    /// C++: `double IntermodalEdge::getIntended(const double, std::string &) const` [const, virtual]
    /// get intended vehicle id and departure time of next public transport ride
    // virtual inline double getIntended(const double /* time */, std::string& /* intended */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended(&self, arg0: f64, arg1: &mut String) -> f64 {
        todo!("port IntermodalEdge::getIntended")
    }

    /// C++: `double IntermodalEdge::getTravelTimeStatic(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeStatic(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTime(trip, time) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port IntermodalEdge::getTravelTimeStatic")
    }

    /// C++: `double IntermodalEdge::getTravelTimeStaticRandomized(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeStaticRandomized(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTime(trip, time) * RandHelper::rand(1., gWeightsRandomFactor) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port IntermodalEdge::getTravelTimeStaticRandomized")
    }

    /// C++: `double IntermodalEdge::getTravelTimeAggregated(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getTravelTimeAggregated(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr ? 0. : edge->getTravelTimeAggregated(trip, time) * getRoutingFactor(edge, trip);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated_2(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port IntermodalEdge::getTravelTimeAggregated")
    }

    /// C++: `double IntermodalEdge::getRoutingFactor(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const)` [static]
    // static inline double getRoutingFactor(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip) {
    //         return (!gRoutingPreferences || edge == nullptr || edge->myEdge == nullptr || trip == nullptr) ? 1 : 1 / edge->myEdge->getPreference(trip->getVTypeParameter());
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_factor(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        todo!("port IntermodalEdge::getRoutingFactor")
    }

    /// C++: `double IntermodalEdge::getEffort(const IntermodalTrip<E, N, V> *const, double) const` [const, virtual]
    // virtual double getEffort(const IntermodalTrip<E, N, V>* const /* trip */, double /* time */) const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_effort(&self, arg0: Ptr<IntermodalTrip<E, N, V>>, arg1: f64) -> f64 {
        todo!("port IntermodalEdge::getEffort")
    }

    /// C++: `double IntermodalEdge::getEffortStatic(const IntermodalEdge<E, L, N, V> *const, const IntermodalTrip<E, N, V> *const, double)` [static]
    // static inline double getEffortStatic(const IntermodalEdge* const edge, const IntermodalTrip<E, N, V>* const trip, double time) {
    //         return edge == nullptr || !edge->hasEffort() ? 0. : edge->getEffort(trip, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_effort_static(edge: Ptr<IntermodalEdge<E, L, N, V>>, trip: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port IntermodalEdge::getEffortStatic")
    }

    /// C++: `double IntermodalEdge::getLength() const` [const]
    /// required by DijkstraRouter et al for external effort computation
    // inline double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port IntermodalEdge::getLength")
    }

    /// C++: `void IntermodalEdge::setLength(const double)`
    // inline void setLength(const double length) {
    //         assert(length >= 0);
    //         myLength = length;
    //     }
    #[allow(unused_variables)]
    pub fn set_length(&mut self, length: f64) {
        todo!("port IntermodalEdge::setLength")
    }

    /// C++: `bool IntermodalEdge::isInternal() const` [const]
    // inline bool isInternal() const {
    //         return myEdge != nullptr && myEdge->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        todo!("port IntermodalEdge::isInternal")
    }

    /// C++: `bool IntermodalEdge::hasEffort() const` [const, virtual]
    // virtual bool hasEffort() const {
    //         return myEfforts != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_effort(&self) -> bool {
        todo!("port IntermodalEdge::hasEffort")
    }

    /// C++: `double IntermodalEdge::getStartPos() const` [const, virtual]
    // virtual double getStartPos() const {
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_start_pos(&self) -> f64 {
        todo!("port IntermodalEdge::getStartPos")
    }

    /// C++: `double IntermodalEdge::getEndPos() const` [const, virtual]
    // virtual double getEndPos() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self) -> f64 {
        todo!("port IntermodalEdge::getEndPos")
    }

    /// C++: `double IntermodalEdge::getSpeedLimit() const` [const]
    // inline double getSpeedLimit() const {
    //         return myEdge != nullptr ? myEdge->getSpeedLimit() : 200. / 3.6;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        todo!("port IntermodalEdge::getSpeedLimit")
    }

    /// C++: `double IntermodalEdge::getLengthGeometryFactor() const` [const]
    // inline double getLengthGeometryFactor() const {
    //         return myEdge != nullptr ? myEdge->getLengthGeometryFactor() : 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        todo!("port IntermodalEdge::getLengthGeometryFactor")
    }

    /// C++: `double IntermodalEdge::getDistanceTo(const IntermodalEdge<E, L, N, V> *) const` [const]
    // inline double getDistanceTo(const IntermodalEdge* other) const {
    //         return myEdge != nullptr && other->myEdge != nullptr && myEdge != other->myEdge ? myEdge->getDistanceTo(other->myEdge, true) : 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<IntermodalEdge<E, L, N, V>>) -> f64 {
        todo!("port IntermodalEdge::getDistanceTo")
    }

    /// C++: `double IntermodalEdge::getMinimumTravelTime(const IntermodalTrip<E, N, V> *const) const` [const]
    // inline double getMinimumTravelTime(const IntermodalTrip<E, N, V>* const trip) const {
    //         return myLength / trip->getMaxSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> f64 {
        todo!("port IntermodalEdge::getMinimumTravelTime")
    }

    /// C++: `IntermodalEdge<E, L, N, V> * IntermodalEdge::getBidiEdge() const` [const]
    /// only used by mono-modal routing
    // IntermodalEdge* getBidiEdge() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<IntermodalEdge<E, L, N, V>> {
        todo!("port IntermodalEdge::getBidiEdge")
    }

    /// C++: `void IntermodalEdge::IntermodalEdge<E, L, N, V>(const IntermodalEdge<E, L, N, V> &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &IntermodalEdge<E, L, N, V>) -> Self {
        todo!("port IntermodalEdge::IntermodalEdge<E, L, N, V>")
    }

    /// C++: `IntermodalEdge<E, L, N, V> & IntermodalEdge::operator=(const IntermodalEdge<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &IntermodalEdge<E, L, N, V>) -> &mut IntermodalEdge<E, L, N, V> {
        todo!("port IntermodalEdge::operator=")
    }

}

/// C++: `const L *(const E *, SUMOVehicleClass) getSidewalk`
// template <class E, class L>
// inline const L* getSidewalk(const E* edge, SUMOVehicleClass svc = SVC_PEDESTRIAN) {
//     if (edge == nullptr) {
//         return nullptr;
//     }
//     // prefer lanes that are exclusive to pedestrians
//     const std::vector<L*>& lanes = edge->getLanes();
//     for (const L* const lane : lanes) {
//         if (lane->getPermissions() == svc) {
//             return lane;
//         }
//     }
//     for (const L* const lane : lanes) {
//         if (lane->allowsVehicleClass(svc)) {
//             return lane;
//         }
//     }
//     if (svc != SVC_PEDESTRIAN) {
//         // persons should always be able to use the sidewalk
//         for (const L* const lane : lanes) {
//             if (lane->getPermissions() == SVC_PEDESTRIAN) {
//                 return lane;
//             }
//         }
//         for (const L* const lane : lanes) {
//             if (lane->allowsVehicleClass(SVC_PEDESTRIAN)) {
//                 return lane;
//             }
//         }
//     }
//     return nullptr;
// }
#[allow(unused_variables)]
pub fn get_sidewalk() -> Ptr<Unmapped /* L */> {
    todo!("port getSidewalk")
}

