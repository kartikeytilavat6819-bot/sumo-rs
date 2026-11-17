//! Port of microsim/output/MSE3Collector.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::ms_detector_file_output::MSDetectorFileOutputVt;

/// C++ `MSPModel::FORWARD`
const MSP_FORWARD: i32 = 1;

fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
/// C++ macro `SIMTIME`
fn simtime() -> f64 {
    steps2time(MSNet::get_instance().get_current_time_step())
}
fn current_step() -> SUMOTime {
    MSNet::get_instance().get_current_time_step()
}
/// C++ `getCategory()` of the traffic object kinds seen by E3 detectors
// shortcut: SUMOTrafficObject::get_category is still todo!() in utils, so the category is derived from the kind flags, upgrade when it is ported
fn category_of(veh: &SUMOTrafficObject) -> &'static str {
    if veh.is_person() {
        "person"
    } else if veh.is_vehicle() {
        "vehicle"
    } else {
        "container"
    }
}
/// The port has no Clone for MSCrossSection; copy the list field by field
fn copy_cross_sections(v: &CrossSectionVector) -> CrossSectionVector {
    v.iter().map(|c| MSCrossSection::new(c.myLane, c.myPosition)).collect()
}

/// Internal storage for values from a vehicle
/// C++ class `MSE3Collector::E3Values` (microsim/output/MSE3Collector.h)
#[derive(Clone, Copy)]
pub struct MSE3Collector_E3Values {
    /// The vehicle's entry time
    pub entryTime: f64,
    /// The time the vehicle's front was crossing the leave line
    pub frontLeaveTime: f64,
    /// The time the vehicle's back was crossing the leave line
    pub backLeaveTime: f64,
    /// The sum of registered speeds the vehicle has/had inside the area
    pub speedSum: f64,
    /// The sum of haltings the vehicle has/had within the area
    pub haltings: i32,
    /// Begin time of last halt begin
    pub haltingBegin: SUMOTime,
    /// The sum of registered speeds the vehicle has/had inside the area during the current interval
    pub intervalSpeedSum: f64,
    /// The sum of haltings the vehicle has/had within the area during the current interval
    pub intervalHaltings: i32,
    /// The timeLoss of the vehicle when entering. Updated to the actual time loss within the area when leaving
    pub timeLoss: SUMOTime,
    /// The timeLoss of the vehicle when entering. Updated to the current timeLoss at interval write
    pub intervalTimeLoss: SUMOTime,
    /// An internal information whether the update step was performed
    pub hadUpdate: bool,
    /// the reminder on which the vehicle entered the detector
    pub entryReminder: Ptr<MSE3Collector_MSE3EntryReminder>,
}

impl MSE3Collector_E3Values {
}

/// A detector of vehicles passing an area between entry/exit points
/// C++ class `MSE3Collector` (microsim/output/MSE3Collector.h)
#[repr(C)]
pub struct MSE3Collector {
    /// C++ base class `MSDetectorFileOutput`
    pub base: MSDetectorFileOutput,
    /// name
    pub myName: String,
    /// The detector's entrys
    pub myEntries: CrossSectionVector,
    /// The detector's exits
    pub myExits: CrossSectionVector,
    /// The detector's built entry reminder
    pub myEntryReminders: Vec<Ptr<MSE3Collector_MSE3EntryReminder>>,
    /// The detector's built exit reminder
    pub myLeaveReminders: Vec<Ptr<MSE3Collector_MSE3LeaveReminder>>,
    pub myHaltingTimeThreshold: SUMOTime,
    /// Speed-threshold to determine if a vehicle is halting.
    pub myHaltingSpeedThreshold: f64,
    /// Container for vehicles that have entered the area
    pub myEnteredContainer: BTreeMap<Ptr<SUMOTrafficObject>, MSE3Collector_E3Values>,
    /// Container for vehicles that have left the area
    pub myLeftContainer: Vec<MSE3Collector_E3Values>,
    /// The current mean speed of known vehicles (inside)
    pub myCurrentMeanSpeed: f64,
    /// The current number of haltings (inside)
    pub myCurrentHaltingsNumber: i32,
    /// @{
    pub myLastMeanTravelTime: f64,
    pub myLastMeanHaltsPerVehicle: f64,
    pub myLastMeanTimeLoss: f64,
    pub myLastVehicleSum: i32,
    /// Information when the last reset has been done
    pub myLastResetTime: SUMOTime,
    /// whether this dector is declared as having incomplete entry detectors
    pub myOpenEntry: bool,
    /// Whether the detector expects vehicles to arrive inside (and doesn't issue a warning in this case)
    pub myExpectArrival: bool,
}

impl std::ops::Deref for MSE3Collector {
    type Target = MSDetectorFileOutput;
    fn deref(&self) -> &MSDetectorFileOutput {
        &self.base
    }
}
impl std::ops::DerefMut for MSE3Collector {
    fn deref_mut(&mut self) -> &mut MSDetectorFileOutput {
        &mut self.base
    }
}

fn mse3_entry_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3EntryReminder>().m().notify_enter(veh, reason, enteredLane)
}
fn mse3_entry_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3EntryReminder>().m().notify_move(veh, oldPos, newPos, newSpeed)
}
fn mse3_entry_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3EntryReminder>().m().notify_leave(veh, lastPos, reason, enteredLane)
}
static MSE3_ENTRY_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: mse3_entry_notify_enter,
    notify_move: mse3_entry_notify_move,
    notify_leave: mse3_entry_notify_leave,
};

fn mse3_leave_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3LeaveReminder>().m().notify_enter(veh, reason, enteredLane)
}
fn mse3_leave_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3LeaveReminder>().m().notify_move(veh, oldPos, newPos, newSpeed)
}
fn mse3_leave_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    Ptr::from_mut(mr).cast::<MSE3Collector_MSE3LeaveReminder>().m().notify_leave(veh, lastPos, reason, enteredLane)
}
static MSE3_LEAVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: mse3_leave_notify_enter,
    notify_move: mse3_leave_notify_move,
    notify_leave: mse3_leave_notify_leave,
};

// the MSDetectorFileOutput base is the first field here, so the collector pointer is the base pointer
fn mse3_of(d: *mut MSDetectorFileOutput) -> *mut MSE3Collector {
    (d as *mut u8).wrapping_sub(std::mem::offset_of!(MSE3Collector, base)) as *mut MSE3Collector
}
fn mse3_write_xml_output(d: &mut MSDetectorFileOutput, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
    unsafe { (*mse3_of(d)).write_xml_output(dev, startTime, stopTime) }
}
fn mse3_write_xml_detector_prolog(d: &MSDetectorFileOutput, dev: &mut OutputDevice) {
    unsafe { (*mse3_of(d as *const MSDetectorFileOutput as *mut MSDetectorFileOutput)).write_xml_detector_prolog(dev) }
}
fn mse3_reset(d: &mut MSDetectorFileOutput) {
    unsafe { (*mse3_of(d)).reset() }
}
fn mse3_detector_update(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    unsafe { (*mse3_of(d)).detector_update(step) }
}
fn mse3_clear_state(d: &mut MSDetectorFileOutput, step: SUMOTime) {
    unsafe { (*mse3_of(d)).clear_state(step) }
}

static MSE3_DETECTOR_FILE_OUTPUT_VT: MSDetectorFileOutputVt = MSDetectorFileOutputVt {
    write_xml_output: mse3_write_xml_output,
    write_xml_detector_prolog: mse3_write_xml_detector_prolog,
    reset: mse3_reset,
    detector_update: mse3_detector_update,
    clear_state: mse3_clear_state,
};

impl MSE3Collector {
    /// C++: `void MSE3Collector::MSE3Collector(const std::string &, const CrossSectionVector &, const CrossSectionVector &, double, SUMOTime, const std::string, const std::string &, const std::string &, int, bool, bool)`
    /// Constructor
    // MSE3Collector::MSE3Collector(const std::string& id,
    //                              const CrossSectionVector& entries,
    //                              const CrossSectionVector& exits,
    //                              double haltingSpeedThreshold,
    //                              SUMOTime haltingTimeThreshold,
    //                              const std::string name, const std::string& vTypes,
    //                              const std::string& nextEdges,
    //                              int detectPersons,
    //                              bool openEntry, bool expectArrival) :
    //     MSDetectorFileOutput(id, vTypes, nextEdges, detectPersons),
    //     myName(name),
    //     myEntries(entries),
    //     myExits(exits),
    //     myHaltingTimeThreshold(haltingTimeThreshold), myHaltingSpeedThreshold(haltingSpeedThreshold),
    //     myCurrentMeanSpeed(0), myCurrentHaltingsNumber(0),
    //     myLastMeanTravelTime(0), myLastMeanHaltsPerVehicle(0), myLastMeanTimeLoss(0), myLastVehicleSum(0),
    //     myLastResetTime(-1), myOpenEntry(openEntry), myExpectArrival(expectArrival) {
    //     // Set MoveReminders to entries and exits
    //     for (CrossSectionVectorConstIt crossSec1 = entries.begin(); crossSec1 != entries.end(); ++crossSec1) {
    //         myEntryReminders.push_back(new MSE3EntryReminder(*crossSec1, *this));
    //     }
    //     for (CrossSectionVectorConstIt crossSec2 = exits.begin(); crossSec2 != exits.end(); ++crossSec2) {
    //         myLeaveReminders.push_back(new MSE3LeaveReminder(*crossSec2, *this));
    //     }
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, entries: &CrossSectionVector, exits: &CrossSectionVector, haltingSpeedThreshold: f64, haltingTimeThreshold: SUMOTime, name: String, vTypes: &str, nextEdges: &str, detectPersons: i32, openEntry: bool, expectArrival: bool) -> Self {
        let mut s = MSE3Collector {
            base: MSDetectorFileOutput::new(id, vTypes, nextEdges, detectPersons).unwrap_or_else(|e| panic!("{e}")),
            myName: name,
            myEntries: copy_cross_sections(entries),
            myExits: copy_cross_sections(exits),
            myEntryReminders: Vec::new(),
            myLeaveReminders: Vec::new(),
            myHaltingTimeThreshold: haltingTimeThreshold,
            myHaltingSpeedThreshold: haltingSpeedThreshold,
            myEnteredContainer: BTreeMap::new(),
            myLeftContainer: Vec::new(),
            myCurrentMeanSpeed: 0.,
            myCurrentHaltingsNumber: 0,
            myLastMeanTravelTime: 0.,
            myLastMeanHaltsPerVehicle: 0.,
            myLastMeanTimeLoss: 0.,
            myLastVehicleSum: 0,
            myLastResetTime: -1,
            myOpenEntry: openEntry,
            myExpectArrival: expectArrival,
        };
        s.base.vt = &MSE3_DETECTOR_FILE_OUTPUT_VT;
        s.reset();
        s
    }

    /// C++: `void MSE3Collector::~MSE3Collector() noexcept` [virtual]
    /// Destructor
    // MSE3Collector::~MSE3Collector() {
    //     for (std::vector<MSE3EntryReminder*>::iterator i = myEntryReminders.begin(); i != myEntryReminders.end(); ++i) {
    //         delete *i;
    //     }
    //     for (std::vector<MSE3LeaveReminder*>::iterator i = myLeaveReminders.begin(); i != myLeaveReminders.end(); ++i) {
    //         delete *i;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for r in self.myEntryReminders.drain(..) {
            // C++ `delete *i`
            unsafe { drop(Box::from_raw(r.as_raw())) };
        }
        for r in self.myLeaveReminders.drain(..) {
            // C++ `delete *i`
            unsafe { drop(Box::from_raw(r.as_raw())) };
        }
    }

    /// Creates and registers the entry and exit reminders, owned by `this`. Callers must invoke this once the collector is boxed.
    // shortcut: C++ creates these in the constructor, but a by-value Rust constructor has no stable address for the back-pointer, upgrade when constructors return Box<Self>
    pub fn build_reminders(&mut self, this: Ptr<MSE3Collector>) {
        for cs in self.myEntries.iter() {
            let rem = Ptr::from_raw(Box::into_raw(Box::new(MSE3Collector_MSE3EntryReminder::new(cs, this.m()))));
            rem.m().base.vt = &MSE3_ENTRY_REMINDER_VT;
            cs.myLane.m().add_move_reminder(rem.cast::<MSMoveReminder>(), true);
            self.myEntryReminders.push(rem);
        }
        for cs in self.myExits.iter() {
            let rem = Ptr::from_raw(Box::into_raw(Box::new(MSE3Collector_MSE3LeaveReminder::new(cs, this.m()))));
            rem.m().base.vt = &MSE3_LEAVE_REMINDER_VT;
            cs.myLane.m().add_move_reminder(rem.cast::<MSMoveReminder>(), true);
            self.myLeaveReminders.push(rem);
        }
    }

    /// C++: `void MSE3Collector::reset()` [virtual]
    /// Resets all generated values to allow computation of next interval
    // void
    // MSE3Collector::reset() {
    //     myLeftContainer.clear();
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.myLeftContainer.clear();
    }

    /// C++: `void MSE3Collector::enter(const SUMOTrafficObject &, const double, const double, MSE3EntryReminder *, bool)`
    /// Called if a vehicle touches an entry-cross-section.
    // void
    // MSE3Collector::enter(const SUMOTrafficObject& veh, const double entryTimestep, const double fractionTimeOnDet, MSE3EntryReminder* entryReminder, bool isBackward) {
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         const MSBaseVehicle& v = dynamic_cast<const MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             enter(*p, entryTimestep, fractionTimeOnDet, entryReminder);
    //         }
    //         return;
    //     }
    //     if (!vehicleApplies(veh)) {
    //         return;
    //     }
    //     if (veh.isPerson() && !isBackward && dynamic_cast<const MSTransportable&>(veh).getDirection() != MSPModel::FORWARD) {
    //         // walking backward over an entry detector means "leaving"
    //         // std::cout << veh.getID() << " leave at entryDetector\n";
    //         leave(veh, entryTimestep, fractionTimeOnDet, true);
    //         return;
    //     }
    //     if (myEnteredContainer.find(&veh) != myEnteredContainer.end()) {
    //         WRITE_WARNINGF("% '%' reentered % '%', time=%.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), getID(), time2string(SIMSTEP));
    //         return;
    //     }
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //     std::cout << veh.getID() << " enters\n";
    // #endif
    //     const double speedFraction = veh.getSpeed() * fractionTimeOnDet;
    //     E3Values v;
    //     v.entryTime = entryTimestep;
    //     v.frontLeaveTime = 0;
    //     v.backLeaveTime = 0;
    //     v.speedSum = speedFraction;
    //     v.haltingBegin = veh.getSpeed() < myHaltingSpeedThreshold ? TIME2STEPS(entryTimestep) : -1;
    //     v.intervalSpeedSum = entryTimestep >= STEPS2TIME(myLastResetTime) ? speedFraction : 0;
    //     v.haltings = 0;
    //     v.intervalHaltings = 0;
    //     if (veh.getSpeed() < myHaltingSpeedThreshold) {
    //         if (TIME2STEPS(fractionTimeOnDet) > myHaltingTimeThreshold) {
    //             v.haltings++;
    //             v.intervalHaltings++;
    //         }
    //     }
    //     v.hadUpdate = false;
    //     if (!MSGlobals::gUseMesoSim && veh.isVehicle()) {
    //         v.timeLoss = dynamic_cast<const MSVehicle&>(veh).getTimeLoss();
    //         v.intervalTimeLoss = v.timeLoss;
    //     }
    //     v.entryReminder = entryReminder;
    //     myEnteredContainer[&veh] = v;
    // }
    #[allow(unused_variables)]
    pub fn enter(&mut self, veh: &SUMOTrafficObject, entryTimestep: f64, fractionTimeOnDet: f64, entryReminder: Ptr<MSE3Collector_MSE3EntryReminder>, isBackward: bool) {
        if self.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            // shortcut: C++ forwards to each person on the vehicle (dynamic_cast<const MSBaseVehicle&>), MSBaseVehicle is not #[repr(C)] so the cast is unavailable; persons riding in vehicles are skipped, upgrade when ms_base_vehicle.rs is #[repr(C)]
            return;
        }
        if !self.vehicle_applies(veh) {
            return;
        }
        // shortcut: C++ treats a person walking backwards over an entry detector as leaving; MSTransportable is not #[repr(C)] so its direction is not read here, upgrade when ms_transportable.rs is #[repr(C)]
        let key = Ptr::new(veh);
        if self.myEnteredContainer.contains_key(&key) {
            write_warningf!("% '%' reentered % '%', time=%.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), self.base.base.get_id(), time2string(current_step()));
            return;
        }
        let speed_fraction = veh.get_speed() * fractionTimeOnDet;
        let mut v = MSE3Collector_E3Values {
            entryTime: entryTimestep,
            frontLeaveTime: 0.,
            backLeaveTime: 0.,
            speedSum: speed_fraction,
            haltings: 0,
            haltingBegin: if veh.get_speed() < self.myHaltingSpeedThreshold { time2steps(entryTimestep) } else { -1 },
            intervalSpeedSum: if entryTimestep >= steps2time(self.myLastResetTime) { speed_fraction } else { 0. },
            intervalHaltings: 0,
            timeLoss: 0,
            intervalTimeLoss: 0,
            hadUpdate: false,
            entryReminder,
        };
        if veh.get_speed() < self.myHaltingSpeedThreshold && time2steps(fractionTimeOnDet) > self.myHaltingTimeThreshold {
            v.haltings += 1;
            v.intervalHaltings += 1;
        }
        // shortcut: C++ sets timeLoss from MSVehicle::getTimeLoss() for vehicles (dynamic_cast), MSVehicle is not #[repr(C)] so time loss stays 0, upgrade when ms_vehicle.rs is #[repr(C)]
        self.myEnteredContainer.insert(key, v);
    }

    /// C++: `void MSE3Collector::leaveFront(const SUMOTrafficObject &, const double)`
    /// Called if a vehicle front passes a leave-cross-section.
    // void
    // MSE3Collector::leaveFront(const SUMOTrafficObject& veh, const double leaveTimestep) {
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         const MSBaseVehicle& v = dynamic_cast<const MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             leaveFront(*p, leaveTimestep);
    //         }
    //         return;
    //     }
    //     if (!vehicleApplies(veh)) {
    //         return;
    //     }
    //     if (myEnteredContainer.find(&veh) == myEnteredContainer.end()) {
    //         if (!myOpenEntry && veh.isVehicle()) {
    //             WRITE_WARNINGF("% '%' left % '%' without entering it, time=%.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), getID(), time2string(SIMSTEP));
    //         }
    //     } else {
    //         myEnteredContainer[&veh].frontLeaveTime = leaveTimestep;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn leave_front(&mut self, veh: &SUMOTrafficObject, leaveTimestep: f64) {
        if self.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            // shortcut: C++ forwards to each person on the vehicle, see enter
            return;
        }
        if !self.vehicle_applies(veh) {
            return;
        }
        let key = Ptr::new(veh);
        if !self.myEnteredContainer.contains_key(&key) {
            if !self.myOpenEntry && veh.is_vehicle() {
                write_warningf!("% '%' left % '%' without entering it, time=%.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), self.base.base.get_id(), time2string(current_step()));
            }
        } else {
            self.myEnteredContainer.get_mut(&key).unwrap().frontLeaveTime = leaveTimestep;
        }
    }

    /// C++: `void MSE3Collector::leave(const SUMOTrafficObject &, const double, const double, bool)`
    /// Called if a vehicle back passes a leave-cross-section.
    // void
    // MSE3Collector::leave(const SUMOTrafficObject& veh, const double leaveTimestep, const double fractionTimeOnDet, bool isBackward) {
    //     if (myDetectPersons > (int)PersonMode::WALK && !veh.isPerson()) {
    //         const MSBaseVehicle& v = dynamic_cast<const MSBaseVehicle&>(veh);
    //         for (MSTransportable* p : v.getPersons()) {
    //             leave(*p, leaveTimestep, fractionTimeOnDet);
    //         }
    //         return;
    //     }
    //     if (!vehicleApplies(veh)) {
    //         return;
    //     }
    //     if (veh.isPerson() && !isBackward && dynamic_cast<const MSTransportable&>(veh).getDirection() != MSPModel::FORWARD) {
    //         // walking backward over an exit detector means "entering"
    //         // std::cout << veh.getID() << " enter at exitDetector\n";
    //         enter(veh, leaveTimestep, fractionTimeOnDet, nullptr, true);
    //         return;
    //     }
    //     if (myEnteredContainer.find(&veh) == myEnteredContainer.end()) {
    //         if (!myOpenEntry && veh.isVehicle()) {
    //             WRITE_WARNINGF("% '%' left % '%' without entering it, time=%.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), getID(), time2string(SIMSTEP));
    //         }
    //     } else {
    // #ifdef DEBUG_E3_NOTIFY_LEAVE
    //         std::cout << veh.getID() << " leaves\n";
    // #endif
    //         E3Values values = myEnteredContainer[&veh];
    //         values.backLeaveTime = leaveTimestep;
    //         const double speedFraction = veh.getSpeed() * (TS - fractionTimeOnDet);
    //         values.speedSum -= speedFraction;
    //         values.intervalSpeedSum -= speedFraction;
    //         if (MSGlobals::gUseMesoSim || !veh.isVehicle()) {
    //             // not yet supported
    //             values.timeLoss = 0;
    //             if (isBackward) {
    //                 // leaveFront may not have been called
    //                 values.frontLeaveTime = leaveTimestep;
    //             }
    //         } else {
    //             // timeLoss was initialized when entering
    //             values.timeLoss = dynamic_cast<const MSVehicle&>(veh).getTimeLoss() - values.timeLoss;
    //         }
    //         myEnteredContainer.erase(&veh);
    //         myLeftContainer.push_back(values);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn leave(&mut self, veh: &SUMOTrafficObject, leaveTimestep: f64, fractionTimeOnDet: f64, isBackward: bool) {
        if self.myDetectPersons > PersonMode::WALK as i32 && !veh.is_person() {
            // shortcut: C++ forwards to each person on the vehicle, see enter
            return;
        }
        if !self.vehicle_applies(veh) {
            return;
        }
        // shortcut: C++ handles a person walking backwards over an exit detector as entering, see enter
        let key = Ptr::new(veh);
        if !self.myEnteredContainer.contains_key(&key) {
            if !self.myOpenEntry && veh.is_vehicle() {
                write_warningf!("% '%' left % '%' without entering it, time=%.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), self.base.base.get_id(), time2string(current_step()));
            }
        } else {
            let mut values = self.myEnteredContainer[&key];
            values.backLeaveTime = leaveTimestep;
            let speed_fraction = veh.get_speed() * (ts() - fractionTimeOnDet);
            values.speedSum -= speed_fraction;
            values.intervalSpeedSum -= speed_fraction;
            if MSGlobals::use_meso_sim() || !veh.is_vehicle() {
                // not yet supported
                values.timeLoss = 0;
                if isBackward {
                    // leaveFront may not have been called
                    values.frontLeaveTime = leaveTimestep;
                }
            } else {
                // shortcut: C++ sets timeLoss = MSVehicle::getTimeLoss() - entry timeLoss, MSVehicle is not #[repr(C)] so it stays 0, upgrade when ms_vehicle.rs is #[repr(C)]
                values.timeLoss = 0;
            }
            self.myEnteredContainer.remove(&key);
            self.myLeftContainer.push(values);
        }
    }

    /// C++: `const CrossSectionVector & MSE3Collector::getEntries() const` [const]
    /// Returns the entry cross sections
    // const CrossSectionVector&
    // MSE3Collector::getEntries() const {
    //     return myEntries;
    // }
    #[allow(unused_variables)]
    pub fn get_entries(&self) -> &CrossSectionVector {
        &self.myEntries
    }

    /// C++: `const CrossSectionVector & MSE3Collector::getExits() const` [const]
    /// Returns the exit cross sections
    // const CrossSectionVector&
    // MSE3Collector::getExits() const {
    //     return myExits;
    // }
    #[allow(unused_variables)]
    pub fn get_exits(&self) -> &CrossSectionVector {
        &self.myExits
    }

    /// C++: `double MSE3Collector::getCurrentMeanSpeed() const` [const]
    /// Returns the mean speed within the area
    // double
    // MSE3Collector::getCurrentMeanSpeed() const {
    //     return myCurrentMeanSpeed;
    // }
    #[allow(unused_variables)]
    pub fn get_current_mean_speed(&self) -> f64 {
        self.myCurrentMeanSpeed
    }

    /// C++: `int MSE3Collector::getCurrentHaltingNumber() const` [const]
    /// Returns the number of current haltings within the area
    // int
    // MSE3Collector::getCurrentHaltingNumber() const {
    //     return myCurrentHaltingsNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_current_halting_number(&self) -> i32 {
        self.myCurrentHaltingsNumber
    }

    /// C++: `int MSE3Collector::getVehiclesWithin() const` [const]
    /// Returns the number of vehicles within the area
    // int
    // MSE3Collector::getVehiclesWithin() const {
    //     return (int) myEnteredContainer.size();
    // }
    #[allow(unused_variables)]
    pub fn get_vehicles_within(&self) -> i32 {
        self.myEnteredContainer.len() as i32
    }

    /// C++: `std::vector<std::string> MSE3Collector::getCurrentVehicleIDs() const` [const]
    /// Returns the number of vehicles within the area
    // std::vector<std::string>
    // MSE3Collector::getCurrentVehicleIDs() const {
    //     std::vector<std::string> ret;
    //     for (std::map<const SUMOTrafficObject*, E3Values>::const_iterator pair = myEnteredContainer.begin(); pair != myEnteredContainer.end(); ++pair) {
    //         ret.push_back((*pair).first->getID());
    //     }
    //     std::sort(ret.begin(), ret.end());
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_current_vehicle_i_ds(&self) -> Vec<String> {
        let mut ret: Vec<String> = self.myEnteredContainer.keys().map(|v| v.base.get_id().to_string()).collect();
        ret.sort();
        ret
    }

    /// C++: `double MSE3Collector::getLastIntervalMeanTravelTime() const` [const]
    /// @{
    // double getLastIntervalMeanTravelTime() const {
    //         return myLastMeanTravelTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_travel_time(&self) -> f64 {
        self.myLastMeanTravelTime
    }

    /// C++: `double MSE3Collector::getLastIntervalMeanHaltsPerVehicle() const` [const]
    // double getLastIntervalMeanHaltsPerVehicle() const {
    //         return myLastMeanHaltsPerVehicle;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_halts_per_vehicle(&self) -> f64 {
        self.myLastMeanHaltsPerVehicle
    }

    /// C++: `double MSE3Collector::getLastIntervalMeanTimeLoss() const` [const]
    // double getLastIntervalMeanTimeLoss() const {
    //         return myLastMeanTimeLoss;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_mean_time_loss(&self) -> f64 {
        self.myLastMeanTimeLoss
    }

    /// C++: `int MSE3Collector::getLastIntervalVehicleSum() const` [const]
    // int getLastIntervalVehicleSum() const {
    //         return myLastVehicleSum;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_interval_vehicle_sum(&self) -> i32 {
        self.myLastVehicleSum
    }

    /// C++: `void MSE3Collector::writeXMLOutput(OutputDevice &, SUMOTime, SUMOTime)` [virtual]
    /// Writes collected values into the given stream
    // void
    // MSE3Collector::writeXMLOutput(OutputDevice& dev,
    //                               SUMOTime startTime, SUMOTime stopTime) {
    //     dev.openTag(SUMO_TAG_INTERVAL).writeTime(SUMO_ATTR_BEGIN, startTime).writeTime(SUMO_ATTR_END, stopTime);
    //     dev.writeAttr(SUMO_ATTR_ID, myID);
    //     // collect values about vehicles that have left the area
    //     myLastVehicleSum = (int) myLeftContainer.size();
    //     myLastMeanTravelTime = 0;
    //     double meanOverlapTravelTime = 0.;
    //     double meanSpeed = 0.;
    //     myLastMeanHaltsPerVehicle = 0;
    //     myLastMeanTimeLoss = 0.;
    //     for (const E3Values& values : myLeftContainer) {
    //         myLastMeanHaltsPerVehicle += (double)values.haltings;
    //         myLastMeanTravelTime += values.frontLeaveTime - values.entryTime;
    //         const double steps = values.backLeaveTime - values.entryTime;
    //         meanOverlapTravelTime += steps;
    //         meanSpeed += (values.speedSum / steps);
    //         myLastMeanTimeLoss += STEPS2TIME(values.timeLoss);
    //     }
    //     myLastMeanTravelTime = myLastVehicleSum != 0 ? myLastMeanTravelTime / (double)myLastVehicleSum : -1;
    //     meanOverlapTravelTime = myLastVehicleSum != 0 ? meanOverlapTravelTime / (double)myLastVehicleSum : -1;
    //     meanSpeed = myLastVehicleSum != 0 ? meanSpeed / (double)myLastVehicleSum : -1;
    //     myLastMeanHaltsPerVehicle = myLastVehicleSum != 0 ? myLastMeanHaltsPerVehicle / (double) myLastVehicleSum : -1;
    //     myLastMeanTimeLoss = myLastVehicleSum != 0 ? myLastMeanTimeLoss / (double) myLastVehicleSum : -1;
    //     // clear container
    //     myLeftContainer.clear();
    //
    //     // collect values about vehicles within the container
    //     const int vehicleSumWithin = (int) myEnteredContainer.size();
    //     double meanSpeedWithin = 0.;
    //     double meanDurationWithin = 0.;
    //     double meanHaltsPerVehicleWithin = 0.;
    //     double meanIntervalSpeedWithin = 0.;
    //     double meanIntervalHaltsPerVehicleWithin = 0.;
    //     double meanIntervalDurationWithin = 0.;
    //     double meanTimeLossWithin = 0.;
    //     for (std::map<const SUMOTrafficObject*, E3Values>::iterator i = myEnteredContainer.begin(); i != myEnteredContainer.end(); ++i) {
    //         meanHaltsPerVehicleWithin += (double)(*i).second.haltings;
    //         meanIntervalHaltsPerVehicleWithin += (double)(*i).second.intervalHaltings;
    //         const double end = (*i).second.backLeaveTime == 0 ? STEPS2TIME(stopTime) : (*i).second.backLeaveTime;
    //         const double time = end - (*i).second.entryTime;
    //         const double timeWithin = MIN2(time, end - STEPS2TIME(startTime));
    //         if (i->second.speedSum > 0.) {
    //             meanSpeedWithin += i->second.speedSum / time;
    //         }
    //         if (i->second.intervalSpeedSum > 0.) {
    //             meanIntervalSpeedWithin += i->second.intervalSpeedSum / timeWithin;
    //         }
    //         meanDurationWithin += time;
    //         meanIntervalDurationWithin += timeWithin;
    //         // reset interval values
    //         (*i).second.intervalHaltings = 0;
    //         (*i).second.intervalSpeedSum = 0;
    //
    //         if (!MSGlobals::gUseMesoSim && i->first->isVehicle()) {
    //             const SUMOTime currentTimeLoss = dynamic_cast<const MSVehicle*>(i->first)->getTimeLoss();
    //             meanTimeLossWithin += STEPS2TIME(currentTimeLoss - (*i).second.intervalTimeLoss);
    //             (*i).second.intervalTimeLoss = currentTimeLoss;
    //         }
    //     }
    //     myLastResetTime = stopTime;
    //     meanSpeedWithin = vehicleSumWithin != 0 ?  meanSpeedWithin / (double) vehicleSumWithin : -1;
    //     meanHaltsPerVehicleWithin = vehicleSumWithin != 0 ? meanHaltsPerVehicleWithin / (double) vehicleSumWithin : -1;
    //     meanDurationWithin = vehicleSumWithin != 0 ? meanDurationWithin / (double) vehicleSumWithin : -1;
    //     meanIntervalSpeedWithin = vehicleSumWithin != 0 ?  meanIntervalSpeedWithin / (double) vehicleSumWithin : -1;
    //     meanIntervalHaltsPerVehicleWithin = vehicleSumWithin != 0 ? meanIntervalHaltsPerVehicleWithin / (double) vehicleSumWithin : -1;
    //     meanIntervalDurationWithin = vehicleSumWithin != 0 ? meanIntervalDurationWithin / (double) vehicleSumWithin : -1;
    //     meanTimeLossWithin = vehicleSumWithin != 0 ? meanTimeLossWithin / (double) vehicleSumWithin : -1;
    //
    //     // write values
    //     dev.writeAttr("meanTravelTime", myLastMeanTravelTime)
    //     .writeAttr("meanOverlapTravelTime", meanOverlapTravelTime)
    //     .writeAttr("meanSpeed", meanSpeed)
    //     .writeAttr("meanHaltsPerVehicle", myLastMeanHaltsPerVehicle)
    //     .writeAttr("meanTimeLoss", myLastMeanTimeLoss)
    //     .writeAttr("vehicleSum", myLastVehicleSum)
    //     .writeAttr("meanSpeedWithin", meanSpeedWithin)
    //     .writeAttr("meanHaltsPerVehicleWithin", meanHaltsPerVehicleWithin)
    //     .writeAttr("meanDurationWithin", meanDurationWithin)
    //     .writeAttr("vehicleSumWithin", vehicleSumWithin)
    //     .writeAttr("meanIntervalSpeedWithin", meanIntervalSpeedWithin)
    //     .writeAttr("meanIntervalHaltsPerVehicleWithin", meanIntervalHaltsPerVehicleWithin)
    //     .writeAttr("meanIntervalDurationWithin", meanIntervalDurationWithin)
    //     .writeAttr("meanTimeLossWithin", meanTimeLossWithin)
    //     .closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write_xml_output(&mut self, dev: &mut OutputDevice, startTime: SUMOTime, stopTime: SUMOTime) {
        dev.open_tag_id(SUMO_TAG_INTERVAL).write_time(SUMO_ATTR_BEGIN, startTime).write_time(SUMO_ATTR_END, stopTime);
        dev.write_attr(SUMO_ATTR_ID, self.base.base.get_id());
        // collect values about vehicles that have left the area
        self.myLastVehicleSum = self.myLeftContainer.len() as i32;
        self.myLastMeanTravelTime = 0.;
        let mut mean_overlap_travel_time = 0.;
        let mut mean_speed = 0.;
        self.myLastMeanHaltsPerVehicle = 0.;
        self.myLastMeanTimeLoss = 0.;
        for values in self.myLeftContainer.iter() {
            self.myLastMeanHaltsPerVehicle += values.haltings as f64;
            self.myLastMeanTravelTime += values.frontLeaveTime - values.entryTime;
            let steps = values.backLeaveTime - values.entryTime;
            mean_overlap_travel_time += steps;
            mean_speed += values.speedSum / steps;
            self.myLastMeanTimeLoss += steps2time(values.timeLoss);
        }
        let n = self.myLastVehicleSum;
        let nf = n as f64;
        self.myLastMeanTravelTime = if n != 0 { self.myLastMeanTravelTime / nf } else { -1. };
        mean_overlap_travel_time = if n != 0 { mean_overlap_travel_time / nf } else { -1. };
        mean_speed = if n != 0 { mean_speed / nf } else { -1. };
        self.myLastMeanHaltsPerVehicle = if n != 0 { self.myLastMeanHaltsPerVehicle / nf } else { -1. };
        self.myLastMeanTimeLoss = if n != 0 { self.myLastMeanTimeLoss / nf } else { -1. };
        // clear container
        self.myLeftContainer.clear();
        // collect values about vehicles within the container
        let vehicle_sum_within = self.myEnteredContainer.len() as i32;
        let mut mean_speed_within = 0.;
        let mut mean_duration_within = 0.;
        let mut mean_halts_within = 0.;
        let mut mean_interval_speed_within = 0.;
        let mut mean_interval_halts_within = 0.;
        let mut mean_interval_duration_within = 0.;
        // shortcut: time loss within the area (C++ MSVehicle::getTimeLoss differences) is not tracked, stays 0, upgrade when ms_vehicle.rs is #[repr(C)]
        let mean_time_loss_within = 0.;
        for values in self.myEnteredContainer.values_mut() {
            mean_halts_within += values.haltings as f64;
            mean_interval_halts_within += values.intervalHaltings as f64;
            let end = if values.backLeaveTime == 0. { steps2time(stopTime) } else { values.backLeaveTime };
            let time = end - values.entryTime;
            let time_within = min2(time, end - steps2time(startTime));
            if values.speedSum > 0. {
                mean_speed_within += values.speedSum / time;
            }
            if values.intervalSpeedSum > 0. {
                mean_interval_speed_within += values.intervalSpeedSum / time_within;
            }
            mean_duration_within += time;
            mean_interval_duration_within += time_within;
            // reset interval values
            values.intervalHaltings = 0;
            values.intervalSpeedSum = 0.;
        }
        self.myLastResetTime = stopTime;
        let norm = |x: f64| if vehicle_sum_within != 0 { x / vehicle_sum_within as f64 } else { -1. };
        let mean_speed_within = norm(mean_speed_within);
        let mean_halts_within = norm(mean_halts_within);
        let mean_duration_within = norm(mean_duration_within);
        let mean_interval_speed_within = norm(mean_interval_speed_within);
        let mean_interval_halts_within = norm(mean_interval_halts_within);
        let mean_interval_duration_within = norm(mean_interval_duration_within);
        let mean_time_loss_within = norm(mean_time_loss_within);
        // write values
        dev.write_attr("meanTravelTime", &self.myLastMeanTravelTime)
            .write_attr("meanOverlapTravelTime", &mean_overlap_travel_time)
            .write_attr("meanSpeed", &mean_speed)
            .write_attr("meanHaltsPerVehicle", &self.myLastMeanHaltsPerVehicle)
            .write_attr("meanTimeLoss", &self.myLastMeanTimeLoss)
            .write_attr("vehicleSum", &self.myLastVehicleSum)
            .write_attr("meanSpeedWithin", &mean_speed_within)
            .write_attr("meanHaltsPerVehicleWithin", &mean_halts_within)
            .write_attr("meanDurationWithin", &mean_duration_within)
            .write_attr("vehicleSumWithin", &vehicle_sum_within)
            .write_attr("meanIntervalSpeedWithin", &mean_interval_speed_within)
            .write_attr("meanIntervalHaltsPerVehicleWithin", &mean_interval_halts_within)
            .write_attr("meanIntervalDurationWithin", &mean_interval_duration_within)
            .write_attr("meanTimeLossWithin", &mean_time_loss_within)
            .close_tag("");
    }

    /// C++: `void MSE3Collector::writeXMLDetectorProlog(OutputDevice &) const` [const, virtual]
    /// Opens the XML-output using "e3Detector" as root element
    // void
    // MSE3Collector::writeXMLDetectorProlog(OutputDevice& dev) const {
    //     dev.writeXMLHeader("e3Detector", "det_e3_file.xsd");
    // }
    #[allow(unused_variables)]
    pub fn write_xml_detector_prolog(&self, dev: &mut OutputDevice) {
        dev.write_xml_header("e3Detector", "det_e3_file.xsd", BTreeMap::new(), true);
    }

    /// C++: `void MSE3Collector::detectorUpdate(const SUMOTime)` [virtual]
    /// Computes the detector values in each time step
    // void
    // MSE3Collector::detectorUpdate(const SUMOTime step) {
    //
    //     if (myDetectPersons != (int)PersonMode::NONE) {
    //         for (auto rem : myEntryReminders) {
    //             const MSLane* lane = rem->getLane();
    //             if (lane->hasPedestrians()) {
    //                 for (MSTransportable* p : lane->getEdge().getPersons()) {
    //                     if (p->getLane() == lane && vehicleApplies(*p)) {
    //                         notifyMovePerson(p, rem, rem->getPosition(), p->getDirection(), p->getPositionOnLane());
    //                     }
    //                 }
    //             }
    //         }
    //         for (auto rem : myLeaveReminders) {
    //             const MSLane* lane = rem->getLane();
    //             if (lane->hasPedestrians()) {
    //                 for (MSTransportable* p : lane->getEdge().getPersons()) {
    //                     if (p->getLane() == lane && vehicleApplies(*p)) {
    //                         notifyMovePerson(p, rem, rem->getPosition(), p->getDirection(), p->getPositionOnLane());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     myCurrentMeanSpeed = 0;
    //     myCurrentHaltingsNumber = 0;
    //     for (std::map<const SUMOTrafficObject*, E3Values>::iterator pair = myEnteredContainer.begin(); pair != myEnteredContainer.end(); ++pair) {
    //         const SUMOTrafficObject* veh = pair->first;
    // #ifdef DEBUG_E3_DETECTORUPDATE
    //         //if (DEBUG_COND(*this) && DEBUG_COND_VEH(*veh)) {
    //         if (DEBUG_COND(*this)) {
    //             std::cout << SIMTIME << " vehPtr=" << veh << "\n";
    //             std::cout << "       veh=" << veh->getID() << "\n";
    //         }
    // #endif
    //         E3Values& values = pair->second;
    //         myCurrentMeanSpeed += veh->getSpeed();
    //         values.hadUpdate = true;
    //         values.speedSum += veh->getSpeed() * TS;
    //         values.intervalSpeedSum += veh->getSpeed() * TS;
    //         if (veh->getSpeed() < myHaltingSpeedThreshold) {
    //             if (values.haltingBegin == -1) {
    //                 values.haltingBegin = step;
    //             }
    //             SUMOTime haltingDuration = step - values.haltingBegin;
    //             if (haltingDuration >= myHaltingTimeThreshold
    //                     && haltingDuration < (myHaltingTimeThreshold + DELTA_T)) {
    //                 values.haltings++;
    //                 values.intervalHaltings++;
    //                 myCurrentHaltingsNumber++;
    //             }
    //         } else {
    //             values.haltingBegin = -1;
    //         }
    //     }
    //     if (myEnteredContainer.size() == 0) {
    //         myCurrentMeanSpeed = -1;
    //     } else {
    //         myCurrentMeanSpeed /= (double)myEnteredContainer.size();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detector_update(&mut self, step: SUMOTime) {
        if self.myDetectPersons != PersonMode::NONE as i32 {
            for rem in self.myEntryReminders.clone() {
                let lane = rem.get_lane();
                if lane.has_pedestrians() {
                    let persons: Vec<Ptr<MSTransportable>> = lane.get_edge().get_persons().iter().copied().collect();
                    for p in persons {
                        if p.get_lane() == lane && self.vehicle_applies(&p.base) {
                            self.notify_move_person(p, rem.cast::<MSMoveReminder>(), rem.get_position(), p.get_direction(), p.get_position_on_lane());
                        }
                    }
                }
            }
            for rem in self.myLeaveReminders.clone() {
                let lane = rem.get_lane();
                if lane.has_pedestrians() {
                    let persons: Vec<Ptr<MSTransportable>> = lane.get_edge().get_persons().iter().copied().collect();
                    for p in persons {
                        if p.get_lane() == lane && self.vehicle_applies(&p.base) {
                            self.notify_move_person(p, rem.cast::<MSMoveReminder>(), rem.get_position(), p.get_direction(), p.get_position_on_lane());
                        }
                    }
                }
            }
        }

        self.myCurrentMeanSpeed = 0.;
        self.myCurrentHaltingsNumber = 0;
        for (veh, values) in self.myEnteredContainer.iter_mut() {
            let vehp = *veh;
            self.myCurrentMeanSpeed += vehp.get_speed();
            values.hadUpdate = true;
            values.speedSum += vehp.get_speed() * ts();
            values.intervalSpeedSum += vehp.get_speed() * ts();
            if vehp.get_speed() < self.myHaltingSpeedThreshold {
                if values.haltingBegin == -1 {
                    values.haltingBegin = step;
                }
                let haltingDuration = step - values.haltingBegin;
                if haltingDuration >= self.myHaltingTimeThreshold && haltingDuration < (self.myHaltingTimeThreshold + delta_t()) {
                    values.haltings += 1;
                    values.intervalHaltings += 1;
                    self.myCurrentHaltingsNumber += 1;
                }
            } else {
                values.haltingBegin = -1;
            }
        }
        if self.myEnteredContainer.is_empty() {
            self.myCurrentMeanSpeed = -1.;
        } else {
            self.myCurrentMeanSpeed /= self.myEnteredContainer.len() as f64;
        }
    }

    /// C++: `void MSE3Collector::clearState(SUMOTime)` [virtual]
    /// Remove all vehicles before quick-loading state
    // void
    // MSE3Collector::clearState(SUMOTime /* step */) {
    //     myEnteredContainer.clear();
    //     myLeftContainer.clear();
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, step: SUMOTime) {
        self.myEnteredContainer.clear();
        self.myLeftContainer.clear();
    }

    /// C++: `void MSE3Collector::notifyMovePerson(MSTransportable *, MSMoveReminder *, double, int, double)` [protected]
    // void
    // MSE3Collector::notifyMovePerson(MSTransportable* p, MSMoveReminder* rem, double detPos, int dir, double pos) {
    //     if (personApplies(*p, dir)) {
    //         const double newSpeed = p->getSpeed();
    //         const double newPos = (dir == MSPModel::FORWARD
    //                                ? pos
    //                                // position relative to detector end position
    //                                : detPos - (pos - detPos));
    //         const double oldPos = newPos - SPEED2DIST(newSpeed);
    //         if (oldPos - p->getVehicleType().getLength() <= detPos) {
    //             rem->notifyMove(*p, oldPos, newPos, newSpeed);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn notify_move_person(&mut self, p: Ptr<MSTransportable>, rem: Ptr<MSMoveReminder>, detPos: f64, dir: i32, pos: f64) {
        if self.person_applies(&p, dir) {
            let new_speed = p.get_speed();
            let new_pos = if dir == MSP_FORWARD {
                pos
            } else {
                // position relative to detector end position
                detPos - (pos - detPos)
            };
            let old_pos = new_pos - speed2dist(new_speed);
            if old_pos - p.get_vehicle_type().get_length() <= detPos {
                if self.myEntryReminders.iter().any(|e| e.cast::<MSMoveReminder>() == rem) {
                    rem.cast::<MSE3Collector_MSE3EntryReminder>().m().notify_move(&mut p.m().base, old_pos, new_pos, new_speed);
                } else {
                    rem.cast::<MSE3Collector_MSE3LeaveReminder>().m().notify_move(&mut p.m().base, old_pos, new_pos, new_speed);
                }
            }
        }
    }

    /// C++: `void MSE3Collector::MSE3Collector(const MSE3Collector &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSE3Collector) -> Self {
        panic!("MSE3Collector copy constructor is private and invalidated in SUMO")
    }

    /// C++: `MSE3Collector & MSE3Collector::operator=(const MSE3Collector &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSE3Collector) -> &mut MSE3Collector {
        panic!("MSE3Collector assignment operator is private and invalidated in SUMO")
    }

}

/// A place on the road net (at a certain lane and position on it) where the E3 area begins
/// C++ class `MSE3Collector::MSE3EntryReminder` (microsim/output/MSE3Collector.h)
#[repr(C)]
pub struct MSE3Collector_MSE3EntryReminder {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// The parent collector
    pub myCollector: Ptr<MSE3Collector>,
    /// The position on the lane
    pub myPosition: f64,
}

impl std::ops::Deref for MSE3Collector_MSE3EntryReminder {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSE3Collector_MSE3EntryReminder {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSE3Collector_MSE3EntryReminder {
    /// C++: `void MSE3EntryReminder::MSE3EntryReminder(const MSCrossSection &, MSE3Collector &)`
    /// Constructor
    // MSE3Collector::MSE3EntryReminder::MSE3EntryReminder(
    //     const MSCrossSection& crossSection, MSE3Collector& collector) :
    //     MSMoveReminder(collector.getID() + "_entry", crossSection.myLane),
    //     myCollector(collector), myPosition(crossSection.myPosition) {
    // }
    #[allow(unused_variables)]
    pub fn new(crossSection: &MSCrossSection, collector: &mut MSE3Collector) -> Self {
        // lane registration is done by MSE3Collector::build_reminders (see shortcut there)
        MSE3Collector_MSE3EntryReminder {
            base: MSMoveReminder::new(&format!("{}_entry", collector.base.base.get_id()), crossSection.myLane, false),
            myCollector: Ptr::from_mut(collector),
            myPosition: crossSection.myPosition,
        }
    }

    /// C++: `bool MSE3EntryReminder::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Checks whether the reminder is activated by a vehicle entering the lane
    // bool
    // MSE3Collector::MSE3EntryReminder::notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3EntryReminder::notifyEnter() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " enteredLane=" << enteredLane->getID()
    //                   << " reason=" << reason
    //                   << "\n";
    //     }
    // #endif
    //     if (reason != NOTIFICATION_JUNCTION) {
    //         const double posOnLane = veh.getBackPositionOnLane(enteredLane) + veh.getVehicleType().getLength();
    //         if (myLane == enteredLane && posOnLane > myPosition) {
    // #ifdef HAVE_FOX
    //             ScopedLocker<> lock(myCollector.myContainerMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //             const auto& itVeh = myCollector.myEnteredContainer.find(&veh);
    //             if (itVeh == myCollector.myEnteredContainer.end() ||
    //                     itVeh->second.entryReminder != this) {
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //                 if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //                     std::cout << "  assume already known\n";
    //                 }
    // #endif
    //                 // if the vehicle changes into a covered section we assume it was already registered on another lane
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            let pos_on_lane = veh.get_back_position_on_lane(enteredLane) + veh.get_vehicle_type().get_length();
            if self.myLane == enteredLane && pos_on_lane > self.myPosition {
                let collector = self.myCollector;
                let known_by_this = match collector.myEnteredContainer.get(&Ptr::new(veh)) {
                    Some(v) => v.entryReminder == Ptr::from_mut(self),
                    None => false,
                };
                if !known_by_this {
                    // if the vehicle changes into a covered section we assume it was already registered on another lane
                    return false;
                }
            }
        }
        true
    }

    /// C++: `bool MSE3EntryReminder::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the vehicle enters
    // bool
    // MSE3Collector::MSE3EntryReminder::notifyMove(SUMOTrafficObject& veh, double oldPos,
    //         double newPos, double newSpeed) {
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3EntryReminder::notifyMove() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " entered. oldPos=" << oldPos << " newPos=" << newPos << " newSpeed=" << newSpeed
    //                   << " myPosition=" << myPosition
    //                   << "\n";
    //     }
    // #endif
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myCollector.myContainerMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     if ((myCollector.myEnteredContainer.find(&veh) == myCollector.myEnteredContainer.end() ||
    //             (veh.isPerson() && dynamic_cast<const MSTransportable&>(veh).getDirection() != MSPModel::FORWARD))
    //             && newPos > myPosition) {
    //         if (oldPos > myPosition) {
    //             // was behind the detector already in the last step
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //             if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //                 std::cout << "    already behind, isPerson=" << veh.isPerson() << "\n";
    //             }
    // #endif
    //             return false;
    //         } else {
    //             if (veh.isVehicle() && &veh.getLane()->getEdge() == &myLane->getEdge() && veh.getLane() != myLane) {
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //                 std::cout << veh.getID() << " is on the wrong lane\n";
    // #endif
    //                 return true;
    //             }
    //             // entered in this step
    //             const double oldSpeed = veh.getPreviousSpeed();
    //             const double entryTime = STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep());
    //             assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed != 0); // how could it move across the detector otherwise
    //             const double timeBeforeEnter = MSCFModel::passingTime(oldPos, myPosition, newPos, oldSpeed, newSpeed);
    //             const double fractionTimeOnDet = TS - timeBeforeEnter;
    //             myCollector.enter(veh, entryTime - fractionTimeOnDet, fractionTimeOnDet, this);
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //             if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //                 std::cout << "    enter\n";
    //             }
    // #endif
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        let collector = self.myCollector;
        // shortcut: C++ also matches a person walking backwards (MSTransportable direction), not readable until ms_transportable.rs is #[repr(C)]
        if !collector.myEnteredContainer.contains_key(&Ptr::new(veh)) && newPos > self.myPosition {
            if oldPos > self.myPosition {
                // was behind the detector already in the last step
                return false;
            }
            if veh.is_vehicle() && std::ptr::eq(&*veh.get_lane().get_edge(), &*self.myLane.get_edge()) && veh.get_lane() != self.myLane {
                // the vehicle is on the wrong lane of the same edge
                return true;
            }
            // entered in this step
            let old_speed = veh.get_previous_speed();
            let entry_time = steps2time(current_step());
            debug_assert!(!MSGlobals::semi_implicit_euler_update() || newSpeed != 0.); // how could it move across the detector otherwise
            let time_before_enter = MSCFModel::passing_time(oldPos, self.myPosition, newPos, old_speed, newSpeed);
            let fraction_time_on_det = ts() - time_before_enter;
            collector.m().enter(&*veh, entry_time - fraction_time_on_det, fraction_time_on_det, Ptr::from_mut(self), false);
        }
        true
    }

    /// C++: `bool MSE3EntryReminder::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Processes state changes of a vehicle
    // bool
    // MSE3Collector::MSE3EntryReminder::notifyLeave(SUMOTrafficObject& veh, double, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    // #ifdef DEBUG_E3_NOTIFY_LEAVE
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3EntryReminder::notifyLeave() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " reason=" << reason
    //                   << "\n";
    //     }
    // #endif
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    // #ifdef HAVE_FOX
    //         ScopedLocker<> lock(myCollector.myContainerMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //         if (myCollector.myEnteredContainer.erase(&veh) > 0) {
    //             if (!myCollector.myExpectArrival) {
    //                 WRITE_WARNINGF("% '%' arrived inside % '%', time=%.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), myCollector.getID(), time2string(SIMSTEP));
    //             }
    //         }
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason >= MSMoveReminder_Notification::NOTIFICATION_ARRIVED {
            let collector = self.myCollector;
            if collector.m().myEnteredContainer.remove(&Ptr::new(veh)).is_some() && !collector.myExpectArrival {
                write_warningf!("% '%' arrived inside % '%', time=%.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), collector.base.base.get_id(), time2string(current_step()));
            }
            return false;
        }
        true
    }

    /// C++: `double MSE3EntryReminder::getPosition() const` [const]
    /// @}
    // double getPosition() const {
    //             return myPosition;
    //         }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        self.myPosition
    }

    /// C++: `void MSE3EntryReminder::MSE3EntryReminder(const MSE3EntryReminder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSE3Collector_MSE3EntryReminder) -> Self {
        panic!("MSE3EntryReminder copy constructor is private and invalidated in SUMO")
    }

    /// C++: `MSE3EntryReminder & MSE3EntryReminder::operator=(const MSE3EntryReminder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSE3Collector_MSE3EntryReminder) -> &mut MSE3Collector_MSE3EntryReminder {
        panic!("MSE3EntryReminder assignment operator is private and invalidated in SUMO")
    }

}

/// A place on the road net (at a certain lane and position on it) where the E3 area ends
/// C++ class `MSE3Collector::MSE3LeaveReminder` (microsim/output/MSE3Collector.h)
#[repr(C)]
pub struct MSE3Collector_MSE3LeaveReminder {
    /// C++ base class `MSMoveReminder`
    pub base: MSMoveReminder,
    /// The parent collector
    pub myCollector: Ptr<MSE3Collector>,
    /// The position on the lane
    pub myPosition: f64,
}

impl std::ops::Deref for MSE3Collector_MSE3LeaveReminder {
    type Target = MSMoveReminder;
    fn deref(&self) -> &MSMoveReminder {
        &self.base
    }
}
impl std::ops::DerefMut for MSE3Collector_MSE3LeaveReminder {
    fn deref_mut(&mut self) -> &mut MSMoveReminder {
        &mut self.base
    }
}

impl MSE3Collector_MSE3LeaveReminder {
    /// C++: `void MSE3LeaveReminder::MSE3LeaveReminder(const MSCrossSection &, MSE3Collector &)`
    /// Constructor
    // MSE3Collector::MSE3LeaveReminder::MSE3LeaveReminder(
    //     const MSCrossSection& crossSection, MSE3Collector& collector) :
    //     MSMoveReminder(collector.getID() + "_exit", crossSection.myLane),
    //     myCollector(collector), myPosition(crossSection.myPosition) {}
    #[allow(unused_variables)]
    pub fn new(crossSection: &MSCrossSection, collector: &mut MSE3Collector) -> Self {
        // lane registration is done by MSE3Collector::build_reminders (see shortcut there)
        MSE3Collector_MSE3LeaveReminder {
            base: MSMoveReminder::new(&format!("{}_exit", collector.base.base.get_id()), crossSection.myLane, false),
            myCollector: Ptr::from_mut(collector),
            myPosition: crossSection.myPosition,
        }
    }

    /// C++: `bool MSE3LeaveReminder::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    /// Checks whether the reminder is activated by a vehicle entering the lane
    // bool
    // MSE3Collector::MSE3LeaveReminder::notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3LeaveReminder::notifyEnter() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " enteredLane=" << enteredLane->getID()
    //                   << " reason=" << reason
    //                   << "\n";
    //     }
    // #endif
    //     // this method does not access containers, so no locking here
    //     if (reason != NOTIFICATION_JUNCTION) {
    //         const double backPosOnLane = veh.getBackPositionOnLane(enteredLane);
    //         if (backPosOnLane > myPosition) {
    //             // if the vehicle changes into a covered section we assume it was already registered on another lane
    //             // however, if it is not fully past the detector we still need to track it
    // #ifdef DEBUG_E3_NOTIFY_ENTER
    //             if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //                 std::cout << "  assume already known\n";
    //             }
    // #endif
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        // this method does not access containers, so no locking here
        if reason != MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            let back_pos_on_lane = veh.get_back_position_on_lane(enteredLane);
            if back_pos_on_lane > self.myPosition {
                // if the vehicle changes into a covered section we assume it was already registered on another lane
                // however, if it is not fully past the detector we still need to track it
                return false;
            }
        }
        true
    }

    /// C++: `bool MSE3LeaveReminder::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks whether the vehicle leaves
    // bool
    // MSE3Collector::MSE3LeaveReminder::notifyMove(SUMOTrafficObject& veh, double oldPos,
    //         double newPos, double newSpeed) {
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3LeaveReminder::notifyMove() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " entered. oldPos=" << oldPos << " newPos=" << newPos << " newSpeed=" << newSpeed
    //                   << " myPosition=" << myPosition
    //                   << "\n";
    //     }
    // #endif
    //     if (newPos < myPosition) {
    //         // crossSection not yet reached
    //         return true;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myCollector.myContainerMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     const double oldSpeed = veh.getPreviousSpeed();
    //     if (oldPos < myPosition) {
    //         assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed != 0); // how could it move across the detector otherwise
    //         const double timeBeforeLeave = MSCFModel::passingTime(oldPos, myPosition, newPos, oldSpeed, newSpeed);
    // //        const double leaveTimeFront = SIMTIME - TS + (myPosition - oldPos) / newSpeed;
    //         const double leaveTimeFront = SIMTIME - TS + timeBeforeLeave;
    //         myCollector.leaveFront(veh, leaveTimeFront);
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //         if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //             std::cout << "    leaveFront\n";
    //         }
    // #endif
    //     }
    //     const double backPos = newPos - veh.getVehicleType().getLength();
    //     if (backPos < myPosition) {
    //         // crossSection not yet left
    //         return true;
    //     }
    //     // crossSection left
    //     const double oldBackPos = oldPos - veh.getVehicleType().getLength();
    //     const double leaveStep = SIMTIME;
    //     assert(!MSGlobals::gSemiImplicitEulerUpdate || newSpeed != 0); // how could it move across the detector otherwise
    //     const double timeBeforeLeave = MSCFModel::passingTime(oldBackPos, myPosition, backPos, oldSpeed, newSpeed);
    //     myCollector.leave(veh, leaveStep - TS + timeBeforeLeave, timeBeforeLeave);
    // #ifdef DEBUG_E3_NOTIFY_MOVE
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << "    leave\n";
    //     }
    // #endif
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        if newPos < self.myPosition {
            // crossSection not yet reached
            return true;
        }
        let collector = self.myCollector;
        let old_speed = veh.get_previous_speed();
        if oldPos < self.myPosition {
            debug_assert!(!MSGlobals::semi_implicit_euler_update() || newSpeed != 0.); // how could it move across the detector otherwise
            let time_before_leave = MSCFModel::passing_time(oldPos, self.myPosition, newPos, old_speed, newSpeed);
            let leave_time_front = simtime() - ts() + time_before_leave;
            collector.m().leave_front(&*veh, leave_time_front);
        }
        let back_pos = newPos - veh.get_vehicle_type().get_length();
        if back_pos < self.myPosition {
            // crossSection not yet left
            return true;
        }
        // crossSection left
        let old_back_pos = oldPos - veh.get_vehicle_type().get_length();
        let leave_step = simtime();
        debug_assert!(!MSGlobals::semi_implicit_euler_update() || newSpeed != 0.); // how could it move across the detector otherwise
        let time_before_leave = MSCFModel::passing_time(old_back_pos, self.myPosition, back_pos, old_speed, newSpeed);
        collector.m().leave(&*veh, leave_step - ts() + time_before_leave, time_before_leave, false);
        false
    }

    /// C++: `bool MSE3LeaveReminder::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Processes state changes of a vehicle
    // bool
    // MSE3Collector::MSE3LeaveReminder::notifyLeave(SUMOTrafficObject&  veh, double /* lastPos */, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    // #ifdef DEBUG_E3_NOTIFY_LEAVE
    //     if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //         std::cout << SIMTIME
    //                   << " MSE3LeaveReminder::notifyLeave() (" << getDescription() << " on lane '" << myLane->getID() << "')"
    //                   << " vehicle '" << veh.getID() << "'"
    //                   << " reason=" << reason
    //                   << "\n";
    //     }
    // #endif
    //     if (reason == MSMoveReminder::NOTIFICATION_LANE_CHANGE && &enteredLane->getEdge() == &myLane->getEdge()) {
    //         // keep the detector when changing while still on the exit detector but already on a new lane (#4803)
    // #ifdef DEBUG_E3_NOTIFY_LEAVE
    //         if (DEBUG_COND(myCollector) && DEBUG_COND_VEH(veh)) {
    //             std::cout << "  remove reminder, keep in container\n";
    //         }
    // #endif
    //         return false;
    //     }
    // #ifdef HAVE_FOX
    //     ScopedLocker<> lock(myCollector.myContainerMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         WRITE_WARNINGF("% '%' teleported from % '%'.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), myCollector.getID());
    //         myCollector.myEnteredContainer.erase(&veh);
    //         return false;
    //     }
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    //         if (myCollector.myEnteredContainer.erase(&veh) > 0) {
    //             if (!myCollector.myExpectArrival) {
    //                 WRITE_WARNINGF("% '%' arrived inside % '%', time=%.", veh.getCategory(), veh.getID(), toString(SUMO_TAG_E3DETECTOR), myCollector.getID(), time2string(SIMSTEP));
    //             }
    //         }
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason == MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE && std::ptr::eq(&*enteredLane.get_edge(), &*self.myLane.get_edge()) {
            // keep the detector when changing while still on the exit detector but already on a new lane (#4803)
            return false;
        }
        let collector = self.myCollector;
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            write_warningf!("% '%' teleported from % '%'.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), collector.base.base.get_id());
            collector.m().myEnteredContainer.remove(&Ptr::new(veh));
            return false;
        }
        if reason >= MSMoveReminder_Notification::NOTIFICATION_ARRIVED {
            if collector.m().myEnteredContainer.remove(&Ptr::new(veh)).is_some() && !collector.myExpectArrival {
                write_warningf!("% '%' arrived inside % '%', time=%.", category_of(veh), veh.base.get_id(), Tags.get_string(SUMO_TAG_E3DETECTOR), collector.base.base.get_id(), time2string(current_step()));
            }
            return false;
        }
        true
    }

    /// C++: `double MSE3LeaveReminder::getPosition() const` [const]
    // double getPosition() const {
    //             return myPosition;
    //         }
    #[allow(unused_variables)]
    pub fn get_position(&self) -> f64 {
        self.myPosition
    }

    /// C++: `void MSE3LeaveReminder::MSE3LeaveReminder(const MSE3LeaveReminder &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSE3Collector_MSE3LeaveReminder) -> Self {
        panic!("MSE3LeaveReminder copy constructor is private and invalidated in SUMO")
    }

    /// C++: `MSE3LeaveReminder & MSE3LeaveReminder::operator=(const MSE3LeaveReminder &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSE3Collector_MSE3LeaveReminder) -> &mut MSE3Collector_MSE3LeaveReminder {
        panic!("MSE3LeaveReminder assignment operator is private and invalidated in SUMO")
    }

}

