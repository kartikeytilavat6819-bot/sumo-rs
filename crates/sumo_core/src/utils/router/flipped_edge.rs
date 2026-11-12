//! Port of utils/router/FlippedEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::pair<const FlippedEdge<E, N, V> *, const FlippedEdge<E, N, V> *>> FlippedEdge::ConstFlippedEdgePairVector`
pub type FlippedEdge_ConstFlippedEdgePairVector = Unmapped /* std::vector<std::pair<const FlippedEdge<E, N, V> *, const FlippedEdge<E, N, V> *>> */;

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
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, FlippedEdge_ConstFlippedEdgePairVector>,
    pub myViaSuccessors: FlippedEdge_ConstFlippedEdgePairVector,
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

impl<E, N, V> FlippedEdge<E, N, V> {
    /// C++: `void FlippedEdge::FlippedEdge<E, N, V>(const E *)`
    /// Constructor
    // FlippedEdge(const E* originalEdge) :
    //         ReversedEdge<E, V>(originalEdge),
    //         myFromJunction(originalEdge->getToJunction()->getFlippedRoutingNode()),
    //         myToJunction(originalEdge->getFromJunction()->getFlippedRoutingNode())
    //     {}
    #[allow(unused_variables)]
    pub fn new(originalEdge: Ptr<E>) -> Self {
        todo!("port FlippedEdge::FlippedEdge<E, N, V>")
    }

    /// C++: `void FlippedEdge::~FlippedEdge<E, N, V>()`
    /// Destructor
    // ~FlippedEdge() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port FlippedEdge::~FlippedEdge<E, N, V>")
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
        todo!("port FlippedEdge::init")
    }

    /// C++: `const FlippedNode<E, N, V> * FlippedEdge::getFromJunction() const` [const]
    /// Returns the from-junction
    // const FlippedNode<E, N, V>* getFromJunction() const {
    //         return myFromJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<FlippedNode<E, N, V>> {
        todo!("port FlippedEdge::getFromJunction")
    }

    /// C++: `const FlippedNode<E, N, V> * FlippedEdge::getToJunction() const` [const]
    /// Returns the to-junction
    // const FlippedNode<E, N, V>* getToJunction() const {
    //         return myToJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<FlippedNode<E, N, V>> {
        todo!("port FlippedEdge::getToJunction")
    }

    /// C++: `double FlippedEdge::getTravelTimeStatic(const FlippedEdge<E, N, V> *const, const V *const, double)` [static]
    /// Returns the time for travelling on the given edge with the given vehicle at the given time
    // static double getTravelTimeStatic(const FlippedEdge<E, N, V>* const edge, const V* const veh, double time) {
    //         return edge->getOriginalEdge()->getTravelTime(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<FlippedEdge<E, N, V>>, veh: Ptr<V>, time: f64) -> f64 {
        todo!("port FlippedEdge::getTravelTimeStatic")
    }

    /// C++: `double FlippedEdge::getTravelTimeStaticRandomized(const FlippedEdge<E, N, V> *const, const V *const, double)` [static]
    /// Returns the randomized time for travelling on the given edge with the given vehicle at the given time
    // static double getTravelTimeStaticRandomized(const FlippedEdge<E, N, V>* const edge, const V* const veh, double time) {
    //         return edge->getOriginalEdge()->getTravelTimeStaticRandomized(edge->getOriginalEdge(), veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<FlippedEdge<E, N, V>>, veh: Ptr<V>, time: f64) -> f64 {
        todo!("port FlippedEdge::getTravelTimeStaticRandomized")
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
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &FlippedEdge_ConstFlippedEdgePairVector {
        todo!("port FlippedEdge::getViaSuccessors")
    }

    /// C++: `const FlippedEdge<E, N, V> * FlippedEdge::getBidiEdge() const` [const]
    /// Returns the bidirectional edge
    // const FlippedEdge<E, N, V>* getBidiEdge() const {
    //         return this->getOriginalEdge()->getBidiEdge()->getFlippedRoutingEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<FlippedEdge<E, N, V>> {
        todo!("port FlippedEdge::getBidiEdge")
    }

    /// C++: `double FlippedEdge::getDistanceTo(const FlippedEdge<E, N, V> *, const bool) const` [const]
    /// Returns the distance to another flipped edge param[in] other The other flipped edge param[in] doBoundaryEstimate The boolean flag indicating whether the distance is estimated using both boundaries or not
    // double getDistanceTo(const FlippedEdge<E, N, V>* other, const bool doBoundaryEstimate = false) const {
    //         return this->getOriginalEdge()->getDistanceTo(other->getOriginalEdge(), doBoundaryEstimate);
    //     }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<FlippedEdge<E, N, V>>, doBoundaryEstimate: bool) -> f64 {
        todo!("port FlippedEdge::getDistanceTo")
    }

    /// C++: `double FlippedEdge::getMinimumTravelTime(const V *const) const` [const]
    /// Returns the minimum travel time
    // double getMinimumTravelTime(const V* const veh) const {
    //         return this->getOriginalEdge()->getMinimumTravelTime(veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, veh: Ptr<V>) -> f64 {
        todo!("port FlippedEdge::getMinimumTravelTime")
    }

    /// C++: `double FlippedEdge::getTimePenalty() const` [const]
    /// Returns the time penalty
    // double getTimePenalty() const {
    //         return this->getOriginalEdge()->getTimePenalty();
    //     }
    #[allow(unused_variables)]
    pub fn get_time_penalty(&self) -> f64 {
        todo!("port FlippedEdge::getTimePenalty")
    }

    /// C++: `bool FlippedEdge::hasLoadedTravelTimes() const` [const]
    /// Returns a boolean flag indicating whether this edge has loaded travel times or not
    // bool hasLoadedTravelTimes() const {
    //         return this->getOriginalEdge()->hasLoadedTravelTimes();
    //     }
    #[allow(unused_variables)]
    pub fn has_loaded_travel_times(&self) -> bool {
        todo!("port FlippedEdge::hasLoadedTravelTimes")
    }

    /// C++: `double FlippedEdge::getSpeedLimit() const` [const]
    /// Returns the speed allowed on this edge
    // double getSpeedLimit() const {
    //         return this->getOriginalEdge()->getSpeedLimit();
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        todo!("port FlippedEdge::getSpeedLimit")
    }

    /// C++: `double FlippedEdge::getLengthGeometryFactor() const` [const]
    /// Returns a lower bound on shape.length() / myLength
    // double getLengthGeometryFactor() const {
    //         return this->getOriginalEdge()->getLengthGeometryFactor();
    //     }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        todo!("port FlippedEdge::getLengthGeometryFactor")
    }

    /// C++: `int FlippedEdge::getPriority() const` [const]
    /// Returns the edge priority (road class)
    // int getPriority() const {
    //         return this->getOriginalEdge()->getPriority();
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        todo!("port FlippedEdge::getPriority")
    }

}

