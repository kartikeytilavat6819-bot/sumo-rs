//! Port of utils/router/ReversedEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        // shortcut: needs router trait bounds (lead), E::isInternal/getViaSuccessors/getReversedRoutingEdge are not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getViaSuccessors")
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
        // shortcut: needs router trait bounds (lead), E::getNumericalID is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getNumericalID")
    }

    /// C++: `const std::string & ReversedEdge::getID() const` [const]
    /// Returns the id of the edge
    // const std::string& getID() const {
    //         return myOriginal->getID();
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        // shortcut: needs router trait bounds (lead), E::getID is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getID")
    }

    /// C++: `double ReversedEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myOriginal->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        // shortcut: needs router trait bounds (lead), E::getLength is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getLength")
    }

    /// C++: `const ReversedEdge<E, V> * ReversedEdge::getBidiEdge() const` [const]
    // const ReversedEdge* getBidiEdge() const {
    //         return myOriginal->getBidiEdge()->getReversedRoutingEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<ReversedEdge<E, V>> {
        // shortcut: needs router trait bounds (lead), E::getBidiEdge is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getBidiEdge")
    }

    /// C++: `bool ReversedEdge::isInternal() const` [const]
    // bool isInternal() const {
    //         return myOriginal->isInternal();
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        // shortcut: needs router trait bounds (lead), E::isInternal is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::isInternal")
    }

    /// C++: `bool ReversedEdge::prohibits(const V *const) const` [const]
    // inline bool prohibits(const V* const vehicle) const {
    //         return myOriginal->prohibits(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<V>) -> bool {
        // shortcut: needs router trait bounds (lead), E::prohibits is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::prohibits")
    }

    /// C++: `bool ReversedEdge::restricts(const V *const) const` [const]
    // inline bool restricts(const V* const vehicle) const {
    //         return myOriginal->restricts(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<V>) -> bool {
        // shortcut: needs router trait bounds (lead), E::restricts is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::restricts")
    }

    /// C++: `double ReversedEdge::getTravelTimeStatic(const ReversedEdge<E, V> *const, const V *const, double)` [static]
    // static inline double getTravelTimeStatic(const ReversedEdge<E, V>* const edge, const V* const veh, double time) {
    //         return edge->myOriginal->getTravelTime(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<ReversedEdge<E, V>>, veh: Ptr<V>, time: f64) -> f64 {
        // shortcut: needs router trait bounds (lead), E::getTravelTime is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getTravelTime")
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
        if vClass == SVC_IGNORING {
            return &self.myViaSuccessors;
        }
        // shortcut: needs router trait bounds (lead), E::isTazConnector/isConnectedTo are not callable on generic E; the cache rebuild also needs Ptr::new(self).m()
        panic!("shortcut: needs router trait bounds (lead), E::isTazConnector")
    }

}

