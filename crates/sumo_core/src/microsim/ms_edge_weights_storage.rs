//! Port of microsim/MSEdgeWeightsStorage.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A storage for edge travel times and efforts
/// C++ class `MSEdgeWeightsStorage` (microsim/MSEdgeWeightsStorage.h)
pub struct MSEdgeWeightsStorage {
    /// A map of edge->time->travel time
    pub myTravelTimes: BTreeMap<Ptr<MSEdge>, Unmapped /* ValueTimeLine<double> */>,
    /// A map of edge->time->effort
    pub myEfforts: BTreeMap<Ptr<MSEdge>, Unmapped /* ValueTimeLine<double> */>,
}

impl MSEdgeWeightsStorage {
    /// C++: `void MSEdgeWeightsStorage::MSEdgeWeightsStorage()`
    /// Constructor
    // MSEdgeWeightsStorage::MSEdgeWeightsStorage() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSEdgeWeightsStorage::MSEdgeWeightsStorage")
    }

    /// C++: `void MSEdgeWeightsStorage::~MSEdgeWeightsStorage()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSEdgeWeightsStorage::~MSEdgeWeightsStorage")
    }

    /// C++: `bool MSEdgeWeightsStorage::retrieveExistingTravelTime(const MSEdge *const, const double, double &) const` [const]
    /// Returns a travel time for an edge and time if stored
    // bool
    // MSEdgeWeightsStorage::retrieveExistingTravelTime(const MSEdge* const e, const double t, double& value) const {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::const_iterator i = myTravelTimes.find(e);
    //     if (i == myTravelTimes.end()) {
    //         return false;
    //     }
    //     const ValueTimeLine<double>& tl = (*i).second;
    //     if (!tl.describesTime(t)) {
    //         return false;
    //     }
    //     value = tl.getValue(t);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_existing_travel_time(&self, e: Ptr<MSEdge>, t: f64, value: &mut f64) -> bool {
        todo!("port MSEdgeWeightsStorage::retrieveExistingTravelTime")
    }

    /// C++: `bool MSEdgeWeightsStorage::retrieveExistingEffort(const MSEdge *const, const double, double &) const` [const]
    /// Returns an effort for an edge and time if stored
    // bool
    // MSEdgeWeightsStorage::retrieveExistingEffort(const MSEdge* const e, const double t, double& value) const {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::const_iterator i = myEfforts.find(e);
    //     if (i == myEfforts.end()) {
    //         return false;
    //     }
    //     const ValueTimeLine<double>& tl = (*i).second;
    //     if (!tl.describesTime(t)) {
    //         return false;
    //     }
    //     value = tl.getValue(t);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_existing_effort(&self, e: Ptr<MSEdge>, t: f64, value: &mut f64) -> bool {
        todo!("port MSEdgeWeightsStorage::retrieveExistingEffort")
    }

    /// C++: `void MSEdgeWeightsStorage::addTravelTime(const MSEdge *const, double, double, double)`
    /// Adds a travel time information for an edge and a time span
    // void
    // MSEdgeWeightsStorage::addTravelTime(const MSEdge* const e,
    //                                     double begin, double end,
    //                                     double value) {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::iterator i = myTravelTimes.find(e);
    //     if (i == myTravelTimes.end()) {
    //         myTravelTimes[e] = ValueTimeLine<double>();
    //         i = myTravelTimes.find(e);
    //     }
    //     (*i).second.add(begin, end, value);
    // }
    #[allow(unused_variables)]
    pub fn add_travel_time(&mut self, e: Ptr<MSEdge>, begin: f64, end: f64, value: f64) {
        todo!("port MSEdgeWeightsStorage::addTravelTime")
    }

    /// C++: `void MSEdgeWeightsStorage::addEffort(const MSEdge *const, double, double, double)`
    /// Adds an effort information for an edge and a time span
    // void
    // MSEdgeWeightsStorage::addEffort(const MSEdge* const e,
    //                                 double begin, double end,
    //                                 double value) {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::iterator i = myEfforts.find(e);
    //     if (i == myEfforts.end()) {
    //         myEfforts[e] = ValueTimeLine<double>();
    //         i = myEfforts.find(e);
    //     }
    //     (*i).second.add(begin, end, value);
    // }
    #[allow(unused_variables)]
    pub fn add_effort(&mut self, e: Ptr<MSEdge>, begin: f64, end: f64, value: f64) {
        todo!("port MSEdgeWeightsStorage::addEffort")
    }

    /// C++: `void MSEdgeWeightsStorage::removeTravelTime(const MSEdge *const)`
    /// Removes the travel time information for an edge
    // void
    // MSEdgeWeightsStorage::removeTravelTime(const MSEdge* const e) {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::iterator i = myTravelTimes.find(e);
    //     if (i != myTravelTimes.end()) {
    //         myTravelTimes.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_travel_time(&mut self, e: Ptr<MSEdge>) {
        todo!("port MSEdgeWeightsStorage::removeTravelTime")
    }

    /// C++: `void MSEdgeWeightsStorage::removeEffort(const MSEdge *const)`
    /// Removes the effort information for an edge
    // void
    // MSEdgeWeightsStorage::removeEffort(const MSEdge* const e) {
    //     std::map<const MSEdge*, ValueTimeLine<double> >::iterator i = myEfforts.find(e);
    //     if (i != myEfforts.end()) {
    //         myEfforts.erase(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_effort(&mut self, e: Ptr<MSEdge>) {
        todo!("port MSEdgeWeightsStorage::removeEffort")
    }

    /// C++: `bool MSEdgeWeightsStorage::empty() const` [const]
    /// Returns whether neither travel times nor efforts are stored
    // bool empty() const {
    //         return myTravelTimes.empty() && myEfforts.empty();
    //     }
    #[allow(unused_variables)]
    pub fn empty(&self) -> bool {
        todo!("port MSEdgeWeightsStorage::empty")
    }

    /// C++: `bool MSEdgeWeightsStorage::knowsTravelTime(const MSEdge *const) const` [const]
    /// Returns the information whether any travel time is known for the given edge
    // bool
    // MSEdgeWeightsStorage::knowsTravelTime(const MSEdge* const e) const {
    //     return myTravelTimes.find(e) != myTravelTimes.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_travel_time(&self, e: Ptr<MSEdge>) -> bool {
        todo!("port MSEdgeWeightsStorage::knowsTravelTime")
    }

    /// C++: `bool MSEdgeWeightsStorage::knowsEffort(const MSEdge *const) const` [const]
    /// Returns the information whether any effort is known for the given edge
    // bool
    // MSEdgeWeightsStorage::knowsEffort(const MSEdge* const e) const {
    //     return myEfforts.find(e) != myEfforts.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_effort(&self, e: Ptr<MSEdge>) -> bool {
        todo!("port MSEdgeWeightsStorage::knowsEffort")
    }

    /// C++: `void MSEdgeWeightsStorage::MSEdgeWeightsStorage(const MSEdgeWeightsStorage &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSEdgeWeightsStorage) -> Self {
        todo!("port MSEdgeWeightsStorage::MSEdgeWeightsStorage")
    }

    /// C++: `MSEdgeWeightsStorage & MSEdgeWeightsStorage::operator=(const MSEdgeWeightsStorage &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEdgeWeightsStorage) -> &mut MSEdgeWeightsStorage {
        todo!("port MSEdgeWeightsStorage::operator=")
    }

}

