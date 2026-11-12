//! Port of mesosim/MELSegment.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<SUMOTime> MELSegment::GapTimes`
pub type MELSegment_GapTimes = Vec<SUMOTime>;

/// A single mesoscopic segment (cell)
/// C++ class `MELSegment` (mesosim/MELSegment.h)
pub struct MELSegment {
    /// C++ base class `MESegment`
    pub base: MESegment,
    pub myGapTimes: Vec<MELSegment_GapTimes>,
}

impl std::ops::Deref for MELSegment {
    type Target = MESegment;
    fn deref(&self) -> &MESegment {
        &self.base
    }
}
impl std::ops::DerefMut for MELSegment {
    fn deref_mut(&mut self) -> &mut MESegment {
        &mut self.base
    }
}

/// C++ macro `DEFAULT_VEH_LENGTH_WITH_GAP` (mesosim/MELSegment.cpp)
fn default_veh_length_with_gap() -> f64 {
    let d = SUMOVTypeParameter::get_default();
    d.length + d.minGap
}

impl MELSegment {
    /// C++: `void MELSegment::MELSegment(const std::string &, const MSEdge &, MESegment *, const double, const double, const int, const bool, const MesoEdgeType &)`
    /// constructor
    // MELSegment::MELSegment(const std::string& id,
    //                        const MSEdge& parent, MESegment* next,
    //                        const double length, const double speed,
    //                        const int idx,
    //                        const bool multiQueue,
    //                        const MesoEdgeType& edgeType):
    //     MESegment(id, parent, next, length, speed, idx, multiQueue, edgeType) {
    //     myGapTimes.resize(myQueues.size());
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, parent: &MSEdge, next: Ptr<MESegment>, length: f64, speed: f64, idx: i32, multiQueue: bool, edgeType: &MESegment_MesoEdgeType) -> Self {
        let base = MESegment::new(id, parent, next, length, speed, idx, multiQueue, edgeType);
        let myGapTimes = vec![Vec::new(); base.myQueues.len()];
        MELSegment { base, myGapTimes }
    }

    /// C++: `void MELSegment::send(MEVehicle *, MESegment *const, const int, SUMOTime, const MSMoveReminder::Notification)` [virtual]
    /// Removes the vehicle from the segment, adapting its parameters
    // void
    // MELSegment::send(MEVehicle* veh, MESegment* const next, const int nextQIdx, SUMOTime time, const MSMoveReminder::Notification reason) {
    //     GapTimes& gapTimes = myGapTimes[veh->getQueIndex()];
    //     // record time when the gap vacated by ego will reach the upstream end of the segment
    //     // gaps travel quickly in free flow (the number of gaps and vehicles stays below the segment capacity)
    //     // but they travel more slowly when queued (startupDelay)
    //     gapTimes.insert(gapTimes.begin(), time + (SUMOTime)(myLength * (double)(veh->getQueuingTimeLoss() > 0 ? myTau_jj : myTau_ff) / DEFAULT_VEH_LENGTH_WITH_GAP));
    //     MESegment::send(veh, next, nextQIdx, time, reason);
    // }
    #[allow(unused_variables)]
    pub fn send(&mut self, veh: Ptr<MEVehicle>, next: Ptr<MESegment>, nextQIdx: i32, time: SUMOTime, reason: MSMoveReminder_Notification) {
        let qIdx = veh.get_que_index() as usize;
        let tau = if veh.get_queuing_time_loss() > 0 { self.myTau_jj } else { self.myTau_ff };
        let gapTime = time + (self.myLength * (tau as f64) / default_veh_length_with_gap()) as SUMOTime;
        self.myGapTimes[qIdx].insert(0, gapTime);
        self.base.send(veh, next, nextQIdx, time, reason);
    }

    /// C++: `void MELSegment::updateEntryBlockTime(SUMOTime)` [virtual]
    /// update entry blockTime for all queues
    // void
    // MELSegment::updateEntryBlockTime(SUMOTime time) {
    //     for (int qIdx = 0; qIdx < (int)myQueues.size(); qIdx++) {
    //         Queue& q = myQueues[qIdx];
    //         GapTimes& gapTimes = myGapTimes[qIdx];
    //         while (!gapTimes.empty() && gapTimes.back() <= time) {
    //             gapTimes.pop_back();
    //         }
    //         if (!gapTimes.empty() && q.getOccupancy() + (double)gapTimes.size() * DEFAULT_VEH_LENGTH_WITH_GAP > myQueueCapacity) {
    //             // segment is jammed because the empty spaces have not yet reached the upstream end.
    //             q.setEntryBlockTime(MAX2(q.getEntryBlockTime(), gapTimes.back()));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_entry_block_time(&mut self, time: SUMOTime) {
        for qIdx in 0..self.base.myQueues.len() {
            let q = &mut self.base.myQueues[qIdx];
            let gapTimes = &mut self.myGapTimes[qIdx];
            while !gapTimes.is_empty() && *gapTimes.last().unwrap() <= time {
                gapTimes.pop();
            }
            if !gapTimes.is_empty() && q.get_occupancy() + (gapTimes.len() as f64) * default_veh_length_with_gap() > self.base.myQueueCapacity {
                // segment is jammed because the empty spaces have not yet reached the upstream end.
                q.set_entry_block_time(std::cmp::max(q.get_entry_block_time(), *gapTimes.last().unwrap()));
            }
        }
    }

    /// C++: `std::string MELSegment::getQueueExtraState(int) const` [const, virtual]
    /// the gap times of the given queue for saving / loading
    // std::string
    // MELSegment::getQueueExtraState(int qIdx) const {
    //     return joinToString(myGapTimes[qIdx], " ");
    // }
    #[allow(unused_variables)]
    pub fn get_queue_extra_state(&self, qIdx: i32) -> String {
        utils::common::to_string::join_to_string(&self.myGapTimes[qIdx as usize], " ")
    }

    /// C++: `void MELSegment::loadQueueExtraState(int, const std::string &, SUMOTime)` [virtual]
    // void
    // MELSegment::loadQueueExtraState(int qIdx, const std::string& state, SUMOTime offset) {
    //     GapTimes& gapTimes = myGapTimes[qIdx];
    //     gapTimes.clear();
    //     for (const std::string& t : StringTokenizer(state).getVector()) {
    //         gapTimes.push_back(StringUtils::toLong(t) - offset);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_queue_extra_state(&mut self, qIdx: i32, state: &str, offset: SUMOTime) {
        let gapTimes = &mut self.myGapTimes[qIdx as usize];
        gapTimes.clear();
        for t in StringTokenizer::new(state).get_vector() {
            gapTimes.push(utils::common::string_utils::to_long(&t).unwrap() - offset);
        }
    }

    /// C++: `SUMOTime MELSegment::computeHeadway(Queue &, const Queue &, const MESegment *const, const MEVehicle *) const` [const, virtual, protected]
    /// compute current headway depending on traffic state
    // SUMOTime
    // MELSegment::computeHeadway(Queue& /*q*/, const Queue& /*qNext*/, const MESegment* const /*next*/, const MEVehicle* veh) const {
    //     // @note: it might seem sensible to check veh->getQueuingTimeLoss() here but
    //     // then the queing state would be "sticky" because queuingTimeLoss is computed with tau_ff
    //     // and every leader car that uses tau_jf applies queing timeLoss to it's followers
    //     SUMOTime headway = tauWithVehLength(veh->getWaitingTime() > 0 ? myTau_jf : myTau_ff,
    //                                         veh->getVehicleType().getLengthWithGap(),
    //                                         veh->getVehicleType().getCarFollowModel().getHeadwayTime());
    //     if (myTLSPenalty) {
    //         const MSLink* const tllink = getLink(veh, true);
    //         if (tllink != nullptr && tllink->isTLSControlled()) {
    //             assert(tllink->getGreenFraction() > 0);
    //             headway = (SUMOTime)((double)headway / tllink->getGreenFraction());
    //         }
    //     }
    //     return headway;
    // }
    #[allow(unused_variables)]
    pub fn compute_headway(&self, q: &mut MESegment_Queue, qNext: &MESegment_Queue, next: Ptr<MESegment>, veh: Ptr<MEVehicle>) -> SUMOTime {
        let mut headway = self.tau_with_veh_length(
            if veh.get_waiting_time(false) > 0 { self.myTau_jf } else { self.myTau_ff },
            veh.get_vehicle_type().get_length_with_gap(),
            veh.get_vehicle_type().get_car_follow_model().get_headway_time(),
        );
        if self.myTLSPenalty {
            let tllink = self.get_link(veh, true);
            if !tllink.is_null() && tllink.is_tls_controlled() {
                debug_assert!(tllink.get_green_fraction() > 0.);
                headway = ((headway as f64) / tllink.get_green_fraction()) as SUMOTime;
            }
        }
        headway
    }

    /// C++: `bool MELSegment::hasSpaceForInsertion(const Queue &, int, double, SUMOTime) const` [const, virtual, protected]
    /// check jam-avoidance constraints during insertion
    // bool
    // MELSegment::hasSpaceForInsertion(const Queue& q, int qIdx, double /*newOccupancy*/, SUMOTime entryTime) const {
    //     GapTimes& gapTimes = const_cast<GapTimes&>(myGapTimes[qIdx]);
    //     while (!gapTimes.empty() && gapTimes.back() <= entryTime) {
    //         gapTimes.pop_back();
    //     }
    //     return q.getOccupancy() + (double)gapTimes.size() * DEFAULT_VEH_LENGTH_WITH_GAP <= myQueueCapacity;
    // }
    #[allow(unused_variables)]
    pub fn has_space_for_insertion(&self, q: &MESegment_Queue, qIdx: i32, newOccupancy: f64, entryTime: SUMOTime) -> bool {
        // C++ const_cast: the gap-time list is pruned even though the method is const
        let gapTimesPtr = Ptr::new(&self.myGapTimes[qIdx as usize]);
        let gapTimes = gapTimesPtr.m();
        while !gapTimes.is_empty() && *gapTimes.last().unwrap() <= entryTime {
            gapTimes.pop();
        }
        q.get_occupancy() + (gapTimes.len() as f64) * default_veh_length_with_gap() <= self.myQueueCapacity
    }

    /// C++: `void MELSegment::MELSegment(const MELSegment &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MELSegment) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("MELSegment is not copyable (private copy constructor in C++)")
    }

    /// C++: `MELSegment & MELSegment::operator=(const MELSegment &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MELSegment) -> &mut MELSegment {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("MELSegment is not assignable (private operator= in C++)")
    }

    /// C++: `void MELSegment::MELSegment(const std::string &)` [private]
    /// constructor for dummy segment
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(id: &str) -> Self {
        // C++ constructor for a dummy segment is declared private and never defined
        panic!("MELSegment(id) dummy constructor has no C++ definition")
    }

}

