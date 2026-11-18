//! Port of utils/router/FlippedEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;
use utils::common::sumo_vehicle_class::SVC_IGNORING;
use utils::common::stable_cache::StableCache;

/// C++ typedef `std::vector<std::pair<const FlippedEdge<E, N, V> *, const FlippedEdge<E, N, V> *>> FlippedEdge::ConstFlippedEdgePairVector`
pub type FlippedEdge_ConstFlippedEdgePairVector<E, N, V> = Vec<(Ptr<FlippedEdge<E, N, V>>, Ptr<FlippedEdge<E, N, V>>)>;

/// The edge type representing backward edges with flipped nodes
/// C++ class `FlippedEdge` (utils/router/FlippedEdge.h)
#[repr(C)]
pub struct FlippedEdge<E, N, V> {
    /// C++ base class `ReversedEdge<E, V>`
    pub base: ReversedEdge<E, V>,
    /// The junctions for this edge
    pub myFromJunction: Ptr<FlippedNode<E, N, V>>,
    pub myToJunction: Ptr<FlippedNode<E, N, V>>,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: StableCache<SUMOVehicleClass, FlippedEdge_ConstFlippedEdgePairVector<E, N, V>>,
    pub myViaSuccessors: FlippedEdge_ConstFlippedEdgePairVector<E, N, V>,
    _p: std::marker::PhantomData<(E, N, V,)>,
}

impl<E, N, V> std::ops::Deref for FlippedEdge<E, N, V> {
    type Target = ReversedEdge<E, V>;
    fn deref(&self) -> &ReversedEdge<E, V> {
        &self.base
    }
}
impl<E, N, V> std::ops::DerefMut for FlippedEdge<E, N, V> {
    fn deref_mut(&mut self) -> &mut ReversedEdge<E, V> {
        &mut self.base
    }
}

impl<E, N, V> FlippedEdge<E, N, V>
where
    E: RouterEdge<V, Node = N>,
{
    /// C++: `void FlippedEdge::FlippedEdge<E, N, V>(const E *)`
    /// Constructor
    // FlippedEdge(const E* originalEdge) :
    //         ReversedEdge<E, V>(originalEdge),
    //         myFromJunction(originalEdge->getToJunction()->getFlippedRoutingNode()),
    //         myToJunction(originalEdge->getFromJunction()->getFlippedRoutingNode())
    //     {}
    #[allow(unused_variables)]
    pub fn new(originalEdge: Ptr<E>) -> Self {
        // shortcut: needs router trait bounds (lead), E::getToJunction/getFromJunction and N::getFlippedRoutingNode are not callable on generic E/N
        panic!("shortcut: needs router trait bounds (lead), E::getToJunction")
    }

    /// C++: `void FlippedEdge::~FlippedEdge<E, N, V>()`
    /// Destructor
    // ~FlippedEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {

    }

    /// C++: `void FlippedEdge::init()`
    /// Initialize the flipped edge
    // template<class E, class N, class V>
    // void FlippedEdge<E, N, V>::init() {
    //     if (!this->getOriginalEdge()->isInternal()) {
    //         for (const auto& viaPair : this->getOriginalEdge()->getViaSuccessors()) {
    //             const FlippedEdge<E, N, V>* revSource = viaPair.first->getFlippedRoutingEdge();
    //             const E* via = viaPair.second;
    //             const FlippedEdge<E, N, V>* preVia = nullptr;
    //             while (via != nullptr && via->isInternal()) {
    //                 via->getFlippedRoutingEdge()->myViaSuccessors.push_back(std::make_pair(this, preVia));
    //                 preVia = via->getFlippedRoutingEdge();
    //                 via = via->getViaSuccessors().front().second;
    //             }
    //             revSource->myViaSuccessors.push_back(std::make_pair(this, preVia));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        let original = self.base.get_original_edge();
        if !original.is_internal() {
            let this: Ptr<FlippedEdge<E, N, V>> = Ptr::from_mut(self);
            for viaPair in original.via_successors(SVC_IGNORING, false).to_vec() {
                let revSource = viaPair.0.flipped_routing_edge();
                let mut via = viaPair.1;
                let mut preVia: Ptr<FlippedEdge<E, N, V>> = Ptr::null();
                while !via.is_null() && via.is_internal() {
                    via.flipped_routing_edge().m().myViaSuccessors.push((this, preVia));
                    preVia = via.flipped_routing_edge();
                    via = via.via_successors(SVC_IGNORING, false)[0].1;
                }
                revSource.m().myViaSuccessors.push((this, preVia));
            }
        }
    }

    /// C++: `const FlippedNode<E, N, V> * FlippedEdge::getFromJunction() const` [const]
    /// Returns the from-junction
    // const FlippedNode<E, N, V>* getFromJunction() const {
    //         return myFromJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<FlippedNode<E, N, V>> {
        self.myFromJunction
    }

    /// C++: `const FlippedNode<E, N, V> * FlippedEdge::getToJunction() const` [const]
    /// Returns the to-junction
    // const FlippedNode<E, N, V>* getToJunction() const {
    //         return myToJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<FlippedNode<E, N, V>> {
        self.myToJunction
    }

    /// C++: `double FlippedEdge::getTravelTimeStatic(const FlippedEdge<E, N, V> *const, const V *const, double)` [static]
    /// Returns the time for travelling on the given edge with the given vehicle at the given time
    // static double getTravelTimeStatic(const FlippedEdge<E, N, V>* const edge, const V* const veh, double time) {
    //         return edge->getOriginalEdge()->getTravelTime(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<FlippedEdge<E, N, V>>, veh: Ptr<V>, time: f64) -> f64 {
        edge.get_original_edge().travel_time(veh, time)
    }

    /// C++: `double FlippedEdge::getTravelTimeStaticRandomized(const FlippedEdge<E, N, V> *const, const V *const, double)` [static]
    /// Returns the randomized time for travelling on the given edge with the given vehicle at the given time
    // static double getTravelTimeStaticRandomized(const FlippedEdge<E, N, V>* const edge, const V* const veh, double time) {
    //         return edge->getOriginalEdge()->getTravelTimeStaticRandomized(edge->getOriginalEdge(), veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<FlippedEdge<E, N, V>>, veh: Ptr<V>, time: f64) -> f64 {
        // shortcut: needs router trait bounds (lead), E::getTravelTimeStaticRandomized is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getTravelTimeStaticRandomized")
    }

    /// C++: `const ConstFlippedEdgePairVector & FlippedEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    /// Returns the via successors
    // const ConstFlippedEdgePairVector& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         UNUSED_PARAMETER(ignoreTransientPermissions); // @todo this should be changed (somewhat hidden by #14756)
    //         if (vClass == SVC_IGNORING || this->getOriginalEdge()->isTazConnector()) { // || !MSNet::getInstance()->hasPermissions()) {
    //             return myViaSuccessors;
    //         }
    // #ifdef HAVE_FOX
    //         FXMutexLock lock(mySuccessorMutex);
    // #endif
    //         auto i = myClassesViaSuccessorMap.find(vClass);
    //         if (i != myClassesViaSuccessorMap.end()) {
    //             // can use cached value
    //             return i->second;
    //         }
    //         // instantiate vector
    //         ConstFlippedEdgePairVector& result = myClassesViaSuccessorMap[vClass];
    //         // this vClass is requested for the first time. rebuild all successors
    //         for (const auto& viaPair : myViaSuccessors) {
    //             if (viaPair.first->getOriginalEdge()->isTazConnector()
    //                     || viaPair.first->getOriginalEdge()->isConnectedTo(*(this->getOriginalEdge()), vClass)) {
    //                 result.push_back(viaPair);
    //             }
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &FlippedEdge_ConstFlippedEdgePairVector<E, N, V> {
        if vClass == SVC_IGNORING || self.get_original_edge().is_taz_connector() {
            return &self.myViaSuccessors;
        }
        // C++ mutable cache: the const method fills myClassesViaSuccessorMap[vClass] in place
        self.myClassesViaSuccessorMap.get_or_insert_with(vClass, || {
            let mut result: FlippedEdge_ConstFlippedEdgePairVector<E, N, V> = Vec::new();
            for viaPair in self.myViaSuccessors.iter() {
                if viaPair.0.get_original_edge().is_taz_connector() || viaPair.0.get_original_edge().is_connected_to(&*self.get_original_edge(), vClass, false) {
                    result.push(*viaPair);
                }
            }
            result
        })
    }

    /// C++: `const FlippedEdge<E, N, V> * FlippedEdge::getBidiEdge() const` [const]
    /// Returns the bidirectional edge
    // const FlippedEdge<E, N, V>* getBidiEdge() const {
    //         return this->getOriginalEdge()->getBidiEdge()->getFlippedRoutingEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<FlippedEdge<E, N, V>> {
        self.get_original_edge().bidi_edge().flipped_routing_edge()
    }

    /// C++: `double FlippedEdge::getDistanceTo(const FlippedEdge<E, N, V> *, const bool) const` [const]
    /// Returns the distance to another flipped edge param[in] other The other flipped edge param[in] doBoundaryEstimate The boolean flag indicating whether the distance is estimated using both boundaries or not
    // double getDistanceTo(const FlippedEdge<E, N, V>* other, const bool doBoundaryEstimate = false) const {
    //         return this->getOriginalEdge()->getDistanceTo(other->getOriginalEdge(), doBoundaryEstimate);
    //     }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<FlippedEdge<E, N, V>>, doBoundaryEstimate: bool) -> f64 {
        self.get_original_edge().distance_to(other.get_original_edge(), doBoundaryEstimate)
    }

    /// C++: `double FlippedEdge::getMinimumTravelTime(const V *const) const` [const]
    /// Returns the minimum travel time
    // double getMinimumTravelTime(const V* const veh) const {
    //         return this->getOriginalEdge()->getMinimumTravelTime(veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, veh: Ptr<V>) -> f64 {
        self.get_original_edge().minimum_travel_time(veh)
    }

    /// C++: `double FlippedEdge::getTimePenalty() const` [const]
    /// Returns the time penalty
    // double getTimePenalty() const {
    //         return this->getOriginalEdge()->getTimePenalty();
    //     }
    #[allow(unused_variables)]
    pub fn get_time_penalty(&self) -> f64 {
        self.get_original_edge().time_penalty()
    }

    /// C++: `bool FlippedEdge::hasLoadedTravelTimes() const` [const]
    /// Returns a boolean flag indicating whether this edge has loaded travel times or not
    // bool hasLoadedTravelTimes() const {
    //         return this->getOriginalEdge()->hasLoadedTravelTimes();
    //     }
    #[allow(unused_variables)]
    pub fn has_loaded_travel_times(&self) -> bool {
        // shortcut: needs router trait bounds (lead), E::hasLoadedTravelTimes is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::hasLoadedTravelTimes")
    }

    /// C++: `double FlippedEdge::getSpeedLimit() const` [const]
    /// Returns the speed allowed on this edge
    // double getSpeedLimit() const {
    //         return this->getOriginalEdge()->getSpeedLimit();
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        self.get_original_edge().speed_limit()
    }

    /// C++: `double FlippedEdge::getLengthGeometryFactor() const` [const]
    /// Returns a lower bound on shape.length() / myLength
    // double getLengthGeometryFactor() const {
    //         return this->getOriginalEdge()->getLengthGeometryFactor();
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        self.get_original_edge().length_geometry_factor()
    }

    /// C++: `int FlippedEdge::getPriority() const` [const]
    /// Returns the edge priority (road class)
    // int getPriority() const {
    //         return this->getOriginalEdge()->getPriority();
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        // shortcut: needs router trait bounds (lead), E::getPriority is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getPriority")
    }

}

