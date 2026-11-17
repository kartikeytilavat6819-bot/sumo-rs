//! Port of microsim/output/MSInductLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cmp::Reverse;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::string_utils;
use utils::common::time::{delta_t, speed2dist, steps2time, time2steps, ts, SUMOTime};
use utils::geom::NUMERICAL_EPS;

/// C++ `#define HAS_NOT_LEFT_DETECTOR -1` (MSInductLoop.cpp)
const HAS_NOT_LEFT_DETECTOR: f64 = -1.;
// shortcut: MSPModel statics are not ported yet; values copied from MSPModel.cpp, upgrade when msp_model.rs exports them
const MSPMODEL_FORWARD: i32 = 1;
const MSPMODEL_BACKWARD: i32 = -1;

// shortcut: MSGlobals flags are not ported (ms_globals.rs is empty); these return the C++ defaults, upgrade when MSGlobals is filled
fn g_use_meso_sim() -> bool {
    false
}
fn g_num_sim_threads() -> i32 {
    1
}

/// C++ `SIMSTEP` macro
fn sim_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}
/// C++ `SIMTIME` macro
fn sim_time() -> f64 {
    steps2time(sim_step())
}
/// C++ `MIN2` / `MAX2` on doubles (same operand order)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
/// C++ `TimeQueue::top()`
fn tq_top(tq: &MSInductLoop_TimeQueue) -> SUMOTime {
    tq.peek().unwrap().0
}

/// C++ typedef `std::priority_queue<SUMOTime, std::vector<SUMOTime>, std::greater<SUMOTime>> MSInductLoop::TimeQueue`
pub type MSInductLoop_TimeQueue = BinaryHeap<Reverse<SUMOTime>>;

/// C++ typedef `std::deque<VehicleData> MSInductLoop::VehicleDataCont`
pub type MSInductLoop_VehicleDataCont = VecDeque<MSInductLoop_VehicleData>;

fn ms_induct_loop_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSInductLoop>().m().notify_enter(veh, reason, enteredLane)
}
fn ms_induct_loop_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    Ptr::from_mut(mr).cast::<MSInductLoop>().m().notify_move(veh, oldPos, newPos, newSpeed)
}
fn ms_induct_loop_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSInductLoop>().m().notify_leave(veh, lastPos, reason, enteredLane)
}
static MS_INDUCT_LOOP_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_induct_loop_notify_enter,
    notify_move: ms_induct_loop_notify_move,
    notify_leave: ms_induct_loop_notify_leave,
};

/// An unextended detector measuring at a fixed position on a fixed lane.
/// C++ class `MSInductLoop` (microsim/output/MSInductLoop.h)
#[repr(C)]
pub struct MSInductLoop {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// C++ base class `MSDetectorFileOutput`
    pub base2: MSDetectorFileOutput,
    /// detecto name
    pub myName: String,
    /// Detector's position on lane [m]
    pub myPosition: f64,
    /// Detector's end position (defaults to myPosition)
    pub myEndPosition: f64,
    /// whether internals need to be guarded against concurrent access (GUI or multi threading)
    pub myNeedLock: bool,
    /// Leave-time of the last vehicle detected [s]
    pub myLastLeaveTime: f64,
    /// extrapolated leave times for meso vehicle
    pub myNextMesoLeaveTimes: MSInductLoop_TimeQueue,
    pub myLastMesoLeaveTimes: MSInductLoop_TimeQueue,
    /// overrides the time since last detection
    pub myOverrideTime: f64,
    /// records the time at which overrideTimeSinceDetection was activated
    pub myOverrideEntryTime: f64,
    /// The number of entered vehicles
    pub myEnteredVehicleNumber: i32,
    /// Data of vehicles that have completely passed the detector
    pub myVehicleDataCont: MSInductLoop_VehicleDataCont,
    /// Data of vehicles that have completely passed the detector in the last time interval
    pub myLastVehicleDataCont: MSInductLoop_VehicleDataCont,
    /// Data for vehicles that have entered the detector (vehicle -> enter time)
    pub myVehiclesOnDet: BTreeMap<Ptr<SUMOTrafficObject>, f64>,
    pub myLastIntervalEnd: SUMOTime,
    pub myLastIntervalBegin: SUMOTime,
    /// current segment (for actuated tls in meso)
    pub mySegment: Ptr<MESegment>,
    /// position relative to the current segment
    pub mySegmentPos: f64,
}

impl std::ops::Deref for MSInductLoop {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSInductLoop {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSInductLoop {
    /// C++: `void MSInductLoop::MSInductLoop(const std::string &, MSLane *const, double, double, std::string, const std::string &, const std::string &, int, const bool)`
    /// Constructor.
    // MSInductLoop::MSInductLoop(const std::string& id, MSLane* const lane,
    //                            double positionInMeters,
    //                            double length, std::string name,
    //                            const std::string& vTypes,
    //                            const std::string& nextEdges,
    //                            int detectPersons,
    //                            const bool needLocking) :
    //     MSMoveReminder(id, lane),
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myName(name),
    //     myPosition(positionInMeters),
    //     myEndPosition(myPosition + length),
    //     myNeedLock(needLocking || MSGlobals::gNumSimThreads > 1),
    //     // initialize in a way which doesn't impact actualted traffic lights at simulation start (yet doesn't look ugly in the outputs)
    //     myLastLeaveTime(-3600),
    //     myOverrideTime(-1),
    //     myOverrideEntryTime(-1),
    //     myVehicleDataCont(),
    //     myVehiclesOnDet(),
    //     myLastIntervalEnd(-1),
    //     mySegment(nullptr) {
    //     assert(length >= 0);
    //     assert(myPosition >= 0 && myEndPosition <= myLane->getLength());
    //     reset();
    //     if (MSGlobals::gUseMesoSim) {
    //         // used by actuated tls
    //         mySegment = MSGlobals::gMesoNet->getSegmentForEdge(lane->getEdge(), myPosition);
    //         mySegment->addDetector(this);
    //         mySegmentPos = myPosition - mySegment->getIndex() * mySegment->getLength();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, lane: Ptr<MSLane>, positionInMeters: f64, length: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, needLocking: bool) -> Box<Self> {
        debug_assert!(length >= 0.);
        debug_assert!(positionInMeters >= 0. && positionInMeters + length <= lane.get_length());
        // shortcut: gUseMesoSim branch (MESegment lookup and addDetector(this)) not ported; meso is off while g_use_meso_sim() is false
        let mut s = Box::new(MSInductLoop {
            base: MSMoveReminder {
                vt: &MS_INDUCT_LOOP_MSMOVE_REMINDER_VT,
                ..MSMoveReminder::new(id, lane, true)
            },
            // shortcut: C++ constructor throws ProcessError on an unknown vehicle type or edge; new() returns Self, so it panics instead, upgrade when constructors return Result
            base2: MSDetectorFileOutput::new(id, vTypes, nextEdges, detectPersons).unwrap(),
            myName: name,
            myPosition: positionInMeters,
            myEndPosition: positionInMeters + length,
            myNeedLock: needLocking || g_num_sim_threads() > 1,
            // initialize in a way which doesn't impact actualted traffic lights at simulation start (yet doesn't look ugly in the outputs)
            myLastLeaveTime: -3600.,
            myNextMesoLeaveTimes: MSInductLoop_TimeQueue::default(),
            myLastMesoLeaveTimes: MSInductLoop_TimeQueue::default(),
            myOverrideTime: -1.,
            myOverrideEntryTime: -1.,
            myEnteredVehicleNumber: 0,
            myVehicleDataCont: VecDeque::new(),
            myLastVehicleDataCont: VecDeque::new(),
            myVehiclesOnDet: BTreeMap::new(),
            myLastIntervalBegin: 0,
            myLastIntervalEnd: -1,
            mySegment: Ptr::null(),
            mySegmentPos: 0.,
        });
        // C++ MSMoveReminder base constructor (doAdd = true): myLane->addMoveReminder(this)
        lane.m().add_move_reminder(Ptr::from_mut(&mut s.base), true);
        s.reset();
        s
    }

    /// C++: `void MSInductLoop::~MSInductLoop() noexcept` [virtual]
    /// Destructor
    // MSInductLoop::~MSInductLoop() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; the base-class destructors do the cleanup
    }

    /// C++: `void MSInductLoop::reset()` [virtual]
    /// Resets all generated values to allow computation of next interval
    // void
    // MSInductLoop::reset() {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //     myEnteredVehicleNumber = 0;
    //     myLastVehicleDataCont = myVehicleDataCont;
    //     myVehicleDataCont.clear();
    //     myLastIntervalBegin = myLastIntervalEnd;
    //     myLastIntervalEnd = SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myEnteredVehicleNumber = 0;
        self.myLastVehicleDataCont = self.myVehicleDataCont.clone();
        self.myVehicleDataCont.clear();
        self.myLastIntervalBegin = self.myLastIntervalEnd;
        self.myLastIntervalEnd = sim_step();
    }

    /// C++: `std::string MSInductLoop::getName() const` [const]
    /// get name
    // std::string getName() const {
    //         return myName;
    //     }
    #[allow(unused_variables)]
    pub fn get_name(&self) -> String {
        self.myName.clone()
    }

    /// C++: `double MSInductLoop::getPosition() const` [const]
    /// Returns the position of the detector on the lane
    // double getPosition() const {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        self.myPosition
    }

    /// C++: `double MSInductLoop::getEndPosition() const` [const]
    /// Returns the end position of the detector on the lane
    // double getEndPosition() const {
    //         return myEndPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_end_position(&self) -> f64 {
        self.myEndPosition
    }

    /// C++: `bool MSInductLoop::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Checks whether the reminder is activated by a vehicle entering the lane
    // bool
    // MSInductLoop::notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* /* enteredLane */) {
    //     // vehicles must be kept if the "inductionloop" wants to detect passeengers
    //     if (!vehicleApplies(veh) && (veh.isPerson() || myDetectPersons <= (int)PersonMode::WALK)) {
    //         return false;
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         MEVehicle* mesoveh = dynamic_cast<MEVehicle*>(&veh);
    //         assert(mesoveh != nullptr);
    //         const MESegment* seg = mesoveh->getSegment();
    //         if (seg != nullptr && (seg->numQueues() == 1 || mesoveh->getQueIndex() == myLane->getIndex())) {
    //             const SUMOTime onSegTime = mesoveh->getEventTime() - mesoveh->getLastEntryTime();
    //             // extrapolate movement
    //             const SUMOTime exLeaveTime = mesoveh->getLastEntryTime() + (SUMOTime)((double)onSegTime * mySegmentPos / seg->getLength());
    //             //std::cout << SIMTIME << " det=" << getID() << " veh=" << veh.getID() << " entry=" << STEPS2TIME(mesoveh->getLastEntryTime()) << " et=" << STEPS2TIME(mesoveh->getEventTime()) << " exLeaveTime=" << STEPS2TIME(exLeaveTime) << "\n";
    //             myNextMesoLeaveTimes.push(exLeaveTime);
    //         }
    //         return true;
    //     }
    //     if (reason != NOTIFICATION_JUNCTION) { // the junction case is handled in notifyMove
    //         if (veh.getBackPositionOnLane(myLane) >= myPosition) {
    //             return false;
    //         }
    //         if (veh.getPositionOnLane() >= myPosition) {
    // #ifdef HAVE_FOX
    //             ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //             myVehiclesOnDet[&veh] = SIMTIME;
    //             myEnteredVehicleNumber++;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        // vehicles must be kept if the "inductionloop" wants to detect passeengers
        if !self.base2.vehicle_applies(veh) && (veh.is_person() || self.base2.myDetectPersons <= PersonMode::WALK as i32) {
            return false;
        }
        if g_use_meso_sim() {
            // shortcut: mesoscopic branch (dynamic_cast<MEVehicle*>, extrapolated segment leave times) not ported; unreachable while g_use_meso_sim() is false
            return true;
        }
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            // the junction case is handled in notifyMove
            if veh.get_back_position_on_lane(self.myLane) >= self.myPosition {
                return false;
            }
            if veh.get_position_on_lane() >= self.myPosition {
                self.myVehiclesOnDet.insert(Ptr::from_mut(veh), sim_time());
                self.myEnteredVehicleNumber += 1;
            }
        }
        true
    }

    /// C++: `bool MSInductLoop::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the vehicle shall be counted and/or shall still touch this MSMoveReminder
    // bool
    // MSInductLoop::notifyMove(SUMOTrafficObject& veh, double oldPos,
    //                          double newPos, double newSpeed) {
    //     if (newPos < myPosition) {
    //         // detector not reached yet
    //         return true;
    //     }
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         bool keep = false;
    //         MSBaseVehicle& v = dynamic_cast<MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             keep = notifyMove(*p, oldPos, newPos, newSpeed);
    //         }
    //         return keep;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //     const double oldSpeed = veh.getPreviousSpeed();
    //     if (newPos >= myPosition && oldPos < myPosition) {
    //         // entered the detector by move
    //         const double timeBeforeEnter = MSCFModel::passingTime(oldPos, myPosition, newPos, oldSpeed, newSpeed);
    //         myVehiclesOnDet[&veh] = SIMTIME + timeBeforeEnter;
    //         myEnteredVehicleNumber++;
    // #ifdef DEBUG_E1_NOTIFY_MOVE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " det=" << getID() << " enteredVeh=" << veh.getID() << "\n";
    //         }
    // #endif
    //     }
    //     double oldBackPos = oldPos - veh.getVehicleType().getLength();
    //     double newBackPos = newPos - veh.getVehicleType().getLength();
    //     if (newBackPos > myEndPosition) {
    //         // vehicle passed the detector (it may have changed onto this lane somewhere past the detector)
    //         // assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed > 0 || myVehiclesOnDet.find(&veh) == myVehiclesOnDet.end());
    //         // assertion is invalid in case of teleportation
    //         if (oldBackPos <= myEndPosition) {
    //             const std::map<SUMOTrafficObject*, double>::iterator it = myVehiclesOnDet.find(&veh);
    //             if (it != myVehiclesOnDet.end()) {
    //                 const double entryTime = it->second;
    //                 const double leaveTime = SIMTIME + MSCFModel::passingTime(oldBackPos, myEndPosition, newBackPos, oldSpeed, newSpeed);
    //                 myVehiclesOnDet.erase(it);
    //                 assert(entryTime <= leaveTime);
    //                 myVehicleDataCont.push_back(VehicleData(veh, entryTime, leaveTime, false, myEndPosition - myPosition));
    //                 myLastLeaveTime = leaveTime;
    // #ifdef DEBUG_E1_NOTIFY_MOVE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " det=" << getID() << " leftVeh=" << veh.getID() << " oldBackPos=" << oldBackPos << " newBackPos=" << newBackPos << "\n";
    //                 }
    // #endif
    //             } else {
    // #ifdef DEBUG_E1_NOTIFY_MOVE
    //                 if (DEBUG_COND) {
    //                     std::cout << SIMTIME << " det=" << getID() << " leftVeh=" << veh.getID() << " oldBackPos=" << oldBackPos << " newBackPos=" << newBackPos << " (notFound)\n";
    //                 }
    // #endif
    //             }
    //         } else {
    //             // vehicle is already beyond the detector...
    //             // This can happen even if it is still registered in myVehiclesOnDet, e.g., after teleport.
    //             myVehiclesOnDet.erase(&veh);
    // #ifdef DEBUG_E1_NOTIFY_MOVE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " det=" << getID() << " leftVeh=" << veh.getID() << " oldBackPos=" << oldBackPos << " newBackPos=" << newBackPos << " (unusual)\n";
    //             }
    // #endif
    //         }
    //         return false;
    //     }
    //     // vehicle stays on the detector
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if newPos < self.myPosition {
            // detector not reached yet
            return true;
        }
        if self.base2.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            // shortcut: dynamic_cast<MSBaseVehicle&>(veh).getPersons() needs a runtime type check on SUMOTrafficObject, which does not exist yet; the persons' keep flags are not computed and the vehicle is dropped
            return false;
        }
        let key = Ptr::from_mut(veh);
        let oldSpeed = veh.get_previous_speed();
        if newPos >= self.myPosition && oldPos < self.myPosition {
            // entered the detector by move
            let timeBeforeEnter = MSCFModel::passing_time(oldPos, self.myPosition, newPos, oldSpeed, newSpeed);
            self.myVehiclesOnDet.insert(key, sim_time() + timeBeforeEnter);
            self.myEnteredVehicleNumber += 1;
        }
        let oldBackPos = oldPos - veh.get_vehicle_type().get_length();
        let newBackPos = newPos - veh.get_vehicle_type().get_length();
        if newBackPos > self.myEndPosition {
            // vehicle passed the detector (it may have changed onto this lane somewhere past the detector)
            if oldBackPos <= self.myEndPosition {
                if let Some(entryTime) = self.myVehiclesOnDet.get(&key).copied() {
                    let leaveTime = sim_time() + MSCFModel::passing_time(oldBackPos, self.myEndPosition, newBackPos, oldSpeed, newSpeed);
                    self.myVehiclesOnDet.remove(&key);
                    debug_assert!(entryTime <= leaveTime);
                    self.myVehicleDataCont.push_back(MSInductLoop_VehicleData::new(veh, entryTime, leaveTime, false, self.myEndPosition - self.myPosition));
                    self.myLastLeaveTime = leaveTime;
                }
            } else {
                // vehicle is already beyond the detector...
                // This can happen even if it is still registered in myVehiclesOnDet, e.g., after teleport.
                self.myVehiclesOnDet.remove(&key);
            }
            return false;
        }
        // vehicle stays on the detector
        true
    }

    /// C++: `bool MSInductLoop::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Dismisses the vehicle if it is on the detector due to a lane change
    // bool
    // MSInductLoop::notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (veh.isPerson() && myDetectPersons != (int)PersonMode::NONE) {
    //         const int lastDir = lastPos < 0 ? MSPModel::BACKWARD : MSPModel::FORWARD;
    //         notifyMovePerson(dynamic_cast<MSTransportable*>(&veh), lastDir, lastPos);
    //     }
    //     if (MSGlobals::gUseMesoSim && reason <= MSMoveReminder::NOTIFICATION_SEGMENT && vehicleApplies(veh)) {
    //         MEVehicle* mesoveh = dynamic_cast<MEVehicle*>(&veh);
    //         assert(mesoveh != nullptr);
    //         const MESegment* seg = mesoveh->getSegment();
    //         if (seg != nullptr && (seg->numQueues() == 1 || mesoveh->getQueIndex() == myLane->getIndex())) {
    //             // interpolate movement + 1 extra second
    //             // We can only interpolate the passing time at the detector after the vehicle leaves the segment. Consequently our information is always out of date
    //             // (by the time the current vehicle left, the follower vehicle should already have reached the detector so the estimated time gap to the last detection is always too large)
    //             // the prior implementation used seg->getBlockTime() which included the headway which somewhat worked around this deficit by accident
    //             const SUMOTime inLeaveTime = mesoveh->getEventTime() - TIME2STEPS((seg->getLength() - mySegmentPos - mesoveh->getLength()) / myLane->getSpeedLimit());
    //             //std::cout << SIMTIME << " det=" << getID() << " veh=" << veh.getID() << " et=" << STEPS2TIME(mesoveh->getEventTime()) << " inLeaveTime=" << STEPS2TIME(inLeaveTime) << "\n";
    //             auto q = seg->getQueue(mesoveh->getQueIndex());
    //             if (q.size() > 1 && vehicleApplies(*q[q.size() - 2])) {
    //                 myLastMesoLeaveTimes.push(inLeaveTime + TIME2STEPS(seg->getLastHeadwaySeconds()));
    //                 //std::cout << "    inLeaveTime2=" << (STEPS2TIME(inLeaveTime) + seg->getLastHeadwaySeconds()) << "\n";
    //             }
    //             myLastMesoLeaveTimes.push(inLeaveTime);
    //             myEnteredVehicleNumber++;
    //         }
    //         return false;
    //     }
    //     if (reason != MSMoveReminder::NOTIFICATION_JUNCTION || (veh.isPerson() && myDetectPersons != (int)PersonMode::NONE)) {
    // #ifdef HAVE_FOX
    //         ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //         const std::map<SUMOTrafficObject*, double>::iterator it = myVehiclesOnDet.find(&veh);
    //         if (it != myVehiclesOnDet.end()) {
    //             const double entryTime = it->second;
    //             const double leaveTime = SIMTIME + TS;
    //             myVehiclesOnDet.erase(it);
    //             myVehicleDataCont.push_back(VehicleData(veh, entryTime, leaveTime, true));
    //             myLastLeaveTime = leaveTime;
    //         }
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if veh.is_person() && self.base2.myDetectPersons != PersonMode::NONE as i32 {
            let lastDir = if lastPos < 0. { MSPMODEL_BACKWARD } else { MSPMODEL_FORWARD };
            // dynamic_cast<MSTransportable*>(&veh): a person is looked up by its ID in the person control
            let p = MSNet::get_instance().m().get_person_control().get(veh.get_id());
            self.notify_move_person(p, lastDir, lastPos);
        }
        if g_use_meso_sim() && reason <= MSMoveReminder_Notification::NOTIFICATION_SEGMENT && self.base2.vehicle_applies(veh) {
            // shortcut: mesoscopic branch (dynamic_cast<MEVehicle*>, interpolated leave times) not ported; unreachable while g_use_meso_sim() is false
            return false;
        }
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION || (veh.is_person() && self.base2.myDetectPersons != PersonMode::NONE as i32) {
            let key = Ptr::from_mut(veh);
            if let Some(entryTime) = self.myVehiclesOnDet.get(&key).copied() {
                let leaveTime = sim_time() + ts();
                self.myVehiclesOnDet.remove(&key);
                self.myVehicleDataCont.push_back(MSInductLoop_VehicleData::new(veh, entryTime, leaveTime, true, 0.));
                self.myLastLeaveTime = leaveTime;
            }
            return false;
        }
        true
    }

    /// C++: `double MSInductLoop::getSpeed(const int) const` [const]
    /// Returns the speed of the vehicle on the detector
    // double
    // MSInductLoop::getSpeed(const int offset) const {
    //     const std::vector<VehicleData>& d = collectVehiclesOnDet(SIMSTEP - offset);
    //     return d.empty() ? -1. : std::accumulate(d.begin(), d.end(), 0.0, speedSum) / (double) d.size();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self, offset: i32) -> f64 {
        let d = self.collect_vehicles_on_det(sim_step() - offset as SUMOTime, false, false, false, false);
        if d.is_empty() { -1. } else { d.iter().fold(0., MSInductLoop::speed_sum) / d.len() as f64 }
    }

    /// C++: `double MSInductLoop::getVehicleLength(const int) const` [const]
    /// Returns the length of the vehicle on the detector
    // double
    // MSInductLoop::getVehicleLength(const int offset) const {
    //     const std::vector<VehicleData>& d = collectVehiclesOnDet(SIMSTEP - offset);
    //     return d.empty() ? -1. : std::accumulate(d.begin(), d.end(), 0.0, lengthSum) / (double)d.size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_length(&self, offset: i32) -> f64 {
        let d = self.collect_vehicles_on_det(sim_step() - offset as SUMOTime, false, false, false, false);
        if d.is_empty() { -1. } else { d.iter().fold(0., MSInductLoop::length_sum) / d.len() as f64 }
    }

    /// C++: `double MSInductLoop::getOccupancy() const` [const]
    /// Returns the current occupancy
    // double
    // MSInductLoop::getOccupancy() const {
    //     if (myOverrideTime >= 0) {
    //         return myOverrideTime < TS ? (TS - myOverrideTime) / TS * 100 : 0;
    //     }
    //     const SUMOTime tbeg = SIMSTEP - DELTA_T;
    //     double occupancy = 0;
    //     const double csecond = SIMTIME;
    //     for (const VehicleData& i : collectVehiclesOnDet(tbeg, false, false, true)) {
    //         const double leaveTime = i.leaveTimeM == HAS_NOT_LEFT_DETECTOR ? csecond : MIN2(i.leaveTimeM, csecond);
    //         const double entryTime = MAX2(i.entryTimeM, STEPS2TIME(tbeg));
    //         occupancy += MIN2(leaveTime - entryTime, TS);
    //     }
    //     return occupancy / TS * 100.;
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy(&self) -> f64 {
        if self.myOverrideTime >= 0. {
            return if self.myOverrideTime < ts() { (ts() - self.myOverrideTime) / ts() * 100. } else { 0. };
        }
        let tbeg = sim_step() - delta_t();
        let mut occupancy = 0.;
        let csecond = sim_time();
        for i in self.collect_vehicles_on_det(tbeg, false, false, true, false) {
            let leaveTime = if i.leaveTimeM == HAS_NOT_LEFT_DETECTOR { csecond } else { min2(i.leaveTimeM, csecond) };
            let entryTime = max2(i.entryTimeM, steps2time(tbeg));
            occupancy += min2(leaveTime - entryTime, ts());
        }
        occupancy / ts() * 100.
    }

    /// C++: `double MSInductLoop::getEnteredNumber(const int) const` [const]
    /// Returns the number of vehicles that have passed the detector
    // double
    // MSInductLoop::getEnteredNumber(const int offset) const {
    //     if (myOverrideTime >= 0) {
    //         return myOverrideTime < TS ? 1 : 0;
    //     }
    //     return (double)collectVehiclesOnDet(SIMSTEP - offset, true, true).size();
    // }
    #[allow(unused_variables)]
    pub fn get_entered_number(&self, offset: i32) -> f64 {
        if self.myOverrideTime >= 0. {
            return if self.myOverrideTime < ts() { 1. } else { 0. };
        }
        self.collect_vehicles_on_det(sim_step() - offset as SUMOTime, true, true, false, false).len() as f64
    }

    /// C++: `std::vector<std::string> MSInductLoop::getVehicleIDs(const int) const` [const]
    /// Returns the ids of vehicles that have passed the detector
    // std::vector<std::string>
    // MSInductLoop::getVehicleIDs(const int offset) const {
    //     std::vector<std::string> ret;
    //     for (const VehicleData& i : collectVehiclesOnDet(SIMSTEP - offset, true, true)) {
    //         ret.push_back(i.idM);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_i_ds(&self, offset: i32) -> Vec<String> {
        self.collect_vehicles_on_det(sim_step() - offset as SUMOTime, true, true, false, false).into_iter().map(|i| i.idM).collect()
    }

    /// C++: `double MSInductLoop::getIntervalOccupancy(bool) const` [const]
    // double
    // MSInductLoop::getIntervalOccupancy(bool lastInterval) const {
    //     double occupancy = 0;
    //     const double csecond = lastInterval ? STEPS2TIME(myLastIntervalEnd) : SIMTIME;
    //     const double aggTime = csecond - STEPS2TIME(lastInterval ? myLastIntervalBegin : myLastIntervalEnd);
    //     if (aggTime == 0) {
    //         return 0;
    //     }
    //     for (const VehicleData& i : collectVehiclesOnDet(myLastIntervalEnd, false, false, true, lastInterval)) {
    //         const double leaveTime = i.leaveTimeM == HAS_NOT_LEFT_DETECTOR ? csecond : MIN2(i.leaveTimeM, csecond);
    //         const double entryTime = MAX2(i.entryTimeM, STEPS2TIME(lastInterval ? myLastIntervalBegin : myLastIntervalEnd));
    //         occupancy += MIN2(leaveTime - entryTime, aggTime);
    //     }
    //     return occupancy / aggTime * 100.;
    // }
    #[allow(unused_variables)]
    pub fn get_interval_occupancy(&self, lastInterval: bool) -> f64 {
        let mut occupancy = 0.;
        let csecond = if lastInterval { steps2time(self.myLastIntervalEnd) } else { sim_time() };
        let refTime = steps2time(if lastInterval { self.myLastIntervalBegin } else { self.myLastIntervalEnd });
        let aggTime = csecond - refTime;
        if aggTime == 0. {
            return 0.;
        }
        for i in self.collect_vehicles_on_det(self.myLastIntervalEnd, false, false, true, lastInterval) {
            let leaveTime = if i.leaveTimeM == HAS_NOT_LEFT_DETECTOR { csecond } else { min2(i.leaveTimeM, csecond) };
            let entryTime = max2(i.entryTimeM, refTime);
            occupancy += min2(leaveTime - entryTime, aggTime);
        }
        occupancy / aggTime * 100.
    }

    /// C++: `double MSInductLoop::getIntervalMeanSpeed(bool) const` [const]
    // double
    // MSInductLoop::getIntervalMeanSpeed(bool lastInterval) const {
    //     const std::vector<VehicleData>& d = collectVehiclesOnDet(myLastIntervalEnd, false, false, false, lastInterval);
    //     return d.empty() ? -1. : std::accumulate(d.begin(), d.end(), 0.0, speedSum) / (double) d.size();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_mean_speed(&self, lastInterval: bool) -> f64 {
        let d = self.collect_vehicles_on_det(self.myLastIntervalEnd, false, false, false, lastInterval);
        if d.is_empty() { -1. } else { d.iter().fold(0., MSInductLoop::speed_sum) / d.len() as f64 }
    }

    /// C++: `int MSInductLoop::getIntervalVehicleNumber(bool) const` [const]
    // int
    // MSInductLoop::getIntervalVehicleNumber(bool lastInterval) const {
    //     return (int)collectVehiclesOnDet(myLastIntervalEnd, false, false, false, lastInterval).size();
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_number(&self, lastInterval: bool) -> i32 {
        self.collect_vehicles_on_det(self.myLastIntervalEnd, false, false, false, lastInterval).len() as i32
    }

    /// C++: `std::vector<std::string> MSInductLoop::getIntervalVehicleIDs(bool) const` [const]
    // std::vector<std::string>
    // MSInductLoop::getIntervalVehicleIDs(bool lastInterval) const {
    //     std::vector<std::string> ret;
    //     for (const VehicleData& i : collectVehiclesOnDet(myLastIntervalEnd, false, false, false, lastInterval)) {
    //         ret.push_back(i.idM);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_interval_vehicle_i_ds(&self, lastInterval: bool) -> Vec<String> {
        self.collect_vehicles_on_det(self.myLastIntervalEnd, false, false, false, lastInterval).into_iter().map(|i| i.idM).collect()
    }

    /// C++: `double MSInductLoop::getTimeSinceLastDetection() const` [const]
    /// Returns the time since the last vehicle left the detector
    // double
    // MSInductLoop::getTimeSinceLastDetection() const {
    //     if (myOverrideTime >= 0) {
    //         return myOverrideTime;
    //     }
    //     if (myVehiclesOnDet.size() != 0) {
    //         // detector is occupied
    //         return 0;
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         //std::cout << SIMTIME << " det=" << getID() << " qBefore=" << myNextMesoLeaveTimes.size() << " last=" << STEPS2TIME(getLastDetectionTime()) << " qAfter=" << myNextMesoLeaveTimes.size() << " top=" << (myNextMesoLeaveTimes.empty() ? -1 : myNextMesoLeaveTimes.top()) << "\n";
    //         return SIMTIME - STEPS2TIME(getLastDetectionTime());
    //     }
    //     return SIMTIME - myLastLeaveTime;
    // }
    #[allow(unused_variables)]
    pub fn get_time_since_last_detection(&self) -> f64 {
        if self.myOverrideTime >= 0. {
            return self.myOverrideTime;
        }
        if !self.myVehiclesOnDet.is_empty() {
            // detector is occupied
            return 0.;
        }
        if g_use_meso_sim() {
            return sim_time() - steps2time(self.get_last_detection_time());
        }
        sim_time() - self.myLastLeaveTime
    }

    /// C++: `double MSInductLoop::getOccupancyTime() const` [const]
    /// Returns the time of continous occupation by the same vehicle in seconds or 0 if there is no vehicle on the detector
    // double
    // MSInductLoop::getOccupancyTime() const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //     if (myOverrideTime >= 0) {
    //         return SIMTIME - myOverrideEntryTime;
    //     }
    //     if (myVehiclesOnDet.size() == 0) {
    //         // detector is unoccupied
    //         return 0;
    //     } else {
    //         double minEntry = std::numeric_limits<double>::max();
    //         for (const auto& i : myVehiclesOnDet) {
    //             minEntry = MIN2(i.second, minEntry);
    //         }
    //         return SIMTIME - minEntry;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_occupancy_time(&self) -> f64 {
        if self.myOverrideTime >= 0. {
            return sim_time() - self.myOverrideEntryTime;
        }
        if self.myVehiclesOnDet.is_empty() {
            // detector is unoccupied
            return 0.;
        }
        let mut minEntry = f64::MAX;
        for &entry in self.myVehiclesOnDet.values() {
            minEntry = min2(entry, minEntry);
        }
        sim_time() - minEntry
    }

    /// C++: `double MSInductLoop::getArrivalDelay() const` [const]
    /// Returns the maximum stop arrival delay of public transport vehicles that are on the detector or passed the detector in the last step or -INVALID_DOUBLE
    // double
    // MSInductLoop::getArrivalDelay() const {
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, myNeedLock);
    // #endif
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     double result = -INVALID_DOUBLE;
    //     for (const auto& item : collectVehiclesOnDet(SIMSTEP - DELTA_T)) {
    //         SUMOVehicle* v = vc.getVehicle(item.idM);
    //         if (v != nullptr) {
    //             MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(v);
    //             double ad = veh->getStopArrivalDelay();
    //             if (ad != INVALID_DOUBLE) {
    //                 result = MAX2(result, ad);
    //             }
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_delay(&self) -> f64 {
        // shortcut: C++ looks each detected vehicle up in MSVehicleControl and dynamic_casts it to MSBaseVehicle to read getStopArrivalDelay(); no SUMOVehicle -> MSBaseVehicle downcast exists yet, so this returns -INVALID_DOUBLE. Upgrade when that downcast is ported
        -INVALID_DOUBLE
    }

    /// C++: `SUMOTime MSInductLoop::getLastDetectionTime() const` [const]
    /// return last time a vehicle was on the detector
    // SUMOTime
    // MSInductLoop::getLastDetectionTime() const {
    //     if (myOverrideTime >= 0) {
    //         return SIMSTEP - TIME2STEPS(myOverrideTime);
    //     }
    //     if (myVehiclesOnDet.size() != 0) {
    //         return MSNet::getInstance()->getCurrentTimeStep();
    //     }
    //     const SUMOTime now = SIMSTEP;
    //     if (MSGlobals::gUseMesoSim
    //             && !myNextMesoLeaveTimes.empty()
    //             && myNextMesoLeaveTimes.top() < now) {
    //         discardOldTimes(myNextMesoLeaveTimes, now);
    //         SUMOTime last = myNextMesoLeaveTimes.top();
    //         if (!myLastMesoLeaveTimes.empty() && myLastMesoLeaveTimes.top() < now) {
    //             discardOldTimes(myLastMesoLeaveTimes, now);
    //             last = MAX2(last, myLastMesoLeaveTimes.top());
    //         }
    //         //std::cout << SIMTIME << " det=" << getID() << " result=" << last << " last=" << myNextMesoLeaveTimes.top() << " last2=" << (myLastMesoLeaveTimes.size() > 0 ? myLastMesoLeaveTimes.top() : -1) << "\n";
    //         return last;
    //     }
    //     return TIME2STEPS(myLastLeaveTime);
    // }
    #[allow(unused_variables)]
    pub fn get_last_detection_time(&self) -> SUMOTime {
        if self.myOverrideTime >= 0. {
            return sim_step() - time2steps(self.myOverrideTime);
        }
        if !self.myVehiclesOnDet.is_empty() {
            return MSNet::get_instance().get_current_time_step();
        }
        let now = sim_step();
        if g_use_meso_sim() && !self.myNextMesoLeaveTimes.is_empty() && tq_top(&self.myNextMesoLeaveTimes) < now {
            // shortcut: C++ discards old times in its `mutable` queues; this method takes &self, so the cleanup runs on copies (meso is off)
            let mut next = self.myNextMesoLeaveTimes.clone();
            MSInductLoop::discard_old_times(&mut next, now);
            let mut last = tq_top(&next);
            if !self.myLastMesoLeaveTimes.is_empty() && tq_top(&self.myLastMesoLeaveTimes) < now {
                let mut lastQueue = self.myLastMesoLeaveTimes.clone();
                MSInductLoop::discard_old_times(&mut lastQueue, now);
                last = last.max(tq_top(&lastQueue));
            }
            return last;
        }
        time2steps(self.myLastLeaveTime)
    }

    /// C++: `double MSInductLoop::getOverrideTime() const` [const]
    // double getOverrideTime() const {
    //         return myOverrideTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_override_time(&self) -> f64 {
        self.myOverrideTime
    }

    /// C++: `void MSInductLoop::overrideTimeSinceDetection(double)`
    // void
    // MSInductLoop::overrideTimeSinceDetection(double time) {
    //     myOverrideTime = time;
    //     if (time < 0) {
    //         myOverrideEntryTime = -1;
    //     } else {
    //         const double entryTime = MAX2(0.0, SIMTIME - time);
    //         if (myOverrideEntryTime >= 0) {
    //             // maintain earlier entry time to achive continous detection
    //             myOverrideEntryTime = MIN2(myOverrideEntryTime, entryTime);
    //         } else {
    //             myOverrideEntryTime = entryTime;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn override_time_since_detection(&mut self, time: f64) {
        self.myOverrideTime = time;
        if time < 0. {
            self.myOverrideEntryTime = -1.;
        } else {
            let entryTime = max2(0., sim_time() - time);
            if self.myOverrideEntryTime >= 0. {
                // maintain earlier entry time to achive continous detection
                self.myOverrideEntryTime = min2(self.myOverrideEntryTime, entryTime);
            } else {
                self.myOverrideEntryTime = entryTime;
            }
        }
    }

    /// C++: `void MSInductLoop::saveState(OutputDevice &) const` [const, virtual]
    /// Saves the time of the last detection and the pending meso detections
    // void
    // MSInductLoop::saveState(OutputDevice& out) const {
    //     out.openTag(SUMO_TAG_INDUCTION_LOOP);
    //     out.writeAttr(SUMO_ATTR_ID, getID());
    //     out.writeAttr(SUMO_ATTR_TIME, myLastLeaveTime);
    //     if (mySegment != nullptr) {
    //         out.writeAttr(SUMO_ATTR_NEXT, getTimes(myNextMesoLeaveTimes, SIMSTEP));
    //         out.writeAttr(SUMO_ATTR_LAST, getTimes(myLastMesoLeaveTimes, SIMSTEP));
    //         // used for the next detection when a vehicle leaves the segment
    //         out.writeAttr(SUMO_ATTR_HEADWAY, toString<SUMOTime>(mySegment->getLastHeadway()));
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_INDUCTION_LOOP);
        out.write_attr(SUMO_ATTR_ID, self.base2.base.get_id());
        out.write_attr(SUMO_ATTR_TIME, &self.myLastLeaveTime);
        if !self.mySegment.is_null() {
            out.write_attr(SUMO_ATTR_NEXT, &MSInductLoop::get_times(self.myNextMesoLeaveTimes.clone(), sim_step()));
            out.write_attr(SUMO_ATTR_LAST, &MSInductLoop::get_times(self.myLastMesoLeaveTimes.clone(), sim_step()));
            // used for the next detection when a vehicle leaves the segment
            out.write_attr(SUMO_ATTR_HEADWAY, &self.mySegment.get_last_headway().to_string());
        }
        out.close_tag("");
    }

    /// C++: `void MSInductLoop::loadState(const SUMOSAXAttributes &, const SUMOTime)` [virtual]
    /// Loads the detection state, shifting all times by the given offset
    // void
    // MSInductLoop::loadState(const SUMOSAXAttributes& attrs, const SUMOTime offset) {
    //     bool ok = true;
    //     myLastLeaveTime = attrs.get<double>(SUMO_ATTR_TIME, getID().c_str(), ok) - STEPS2TIME(offset);
    //     if (mySegment != nullptr) {
    //         for (const std::string& t : StringTokenizer(attrs.getString(SUMO_ATTR_NEXT)).getVector()) {
    //             myNextMesoLeaveTimes.push(StringUtils::toLong(t) - offset);
    //         }
    //         for (const std::string& t : StringTokenizer(attrs.getString(SUMO_ATTR_LAST)).getVector()) {
    //             myLastMesoLeaveTimes.push(StringUtils::toLong(t) - offset);
    //         }
    //         mySegment->setLastHeadway(StringUtils::toLong(attrs.getString(SUMO_ATTR_HEADWAY)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes, offset: SUMOTime) {
        let mut ok = true;
        self.myLastLeaveTime = attrs.get::<f64>(SUMO_ATTR_TIME, self.base2.base.get_id(), &mut ok, true) - steps2time(offset);
        if !self.mySegment.is_null() {
            for t in StringTokenizer::new(&attrs.get_string(SUMO_ATTR_NEXT)).get_vector() {
                self.myNextMesoLeaveTimes.push(Reverse(string_utils::to_long(&t).unwrap() - offset));
            }
            for t in StringTokenizer::new(&attrs.get_string(SUMO_ATTR_LAST)).get_vector() {
                self.myLastMesoLeaveTimes.push(Reverse(string_utils::to_long(&t).unwrap() - offset));
            }
            self.mySegment.m().set_last_headway(string_utils::to_long(&attrs.get_string(SUMO_ATTR_HEADWAY)).unwrap());
        }
    }

    /// C++: `void MSInductLoop::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Writes collected values into the given stream
    // void
    // MSInductLoop::writeXMLOutput(OutputDevice& dev, SUMOTime startTime, SUMOTime stopTime) {
    //     if (dev.isNull()) {
    //         reset();
    //         return;
    //     }
    //     const double t(STEPS2TIME(stopTime - startTime));
    //     double occupancy = 0.;
    //     double speedSum = 0.;
    //     double lengthSum = 0.;
    //     int contrib = 0;
    //     // to approximate the space mean speed
    //     double inverseSpeedSum = 0.;
    //     for (const VehicleData& vData : myVehicleDataCont) {
    //         const double timeOnDetDuringInterval = vData.leaveTimeM - MAX2(STEPS2TIME(startTime), vData.entryTimeM);
    //         occupancy += MIN2(timeOnDetDuringInterval, t);
    //         if (!vData.leftEarlyM) {
    //             speedSum += vData.speedM;
    //             assert(vData.speedM > 0.);
    //             inverseSpeedSum += 1. / vData.speedM;
    //             lengthSum += vData.lengthM;
    //             contrib++;
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         contrib = myEnteredVehicleNumber;
    //     }
    //     const double flow = (double)contrib / t * 3600.;
    //     for (std::map< SUMOTrafficObject*, double >::const_iterator i = myVehiclesOnDet.begin(); i != myVehiclesOnDet.end(); ++i) {
    //         occupancy += STEPS2TIME(stopTime) - MAX2(STEPS2TIME(startTime), i->second);
    //     }
    //     occupancy *= 100. / t;
    //     const double meanSpeed = contrib != 0 ? speedSum / (double)contrib : -1;
    //     const double harmonicMeanSpeed = contrib != 0 ? (double)contrib / inverseSpeedSum : -1;
    //     const double meanLength = contrib != 0 ? lengthSum / (double)contrib : -1;
    //     dev.openTag(SUMO_TAG_INTERVAL).writeTime(SUMO_ATTR_BEGIN, startTime).writeTime(SUMO_ATTR_END, stopTime);
    //     dev.writeAttr(SUMO_ATTR_ID, StringUtils::escapeXML(getID())).writeAttr("nVehContrib", contrib);
    //     dev.writeAttr("flow", flow).writeAttr("occupancy", occupancy).writeAttr("speed", meanSpeed).writeAttr("harmonicMeanSpeed", harmonicMeanSpeed);
    //     dev.writeAttr("length", meanLength).writeAttr("nVehEntered", myEnteredVehicleNumber).closeTag();
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        if dev.is_null() {
            self.reset();
            return;
        }
        let t = steps2time(stopTime - startTime);
        let mut occupancy = 0.;
        let mut speedSum = 0.;
        let mut lengthSum = 0.;
        let mut contrib: i32 = 0;
        // to approximate the space mean speed
        let mut inverseSpeedSum = 0.;
        for vData in self.myVehicleDataCont.iter() {
            let timeOnDetDuringInterval = vData.leaveTimeM - max2(steps2time(startTime), vData.entryTimeM);
            occupancy += min2(timeOnDetDuringInterval, t);
            if !vData.leftEarlyM {
                speedSum += vData.speedM;
                debug_assert!(vData.speedM > 0.);
                inverseSpeedSum += 1. / vData.speedM;
                lengthSum += vData.lengthM;
                contrib += 1;
            }
        }
        if g_use_meso_sim() {
            contrib = self.myEnteredVehicleNumber;
        }
        let flow = contrib as f64 / t * 3600.;
        for &second in self.myVehiclesOnDet.values() {
            occupancy += steps2time(stopTime) - max2(steps2time(startTime), second);
        }
        occupancy *= 100. / t;
        let meanSpeed = if contrib != 0 { speedSum / contrib as f64 } else { -1. };
        let harmonicMeanSpeed = if contrib != 0 { contrib as f64 / inverseSpeedSum } else { -1. };
        let meanLength = if contrib != 0 { lengthSum / contrib as f64 } else { -1. };
        dev.open_tag_id(SUMO_TAG_INTERVAL).write_time(SUMO_ATTR_BEGIN, startTime).write_time(SUMO_ATTR_END, stopTime);
        let escapedID = string_utils::escape_xml(self.base2.base.get_id(), false);
        dev.write_attr(SUMO_ATTR_ID, &escapedID).write_attr("nVehContrib", &contrib);
        dev.write_attr("flow", &flow).write_attr("occupancy", &occupancy).write_attr("speed", &meanSpeed).write_attr("harmonicMeanSpeed", &harmonicMeanSpeed);
        dev.write_attr("length", &meanLength).write_attr("nVehEntered", &self.myEnteredVehicleNumber).close_tag("");
        self.reset();
    }

    /// C++: `void MSInductLoop::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "detector" as root element
    // void
    // MSInductLoop::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("detector", "det_e1_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("detector", "det_e1_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `void MSInductLoop::detectorUpdate(const SUMOTime)` [virtual]
    /// Updates the detector (computes values) only used when detecting persons
    // void
    // MSInductLoop::detectorUpdate(const SUMOTime /* step */) {
    //     if (myDetectPersons == (int)PersonMode::NONE) {
    //         return;
    //     }
    //     if (myLane->hasPedestrians()) {
    //         for (MSTransportable* p : myLane->getEdge().getPersons()) {
    //             if (p->getLane() != myLane || !vehicleApplies(*p)) {
    //                 continue;
    //             }
    //             notifyMovePerson(p, p->getDirection(), p->getPositionOnLane());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        if self.base2.myDetectPersons == PersonMode::NONE as i32 {
            return;
        }
        let lane = self.myLane;
        if lane.has_pedestrians() {
            for &p in lane.get_edge().get_persons() {
                if p.get_lane() != lane || !self.base2.vehicle_applies(&p.base) {
                    continue;
                }
                self.notify_move_person(p, p.get_direction(), p.get_position_on_lane());
            }
        }
    }

    /// C++: `std::vector<VehicleData> MSInductLoop::collectVehiclesOnDet(SUMOTime, bool, bool, bool, bool) const` [const]
    /// Returns vehicle data for vehicles that have been on the detector starting at the given time
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn collect_vehicles_on_det(&self, t: SUMOTime, includeEarly: bool, leaveTime: bool, forOccupancy: bool, lastInterval: bool) -> Vec<MSInductLoop_VehicleData> {
        let tt = steps2time(t);
        let mut ret: Vec<MSInductLoop_VehicleData> = Vec::new();
        for i in self.myVehicleDataCont.iter() {
            if (includeEarly || !i.leftEarlyM) && (!lastInterval || i.entryTimeM < tt) {
                if i.entryTimeM >= tt || (leaveTime && i.leaveTimeM >= tt) {
                    ret.push(i.clone());
                }
            }
        }
        for i in self.myLastVehicleDataCont.iter() {
            if includeEarly || !i.leftEarlyM {
                if (!lastInterval && (i.entryTimeM >= tt || (leaveTime && i.leaveTimeM >= tt)))
                    || (lastInterval && i.leaveTimeM <= tt + steps2time(self.myLastIntervalEnd - self.myLastIntervalBegin)) {
                    // TODO: check duration of last interval (kept from C++)
                    ret.push(i.clone());
                }
            }
        }
        for (&v, &entry) in self.myVehiclesOnDet.iter() {
            if (!lastInterval && (entry >= tt || leaveTime || forOccupancy))
                || (lastInterval && entry < tt && tt - entry < steps2time(delta_t())) {
                // no need to check leave time, they are still on the detector
                let mut d = MSInductLoop_VehicleData::new(&v, entry, HAS_NOT_LEFT_DETECTOR, false, 0.);
                d.speedM = v.get_speed();
                ret.push(d);
            }
        }
        ret
    }

    /// C++: `void MSInductLoop::setSpecialColor(const RGBColor *)` [virtual]
    /// allows for special color in the gui version
    // virtual void setSpecialColor(const RGBColor* /*color*/) {}
    #[allow(unused_variables)]
    pub fn set_special_color(&mut self, arg0: Ptr<RGBColor>) {
        // C++ default implementation is empty (GUI versions override it)
    }

    /// C++: `void MSInductLoop::setVisible(bool)` [virtual]
    // virtual void setVisible(bool /*show*/) {}
    #[allow(unused_variables)]
    pub fn set_visible(&mut self, arg0: bool) {
        // C++ default implementation is empty (GUI versions override it)
    }

    /// C++: `void MSInductLoop::clearState(SUMOTime)` [virtual]
    /// Remove all vehicles before quick-loading state
    // void
    // MSInductLoop::clearState(SUMOTime time) {
    //     myLastLeaveTime = STEPS2TIME(time);
    //     myEnteredVehicleNumber = 0;
    //     myLastVehicleDataCont.clear();
    //     myVehicleDataCont.clear();
    //     myVehiclesOnDet.clear();
    //     myNextMesoLeaveTimes = TimeQueue();
    //     myLastMesoLeaveTimes = TimeQueue();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, time: SUMOTime) {
        self.myLastLeaveTime = steps2time(time);
        self.myEnteredVehicleNumber = 0;
        self.myLastVehicleDataCont.clear();
        self.myVehicleDataCont.clear();
        self.myVehiclesOnDet.clear();
        self.myNextMesoLeaveTimes = MSInductLoop_TimeQueue::default();
        self.myLastMesoLeaveTimes = MSInductLoop_TimeQueue::default();
    }

    /// C++: `double MSInductLoop::speedSum(double, const MSInductLoop::VehicleData &)` [static, protected]
    /// Adds up VehicleData::speedM
    // static inline double speedSum(double sumSoFar, const MSInductLoop::VehicleData& data) {
    //         return sumSoFar + data.speedM;
    //     }
    #[allow(unused_variables)]
    pub fn speed_sum(sumSoFar: f64, data: &MSInductLoop_VehicleData) -> f64 {
        sumSoFar + data.speedM
    }

    /// C++: `double MSInductLoop::lengthSum(double, const MSInductLoop::VehicleData &)` [static, protected]
    /// Adds up VehicleData::lengthM
    // static inline double lengthSum(double sumSoFar, const MSInductLoop::VehicleData& data) {
    //         return sumSoFar + data.lengthM;
    //     }
    #[allow(unused_variables)]
    pub fn length_sum(sumSoFar: f64, data: &MSInductLoop_VehicleData) -> f64 {
        sumSoFar + data.lengthM
    }

    /// C++: `void MSInductLoop::notifyMovePerson(MSTransportable *, int, double)` [protected]
    /// helper function for mapping person movement
    // void
    // MSInductLoop::notifyMovePerson(MSTransportable* p, int dir, double pos) {
    //     if (personApplies(*p, dir)) {
    //         const double newSpeed = p->getSpeed();
    //         const double newPos = (dir == MSPModel::FORWARD
    //                                ? pos
    //                                // position relative to detector
    //                                : myPosition - (pos - myPosition));
    //         const double oldPos = newPos - SPEED2DIST(newSpeed);
    //         if (oldPos - p->getVehicleType().getLength() <= myPosition) {
    //             notifyMove(*p, oldPos, newPos, newSpeed);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_person(&mut self, p: Ptr<MSTransportable>, dir: i32, pos: f64) {
        if self.base2.person_applies(&p, dir) {
            let newSpeed = p.get_speed();
            let newPos = if dir == MSPMODEL_FORWARD { pos } else { self.myPosition - (pos - self.myPosition) };
            let oldPos = newPos - speed2dist(newSpeed);
            if oldPos - p.get_vehicle_type().get_length() <= self.myPosition {
                self.notify_move(&mut p.m().base, oldPos, newPos, newSpeed);
            }
        }
    }

    /// C++: `void MSInductLoop::discardOldTimes(TimeQueue &, SUMOTime)` [static, protected]
    // void
    // MSInductLoop::discardOldTimes(TimeQueue& tq, SUMOTime now) {
    //     // discard times in the past except for the latest one
    //     SUMOTime last = tq.top();
    //     if (tq.size() > 1) {
    //         tq.pop();
    //         while (tq.size() > 0 && tq.top() < now) {
    //             last = tq.top();
    //             tq.pop();
    //         }
    //         tq.push(last);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn discard_old_times(tq: &mut MSInductLoop_TimeQueue, now: SUMOTime) {
        // discard times in the past except for the latest one
        let mut last = tq_top(tq);
        if tq.len() > 1 {
            tq.pop();
            while !tq.is_empty() && tq_top(tq) < now {
                last = tq_top(tq);
                tq.pop();
            }
            tq.push(Reverse(last));
        }
    }

    /// C++: `std::vector<SUMOTime> MSInductLoop::getTimes(TimeQueue, SUMOTime)` [static, protected]
    // std::vector<SUMOTime>
    // MSInductLoop::getTimes(TimeQueue tq, SUMOTime now) {
    //     std::vector<SUMOTime> result;
    //     if (!tq.empty()) {
    //         // the next query discards them anyway
    //         discardOldTimes(tq, now);
    //     }
    //     while (!tq.empty()) {
    //         result.push_back(tq.top());
    //         tq.pop();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_times(mut tq: MSInductLoop_TimeQueue, now: SUMOTime) -> Vec<SUMOTime> {
        let mut result = Vec::new();
        if !tq.is_empty() {
            // the next query discards them anyway
            MSInductLoop::discard_old_times(&mut tq, now);
        }
        while let Some(Reverse(x)) = tq.pop() {
            result.push(x);
        }
        result
    }

    /// C++: `void MSInductLoop::MSInductLoop(const MSInductLoop &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInductLoop) -> Self {
        // C++ declares this copy constructor private and never defines it, so it cannot be called
        panic!("MSInductLoop copy constructor is invalidated (private, undefined in C++)")
    }

    /// C++: `MSInductLoop & MSInductLoop::operator=(const MSInductLoop &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInductLoop) -> &mut MSInductLoop {
        // C++ declares this assignment operator private and never defines it, so it cannot be called
        panic!("MSInductLoop::operator= is invalidated (private, undefined in C++)")
    }

}

/// Struct to store the data of the counted vehicle internally.
/// C++ class `MSInductLoop::VehicleData` (microsim/output/MSInductLoop.h)
#[derive(Clone)]
pub struct MSInductLoop_VehicleData {
    /// The id of the vehicle
    pub idM: String,
    /// Length of the vehicle
    pub lengthM: f64,
    /// Entry-time of the vehicle in [s]
    pub entryTimeM: f64,
    /// Leave-time of the vehicle in [s]
    pub leaveTimeM: f64,
    /// Speed of the vehicle in [m/s]
    pub speedM: f64,
    /// Type of the vehicle
    pub typeIDM: String,
    /// whether the vehicle left the detector with a lane change / teleport etc.
    pub leftEarlyM: bool,
}

impl MSInductLoop_VehicleData {
    /// C++: `void VehicleData::VehicleData(const SUMOTrafficObject &, double, double, const bool, const double)`
    /// Constructor
    // MSInductLoop::VehicleData::VehicleData(const SUMOTrafficObject& v, double entryTimestep,
    //                                        double leaveTimestep, const bool leftEarly, const double detLength)
    //     : idM(v.getID()), lengthM(v.getVehicleType().getLength()), entryTimeM(entryTimestep), leaveTimeM(leaveTimestep),
    //       speedM((v.getVehicleType().getLength() + detLength) / MAX2(leaveTimestep - entryTimestep, NUMERICAL_EPS)), typeIDM(v.getVehicleType().getID()),
    //       leftEarlyM(leftEarly) {}
    #[allow(unused_variables)]
    pub fn new(v: &SUMOTrafficObject, entryTimestep: f64, leaveTimestep: f64, leftEarly: bool, detLength: f64) -> Self {
        let len = v.get_vehicle_type().get_length();
        MSInductLoop_VehicleData {
            idM: v.get_id().to_string(),
            lengthM: len,
            entryTimeM: entryTimestep,
            leaveTimeM: leaveTimestep,
            speedM: (len + detLength) / max2(leaveTimestep - entryTimestep, NUMERICAL_EPS),
            typeIDM: v.get_vehicle_type().get_id().to_string(),
            leftEarlyM: leftEarly,
        }
    }

}

