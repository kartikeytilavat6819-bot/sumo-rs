//! Port of utils/router/PublicTransportEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// the public transport edge type connecting the stop edges
/// C++ class `PublicTransportEdge` (utils/router/PublicTransportEdge.h)
#[repr(C)]
pub struct PublicTransportEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    pub mySchedules: BTreeMap<SUMOTime, PublicTransportEdge_Schedule>,
    pub myEntryStop: Ptr<IntermodalEdge<E, L, N, V>>,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for PublicTransportEdge<E, L, N, V> {
    type Target = IntermodalEdge<E, L, N, V>;
    fn deref(&self) -> &IntermodalEdge<E, L, N, V> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for PublicTransportEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut IntermodalEdge<E, L, N, V> {
        &mut self.base
    }
}

impl<E, L, N, V> PublicTransportEdge<E, L, N, V> {
    /// C++: `void PublicTransportEdge::PublicTransportEdge<E, L, N, V>(const std::string, int, const IntermodalEdge<E, L, N, V> *, const E *, const std::string &, const double)`
    // PublicTransportEdge(const std::string id, int numericalID, const IntermodalEdge<E, L, N, V>* entryStop, const E* endEdge, const std::string& line, const double length) :
    //         IntermodalEdge<E, L, N, V>(line + ":" + (id != "" ? id : endEdge->getID()), numericalID, endEdge, line, length), myEntryStop(entryStop) { }
    #[allow(unused_variables)]
    pub fn new(id: String, numericalID: i32, entryStop: Ptr<IntermodalEdge<E, L, N, V>>, endEdge: Ptr<E>, line: &str, length: f64) -> Self {
        todo!("port PublicTransportEdge::PublicTransportEdge<E, L, N, V>")
    }

    /// C++: `bool PublicTransportEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool /* allEdges */) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        todo!("port PublicTransportEdge::includeInRoute")
    }

    /// C++: `bool PublicTransportEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         return (trip->modeSet & SVC_BUS) == 0;
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        todo!("port PublicTransportEdge::prohibits")
    }

    /// C++: `const IntermodalEdge<E, L, N, V> * PublicTransportEdge::getEntryStop() const` [const]
    // const IntermodalEdge<E, L, N, V>* getEntryStop() const {
    //         return myEntryStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_entry_stop(&self) -> Ptr<IntermodalEdge<E, L, N, V>> {
        todo!("port PublicTransportEdge::getEntryStop")
    }

    /// C++: `bool PublicTransportEdge::hasSchedule(const SUMOTime) const` [const]
    // bool hasSchedule(const SUMOTime begin) const {
    //         return mySchedules.find(begin) != mySchedules.end();
    //     }
    #[allow(unused_variables)]
    pub fn has_schedule(&self, begin: SUMOTime) -> bool {
        todo!("port PublicTransportEdge::hasSchedule")
    }

    /// C++: `void PublicTransportEdge::addSchedule(const std::string, const SUMOTime, const long long, const SUMOTime, const SUMOTime)`
    // void addSchedule(const std::string id, const SUMOTime begin, const long long int repetitionNumber, const SUMOTime period, const SUMOTime travelTime) {
    //         // try to merge with existing vehicle or flow
    //         bool found = false;
    //         for (auto& it : mySchedules) {
    //             Schedule& s = it.second;
    //             if (travelTime == s.travelTime) {
    //                 if (repetitionNumber == -1 && s.repetitionNumber == 1) {
    //                     if (begin > s.begin) {
    //                         s.period = begin - s.begin;
    //                         found = true;
    //                     }
    //                 } else if (begin == s.begin + s.repetitionNumber * s.period) {
    //                     found = true;
    //                 }
    //                 if (found) {
    //                     s.repetitionNumber += MAX2(repetitionNumber, 1ll);
    //                     s.ids.push_back(id);
    //                     break;
    //                 }
    //             }
    //         }
    //         if (!found) {
    //             mySchedules.insert(std::make_pair(begin, Schedule(id, begin, MAX2(repetitionNumber, 1ll), MAX2<SUMOTime>(period, 1), travelTime)));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn add_schedule(&mut self, id: String, begin: SUMOTime, repetitionNumber: i64, period: SUMOTime, travelTime: SUMOTime) {
        todo!("port PublicTransportEdge::addSchedule")
    }

    /// C++: `double PublicTransportEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTime(const IntermodalTrip<E, N, V>* const /* trip */, double time) const {
    //         SUMOTime minArrival = SUMOTime_MAX;
    //         const SUMOTime step = TIME2STEPS(time);
    //         for (typename std::multimap<SUMOTime, Schedule>::const_iterator it = mySchedules.begin(); it != mySchedules.end(); ++it) {
    //             const Schedule& s = it->second;
    //             if (it->first > minArrival) {
    //                 break;
    //             }
    //             const SUMOTime offset = MAX2<SUMOTime>(0, step - s.begin);
    //             int running = (int)(offset / s.period);
    //             if (offset % s.period != 0) {
    //                 running++;
    //             }
    //             if (running < s.repetitionNumber) {
    //                 const SUMOTime nextDepart = s.begin + running * s.period;
    //                 minArrival = MIN2(nextDepart + s.travelTime, minArrival);
    //                 //std::cout << " edge=" << myEntryStop->getID() << "->" << this->getID() << " beg=" << s.begin << " end=" << s.end
    //                 //    << " atTime=" << time
    //                 //    << " running=" << running << " nextDepart=" << nextDepart
    //                 //    << " minASec=" << minArrivalSec << " travelTime=" << minArrivalSec - time << "\n";
    //             }
    //         }
    //         if (minArrival != SUMOTime_MAX) {
    //             return STEPS2TIME(minArrival - step);
    //         } else {
    //             // indicate failure
    //             return std::numeric_limits<double>::max();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, arg0: Ptr<IntermodalTrip<E, N, V>>, time: f64) -> f64 {
        todo!("port PublicTransportEdge::getTravelTime")
    }

    /// C++: `double PublicTransportEdge::getIntended(const double, std::string &) const` [const]
    // double getIntended(const double time, std::string& intended) const {
    //         /// @note: duplicates some code of getTravelTime()
    //         SUMOTime minArrival = SUMOTime_MAX;
    //         double bestDepartTime = std::numeric_limits<double>::max();
    //         const SUMOTime step = TIME2STEPS(time);
    //         for (typename std::multimap<SUMOTime, Schedule>::const_iterator it = mySchedules.begin(); it != mySchedules.end(); ++it) {
    //             const Schedule& s = it->second;
    //             if (it->first > minArrival) {
    //                 break;
    //             }
    //             const SUMOTime offset = MAX2<SUMOTime>(0, step - s.begin);
    //             int running = (int)(offset / s.period);
    //             if (offset % s.period != 0) {
    //                 running++;
    //             }
    //             if (running < s.repetitionNumber) {
    //                 const SUMOTime nextDepart = s.begin + running * s.period;
    //                 if (nextDepart + s.travelTime < minArrival) {
    //                     minArrival = nextDepart + s.travelTime;
    //                     bestDepartTime = STEPS2TIME(nextDepart);
    //                     // see naming scheme inMSInsertionControl::determineCandidates()
    //                     if (s.ids.size() == 1 || running >= (int)s.ids.size()) {
    //                         intended = s.repetitionNumber == 1 ? s.ids[0] : s.ids[0] + "." + toString(running);
    //                     } else {
    //                         intended = s.ids[running];
    //                     }
    //                 }
    //             }
    //         }
    //         return bestDepartTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_intended(&self, time: f64, intended: &mut String) -> f64 {
        todo!("port PublicTransportEdge::getIntended")
    }

}

/// C++ class `PublicTransportEdge::Schedule` (utils/router/PublicTransportEdge.h)
pub struct PublicTransportEdge_Schedule {
    pub ids: Vec<String>,
    pub begin: SUMOTime,
    pub repetitionNumber: i64,
    pub period: SUMOTime,
    pub travelTime: SUMOTime,
}

impl PublicTransportEdge_Schedule {
    /// C++: `void Schedule::Schedule(const std::string &, const SUMOTime, const long long, const SUMOTime, const SUMOTime)`
    // Schedule(const std::string& _id, const SUMOTime _begin, const long long int _repetitionNumber, const SUMOTime _period, const SUMOTime _travelTime)
    //             : ids({
    //             _id
    //         }), begin(_begin), repetitionNumber(_repetitionNumber), period(_period), travelTime(_travelTime) {}
    #[allow(unused_variables)]
    pub fn new(_id: &str, _begin: SUMOTime, _repetitionNumber: i64, _period: SUMOTime, _travelTime: SUMOTime) -> Self {
        todo!("port Schedule::Schedule")
    }

}

