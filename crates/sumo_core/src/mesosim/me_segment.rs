//! Port of mesosim/MESegment.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::Cell;


// C++ `#define MESO_MIN_SPEED (0.05)` (MESegment.cpp)
const MESO_MIN_SPEED: f64 = 0.05;

use utils::common::sumo_vehicle_class::{SVC_ALL, SVC_IGNORING, get_vehicle_class_names};
use utils::common::time::{SUMOTIME_MAX, SUMOTIME_MIN, steps2time, time2steps};
use crate::utils::vehicle::sumov_type_parameter::SUMOVTypeParameter;

// C++ `static const int MESegment::PARKING_QUEUE = -1`
const PARKING_QUEUE: i32 = -1;
// C++ `static const double MESegment::DO_NOT_PATCH_JAM_THRESHOLD`
const DO_NOT_PATCH_JAM_THRESHOLD: f64 = f64::MAX;

// C++ #define DEFAULT_VEH_LENGTH_WITH_GAP (SUMOVTypeParameter::getDefault().length + SUMOVTypeParameter::getDefault().minGap)
fn default_veh_length_with_gap() -> f64 {
    let d = SUMOVTypeParameter::get_default();
    d.length + d.minGap
}

// C++ MAX2 / MIN2 on doubles (same comparison order as the C++ macros)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

thread_local! {
    // static MESegment::myMultiModalWarnings
    static MY_MULTI_MODAL_WARNINGS: std::cell::Cell<SVCPermissions> = const { std::cell::Cell::new(0) };
    // static MESegment::myDummyParent
    static MY_DUMMY_PARENT: Ptr<MSEdge> = Ptr::from_mut(Box::leak(Box::new(MSEdge::new("MESegmentDummyParent", -1, SumoXMLEdgeFunc::UNKNOWN, "", "", "", -1, 0.))));
    // static MESegment::myVaporizationTarget
    static MY_VAPORIZATION_TARGET: Ptr<MESegment> = Ptr::from_mut(Box::leak(Box::new(MESegment::new_3("vaporizationTarget"))));
}

fn my_dummy_parent() -> Ptr<MSEdge> {
    MY_DUMMY_PARENT.with(|p| *p)
}

fn my_vaporization_target() -> Ptr<MESegment> {
    MY_VAPORIZATION_TARGET.with(|p| *p)
}

/// Virtual dispatch table for the model-specific MESegment methods (C++ virtuals).
/// MELSegment (meso-LTM) overrides all four; the base table holds the plain MESegment bodies.
pub struct MESegmentVt {
    pub send: fn(&mut MESegment, Ptr<MEVehicle>, Ptr<MESegment>, i32, SUMOTime, MSMoveReminder_Notification),
    pub update_entry_block_time: fn(&mut MESegment, SUMOTime),
    pub compute_headway: fn(&MESegment, &mut MESegment_Queue, &MESegment_Queue, Ptr<MESegment>, Ptr<MEVehicle>) -> SUMOTime,
    pub has_space_for_insertion: fn(&MESegment, &MESegment_Queue, i32, f64, SUMOTime) -> bool,
}

/// A single mesoscopic segment (cell)
/// C++ class `MESegment` (mesosim/MESegment.h)
#[repr(C)]
pub struct MESegment {
    /// C++ base class `Named`
    pub base: Named,
    /// The microsim edge this segment belongs to
    pub myEdge: Ptr<MSEdge>,
    /// The next segment of this edge, 0 if this is the last segment of this edge
    pub myNextSegment: Ptr<MESegment>,
    /// The segment's length
    pub myLength: f64,
    /// Running number of the segment in the edge
    pub myIndex: i32,
    /// The time headway parameters, see the Eissfeldt thesis
    pub myTau_ff: SUMOTime,
    /// The time headway parameters, see the Eissfeldt thesis
    pub myTau_fj: SUMOTime,
    /// The time headway parameters, see the Eissfeldt thesis
    pub myTau_jf: SUMOTime,
    /// The time headway parameters, see the Eissfeldt thesis
    pub myTau_jj: SUMOTime,
    /// Whether tls penalty is enabled
    pub myTLSPenalty: bool,
    /// penalty for minor links
    pub myCheckMinorPenalty: bool,
    pub myMinorPenalty: SUMOTime,
    /// Whether junction control is enabled
    pub myJunctionControl: bool,
    /// Whether overtaking is permitted on this segment
    pub myOvertaking: bool,
    /// Headway parameter for computing gross time headyway from net time headway, length and edge speed
    pub myTau_length: f64,
    /// The number of lanes represented by the queue * the length of the lane
    pub myCapacity: f64,
    /// The number of lanes represented by the queue * the length of the lane
    pub myQueueCapacity: f64,
    /// The space (in m) which needs to be occupied before the segment is considered jammed
    pub myJamThreshold: f64,
    /// The car queues. Vehicles are inserted in the front and removed in the back
    pub myQueues: Vec<MESegment_Queue>,
    /// The cached value for the number of vehicles
    pub myNumVehicles: i32,
    /// The follower edge to allowed que index mapping for multi queue segments
    pub myFollowerMap: BTreeMap<Ptr<MSEdge>, i32>,
    /// the last headway
    pub myLastHeadway: SUMOTime,
    /// the mean speed on this segment. Updated at event time or on demand (C++ `mutable`)
    pub myMeanSpeed: Cell<f64>,
    /// the time at which myMeanSpeed was last updated (C++ `mutable`)
    pub myLastMeanSpeedUpdate: Cell<SUMOTime>,
    /// Virtual dispatch table; installed by the most derived constructor
    pub vt: &'static MESegmentVt,
}

impl std::ops::Deref for MESegment {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MESegment {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MESegment (port as module-level statics, see AGENTS.md):
//   static const double MESegment::DO_NOT_PATCH_JAM_THRESHOLD;
//   static const int MESegment::PARKING_QUEUE;
//   static const std::string MESegment::OVERRIDE_TLS_PENALTIES;
//   static MSEdge MESegment::myDummyParent;
//   static MESegment MESegment::myVaporizationTarget;
//   static SVCPermissions MESegment::myMultiModalWarnings;

/// C++ base-class vtable: the plain MESegment bodies.
static ME_SEGMENT_BASE_VT: MESegmentVt = MESegmentVt {
    send: MESegment::send_base,
    update_entry_block_time: MESegment::update_entry_block_time_base,
    compute_headway: MESegment::compute_headway_base,
    has_space_for_insertion: MESegment::has_space_for_insertion_base,
};

impl MESegment {
    /// C++: `void MESegment::MESegment(const std::string &, const MSEdge &, MESegment *, const double, const double, const int, const bool, const MesoEdgeType &)`
    /// constructor
    // MESegment::MESegment(const std::string& id,
    //                      const MSEdge& parent, MESegment* next,
    //                      const double length, const double speed,
    //                      const int idx,
    //                      const bool multiQueue,
    //                      const MesoEdgeType& edgeType):
    //     Named(id), myEdge(parent), myNextSegment(next),
    //     myLength(length), myIndex(idx),
    //     myTau_length((double)TIME2STEPS(1) / MAX2(MESO_MIN_SPEED, speed)),
    //     myNumVehicles(0),
    //     myLastHeadway(TIME2STEPS(-1)),
    //     myMeanSpeed(speed),
    //     myLastMeanSpeedUpdate(SUMOTime_MIN) {
    //
    //     const std::vector<MSLane*>& lanes = parent.getLanes();
    //     int usableLanes = 0;
    //     SVCPermissions maxAllow = 0;
    //     SVCPermissions minAllow = SVCAll;
    //     for (MSLane* const l : lanes) {
    //         const SVCPermissions allow = MSEdge::getMesoPermissions(l->getPermissions());
    //         if (multiQueue) {
    //             myQueues.push_back(Queue(allow));
    //         }
    //         if (allow != 0) {
    //             usableLanes++;
    //             maxAllow |= allow;
    //             minAllow &= allow;
    //         }
    //     }
    //     if (usableLanes == 0) {
    //         // cars won't drive here. Give sensible tau values capacity for the ignored classes
    //         usableLanes = 1;
    //     }
    //     if (multiQueue) {
    //         if (next == nullptr) {
    //             for (const MSEdge* const edge : parent.getSuccessors()) {
    //                 if (edge->isTazConnector()) {
    //                     continue;
    //                 }
    //                 const std::vector<MSLane*>* const allowed = parent.allowedLanes(*edge);
    //                 assert(allowed != nullptr);
    //                 assert(allowed->size() > 0);
    //                 for (MSLane* const l : *allowed) {
    //                     std::vector<MSLane*>::const_iterator it = std::find(lanes.begin(), lanes.end(), l);
    //                     myFollowerMap[edge] |= (1 << distance(lanes.begin(), it));
    //                 }
    //             }
    //         }
    //         myQueueCapacity = length;
    //     } else {
    //         myQueues.push_back(Queue(parent.getPermissions()));
    //         maxAllow &= ~minAllow;
    //         if ((maxAllow & ~myMultiModalWarnings) != 0) {
    //             WRITE_WARNINGF("Mismodelled capacity on edge '%'. Not all lanes are usable by vClass '%'. Use option --meso-lane-queue or add unused class to option --meso-ignore-lanes-by-vclass.",
    //                            myEdge.getID(), getVehicleClassNames(maxAllow & ~myMultiModalWarnings));
    //             myMultiModalWarnings |= maxAllow;
    //         }
    //     }
    //
    //     initSegment(edgeType, parent, length * usableLanes);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, parent: &MSEdge, next: Ptr<MESegment>, length: f64, speed: f64, idx: i32, multiQueue: bool, edgeType: &MESegment_MesoEdgeType) -> Self {
        let mut seg = MESegment {
            base: Named::new(id),
            myEdge: Ptr::new(parent),
            myNextSegment: next,
            myLength: length,
            myIndex: idx,
            myTau_ff: 0,
            myTau_fj: 0,
            myTau_jf: 0,
            myTau_jj: 0,
            myTLSPenalty: false,
            myCheckMinorPenalty: false,
            myMinorPenalty: 0,
            myJunctionControl: false,
            myOvertaking: false,
            myTau_length: (time2steps(1.0) as f64) / max2(MESO_MIN_SPEED, speed),
            myCapacity: 0.,
            myQueueCapacity: 0.,
            myJamThreshold: 0.,
            myQueues: Vec::new(),
            myNumVehicles: 0,
            myFollowerMap: BTreeMap::new(),
            myLastHeadway: time2steps(-1.0),
            myMeanSpeed: Cell::new(speed),
            myLastMeanSpeedUpdate: Cell::new(SUMOTIME_MIN),
            vt: &ME_SEGMENT_BASE_VT,
        };

        let lanes = parent.get_lanes();
        let mut usableLanes: i32 = 0;
        let mut maxAllow: SVCPermissions = 0;
        let mut minAllow: SVCPermissions = SVC_ALL;
        for l in lanes.iter() {
            let allow = MSEdge::get_meso_permissions(l.get_permissions(), 0);
            if multiQueue {
                seg.myQueues.push(MESegment_Queue::new(allow));
            }
            if allow != 0 {
                usableLanes += 1;
                maxAllow |= allow;
                minAllow &= allow;
            }
        }
        if usableLanes == 0 {
            // cars won't drive here. Give sensible tau values capacity for the ignored classes
            usableLanes = 1;
        }
        if multiQueue {
            if next.is_null() {
                for edge in parent.get_successors(SVC_IGNORING).iter() {
                    if edge.is_taz_connector() {
                        continue;
                    }
                    let allowed = parent.allowed_lanes(&**edge, SVC_IGNORING, false);
                    assert!(!allowed.is_null());
                    assert!(allowed.len() > 0);
                    for l in allowed.iter() {
                        let pos = lanes.iter().position(|x| x == l).unwrap_or(lanes.len());
                        *seg.myFollowerMap.entry(*edge).or_insert(0) |= 1i32 << pos;
                    }
                }
            }
            seg.myQueueCapacity = length;
        } else {
            seg.myQueues.push(MESegment_Queue::new(parent.get_permissions()));
            maxAllow &= !minAllow;
            let warnings = MY_MULTI_MODAL_WARNINGS.with(|w| w.get());
            if (maxAllow & !warnings) != 0 {
                write_warningf!("Mismodelled capacity on edge '%'. Not all lanes are usable by vClass '%'. Use option --meso-lane-queue or add unused class to option --meso-ignore-lanes-by-vclass.",
                               parent.get_id(), get_vehicle_class_names(maxAllow & !warnings, false));
                MY_MULTI_MODAL_WARNINGS.with(|w| w.set(warnings | maxAllow));
            }
        }

        seg.init_segment(edgeType, parent, length * (usableLanes as f64));
        seg
    }

    /// C++: `void MESegment::initSegment(const MesoEdgeType &, const MSEdge &, const double)`
    /// set model parameters (may be updated from additional file after network loading is complete)
    // void
    // MESegment::initSegment(const MesoEdgeType& edgeType, const MSEdge& parent, const double capacity) {
    //
    //     myCapacity = capacity;
    //     if (myQueues.size() == 1) {
    //         const double laneScale = capacity / myLength;
    //         myQueueCapacity = capacity;
    //         myTau_length = (double)TIME2STEPS(1) / MAX2(MESO_MIN_SPEED, myMeanSpeed) / laneScale;
    //         // Eissfeldt p. 90 and 151 ff.
    //         myTau_ff = (SUMOTime)((double)edgeType.tauff / laneScale);
    //         myTau_fj = (SUMOTime)((double)edgeType.taufj / laneScale);
    //         myTau_jf = (SUMOTime)((double)edgeType.taujf / laneScale);
    //         myTau_jj = (SUMOTime)((double)edgeType.taujj / laneScale);
    //     } else {
    //         myTau_ff = edgeType.tauff;
    //         myTau_fj = edgeType.taufj;
    //         myTau_jf = edgeType.taujf;
    //         myTau_jj = edgeType.taujj;
    //     }
    //
    //     myJunctionControl = myNextSegment == nullptr && (edgeType.junctionControl || MELoop::isEnteringRoundabout(parent));
    //     myTLSPenalty = ((edgeType.tlsPenalty > 0 || edgeType.tlsFlowPenalty > 0) &&
    //                     // only apply to the last segment of a tls-controlled edge
    //                     myNextSegment == nullptr && (
    //                         parent.getToJunction()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT ||
    //                         parent.getToJunction()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION ||
    //                         parent.getToJunction()->getType() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED));
    //
    //     // only apply to the last segment of an uncontrolled edge that has at least 1 minor link
    //     myCheckMinorPenalty = (edgeType.minorPenalty > 0 &&
    //                            myNextSegment == nullptr &&
    //                            parent.getToJunction()->getType() != SumoXMLNodeType::TRAFFIC_LIGHT &&
    //                            parent.getToJunction()->getType() != SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION &&
    //                            parent.getToJunction()->getType() != SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED &&
    //                            parent.hasMinorLink());
    //     myMinorPenalty = edgeType.minorPenalty;
    //     myOvertaking = edgeType.overtaking && myCapacity > myLength;
    //
    //     //std::cout << getID() << " myMinorPenalty=" << myMinorPenalty << " myTLSPenalty=" << myTLSPenalty << " myJunctionControl=" << myJunctionControl << " myOvertaking=" << myOvertaking << "\n";
    //
    //     recomputeJamThreshold(edgeType.jamThreshold);
    // }
    #[allow(unused_variables)]
    pub fn init_segment(&mut self, edgeType: &MESegment_MesoEdgeType, parent: &MSEdge, capacity: f64) {
        self.myCapacity = capacity;
        if self.myQueues.len() == 1 {
            let laneScale = capacity / self.myLength;
            self.myQueueCapacity = capacity;
            self.myTau_length = (time2steps(1.0) as f64) / max2(MESO_MIN_SPEED, self.myMeanSpeed.get()) / laneScale;
            // Eissfeldt p. 90 and 151 ff.
            self.myTau_ff = ((edgeType.tauff as f64) / laneScale) as SUMOTime;
            self.myTau_fj = ((edgeType.taufj as f64) / laneScale) as SUMOTime;
            self.myTau_jf = ((edgeType.taujf as f64) / laneScale) as SUMOTime;
            self.myTau_jj = ((edgeType.taujj as f64) / laneScale) as SUMOTime;
        } else {
            self.myTau_ff = edgeType.tauff;
            self.myTau_fj = edgeType.taufj;
            self.myTau_jf = edgeType.taujf;
            self.myTau_jj = edgeType.taujj;
        }

        self.myJunctionControl = self.myNextSegment.is_null() && (edgeType.junctionControl || MELoop::is_entering_roundabout(parent));
        self.myTLSPenalty = (edgeType.tlsPenalty > 0. || edgeType.tlsFlowPenalty > 0.) &&
                            // only apply to the last segment of a tls-controlled edge
                            self.myNextSegment.is_null() && (
                                parent.get_to_junction().get_type() == SumoXMLNodeType::TRAFFIC_LIGHT ||
                                parent.get_to_junction().get_type() == SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION ||
                                parent.get_to_junction().get_type() == SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED);

        // only apply to the last segment of an uncontrolled edge that has at least 1 minor link
        self.myCheckMinorPenalty = edgeType.minorPenalty > 0 &&
                               self.myNextSegment.is_null() &&
                               parent.get_to_junction().get_type() != SumoXMLNodeType::TRAFFIC_LIGHT &&
                               parent.get_to_junction().get_type() != SumoXMLNodeType::TRAFFIC_LIGHT_NOJUNCTION &&
                               parent.get_to_junction().get_type() != SumoXMLNodeType::TRAFFIC_LIGHT_RIGHT_ON_RED &&
                               parent.has_minor_link();
        self.myMinorPenalty = edgeType.minorPenalty;
        self.myOvertaking = edgeType.overtaking && self.myCapacity > self.myLength;

        self.recompute_jam_threshold(edgeType.jamThreshold);
    }

    /// C++: `void MESegment::addDetector(MSMoveReminder *, int)`
    /// Adds a data collector for a detector to this segment
    // void
    // MESegment::addDetector(MSMoveReminder* data, int queueIndex) {
    //     if (queueIndex == -1) {
    //         for (Queue& q : myQueues) {
    //             q.addDetector(data);
    //         }
    //     } else {
    //         assert(queueIndex < (int)myQueues.size());
    //         myQueues[queueIndex].addDetector(data);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_detector(&mut self, data: Ptr<MSMoveReminder>, queueIndex: i32) {
        if queueIndex == -1 {
            for q in self.myQueues.iter_mut() {
                q.add_detector(data);
            }
        } else {
            assert!((queueIndex as usize) < self.myQueues.len());
            self.myQueues[queueIndex as usize].add_detector(data);
        }
    }

    /// C++: `void MESegment::prepareDetectorForWriting(MSMoveReminder &, int)`
    /// Updates data of a detector for one or all vehicle queues
    // void
    // MESegment::prepareDetectorForWriting(MSMoveReminder& data, int queueIndex) {
    //     const SUMOTime currentTime = MSNet::getInstance()->getCurrentTimeStep();
    //     if (queueIndex == -1) {
    //         for (const Queue& q : myQueues) {
    //             SUMOTime earliestExitTime = currentTime;
    //             for (std::vector<MEVehicle*>::const_reverse_iterator i = q.getVehicles().rbegin(); i != q.getVehicles().rend(); ++i) {
    //                 const SUMOTime exitTime = MAX2(earliestExitTime, (*i)->getEventTime());
    //                 (*i)->updateDetectorForWriting(&data, currentTime, exitTime);
    //                 earliestExitTime = exitTime + tauWithVehLength(myTau_ff, (*i)->getVehicleType().getLengthWithGap(), (*i)->getVehicleType().getCarFollowModel().getHeadwayTime());
    //             }
    //         }
    //     } else {
    //         SUMOTime earliestExitTime = currentTime;
    //         for (std::vector<MEVehicle*>::const_reverse_iterator i = myQueues[queueIndex].getVehicles().rbegin(); i != myQueues[queueIndex].getVehicles().rend(); ++i) {
    //             const SUMOTime exitTime = MAX2(earliestExitTime, (*i)->getEventTime());
    //             (*i)->updateDetectorForWriting(&data, currentTime, exitTime);
    //             earliestExitTime = exitTime + tauWithVehLength(myTau_ff, (*i)->getVehicleType().getLengthWithGap(), (*i)->getVehicleType().getCarFollowModel().getHeadwayTime());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn prepare_detector_for_writing(&mut self, data: &mut MSMoveReminder, queueIndex: i32) {
        let currentTime = MSNet::get_instance().get_current_time_step();
        let dataPtr = Ptr::from_mut(&mut *data);
        if queueIndex == -1 {
            for q in self.myQueues.iter() {
                let mut earliestExitTime = currentTime;
                for veh in q.get_vehicles().iter().rev() {
                    let exitTime = std::cmp::max(earliestExitTime, veh.get_event_time());
                    veh.m().update_detector_for_writing(dataPtr, currentTime, exitTime);
                    earliestExitTime = exitTime + self.tau_with_veh_length(self.myTau_ff, veh.get_vehicle_type().get_length_with_gap(), veh.get_vehicle_type().get_car_follow_model().get_headway_time());
                }
            }
        } else {
            let mut earliestExitTime = currentTime;
            for veh in self.myQueues[queueIndex as usize].get_vehicles().iter().rev() {
                let exitTime = std::cmp::max(earliestExitTime, veh.get_event_time());
                veh.m().update_detector_for_writing(dataPtr, currentTime, exitTime);
                earliestExitTime = exitTime + self.tau_with_veh_length(self.myTau_ff, veh.get_vehicle_type().get_length_with_gap(), veh.get_vehicle_type().get_car_follow_model().get_headway_time());
            }
        }
    }

    /// C++: `SUMOTime MESegment::hasSpaceFor(const MEVehicle *const, const SUMOTime, int &, const bool) const` [const]
    /// Returns whether the given vehicle would still fit into the segment
    // SUMOTime
    // MESegment::hasSpaceFor(const MEVehicle* const veh, const SUMOTime entryTime, int& qIdx, const bool init) const {
    //     SUMOTime earliestEntry = SUMOTime_MAX;
    //     qIdx = 0;
    //     if (myNumVehicles == 0 && myQueues.size() == 1) {
    //         // we have always space for at least one vehicle
    //         if (myQueues.front().allows(veh->getVClass())) {
    //             return entryTime;
    //         }  else {
    //             return earliestEntry;
    //         }
    //     }
    //     const SUMOVehicleClass svc = veh->getVClass();
    //     int minSize = std::numeric_limits<int>::max();
    //     const MSEdge* const succ = myNextSegment == nullptr ? veh->succEdge(veh->getEdge() == &myEdge ? 1 : 2) : nullptr;
    //     for (int i = 0; i < (int)myQueues.size(); i++) {
    //         const Queue& q = myQueues[i];
    //         const double newOccupancy = q.size() == 0 ? 0. : q.getOccupancy() + veh->getVehicleType().getLengthWithGap();
    //         if (newOccupancy <= myQueueCapacity) { // we must ensure that occupancy remains below capacity
    //             if (succ == nullptr || myFollowerMap.count(succ) == 0 || ((myFollowerMap.find(succ)->second & (1 << i)) != 0)) {
    //                 if (q.allows(svc) && q.size() < minSize) {
    //                     if (init) {
    //                         // regular insertions and initial insertions must respect different constraints:
    //                         if (veh->getInsertionChecks() == (int)InsertionCheck::NONE || hasSpaceForInsertion(q, i, newOccupancy, entryTime)) {
    //                             qIdx = i;
    //                             minSize = q.size();
    //                         }
    //                     } else if (entryTime >= q.getEntryBlockTime()) {
    //                         qIdx = i;
    //                         minSize = q.size();
    //                     } else {
    //                         earliestEntry = MIN2(earliestEntry, q.getEntryBlockTime());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     if (minSize == std::numeric_limits<int>::max()) {
    //         return earliestEntry;
    //     }
    //     return entryTime;
    // }
    #[allow(unused_variables)]
    pub fn has_space_for(&self, veh: Ptr<MEVehicle>, entryTime: SUMOTime, qIdx: &mut i32, init: bool) -> SUMOTime {
        let mut earliestEntry: SUMOTime = SUMOTIME_MAX;
        *qIdx = 0;
        if self.myNumVehicles == 0 && self.myQueues.len() == 1 {
            // we have always space for at least one vehicle
            if self.myQueues[0].allows(veh.get_v_class()) {
                return entryTime;
            } else {
                return earliestEntry;
            }
        }
        let svc = veh.get_v_class();
        let mut minSize: i32 = i32::MAX;
        let succ = if self.myNextSegment.is_null() { veh.succ_edge(if veh.get_edge() == self.myEdge { 1 } else { 2 }) } else { Ptr::null() };
        for i in 0..self.myQueues.len() {
            let q = &self.myQueues[i];
            let newOccupancy = if q.size() == 0 { 0. } else { q.get_occupancy() + veh.get_vehicle_type().get_length_with_gap() };
            if newOccupancy <= self.myQueueCapacity {
                // we must ensure that occupancy remains below capacity
                if succ.is_null() || self.myFollowerMap.get(&succ).map_or(true, |f| (f & (1i32 << i)) != 0) {
                    if q.allows(svc) && q.size() < minSize {
                        if init {
                            // regular insertions and initial insertions must respect different constraints:
                            if veh.get_insertion_checks() == InsertionCheck::NONE as i32 || self.has_space_for_insertion(q, i as i32, newOccupancy, entryTime) {
                                *qIdx = i as i32;
                                minSize = q.size();
                            }
                        } else if entryTime >= q.get_entry_block_time() {
                            *qIdx = i as i32;
                            minSize = q.size();
                        } else {
                            earliestEntry = std::cmp::min(earliestEntry, q.get_entry_block_time());
                        }
                    }
                }
            }
        }
        if minSize == i32::MAX {
            return earliestEntry;
        }
        entryTime
    }

    /// C++ virtual `MESegment::updateEntryBlockTime`: dispatches to the model table
    pub fn update_entry_block_time(&mut self, arg0: SUMOTime) {
        (self.vt.update_entry_block_time)(self, arg0)
    }

    /// C++: `void MESegment::updateEntryBlockTime(SUMOTime)` [virtual]
    /// update entry blockTime for all queues
    // virtual void updateEntryBlockTime(SUMOTime /*time*/) {}
    #[allow(unused_variables)]
    pub fn update_entry_block_time_base(&mut self, arg0: SUMOTime) {
        // virtual hook with an empty base implementation
    }

    /// C++: `std::string MESegment::getQueueExtraState(int) const` [const, virtual]
    /// model specific queue state (e.g. the gap times of meso-ltm) for saving / loading
    // virtual std::string getQueueExtraState(int /*qIdx*/) const {
    //         return "";
    //     }
    #[allow(unused_variables)]
    pub fn get_queue_extra_state(&self, arg0: i32) -> String {
        String::new()
    }

    /// C++: `void MESegment::loadQueueExtraState(int, const std::string &, SUMOTime)` [virtual]
    // virtual void loadQueueExtraState(int /*qIdx*/, const std::string& /*state*/, SUMOTime /*offset*/) {}
    #[allow(unused_variables)]
    pub fn load_queue_extra_state(&mut self, arg0: i32, arg1: &str, arg2: SUMOTime) {
        // virtual hook with an empty base implementation
    }

    /// C++: `bool MESegment::initialise(MEVehicle *, SUMOTime)`
    /// Inserts (emits) vehicle into the segment
    // bool
    // MESegment::initialise(MEVehicle* veh, SUMOTime time) {
    //     int qIdx = 0;
    //     if (hasSpaceFor(veh, time, qIdx, true) == time) {
    //         const bool isRail = veh->isRail();
    //         // see MSLane::isInsertionSuccess
    //         if (isRail && veh->getInsertionChecks() != (int)InsertionCheck::NONE
    //                 && veh->getParameter().departProcedure != DepartDefinition::SPLIT
    //                 && MSRailSignalControl::isSignalized(veh->getVClass())
    //                 && isRailwayOrShared(myEdge.getPermissions())) {
    //             const MSDriveWay* dw = MSDriveWay::getDepartureDriveway(veh);
    //             MSEdgeVector occupied;
    //             if (dw->foeDriveWayOccupied(false, veh, occupied)) {
    //                 myEdge.getLanes()[0]->setParameter("insertionBlocked:" + veh->getID(), dw->getID());
    //                 return false;
    //             }
    //         }
    //         receive(veh, qIdx, time, true);
    //         if (isRail) {
    //             myEdge.getLanes()[0]->unsetParameter("insertionConstraint:" + veh->getID());
    //             //unsetParameter("insertionOrder:" + veh->getID());
    //             //unsetParameter("insertionBlocked:" + veh->getID());
    //             //// rail_signal (not traffic_light) requires approach information for
    //             //// switching correctly at the start of the next simulation step
    //             //if (firstRailSignal != nullptr && firstRailSignal->getJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             //    veh->registerInsertionApproach(firstRailSignal, firstRailSignalDist);
    //             //}
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn initialise(&mut self, veh: Ptr<MEVehicle>, time: SUMOTime) -> bool {
        let mut qIdx: i32 = 0;
        if self.has_space_for(veh, time, &mut qIdx, true) == time {
            let isRail = veh.is_rail();
            // see MSLane::isInsertionSuccess
            if isRail && veh.get_insertion_checks() != InsertionCheck::NONE as i32
                    && !matches!(veh.get_parameter().departProcedure, DepartDefinition::SPLIT)
                    && MSRailSignalControl::is_signalized(veh.get_v_class())
                    && utils::common::sumo_vehicle_class::is_railway_or_shared(self.myEdge.get_permissions()) {
                let dw = MSDriveWay::get_departure_driveway(veh.cast::<SUMOVehicle>(), false);
                let mut occupied: MSEdgeVector = Vec::new();
                if dw.foe_drive_way_occupied(false, veh.cast::<SUMOVehicle>(), &mut occupied) {
                    self.myEdge.get_lanes()[0].m().base2.set_parameter(&format!("insertionBlocked:{}", veh.get_id()), dw.m().base2.get_id());
                    return false;
                }
            }
            self.receive(veh, qIdx, time, true, false, false);
            if isRail {
                self.myEdge.get_lanes()[0].m().base2.unset_parameter(&format!("insertionConstraint:{}", veh.get_id()));
                //unsetParameter("insertionOrder:" + veh->getID());
                //unsetParameter("insertionBlocked:" + veh->getID());
                //// rail_signal (not traffic_light) requires approach information for
                //// switching correctly at the start of the next simulation step
            }
            return true;
        }
        false
    }

    /// C++: `int MESegment::getCarNumber() const` [const]
    /// Returns the total number of cars on the segment
    // inline int getCarNumber() const {
    //         return myNumVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_number(&self) -> i32 {
        self.myNumVehicles
    }

    /// C++: `int MESegment::numQueues() const` [const]
    /// return the number of queues
    // inline int numQueues() const {
    //         return (int)myQueues.size();
    //     }
    #[allow(unused_variables)]
    pub fn num_queues(&self) -> i32 {
        self.myQueues.len() as i32
    }

    /// C++: `const std::vector<MEVehicle *> & MESegment::getQueue(int) const` [const]
    /// Returns the cars in the queue with the given index for visualization
    // inline const std::vector<MEVehicle*>& getQueue(int index) const {
    //         assert(index < (int)myQueues.size());
    //         return myQueues[index].getVehicles();
    //     }
    #[allow(unused_variables)]
    pub fn get_queue(&self, index: i32) -> &Vec<Ptr<MEVehicle>> {
        assert!((index as usize) < self.myQueues.len());
        self.myQueues[index as usize].get_vehicles()
    }

    /// C++: `SUMOTime MESegment::getQueueBlockTime(int) const` [const]
    // inline SUMOTime getQueueBlockTime(int index) const {
    //         assert(index < (int)myQueues.size());
    //         return myQueues[index].getBlockTime();
    //     }
    #[allow(unused_variables)]
    pub fn get_queue_block_time(&self, index: i32) -> SUMOTime {
        assert!((index as usize) < self.myQueues.len());
        self.myQueues[index as usize].get_block_time()
    }

    /// C++: `int MESegment::getIndex() const` [const]
    /// Returns the running index of the segment in the edge (0 is the most upstream).
    // inline int getIndex() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_index(&self) -> i32 {
        self.myIndex
    }

    /// C++: `MESegment * MESegment::getNextSegment() const` [const]
    /// Returns the following segment on the same edge (0 if it is the last).
    // inline MESegment* getNextSegment() const {
    //         return myNextSegment;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_segment(&self) -> Ptr<MESegment> {
        self.myNextSegment
    }

    /// C++: `double MESegment::getLength() const` [const]
    /// Returns the length of the segment in meters.
    // inline double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `double MESegment::getCapacity() const` [const]
    /// Returns the sum of the lengths of all usable lanes of the segment in meters.
    // inline double getCapacity() const {
    //         return myCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_capacity(&self) -> f64 {
        self.myCapacity
    }

    /// C++: `double MESegment::getBruttoOccupancy() const` [const]
    /// Returns the occupany of the segment (the sum of the vehicle lengths + minGaps)
    // inline double getBruttoOccupancy() const {
    //         double occ = 0.;
    //         for (const Queue& q : myQueues) {
    //             occ += q.getOccupancy();
    //         }
    //         return occ;
    //     }
    #[allow(unused_variables)]
    pub fn get_brutto_occupancy(&self) -> f64 {
        let mut occ = 0.;
        for q in self.myQueues.iter() {
            occ += q.get_occupancy();
        }
        occ
    }

    /// C++: `double MESegment::getRelativeOccupancy() const` [const]
    /// Returns the relative occupany of the segment (percentage of road used))
    // inline double getRelativeOccupancy() const {
    //         return getBruttoOccupancy() / myCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_relative_occupancy(&self) -> f64 {
        self.get_brutto_occupancy() / self.myCapacity
    }

    /// C++: `double MESegment::getRelativeJamThreshold() const` [const]
    /// Returns the relative occupany of the segment (percentage of road used)) at which the segment is considered jammed
    // inline double getRelativeJamThreshold() const {
    //         return myJamThreshold / myCapacity;
    //     }
    #[allow(unused_variables)]
    pub fn get_relative_jam_threshold(&self) -> f64 {
        self.myJamThreshold / self.myCapacity
    }

    /// C++: `double MESegment::getMeanSpeed(bool) const` [const]
    /// Returns the average speed of vehicles on the segment in meters per second. If there is no vehicle on the segment it returns the maximum allowed speed
    // double
    // MESegment::getMeanSpeed(bool useCached) const {
    //     const SUMOTime currentTime = MSNet::getInstance()->getCurrentTimeStep();
    //     if (currentTime != myLastMeanSpeedUpdate || !useCached) {
    //         myLastMeanSpeedUpdate = currentTime;
    //         double v = 0;
    //         int count = 0;
    //         for (const Queue& q : myQueues) {
    //             const SUMOTime tau = q.getOccupancy() < myJamThreshold ? myTau_ff : myTau_jf;
    //             SUMOTime earliestExitTime = currentTime;
    //             count += q.size();
    //             for (std::vector<MEVehicle*>::const_reverse_iterator veh = q.getVehicles().rbegin(); veh != q.getVehicles().rend(); ++veh) {
    //                 v += (*veh)->getConservativeSpeed(earliestExitTime); // earliestExitTime is updated!
    //                 earliestExitTime += tauWithVehLength(tau, (*veh)->getVehicleType().getLengthWithGap(), (*veh)->getVehicleType().getCarFollowModel().getHeadwayTime());
    //             }
    //         }
    //         if (count == 0) {
    //             myMeanSpeed = myEdge.getSpeedLimit();
    //         } else {
    //             myMeanSpeed = v / (double) count;
    //         }
    //     }
    //     return myMeanSpeed;
    // }
    #[allow(unused_variables)]
    pub fn get_mean_speed(&self, useCache: bool) -> f64 {
        let currentTime = MSNet::get_instance().get_current_time_step();
        if currentTime != self.myLastMeanSpeedUpdate.get() || !useCache {
            self.myLastMeanSpeedUpdate.set(currentTime);
            let mut v: f64 = 0.;
            let mut count: i32 = 0;
            for q in self.myQueues.iter() {
                let tau = if q.get_occupancy() < self.myJamThreshold { self.myTau_ff } else { self.myTau_jf };
                let mut earliestExitTime = currentTime;
                count += q.size();
                for veh in q.get_vehicles().iter().rev() {
                    v += veh.get_conservative_speed(&mut earliestExitTime); // earliestExitTime is updated!
                    earliestExitTime += self.tau_with_veh_length(tau, veh.get_vehicle_type().get_length_with_gap(), veh.get_vehicle_type().get_car_follow_model().get_headway_time());
                }
            }
            if count == 0 {
                self.myMeanSpeed.set(self.myEdge.get_speed_limit());
            } else {
                self.myMeanSpeed.set(v / (count as f64));
            }
        }
        self.myMeanSpeed.get()
    }

    /// C++: `void MESegment::resetCachedSpeeds()`
    /// reset myLastMeanSpeedUpdate
    // void
    // MESegment::resetCachedSpeeds() {
    //     myLastMeanSpeedUpdate = SUMOTime_MIN;
    // }
    #[allow(unused_variables)]
    pub fn reset_cached_speeds(&mut self) {
        self.myLastMeanSpeedUpdate.set(SUMOTIME_MIN);
    }

    /// C++: `double MESegment::getMeanSpeed() const` [const]
    /// wrapper to satisfy the FunctionBinding signature
    // inline double getMeanSpeed() const {
    //         return getMeanSpeed(true);
    //     }
    #[allow(unused_variables)]
    pub fn get_mean_speed_2(&self) -> f64 {
        self.get_mean_speed(true)
    }

    /// C++: `void MESegment::writeVehicles(OutputDevice &) const` [const]
    // void
    // MESegment::writeVehicles(OutputDevice& of) const {
    //     for (const Queue& q : myQueues) {
    //         for (const MEVehicle* const veh : q.getVehicles()) {
    //             MSXMLRawOut::writeVehicle(of, *veh);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_vehicles(&self, of: &mut OutputDevice) {
        for q in self.myQueues.iter() {
            for veh in q.get_vehicles().iter() {
                MSXMLRawOut::write_vehicle(of, &**veh);
            }
        }
    }

    /// C++: `MEVehicle * MESegment::removeCar(MEVehicle *, SUMOTime, const MSMoveReminder::Notification)`
    /// Removes the given car from the edge's que
    // MEVehicle*
    // MESegment::removeCar(MEVehicle* v, SUMOTime leaveTime, const MSMoveReminder::Notification reason) {
    //     Queue& q = myQueues[v->getQueIndex()];
    //     // One could be tempted to do  v->setSegment(next); here but position on lane will be invalid if next == 0
    //     v->updateDetectors(leaveTime, v->getEventTime(), true, reason);
    //     myNumVehicles--;
    //     myEdge.lock();
    //     MEVehicle* nextLeader = q.remove(v);
    //     myEdge.invalidateMesoCache();
    //     myEdge.unlock();
    //     return nextLeader;
    // }
    #[allow(unused_variables)]
    pub fn remove_car(&mut self, v: Ptr<MEVehicle>, leaveTime: SUMOTime, reason: MSMoveReminder_Notification) -> Ptr<MEVehicle> {
        let qi = v.get_que_index() as usize;
        // One could be tempted to do  v->setSegment(next); here but position on lane will be invalid if next == 0
        v.m().update_detectors(leaveTime, v.get_event_time(), true, reason);
        self.myNumVehicles -= 1;
        self.myEdge.lock();
        let nextLeader = self.myQueues[qi].remove(v);
        self.myEdge.invalidate_meso_cache();
        self.myEdge.unlock();
        nextLeader
    }

    /// C++: `MSLink * MESegment::getLink(const MEVehicle *, bool) const` [const]
    /// Returns the link the given car will use when passing the next junction
    // MSLink*
    // MESegment::getLink(const MEVehicle* veh, bool penalty) const {
    //     if (myJunctionControl || penalty) {
    //         const MSEdge* const nextEdge = veh->succEdge(1);
    //         if (nextEdge == nullptr || veh->getQueIndex() == PARKING_QUEUE) {
    //             return nullptr;
    //         }
    //         // try to find any link leading to our next edge, start with the lane pointed to by the que index
    //         const MSLane* const bestLane = myEdge.getLanes()[veh->getQueIndex()];
    //         for (MSLink* const link : bestLane->getLinkCont()) {
    //             if (&link->getLane()->getEdge() == nextEdge) {
    //                 return link;
    //             }
    //         }
    //         // this is for the non-multique case, maybe we should use caching here !!!
    //         for (const MSLane* const lane : myEdge.getLanes()) {
    //             if (lane != bestLane) {
    //                 for (MSLink* const link : lane->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == nextEdge) {
    //                         return link;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_link(&self, veh: Ptr<MEVehicle>, tlsPenalty: bool) -> Ptr<MSLink> {
        if self.myJunctionControl || tlsPenalty {
            let nextEdge = veh.succ_edge(1);
            if nextEdge.is_null() || veh.get_que_index() == PARKING_QUEUE {
                return Ptr::null();
            }
            // try to find any link leading to our next edge, start with the lane pointed to by the que index
            let bestLane = self.myEdge.get_lanes()[veh.get_que_index() as usize];
            for link in bestLane.get_link_cont().iter() {
                if Ptr::from_mut(link.get_lane().get_edge()) == nextEdge {
                    return *link;
                }
            }
            // this is for the non-multique case, maybe we should use caching here !!!
            for lane in self.myEdge.get_lanes().iter() {
                if *lane != bestLane {
                    for link in lane.get_link_cont().iter() {
                        if Ptr::from_mut(link.get_lane().get_edge()) == nextEdge {
                            return *link;
                        }
                    }
                }
            }
        }
        Ptr::null()
    }

    /// C++: `bool MESegment::isOpen(const MEVehicle *) const` [const]
    /// Returns whether the vehicle may use the next link
    // bool
    // MESegment::isOpen(const MEVehicle* veh) const {
    // #ifdef DEBUG_OPENED
    //     if (DEBUG_COND || DEBUG_COND2(veh)) {
    //         gDebugFlag1 = true;
    //         std::cout << SIMTIME << " opened seg=" << getID() << " veh=" << Named::getIDSecure(veh)
    //                   << " tlsPenalty=" << myTLSPenalty;
    //         const MSLink* link = getLink(veh);
    //         if (link == 0) {
    //             std::cout << " link=0";
    //         } else {
    //             std::cout << " prio=" << link->havePriority()
    //                       << " override=" << limitedControlOverride(link)
    //                       << " isOpen=" << link->opened(veh->getEventTime(), veh->getSpeed(), veh->estimateLeaveSpeed(link),
    //                                                     veh->getVehicleType().getLengthWithGap(), veh->getImpatience(),
    //                                                     veh->getVehicleType().getCarFollowModel().getMaxDecel(), veh->getWaitingTime(),
    //                                                     0, nullptr, false, veh)
    //                       << " et=" << veh->getEventTime()
    //                       << " v=" << veh->getSpeed()
    //                       << " vLeave=" << veh->estimateLeaveSpeed(link)
    //                       << " impatience=" << veh->getImpatience()
    //                       << " tWait=" << veh->getWaitingTime();
    //         }
    //         std::cout << "\n";
    //         gDebugFlag1 = false;
    //     }
    // #endif
    //     if (myTLSPenalty) {
    //         // XXX should limited control take precedence over tls penalty?
    //         return true;
    //     }
    //     const MSLink* link = getLink(veh);
    //     return (link == nullptr
    //             || link->havePriority()
    //             || limitedControlOverride(link)
    //             || link->opened(veh->getEventTime(), veh->getSpeed(), veh->estimateLeaveSpeed(link),
    //                             veh->getVehicleType().getLengthWithGap(), veh->getImpatience(),
    //                             veh->getVehicleType().getCarFollowModel().getMaxDecel(), veh->getWaitingTime(),
    //                             0, nullptr, false, veh));
    // }
    #[allow(unused_variables)]
    pub fn is_open(&self, veh: Ptr<MEVehicle>) -> bool {
        // C++ `#ifdef DEBUG_OPENED` block not ported: MESegment.cpp:51 has `//#define DEBUG_OPENED`, so it is never compiled
        if self.myTLSPenalty {
            // XXX should limited control take precedence over tls penalty?
            return true;
        }
        let link = self.get_link(veh, false);
        link.is_null()
            || link.have_priority()
            || self.limited_control_override(link)
            || link.opened(veh.get_event_time(), veh.get_speed(), veh.estimate_leave_speed(link),
                            veh.get_vehicle_type().get_length_with_gap(), veh.get_impatience(),
                            veh.get_vehicle_type().get_car_follow_model().get_max_decel(), veh.get_waiting_time(false),
                            0., Ptr::null(), false, veh.cast::<SUMOTrafficObject>(), -1.)
    }

    /// C++: `void MESegment::send(MEVehicle *, MESegment *const, const int, SUMOTime, const MSMoveReminder::Notification)` [virtual]
    /// Removes the vehicle from the segment, adapting its parameters
    // void
    // MESegment::send(MEVehicle* veh, MESegment* const next, const int nextQIdx, SUMOTime time, const MSMoveReminder::Notification reason) {
    //     Queue& q = myQueues[veh->getQueIndex()];
    //     assert(isInvalid(next) || time >= q.getBlockTime());
    //     MSLink* const link = getLink(veh);
    //     if (link != nullptr) {
    //         link->removeApproaching(veh);
    //     }
    //     if (veh->isStopped()) {
    //         veh->processStop();
    //     }
    //     MEVehicle* lc = removeCar(veh, time, reason); // new leaderCar
    //     q.setBlockTime(time);
    //     if (myEdge.isNormal() && myCapacity >= 22.5) {
    //         veh->markJammed(q.getOccupancy() > myJamThreshold);
    //     }
    //     if (!isInvalid(next)) {
    //         myLastHeadway = computeHeadway(q, next->myQueues[nextQIdx], next, veh);
    //         q.setBlockTime(time + myLastHeadway);
    //     }
    //     if (lc != nullptr) {
    //         lc->setEventTime(MAX2(lc->getEventTime(), q.getBlockTime()));
    //         MSGlobals::gMesoNet->addLeaderCar(lc, getLink(lc));
    //     }
    // }
    /// C++ virtual `MESegment::send`: dispatches to the model table
    pub fn send(&mut self, veh: Ptr<MEVehicle>, next: Ptr<MESegment>, nextQIdx: i32, time: SUMOTime, reason: MSMoveReminder_Notification) {
        (self.vt.send)(self, veh, next, nextQIdx, time, reason)
    }

    #[allow(unused_variables)]
    pub fn send_base(&mut self, veh: Ptr<MEVehicle>, next: Ptr<MESegment>, nextQIdx: i32, time: SUMOTime, reason: MSMoveReminder_Notification) {
        let qi = veh.get_que_index() as usize;
        debug_assert!(Self::is_invalid(next) || time >= self.myQueues[qi].get_block_time());
        let link = self.get_link(veh, false);
        if !link.is_null() {
            link.m().remove_approaching(veh.cast::<SUMOVehicle>());
        }
        if veh.is_stopped() {
            veh.m().process_stop();
        }
        let lc = self.remove_car(veh, time, reason); // new leaderCar
        self.myQueues[qi].set_block_time(time);
        if self.myEdge.is_normal() && self.myCapacity >= 22.5 {
            veh.m().mark_jammed(self.myQueues[qi].get_occupancy() > self.myJamThreshold);
        }
        if !Self::is_invalid(next) {
            let qp: Ptr<MESegment_Queue> = Ptr::from_mut(&mut self.myQueues[qi]);
            self.myLastHeadway = self.compute_headway(qp.m(), &next.myQueues[nextQIdx as usize], next, veh);
            self.myQueues[qi].set_block_time(time + self.myLastHeadway);
        }
        if !lc.is_null() {
            lc.m().set_event_time(std::cmp::max(lc.get_event_time(), self.myQueues[qi].get_block_time()), true);
            let lcLink = self.get_link(lc, false);
            meso_net().m().add_leader_car(lc, lcLink, false);
        }
    }

    /// C++: `void MESegment::receive(MEVehicle *, const int, SUMOTime, const bool, const bool, const bool)`
    /// Adds the vehicle to the segment, adapting its parameters
    // void
    // MESegment::receive(MEVehicle* veh, const int qIdx, SUMOTime time, const bool isDepart, const bool isTeleport, const bool newEdge) {
    //     const double speed = isDepart ? -1 : MAX2(veh->getSpeed(), MESO_MIN_SPEED); // on the previous segment
    //     veh->setSegment(this); // for arrival checking
    //     veh->setLastEntryTime(time);
    //     veh->setBlockTime(SUMOTime_MAX);
    //     if (!isDepart && (
    //                 // arrival on entering a new edge
    //                 (newEdge && myEdge.isNormal() && veh->moveRoutePointer())
    //                 // arrival on entering a new segment
    //                 || veh->hasArrived())) {
    //         // route has ended
    //         veh->setEventTime(time + TIME2STEPS(myLength / speed)); // for correct arrival speed
    //         addReminders(veh);
    //         veh->activateReminders(MSMoveReminder::NOTIFICATION_JUNCTION);
    //         veh->updateDetectors(time, veh->getEventTime(), true,
    //                              veh->getEdge()->isVaporizing() ? MSMoveReminder::NOTIFICATION_VAPORIZED_VAPORIZER : MSMoveReminder::NOTIFICATION_ARRIVED);
    //         MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         return;
    //     }
    //     assert(veh->getEdge() == &getEdge() || getEdge().isInternal());
    //     // route continues
    //     Queue& q = myQueues[qIdx];
    //     const double maxSpeedOnEdge = veh->getEdge()->getLanes()[qIdx]->getVehicleMaxSpeed(veh);
    //     const double uspeed = MAX2(maxSpeedOnEdge, MESO_MIN_SPEED);
    //     std::vector<MEVehicle*>& cars = q.getModifiableVehicles();
    //     MEVehicle* newLeader = nullptr; // first vehicle in the current queue
    //     const SUMOTime stopTime = veh->checkStop(time);
    //     SUMOTime tleave = MAX2(stopTime + TIME2STEPS(myLength / uspeed) + getLinkPenalty(veh), q.getBlockTime());
    //     if (veh->isStopped()) {
    //         myEdge.addWaiting(veh);
    //     }
    //     if (veh->isParking()) {
    //         // parking stops should take at least 1ms
    //         veh->setEventTime(MAX2(stopTime, veh->getEventTime() + 1));
    //         veh->setSegment(this, PARKING_QUEUE);
    //         myEdge.getLanes()[0]->addParking(veh);  // TODO for GUI only
    //     } else {
    //         myEdge.lock();
    //         if (cars.empty()) {
    //             cars.push_back(veh);
    //             newLeader = veh;
    //         } else {
    //             SUMOTime leaderOut = cars[0]->getEventTime();
    //             if (!isDepart && leaderOut > tleave && overtake()) {
    //                 if (cars.size() == 1) {
    //                     // the overtaken car is no longer the leader and must not block foes
    //                     MSLink* const link = getLink(cars[0]);
    //                     if (link != nullptr) {
    //                         link->removeApproaching(cars[0]);
    //                     }
    //                     MSGlobals::gMesoNet->removeLeaderCar(cars[0]);
    //                     newLeader = veh;
    //                 }
    //                 cars.insert(cars.begin() + 1, veh);
    //             } else {
    //                 tleave = MAX2(leaderOut + tauWithVehLength(myTau_ff, cars[0]->getVehicleType().getLengthWithGap(), cars[0]->getVehicleType().getCarFollowModel().getHeadwayTime()), tleave);
    //                 cars.insert(cars.begin(), veh);
    //             }
    //         }
    //         myEdge.invalidateMesoCache();
    //         myEdge.unlock();
    //         myNumVehicles++;
    //         if (!isDepart && !isTeleport) {
    //             // departs and teleports could take place anywhere on the edge so they should not block regular flow
    //             // the -1 facilitates interleaving of multiple streams
    //             q.setEntryBlockTime(time + tauWithVehLength(myTau_ff, veh->getVehicleType().getLengthWithGap(), veh->getVehicleType().getCarFollowModel().getHeadwayTime()) - 1);
    //         }
    //         q.setOccupancy(MIN2(myQueueCapacity, q.getOccupancy() + veh->getVehicleType().getLengthWithGap()));
    //         veh->setEventTime(tleave);
    //         veh->setUnqueuedEventTime(tleave);
    //         veh->setSegment(this, qIdx);
    //     }
    //     addReminders(veh);
    //     if (isDepart) {
    //         veh->onDepart();
    //         veh->activateReminders(MSMoveReminder::NOTIFICATION_DEPARTED);
    //     } else if (newEdge) {
    //         veh->activateReminders(MSMoveReminder::NOTIFICATION_JUNCTION);
    //     } else {
    //         veh->activateReminders(MSMoveReminder::NOTIFICATION_SEGMENT);
    //     }
    //     if (veh->isParking()) {
    //         MSGlobals::gMesoNet->addLeaderCar(veh, nullptr);
    //     } else {
    //         if (newLeader != nullptr) {
    //             MSGlobals::gMesoNet->addLeaderCar(newLeader, getLink(newLeader));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn receive(&mut self, veh: Ptr<MEVehicle>, qIdx: i32, time: SUMOTime, isDepart: bool, isTeleport: bool, newEdge: bool) {
        let speed = if isDepart { -1. } else { max2(veh.get_speed(), MESO_MIN_SPEED) }; // on the previous segment
        veh.m().set_segment(Ptr::from_mut(self), 0); // for arrival checking
        veh.m().set_last_entry_time(time);
        veh.m().set_block_time(SUMOTIME_MAX);
        if !isDepart && (
                    // arrival on entering a new edge
                    (newEdge && self.myEdge.is_normal() && veh.m().move_route_pointer())
                    // arrival on entering a new segment
                    || veh.has_arrived()) {
            // route has ended
            veh.m().set_event_time(time + time2steps(self.myLength / speed), true); // for correct arrival speed
            self.add_reminders(veh);
            veh.m().activate_reminders(MSMoveReminder_Notification::NOTIFICATION_JUNCTION, Ptr::null());
            let reason = if veh.get_edge().is_vaporizing() { MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_VAPORIZER } else { MSMoveReminder_Notification::NOTIFICATION_ARRIVED };
            veh.m().update_detectors(time, veh.get_event_time(), true, reason);
            MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(veh.cast::<SUMOVehicle>(), false);
            return;
        }
        // assert(veh->getEdge() == &getEdge() || getEdge().isInternal());
        // route continues
        let qi = qIdx as usize;
        let maxSpeedOnEdge = veh.get_edge().get_lanes()[qi].get_vehicle_max_speed(veh.cast::<SUMOTrafficObject>());
        let uspeed = max2(maxSpeedOnEdge, MESO_MIN_SPEED);
        let mut newLeader: Ptr<MEVehicle> = Ptr::null(); // first vehicle in the current queue
        let stopTime = veh.m().check_stop(time);
        let linkPenalty = self.get_link_penalty(veh);
        let mut tleave = std::cmp::max(stopTime + time2steps(self.myLength / uspeed) + linkPenalty, self.myQueues[qi].get_block_time());
        if veh.is_stopped() {
            self.myEdge.add_waiting(veh.cast::<SUMOVehicle>());
        }
        if veh.is_parking() {
            // parking stops should take at least 1ms
            veh.m().set_event_time(std::cmp::max(stopTime, veh.get_event_time() + 1), true);
            veh.m().set_segment(Ptr::from_mut(self), PARKING_QUEUE);
            self.myEdge.get_lanes()[0].m().add_parking(veh.cast::<MSBaseVehicle>()); // TODO for GUI only
        } else {
            self.myEdge.lock();
            if self.myQueues[qi].myVehicles.is_empty() {
                self.myQueues[qi].myVehicles.push(veh);
                newLeader = veh;
            } else {
                let leaderOut = self.myQueues[qi].myVehicles[0].get_event_time();
                if !isDepart && leaderOut > tleave && self.overtake() {
                    if self.myQueues[qi].myVehicles.len() == 1 {
                        meso_net().m().remove_leader_car(self.myQueues[qi].myVehicles[0]);
                        newLeader = veh;
                    }
                    self.myQueues[qi].myVehicles.insert(1, veh);
                } else {
                    let car0 = self.myQueues[qi].myVehicles[0];
                    let headway = self.tau_with_veh_length(self.myTau_ff, car0.get_vehicle_type().get_length_with_gap(), car0.get_vehicle_type().get_car_follow_model().get_headway_time());
                    tleave = std::cmp::max(leaderOut + headway, tleave);
                    self.myQueues[qi].myVehicles.insert(0, veh);
                }
            }
            self.myEdge.invalidate_meso_cache();
            self.myEdge.unlock();
            self.myNumVehicles += 1;
            if !isDepart && !isTeleport {
                // departs and teleports could take place anywhere on the edge so they should not block regular flow
                // the -1 facilitates interleaving of multiple streams
                let entry = time + self.tau_with_veh_length(self.myTau_ff, veh.get_vehicle_type().get_length_with_gap(), veh.get_vehicle_type().get_car_follow_model().get_headway_time()) - 1;
                self.myQueues[qi].set_entry_block_time(entry);
            }
            let occ = self.myQueues[qi].get_occupancy() + veh.get_vehicle_type().get_length_with_gap();
            self.myQueues[qi].set_occupancy(min2(self.myQueueCapacity, occ));
            veh.m().set_event_time(tleave, true);
            veh.m().set_unqueued_event_time(tleave);
            veh.m().set_segment(Ptr::from_mut(self), qIdx);
        }
        self.add_reminders(veh);
        if isDepart {
            veh.m().on_depart();
            veh.m().activate_reminders(MSMoveReminder_Notification::NOTIFICATION_DEPARTED, Ptr::null());
        } else if newEdge {
            veh.m().activate_reminders(MSMoveReminder_Notification::NOTIFICATION_JUNCTION, Ptr::null());
        } else {
            veh.m().activate_reminders(MSMoveReminder_Notification::NOTIFICATION_SEGMENT, Ptr::null());
        }
        if veh.is_parking() {
            meso_net().m().add_leader_car(veh, Ptr::null(), false);
        } else if !newLeader.is_null() {
            let leaderLink = self.get_link(newLeader, false);
            meso_net().m().add_leader_car(newLeader, leaderLink, false);
        }
    }

    /// C++: `bool MESegment::vaporizeAnyCar(SUMOTime, const MSDetectorFileOutput *)`
    /// tries to remove any car from this segment
    // bool
    // MESegment::vaporizeAnyCar(SUMOTime currentTime, const MSDetectorFileOutput* filter) {
    //     for (const Queue& q : myQueues) {
    //         if (q.size() > 0) {
    //             for (MEVehicle* const veh : q.getVehicles()) {
    //                 if (filter->vehicleApplies(*veh)) {
    //                     MSGlobals::gMesoNet->removeLeaderCar(veh);
    //                     MSGlobals::gMesoNet->changeSegment(veh, currentTime + 1, &myVaporizationTarget, MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR);
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn vaporize_any_car(&mut self, currentTime: SUMOTime, filter: Ptr<MSDetectorFileOutput>) -> bool {
        for q in self.myQueues.iter() {
            if q.size() > 0 {
                for veh in q.get_vehicles().iter() {
                    if filter.vehicle_applies(&**veh) {
                        meso_net().m().remove_leader_car(*veh);
                        meso_net().m().change_segment(*veh, currentTime + 1, my_vaporization_target(), MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR, false);
                        return true;
                    }
                }
            }
        }
        false
    }

    /// C++: `const MSEdge & MESegment::getEdge() const` [const]
    /// Returns the edge this segment belongs to
    // inline const MSEdge& getEdge() const {
    //         return myEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> &MSEdge {
        &*self.myEdge
    }

    /// C++: `void MESegment::setSpeed(double, SUMOTime, double, int)`
    /// reset mySpeed and patch the speed of all vehicles in it. Also set/recompute myJamThreshold
    // void
    // MESegment::setSpeed(double newSpeed, SUMOTime currentTime, double jamThresh, int qIdx) {
    //     recomputeJamThreshold(jamThresh);
    //     //myTau_length = MAX2(MESO_MIN_SPEED, newSpeed) * myEdge.getLanes().size() / TIME2STEPS(1);
    //     int i = 0;
    //     for (const Queue& q : myQueues) {
    //         if (q.size() != 0) {
    //             if (qIdx == -1 || qIdx == i) {
    //                 setSpeedForQueue(newSpeed, currentTime, q.getBlockTime(), q.getVehicles());
    //             }
    //         }
    //         i++;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, newSpeed: f64, currentTime: SUMOTime, jamThresh: f64, qIdx: i32) {
        self.recompute_jam_threshold(jamThresh);
        //myTau_length = MAX2(MESO_MIN_SPEED, newSpeed) * myEdge.getLanes().size() / TIME2STEPS(1);
        for i in 0..self.myQueues.len() {
            if self.myQueues[i].size() != 0 {
                if qIdx == -1 || qIdx == i as i32 {
                    let bt = self.myQueues[i].get_block_time();
                    let vehs = self.myQueues[i].get_vehicles().clone();
                    self.set_speed_for_queue(newSpeed, currentTime, bt, &vehs);
                }
            }
        }
    }

    /// C++: `SUMOTime MESegment::getEventTime() const` [const]
    /// Returns the (planned) time at which the next vehicle leaves this segment
    // SUMOTime
    // MESegment::getEventTime() const {
    //     SUMOTime result = SUMOTime_MAX;
    //     for (const Queue& q : myQueues) {
    //         if (q.size() != 0 && q.getVehicles().back()->getEventTime() < result) {
    //             result = q.getVehicles().back()->getEventTime();
    //         }
    //     }
    //     if (result < SUMOTime_MAX) {
    //         return result;
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_event_time(&self) -> SUMOTime {
        let mut result = SUMOTIME_MAX;
        for q in self.myQueues.iter() {
            if q.size() != 0 && q.get_vehicles().last().unwrap().get_event_time() < result {
                result = q.get_vehicles().last().unwrap().get_event_time();
            }
        }
        if result < SUMOTIME_MAX {
            return result;
        }
        -1
    }

    /// C++: `double MESegment::getEventTimeSeconds() const` [const]
    /// Like getEventTime but returns seconds (for visualization)
    // inline double getEventTimeSeconds() const {
    //         return STEPS2TIME(getEventTime());
    //     }
    #[allow(unused_variables)]
    pub fn get_event_time_seconds(&self) -> f64 {
        steps2time(self.get_event_time())
    }

    /// C++: `double MESegment::getLastHeadwaySeconds() const` [const]
    /// get the last headway time in seconds
    // inline double getLastHeadwaySeconds() const {
    //         return STEPS2TIME(myLastHeadway);
    //     }
    #[allow(unused_variables)]
    pub fn get_last_headway_seconds(&self) -> f64 {
        steps2time(self.myLastHeadway)
    }

    /// C++: `SUMOTime MESegment::getLastHeadway() const` [const]
    /// get the last headway time
    // inline SUMOTime getLastHeadway() const {
    //         return myLastHeadway;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_headway(&self) -> SUMOTime {
        self.myLastHeadway
    }

    /// C++: `void MESegment::setLastHeadway(SUMOTime)`
    /// restores the last headway time (when loading state)
    // void setLastHeadway(SUMOTime headway) {
    //         myLastHeadway = headway;
    //     }
    #[allow(unused_variables)]
    pub fn set_last_headway(&mut self, headway: SUMOTime) {
        self.myLastHeadway = headway;
    }

    /// C++: `double MESegment::getEntryBlockTimeSeconds() const` [const]
    /// get the earliest entry time in seconds
    // inline double getEntryBlockTimeSeconds() const {
    //         SUMOTime t = SUMOTime_MAX;
    //         for (const Queue& q : myQueues) {
    //             t = MIN2(t, q.getEntryBlockTime());
    //         }
    //         return STEPS2TIME(t);
    //     }
    #[allow(unused_variables)]
    pub fn get_entry_block_time_seconds(&self) -> f64 {
        let mut t = SUMOTIME_MAX;
        for q in self.myQueues.iter() {
            t = std::cmp::min(t, q.get_entry_block_time());
        }
        steps2time(t)
    }

    /// C++: `double MESegment::getWaitingSeconds() const` [const]
    /// Get the waiting time for vehicles in all queues
    // double
    // MESegment::getWaitingSeconds() const {
    //     double result = 0;
    //     for (const Queue& q : myQueues) {
    //         // @note: only the leader currently accumulates waitingTime but this might change in the future
    //         for (const MEVehicle* veh : q.getVehicles()) {
    //             result += veh->getWaitingSeconds();
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        let mut result = 0.;
        for q in self.myQueues.iter() {
            // @note: only the leader currently accumulates waitingTime but this might change in the future
            for veh in q.get_vehicles().iter() {
                result += veh.get_waiting_seconds();
            }
        }
        result
    }

    /// C++: `void MESegment::saveState(OutputDevice &) const` [const]
    /// Saves the state of this segment into the given stream
    // void
    // MESegment::saveState(OutputDevice& out) const {
    //     bool write = false;
    //     for (int i = 0; i < (int)myQueues.size(); i++) {
    //         if (myQueues[i].getBlockTime() != -1 || !myQueues[i].getVehicles().empty() || getQueueExtraState(i) != "") {
    //             write = true;
    //             break;
    //         }
    //     }
    //     if (write) {
    //         out.openTag(SUMO_TAG_SEGMENT).writeAttr(SUMO_ATTR_ID, getID());
    //         for (int i = 0; i < (int)myQueues.size(); i++) {
    //             const Queue& q = myQueues[i];
    //             out.openTag(SUMO_TAG_VIEWSETTINGS_VEHICLES);
    //             out.writeAttr(SUMO_ATTR_TIME, toString<SUMOTime>(q.getBlockTime()));
    //             out.writeAttr(SUMO_ATTR_BLOCKTIME, toString<SUMOTime>(q.getEntryBlockTime()));
    //             out.writeAttr(SUMO_ATTR_VALUE, q.getVehicles());
    //             const std::string extra = getQueueExtraState(i);
    //             if (extra != "") {
    //                 out.writeAttr(SUMO_ATTR_STATE, extra);
    //             }
    //             out.closeTag();
    //         }
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        let mut write = false;
        for i in 0..self.myQueues.len() {
            if self.myQueues[i].get_block_time() != -1 || !self.myQueues[i].get_vehicles().is_empty() || self.get_queue_extra_state(i as i32) != "" {
                write = true;
                break;
            }
        }
        if write {
            out.open_tag_id(SUMO_TAG_SEGMENT).write_attr(SUMO_ATTR_ID, self.get_id());
            for i in 0..self.myQueues.len() {
                let q = &self.myQueues[i];
                out.open_tag_id(SUMO_TAG_VIEWSETTINGS_VEHICLES);
                out.write_attr(SUMO_ATTR_TIME, &q.get_block_time());
                out.write_attr(SUMO_ATTR_BLOCKTIME, &q.get_entry_block_time());
                let ids: Vec<&str> = q.get_vehicles().iter().map(|v| v.get_id()).collect();
                out.write_attr(SUMO_ATTR_VALUE, &ids.join(" "));
                let extra = self.get_queue_extra_state(i as i32);
                if extra != "" {
                    out.write_attr(SUMO_ATTR_STATE, &extra);
                }
                out.close_tag("");
            }
            out.close_tag("");
        }
    }

    /// C++: `void MESegment::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MESegment::clearState() {
    //     for (Queue& q : myQueues) {
    //         q.getModifiableVehicles().clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        for q in self.myQueues.iter_mut() {
            q.get_modifiable_vehicles().clear();
        }
    }

    /// C++: `void MESegment::loadState(const std::vector<SUMOVehicle *> &, const SUMOTime, const SUMOTime, const int)`
    /// Loads the state of this segment with the given parameters
    // void
    // MESegment::loadState(const std::vector<SUMOVehicle*>& vehs, const SUMOTime blockTime, const SUMOTime entryBlockTime, const int queIdx) {
    //     Queue& q = myQueues[queIdx];
    //     for (SUMOVehicle* veh : vehs) {
    //         MEVehicle* v = static_cast<MEVehicle*>(veh);
    //         assert(v->getSegment() == this || myEdge.isInternal());
    //         if (myEdge.isInternal()) {
    //             v->setSegment(this, v->getQueIndex());
    //         }
    //         q.getModifiableVehicles().push_back(v);
    //         myNumVehicles++;
    //         q.setOccupancy(q.getOccupancy() + v->getVehicleType().getLengthWithGap());
    //         addReminders(v);
    //     }
    //     if (q.size() != 0) {
    //         // add the last vehicle of this queue
    //         // !!! one question - what about the previously added vehicle? Is it stored twice?
    //         MEVehicle* veh = q.getVehicles().back();
    //         MSGlobals::gMesoNet->addLeaderCar(veh, getLink(veh), true);
    //     }
    //     q.setBlockTime(blockTime);
    //     q.setEntryBlockTime(entryBlockTime);
    //     q.setOccupancy(MIN2(q.getOccupancy(), myQueueCapacity));
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, vehs: &Vec<Ptr<SUMOVehicle>>, blockTime: SUMOTime, entryBlockTime: SUMOTime, queIdx: i32) {
        let qi = queIdx as usize;
        for veh in vehs.iter() {
            let v: Ptr<MEVehicle> = veh.cast::<MEVehicle>();
            // assert(v->getSegment() == this || myEdge.isInternal());
            if self.myEdge.is_internal() {
                v.m().set_segment(Ptr::from_mut(self), v.get_que_index());
            }
            self.myQueues[qi].get_modifiable_vehicles().push(v);
            self.myNumVehicles += 1;
            let occ = self.myQueues[qi].get_occupancy() + v.get_vehicle_type().get_length_with_gap();
            self.myQueues[qi].set_occupancy(occ);
            self.add_reminders(v);
        }
        if self.myQueues[qi].size() != 0 {
            // add the last vehicle of this queue
            // !!! one question - what about the previously added vehicle? Is it stored twice?
            let veh = *self.myQueues[qi].get_vehicles().last().unwrap();
            let link = self.get_link(veh, false);
            meso_net().m().add_leader_car(veh, link, true);
        }
        self.myQueues[qi].set_block_time(blockTime);
        self.myQueues[qi].set_entry_block_time(entryBlockTime);
        let occ = min2(self.myQueues[qi].get_occupancy(), self.myQueueCapacity);
        self.myQueues[qi].set_occupancy(occ);
    }

    /// C++: `std::vector<const MEVehicle *> MESegment::getVehicles() const` [const]
    /// returns all vehicles (for debugging)
    // std::vector<const MEVehicle*>
    // MESegment::getVehicles() const {
    //     std::vector<const MEVehicle*> result;
    //     for (const Queue& q : myQueues) {
    //         result.insert(result.end(), q.getVehicles().begin(), q.getVehicles().end());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicles(&self) -> Vec<Ptr<MEVehicle>> {
        let mut result: Vec<Ptr<MEVehicle>> = Vec::new();
        for q in self.myQueues.iter() {
            result.extend(q.get_vehicles().iter().copied());
        }
        result
    }

    /// C++: `double MESegment::getFlow() const` [const]
    /// returns flow based on headway
    // double
    // MESegment::getFlow() const {
    //     return 3600 * getCarNumber() * getMeanSpeed() / myLength;
    // }
    #[allow(unused_variables)]
    pub fn get_flow(&self) -> f64 {
        3600. * (self.get_car_number() as f64) * self.get_mean_speed_2() / self.myLength
    }

    /// C++: `bool MESegment::isInvalid(const MESegment *)` [static]
    /// whether the given segment is 0 or encodes vaporization
    // static inline bool isInvalid(const MESegment* segment) {
    //         return segment == nullptr || segment == &myVaporizationTarget;
    //     }
    #[allow(unused_variables)]
    pub fn is_invalid(segment: Ptr<MESegment>) -> bool {
        segment.is_null() || segment == my_vaporization_target()
    }

    /// C++: `SUMOTime MESegment::getNextInsertionTime(SUMOTime) const` [const]
    /// return a time after earliestEntry at which a vehicle may be inserted at full speed
    // SUMOTime
    // MESegment::getNextInsertionTime(SUMOTime earliestEntry) const {
    //     // since we do not know which queue will be used we give a conservative estimate
    //     SUMOTime earliestLeave = earliestEntry;
    //     SUMOTime latestEntry = -1;
    //     for (const Queue& q : myQueues) {
    //         earliestLeave = MAX2(earliestLeave, q.getBlockTime());
    //         latestEntry = MAX2(latestEntry, q.getEntryBlockTime());
    //     }
    //     if (myEdge.getSpeedLimit() == 0) {
    //         return MAX2(earliestEntry, latestEntry);    // FIXME: This line is just an adhoc-fix to avoid division by zero (Leo)
    //     } else {
    //         return MAX3(earliestEntry, earliestLeave - TIME2STEPS(myLength / myEdge.getSpeedLimit()), latestEntry);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_next_insertion_time(&self, earliestEntry: SUMOTime) -> SUMOTime {
        // since we do not know which queue will be used we give a conservative estimate
        let mut earliestLeave = earliestEntry;
        let mut latestEntry: SUMOTime = -1;
        for q in self.myQueues.iter() {
            earliestLeave = std::cmp::max(earliestLeave, q.get_block_time());
            latestEntry = std::cmp::max(latestEntry, q.get_entry_block_time());
        }
        if self.myEdge.get_speed_limit() == 0. {
            std::cmp::max(earliestEntry, latestEntry) // FIXME: This line is just an adhoc-fix to avoid division by zero (Leo)
        } else {
            std::cmp::max(std::cmp::max(earliestEntry, earliestLeave - time2steps(self.myLength / self.myEdge.get_speed_limit())), latestEntry)
        }
    }

    /// C++: `int MESegment::remainingVehicleCapacity(const double) const` [const]
    /// return the remaining physical space on this segment
    // inline int remainingVehicleCapacity(const double vehLength) const {
    //         int cap = 0;
    //         for (const Queue& q : myQueues) {
    //             if (q.getOccupancy() == 0. && myQueueCapacity < vehLength) {
    //                 // even small segments can hold at least one vehicle
    //                 cap += 1;
    //             } else {
    //                 cap += (int)((myQueueCapacity - q.getOccupancy()) / vehLength);
    //             }
    //         }
    //         return cap;
    //     }
    #[allow(unused_variables)]
    pub fn remaining_vehicle_capacity(&self, vehLength: f64) -> i32 {
        let mut cap: i32 = 0;
        for q in self.myQueues.iter() {
            if q.get_occupancy() == 0. && self.myQueueCapacity < vehLength {
                // even small segments can hold at least one vehicle
                cap += 1;
            } else {
                cap += ((self.myQueueCapacity - q.get_occupancy()) / vehLength) as i32;
            }
        }
        cap
    }

    /// C++: `SUMOTime MESegment::getMinimumHeadwayTime() const` [const]
    /// return the minimum headway-time with which vehicles may enter or leave this segment
    // inline SUMOTime getMinimumHeadwayTime() const {
    //         return myTau_ff;
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_headway_time(&self) -> SUMOTime {
        self.myTau_ff
    }

    /// C++: `void MESegment::addReminders(MEVehicle *) const` [const]
    /// add this lanes MoveReminders to the given vehicle
    // void
    // MESegment::addReminders(MEVehicle* veh) const {
    //     if (veh->getQueIndex() != PARKING_QUEUE) {
    //         myQueues[veh->getQueIndex()].addReminders(veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_reminders(&self, veh: Ptr<MEVehicle>) {
        if veh.get_que_index() != PARKING_QUEUE {
            self.myQueues[veh.get_que_index() as usize].add_reminders(veh);
        }
    }

    /// C++: `SUMOTime MESegment::getLinkPenalty(const MEVehicle *) const` [const]
    /// Returns the penalty time for passing a link (if using gMesoTLSPenalty > 0 or gMesoMinorPenalty > 0)
    // SUMOTime
    // MESegment::getLinkPenalty(const MEVehicle* veh) const {
    //     const MSLink* link = getLink(veh, myTLSPenalty || myCheckMinorPenalty);
    //     if (link != nullptr) {
    //         SUMOTime result = 0;
    //         if (link->isTLSControlled() && myTLSPenalty) {
    //             result += link->getMesoTLSPenalty();
    //         }
    //         // minor tls links may get an additional penalty
    //         if (!link->havePriority() &&
    //                 // do not apply penalty on top of tLSPenalty
    //                 !myTLSPenalty &&
    //                 // do not apply penalty if limited control is active
    //                 (!MSGlobals::gMesoLimitedJunctionControl || limitedControlOverride(link))) {
    //             result += myMinorPenalty;
    //         }
    //         return result;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_link_penalty(&self, veh: Ptr<MEVehicle>) -> SUMOTime {
        let link = self.get_link(veh, self.myTLSPenalty || self.myCheckMinorPenalty);
        if !link.is_null() {
            let mut result: SUMOTime = 0;
            if link.is_tls_controlled() && self.myTLSPenalty {
                result += link.get_meso_tls_penalty();
            }
            // minor tls links may get an additional penalty
            if !link.have_priority() &&
                    // do not apply penalty on top of tLSPenalty
                    !self.myTLSPenalty &&
                    // do not apply penalty if limited control is active
                    (!MSGlobals::meso_limited_junction_control() || self.limited_control_override(link)) {
                result += self.myMinorPenalty;
            }
            result
        } else {
            0
        }
    }

    /// C++: `void MESegment::updatePermissions()`
    /// called when permissions change due to Rerouter or TraCI
    // void
    // MESegment::updatePermissions() {
    //     if (myQueues.size() > 1) {
    //         for (MSLane* lane : myEdge.getLanes()) {
    //             myQueues[lane->getIndex()].setPermissions(lane->getPermissions());
    //         }
    //     } else {
    //         myQueues.back().setPermissions(myEdge.getPermissions());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_permissions(&mut self) {
        if self.myQueues.len() > 1 {
            for lane in self.myEdge.get_lanes().iter() {
                self.myQueues[lane.get_index() as usize].set_permissions(lane.get_permissions());
            }
        } else {
            let p = self.myEdge.get_permissions();
            self.myQueues.last_mut().unwrap().set_permissions(p);
        }
    }

    /// C++: `void MESegment::overrideTLSPenalty()`
    /// whether the traffic light should use normal junction control despite penalty options
    // void overrideTLSPenalty() {
    //         myTLSPenalty = false;
    //     }
    #[allow(unused_variables)]
    pub fn override_tls_penalty(&mut self) {
        self.myTLSPenalty = false;
    }

    /// C++: `SUMOTime MESegment::getMinTauWithVehLength(double, double) const` [const]
    /// convert net time gap (leader back to follower front) to gross time gap (leader front to follower front)
    // inline SUMOTime getMinTauWithVehLength(double lengthWithGap, double vehicleTau) const {
    //         return (SUMOTime)((double)myTau_ff * vehicleTau + lengthWithGap * myTau_length);
    //     }
    #[allow(unused_variables)]
    pub fn get_min_tau_with_veh_length(&self, lengthWithGap: f64, vehicleTau: f64) -> SUMOTime {
        ((self.myTau_ff as f64) * vehicleTau + lengthWithGap * self.myTau_length) as SUMOTime
    }

    /// C++: `SUMOTime MESegment::getTauWithVehLength(int, double, double) const` [const]
    /// like getMinTauWithVehLength but using the jam-state dependent net time gap of the given queue (as in getMeanSpeed)
    // inline SUMOTime getTauWithVehLength(int qIdx, double lengthWithGap, double vehicleTau) const {
    //         assert(qIdx >= 0 && qIdx < (int)myQueues.size());
    //         const SUMOTime tau = myQueues[qIdx].getOccupancy() < myJamThreshold ? myTau_ff : myTau_jf;
    //         return tauWithVehLength(tau, lengthWithGap, vehicleTau);
    //     }
    #[allow(unused_variables)]
    pub fn get_tau_with_veh_length(&self, qIdx: i32, lengthWithGap: f64, vehicleTau: f64) -> SUMOTime {
        assert!(qIdx >= 0 && (qIdx as usize) < self.myQueues.len());
        let tau = if self.myQueues[qIdx as usize].get_occupancy() < self.myJamThreshold { self.myTau_ff } else { self.myTau_jf };
        self.tau_with_veh_length(tau, lengthWithGap, vehicleTau)
    }

    /// C++: `bool MESegment::overtake()` [protected]
    // bool
    // MESegment::overtake() {
    //     return myOvertaking && RandHelper::rand() > (getBruttoOccupancy() / myCapacity);
    // }
    #[allow(unused_variables)]
    pub fn overtake(&mut self) -> bool {
        self.myOvertaking && utils::common::rand_helper::rand() > (self.get_brutto_occupancy() / self.myCapacity)
    }

    /// C++: `void MESegment::setSpeedForQueue(double, SUMOTime, SUMOTime, const std::vector<MEVehicle *> &)` [protected]
    // void
    // MESegment::setSpeedForQueue(double newSpeed, SUMOTime currentTime, SUMOTime blockTime, const std::vector<MEVehicle*>& vehs) {
    //     MEVehicle* v = vehs.back();
    //     SUMOTime oldEarliestExitTime = currentTime;
    //     const SUMOTime oldExit = MAX2(oldEarliestExitTime, v->getEventTime());
    //     v->updateDetectors(currentTime, oldExit, false);
    //     oldEarliestExitTime = oldExit + tauWithVehLength(myTau_ff, v->getVehicleType().getLengthWithGap(), v->getVehicleType().getCarFollowModel().getHeadwayTime());
    //     SUMOTime newEvent = MAX2(newArrival(v, newSpeed, currentTime), blockTime);
    //     if (v->getEventTime() != newEvent) {
    //         MSGlobals::gMesoNet->removeLeaderCar(v);
    //         v->setEventTime(newEvent);
    //         MSGlobals::gMesoNet->addLeaderCar(v, getLink(v));
    //     }
    //     for (std::vector<MEVehicle*>::const_reverse_iterator i = vehs.rbegin() + 1; i != vehs.rend(); ++i) {
    //         const SUMOTime oldExitTime = MAX2(oldEarliestExitTime, (*i)->getEventTime());
    //         (*i)->updateDetectors(currentTime, oldExitTime, false);
    //         const SUMOTime minTau = tauWithVehLength(myTau_ff, (*i)->getVehicleType().getLengthWithGap(), (*i)->getVehicleType().getCarFollowModel().getHeadwayTime());
    //         oldEarliestExitTime = oldExitTime + minTau;
    //         newEvent = MAX2(newArrival(*i, newSpeed, currentTime), newEvent + minTau);
    //         (*i)->setEventTime(newEvent);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_speed_for_queue(&mut self, newSpeed: f64, currentTime: SUMOTime, blockTime: SUMOTime, vehs: &Vec<Ptr<MEVehicle>>) {
        let v = *vehs.last().unwrap();
        let mut oldEarliestExitTime = currentTime;
        let oldExit = std::cmp::max(oldEarliestExitTime, v.get_event_time());
        v.m().update_detectors(currentTime, oldExit, false, MSMoveReminder_Notification::NOTIFICATION_JUNCTION);
        oldEarliestExitTime = oldExit + self.tau_with_veh_length(self.myTau_ff, v.get_vehicle_type().get_length_with_gap(), v.get_vehicle_type().get_car_follow_model().get_headway_time());
        let mut newEvent = std::cmp::max(self.new_arrival(v, newSpeed, currentTime), blockTime);
        if v.get_event_time() != newEvent {
            meso_net().m().remove_leader_car(v);
            v.m().set_event_time(newEvent, true);
            let vLink = self.get_link(v, false);
            meso_net().m().add_leader_car(v, vLink, false);
        }
        for i in vehs.iter().rev().skip(1) {
            let i = *i;
            let oldExitTime = std::cmp::max(oldEarliestExitTime, i.get_event_time());
            i.m().update_detectors(currentTime, oldExitTime, false, MSMoveReminder_Notification::NOTIFICATION_JUNCTION);
            let minTau = self.tau_with_veh_length(self.myTau_ff, i.get_vehicle_type().get_length_with_gap(), i.get_vehicle_type().get_car_follow_model().get_headway_time());
            oldEarliestExitTime = oldExitTime + minTau;
            newEvent = std::cmp::max(self.new_arrival(i, newSpeed, currentTime), newEvent + minTau);
            i.m().set_event_time(newEvent, true);
        }
    }

    /// C++: `SUMOTime MESegment::newArrival(const MEVehicle *const, double, SUMOTime)` [protected]
    /// compute the new arrival time when switching speed
    // SUMOTime
    // MESegment::newArrival(const MEVehicle* const v, double newSpeed, SUMOTime currentTime) {
    //     // since speed is only an upper bound, pos may be too optimistic
    //     const double pos = MIN2(myLength, STEPS2TIME(currentTime - v->getLastEntryTime()) * v->getSpeed());
    //     // traveltime may not be 0
    //     double tt = (myLength - pos) / MAX2(newSpeed, MESO_MIN_SPEED);
    //     return currentTime + MAX2(TIME2STEPS(tt), SUMOTime(1));
    // }
    #[allow(unused_variables)]
    pub fn new_arrival(&mut self, v: Ptr<MEVehicle>, newSpeed: f64, currentTime: SUMOTime) -> SUMOTime {
        // since speed is only an upper bound, pos may be too optimistic
        let pos = min2(self.myLength, steps2time(currentTime - v.get_last_entry_time()) * v.get_speed());
        // traveltime may not be 0
        let tt = (self.myLength - pos) / max2(newSpeed, MESO_MIN_SPEED);
        currentTime + std::cmp::max(time2steps(tt), 1)
    }

    /// C++: `bool MESegment::hasBlockedLeader() const` [const, protected]
    /// whether a leader in any queue is blocked
    // bool
    // MESegment::hasBlockedLeader() const {
    //     for (const Queue& q : myQueues) {
    //         if (q.size() > 0 && q.getVehicles().back()->getWaitingTime() > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_blocked_leader(&self) -> bool {
        for q in self.myQueues.iter() {
            if q.size() > 0 && q.get_vehicles().last().unwrap().get_waiting_time(false) > 0 {
                return true;
            }
        }
        false
    }

    /// C++: `void MESegment::recomputeJamThreshold(double)` [protected]
    /// compute a value for myJamThreshold if jamThresh is negative, compute a value which allows free flow at mySpeed interpret jamThresh as the relative occupation at which jam starts
    // void
    // MESegment::recomputeJamThreshold(double jamThresh) {
    //     if (jamThresh == DO_NOT_PATCH_JAM_THRESHOLD) {
    //         return;
    //     }
    //     if (jamThresh < 0) {
    //         // compute based on speed
    //         myJamThreshold = jamThresholdForSpeed(myEdge.getSpeedLimit(), jamThresh);
    //     } else {
    //         // compute based on specified percentage
    //         myJamThreshold = jamThresh * myCapacity;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn recompute_jam_threshold(&mut self, jamThresh: f64) {
        if jamThresh == DO_NOT_PATCH_JAM_THRESHOLD {
            return;
        }
        if jamThresh < 0. {
            // compute based on speed
            self.myJamThreshold = self.jam_threshold_for_speed(self.myEdge.get_speed_limit(), jamThresh);
        } else {
            // compute based on specified percentage
            self.myJamThreshold = jamThresh * self.myCapacity;
        }
    }

    /// C++: `double MESegment::jamThresholdForSpeed(double, double) const` [const, protected]
    /// compute jam threshold for the given speed and jam-threshold option
    // double
    // MESegment::jamThresholdForSpeed(double speed, double jamThresh) const {
    //     // vehicles driving freely at maximum speed should not jam
    //     // we compute how many vehicles could possible enter the segment until the first vehicle leaves
    //     // and multiply by the space these vehicles would occupy
    //     // the jamThresh parameter is scale the resulting value
    //     if (speed == 0) {
    //         return std::numeric_limits<double>::max();  // never jam. Irrelevant at speed 0 anyway
    //     }
    // #ifdef DEBUG_JAMTHRESHOLD
    //     if (true || DEBUG_COND) {
    //         std::cout << "jamThresholdForSpeed seg=" << getID() << " speed=" << speed << " jamThresh=" << jamThresh << " ffVehs=" << std::ceil(myLength / (-jamThresh * speed * STEPS2TIME(tauWithVehLength(myTau_ff, DEFAULT_VEH_LENGTH_WITH_GAP)))) << " thresh=" << std::ceil(myLength / (-jamThresh * speed * STEPS2TIME(tauWithVehLength(myTau_ff, DEFAULT_VEH_LENGTH_WITH_GAP)))) * DEFAULT_VEH_LENGTH_WITH_GAP
    //                   << "\n";
    //     }
    // #endif
    //     return std::ceil(myLength / (-jamThresh * speed * STEPS2TIME(tauWithVehLength(myTau_ff, DEFAULT_VEH_LENGTH_WITH_GAP, 1.)))) * DEFAULT_VEH_LENGTH_WITH_GAP;
    // }
    #[allow(unused_variables)]
    pub fn jam_threshold_for_speed(&self, speed: f64, jamThresh: f64) -> f64 {
        // vehicles driving freely at maximum speed should not jam
        // we compute how many vehicles could possible enter the segment until the first vehicle leaves
        // and multiply by the space these vehicles would occupy
        // the jamThresh parameter is scale the resulting value
        if speed == 0. {
            return f64::MAX; // never jam. Irrelevant at speed 0 anyway
        }
        let dvl = default_veh_length_with_gap();
        (self.myLength / (-jamThresh * speed * steps2time(self.tau_with_veh_length(self.myTau_ff, dvl, 1.)))).ceil() * dvl
    }

    /// C++: `bool MESegment::limitedControlOverride(const MSLink *) const` [const, protected]
    /// whether the given link may be passed because the option meso-junction-control.limited is set
    // bool
    // MESegment::limitedControlOverride(const MSLink* link) const {
    //     assert(link != nullptr);
    //     if (!MSGlobals::gMesoLimitedJunctionControl) {
    //         return false;
    //     }
    //     // if the target segment of this link is not saturated junction control is disabled
    //     const MSEdge& targetEdge = link->getLane()->getEdge();
    //     const MESegment* target = MSGlobals::gMesoNet->getSegmentForEdge(targetEdge);
    //     return (target->getBruttoOccupancy() * 2 < target->myJamThreshold) && !targetEdge.isRoundabout();
    // }
    #[allow(unused_variables)]
    pub fn limited_control_override(&self, link: Ptr<MSLink>) -> bool {
        assert!(!link.is_null());
        if !MSGlobals::meso_limited_junction_control() {
            return false;
        }
        // if the target segment of this link is not saturated junction control is disabled
        let lane = link.get_lane();
        let targetEdge = lane.get_edge();
        let target = meso_net().m().get_segment_for_edge(&*targetEdge, 0.);
        (target.get_brutto_occupancy() * 2. < target.myJamThreshold) && !targetEdge.is_roundabout()
    }

    /// C++: `SUMOTime MESegment::tauWithVehLength(SUMOTime, double, double) const` [const, protected]
    /// convert net time gap (leader back to follower front) to gross time gap (leader front to follower front)
    // inline SUMOTime tauWithVehLength(SUMOTime tau, double lengthWithGap, double vehicleTau) const {
    //         return (SUMOTime)((double)tau * vehicleTau + lengthWithGap * myTau_length);
    //     }
    #[allow(unused_variables)]
    pub fn tau_with_veh_length(&self, tau: SUMOTime, lengthWithGap: f64, vehicleTau: f64) -> SUMOTime {
        ((tau as f64) * vehicleTau + lengthWithGap * self.myTau_length) as SUMOTime
    }

    /// C++: `SUMOTime MESegment::getTauJJ(double, double, double) const` [const, protected]
    // SUMOTime
    // MESegment::getTauJJ(double nextQueueSize, double nextQueueCapacity, double nextJamThreshold) const {
    //     // compute coefficients for the jam-jam headway function
    //     // this function models the effect that "empty space" needs to move
    //     // backwards through the downstream segment before the upstream segment may
    //     // send annother vehicle.
    //     // this allows jams to clear and move upstream.
    //     // the headway function f(x) depends on the number of vehicles in the
    //     // downstream segment x
    //     // f is a linear function that passes through the following fixed points:
    //     // f(n_jam_threshold) = tau_jf_withLength (for continuity)
    //     // f(headwayCapacity) = myTau_jj * headwayCapacity
    //
    //     const SUMOTime tau_jf_withLength = tauWithVehLength(myTau_jf, DEFAULT_VEH_LENGTH_WITH_GAP, 1.);
    //     // number of vehicles that fit into the NEXT queue (could be larger than expected with DEFAULT_VEH_LENGTH_WITH_GAP!)
    //     const double headwayCapacity = MAX2(nextQueueSize, nextQueueCapacity / DEFAULT_VEH_LENGTH_WITH_GAP);
    //     // number of vehicles above which the NEXT queue is jammed
    //     const double n_jam_threshold = headwayCapacity * nextJamThreshold / nextQueueCapacity;
    //
    //     // slope a and axis offset b for the jam-jam headway function
    //     // solving f(x) = a * x + b
    //     const double a = (STEPS2TIME(myTau_jj) * headwayCapacity - STEPS2TIME(tau_jf_withLength)) / (headwayCapacity - n_jam_threshold);
    //     const double b = headwayCapacity * (STEPS2TIME(myTau_jj) - a);
    //
    //     // it is only well defined for nextQueueSize >= n_jam_threshold (which may not be the case for longer vehicles), so we take the MAX
    //     return TIME2STEPS(a * MAX2(nextQueueSize, n_jam_threshold) + b);
    // }
    #[allow(unused_variables)]
    pub fn get_tau_jj(&self, nextQueueSize: f64, nextQueueCapacity: f64, nextJamThreshold: f64) -> SUMOTime {
        // compute coefficients for the jam-jam headway function
        // this function models the effect that "empty space" needs to move
        // backwards through the downstream segment before the upstream segment may
        // send annother vehicle.
        // this allows jams to clear and move upstream.
        // the headway function f(x) depends on the number of vehicles in the
        // downstream segment x
        // f is a linear function that passes through the following fixed points:
        // f(n_jam_threshold) = tau_jf_withLength (for continuity)
        // f(headwayCapacity) = myTau_jj * headwayCapacity
        let dvl = default_veh_length_with_gap();
        let tau_jf_withLength = self.tau_with_veh_length(self.myTau_jf, dvl, 1.);
        // number of vehicles that fit into the NEXT queue (could be larger than expected with DEFAULT_VEH_LENGTH_WITH_GAP!)
        let headwayCapacity = max2(nextQueueSize, nextQueueCapacity / dvl);
        // number of vehicles above which the NEXT queue is jammed
        let n_jam_threshold = headwayCapacity * nextJamThreshold / nextQueueCapacity;
        // slope a and axis offset b for the jam-jam headway function
        // solving f(x) = a * x + b
        let a = (steps2time(self.myTau_jj) * headwayCapacity - steps2time(tau_jf_withLength)) / (headwayCapacity - n_jam_threshold);
        let b = headwayCapacity * (steps2time(self.myTau_jj) - a);
        // it is only well defined for nextQueueSize >= n_jam_threshold (which may not be the case for longer vehicles), so we take the MAX
        time2steps(a * max2(nextQueueSize, n_jam_threshold) + b)
    }

    /// C++: `SUMOTime MESegment::computeHeadway(Queue &, const Queue &, const MESegment *const, const MEVehicle *) const` [const, virtual, protected]
    /// compute current headway depending on traffic state
    // SUMOTime
    // MESegment::computeHeadway(Queue& /*q*/, const Queue& qNext, const MESegment* const next, const MEVehicle* veh) const {
    //     const bool nextFree = qNext.getOccupancy() <= next->myJamThreshold;
    //     const SUMOTime tau = (!veh->wasJammed()
    //                           ? (nextFree ? myTau_ff : myTau_fj)
    //                           : (nextFree ? myTau_jf : getTauJJ((double)qNext.size(), next->myQueueCapacity, next->myJamThreshold)));
    //     assert(tau >= 0);
    //     SUMOTime headway = tauWithVehLength(tau, veh->getVehicleType().getLengthWithGap(), veh->getVehicleType().getCarFollowModel().getHeadwayTime());
    //     if (myTLSPenalty) {
    //         const MSLink* const tllink = getLink(veh, true);
    //         if (tllink != nullptr && tllink->isTLSControlled()) {
    //             assert(tllink->getGreenFraction() > 0);
    //             headway = (SUMOTime)((double)headway / tllink->getGreenFraction());
    //         }
    //     }
    //     return headway;
    // }
    /// C++ virtual `MESegment::computeHeadway`: dispatches to the model table
    pub fn compute_headway(&self, q: &mut MESegment_Queue, qNext: &MESegment_Queue, next: Ptr<MESegment>, veh: Ptr<MEVehicle>) -> SUMOTime {
        (self.vt.compute_headway)(self, q, qNext, next, veh)
    }

    #[allow(unused_variables)]
    pub fn compute_headway_base(&self, q: &mut MESegment_Queue, qNext: &MESegment_Queue, next: Ptr<MESegment>, veh: Ptr<MEVehicle>) -> SUMOTime {
        let _ = q; // C++ parameter /*q*/ is unused
        let nextFree = qNext.get_occupancy() <= next.myJamThreshold;
        let tau = if !veh.was_jammed() {
            if nextFree { self.myTau_ff } else { self.myTau_fj }
        } else if nextFree {
            self.myTau_jf
        } else {
            self.get_tau_jj(qNext.size() as f64, next.myQueueCapacity, next.myJamThreshold)
        };
        let mut headway = self.tau_with_veh_length(tau, veh.get_vehicle_type().get_length_with_gap(), veh.get_vehicle_type().get_car_follow_model().get_headway_time());
        if self.myTLSPenalty {
            let tllink = self.get_link(veh, true);
            if !tllink.is_null() && tllink.is_tls_controlled() {
                assert!(tllink.get_green_fraction() > 0.);
                headway = ((headway as f64) / tllink.get_green_fraction()) as SUMOTime;
            }
        }
        headway
    }

    /// C++: `bool MESegment::hasSpaceForInsertion(const Queue &, int, double, SUMOTime) const` [const, virtual, protected]
    /// check jam-avoidance constraints during insertion
    // bool
    // MESegment::hasSpaceForInsertion(const Queue& q, int /*qIdx*/, double newOccupancy, SUMOTime /*entryTime*/) const {
    //     // - regular insertions must respect entryBlockTime
    //     // - initial insertions should not cause additional jamming
    //     // - inserted vehicle should be able to continue at the current speed
    //     if (q.getOccupancy() <= myJamThreshold && !hasBlockedLeader() && !myTLSPenalty) {
    //         return newOccupancy <= myJamThreshold;
    //     } else {
    //         return newOccupancy <= jamThresholdForSpeed(getMeanSpeed(false), -1);
    //     }
    // }
    /// C++ virtual `MESegment::hasSpaceForInsertion`: dispatches to the model table
    pub fn has_space_for_insertion(&self, q: &MESegment_Queue, qIdx: i32, newOccupancy: f64, entryTime: SUMOTime) -> bool {
        (self.vt.has_space_for_insertion)(self, q, qIdx, newOccupancy, entryTime)
    }

    #[allow(unused_variables)]
    pub fn has_space_for_insertion_base(&self, q: &MESegment_Queue, qIdx: i32, newOccupancy: f64, entryTime: SUMOTime) -> bool {
        // - regular insertions must respect entryBlockTime
        // - initial insertions should not cause additional jamming
        // - inserted vehicle should be able to continue at the current speed
        if q.get_occupancy() <= self.myJamThreshold && !self.has_blocked_leader() && !self.myTLSPenalty {
            newOccupancy <= self.myJamThreshold
        } else {
            newOccupancy <= self.jam_threshold_for_speed(self.get_mean_speed(false), -1.)
        }
    }

    /// C++: `void MESegment::MESegment(const MESegment &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MESegment) -> Self {
        panic!("MESegment copy constructor is private and invalidated in C++; it cannot be called")
    }

    /// C++: `MESegment & MESegment::operator=(const MESegment &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MESegment) -> &mut MESegment {
        panic!("MESegment assignment operator is private and undefined in C++; it cannot be called")
    }

    /// C++: `void MESegment::MESegment(const std::string &)` [private]
    /// constructor for dummy segment
    // MESegment::MESegment(const std::string& id):
    //     Named(id),
    //     myEdge(myDummyParent), // arbitrary edge needed to supply the needed reference
    //     myNextSegment(nullptr), myLength(0), myIndex(0),
    //     myTau_ff(0), myTau_fj(0), myTau_jf(0), myTau_jj(0),
    //     myTLSPenalty(false),
    //     myCheckMinorPenalty(false),
    //     myMinorPenalty(0),
    //     myJunctionControl(false),
    //     myOvertaking(false),
    //     myTau_length(1) {
    // }
    #[allow(unused_variables)]
    pub fn new_3(id: &str) -> Self {
        MESegment {
            base: Named::new(id),
            myEdge: my_dummy_parent(),
            myNextSegment: Ptr::null(),
            myLength: 0.,
            myIndex: 0,
            myTau_ff: 0,
            myTau_fj: 0,
            myTau_jf: 0,
            myTau_jj: 0,
            myTLSPenalty: false,
            myCheckMinorPenalty: false,
            myMinorPenalty: 0,
            myJunctionControl: false,
            myOvertaking: false,
            myTau_length: 1.,
            myCapacity: 0.,
            myQueueCapacity: 0.,
            myJamThreshold: 0.,
            myQueues: Vec::new(),
            myNumVehicles: 0,
            myFollowerMap: BTreeMap::new(),
            myLastHeadway: 0,
            myMeanSpeed: Cell::new(0.),
            myLastMeanSpeedUpdate: Cell::new(SUMOTIME_MIN),
            vt: &ME_SEGMENT_BASE_VT,
        }
    }

}

/// edge type specific meso parameters
/// C++ class `MESegment::MesoEdgeType` (mesosim/MESegment.h)
#[derive(Clone, Copy)]
pub struct MESegment_MesoEdgeType {
    pub tauff: SUMOTime,
    pub taufj: SUMOTime,
    pub taujf: SUMOTime,
    pub taujj: SUMOTime,
    pub jamThreshold: f64,
    pub junctionControl: bool,
    pub tlsPenalty: f64,
    pub tlsFlowPenalty: f64,
    pub minorPenalty: SUMOTime,
    pub overtaking: bool,
    pub edgeLength: f64,
    pub lookahead: f64,
}

impl MESegment_MesoEdgeType {
}

/// C++ class `MESegment::Queue` (mesosim/MESegment.h)
pub struct MESegment_Queue {
    /// The vClass permissions for this queue
    pub myPermissions: SVCPermissions,
    pub myVehicles: Vec<Ptr<MEVehicle>>,
    /// The occupied space (in m) in the queue
    pub myOccupancy: f64,
    /// The block time for vehicles who wish to enter this queue
    pub myEntryBlockTime: SUMOTime,
    /// The block time
    pub myBlockTime: SUMOTime,
    /// The data collection for all kinds of detectors
    pub myDetectorData: Vec<Ptr<MSMoveReminder>>,
}

impl MESegment_Queue {
    /// C++: `void Queue::Queue(const SVCPermissions)`
    // Queue(const SVCPermissions permissions) : myPermissions(permissions) {}
    #[allow(unused_variables)]
    pub fn new(permissions: SVCPermissions) -> Self {
        MESegment_Queue {
            myPermissions: permissions,
            myVehicles: Vec::new(),
            myOccupancy: 0.,
            myEntryBlockTime: SUMOTIME_MIN,
            myBlockTime: -1,
            myDetectorData: Vec::new(),
        }
    }

    /// C++: `int Queue::size() const` [const]
    // inline int size() const {
    //             return (int)myVehicles.size();
    //         }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myVehicles.len() as i32
    }

    /// C++: `const std::vector<MEVehicle *> & Queue::getVehicles() const` [const]
    // inline const std::vector<MEVehicle*>& getVehicles() const {
    //             return myVehicles;
    //         }
    #[allow(unused_variables)]
    pub fn get_vehicles(&self) -> &Vec<Ptr<MEVehicle>> {
        &self.myVehicles
    }

    /// C++: `MEVehicle * Queue::remove(MEVehicle *)`
    // MEVehicle*
    // MESegment::Queue::remove(MEVehicle* v) {
    //     myOccupancy -= v->getVehicleType().getLengthWithGap();
    //     assert(std::find(myVehicles.begin(), myVehicles.end(), v) != myVehicles.end());
    //     if (v == myVehicles.back()) {
    //         myVehicles.pop_back();
    //         if (myVehicles.empty()) {
    //             myOccupancy = 0.;
    //         } else {
    //             return myVehicles.back();
    //         }
    //     } else {
    //         myVehicles.erase(std::find(myVehicles.begin(), myVehicles.end(), v));
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn remove(&mut self, v: Ptr<MEVehicle>) -> Ptr<MEVehicle> {
        self.myOccupancy -= v.get_vehicle_type().get_length_with_gap();
        if self.myVehicles.last() == Some(&v) {
            self.myVehicles.pop();
            if self.myVehicles.is_empty() {
                self.myOccupancy = 0.;
            } else {
                return *self.myVehicles.last().unwrap();
            }
        } else {
            let pos = self.myVehicles.iter().position(|x| *x == v).unwrap();
            self.myVehicles.remove(pos);
        }
        Ptr::null()
    }

    /// C++: `std::vector<MEVehicle *> & Queue::getModifiableVehicles()`
    // inline std::vector<MEVehicle*>& getModifiableVehicles() {
    //             return myVehicles;
    //         }
    #[allow(unused_variables)]
    pub fn get_modifiable_vehicles(&mut self) -> &mut Vec<Ptr<MEVehicle>> {
        &mut self.myVehicles
    }

    /// C++: `double Queue::getOccupancy() const` [const]
    // inline double getOccupancy() const {
    //             return myOccupancy;
    //         }
    #[allow(unused_variables)]
    pub fn get_occupancy(&self) -> f64 {
        self.myOccupancy
    }

    /// C++: `void Queue::setOccupancy(const double)`
    // inline void setOccupancy(const double occ) {
    //             myOccupancy = occ;
    //         }
    #[allow(unused_variables)]
    pub fn set_occupancy(&mut self, occ: f64) {
        self.myOccupancy = occ;
    }

    /// C++: `bool Queue::allows(SUMOVehicleClass) const` [const]
    // inline bool allows(SUMOVehicleClass vclass) const {
    //             return (myPermissions & vclass) == vclass;
    //         }
    #[allow(unused_variables)]
    pub fn allows(&self, vclass: SUMOVehicleClass) -> bool {
        (self.myPermissions & vclass) == vclass
    }

    /// C++: `SUMOTime Queue::getEntryBlockTime() const` [const]
    /// return the next time at which a vehicle may enter this queue
    // inline SUMOTime getEntryBlockTime() const {
    //             return myEntryBlockTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_entry_block_time(&self) -> SUMOTime {
        self.myEntryBlockTime
    }

    /// C++: `void Queue::setEntryBlockTime(SUMOTime)`
    /// set the next time at which a vehicle may enter this queue
    // inline void setEntryBlockTime(SUMOTime entryBlockTime) {
    //             myEntryBlockTime = entryBlockTime;
    //         }
    #[allow(unused_variables)]
    pub fn set_entry_block_time(&mut self, entryBlockTime: SUMOTime) {
        self.myEntryBlockTime = entryBlockTime;
    }

    /// C++: `SUMOTime Queue::getBlockTime() const` [const]
    // inline SUMOTime getBlockTime() const {
    //             return myBlockTime;
    //         }
    #[allow(unused_variables)]
    pub fn get_block_time(&self) -> SUMOTime {
        self.myBlockTime
    }

    /// C++: `void Queue::setBlockTime(SUMOTime)`
    // inline void setBlockTime(SUMOTime t) {
    //             myBlockTime = t;
    //         }
    #[allow(unused_variables)]
    pub fn set_block_time(&mut self, t: SUMOTime) {
        self.myBlockTime = t;
    }

    /// C++: `void Queue::setPermissions(SVCPermissions)`
    // inline void setPermissions(SVCPermissions p) {
    //             myPermissions = p;
    //         }
    #[allow(unused_variables)]
    pub fn set_permissions(&mut self, p: SVCPermissions) {
        self.myPermissions = p;
    }

    /// C++: `void Queue::addDetector(MSMoveReminder *)`
    // void
    // MESegment::Queue::addDetector(MSMoveReminder* data) {
    //     myDetectorData.push_back(data);
    //     for (MEVehicle* const v : myVehicles) {
    //         v->addReminder(data);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_detector(&mut self, data: Ptr<MSMoveReminder>) {
        self.myDetectorData.push(data);
        for v in self.myVehicles.iter() {
            v.m().add_reminder(data, 0.);
        }
    }

    /// C++: `void Queue::addReminders(MEVehicle *) const` [const]
    // void
    // MESegment::Queue::addReminders(MEVehicle* veh) const {
    //     for (MSMoveReminder* rem : myDetectorData) {
    //         veh->addReminder(rem);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_reminders(&self, veh: Ptr<MEVehicle>) {
        for rem in self.myDetectorData.iter() {
            veh.m().add_reminder(*rem, 0.);
        }
    }

}

