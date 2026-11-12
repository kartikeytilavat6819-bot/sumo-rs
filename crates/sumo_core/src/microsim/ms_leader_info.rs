//! Port of microsim/MSLeaderInfo.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::delta_t;
use utils::common::to_string::f64_to_string_prec;
use utils::common::util_exceptions::ProcessError;
use utils::geom::{NUMERICAL_EPS, POSITION_EPS};

// shortcut: MSGlobals::gLateralResolution is a static that is not ported yet (ms_globals.rs has no statics), this stub returns the C++ default -1 (unit-test default, sublane model off), upgrade when MSGlobals statics are ported
pub fn g_lateral_resolution() -> f64 {
    -1.
}

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// Named::getIDSecure(veh) for a possibly null vehicle pointer
fn get_id_secure_veh(veh: Ptr<MSVehicle>) -> String {
    veh.get().map_or_else(|| "NULL".to_string(), |v| v.get_id().to_string())
}

/// C++ typedef `std::pair<const MSVehicle *, double> CLeaderDist`
pub type CLeaderDist = (Ptr<MSVehicle>, f64);

/// C++ typedef `std::pair<MSVehicle *, double> LeaderDist`
pub type LeaderDist = (Ptr<MSVehicle>, f64);

/// C++ class `MSCriticalFollowerDistanceInfo` (microsim/MSLeaderInfo.h)
pub struct MSCriticalFollowerDistanceInfo {
    /// C++ base class `MSLeaderDistanceInfo`
    pub base: MSLeaderDistanceInfo,
    pub myMissingGaps: Vec<f64>,
    pub myHaveOppositeLeaders: bool,
}

impl std::ops::Deref for MSCriticalFollowerDistanceInfo {
    type Target = MSLeaderDistanceInfo;
    fn deref(&self) -> &MSLeaderDistanceInfo {
        &self.base
    }
}
impl std::ops::DerefMut for MSCriticalFollowerDistanceInfo {
    fn deref_mut(&mut self) -> &mut MSLeaderDistanceInfo {
        &mut self.base
    }
}

impl MSCriticalFollowerDistanceInfo {
    /// C++: `void MSCriticalFollowerDistanceInfo::MSCriticalFollowerDistanceInfo(const double, const MSVehicle *, const double, const bool)`
    /// Constructor
    // MSCriticalFollowerDistanceInfo::MSCriticalFollowerDistanceInfo(const double laneWidth, const MSVehicle* ego, const double latOffset, const bool haveOppositeLeaders) :
    //     MSLeaderDistanceInfo(laneWidth, ego, latOffset),
    //     myMissingGaps(myVehicles.size(), -std::numeric_limits<double>::max()),
    //     myHaveOppositeLeaders(haveOppositeLeaders)
    // { }
    #[allow(unused_variables)]
    pub fn new(laneWidth: f64, ego: Ptr<MSVehicle>, latOffset: f64, haveOppositeLeaders: bool) -> Self {
        let base = MSLeaderDistanceInfo::new(laneWidth, ego, latOffset);
        let n = base.myVehicles.len();
        MSCriticalFollowerDistanceInfo { base, myMissingGaps: vec![-f64::MAX; n], myHaveOppositeLeaders: haveOppositeLeaders }
    }

    /// C++: `void MSCriticalFollowerDistanceInfo::~MSCriticalFollowerDistanceInfo() noexcept` [virtual]
    /// Destructor
    // MSCriticalFollowerDistanceInfo::~MSCriticalFollowerDistanceInfo() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `int MSCriticalFollowerDistanceInfo::addFollower(const MSVehicle *, const MSVehicle *, double, double, int)`
    // int
    // MSCriticalFollowerDistanceInfo::addFollower(const MSVehicle* veh, const MSVehicle* ego, double gap, double latOffset, int sublane) {
    //     if (veh == nullptr) {
    //         return myFreeSublanes;
    //     }
    //     const double requiredGap = (myHaveOppositeLeaders ? 0
    //                                 : veh->getCarFollowModel().getSecureGap(veh, ego, veh->getSpeed(), ego->getSpeed(), ego->getCarFollowModel().getMaxDecel()));
    //     const double missingGap = requiredGap - gap;
    //     /*
    //     if (ego->getID() == "disabled" || gDebugFlag1) {
    //         std::cout << "   addFollower veh=" << veh->getID()
    //             << " ego=" << ego->getID()
    //             << " gap=" << gap
    //             << " reqGap=" << requiredGap
    //             << " missingGap=" << missingGap
    //             << " latOffset=" << latOffset
    //             << " sublane=" << sublane
    //             << "\n";
    //         if (sublane > 0) {
    //             std::cout
    //                 << "        dists[s]=" << myDistances[sublane]
    //                 << " gaps[s]=" << myMissingGaps[sublane]
    //                 << "\n";
    //         } else {
    //             std::cout << toString() << "\n";
    //         }
    //     }
    //     */
    //     if (myVehicles.size() == 1) {
    //         // speedup for the simple case
    //         sublane = 0;
    //     }
    //     if (sublane >= 0 && sublane < (int)myVehicles.size()) {
    //         // sublane is already given
    //         // overlapping vehicles are stored preferably
    //         // among those vehicles with missing gap, closer ones are preferred
    //         if ((missingGap > myMissingGaps[sublane]
    //                 || (missingGap > 0 && gap < myDistances[sublane])
    //                 || (gap < 0 && myDistances[sublane] > 0))
    //                 && !(gap > 0 && myDistances[sublane] < 0)
    //                 && !(myMissingGaps[sublane] > 0 && myDistances[sublane] < gap)
    //            ) {
    //             if (myVehicles[sublane] == 0) {
    //                 myFreeSublanes--;
    //             }
    //             myVehicles[sublane] = veh;
    //             myDistances[sublane] = gap;
    //             myMissingGaps[sublane] = missingGap;
    //             myHasVehicles = true;
    //         }
    //         return myFreeSublanes;
    //     }
    //     int rightmost, leftmost;
    //     getSubLanes(veh, latOffset, rightmost, leftmost);
    //     for (int sublaneIdx = rightmost; sublaneIdx <= leftmost; ++sublaneIdx) {
    //         if ((egoRightMost < 0 || (egoRightMost <= sublaneIdx && sublaneIdx <= egoLeftMost))
    //                 // overlapping vehicles are stored preferably
    //                 // among those vehicles with missing gap, closer ones are preferred
    //                 && (missingGap > myMissingGaps[sublaneIdx]
    //                     || (missingGap > 0 && gap < myDistances[sublaneIdx])
    //                     || (gap < 0 && myDistances[sublaneIdx] > 0))
    //                 && !(gap > 0 && myDistances[sublaneIdx] < 0)
    //                 && !(myMissingGaps[sublaneIdx] > 0 && myDistances[sublaneIdx] < gap)
    //            ) {
    //             if (myVehicles[sublaneIdx] == 0) {
    //                 myFreeSublanes--;
    //             }
    //             myVehicles[sublaneIdx] = veh;
    //             myDistances[sublaneIdx] = gap;
    //             myMissingGaps[sublaneIdx] = missingGap;
    //             myHasVehicles = true;
    //         }
    //     }
    //     return myFreeSublanes;
    // }
    #[allow(unused_variables)]
    pub fn add_follower(&mut self, veh: Ptr<MSVehicle>, ego: Ptr<MSVehicle>, gap: f64, latOffset: f64, sublane: i32) -> i32 {
        if veh.is_null() {
            return self.myFreeSublanes;
        }
        let mut sublane = sublane;
        let requiredGap = if self.myHaveOppositeLeaders {
            0.0
        } else {
            veh.get_car_follow_model().get_secure_gap(veh, ego, veh.get_speed(), ego.get_speed(), ego.get_car_follow_model().get_max_decel())
        };
        let missingGap = requiredGap - gap;
        if self.myVehicles.len() == 1 {
            // speedup for the simple case
            sublane = 0;
        }
        if sublane >= 0 && sublane < self.myVehicles.len() as i32 {
            // sublane is already given
            // overlapping vehicles are stored preferably
            // among those vehicles with missing gap, closer ones are preferred
            let s = sublane as usize;
            if (missingGap > self.myMissingGaps[s]
                || (missingGap > 0.0 && gap < self.myDistances[s])
                || (gap < 0.0 && self.myDistances[s] > 0.0))
                && !(gap > 0.0 && self.myDistances[s] < 0.0)
                && !(self.myMissingGaps[s] > 0.0 && self.myDistances[s] < gap)
            {
                if self.myVehicles[s].is_null() {
                    self.myFreeSublanes -= 1;
                }
                self.myVehicles[s] = veh;
                self.myDistances[s] = gap;
                self.myMissingGaps[s] = missingGap;
                self.myHasVehicles = true;
            }
            return self.myFreeSublanes;
        }
        let mut rightmost = 0;
        let mut leftmost = 0;
        self.get_sub_lanes(veh, latOffset, &mut rightmost, &mut leftmost);
        for sublaneIdx in rightmost..=leftmost {
            let s = sublaneIdx as usize;
            if (self.egoRightMost < 0 || (self.egoRightMost <= sublaneIdx && sublaneIdx <= self.egoLeftMost))
                // overlapping vehicles are stored preferably
                // among those vehicles with missing gap, closer ones are preferred
                && (missingGap > self.myMissingGaps[s]
                    || (missingGap > 0.0 && gap < self.myDistances[s])
                    || (gap < 0.0 && self.myDistances[s] > 0.0))
                && !(gap > 0.0 && self.myDistances[s] < 0.0)
                && !(self.myMissingGaps[s] > 0.0 && self.myDistances[s] < gap)
            {
                if self.myVehicles[s].is_null() {
                    self.myFreeSublanes -= 1;
                }
                self.myVehicles[s] = veh;
                self.myDistances[s] = gap;
                self.myMissingGaps[s] = missingGap;
                self.myHasVehicles = true;
            }
        }
        self.myFreeSublanes
    }

    /// C++: `int MSCriticalFollowerDistanceInfo::addLeader(const MSVehicle *, double, double, int)` [virtual]
    // virtual int addLeader(const MSVehicle* veh, double gap, double latOffset = 0, int sublane = -1) {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(gap);
    //         UNUSED_PARAMETER(latOffset);
    //         UNUSED_PARAMETER(sublane);
    //         throw ProcessError(TL("Method not supported"));
    //     }
    #[allow(unused_variables)]
    pub fn add_leader(&mut self, veh: Ptr<MSVehicle>, gap: f64, latOffset: f64, sublane: i32) -> i32 {
        panic!("{}", ProcessError::new("Method not supported"))
    }

    /// C++: `int MSCriticalFollowerDistanceInfo::addLeader(const MSVehicle *, bool, double)` [virtual]
    // virtual int addLeader(const MSVehicle* veh, bool beyond, double latOffset = 0) {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(beyond);
    //         UNUSED_PARAMETER(latOffset);
    //         throw ProcessError(TL("Method not supported"));
    //     }
    #[allow(unused_variables)]
    pub fn add_leader_2(&mut self, veh: Ptr<MSVehicle>, beyond: bool, latOffset: f64) -> i32 {
        panic!("{}", ProcessError::new("Method not supported"))
    }

    /// C++: `void MSCriticalFollowerDistanceInfo::clear()` [virtual]
    /// discard all information
    // void
    // MSCriticalFollowerDistanceInfo::clear() {
    //     MSLeaderDistanceInfo::clear();
    //     myMissingGaps.assign(myVehicles.size(), -std::numeric_limits<double>::max());
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        self.base.clear();
        self.myMissingGaps = vec![-f64::MAX; self.myVehicles.len()];
    }

    /// C++: `std::string MSCriticalFollowerDistanceInfo::toString() const` [const, virtual]
    /// print a debugging representation
    // std::string
    // MSCriticalFollowerDistanceInfo::toString() const {
    //     std::ostringstream oss;
    //     oss.setf(std::ios::fixed, std::ios::floatfield);
    //     oss << std::setprecision(2);
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         oss << Named::getIDSecure(myVehicles[i]) << ":";
    //         if (myVehicles[i] == 0) {
    //             oss << "inf:-inf";
    //         } else {
    //             oss << myDistances[i] << ":" << myMissingGaps[i];
    //         }
    //         if (i < (int)myVehicles.size() - 1) {
    //             oss << ", ";
    //         }
    //     }
    //     oss << " free=" << myFreeSublanes;
    //     return oss.str();
    // }
    #[allow(unused_variables)]
    pub fn to_string(&self) -> String {
        let mut oss = String::new();
        for i in 0..self.myVehicles.len() {
            oss.push_str(&get_id_secure_veh(self.myVehicles[i]));
            oss.push(':');
            if self.myVehicles[i].is_null() {
                oss.push_str("inf:-inf");
            } else {
                oss.push_str(&f64_to_string_prec(self.myDistances[i], 2));
                oss.push(':');
                oss.push_str(&f64_to_string_prec(self.myMissingGaps[i], 2));
            }
            if i + 1 < self.myVehicles.len() {
                oss.push_str(", ");
            }
        }
        oss.push_str(&format!(" free={}", self.myFreeSublanes));
        oss
    }

}

/// saves leader/follower vehicles and their distances relative to an ego vehicle
/// C++ class `MSLeaderDistanceInfo` (microsim/MSLeaderInfo.h)
#[derive(Clone)]
pub struct MSLeaderDistanceInfo {
    /// C++ base class `MSLeaderInfo`
    pub base: MSLeaderInfo,
    pub myDistances: Vec<f64>,
}

impl std::ops::Deref for MSLeaderDistanceInfo {
    type Target = MSLeaderInfo;
    fn deref(&self) -> &MSLeaderInfo {
        &self.base
    }
}
impl std::ops::DerefMut for MSLeaderDistanceInfo {
    fn deref_mut(&mut self) -> &mut MSLeaderInfo {
        &mut self.base
    }
}

impl MSLeaderDistanceInfo {
    /// C++: `void MSLeaderDistanceInfo::MSLeaderDistanceInfo(const double, const MSVehicle *, const double)`
    /// Constructor
    // MSLeaderDistanceInfo::MSLeaderDistanceInfo(const double laneWidth, const MSVehicle* ego, const double latOffset) :
    //     MSLeaderInfo(laneWidth, ego, latOffset),
    //     myDistances(myVehicles.size(), std::numeric_limits<double>::max()) {
    // }
    #[allow(unused_variables)]
    pub fn new(laneWidth: f64, ego: Ptr<MSVehicle>, latOffset: f64) -> Self {
        let base = MSLeaderInfo::new(laneWidth, ego, latOffset);
        let n = base.myVehicles.len();
        MSLeaderDistanceInfo { base, myDistances: vec![f64::MAX; n] }
    }

    /// C++: `void MSLeaderDistanceInfo::MSLeaderDistanceInfo(const CLeaderDist &, const double)`
    /// Construct for the non-sublane-case
    // MSLeaderDistanceInfo::MSLeaderDistanceInfo(const CLeaderDist& cLeaderDist, const double laneWidth) :
    //     MSLeaderInfo(laneWidth, nullptr, 0.),
    //     myDistances(1, cLeaderDist.second) {
    //     assert(myVehicles.size() == 1);
    //     myVehicles[0] = cLeaderDist.first;
    //     myHasVehicles = cLeaderDist.first != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn new_2(cLeaderDist: &CLeaderDist, laneWidth: f64) -> Self {
        let mut base = MSLeaderInfo::new(laneWidth, Ptr::null(), 0.0);
        base.myVehicles[0] = cLeaderDist.0;
        base.myHasVehicles = !cLeaderDist.0.is_null();
        MSLeaderDistanceInfo { base, myDistances: vec![cLeaderDist.1] }
    }

    /// C++: `void MSLeaderDistanceInfo::~MSLeaderDistanceInfo() noexcept` [virtual]
    /// Destructor
    // MSLeaderDistanceInfo::~MSLeaderDistanceInfo() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `int MSLeaderDistanceInfo::addLeader(const MSVehicle *, double, double, int)` [virtual]
    // int
    // MSLeaderDistanceInfo::addLeader(const MSVehicle* veh, double gap, double latOffset, int sublane) {
    //     //if (SIMTIME == 31 && gDebugFlag1 && veh != 0 && veh->getID() == "cars.8") {
    //     //    std::cout << " BREAKPOINT\n";
    //     //}
    //     if (veh == nullptr) {
    //         return myFreeSublanes;
    //     }
    //     if (myVehicles.size() == 1) {
    //         // speedup for the simple case
    //         sublane = 0;
    //     }
    //     if (sublane >= 0 && sublane < (int)myVehicles.size()) {
    //         // sublane is already given
    //         if (gap < myDistances[sublane]) {
    //             if (myVehicles[sublane] == 0) {
    //                 myFreeSublanes--;
    //             }
    //             myVehicles[sublane] = veh;
    //             myDistances[sublane] = gap;
    //             myHasVehicles = true;
    //         }
    //         return myFreeSublanes;
    //     }
    //     int rightmost, leftmost;
    //     getSubLanes(veh, latOffset, rightmost, leftmost);
    //     //if (gDebugFlag1) std::cout << " addLeader veh=" << veh->getID() << " gap=" << gap << " latOffset=" << latOffset << " sublaneOffset=" << myOffset
    //     //    << " rightmost=" << rightmost << " leftmost=" << leftmost
    //     //    << " eRM=" << egoRightMost << " eLM=" << egoLeftMost
    //     //        << " myFreeSublanes=" << myFreeSublanes << "\n";
    //     for (int sublaneIdx = rightmost; sublaneIdx <= leftmost; ++sublaneIdx) {
    //         if ((egoRightMost < 0 || (egoRightMost <= sublaneIdx && sublaneIdx <= egoLeftMost))
    //                 && gap < myDistances[sublaneIdx]) {
    //             if (myVehicles[sublaneIdx] == 0) {
    //                 myFreeSublanes--;
    //             }
    //             myVehicles[sublaneIdx] = veh;
    //             myDistances[sublaneIdx] = gap;
    //             myHasVehicles = true;
    //         }
    //     }
    //     return myFreeSublanes;
    // }
    #[allow(unused_variables)]
    pub fn add_leader(&mut self, veh: Ptr<MSVehicle>, gap: f64, latOffset: f64, sublane: i32) -> i32 {
        let mut sublane = sublane;
        if veh.is_null() {
            return self.myFreeSublanes;
        }
        if self.myVehicles.len() == 1 {
            // speedup for the simple case
            sublane = 0;
        }
        if sublane >= 0 && sublane < self.myVehicles.len() as i32 {
            // sublane is already given
            let s = sublane as usize;
            if gap < self.myDistances[s] {
                if self.myVehicles[s].is_null() {
                    self.myFreeSublanes -= 1;
                }
                self.myVehicles[s] = veh;
                self.myDistances[s] = gap;
                self.myHasVehicles = true;
            }
            return self.myFreeSublanes;
        }
        let mut rightmost = 0;
        let mut leftmost = 0;
        self.get_sub_lanes(veh, latOffset, &mut rightmost, &mut leftmost);
        for sublaneIdx in rightmost..=leftmost {
            let s = sublaneIdx as usize;
            if (self.egoRightMost < 0 || (self.egoRightMost <= sublaneIdx && sublaneIdx <= self.egoLeftMost))
                && gap < self.myDistances[s]
            {
                if self.myVehicles[s].is_null() {
                    self.myFreeSublanes -= 1;
                }
                self.myVehicles[s] = veh;
                self.myDistances[s] = gap;
                self.myHasVehicles = true;
            }
        }
        self.myFreeSublanes
    }

    /// C++: `int MSLeaderDistanceInfo::addLeader(const MSVehicle *, bool, double)` [virtual]
    // virtual int addLeader(const MSVehicle* veh, bool beyond, double latOffset = 0) {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(beyond);
    //         UNUSED_PARAMETER(latOffset);
    //         throw ProcessError(TL("Method not supported"));
    //     }
    #[allow(unused_variables)]
    pub fn add_leader_2(&mut self, veh: Ptr<MSVehicle>, beyond: bool, latOffset: f64) -> i32 {
        panic!("{}", ProcessError::new("Method not supported"))
    }

    /// C++: `void MSLeaderDistanceInfo::addLeaders(MSLeaderDistanceInfo &)` [virtual]
    /// updatd empty sublanes with vehicles and gaps from other
    // void
    // MSLeaderDistanceInfo::addLeaders(MSLeaderDistanceInfo& other) {
    //     const int maxSubLane = MIN2(numSublanes(), other.numSublanes());
    //     for (int i = 0; i < maxSubLane; i++) {
    //         addLeader(other[i].first, other[i].second, 0, i);
    //         //if ((myDistances[i] > 0 && myDistances[i] > other.myDistances[i])
    //         //        || (other.myDistances[i] < 0 && myDistances[i] < other.myDistances[i])) {
    //         //    addLeader(other[i].first, other[i].second, 0, i);
    //         //}
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_leaders(&mut self, other: &mut MSLeaderDistanceInfo) {
        let maxSubLane = self.num_sublanes().min(other.num_sublanes());
        for i in 0..maxSubLane {
            let (veh, dist) = other.index_op(i);
            self.add_leader(veh, dist, 0.0, i);
        }
    }

    /// C++: `void MSLeaderDistanceInfo::clear()` [virtual]
    /// discard all information
    // void
    // MSLeaderDistanceInfo::clear() {
    //     MSLeaderInfo::clear();
    //     myDistances.assign(myVehicles.size(), std::numeric_limits<double>::max());
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        self.base.clear();
        self.myDistances = vec![f64::MAX; self.myVehicles.len()];
    }

    /// C++: `CLeaderDist MSLeaderDistanceInfo::operator[](int) const` [const]
    /// return the vehicle and its distance for the given sublane
    // CLeaderDist
    // MSLeaderDistanceInfo::operator[](int sublane) const {
    //     assert(sublane >= 0);
    //     assert(sublane < (int)myVehicles.size());
    //     return std::make_pair(myVehicles[sublane], myDistances[sublane]);
    // }
    #[allow(unused_variables)]
    pub fn index_op(&self, sublane: i32) -> CLeaderDist {
        let s = sublane as usize;
        (self.myVehicles[s], self.myDistances[s])
    }

    /// C++: `std::string MSLeaderDistanceInfo::toString() const` [const, virtual]
    /// print a debugging representation
    // std::string
    // MSLeaderDistanceInfo::toString() const {
    //     std::ostringstream oss;
    //     oss.setf(std::ios::fixed, std::ios::floatfield);
    //     oss << std::setprecision(2);
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         oss << Named::getIDSecure(myVehicles[i]) << ":";
    //         if (myVehicles[i] == 0) {
    //             oss << "inf";
    //         } else {
    //             oss << myDistances[i];
    //         }
    //         if (i < (int)myVehicles.size() - 1) {
    //             oss << ", ";
    //         }
    //     }
    //     oss << " free=" << myFreeSublanes;
    //     return oss.str();
    // }
    #[allow(unused_variables)]
    pub fn to_string(&self) -> String {
        let mut oss = String::new();
        for i in 0..self.myVehicles.len() {
            oss.push_str(&get_id_secure_veh(self.myVehicles[i]));
            oss.push(':');
            if self.myVehicles[i].is_null() {
                oss.push_str("inf");
            } else {
                oss.push_str(&f64_to_string_prec(self.myDistances[i], 2));
            }
            if i + 1 < self.myVehicles.len() {
                oss.push_str(", ");
            }
        }
        oss.push_str(&format!(" free={}", self.myFreeSublanes));
        oss
    }

    /// C++: `const std::vector<double> & MSLeaderDistanceInfo::getDistances() const` [const]
    // const std::vector<double>& getDistances() const {
    //         return myDistances;
    //     }
    #[allow(unused_variables)]
    pub fn get_distances(&self) -> &Vec<f64> {
        &self.myDistances
    }

    /// C++: `void MSLeaderDistanceInfo::fixOppositeGaps(bool)`
    /// subtract vehicle length from all gaps if the leader vehicle is driving in the opposite direction
    // void
    // MSLeaderDistanceInfo::fixOppositeGaps(bool isFollower) {
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] != nullptr) {
    //             if (myVehicles[i]->getLaneChangeModel().isOpposite()) {
    //                 myDistances[i] -= myVehicles[i]->getVehicleType().getLength();
    //             } else if (isFollower && myDistances[i] > POSITION_EPS) {
    //                 // can ignore oncoming followers once they are past
    //                 myVehicles[i] = nullptr;
    //                 myDistances[i] = -1;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn fix_opposite_gaps(&mut self, isFollower: bool) {
        for i in 0..self.myVehicles.len() {
            if !self.myVehicles[i].is_null() {
                if self.myVehicles[i].get_lane_change_model_2().is_opposite() {
                    let length = self.myVehicles[i].get_vehicle_type().get_length();
                    self.myDistances[i] -= length;
                } else if isFollower && self.myDistances[i] > POSITION_EPS {
                    // can ignore oncoming followers once they are past
                    self.myVehicles[i] = Ptr::null();
                    self.myDistances[i] = -1.0;
                }
            }
        }
    }

    /// C++: `void MSLeaderDistanceInfo::patchGaps(double)`
    /// add given value to all gaps
    // void
    // MSLeaderDistanceInfo::patchGaps(double amount) {
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] != nullptr) {
    //             myDistances[i] += amount;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn patch_gaps(&mut self, amount: f64) {
        for i in 0..self.myVehicles.len() {
            if !self.myVehicles[i].is_null() {
                self.myDistances[i] += amount;
            }
        }
    }

    /// C++: `CLeaderDist MSLeaderDistanceInfo::getClosest() const` [const]
    /// return vehicle with the smalles gap
    // CLeaderDist
    // MSLeaderDistanceInfo::getClosest() const {
    //     double minGap = -1;
    //     const MSVehicle* veh = nullptr;
    //     if (hasVehicles()) {
    //         minGap = std::numeric_limits<double>::max();
    //         for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //             if (myVehicles[i] != nullptr && myDistances[i] < minGap) {
    //                 minGap = myDistances[i];
    //                 veh = myVehicles[i];
    //             }
    //         }
    //     }
    //     return std::make_pair(veh, minGap);
    // }
    #[allow(unused_variables)]
    pub fn get_closest(&self) -> CLeaderDist {
        let mut minGap = -1.0;
        let mut veh: Ptr<MSVehicle> = Ptr::null();
        if self.has_vehicles() {
            minGap = f64::MAX;
            for i in 0..self.myVehicles.len() {
                if !self.myVehicles[i].is_null() && self.myDistances[i] < minGap {
                    minGap = self.myDistances[i];
                    veh = self.myVehicles[i];
                }
            }
        }
        (veh, minGap)
    }

    /// C++: `void MSLeaderDistanceInfo::moveSamePosTo(const MSVehicle *, MSLeaderDistanceInfo &)`
    // void
    // MSLeaderDistanceInfo::moveSamePosTo(const MSVehicle* ego, MSLeaderDistanceInfo& other) {
    //     const double pos = ego->getPositionOnLane();
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] != nullptr && myDistances[i] < 0 && myVehicles[i]->getPositionOnLane() == pos
    //                 && &myVehicles[i]->getLane()->getEdge() == &ego->getLane()->getEdge()) {
    //             other.myVehicles[i] = myVehicles[i];
    //             other.myDistances[i] = myDistances[i];
    //             myVehicles[i] = nullptr;
    //             myDistances[i] = -1;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn move_same_pos_to(&mut self, ego: Ptr<MSVehicle>, other: &mut MSLeaderDistanceInfo) {
        let pos = ego.get_position_on_lane();
        for i in 0..self.myVehicles.len() {
            if !self.myVehicles[i].is_null()
                && self.myDistances[i] < 0.0
                && self.myVehicles[i].get_position_on_lane() == pos
                && std::ptr::eq(self.myVehicles[i].get_lane().get_edge(), ego.get_lane().get_edge())
            {
                other.myVehicles[i] = self.myVehicles[i];
                other.myDistances[i] = self.myDistances[i];
                self.myVehicles[i] = Ptr::null();
                self.myDistances[i] = -1.0;
            }
        }
    }

    /// C++: `double MSLeaderDistanceInfo::getMinDistToStopped() const` [const]
    /// return minimum distance to a stopped vehicle or max double
    // double
    // MSLeaderDistanceInfo::getMinDistToStopped() const {
    //     double result = std::numeric_limits<double>::max();
    //     if (!myHasVehicles) {
    //         return result;
    //     }
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] != 0 && myVehicles[i]->isStopped()) {
    //             result = MIN2(result, myDistances[i]);
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_min_dist_to_stopped(&self) -> f64 {
        let mut result = f64::MAX;
        if !self.myHasVehicles {
            return result;
        }
        for i in 0..self.myVehicles.len() {
            if !self.myVehicles[i].is_null() && self.myVehicles[i].is_stopped() {
                result = min2(result, self.myDistances[i]);
            }
        }
        result
    }

}

/// C++ class `MSLeaderInfo` (microsim/MSLeaderInfo.h)
#[derive(Clone)]
pub struct MSLeaderInfo {
    pub myWidth: f64,
    /// an extra offset for shifting the interpretation of sublane borders (default [0,myWidth])
    pub myOffset: i32,
    pub myVehicles: Vec<Ptr<MSVehicle>>,
    /// the number of free sublanes
    pub myFreeSublanes: i32,
    /// borders of the ego vehicle for filtering of free sublanes
    pub egoRightMost: i32,
    pub egoLeftMost: i32,
    pub myHasVehicles: bool,
}

impl MSLeaderInfo {
    /// C++: `void MSLeaderInfo::MSLeaderInfo(const double, const MSVehicle *, const double)`
    /// Constructor
    // MSLeaderInfo::MSLeaderInfo(const double laneWidth, const MSVehicle* ego, const double latOffset) :
    //     myWidth(laneWidth),
    //     myOffset(0),
    //     myVehicles(MAX2(1, int(ceil(laneWidth / MSGlobals::gLateralResolution))), (MSVehicle*)nullptr),
    //     myFreeSublanes((int)myVehicles.size()),
    //     egoRightMost(-1),
    //     egoLeftMost(-1),
    //     myHasVehicles(false) {
    //     if (ego != nullptr) {
    //         getSubLanes(ego, latOffset, egoRightMost, egoLeftMost);
    //         // filter out sublanes not of interest to ego
    //         myFreeSublanes -= egoRightMost;
    //         myFreeSublanes -= (int)myVehicles.size() - 1 - egoLeftMost;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(laneWidth: f64, ego: Ptr<MSVehicle>, latOffset: f64) -> Self {
        let n = std::cmp::max(1, (laneWidth / g_lateral_resolution()).ceil() as i32);
        let mut s = MSLeaderInfo {
            myWidth: laneWidth,
            myOffset: 0,
            myVehicles: vec![Ptr::null(); n as usize],
            myFreeSublanes: n,
            egoRightMost: -1,
            egoLeftMost: -1,
            myHasVehicles: false,
        };
        if !ego.is_null() {
            let mut egoRightMost = -1;
            let mut egoLeftMost = -1;
            s.get_sub_lanes(ego, latOffset, &mut egoRightMost, &mut egoLeftMost);
            s.egoRightMost = egoRightMost;
            s.egoLeftMost = egoLeftMost;
            // filter out sublanes not of interest to ego
            s.myFreeSublanes -= s.egoRightMost;
            s.myFreeSublanes -= s.myVehicles.len() as i32 - 1 - s.egoLeftMost;
        }
        s
    }

    /// C++: `void MSLeaderInfo::~MSLeaderInfo() noexcept` [virtual]
    /// Destructor
    // MSLeaderInfo::~MSLeaderInfo() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `int MSLeaderInfo::addLeader(const MSVehicle *, bool, double)` [virtual]
    // int
    // MSLeaderInfo::addLeader(const MSVehicle* veh, bool beyond, double latOffset) {
    //     if (veh == nullptr) {
    //         return myFreeSublanes;
    //     }
    //     if (myVehicles.size() == 1) {
    //         // speedup for the simple case
    //         if (!beyond || myVehicles[0] == 0) {
    //             myVehicles[0] = veh;
    //             myFreeSublanes = 0;
    //             myHasVehicles = true;
    //         }
    //         return myFreeSublanes;
    //     }
    //     // map center-line based coordinates into [0, myWidth] coordinates
    //     int rightmost, leftmost;
    //     getSubLanes(veh, latOffset, rightmost, leftmost);
    //     //if (gDebugFlag1) std::cout << " addLeader veh=" << veh->getID() << " beyond=" << beyond << " latOffset=" << latOffset << " sublaneOffset=" << myOffset
    //     //    << " rightmost=" << rightmost << " leftmost=" << leftmost
    //     //    << " eRM=" << egoRightMost << " eLM=" << egoLeftMost
    //     //        << " myFreeSublanes=" << myFreeSublanes << "\n";
    //     for (int sublane = rightmost; sublane <= leftmost; ++sublane) {
    //         if ((egoRightMost < 0 || (egoRightMost <= sublane && sublane <= egoLeftMost))
    //                 && (!beyond || myVehicles[sublane] == 0)) {
    //             if (myVehicles[sublane] == 0) {
    //                 myFreeSublanes--;
    //             }
    //             myVehicles[sublane] = veh;
    //             myHasVehicles = true;
    //         }
    //     }
    //     return myFreeSublanes;
    // }
    #[allow(unused_variables)]
    pub fn add_leader(&mut self, veh: Ptr<MSVehicle>, beyond: bool, latOffset: f64) -> i32 {
        if veh.is_null() {
            return self.myFreeSublanes;
        }
        if self.myVehicles.len() == 1 {
            // speedup for the simple case
            if !beyond || self.myVehicles[0].is_null() {
                self.myVehicles[0] = veh;
                self.myFreeSublanes = 0;
                self.myHasVehicles = true;
            }
            return self.myFreeSublanes;
        }
        // map center-line based coordinates into [0, myWidth] coordinates
        let mut rightmost = 0;
        let mut leftmost = 0;
        self.get_sub_lanes(veh, latOffset, &mut rightmost, &mut leftmost);
        for sublane in rightmost..=leftmost {
            let s = sublane as usize;
            if (self.egoRightMost < 0 || (self.egoRightMost <= sublane && sublane <= self.egoLeftMost))
                && (!beyond || self.myVehicles[s].is_null())
            {
                if self.myVehicles[s].is_null() {
                    self.myFreeSublanes -= 1;
                }
                self.myVehicles[s] = veh;
                self.myHasVehicles = true;
            }
        }
        self.myFreeSublanes
    }

    /// C++: `void MSLeaderInfo::clear()` [virtual]
    /// discard all information
    // void
    // MSLeaderInfo::clear() {
    //     myVehicles.assign(myVehicles.size(), (MSVehicle*)nullptr);
    //     myFreeSublanes = (int)myVehicles.size();
    //     if (egoRightMost >= 0) {
    //         myFreeSublanes -= egoRightMost;
    //         myFreeSublanes -= (int)myVehicles.size() - 1 - egoLeftMost;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear(&mut self) {
        for v in self.myVehicles.iter_mut() {
            *v = Ptr::null();
        }
        self.myFreeSublanes = self.myVehicles.len() as i32;
        if self.egoRightMost >= 0 {
            self.myFreeSublanes -= self.egoRightMost;
            self.myFreeSublanes -= self.myVehicles.len() as i32 - 1 - self.egoLeftMost;
        }
    }

    /// C++: `void MSLeaderInfo::getSubLanes(const MSVehicle *, double, int &, int &) const` [const]
    // void
    // MSLeaderInfo::getSubLanes(const MSVehicle* veh, double latOffset, int& rightmost, int& leftmost) const {
    //     if (myVehicles.size() == 1) {
    //         // speedup for the simple case
    //         rightmost = 0;
    //         leftmost = 0;
    //         return;
    //     }
    //     // map center-line based coordinates into [0, myWidth] coordinates
    //     const double vehCenter = veh->getLateralPositionOnLane() + 0.5 * myWidth + latOffset + myOffset * MSGlobals::gLateralResolution;
    //     const double vehHalfWidth = 0.5 * veh->getVehicleType().getWidth();
    //     double rightVehSide = vehCenter - vehHalfWidth;
    //     double leftVehSide = vehCenter + vehHalfWidth;
    //     // Reserve some additional space if the vehicle is performing a maneuver continuation.
    //     if (veh->getActionStepLength() != DELTA_T) {
    //         if (veh->getLaneChangeModel().getManeuverDist() < 0. || veh->getLaneChangeModel().getSpeedLat() < 0.) {
    //             const double maneuverDist = MIN2(veh->getVehicleType().getMaxSpeedLat() * veh->getActionStepLengthSecs(), -MIN2(0., veh->getLaneChangeModel().getManeuverDist()));
    //             rightVehSide -= maneuverDist;
    //         }
    //         if (veh->getLaneChangeModel().getManeuverDist() > 0. || veh->getLaneChangeModel().getSpeedLat() > 0.) {
    //             const double maneuverDist = MIN2(veh->getVehicleType().getMaxSpeedLat() * veh->getActionStepLengthSecs(), MAX2(0., veh->getLaneChangeModel().getManeuverDist()));
    //             leftVehSide += maneuverDist;
    //         }
    //     }
    //     if (rightVehSide > myWidth || leftVehSide < 0) {
    //         // vehicle does not touch this lane
    //         // set the values so that an iteration
    //         // for (i = rightmost; i  <= leftmost; i++) stops immediately
    //         rightmost = -1000;
    //         leftmost = -2000;
    //     } else {
    //         rightmost = MAX2(0, (int)floor((rightVehSide + NUMERICAL_EPS) / MSGlobals::gLateralResolution));
    //         leftmost = MIN2((int)myVehicles.size() - 1, (int)floor(MAX2(0.0, leftVehSide - NUMERICAL_EPS) / MSGlobals::gLateralResolution));
    //     }
    //     //if (veh->isSelected()) std::cout << SIMTIME << " veh=" << veh->getID()
    //     //    << std::setprecision(2)
    //     //    << " posLat=" << veh->getLateralPositionOnLane()
    //     //    << " latOffset=" << latOffset
    //     //    << " vehCenter=" << vehCenter
    //     //    << " rightVehSide=" << rightVehSide
    //     //    << " leftVehSide=" << leftVehSide
    //     //    << " rightmost=" << rightmost
    //     //    << " leftmost=" << leftmost
    //     //    << " myOffset=" << myOffset
    //     //    << std::setprecision(2)
    //     //    << "\n";
    // }
    #[allow(unused_variables)]
    pub fn get_sub_lanes(&self, veh: Ptr<MSVehicle>, latOffset: f64, rightmost: &mut i32, leftmost: &mut i32) {
        if self.myVehicles.len() == 1 {
            // speedup for the simple case
            *rightmost = 0;
            *leftmost = 0;
            return;
        }
        let res = g_lateral_resolution();
        // map center-line based coordinates into [0, myWidth] coordinates
        let vehCenter = veh.get_lateral_position_on_lane() + 0.5 * self.myWidth + latOffset + self.myOffset as f64 * res;
        let vehHalfWidth = 0.5 * veh.get_vehicle_type().get_width();
        let mut rightVehSide = vehCenter - vehHalfWidth;
        let mut leftVehSide = vehCenter + vehHalfWidth;
        // Reserve some additional space if the vehicle is performing a maneuver continuation.
        if veh.get_action_step_length() != delta_t() {
            if veh.get_lane_change_model_2().get_maneuver_dist() < 0. || veh.get_lane_change_model_2().get_speed_lat() < 0. {
                let maneuverDist = min2(
                    veh.get_vehicle_type().get_max_speed_lat() * veh.get_action_step_length_secs(),
                    -min2(0., veh.get_lane_change_model_2().get_maneuver_dist()),
                );
                rightVehSide -= maneuverDist;
            }
            if veh.get_lane_change_model_2().get_maneuver_dist() > 0. || veh.get_lane_change_model_2().get_speed_lat() > 0. {
                let maneuverDist = min2(
                    veh.get_vehicle_type().get_max_speed_lat() * veh.get_action_step_length_secs(),
                    max2(0., veh.get_lane_change_model_2().get_maneuver_dist()),
                );
                leftVehSide += maneuverDist;
            }
        }
        if rightVehSide > self.myWidth || leftVehSide < 0. {
            // vehicle does not touch this lane
            // set the values so that an iteration
            // for (i = rightmost; i  <= leftmost; i++) stops immediately
            *rightmost = -1000;
            *leftmost = -2000;
        } else {
            *rightmost = std::cmp::max(0, ((rightVehSide + NUMERICAL_EPS) / res).floor() as i32);
            *leftmost = std::cmp::min(
                self.myVehicles.len() as i32 - 1,
                (max2(0.0, leftVehSide - NUMERICAL_EPS) / res).floor() as i32,
            );
        }
    }

    /// C++: `void MSLeaderInfo::getSublaneBorders(int, double, double &, double &) const` [const]
    // void
    // MSLeaderInfo::getSublaneBorders(int sublane, double latOffset, double& rightSide, double& leftSide) const {
    //     assert(sublane >= 0);
    //     assert(sublane < (int)myVehicles.size());
    //     const double res = MSGlobals::gLateralResolution > 0 ? MSGlobals::gLateralResolution : myWidth;
    //     rightSide = sublane * res + latOffset - myOffset * MSGlobals::gLateralResolution;
    //     leftSide = MIN2((sublane + 1) * res, myWidth) + latOffset - myOffset * MSGlobals::gLateralResolution;
    // }
    #[allow(unused_variables)]
    pub fn get_sublane_borders(&self, sublane: i32, latOffset: f64, rightSide: &mut f64, leftSide: &mut f64) {
        let res = if g_lateral_resolution() > 0. { g_lateral_resolution() } else { self.myWidth };
        *rightSide = sublane as f64 * res + latOffset - self.myOffset as f64 * g_lateral_resolution();
        *leftSide = min2((sublane + 1) as f64 * res, self.myWidth) + latOffset - self.myOffset as f64 * g_lateral_resolution();
    }

    /// C++: `const MSVehicle * MSLeaderInfo::operator[](int) const` [const]
    /// return the vehicle for the given sublane
    // const MSVehicle*
    // MSLeaderInfo::operator[](int sublane) const {
    //     assert(sublane >= 0);
    //     assert(sublane < (int)myVehicles.size());
    //     return myVehicles[sublane];
    // }
    #[allow(unused_variables)]
    pub fn index_op(&self, sublane: i32) -> Ptr<MSVehicle> {
        self.myVehicles[sublane as usize]
    }

    /// C++: `int MSLeaderInfo::numSublanes() const` [const]
    // int numSublanes() const {
    //         return (int)myVehicles.size();
    //     }
    #[allow(unused_variables)]
    pub fn num_sublanes(&self) -> i32 {
        self.myVehicles.len() as i32
    }

    /// C++: `int MSLeaderInfo::numFreeSublanes() const` [const]
    // int numFreeSublanes() const {
    //         return myFreeSublanes;
    //     }
    #[allow(unused_variables)]
    pub fn num_free_sublanes(&self) -> i32 {
        self.myFreeSublanes
    }

    /// C++: `bool MSLeaderInfo::hasVehicles() const` [const]
    // bool hasVehicles() const {
    //         return myHasVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn has_vehicles(&self) -> bool {
        self.myHasVehicles
    }

    /// C++: `const std::vector<const MSVehicle *> & MSLeaderInfo::getVehicles() const` [const]
    // const std::vector<const MSVehicle*>& getVehicles() const {
    //         return myVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicles(&self) -> &Vec<Ptr<MSVehicle>> {
        &self.myVehicles
    }

    /// C++: `int MSLeaderInfo::getSublaneOffset() const` [const]
    // int getSublaneOffset() const {
    //         return myOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_sublane_offset(&self) -> i32 {
        self.myOffset
    }

    /// C++: `void MSLeaderInfo::setSublaneOffset(int)`
    /// set number of sublanes by which to shift positions
    // void
    // MSLeaderInfo::setSublaneOffset(int offset) {
    //     assert(MSGlobals::gLateralResolution > 0);
    //     myOffset = offset;
    // }
    #[allow(unused_variables)]
    pub fn set_sublane_offset(&mut self, offset: i32) {
        // C++ asserts gLateralResolution > 0; not checked while gLateralResolution is a stub
        self.myOffset = offset;
    }

    /// C++: `bool MSLeaderInfo::hasStoppedVehicle() const` [const]
    /// whether a stopped vehicle is leader
    // bool
    // MSLeaderInfo::hasStoppedVehicle() const {
    //     if (!myHasVehicles) {
    //         return false;
    //     }
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] != 0 && myVehicles[i]->isStopped()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_stopped_vehicle(&self) -> bool {
        if !self.myHasVehicles {
            return false;
        }
        for &v in self.myVehicles.iter() {
            if !v.is_null() && v.is_stopped() {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSLeaderInfo::hasVehicle(const MSVehicle *) const` [const]
    /// whether the given vehicle is part of this leaderInfo
    // bool
    // MSLeaderInfo::hasVehicle(const MSVehicle* veh) const {
    //     if (!myHasVehicles) {
    //         return false;
    //     }
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         if (myVehicles[i] == veh) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_vehicle(&self, veh: Ptr<MSVehicle>) -> bool {
        if !self.myHasVehicles {
            return false;
        }
        for &v in self.myVehicles.iter() {
            if v == veh {
                return true;
            }
        }
        false
    }

    /// C++: `void MSLeaderInfo::removeOpposite(const MSLane *)`
    /// remove vehicles that are driving in the opposite direction (fully or partially) on the given lane
    // void
    // MSLeaderInfo::removeOpposite(const MSLane* lane) {
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         const MSVehicle* veh = myVehicles[i];
    //         if (veh != 0 &&
    //                 (veh->getLaneChangeModel().isOpposite()
    //                  || &lane->getEdge() != &veh->getLane()->getEdge())) {
    //             myVehicles[i] = nullptr;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_opposite(&mut self, lane: Ptr<MSLane>) {
        for i in 0..self.myVehicles.len() {
            let veh = self.myVehicles[i];
            if !veh.is_null()
                && (veh.get_lane_change_model_2().is_opposite() || !std::ptr::eq(lane.get_edge(), veh.get_lane().get_edge()))
            {
                self.myVehicles[i] = Ptr::null();
            }
        }
    }

    /// C++: `std::string MSLeaderInfo::toString() const` [const, virtual]
    /// print a debugging representation
    // std::string
    // MSLeaderInfo::toString() const {
    //     std::ostringstream oss;
    //     oss.setf(std::ios::fixed, std::ios::floatfield);
    //     oss << std::setprecision(2);
    //     for (int i = 0; i < (int)myVehicles.size(); ++i) {
    //         oss << Named::getIDSecure(myVehicles[i]);
    //         if (i < (int)myVehicles.size() - 1) {
    //             oss << ", ";
    //         }
    //     }
    //     oss << " free=" << myFreeSublanes;
    //     return oss.str();
    // }
    #[allow(unused_variables)]
    pub fn to_string(&self) -> String {
        let mut oss = String::new();
        for i in 0..self.myVehicles.len() {
            oss.push_str(&get_id_secure_veh(self.myVehicles[i]));
            if i + 1 < self.myVehicles.len() {
                oss.push_str(", ");
            }
        }
        oss.push_str(&format!(" free={}", self.myFreeSublanes));
        oss
    }

}

