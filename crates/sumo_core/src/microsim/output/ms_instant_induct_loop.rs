//! Port of microsim/output/MSInstantInductLoop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::common::to_string::f64_to_string_default;
use crate::prelude::*;
use super::ms_detector_file_output::{DEFAULT_MSDETECTOR_FILE_OUTPUT_VT, MSDetectorFileOutputVt};

/// C++ macro `SIMTIME`: STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep())
fn simtime() -> f64 {
    steps2time(MSNet::get_instance().get_current_time_step())
}

/// An instantaneous induction loop
/// C++ class `MSInstantInductLoop` (microsim/output/MSInstantInductLoop.h)
#[repr(C)]
pub struct MSInstantInductLoop {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// C++ base class `MSDetectorFileOutput`
    pub base2: MSDetectorFileOutput,
    /// name
    pub myName: String,
    /// The output device to use
    pub myOutputDevice: Ptr<OutputDevice>,
    /// Detector's position on lane [m]
    pub myPosition: f64,
    /// The last exit time
    pub myLastExitTime: f64,
    /// The last exit time
    pub myEntryTimes: BTreeMap<Ptr<SUMOTrafficObject>, f64>,
}

impl std::ops::Deref for MSInstantInductLoop {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSInstantInductLoop {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

fn ms_instant_induct_loop_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSInstantInductLoop>().m().notify_enter(veh, reason, enteredLane)
}
fn ms_instant_induct_loop_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    Ptr::from_mut(mr).cast::<MSInstantInductLoop>().m().notify_move(veh, oldPos, newPos, newSpeed)
}
fn ms_instant_induct_loop_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSInstantInductLoop>().m().notify_leave(veh, lastPos, reason, enteredLane)
}

static MS_INSTANT_INDUCT_LOOP_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_instant_induct_loop_notify_enter,
    notify_move: ms_instant_induct_loop_notify_move,
    notify_leave: ms_instant_induct_loop_notify_leave,
};

// the MSDetectorFileOutput base is base2, not the first field: step back to the enclosing loop
fn ms_instant_induct_loop_of(d: *mut MSDetectorFileOutput) -> *mut MSInstantInductLoop {
    (d as *mut u8).wrapping_sub(std::mem::offset_of!(MSInstantInductLoop, base2)) as *mut MSInstantInductLoop
}
fn ms_instant_induct_loop_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    unsafe { (*ms_instant_induct_loop_of(d)).write_xml_output(dev, startTime, stopTime) }
}
fn ms_instant_induct_loop_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    unsafe { (*ms_instant_induct_loop_of(d as *const MSDetectorFileOutput as *mut MSDetectorFileOutput)).write_xml_detector_prolog(dev) }
}

static MS_INSTANT_INDUCT_LOOP_DETECTOR_FILE_OUTPUT_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: ms_instant_induct_loop_write_xml_output,
    write_xml_detector_prolog: ms_instant_induct_loop_write_xml_detector_prolog,
    ..DEFAULT_MSDETECTOR_FILE_OUTPUT_VT
};

impl MSInstantInductLoop {
    /// C++: `void MSInstantInductLoop::MSInstantInductLoop(const std::string &, OutputDevice &, MSLane *const, double, const std::string, const std::string &, const std::string &, int)`
    /// Constructor.
    // MSInstantInductLoop::MSInstantInductLoop(const std::string& id,
    //         OutputDevice& od, MSLane* const lane, double positionInMeters,
    //         const std::string name, const std::string& vTypes,
    //         const std::string& nextEdges,
    //         int detectPersons) :
    //     MSMoveReminder(id, lane),
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myName(name),
    //     myOutputDevice(od),
    //     myPosition(positionInMeters), myLastExitTime(-1) {
    //     assert(myPosition >= 0 && myPosition <= myLane->getLength());
    //     writeXMLDetectorProlog(od);
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, od: &mut OutputDevice, lane: Ptr<MSLane>, positionInMeters: f64, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32) -> Self {
        // shortcut: C++ MSMoveReminder(id, lane) registers this reminder with the lane (doAdd); new() returns by value, so the caller must call MSLane::add_move_reminder after placing it
        let mut this = MSInstantInductLoop {
            base: MSMoveReminder::new(id, lane, true),
            base2: MSDetectorFileOutput::new(id, vTypes, nextEdges, detectPersons).unwrap(),
            myName: name,
            myOutputDevice: Ptr::from_mut(od),
            myPosition: positionInMeters,
            myLastExitTime: -1.0,
            myEntryTimes: BTreeMap::new(),
        };
        this.base.vt = &MS_INSTANT_INDUCT_LOOP_MSMOVE_REMINDER_VT;
        this.base2.vt = &MS_INSTANT_INDUCT_LOOP_DETECTOR_FILE_OUTPUT_VT;
        assert!(positionInMeters >= 0.0 && positionInMeters <= lane.get_length());
        this.write_xml_detector_prolog(od);
        this
    }

    /// C++: `void MSInstantInductLoop::~MSInstantInductLoop() noexcept` [virtual]
    /// Destructor
    // MSInstantInductLoop::~MSInstantInductLoop() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `bool MSInstantInductLoop::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Checks whether the reminder is activated by a vehicle entering the lane
    // bool
    // MSInstantInductLoop::notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* /*enteredLane*/) {
    //     // vehicles must be kept if the "inductionloop" wants to detect passeengers
    //     if (!vehicleApplies(veh)) {
    //         return false;
    //     }
    //     if (reason != NOTIFICATION_JUNCTION) { // the junction case is handled in notifyMove
    //         if (veh.getBackPositionOnLane(myLane) >= myPosition) {
    //             return false;
    //         }
    //         if (veh.getPositionOnLane() >= myPosition) {
    //             double entryTime = SIMTIME;
    //             if (myLastExitTime >= 0) {
    //                 write("enter", entryTime, veh, veh.getSpeed(), "gap", entryTime - myLastExitTime);
    //             } else {
    //                 write("enter", entryTime, veh, veh.getSpeed());
    //             }
    //             myEntryTimes[&veh] = entryTime;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        // vehicles must be kept if the "inductionloop" wants to detect passeengers
        if !self.base2.vehicle_applies(veh) {
            return false;
        }
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            // the junction case is handled in notifyMove
            if veh.get_back_position_on_lane(self.base.myLane) >= self.myPosition {
                return false;
            }
            if veh.get_position_on_lane() >= self.myPosition {
                let entryTime = simtime();
                let speed = veh.get_speed();
                if self.myLastExitTime >= 0.0 {
                    self.write("enter", entryTime, veh, speed, "gap", entryTime - self.myLastExitTime);
                } else {
                    self.write("enter", entryTime, veh, speed, "", 0.0);
                }
                self.myEntryTimes.insert(Ptr::from_mut(veh), entryTime);
            }
        }
        true
    }

    /// C++: `bool MSInstantInductLoop::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the vehicle shall be counted and/or shall still touch this MSMoveReminder
    // bool
    // MSInstantInductLoop::notifyMove(SUMOTrafficObject& veh, double oldPos,
    //                                 double newPos, double newSpeed) {
    //     if (!vehicleApplies(veh)) {
    //         return false;
    //     }
    //     if (newPos < myPosition) {
    //         // detector not reached yet
    //         return true;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //
    //     const double oldSpeed = veh.getPreviousSpeed();
    //     double enterSpeed = MSGlobals::gSemiImplicitEulerUpdate ? newSpeed : oldSpeed; // NOTE: For the euler update, the vehicle is assumed to travel at constant speed for the whole time step
    //
    //     if (newPos >= myPosition && oldPos < myPosition/* && static_cast<MSVehicle&>(veh).getLane() == myLane*/) {
    //         const double timeBeforeEnter = MSCFModel::passingTime(oldPos, myPosition, newPos, oldSpeed, newSpeed);
    //         const double entryTime = SIMTIME - TS + timeBeforeEnter;
    //         enterSpeed = MSCFModel::speedAfterTime(timeBeforeEnter, oldSpeed, newPos - oldPos);
    //         if (myLastExitTime >= 0) {
    //             write("enter", entryTime, veh, enterSpeed, "gap", entryTime - myLastExitTime);
    //         } else {
    //             write("enter", entryTime, veh, enterSpeed);
    //         }
    //         myEntryTimes[&veh] = entryTime;
    //     }
    //     const double newBackPos = newPos - veh.getVehicleType().getLength();
    //     const double oldBackPos = oldPos - veh.getVehicleType().getLength();
    //     if (newBackPos > myPosition) {
    //         std::map<SUMOTrafficObject*, double>::iterator i = myEntryTimes.find(&veh);
    //         if (i != myEntryTimes.end()) {
    //             // vehicle passed the detector
    //             const double timeBeforeLeave = MSCFModel::passingTime(oldBackPos, myPosition, newBackPos, oldSpeed, newSpeed);
    //             const double leaveTime = SIMTIME - TS + timeBeforeLeave;
    //             write("leave", leaveTime, veh, newSpeed, "occupancy", leaveTime - (*i).second);
    //             myEntryTimes.erase(i);
    //             myLastExitTime = leaveTime;
    //         }
    //         return false;
    //     }
    //     // vehicle stays on the detector
    //     write("stay", SIMTIME, veh, newSpeed);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if !self.base2.vehicle_applies(veh) {
            return false;
        }
        if newPos < self.myPosition {
            // detector not reached yet
            return true;
        }
        // C++ ScopedLocker on myNotificationMutex: only relevant with HAVE_FOX and multiple sim threads

        let oldSpeed = veh.get_previous_speed();
        // NOTE: For the euler update, the vehicle is assumed to travel at constant speed for the whole time step
        let mut enterSpeed = if MSGlobals::semi_implicit_euler_update() { newSpeed } else { oldSpeed };

        if newPos >= self.myPosition && oldPos < self.myPosition {
            let timeBeforeEnter = MSCFModel::passing_time(oldPos, self.myPosition, newPos, oldSpeed, newSpeed);
            let entryTime = simtime() - ts() + timeBeforeEnter;
            enterSpeed = MSCFModel::speed_after_time(timeBeforeEnter, oldSpeed, newPos - oldPos);
            if self.myLastExitTime >= 0.0 {
                self.write("enter", entryTime, veh, enterSpeed, "gap", entryTime - self.myLastExitTime);
            } else {
                self.write("enter", entryTime, veh, enterSpeed, "", 0.0);
            }
            self.myEntryTimes.insert(Ptr::from_mut(veh), entryTime);
        }
        let newBackPos = newPos - veh.get_vehicle_type().get_length();
        let oldBackPos = oldPos - veh.get_vehicle_type().get_length();
        if newBackPos > self.myPosition {
            let key = Ptr::from_mut(veh);
            if let Some(entry) = self.myEntryTimes.get(&key).copied() {
                // vehicle passed the detector
                let timeBeforeLeave = MSCFModel::passing_time(oldBackPos, self.myPosition, newBackPos, oldSpeed, newSpeed);
                let leaveTime = simtime() - ts() + timeBeforeLeave;
                self.write("leave", leaveTime, veh, newSpeed, "occupancy", leaveTime - entry);
                self.myEntryTimes.remove(&key);
                self.myLastExitTime = leaveTime;
            }
            return false;
        }
        // vehicle stays on the detector
        self.write("stay", simtime(), veh, newSpeed, "", 0.0);
        true
    }

    /// C++: `bool MSInstantInductLoop::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Dismisses the vehicle if it is on the detector due to a lane change
    // bool
    // MSInstantInductLoop::notifyLeave(SUMOTrafficObject& veh, double /* lastPos */, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason == MSMoveReminder::NOTIFICATION_JUNCTION) {
    //         // vehicle might have jumped over detector at the end of the lane. we need
    //         // one more notifyMove to register it
    //         return true;
    //     }
    //     std::map<SUMOTrafficObject*, double>::iterator i = myEntryTimes.find(&veh);
    //     if (i != myEntryTimes.end()) {
    //         write("leave", SIMTIME, veh, veh.getSpeed());
    //         myEntryTimes.erase(i);
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            // vehicle might have jumped over detector at the end of the lane. we need
            // one more notifyMove to register it
            return true;
        }
        let key = Ptr::from_mut(veh);
        if self.myEntryTimes.contains_key(&key) {
            let speed = veh.get_speed();
            self.write("leave", simtime(), veh, speed, "", 0.0);
            self.myEntryTimes.remove(&key);
        }
        false
    }

    /// C++: `void MSInstantInductLoop::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Write the generated output to the given device
    // void writeXMLOutput(OutputDevice& dev,
    //                         SUMOTime startTime, SUMOTime stopTime) {
    //         UNUSED_PARAMETER(dev);
    //         UNUSED_PARAMETER(startTime);
    //         UNUSED_PARAMETER(stopTime);
    //     }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        // C++ body is empty (UNUSED_PARAMETER only)
    }

    /// C++: `void MSInstantInductLoop::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Open the XML-output
    // void
    // MSInstantInductLoop::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("instantE1", "instant_e1_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("instantE1", "instant_e1_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `void MSInstantInductLoop::write(const char *, double, SUMOTrafficObject &, double, const char *, double)` [protected]
    /// Writes an event line
    // void
    // MSInstantInductLoop::write(const char* state, double t, SUMOTrafficObject& veh, double speed, const char* add, double addValue) {
    //     if (!myOutputDevice.isNull()) {
    //         myOutputDevice.openTag("instantOut").writeAttr(
    //             "id", getID()).writeAttr("time", toString(t)).writeAttr("state", state).writeAttr(
    //                 "vehID", veh.getID()).writeAttr("speed", toString(speed)).writeAttr(
    //                     "length", toString(veh.getVehicleType().getLength())).writeAttr(
    //                         "type", veh.getVehicleType().getID());
    //         if (add != nullptr) {
    //             myOutputDevice.writeAttr(add, toString(addValue));
    //         }
    //         myOutputDevice.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write(&mut self, state: &str, t: f64, veh: &mut SUMOTrafficObject, speed: f64, add: &str, addValue: f64) {
        // an empty `add` stands for C++ add == nullptr (the 4-argument overload)
        if !self.myOutputDevice.is_null() {
            let dev = self.myOutputDevice.m();
            dev.open_tag("instantOut")
                .write_attr("id", self.base2.base.get_id())
                .write_attr("time", &f64_to_string_default(t))
                .write_attr("state", state)
                .write_attr("vehID", veh.get_id())
                .write_attr("speed", &f64_to_string_default(speed))
                .write_attr("length", &f64_to_string_default(veh.get_vehicle_type().get_length()))
                .write_attr("type", veh.get_vehicle_type().get_id());
            if !add.is_empty() {
                dev.write_attr(add, &f64_to_string_default(addValue));
            }
            dev.close_tag("");
        }
    }

    /// C++: `void MSInstantInductLoop::MSInstantInductLoop(const MSInstantInductLoop &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInstantInductLoop) -> Self {
        panic!("MSInstantInductLoop copy constructor is declared private and never defined in C++")
    }

    /// C++: `MSInstantInductLoop & MSInstantInductLoop::operator=(const MSInstantInductLoop &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInstantInductLoop) -> &mut MSInstantInductLoop {
        panic!("MSInstantInductLoop assignment operator is declared private and never defined in C++")
    }

}

