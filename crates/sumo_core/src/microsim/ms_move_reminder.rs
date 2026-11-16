//! Port of microsim/MSMoveReminder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `MSMoveReminder::Notification`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSMoveReminder_Notification {
    NOTIFICATION_DEPARTED = 0,
    NOTIFICATION_JUNCTION = 1,
    NOTIFICATION_SEGMENT = 2,
    NOTIFICATION_LANE_CHANGE = 3,
    NOTIFICATION_LOAD_STATE = 4,
    NOTIFICATION_TELEPORT = 5,
    NOTIFICATION_TELEPORT_CONTINUATION = 6,
    NOTIFICATION_PARKING = 7,
    NOTIFICATION_REROUTE = 8,
    NOTIFICATION_PARKING_REROUTE = 9,
    NOTIFICATION_ARRIVED = 10,
    NOTIFICATION_TELEPORT_ARRIVED = 11,
    NOTIFICATION_VAPORIZED_CALIBRATOR = 12,
    NOTIFICATION_VAPORIZED_COLLISION = 13,
    NOTIFICATION_VAPORIZED_TRACI = 14,
    NOTIFICATION_VAPORIZED_GUI = 15,
    NOTIFICATION_VAPORIZED_VAPORIZER = 16,
    NOTIFICATION_VAPORIZED_BREAKDOWN = 17,
    NOTIFICATION_NONE = 18,
}

/// Virtual dispatch table for the polymorphic methods of MSMoveReminder.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct MSMoveReminderVt {
    pub notify_enter: fn(&mut MSMoveReminder, &mut SUMOTrafficObject, MSMoveReminder_Notification, Ptr<MSLane>) -> bool,
    pub notify_move: fn(&mut MSMoveReminder, &mut SUMOTrafficObject, f64, f64, f64) -> bool,
    pub notify_leave: fn(&mut MSMoveReminder, &mut SUMOTrafficObject, f64, MSMoveReminder_Notification, Ptr<MSLane>) -> bool,
}

/// Virtual dispatch table for the idle, parking and stop-ended notifications; kept apart from
/// `MSMoveReminderVt` so the existing per-subclass tables stay valid.
pub struct MSMoveReminderIdleVt {
    pub notify_idle: fn(&mut MSMoveReminder, &mut SUMOTrafficObject) -> bool,
    pub notify_parking: fn(&mut MSMoveReminder),
    pub notify_stop_ended: fn(&mut MSMoveReminder),
}

/// Something on a lane to be noticed about vehicle movement
/// C++ class `MSMoveReminder` (microsim/MSMoveReminder.h)
#[repr(C)]
pub struct MSMoveReminder {
    /// Lane on which the reminder works
    pub myLane: Ptr<MSLane>,
    /// a description of this moveReminder
    pub myDescription: String,
    pub myLastVehicleUpdateValues: BTreeMap<i64, (SUMOTime, f64)>, // C++: std::map<long long, std::pair<SUMOTime, double>>
    /// Virtual dispatch table; installed by the most derived constructor
    pub vt: &'static MSMoveReminderVt,
    /// Virtual dispatch table for idle/parking/stop-ended; installed by the most derived constructor
    pub idle_vt: &'static MSMoveReminderIdleVt,
}

// C++ static members of MSMoveReminder (port as module-level statics, see AGENTS.md):
//   static StringBijection<Notification> MSMoveReminder::Notifications;
//   static StringBijection<Notification>::Entry[] MSMoveReminder::NotificationValues;

impl MSMoveReminder {
    /// C++: `void MSMoveReminder::MSMoveReminder(const std::string &, MSLane *const, const bool)`
    /// Constructor.
    // MSMoveReminder::MSMoveReminder(const std::string& description, MSLane* const lane, const bool doAdd) :
    //     myLane(lane),
    //     myDescription(description)
    // #ifdef HAVE_FOX
    //     , myNotificationMutex(true)
    // #endif
    // {
    //     if (myLane != nullptr && doAdd) {
    //         // add reminder to lane
    //         myLane->addMoveReminder(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(description: &str, lane: Ptr<MSLane>, doAdd: bool) -> Self {
        // shortcut: C++ registers `this` with myLane when doAdd (myLane->addMoveReminder(this)); Rust returns Self by value so callers must register the placed reminder via MSLane::add_move_reminder, upgrade when constructors can take a stable address
        let _ = doAdd;
        Self {
            myLane: lane,
            myDescription: description.to_string(),
            myLastVehicleUpdateValues: BTreeMap::new(),
            vt: &DEFAULT_MSMOVE_REMINDER_VT,
            idle_vt: &DEFAULT_MSMOVE_REMINDER_IDLE_VT,
        }
    }

    /// C++: `void MSMoveReminder::~MSMoveReminder() noexcept` [virtual]
    /// Destructor
    // virtual ~MSMoveReminder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty.
    }

    /// C++: `const MSLane * MSMoveReminder::getLane() const` [const]
    /// Returns the lane the reminder works on.
    // const MSLane* getLane() const {
    //         return myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        self.myLane
    }

    /// C++: `bool MSMoveReminder::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Checks whether the reminder is activated by a vehicle entering the lane
    // virtual bool notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane) {
    //         UNUSED_PARAMETER(reason);
    //         UNUSED_PARAMETER(&veh);
    //         UNUSED_PARAMETER(&enteredLane);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        (self.vt.notify_enter)(self, veh, reason, enteredLane)
    }

    /// C++: `bool MSMoveReminder::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the reminder still has to be notified about the vehicle moves
    // virtual bool notifyMove(SUMOTrafficObject& veh,
    //                             double oldPos,
    //                             double newPos,
    //                             double newSpeed) {
    //         UNUSED_PARAMETER(oldPos);
    //         UNUSED_PARAMETER(newPos);
    //         UNUSED_PARAMETER(newSpeed);
    //         UNUSED_PARAMETER(&veh);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        (self.vt.notify_move)(self, veh, oldPos, newPos, newSpeed)
    }

    /// C++: `bool MSMoveReminder::notifyIdle(SUMOTrafficObject &)` [virtual]
    /// Computes idling emission values and adds them to the emission sums
    // virtual bool notifyIdle(SUMOTrafficObject& veh) {
    //         UNUSED_PARAMETER(&veh);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        (self.idle_vt.notify_idle)(self, veh)
    }

    /// C++: `void MSMoveReminder::notifyParking()` [virtual]
    /// called to update state for parking vehicles
    // virtual void notifyParking() {}
    #[allow(unused_variables)]
    pub fn notify_parking(&mut self) {
        (self.idle_vt.notify_parking)(self)
    }

    /// C++: `void MSMoveReminder::notifyStopEnded()` [virtual]
    /// called to update state for stopped vehicles
    // virtual void notifyStopEnded() {}
    #[allow(unused_variables)]
    pub fn notify_stop_ended(&mut self) {
        (self.idle_vt.notify_stop_ended)(self)
    }

    /// C++: `bool MSMoveReminder::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    /// Called if the vehicle leaves the reminder's lane
    // virtual bool notifyLeave(SUMOTrafficObject& veh, double lastPos, Notification reason, const MSLane* enteredLane = nullptr) {
    //         UNUSED_PARAMETER(&veh);
    //         UNUSED_PARAMETER(lastPos);
    //         UNUSED_PARAMETER(reason);
    //         UNUSED_PARAMETER(enteredLane);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        (self.vt.notify_leave)(self, veh, lastPos, reason, enteredLane)
    }

    /// C++: `bool MSMoveReminder::notifyLeaveBack(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Called if the vehicle's back leaves the reminder's lane
    // virtual bool notifyLeaveBack(SUMOTrafficObject& veh, Notification reason, const MSLane* leftLane) {
    //         UNUSED_PARAMETER(&veh);
    //         UNUSED_PARAMETER(reason);
    //         UNUSED_PARAMETER(leftLane);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave_back(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, leftLane: Ptr<MSLane>) -> bool {
        // UNUSED_PARAMETER(veh, reason, leftLane)
        true
    }

    /// C++: `bool MSMoveReminder::notifyReroute(SUMOTrafficObject &)` [virtual]
    /// Called if the vehicle change it's route
    // virtual bool notifyReroute(SUMOTrafficObject& veh) {
    //         UNUSED_PARAMETER(&veh);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_reroute(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        // UNUSED_PARAMETER(veh)
        true
    }

    /// C++: `void MSMoveReminder::updateDetector(SUMOTrafficObject &, double, double, SUMOTime, SUMOTime, SUMOTime, bool)`
    // void
    // MSMoveReminder::updateDetector(SUMOTrafficObject& veh, double entryPos, double leavePos,
    //                                SUMOTime entryTime, SUMOTime currentTime, SUMOTime leaveTime,
    //                                bool cleanUp) {
    //     // each vehicle is tracked linearly across its segment. For each vehicle,
    //     // the time and position of the previous call are maintained and only
    //     // the increments are sent to notifyMoveInternal
    //     if (entryTime > currentTime) {
    //         return; // calibrator may insert vehicles a tiny bit into the future; ignore those
    //     }
    //     auto j = myLastVehicleUpdateValues.find(veh.getNumericalID());
    //     if (j != myLastVehicleUpdateValues.end()) {
    //         // the vehicle already has reported its values before; use these
    //         // however, if this was called from prepareDetectorForWriting the time
    //         // only has a resolution of DELTA_T and might be invalid
    //         const SUMOTime previousUpdateTime = j->second.first;
    //         if (previousUpdateTime <= currentTime) {
    //             entryTime = previousUpdateTime;
    //             entryPos = j->second.second;
    //         }
    //     }
    //     assert(entryTime <= currentTime);
    //     if ((entryTime < leaveTime) && (entryPos <= leavePos)) {
    //         const double timeOnLane = STEPS2TIME(currentTime - entryTime);
    //         const double speed = (leavePos - entryPos) / STEPS2TIME(leaveTime - entryTime);
    //         myLastVehicleUpdateValues[veh.getNumericalID()] = std::pair<SUMOTime, double>(currentTime, entryPos + speed * timeOnLane);
    //         assert(timeOnLane >= 0);
    //         notifyMoveInternal(veh, timeOnLane, timeOnLane, speed, speed, speed * timeOnLane, speed * timeOnLane, 0.);
    //     } else {
    //         // it would be natural to
    //         // assert(entryTime == leaveTime);
    //         // assert(entryPos == leavePos);
    //         // However, in the presence of calibrators, vehicles may jump a bit
    //         myLastVehicleUpdateValues[veh.getNumericalID()] = std::pair<SUMOTime, double>(leaveTime, leavePos);
    //     }
    //     if (cleanUp) {
    //         // clean up after the vehicle has left the area of this reminder
    //         removeFromVehicleUpdateValues(veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_detector(&mut self, veh: &mut SUMOTrafficObject, entryPos: f64, leavePos: f64, entryTime: SUMOTime, currentTime: SUMOTime, leaveTime: SUMOTime, cleanUp: bool) {
        // each vehicle is tracked linearly across its segment. For each vehicle,
        // the time and position of the previous call are maintained and only
        // the increments are sent to notifyMoveInternal
        let (mut entryPos, mut entryTime) = (entryPos, entryTime);
        if entryTime > currentTime {
            return; // calibrator may insert vehicles a tiny bit into the future; ignore those
        }
        let id = veh.get_numerical_id();
        if let Some(&(previousUpdateTime, previousPos)) = self.myLastVehicleUpdateValues.get(&id) {
            // the vehicle already has reported its values before; use these
            // however, if this was called from prepareDetectorForWriting the time
            // only has a resolution of DELTA_T and might be invalid
            if previousUpdateTime <= currentTime {
                entryTime = previousUpdateTime;
                entryPos = previousPos;
            }
        }
        assert!(entryTime <= currentTime);
        if (entryTime < leaveTime) && (entryPos <= leavePos) {
            let timeOnLane = steps2time(currentTime - entryTime);
            let speed = (leavePos - entryPos) / steps2time(leaveTime - entryTime);
            self.myLastVehicleUpdateValues.insert(id, (currentTime, entryPos + speed * timeOnLane));
            assert!(timeOnLane >= 0.0);
            self.notify_move_internal(veh, timeOnLane, timeOnLane, speed, speed, speed * timeOnLane, speed * timeOnLane, 0.);
        } else {
            // it would be natural to
            // assert(entryTime == leaveTime);
            // assert(entryPos == leavePos);
            // However, in the presence of calibrators, vehicles may jump a bit
            self.myLastVehicleUpdateValues.insert(id, (leaveTime, leavePos));
        }
        if cleanUp {
            // clean up after the vehicle has left the area of this reminder
            self.remove_from_vehicle_update_values(veh);
        }
    }

    /// C++: `void MSMoveReminder::notifyMoveInternal(const SUMOTrafficObject &, const double, const double, const double, const double, const double, const double, const double)` [virtual]
    /// Internal notification about the vehicle moves.
    // virtual void notifyMoveInternal(const SUMOTrafficObject& veh,
    //                                     const double frontOnLane,
    //                                     const double timeOnLane,
    //                                     const double meanSpeedFrontOnLane,
    //                                     const double meanSpeedVehicleOnLane,
    //                                     const double travelledDistanceFrontOnLane,
    //                                     const double travelledDistanceVehicleOnLane,
    //                                     const double meanLengthOnLane) {
    //         UNUSED_PARAMETER(&veh);
    //         UNUSED_PARAMETER(frontOnLane);
    //         UNUSED_PARAMETER(timeOnLane);
    //         UNUSED_PARAMETER(meanSpeedFrontOnLane);
    //         UNUSED_PARAMETER(meanSpeedVehicleOnLane);
    //         UNUSED_PARAMETER(travelledDistanceFrontOnLane);
    //         UNUSED_PARAMETER(travelledDistanceVehicleOnLane);
    //         UNUSED_PARAMETER(meanLengthOnLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move_internal(&mut self, veh: &SUMOTrafficObject, frontOnLane: f64, timeOnLane: f64, meanSpeedFrontOnLane: f64, meanSpeedVehicleOnLane: f64, travelledDistanceFrontOnLane: f64, travelledDistanceVehicleOnLane: f64, meanLengthOnLane: f64) {
        // C++ body only consumes its parameters (UNUSED_PARAMETER); nothing to do.
    }

    /// C++: `void MSMoveReminder::setDescription(const std::string &)`
    // void setDescription(const std::string& description) {
    //         myDescription = description;
    //     }
    #[allow(unused_variables)]
    pub fn set_description(&mut self, description: &str) {
        self.myDescription = description.to_string();
    }

    /// C++: `const std::string & MSMoveReminder::getDescription() const` [const]
    // const std::string& getDescription() const {
    //         return myDescription;
    //     }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> &str {
        &self.myDescription
    }

    /// C++: `bool MSMoveReminder::isParkingRerouter() const` [const, virtual]
    // virtual bool isParkingRerouter() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_parking_rerouter(&self) -> bool {
        false
    }

    /// C++: `void MSMoveReminder::saveReminderState(OutputDevice &, const SUMOTrafficObject &)`
    /// Saves the current state into the given stream
    // void
    // MSMoveReminder::saveReminderState(OutputDevice& out, const SUMOTrafficObject& veh) {
    //     auto j = myLastVehicleUpdateValues.find(veh.getNumericalID());
    //     if (j != myLastVehicleUpdateValues.end()) {
    //         out.openTag(SUMO_TAG_REMINDER);
    //         out.writeAttr(SUMO_ATTR_ID, getDescription());
    //         out.writeAttr(SUMO_ATTR_TIME, (*j).second.first);
    //         out.writeAttr(SUMO_ATTR_POSITION, (*j).second.second);
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_reminder_state(&mut self, out: &mut OutputDevice, veh: &SUMOTrafficObject) {
        if let Some(&(time, pos)) = self.myLastVehicleUpdateValues.get(&veh.get_numerical_id()) {
            out.open_tag_id(SUMO_TAG_REMINDER);
            out.write_attr(SUMO_ATTR_ID, self.get_description());
            out.write_attr(SUMO_ATTR_TIME, &time);
            out.write_attr(SUMO_ATTR_POSITION, &pos);
            out.close_tag("");
        }
    }

    /// C++: `void MSMoveReminder::loadReminderState(long long, SUMOTime, double)`
    // void
    // MSMoveReminder::loadReminderState(long long int numID, SUMOTime time, double pos) {
    //     myLastVehicleUpdateValues[numID] = std::make_pair(time, pos);
    // }
    #[allow(unused_variables)]
    pub fn load_reminder_state(&mut self, numID: i64, time: SUMOTime, pos: f64) {
        self.myLastVehicleUpdateValues.insert(numID, (time, pos));
    }

    /// C++: `void MSMoveReminder::removeFromVehicleUpdateValues(SUMOTrafficObject &)` [protected]
    // void
    // MSMoveReminder::removeFromVehicleUpdateValues(SUMOTrafficObject& veh) {
    //     myLastVehicleUpdateValues.erase(veh.getNumericalID());
    // }
    #[allow(unused_variables)]
    pub fn remove_from_vehicle_update_values(&mut self, veh: &mut SUMOTrafficObject) {
        self.myLastVehicleUpdateValues.remove(&veh.get_numerical_id());
    }

    /// C++: `MSMoveReminder & MSMoveReminder::operator=(const MSMoveReminder &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMoveReminder) -> &mut MSMoveReminder {
        // C++ assignment operator is private and only declared (avoids a compiler warning); it must not be called.
        panic!("MSMoveReminder assignment operator is private and undefined in C++")
    }

}

pub(crate) fn default_notify_enter(_: &mut MSMoveReminder, _: &mut SUMOTrafficObject, _: MSMoveReminder_Notification, _: Ptr<MSLane>) -> bool {
    true
}
pub(crate) fn default_notify_move(_: &mut MSMoveReminder, _: &mut SUMOTrafficObject, _: f64, _: f64, _: f64) -> bool {
    true
}
pub(crate) fn default_notify_leave(_: &mut MSMoveReminder, _: &mut SUMOTrafficObject, _: f64, _: MSMoveReminder_Notification, _: Ptr<MSLane>) -> bool {
    true
}

pub(crate) fn default_notify_idle(_: &mut MSMoveReminder, _: &mut SUMOTrafficObject) -> bool {
    true
}
pub(crate) fn default_notify_parking(_: &mut MSMoveReminder) {}
pub(crate) fn default_notify_stop_ended(_: &mut MSMoveReminder) {}

static DEFAULT_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: default_notify_enter,
    notify_move: default_notify_move,
    notify_leave: default_notify_leave,
};

static DEFAULT_MSMOVE_REMINDER_IDLE_VT: MSMoveReminderIdleVt = MSMoveReminderIdleVt {
    notify_idle: default_notify_idle,
    notify_parking: default_notify_parking,
    notify_stop_ended: default_notify_stop_ended,
};

