//! Port of mesosim/MELoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::SUMOTIME_MAX;

/// C++ typedef `std::priority_queue<LeaderEvent> MELoop::LeaderEventQeue` (top = earliest event)
pub type MELoop_LeaderEventQeue = BinaryHeap<MELoop_LeaderEvent>;

// C++ `MESegment::PARKING_QUEUE = -1`
const PARKING_QUEUE: i32 = -1;

// C++ static `MELoop::LeaderEvent::myEventCounter` (initialised to 0)
static LEADER_EVENT_COUNTER: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

/// C++ class `MELoop::LeaderEvent` (mesosim/MELoop.h)
#[derive(Clone)]
pub struct MELoop_LeaderEvent {
    pub time: SUMOTime,
    pub eventIndex: i64,
    pub nid: i64,
    pub veh: Ptr<MEVehicle>,
}

// C++ std::priority_queue uses operator< as "less"; the Rust max-heap takes the greatest under Ord.
// determinism: ties with equal time and eventIndex may pop in a different order than libstdc++'s heap.
impl PartialEq for MELoop_LeaderEvent {
    fn eq(&self, other: &Self) -> bool {
        self.eq_op(other)
    }
}
impl Eq for MELoop_LeaderEvent {}
impl PartialOrd for MELoop_LeaderEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MELoop_LeaderEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.lt_op(other) {
            std::cmp::Ordering::Less
        } else if other.lt_op(self) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl MELoop_LeaderEvent {
    /// C++: `void LeaderEvent::LeaderEvent(MEVehicle *)`
    // MELoop::LeaderEvent::LeaderEvent(MEVehicle* v):
    //     time(v->getEventTime()),
    //     eventIndex(v->getEventIndex()),
    //     nid(v->getNumericalID()),
    //     veh(v) { }
    #[allow(unused_variables)]
    pub fn new(v: Ptr<MEVehicle>) -> Self {
        MELoop_LeaderEvent {
            time: v.get_event_time(),
            eventIndex: v.get_event_index(),
            nid: v.get_numerical_id(),
            veh: v,
        }
    }

    /// C++: `bool LeaderEvent::operator<(const LeaderEvent &) const` [const]
    // bool operator<(const LeaderEvent& b) const {
    //             if (time == b.time) {
    //                 return eventIndex > b.eventIndex;
    //             }
    //             return time > b.time;
    //         }
    #[allow(unused_variables)]
    pub fn lt_op(&self, b: &MELoop_LeaderEvent) -> bool {
        if self.time == b.time {
            return self.eventIndex > b.eventIndex;
        }
        self.time > b.time
    }

    /// C++: `bool LeaderEvent::operator==(const LeaderEvent &) const` [const]
    // bool operator==(const LeaderEvent& b) const {
    //             return time == b.time && nid == b.nid;
    //         }
    #[allow(unused_variables)]
    pub fn eq_op(&self, b: &MELoop_LeaderEvent) -> bool {
        self.time == b.time && self.nid == b.nid
    }

}

/// The main mesocopic simulation loop
/// C++ class `MELoop` (mesosim/MELoop.h)
pub struct MELoop {
    pub myLeaderCars: MELoop_LeaderEventQeue,
    pub myInvalidatedLeaderCars: MELoop_LeaderEventQeue,
    /// mapping from internal edge ids to their initial segments
    pub myEdges2FirstSegments: Vec<Ptr<MESegment>>,
    /// the interval at which to recheck at full segments (<=0 means asap)
    pub myFullRecheckInterval: SUMOTime,
    /// the interval at which to recheck at blocked junctions (<=0 means asap)
    pub myLinkRecheckInterval: SUMOTime,
}

impl MELoop {
    /// C++: `void MELoop::MELoop(const SUMOTime)`
    /// SUMO constructor
    // MELoop::MELoop(const SUMOTime recheckInterval) : myFullRecheckInterval(recheckInterval), myLinkRecheckInterval(TIME2STEPS(1)) {
    // }
    #[allow(unused_variables)]
    pub fn new(recheckInterval: SUMOTime) -> Self {
        MELoop {
            myLeaderCars: BinaryHeap::new(),
            myInvalidatedLeaderCars: BinaryHeap::new(),
            myEdges2FirstSegments: Vec::new(),
            myFullRecheckInterval: recheckInterval,
            myLinkRecheckInterval: utils::common::time::time2steps(1.),
        }
    }

    /// C++: `void MELoop::~MELoop()`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit: the queues and the segment map are released by Rust's drop glue
    }

    /// C++: `void MELoop::simulate(SUMOTime)`
    /// Perform simulation up to the given time
    // void
    // MELoop::simulate(SUMOTime tMax) {
    //     while (!myLeaderCars.empty()) {
    //         LeaderEvent e = myLeaderCars.top();
    //         assert(e.time > tMax - DELTA_T);
    //         if (e.time > tMax) {
    //             return;
    //         }
    //         myLeaderCars.pop();
    //         while (!myInvalidatedLeaderCars.empty() && e.time > myInvalidatedLeaderCars.top().time) {
    //             myInvalidatedLeaderCars.pop();
    //         }
    //         if (!myInvalidatedLeaderCars.empty()) {
    //             if (wasInvalidated(e)) {
    //                 continue;
    //             }
    //         }
    //         //std::cout << SIMTIME << " checkChar " << e.veh->getID() << " on=" << e.veh->getSegment()->getID() << " t=" << e.time << " nid=" << e.veh->getNumericalID() << "\n";
    //         checkCar(e.veh);
    //         assert(myLeaderCars.empty() || myLeaderCars.top().time >= e.time);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn simulate(&mut self, tMax: SUMOTime) {
        while !self.myLeaderCars.is_empty() {
            let e = self.myLeaderCars.peek().unwrap().clone();
            debug_assert!(e.time > tMax - utils::common::time::delta_t());
            if e.time > tMax {
                return;
            }
            self.myLeaderCars.pop();
            while !self.myInvalidatedLeaderCars.is_empty() && e.time > self.myInvalidatedLeaderCars.peek().unwrap().time {
                self.myInvalidatedLeaderCars.pop();
            }
            if !self.myInvalidatedLeaderCars.is_empty() {
                if self.was_invalidated(&e) {
                    continue;
                }
            }
            //std::cout << SIMTIME << " checkChar " << e.veh->getID() << " on=" << e.veh->getSegment()->getID() << " t=" << e.time << " nid=" << e.veh->getNumericalID() << "\n";
            self.check_car(e.veh);
            debug_assert!(self.myLeaderCars.is_empty() || self.myLeaderCars.peek().unwrap().time >= e.time);
        }
    }

    /// C++: `void MELoop::addLeaderCar(MEVehicle *, MSLink *, const bool)`
    /// Adds the given car to the leading vehicles
    // void
    // MELoop::addLeaderCar(MEVehicle* veh, MSLink* link, const bool fromState) {
    //     if (!fromState) {
    //         veh->setEventIndex(LeaderEvent::myEventCounter++);
    //     }
    //     myLeaderCars.push(LeaderEvent(veh));
    //     veh->setApproaching(link);
    // }
    #[allow(unused_variables)]
    pub fn add_leader_car(&mut self, veh: Ptr<MEVehicle>, link: Ptr<MSLink>, fromState: bool) {
        if !fromState {
            veh.m().set_event_index(LEADER_EVENT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        }
        self.myLeaderCars.push(MELoop_LeaderEvent::new(veh));
        veh.m().set_approaching(link);
    }

    /// C++: `void MELoop::removeLeaderCar(MEVehicle *)`
    /// Removes the given car from the leading vehicles
    // void
    // MELoop::removeLeaderCar(MEVehicle* v) {
    //     myInvalidatedLeaderCars.push(LeaderEvent(v));
    // }
    #[allow(unused_variables)]
    pub fn remove_leader_car(&mut self, v: Ptr<MEVehicle>) {
        self.myInvalidatedLeaderCars.push(MELoop_LeaderEvent::new(v));
    }

    /// C++: `void MELoop::vaporizeCar(MEVehicle *, MSMoveReminder::Notification)`
    /// remove the given car and clean up the relevant data structures
    // void
    // MELoop::vaporizeCar(MEVehicle* v, MSMoveReminder::Notification reason) {
    //     int qIdx = 0;
    //     v->getSegment()->send(v, nullptr, qIdx, MSNet::getInstance()->getCurrentTimeStep(), reason);
    //     removeLeaderCar(v);
    // }
    #[allow(unused_variables)]
    pub fn vaporize_car(&mut self, v: Ptr<MEVehicle>, reason: MSMoveReminder_Notification) {
        let qIdx = 0;
        let now = MSNet::get_instance().get_current_time_step();
        v.get_segment().m().send(v, Ptr::null(), qIdx, now, reason);
        self.remove_leader_car(v);
    }

    /// C++: `bool MELoop::isEnteringRoundabout(const MSEdge &)` [static]
    /// whether the given edge is entering a roundabout
    // bool
    // MELoop::isEnteringRoundabout(const MSEdge& e) {
    //     for (const MSEdge* succ : e.getSuccessors()) {
    //         if (succ->isRoundabout()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_entering_roundabout(e: &MSEdge) -> bool {
        for succ in e.get_successors(utils::common::sumo_vehicle_class::SVC_IGNORING).iter() {
            if succ.is_roundabout() {
                return true;
            }
        }
        false
    }

    /// C++: `int MELoop::numSegmentsFor(const double, const double)` [static]
    /// Compute number of segments per edge (best value stay close to the configured segment length)
    // int
    // MELoop::numSegmentsFor(const double length, const double sLength) {
    //     int no = (int)floor(length / sLength + 0.5);
    //     if (no == 0) { // assure there is at least one segment
    //         return 1;
    //     } else {
    //         return no;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn num_segments_for(length: f64, slength: f64) -> i32 {
        let no = (length / slength + 0.5).floor() as i32;
        if no == 0 {
            // assure there is at least one segment
            1
        } else {
            no
        }
    }

    /// C++: `void MELoop::buildSegmentsFor(const MSEdge &, const OptionsCont &)`
    /// Build the segments for a given edge
    // void
    // MELoop::buildSegmentsFor(const MSEdge& e, const OptionsCont& oc) {
    //     const MESegment::MesoEdgeType& edgeType = MSNet::getInstance()->getMesoType(e.getEdgeType());
    //     const double length = e.getLength();
    //     const int numSegments = numSegmentsFor(length, edgeType.edgeLength);
    //     const double slength = length / (double)numSegments;
    //     MESegment* newSegment = nullptr;
    //     MESegment* nextSegment = nullptr;
    //     const bool laneQueue = oc.getBool("meso-lane-queue");
    //     const bool lift = oc.getBool("meso-ltm");
    //     bool multiQueue = laneQueue || (oc.getBool("meso-multi-queue") && e.getLanes().size() > 1 && e.getNumSuccessors() > 1);
    //     for (int s = numSegments - 1; s >= 0; s--) {
    //         std::string id = e.getID() + ":" + toString(s);
    //         newSegment = lift
    //                      ? new MELSegment(id, e, nextSegment, slength, e.getLanes()[0]->getSpeedLimit(), s, multiQueue, edgeType)
    //                      : new MESegment(id, e, nextSegment, slength, e.getLanes()[0]->getSpeedLimit(), s, multiQueue, edgeType);
    //         multiQueue = laneQueue;
    //         nextSegment = newSegment;
    //     }
    //     while (e.getNumericalID() >= static_cast<int>(myEdges2FirstSegments.size())) {
    //         myEdges2FirstSegments.push_back(0);
    //     }
    //     myEdges2FirstSegments[e.getNumericalID()] = newSegment;
    //     for (MSLane* lane : e.getLanes()) {
    //         lane->updateMesoGUISegments();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_segments_for(&mut self, e: &MSEdge, oc: &OptionsCont) {
        let net = MSNet::get_instance();
        let edgeType: &MESegment_MesoEdgeType = net.m().get_meso_type(e.get_edge_type());
        let length = e.get_length();
        let numSegments = Self::num_segments_for(length, edgeType.edgeLength);
        let slength = length / (numSegments as f64);
        let mut newSegment: Ptr<MESegment> = Ptr::null();
        let mut nextSegment: Ptr<MESegment> = Ptr::null();
        let laneQueue = oc.get_bool("meso-lane-queue").unwrap();
        let lift = oc.get_bool("meso-ltm").unwrap();
        let mut multiQueue = laneQueue || (oc.get_bool("meso-multi-queue").unwrap() && e.get_lanes().len() > 1 && e.get_num_successors() > 1);
        for s in (0..numSegments).rev() {
            let id = format!("{}:{}", e.get_id(), s);
            let speed = e.get_lanes()[0].get_speed_limit();
            // segments live for the whole run (C++ owns them with raw new); leaked here so the Ptr stays valid
            // shortcut: segments are never freed, matching C++ (no delete in the MELoop destructor)
            newSegment = if lift {
                let seg: &mut MELSegment = Box::leak(Box::new(MELSegment::new(&id, e, nextSegment, slength, speed, s, multiQueue, edgeType)));
                Ptr::from_mut(&mut seg.base)
            } else {
                let seg: &mut MESegment = Box::leak(Box::new(MESegment::new(&id, e, nextSegment, slength, speed, s, multiQueue, edgeType)));
                Ptr::from_mut(seg)
            };
            multiQueue = laneQueue;
            nextSegment = newSegment;
        }
        while e.get_numerical_id() >= self.myEdges2FirstSegments.len() as i32 {
            self.myEdges2FirstSegments.push(Ptr::null());
        }
        self.myEdges2FirstSegments[e.get_numerical_id() as usize] = newSegment;
        for lane in e.get_lanes().iter() {
            lane.m().update_meso_gui_segments();
        }
    }

    /// C++: `MESegment * MELoop::getSegmentForEdge(const MSEdge &, double)`
    /// Get the segment for a given edge at a given position
    // MESegment*
    // MELoop::getSegmentForEdge(const MSEdge& e, double pos) {
    //     if (e.getNumericalID() >= (int)myEdges2FirstSegments.size()) {
    //         return nullptr;
    //     }
    //     MESegment* s = myEdges2FirstSegments[e.getNumericalID()];
    //     if (pos > 0) {
    //         double cpos = 0;
    //         while (s->getNextSegment() != nullptr && cpos + s->getLength() < pos) {
    //             cpos += s->getLength();
    //             s = s->getNextSegment();
    //         }
    //     }
    //     return s;
    // }
    #[allow(unused_variables)]
    pub fn get_segment_for_edge(&mut self, e: &MSEdge, pos: f64) -> Ptr<MESegment> {
        if e.get_numerical_id() >= self.myEdges2FirstSegments.len() as i32 {
            return Ptr::null();
        }
        let mut s = self.myEdges2FirstSegments[e.get_numerical_id() as usize];
        if pos > 0. {
            let mut cpos = 0.;
            while !s.get_next_segment().is_null() && cpos + s.get_length() < pos {
                cpos += s.get_length();
                s = s.get_next_segment();
            }
        }
        s
    }

    /// C++: `SUMOTime MELoop::changeSegment(MEVehicle *, SUMOTime, MESegment *const, MSMoveReminder::Notification, const bool) const` [const]
    /// change to the next segment this handles combinations of the following cases: (ending / continuing route) and (leaving segment / finishing teleport)
    // SUMOTime
    // MELoop::changeSegment(MEVehicle* veh, SUMOTime leaveTime, MESegment* const toSegment, MSMoveReminder::Notification reason, const bool ignoreLink) const {
    //     int qIdx = 0;
    //     MESegment* const onSegment = veh->getSegment();
    //     if (MESegment::isInvalid(toSegment)) {
    //         if (veh->isStoppedTriggered()) {
    //             return leaveTime + MAX2(SUMOTime(1), myLinkRecheckInterval);
    //         }
    //         if (onSegment != nullptr) {
    //             onSegment->send(veh, toSegment, qIdx, leaveTime, reason);
    //         } else {
    //             WRITE_WARNINGF(TL("Vehicle '%' teleports beyond arrival edge '%', time=%."),
    //                            veh->getID(), veh->getEdge()->getID(), time2string(leaveTime));
    //         }
    //         veh->setSegment(toSegment); // signal arrival
    //         MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         return leaveTime;
    //     } else if ((!MSGlobals::gCheckRoutes || veh->ignoreTransientPermissions())
    //                && !ignoreLink
    //                && !MESegment::isInvalid(onSegment)
    //                && &onSegment->getEdge() != &toSegment->getEdge()
    //                && veh->getEdge()->allowedLanes(*veh->succEdge(1), veh->getVClass()) == nullptr) {
    //         if (veh->isStopped()) {
    //             veh->processStop();
    //         }
    //         // all queues on the next segment are forbidden. We don't want to check
    //         // too often whether the forbiden edge has become permitted again
    //         if (MSGlobals::gTimeToGridlock > 0) {
    //             // if teleporting is enabled, make sure we look at the vehicle when the gridlock-time is up
    //             const SUMOTime recheck = MSGlobals::gTimeToTeleportDisconnected >= 0 ? MIN2(MSGlobals::gTimeToGridlock, MSGlobals::gTimeToTeleportDisconnected) : MSGlobals::gTimeToGridlock;
    //             return MAX2(MIN2(leaveTime + myLinkRecheckInterval, MIN2(veh->getBlockTime(), leaveTime) + recheck + 1), leaveTime + 1);
    //         }
    //         return leaveTime + myLinkRecheckInterval;
    //     }
    //     toSegment->updateEntryBlockTime(leaveTime);
    //     const SUMOTime entry = toSegment->hasSpaceFor(veh, leaveTime, qIdx);
    //     if (entry == leaveTime && (ignoreLink || veh->mayProceed())) {
    //         if (onSegment != nullptr) {
    //             if (veh->getQueIndex() == MESegment::PARKING_QUEUE) { // parking or just aborted parking
    //                 if (veh->isParking()) {
    //                     veh->processStop();
    //                 }
    //                 veh->getEdge()->getLanes()[0]->removeParking(veh);  // TODO for GUI only
    //             } else {
    //                 onSegment->send(veh, toSegment, qIdx, leaveTime, onSegment->getNextSegment() == nullptr ? MSMoveReminder::NOTIFICATION_JUNCTION : MSMoveReminder::NOTIFICATION_SEGMENT);
    //             }
    //             toSegment->receive(veh, qIdx, leaveTime, false, ignoreLink, &onSegment->getEdge() != &toSegment->getEdge());
    //         } else {
    //             WRITE_WARNINGF(TL("Vehicle '%' ends teleporting on edge '%':%, time=%."),
    //                            veh->getID(), toSegment->getEdge().getID(), toSegment->getIndex(), time2string(leaveTime));
    //             // this is not quite correct but suffices for interrogation by
    //             // subsequent methods (veh->getSpeed() needs segment != 0)
    //             veh->setSegment(myEdges2FirstSegments[veh->getEdge()->getNumericalID()]);
    //             // clean up detectors (do not add traffic data)
    //             // note: updateDatector is not called if leaveTime == getLastEntryTime()
    //             veh->updateDetectors(veh->getLastEntryTime(), veh->getEventTime(), true, MSMoveReminder::NOTIFICATION_TELEPORT);
    //             toSegment->receive(veh, qIdx, leaveTime, false, true, true);
    //         }
    //         return entry;
    //     } else {
    //         if (veh->getQueIndex() != MESegment::PARKING_QUEUE && veh->isStopped() && veh->endTriggeredStop()) {
    //             veh->processStop();
    //         }
    //     }
    //     if (entry == leaveTime && !ignoreLink) { // this is a long way of saying !veh->mayProceed() (which is a costly call)
    //         return entry + MAX2(SUMOTime(1), myLinkRecheckInterval);
    //     }
    //     return entry;
    // }
    #[allow(unused_variables)]
    pub fn change_segment(&self, veh: Ptr<MEVehicle>, leaveTime: SUMOTime, toSegment: Ptr<MESegment>, reason: MSMoveReminder_Notification, ignoreLink: bool) -> SUMOTime {
        let qIdx: i32 = 0;
        let onSegment = veh.get_segment();
        if MESegment::is_invalid(toSegment) {
            if veh.is_stopped_triggered() {
                return leaveTime + std::cmp::max(1, self.myLinkRecheckInterval);
            }
            if !onSegment.is_null() {
                onSegment.m().send(veh, toSegment, qIdx, leaveTime, reason);
            } else {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' teleports beyond arrival edge '%', time=%.",
                    veh.get_id(), veh.get_edge().get_id(), utils::common::time::time2string(leaveTime)));
            }
            veh.m().set_segment(toSegment, 0); // signal arrival
            MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(veh.cast::<SUMOVehicle>(), false);
            return leaveTime;
        } else if (!MSGlobals::check_routes() || veh.ignore_transient_permissions())
            && !ignoreLink
            && !MESegment::is_invalid(onSegment)
            && !std::ptr::eq(onSegment.get_edge(), toSegment.get_edge())
            && veh.get_edge().allowed_lanes(&veh.succ_edge(1), veh.get_v_class(), false).is_null() {
            if veh.is_stopped() {
                veh.m().process_stop();
            }
            // all queues on the next segment are forbidden. We don't want to check
            // too often whether the forbiden edge has become permitted again
            if MSGlobals::time_to_gridlock() > 0 {
                // if teleporting is enabled, make sure we look at the vehicle when the gridlock-time is up
                let recheck = if MSGlobals::time_to_teleport_disconnected() >= 0 { std::cmp::min(MSGlobals::time_to_gridlock(), MSGlobals::time_to_teleport_disconnected()) } else { MSGlobals::time_to_gridlock() };
                return std::cmp::max(std::cmp::min(leaveTime + self.myLinkRecheckInterval, std::cmp::min(veh.get_block_time(), leaveTime) + recheck + 1), leaveTime + 1);
            }
            return leaveTime + self.myLinkRecheckInterval;
        }
        toSegment.m().update_entry_block_time(leaveTime);
        let mut qIdx = qIdx;
        let entry = toSegment.has_space_for(veh, leaveTime, &mut qIdx, false);
        if entry == leaveTime && (ignoreLink || veh.m().may_proceed()) {
            if !onSegment.is_null() {
                if veh.get_que_index() == PARKING_QUEUE { // parking or just aborted parking
                    if veh.is_parking() {
                        veh.m().process_stop();
                    }
                    veh.get_edge().get_lanes()[0].m().remove_parking(veh.cast::<MSBaseVehicle>()); // TODO for GUI only
                } else {
                    let nextReason = if onSegment.get_next_segment().is_null() { MSMoveReminder_Notification::NOTIFICATION_JUNCTION } else { MSMoveReminder_Notification::NOTIFICATION_SEGMENT };
                    onSegment.m().send(veh, toSegment, qIdx, leaveTime, nextReason);
                }
                toSegment.m().receive(veh, qIdx, leaveTime, false, ignoreLink, !std::ptr::eq(onSegment.get_edge(), toSegment.get_edge()));
            } else {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ends teleporting on edge '%':%, time=%.",
                    veh.get_id(), toSegment.get_edge().get_id(), toSegment.get_index(), utils::common::time::time2string(leaveTime)));
                // this is not quite correct but suffices for interrogation by
                // subsequent methods (veh->getSpeed() needs segment != 0)
                veh.m().set_segment(self.myEdges2FirstSegments[veh.get_edge().get_numerical_id() as usize], 0);
                // clean up detectors (do not add traffic data)
                // note: updateDatector is not called if leaveTime == getLastEntryTime()
                veh.m().update_detectors(veh.get_last_entry_time(), veh.get_event_time(), true, MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
                toSegment.m().receive(veh, qIdx, leaveTime, false, true, true);
            }
            return entry;
        } else {
            if veh.get_que_index() != PARKING_QUEUE && veh.is_stopped() && veh.m().end_triggered_stop() {
                veh.m().process_stop();
            }
        }
        if entry == leaveTime && !ignoreLink { // this is a long way of saying !veh->mayProceed() (which is a costly call)
            return entry + std::cmp::max(1, self.myLinkRecheckInterval);
        }
        entry
    }

    /// C++: `void MELoop::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MELoop::clearState() {
    //     myLeaderCars = LeaderEventQeue();
    //     myInvalidatedLeaderCars = LeaderEventQeue();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        self.myLeaderCars = BinaryHeap::new();
        self.myInvalidatedLeaderCars = BinaryHeap::new();
    }

    /// C++: `long long MELoop::getEventCounter()` [static]
    /// the number of leader events so far (for saving and loading state)
    // static long long int getEventCounter() {
    //         return LeaderEvent::myEventCounter;
    //     }
    #[allow(unused_variables)]
    pub fn get_event_counter() -> i64 {
        LEADER_EVENT_COUNTER.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// C++: `void MELoop::setEventCounter(long long)` [static]
    // static void setEventCounter(long long int counter) {
    //         LeaderEvent::myEventCounter = counter;
    //     }
    #[allow(unused_variables)]
    pub fn set_event_counter(counter: i64) {
        LEADER_EVENT_COUNTER.store(counter, std::sync::atomic::Ordering::Relaxed);
    }

    /// C++: `void MELoop::checkCar(MEVehicle *)` [private]
    /// Check whether the vehicle may move
    // void
    // MELoop::checkCar(MEVehicle* veh) {
    //     const SUMOTime leaveTime = veh->getEventTime();
    //     MESegment* const onSegment = veh->getSegment();
    //     MESegment* const toSegment = veh->getQueIndex() == MESegment::PARKING_QUEUE ? onSegment : nextSegment(onSegment, veh);
    //     const bool teleporting = (onSegment == nullptr); // is the vehicle currently teleporting?
    //     // @note reason is only evaluated if toSegment == nullptr
    //     const SUMOTime nextEntry = changeSegment(veh, leaveTime, toSegment, MSMoveReminder::NOTIFICATION_ARRIVED, teleporting);
    //     if (nextEntry == leaveTime) {
    //         return;
    //     }
    //     const bool r1 = MSGlobals::gTimeToGridlock > 0 && veh->getWaitingTime() > MSGlobals::gTimeToGridlock;
    //     const bool r3 = MSGlobals::gTimeToTeleportDisconnected >= 0 && veh->getWaitingTime() > MSGlobals::gTimeToTeleportDisconnected;
    //     if (!veh->isStopped() && (r1 || r3)) {
    //         const bool disconnected = (MSGlobals::gTimeToTeleportDisconnected >= 0
    //                                    && veh->succEdge(1) != nullptr
    //                                    && veh->getEdge()->allowedLanes(*veh->succEdge(1), veh->getVClass()) == nullptr);
    //         if ((r1 && !disconnected) || (r3 && disconnected)) {
    //             teleportVehicle(veh, toSegment, disconnected);
    //             return;
    //         }
    //     }
    //     if (veh->getBlockTime() == SUMOTime_MAX && (!veh->isStopped()
    //             || (!veh->isStoppedTriggered() && veh->isStoppedParking()))) {
    //         veh->setBlockTime(leaveTime);
    //     }
    //     if (nextEntry == SUMOTime_MAX) {
    //         // all usable queues on the next segment are full
    //         SUMOTime newEventTime = MAX3(toSegment->getEventTime() + 1, leaveTime + 1, leaveTime + myFullRecheckInterval);
    //         if (MSGlobals::gTimeToGridlock > 0) {
    //             // if teleporting is enabled, make sure we look at the vehicle when the gridlock-time is up
    //             const SUMOTime recheck = MSGlobals::gTimeToTeleportDisconnected >= 0 ? MIN2(MSGlobals::gTimeToGridlock, MSGlobals::gTimeToTeleportDisconnected) : MSGlobals::gTimeToGridlock;
    //             newEventTime = MAX2(MIN2(newEventTime, veh->getBlockTime() + recheck + 1), leaveTime + 1);
    //         }
    //         veh->setEventTime(newEventTime);
    //     } else {
    //         // receiving segment has recently received another vehicle or the junction is blocked
    //         veh->setEventTime(nextEntry);
    //     }
    //     addLeaderCar(veh, teleporting ? nullptr : onSegment->getLink(veh));
    // }
    #[allow(unused_variables)]
    pub fn check_car(&mut self, veh: Ptr<MEVehicle>) {
        let leaveTime = veh.get_event_time();
        let onSegment = veh.get_segment();
        let toSegment = if veh.get_que_index() == PARKING_QUEUE { onSegment } else { self.next_segment(onSegment, veh) };
        let teleporting = onSegment.is_null(); // is the vehicle currently teleporting?
        // @note reason is only evaluated if toSegment == nullptr
        let nextEntry = self.change_segment(veh, leaveTime, toSegment, MSMoveReminder_Notification::NOTIFICATION_ARRIVED, teleporting);
        if nextEntry == leaveTime {
            return;
        }
        let r1 = MSGlobals::time_to_gridlock() > 0 && veh.get_waiting_time(false) > MSGlobals::time_to_gridlock();
        let r3 = MSGlobals::time_to_teleport_disconnected() >= 0 && veh.get_waiting_time(false) > MSGlobals::time_to_teleport_disconnected();
        if !veh.is_stopped() && (r1 || r3) {
            let disconnected = MSGlobals::time_to_teleport_disconnected() >= 0
                && !veh.succ_edge(1).is_null()
                && veh.get_edge().allowed_lanes(&veh.succ_edge(1), veh.get_v_class(), false).is_null();
            if (r1 && !disconnected) || (r3 && disconnected) {
                self.teleport_vehicle(veh, toSegment, disconnected);
                return;
            }
        }
        if veh.get_block_time() == SUMOTIME_MAX && (!veh.is_stopped() || (!veh.is_stopped_triggered() && veh.is_stopped_parking())) {
            veh.m().set_block_time(leaveTime);
        }
        if nextEntry == SUMOTIME_MAX {
            // all usable queues on the next segment are full
            let mut newEventTime = std::cmp::max(std::cmp::max(toSegment.get_event_time() + 1, leaveTime + 1), leaveTime + self.myFullRecheckInterval);
            if MSGlobals::time_to_gridlock() > 0 {
                // if teleporting is enabled, make sure we look at the vehicle when the gridlock-time is up
                let recheck = if MSGlobals::time_to_teleport_disconnected() >= 0 { std::cmp::min(MSGlobals::time_to_gridlock(), MSGlobals::time_to_teleport_disconnected()) } else { MSGlobals::time_to_gridlock() };
                newEventTime = std::cmp::max(std::cmp::min(newEventTime, veh.get_block_time() + recheck + 1), leaveTime + 1);
            }
            veh.m().set_event_time(newEventTime, true);
        } else {
            // receiving segment has recently received another vehicle or the junction is blocked
            veh.m().set_event_time(nextEntry, true);
        }
        let link = if teleporting { Ptr::null() } else { onSegment.get_link(veh, false) };
        self.add_leader_car(veh, link, false);
    }

    /// C++: `MESegment * MELoop::nextSegment(MESegment *, MEVehicle *)` [private]
    /// Retrieve next segment
    // MESegment*
    // MELoop::nextSegment(MESegment* s, MEVehicle* v) {
    //     if (s != nullptr) { // vehicle is not teleporting
    //         MESegment* next = s->getNextSegment();
    //         if (next != nullptr) {
    //             // ok, the street continues
    //             return next;
    //         }
    //     }
    //     // we have to check the next edge in the vehicle's route
    //     const MSEdge* nextEdge = v->succEdge(1);
    //     if (nextEdge == nullptr) {
    //         // end of route
    //         return nullptr;
    //     }
    //     if (MSGlobals::gUsingInternalLanes && s != nullptr && s->getEdge().isNormal()) {
    //         const MSEdge* internal = s->getEdge().getInternalFollowingEdge(nextEdge, v->getVClass());
    //         if (internal) {
    //             nextEdge = internal;
    //         }
    //     }
    //     return myEdges2FirstSegments[nextEdge->getNumericalID()];
    // }
    #[allow(unused_variables)]
    pub fn next_segment(&mut self, s: Ptr<MESegment>, v: Ptr<MEVehicle>) -> Ptr<MESegment> {
        if !s.is_null() { // vehicle is not teleporting
            let next = s.get_next_segment();
            if !next.is_null() {
                // ok, the street continues
                return next;
            }
        }
        // we have to check the next edge in the vehicle's route
        let mut nextEdge = v.succ_edge(1);
        if nextEdge.is_null() {
            // end of route
            return Ptr::null();
        }
        if MSGlobals::using_internal_lanes() && !s.is_null() && s.get_edge().is_normal() {
            let internal = s.get_edge().get_internal_following_edge(nextEdge, v.get_v_class());
            if !internal.is_null() {
                nextEdge = internal;
            }
        }
        self.myEdges2FirstSegments[nextEdge.get_numerical_id() as usize]
    }

    /// C++: `void MELoop::teleportVehicle(MEVehicle *, MESegment *const, bool)` [private]
    /// teleports a vehicle or continues a teleport
    // void
    // MELoop::teleportVehicle(MEVehicle* veh, MESegment* const toSegment, bool disconnected) {
    //     const SUMOTime leaveTime = veh->getEventTime();
    //     MESegment* const onSegment = veh->getSegment();
    //     if (MSGlobals::gRemoveGridlocked) {
    //         WRITE_WARNINGF(TL("Teleporting vehicle '%'; waited too long, from edge '%':%, time=%."),
    //                        veh->getID(), onSegment->getEdge().getID(), onSegment->getIndex(),
    //                        time2string(leaveTime));
    //         MSNet::getInstance()->getVehicleControl().registerTeleportJam();
    //         int qIdx = 0;
    //         onSegment->send(veh, nullptr, qIdx, leaveTime, MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED);
    //         veh->setSegment(nullptr);
    //         MSNet::getInstance()->getVehicleControl().scheduleVehicleRemoval(veh);
    //         return;
    //     }
    //     const bool teleporting = (onSegment == nullptr); // is the vehicle already teleporting?
    //     // try to find a place on the current edge
    //     MESegment* teleSegment = disconnected ? toSegment : toSegment->getNextSegment();
    //     while (teleSegment != nullptr && changeSegment(veh, leaveTime, teleSegment, MSMoveReminder::NOTIFICATION_TELEPORT, true) != leaveTime) {
    //         // @caution the time to get to the next segment here is ignored XXX
    //         teleSegment = teleSegment->getNextSegment();
    //     }
    //     if (teleSegment != nullptr) {
    //         if (!teleporting) {
    //             // we managed to teleport in a single jump
    //             const std::string reason = disconnected ? " (disconnected)" : "";
    //             WRITE_WARNINGF(TL("Teleporting vehicle '%'; waited too long%, from edge '%':% to edge '%':%, time=%."),
    //                            veh->getID(), reason, onSegment->getEdge().getID(), onSegment->getIndex(),
    //                            teleSegment->getEdge().getID(), teleSegment->getIndex(), time2string(leaveTime));
    //             MSNet::getInstance()->getVehicleControl().registerTeleportJam();
    //         }
    //     } else {
    //         // teleport across the current edge and try insertion later
    //         if (!teleporting) {
    //             int qIdx = 0;
    //             // announce start of multi-step teleport, arrival will be announced in changeSegment()
    //             WRITE_WARNINGF(TL("Teleporting vehicle '%'; waited too long, from edge '%':%, time=%."),
    //                            veh->getID(), onSegment->getEdge().getID(), onSegment->getIndex(), time2string(leaveTime));
    //             MSNet::getInstance()->getVehicleControl().registerTeleportJam();
    //             // remove from current segment
    //             onSegment->send(veh, nullptr, qIdx, leaveTime, MSMoveReminder::NOTIFICATION_TELEPORT);
    //             // mark veh as teleporting
    //             veh->setSegment(nullptr);
    //         } else {
    //             veh->updateDetectors(veh->getLastEntryTime(), leaveTime, true, MSMoveReminder::NOTIFICATION_TELEPORT);
    //         }
    //         // @caution microsim uses current travel time teleport duration
    //         const SUMOTime teleArrival = leaveTime + TIME2STEPS(veh->getEdge()->getLength() / MAX2(veh->getEdge()->getSpeedLimit(), NUMERICAL_EPS));
    //         const bool atDest = veh->moveRoutePointer();
    //         if (atDest) {
    //             // teleporting to end of route
    //             changeSegment(veh, teleArrival, nullptr, MSMoveReminder::NOTIFICATION_TELEPORT_ARRIVED, true);
    //         } else {
    //             veh->setEventTime(teleArrival);
    //             addLeaderCar(veh, nullptr);
    //             // teleporting vehicles must react to rerouters
    //             getSegmentForEdge(*veh->getEdge())->addReminders(veh);
    //             veh->activateReminders(MSMoveReminder::NOTIFICATION_JUNCTION);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn teleport_vehicle(&mut self, veh: Ptr<MEVehicle>, toSegment: Ptr<MESegment>, disconnected: bool) {
        let leaveTime = veh.get_event_time();
        let onSegment = veh.get_segment();
        if MSGlobals::remove_gridlocked() {
            utils::common::msg_handler::write_warning(&utils::tlf!("Teleporting vehicle '%'; waited too long, from edge '%':%, time=%.",
                veh.get_id(), onSegment.get_edge().get_id(), onSegment.get_index(),
                utils::common::time::time2string(leaveTime)));
            MSNet::get_instance().m().get_vehicle_control().register_teleport_jam();
            let qIdx = 0;
            onSegment.m().send(veh, Ptr::null(), qIdx, leaveTime, MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED);
            veh.m().set_segment(Ptr::null(), 0);
            MSNet::get_instance().m().get_vehicle_control().schedule_vehicle_removal(veh.cast::<SUMOVehicle>(), false);
            return;
        }
        let teleporting = onSegment.is_null(); // is the vehicle already teleporting?
        // try to find a place on the current edge
        let mut teleSegment = if disconnected { toSegment } else { toSegment.get_next_segment() };
        while !teleSegment.is_null() && self.change_segment(veh, leaveTime, teleSegment, MSMoveReminder_Notification::NOTIFICATION_TELEPORT, true) != leaveTime {
            // @caution the time to get to the next segment here is ignored XXX
            teleSegment = teleSegment.get_next_segment();
        }
        if !teleSegment.is_null() {
            if !teleporting {
                // we managed to teleport in a single jump
                let reason = if disconnected { " (disconnected)" } else { "" };
                utils::common::msg_handler::write_warning(&utils::tlf!("Teleporting vehicle '%'; waited too long%, from edge '%':% to edge '%':%, time=%.",
                    veh.get_id(), reason, onSegment.get_edge().get_id(), onSegment.get_index(),
                    teleSegment.get_edge().get_id(), teleSegment.get_index(), utils::common::time::time2string(leaveTime)));
                MSNet::get_instance().m().get_vehicle_control().register_teleport_jam();
            }
        } else {
            // teleport across the current edge and try insertion later
            if !teleporting {
                let qIdx = 0;
                // announce start of multi-step teleport, arrival will be announced in changeSegment()
                utils::common::msg_handler::write_warning(&utils::tlf!("Teleporting vehicle '%'; waited too long, from edge '%':%, time=%.",
                    veh.get_id(), onSegment.get_edge().get_id(), onSegment.get_index(), utils::common::time::time2string(leaveTime)));
                MSNet::get_instance().m().get_vehicle_control().register_teleport_jam();
                // remove from current segment
                onSegment.m().send(veh, Ptr::null(), qIdx, leaveTime, MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
                // mark veh as teleporting
                veh.m().set_segment(Ptr::null(), 0);
            } else {
                veh.m().update_detectors(veh.get_last_entry_time(), leaveTime, true, MSMoveReminder_Notification::NOTIFICATION_TELEPORT);
            }
            // @caution microsim uses current travel time teleport duration
            let speedLimit = veh.get_edge().get_speed_limit();
            let denom = if speedLimit > utils::geom::NUMERICAL_EPS { speedLimit } else { utils::geom::NUMERICAL_EPS };
            let teleArrival = leaveTime + utils::common::time::time2steps(veh.get_edge().get_length() / denom);
            let atDest = veh.m().move_route_pointer();
            if atDest {
                // teleporting to end of route
                self.change_segment(veh, teleArrival, Ptr::null(), MSMoveReminder_Notification::NOTIFICATION_TELEPORT_ARRIVED, true);
            } else {
                veh.m().set_event_time(teleArrival, true);
                self.add_leader_car(veh, Ptr::null(), false);
                // teleporting vehicles must react to rerouters
                let edge = veh.get_edge();
                self.get_segment_for_edge(&edge, 0.).add_reminders(veh);
                veh.m().activate_reminders(MSMoveReminder_Notification::NOTIFICATION_JUNCTION, Ptr::null());
            }
        }
    }

    /// C++: `bool MELoop::wasInvalidated(const LeaderEvent &)` [private]
    /// check whether the current car was invalidated and remove it from myInvalidatedLeaderCars
    // bool
    // MELoop::wasInvalidated(const LeaderEvent& e) {
    //     if (e.time == myInvalidatedLeaderCars.top().time) {
    //         // we need to check whether any of the events with the same time
    //         // is our current vehicle because the order in which
    //         // removeLeaderCar was called is unrelated to the order of addLeaderCar
    //         std::vector<LeaderEvent> tmp;
    //         bool curInvalid = false;
    //         while (!myInvalidatedLeaderCars.empty() && e.time == myInvalidatedLeaderCars.top().time) {
    //             if (e == myInvalidatedLeaderCars.top()) {
    //                 myInvalidatedLeaderCars.pop();
    //                 curInvalid = true;
    //                 break;
    //             } else {
    //                 tmp.push_back(myInvalidatedLeaderCars.top());
    //                 myInvalidatedLeaderCars.pop();
    //             }
    //         }
    //         for (LeaderEvent& le : tmp) {
    //             myInvalidatedLeaderCars.push(le);
    //         }
    //         return curInvalid;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn was_invalidated(&mut self, e: &MELoop_LeaderEvent) -> bool {
        if e.time == self.myInvalidatedLeaderCars.peek().unwrap().time {
            // we need to check whether any of the events with the same time
            // is our current vehicle because the order in which
            // removeLeaderCar was called is unrelated to the order of addLeaderCar
            let mut tmp: Vec<MELoop_LeaderEvent> = Vec::new();
            let mut curInvalid = false;
            while !self.myInvalidatedLeaderCars.is_empty() && e.time == self.myInvalidatedLeaderCars.peek().unwrap().time {
                let top = self.myInvalidatedLeaderCars.pop().unwrap();
                if e.eq_op(&top) {
                    curInvalid = true;
                    break;
                } else {
                    tmp.push(top);
                }
            }
            for le in tmp {
                self.myInvalidatedLeaderCars.push(le);
            }
            return curInvalid;
        }
        false
    }

    /// C++: `void MELoop::MELoop(const MELoop &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MELoop) -> Self {
        panic!("MELoop copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `MELoop & MELoop::operator=(const MELoop &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MELoop) -> &mut MELoop {
        panic!("MELoop assignment operator is private and undefined in C++; it cannot be called")
    }

}

