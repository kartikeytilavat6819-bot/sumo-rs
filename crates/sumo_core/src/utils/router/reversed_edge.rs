//! Port of utils/router/ReversedEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;
use utils::common::sumo_vehicle_class::SVC_IGNORING;

/// C++ typedef `std::vector<std::pair<const ReversedEdge<E, V> *, const ReversedEdge<E, V> *>> ReversedEdge::ConstEdgePairVector`
pub type ReversedEdge_ConstEdgePairVector<E, V> = Vec<(Ptr<ReversedEdge<E, V>>, Ptr<ReversedEdge<E, V>>)>;

/// the edge type representing backward edges
/// C++ class `ReversedEdge` (utils/router/ReversedEdge.h)
pub struct ReversedEdge<E, V> {
    pub myOriginal: Ptr<E>,
    /// The successors available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, ReversedEdge_ConstEdgePairVector<E, V>>,
    pub myViaSuccessors: ReversedEdge_ConstEdgePairVector<E, V>,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> ReversedEdge<E, V> {
    /// C++: `void ReversedEdge::ReversedEdge<E, V>(const E *)`
    // ReversedEdge(const E* orig) : myOriginal(orig) {
    //     }
    #[allow(unused_variables)]
    pub fn new(orig: Ptr<E>) -> Self {
        Self {
            myOriginal: orig,
            myClassesViaSuccessorMap: BTreeMap::new(),
            myViaSuccessors: Vec::new(),
            _p: std::marker::PhantomData,
        }
    }
}

impl<E, V> ReversedEdge<E, V>
where
    E: RouterEdge<V>,
{
    /// C++: `void ReversedEdge::init()`
    // void init() {
    //         if (!myOriginal->isInternal()) {
    //             for (const auto& viaPair : myOriginal->getViaSuccessors()) {
    //                 const ReversedEdge<E, V>* revSource = viaPair.first->getReversedRoutingEdge();
    //                 const E* via = viaPair.second;
    //                 const ReversedEdge<E, V>* preVia = nullptr;
    //                 while (via != nullptr && via->isInternal()) {
    //                     via->getReversedRoutingEdge()->myViaSuccessors.push_back(std::make_pair(this, preVia));
    //                     preVia = via->getReversedRoutingEdge();
    //                     via = via->getViaSuccessors().front().second;
    //                 }
    //                 revSource->myViaSuccessors.push_back(std::make_pair(this, preVia));
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn init(&mut self) {
        if !self.myOriginal.is_internal() {
            let this: Ptr<ReversedEdge<E, V>> = Ptr::from_mut(self);
            let original = self.myOriginal;
            for viaPair in original.via_successors(SVC_IGNORING, false).to_vec() {
                let revSource = viaPair.0.reversed_routing_edge();
                let mut via = viaPair.1;
                let mut preVia: Ptr<ReversedEdge<E, V>> = Ptr::null();
                while !via.is_null() && via.is_internal() {
                    via.reversed_routing_edge().m().myViaSuccessors.push((this, preVia));
                    preVia = via.reversed_routing_edge();
                    via = via.via_successors(SVC_IGNORING, false)[0].1;
                }
                revSource.m().myViaSuccessors.push((this, preVia));
            }
        }
    }

    /// C++: `const E * ReversedEdge::getOriginalEdge() const` [const]
    /// Returns the original edge
    // const E* getOriginalEdge() const {
    //         return myOriginal;
    //     }
    #[allow(unused_variables)]
    pub fn get_original_edge(&self) -> Ptr<E> {
        self.myOriginal
    }

    /// C++: `int ReversedEdge::getNumericalID() const` [const]
    /// Returns the index (numeric id) of the edge
    // int getNumericalID() const {
    //         return myOriginal->getNumericalID();
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        self.myOriginal.numerical_id()
    }

    /// C++: `const std::string & ReversedEdge::getID() const` [const]
    /// Returns the id of the edge
    // const std::string& getID() const {
    //         return myOriginal->getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        self.myOriginal.id()
    }

    /// C++: `double ReversedEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myOriginal->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myOriginal.length()
    }

    /// C++: `const ReversedEdge<E, V> * ReversedEdge::getBidiEdge() const` [const]
    // const ReversedEdge* getBidiEdge() const {
    //         return myOriginal->getBidiEdge()->getReversedRoutingEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<ReversedEdge<E, V>> {
        self.myOriginal.bidi_edge().reversed_routing_edge()
    }

    /// C++: `bool ReversedEdge::isInternal() const` [const]
    // bool isInternal() const {
    //         return myOriginal->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        self.myOriginal.is_internal()
    }

    /// C++: `bool ReversedEdge::prohibits(const V *const) const` [const]
    // inline bool prohibits(const V* const vehicle) const {
    //         return myOriginal->prohibits(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<V>) -> bool {
        // C++ ROEdge::prohibits(vehicle) defaults checkRestrictions to false
        self.myOriginal.prohibits(vehicle, false)
    }

    /// C++: `bool ReversedEdge::restricts(const V *const) const` [const]
    // inline bool restricts(const V* const vehicle) const {
    //         return myOriginal->restricts(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<V>) -> bool {
        self.myOriginal.restricts(vehicle)
    }

    /// C++: `double ReversedEdge::getTravelTimeStatic(const ReversedEdge<E, V> *const, const V *const, double)` [static]
    // static inline double getTravelTimeStatic(const ReversedEdge<E, V>* const edge, const V* const veh, double time) {
    //         return edge->myOriginal->getTravelTime(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<ReversedEdge<E, V>>, veh: Ptr<V>, time: f64) -> f64 {
        edge.myOriginal.travel_time(veh, time)
    }

    /// C++: `const ConstEdgePairVector & ReversedEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    // const ConstEdgePairVector& getViaSuccessors(SUMOVehicleClass vClass = SVC_IGNORING, bool ignoreTransientPermissions = false) const {
    //         UNUSED_PARAMETER(ignoreTransientPermissions); // @todo this should be changed (somewhat hidden by #14756)
    //         if (vClass == SVC_IGNORING || myOriginal->isTazConnector()) { // || !MSNet::getInstance()->hasPermissions()) {
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
    //         ConstEdgePairVector& result = myClassesViaSuccessorMap[vClass];
    //         // this vClass is requested for the first time. rebuild all successors
    //         for (const auto& viaPair : myViaSuccessors) {
    //             if (viaPair.first->myOriginal->isTazConnector() || viaPair.first->myOriginal->isConnectedTo(*myOriginal, vClass)) {
    //                 result.push_back(viaPair);
    //             }
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ReversedEdge_ConstEdgePairVector<E, V> {
        if vClass == SVC_IGNORING || self.myOriginal.is_taz_connector() {
            return &self.myViaSuccessors;
        }
        if let Some(cached) = self.myClassesViaSuccessorMap.get(&vClass) {
            return cached;
        }
        // C++ mutable cache: the const method fills myClassesViaSuccessorMap[vClass] in place
        let me: &mut ReversedEdge<E, V> = Ptr::new(self).get_mut().unwrap();
        let mut result: ReversedEdge_ConstEdgePairVector<E, V> = Vec::new();
        for viaPair in self.myViaSuccessors.iter() {
            if viaPair.0.myOriginal.is_taz_connector() || viaPair.0.myOriginal.is_connected_to(&self.myOriginal, vClass, ignoreTransientPermissions) {
                result.push(*viaPair);
            }
        }
        me.myClassesViaSuccessorMap.insert(vClass, result);
        me.myClassesViaSuccessorMap.get(&vClass).unwrap()
    }

}

