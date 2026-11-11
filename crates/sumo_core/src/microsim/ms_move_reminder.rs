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
    NOTIFICATION_TELEPORT_CONTINUATION_ARRIVED = 12,
    NOTIFICATION_VAPORIZED_CALIBRATOR = 13,
    NOTIFICATION_VAPORIZED_COLLISION = 14,
    NOTIFICATION_VAPORIZED_TRACI = 15,
    NOTIFICATION_VAPORIZED_GUI = 16,
    NOTIFICATION_VAPORIZED_VAPORIZER = 17,
    NOTIFICATION_VAPORIZED_BREAKDOWN = 18,
    NOTIFICATION_NONE = 19,
}

/// Something on a lane to be noticed about vehicle movement
/// C++ class `MSMoveReminder` (microsim/MSMoveReminder.h)
pub struct MSMoveReminder {
    /// Lane on which the reminder works
    pub myLane: Ptr<MSLane>,
    /// a description of this moveReminder
    pub myDescription: String,
    pub myLastVehicleUpdateValues: BTreeMap<i64, (SUMOTime, f64)>, // C++: std::map<long long, std::pair<SUMOTime, double>>
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
        todo!("port MSMoveReminder::MSMoveReminder")
    }

    /// C++: `void MSMoveReminder::~MSMoveReminder() noexcept` [virtual]
    /// Destructor
    // virtual ~MSMoveReminder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSMoveReminder::~MSMoveReminder")
    }

    /// C++: `const MSLane * MSMoveReminder::getLane() const` [const]
    /// Returns the lane the reminder works on.
    // const MSLane* getLane() const {
    //         return myLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        todo!("port MSMoveReminder::getLane")
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
        todo!("port MSMoveReminder::notifyEnter")
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
        todo!("port MSMoveReminder::notifyMove")
    }

    /// C++: `bool MSMoveReminder::notifyIdle(SUMOTrafficObject &)` [virtual]
    /// Computes idling emission values and adds them to the emission sums
    // virtual bool notifyIdle(SUMOTrafficObject& veh) {
    //         UNUSED_PARAMETER(&veh);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_idle(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        todo!("port MSMoveReminder::notifyIdle")
    }

    /// C++: `void MSMoveReminder::notifyParking()` [virtual]
    /// called to update state for parking vehicles
    // virtual void notifyParking() {}
    #[allow(unused_variables)]
    pub fn notify_parking(&mut self) {
        todo!("port MSMoveReminder::notifyParking")
    }

    /// C++: `void MSMoveReminder::notifyStopEnded()` [virtual]
    /// called to update state for stopped vehicles
    // virtual void notifyStopEnded() {}
    #[allow(unused_variables)]
    pub fn notify_stop_ended(&mut self) {
        todo!("port MSMoveReminder::notifyStopEnded")
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
        todo!("port MSMoveReminder::notifyLeave")
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
        todo!("port MSMoveReminder::notifyLeaveBack")
    }

    /// C++: `bool MSMoveReminder::notifyReroute(SUMOTrafficObject &)` [virtual]
    /// Called if the vehicle change it's route
    // virtual bool notifyReroute(SUMOTrafficObject& veh) {
    //         UNUSED_PARAMETER(&veh);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn notify_reroute(&mut self, veh: &mut SUMOTrafficObject) -> bool {
        todo!("port MSMoveReminder::notifyReroute")
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
        todo!("port MSMoveReminder::updateDetector")
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
        todo!("port MSMoveReminder::notifyMoveInternal")
    }

    /// C++: `void MSMoveReminder::setDescription(const std::string &)`
    // void setDescription(const std::string& description) {
    //         myDescription = description;
    //     }
    #[allow(unused_variables)]
    pub fn set_description(&mut self, description: &str) {
        todo!("port MSMoveReminder::setDescription")
    }

    /// C++: `const std::string & MSMoveReminder::getDescription() const` [const]
    // const std::string& getDescription() const {
    //         return myDescription;
    //     }
    #[allow(unused_variables)]
    pub fn get_description(&self) -> &str {
        todo!("port MSMoveReminder::getDescription")
    }

    /// C++: `bool MSMoveReminder::isParkingRerouter() const` [const, virtual]
    // virtual bool isParkingRerouter() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_parking_rerouter(&self) -> bool {
        todo!("port MSMoveReminder::isParkingRerouter")
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
        todo!("port MSMoveReminder::saveReminderState")
    }

    /// C++: `void MSMoveReminder::loadReminderState(long long, SUMOTime, double)`
    // void
    // MSMoveReminder::loadReminderState(long long int numID, SUMOTime time, double pos) {
    //     myLastVehicleUpdateValues[numID] = std::make_pair(time, pos);
    // }
    #[allow(unused_variables)]
    pub fn load_reminder_state(&mut self, numID: i64, time: SUMOTime, pos: f64) {
        todo!("port MSMoveReminder::loadReminderState")
    }

    /// C++: `void MSMoveReminder::removeFromVehicleUpdateValues(SUMOTrafficObject &)` [protected]
    // void
    // MSMoveReminder::removeFromVehicleUpdateValues(SUMOTrafficObject& veh) {
    //     myLastVehicleUpdateValues.erase(veh.getNumericalID());
    // }
    #[allow(unused_variables)]
    pub fn remove_from_vehicle_update_values(&mut self, veh: &mut SUMOTrafficObject) {
        todo!("port MSMoveReminder::removeFromVehicleUpdateValues")
    }

    /// C++: `MSMoveReminder & MSMoveReminder::operator=(const MSMoveReminder &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSMoveReminder) -> &mut MSMoveReminder {
        todo!("port MSMoveReminder::operator=")
    }

}

