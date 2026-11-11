//! Port of microsim/MSEdgeWeightsStorage.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cmp::Ordering;
use std::ops::Bound::{Excluded, Unbounded};

/// Key ordering of std::map<double, ...> (times are finite)
#[derive(Clone, Copy, PartialEq)]
struct MSEdgeWeightsStorage_TimeKey(f64);
impl Eq for MSEdgeWeightsStorage_TimeKey {}
impl PartialOrd for MSEdgeWeightsStorage_TimeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MSEdgeWeightsStorage_TimeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

/// C++ `ValueTimeLine<double>` (utils/common/ValueTimeLine.h): time -> (is valid, value)
// shortcut: utils/common/ValueTimeLine<T> is not ported, only the double variant used here is ported (add/getValue/describesTime), upgrade to a generic utils port when it lands
#[derive(Default, Clone)]
pub struct ValueTimeLineDouble {
    myValues: BTreeMap<MSEdgeWeightsStorage_TimeKey, (bool, f64)>,
}

impl ValueTimeLineDouble {
    /// first entry with key > time (std::map::upper_bound)
    fn first_gt(&self, time: f64) -> Option<(&MSEdgeWeightsStorage_TimeKey, &(bool, f64))> {
        self.myValues.range((Excluded(MSEdgeWeightsStorage_TimeKey(time)), Unbounded)).next()
    }
    /// last entry with key <= time (std::prev(std::map::upper_bound))
    fn last_le(&self, time: f64) -> Option<(&MSEdgeWeightsStorage_TimeKey, &(bool, f64))> {
        self.myValues.range(..=MSEdgeWeightsStorage_TimeKey(time)).next_back()
    }
    /// last entry with key < time (std::prev(std::map::lower_bound))
    fn last_lt(&self, time: f64) -> Option<(&MSEdgeWeightsStorage_TimeKey, &(bool, f64))> {
        self.myValues.range(..MSEdgeWeightsStorage_TimeKey(time)).next_back()
    }

    /// C++: `void ValueTimeLine<T>::add(double, double, T)`
    pub fn add(&mut self, begin: f64, end: f64, value: f64) {
        // inserting strictly before the first or after the last interval (includes empty case)
        if self.first_gt(begin).is_none() || self.last_le(end).is_none() {
            self.myValues.insert(MSEdgeWeightsStorage_TimeKey(begin), (true, value));
            self.myValues.insert(MSEdgeWeightsStorage_TimeKey(end), (false, value));
            return;
        }
        // our end already has a value
        if self.myValues.contains_key(&MSEdgeWeightsStorage_TimeKey(end)) {
            self.myValues.retain(|k, _| !(k.0 > begin && k.0 < end));
            self.myValues.insert(MSEdgeWeightsStorage_TimeKey(begin), (true, value));
            return;
        }
        // we have at least one entry strictly before our end
        let oldEndValue = self.last_lt(end).expect("ValueTimeLine::add: entry before end").1;
        self.myValues.retain(|k, _| !(k.0 > begin && k.0 < end));
        self.myValues.insert(MSEdgeWeightsStorage_TimeKey(begin), (true, value));
        self.myValues.insert(MSEdgeWeightsStorage_TimeKey(end), oldEndValue);
    }

    /// C++: `T ValueTimeLine<T>::getValue(double) const`
    pub fn get_value(&self, time: f64) -> f64 {
        match self.last_le(time) {
            Some((_, &(_, v))) => v,
            None => panic!("ValueTimeLine::getValue: no value set for time {}", time),
        }
    }

    /// C++: `bool ValueTimeLine<T>::describesTime(double) const`
    pub fn describes_time(&self, time: f64) -> bool {
        match self.last_le(time) {
            Some((_, &(valid, _))) => valid,
            None => false,
        }
    }
}

/// A storage for edge travel times and efforts
/// C++ class `MSEdgeWeightsStorage` (microsim/MSEdgeWeightsStorage.h)
pub struct MSEdgeWeightsStorage {
    /// A map of edge->time->travel time
    pub myTravelTimes: BTreeMap<Ptr<MSEdge>, ValueTimeLineDouble>,
    /// A map of edge->time->effort
    pub myEfforts: BTreeMap<Ptr<MSEdge>, ValueTimeLineDouble>,
}

impl MSEdgeWeightsStorage {
    /// C++: `void MSEdgeWeightsStorage::MSEdgeWeightsStorage()`
    /// Constructor
    // MSEdgeWeightsStorage::MSEdgeWeightsStorage() {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSEdgeWeightsStorage {
            myTravelTimes: BTreeMap::new(),
            myEfforts: BTreeMap::new(),
        }
    }

    /// C++: `void MSEdgeWeightsStorage::~MSEdgeWeightsStorage()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: the maps release themselves
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
        match self.myTravelTimes.get(&e) {
            None => false,
            Some(tl) => {
                if !tl.describes_time(t) {
                    return false;
                }
                *value = tl.get_value(t);
                true
            }
        }
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
        match self.myEfforts.get(&e) {
            None => false,
            Some(tl) => {
                if !tl.describes_time(t) {
                    return false;
                }
                *value = tl.get_value(t);
                true
            }
        }
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
        self.myTravelTimes.entry(e).or_default().add(begin, end, value);
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
        self.myEfforts.entry(e).or_default().add(begin, end, value);
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
        self.myTravelTimes.remove(&e);
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
        self.myEfforts.remove(&e);
    }

    /// C++: `bool MSEdgeWeightsStorage::empty() const` [const]
    /// Returns whether neither travel times nor efforts are stored
    // bool empty() const {
    //         return myTravelTimes.empty() && myEfforts.empty();
    //     }
    #[allow(unused_variables)]
    pub fn empty(&self) -> bool {
        self.myTravelTimes.is_empty() && self.myEfforts.is_empty()
    }

    /// C++: `bool MSEdgeWeightsStorage::knowsTravelTime(const MSEdge *const) const` [const]
    /// Returns the information whether any travel time is known for the given edge
    // bool
    // MSEdgeWeightsStorage::knowsTravelTime(const MSEdge* const e) const {
    //     return myTravelTimes.find(e) != myTravelTimes.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_travel_time(&self, e: Ptr<MSEdge>) -> bool {
        self.myTravelTimes.contains_key(&e)
    }

    /// C++: `bool MSEdgeWeightsStorage::knowsEffort(const MSEdge *const) const` [const]
    /// Returns the information whether any effort is known for the given edge
    // bool
    // MSEdgeWeightsStorage::knowsEffort(const MSEdge* const e) const {
    //     return myEfforts.find(e) != myEfforts.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_effort(&self, e: Ptr<MSEdge>) -> bool {
        self.myEfforts.contains_key(&e)
    }

    /// C++: `void MSEdgeWeightsStorage::MSEdgeWeightsStorage(const MSEdgeWeightsStorage &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSEdgeWeightsStorage) -> Self {
        panic!("MSEdgeWeightsStorage copy constructor is invalidated")
    }

    /// C++: `MSEdgeWeightsStorage & MSEdgeWeightsStorage::operator=(const MSEdgeWeightsStorage &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEdgeWeightsStorage) -> &mut MSEdgeWeightsStorage {
        panic!("MSEdgeWeightsStorage assignment operator is invalidated")
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_time_line_add_splits_interval() {
        // C++ ValueTimeLine<double>: add(0,10,1) then add(3,5,2) splits the first interval
        let mut tl = ValueTimeLineDouble::default();
        tl.add(0., 10., 1.);
        tl.add(3., 5., 2.);
        assert_eq!(tl.get_value(4.), 2.);
        assert_eq!(tl.get_value(6.), 1.);
        assert!(tl.describes_time(9.5));
        assert!(!tl.describes_time(10.));
        assert!(!tl.describes_time(-0.5));
    }
}
