//! Port of utils/router/PublicTransportEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

/// the public transport edge type connecting the stop edges
/// C++ class `PublicTransportEdge` (utils/router/PublicTransportEdge.h)
#[repr(C)]
pub struct PublicTransportEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    // multimap in C++: several schedules may share a begin time, kept in insertion order
    pub mySchedules: BTreeMap<SUMOTime, Vec<PublicTransportEdge_Schedule>>,
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
    pub fn new(id: String, numericalID: i32, entryStop: Ptr<IntermodalEdge<E, L, N, V>>, endEdge: Ptr<E>, line: &str, length: f64) -> Self
    where
        E: RouterEdge<V>,
    {
        let base_id = format!("{}:{}", line, if id != "" { id } else { endEdge.id().to_string() });
        Self {
            base: IntermodalEdge::new(base_id, numericalID, endEdge, line, length),
            mySchedules: BTreeMap::new(),
            myEntryStop: entryStop,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `bool PublicTransportEdge::includeInRoute(bool) const` [const]
    // bool includeInRoute(bool /* allEdges */) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn include_in_route(&self, arg0: bool) -> bool {
        true
    }

    /// C++: `bool PublicTransportEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         return (trip->modeSet & SVC_BUS) == 0;
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        (trip.modeSet & SVC_BUS) == 0
    }

    /// C++: `const IntermodalEdge<E, L, N, V> * PublicTransportEdge::getEntryStop() const` [const]
    // const IntermodalEdge<E, L, N, V>* getEntryStop() const {
    //         return myEntryStop;
    //     }
    #[allow(unused_variables)]
    pub fn get_entry_stop(&self) -> Ptr<IntermodalEdge<E, L, N, V>> {
        self.myEntryStop
    }

    /// C++: `bool PublicTransportEdge::hasSchedule(const SUMOTime) const` [const]
    // bool hasSchedule(const SUMOTime begin) const {
    //         return mySchedules.find(begin) != mySchedules.end();
    //     }
    #[allow(unused_variables)]
    pub fn has_schedule(&self, begin: SUMOTime) -> bool {
        self.mySchedules.contains_key(&begin)
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
        // try to merge with existing vehicle or flow
        let mut found = false;
        for s in self.mySchedules.values_mut().flat_map(|v| v.iter_mut()) {
            if travelTime == s.travelTime {
                if repetitionNumber == -1 && s.repetitionNumber == 1 {
                    if begin > s.begin {
                        s.period = begin - s.begin;
                        found = true;
                    }
                } else if begin == s.begin + s.repetitionNumber * s.period {
                    found = true;
                }
                if found {
                    s.repetitionNumber += std::cmp::max(repetitionNumber, 1);
                    s.ids.push(id.clone());
                    break;
                }
            }
        }
        if !found {
            self.mySchedules.entry(begin).or_default().push(PublicTransportEdge_Schedule::new(&id, begin, std::cmp::max(repetitionNumber, 1), std::cmp::max(period, 1), travelTime));
        }
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
        let mut minArrival: SUMOTime = SUMOTIME_MAX;
        let step = time2steps(time);
        for (key, s) in self.mySchedules.iter().flat_map(|(k, v)| v.iter().map(move |s| (*k, s))) {
            if key > minArrival {
                break;
            }
            let offset = std::cmp::max(0, step - s.begin);
            let mut running = (offset / s.period) as i32;
            if offset % s.period != 0 {
                running += 1;
            }
            if (running as i64) < s.repetitionNumber {
                let nextDepart = s.begin + running as i64 * s.period;
                minArrival = std::cmp::min(nextDepart + s.travelTime, minArrival);
            }
        }
        if minArrival != SUMOTIME_MAX {
            steps2time(minArrival - step)
        } else {
            // indicate failure
            f64::MAX
        }
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
        // @note: duplicates some code of getTravelTime()
        let mut minArrival: SUMOTime = SUMOTIME_MAX;
        let mut bestDepartTime = f64::MAX;
        let step = time2steps(time);
        for (key, s) in self.mySchedules.iter().flat_map(|(k, v)| v.iter().map(move |s| (*k, s))) {
            if key > minArrival {
                break;
            }
            let offset = std::cmp::max(0, step - s.begin);
            let mut running = (offset / s.period) as i32;
            if offset % s.period != 0 {
                running += 1;
            }
            if (running as i64) < s.repetitionNumber {
                let nextDepart = s.begin + running as i64 * s.period;
                if nextDepart + s.travelTime < minArrival {
                    minArrival = nextDepart + s.travelTime;
                    bestDepartTime = steps2time(nextDepart);
                    // see naming scheme inMSInsertionControl::determineCandidates()
                    if s.ids.len() == 1 || running >= s.ids.len() as i32 {
                        *intended = if s.repetitionNumber == 1 { s.ids[0].clone() } else { format!("{}.{}", s.ids[0], running) };
                    } else {
                        *intended = s.ids[running as usize].clone();
                    }
                }
            }
        }
        bestDepartTime
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
        Self {
            ids: vec![_id.to_string()],
            begin: _begin,
            repetitionNumber: _repetitionNumber,
            period: _period,
            travelTime: _travelTime,
        }
    }

}

